/** 🗃️ Joins rendered sheets into one contact sheet so that a reviewer opens a few images per asset instead of dozens: every input PNG becomes a tile of the given width with its file name as caption, laid out in columns.
 *
 * `$BUN finish-contact.ts <out.png> <columns> <tile width px> <in.png> …`
 *
 * @see ./finish-sheets.py — renders the sheets and calls this */
import sharp from "sharp";
import { basename } from "node:path";

const [out, columnsText, widthText, ...files] = process.argv.slice(2);
const columns = Number(columnsText);
const width = Number(widthText);
const CAPTION = 22;
const tiles = await Promise.all(files.map(async (file) => {
  const image = sharp(file).resize({ width });
  const { data, info } = await image.png().toBuffer({ resolveWithObject: true });
  return { file, data, height: info.height };
}));
const rows: (typeof tiles)[] = [];
for (let index = 0; index < tiles.length; index += columns) rows.push(tiles.slice(index, index + columns));
const heights = rows.map((row) => Math.max(...row.map((tile) => tile.height)) + CAPTION);
const total = heights.reduce((sum, height) => sum + height, 0);
const layers: sharp.OverlayOptions[] = [];
let top = 0;
rows.forEach((row, r) => {
  row.forEach((tile, c) => {
    const caption = basename(tile.file).replace(/&/g, "&amp;").replace(/</g, "&lt;");
    layers.push({ input: Buffer.from(`<svg xmlns="http://www.w3.org/2000/svg" width="${width}" height="${CAPTION}"><rect width="100%" height="100%" fill="#222"/><text x="6" y="16" font-family="Helvetica" font-size="14" fill="#fff">${caption}</text></svg>`), left: c * width, top });
    layers.push({ input: tile.data, left: c * width, top: top + CAPTION });
  });
  top += heights[r];
});
await sharp({ create: { width: columns * width, height: total, channels: 3, background: "#ffffff" } }).composite(layers).png().toFile(out);
console.log(`wrote ${out}: ${tiles.length} tiles, ${columns * width} x ${total}`);
