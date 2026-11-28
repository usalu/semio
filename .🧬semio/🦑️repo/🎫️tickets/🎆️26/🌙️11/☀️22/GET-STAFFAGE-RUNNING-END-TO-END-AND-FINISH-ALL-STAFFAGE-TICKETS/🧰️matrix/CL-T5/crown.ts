/** 👑️ Which corner points of its faceted foliage lobes carry the outline of a tree whose lobes already are convex polyhedra (the faceted ellipsoids of lane CL-T1 or handcrafted masses): `../CL-T4/crown.ts` for polyhedra. Per scale every lobe (`--match`, a convex polyhedron of the body) in the default pose and its convex outline in the three views of the silhouette guard. Per view, greedy and inner lobes first, a lobe whose outline grown by `--grow` lies inside the union of the outlines of the lobes that still carry that view stops carrying it; of a carrier's outline a run of corner points whose cap lies inside the other carriers is left out; so the union of the outlines stays the union of all, to the point. A lobe keeps the corner points on the showing part of its outline in the views it carries, for its roundness from every other side its corner points nearest to those of an ellipsoid of `--segments` facets in its own box, and under an emitter of the thermal list (`--emitters`) every corner point within `--spot` of the centre of the emitter: the convex polyhedron of these points lies inside the lobe. A lobe that is not convex, has a `repeat` or would keep `--most` (default 0.8) of its corner points or more is left as it is. crown.ts <asset id | file.json> --out <ops.json> [--segments 8] [--grow 1.003] [--near 0.2] [--spot 1] [--match <regular expression>] [--emitters <regular expression>] writes the `keep` ops of `facet.ts` and prints the counts.
 * @see ../CL-T4/crown.ts
 * @see ./facet.ts */
import { writeFileSync } from "node:fs";
import { parseArgs } from "node:util";
import pc from "polygon-clipping";
import { tessellateSolid } from "../../../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
import { worldSolids } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { evaluate } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { qualityOf } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/⏱️job/🟦️.ts";
import { readDoc } from "../MR/lib.ts";

type P = [number, number];
type W = { id: string; positions: Float64Array };
const { values, positionals } = parseArgs({ options: { out: { type: "string" }, segments: { type: "string" }, grow: { type: "string" }, match: { type: "string" }, emitters: { type: "string" }, near: { type: "string" }, spot: { type: "string" }, most: { type: "string" } }, allowPositionals: true });
const asset = readDoc(positionals[0]!).doc.asset;
const quality = qualityOf(asset);
const grow = Number(values.grow ?? 1.003);
const segments = Number(values.segments ?? 8);
const pattern = new RegExp(values.match ?? "^lobe-", "u");
const outlets = new RegExp(values.emitters ?? "^em-", "u");
const near = Number(values.near ?? 0.2);
const spot = Number(values.spot ?? 1);
const most = Number(values.most ?? 0.8);
const AXES: [number, number][] = [[0, 1], [2, 1], [0, 2]];
const unit = tessellateSolid({ id: "lobe", node: "n", material: "m", kind: "ellipsoid", radii: [1, 1, 1] } as never, { chordM: quality.chordM, minSegments: segments, maxSegments: segments }).positions;

/** 🪢️ The indices of the convex hull of points in the plane, counter-clockwise, without the points on its edges. */
function hull(points: P[]): number[] {
  const order = points.map((_, i) => i).sort((a, b) => points[a]![0] - points[b]![0] || points[a]![1] - points[b]![1]);
  const cross = (o: number, a: number, b: number): number => (points[a]![0] - points[o]![0]) * (points[b]![1] - points[o]![1]) - (points[a]![1] - points[o]![1]) * (points[b]![0] - points[o]![0]);
  const lower: number[] = [], upper: number[] = [];
  for (const q of order) { while (lower.length >= 2 && cross(lower[lower.length - 2]!, lower[lower.length - 1]!, q) <= 1e-12) lower.pop(); lower.push(q); }
  for (const q of [...order].reverse()) { while (upper.length >= 2 && cross(upper[upper.length - 2]!, upper[upper.length - 1]!, q) <= 1e-12) upper.pop(); upper.push(q); }
  return [...lower.slice(0, -1), ...upper.slice(0, -1)];
}

/** 🥚️ Whether the facets of a polyhedron are a convex shell: no corner point lies more than a twentieth of a millimetre outside (the corner points are written to a hundredth of a millimetre) the plane of any facet. */
function convex(vertices: number[][], triangles: number[][]): boolean {
  for (const [a, b, c] of triangles) {
    const [p, q, r] = [vertices[a!]!, vertices[b!]!, vertices[c!]!];
    const n = [(q[1]! - p[1]!) * (r[2]! - p[2]!) - (q[2]! - p[2]!) * (r[1]! - p[1]!), (q[2]! - p[2]!) * (r[0]! - p[0]!) - (q[0]! - p[0]!) * (r[2]! - p[2]!), (q[0]! - p[0]!) * (r[1]! - p[1]!) - (q[1]! - p[1]!) * (r[0]! - p[0]!)];
    const length = Math.hypot(n[0]!, n[1]!, n[2]!);
    if (!(length > 0)) continue;
    for (const v of vertices) if (((v[0]! - p[0]!) * n[0]! + (v[1]! - p[1]!) * n[1]! + (v[2]! - p[2]!) * n[2]!) / length > 5e-5) return false;
  }
  return true;
}

const ops: unknown[] = [];
for (const scale of [50, 100, 200, 500]) {
  const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } } as never);
  const authored = new Map((asset.volumes[`${scale}`].body as Record<string, unknown>[]).map((s) => [s.id as string, s]));
  let skipped = 0;
  const lobes = (worldSolids(asset, state.bones as never, scale) as unknown as W[]).filter((w) => {
    const s = authored.get(w.id);
    if (!pattern.test(w.id) || s?.kind !== "polyhedron") return false;
    if (s.repeat !== undefined || !convex(s.vertices as number[][], s.triangles as number[][])) { skipped += 1; return false; }
    return true;
  });
  const emitters = (worldSolids(asset, state.bones as never, scale, "thermal") as unknown as W[]).filter((w) => outlets.test(w.id));
  const centre = (w: W): number[] => { const c = [0, 0, 0]; for (let i = 0; i < w.positions.length; i++) c[i % 3]! += w.positions[i]! / (w.positions.length / 3); return c; };
  const reach = (w: W, c: number[]): number => { let r = 0; for (let i = 0; i < w.positions.length; i += 3) r = Math.max(r, Math.hypot(w.positions[i]! - c[0]!, w.positions[i + 1]! - c[1]!, w.positions[i + 2]! - c[2]!)); return r; };
  const gap = (w: W, c: number[]): number => { let d = Infinity; for (let i = 0; i < w.positions.length; i += 3) d = Math.min(d, Math.hypot(w.positions[i]! - c[0]!, w.positions[i + 1]! - c[1]!, w.positions[i + 2]! - c[2]!)); return d; };
  const centres = lobes.map(centre);
  const left = new Set<number>();
  for (const e of emitters) { const c = centre(e); const gaps = lobes.map((w, i) => (Math.hypot(centres[i]![0]! - c[0]!, centres[i]![1]! - c[1]!, centres[i]![2]! - c[2]!) < reach(w, centres[i]!) + reach(e, c) + near ? gap(w, c) : Infinity)); const nearest = Math.min(...gaps); gaps.forEach((d, i) => { if (d < Infinity && (d === nearest || d < near)) left.add(i); }); }
  const welded = lobes.map((w) => {
    const solid = authored.get(w.id)!;
    const vertices = solid.vertices as number[][];
    const local = tessellateSolid({ id: "lobe", node: "n", material: "m", kind: "polyhedron", vertices, triangles: solid.triangles } as never, quality).positions;
    if (local.length !== w.positions.length) throw new Error(`${w.id}: ${local.length / 3} local and ${w.positions.length / 3} posed vertices`);
    const index = new Map(vertices.map((p, k) => [p.join(","), k]));
    const remap: number[] = [];
    for (let i = 0; i < local.length; i += 3) {
      let k = index.get(`${local[i]},${local[i + 1]},${local[i + 2]}`);
      if (k === undefined) { let d = Infinity; vertices.forEach((p, at) => { const x = (p[0]! - local[i]!) ** 2 + (p[1]! - local[i + 1]!) ** 2 + (p[2]! - local[i + 2]!) ** 2; if (x < d) { d = x; k = at; } }); if (d > 1e-12) throw new Error(`${w.id}: a posed vertex is ${Math.sqrt(d)} m from every authored corner point`); }
      remap.push(k!);
    }
    const lo = [0, 1, 2].map((k) => Math.min(...vertices.map((p) => p[k]!))), hi = [0, 1, 2].map((k) => Math.max(...vertices.map((p) => p[k]!)));
    const base = new Set<number>();
    for (let i = 0; i < unit.length; i += 3) {
      const t = [0, 1, 2].map((k) => (lo[k]! + hi[k]!) / 2 + (unit[i + k]! * (hi[k]! - lo[k]!)) / 2);
      let best = 0, d = Infinity;
      vertices.forEach((p, k) => { const x = (p[0]! - t[0]!) ** 2 + (p[1]! - t[1]!) ** 2 + (p[2]! - t[2]!) ** 2; if (x < d) { d = x; best = k; } });
      base.add(best);
    }
    return { remap, base, count: vertices.length };
  });
  const rims = AXES.map(([ax, ay]) => lobes.map((w) => { const pts: P[] = []; for (let i = 0; i < w.positions.length; i += 3) pts.push([w.positions[i + ax]!, w.positions[i + ay]!]); const at = hull(pts); return { at, ring: at.map((k) => pts[k]!) }; }));
  const boxes = rims.map((list) => list.map(({ ring }) => [Math.min(...ring.map((q) => q[0])), Math.min(...ring.map((q) => q[1])), Math.max(...ring.map((q) => q[0])), Math.max(...ring.map((q) => q[1]))]));
  const middle = [0, 1, 2].map((k) => centres.reduce((s, c) => s + c[k]! / centres.length, 0));
  const far = (i: number): number => Math.hypot(centres[i]![0]! - middle[0]!, centres[i]![1]! - middle[1]!, centres[i]![2]! - middle[2]!);
  const order = lobes.map((_, i) => i).sort((a, b) => far(a) - far(b));
  const carriers = AXES.map(() => new Set(order));
  const touching = (v: number, i: number): number[] => { const box = boxes[v]![i]!; return [...carriers[v]!].filter((j) => j !== i && boxes[v]![j]![0]! < box[2]! && boxes[v]![j]![2]! > box[0]! && boxes[v]![j]![1]! < box[3]! && boxes[v]![j]![3]! > box[1]!); };
  const covered = (ring: P[], others: unknown[]): boolean => {
    const c: P = [ring.reduce((t, q) => t + q[0], 0) / ring.length, ring.reduce((t, q) => t + q[1], 0) / ring.length];
    const grown = ring.map((q): P => [c[0] + (q[0] - c[0]) * grow, c[1] + (q[1] - c[1]) * grow]);
    try { return pc.difference([[...grown, grown[0]!]], ...(others as never[])).length === 0; } catch { return false; }
  };
  AXES.forEach((_, v) => {
    for (const i of order) {
      const others = touching(v, i).map((j) => [[...rims[v]![j]!.ring, rims[v]![j]!.ring[0]!]]);
      if (others.length > 0 && covered(rims[v]![i]!.ring, others)) carriers[v]!.delete(i);
    }
  });
  let cut = 0;
  const inside = (p: P, ring: P[], shrink: number): boolean => {
    const c: P = [ring.reduce((t, q) => t + q[0], 0) / ring.length, ring.reduce((t, q) => t + q[1], 0) / ring.length];
    let hit = false;
    for (let a = 0, b = ring.length - 1; a < ring.length; b = a++) {
      const [ax, ay, bx, by] = [c[0] + (ring[a]![0] - c[0]) * shrink, c[1] + (ring[a]![1] - c[1]) * shrink, c[0] + (ring[b]![0] - c[0]) * shrink, c[1] + (ring[b]![1] - c[1]) * shrink];
      if (ay > p[1] !== by > p[1] && p[0] < ((bx - ax) * (p[1] - ay)) / (by - ay) + ax) hit = !hit;
    }
    return hit;
  };
  AXES.forEach((_, v) => {
    for (const i of order) {
      if (!carriers[v]!.has(i)) continue;
      const rim = rims[v]![i]!;
      const close = touching(v, i);
      const others = close.map((j) => [[...rims[v]![j]!.ring, rims[v]![j]!.ring[0]!]]);
      const n = rim.ring.length;
      const open = rim.ring.map((p) => !close.some((j) => inside(p, rims[v]![j]!.ring, 1 / grow)));
      if (others.length === 0 || open.every((x) => !x) || open.every((x) => x)) continue;
      const gone = new Set<number>();
      const start = open.findIndex((x) => x);
      const drop = (from: number, to: number): void => {
        if (to - from < 2) return;
        if (covered(Array.from({ length: to - from + 1 }, (_, k) => rim.ring[(start + from + k) % n]!), others)) { for (let k = from + 1; k < to; k++) gone.add((start + k) % n); return; }
        if (to - from < 4) return;
        const half = (from + to) >> 1;
        drop(from, half); drop(half, to);
      };
      let run = -1;
      for (let k = 1; k <= n; k++) {
        const closed = !open[(start + k) % n]!;
        if (closed && run < 0) run = k;
        if (!closed && run >= 0) { drop(run, k - 1); run = -1; }
      }
      if (gone.size === 0) continue;
      cut += gone.size;
      rims[v]![i] = { at: rim.at.filter((_, k) => !gone.has(k)), ring: rim.ring.filter((_, k) => !gone.has(k)) };
    }
  });
  const spots = emitters.map(centre);
  const keep: Record<string, number[]> = {};
  let points = 0, none = 0, whole = 0, before = 0;
  lobes.forEach((w, i) => {
    const chosen = new Set(welded[i]!.base);
    if (left.has(i)) for (let k = 0; k < w.positions.length / 3; k++) if (spots.some((c) => Math.hypot(w.positions[k * 3]! - c[0]!, w.positions[k * 3 + 1]! - c[1]!, w.positions[k * 3 + 2]! - c[2]!) < spot)) chosen.add(welded[i]!.remap[k]!);
    let carries = 0;
    AXES.forEach((_, v) => { if (carriers[v]!.has(i)) { carries += 1; for (const k of rims[v]![i]!.at) chosen.add(welded[i]!.remap[k]!); } });
    if (carries === 0) none += 1;
    before += welded[i]!.count;
    if (chosen.size >= most * welded[i]!.count) { whole += 1; points += welded[i]!.count; return; }
    keep[w.id] = [...chosen].sort((a, b) => a - b);
    points += chosen.size;
  });
  console.log(`1:${scale}: ${lobes.length} lobes of ${before} corner points (${skipped} not convex or repeated and left), ${left.size} under an emitter keep every corner point within ${spot} m of it; ${cut} outline corner points lie with their whole cap inside other carriers and are left out; carriers elevation ${carriers[0]!.size}, front ${carriers[1]!.size}, plan ${carriers[2]!.size}; ${none} carry no view; ${whole} keep their facets; ${Object.keys(keep).length} lobes keep chosen corner points, all lobes ${points} corner points (${(points / Math.max(1, lobes.length)).toFixed(1)} each, about ${2 * points - 4 * lobes.length} triangles)`);
  if (Object.keys(keep).length > 0) ops.push({ keep, list: "body", scales: [scale] });
}
if (values.out !== undefined) writeFileSync(values.out, JSON.stringify(ops) + "\n");
