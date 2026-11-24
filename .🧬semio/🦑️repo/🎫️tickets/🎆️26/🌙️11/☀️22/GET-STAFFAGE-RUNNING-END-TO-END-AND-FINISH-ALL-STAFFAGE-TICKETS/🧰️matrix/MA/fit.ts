/** 🪡️ The multiview repair of one asset, the last step of its chain: `bun fit.ts <id|file.json> --ops <ops.json> [--turns <turns.json>] [--write] [--out <file>]` applies the ops of the file in order and prints what each did (an op that cannot be applied is refused by name, the others still run, exit code 1); without `--write` nothing is written. Every op takes the ELEVATION twin (the shape of the same `id` and bone in `geometry.elevation`, or `twin`) as the profile of the part and rebuilds or refits the front or plan shape from it, so a second run changes nothing.
 *
 * Ops (`scale` is the representation key, `id` the shape id in the target view, `bone` only when the id is not unique):
 * - `{ "op": "front-sweep", scale, id }`: the front shape becomes (or stays) an `extruded` solid with `sweep: "v"`; its outline is kept (a flat front outline is first re-expressed in the bone-local (u, w) plane), cut along the bone to the u-range of the elevation twin (stretched to it where it falls short; `"range": "stretch"` only stretches, `"keep"` leaves it; `"lateral": "plan"` cuts it the same way across the bone to the lateral range of the plan twin), and `z0`, `z1`, `zTip` are fitted so that the (u, v) footprint of the prism has the support function of the elevation outline in every direction (so the heights agree at every sagittal angle).
 * - `{ "op": "view-sweep", scale, view, id }` (added for the animals of batch MA): the front or plan shape keeps its own outline and becomes an `extruded` solid swept along the bone axis nearest to the view direction (`u` for a bone that points at the viewer: the outline is its cross-section in (v, w); `v` for a bone seen from its normal: the outline in (u, w)); the outline is cut to the range of the elevation twin on its first axis and `z0`, `z1`, `zTip` are fitted to the (u, v) footprint of the elevation twin (`"taper": false` fits no `zTip`, `"lateral": "front" | "plan"` cuts the outline across the bone to the lateral range of the twin of that view, `"sweep"` forces the axis).
 * - `{ "op": "solid", scale, view, id }` (batch MA): `view-sweep` where its footprint mismatch stays within `"limit"` (share of the tolerance of the asset, default 0.4), else `extrude`; `"across": "plan" | "front"` gives `extrude` the lateral taper of the twin of that view.
 * - `{ "op": "extrude", scale, id, "view": "front" | "plan" }`: the exact alternative for a part whose front outline is a plain taper (and for the plan of a part that is a prism): the shape takes the elevation outline as its profile and becomes an `extruded` solid with `sweep: "w"` whose lateral span `z0`, `z1`, `zTip` is fitted to the (u, w) footprint the shape had; elevation and this view then agree at every pose by construction, and the view draws the hull of the swept outline instead of its own outline. It is also the op for a bone that points at the viewer of the front view (a foot, a toe): there no front outline can be swept along v and `front-sweep` refuses.
 * - `{ "op": "plan-revolve", scale, id }`: the plan shape becomes (or stays) a `revolved` solid about the bone axis. An elevation twin that is itself revolved is copied (one body in three views). Else an existing revolved profile is kept in shape, cut or stretched to the u-range of the elevation twin, and its axis offset and radius scale are fitted to the (u, v) footprint of the elevation twin and the (u, w) footprint of the front twin; a flat plan shape gets a new profile of the mean half thickness of the two twins at nine stations (`"stations": n`).
 * - `{ "op": "copy", scale, id, "from": view, "to": [view…] }`: the geometry and the solid of the shape in `from` replace those of its twins in `to` (the same revolved or extruded body in several views); `"create": true` adds a missing twin with the paint fields of the source.
 * - `{ "op": "set", scale, view, id, "set": {…} }` and `{ "op": "remove", scale, view, id }`: a hand-made correction of single fields, or the removal of a shape.
 * - `{ "op": "anchor", id, "set": {…} }`: binds an anchor (`bone`, `x`, `y`, `z`).
 * @see ./gate.ts — the measurement that says which bones need an op */
import { join } from "node:path";
import { S, dot, readDoc, round, sub, support, writeDoc, type V3 } from "../MR/lib.ts";

const { restFrames } = await import(join(S, "🔨️modules", "🦴️rig", "🟦️.ts"));
const { flattenGeometry, evaluate } = await import(join(S, "🔨️modules", "🧮️evaluation", "🟦️.ts"));
const coverage = await import(join(S, "🧪️tests", "🔭️multiview-coverage", "🧰️.ts"));

type P2 = readonly [number, number];
type Frame = { o: V3; u: V3; v: V3; w: V3 };
const args = process.argv.slice(2);
const flag = (name: string): string | undefined => {
  const i = args.indexOf(`--${name}`);
  return i < 0 ? undefined : args[i + 1];
};
const read = readDoc(args[0]!);
const asset = read.doc.asset;
const ops = JSON.parse(await Bun.file(flag("ops")!).text()) as any[];
const frames = restFrames(asset.rig) as Record<string, { frame: Frame }>;
const lengthOf = (bone: string): number => asset.rig.bones.find((candidate: any) => candidate.id === bone).lengthM;
const ANGLES = Array.from({ length: 72 }, (_, i) => (2 * Math.PI * i) / 72);

const shapesOf = (scale: string, view: string): any[] => asset.representations[scale].geometry[view];
const find = (scale: string, view: string, id: string, bone?: string): any => shapesOf(scale, view).find((shape) => shape.id === id && (bone === undefined || shape.bone === bone));
const need = (scale: string, view: string, id: string, bone?: string): any => {
  const shape = find(scale, view, id, bone);
  if (shape === undefined) throw new Error(`1:${scale} ${view} has no shape "${id}"${bone ? ` on ${bone}` : ""}`);
  return shape;
};
const pointsOf = (shape: any): P2[] => (shape.kind === "poly" ? (shape.points as P2[]) : flattenGeometry(shape).rings.flat().map((p: any) => [p.x, p.y] as const));
const rangeOf = (points: readonly P2[], k: 0 | 1): [number, number] => [Math.min(...points.map((p) => p[k])), Math.max(...points.map((p) => p[k]))];
const GEOMETRY_KEYS = ["points", "closed", "x", "y", "w", "h", "x0", "y0", "x1", "y1", "r0", "r1", "r", "cx", "cy", "rx", "ry", "d", "followTip", "solid"];

/** 🧭️ The local offset (v, w) of the axis of a revolved solid of a bone, and the profile points as (u, radius). */
function revolvedOf(shape: any, frame: Frame): { av: number; aw: number; radial: P2[] } {
  const offset: V3 = [shape.solid.axisX, 0, shape.solid.axisZ ?? 0];
  const [av, aw] = [dot(offset, frame.v), dot(offset, frame.w)];
  return { av, aw, radial: pointsOf(shape).map(([u, y]) => [u, Math.hypot(y - av, (shape.solid.axisZ ?? 0) - aw)] as const) };
}

/** 🦶️ The footprint of a shape's implied solid in the bone-local (u, v) plane (`plane` 1) or (u, w) plane (`plane` 2), as points whose hull it is; undefined where the shape gives none. */
function footprint(shape: any, view: string, frame: Frame, length: number, plane: 1 | 2): P2[] | undefined {
  const points = pointsOf(shape);
  const solid = shape.solid;
  if (solid?.kind === "revolved") {
    const { av, aw, radial } = revolvedOf(shape, frame);
    const centre = plane === 1 ? av : aw;
    return radial.flatMap(([u, r]) => [[u, centre - r] as const, [u, centre + r] as const]);
  }
  if (solid?.kind === "extruded" && solid.sweep === "v") {
    if (plane === 2) return points;
    return points.flatMap(([u]) => {
      const t = solid.zTip === undefined || length <= 0 ? 0 : Math.min(Math.max(u / length, 0), 1);
      return [[u, solid.z0 + ((solid.zTip?.[0] ?? solid.z0) - solid.z0) * t] as const, [u, solid.z1 + ((solid.zTip?.[1] ?? solid.z1) - solid.z1) * t] as const];
    });
  }
  if (solid?.kind === "extruded" && solid.sweep === "w") return plane === 1 ? points : points.flatMap(([u]) => [[u, solid.z0 - frame.o[2]] as const, [u, solid.z1 - frame.o[2]] as const]);
  if (solid !== undefined) return undefined;
  if (view === "elevation") return plane === 1 ? points : undefined;
  if (view === "front" && plane === 2) return frontLocal(points, frame);
  return undefined;
}

/** 🔁️ A flat front outline (lateral, height) in asset metres at rest re-expressed in the bone-local (u, w) plane through the bone origin, seen along the forward axis. */
function frontLocal(points: readonly P2[], frame: Frame): P2[] {
  const det = frame.u[1] * frame.w[2] - frame.w[1] * frame.u[2];
  if (Math.abs(det) < 0.2) throw new Error("the (u, w) plane of the bone is nearly edge-on in the front view: no front outline can be swept along v");
  return points.map(([z, y]) => {
    const [dy, dz] = [y - frame.o[1], z - frame.o[2]];
    return [(dy * frame.w[2] - dz * frame.w[1]) / det, (frame.u[1] * dz - frame.u[2] * dy) / det] as const;
  });
}

/** 🪨️ Keeps the geometry a shape had when the refit differs from it by less than 0.3 mm in every number (the rounding of a fit of a fit), so a second run changes nothing; says whether the shape changed. */
function settle(shape: any, before: { kind: unknown; points: unknown; solid: unknown }): boolean {
  const close = (a: unknown, b: unknown): boolean => {
    if (typeof a === "number" && typeof b === "number") return Math.abs(a - b) <= 0.0003;
    if (Array.isArray(a) && Array.isArray(b)) return a.length === b.length && a.every((value, i) => close(value, b[i]));
    if (a !== null && b !== null && typeof a === "object" && typeof b === "object") return Object.keys(a).length === Object.keys(b).length && Object.entries(a).every(([key, value]) => close(value, (b as Record<string, unknown>)[key]));
    return a === b;
  };
  if (!close(before, { kind: shape.kind, points: shape.points, solid: shape.solid })) return true;
  Object.assign(shape, structuredClone(before));
  return false;
}
const snapshot = (shape: any): { kind: unknown; points: unknown; solid: unknown } => structuredClone({ kind: shape.kind, points: shape.points, solid: shape.solid });

/** 🎯️ Minimises a function of a few parameters by a pattern search with halving steps. */
function search(start: readonly number[], cost: (p: readonly number[]) => number, step = 0.02, floor = 0.0001): number[] {
  const p = [...start];
  let best = cost(p);
  for (let s = step; s >= floor; s /= 2) {
    for (let moved = true, guard = 0; moved && guard < 200; guard++) {
      moved = false;
      for (let i = 0; i < p.length; i++)
        for (const d of [s, -s]) {
          p[i]! += d;
          const c = cost(p);
          if (c < best - 1e-9) (best = c), (moved = true);
          else p[i]! -= d;
        }
    }
  }
  return p;
}

const turns = flag("turns") === undefined ? undefined : (JSON.parse(await Bun.file(flag("turns")!).text()) as Record<string, { min: number; max: number; lateral: number }>);
let aimed: number[] | undefined;

/** 🎯️ Aims the footprint fit of the next op at the directions the bone really shows to the three views: the world axes seen from the bone frame over the range of sagittal angles the bone takes in the poses, clips and transitions (`--turns`, from `turns.ts`), with a margin of three degrees; a bone that also leaves its plane (spread, twist of ten degrees or more) or an asset without `--turns` is fitted in every direction. */
function aim(bone: string, frame: Frame): void {
  const turn = turns?.[bone];
  aimed = undefined;
  if (turn === undefined || turn.lateral >= 10 || turn.max - turn.min >= 170) return;
  const base = Math.atan2(frame.v[0], frame.u[0]);
  aimed = [];
  const [from, to] = [turn.min - 3, turn.max + 3];
  const steps = Math.max(2, Math.ceil((to - from) / 3));
  for (let i = 0; i <= steps; i++) for (let quarter = 0; quarter < 4; quarter++) aimed.push(base - ((from + ((to - from) * i) / steps) * Math.PI) / 180 + (quarter * Math.PI) / 2);
}

const mismatch = (a: readonly P2[], b: readonly P2[]): number => {
  let worst = 0;
  let sum = 0;
  for (const angle of ANGLES) {
    const d = Math.abs(support(a, angle) - support(b, angle));
    if (aimed === undefined) worst = Math.max(worst, d);
    sum += d;
  }
  if (aimed !== undefined) for (const angle of aimed) worst = Math.max(worst, Math.abs(support(a, angle) - support(b, angle)));
  return worst + ((aimed === undefined ? 0.05 : 0.01) * sum) / ANGLES.length;
};

const stretch = (points: readonly P2[], to: readonly [number, number]): P2[] => {
  const from = rangeOf(points, 0);
  const k = from[1] === from[0] ? 1 : (to[1] - to[0]) / (from[1] - from[0]);
  return points.map(([u, y]) => [to[0] + (u - from[0]) * k, y] as const);
};

/** ✂️ A closed outline cut to the u-interval (what lies beyond the ends of the elevation twin is hidden length inside the neighbouring part), then stretched to it where it falls short. */
const clipStretch = (points: readonly P2[], to: readonly [number, number]): P2[] => {
  let ring = [...points];
  for (const [limit, keep] of [[to[0], (u: number) => u >= to[0] - 0.00015], [to[1], (u: number) => u <= to[1] + 0.00015]] as const) {
    const next: P2[] = [];
    for (let i = 0; i < ring.length; i++) {
      const [p, q] = [ring[i]!, ring[(i + 1) % ring.length]!];
      if (keep(p[0])) next.push(p);
      if (keep(p[0]) !== keep(q[0])) next.push([limit, p[1] + ((q[1] - p[1]) * (limit - p[0])) / (q[0] - p[0])] as const);
    }
    ring = next;
  }
  const clean = ring.filter((p, i) => Math.hypot(p[0] - ring[(i + 1) % ring.length]![0], p[1] - ring[(i + 1) % ring.length]![1]) > 1e-6);
  return clean.length < 3 ? stretch(points, to) : stretch(clean, to);
};

function frontSweep(op: any): string {
  const shape = need(op.scale, "front", op.id, op.bone);
  const frame = frames[shape.bone]!.frame;
  const length = lengthOf(shape.bone);
  const twin = find(op.scale, "elevation", op.twin ?? op.id, shape.bone);
  if (twin === undefined) throw new Error(`1:${op.scale} "${op.id}" has no elevation twin on ${shape.bone}`);
  const target = footprint(twin, "elevation", frame, length, 1);
  if (target === undefined || twin.solid?.kind === "revolved") throw new Error(`1:${op.scale} "${op.id}": the elevation twin is a revolved solid, use the op "copy"`);
  if (shape.solid !== undefined && !(shape.solid.kind === "extruded" && shape.solid.sweep === "v")) throw new Error(`1:${op.scale} front "${op.id}" is a ${shape.solid.kind} solid, not a sweep along v`);
  const before = snapshot(shape);
  aim(shape.bone, frame);
  let profile = shape.solid === undefined ? frontLocal(pointsOf(shape), frame) : pointsOf(shape);
  const range = rangeOf(target, 0);
  if (op.range !== "keep") profile = op.range === "stretch" ? stretch(profile, range) : clipStretch(profile, range);
  if (op.lateral === "plan") {
    const planTwin = find(op.scale, "plan", op.planTwin ?? op.id, shape.bone);
    const across = planTwin === undefined ? undefined : footprint(planTwin, "plan", frame, length, 2);
    if (across === undefined) throw new Error(`1:${op.scale} "${op.id}": "lateral": "plan" needs a revolved or laterally extruded plan twin`);
    const flip = (points: readonly P2[]): P2[] => points.map(([x, y]) => [y, x] as const);
    profile = flip(clipStretch(flip(profile), rangeOf(across, 1)));
  }
  const [a, b] = rangeOf(profile, 0);
  const prism = (p: readonly number[]): P2[] =>
    [a, Math.min(Math.max(0, a), b), Math.max(Math.min(length, b), a), b].flatMap((u) => {
      const t = length <= 0 ? 0 : Math.min(Math.max(u / length, 0), 1);
      return [[u, p[0]! + (p[2]! - p[0]!) * t] as const, [u, p[1]! + (p[3]! - p[1]!) * t] as const];
    });
  const [v0, v1] = rangeOf(target, 1);
  const fitted = search([v0, v1, v0, v1], (p) => mismatch(prism(p), target) + (p[0]! >= p[1]! - 0.002 || p[2]! >= p[3]! - 0.002 ? 1 : 0));
  for (const key of GEOMETRY_KEYS) if (key !== "closed") delete shape[key];
  Object.assign(shape, { kind: "poly", points: profile.map(([u, w]) => [round(u, 4), round(w, 4)]), closed: true, solid: { kind: "extruded", sweep: "v", z0: round(fitted[0]!), z1: round(fitted[1]!), zTip: [round(fitted[2]!), round(fitted[3]!)] } });
  const left = mismatch(prism(fitted), target);
  return `${settle(shape, before) ? "fitted" : "unchanged"}: u [${range.map((v) => v.toFixed(3))}], span ${JSON.stringify(shape.solid)} footprint mismatch ${(left * 1000).toFixed(1)} mm`;
}

/** 🧇️ A flat front or plan outline in asset metres at rest re-expressed in the plane of two bone-local axes through the bone origin, seen along the view direction: `first` is the first profile axis (u for a sweep along v, v for a sweep along u), the second is always w. */
function viewLocal(points: readonly P2[], frame: Frame, view: "front" | "plan", first: V3): P2[] {
  const [i, j] = view === "front" ? [1, 2] : [0, 2];
  const det = first[i]! * frame.w[j]! - frame.w[i]! * first[j]!;
  if (Math.abs(det) < 0.2) throw new Error(`the profile plane of the bone is nearly edge-on in the ${view}: no outline of this view can be swept along that axis`);
  return points.map((point) => {
    const [di, dj] = view === "front" ? [point[1] - frame.o[1], point[0] - frame.o[2]] : [point[0] - frame.o[0], point[1] - frame.o[2]];
    return [(di * frame.w[j]! - dj * frame.w[i]!) / det, (first[i]! * dj - first[j]! * di) / det] as const;
  });
}

/** 🧈️ The op `view-sweep`: a front or plan shape keeps its own outline and becomes the prism of that outline swept along the bone axis nearest to the view direction (u: a cross-section of a bone that points at the viewer; v: the outline of a bone seen from its normal), the outline cut to the range of the elevation twin on its first axis and the sweep span fitted to the (u, v) footprint of the elevation twin. */
function viewSweep(op: any): string {
  const view = op.view as "front" | "plan";
  const shape = need(op.scale, view, op.id, op.bone);
  const frame = frames[shape.bone]!.frame;
  const length = lengthOf(shape.bone);
  const twin = find(op.scale, "elevation", op.twin ?? op.id, shape.bone);
  if (twin === undefined) throw new Error(`1:${op.scale} "${op.id}" has no elevation twin on ${shape.bone}`);
  if (twin.solid?.kind === "revolved") throw new Error(`1:${op.scale} "${op.id}": the elevation twin is a revolved solid, use the op "copy"`);
  const target = footprint(twin, "elevation", frame, length, 1)!;
  aim(shape.bone, frame);
  const d = view === "front" ? 0 : 1;
  const sweep: "u" | "v" = op.sweep ?? (Math.abs(frame.u[d]!) >= Math.abs(frame.v[d]!) ? "u" : "v");
  if (shape.solid !== undefined && !(shape.solid.kind === "extruded" && shape.solid.sweep === sweep)) throw new Error(`1:${op.scale} ${view} "${op.id}" is a ${shape.solid.kind} solid${shape.solid.sweep ? ` swept along ${shape.solid.sweep}` : ""}, not a sweep along ${sweep}`);
  if (shape.solid === undefined && shape.kind !== "poly" && flattenGeometry(shape).rings.length !== 1) throw new Error(`1:${op.scale} ${view} "${op.id}" has several rings`);
  const before = snapshot(shape);
  let profile = shape.solid === undefined ? viewLocal(pointsOf(shape), frame, view, sweep === "u" ? frame.v : frame.u) : pointsOf(shape);
  const k = sweep === "u" ? 1 : 0;
  const range = rangeOf(target, k);
  if (op.range !== "keep") profile = op.range === "stretch" ? stretch(profile, range) : clipStretch(profile, range);
  if (op.hull === true) profile = hullOf(profile);
  if (op.lateral !== undefined) {
    const other = find(op.scale, op.lateral, op.lateralTwin ?? op.id, shape.bone);
    const across = other === undefined ? undefined : lateralRange(other, op.lateral, frame);
    if (across === undefined) throw new Error(`1:${op.scale} "${op.id}": no lateral range in the ${op.lateral} twin`);
    const flip = (points: readonly P2[]): P2[] => points.map(([x, y]) => [y, x] as const);
    profile = flip(clipStretch(flip(profile), across));
  }
  const [a, b] = rangeOf(profile, 0);
  const stations = [a, Math.min(Math.max(0, a), b), Math.max(Math.min(length, b), a), b];
  const prism = (p: readonly number[]): P2[] =>
    stations.flatMap((s) => {
      const t = length <= 0 ? 0 : Math.min(Math.max(s / length, 0), 1);
      const [lo, hi] = [p[0]! + (p[2]! - p[0]!) * t, p[1]! + (p[3]! - p[1]!) * t];
      return sweep === "v" ? [[s, lo] as const, [s, hi] as const] : [[lo, s] as const, [hi, s] as const];
    });
  const [s0, s1] = rangeOf(target, sweep === "v" ? 1 : 0);
  const taper = op.taper !== false;
  const fitted = search([s0, s1, s0, s1], (p) => mismatch(prism(taper ? p : [p[0]!, p[1]!, p[0]!, p[1]!]), target) + (p[0]! >= p[1]! - 0.002 || p[2]! >= p[3]! - 0.002 ? 1 : 0));
  if (!taper) [fitted[2], fitted[3]] = [fitted[0]!, fitted[1]!];
  for (const key of GEOMETRY_KEYS) if (key !== "closed") delete shape[key];
  Object.assign(shape, { kind: "poly", points: profile.map(([x, y]) => [round(x, 4), round(y, 4)]), closed: true, solid: { kind: "extruded", sweep, z0: round(fitted[0]!), z1: round(fitted[1]!), ...(taper ? { zTip: [round(fitted[2]!), round(fitted[3]!)] } : {}) } });
  const left = mismatch(prism(fitted), target);
  return `${settle(shape, before) ? "fitted" : "unchanged"}: sweep ${sweep}, first axis [${range.map((v) => v.toFixed(3))}], span ${JSON.stringify(shape.solid)} footprint mismatch ${(left * 1000).toFixed(1)} mm`;
}

/** ↔️ The bone-local lateral range (w) a shape of a view covers at rest. */
function lateralRange(shape: any, view: string, frame: Frame): [number, number] | undefined {
  const solid = shape.solid;
  if (solid?.kind === "extruded" && solid.sweep === "w") return [Math.min(solid.z0, solid.zTip?.[0] ?? solid.z0) - frame.o[2], Math.max(solid.z1, solid.zTip?.[1] ?? solid.z1) - frame.o[2]];
  if (solid?.kind === "extruded") return rangeOf(pointsOf(shape), 1);
  if (solid !== undefined || view === "elevation") return undefined;
  return rangeOf(pointsOf(shape).map((point) => [0, (view === "front" ? point[0] : point[1]) - frame.o[2]] as const), 1);
}

/** 🥚️ The convex hull of an outline (monotone chain, counter-clockwise): the prism of the hull has the extents of the prism of the outline in every direction, and the union the evaluator builds of it cannot fail on a notch. */
function hullOf(points: readonly P2[]): P2[] {
  const sorted = [...points].sort((p, q) => p[0] - q[0] || p[1] - q[1]);
  const turn = (o: P2, a: P2, b: P2): number => (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0]);
  const half = (list: readonly P2[]): P2[] => {
    const chain: P2[] = [];
    for (const p of list) {
      while (chain.length >= 2 && turn(chain[chain.length - 2]!, chain[chain.length - 1]!, p) <= 1e-12) chain.pop();
      chain.push(p);
    }
    chain.pop();
    return chain;
  };
  return [...half(sorted), ...half([...sorted].reverse())];
}

/** 🪟️ The outline of a flat front or plan shape in the (u, w) plane of its bone, undefined where the bone points at the viewer rather than across the view (the outline is a cross-section then and only its lateral range counts). */
function tryLocal(shape: any, view: "front" | "plan", frame: Frame): P2[] | undefined {
  const d = view === "front" ? 0 : 1;
  if (Math.abs(frame.u[d]!) >= Math.abs(frame.v[d]!)) return undefined;
  try {
    return viewLocal(pointsOf(shape), frame, view, frame.u);
  } catch {
    return undefined;
  }
}

function frontExtrude(op: any): string {
  aimed = undefined;
  const shape = need(op.scale, op.view ?? "front", op.id, op.bone);
  const frame = frames[shape.bone]!.frame;
  const length = lengthOf(shape.bone);
  const twin = find(op.scale, "elevation", op.twin ?? op.id, shape.bone);
  if (twin === undefined || twin.solid?.kind === "revolved") throw new Error(`1:${op.scale} "${op.id}" needs a flat or extruded elevation twin on ${shape.bone}`);
  const before = snapshot(shape);
  const profile = hullOf(pointsOf(twin)).map(([u, v]) => [round(u, 5), round(v, 5)] as const);
  const [a, b] = rangeOf(profile, 0);
  const edgeOn = Math.abs(frame.u[1] * frame.w[2] - frame.w[1] * frame.u[2]) < 0.2;
  const own = undefined as P2[] | undefined;
  const lateral = shape.solid !== undefined || own !== undefined ? undefined : op.view === "plan" ? rangeOf(pointsOf(shape), 1).map((z) => z - frame.o[2]) : edgeOn ? rangeOf(pointsOf(shape), 0).map((z) => z - frame.o[2]) : undefined;
  const other = op.across === undefined ? undefined : find(op.scale, op.across, op.acrossTwin ?? op.id, shape.bone);
  const lent = other === undefined || other === shape ? undefined : other.solid === undefined ? tryLocal(other, op.across, frame) : footprint(other, op.across, frame, length, 2);
  const mine = shape.solid === undefined && op.taper !== false ? tryLocal(shape, op.view ?? "front", frame) : undefined;
  const borrowed = (op.view ?? "front") === "plan" ? (lent ?? mine) : (mine ?? lent);
  const across = shape.solid?.kind === "extruded" && shape.solid.sweep === "w" ? undefined : borrowed !== undefined ? clipStretch(borrowed, [a, b]) : own !== undefined ? clipStretch(own, [a, b]) : lateral !== undefined ? [a, b].flatMap((u) => lateral.map((w) => [u, w] as const)) : footprint(shape, op.view ?? "front", frame, length, 2);
  if (across !== undefined) {
    const prism = (p: readonly number[]): P2[] =>
      [a, Math.min(Math.max(0, a), b), Math.max(Math.min(length, b), a), b].flatMap((u) => {
        const t = length <= 0 ? 0 : Math.min(Math.max(u / length, 0), 1);
        return [[u, p[0]! + (p[2]! - p[0]!) * t] as const, [u, p[1]! + (p[3]! - p[1]!) * t] as const];
      });
    const [w0, w1] = rangeOf(across, 1);
    const fitted = search([w0, w1, w0, w1], (p) => mismatch(prism(op.taper === false ? [p[0]!, p[1]!, p[0]!, p[1]!] : p), across) + (p[0]! >= p[1]! - 0.002 || p[2]! >= p[3]! - 0.002 ? 1 : 0));
    if (op.taper === false) [fitted[2], fitted[3]] = [fitted[0]!, fitted[1]!];
    const z = frame.o[2];
    for (const key of GEOMETRY_KEYS) if (key !== "closed") delete shape[key];
    Object.assign(shape, { kind: "poly", points: structuredClone(profile), closed: true, solid: { kind: "extruded", sweep: "w", z0: round(fitted[0]! + z), z1: round(fitted[1]! + z), zTip: [round(fitted[2]! + z), round(fitted[3]! + z)] } });
  } else Object.assign(shape, { points: structuredClone(profile) });
  return `${settle(shape, before) ? "extruded" : "unchanged"}: the elevation outline swept laterally, span ${JSON.stringify(shape.solid)}`;
}

function copyGeometry(source: any, target: any): void {
  for (const key of GEOMETRY_KEYS) delete target[key];
  target.kind = source.kind;
  for (const key of GEOMETRY_KEYS) if (source[key] !== undefined) target[key] = structuredClone(source[key]);
}

/** 📦️ The largest difference of the lateral ranges of two point sets (the comparison the matrix makes across the bone). */
const boxMiss = (a: readonly P2[], b: readonly P2[]): number => Math.max(Math.abs(rangeOf(a, 1)[0] - rangeOf(b, 1)[0]), Math.abs(rangeOf(a, 1)[1] - rangeOf(b, 1)[1]));

function planRevolve(op: any): string {
  aimed = undefined;
  const view: "front" | "plan" = op.view ?? "plan";
  const shape = need(op.scale, view, op.id, op.bone);
  const frame = frames[shape.bone]!.frame;
  const length = lengthOf(shape.bone);
  const twin = find(op.scale, "elevation", op.twin ?? op.id, shape.bone);
  if (twin === undefined) throw new Error(`1:${op.scale} "${op.id}" has no elevation twin on ${shape.bone}`);
  const before = snapshot(shape);
  if (twin.solid?.kind === "revolved") {
    copyGeometry(twin, shape);
    return `${settle(shape, before) ? "copied" : "unchanged"}: the revolved elevation twin`;
  }
  const side = footprint(twin, "elevation", frame, length, 1)!;
  const otherView = view === "plan" ? "front" : "plan";
  const frontTwin = find(op.scale, otherView, op.frontTwin ?? op.id, shape.bone);
  const across = (shape.solid === undefined ? tryLocal(shape, view, frame) : undefined) ?? (frontTwin === undefined ? undefined : frontTwin.solid === undefined ? tryLocal(frontTwin, otherView, frame) : footprint(frontTwin, otherView, frame, length, 2));
  const range = rangeOf(side, 0);
  const box = across === undefined && shape.solid === undefined ? lateralRange(shape, view, frame) : undefined;
  const acrossBox = across ?? (box === undefined ? undefined : ([[range[0], box[0]], [range[0], box[1]], [range[1], box[0]], [range[1], box[1]]] as P2[]));
  let radial: P2[];
  if (shape.solid?.kind === "revolved" && op.rebuild !== true) radial = op.range === "stretch" ? stretch(revolvedOf(shape, frame).radial, range) : clipStretch(revolvedOf(shape, frame).radial, range);
  else {
    const n = op.stations ?? 9;
    const half = (points: readonly P2[], u: number): number | undefined => {
      const hits: number[] = [];
      for (let i = 0; i < points.length; i++) {
        const [p, q] = [points[i]!, points[(i + 1) % points.length]!];
        if (Math.min(p[0], q[0]) > u || Math.max(p[0], q[0]) < u) continue;
        if (q[0] === p[0]) hits.push(p[1], q[1]);
        else hits.push(p[1] + ((q[1] - p[1]) * (u - p[0])) / (q[0] - p[0]));
      }
      return hits.length < 2 ? undefined : (Math.max(...hits) - Math.min(...hits)) / 2;
    };
    radial = Array.from({ length: n }, (_, i) => {
      const u = range[0] + ((range[1] - range[0]) * (i === 0 ? 0.02 : i === n - 1 ? n - 1.02 : i)) / (n - 1);
      const values = [half(pointsOf(twin), u), across === undefined ? undefined : half(across, u)].filter((value): value is number => value !== undefined);
      return [i === 0 ? range[0] : i === n - 1 ? range[1] : u, values.length === 0 ? 0 : values.reduce((x, y) => x + y, 0) / values.length] as const;
    });
  }
  const band = (centre: number, scale: number): P2[] => radial.flatMap(([u, r]) => [[u, centre - r * scale] as const, [u, centre + r * scale] as const]);
  const mid = (points: readonly P2[]): number => (rangeOf(points, 1)[0] + rangeOf(points, 1)[1]) / 2;
  const cost = (p: readonly number[]): number => Math.max(mismatch(band(p[0]!, p[2]!), side), acrossBox === undefined ? 0 : boxMiss(band(p[1]!, p[2]!), acrossBox)) + (p[2]! < 0.2 ? 1 : 0);
  const held = shape.solid?.kind === "revolved" && op.rebuild !== true ? [revolvedOf(shape, frame).av, revolvedOf(shape, frame).aw, 1] : undefined;
  const found = search(held ?? [mid(side), acrossBox === undefined ? 0 : mid(acrossBox), 1], cost, 0.02, 0.00005);
  const [av, aw, scale] = (held !== undefined && cost(held) - cost(found) < 0.0003 ? held : found) as [number, number, number];
  const t = Math.abs(frame.u[1]) < 0.05 ? 0 : -(av * frame.v[1] + aw * frame.w[1]) / frame.u[1];
  const offset: V3 = [av * frame.v[0] + aw * frame.w[0] + t * frame.u[0], 0, av * frame.v[2] + aw * frame.w[2] + t * frame.u[2]];
  if (Math.abs(dot(offset, frame.v) - av) > 0.0005) throw new Error(`1:${op.scale} "${op.id}": the bone lies level, so the axis of a revolved solid cannot be offset along its normal by ${(av * 1000).toFixed(0)} mm (the offset of an axis is horizontal): use the op "extrude" with "view": "plan"`);
  for (const key of GEOMETRY_KEYS) if (key !== "closed") delete shape[key];
  const skew = round(offset[2]) - aw;
  const outline = [...radial.map(([u, r]) => [round(u, 4), round(av + Math.sqrt(Math.max(0, (r * scale) ** 2 - skew ** 2)), 4)]), ...(radial.some(([, r]) => r * scale < Math.abs(skew) + 0.0005) ? [] : [[round(range[1], 4), round(av, 4)], [round(range[0], 4), round(av, 4)]])];
  Object.assign(shape, { kind: "poly", points: outline, closed: true, solid: { kind: "revolved", axisBone: shape.bone, axis: "u", axisX: round(offset[0]), axisZ: round(offset[2]), mirror: true } });
  const left = [mismatch(band(av, scale), side), acrossBox === undefined ? 0 : boxMiss(band(aw, scale), acrossBox)];
  return `${settle(shape, before) ? "fitted" : "unchanged"}: u [${range.map((v) => v.toFixed(3))}], axis v ${av.toFixed(4)} w ${aw.toFixed(4)}, radius × ${scale.toFixed(3)}, footprint mismatch side ${(left[0]! * 1000).toFixed(1)} mm, front ${(left[1]! * 1000).toFixed(1)} mm`;
}

function copy(op: any): string {
  const source = need(op.scale, op.from, op.id, op.bone);
  const done: string[] = [];
  for (const view of op.to as string[]) {
    let target = find(op.scale, view, op.as ?? op.id, source.bone);
    if (target === undefined) {
      if (op.create !== true) throw new Error(`1:${op.scale} ${view} has no shape "${op.as ?? op.id}" on ${source.bone} (state "create": true to add it)`);
      target = { ...structuredClone(source), id: op.as ?? op.id, ...(op.paint ?? {}) };
      shapesOf(op.scale, view).push(target);
      done.push(`${view} created`);
      continue;
    }
    const before = JSON.stringify(target);
    copyGeometry(source, target);
    done.push(`${view} ${JSON.stringify(target) === before ? "unchanged" : "copied"}`);
  }
  return done.join(", ");
}

/** 🧮️ The tolerance of the matrix for the asset (D18): 2 % of its largest dimension, at least 15 mm. */
const toleranceM = Math.max(0.015, 0.02 * Math.max(asset.dimensions.widthM, asset.dimensions.heightM, asset.dimensions.depthM));

/** 🧩️ The op `solid`: `view-sweep` where the prism of the view's own outline reproduces the elevation footprint within `limit` (a share of the tolerance of the asset, default 0.4) over the directions the bone takes, else `extrude` (the elevation outline swept laterally, exact at every pose), its lateral taper taken from the twin of `across` when that is stated; a shape that already is a solid is refitted by the op of its sweep. */
const LIMIT = Number(flag("limit") ?? 0.4);

function solidOp(op: any): string {
  const shape = need(op.scale, op.view, op.id, op.bone);
  if (shape.solid !== undefined) return "unchanged: a solid already";
  const kept = structuredClone(shape);
  const restore = (): void => {
    for (const key of Object.keys(shape)) delete shape[key];
    Object.assign(shape, structuredClone(kept));
  };
  const limit = (op.limit ?? LIMIT) * toleranceM * 1000;
  const driftLimit = (op.drift ?? 1) * 0.15 * Number(op.scale);
  const was = restRings(op.scale, op.view, shape);
  const tried: { kind: string; state: any; score: number; note: string }[] = [];
  for (const kind of (op.order ?? ["view-sweep", "revolve", "extrude"]) as string[]) {
    try {
      const result = kind === "extrude" ? frontExtrude(op) : kind === "view-sweep" ? viewSweep(op) : planRevolve({ ...op, across: undefined });
      const left = kind === "extrude" ? 0 : kind === "view-sweep" ? Number(/footprint mismatch ([\d.]+) mm/.exec(result)![1]) : Math.max(Number(/side ([\d.]+) mm/.exec(result)![1]), Number(/front ([\d.]+) mm/.exec(result)![1]));
      const drift = hausdorff(was, restRings(op.scale, op.view, shape)) * 1000;
      const note = `${kind} footprint ${left.toFixed(0)} mm, rest outline ${drift.toFixed(0)} mm`;
      if (left <= limit && drift <= driftLimit) return `fitted: ${note}${tried.length > 0 ? `; not ${tried.map((entry) => entry.note).join("; not ")}` : ""}`;
      tried.push({ kind, state: structuredClone(shape), score: Math.max(left / limit, drift / driftLimit), note });
    } catch (error) {
      tried.push({ kind, state: undefined, score: Number.POSITIVE_INFINITY, note: `${kind} refused (${(error as Error).message.slice(0, 60)})` });
    }
    restore();
  }
  const best = tried.filter((entry) => entry.state !== undefined).sort((x, y) => x.score - y.score)[0];
  if (best === undefined || op.strict === true) return `unchanged: no construction within footprint ${limit.toFixed(0)} mm and rest outline ${driftLimit.toFixed(0)} mm (${tried.map((entry) => entry.note).join("; ")})`;
  for (const key of Object.keys(shape)) delete shape[key];
  Object.assign(shape, best.state);
  return `fitted, COMPROMISE: ${best.note}; not ${tried.filter((entry) => entry !== best).map((entry) => entry.note).join("; not ")}`;
}

/** 🖼️ The rings a shape draws in a view in the default pose, by the evaluator itself. */
function restRings(scale: string, view: string, shape: any): P2[][] {
  const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, coverage.scaleSelector(Number(scale)));
  const drawn = state.shapes[view].find((entry: any) => entry.shape === shape || (entry.shape.id === shape.id && entry.shape.bone === shape.bone));
  if (drawn === undefined) throw new Error(`"${shape.id}" is not drawn in the ${view}`);
  return drawn.rings.map((ring: any[]) => ring.map((point) => [point.x, point.y] as const));
}

/** 📏️ The symmetric Hausdorff distance of two sets of closed rings (boundaries sampled every 2 mm). */
function hausdorff(a: readonly P2[][], b: readonly P2[][]): number {
  const sample = (rings: readonly P2[][]): P2[] =>
    rings.flatMap((ring) =>
      ring.flatMap((p, i) => {
        const q = ring[(i + 1) % ring.length]!;
        const n = Math.max(1, Math.ceil(Math.hypot(q[0] - p[0], q[1] - p[1]) / 0.002));
        return Array.from({ length: n }, (_, k) => [p[0] + ((q[0] - p[0]) * k) / n, p[1] + ((q[1] - p[1]) * k) / n] as const);
      }),
    );
  const [sa, sb] = [sample(a), sample(b)];
  const one = (from: readonly P2[], to: readonly P2[]): number => {
    let worst = 0;
    for (const p of from) {
      let best = Number.POSITIVE_INFINITY;
      for (const q of to) {
        const d = (p[0] - q[0]) ** 2 + (p[1] - q[1]) ** 2;
        if (d < best) best = d;
      }
      if (best > worst) worst = best;
    }
    return Math.sqrt(worst);
  };
  return Math.max(one(sa, sb), one(sb, sa));
}

function apply(op: any): string {
  switch (op.op) {
    case "solid": return solidOp(op);
    case "front-sweep": return frontSweep(op);
    case "view-sweep": return viewSweep(op);
    case "plan-revolve": return planRevolve(op);
    case "extrude": return frontExtrude(op);
    case "copy": return copy(op);
    case "set": {
      const shape = need(op.scale, op.view, op.id, op.bone);
      const before = JSON.stringify(shape);
      for (const [key, value] of Object.entries(op.set)) value === null ? delete shape[key] : (shape[key] = structuredClone(value));
      return JSON.stringify(shape) === before ? "unchanged" : "set";
    }
    case "remove": {
      const list = shapesOf(op.scale, op.view);
      const index = list.findIndex((shape) => shape.id === op.id && (op.bone === undefined || shape.bone === op.bone));
      if (index < 0) return "unchanged";
      list.splice(index, 1);
      return "removed";
    }
    case "anchor": {
      const anchor = asset.anchors.find((candidate: any) => candidate.id === op.id);
      if (anchor === undefined) throw new Error(`no anchor "${op.id}"`);
      const before = JSON.stringify(anchor);
      for (const [key, value] of Object.entries(op.set)) value === null ? delete anchor[key] : (anchor[key] = value);
      return JSON.stringify(anchor) === before ? "unchanged" : "set";
    }
    default: throw new Error(`unknown op "${op.op}"`);
  }
}

let changed = 0;
let refused = 0;
for (const op of ops) {
  let result: string;
  try {
    result = apply(op);
  } catch (error) {
    refused++;
    result = `unchanged, REFUSED: ${(error as Error).message}`;
  }
  if (!result.startsWith("unchanged") && !/^(\w+ unchanged(, )?)+$/.test(result)) changed++;
  console.log(`${asset.id} ${op.op} ${op.scale ? `1:${op.scale} ` : ""}${op.view ?? ""}${op.view ? " " : ""}${op.id}: ${result}`);
}
if (args.includes("--write") || flag("out") !== undefined) console.log(writeDoc(read, flag("out")) ? `written: ${flag("out") ?? read.file}` : "file unchanged");
console.log(`${asset.id}: ${ops.length} ops, ${changed} changed something, ${refused} refused`);
process.exit(refused === 0 ? 0 : 1);
