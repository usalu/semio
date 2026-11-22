/** 🔎️ Finisher A locator: where the body of one scale and its drawing disagree in one view. Rasterises the product's world solids (`worldSolids`, default pose) and the filled non-texture shapes the product places (`evaluate`), prints IoU, percentiles and the farthest boundary cells with the solids or shapes that cover them.
 * Usage: `bun where.ts <asset document json> <scale> <elevation|front|plan> [count]`
 * @see ./guard.ts */
import { readFileSync } from "node:fs";
import { evaluate } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { project, worldSolids } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { Grid, edt, type P2 } from "../PW-08/lib.ts";

const [file, scaleText, view, countText] = process.argv.slice(2) as [string, string, "elevation" | "front" | "plan", string | undefined];
const scale = Number(scaleText);
const asset = JSON.parse(readFileSync(file, "utf8")).asset;
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } }) as any;
const solids = worldSolids(asset, state.bones, scale).filter((s) => s.texture !== true);
const cell = 0.01 * (scale / 100);
const meshes = solids.map((s) => {
  const points: P2[] = [];
  for (let k = 0; k < s.positions.length; k += 3) {
    const p = project(view, s.positions[k]!, s.positions[k + 1]!, s.positions[k + 2]!);
    points.push([p.x, p.y]);
  }
  return { id: s.id, points, indices: s.indices };
});
const shapes = (state.shapes[view] as any[]).filter((e) => e.shape.filled && e.shape.role !== "texture");
const all: P2[] = [...meshes.flatMap((m) => m.points), ...shapes.flatMap((e) => e.rings.flatMap((r: any[]) => r.map((q) => [q.x, q.y] as P2)))];
const body = Grid.around(all, cell, 0.5);
for (const m of meshes) for (let k = 0; k < m.indices.length; k += 3) body.tri(m.points[m.indices[k]!]!, m.points[m.indices[k + 1]!]!, m.points[m.indices[k + 2]!]!);
const drawn = body.like();
for (const e of shapes) for (const ring of e.rings) drawn.ring(ring.map((q: any) => [q.x, q.y] as P2));
const boundary = (g: Grid): Grid => {
  const b = g.like();
  for (let j = 0; j < g.h; j++) for (let i = 0; i < g.w; i++) {
    const k = j * g.w + i;
    if (g.d[k] && (i === 0 || j === 0 || i === g.w - 1 || j === g.h - 1 || !g.d[k - 1] || !g.d[k + 1] || !g.d[k - g.w] || !g.d[k + g.w])) b.d[k] = 1;
  }
  return b;
};
const [bodyEdge, drawnEdge] = [boundary(body), boundary(drawn)];
const [toBody, toDrawn] = [edt(bodyEdge, (v) => v === 0), edt(drawnEdge, (v) => v === 0)];
const samples: { d: number; x: number; y: number; side: string }[] = [];
for (let k = 0; k < body.d.length; k++) {
  const [x, y] = [body.x0 + ((k % body.w) + 0.5) * cell, body.y0 + (Math.floor(k / body.w) + 0.5) * cell];
  if (drawnEdge.d[k]) samples.push({ d: Math.sqrt(toBody[k]!) * cell, x, y, side: "drawing" });
  if (bodyEdge.d[k]) samples.push({ d: Math.sqrt(toDrawn[k]!) * cell, x, y, side: "body" });
}
samples.sort((a, b) => a.d - b.d);
const at = (q: number): number => samples[Math.floor(q * (samples.length - 1))]!.d * 1000;
let [both, either] = [0, 0];
for (let k = 0; k < body.d.length; k++) {
  if (body.d[k] || drawn.d[k]) either++;
  if (body.d[k] && drawn.d[k]) both++;
}
console.log(`1:${scale} ${view}: iou ${(both / either).toFixed(4)} p50 ${at(0.5).toFixed(1)} p95 ${at(0.95).toFixed(1)} max ${at(1).toFixed(1)} mm (cell ${cell * 1000} mm, ${solids.length} solids, ${shapes.length} filled shapes)`);
const inTriangle = (p: P2, a: P2, b: P2, c: P2): boolean => {
  const s = (u: P2, v: P2): number => (v[0] - u[0]) * (p[1] - u[1]) - (v[1] - u[1]) * (p[0] - u[0]);
  const [d1, d2, d3] = [s(a, b), s(b, c), s(c, a)];
  return !((d1 < 0 || d2 < 0 || d3 < 0) && (d1 > 0 || d2 > 0 || d3 > 0));
};
const inRing = (p: P2, ring: { x: number; y: number }[]): boolean => {
  let inside = false;
  for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) {
    const [a, b] = [ring[i]!, ring[j]!];
    if (a.y > p[1] !== b.y > p[1] && p[0] < ((b.x - a.x) * (p[1] - a.y)) / (b.y - a.y) + a.x) inside = !inside;
  }
  return inside;
};
const seen: { x: number; y: number }[] = [];
for (const s of [...samples].reverse()) {
  if (seen.length >= Number(countText ?? 8)) break;
  if (seen.some((q) => Math.hypot(q.x - s.x, q.y - s.y) < 12 * cell)) continue;
  seen.push(s);
  const p: P2 = [s.x, s.y];
  const near = (q: P2): P2[] => [q, [q[0] + cell, q[1]], [q[0] - cell, q[1]], [q[0], q[1] + cell], [q[0], q[1] - cell]];
  const covering = s.side === "body"
    ? meshes.filter((m) => near(p).some((q) => { for (let k = 0; k < m.indices.length; k += 3) if (inTriangle(q, m.points[m.indices[k]!]!, m.points[m.indices[k + 1]!]!, m.points[m.indices[k + 2]!]!)) return true; return false; })).map((m) => m.id)
    : shapes.filter((e) => near(p).some((q) => e.rings.some((r: any) => inRing(q, r)))).map((e) => `${e.shape.id}@${e.shape.bone}`);
  console.log(`  ${s.side} edge ${(s.d * 1000).toFixed(0)} mm from the other at ${s.x.toFixed(2)}, ${s.y.toFixed(2)}: ${covering.slice(0, 5).join(", ")}`);
}
