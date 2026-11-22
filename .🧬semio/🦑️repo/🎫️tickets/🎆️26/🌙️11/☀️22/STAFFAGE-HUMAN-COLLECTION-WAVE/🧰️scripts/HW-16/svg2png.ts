/** 🖼️ Rasterises SVG files to PNG with sharp (librsvg) at a given density: bun svg2png.ts <dpi> <file.svg>... */
import sharp from "../../../../../../../../../node_modules/sharp/lib/index.js";
const [dpi, ...files] = process.argv.slice(2);
for (const f of files) {
  await sharp(f, { density: Number(dpi) }).flatten({ background: "#ffffff" }).png().toFile(f.replace(/\.svg$/, ".png"));
  console.log(f.replace(/\.svg$/, ".png"));
}
