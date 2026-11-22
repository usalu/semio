/** 🖼️ Rasterises preview SVGs to PNG with sharp (rsvg-convert is not installed on this machine).
 *  bun png.ts <density> file.svg ... */
import sharp from "../../../../../../../../../node_modules/sharp/lib/index.js";

const [density, ...files] = process.argv.slice(2);
for (const file of files) await sharp(file, { density: Number(density) }).flatten({ background: "#ffffff" }).png().toFile(file.replace(/\.svg$/u, ".png"));
