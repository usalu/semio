/** 🧩️ Places the sheet PNGs of a directory side by side in one image, coarsest scale last, each resized to the same height: bun montage.ts <dir> <out.png> [height-px]
 * @see ../HW-01/png.ts — the rasteriser of the sheets */
import { Glob } from "bun";
import { join } from "node:path";
import sharp from "../../../../../../../../../node_modules/sharp/lib/index.js";

const [dir, out, heightArg] = process.argv.slice(2);
const height = Number(heightArg ?? 1000);
const files = [...new Glob("*.png").scanSync(dir!)].sort((a, b) => Number(/1-(\d+)\.png$/.exec(a)?.[1] ?? 0) - Number(/1-(\d+)\.png$/.exec(b)?.[1] ?? 0));
const tiles = await Promise.all(files.map(async (f) => { const { data, info } = await sharp(join(dir!, f)).resize({ height }).png().toBuffer({ resolveWithObject: true }); return { data, width: info.width }; }));
let left = 0;
const composite = tiles.map((t) => { const at = { input: t.data, left, top: 0 }; left += t.width + 8; return at; });
await sharp({ create: { width: left, height, channels: 3, background: "#ffffff" } }).composite(composite).png().toFile(out!);
console.log(out);
