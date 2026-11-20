/** 📚️ Round 3 mallard: stacks PNGs of equal width vertically into one image. Usage: bun r3-mallard-duck-stack.ts <out.png> <in.png>... */
import sharp from "sharp";

const [out, ...files] = process.argv.slice(2);
const metas = await Promise.all(files.map((f) => sharp(f).metadata()));
let top = 0;
const tiles = files.map((f, i) => { const t = { input: f, left: 0, top }; top += metas[i]!.height!; return t; });
await sharp({ create: { width: Math.max(...metas.map((m) => m.width!)), height: top, channels: 3, background: "#ffffff" } }).composite(tiles).png().toFile(out!);
console.log(out);
