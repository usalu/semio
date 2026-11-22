/** 🔍️ Rasterises a window of an SVG (fractions of its width and height) at a density in dpi to a PNG with sharp.
 *  bun zoom.ts <in.svg> <out.png> <x0> <y0> <w> <h> [dpi] */
import sharp from "../../../../../../../../../node_modules/sharp/lib/index.js";

const [input, out, x0, y0, w, h, dpiArg] = process.argv.slice(2);
const img = sharp(input!, { density: Number(dpiArg ?? 400), limitInputPixels: false }).flatten({ background: "#ffffff" });
const { width, height } = await img.metadata();
const buf = await img.png().toBuffer();
await sharp(buf, { limitInputPixels: false }).extract({ left: Math.round(width! * Number(x0)), top: Math.round(height! * Number(y0)), width: Math.round(width! * Number(w)), height: Math.round(height! * Number(h)) }).png().toFile(out!);
console.log(out);
