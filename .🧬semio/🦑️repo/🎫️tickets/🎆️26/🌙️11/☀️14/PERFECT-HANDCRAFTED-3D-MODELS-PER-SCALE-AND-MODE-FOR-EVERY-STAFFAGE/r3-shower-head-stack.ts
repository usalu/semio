/** 🥞 Stacks PNGs under each other at a common width of 2000 px, to look at several renders in one image.
 *
 * Usage: `bun r3-shower-head-stack.ts <out.png> <source.png> [<source.png> ...]` */
import sharp from "sharp";
const [out, ...srcs] = process.argv.slice(2);
const parts: { input: Buffer; left: number; top: number }[] = [];
let y = 0, w = 0;
for (const s of srcs) {
  const buf = await sharp(s).resize({ width: 2000 }).png().toBuffer();
  const m = await sharp(buf).metadata();
  parts.push({ input: buf, left: 0, top: y });
  y += m.height! + 6; w = Math.max(w, m.width!);
}
await sharp({ create: { width: w, height: y, channels: 3, background: "#ffffff" } }).composite(parts).png().toFile(out);
console.log(out);
