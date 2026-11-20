/** 🔎 Crops a region of a rendered PNG and enlarges it so the clamp, the bolts and the handrail can be judged: `bun r3-glass-balustrade-crop.ts <png> <left> <top> <width> <height> [factor] [name]`; writes under 🗑️generated/r3-glass-balustrade. */
import sharp from "sharp";
import { mkdirSync } from "node:fs";
import { basename, join } from "node:path";

const [file, l, t, w, h, f, name] = process.argv.slice(2);
const factor = Number(f ?? 3);
const folder = join(import.meta.dir, "🗑️generated", "r3-glass-balustrade");
mkdirSync(folder, { recursive: true });
const out = join(folder, `${basename(file!, ".png")}.${name ?? `crop-${l}-${t}`}.png`);
await sharp(file!).extract({ left: Number(l), top: Number(t), width: Number(w), height: Number(h) }).resize(Math.round(Number(w) * factor), Math.round(Number(h) * factor), { kernel: "lanczos3" }).png().toFile(out);
console.log(out);
