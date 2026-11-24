/** 🪞️ Before and after of the product sheets of a person at one scale as one picture: the figures of the line row of the backup sheet, of the catalog sheet, then the same of the solid row, cropped from the sheets `look.sh` rendered. Run from the 3D ticket folder (it resolves sharp there): bun compare.ts <slug> <scale>; prints the PNG in 🗑️generated/silhouette/SH-2/compare. */
import { mkdirSync } from "node:fs";
import sharp from "sharp";

const [slug, scale] = process.argv.slice(2) as [string, string];
const gen = `${import.meta.dir}/../../🗑️generated/silhouette/SH-2`;
const bands: Buffer[] = [];
let width = 0, height = 0;
for (const row of [0, 1]) for (const tag of ["before", "after"]) {
  const file = `${gen}/sheets/${slug}.s${scale}.${tag}.png`;
  const meta = await sharp(file).metadata();
  width = meta.width!; height = Math.round(meta.height! * 0.29);
  bands.push(await sharp(file).extract({ left: 0, top: Math.round(meta.height! * (row * 0.488 + 0.02)), width, height }).toBuffer());
}
mkdirSync(`${gen}/compare`, { recursive: true });
const out = `${gen}/compare/${slug}.s${scale}.png`;
await sharp({ create: { width, height: height * 4, channels: 3, background: "white" } }).composite(bands.map((input, i) => ({ input, left: 0, top: i * height }))).png().toFile(out);
console.log(out);
