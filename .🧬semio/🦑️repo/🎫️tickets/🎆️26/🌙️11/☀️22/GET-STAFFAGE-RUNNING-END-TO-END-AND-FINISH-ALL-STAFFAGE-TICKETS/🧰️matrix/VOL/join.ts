/** 🧷️ Joins sheets one above the other at 1300 px width: join.ts <out.png> <sheet.png>… */
import sharp from "sharp";
const [out, ...parts] = process.argv.slice(2) as [string, ...string[]];
const W = 1300;
const images = await Promise.all(parts.map((p) => sharp(p).resize({ width: W }).png().toBuffer({ resolveWithObject: true })));
let y = 0;
const layers = images.map(({ data, info }) => { const top = y; y += info.height; return { input: data, left: 0, top }; });
await sharp({ create: { width: W, height: y, channels: 3, background: "white" } }).composite(layers).png().toFile(out);
