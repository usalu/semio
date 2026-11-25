/** 🔬️ Prints RGB pixels of a png. Usage: node pixels.mjs <png> <x0> <y0> <x1> <y1> */
import { readFileSync } from "node:fs";
import { chromium } from "playwright";
const [file, ...box] = process.argv.slice(2);
const [x0, y0, x1, y1] = box.map(Number);
const browser = await chromium.launch({ channel: "chromium" });
const page = await browser.newPage();
const data = readFileSync(file).toString("base64");
const rows = await page.evaluate(async ([data, x0, y0, x1, y1]) => {
  const image = new Image(); image.src = "data:image/png;base64," + data; await image.decode();
  const canvas = document.createElement("canvas"); canvas.width = image.width; canvas.height = image.height;
  const context = canvas.getContext("2d"); context.drawImage(image, 0, 0);
  const out = [];
  for (let y = y0; y <= y1; y++) { const row = []; for (let x = x0; x <= x1; x++) { const p = context.getImageData(x, y, 1, 1).data; row.push(`${p[0]},${p[1]},${p[2]}`); } out.push(`y${y}: ${row.join(" | ")}`); }
  return out;
}, [data, x0, y0, x1, y1]);
console.log(rows.join("\n"));
await browser.close();
