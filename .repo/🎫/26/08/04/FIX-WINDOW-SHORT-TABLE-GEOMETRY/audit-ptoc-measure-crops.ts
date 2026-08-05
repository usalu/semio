#!/usr/bin/env bun
/** 📏 [DEBUG] Measure pad/inset from audit-ptoc crop PNGs. */
import { createRequire } from "node:module";
import { readdirSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";

const ticketDir = dirname(fileURLToPath(import.meta.url));
const pdfjsEntry = fileURLToPath(
  new URL("../../../../../../node_modules/pdfjs-dist/legacy/build/pdf.mjs", import.meta.url),
);
const { loadImage, createCanvas } = createRequire(pdfjsEntry)("@napi-rs/canvas");
const scale = 6;

const files = readdirSync(ticketDir).filter((f) => f.startsWith("audit-ptoc-") && f.endsWith(".png"));
const out: Record<string, unknown>[] = [];

for (const f of files) {
  if (f.includes("-page-")) continue;
  const img = await loadImage(join(ticketDir, f));
  const c = createCanvas(img.width, img.height);
  const ctx = c.getContext("2d");
  ctx.drawImage(img, 0, 0);
  const { data } = ctx.getImageData(0, 0, img.width, img.height);
  const rgb = (x: number, y: number) => {
    const i = (y * img.width + x) * 4;
    return [data[i], data[i + 1], data[i + 2]] as const;
  };
  const lum = (x: number, y: number) => {
    const [r, g, b] = rgb(x, y);
    return 0.2126 * r + 0.7152 * g + 0.0722 * b;
  };
  const isRule = (x: number, y: number) => {
    const L = lum(x, y);
    const [r, g, b] = rgb(x, y);
    return L > 45 && L < 175 && Math.abs(r - g) < 45 && Math.abs(g - b) < 45;
  };
  const isPhoto = (x: number, y: number) => {
    const [r, g, b] = rgb(x, y);
    return Math.max(r, g, b) - Math.min(r, g, b) > 35;
  };
  const isInk = (x: number, y: number) => {
    const L = lum(x, y);
    const [r, g, b] = rgb(x, y);
    const canvasDist = Math.abs(r - 240) + Math.abs(g - 236) + Math.abs(b - 221);
    return L < 175 && canvasDist > 24 && !isRule(x, y);
  };

  let borderX = 0;
  let best = 0;
  for (let x = 0; x < Math.min(40, img.width); x++) {
    let n = 0;
    for (let y = 0; y < img.height; y++) if (isRule(x, y)) n++;
    if (n > best) {
      best = n;
      borderX = x;
    }
  }

  // first hrule cluster
  let ruleY0 = -1;
  let ruleY1 = -1;
  for (let y = 0; y < img.height; y++) {
    let n = 0;
    for (let x = borderX + 6; x < Math.min(img.width - 2, borderX + 200); x += 2) if (isRule(x, y)) n++;
    if (n > 25) {
      if (ruleY0 < 0) ruleY0 = y;
      ruleY1 = y;
    } else if (ruleY0 >= 0) break;
  }

  let inkX = -1;
  let inkY = -1;
  if (ruleY1 >= 0) {
    outer: for (let y = ruleY1 + 1; y < Math.min(img.height, ruleY1 + 80); y++) {
      for (let x = borderX + 2; x < Math.min(img.width - 2, borderX + 120); x++) {
        if (!isInk(x, y)) continue;
        let run = 0;
        for (let dx = 0; dx < 6; dx++) if (isInk(x + dx, y)) run++;
        if (run >= 3) {
          inkX = x;
          inkY = y;
          break outer;
        }
      }
    }
  }

  let photoY = -1;
  for (let y = Math.max(0, ruleY1 + 1); y < img.height; y++) {
    let n = 0;
    for (let x = borderX + 4; x < Math.min(img.width, borderX + 140); x += 2) if (isPhoto(x, y)) n++;
    if (n > 12) {
      photoY = y;
      break;
    }
  }

  // PAGE cream inside L border on every hrule (join notch)
  let pageInsideJoins = 0;
  for (let y = 0; y < img.height; y++) {
    let n = 0;
    for (let x = borderX + 8; x < Math.min(img.width - 2, borderX + 160); x += 2) if (isRule(x, y)) n++;
    if (n < 20) continue;
    for (const dx of [1, 2, 3]) {
      const [r, g, b] = rgb(borderX + dx, y);
      if (Math.abs(r - 247) + Math.abs(g - 243) + Math.abs(b - 227) <= 16) {
        pageInsideJoins++;
        break;
      }
    }
  }

  out.push({
    file: f,
    borderX,
    ruleY0,
    ruleY1,
    leftInsetPt: inkX >= 0 ? Number(((inkX - borderX) / scale).toFixed(2)) : null,
    topInsetPt: inkY >= 0 && ruleY1 >= 0 ? Number(((inkY - ruleY1) / scale).toFixed(2)) : null,
    photoPadPt: photoY >= 0 && ruleY1 >= 0 ? Number(((photoY - ruleY1 - 1) / scale).toFixed(2)) : null,
    pageInsideJoins,
  });
}

writeFileSync(join(ticketDir, "audit-ptoc-crop-measures.json"), JSON.stringify(out, null, 2));
for (const r of out) {
  if (String(r.file).includes("Ljoin") || String(r.file).includes("photo") || String(r.file).includes("body") || String(r.file).includes("chip")) {
    console.log(
      `[DEBUG] ${r.file} L=${r.leftInsetPt} T=${r.topInsetPt} photo=${r.photoPadPt} pageJoins=${r.pageInsideJoins}`,
    );
  }
}
