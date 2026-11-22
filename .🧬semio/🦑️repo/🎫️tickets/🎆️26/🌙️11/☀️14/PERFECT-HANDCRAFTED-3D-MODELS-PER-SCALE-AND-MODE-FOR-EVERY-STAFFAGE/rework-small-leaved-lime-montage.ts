/** 🖼️ Small-leaved lime author: lays sheets side by side into one picture (`out cols tileWidth image...`) so a set of poses or lists is read at one glance. */
import sharp from "sharp";

const [out, colsArg, widthArg, ...files] = process.argv.slice(2);
const cols = Number(colsArg);
const width = Number(widthArg);
const tiles = await Promise.all(files.map((file) => sharp(file!).resize({ width }).png().toBuffer({ resolveWithObject: true })));
const height = Math.max(...tiles.map((tile) => tile.info.height));
const rows = Math.ceil(tiles.length / cols);
await sharp({ create: { width: cols * width, height: rows * height, channels: 3, background: "#ffffff" } })
  .composite(tiles.map((tile, index) => ({ input: tile.data, left: (index % cols) * width, top: Math.floor(index / cols) * height })))
  .png()
  .toFile(out!);
console.log(out);
