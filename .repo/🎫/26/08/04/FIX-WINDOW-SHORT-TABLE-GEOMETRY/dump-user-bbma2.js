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

// Find horizontal rules: rows with many mid-grey pixels
const rules = [];
for (let y = 0; y < H; y++) {
  let mid = 0;
  for (let x = 0; x < W; x++) {
    const L = lum(x, y);
    if (L > 80 && L < 180) mid++;
  }
  if (mid > W * 0.25) rules.push({ y, mid });
}
console.log(
  "rules",
  rules.filter((r, i, a) => i === 0 || r.y > a[i - 1].y + 2).slice(0, 20),
);

// For each rule y, find leftmost mid-grey
for (const r of rules.filter((r, i, a) => i === 0 || r.y > a[i - 1].y + 2).slice(0, 10)) {
  let Lx = -1;
  for (let x = 0; x < W; x++) {
    const L = lum(x, r.y);
    if (L > 80 && L < 180) {
      Lx = x;
      break;
    }
  }
  let Rx = -1;
  for (let x = W - 1; x >= 0; x--) {
    const L = lum(x, r.y);
    if (L > 80 && L < 180) {
      Rx = x;
      break;
    }
  }
  console.log("rule y", r.y, "L", Lx, "R", Rx, "span", Rx - Lx);
}

// Chip left at 37; scan full height at x=37 and x=36..45 for continuity into table
for (const x of [35, 36, 37, 38, 39, 40, 41, 50, 51, 52]) {
  const ys = [];
  for (let y = 40; y < 220; y++) {
    if (lum(x, y) > 80) ys.push(y);
  }
  // collapse
  const bands = [];
  for (const y of ys) {
    const last = bands[bands.length - 1];
    if (last && y === last[1] + 1) last[1] = y;
    else bands.push([y, y]);
  }
  console.log(
    "x" + x,
    bands
      .slice(0, 6)
      .map((b) => b[0] + "-" + b[1])
      .join(", "),
  );
}
