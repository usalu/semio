/** 🖼️ Composes rendered sheets into one contact sheet: a grid of equally sized tiles, each with its file name as caption, so a whole asset can be looked at in a few images.
 *
 * `$BUN contact.ts <output png> <columns> <tile width> <tile height> <png> [<png> …]`
 *
 * @see ./sheets.sh — the renders this script arranges */
import { basename } from "node:path";
import sharp from "sharp";

const [output = "", columnsText = "4", widthText = "480", heightText = "360", ...files] = process.argv.slice(2);
const [columns, width, height] = [Number(columnsText), Number(widthText), Number(heightText)];
const rows = Math.ceil(files.length / columns);
const caption = 16;
const parts: { input: Buffer; left: number; top: number }[] = [];
for (const [index, file] of files.entries()) {
  const left = (index % columns) * width;
  const top = Math.floor(index / columns) * (height + caption);
  const label = basename(file).replace(/\.png$/, "").replace(/&/g, "+").replace(/</g, "(");
  parts.push({ input: Buffer.from(`<svg xmlns="http://www.w3.org/2000/svg" width="${width}" height="${caption}"><rect width="100%" height="100%" fill="#222"/><text x="4" y="12" font-family="Helvetica" font-size="11" fill="#fff">${label}</text></svg>`), left, top });
  parts.push({ input: await sharp(file).resize(width, height, { fit: "contain", background: "#ffffff" }).flatten({ background: "#ffffff" }).png().toBuffer(), left, top: top + caption });
}
await sharp({ create: { width: columns * width, height: rows * (height + caption), channels: 3, background: "#ffffff" } }).composite(parts).png().toFile(output);
console.log(`${output}: ${files.length} tiles`);
