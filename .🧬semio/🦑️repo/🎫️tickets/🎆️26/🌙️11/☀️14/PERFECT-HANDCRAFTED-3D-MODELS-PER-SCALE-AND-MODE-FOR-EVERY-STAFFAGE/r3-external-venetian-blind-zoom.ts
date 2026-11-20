/** 🔎 Rasterises one SVG panel of r3-2d.ts at a high density and optionally crops it: `bun r3-external-venetian-blind-zoom.ts <svg> <name> [density] [left top width height]` (crop in fractions 0..1); writes 🗑️generated/r3-external-venetian-blind/<name>.png. */
import sharp from "sharp";
import { mkdirSync } from "node:fs";
import { dirname, join } from "node:path";

const [file, name, density, l, t, w, h] = process.argv.slice(2);
const image = sharp(file!, { density: Number(density ?? 900), limitInputPixels: false }).flatten({ background: "#ffffff" });
const meta = await image.metadata();
const out = join(import.meta.dir, "🗑️generated", "r3-external-venetian-blind", `${name}.png`);
mkdirSync(dirname(out), { recursive: true });
const cut = l === undefined ? image : image.extract({ left: Math.round(Number(l) * meta.width!), top: Math.round(Number(t) * meta.height!), width: Math.round(Number(w) * meta.width!), height: Math.round(Number(h) * meta.height!) });
await cut.png().toFile(out);
console.log(out, meta.width, meta.height);
