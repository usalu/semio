/** 📦️ Brings the body of every authored scale of a plant onto its declared dimensions (within one smallest feature per axis). Per axis: the declared value stays when every scale has it; else, when one value lies within a smallest feature of every scale, the axis is redeclared at the such value nearest to the declared one (no geometry changes; written to `manifest-changes.json` too); else the declared value stays when at least one scale has it, and otherwise the axis is redeclared at the extent of the finest scale. A scale that then still misses the axis gets its outermost solids moved along that axis (a plain polyhedron is stretched along the axis from its far end instead, so it stays joined to what carries it) until both ends of its extent stand where the finest agreeing scale has them (widened evenly to the declared value; the height keeps its foot). The ops go to the head of `ops/<id>.json`.
 *  bun bbox.ts <id> [--file <asset.json>] [--dry] [--keep widthM,heightM,depthM] [--set depthM=15.878]…   (--keep: never redeclare these axes; --set: declare an axis at this value, for instance where the symbol of the asset bounds it) */
import { parseArgs } from "node:util";
import { featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { applyOps, mergeOps, readOps, writeOps, type Op } from "./ops.ts";
import { fileOf, opsFile } from "./plants.ts";

const { values, positionals } = parseArgs({ allowPositionals: true, options: { file: { type: "string" }, dry: { type: "boolean" }, keep: { type: "string" }, set: { type: "string", multiple: true } } });
const id = positionals[0]!;
let asset = (await Bun.file(values.file ?? fileOf(id)).json()).asset;
const start = await readOps(opsFile(id));
applyOps(asset, start);
const AXES = ["widthM", "heightM", "depthM"] as const;
const scales = [50, 100, 200, 500].filter((d) => asset.volumes[String(d)]?.status === "authored");
const worlds = (scale: number): any[] => {
  const state: any = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose });
  return (worldSolids(asset, state.bones as never, scale) as any[]).filter((w) => w.texture !== true);
};
const span = (w: any, k: number): [number, number] => { let [lo, hi] = [Infinity, -Infinity]; for (let i = k; i < w.positions.length; i += 3) { lo = Math.min(lo, w.positions[i]); hi = Math.max(hi, w.positions[i]); } return [lo, hi]; };
const extent = (ws: any[], k: number): [number, number] => ws.reduce(([lo, hi], w) => { const s = span(w, k); return [Math.min(lo, s[0]), Math.max(hi, s[1])]; }, [Infinity, -Infinity] as [number, number]);
const before = Object.fromEntries(scales.map((d) => [d, worlds(d)]));
const added: Op[] = [];
const dims: Record<string, number> = {};
AXES.forEach((axis, k) => {
  const declared = asset.dimensions[axis] as number;
  const ext = Object.fromEntries(scales.map((d) => [d, extent(before[d]!, k)])) as Record<number, [number, number]>;
  const size = (d: number): number => ext[d]![1] - ext[d]![0];
  const agree = (value: number): number[] => scales.filter((d) => Math.abs(size(d) - value) <= featureOf(d) * 0.98);
  const [lo, hi] = [Math.max(...scales.map((d) => size(d) - featureOf(d))), Math.min(...scales.map((d) => size(d) + featureOf(d)))];
  const margin = Math.min(0.15 * featureOf(scales[0]!), (hi - lo) / 2);
  const shared = Number(Math.min(Math.max(declared, lo + margin), hi - margin).toFixed(4));
  const forced = (values.set ?? []).map((x) => x.split("=")).find(([name]) => name === axis)?.[1];
  const best = forced !== undefined ? Number(forced) : agree(declared).length === scales.length || (values.keep ?? "").split(",").includes(axis) ? declared : lo <= hi && agree(shared).length === scales.length ? shared : agree(declared).length > 0 ? declared : Number(size(scales[0]!).toFixed(3));
  const reference = agree(best)[0];
  if (best !== declared) dims[axis] = best;
  const off = scales.filter((d) => !agree(best).includes(d));
  console.log(`${id} ${axis}: declared ${declared}${best !== declared ? ` -> ${best}` : ""}; bodies ${scales.map((d) => `1:${d} ${size(d).toFixed(4)}`).join(", ")}; ${off.length === 0 ? "all agree" : `to move: ${off.map((d) => `1:${d}`).join(", ")}${reference === undefined ? " (no scale agrees: the ends are kept centred as they are)" : ` onto the ends of 1:${reference}`}`}`);
  for (const d of off) {
    const feature = featureOf(d);
    const from = ext[reference ?? d]!;
    const target: [number, number] = [from[0] - (best - (from[1] - from[0])) / 2, from[1] + (best - (from[1] - from[0])) / 2];
    if (k === 1) { target[1] += from[0] - target[0]; target[0] = from[0]; }
    const moved = new Map<string, number>();
    const stretched = new Set<string>();
    for (let step = 0; step < 200; step++) {
      const ws = worlds(d);
      const now = extent(ws, k);
      const errors = [now[0] - target[0], now[1] - target[1]];
      const side = Math.abs(errors[0]!) > Math.abs(errors[1]!) ? 0 : 1;
      if (Math.abs(errors[side]!) <= 0.2 * feature) break;
      const w = ws.reduce((p, q) => ((side === 0 ? span(q, k)[0] < span(p, k)[0] : span(q, k)[1] > span(p, k)[1]) ? q : p));
      const delta = -errors[side]!;
      const list = asset.volumes[String(d)].body as any[];
      const solid = list.find((x) => x.id === w.id);
      const f = w.frame;
      const axisOf = (v: any): number => (k === 0 ? v.x : k === 1 ? v.y : v.z);
      const local = [axisOf(f.u) * delta, axisOf(f.v) * delta, axisOf(f.w) * delta];
      if (solid.kind === "polyhedron" && solid.transform === undefined && solid.repeat === undefined && solid.weights === undefined) {
        const unit = [axisOf(f.u), axisOf(f.v), axisOf(f.w)];
        const along = (solid.vertices as number[][]).map((v) => v[0]! * unit[0]! + v[1]! * unit[1]! + v[2]! * unit[2]!);
        const [lo, hi] = [Math.min(...along), Math.max(...along)];
        const pivot = side === 0 ? hi : lo;
        const factor = (side === 0 ? pivot - (lo + delta) : hi + delta - pivot) / (hi - lo);
        solid.vertices = (solid.vertices as number[][]).map((v, i) => v.map((x, c) => Number((x + (factor - 1) * (along[i]! - pivot) * unit[c]!).toFixed(6))));
        stretched.add(w.id);
      } else {
        const t = solid.transform?.translation ?? [0, 0, 0];
        solid.transform = { ...(solid.transform ?? {}), translation: t.map((x: number, i: number) => Number((x + local[i]!).toFixed(5))) };
      }
      moved.set(w.id, (moved.get(w.id) ?? 0) + delta);
      asset = structuredClone(asset);
    }
    const after = extent(worlds(d), k);
    for (const [sid] of moved) { const now = (asset.volumes[String(d)].body as any[]).find((x) => x.id === sid); added.push({ op: "solid-set", scale: String(d), id: sid, set: stretched.has(sid) ? { vertices: now.vertices } : { transform: now.transform } }); }
    console.log(`   1:${d}: ${[...moved].map(([sid, delta]) => `${sid} ${(delta * 1000).toFixed(1)} mm`).join(", ")} -> ${(after[1] - after[0]).toFixed(4)} m (ends ${after[0].toFixed(4)} .. ${after[1].toFixed(4)}, wanted ${target[0].toFixed(4)} .. ${target[1].toFixed(4)})${Math.abs(after[1] - after[0] - best) > feature ? " STILL OFF" : ""}`);
  }
});
if (Object.keys(dims).length > 0) added.unshift({ op: "dims", set: dims });
if (!values.dry && added.length > 0) {
  await writeOps(opsFile(id), mergeOps(start, added));
  if (Object.keys(dims).length > 0) {
    const file = `${import.meta.dir}/manifest-changes.json`;
    const all = (await Bun.file(file).exists()) ? await Bun.file(file).json() : {};
    all[id] = { ...(all[id] ?? {}), ...dims };
    await Bun.write(file, JSON.stringify(Object.fromEntries(Object.entries(all).sort(([a], [b]) => a.localeCompare(b))), null, 2) + "\n");
  }
}
console.log(`${id}: ${added.length} ops${values.dry ? " (dry)" : ""}`);
