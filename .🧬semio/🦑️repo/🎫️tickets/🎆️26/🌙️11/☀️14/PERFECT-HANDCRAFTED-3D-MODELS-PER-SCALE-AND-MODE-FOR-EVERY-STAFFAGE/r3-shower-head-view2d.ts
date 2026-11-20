/** 🔬️ Enlarges the product's own 2D drawings of appliances/shower-head (the SVG panels that `r3-2d.ts --svg` keeps): the three views of one scale side by side, each cropped to its drawn part (the upper left of the sheet) and enlarged before it is rasterised, so that shapes of a few millimetres on paper can be judged.
 *
 * Usage: `bun r3-2d.ts appliances/shower-head --scale N --views light-solid --svg` first, then `bun r3-shower-head-view2d.ts --scale N [--view light-solid] [--zoom 1]`
 * Output: `🗑️generated/r3-shower-head/view2d.s<N>.png`
 * @see ./r3-2d.ts */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import sharp from "sharp";
import { GENERATED } from "./volumes-lib.ts";
import { cli } from "./r3-lib.ts";

const { flag } = cli(["--scale", "--view", "--zoom"]);
const scale = Number(flag("--scale"));
const zoom = Number(flag("--zoom") ?? 1);
const parts: { input: Buffer; left: number; top: number }[] = [];
let x = 0;
let height = 0;
for (const view of ["elevation", "front", "plan"]) {
  const svg = Buffer.from(readFileSync(join(GENERATED, "r3-2d", "appliances", `shower-head.s${scale}.${view}.${flag("--view") ?? "light-solid"}.svg`), "utf8").replace(/width="([\d.]+)mm" height="([\d.]+)mm"/, (_, w, h) => `width="${Number(w) * zoom}mm" height="${Number(h) * zoom}mm"`));
  const full = sharp(svg, { density: 96, limitInputPixels: false }).flatten({ background: "#ffffff" });
  const trimmed = await full.png().toBuffer();
  const meta = await sharp(trimmed).metadata();
  const crop = await sharp(trimmed).extract({ left: 0, top: 0, width: Math.round(meta.width! * (view === "plan" ? 0.3 : 0.62)), height: Math.round(meta.height! * (view === "plan" ? 0.45 : 0.52)) }).trim({ threshold: 40 }).png().toBuffer();
  const m = await sharp(crop).metadata();
  parts.push({ input: crop, left: x, top: 0 });
  x += m.width! + 40;
  height = Math.max(height, m.height!);
}
const path = join(GENERATED, "r3-shower-head", `view2d.s${scale}.png`);
await sharp({ create: { width: x, height, channels: 3, background: "#ffffff" } }).composite(parts).png().toFile(path);
process.stdout.write(`${path}\n`);
