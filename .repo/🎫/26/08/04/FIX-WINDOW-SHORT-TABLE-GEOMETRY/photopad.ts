#!/usr/bin/env bun
/** 📷 [DEBUG] temp: measure dark/canvas pad between top rule and blue-sky photo. */
import { createRequire } from "node:module";
import { fileURLToPath } from "node:url";

const [pngPath] = process.argv.slice(2);
const pdfjsEntry = fileURLToPath(new URL("../../../../../../node_modules/pdfjs-dist/legacy/build/pdf.mjs", import.meta.url));
const { loadImage, createCanvas } = createRequire(pdfjsEntry)("@napi-rs/canvas");
const img = await loadImage(pngPath);
const canvas = createCanvas(img.width, img.height);
const ctx = canvas.getContext("2d");
ctx.drawImage(img, 0, 0);
const { data } = ctx.getImageData(0, 0, img.width, img.height);
const scale = 4;

function px(x: number, y: number) {
  const i = (y * img.width + x) * 4;
  return { r: data[i], g: data[i + 1], b: data[i + 2] };
}

// Sample through the photo column (left third, inset from borders)
const x0 = Math.floor(img.width * 0.15);
const x1 = Math.floor(img.width * 0.4);

type Kind = "rule" | "sky" | "dark" | "other";
const kinds: Kind[] = [];
for (let y = 0; y < img.height; y++) {
  let rule = 0,
    sky = 0,
    dark = 0;
  for (let x = x0; x < x1; x++) {
    const { r, g, b } = px(x, y);
    const L = 0.2126 * r + 0.7152 * g + 0.0722 * b;
    // blue-ish sky
    if (b > r + 15 && b > g && b > 80) sky++;
    else if (L > 80 && L < 200 && Math.abs(r - g) < 25 && Math.abs(g - b) < 25) rule++;
    else if (L < 55) dark++;
  }
  const span = x1 - x0;
  if (rule > span * 0.25) kinds.push("rule");
  else if (sky > span * 0.15) kinds.push("sky");
  else if (dark > span * 0.4) kinds.push("dark");
  else kinds.push("other");
}

let lastRule = -1;
for (let y = 0; y < kinds.length; y++) if (kinds[y] === "rule") lastRule = y;
let firstSky = -1;
for (let y = 0; y < kinds.length; y++) {
  if (kinds[y] === "sky") {
    firstSky = y;
    break;
  }
}
let darkBetween = 0;
if (lastRule >= 0 && firstSky > lastRule) {
  for (let y = lastRule + 1; y < firstSky; y++) if (kinds[y] === "dark" || kinds[y] === "other") darkBetween++;
}

console.log(`[DEBUG] ${pngPath}`);
console.log(`[DEBUG] lastRuleY=${lastRule} firstSkyY=${firstSky} gapPx=${firstSky - lastRule - 1} gapPt=${((firstSky - lastRule - 1) / scale).toFixed(2)} dark/otherBetweenPt=${(darkBetween / scale).toFixed(2)} target=5.50pt`);
for (let y = Math.max(0, lastRule - 2); y < Math.min(img.height, firstSky + 6); y++) {
  console.log(`[DEBUG] y=${y} ${kinds[y]}`);
}
