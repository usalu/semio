/** 🧩️ Puts PNGs side by side in a grid (sharp), each scaled to a cell width. bun montage.ts <out.png> <cellWidth> <cols> <png...> */
import sharp from "../../../../../../../../../node_modules/sharp/lib/index.js";
const [out, cw, cols, ...files] = process.argv.slice(2);
const w = Number(cw), c = Number(cols);
const imgs = await Promise.all(files.map(async (f) => { const b = await sharp(f).resize({ width: w }).png().toBuffer(); const m = await sharp(b).metadata(); return { b, h: m.height! }; }));
const rows = Math.ceil(imgs.length / c), rh = Math.max(...imgs.map((i) => i.h));
await sharp({ create: { width: w * c, height: rh * rows, channels: 3, background: "#ffffff" } }).composite(imgs.map((i, k) => ({ input: i.b, left: (k % c) * w, top: Math.floor(k / c) * rh }))).png().toFile(out!);
console.log(out);
