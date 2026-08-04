#!/usr/bin/env bun
/** 📏 [DEBUG] temp: scan left-border column for vertical continuity + photo top air. */
import { createRequire } from "node:module";
import { fileURLToPath } from "node:url";

const [pngPath, mode = "border"] = process.argv.slice(2);
const pdfjsEntry = fileURLToPath(new URL("../../../../../../node_modules/pdfjs-dist/legacy/build/pdf.mjs", import.meta.url));
const { loadImage, createCanvas } = createRequire(pdfjsEntry)("@napi-rs/canvas");
const img = await loadImage(pngPath);
const canvas = createCanvas(img.width, img.height);
const ctx = canvas.getContext("2d");
ctx.drawImage(img, 0, 0);
const { data } = ctx.getImageData(0, 0, img.width, img.height);
const scale = 4;

function lum(x: number, y: number) {
  const i = (y * img.width + x) * 4;
  return 0.2126 * data[i] + 0.7152 * data[i + 1] + 0.0722 * data[i + 2];
}

function isRule(L: number) {
  return L > 70 && L < 190;
}
function isPage(L: number) {
  return L > 210;
}
function isInk(L: number) {
  return L < 55;
}

if (mode === "border") {
  // Find leftmost vertical rule column
  let borderX = -1;
  for (let x = 0; x < Math.min(img.width, 80); x++) {
    let rule = 0;
    for (let y = 0; y < img.height; y++) if (isRule(lum(x, y))) rule++;
    if (rule > img.height * 0.35) {
      borderX = x;
      break;
    }
  }
  console.log(`[DEBUG] ${pngPath} borderX=${borderX}`);
  if (borderX < 0) process.exit(0);

  type Band = { kind: string; y0: number; y1: number };
  const bands: Band[] = [];
  for (let y = 0; y < img.height; y++) {
    const L = lum(borderX, y);
    const kind = isRule(L) ? "rule" : isPage(L) ? "page" : isInk(L) ? "ink" : "other";
    const last = bands[bands.length - 1];
    if (last && last.kind === kind && y === last.y1 + 1) last.y1 = y;
    else bands.push({ kind, y0: y, y1: y });
  }
  for (const b of bands) {
    const h = (b.y1 - b.y0 + 1) / scale;
    if (b.kind === "page" || b.kind === "rule" || h >= 0.5) {
      console.log(`[DEBUG] ${b.kind.padEnd(5)} y=${b.y0}-${b.y1} h=${h.toFixed(2)}pt`);
    }
  }
  // Gaps: page bands between rule bands
  for (let i = 1; i < bands.length - 1; i++) {
    const b = bands[i];
    if (b.kind === "page" && bands[i - 1].kind === "rule" && bands[i + 1].kind === "rule") {
      console.log(`[DEBUG] GAP between rules: ${(b.y1 - b.y0 + 1) / scale}pt at y=${b.y0}-${b.y1}`);
    }
  }
} else if (mode === "photo") {
  // Find topmost horizontal rule, then first non-canvas/non-page ink or photo below
  const midX0 = Math.floor(img.width * 0.1);
  const midX1 = Math.floor(img.width * 0.45);
  const rowKind: string[] = [];
  for (let y = 0; y < img.height; y++) {
    let rule = 0,
      page = 0,
      dark = 0,
      mid = 0;
    for (let x = midX0; x < midX1; x++) {
      const L = lum(x, y);
      if (isRule(L)) rule++;
      else if (isPage(L)) page++;
      else if (L < 100) dark++;
      else mid++;
    }
    const span = midX1 - midX0;
    if (rule > span * 0.35) rowKind.push("rule");
    else if (dark > 20 || mid > span * 0.15) rowKind.push("photo");
    else if (page > span * 0.5) rowKind.push("page");
    else rowKind.push("canvas");
  }
  let ruleY = -1;
  for (let y = 0; y < img.height; y++) {
    if (rowKind[y] === "rule") {
      ruleY = y;
      // take last of contiguous
      while (y + 1 < img.height && rowKind[y + 1] === "rule") y++;
      ruleY = y;
      break;
    }
  }
  let photoY = -1;
  for (let y = ruleY + 1; y < img.height; y++) {
    if (rowKind[y] === "photo") {
      photoY = y;
      break;
    }
  }
  const gap = photoY < 0 || ruleY < 0 ? -1 : (photoY - ruleY - 1) / scale;
  console.log(`[DEBUG] ${pngPath} topRuleY=${ruleY} photoY=${photoY} gap=${gap.toFixed(2)}pt (target ~5.5pt)`);
  for (let y = Math.max(0, ruleY - 2); y < Math.min(img.height, (photoY < 0 ? ruleY : photoY) + 8); y++) {
    console.log(`[DEBUG] y=${y} kind=${rowKind[y]}`);
  }
}
