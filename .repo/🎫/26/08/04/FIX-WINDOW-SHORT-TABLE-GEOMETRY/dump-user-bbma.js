import { writeFileSync } from "node:fs";
import { createCanvas, loadImage } from "@napi-rs/canvas";

const path =
  "C:/Users/Kinosh/.cursor/projects/e-semio/assets/c__Users_Kinosh_AppData_Roaming_Cursor_User_workspaceStorage_empty-window_images_image-03be6b14-2a67-429d-85ac-2fef38c699f2.png";
const ticket = ".repo/🎫/26/08/04/FIX-WINDOW-SHORT-TABLE-GEOMETRY";
const img = await loadImage(path);
const c = createCanvas(img.width, img.height);
const ctx = c.getContext("2d");
ctx.drawImage(img, 0, 0);
const { width: W, height: H } = c;
const d = ctx.getImageData(0, 0, W, H).data;
const lum = (x, y) => {
  const i = (y * W + x) * 4;
  return 0.2126 * d[i] + 0.7152 * d[i + 1] + 0.0722 * d[i + 2];
};
let bg = 0;
for (let i = 0; i < 10; i++) bg += lum(2, 2 + i);
bg /= 10;

const rows = [];
for (let y = 0; y < H; y++) {
  // find first bright column, and also profile x=30..60
  let first = -1;
  const prof = [];
  for (let x = 0; x < 80; x++) {
    const L = lum(x, y);
    if (L > bg + 35 && first < 0) first = x;
    if (x >= 30 && x <= 60) prof.push(Math.round(L));
  }
  if (y % 5 === 0 || (first >= 0 && (y < 220))) {
    rows.push({ y, first, prof: y % 10 === 0 ? prof : undefined });
  }
}
console.log(JSON.stringify({ bg, sample: rows.filter((r) => r.first >= 0).slice(0, 40) }, null, 2));

// For y in chip mid and body mid, dump x=0..70 luminance
function dumpY(y) {
  const a = [];
  for (let x = 0; x < 70; x++) a.push(`${x}:${Math.round(lum(x, y))}`);
  console.log("y" + y, a.join(" "));
}
dumpY(80); // chip
dumpY(140); // join / header
dumpY(160);
dumpY(190); // body
