/** 🔬 Rasterises a window of a product SVG (kept by `r3-2d.ts --svg`) at high density for a close look at the luminaire or the base of the 6 m street lamp.
 *
 * Usage: `bun r3-street-lamp-svgzoom.ts <in.svg> <out.png> <left share> <top share> <width share> <height share>`
 * @see ./r3-2d.ts */
import sharp from "sharp";
const [src, out, l, t, w, h] = process.argv.slice(2);
const big = await sharp(src!, { density: Number(process.env.DPI ?? 300), limitInputPixels: false }).flatten({ background: "#ffffff" }).png().toBuffer();
const m = await sharp(big).metadata();
await sharp(big).extract({ left: Math.round(m.width! * Number(l)), top: Math.round(m.height! * Number(t)), width: Math.round(m.width! * Number(w)), height: Math.round(m.height! * Number(h)) }).resize({ width: 1600, withoutEnlargement: true }).png().toFile(out!);
console.log(out, m.width, m.height);
