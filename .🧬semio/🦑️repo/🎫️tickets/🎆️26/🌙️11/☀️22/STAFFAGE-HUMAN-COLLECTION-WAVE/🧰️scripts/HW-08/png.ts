/** 🖼️ Rasterises SVG files to PNG with sharp (librsvg inside), since `rsvg-convert` is not on this machine's PATH.
 *  bun png.ts <file.svg ...> [--dpi 150] */
import sharp from "../../../../../../../../../node_modules/sharp/lib/index.js";
const args = process.argv.slice(2);
const at = args.indexOf("--dpi");
const dpi = at >= 0 ? Number(args[at + 1]) : 150;
for (const file of args.filter((a, i) => a.endsWith(".svg") && args[i - 1] !== "--dpi")) {
  await sharp(file, { density: dpi }).flatten({ background: "#ffffff" }).png().toFile(file.replace(/\.svg$/u, ".png"));
  console.log(file.replace(/\.svg$/u, ".png"));
}
