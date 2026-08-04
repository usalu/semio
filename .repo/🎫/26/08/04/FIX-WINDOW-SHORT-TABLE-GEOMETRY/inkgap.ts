#!/usr/bin/env bun
/** 📐 [DEBUG] temp: measure px from topmost hairline to first text ink in a crop PNG. */
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { fileURLToPath } from "node:url";

const [pngPath] = process.argv.slice(2);
const pdfjsEntry = fileURLToPath(new URL("../../../../../../node_modules/pdfjs-dist/legacy/build/pdf.mjs", import.meta.url));
const { loadImage, createCanvas } = createRequire(pdfjsEntry)("@napi-rs/canvas");
const img = await loadImage(pngPath);
const canvas = createCanvas(img.width, img.height);
const ctx = canvas.getContext("2d");
ctx.drawImage(img, 0, 0);
const data = ctx.getImageData(0, 0, img.width, img.height).data;

function lum(i: number) {
  return 0.2126 * data[i] + 0.7152 * data[i + 1] + 0.0722 * data[i + 2];
}

const midX0 = Math.floor(img.width * 0.02);
const midX1 = Math.floor(img.width * 0.98);
const rowInk: number[] = [];
const rowRule: number[] = [];
for (let y = 0; y < img.height; y++) {
  let ink = 0;
  let rule = 0;
  let dark = 0;
  for (let x = midX0; x < midX1; x++) {
    const i = (y * img.width + x) * 4;
    const L = lum(i);
    if (L < 40) dark++;
    else if (L < 160) rule++;
    else ink++;
  }
  const span = midX1 - midX0;
  rowInk.push(ink);
  rowRule.push(rule > span * 0.25 ? rule : 0);
}

const rules: number[] = [];
for (let y = 0; y < img.height; y++) if (rowRule[y] > 0) rules.push(y);
const collapsedRules: { y0: number; y1: number }[] = [];
for (const y of rules) {
  const last = collapsedRules[collapsedRules.length - 1];
  if (last && y === last.y1 + 1) last.y1 = y;
  else collapsedRules.push({ y0: y, y1: y });
}

console.log(`[DEBUG] ${pngPath} ${img.width}x${img.height}`);
for (let r = 0; r < collapsedRules.length; r++) {
  const rule = collapsedRules[r];
  // find first ink below this rule
  let inkY = -1;
  for (let y = rule.y1 + 1; y < img.height; y++) {
    if (rowInk[y] > 12) {
      inkY = y;
      break;
    }
  }
  let inkAbove = -1;
  for (let y = rule.y0 - 1; y >= 0; y--) {
    if (rowInk[y] > 12) {
      inkAbove = y;
      break;
    }
  }
  const below = inkY < 0 ? "n/a" : `${((inkY - rule.y1 - 1) / 4).toFixed(2)}pt`;
  const above = inkAbove < 0 ? "n/a" : `${((rule.y0 - inkAbove - 1) / 4).toFixed(2)}pt`;
  console.log(`[DEBUG] rule ${rule.y0}-${rule.y1}: gap-above-ink=${above}  gap-below-ink=${below}`);
}
