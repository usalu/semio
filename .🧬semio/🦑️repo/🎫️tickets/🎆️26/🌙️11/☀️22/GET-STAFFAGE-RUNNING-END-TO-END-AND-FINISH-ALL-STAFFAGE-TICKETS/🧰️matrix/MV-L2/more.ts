/** 🔦️ Where one drawing fills what another leaves empty, in the default pose: `bun more.ts <before.json> <after.json> <scale> <view> [grid metres, default 0.005]` samples the box of the view on a grid and prints, per filled shape of the second document, how many grid points it covers that no filled shape of the first document covers (with the box of those points), most first.
 * @see ../MV-3/refdiff.ts — the same question in the cells of the silhouette guard */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { S, fileOf } from "../MR/lib.ts";

const engine = await import(join(S, "🧪️tests", "🔭️multiview-coverage", "🧰️.ts"));
const { evaluate } = await import(join(S, "🔨️modules", "🧮️evaluation", "🟦️.ts"));
type P = { x: number; y: number };
const [first, second, scale, view, grid] = process.argv.slice(2);
const step = Number(grid ?? 0.005);
const shapesOf = (file: string): { id: string; rings: P[][]; box: number[] }[] => {
  const asset = JSON.parse(readFileSync(fileOf(file), "utf8")).asset;
  const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, engine.scaleSelector(Number(scale)));
  return state.shapes[view!].filter((entry: any) => entry.shape.filled === true && entry.shape.role !== "ground" && entry.rings.flat().length > 2).map((entry: any) => { const points: P[] = entry.rings.flat(); return { id: `${entry.shape.id} on ${entry.shape.bone} (${entry.shape.kind})`, rings: entry.rings, box: [Math.min(...points.map((p) => p.x)), Math.max(...points.map((p) => p.x)), Math.min(...points.map((p) => p.y)), Math.max(...points.map((p) => p.y))] }; });
};
const covers = (shape: { rings: P[][]; box: number[] }, p: P): boolean => {
  if (p.x < shape.box[0]! || p.x > shape.box[1]! || p.y < shape.box[2]! || p.y > shape.box[3]!) return false;
  let inside = false;
  for (const ring of shape.rings) for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) { const [u, v] = [ring[i]!, ring[j]!]; if ((u.y > p.y) !== (v.y > p.y) && p.x < u.x + ((v.x - u.x) * (p.y - u.y)) / (v.y - u.y)) inside = !inside; }
  return inside;
};
const [a, b] = [shapesOf(first!), shapesOf(second!)];
const all = [...a, ...b];
const [x0, x1, y0, y1] = [Math.min(...all.map((s) => s.box[0]!)), Math.max(...all.map((s) => s.box[1]!)), Math.min(...all.map((s) => s.box[2]!)), Math.max(...all.map((s) => s.box[3]!))];
const found = new Map<string, { n: number; box: number[] }>();
for (let x = x0; x <= x1; x += step) for (let y = y0; y <= y1; y += step) {
  const p = { x, y };
  if (a.some((shape) => covers(shape, p))) continue;
  const by = b.find((shape) => covers(shape, p));
  if (by === undefined) continue;
  const entry = found.get(by.id) ?? { n: 0, box: [x, x, y, y] };
  found.set(by.id, { n: entry.n + 1, box: [Math.min(entry.box[0]!, x), Math.max(entry.box[1]!, x), Math.min(entry.box[2]!, y), Math.max(entry.box[3]!, y)] });
}
for (const [id, entry] of [...found].sort((p, q) => q[1].n - p[1].n).slice(0, 25)) console.log(`${entry.n} points: ${id}, x ${entry.box[0]!.toFixed(3)} … ${entry.box[1]!.toFixed(3)}, y ${entry.box[2]!.toFixed(3)} … ${entry.box[3]!.toFixed(3)}`);
console.log(`1:${scale} ${view}: ${[...found.values()].reduce((sum, entry) => sum + entry.n, 0)} grid points of ${step} m filled only by the second document, in ${found.size} shapes`);
