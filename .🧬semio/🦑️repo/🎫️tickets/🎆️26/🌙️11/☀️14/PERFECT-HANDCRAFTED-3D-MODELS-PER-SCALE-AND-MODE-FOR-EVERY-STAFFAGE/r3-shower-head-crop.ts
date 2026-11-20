/** ✂️ Cuts boxes `left,top,width,height[,zoom]` out of one PNG and sets them side by side, to look closely at parts of a large render.
 *
 * Usage: `bun r3-shower-head-crop.ts <source.png> <out.png> <box> [<box> ...]` */
import sharp from "sharp";
const [src, out, ...boxes] = process.argv.slice(2);
const parts: { input: Buffer; left: number; top: number }[] = [];
let x = 0, h = 0;
for (const b of boxes) {
  const [l, t, w, hh, z] = b.split(",").map(Number);
  const buf = await sharp(src).extract({ left: l, top: t, width: w, height: hh }).resize({ width: Math.round(w * (z || 1)) }).png().toBuffer();
  const m = await sharp(buf).metadata();
  parts.push({ input: buf, left: x, top: 0 });
  x += m.width! + 20; h = Math.max(h, m.height!);
}
await sharp({ create: { width: x, height: h, channels: 3, background: "#ffffff" } }).composite(parts).png().toFile(out);
console.log(out);
