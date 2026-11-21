/** 🧷️ Round 5, objects: lays PNG sheets out two per row at half size so that poses, scales or lists can be judged side by side.
 * Usage: `bun r5-objects-montage.ts <out.png> <a.png> [b.png ...]`
 * @see ./r5-brief.md */
import sharp from "sharp";

const [out, ...inputs] = process.argv.slice(2);
const tiles = await Promise.all(inputs.map((p) => sharp(p!).resize({ width: 1200 }).png().toBuffer({ resolveWithObject: true })));
const w = 1200;
const h = Math.max(...tiles.map((t) => t.info.height));
await sharp({ create: { width: 2 * w, height: Math.ceil(tiles.length / 2) * h, channels: 3, background: { r: 255, g: 255, b: 255 } } })
  .composite(tiles.map((t, i) => ({ input: t.data, left: (i % 2) * w, top: Math.floor(i / 2) * h })))
  .png()
  .toFile(out!);
console.log(out);
