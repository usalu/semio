/** 🔎 Zooms the six SVG panels that `r3-2d.ts --svg` kept for one scale onto the drawn object (bounding box of every shape and outline group, ground and annotations left out) and composes them into one 2400 x 1600 PNG, so the 2D line work can be judged stroke by stroke.
 *
 * Usage: `bun r3-toaster-zoom.ts <svg folder> <stem> <out.png>` (stem e.g. `r3-toaster.s50`).
 * @see ./r3-2d.ts */
import { readFileSync } from "node:fs";
import sharp from "sharp";

const [dir, stem, out] = process.argv.slice(2);
const panels: Buffer[] = [];
for (const view of ["light-line", "light-solid"]) {
  for (const viewpoint of ["elevation", "front", "plan"]) {
    const svg = readFileSync(`${dir}/${stem}.${viewpoint}.${view}.svg`, "utf8");
    const xs: number[] = [];
    const ys: number[] = [];
    for (const g of svg.matchAll(/<g id="(?:shape|outline)-[^"]*"[^>]*>(.*?)<\/g>/gsu)) {
      for (const d of g[1].matchAll(/ d="([^"]*)"/gu)) {
        const nums = [...d[1].matchAll(/-?\d+(?:\.\d+)?/gu)].map((m) => Number(m[0]));
        for (let k = 0; k + 1 < nums.length; k += 2) {
          xs.push(nums[k]);
          ys.push(nums[k + 1]);
        }
      }
    }
    const [x0, x1, y0, y1] = [Math.min(...xs), Math.max(...xs), Math.min(...ys), Math.max(...ys)];
    const m = 0.08 * Math.max(x1 - x0, y1 - y0);
    const vb = `${x0 - m} ${y0 - m} ${x1 - x0 + 2 * m} ${y1 - y0 + 2 * m}`;
    const zoomed = svg.replace(/width="[^"]*" height="[^"]*" viewBox="[^"]*"/u, `width="${(x1 - x0 + 2 * m) * 10}mm" height="${(y1 - y0 + 2 * m) * 10}mm" viewBox="${vb}"`);
    const png = await sharp(Buffer.from(zoomed), { density: 300, limitInputPixels: false }).resize(780, 760, { fit: "contain", background: "#ffffff" }).flatten({ background: "#ffffff" }).png().toBuffer();
    panels.push(png);
  }
}
const base = sharp({ create: { width: 2400, height: 1560, channels: 3, background: "#ffffff" } });
await base.composite(panels.map((input, k) => ({ input, left: (k % 3) * 800 + 10, top: Math.floor(k / 3) * 780 + 10 }))).png().toFile(out);
console.log(out);
