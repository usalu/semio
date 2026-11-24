/** ✂️ Crops the before and the after sheet of a person at one scale to the same fraction of the sheet and sets them side by side, enlarged, for looking at one view closely (sharp; a tool of the ticket).
 *  bun crop.ts <look dir> <slug> <scale> <left 0..1> <top 0..1> <width 0..1> <height 0..1> <out.png> */
import { readdirSync } from "node:fs";
import sharp from "/Users/ueli/Documents/semio/node_modules/sharp/lib/index.js";

const [dir, slug, scale, l, t, w, h, out] = process.argv.slice(2) as string[];
const parts: { input: Buffer; left: number; top: number }[] = [];
for (const [i, tag] of ["before", "after"].entries()) {
  const name = readdirSync(`${dir}/${slug}/${tag}`).find((n) => n.endsWith(`1-${scale}.png`))!;
  const image = sharp(`${dir}/${slug}/${tag}/${name}`);
  const meta = await image.metadata();
  const region = { left: Math.round(Number(l) * meta.width!), top: Math.round(Number(t) * meta.height!), width: Math.round(Number(w) * meta.width!), height: Math.round(Number(h) * meta.height!) };
  parts.push({ input: await image.extract(region).resize({ width: 700, height: 700, fit: "inside" }).flatten({ background: "#ffffff" }).png().toBuffer(), left: i * 720, top: 0 });
}
await sharp({ create: { width: 1440, height: 700, channels: 3, background: "#ffffff" } }).composite(parts).png().toFile(out!);
console.log(out);
