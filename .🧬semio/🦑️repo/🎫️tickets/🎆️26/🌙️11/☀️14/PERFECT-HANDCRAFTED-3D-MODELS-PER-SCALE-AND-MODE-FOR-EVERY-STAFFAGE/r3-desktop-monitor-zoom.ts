/** 🔍️ Rasterises one SVG panel of `r3-2d.ts --svg` cropped to a paper-millimetre window at a large size, so the strokes of a small drawing can be judged.
 *
 * Usage: bun r3-desktop-monitor-zoom.ts <svg> <x0> <y0> <w> <h> <out.png>
 * @see ./r3-2d.ts */
import { readFileSync } from "node:fs";
import sharp from "sharp";

const [path, x0, y0, w, h, out] = process.argv.slice(2);
const svg = readFileSync(path!, "utf8").replace(/width="[^"]*mm" height="[^"]*mm" viewBox="[^"]*"/u, `width="${w}mm" height="${h}mm" viewBox="${x0} ${y0} ${w} ${h}"`);
await sharp(Buffer.from(svg), { density: 2400, limitInputPixels: false }).resize(1800, 1800, { fit: "inside" }).flatten({ background: "#ffffff" }).png().toFile(out!);
console.log(out);
