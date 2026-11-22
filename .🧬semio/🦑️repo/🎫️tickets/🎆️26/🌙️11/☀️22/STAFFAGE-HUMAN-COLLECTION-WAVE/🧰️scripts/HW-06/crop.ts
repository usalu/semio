/** ✂️ Crops a PNG to a fractional window and scales it: bun crop.ts <in.png> <out.png> <x0> <y0> <x1> <y1> [width] */
import sharp from "/Users/ueli/Documents/semio/node_modules/sharp";

const [input, output, ...rest] = process.argv.slice(2);
const [x0, y0, x1, y1, width] = rest.map(Number);
const meta = await sharp(input!).metadata();
const left = Math.round(x0! * meta.width!), top = Math.round(y0! * meta.height!);
await sharp(input!).extract({ left, top, width: Math.round((x1! - x0!) * meta.width!), height: Math.round((y1! - y0!) * meta.height!) }).resize({ width: width || 1600 }).toFile(output!);
console.log(output);
