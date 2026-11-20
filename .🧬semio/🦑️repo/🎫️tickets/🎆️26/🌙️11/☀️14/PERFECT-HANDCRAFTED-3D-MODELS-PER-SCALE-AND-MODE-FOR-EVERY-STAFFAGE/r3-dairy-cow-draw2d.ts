/** 🖊️ Round 3 dairy cow, ruling E1: redraws the 2D shapes of every scale and view as the true orthographic view of that scale's handcrafted body in the default pose, shape by shape (same ids, roles, bones and layers), and measures the dimensions.
 *
 * Every filled shape is the outline of the solids it stands for (`group` of the solid in 🗑️generated/r3-dairy-cow/groups.json, else its bone), traced on a fine raster and reduced to an economical polygon. Coat patches are decals: their shape is the part of the patch that is the nearest surface in that view. Elevation shapes are written in the frame of their bone; front and plan shapes in rest coordinates that the product's own placement carries onto the outline (found by iterating the product evaluation). Detail and texture strokes are stated by hand below.
 *
 * Usage: bun r3-dairy-cow-draw2d.ts   → 🗑️generated/r3-dairy-cow/members.json (for `python3 r3-dairy-cow-root.py`)
 * @see ./round3-brief.md */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { GENERATED, entryOf, loadSource, merged, meshesAt, poseState, type Json } from "./volumes-lib.ts";

type P = [number, number];
type Tri = { p: [P, P, P]; d: [number, number, number]; own: boolean };
const VIEWS = ["elevation", "front", "plan"] as const;
type View = (typeof VIEWS)[number];
const CELL: Record<number, number> = { 50: 0.0025, 100: 0.004, 200: 0.006, 500: 0.012 };
const TOL: Record<number, number> = { 50: 0.003, 100: 0.005, 200: 0.008, 500: 0.018 };
const GEOMETRY_KEYS = new Set(["kind", "points", "closed", "solid", "followTip", "x0", "y0", "x1", "y1", "r0", "r1", "cx", "cy", "rx", "ry", "rotationDeg", "d", "x", "y", "width", "height"]);
const ALIAS: Record<string, string> = { eye: "near-eye", "eye-n": "near-eye", "eye-f": "far-eye", "teat-front": "teat-front-near", "teat-rear": "teat-rear-near", "teat-near": "teat-front-near", "teat-far": "teat-front-far", "nostril-n": "near-nostril", "nostril-f": "far-nostril", "hoof-hf": "far-hind-hoof", "hoof-ff": "far-fore-hoof", "hoof-hn": "near-hind-hoof", "hoof-fn": "near-fore-hoof" };
const FALLBACK: Record<string, string> = { pelvis: "lumbar", jaw: "head", "ear-n": "head", "ear-f": "head", "tail-1": "tail-2", "tail-3": "tail-2" };
for (const s of ["f", "n"]) Object.assign(FALLBACK, { [`scapula-f${s}`]: `humerus-f${s}`, [`hoof-h${s}`]: `metatarsus-h${s}`, [`hoof-f${s}`]: `cannon-f${s}`, [`metatarsus-h${s}`]: `shank-h${s}`, [`cannon-f${s}`]: `forearm-f${s}` });

/** 🧮️ The outlines of the cells where a triangle marked `own` is the nearest one (smallest depth), on a raster of `cell` metres, reduced with the tolerance `tol`. */
function outline(tris: readonly Tri[], cell: number, tol: number): P[][] {
  let [x0, y0, x1, y1] = [Infinity, Infinity, -Infinity, -Infinity];
  for (const t of tris) for (const [x, y] of t.p) { x0 = Math.min(x0, x); y0 = Math.min(y0, y); x1 = Math.max(x1, x); y1 = Math.max(y1, y); }
  const ox = x0 - 2 * cell;
  const oy = y0 - 2 * cell;
  const w = Math.ceil((x1 - x0) / cell) + 4;
  const h = Math.ceil((y1 - y0) / cell) + 4;
  const g = new Uint8Array(w * h);
  const depth = new Float64Array(w * h).fill(Infinity);
  for (const { p: [a, b, c], d: [da, db, dc], own } of tris) {
    const d = (b[0] - a[0]) * (c[1] - a[1]) - (c[0] - a[0]) * (b[1] - a[1]);
    if (Math.abs(d) < 1e-14) continue;
    const i0 = Math.max(0, Math.floor((Math.min(a[0], b[0], c[0]) - ox) / cell));
    const i1 = Math.min(w - 1, Math.ceil((Math.max(a[0], b[0], c[0]) - ox) / cell));
    const j0 = Math.max(0, Math.floor((Math.min(a[1], b[1], c[1]) - oy) / cell));
    const j1 = Math.min(h - 1, Math.ceil((Math.max(a[1], b[1], c[1]) - oy) / cell));
    for (let j = j0; j <= j1; j++) for (let i = i0; i <= i1; i++) {
      const px = ox + (i + 0.5) * cell;
      const py = oy + (j + 0.5) * cell;
      const u = ((b[0] - a[0]) * (py - a[1]) - (px - a[0]) * (b[1] - a[1])) / d;
      const v = ((px - a[0]) * (c[1] - a[1]) - (c[0] - a[0]) * (py - a[1])) / d;
      if (u < -1e-9 || v < -1e-9 || u + v > 1 + 1e-9) continue;
      const z = da + v * (db - da) + u * (dc - da);
      if (z < depth[j * w + i]!) { depth[j * w + i] = z; g[j * w + i] = own ? 1 : 0; }
    }
  }
  const at = (i: number, j: number): number => (i < 0 || j < 0 || i >= w || j >= h ? 0 : g[j * w + i]!);
  const next = new Map<number, number[]>();
  const key = (i: number, j: number): number => j * (w + 1) + i;
  const add = (a: number, b: number): void => { const l = next.get(a); if (l) l.push(b); else next.set(a, [b]); };
  for (let j = 0; j < h; j++) for (let i = 0; i < w; i++) if (at(i, j)) {
    if (!at(i, j - 1)) add(key(i, j), key(i + 1, j));
    if (!at(i + 1, j)) add(key(i + 1, j), key(i + 1, j + 1));
    if (!at(i, j + 1)) add(key(i + 1, j + 1), key(i, j + 1));
    if (!at(i - 1, j)) add(key(i, j + 1), key(i, j));
  }
  const loops: P[][] = [];
  for (const start of [...next.keys()]) {
    while ((next.get(start)?.length ?? 0) > 0) {
      const loop: P[] = [];
      let cur = start;
      for (;;) {
        const l = next.get(cur);
        if (!l || l.length === 0) break;
        const to = l.pop()!;
        loop.push([ox + (cur % (w + 1)) * cell, oy + Math.floor(cur / (w + 1)) * cell]);
        cur = to;
        if (cur === start) break;
      }
      loops.push(loop);
    }
  }
  const area = (r: readonly P[]): number => r.reduce((s, p, i) => s + p[0] * r[(i + 1) % r.length]![1] - r[(i + 1) % r.length]![0] * p[1], 0) / 2;
  const outer = loops.filter((r) => area(r) > 12 * cell * cell).sort((a, b) => area(b) - area(a));
  return outer.map((r) => {
    const mid: P[] = r.map((p, i) => [(p[0] + r[(i + 1) % r.length]![0]) / 2, (p[1] + r[(i + 1) % r.length]![1]) / 2]);
    let far = 0;
    for (let i = 1; i < mid.length; i++) if (Math.hypot(mid[i]![0] - mid[0]![0], mid[i]![1] - mid[0]![1]) > Math.hypot(mid[far]![0] - mid[0]![0], mid[far]![1] - mid[0]![1])) far = i;
    const dp = (pts: P[]): P[] => {
      if (pts.length < 3) return pts;
      const [a, b] = [pts[0]!, pts[pts.length - 1]!];
      const len = Math.hypot(b[0] - a[0], b[1] - a[1]) || 1e-12;
      let worst = 0;
      let k = 0;
      for (let i = 1; i < pts.length - 1; i++) {
        const dist = Math.abs((b[0] - a[0]) * (a[1] - pts[i]![1]) - (a[0] - pts[i]![0]) * (b[1] - a[1])) / len;
        if (dist > worst) { worst = dist; k = i; }
      }
      return worst <= tol ? [a, b] : [...dp(pts.slice(0, k + 1)).slice(0, -1), ...dp(pts.slice(k))];
    };
    const one = dp(mid.slice(0, far + 1));
    const two = dp([...mid.slice(far), mid[0]!]);
    return [...one.slice(0, -1), ...two.slice(0, -1)];
  });
}

const { entry, asset: catalog } = entryOf("animals/dairy-cow");
const asset = merged(catalog, loadSource(entry)!);
const groups = JSON.parse(readFileSync(join(GENERATED, "r3-dairy-cow", "groups.json"), "utf8"));
const parent: Record<string, string | null> = Object.fromEntries((asset.rig.bones as Json[]).map((b) => [b.id, b.parent]));
const round = (v: number): number => Math.round(v * 10000) / 10000;

/** ✏️ Hand-drawn strokes: [bone-local points of the elevation] per scale and shape id; `world: true` rows are (x, height) of the default pose and are carried into the bone frame. */
const STROKES: Record<number, Record<string, { points: P[]; world?: boolean }>> = {
  50: {
    nostril: { points: [[0.502, 0.0], [0.516, 0.014], [0.53, 0.006]] },
    "mouth-line": { points: [[0.515, -0.078], [0.44, -0.074], [0.36, -0.07], [0.31, -0.076]] },
    "hoof-cleft-h": { points: [[0.058, -0.03], [0.078, 0.012], [0.105, 0.04]] },
    "hoof-cleft-f": { points: [[0.058, -0.03], [0.078, 0.012], [0.105, 0.04]] },
    "knee-line": { points: [[0.35, -0.042], [0.356, 0.0], [0.35, 0.044]] },
    "hair-1": { points: [[1.12, 1.2], [1.16, 1.08], [1.17, 0.96]], world: true },
    "hair-2": { points: [[0.36, 1.26], [0.43, 1.14], [0.45, 1.02]], world: true },
    "neck-fold": { points: [[1.52, 1.22], [1.58, 1.1], [1.62, 1.0]], world: true },
  },
  100: { "hip-line": { points: [[0.2, 1.37], [0.27, 1.22], [0.3, 1.08]], world: true } },
  200: {},
  500: {},
};

const representations: Json = JSON.parse(JSON.stringify(asset.representations));
if (!(representations["200"].geometry.elevation as Json[]).some((s) => s.id === "patch-barrel")) {
  for (const view of VIEWS) {
    const from: Json[] = representations["100"].geometry[view];
    const to: Json[] = representations["200"].geometry[view];
    for (const [id, bone] of [["patch-rump", "lumbar"], ["patch-barrel", "thorax"]] as const) {
      const model = from.find((s) => s.id === id);
      const host = to.findIndex((s) => s.bone === bone && s.filled === true);
      if (!model || host < 0) continue;
      const copy: Json = JSON.parse(JSON.stringify(model));
      copy.z = Math.round((to[host]!.z + 0.05) * 100) / 100;
      to.splice(to.findLastIndex((s) => s.bone === "lumbar" || s.bone === "thorax") + 1, 0, copy);
    }
  }
}
for (const s of representations["200"].geometry.elevation as Json[]) if (s.id === "mass-200-lumbar") s.z = 3;
let dims: Json = {};
for (const scale of [50, 100, 200, 500]) {
  const meta: Record<string, Json> = groups[String(scale)].body;
  const body: Json[] = asset.volumes[String(scale)].body;
  const flank: Json[] = (asset.volumes[String(scale)].thermal as Json[]).filter((s) => /rumen-flank/.test(s.id) && s.kind);
  const world = meshesAt(asset, scale, [...body, ...flank]);
  const meshOf = new Map(world.map((m) => [m.id, m]));
  const frames = poseState(asset, scale).frames as Record<string, { o: number[]; u: number[]; v: number[] }>;
  const groupOf = (x: Json): string => (x.id.startsWith("th-") ? "rumen-flank" : meta[x.id]?.group ?? x.bone);
  const decal = (x: Json): boolean => x.id.startsWith("th-") || meta[x.id]?.decal === true;
  if (scale === 50) {
    const lo = [Infinity, Infinity, Infinity];
    const hi = [-Infinity, -Infinity, -Infinity];
    for (const m of world) for (let i = 0; i < m.world.length; i++) { lo[i % 3] = Math.min(lo[i % 3]!, m.world[i]!); hi[i % 3] = Math.max(hi[i % 3]!, m.world[i]!); }
    dims = { heightM: Math.round(hi[1]! * 100) / 100, widthM: Math.round((hi[0]! - lo[0]!) * 100) / 100, depthM: Math.round((hi[2]! - lo[2]!) * 100) / 100, massKg: asset.dimensions.massKg };
    console.log("measured", JSON.stringify(dims), "ground", lo[1]!.toFixed(4));
  }
  const rep = representations[String(scale)];
  const targets: Record<string, Map<string, P[]>> = {};
  for (const view of VIEWS) {
    let shapes: Json[] = rep.geometry[view];
    const all = [...body, ...flank];
    const claimed = new Map<string, Json[]>(shapes.map((s) => [s.id, []]));
    const used = new Set<string>();
    const give = (s: Json, list: Json[]): void => { for (const x of list) if (!used.has(x.id)) { used.add(x.id); claimed.get(s.id)!.push(x); } };
    const wanted = (s: Json): string => ALIAS[s.id] ?? s.id.replace(/^mass-\d+-/, "");
    const traced = shapes.filter((s) => s.kind !== "member" && (s.filled === true || /^nostril-[nf]$/.test(s.id) || /-(hind|fore)-leg$/.test(s.id) || (s.id === "eye" && s.role === "detail")));
    for (const s of traced) give(s, all.filter((x) => groupOf(x) === wanted(s)));
    for (const s of traced) if (claimed.get(s.id)!.length === 0) give(s, body.filter((x) => x.bone === s.bone && !decal(x)));
    for (const x of body) {
      if (used.has(x.id) || x.texture === true) continue;
      let b: string | null | undefined = x.bone;
      let target: Json | undefined;
      const seen = new Set<string>();
      while (b && !seen.has(b)) {
        seen.add(b);
        target = traced.find((s) => s.bone === b && s.role === "body" && !claimed.get(s.id)!.some(decal) && claimed.get(s.id)!.length > 0);
        if (target) break;
        b = FALLBACK[b] ?? parent[b];
      }
      if (target) give(target, [x]);
      else console.log(`  1:${scale} ${view}: solid ${x.id} has no shape`);
    }
    targets[view] = new Map();
    const dropped: string[] = [];
    shapes = shapes.map((s) => {
      const stroke = view === "elevation" ? STROKES[scale]![s.id] : undefined;
      const f = frames[s.bone]!;
      const local = (X: number, Y: number): P => { const d = [X - f.o[0]!, Y - f.o[1]!]; return [d[0]! * f.u[0]! + d[1]! * f.u[1]!, d[0]! * f.v[0]! + d[1]! * f.v[1]!]; };
      const next: Json = {};
      for (const [k, v] of Object.entries(s)) if (!GEOMETRY_KEYS.has(k)) next[k] = v;
      if (stroke) {
        next.kind = "poly";
        next.points = stroke.points.map((p) => (stroke.world ? local(p[0], p[1]) : p).map(round));
        next.closed = false;
        return next;
      }
      const solids = claimed.get(s.id) ?? [];
      if (view !== "elevation" && s.id === "rumen-flank") { dropped.push(s.id); return null; }
      if (view === "front" && /^(scapula|humerus|forearm|cannon|hoof)-f[fn]$/.test(s.bone)) next.z = 3.25;
      const swept = s.solid?.kind === "extruded" && /teat|switch/.test(s.id);
      if (s.id === "ear-span") {
        const tips = ["near-ear", "far-ear"].map((id) => { const m = meshOf.get(id)!; let best = 0; for (let k = 0; k < m.world.length; k += 3) if (Math.abs(m.world[k + 2]!) > Math.abs(m.world[best + 2]!)) best = k; return [m.world[best]!, m.world[best + 1]!, m.world[best + 2]!]; });
        next.kind = "poly";
        next.points = tips.map((t) => (view === "front" ? [t[2]!, t[1]!] : [t[0]!, t[2]!]).map(round));
        next.closed = false;
        targets[view]!.set(s.id, next.points.map((p: P) => [p[0], p[1]]));
        return next;
      }
      if (solids.length === 0) {
        if (traced.includes(s)) { dropped.push(s.id); return null; }
        return s;
      }
      const own = new Set(solids.map((x) => x.id));
      const isDecal = solids.every(decal);
      const span: number[] = [];
      const tris: Tri[] = [];
      for (const x of isDecal ? all.filter((y) => ["lumbar", "thorax", "pelvis", "head", "jaw"].includes(y.bone) || own.has(y.id)) : solids) {
        const m = meshOf.get(x.id)!;
        const at = (k: number): { p: P; d: number } => {
          const [X, Y, Z] = [m.world[3 * k]!, m.world[3 * k + 1]!, m.world[3 * k + 2]!];
          if (own.has(x.id)) span.push(Z);
          if (view === "front" && !swept) return { p: [Z, Y], d: -X };
          if (view === "plan" && !swept) return { p: [X, Z], d: -Y };
          return { p: local(X, Y), d: Z };
        };
        for (let t = 0; t < m.indices.length; t += 3) {
          const [a, b, c] = [at(m.indices[t]!), at(m.indices[t + 1]!), at(m.indices[t + 2]!)];
          tris.push({ p: [a.p, b.p, c.p], d: [a.d, b.d, c.d], own: own.has(x.id) });
        }
      }
      const rings = outline(tris, CELL[scale]!, TOL[scale]!);
      if (rings.length === 0) { dropped.push(s.id); return null; }
      if (rings.length > 1) console.log(`  1:${scale} ${view} ${s.id}: ${rings.length} outlines, the largest kept`);
      const points = rings[0]!.map((p) => [round(p[0]), round(p[1])] as P);
      if (view !== "elevation" && !swept) targets[view]!.set(s.id, points.map((p) => [p[0], p[1]]));
      next.kind = "poly";
      next.points = points;
      next.closed = true;
      if (/-(hind|fore)-leg$/.test(s.id)) { next.filled = true; delete next.penMm; }
      const first = solids[0]!;
      next.tone = Math.round(first.tone * (/^far-/.test(s.id) && view === "elevation" ? 0.86 : 1) * 100) / 100;
      if (swept) next.solid = { kind: "extruded", sweep: "w", z0: round(Math.min(...span)), z1: round(Math.max(...span)) };
      else if (s.followTip === true) next.followTip = true;
      return next;
    }).filter((s): s is Json => s !== null);
    if (dropped.length > 0) console.log(`  1:${scale} ${view}: shape removed (not visible, or a sliver on both flanks): ${dropped.join(", ")}`);
    rep.geometry[view] = shapes;
  }
  for (let pass = 0; pass < 12; pass++) {
    const state = evaluate({ ...asset, representations } as never, { kind: "pose", pose: asset.presentation.defaultPose } as never, { kind: "scale", scale: { denominator: scale } } as never) as Json;
    let worst = 0;
    for (const view of ["front", "plan"] as const) {
      for (const e of state.shapes[view] as Json[]) {
        const want = targets[view]!.get(e.shape.id);
        if (!want) continue;
        const got: Json[] | undefined = e.rings?.[0];
        if (!got) continue;
        const shape = (rep.geometry[view] as Json[]).find((s) => s.id === e.shape.id)!;
        if (got.length !== want.length) { console.log(`  ${view} ${e.shape.id}: ring ${got.length} != ${want.length}`); continue; }
        shape.points = (shape.points as P[]).map((p, i) => {
          const dx = got[i].x - want[i]![0];
          const dy = got[i].y - want[i]![1];
          worst = Math.max(worst, Math.hypot(dx, dy));
          return [round(p[0] - dx), round(p[1] - dy)];
        });
      }
    }
    console.log(`1:${scale} pass ${pass}: largest placement error ${(worst * 1000).toFixed(2)} mm`);
    if (worst < 0.0003) break;
  }
  const count = VIEWS.map((v) => (rep.geometry[v] as Json[]).reduce((n, s) => n + (s.points?.length ?? 0), 0));
  console.log(`1:${scale} points per view`, count.join(" / "));
}
writeFileSync(join(GENERATED, "r3-dairy-cow", "members.json"), JSON.stringify({ dimensions: dims, representations }));
