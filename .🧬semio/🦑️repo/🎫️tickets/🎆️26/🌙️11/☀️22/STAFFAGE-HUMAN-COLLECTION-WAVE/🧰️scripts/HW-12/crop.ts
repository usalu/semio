/** ✂️ Crops a PNG region (original pixels) and scales it up: bun crop.ts <in> <out> <x> <y> <w> <h> [width] */
import sharp from "/Users/ueli/Documents/semio/node_modules/sharp/lib/index.js";
const [i, o, x, y, w, h, W] = process.argv.slice(2);
await sharp(i).extract({ left: +x!, top: +y!, width: +w!, height: +h! }).resize({ width: +(W ?? 900), kernel: "nearest" }).toFile(o!);
