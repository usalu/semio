#!/usr/bin/env bun
/**
 * 🔬 [DEBUG] Measure text→border insets for first body cell under chip baseline.
 * Usage: bun qaV-insets.ts <pdf> <page> <titlePhrase> <bodyPhrase> [scale]
 */
import { createRequire } from "node:module";
import { readFileSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";

const [pdfPath, pageArg, titlePhrase, bodyPhrase, scaleArg] = process.argv.slice(2);
const scale = Number(scaleArg ?? 6);
const pageNum = Number(pageArg);
const ticketDir = dirname(fileURLToPath(import.meta.url));

const pdfjsEntry = fileURLToPath(
  new URL("../../../../../../node_modules/pdfjs-dist/legacy/build/pdf.mjs", import.meta.url),
);
const { createCanvas } = createRequire(pdfjsEntry)("@napi-rs/canvas");
const pdfjs = await import(pdfjsEntry);
const doc = await pdfjs.getDocument({
  data: new Uint8Array(readFileSync(pdfPath)),
  useSystemFonts: true,
}).promise;
const page = await doc.getPage(pageNum);
const viewport = page.getViewport({ scale });
const content = await page.getTextContent();
const items = (content.items as { str: string; transform: number[] }[]).map((it) => {
  const [x, y] = viewport.convertToViewportPoint(it.transform[4], it.transform[5]);
  return { x, y, str: it.str };
});

const title =
  items
    .filter((c) => c.str && c.str.includes(titlePhrase))
    .sort((a, b) => b.str.length - a.str.length || a.y - b.y)[0] ?? null;
if (!title) {
  console.log(JSON.stringify({ error: "no title", titlePhrase, pageNum }));
  process.exit(1);
}

const canvas = createCanvas(Math.ceil(viewport.width), Math.ceil(viewport.height));
const ctx = canvas.getContext("2d");
await page.render({ canvasContext: ctx, viewport }).promise;
const W = canvas.width;
const H = canvas.height;
const data = ctx.getImageData(0, 0, W, H).data;
const rgb = (x: number, y: number) => {
  const i = (y * W + x) * 4;
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
const isInk = (x: number, y: number) => lum(x, y) < 100;

// chip baseline
const xL = Math.max(4, Math.floor(title.x - 20 * scale));
const xR = Math.min(W - 4, Math.floor(title.x + 180 * scale));
let base0 = -1;
let base1 = -1;
for (let y = Math.floor(title.y) + 1; y < Math.floor(title.y) + Math.round(40 * scale); y++) {
  let n = 0;
  for (let x = xL; x < xR; x += 2) if (isRule(x, y)) n++;
  if (n > ((xR - xL) / 2) * 0.25) {
    if (base0 < 0) base0 = y;
    base1 = y;
  } else if (base0 >= 0) break;
}

// left border
let borderX = Math.floor(title.x);
let best = 0;
for (let x = Math.max(2, Math.floor(title.x) - Math.round(25 * scale)); x < Math.floor(title.x) + Math.round(15 * scale); x++) {
  let n = 0;
  for (let y = base0; y < base0 + Math.round(200 * scale); y++) if (isRule(x, y)) n++;
  if (n > best) {
    best = n;
    borderX = x;
  }
}

// body text candidates BELOW chip baseline (skip header chips that share title words)
const bodyCands = items
  .filter((c) => c.str && c.y > base1 + 2 * scale && c.y < base1 + 90 * scale)
  .filter((c) => c.x > borderX + 2 * scale && c.x < borderX + 140 * scale)
  .filter((c) => c.str.length >= 2);
bodyCands.sort((a, b) => a.y - b.y || a.x - b.x);
const body =
  bodyCands.find((c) => c.str.includes(bodyPhrase)) ||
  bodyCands.find((c) => /^[A-Za-zÄÖÜ0-9]/.test(c.str)) ||
  bodyCands[0];

// Find first body ink by scanning pixels under header row
// First find header→body hrule: second major hrule below chip baseline
const hrules: number[] = [];
for (let y = base1 + 1; y < base1 + Math.round(100 * scale); y++) {
  let n = 0;
  for (let x = borderX + Math.round(10 * scale); x < borderX + Math.round(160 * scale); x += 2) if (isRule(x, y)) n++;
  if (n > 30) {
    if (!hrules.length || y > hrules[hrules.length - 1]! + 2) hrules.push(y);
  }
}

// first body ink after first hrule under chips (header row bottom) or after chip if no header
const afterHeader = hrules[0] ?? base1;
let inkTop = -1;
let inkLeft = -1;
for (let y = afterHeader + 1; y < afterHeader + Math.round(40 * scale); y++) {
  for (let x = borderX + 2; x < borderX + Math.round(100 * scale); x++) {
    if (isInk(x, y) && !isRule(x, y)) {
      // confirm a few ink neighbors (glyph, not noise)
      let neigh = 0;
      for (let dx = 0; dx < 8; dx++) if (isInk(x + dx, y)) neigh++;
      if (neigh >= 3) {
        inkTop = y;
        inkLeft = x;
        break;
      }
    }
  }
  if (inkTop >= 0) break;
}

// refine left: leftmost ink on that glyph band
if (inkTop >= 0) {
  let leftmost = inkLeft;
  for (let y = inkTop; y < inkTop + Math.round(7 * scale); y++) {
    for (let x = borderX + 1; x < inkLeft + Math.round(30 * scale); x++) {
      if (isInk(x, y) && !isRule(x, y)) {
        leftmost = Math.min(leftmost, x);
        break;
      }
    }
  }
  inkLeft = leftmost;
}

const topInsetPt = inkTop >= 0 ? +((inkTop - afterHeader - 1) / scale).toFixed(2) : null;
const leftInsetPt = inkLeft >= 0 ? +((inkLeft - borderX - 1) / scale).toFixed(2) : null;

// also measure chip→header top air if header text exists
let headerTopInsetPt: number | null = null;
if (hrules.length >= 0 && body) {
  // first ink between chip baseline and first hrule
  let hTop = -1;
  for (let y = base1 + 1; y < (hrules[0] ?? base1 + Math.round(30 * scale)); y++) {
    for (let x = borderX + 2; x < borderX + Math.round(80 * scale); x++) {
      if (isInk(x, y) && !isRule(x, y)) {
        hTop = y;
        break;
      }
    }
    if (hTop >= 0) break;
  }
  if (hTop >= 0) headerTopInsetPt = +((hTop - base1 - 1) / scale).toFixed(2);
}

const crop = (name: string, x0: number, y0: number, w: number, h: number) => {
  const c = createCanvas(w, h);
  c.getContext("2d").drawImage(canvas, x0, y0, w, h, 0, 0, w, h);
  const p = join(ticketDir, `qaV-ins-p${pageNum}-${name}.png`);
  writeFileSync(p, c.toBuffer("image/png"));
  return p;
};

const out = {
  page: pageNum,
  scale,
  title: title.str,
  body: body?.str ?? null,
  base0,
  base1,
  hairlinePt: +((base1 - base0 + 1) / scale).toFixed(2),
  borderX,
  hrules: hrules.slice(0, 4).map((y) => ({ y, ptFromBase: +((y - base1) / scale).toFixed(2) })),
  afterHeader,
  inkTop,
  inkLeft,
  topInsetPt,
  leftInsetPt,
  headerTopInsetPt,
  cropBody: inkTop >= 0 ? crop("bodycell", Math.max(0, borderX - 4), afterHeader - Math.round(2 * scale), Math.round(160 * scale), Math.round(36 * scale)) : null,
  cropSeam: crop("seam", Math.max(0, borderX - 4), base0 - Math.round(14 * scale), Math.round(180 * scale), Math.round(40 * scale)),
};
console.log(JSON.stringify(out, null, 2));
