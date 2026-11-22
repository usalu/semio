/** 🖼️ Finisher A contact sheet: lays PNG files out in a grid of two columns and writes one PNG of the given width.
 * Usage: `bun compose.ts <out.png> <width px> <in.png>...`
 * @see ./look.sh */
import sharp from "../../../../../../../../../node_modules/sharp/lib/index.js";

const [out, widthText, ...files] = process.argv.slice(2);
const cell = Math.round(Number(widthText) / 2);
const tiles = await Promise.all(files.map(async (file) => {
  const data = await sharp(file).resize({ width: cell }).png().toBuffer();
  return { data, height: (await sharp(data).metadata()).height! };
}));
const rowHeight = Math.max(...tiles.map((tile) => tile.height));
const rows = Math.ceil(tiles.length / 2);
await sharp({ create: { width: cell * 2, height: rowHeight * rows, channels: 3, background: "#ffffff" } })
  .composite(tiles.map((tile, index) => ({ input: tile.data, left: (index % 2) * cell, top: Math.floor(index / 2) * rowHeight })))
  .png()
  .toFile(out!);
console.log(out);
