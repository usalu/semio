/** 🧵️ The multiview repair of batch MH-2: `../MR/fit.ts` (same ops, same behaviour) with two further ops for what it could not build.
 * - `{ "op": "plan-prism", scale, id }`: the plan shape of an upright part (pelvis, trunk, head) becomes an `extruded` solid with `sweep: "u"`: its plan outline is kept as the transverse (v, w) profile (a flat outline is first re-expressed in the bone frame at the middle of the sweep) and swept along the bone over the u-range of the elevation twin, `z0`, `z1` (and `zTip` where the elevation outline is skew by more than 2 mm) fitted to the (u, v) footprint of the elevation twin; the plan then shows the true length of the part when the bone pitches.
 * - `{ "op": "elevation-extrude", scale, id, "z"?: [z0, z1] }`: a flat elevation shape on a bone that a pose twists about its own axis becomes an `extruded` solid with `sweep: "w"` over the lateral span of its front twin (or of its plan twin, or `z`), so the side view widens with the twist as front and plan do.
 * - `{ "op": "plan-core", scale, id, "as": coreId, "half": metres }`: for a trunk whose plan outline no prism reproduces (a rounded outline over a long skew coat): the flat plan outline stays and a second plan shape `coreId` is added beneath it on the same bone (paint order `z` just below the outline), the elevation outline swept laterally over ±`half` (inside the outline at rest), so the plan reaches as far ahead and behind as the side view when the trunk pitches. The bone then has two plan shapes: the matrix reports it in the group tier.
 *   With `"z": [z0, z1]` the core takes that lateral span (rest pose, asset coordinates) instead of ±`half` about the bone, and with `"view": "front"` it is added to the front view beneath the front outline of that id: the op for a hand, an arm or a shoe whose flat outline is right in the default pose and only falls short of the side view when the limb swings (the core is the slab of the part at its true thickness; the outline above it keeps what the slab does not draw, a thumb or a toe cap).
 * - `{ "op": "plan-slabs", scale, id, "as": prefix, "slices": n }`: for a flat plan outline that is right at rest and only falls short of the side view when its bone pitches, and that lies askew in the plan (a hanging hand turned on its arm, a shoe), where one core would leave the outline: the elevation outline is cut into n bands along the bone axis that points ahead or behind at rest, every band becomes a prism `prefix-i` (the convex hull of the outline in the band, cut to the forward range the plan outline has at rest) swept laterally over the widest lateral span that the plan outline covers along the whole band (2 mm inside it), beneath the outline. Bands the outline does not cover over at least 6 mm get no box; boxes of an earlier run that are no longer made are removed.
 * - `{ "op": "plan-depth", scale, id }`: a flat plan outline that was drawn for a leaning default pose (deeper than the part is thick) is scaled along the forward axis to the rest depth of its elevation twin (cause G: equal rest extents); the reach in a leaning pose then comes from the core solid of `plan-core`.
 * `plan-revolve` takes `"radius": "front"`: the radius profile and the lateral place of the axis come from the front twin alone (the width the plan shows), the axis sits in the middle of the elevation outline; for a limb of elliptic section, where the mean of width and thickness misses the plan by their difference.
 * `copy` takes `"solid": { view: solid }` to give a copied twin its own span in that view (the same profile over another lateral span), in one idempotent step.
 * `extrude` of a flat front shape always takes the lateral range the shape draws (`fit.ts` did so only for a bone that points at the viewer and otherwise fitted a re-expressed outline, which doubles the span on a bone that leans 60 degrees); `"fit": true` asks for that fit on an upright bone, where it gives the prism the taper of the front outline (`zTip`).
 * One correction to `extrude`: a shape that is a sweep along u (a transverse profile) gives its lateral span from that profile; `fit.ts` kept its solid and only replaced the points.
 * Original header: `bun fit.ts <id|file.json> --ops <ops.json> [--write] [--out <file>]` applies the ops of the file in order and prints what each did (an op that cannot be applied is refused by name, the others still run, exit code 1); without `--write` nothing is written. Every op takes the ELEVATION twin (the shape of the same `id` and bone in `geometry.elevation`, or `twin`) as the profile of the part and rebuilds or refits the front or plan shape from it, so a second run changes nothing.
 *
 * Ops (`scale` is the representation key, `id` the shape id in the target view, `bone` only when the id is not unique):
 * - `{ "op": "front-sweep", scale, id }`: the front shape becomes (or stays) an `extruded` solid with `sweep: "v"`; its outline is kept (a flat front outline is first re-expressed in the bone-local (u, w) plane), cut along the bone to the u-range of the elevation twin (stretched to it where it falls short; `"range": "stretch"` only stretches, `"keep"` leaves it; `"lateral": "plan"` cuts it the same way across the bone to the lateral range of the plan twin), and `z0`, `z1`, `zTip` are fitted so that the (u, v) footprint of the prism has the support function of the elevation outline in every direction (so the heights agree at every sagittal angle).
 * - `{ "op": "extrude", scale, id, "view": "front" | "plan" }`: the exact alternative for a part whose front outline is a plain taper (and for the plan of a part that is a prism): the shape takes the elevation outline as its profile and becomes an `extruded` solid with `sweep: "w"` whose lateral span `z0`, `z1`, `zTip` is fitted to the (u, w) footprint the shape had; elevation and this view then agree at every pose by construction, and the view draws the hull of the swept outline instead of its own outline. It is also the op for a bone that points at the viewer of the front view (a foot, a toe): there no front outline can be swept along v and `front-sweep` refuses.
 * - `{ "op": "plan-revolve", scale, id }`: the plan shape becomes (or stays) a `revolved` solid about the bone axis. An elevation twin that is itself revolved is copied (one body in three views). Else an existing revolved profile is kept in shape, cut or stretched to the u-range of the elevation twin, and its axis offset and radius scale are fitted to the (u, v) footprint of the elevation twin and the (u, w) footprint of the front twin; a flat plan shape gets a new profile of the mean half thickness of the two twins at nine stations (`"stations": n`).
 * - `{ "op": "copy", scale, id, "from": view, "to": [view…] }`: the geometry and the solid of the shape in `from` replace those of its twins in `to` (the same revolved or extruded body in several views); `"create": true` adds a missing twin with the paint fields of the source.
 * - `{ "op": "set", scale, view, id, "set": {…} }` and `{ "op": "remove", scale, view, id }`: a hand-made correction of single fields, or the removal of a shape.
 * - `{ "op": "anchor", id, "set": {…} }`: binds an anchor (`bone`, `x`, `y`, `z`).
 * @see ./gate.ts — the measurement that says which bones need an op */
import { join } from "node:path";
import { S, dot, readDoc, round, sub, support, writeDoc, type V3 } from "../MR/lib.ts";

const { restFrames } = await import(join(S, "🔨️modules", "🦴️rig", "🟦️.ts"));
const { flattenGeometry } = await import(join(S, "🔨️modules", "🧮️evaluation", "🟦️.ts"));

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

const mismatch = (a: readonly P2[], b: readonly P2[]): number => {
  let worst = 0;
  let sum = 0;
  for (const angle of ANGLES) {
    const d = Math.abs(support(a, angle) - support(b, angle));
    worst = Math.max(worst, d);
    sum += d;
  }
  return worst + (0.05 * sum) / ANGLES.length;
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

function frontExtrude(op: any): string {
  const shape = need(op.scale, op.view ?? "front", op.id, op.bone);
  const frame = frames[shape.bone]!.frame;
  const length = lengthOf(shape.bone);
  const twin = find(op.scale, "elevation", op.twin ?? op.id, shape.bone);
  if (twin === undefined || twin.solid?.kind === "revolved") throw new Error(`1:${op.scale} "${op.id}" needs a flat or extruded elevation twin on ${shape.bone}`);
  const before = snapshot(shape);
  const profile = pointsOf(twin);
  const [a, b] = rangeOf(profile, 0);
  const edgeOn = Math.abs(frame.u[1] * frame.w[2] - frame.w[1] * frame.u[2]) < 0.2;
  const lateral = shape.solid?.kind === "extruded" && shape.solid.sweep === "u" ? rangeOf(pointsOf(shape), 1) : shape.solid !== undefined ? undefined : op.view === "plan" ? rangeOf(pointsOf(shape), 1).map((z) => z - frame.o[2]) : op.fit === true && !edgeOn ? undefined : rangeOf(pointsOf(shape), 0).map((z) => z - frame.o[2]);
  const across = shape.solid?.kind === "extruded" && shape.solid.sweep === "w" ? undefined : lateral !== undefined ? [a, b].flatMap((u) => lateral.map((w) => [u, w] as const)) : footprint(shape, op.view ?? "front", frame, length, 2);
  if (across !== undefined) {
    const prism = (p: readonly number[]): P2[] =>
      [a, Math.min(Math.max(0, a), b), Math.max(Math.min(length, b), a), b].flatMap((u) => {
        const t = length <= 0 ? 0 : Math.min(Math.max(u / length, 0), 1);
        return [[u, p[0]! + (p[2]! - p[0]!) * t] as const, [u, p[1]! + (p[3]! - p[1]!) * t] as const];
      });
    const [w0, w1] = rangeOf(across, 1);
    const fitted = search([w0, w1, w0, w1], (p) => mismatch(prism(p), across) + (p[0]! >= p[1]! - 0.002 || p[2]! >= p[3]! - 0.002 ? 1 : 0));
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

function planRevolve(op: any): string {
  const shape = need(op.scale, "plan", op.id, op.bone);
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
  const frontTwin = find(op.scale, "front", op.frontTwin ?? op.id, shape.bone);
  const across = frontTwin === undefined ? undefined : footprint(frontTwin, "front", frame, length, 2);
  const range = rangeOf(side, 0);
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
      const values = [op.radius === "front" && across !== undefined ? undefined : half(pointsOf(twin), u), across === undefined ? undefined : half(across, u)].filter((value): value is number => value !== undefined);
      return [i === 0 ? range[0] : i === n - 1 ? range[1] : u, values.length === 0 ? 0 : values.reduce((x, y) => x + y, 0) / values.length] as const;
    });
  }
  const band = (centre: number, scale: number): P2[] => radial.flatMap(([u, r]) => [[u, centre - r * scale] as const, [u, centre + r * scale] as const]);
  const mid = (points: readonly P2[]): number => (rangeOf(points, 1)[0] + rangeOf(points, 1)[1]) / 2;
  const cost = (p: readonly number[]): number => (op.radius === "front" && across !== undefined ? mismatch(band(p[1]!, p[2]!), across) + Math.abs(p[0]! - mid(side)) : Math.max(mismatch(band(p[0]!, p[2]!), side), mismatch(band(p[1]!, p[2]!), across ?? band(p[1]!, p[2]!)))) + (p[2]! < 0.2 ? 1 : 0);
  const held = shape.solid?.kind === "revolved" && op.rebuild !== true ? [revolvedOf(shape, frame).av, revolvedOf(shape, frame).aw, 1] : undefined;
  const found = search(held ?? [mid(side), across === undefined ? 0 : mid(across), 1], cost, 0.02, 0.00005);
  const [av, aw, scale] = (held !== undefined && cost(held) - cost(found) < 0.0003 ? held : found) as [number, number, number];
  const t = Math.abs(frame.u[1]) < 0.05 ? 0 : -(av * frame.v[1] + aw * frame.w[1]) / frame.u[1];
  const offset: V3 = [av * frame.v[0] + aw * frame.w[0] + t * frame.u[0], 0, av * frame.v[2] + aw * frame.w[2] + t * frame.u[2]];
  if (Math.abs(dot(offset, frame.v) - av) > 0.0005) throw new Error(`1:${op.scale} "${op.id}": the bone lies level, so the axis of a revolved solid cannot be offset along its normal by ${(av * 1000).toFixed(0)} mm (the offset of an axis is horizontal): use the op "extrude" with "view": "plan"`);
  for (const key of GEOMETRY_KEYS) if (key !== "closed") delete shape[key];
  const skew = round(offset[2]) - aw;
  const outline = [...radial.map(([u, r]) => [round(u, 4), round(av + Math.sqrt(Math.max(0, (r * scale) ** 2 - skew ** 2)), 4)]), ...(radial.some(([, r]) => r * scale < Math.abs(skew) + 0.0005) ? [] : [[round(range[1], 4), round(av, 4)], [round(range[0], 4), round(av, 4)]])];
  Object.assign(shape, { kind: "poly", points: outline, closed: true, solid: { kind: "revolved", axisBone: shape.bone, axis: "u", axisX: round(offset[0]), axisZ: round(offset[2]), mirror: true } });
  const left = [mismatch(band(av, scale), side), across === undefined ? 0 : mismatch(band(aw, scale), across)];
  return `${settle(shape, before) ? "fitted" : "unchanged"}: u [${range.map((v) => v.toFixed(3))}], axis v ${av.toFixed(4)} w ${aw.toFixed(4)}, radius × ${scale.toFixed(3)}, footprint mismatch side ${(left[0]! * 1000).toFixed(1)} mm, front ${(left[1]! * 1000).toFixed(1)} mm`;
}

function copy(op: any): string {
  const source = need(op.scale, op.from, op.id, op.bone);
  const done: string[] = [];
  for (const view of op.to as string[]) {
    let target = find(op.scale, view, op.as ?? op.id, source.bone);
    if (target === undefined) {
      if (op.create !== true) throw new Error(`1:${op.scale} ${view} has no shape "${op.as ?? op.id}" on ${source.bone} (state "create": true to add it)`);
      target = { ...structuredClone(source), id: op.as ?? op.id, ...(op.paint ?? {}), ...(op.solid?.[view] !== undefined ? { solid: structuredClone(op.solid[view]) } : {}) };
      shapesOf(op.scale, view).push(target);
      done.push(`${view} created`);
      continue;
    }
    const before = JSON.stringify(target);
    copyGeometry(source, target);
    if (op.solid?.[view] !== undefined) target.solid = structuredClone(op.solid[view]);
    done.push(`${view} ${JSON.stringify(target) === before ? "unchanged" : "copied"}`);
  }
  return done.join(", ");
}

function planPrism(op: any): string {
  const shape = need(op.scale, "plan", op.id, op.bone);
  const frame = frames[shape.bone]!.frame;
  const length = lengthOf(shape.bone);
  const twin = find(op.scale, "elevation", op.twin ?? op.id, shape.bone);
  if (twin === undefined) throw new Error(`1:${op.scale} "${op.id}" has no elevation twin on ${shape.bone}`);
  const side = footprint(twin, "elevation", frame, length, 1);
  if (side === undefined) throw new Error(`1:${op.scale} "${op.id}": the elevation twin gives no (u, v) footprint`);
  const before = snapshot(shape);
  const [u0, u1] = rangeOf(side, 0);
  let profile: P2[];
  if (shape.solid?.kind === "extruded" && shape.solid.sweep === "u") profile = pointsOf(shape);
  else if (shape.solid === undefined) {
    if (Math.abs(frame.v[0]) < 0.8 || Math.abs(frame.w[2]) < 0.95) throw new Error(`1:${op.scale} "${op.id}": bone ${shape.bone} is not upright in the sagittal plane (v.x ${frame.v[0].toFixed(2)}, w.z ${frame.w[2].toFixed(2)}): no plan outline can be swept along it`);
    const mid = (u0 + u1) / 2;
    profile = pointsOf(shape).map(([x, z]) => [(x - frame.o[0] - mid * frame.u[0]) / frame.v[0], (z - frame.o[2]) / frame.w[2]] as const);
  } else throw new Error(`1:${op.scale} plan "${op.id}" is a ${shape.solid.kind} solid, not a sweep along u`);
  const [a0, a1] = rangeOf(profile, 0);
  const prism = (p: readonly number[]): P2[] =>
    [a0, Math.min(Math.max(0, a0), a1), Math.max(Math.min(length, a1), a0), a1].flatMap((a) => {
      const t = length <= 0 ? 0 : Math.min(Math.max(a / length, 0), 1);
      return [[p[0]! + (p[2]! - p[0]!) * t, a] as const, [p[1]! + (p[3]! - p[1]!) * t, a] as const];
    });
  const cost = (p: readonly number[]): number => mismatch(prism(p), side) + (p[0]! >= p[1]! - 0.002 || p[2]! >= p[3]! - 0.002 ? 1 : 0);
  const plain = search([u0, u1], (p) => cost([p[0]!, p[1]!, p[0]!, p[1]!]));
  const skew = search([plain[0]!, plain[1]!, plain[0]!, plain[1]!], cost);
  const tapered = op.taper !== false && cost([plain[0]!, plain[1]!, plain[0]!, plain[1]!]) - cost(skew) > 0.002;
  const fitted = tapered ? skew : [plain[0]!, plain[1]!, plain[0]!, plain[1]!];
  for (const key of GEOMETRY_KEYS) if (key !== "closed") delete shape[key];
  Object.assign(shape, { kind: "poly", points: profile.map(([a, b]) => [round(a, 4), round(b, 4)]), closed: true, solid: { kind: "extruded", sweep: "u", z0: round(fitted[0]!), z1: round(fitted[1]!), ...(tapered ? { zTip: [round(fitted[2]!), round(fitted[3]!)] } : {}) } });
  return `${settle(shape, before) ? "fitted" : "unchanged"}: v [${a0.toFixed(3)},${a1.toFixed(3)}], span ${JSON.stringify(shape.solid)} footprint mismatch ${(mismatch(prism(fitted), side) * 1000).toFixed(1)} mm`;
}

function elevationExtrude(op: any): string {
  const shape = need(op.scale, "elevation", op.id, op.bone);
  const frame = frames[shape.bone]!.frame;
  const length = lengthOf(shape.bone);
  if (shape.solid !== undefined && !(shape.solid.kind === "extruded" && shape.solid.sweep === "w")) throw new Error(`1:${op.scale} elevation "${op.id}" is a ${shape.solid.kind} solid already`);
  const before = snapshot(shape);
  let span: [number, number] | undefined = op.z;
  if (span === undefined) {
    const front = find(op.scale, "front", op.frontTwin ?? op.id, shape.bone);
    const across = front === undefined ? undefined : footprint(front, "front", frame, length, 2);
    if (across !== undefined) span = rangeOf(across, 1).map((w) => w + frame.o[2]) as [number, number];
  }
  if (span === undefined) {
    const plan = find(op.scale, "plan", op.planTwin ?? op.id, shape.bone);
    if (plan !== undefined && plan.solid === undefined) span = rangeOf(pointsOf(plan), 1);
    else if (plan?.solid?.kind === "extruded" && plan.solid.sweep === "u") span = rangeOf(pointsOf(plan), 1).map((w) => w + frame.o[2]) as [number, number];
  }
  if (span === undefined) throw new Error(`1:${op.scale} "${op.id}": no front or plan twin gives the lateral span, state "z": [z0, z1]`);
  const points = pointsOf(shape).map(([x, y]) => [x, y]);
  for (const key of GEOMETRY_KEYS) if (key !== "closed") delete shape[key];
  Object.assign(shape, { kind: "poly", points: before.kind === "poly" ? structuredClone(before.points) : points, ...(before.kind === "poly" ? {} : { closed: true }), solid: { kind: "extruded", sweep: "w", z0: round(span[0]), z1: round(span[1]) } });
  return `${settle(shape, before) ? "extruded" : "unchanged"}: the elevation outline over the lateral span ${JSON.stringify(shape.solid)}`;
}

function planCore(op: any): string {
  const view = op.view ?? "plan";
  const outline = need(op.scale, view, op.id, op.bone);
  const frame = frames[outline.bone]!.frame;
  const twin = find(op.scale, "elevation", op.twin ?? op.id, outline.bone);
  if (twin === undefined || twin.solid !== undefined) throw new Error(`1:${op.scale} "${op.id}" needs a flat elevation twin on ${outline.bone}`);
  const list = shapesOf(op.scale, view);
  let core = list.find((shape) => shape.id === op.as && shape.bone === outline.bone);
  const before = core === undefined ? undefined : JSON.stringify(core);
  if (core === undefined) {
    core = Object.fromEntries(Object.entries(outline).filter(([key]) => !GEOMETRY_KEYS.includes(key) && key !== "kind"));
    core.id = op.as;
    if (typeof outline.z === "number") core.z = round(outline.z - 0.01, 4);
    list.splice(list.indexOf(outline), 0, core);
  }
  for (const key of GEOMETRY_KEYS) delete core[key];
  Object.assign(core, { kind: "poly", points: structuredClone(pointsOf(twin)), closed: true, solid: { kind: "extruded", sweep: "w", z0: round(op.z?.[0] ?? frame.o[2] - op.half), z1: round(op.z?.[1] ?? frame.o[2] + op.half) } });
  return before === JSON.stringify(core) ? "unchanged" : `core "${op.as}": the elevation outline swept laterally over [${core.solid.z0}, ${core.solid.z1}] m beneath the ${view} outline`;
}

const hull = (points: readonly P2[]): P2[] => {
  const sorted = [...points].sort((a, b) => a[0] - b[0] || a[1] - b[1]);
  const turn = (o: P2, a: P2, b: P2): number => (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0]);
  const half = (list: readonly P2[]): P2[] => {
    const chain: P2[] = [];
    for (const point of list) {
      while (chain.length >= 2 && turn(chain[chain.length - 2]!, chain[chain.length - 1]!, point) <= 0) chain.pop();
      chain.push(point);
    }
    chain.pop();
    return chain;
  };
  return [...half(sorted), ...half([...sorted].reverse())];
};

function planSlabs(op: any): string {
  const outline = need(op.scale, "plan", op.id, op.bone);
  const frame = frames[outline.bone]!.frame;
  const twin = find(op.scale, "elevation", op.twin ?? op.id, outline.bone);
  if (outline.solid !== undefined || outline.kind !== "poly" || twin === undefined || twin.solid !== undefined) throw new Error(`1:${op.scale} "${op.id}" needs a flat plan poly and a flat elevation twin on ${outline.bone}`);
  const list = shapesOf(op.scale, "plan");
  const prefix = `${op.as}-`;
  const stale = list.filter((shape) => shape.bone === outline.bone && typeof shape.id === "string" && shape.id.startsWith(prefix));
  const before = JSON.stringify(stale);
  for (const shape of stale) list.splice(list.indexOf(shape), 1);
  const profile = pointsOf(twin);
  const plan = pointsOf(outline);
  const axis = Math.abs(frame.u[0]) >= Math.abs(frame.v[0]) ? 0 : 1;
  const other = 1 - axis;
  const cut = (points: readonly P2[], along: number, at: number): number[] =>
    points.flatMap((a, i) => {
      const b = points[(i + 1) % points.length]!;
      return (a[along]! - at) * (b[along]! - at) < 0 ? [a[1 - along]! + ((b[1 - along]! - a[1 - along]!) * (at - a[along]!)) / (b[along]! - a[along]!)] : [];
    });
  const [a0, a1] = rangeOf(profile, axis);
  const [x0, x1] = rangeOf(plan, 0);
  const count = op.slices ?? 3;
  const made: any[] = [];
  const notes: string[] = [];
  for (let i = 0; i < count; i++) {
    const from = a0 + ((a1 - a0) * i) / count;
    const to = a0 + ((a1 - a0) * (i + 1)) / count;
    const inside: P2[] = [...profile.filter((point) => point[axis]! >= from && point[axis]! <= to), ...[from + 1e-6, to - 1e-6].flatMap((at) => cut(profile, axis, at).map((value) => (axis === 0 ? [at, value] : [value, at]) as P2))];
    if (inside.length < 3) continue;
    const aheadOf = ([u, v]: P2): number => frame.o[0] + u * frame.u[0] + v * frame.v[0];
    const clip = (points: P2[], keep: (point: P2) => number): P2[] =>
      points.flatMap((a, k) => {
        const b = points[(k + 1) % points.length]!;
        const da = keep(a);
        const db = keep(b);
        const crossing: P2[] = da * db < 0 ? [[a[0] + ((b[0] - a[0]) * da) / (da - db), a[1] + ((b[1] - a[1]) * da) / (da - db)]] : [];
        return da >= 0 ? [a, ...crossing] : crossing;
      });
    const corners = clip(clip(hull(inside), (point) => aheadOf(point) - x0), (point) => x1 - aheadOf(point));
    if (corners.length < 3) continue;
    const ahead = corners.map(aheadOf);
    const xa = Math.min(...ahead);
    const xb = Math.max(...ahead);
    let z0 = -Infinity;
    let z1 = Infinity;
    for (let k = 0; k <= 8; k++) {
      const x = Math.min(Math.max(xa + ((xb - xa) * k) / 8, x0 + 1e-4), x1 - 1e-4);
      const crossings = cut(plan, 0, x).sort((p, q) => p - q);
      let widest: [number, number] = [0, 0];
      for (let j = 0; j + 1 < crossings.length; j += 2) if (crossings[j + 1]! - crossings[j]! > widest[1] - widest[0]) widest = [crossings[j]!, crossings[j + 1]!];
      z0 = Math.max(z0, widest[0]);
      z1 = Math.min(z1, widest[1]);
    }
    if (z1 - z0 < 0.006) {
      notes.push(`band ${i + 1} not covered`);
      continue;
    }
    const core: any = Object.fromEntries(Object.entries(outline).filter(([key]) => !GEOMETRY_KEYS.includes(key) && key !== "kind"));
    core.id = `${prefix}${i + 1}`;
    if (typeof outline.z === "number") core.z = round(outline.z - 0.01, 4);
    Object.assign(core, { kind: "poly", points: corners.map(([u, v]) => [round(u, 4), round(v, 4)]), closed: true, solid: { kind: "extruded", sweep: "w", z0: round(z0 + 0.002), z1: round(z1 - 0.002) } });
    made.push(core);
  }
  list.splice(list.indexOf(outline), 0, ...made);
  return before === JSON.stringify(made) ? "unchanged" : `${made.length} prism(s) "${prefix}i" beneath the plan outline, bands along ${axis === 0 ? "u" : "v"}: ${made.map((core) => `[${core.solid.z0}, ${core.solid.z1}]`).join(" ")}${notes.length > 0 ? `; ${notes.join(", ")}` : ""}`;
}

function planDepth(op: any): string {
  const shape = need(op.scale, "plan", op.id, op.bone);
  const frame = frames[shape.bone]!.frame;
  const twin = find(op.scale, "elevation", op.twin ?? op.id, shape.bone);
  if (shape.solid !== undefined || shape.kind !== "poly" || twin === undefined) throw new Error(`1:${op.scale} "${op.id}" needs a flat plan poly with an elevation twin`);
  const ahead = pointsOf(twin).map(([u, v]) => frame.o[0] + u * frame.u[0] + v * frame.v[0]);
  const to = [Math.min(...ahead), Math.max(...ahead)] as const;
  const from = rangeOf(shape.points, 0);
  const before = JSON.stringify(shape.points);
  if (Math.abs(from[0] - to[0]) > 0.0003 || Math.abs(from[1] - to[1]) > 0.0003) shape.points = shape.points.map(([x, z]: P2) => [round(to[0] + ((x - from[0]) * (to[1] - to[0])) / (from[1] - from[0]), 4), z]);
  return before === JSON.stringify(shape.points) ? "unchanged" : `depth [${from.map((v) => v.toFixed(3))}] made the rest depth of the elevation twin [${to.map((v) => v.toFixed(3))}]`;
}

function apply(op: any): string {
  switch (op.op) {
    case "plan-depth": return planDepth(op);
    case "plan-core": return planCore(op);
    case "plan-slabs": return planSlabs(op);
    case "plan-prism": return planPrism(op);
    case "elevation-extrude": return elevationExtrude(op);
    case "front-sweep": return frontSweep(op);
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
