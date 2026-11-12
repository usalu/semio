/** Computes candidate solids for the `lateral-ambiguous` shapes of one asset from the authored volume and the old rest outlines, writes a plan (`🗑️generated/lateral/plan-<slug>.json`, old shape -> new shape per scale, view and index).
 * Usage: bun lateral-convert.ts <slug> [--before <asset.json>] */
import { readFileSync, writeFileSync } from "node:fs";
import { evaluate, flattenGeometry } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { listAmbiguous } from "./lateral-list.ts";
import { elevationPlan, extentExcess, fitExtents, posedChoose, revolvedOf, type Item } from "./lateral-elev.ts";
import { localMeshes } from "./lateral-posed.ts";
import { Mask, catalogEntries, frames, loadAsset, meshes, projectWorld, restState, type Json, type SolidMesh, type View } from "./volume-lib.ts";

type P = { x: number; y: number };
type Rings = P[][];
const r4 = (x: number): number => Math.round(x * 1e4) / 1e4;
const r5 = (x: number): number => Math.round(x * 1e5) / 1e5;

export function withRest(asset: Json): Json {
  if (asset.poses.some((p: Json) => p.id === "lat-rest")) return asset;
  const pose = { id: "lat-rest", title: { en: "Rest", de: "Ruhe" }, description: { en: "Rest", de: "Ruhe" }, bones: {}, ground: "authored", contacts: [], thermal: asset.poses[0].thermal, loads: [] };
  return { ...asset, poses: [...asset.poses, pose] };
}

export function restRings(asset: Json, scale: number, view: string, shape: Json): Rings {
  const rep = asset.representations[String(scale)];
  const geometry = { elevation: [] as Json[], front: [] as Json[], plan: [] as Json[] };
  (geometry as Json)[view] = [shape];
  const mini = { ...withRest(asset), representations: { ...asset.representations, [String(scale)]: { ...rep, geometry } } };
  const state = evaluate(mini, { kind: "pose", pose: "lat-rest" }, { kind: "scale", scale: { denominator: scale } } as never) as Json;
  return (state.shapes[view][0]?.rings ?? []) as Rings;
}

export function iouRings(a: Rings, b: Rings): number {
  const pts = [...a.flat(), ...b.flat()];
  if (pts.length === 0) return 1;
  const minX = Math.min(...pts.map((p) => p.x)) - 1e-3, maxX = Math.max(...pts.map((p) => p.x)) + 1e-3;
  const minY = Math.min(...pts.map((p) => p.y)) - 1e-3, maxY = Math.max(...pts.map((p) => p.y)) + 1e-3;
  const cell = Math.max(maxX - minX, maxY - minY) / 300;
  const ma = new Mask({ minX, minY, maxX, maxY }, cell), mb = new Mask({ minX, minY, maxX, maxY }, cell);
  ma.fill(a);
  mb.fill(b);
  let i = 0, u = 0;
  for (let k = 0; k < ma.cells.length; k++) { i += ma.cells[k]! & mb.cells[k]!; u += ma.cells[k]! | mb.cells[k]!; }
  return u === 0 ? 1 : i / u;
}

const range = (rings: Rings, axis: "x" | "y"): [number, number] => {
  const v = rings.flat().map((p) => p[axis]);
  return v.length === 0 ? [0, 0] : [Math.min(...v), Math.max(...v)];
};

export type Ctx = { asset: Json; frame: ReturnType<typeof frames>; vol: Map<string, Json[]>; zOf: Map<string, [number, number]>; scales: number[]; mesh: Map<string, SolidMesh> };

export function context(asset: Json): Ctx {
  const frame = frames(asset);
  const vol = new Map<string, Json[]>();
  for (const s of asset.volume.solids) (vol.get(s.bone) ?? vol.set(s.bone, []).get(s.bone)!).push(s);
  const zOf = new Map<string, [number, number]>();
  const maxDim = Math.max(asset.dimensions.widthM, asset.dimensions.heightM, asset.dimensions.depthM);
  const mesh = new Map<string, SolidMesh>();
  for (const m of meshes(asset, asset.volume.solids, maxDim)) {
    mesh.set(m.id, m);
    let lo = Infinity, hi = -Infinity;
    for (let i = 2; i < m.world.length; i += 3) { lo = Math.min(lo, m.world[i]!); hi = Math.max(hi, m.world[i]!); }
    zOf.set(m.id, [lo, hi]);
  }
  return { asset, frame, vol, zOf, mesh, scales: Object.keys(asset.representations).map(Number) };
}

/** lateral range of an elevation shape: same-id volume solid, else a same-id front/plan twin, else all volume solids of the bone, else the bone z. */
export function lateralRange(ctx: Ctx, scale: number, shape: Json): { range: [number, number]; source: string } {
  const own = ctx.zOf.get(shape.id);
  if (own !== undefined && (ctx.vol.get(shape.bone) ?? []).some((s) => s.id === shape.id)) return { range: own, source: "volume-id" };
  const geometry = ctx.asset.representations[String(scale)].geometry;
  const front = geometry.front.find((s: Json) => s.id === shape.id && s.bone === shape.bone && s.solid === undefined);
  if (front !== undefined) return { range: range(flattenGeometry(front).rings as Rings, "x"), source: "front-twin" };
  const plan = geometry.plan.find((s: Json) => s.id === shape.id && s.bone === shape.bone && s.solid === undefined);
  if (plan !== undefined) return { range: range(flattenGeometry(plan).rings as Rings, "y"), source: "plan-twin" };
  const solids = (ctx.vol.get(shape.bone) ?? []).filter((s) => s.role !== "texture");
  if (solids.length > 0) {
    const zs = solids.map((s) => ctx.zOf.get(s.id)).filter((z): z is [number, number] => z !== undefined);
    return { range: [Math.min(...zs.map((z) => z[0])), Math.max(...zs.map((z) => z[1]))], source: "volume-bone" };
  }
  const z = ctx.frame[shape.bone]!.o[2];
  return { range: [z - 0.03, z + 0.03], source: "bone-z" };
}

const twinOf = (ctx: Ctx, scale: number, shape: Json): Json | undefined => {
  const area = (s: Json): number => { const r = flattenGeometry(s).rings as Rings; const [a, b] = range(r, "x"); const [c, d] = range(r, "y"); return (b - a) * (d - c); };
  const order = [scale, ...ctx.scales.filter((x) => x !== scale).sort((a, b) => Math.abs(Math.log(a / scale)) - Math.abs(Math.log(b / scale)))];
  for (const sc of order) {
    const list = ctx.asset.representations[String(sc)].geometry.elevation.filter((s: Json) => s.bone === shape.bone && s.kind !== "member");
    const same = list.find((s: Json) => s.id === shape.id);
    if (same !== undefined) return same;
    const main = list.filter((s: Json) => s.role === "body").sort((a: Json, b: Json) => area(b) - area(a))[0];
    if (main !== undefined) return main;
  }
  return undefined;
};

function halfProfile(slices: [number, number][]): number[][] {
  const pts = slices.map(([a, r]) => [r4(a), r4(r)]);
  return [[pts[0]![0]!, 0], ...pts, [pts[pts.length - 1]![0]!, 0]];
}

function revolved(shape: Json, bone: string, profile0: number[][], axisX: number, axisZ: number, vx = 0): Json {
  const profile = axisX === 0 ? profile0 : profile0.map(([a, r]) => [a!, r4(r! + axisX * vx)]);
  return { kind: "poly", points: profile, closed: true, ...withoutGeometry(shape), solid: { kind: "revolved", axisBone: bone, axis: "u", axisX: r4(axisX), axisZ: r4(axisZ), mirror: true } };
}

export const RIBBON_M = 0.002;

/** the closed ribbon polygon of an open polyline, `width` wide, mitre-free (vertex normals averaged). */
export function ribbon(line: P[], width = RIBBON_M): P[] {
  const pts = line.filter((p, i) => i === 0 || Math.hypot(p.x - line[i - 1]!.x, p.y - line[i - 1]!.y) > 1e-6);
  if (pts.length < 2) return [];
  const h = width / 2;
  const normal = (i: number): P => {
    const prev = pts[Math.max(0, i - 1)]!, next = pts[Math.min(pts.length - 1, i + 1)]!;
    const dx = next.x - prev.x, dy = next.y - prev.y, l = Math.hypot(dx, dy) || 1;
    return { x: -dy / l, y: dx / l };
  };
  const left = pts.map((p, i) => ({ x: p.x + normal(i).x * h, y: p.y + normal(i).y * h }));
  const right = pts.map((p, i) => ({ x: p.x - normal(i).x * h, y: p.y - normal(i).y * h })).reverse();
  return [...left, ...right].map((p) => ({ x: r4(p.x), y: r4(p.y) }));
}

/** radial profile of an outline: half the extent across the lateral axis `across` as function of the coordinate `along`. */
function envelope(rings: Rings, along: "x" | "y", across: "x" | "y", slices: number): { a0: number; a1: number; centre: number; r: [number, number][] } {
  const [lo, hi] = range(rings, along);
  const [clo, chi] = range(rings, across);
  const centre = (clo + chi) / 2;
  const out: [number, number][] = [];
  const edges = rings.flatMap((ring) => ring.map((p, i) => [p, ring[(i + 1) % ring.length]!] as const));
  for (let i = 0; i < slices; i++) {
    const s = lo + ((hi - lo) * (i + 0.5)) / slices;
    let m = 0;
    for (const [p, q] of edges) {
      const pa = p[along], qa = q[along];
      if ((pa - s) * (qa - s) > 0 || pa === qa) continue;
      const c = p[across] + ((q[across] - p[across]) * (s - pa)) / (qa - pa);
      m = Math.max(m, Math.abs(c - centre));
    }
    out.push([s, m]);
  }
  return { a0: lo, a1: hi, centre, r: out };
}

/** the projection of volume solids to a view as one triangle group per triangle. */
export function volumeGroups(ctx: Ctx, ids: readonly string[], view: View): Rings[] {
  const out: Rings[] = [];
  for (const id of ids) {
    const m = ctx.mesh.get(id);
    if (m === undefined) continue;
    for (let t = 0; t < m.indices.length; t += 3) {
      const tri = [0, 1, 2].map((k) => { const i = m.indices[t + k]! * 3; return projectWorld(view, [m.world[i]!, m.world[i + 1]!, m.world[i + 2]!]); });
      out.push([tri]);
    }
  }
  return out;
}

export function iouGroups(a: Rings[], b: Rings[]): number {
  const pts = [...a, ...b].flat(2);
  if (pts.length === 0) return 1;
  const minX = Math.min(...pts.map((p) => p.x)) - 1e-3, maxX = Math.max(...pts.map((p) => p.x)) + 1e-3;
  const minY = Math.min(...pts.map((p) => p.y)) - 1e-3, maxY = Math.max(...pts.map((p) => p.y)) + 1e-3;
  const cell = Math.max(maxX - minX, maxY - minY) / 300;
  const box = { minX, minY, maxX, maxY };
  const ma = new Mask(box, cell), mb = new Mask(box, cell);
  for (const g of a) ma.fill(g);
  for (const g of b) mb.fill(g);
  let i = 0, u = 0;
  for (let k = 0; k < ma.cells.length; k++) { i += ma.cells[k]! & mb.cells[k]!; u += ma.cells[k]! | mb.cells[k]!; }
  return u === 0 ? 1 : i / u;
}

/** the volume solids that are the evidence of a shape: the same-id solid, else (body shapes) every non-texture solid of the bone when that is a single solid. */
export function evidenceIds(ctx: Ctx, shape: Json): string[] {
  const own = ctx.vol.get(shape.bone) ?? [];
  if (own.some((s) => s.id === shape.id)) return [shape.id];
  if (ctx.asset.volume.solids.some((s: Json) => s.id === shape.id)) return [shape.id];
  const main = own.filter((s) => s.role !== "texture" && s.role !== "detail");
  return shape.role === "body" && main.length > 0 ? main.map((x) => x.id) : [];
}

export type Cand = { name: string; shape: Json };

const GEOMETRY_KEYS = ["kind", "points", "cx", "cy", "rx", "ry", "rotationDeg", "x", "y", "w", "h", "x0", "y0", "x1", "y1", "r0", "r1", "r", "d", "followTip", "closed"];

function withoutGeometry(shape: Json): Json {
  const copy: Json = { ...shape };
  for (const k of GEOMETRY_KEYS) delete copy[k];
  return copy;
}

/** candidate solids for a flat front or plan shape; `sources` are the reference outlines (old outline, volume projection) the profile is measured from. */
export function candidates(ctx: Ctx, scale: number, view: "front" | "plan", shape: Json, sources: { name: string; rings: Rings }[], open = false): Cand[] {
  const f = ctx.frame[shape.bone]!;
  const out: Cand[] = [];
  const along = view === "front" ? "y" : "x";
  const across = view === "front" ? "x" : "y";
  const oAlong = view === "front" ? f.o[1] : f.o[0];
  const oAcross = f.o[2];
  const uAlong = view === "front" ? f.u[1] : f.u[0];
  const lengthM = ctx.asset.rig.bones.find((b: Json) => b.id === shape.bone).lengthM as number;
  const tw = twinOf(ctx, scale, shape);
  const twRings = tw === undefined ? [] : (flattenGeometry(tw).rings as Rings);
  const twLength = twRings.length === 0 ? lengthM : Math.max(lengthM * 0.25, Math.max(...twRings.flat().map((p) => p.x)) - Math.min(0, ...twRings.flat().map((p) => p.x)));
  if (open) {
    const src = sources[0]!;
    const [clo, chi] = range(src.rings, across);
    const [alo, ahi] = range(src.rings, along);
    if (Math.abs(uAlong) >= 0.3 && src.rings.length > 0) {
      const ring = src.rings.reduce((big, r) => (r.length > big.length ? r : big), src.rings[0]!);
      const profile = ring.map((p) => [r4(((view === "front" ? p.y : p.x) - oAlong) / uAlong), r4(p[across] - oAcross)]);
      const twv = tw === undefined ? [] : (flattenGeometry(tw).rings as Rings).flat().map((p) => p.y);
      const mid = twv.length === 0 ? 0 : (Math.min(...twv) + Math.max(...twv)) / 2;
      out.push({ name: "extruded-v-plate(old)", shape: { kind: "poly", points: profile, closed: true, ...withoutGeometry(shape), solid: { kind: "extruded", sweep: "v", z0: r4(mid - 0.003), z1: r4(mid + 0.003) } } });
    }
    const vAlong = view === "front" ? f.v[1] : f.v[0];
    const thinAcross = chi - clo <= ahi - alo;
    const [bu, bv] = Math.abs(uAlong) < 0.3 && Math.abs(vAlong) >= 0.3 ? [lengthM / 2, ((alo + ahi) / 2 - oAlong) / vAlong] : [lengthM / 2, 0];
    const half = Math.max(0.001, (ahi - alo) / 2 / Math.max(Math.abs(vAlong), 0.3));
    const [z0, z1] = thinAcross ? [(clo + chi) / 2 - 0.001, (clo + chi) / 2 + 0.001] : [clo, chi];
    out.push({ name: "extruded-w-bar(old)", shape: { kind: "poly", points: [[r4(bu - 0.001), r4(bv - half)], [r4(bu + 0.001), r4(bv - half)], [r4(bu + 0.001), r4(bv + half)], [r4(bu - 0.001), r4(bv + half)]], closed: true, ...withoutGeometry(shape), solid: { kind: "extruded", sweep: "w", z0: r4(z0), z1: r4(z1) } } });
    return out;
  }
  for (const src of sources) {
    const [clo, chi] = range(src.rings, across);
    const [alo, ahi] = range(src.rings, along);
    const halfAcross = (chi - clo) / 2;
    const centreAcross = (clo + chi) / 2;
    if (Math.abs(uAlong) >= 0.3 && ahi > alo) {
      const e = envelope(src.rings, along, across, 24);
      const slices = e.r.map(([a, r]) => [(a - oAlong) / uAlong, Math.max(r, 0.0005)] as [number, number]);
      if (uAlong < 0) slices.reverse();
      out.push({ name: `revolved-outline(${src.name})`, shape: revolved(shape, shape.bone, halfProfile(slices), 0, e.centre - oAcross) });
    }
    const aligned = Math.abs(uAlong) >= 0.3 && ahi > alo;
    const [d0, d1] = aligned ? [(alo - oAlong) / uAlong, (ahi - oAlong) / uAlong].sort((x, y) => x - y) : [0, twLength];
    if (halfAcross > 0)
      out.push({ name: `revolved-disc(${src.name})`, shape: revolved(shape, shape.bone, halfProfile([[d0, halfAcross], [Math.max(d1, d0 + 0.002), halfAcross]]), view === "plan" && Math.abs(f.u[0]) < 0.05 ? (alo + ahi) / 2 - oAlong : 0, centreAcross - oAcross, f.v[0]) });
    const halfAlong = (ahi - alo) / 2;
    if (halfAcross > 0 && halfAlong > 0)
      out.push({ name: `revolved-disc-mean(${src.name})`, shape: revolved(shape, shape.bone, halfProfile([[d0, Math.sqrt(halfAcross * halfAlong)], [Math.max(d1, d0 + 0.002), Math.sqrt(halfAcross * halfAlong)]]), view === "plan" && Math.abs(f.u[0]) < 0.05 ? (alo + ahi) / 2 - oAlong : 0, centreAcross - oAcross, f.v[0]) });
    if (tw !== undefined && halfAcross > 0)
      for (const k of [1, 0.85, 0.72]) {
        const rv = revolvedOf(ctx, scale, tw, { axisX: view === "plan" && Math.abs(f.u[0]) < 0.05 ? (alo + ahi) / 2 - oAlong : 0, axisZ: r4(centreAcross - oAcross), k });
        if (rv !== undefined) out.push({ name: `revolved-twin${k === 1 ? "" : "-" + k}(${src.name})`, shape: { kind: "poly", points: rv.points, closed: true, ...withoutGeometry(shape), solid: rv.solid } });
      }
    if (Math.abs(uAlong) >= 0.3 && src.rings.length > 0) {
      const ring = src.rings.length === 1 ? src.rings[0]! : src.rings.reduce((big, r) => (r.length > big.length ? r : big), src.rings[0]!);
      const profile = ring.map((p) => [r4(((view === "front" ? p.y : p.x) - oAlong) / uAlong), r4(p[across] - oAcross)]);
      const twv = tw === undefined ? [] : (flattenGeometry(tw).rings as Rings).flat().map((p) => p.y);
      const vlo = twv.length === 0 ? -0.02 : Math.min(...twv), vhi = twv.length === 0 ? 0.02 : Math.max(...twv);
      const thin = (alo === ahi) || (src.rings.flat().length > 0 && Math.min(chi - clo, ahi - alo) < 2.5 * RIBBON_M);
      const [z0, z1] = thin ? [(vlo + vhi) / 2 - 0.003, (vlo + vhi) / 2 + 0.003] : [vlo, vhi];
      out.push({ name: `extruded-v-plate(${src.name})`, shape: { kind: "poly", points: profile, closed: true, ...withoutGeometry(shape), solid: { kind: "extruded", sweep: "v", z0: r4(z0), z1: r4(z1) } } });
    }
    if (tw !== undefined && halfAcross > 0)
      for (const k of [1, 0.9, 0.8]) {
        const base = withoutGeometry(shape);
        const geom: Json = {};
        for (const key of ["kind", "points", "cx", "cy", "rx", "ry", "rotationDeg", "x", "y", "w", "h", "r", "d", "closed"]) if (key in tw) geom[key] = (tw as Json)[key];
        out.push({ name: `extruded-w-twin${k === 1 ? "" : "-" + k}(${src.name})`, shape: { ...geom, ...base, solid: { kind: "extruded", sweep: "w", z0: r4(centreAcross - halfAcross * k), z1: r4(centreAcross + halfAcross * k) } } });
      }
  }
  return out;
}

function coverage(box: { minX: number; minY: number; maxX: number; maxY: number }, cell: number, groups: Rings[]): Mask {
  const m = new Mask(box, cell);
  for (const g of groups) m.fill(g);
  return m;
}

const AUTHORED = new Set(["body", "detail", "glazing"]);

if (import.meta.main) {
  const slug = process.argv[2]!;
  const beforeIdx = process.argv.indexOf("--before");
  const asset = beforeIdx > 0 ? (JSON.parse(readFileSync(process.argv[beforeIdx + 1]!, "utf8")).asset as Json) : loadAsset(catalogEntries().find((x) => x.slug === slug)!.path);
  const ctx = context(asset);
  const tris = localMeshes(asset);
  const rows = listAmbiguous(asset).filter((r: Json) => r.flagged);
  const plan: Json[] = [];
  const summary: string[] = [];
  plan.push(...elevationPlan(asset, ctx, rows.filter((x: Json) => x.view === "elevation")));
  for (const scale of ctx.scales) {
    const st = restState(asset, scale) as Json;
    for (const view of ["front", "plan"] as const) {
      const entries = rows.filter((x: Json) => Number(x.scale) === scale && x.view === view).sort((a: Json, b: Json) => a.index - b.index);
      if (entries.length === 0) continue;
      const authored = (st[view] as Json[]).filter((s) => s.closed && AUTHORED.has(s.shape.role) && s.rings.length > 0);
      const volGroups = volumeGroups(ctx, [...ctx.mesh.keys()], view);
      const pts = [...authored.flatMap((s) => s.rings.flat()), ...volGroups.flat(2)] as P[];
      const minX = Math.min(...pts.map((p) => p.x)), maxX = Math.max(...pts.map((p) => p.x)), minY = Math.min(...pts.map((p) => p.y)), maxY = Math.max(...pts.map((p) => p.y));
      const cell = Math.max(1e-4, Math.max(maxX - minX, maxY - minY) / 640);
      const box = { minX: minX - 2 * cell, minY: minY - 2 * cell, maxX: maxX + 2 * cell, maxY: maxY + 2 * cell };
      const V = coverage(box, cell, volGroups);
      const count = new Int32Array(V.cells.length);
      const add = (m: Mask, sign: number): void => { for (let i = 0; i < m.cells.length; i++) count[i]! += sign * m.cells[i]!; };
      const maskOf = (rings: Rings): Mask => coverage(box, cell, [rings]);
      for (const s of authored) add(maskOf(s.rings as Rings), 1);
      const score = (): number => { let both = 0, either = 0; for (let i = 0; i < count.length; i++) { const a = count[i]! > 0 ? 1 : 0; both += a & V.cells[i]!; either += a | V.cells[i]!; } return either === 0 ? 1 : both / either; };
      const start = score();
      const pending: Item[] = [];
      const oldMasks: Mask[] = [];
      for (const r of entries) {
        const shape = asset.representations[r.scale].geometry[view][r.index];
        const open = (flattenGeometry(shape) as Json).closed === false;
        const line = restRings(asset, scale, view, shape);
        const old = open ? line.map((l) => ribbon(l)).filter((l) => l.length > 2) : line;
        const oldMask = open ? maskOf(line) : maskOf(old);
        const ids = evidenceIds(ctx, shape);
        const sources = [{ name: "old", rings: old }, ...(ids.length > 0 ? [{ name: "volume", rings: volumeGroups(ctx, ids, view).flat() }] : [])];
        const cands = candidates(ctx, scale, view, shape, sources, open);
        add(oldMask, -1);
        const scored = cands.map((c) => { const got = restRings(asset, scale, view, c.shape); const m = maskOf(got); add(m, 1); const total = score(); add(m, -1); return { ...c, total, iouOld: iouRings(old, got), mask: m }; });
        const bestOld = Math.max(...scored.map((c) => c.iouOld));
        const pool = scored.filter((c) => c.iouOld >= bestOld - (view === "plan" ? 0.45 : 0.1));
        const bonus = (c: { name: string; iouOld: number }): number => (open ? (c.name.startsWith("extruded-v-plate") ? 8e-4 : 0) : c.name.startsWith("revolved-") && c.iouOld >= 0.85 ? 8e-4 : 0);
        const best = pool.sort((a, b) => b.total + bonus(b) - (a.total + bonus(a)) || b.iouOld - a.iouOld)[0]!;
        add(best.mask, 1);
        if (scored.length === 0) throw new Error(`no candidate for ${scale} ${view} ${r.index} ${shape.id} (${shape.bone}, ${shape.kind})`);
        const ranked = [...pool].sort((a, b) => b.total + bonus(b) - (a.total + bonus(a)) || b.iouOld - a.iouOld).slice(0, 9);
        if (!ranked.some((c) => c.name === best.name)) ranked.push(best);
        const extras = scored.filter((c) => c.iouOld >= 0.3 && !ranked.some((x) => x.name === c.name)).sort((a, b) => b.iouOld - a.iouOld);
        oldMasks.push(oldMask);
        pending.push({ index: r.index, ids, core: ranked.length, options: [...ranked, ...extras].map((c) => ({ name: c.name, shape: c.shape })) });
        plan.push({ ...r, evidence: ids.join(",") || "old-outline", candidates: scored.map((c) => ({ name: c.name, total: r5(c.total), iouOld: r5(c.iouOld) })), pick: best.name, new: best.shape });
      }
      const chosen = posedChoose(asset, tris, scale, view, pending);
      let fitted = fitExtents(asset, scale, view, pending, chosen.map((c) => c.pick), chosen.map((c) => c.means));
      {
        const cnt = new Int32Array(V.cells.length);
        const addTo = (m: Mask, sign: number): void => { for (let i = 0; i < m.cells.length; i++) cnt[i]! += sign * m.cells[i]!; };
        const scoreNow = (): number => { let both = 0, either = 0; for (let i = 0; i < cnt.length; i++) { const a = cnt[i]! > 0 ? 1 : 0; both += a & V.cells[i]!; either += a | V.cells[i]!; } return either === 0 ? 1 : both / either; };
        for (const s of authored) addTo(maskOf(s.rings as Rings), 1);
        const optMasks = pending.map((it) => it.options.map((o) => maskOf(restRings(asset, scale, view, o.shape))));
        pending.forEach((_, n) => { addTo(oldMasks[n]!, -1); addTo(optMasks[n]![fitted[n]!]!, 1); });
        const base = (n: number): number => { const k = fitted[n]!; const m = chosen[n]!.means[k]; return m ?? -1; };
        for (let n = 0; n < pending.length; n++) {
          addTo(optMasks[n]![fitted[n]!]!, -1);
          const here = scoreWith(n, fitted[n]!);
          const order = pending[n]!.options.slice(0, pending[n]!.core ?? pending[n]!.options.length).map((_, k) => ({ k, t: scoreWith(n, k) })).filter((c) => c.k !== fitted[n] && c.t > here + 2e-4).sort((a, b) => b.t - a.t);
          let take = fitted[n]!;
          for (const c of order) {
            const m = chosen[n]!.means[c.k];
            if (m !== undefined && m !== null && base(n) >= 0 && m < base(n) - 0.02) continue;
            const trial = [...fitted]; trial[n] = c.k;
            const list = asset.representations[String(scale)].geometry[view] as Json[];
            const probe = { ...asset, representations: { ...asset.representations, [String(scale)]: { ...asset.representations[String(scale)], geometry: { ...asset.representations[String(scale)].geometry, [view]: list.map((sh, i) => { const q = pending.findIndex((x) => x.index === i); return q < 0 ? sh : pending[q]!.options[trial[q]!]!.shape; }) } } } };
            if (extentExcess(probe, scale, view) > 1e-9) continue;
            take = c.k;
            break;
          }
          fitted[n] = take;
          addTo(optMasks[n]![take]!, 1);
        }
        function scoreWith(n: number, k: number): number { addTo(optMasks[n]![k]!, 1); const v = scoreNow(); addTo(optMasks[n]![k]!, -1); return v; }
      }
      pending.forEach((it, n) => {
        const entry = plan.find((p) => p.scale === String(scale) && p.view === view && p.index === it.index)!;
        entry.posedMeans = chosen[n]!.means.map((m, k) => `${it.options[k]!.name}:${m === null ? "-" : m.toFixed(4)}`);
        entry.pick = it.options[fitted[n]!]!.name + (fitted[n] !== chosen[n]!.pick ? " (refined)" : "");
        entry.new = it.options[fitted[n]!]!.shape;
      });
      summary.push(`1:${scale} ${view} volume IoU (own estimate) ${start.toFixed(4)} -> ${score().toFixed(4)}`);
    }
  }
  writeFileSync(new URL(`./🗑️generated/lateral/plan-${slug}.json`, import.meta.url), JSON.stringify(plan, null, 1));
  for (const p of plan) console.log(p.scale, p.view, p.index, p.id, p.bone, p.source ?? p.pick, p.candidates ? "[" + p.evidence + "] " + p.candidates.map((c: Json) => `${c.name}:${c.total}/${c.iouOld}`).join(" ") : "");
  console.log(summary.join("\n"));
}
