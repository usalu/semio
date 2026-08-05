#!/usr/bin/env bun
/** 📐 [DEBUG] temp: measure L/R border jog + mid-rule thickness at joins. */
import { createRequire } from "node:module";
import { readFileSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

const [pdfPath, pageArg, needle, outPrefix, scaleArg] = process.argv.slice(2);
const scale = Number(scaleArg ?? 6);
const pdfjsEntry = fileURLToPath(
  new URL("../../../../../../node_modules/pdfjs-dist/legacy/build/pdf.mjs", import.meta.url),
);
const { createCanvas } = createRequire(pdfjsEntry)("@napi-rs/canvas");
const pdfjs = await import(pdfjsEntry);
const doc = await pdfjs
  .getDocument({ data: new Uint8Array(readFileSync(pdfPath)), useSystemFonts: true })
  .promise;
const page = await doc.getPage(Number(pageArg));
const tc = await page.getTextContent();
const items = tc.items as { str: string; transform: number[]; width: number }[];
const hit = items.find((i) => i.str.includes(needle));
if (!hit) throw new Error(`needle not found: ${needle}`);
const hx = hit.transform[4];
const hy = hit.transform[5];

const viewport = page.getViewport({ scale });
const canvas = createCanvas(Math.ceil(viewport.width), Math.ceil(viewport.height));
await page.render({ canvasContext: canvas.getContext("2d"), viewport }).promise;
const ctx = canvas.getContext("2d");
const { data, width, height } = ctx.getImageData(0, 0, canvas.width, canvas.height);
const pageH = viewport.height / scale;
const lum = (x: number, y: number) => {
  if (x < 0 || y < 0 || x >= width || y >= height) return 255;
  const i = (y * width + x) * 4;
  return 0.2126 * data[i] + 0.7152 * data[i + 1] + 0.0722 * data[i + 2];
};
const isBorder = (x: number, y: number) => lum(x, y) < 170;

// Scan a vertical band around the needle for outer L border x per row of pixels.
const yMid = Math.round((pageH - hy) * scale);
const y0 = Math.max(0, yMid - Math.round(90 * scale));
const y1 = Math.min(height - 1, yMid + Math.round(40 * scale));
const xScan0 = Math.max(0, Math.round((hx - 80) * scale));
const xScan1 = Math.min(width - 1, Math.round((hx + 20) * scale));

type Edge = { y: number; leftX: number; rightX: number };
const edges: Edge[] = [];
for (let y = y0; y <= y1; y++) {
  let leftX = -1;
  for (let x = xScan0; x <= xScan1; x++) {
    if (isBorder(x, y)) {
      leftX = x;
      break;
    }
  }
  if (leftX < 0) continue;
  // table right: scan from page-ish right of this table (~linewidth)
  let rightX = -1;
  const r0 = Math.round((hx + 350) * scale);
  const r1 = Math.min(width - 1, Math.round((hx + 520) * scale));
  for (let x = r1; x >= r0; x--) {
    if (isBorder(x, y)) {
      rightX = x;
      break;
    }
  }
  edges.push({ y, leftX, rightX });
}

const leftXs = edges.map((e) => e.leftX).filter((x) => x >= 0);
const rightXs = edges.map((e) => e.rightX).filter((x) => x >= 0);
const mode = (arr: number[]) => {
  const m = new Map<number, number>();
  for (const v of arr) m.set(v, (m.get(v) ?? 0) + 1);
  let best = arr[0];
  let n = 0;
  for (const [v, c] of m) if (c > n) {
    best = v;
    n = c;
  }
  return best;
};
const leftMode = mode(leftXs);
const rightMode = mode(rightXs);
const leftDev = leftXs.filter((x) => Math.abs(x - leftMode) > 1).length;
const rightDev = rightXs.filter((x) => Math.abs(x - rightMode) > 1).length;
const leftUnique = [...new Set(leftXs)].sort((a, b) => a - b);
const rightUnique = [...new Set(rightXs)].sort((a, b) => a - b);

// Mid-rule thickness: find horizontal border rows (many ink px across)
const hRows: { y: number; run: number }[] = [];
for (let y = y0; y <= y1; y++) {
  let run = 0;
  for (let x = leftMode + 5; x < leftMode + Math.round(200 * scale) && x < width; x++) {
    if (isBorder(x, y)) run++;
  }
  if (run > 80 * scale) hRows.push({ y, run });
}
const clusters: number[][] = [];
for (const r of hRows) {
  const last = clusters[clusters.length - 1];
  if (last && r.y - last[last.length - 1] <= 2) last.push(r.y);
  else clusters.push([r.y]);
}
const midThickness = clusters.map((c) => c.length);

// Crop L join zoom
const cx0 = leftMode - 8;
const cx1 = leftMode + 40;
const crop = createCanvas(cx1 - cx0, y1 - y0);
crop.getContext("2d").drawImage(canvas, cx0, y0, cx1 - cx0, y1 - y0, 0, 0, cx1 - cx0, y1 - y0);
writeFileSync(`${outPrefix}-L.png`, crop.toBuffer("image/png"));
const rx0 = rightMode - 40;
const rx1 = rightMode + 8;
const cropR = createCanvas(rx1 - rx0, y1 - y0);
cropR.getContext("2d").drawImage(canvas, rx0, y0, rx1 - rx0, y1 - y0, 0, 0, rx1 - rx0, y1 - y0);
writeFileSync(`${outPrefix}-R.png`, cropR.toBuffer("image/png"));

const result = {
  needle,
  page: Number(pageArg),
  scale,
  leftMode,
  rightMode,
  leftUnique,
  rightUnique,
  leftDevPx: leftDev,
  rightDevPx: rightDev,
  midThicknessPx: midThickness,
  maxMidThickness: Math.max(0, ...midThickness),
  hairlinePx: (0.75 * scale),
};
console.log(JSON.stringify(result, null, 2));
writeFileSync(`${outPrefix}.json`, JSON.stringify(result, null, 2));
