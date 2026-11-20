/** ✂️ Crops a share of a PNG (left, top, width, height as shares of the image) for a close look. Usage: `bun r3-bamboo-clump-crop.ts <in.png> <out.png> l t w h` */
import sharp from "sharp";
const [src, out, l, t, w, h] = process.argv.slice(2);
const m = await sharp(src).metadata();
await sharp(src).extract({ left: Math.round(m.width! * Number(l)), top: Math.round(m.height! * Number(t)), width: Math.round(m.width! * Number(w)), height: Math.round(m.height! * Number(h)) }).toFile(out);
console.log(m.width, m.height);
