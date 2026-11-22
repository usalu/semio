/** 🖼️ Rasterises preview SVGs to PNG for reading (PW-09): `repo tools exec bun -- raster.ts <dpi> <svg>...`; sharp (librsvg) stands in for the missing system `rsvg-convert`. */
import sharp from "sharp";
const [dpi, ...files] = process.argv.slice(2);
for (const file of files) {
  await sharp(file, { density: Number(dpi), limitInputPixels: false }).flatten({ background: "#ffffff" }).png().toFile(file.replace(/\.svg$/, ".png"));
  console.log(`[DEBUG] ${file.replace(/\.svg$/, ".png")}`);
}
