/** 🔬 Round 3 tied canopy: rasterises a window of a product SVG (shares of its viewBox: left, top, width, height) at 1600 px wide, to judge pens and outlines of the 2D at node scale. Usage: bun r3-tied-canopy-svgzoom.ts <in.svg> <out.png> l t w h */
import { readFileSync } from "node:fs";
import sharp from "sharp";

const [src, out, l, t, w, h] = process.argv.slice(2);
const svg = readFileSync(src!, "utf8");
const m = /viewBox="([-\d.]+) ([-\d.]+) ([-\d.]+) ([-\d.]+)"/.exec(svg)!;
const [x0, y0, vw, vh] = m.slice(1).map(Number) as [number, number, number, number];
const box = [x0 + vw * Number(l), y0 + vh * Number(t), vw * Number(w), vh * Number(h)];
const width = 1600;
const height = Math.round((width * box[3]!) / box[2]!);
const zoomed = svg.replace(m[0], `viewBox="${box.join(" ")}"`).replace(/width="[^"]+" height="[^"]+"/, `width="${width}" height="${height}"`);
await sharp(Buffer.from(zoomed), { density: 72 }).flatten({ background: "#ffffff" }).png().toFile(out!);
console.log(out);
