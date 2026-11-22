/** 🧩️ Trims the single-view SVGs of a folder (elevation, front, plan) and joins them left to right in one PNG of a given height.
 *  bun join.ts <folder> <height> */
import { readdirSync } from "node:fs";
const sharp = (await import("../../../../../../../../../node_modules/sharp/lib/index.js")).default;
const [dir, h] = process.argv.slice(2);
const parts: Buffer[] = [];
for (const v of ["elevation", "front", "plan"]) {
  const svg = readdirSync(`${dir}/${v}`).find((f) => f.endsWith(".svg"));
  if (!svg) continue;
  const buf = await sharp(`${dir}/${v}/${svg}`, { density: Number(process.env.DPI ?? 600), limitInputPixels: false }).flatten({ background: "#ffffff" }).png().toBuffer();
  parts.push(await sharp(buf, { limitInputPixels: false }).trim({ threshold: 20 }).resize({ height: Number(h), fit: "inside" }).extend({ left: 20, right: 20, background: "#ffffff" }).png().toBuffer());
}
const metas = await Promise.all(parts.map((p) => sharp(p).metadata()));
const W = metas.reduce((s, m) => s + m.width!, 0);
let x = 0;
const out = `${dir}.png`;
await sharp({ create: { width: W, height: Number(h), channels: 3, background: "#ffffff" } }).composite(parts.map((p, i) => { const r = { input: p, left: x, top: 0 }; x += metas[i]!.width!; return r; })).png().toFile(out);
console.log(out);
