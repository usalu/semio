/** 🔎 Round 3, machinery/expansion-valve: magnifies the product's 2D SVG panels of one scale (written by `r3-2d.ts --svg`) around the tiny object into one PNG. Usage: bun r3-expansion-valve-zoom.ts <scale> */
import { readFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";
import sharp from "sharp";

const scale = process.argv[2] ?? "50";
const dir = join(import.meta.dir, "🗑️generated", "r3-2d", "machinery");
const out = join(import.meta.dir, "🗑️generated", "r3-expansion-valve");
mkdirSync(out, { recursive: true });
const boxes: Record<string, [number, number, number, number]> = { elevation: [11.4, 8.9, 5.2, 3.0], front: [10.4, 8.9, 5.2, 3.0], plan: [4.9, 8.2, 5.2, 3.0] };
const tiles: { input: Buffer; left: number; top: number }[] = [];
let row = 0;
for (const view of ["light-line", "light-solid"]) {
  let col = 0;
  for (const [name, b] of Object.entries(boxes)) {
    const box = b;
    const svg = readFileSync(join(dir, `expansion-valve.s${scale}.${name}.${view}.svg`), "utf8").replace(/width="[^"]*" height="[^"]*" viewBox="[^"]*"/, `width="1040" height="600" viewBox="${box.join(" ")}"`);
    tiles.push({ input: await sharp(Buffer.from(svg), { density: 96 }).resize(1040, 600, { fit: "contain", background: "#f5f1e1" }).png().toBuffer(), left: col * 1050, top: row * 610 });
    col++;
  }
  row++;
}
const file = join(out, `zoom-2d.s${scale}.png`);
await sharp({ create: { width: 3150, height: 1220, channels: 3, background: "#ffffff" } }).composite(tiles).png().toFile(file);
console.log(file);
