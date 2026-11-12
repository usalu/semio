/** IoU of the authored 2D views against the posed 3D volume: for every pose and four instants of every clip, at the given scales, before and after.
 * Usage: bun lateral-posed.ts <slug> <before.json> [--scales 50,200] [--states pose-or-clip-substring] */
import { readFileSync } from "node:fs";
import { tessellateSolid } from "../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🔺️tessellation/🟦️.ts";
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { Mask, QUALITY_FOR, VIEWS, catalogEntries, loadAsset, projectWorld, toWorld, type Json, type V3, type View } from "./volume-lib.ts";
import { select, statesOf } from "./lateral-snap.ts";
export type Tri = { id: string; local: Float64Array; indices: Uint32Array; bone: string };
export function localMeshes(asset: Json): Tri[] {
  const maxDim = Math.max(asset.dimensions.widthM, asset.dimensions.heightM, asset.dimensions.depthM);
  return asset.volume.solids.map((s: Json) => { const g = tessellateSolid({ ...s, node: s.bone } as never, QUALITY_FOR(maxDim)); return { id: s.id, bone: s.bone, local: Float64Array.from(g.positions), indices: Uint32Array.from(g.indices) }; });
}
const AUTH = new Set(["body", "detail", "glazing"]);
export function posedIou(asset: Json, tris: Tri[], state: Json): Record<View, number> {
  const arr = (p: Json) => [p.x, p.y, p.z] as V3;
  const frameOf = (b: string) => { const f = state.bones[b].frame; return { o: arr(f.o), u: arr(f.u), v: arr(f.v), w: arr(f.w) }; };
  const cache = new Map<string, ReturnType<typeof frameOf>>();
  const out = {} as Record<View, number>;
  const world = tris.map((t) => {
    const f = cache.get(t.bone) ?? cache.set(t.bone, frameOf(t.bone)).get(t.bone)!;
    const w = new Float64Array(t.local.length);
    for (let i = 0; i < t.local.length; i += 3) { const p = toWorld(f, [t.local[i]!, t.local[i + 1]!, t.local[i + 2]!]); w[i] = p[0]; w[i + 1] = p[1]; w[i + 2] = p[2]; }
    return w;
  });
  for (const v of VIEWS) {
    const shapes = (state.shapes[v] as Json[]).filter((s) => s.closed && AUTH.has(s.shape.role) && s.rings.length > 0);
    const proj = world.map((w) => { const o = new Float64Array((w.length / 3) * 2); for (let i = 0; i < w.length / 3; i++) { const p = projectWorld(v, [w[i * 3]!, w[i * 3 + 1]!, w[i * 3 + 2]!]); o[i * 2] = p.x; o[i * 2 + 1] = p.y; } return o; });
    let x0 = Infinity, y0 = Infinity, x1 = -Infinity, y1 = -Infinity;
    for (const s of shapes) for (const r of s.rings) for (const p of r) { x0 = Math.min(x0, p.x); x1 = Math.max(x1, p.x); y0 = Math.min(y0, p.y); y1 = Math.max(y1, p.y); }
    for (const o of proj) for (let i = 0; i < o.length; i += 2) { x0 = Math.min(x0, o[i]!); x1 = Math.max(x1, o[i]!); y0 = Math.min(y0, o[i + 1]!); y1 = Math.max(y1, o[i + 1]!); }
    const cell = Math.max(1e-4, Math.max(x1 - x0, y1 - y0) / 500);
    const box = { minX: x0 - 2 * cell, minY: y0 - 2 * cell, maxX: x1 + 2 * cell, maxY: y1 + 2 * cell };
    const a = new Mask(box, cell), b = new Mask(box, cell);
    for (const s of shapes) a.fill(s.rings);
    tris.forEach((t, n) => { const o = proj[n]!; for (let k = 0; k < t.indices.length; k += 3) b.fill([[0, 1, 2].map((j) => ({ x: o[t.indices[k + j]! * 2]!, y: o[t.indices[k + j]! * 2 + 1]! }))]); });
    let i = 0, u = 0;
    for (let k = 0; k < a.cells.length; k++) { i += a.cells[k]! & b.cells[k]!; u += a.cells[k]! | b.cells[k]!; }
    out[v] = u === 0 ? 1 : i / u;
  }
  return out;
}
if (import.meta.main) {
  const slug = process.argv[2]!;
  const before = JSON.parse(readFileSync(process.argv[3]!, "utf8")).asset as Json;
  const after = loadAsset(catalogEntries().find((x) => x.slug === slug)!.path);
  const si = process.argv.indexOf("--scales");
  const scales = si > 0 ? process.argv[si + 1]!.split(",").map(Number) : [50, 100, 200, 500];
  const only = process.argv.indexOf("--states") > 0 ? process.argv[process.argv.indexOf("--states") + 1] : undefined;
  const trisB = localMeshes(before), trisA = localMeshes(after);
  const sum: Record<string, { b: number[]; a: number[] }> = {};
  for (const scale of scales) for (const st of statesOf(after)) {
    if (only && !st.name.includes(only)) continue;
    const sb = evaluate(before, st.query as never, select(scale)) as Json, sa = evaluate(after, st.query as never, select(scale)) as Json;
    const ib = posedIou(before, trisB, sb), ia = posedIou(after, trisA, sa);
    console.log(`1:${scale} ${st.name.padEnd(34)} ${VIEWS.map((v) => `${v} ${ib[v].toFixed(3)} -> ${ia[v].toFixed(3)}`).join("   ")}`);
    for (const v of VIEWS) { (sum[`${scale} ${v}`] ??= { b: [], a: [] }).b.push(ib[v]); sum[`${scale} ${v}`]!.a.push(ia[v]); }
  }
  console.log("# mean over states (before -> after), min after");
  for (const [k, v] of Object.entries(sum)) { const m = (x: number[]) => x.reduce((p, c) => p + c, 0) / x.length; console.log(`${k.padEnd(14)} mean ${m(v.b).toFixed(4)} -> ${m(v.a).toFixed(4)}   min ${Math.min(...v.b).toFixed(4)} -> ${Math.min(...v.a).toFixed(4)}`); }
}
