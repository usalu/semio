/** ✂️ Crops a PNG to a box given in fractions of its size: bun crop.ts <in.png> <out.png> <x0> <y0> <x1> <y1> */
import sharp from "../../../../../../../../../node_modules/sharp/lib/index.js";
const [i, o, x0, y0, x1, y1] = process.argv.slice(2);
const m = await sharp(i).metadata();
const [W, H] = [m.width!, m.height!];
await sharp(i).extract({ left: Math.round(+x0! * W), top: Math.round(+y0! * H), width: Math.round((+x1! - +x0!) * W), height: Math.round((+y1! - +y0!) * H) }).toFile(o);
