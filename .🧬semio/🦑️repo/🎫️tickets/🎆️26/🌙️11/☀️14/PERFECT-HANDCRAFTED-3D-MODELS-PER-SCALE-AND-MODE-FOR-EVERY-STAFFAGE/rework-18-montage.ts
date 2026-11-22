/** 🧩 Tiles PNG renders into one 2-column picture so four scales or lists are read side by side: `bun rework-18-montage.ts <out.png> <png>...`; optional env CROP="l,t,w,h" crops every tile first. */
import sharp from "sharp";

const [out, ...paths] = process.argv.slice(2);
const crop = process.env.CROP?.split(",").map(Number);
const w = 1200;
const h = 800;
const columns = paths.length > 1 ? 2 : 1;
const rows = Math.ceil(paths.length / columns);
const tiles = await Promise.all(paths.map(async (path, index) => {
  let image = sharp(path);
  if (crop) image = image.extract({ left: crop[0]!, top: crop[1]!, width: crop[2]!, height: crop[3]! });
  return { input: await image.resize(w, h, { fit: "contain", background: "#ffffff", kernel: "lanczos3" }).png().toBuffer(), left: (index % columns) * w, top: Math.floor(index / columns) * h };
}));
await sharp({ create: { width: w * columns, height: h * rows, channels: 3, background: "#ffffff" } }).composite(tiles).png().toFile(out!);
console.log(out);
