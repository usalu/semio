/** 👑️ Which corner points of its foliage lobes carry the outline of a tree: per scale every ellipsoid lobe of the body in the default pose at the reference facets (the mesh the silhouette guard measures) and its convex outline in the three views of the guard. Per view, greedy and inner lobes first, a lobe whose outline grown by `--grow` lies inside the union of the outlines of the lobes that still carry that view stops carrying it; the union of the carriers stays the union of all. A lobe keeps the corner points of its reference facets that lie on its outline in the views it carries and, for its roundness from every other side, the corner points nearest to those of the same ellipsoid at `--segments` facets: the convex polyhedron of these points shows the outline of the reference lobe exactly in the views it carries and lies inside the reference lobe everywhere else, so the outline of the tree in the three views does not move by one point. Of the outline of a carrier only the part that shows counts: a run of outline corner points whose cap (the piece of the lobe outline between the chord of the run and the run) lies inside the outlines the other carriers have at that moment is left out, lobe after lobe, so the union still does not change. The lobe nearest to the centre of an emitter of the thermal list, and every lobe with a corner point within `--near` (default 0.2 m) of it, also keeps every corner point within `--spot` (default 1 m) of that centre, so the outlet stays on the very facets it was placed on. crown.ts <asset id | file.json> --out <ops.json> [--segments 12] [--grow 1.003] [--near 0.2] [--match <regular expression over the id>] writes the `keep` ops of `facet.ts` and prints the counts.
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
const { values, positionals } = parseArgs({ options: { out: { type: "string" }, segments: { type: "string" }, grow: { type: "string" }, match: { type: "string" }, near: { type: "string" }, spot: { type: "string" } }, allowPositionals: true });
const asset = readDoc(positionals[0]!).doc.asset;
const quality = qualityOf(asset);
const grow = Number(values.grow ?? 1.003);
const segments = Number(values.segments ?? 12);
const pattern = new RegExp(values.match ?? "^lobe-", "u");
const near = Number(values.near ?? 0.2);
const spot = Number(values.spot ?? 1);
const AXES: [number, number][] = [[0, 1], [2, 1], [0, 2]];
const round = (n: number): number => Number(n.toFixed(5)) + 0;

/** 🪢️ The indices of the convex hull of points in the plane, counter-clockwise, without the points on its edges. */
function hull(points: P[]): number[] {
  const order = points.map((_, i) => i).sort((a, b) => points[a]![0] - points[b]![0] || points[a]![1] - points[b]![1]);
  const cross = (o: number, a: number, b: number): number => (points[a]![0] - points[o]![0]) * (points[b]![1] - points[o]![1]) - (points[a]![1] - points[o]![1]) * (points[b]![0] - points[o]![0]);
  const lower: number[] = [], upper: number[] = [];
  for (const q of order) { while (lower.length >= 2 && cross(lower[lower.length - 2]!, lower[lower.length - 1]!, q) <= 1e-12) lower.pop(); lower.push(q); }
  for (const q of [...order].reverse()) { while (upper.length >= 2 && cross(upper[upper.length - 2]!, upper[upper.length - 1]!, q) <= 1e-12) upper.pop(); upper.push(q); }
  return [...lower.slice(0, -1), ...upper.slice(0, -1)];
}

const ops: unknown[] = [];
for (const scale of [50, 100, 200, 500]) {
  const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } } as never);
  const authored = new Map((asset.volumes[`${scale}`].body as Record<string, unknown>[]).map((s) => [s.id as string, s]));
  const lobes = worldSolids(asset, state.bones as never, scale).filter((w) => pattern.test(w.id) && authored.get(w.id)?.kind === "ellipsoid");
  const emitters = worldSolids(asset, state.bones as never, scale, "thermal").filter((w) => /^em-/u.test(w.id));
  const centre = (w: { positions: Float64Array }): number[] => { const c = [0, 0, 0]; for (let i = 0; i < w.positions.length; i++) c[i % 3]! += w.positions[i]! / (w.positions.length / 3); return c; };
  const reach = (w: { positions: Float64Array }, c: number[]): number => { let r = 0; for (let i = 0; i < w.positions.length; i += 3) r = Math.max(r, Math.hypot(w.positions[i]! - c[0]!, w.positions[i + 1]! - c[1]!, w.positions[i + 2]! - c[2]!)); return r; };
  const centres = lobes.map(centre);
  const left = new Set<number>();
  const gap = (w: { positions: Float64Array }, c: number[]): number => { let d = Infinity; for (let i = 0; i < w.positions.length; i += 3) d = Math.min(d, Math.hypot(w.positions[i]! - c[0]!, w.positions[i + 1]! - c[1]!, w.positions[i + 2]! - c[2]!)); return d; };
  for (const e of emitters) { const c = centre(e); const gaps = lobes.map((w, i) => (Math.hypot(centres[i]![0]! - c[0]!, centres[i]![1]! - c[1]!, centres[i]![2]! - c[2]!) < reach(w, centres[i]!) + reach(e, c) ? gap(w, c) : Infinity)); const nearest = Math.min(...gaps); gaps.forEach((d, i) => { if (d === nearest || d < near) left.add(i); }); }
  const welded = lobes.map((w) => {
    const solid = authored.get(w.id)!;
    const local = tessellateSolid({ id: "lobe", node: "n", material: "m", kind: "ellipsoid", radii: solid.radii } as never, quality).positions;
    if (local.length !== w.positions.length) throw new Error(`${w.id}: ${local.length / 3} local and ${w.positions.length / 3} posed vertices`);
    const index = new Map<string, number>();
    const points: number[][] = [];
    const remap: number[] = [];
    for (let i = 0; i < local.length; i += 3) { const p = [round(local[i]!), round(local[i + 1]!), round(local[i + 2]!)]; const key = p.join(","); if (!index.has(key)) { index.set(key, points.length); points.push(p); } remap.push(index.get(key)!); }
    const coarse = tessellateSolid({ id: "lobe", node: "n", material: "m", kind: "ellipsoid", radii: solid.radii } as never, { chordM: quality.chordM, minSegments: segments, maxSegments: segments }).positions;
    const base = new Set<number>();
    for (let i = 0; i < coarse.length; i += 3) { let best = 0, d = Infinity; points.forEach((p, k) => { const x = (p[0]! - coarse[i]!) ** 2 + (p[1]! - coarse[i + 1]!) ** 2 + (p[2]! - coarse[i + 2]!) ** 2; if (x < d) { d = x; best = k; } }); base.add(best); }
    return { remap, base, count: points.length };
  });
  const rims = AXES.map(([ax, ay]) => lobes.map((w) => { const pts: P[] = []; for (let i = 0; i < w.positions.length; i += 3) pts.push([w.positions[i + ax]!, w.positions[i + ay]!]); const at = hull(pts); return { at, ring: at.map((k) => pts[k]!) }; }));
  const boxes = rims.map((list) => list.map(({ ring }) => [Math.min(...ring.map((q) => q[0])), Math.min(...ring.map((q) => q[1])), Math.max(...ring.map((q) => q[0])), Math.max(...ring.map((q) => q[1]))]));
  const middle = [0, 1, 2].map((k) => centres.reduce((s, c) => s + c[k]! / centres.length, 0));
  const far = (i: number): number => Math.hypot(centres[i]![0]! - middle[0]!, centres[i]![1]! - middle[1]!, centres[i]![2]! - middle[2]!);
  const order = lobes.map((_, i) => i).sort((a, b) => far(a) - far(b));
  const carriers = AXES.map(() => new Set(order));
  AXES.forEach((_, v) => {
    for (const i of order) {
      const ring = rims[v]![i]!.ring, box = boxes[v]![i]!;
      const c: P = [(box[0]! + box[2]!) / 2, (box[1]! + box[3]!) / 2];
      const grown = ring.map((q): P => [c[0] + (q[0] - c[0]) * grow, c[1] + (q[1] - c[1]) * grow]);
      const others = [...carriers[v]!].filter((j) => j !== i && boxes[v]![j]![0]! < box[2]! && boxes[v]![j]![2]! > box[0]! && boxes[v]![j]![1]! < box[3]! && boxes[v]![j]![3]! > box[1]!).map((j) => [[...rims[v]![j]!.ring, rims[v]![j]!.ring[0]!]]);
      if (others.length === 0) continue;
      let covered = false;
      try { covered = pc.difference([[...grown, grown[0]!]], ...(others as never[])).length === 0; } catch { covered = false; }
      if (covered) carriers[v]!.delete(i);
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
      const rim = rims[v]![i]!, box = boxes[v]![i]!;
      const near = [...carriers[v]!].filter((j) => j !== i && boxes[v]![j]![0]! < box[2]! && boxes[v]![j]![2]! > box[0]! && boxes[v]![j]![1]! < box[3]! && boxes[v]![j]![3]! > box[1]!);
      const others = near.map((j) => [[...rims[v]![j]!.ring, rims[v]![j]!.ring[0]!]]);
      const n = rim.ring.length;
      const open = rim.ring.map((p) => !near.some((j) => inside(p, rims[v]![j]!.ring, 1 / grow)));
      if (others.length === 0 || open.every((x) => !x) || open.every((x) => x)) continue;
      const gone = new Set<number>();
      const start = open.findIndex((x) => x);
      const drop = (from: number, to: number): void => {
        if (to - from < 2) return;
        const cap = Array.from({ length: to - from + 1 }, (_, k) => rim.ring[(start + from + k) % n]!);
        const c: P = [cap.reduce((t, q) => t + q[0], 0) / cap.length, cap.reduce((t, q) => t + q[1], 0) / cap.length];
        const grown = cap.map((q): P => [c[0] + (q[0] - c[0]) * grow, c[1] + (q[1] - c[1]) * grow]);
        let covered = false;
        try { covered = pc.difference([[...grown, grown[0]!]], ...(others as never[])).length === 0; } catch { covered = false; }
        if (covered) { for (let k = from + 1; k < to; k++) gone.add((start + k) % n); return; }
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
  let points = 0, none = 0;
  lobes.forEach((w, i) => {
    const chosen = new Set(welded[i]!.base);
    if (left.has(i)) for (let k = 0; k < w.positions.length / 3; k++) if (spots.some((c) => Math.hypot(w.positions[k * 3]! - c[0]!, w.positions[k * 3 + 1]! - c[1]!, w.positions[k * 3 + 2]! - c[2]!) < spot)) chosen.add(welded[i]!.remap[k]!);
    let carries = 0;
    AXES.forEach((_, v) => { if (carriers[v]!.has(i)) { carries += 1; for (const k of rims[v]![i]!.at) chosen.add(welded[i]!.remap[k]!); } });
    if (carries === 0) none += 1;
    keep[w.id] = [...chosen].sort((a, b) => a - b);
    points += chosen.size;
  });
  console.log(`1:${scale}: ${lobes.length} lobes, ${left.size} under an emitter keep every corner point within ${spot} m of it; ${cut} outline corner points lie with their whole cap inside other carriers and are left out; carriers elevation ${carriers[0]!.size}, front ${carriers[1]!.size}, plan ${carriers[2]!.size}; ${none} carry no view; ${Object.keys(keep).length} lobes keep ${points} corner points (${(points / Math.max(1, Object.keys(keep).length)).toFixed(1)} each, about ${2 * points - 4 * Object.keys(keep).length} triangles)`);
  if (Object.keys(keep).length > 0) ops.push({ keep, list: "body", scales: [scale] });
}
if (values.out !== undefined) writeFileSync(values.out, JSON.stringify(ops) + "\n");
