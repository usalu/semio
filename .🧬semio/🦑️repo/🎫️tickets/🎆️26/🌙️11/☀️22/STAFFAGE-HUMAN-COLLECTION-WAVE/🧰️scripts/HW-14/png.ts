/** 🖼️ Rasterises SVG files to PNG with sharp (librsvg) at the given density (default 300 dpi): bun png.ts [--dpi n] a.svg b.svg … */
import sharp from "/Users/ueli/Documents/semio/node_modules/sharp/lib/index.js";
const args = process.argv.slice(2);
let dpi = 300;
const files: string[] = [];
for (let i = 0; i < args.length; i++) args[i] === "--dpi" ? (dpi = Number(args[++i])) : files.push(args[i]!);
for (const f of files) {
  await sharp(f, { density: dpi }).flatten({ background: "#ffffff" }).png().toFile(f.replace(/\.svg$/, ".png"));
  console.log(f.replace(/\.svg$/, ".png"));
}
