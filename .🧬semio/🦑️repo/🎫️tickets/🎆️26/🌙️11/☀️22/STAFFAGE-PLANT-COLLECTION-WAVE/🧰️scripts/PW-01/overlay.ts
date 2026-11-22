/** 🔍️ Overlays the filled body outlines of two scales of a candidate in the front or plan view (world rest coordinates) as a PNG: first scale red, second blue, both purple. `overlay.ts <id> <scaleA> <scaleB> <front|plan>` */
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import sharp from "../../../../../../../../../node_modules/sharp/lib/index.js";

const [id, a, b, view] = process.argv.slice(2) as [string, string, string, "front" | "plan"];
const G = join(dirname(fileURLToPath(import.meta.url)), "../../🗑️generated/PW-01");
const asset = JSON.parse(readFileSync(join(G, id, "🔣️.json"), "utf8")).asset;
const shapes = (s: string): string[] => asset.representations[s].geometry[view].filter((x: { role: string; filled: boolean }) => x.role === "body" && x.filled).map((x: { d: string }) => x.d);
const all = [...shapes(a), ...shapes(b)].join(" ").match(/-?\d+(\.\d+)?/g)!.map(Number);
const xs = all.filter((_, i) => i % 2 === 0), ys = all.filter((_, i) => i % 2 === 1);
const [x0, x1, y0, y1] = [Math.min(...xs) - 0.5, Math.max(...xs) + 0.5, Math.min(...ys) - 0.5, Math.max(...ys) + 0.5];
const k = 1400 / (x1 - x0);
const g = (list: string[], colour: string): string => `<g fill="${colour}" fill-opacity="0.5" stroke="none" fill-rule="nonzero">${list.map((d) => `<path d="${d}"/>`).join("")}</g>`;
const svg = `<svg xmlns="http://www.w3.org/2000/svg" width="1400" height="${Math.round((y1 - y0) * k)}" viewBox="${x0} ${-y1} ${x1 - x0} ${y1 - y0}"><rect x="${x0}" y="${-y1}" width="${x1 - x0}" height="${y1 - y0}" fill="white"/><g transform="scale(1,-1)">${g(shapes(a), "red")}${g(shapes(b), "blue")}</g></svg>`;
await sharp(Buffer.from(svg)).png().toFile(join(G, `overlay-${id}-${a}-${b}-${view}.png`));
console.log("ok");
