/** 🖼️ Rasterises SVG files to PNG with sharp at a given density (default 600 dpi): bun png.ts [--dpi 600] <file.svg> ... */
import sharp from "../../../../../../../../../node_modules/sharp/lib/index.js";
const args = process.argv.slice(2);
let dpi = 600;
let trim = false;
if (args[0] === "--dpi") { dpi = Number(args[1]); args.splice(0, 2); }
if (args[0] === "--trim") { trim = true; args.splice(0, 1); }
for (const file of args) {
  const out = file.replace(/\.svg$/, ".png");
  const img = sharp(file, { density: dpi, limitInputPixels: false }).flatten({ background: "#ffffff" });
  if (trim) { const buf = await img.png().toBuffer(); await sharp(buf, { limitInputPixels: false }).trim({ threshold: 40 }).resize({ width: 2400, height: 2400, fit: "inside", withoutEnlargement: true }).png().toFile(out); } else await img.png().toFile(out);
  console.log(out);
}
