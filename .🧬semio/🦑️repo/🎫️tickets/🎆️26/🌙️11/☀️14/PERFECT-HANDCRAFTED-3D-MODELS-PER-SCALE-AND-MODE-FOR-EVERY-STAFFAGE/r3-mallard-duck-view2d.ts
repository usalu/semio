/** 🔍️ Round 3 mallard: the three 2D drawings of one scale enlarged (the product's sheets show them at paper size, 6 mm long), placed by the product's own `evaluate`; fills in their tones, hairlines instead of pens so that the geometry can be judged. Usage: bun r3-mallard-duck-view2d.ts <scale> [pose] [--catalog]
 * @see ./r3-2d.ts */
import { mkdirSync, readFileSync } from "node:fs";
import sharp from "sharp";
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { entryOf } from "./volumes-lib.ts";

const scale = Number(process.argv[2] ?? 50);
const pose = process.argv[3] !== undefined && !process.argv[3].startsWith("--") ? process.argv[3] : "standing";
const asset = process.argv.includes("--catalog") ? entryOf("animals/mallard-duck").asset : JSON.parse(readFileSync(new URL("./🗑️generated/r3-mallard-duck-full/🔣️.json", import.meta.url), "utf8")).asset;
const state = evaluate(asset as never, { kind: "pose", pose } as never, { kind: "scale", scale: { denominator: scale } } as never) as any;
const K = 1900;
const panels = [["elevation", 40, 720, 0.19], ["front", 960, 720, 0.09], ["plan", 1330, 380, 0.19]] as const;
let svg = `<svg xmlns="http://www.w3.org/2000/svg" width="2400" height="780"><rect width="2400" height="780" fill="#f6f3e8"/>`;
for (const [view, ox, oy, shift] of panels) {
  const shapes = [...state.shapes[view]].sort((a: any, b: any) => a.shape.z - b.shape.z);
  svg += `<text x="${ox}" y="30" font-family="monospace" font-size="22">${view} 1:${scale} ${pose} (${shapes.length} shapes)</text><line x1="${ox}" y1="${oy}" x2="${ox + 300}" y2="${oy}" stroke="#bbb"/>`;
  for (const s of shapes) {
    const d = s.rings.map((r: any[]) => r.map((p, i) => `${i === 0 ? "M" : "L"}${(ox + (p.x + shift) * K).toFixed(1)} ${(oy - p.y * K).toFixed(1)}`).join("") + (s.closed ? "Z" : "")).join("");
    const g = Math.round(255 * (0.25 + 0.75 * s.shape.tone));
    svg += `<path d="${d}" fill="${s.shape.filled ? `rgb(${g},${g},${Math.round(g * 0.96)})` : "none"}" fill-rule="evenodd" stroke="${s.shape.role === "texture" ? "#666" : "#111"}" stroke-width="${s.shape.role === "body" ? 1.6 : 1}"/>`;
  }
}
svg += "</svg>";
mkdirSync(new URL("./🗑️generated/r3-mallard-duck/", import.meta.url), { recursive: true });
const out = new URL(`./🗑️generated/r3-mallard-duck/view2d.s${scale}.${pose}.png`, import.meta.url);
await sharp(Buffer.from(svg)).png().toFile(decodeURIComponent(out.pathname));
console.log(decodeURIComponent(out.pathname));
