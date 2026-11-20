/** 🔍 Enlarges the drawing of every 2D panel SVG that `r3-2d.ts --svg` wrote for the gas meter: a square crop in paper millimetres round the drawing, three viewpoints side by side for the line and the solid view.
 *
 * Usage: `bun r3-gas-meter-zoom.ts <png path written by r3-2d.ts --svg> [size ex ey fx fy px py] [views]` (crop size and the crop centres of elevation, front and plan in millimetres of the panel; default: the 1:50 drawing; views default light-line,light-solid).
 * Output: `🗑️generated/r3-gas-meter/zoom-<name>.png`. */
import { mkdirSync, readFileSync } from "node:fs";
import { basename, join } from "node:path";
import sharp from "sharp";

const png = process.argv[2];
if (!png) throw new Error("usage: r3-gas-meter-zoom.ts <png> [size ex ey fx fy px py] [views]");
const n = process.argv.slice(3, 10).map(Number);
const [size, ex, ey, fx, fy, px, py] = n.length === 7 && n.every(Number.isFinite) ? n : [6.4, 13.4, 8.2, 13.2, 8.2, 7.6, 7.6];
const views = (process.argv[10] ?? "light-line,light-solid").split(",");
const centres: Record<string, [number, number]> = { elevation: [ex, ey], front: [fx, fy], plan: [px, py] };
const out = join(import.meta.dir, "🗑️generated/r3-gas-meter");
mkdirSync(out, { recursive: true });
const tiles: { input: Buffer; left: number; top: number }[] = [];
const side = 780;
let row = 0;
for (const view of views) {
  let column = 0;
  for (const viewpoint of ["elevation", "front", "plan"]) {
    const text = readFileSync(png.replace(/\.png$/u, `.${viewpoint}.${view}.svg`), "utf8");
    const [cx, cy] = centres[viewpoint];
    const svg = text.replace(/width="[0-9.]+mm" height="[0-9.]+mm" viewBox="[^"]+"/u, `width="${side}" height="${side}" viewBox="${cx - size / 2} ${cy - size / 2} ${size} ${size}"`);
    tiles.push({ input: await sharp(Buffer.from(svg)).flatten({ background: "#ffffff" }).png().toBuffer(), left: column * (side + 10), top: row * (side + 10) });
    column += 1;
  }
  row += 1;
}
const target = join(out, `zoom-${basename(png)}`);
await sharp({ create: { width: 3 * side + 20, height: views.length * side + (views.length - 1) * 10, channels: 3, background: "#ffffff" } }).composite(tiles).png().toFile(target);
console.log(target);
