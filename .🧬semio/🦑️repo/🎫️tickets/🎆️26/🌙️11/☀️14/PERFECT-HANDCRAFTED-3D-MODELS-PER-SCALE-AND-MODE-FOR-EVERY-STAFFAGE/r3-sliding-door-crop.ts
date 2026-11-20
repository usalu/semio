/** 🔍️ Crops a region of a rendered PNG and enlarges it, so a detail (hanger, handle, brush) can be judged: `bun r3-sliding-door-crop.ts <png> <left> <top> <width> <height> [factor]`; writes `<png>.crop-<left>-<top>.png` under 🗑️generated/r3-sliding-door. */
import sharp from "sharp";
import { basename, join } from "node:path";

const [file, l, t, w, h, f] = process.argv.slice(2);
const factor = Number(f ?? 3);
const out = join(import.meta.dir, "🗑️generated", "r3-sliding-door", `${basename(file!, ".png")}.crop-${l}-${t}.png`);
await sharp(file!).extract({ left: Number(l), top: Number(t), width: Number(w), height: Number(h) }).resize(Math.round(Number(w) * factor), Math.round(Number(h) * factor), { kernel: "nearest" }).png().toFile(out);
console.log(out);
