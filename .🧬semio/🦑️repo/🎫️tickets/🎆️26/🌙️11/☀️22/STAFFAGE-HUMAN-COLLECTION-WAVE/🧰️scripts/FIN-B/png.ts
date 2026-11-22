/** 🖼️ Rasterises preview SVGs to PNG with sharp; the density grows with the scale denominator in the file name so that every scale shows at a comparable pixel size, and the SVG is removed.
 *  bun png.ts <base density at 1:50> file.svg ... */
import { rmSync } from "node:fs";
import sharp from "../../../../../../../../../node_modules/sharp/lib/index.js";

const [density, ...files] = process.argv.slice(2);
for (const file of files) {
  const denominator = file.includes("--views-") ? Number(/1-(\d+)/u.exec(file)?.[1] ?? 50) : 50;
  await sharp(file, { density: Math.min(600, (Number(density) * denominator) / 50), limitInputPixels: false }).flatten({ background: "#ffffff" }).png().toFile(file.replace(/\.svg$/u, ".png"));
  rmSync(file);
}
