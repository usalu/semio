/** Elevation solids: per flagged shape either an extruded plate (lateral range of the volume) or, for round limb parts, a revolved solid about the bone axis, chosen by the posed volume (IoU of the shape against its evidence solids over all poses and clip instants). */
import { evaluate, flattenGeometry } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { Mask, toWorld, projectWorld, type Json, type V3 } from "./volume-lib.ts";
import { localMeshes, type Tri } from "./lateral-posed.ts";
import { select, statesOf as baseStates } from "./lateral-snap.ts";

/** the poses and clip instants, with the rest pose (all bone offsets zero) counted four times. */
const statesOf = (asset: Json) => [...baseStates(asset), ...Array.from({ length: 3 }, () => ({ name: "rest", query: { kind: "pose", pose: "lat-rest" } }))];
import { evidenceIds, iouRings, lateralRange, restRings, ribbon, withRest, type Ctx } from "./lateral-convert.ts";

type P = { x: number; y: number };
type Rings = P[][];
const r4 = (x: number): number => Math.round(x * 1e4) / 1e4;
const GEOMETRY_KEYS = ["kind", "points", "cx", "cy", "rx", "ry", "rotationDeg", "x", "y", "w", "h", "x0", "y0", "x1", "y1", "r0", "r1", "r", "d", "followTip", "closed"];
const bare = (shape: Json): Json => { const c: Json = { ...shape }; for (const k of GEOMETRY_KEYS) delete c[k]; return c; };

export const THIN_M = 0.01;

export function plateOf(ctx: Ctx, scale: number, shape: Json, thin = false): { shape: Json; source: string } {
  const lr = lateralRange(ctx, scale, shape);
  const flat = flattenGeometry(shape) as Json;
  const mid = (lr.range[0] + lr.range[1]) / 2;
  const solid = thin ? { kind: "extruded", sweep: "w", z0: r4(mid - THIN_M / 2), z1: r4(mid + THIN_M / 2) } : { kind: "extruded", sweep: "w", z0: r4(lr.range[0]), z1: r4(lr.range[1]) };
  if (flat.closed === false) return { shape: { kind: "poly", points: ribbon(flat.rings.flat() as P[]).map((p) => [p.x, p.y]), closed: true, ...bare(shape), solid }, source: `${lr.source}+ribbon` };
  return { shape: { ...shape, solid }, source: lr.source };
}

/** revolved about the bone axis u: eligible for a closed shape of a bone that is vertical at rest (v along x), the profile is the half extent across the axis along u, the axis shifted to the centre of the outline. */
export function revolvedOf(ctx: Ctx, scale: number, shape: Json, override?: { axisX: number; axisZ: number; k?: number }): Json | undefined {
  const f = ctx.frame[shape.bone]!;
  const flat = flattenGeometry(shape) as Json;
  if (flat.closed === false || Math.abs(f.u[0]) > 0.15 || Math.abs(f.v[0]) < 0.9 || shape.kind === "capsule" && false) return undefined;
  const ring = (flat.rings as Rings).reduce((big, r) => (r.length > big.length ? r : big), (flat.rings as Rings)[0] ?? []);
  if (ring.length < 3) return undefined;
  const us = ring.map((p) => p.x), vs = ring.map((p) => p.y);
  const lo = Math.min(...us), hi = Math.max(...us);
  if (hi - lo < 1e-3) return undefined;
  const slices = 24;
  const rows: [number, number, number][] = [];
  for (let i = 0; i < slices; i++) {
    const s = lo + ((hi - lo) * (i + 0.5)) / slices;
    let vlo = Infinity, vhi = -Infinity;
    ring.forEach((p, k) => { const q = ring[(k + 1) % ring.length]!; if ((p.x - s) * (q.x - s) > 0 || p.x === q.x) return; const v = p.y + ((q.y - p.y) * (s - p.x)) / (q.x - p.x); vlo = Math.min(vlo, v); vhi = Math.max(vhi, v); });
    if (Number.isFinite(vlo)) rows.push([s, (vhi - vlo) / 2, (vhi + vlo) / 2]);
  }
  if (rows.length < 3) return undefined;
  const c = rows.reduce((sum, r) => sum + r[2] * r[1], 0) / rows.reduce((sum, r) => sum + r[1], 0);
  const vx = f.v[0] > 0 ? 1 : -1;
  const axisX = override?.axisX ?? c * vx;
  const profile = [[r4(rows[0]![0]), 0], ...rows.map(([a, r]) => [r4(a), r4(Math.max(r * (override?.k ?? 1), 0.0005) + axisX * vx)]), [r4(rows[rows.length - 1]![0]), 0]];
  const lr = lateralRange(ctx, scale, shape).range;
  const axisZ = override?.axisZ ?? r4((lr[0] + lr[1]) / 2 - f.o[2]);
  return { kind: "poly", points: profile, closed: true, ...bare(shape), solid: { kind: "revolved", axisBone: shape.bone, axis: "u", axisX: r4(axisX), axisZ, mirror: true } };
}

function posedGroups(tris: Tri[], ids: ReadonlySet<string>, state: Json, view = "elevation"): Rings[] {
  const arr = (p: Json) => [p.x, p.y, p.z] as V3;
  const out: Rings[] = [];
  for (const t of tris) {
    if (!ids.has(t.id)) continue;
    const f = state.bones[t.bone].frame;
    const fr = { o: arr(f.o), u: arr(f.u), v: arr(f.v), w: arr(f.w) };
    const pts: P[] = [];
    for (let i = 0; i < t.local.length; i += 3) pts.push(projectWorld(view as never, toWorld(fr, [t.local[i]!, t.local[i + 1]!, t.local[i + 2]!])));
    for (let k = 0; k < t.indices.length; k += 3) out.push([[0, 1, 2].map((j) => pts[t.indices[k + j]!]!)]);
  }
  return out;
}

function iou(a: Rings[], b: Rings[]): number {
  const pts = [...a, ...b].flat(2);
  if (pts.length === 0) return 1;
  const minX = Math.min(...pts.map((p) => p.x)), maxX = Math.max(...pts.map((p) => p.x)), minY = Math.min(...pts.map((p) => p.y)), maxY = Math.max(...pts.map((p) => p.y));
  const cell = Math.max(1e-4, Math.max(maxX - minX, maxY - minY) / 160);
  const box = { minX: minX - cell, minY: minY - cell, maxX: maxX + cell, maxY: maxY + cell };
  const ma = new Mask(box, cell), mb = new Mask(box, cell);
  for (const g of a) ma.fill(g);
  for (const g of b) mb.fill(g);
  let i = 0, u = 0;
  for (let k = 0; k < ma.cells.length; k++) { i += ma.cells[k]! & mb.cells[k]!; u += ma.cells[k]! | mb.cells[k]!; }
  return u === 0 ? 1 : i / u;
}

export type Option = { name: string; shape: Json };
export type Item = { index: number; ids: string[]; options: Option[]; core?: number };

export type ElevRow = { scale: string; index: number };

export function elevationPlan(asset0: Json, ctx: Ctx, rows: (ElevRow & Json)[]): Json[] {
  const asset = withRest(asset0);
  const tris = localMeshes(asset);
  const plan: Json[] = [];
  for (const scale of ctx.scales) {
    const mine = rows.filter((r) => Number(r.scale) === scale);
    if (mine.length === 0) continue;
    const list = asset.representations[String(scale)].geometry.elevation as Json[];
    const plates = new Map<number, { shape: Json; source: string }>();
    const thins = new Map<number, { shape: Json; source: string }>();
    const rev = new Map<number, Json>();
    for (const r of mine) {
      plates.set(r.index, plateOf(ctx, scale, list[r.index]));
      thins.set(r.index, plateOf(ctx, scale, list[r.index], true));
      const v = revolvedOf(ctx, scale, list[r.index]);
      if (v !== undefined) rev.set(r.index, v);
    }
    const variant = (pick: (i: number) => Json | undefined): Json => {
      const elevation = list.map((s, i) => (plates.has(i) ? (pick(i) ?? plates.get(i)!.shape) : s));
      return { ...asset, representations: { ...asset.representations, [String(scale)]: { ...asset.representations[String(scale)], geometry: { ...asset.representations[String(scale)].geometry, elevation } } } };
    };
    const assetA = variant(() => undefined);
    const assetC = variant((i) => rev.get(i));
    const assetB = variant((i) => thins.get(i)?.shape);
    const scores = new Map<number, { old: number[]; a: number[]; b: number[]; c: number[] }>();
    for (const r of mine) scores.set(r.index, { old: [], a: [], b: [], c: [] });
    const rest = { kind: "pose", pose: asset.presentation.defaultPose };
    for (const st of statesOf(asset)) {
      const so = evaluate(asset, st.query as never, select(scale)) as Json, sa = evaluate(assetA, st.query as never, select(scale)) as Json, sb = evaluate(assetB, st.query as never, select(scale)) as Json, sc = evaluate(assetC, st.query as never, select(scale)) as Json;
      for (const r of mine) {
        const shape = list[r.index];
        const ids = evidenceIds(ctx, shape);
        if (ids.length === 0) continue;
        const ev = posedGroups(tris, new Set(ids), so);
        const rings = (s: Json): Rings[] => [s.shapes.elevation[r.index].rings as Rings];
        const e = scores.get(r.index)!;
        e.old.push(iou(rings(so), ev));
        e.a.push(iou(rings(sa), ev));
        e.b.push(iou(rings(sb), ev));
        e.c.push(iou(rings(sc), ev));
      }
    }
    void rest;
    const rs = { kind: "pose", pose: "lat-rest" };
    void rs;
    const mean = (x: number[]) => (x.length === 0 ? NaN : x.reduce((p, c) => p + c, 0) / x.length);
    const decided = mine.map((r) => {
      const e = scores.get(r.index)!;
      const shape = list[r.index];
      const old = restRings(asset, scale, "elevation", shape);
      const restC = rev.has(r.index) ? iouRings(old, restRings(asset, scale, "elevation", rev.get(r.index)!)) : 0;
      const useRev = rev.has(r.index) && e.c.length > 0 && restC >= 0.97 && mean(e.c) > Math.max(mean(e.b), mean(e.a)) + 0.01;
      const usePlate = !useRev && e.a.length > 0 && mean(e.a) > mean(e.b) + 0.01;
      const opts: Option[] = [
        { name: "extruded-w-thin", shape: thins.get(r.index)!.shape },
        { name: "extruded-w", shape: plates.get(r.index)!.shape },
        ...(rev.has(r.index) ? [{ name: "revolved", shape: rev.get(r.index)! }] : []),
      ];
      const first = useRev ? "revolved" : usePlate ? "extruded-w" : "extruded-w-thin";
      opts.sort((x, y) => (x.name === first ? -1 : y.name === first ? 1 : 0));
      return { r, e, restC, opts, first };
    });
    const items: Item[] = decided.map((d) => ({ index: d.r.index, ids: [], options: d.opts }));
    const fitted = fitExtents(asset, scale, "elevation", items, items.map(() => 0), items.map((it) => it.options.map(() => 0)));
    decided.forEach((d, n) => {
      const pick = d.opts[fitted[n]!]!;
      plan.push({ ...d.r, source: plates.get(d.r.index)!.source, pick: pick.name + (fitted[n] !== 0 ? " (extent fit)" : ""), restIouRevolved: +d.restC.toFixed(4), posedIou: { old: +mean(d.e.old).toFixed(4), plate: +mean(d.e.a).toFixed(4), thin: +mean(d.e.b).toFixed(4), revolved: rev.has(d.r.index) ? +mean(d.e.c).toFixed(4) : null }, new: pick.shape });
    });
  }
  return plan;
}


/** for every item the option (a candidate shape) with the best mean IoU against the posed evidence solids over all poses and clip instants; items without evidence keep option 0. */
export function posedChoose(asset0: Json, tris: Tri[], scale: number, view: "front" | "plan", items: Item[]): { pick: number; means: (number | null)[] }[] {
  const asset = withRest(asset0);
  const coreOf = (it: Item): number => it.core ?? it.options.length;
  const K = Math.max(...items.map(coreOf));
  const list = asset.representations[String(scale)].geometry[view] as Json[];
  const assets = Array.from({ length: K }, (_, k) => {
    const shapes = list.map((s, i) => { const it = items.find((x) => x.index === i); return it === undefined ? s : it.options[Math.min(k, coreOf(it) - 1)]!.shape; });
    return { ...asset, representations: { ...asset.representations, [String(scale)]: { ...asset.representations[String(scale)], geometry: { ...asset.representations[String(scale)].geometry, [view]: shapes } } } };
  });
  const sums = items.map((it) => it.options.map(() => [] as number[]));
  for (const st of statesOf(asset)) {
    const base = evaluate(asset, st.query as never, select(scale)) as Json;
    const evaluated = assets.map((a) => evaluate(a, st.query as never, select(scale)) as Json);
    items.forEach((it, n) => {
      if (it.ids.length === 0) return;
      const ev = posedGroups(tris, new Set(it.ids), base, view);
      it.options.slice(0, coreOf(it)).forEach((_, k) => sums[n]![k]!.push(iou([evaluated[k]!.shapes[view][it.index].rings as Rings], ev)));
    });
  }
  const mean = (x: number[]) => (x.length === 0 ? null : x.reduce((p, c) => p + c, 0) / x.length);
  return items.map((it, n) => {
    const means = sums[n]!.map(mean).slice(0, coreOf(it));
    let pick = 0;
    means.forEach((m, k) => { if (m !== null && means[pick] !== null && m > means[pick]! + 0.003) pick = k; });
    return { pick, means };
  });
}

const TARGETS: Record<string, [string, string]> = { elevation: ["widthM", "heightM"], front: ["depthM", "heightM"], plan: ["widthM", "depthM"] };
const within = (measured: number, nominal: number): number => Math.max(0, Math.abs(measured - nominal) - Math.max(0.02 * nominal, 0.015));

/** the default-pose extent deviation of a view beyond the validator tolerance (metres, both axes added). */
export function extentExcess(asset: Json, scale: number, view: "elevation" | "front" | "plan"): number {
  const st = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose } as never, select(scale)) as Json;
  const p = (st.shapes[view] as Json[]).filter((s) => s.shape.role !== "ground").flatMap((s) => s.rings.flat());
  if (p.length === 0) return 0;
  const [tx, ty] = TARGETS[view]!;
  const x = Math.max(...p.map((q: Json) => q.x)) - Math.min(...p.map((q: Json) => q.x));
  const y = Math.max(...p.map((q: Json) => q.y)) - Math.min(...p.map((q: Json) => q.y));
  return within(x, asset.dimensions[tx!]) + within(y, asset.dimensions[ty!]);
}

/** swaps options of the items one at a time while the default-pose extent of the view is beyond the validator tolerance: the swap that reduces the excess most (ties: the better posed mean). */
export function fitExtents(asset: Json, scale: number, view: "elevation" | "front" | "plan", items: Item[], picks: number[], means: (number | null)[][]): number[] {
  const list = asset.representations[String(scale)].geometry[view] as Json[];
  const build = (pk: number[]): Json => {
    const shapes = list.map((s, i) => { const n = items.findIndex((x) => x.index === i); return n < 0 ? s : items[n]!.options[pk[n]!]!.shape; });
    return { ...asset, representations: { ...asset.representations, [String(scale)]: { ...asset.representations[String(scale)], geometry: { ...asset.representations[String(scale)].geometry, [view]: shapes } } } };
  };
  const now = [...picks];
  let excess = extentExcess(build(now), scale, view);
  for (let round = 0; round < 12 && excess > 1e-9; round++) {
    let best: { n: number; k: number; excess: number; mean: number } | undefined;
    items.forEach((it, n) => it.options.forEach((_, k) => {
      if (k === now[n]) return;
      const trial = [...now];
      trial[n] = k;
      const e = extentExcess(build(trial), scale, view);
      const m = means[n]![k] ?? -1;
      if (e < excess - 1e-9 && (best === undefined || e < best.excess - 1e-9 || (Math.abs(e - best.excess) <= 1e-9 && m > best.mean))) best = { n, k, excess: e, mean: m };
    }));
    if (best === undefined) break;
    now[best.n] = best.k;
    excess = best.excess;
  }
  return now;
}
