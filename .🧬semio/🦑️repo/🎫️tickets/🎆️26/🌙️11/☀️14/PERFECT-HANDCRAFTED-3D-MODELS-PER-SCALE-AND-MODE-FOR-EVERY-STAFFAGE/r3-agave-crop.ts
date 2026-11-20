/** 🔍️ Round 3, vegetation/agave: cuts a rectangle out of a rendered PNG and enlarges it for looking at details. Usage: bun r3-agave-crop.ts <in.png> <out.png> <left> <top> <width> <height> [zoom] */
import sharp from "sharp";
const [input, output, l, t, w, h, z] = process.argv.slice(2);
await sharp(input!).extract({ left: Number(l), top: Number(t), width: Number(w), height: Number(h) }).resize({ width: Math.round(Number(w) * Number(z ?? 2)), kernel: "lanczos3" }).toFile(output!);
console.log(output);
