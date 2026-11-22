/** 🖼️ Rasterises SVG files to PNG next to them with sharp (librsvg), for reading previews without rsvg-convert.
 *  bun png.ts <file.svg|dir> ... [--density 150] */
import sharp from "/Users/ueli/Documents/semio/node_modules/sharp/lib/index.js";
import { readdirSync, statSync } from "node:fs";
import { join } from "node:path";
const args = process.argv.slice(2);
const di = args.indexOf("--density");
const density = di >= 0 ? Number(args[di + 1]) : 150;
const files = args.filter((a, i) => !a.startsWith("--") && (di < 0 || i !== di + 1)).flatMap((p) => statSync(p).isDirectory() ? readdirSync(p).filter((f) => f.endsWith(".svg")).map((f) => join(p, f)) : [p]);
for (const f of files) {
  await sharp(f, { density }).flatten({ background: "#ffffff" }).png().toFile(f.replace(/\.svg$/, ".png"));
  console.log(f.replace(/\.svg$/, ".png"));
}
