#!/usr/bin/env bun
import { createRequire } from "node:module";
import { writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

const pdfjsEntry = fileURLToPath(
  new URL("../../../../../../node_modules/pdfjs-dist/legacy/build/pdf.mjs", import.meta.url),
);
const { createCanvas, loadImage } = createRequire(pdfjsEntry)("@napi-rs/canvas");
const ticket = ".repo/🎫/26/08/04/FIX-WINDOW-SHORT-TABLE-GEOMETRY";

async function crop(p: number, tag: string, y0f: number, y1f: number, x0f: number, x1f: number) {
  const img = await loadImage(`${ticket}/allA-${String(p).padStart(3, "0")}.png`);
  const w = img.width;
  const h = img.height;
  const x0 = Math.floor(w * x0f);
  const x1 = Math.floor(w * x1f);
  const y0 = Math.floor(h * y0f);
  const y1 = Math.floor(h * y1f);
  const out = createCanvas(x1 - x0, y1 - y0);
  out.getContext("2d").drawImage(img, x0, y0, x1 - x0, y1 - y0, 0, 0, x1 - x0, y1 - y0);
  writeFileSync(`${ticket}/audit-flag-${tag}.png`, out.toBuffer("image/png"));
}

await crop(3, "toc", 0.12, 0.45, 0.12, 0.88);
await crop(24, "pk", 0.28, 0.62, 0.12, 0.88);
await crop(77, "ueb", 0.18, 0.55, 0.12, 0.88);

const img = await loadImage(`${ticket}/allA-077.png`);
const w = img.width;
const h = img.height;
const c = createCanvas(w, h);
c.getContext("2d").drawImage(img, 0, 0);
const { data } = c.getContext("2d").getImageData(0, 0, w, h);
const L = (x: number, y: number) => {
  const i = (y * w + x) * 4;
  return 0.2126 * data[i] + 0.7152 * data[i + 1] + 0.0722 * data[i + 2];
};
let bodyL = -1;
for (let x = 200; x < 400; x++) {
  let ink = 0;
  for (let y = 400; y < 900; y++) if (L(x, y) < 140) ink++;
  if (ink > 150) {
    bodyL = x;
    break;
  }
}
let top = -1;
for (let y = 350; y < 700; y++) {
  let ink = 0;
  for (let x = 300; x < 1400; x += 2) if (L(x, y) < 130) ink++;
  if (ink > 200 && L(bodyL, y) < 150) {
    top = y;
    break;
  }
}
console.log(`[DEBUG] ueb bodyL=${bodyL} top=${top}`);
const sx0 = bodyL - 10;
const sx1 = bodyL + 70;
const sy0 = top - 40;
const sy1 = top + 220;
const slice = createCanvas(sx1 - sx0, sy1 - sy0);
slice.getContext("2d").drawImage(c, sx0, sy0, sx1 - sx0, sy1 - sy0, 0, 0, sx1 - sx0, sy1 - sy0);
const z = 4;
const zoom = createCanvas((sx1 - sx0) * z, (sy1 - sy0) * z);
const zc = zoom.getContext("2d");
zc.imageSmoothingEnabled = false;
zc.drawImage(slice, 0, 0, sx1 - sx0, sy1 - sy0, 0, 0, (sx1 - sx0) * z, (sy1 - sy0) * z);
writeFileSync(`${ticket}/audit-flag-ueb-Lzoom.png`, zoom.toBuffer("image/png"));

// Kopfbau photo pad truth
const pk = await loadImage(`${ticket}/allA-024.png`);
const pw = pk.width;
const ph = pk.height;
const pc = createCanvas(pw, ph);
pc.getContext("2d").drawImage(pk, 0, 0);
const pd = pc.getContext("2d").getImageData(0, 0, pw, ph).data;
const PL = (x: number, y: number) => {
  const i = (y * pw + x) * 4;
  return 0.2126 * pd[i] + 0.7152 * pd[i + 1] + 0.0722 * pd[i + 2];
};
let pL = -1;
for (let x = 250; x < 350; x++) {
  let ink = 0;
  for (let y = 600; y < 1000; y++) if (PL(x, y) < 140) ink++;
  if (ink > 120) {
    pL = x;
    break;
  }
}
let pTop = -1;
for (let y = 500; y < 900; y++) {
  let ink = 0;
  for (let x = 300; x < 1500; x += 2) if (PL(x, y) < 130) ink++;
  if (ink > 220 && PL(pL, y) < 150) {
    pTop = y;
    break;
  }
}
let pad = -1;
for (let y = pTop + 1; y < pTop + 50; y++) {
  let dark = 0;
  for (let x = pL + 20; x < pL + 200; x++) if (PL(x, y) < 90) dark++;
  if (dark > 40) {
    pad = +((y - pTop - 1) / 3).toFixed(2);
    break;
  }
}
console.log(`[DEBUG] kopfbau pL=${pL} top=${pTop} photoPadPt=${pad}`);
