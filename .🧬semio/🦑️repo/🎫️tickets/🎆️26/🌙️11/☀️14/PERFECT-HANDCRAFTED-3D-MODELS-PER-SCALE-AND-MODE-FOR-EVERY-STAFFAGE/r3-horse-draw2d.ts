/** 🖊️ Round 3 horse, ruling E1: redraws the 2D shapes of every scale and view as the true orthographic outline of that scale's handcrafted body in the default pose, shape by shape (same ids, roles, bones, layers and tones), and measures the dimensions.
 *
 * Every filled shape is the outline of the solids it stands for (its own bone; bones without a shape in the view go to the shape that merges them), traced on a fine raster and reduced to an economical polygon. Elevation shapes are written in the frame of their bone; front and plan shapes in rest coordinates that the product's own placement carries onto the outline (found by iterating the product evaluation). Unfilled detail and texture strokes come from `r3-horse-details.py`.
 *
 * Usage: bun r3-horse-draw2d.ts   → 🗑️generated/r3-horse/members.json (for `python3 r3-horse-root.py members`)
 * @see ./round3-brief.md */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { GENERATED, entryOf, loadSource, merged, meshesAt, poseState, type Json } from "./volumes-lib.ts";

type P = [number, number];
const VIEWS = ["elevation", "front", "plan"] as const;
type View = (typeof VIEWS)[number];
const HAIR = "mane-tail-hair";
const CELL: Record<number, number> = { 50: 0.0025, 100: 0.004, 200: 0.006, 500: 0.012 };
const TOL: Record<number, number> = { 50: 0.0022, 100: 0.004, 200: 0.007, 500: 0.016 };
const GEOMETRY_KEYS = new Set(["kind", "points", "closed", "solid", "followTip", "x0", "y0", "x1", "y1", "r0", "r1", "cx", "cy", "rx", "ry", "rotationDeg", "d", "x", "y", "width", "height"]);

/** 🧮️ The outer outlines of a set of triangles on a raster of `cell` metres, reduced with the tolerance `tol`. */
function outline(tris: readonly P[][], cell: number, tol: number): P[][] {
  let [x0, y0, x1, y1] = [Infinity, Infinity, -Infinity, -Infinity];
  for (const t of tris) for (const [x, y] of t) { x0 = Math.min(x0, x); y0 = Math.min(y0, y); x1 = Math.max(x1, x); y1 = Math.max(y1, y); }
  const ox = x0 - 2 * cell;
  const oy = y0 - 2 * cell;
  const w = Math.ceil((x1 - x0) / cell) + 4;
  const h = Math.ceil((y1 - y0) / cell) + 4;
  const g = new Uint8Array(w * h);
  for (const [a, b, c] of tris) {
    const d = (b![0] - a![0]) * (c![1] - a![1]) - (c![0] - a![0]) * (b![1] - a![1]);
    if (Math.abs(d) < 1e-14) continue;
    const i0 = Math.max(0, Math.floor((Math.min(a![0], b![0], c![0]) - ox) / cell));
    const i1 = Math.min(w - 1, Math.ceil((Math.max(a![0], b![0], c![0]) - ox) / cell));
    const j0 = Math.max(0, Math.floor((Math.min(a![1], b![1], c![1]) - oy) / cell));
    const j1 = Math.min(h - 1, Math.ceil((Math.max(a![1], b![1], c![1]) - oy) / cell));
    for (let j = j0; j <= j1; j++) for (let i = i0; i <= i1; i++) {
      const px = ox + (i + 0.5) * cell;
      const py = oy + (j + 0.5) * cell;
      const u = ((b![0] - a![0]) * (py - a![1]) - (px - a![0]) * (b![1] - a![1])) / d;
      const v = ((px - a![0]) * (c![1] - a![1]) - (c![0] - a![0]) * (py - a![1])) / d;
      if (u >= -1e-9 && v >= -1e-9 && u + v <= 1 + 1e-9) g[j * w + i] = 1;
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
  const outer = loops.filter((r) => area(r) > 5 * cell * cell).sort((a, b) => area(b) - area(a));
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

const { entry, asset: catalog } = entryOf("animals/horse");
const asset = merged(catalog, loadSource(entry)!);
const parent: Record<string, string | null> = Object.fromEntries((asset.rig.bones as Json[]).map((b) => [b.id, b.parent]));
const details = JSON.parse(readFileSync(join(GENERATED, "r3-horse", "details.json"), "utf8"));
const FALLBACK: Record<string, string> = { pelvis: "lumbar", "scapula-ff": "humerus-ff", "scapula-fn": "humerus-fn", "ear-f": "ear-n", "tail-1": "tail-2", "tail-3": "tail-4" };
const special = (x: Json): boolean => x.id === "muzzle" || /eye$|nostril$/.test(x.id) || (x.material === HAIR && !x.bone.startsWith("tail"));

/** 🧩️ The body solids every shape of a view stands for. */
function assign(scale: number, view: View, shapes: Json[], body: Json[]): Map<string, Json[]> {
  const out = new Map<string, Json[]>(shapes.map((s) => [s.id, []]));
  const used = new Set<string>();
  const give = (s: Json, list: Json[]): void => { for (const x of list) if (!used.has(x.id)) { used.add(x.id); out.get(s.id)!.push(x); } };
  const filled = shapes.filter((s) => s.filled === true || (s.closed === true && /eye|nostril-[nf]/.test(s.id) && view !== "elevation"));
  for (const s of filled) {
    if (/^(near|far)-eye$/.test(s.id)) give(s, body.filter((x) => x.id === s.id));
    else if (s.id === "eye") give(s, body.filter((x) => x.id === "near-eye"));
    else if (/^nostril-[nf]$/.test(s.id)) give(s, body.filter((x) => x.id === `${s.id.endsWith("n") ? "near" : "far"}-nostril`));
    else if (s.id === "muzzle") give(s, body.filter((x) => x.id === "muzzle"));
    else if (s.id === "forelock") give(s, body.filter((x) => x.id === "forelock"));
    else if (/^mane/.test(s.id)) give(s, body.filter((x) => x.material === HAIR && x.bone === s.bone && x.id !== "forelock"));
  }
  const main = filled.filter((s) => out.get(s.id)!.length === 0 && !/eye|nostril|^muzzle$|^forelock$|^mane/.test(s.id));
  const explicit: Record<string, string[]> = { "tail-upper": ["tail-1", "tail-2"], "tail-lower": ["tail-3", "tail-4"], ears: ["ear-n", "ear-f"] };
  for (const s of main) give(s, body.filter((x) => (explicit[s.id] ?? [s.bone]).includes(x.bone) && (Array.isArray(s.regions) || !special(x))));
  const shapeOn = (bone: string): Json | undefined => main.find((s) => s.bone === bone);
  for (const x of body) {
    if (used.has(x.id)) continue;
    let b: string | null | undefined = x.bone;
    let target: Json | undefined;
    const seen = new Set<string>();
    while (b && !seen.has(b)) {
      seen.add(b);
      target = shapeOn(b);
      if (target) break;
      b = FALLBACK[b] && shapeOn(FALLBACK[b]!) ? FALLBACK[b] : parent[b];
    }
    if (target) give(target, [x]);
    else console.log(`  1:${scale} ${view}: solid ${x.id} has no shape`);
  }
  return out;
}

const round = (v: number): number => Math.round(v * 10000) / 10000;
const representations: Json = JSON.parse(JSON.stringify(asset.representations));
let dims: Json = {};
for (const scale of [50, 100, 200, 500]) {
  const body: Json[] = asset.volumes[String(scale)].body;
  const world = meshesAt(asset, scale, body);
  const meshOf = new Map(world.map((m) => [m.id, m]));
  const frames = poseState(asset, scale).frames as Record<string, { o: number[]; u: number[]; v: number[] }>;
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
    const shapes: Json[] = rep.geometry[view];
    const groups = assign(scale, view, shapes, body);
    targets[view] = new Map();
    rep.geometry[view] = shapes.map((s) => {
      const hand = details[String(scale)]?.[view]?.[s.id];
      const solids = groups.get(s.id) ?? [];
      let points: P[] | undefined;
      let closed = true;
      const swept = /^(tail|ear)-/.test(s.bone);
      let span: [number, number] | undefined;
      if (swept) {
        const zs = body.filter((x) => x.bone === s.bone).flatMap((x) => { const m = meshOf.get(x.id)!; const out: number[] = []; for (let k = 2; k < m.world.length; k += 3) out.push(m.world[k]!); return out; });
        if (zs.length > 0) span = [round(Math.min(...zs)), round(Math.max(...zs))];
      }
      if (hand) { points = hand.points; closed = hand.closed; }
      else if (solids.length > 0) {
        const f = frames[s.bone]!;
        const tris: P[][] = [];
        for (const x of solids) {
          const m = meshOf.get(x.id)!;
          const p2 = (k: number): P => {
            const [X, Y, Z] = [m.world[3 * k]!, m.world[3 * k + 1]!, m.world[3 * k + 2]!];
            if (view === "front" && !swept) return [Z, Y];
            if (view === "plan" && !swept) return [X, Z];
            const d = [X - f.o[0]!, Y - f.o[1]!];
            return [d[0]! * f.u[0]! + d[1]! * f.u[1]!, d[0]! * f.v[0]! + d[1]! * f.v[1]!];
          };
          for (let t = 0; t < m.indices.length; t += 3) tris.push([p2(m.indices[t]!), p2(m.indices[t + 1]!), p2(m.indices[t + 2]!)]);
        }
        const rings = outline(tris, CELL[scale]!, TOL[scale]!);
        if (rings.length === 0) { console.log(`  1:${scale} ${view} ${s.id}: too small to trace, kept as drawn`); return s; }
        if (rings.length > 1) console.log(`  1:${scale} ${view} ${s.id}: ${rings.length} outlines, the largest kept`);
        points = rings[0]!.map((p) => [round(p[0]), round(p[1])]);
        if (view !== "elevation" && !swept) targets[view]!.set(s.id, points.map((p) => [p[0], p[1]]));
      } else {
        if (s.filled) console.log(`  1:${scale} ${view} ${s.id}: no solids, kept as drawn`);
        if (swept && span && s.solid) return { ...s, solid: { ...s.solid, z0: span[0], z1: span[1] } };
        return s;
      }
      const next: Json = {};
      for (const [k, v] of Object.entries(s)) if (!GEOMETRY_KEYS.has(k)) next[k] = v;
      next.kind = "poly";
      next.points = points;
      next.closed = closed;
      if (swept && span) next.solid = { kind: "extruded", sweep: "w", z0: span[0], z1: span[1] };
      else if (s.followTip === true) next.followTip = true;
      return next;
    });
  }
  for (let pass = 0; pass < 9; pass++) {
    const state = evaluate({ ...asset, representations } as never, { kind: "pose", pose: asset.presentation.defaultPose } as never, { kind: "scale", scale: { denominator: scale } } as never) as Json;
    let worst = 0;
    for (const view of ["front", "plan"] as const) {
      for (const e of state.shapes[view] as Json[]) {
        const want = targets[view]!.get(e.shape.id);
        if (!want) continue;
        const got: Json[] = e.rings[0];
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
writeFileSync(join(GENERATED, "r3-horse", "members.json"), JSON.stringify({ dimensions: dims, representations }));
