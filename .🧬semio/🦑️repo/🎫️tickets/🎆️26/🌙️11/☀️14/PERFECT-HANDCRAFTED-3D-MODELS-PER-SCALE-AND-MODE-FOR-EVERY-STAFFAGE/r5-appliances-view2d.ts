/** 🔭️ Round 5 appliances: enlarges the product's own 2D drawings of one asset and scale (the SVG panels kept by `r3-2d.ts --svg`) so that a drawing of a few millimetres on paper can be judged: annotations, ground and paper are hidden, each view is rasterised at `--px` pixels per paper millimetre, trimmed, and the three views (light-line above, light-solid below) are set side by side.
 *
 * Usage: `bun r3-2d.ts <category>/<slug> --scale N --views light-line,light-solid --svg` first, then `bun r5-appliances-view2d.ts <category>/<slug> --scale N [--px 40]`
 * Output: `🗑️generated/r5-<slug>/view2d.s<N>.png`
 * @see ./r3-2d.ts */
import { readFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";
import sharp from "sharp";
import { GENERATED } from "./volumes-lib.ts";

const [asset, ...rest] = process.argv.slice(2);
const arg = (name: string) => { const i = rest.indexOf(name); return i >= 0 ? rest[i + 1] : undefined; };
const scale = Number(arg("--scale"));
const px = Number(arg("--px") ?? 40);
const [category, slug] = asset!.split("/");
const hide = `<style>#staffage-annotations,#staffage-ground,rect[id^="staffage-"]{display:none}</style>`;
const parts: { input: Buffer; left: number; top: number }[] = [];
let top = 0;
let width = 0;
for (const view of ["light-line", "light-solid"]) {
  let x = 0;
  let rowHeight = 0;
  for (const viewpoint of ["elevation", "front", "plan"]) {
    const text = readFileSync(join(GENERATED, "r3-2d", category!, `${slug}.s${scale}.${viewpoint}.${view}.svg`), "utf8");
    const svg = Buffer.from(text.replace(/width="([\d.]+)mm" height="([\d.]+)mm"/, (_, w, h) => `width="${Number(w) * px}px" height="${Number(h) * px}px"`).replace(/(<svg[^>]*>)/, `$1${hide}`));
    const raster = await sharp(svg, { density: 72, limitInputPixels: false }).flatten({ background: "#ffffff" }).png().toBuffer();
    const crop = await sharp(raster).trim({ threshold: 10 }).extend({ top: 20, bottom: 20, left: 20, right: 20, background: "#ffffff" }).png().toBuffer();
    const m = await sharp(crop).metadata();
    parts.push({ input: crop, left: x, top });
    x += m.width! + 40;
    rowHeight = Math.max(rowHeight, m.height!);
  }
  width = Math.max(width, x);
  top += rowHeight + 40;
}
mkdirSync(join(GENERATED, `r5-${slug}`), { recursive: true });
const path = join(GENERATED, `r5-${slug}`, `view2d.s${scale}.png`);
await sharp({ create: { width, height: top, channels: 3, background: "#ffffff" } }).composite(parts).png().toFile(path);
process.stdout.write(`${path}\n`);
