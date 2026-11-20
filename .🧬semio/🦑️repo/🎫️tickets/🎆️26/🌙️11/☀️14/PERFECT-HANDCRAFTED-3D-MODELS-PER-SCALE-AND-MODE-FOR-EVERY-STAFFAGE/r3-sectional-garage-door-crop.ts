/** 🔎️ Crops a region of a rendered PNG or SVG and enlarges it, so a detail (panel joint, hinge, roller, drum) can be judged: `bun r3-sectional-garage-door-crop.ts <png|svg> <left> <top> <width> <height> [factor] [density]`; writes `<name>.crop-<left>-<top>.png` under 🗑️generated/r3-sectional-garage-door. */
import sharp from "sharp";
import { basename, join } from "node:path";

const [file, l, t, w, h, f, density] = process.argv.slice(2);
const factor = Number(f ?? 3);
const out = join(import.meta.dir, "🗑️generated", "r3-sectional-garage-door", `${basename(file!).replace(/\.(png|svg)$/, "")}.crop-${l}-${t}.png`);
const source = file!.endsWith(".svg") ? sharp(file!, { density: Number(density ?? 300) }).flatten({ background: "#f5f1e3" }) : sharp(file!);
const meta = await source.metadata();
console.log(`source ${meta.width} x ${meta.height}`);
await source.extract({ left: Number(l), top: Number(t), width: Number(w), height: Number(h) }).resize(Math.round(Number(w) * factor), Math.round(Number(h) * factor), { kernel: file!.endsWith(".svg") ? "lanczos3" : "nearest" }).png().toFile(out);
console.log(out);
