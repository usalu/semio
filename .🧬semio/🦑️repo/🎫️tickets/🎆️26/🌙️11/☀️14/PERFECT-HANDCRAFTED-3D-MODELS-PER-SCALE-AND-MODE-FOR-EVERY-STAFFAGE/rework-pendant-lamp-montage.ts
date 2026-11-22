/** 🧩 Joins crops of rendered sheets into one image for reading: `bun rework-pendant-lamp-montage.ts <out.png> <cols> <x,y,w,h> <png>...` (a png argument may carry its own crop as `path@x,y,w,h`). */
import sharp from "sharp";
const [out, colsArg, cropArg, ...files] = process.argv.slice(2);
const cols = Number(colsArg);
const base = cropArg.split(",").map(Number);
const cells = await Promise.all(files.map(async (entry) => {
  const [path, own] = entry.split("@");
  const [left, top, width, height] = own ? own.split(",").map(Number) : base;
  return sharp(path).extract({ left, top, width, height }).resize(base[2], base[3], { fit: "contain", background: "#ffffff" }).png().toBuffer();
}));
const rows = Math.ceil(cells.length / cols);
await sharp({ create: { width: cols * base[2], height: rows * base[3], channels: 3, background: "#ffffff" } })
  .composite(cells.map((input, index) => ({ input, left: (index % cols) * base[2], top: Math.floor(index / cols) * base[3] })))
  .png().toFile(out);
console.log(out);
