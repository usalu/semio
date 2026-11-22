/** 🖼️ Rasterises SVG previews to PNG with sharp (rsvg-convert is not installed on this host).
 *  bun raster.ts <dpi> <file.svg ...> */
import sharp from "/Users/ueli/Documents/semio/node_modules/sharp";

const [dpi, ...files] = process.argv.slice(2);
for (const file of files) {
  const out = file.replace(/\.svg$/, ".png");
  await sharp(file, { density: Number(dpi) }).flatten({ background: "#ffffff" }).png().toFile(out);
  console.log(out);
}
