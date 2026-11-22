/** 🔎️ Crops a region of a PNG given as fractions of its size and enlarges it: bun crop.ts <in.png> <out.png> <x0> <y0> <x1> <y1> [width=1200] */
import sharp from "../../../../../../../../../node_modules/sharp/lib/index.js";

const [input, output, x0, y0, x1, y1, width] = process.argv.slice(2);
const meta = await sharp(input!).metadata();
const left = Math.round(Number(x0) * meta.width!), top = Math.round(Number(y0) * meta.height!);
await sharp(input!).extract({ left, top, width: Math.round(Number(x1) * meta.width!) - left, height: Math.round(Number(y1) * meta.height!) - top }).resize({ width: Number(width ?? 1200), kernel: "lanczos3" }).png().toFile(output!);
