/** 🧩️ Tiles PNG files into one picture with a caption strip per tile (names in order), to look at several rasters at once: bun grid.ts <out.png> <columns> <tile px> <png>… */
import sharp from "sharp";
const [out, cols, size, ...files] = process.argv.slice(2);
const n = Number(cols), s = Number(size);
const tiles = await Promise.all(files.map(async (f, i) => ({ input: await sharp(f).resize(s, s, { fit: "contain", background: "#ffffff", kernel: "nearest" }).extend({ top: 0, bottom: 4, left: 0, right: 4, background: "#00aa00" }).png().toBuffer(), left: (i % n) * (s + 4), top: Math.floor(i / n) * (s + 4) })));
await sharp({ create: { width: n * (s + 4), height: Math.ceil(files.length / n) * (s + 4), channels: 3, background: "#ffffff" } }).composite(tiles).png().toFile(out!);
console.log(out);
