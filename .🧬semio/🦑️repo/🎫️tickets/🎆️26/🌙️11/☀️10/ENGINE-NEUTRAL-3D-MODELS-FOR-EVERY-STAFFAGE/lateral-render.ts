/** Renders before/after of the three views of one state as an SVG (rows: before, after; columns: elevation, front, plan).
 * Usage: bun lateral-render.ts <slug> <before.json> <out.svg> (--pose <id> | --clip <id> --t <s>) [--scale 50] [--volume] */
import { readFileSync, writeFileSync } from "node:fs";
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { catalogEntries, loadAsset, meshes, projectWorld, VIEWS, type Json } from "./volume-lib.ts";
const a = process.argv.slice(2);
const opt = (k: string) => { const i = a.indexOf(k); return i >= 0 ? a[i + 1] : undefined; };
const slug = a[0]!;
const before = JSON.parse(readFileSync(a[1]!, "utf8")).asset as Json;
const after = loadAsset(catalogEntries().find((x) => x.slug === slug)!.path);
const scale = Number(opt("--scale") ?? 50);
const query = opt("--clip") ? { kind: "clip", clip: opt("--clip"), timeS: Number(opt("--t") ?? 0) } : { kind: "pose", pose: opt("--pose") ?? after.presentation.defaultPose };
const boneFilter = opt("--bone")?.split(",");
const sets = [before, after].map((asset) => { const sh = (evaluate(asset, query as never, { kind: "scale", scale: { denominator: scale } } as never) as Json).shapes; return boneFilter ? Object.fromEntries(Object.entries(sh).map(([v, l]) => [v, (l as Json[]).filter((x) => boneFilter.includes(x.shape.bone))])) : sh; });
let minX = Infinity, minY = Infinity, maxX = -Infinity, maxY = -Infinity;
const bounds: Record<string, number[]> = {};
for (const v of VIEWS) {
  let x0 = Infinity, y0 = Infinity, x1 = -Infinity, y1 = -Infinity;
  for (const s of sets) for (const sh of s[v]) for (const r of sh.rings) for (const p of r) { x0 = Math.min(x0, p.x); x1 = Math.max(x1, p.x); y0 = Math.min(y0, p.y); y1 = Math.max(y1, p.y); }
  bounds[v] = [x0, y0, x1, y1];
  minX = Math.min(minX, x0); maxX = Math.max(maxX, x1); minY = Math.min(minY, y0); maxY = Math.max(maxY, y1);
}
const PX = 330;
const unitOf = (v: string) => PX / Math.max(bounds[v]![2]! - bounds[v]![0]!, bounds[v]![3]! - bounds[v]![1]!) * 0.92;
let body = "";
const showVolume = a.includes("--volume");
const vol = showVolume ? meshes(after, after.volume.solids, 2) : [];
[before, after].forEach((_, row) => VIEWS.forEach((v, col) => {
  const [x0, y0, x1, y1] = bounds[v]!;
  const unit = unitOf(v);
  const ox = col * (PX + 20) + 10 + (PX - (x1! - x0!) * unit) / 2, oy = row * (PX + 40) + 30 + (PX - (y1! - y0!) * unit) / 2 + (y1! - y0!) * unit;
  const X = (x: number) => (ox + (x - x0!) * unit).toFixed(1), Y = (y: number) => (oy - (y - y0!) * unit).toFixed(1);
  body += `<rect x="${col * (PX + 20) + 10}" y="${row * (PX + 40) + 30}" width="${PX}" height="${PX}" fill="none" stroke="#bbb"/><text x="${col * (PX + 20) + 14}" y="${row * (PX + 40) + 24}" font-size="14" font-family="sans-serif">${row === 0 ? "before" : "after"} ${v}</text>`;
  const shapes = [...sets[row]![v]].sort((p: Json, q: Json) => (p.shape.z ?? 0) - (q.shape.z ?? 0));
  for (const sh of shapes) {
    if (sh.shape.role === "ground") continue;
    const tone = sh.shape.tone ?? 0.5;
    const g = Math.round(255 * (1 - 0.85 * tone));
    const d = sh.rings.map((r: Json[]) => "M" + r.map((p) => `${X(p.x)} ${Y(p.y)}`).join("L") + (sh.closed ? "Z" : "")).join("");
    body += `<path d="${d}" fill="${sh.closed && sh.shape.filled !== false ? `rgb(${g},${g},${g})` : "none"}" fill-opacity="0.85" stroke="#222" stroke-width="0.6"/>`;
  }
  if (showVolume && row === 1) for (const m of vol) { let d = ""; for (let t = 0; t < m.indices.length; t += 3) d += "M" + [0, 1, 2].map((k) => { const i = m.indices[t + k]! * 3; const p = projectWorld(v, [m.world[i]!, m.world[i + 1]!, m.world[i + 2]!]); return `${X(p.x)} ${Y(p.y)}`; }).join("L") + "Z"; body += `<path d="${d}" fill="none" stroke="#d33" stroke-width="0.25" opacity="0.5"/>`; }
}));
writeFileSync(a[2]!, `<svg xmlns="http://www.w3.org/2000/svg" width="${3 * (PX + 20) + 10}" height="${3 * (PX + 20) + 10}" viewBox="0 0 ${3 * (PX + 20) + 10} ${3 * (PX + 20) + 10}"><rect width="100%" height="100%" fill="white"/>${body}</svg>`);
