/** 🧩 Crops the same region (left, top, width, height in sheet pixels) out of several `r3-sheet.ts --only` images and lays them side by side, so the shade, the rose or the lampholder of every scale and list can be judged at full resolution.
 *
 * Usage: `bun r3-pendant-lamp-crops.ts <out.png> <left> <top> <width> <height> <image>...`
 * @see ./r3-sheet.ts */
import sharp from "sharp";

const [out, l, t, w, h, ...images] = process.argv.slice(2);
const [left, top, width, height] = [l, t, w, h].map(Number);
const tiles = await Promise.all(images.map((p) => sharp(p).extract({ left, top, width, height }).png().toBuffer()));
await sharp({ create: { width: width * tiles.length + 10 * (tiles.length - 1), height, channels: 3, background: "#ffffff" } })
  .composite(tiles.map((input, k) => ({ input, left: k * (width + 10), top: 0 })))
  .png()
  .toFile(out);
console.log(out);
