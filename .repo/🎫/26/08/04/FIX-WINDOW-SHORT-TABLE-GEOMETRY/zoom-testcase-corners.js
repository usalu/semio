import { readFileSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { fileURLToPath } from "node:url";

const { createCanvas, loadImage } = createRequire(
  fileURLToPath(new URL("../../../../../../node_modules/@napi-rs/canvas/index.js", import.meta.url)),
)("@napi-rs/canvas");

const src = ".repo/🎫/26/08/04/FIX-WINDOW-SHORT-TABLE-GEOMETRY/qa-testcase-end.png";
const img = await loadImage(src);
const canvas = createCanvas(img.width, img.height);
const ctx = canvas.getContext("2d");
ctx.drawImage(img, 0, 0);
const data = ctx.getImageData(0, 0, img.width, img.height).data;
const W = img.width;
const H = img.height;

function lum(x, y) {
  const i = (y * W + x) * 4;
  return 0.2126 * data[i] + 0.7152 * data[i + 1] + 0.0722 * data[i + 2];
}

// Find bottom horizontal rule: row with many mid-grey pixels
let ruleY = -1;
for (let y = H - 1; y >= 0; y--) {
  let hit = 0;
  for (let x = 0; x < W; x++) {
    const L = lum(x, y);
    if (L > 90 && L < 180) hit++;
  }
  if (hit > W * 0.4) {
    ruleY = y;
    break;
  }
}

// Find L/R at ruleY
let L = -1,
  R = -1;
for (let x = 0; x < W; x++) {
  if (lum(x, ruleY) < 160) {
    if (L < 0) L = x;
    R = x;
  }
}

console.log({ W, H, ruleY, L, R });

// Print luminance map below L and R for 12 rows
function colDump(x, label) {
  const rows = [];
  for (let dy = -3; dy <= 12; dy++) {
    const y = ruleY + dy;
    if (y < 0 || y >= H) continue;
    rows.push({ dy, y, L: +lum(x, y).toFixed(1) });
  }
  console.log(label, JSON.stringify(rows));
}
colDump(L, "L");
colDump(R, "R");

// Zoom corners 8x
function zoom(x0, y0, w, h, name) {
  const z = 8;
  const out = createCanvas(w * z, h * z);
  const octx = out.getContext("2d");
  octx.imageSmoothingEnabled = false;
  octx.drawImage(canvas, x0, y0, w, h, 0, 0, w * z, h * z);
  writeFileSync(
    `.repo/🎫/26/08/04/FIX-WINDOW-SHORT-TABLE-GEOMETRY/${name}`,
    out.toBuffer("image/png"),
  );
}
zoom(L - 4, ruleY - 8, 24, 28, "zoom-tc-left.png");
zoom(R - 20, ruleY - 8, 24, 28, "zoom-tc-right.png");
