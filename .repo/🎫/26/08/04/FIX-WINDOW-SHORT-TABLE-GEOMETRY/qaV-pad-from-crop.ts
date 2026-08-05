#!/usr/bin/env bun
/**
 * 🔬 [DEBUG] Measure left/top air from existing body crops (scale 6).
 * Crops are expected to start a few px above the header→body hrule, left at border.
 */
import { createRequire } from "node:module";
import { fileURLToPath } from "node:url";

const png = process.argv[2]!;
const scale = Number(process.argv[3] ?? 6);
const pdfjsEntry = fileURLToPath(
  new URL("../../../../../../node_modules/pdfjs-dist/legacy/build/pdf.mjs", import.meta.url),
);
const { loadImage, createCanvas } = createRequire(pdfjsEntry)("@napi-rs/canvas");
const img = await loadImage(png);
const c = createCanvas(img.width, img.height);
c.getContext("2d").drawImage(img, 0, 0);
const { data } = c.getContext("2d").getImageData(0, 0, img.width, img.height);
const rgb = (x: number, y: number) => {
  const i = (y * img.width + x) * 4;
  return [data[i], data[i + 1], data[i + 2]] as const;
};
const lum = (x: number, y: number) => {
  const [r, g, b] = rgb(x, y);
  return 0.2126 * r + 0.7152 * g + 0.0722 * b;
};
const isRuleRgb = (x: number, y: number) => {
  const [r, g, b] = rgb(x, y);
  return Math.abs(r - 123) + Math.abs(g - 130) + Math.abs(b - 125) <= 45;
};

// find left border x (most rule-rgb column in left 20px)
let borderX = 0;
let best = 0;
for (let x = 0; x < Math.min(40, img.width); x++) {
  let n = 0;
  for (let y = 0; y < img.height; y++) if (isRuleRgb(x, y)) n++;
  if (n > best) {
    best = n;
    borderX = x;
  }
}

// find first full-width-ish hrule
let ruleY0 = -1;
let ruleY1 = -1;
for (let y = 0; y < img.height; y++) {
  let n = 0;
  for (let x = borderX + 8; x < Math.min(img.width - 4, borderX + Math.round(120 * scale)); x += 2) {
    if (isRuleRgb(x, y)) n++;
  }
  if (n > 30) {
    if (ruleY0 < 0) ruleY0 = y;
    ruleY1 = y;
  } else if (ruleY0 >= 0 && ruleY1 - ruleY0 >= 0) {
    // keep first cluster only
    break;
  }
}

// first ink below rule (body) — allow light-gray ink (TOC keys ~160?)
let inkTop = -1;
let inkLeft = -1;
for (let y = ruleY1 + 1; y < img.height; y++) {
  for (let x = borderX + 2; x < Math.min(img.width - 2, borderX + Math.round(100 * scale)); x++) {
    const L = lum(x, y);
    if (L < 180 && !isRuleRgb(x, y)) {
      // must differ from canvas cream
      const [r, g, b] = rgb(x, y);
      const canvasDist = Math.abs(r - 240) + Math.abs(g - 236) + Math.abs(b - 221);
      if (canvasDist < 20) continue;
      let run = 0;
      for (let dx = 0; dx < 10; dx++) {
        const L2 = lum(x + dx, y);
        const c2 = rgb(x + dx, y);
        if (L2 < 180 && Math.abs(c2[0] - 240) + Math.abs(c2[1] - 236) + Math.abs(c2[2] - 221) >= 20) run++;
      }
      if (run >= 4) {
        inkTop = y;
        inkLeft = x;
        break;
      }
    }
  }
  if (inkTop >= 0) break;
}
if (inkTop >= 0) {
  let left = inkLeft;
  for (let y = inkTop; y < Math.min(img.height, inkTop + Math.round(8 * scale)); y++) {
    for (let x = borderX + 1; x <= inkLeft; x++) {
      const L = lum(x, y);
      const [r, g, b] = rgb(x, y);
      if (L < 180 && !isRuleRgb(x, y) && Math.abs(r - 240) + Math.abs(g - 236) + Math.abs(b - 221) >= 20) {
        left = Math.min(left, x);
        break;
      }
    }
  }
  inkLeft = left;
}

// vertical ladder at border through first join
const borderLadder: { y: number; kind: string; L: number }[] = [];
for (let y = 0; y < img.height; y++) {
  const L = lum(borderX, y);
  const kind = isRuleRgb(borderX, y) ? "RULE" : L > 220 ? "CREAM" : "OTHER";
  borderLadder.push({ y, kind, L: +L.toFixed(0) });
}
// compress runs
const runs: { kind: string; y0: number; y1: number }[] = [];
for (const p of borderLadder) {
  const last = runs[runs.length - 1];
  if (last && last.kind === p.kind) last.y1 = p.y;
  else runs.push({ kind: p.kind, y0: p.y, y1: p.y });
}

console.log(
  JSON.stringify(
    {
      png,
      scale,
      borderX,
      ruleY0,
      ruleY1,
      hairlinePt: ruleY0 >= 0 ? +((ruleY1 - ruleY0 + 1) / scale).toFixed(2) : null,
      inkTop,
      inkLeft,
      topInsetPt: inkTop >= 0 ? +((inkTop - ruleY1 - 1) / scale).toFixed(2) : null,
      leftInsetPt: inkLeft >= 0 ? +((inkLeft - borderX - 1) / scale).toFixed(2) : null,
      borderRuns: runs.filter((r) => r.y1 - r.y0 >= 1 || r.kind === "CREAM"),
      sampleInk: inkTop >= 0 ? rgb(inkLeft, inkTop) : null,
      sampleRule: ruleY0 >= 0 ? rgb(borderX + 40, ruleY0) : null,
    },
    null,
    2,
  ),
);
