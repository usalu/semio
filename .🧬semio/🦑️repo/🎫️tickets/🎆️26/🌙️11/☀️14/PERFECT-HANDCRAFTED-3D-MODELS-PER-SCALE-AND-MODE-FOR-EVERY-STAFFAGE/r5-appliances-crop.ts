/** ✂️ Round 5 appliances (E04): crops one region (left top width height, in source pixels) out of each given image, scales it to the given tile height and lays the tiles side by side, so a small part of a tall sheet (the shade of a pendant lamp, the nozzle of a vacuum cleaner) can be judged at full size.
 * Usage: bun r5-appliances-crop.ts <out.png> <tileHeight> <left,top,width,height> <image> [<left,top,width,height> <image>]...
 * @see ./r5-grid.ts */
import sharp from "sharp";

const [out, th, ...rest] = process.argv.slice(2) as string[];
const h = Number(th);
const tiles: { input: Buffer; w: number }[] = [];
for (let k = 0; k + 1 < rest.length; k += 2) {
  const [left, top, width, height] = rest[k]!.split(",").map(Number) as [number, number, number, number];
  const input = await sharp(rest[k + 1]!).extract({ left, top, width, height }).resize({ height: h }).png().toBuffer();
  tiles.push({ input, w: Math.round((width * h) / height) });
}
const W = tiles.reduce((s, t) => s + t.w + 8, -8);
let x = 0;
await sharp({ create: { width: W, height: h, channels: 3, background: "#ffffff" } })
  .composite(tiles.map((t) => { const o = { input: t.input, left: x, top: 0 }; x += t.w + 8; return o; }))
  .png()
  .toFile(out);
console.log(out);
