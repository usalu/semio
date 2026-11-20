/** 🧩 Joins PNGs into a grid (cols given) at half size each for comparing scales of `objects/street-lamp` side by side. Usage: `bun r3-street-lamp-montage.ts <out.png> <cols> <in.png>...` */
import sharp from "sharp";
const [out, colsArg, ...ins] = process.argv.slice(2);
const cols = Number(colsArg);
const metas = await Promise.all(ins.map((p) => sharp(p).metadata()));
const w = Math.round(metas[0]!.width! / 2), h = Math.round(metas[0]!.height! / 2);
const parts = await Promise.all(ins.map(async (p, i) => ({ input: await sharp(p).resize(w, h, { fit: "contain", background: "#ffffff" }).png().toBuffer(), left: (i % cols) * w, top: Math.floor(i / cols) * h })));
await sharp({ create: { width: w * cols, height: h * Math.ceil(ins.length / cols), channels: 3, background: "#ffffff" } }).composite(parts).png().toFile(out!);
console.log(out);
