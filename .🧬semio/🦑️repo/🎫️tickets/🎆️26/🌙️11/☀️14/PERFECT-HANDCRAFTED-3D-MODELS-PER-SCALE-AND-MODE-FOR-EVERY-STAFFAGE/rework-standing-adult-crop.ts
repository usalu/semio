/** 🔍 Cuts regions out of rendered sheets, enlarges them and sets them side by side: `bun rework-standing-adult-crop.ts <out.png> <factor> <png>:<left>,<top>,<width>,<height> …`. */
import sharp from "sharp";
const [out, factorText, ...entries] = process.argv.slice(2);
const factor = Number(factorText);
const tiles: { input: Buffer; left: number; top: number }[] = [];
let x = 0, tall = 0;
for (const entry of entries) {
  const cut = entry.lastIndexOf(":");
  const [left, top, width, height] = entry.slice(cut + 1).split(",").map(Number) as [number, number, number, number];
  const input = await sharp(entry.slice(0, cut)).extract({ left, top, width, height }).resize(Math.round(width * factor), Math.round(height * factor), { kernel: "lanczos3" }).png().toBuffer();
  tiles.push({ input, left: x, top: 0 });
  x += Math.round(width * factor);
  tall = Math.max(tall, Math.round(height * factor));
}
await sharp({ create: { width: x, height: tall, channels: 3, background: "#ffffff" } }).composite(tiles).png().toFile(out!);
console.log(out);
