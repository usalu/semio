/** 🖼️ PW-16: rasterise an SVG to PNG with sharp (librsvg) for reading previews. `repo tools exec bun -- svg2png.ts in.svg out.png [width]` */
import sharp from "/Users/ueli/Documents/semio/node_modules/sharp/lib/index.js";
const [input, output, width] = process.argv.slice(2) as [string, string, string | undefined];
await sharp(input, { density: 150 }).resize({ width: Number(width ?? 1600) }).flatten({ background: "#ffffff" }).png().toFile(output);
