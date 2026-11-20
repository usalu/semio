/** 🧩️ Round 3 mallard: tiles PNGs into one image (two columns, each tile at half size) so that several poses can be judged at once. Usage: bun r3-mallard-duck-tile.ts <out.png> <in.png>... */
import sharp from "sharp";

const [out, ...files] = process.argv.slice(2);
const metas = await Promise.all(files.map((f) => sharp(f).metadata()));
const [w, h] = [Math.round(metas[0]!.width! / 2), Math.round(metas[0]!.height! / 2)];
const tiles = await Promise.all(files.map(async (f, i) => ({ input: await sharp(f).resize(w, h, { fit: "contain", background: "#ffffff" }).toBuffer(), left: (i % 2) * w, top: Math.floor(i / 2) * h })));
await sharp({ create: { width: 2 * w, height: Math.ceil(files.length / 2) * h, channels: 3, background: "#ffffff" } }).composite(tiles).png().toFile(out!);
console.log(out);
