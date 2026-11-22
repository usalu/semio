/** 🖼️ Rasterises SVG previews to PNG with sharp (librsvg), the oracle the silhouette tests use; the preview CLI's rsvg-convert is not on this host.
 *  bun png.ts <dpi> <file.svg ...> */
import sharp from "../../../../../../../../../node_modules/sharp/lib/index.js";
const [dpi, ...files] = process.argv.slice(2);
for (const f of files) {
  await sharp(f, { density: Number(dpi) }).flatten({ background: "#ffffff" }).png().toFile(f.replace(/\.svg$/, ".png"));
  console.log(f.replace(/\.svg$/, ".png"));
}
