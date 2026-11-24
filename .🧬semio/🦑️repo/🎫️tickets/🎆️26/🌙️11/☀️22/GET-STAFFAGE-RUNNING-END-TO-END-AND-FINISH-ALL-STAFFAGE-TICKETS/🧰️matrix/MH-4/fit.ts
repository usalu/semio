/** 🧵️ The multiview repair of MR (`../MR/fit.ts`, copied; its ops are unchanged) with the ops the people of MH-4 needed in addition (`prism`, `elevation-solid`, listed below the ops of MR). `bun fit.ts <id|file.json> --ops <ops.json> [--write] [--out <file>]` applies the ops of the file in order and prints what each did (an op that cannot be applied is refused by name, the others still run, exit code 1); without `--write` nothing is written. Every op takes the ELEVATION twin (the shape of the same `id` and bone in `geometry.elevation`, or `twin`) as the profile of the part and rebuilds or refits the front or plan shape from it, so a second run changes nothing.
 *
 * Ops (`scale` is the representation key, `id` the shape id in the target view, `bone` only when the id is not unique):
 * - `{ "op": "front-sweep", scale, id }`: the front shape becomes (or stays) an `extruded` solid with `sweep: "v"`; its outline is kept (a flat front outline is first re-expressed in the bone-local (u, w) plane), cut along the bone to the u-range of the elevation twin (stretched to it where it falls short; `"range": "stretch"` only stretches, `"keep"` leaves it; `"lateral": "plan"` cuts it the same way across the bone to the lateral range of the plan twin), and `z0`, `z1`, `zTip` are fitted so that the (u, v) footprint of the prism has the support function of the elevation outline in every direction (so the heights agree at every sagittal angle).
 * - `{ "op": "extrude", scale, id, "view": "front" | "plan" }`: the exact alternative for a part whose front outline is a plain taper (and for the plan of a part that is a prism): the shape takes the elevation outline as its profile and becomes an `extruded` solid with `sweep: "w"` whose lateral span `z0`, `z1`, `zTip` is fitted to the (u, w) footprint the shape had; elevation and this view then agree at every pose by construction, and the view draws the hull of the swept outline instead of its own outline. It is also the op for a bone that points at the viewer of the front view (a foot, a toe): there no front outline can be swept along v and `front-sweep` refuses.
 * - `{ "op": "plan-revolve", scale, id }`: the plan shape becomes (or stays) a `revolved` solid about the bone axis. An elevation twin that is itself revolved is copied (one body in three views). Else an existing revolved profile is kept in shape, cut or stretched to the u-range of the elevation twin, and its axis offset and radius scale are fitted to the (u, v) footprint of the elevation twin and the (u, w) footprint of the front twin; a flat plan shape gets a new profile of the mean half thickness of the two twins at nine stations (`"stations": n`).
 * - `{ "op": "copy", scale, id, "from": view, "to": [view…] }`: the geometry and the solid of the shape in `from` replace those of its twins in `to` (the same revolved or extruded body in several views); `"create": true` adds a missing twin with the paint fields of the source.
 * - `{ "op": "set", scale, view, id, "set": {…} }` and `{ "op": "remove", scale, view, id }`: a hand-made correction of single fields, or the removal of a shape.
 * - `{ "op": "anchor", id, "set": {…} }`: binds an anchor (`bone`, `x`, `y`, `z`).
 * - `{ "op": "prism", scale, view, id }` (MH-4): a flat front or plan shape becomes an `extruded` solid that keeps its own outline: the sweep axis is the bone axis that points at the viewer of the view at rest (`v` for the front of an upright limb, `u` for its plan and for the front of a foot, refused when no axis lies within 25 degrees of the view direction unless `"axis"` names one), the outline is re-expressed in the plane of the other two bone axes through the bone origin, and the span along the sweep axis is the extent of the elevation twin there (`v`: `z0`, `z1`, `zTip` fitted to the (u, v) footprint of the twin as `front-sweep` does; `u`: the span along the bone for which the box of that span and of the depth of the outline has the support function of the (u, v) footprint of the twin in every direction, so a part with a rounded top is not drawn too long when it leans; `w`: the lateral range of the other twin). `"range": "clip"` cuts the outline along the bone to the u-range of the elevation twin first (hidden length inside the neighbouring part, sweep `v` only). A `followTip` flag is dropped (a solid follows by construction). A shape that is a solid already is left alone.
 * - `extrude` takes `"taper": false` (MH-4): the lateral span is the lateral range the flat shape had in asset coordinates, without `zTip` (a shoe keeps its width); a shape that is a solid already is then left alone.
 * - `front-sweep` takes `"view": "plan"` (MH-4): the refit of a plan shape that is a prism swept along v already (a foot, a head drawn so).
 * - `{ "op": "mirror", scale, id, "views"? }` (MH-4): a revolved solid drawn as its half profile (no `mirror`) becomes the whole body of revolution in the views named (default all three); the half profile is a flat meridian sheet that the front shows on edge when the limb points at the viewer, while the plan always draws the whole body.
 * - `plan-revolve` with `"rebuild": true` on a revolved elevation twin (MH-4) does not copy it: the plan body gets the mean of the half thickness of the elevation twin and the half width of the front twin (an arm drawn 0.20 m deep in the side view and 0.10 m wide in the front has no body of revolution that agrees with both; the mean stays within the tolerance of each).
 * - `copy` with `"create": "only"` (MH-4) adds a missing twin and leaves an existing one alone (the twin is then shaped by a later op, and a second run changes nothing).
 * - `extrude` with `"across": "front"` (MH-4): the lateral span and its taper along the bone are fitted to the (u, w) footprint of the front twin, not to the shape itself (the plan of a trunk that leans far: waist and shoulder width come from the front).
 * - `{ "op": "elevation-solid", scale, id }` (MH-4): a flat elevation shape on a bone that poses spread or twist becomes an `extruded` solid with `sweep: "w"`: its outline is kept, the lateral span `z0`, `z1`, `zTip` is fitted to the (u, w) footprint of the front twin (or is the lateral range of the plan twin, or `"span": [z0, z1]`), so the side view shows the turned part instead of a sheet seen edge-on. `"respan": true` fits the span anew for a shape that is such a solid already (a paper-thin one of an earlier round), `"from": "plan"` takes the plan twin. Refused on a bone whose lateral axis is not the world z at rest.
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
  if (solid?.kind === "extruded" && solid.sweep === "u") return [solid.z0, solid.z1].flatMap((u) => rangeOf(points, plane === 1 ? 0 : 1).map((c) => [u, c] as const));
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

/** 🌵️ A ring without spikes (MH-4): a vertex at which the outline turns back on itself by more than 154 degrees over less than 5 mm is dropped until none is left; the cut at an end of the elevation twin leaves such a sliver where the outline ran along the cut, and the union of the engine cannot close a self-overlapping profile. */
const despike = (ring: readonly P2[]): P2[] => {
  const out = [...ring];
  for (let again = true; again && out.length > 3; ) {
    again = false;
    for (let i = 0; i < out.length; i++) {
      const [a, b, c] = [out[(i + out.length - 1) % out.length]!, out[i]!, out[(i + 1) % out.length]!];
      const [p, q] = [[b[0] - a[0], b[1] - a[1]], [c[0] - b[0], c[1] - b[1]]];
      const [lp, lq] = [Math.hypot(p[0]!, p[1]!), Math.hypot(q[0]!, q[1]!)];
      if (Math.min(lp, lq) < 0.005 && (lp < 1e-9 || lq < 1e-9 || (p[0]! * q[0]! + p[1]! * q[1]!) / (lp * lq) < -0.9)) {
        out.splice(i, 1);
        again = true;
        break;
      }
    }
  }
  return out;
};

/** 🪜️ A (u, w) profile of a prism swept along v without an edge that runs exactly along the bone (MH-4): the far end of such an edge is moved 0.4 mm towards the bone axis. In the front view that edge is collinear with the sweep, and the union of the engine failed on it at single instants of a walk (`pregnant-person` 1:50 front `shank-r`, `clip waddle-walk key waddle-contact @0.133 s`: "the union of 11 polygon(s) with area cannot be built"; with the edge 0.5 mm out of level it evaluates). */
const unlevel = (points: readonly P2[]): P2[] => {
  const out = points.map(([u, w]) => [u, w] as [number, number]);
  for (let i = 0; i < out.length; i++) {
    const [p, q] = [out[i]!, out[(i + 1) % out.length]!];
    if (Math.abs(q[1] - p[1]) < 0.0002 && Math.abs(q[0] - p[0]) > 0.003) q[1] -= Math.sign(q[1] || 1) * 0.0004;
  }
  return out;
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
  const clean = despike(ring.filter((p, i) => Math.hypot(p[0] - ring[(i + 1) % ring.length]![0], p[1] - ring[(i + 1) % ring.length]![1]) > 1e-6));
  return clean.length < 3 ? stretch(points, to) : stretch(clean, to);
};

function frontSweep(op: any): string {
  const shape = need(op.scale, op.view ?? "front", op.id, op.bone);
  if (op.view === "plan" && shape.solid === undefined) throw new Error(`1:${op.scale} plan "${op.id}" is flat: use the op "prism"`);
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
  Object.assign(shape, { kind: "poly", points: unlevel(profile).map(([u, w]) => [round(u, 4), round(w, 4)]), closed: true, solid: { kind: "extruded", sweep: "v", z0: round(fitted[0]!), z1: round(fitted[1]!), zTip: [round(fitted[2]!), round(fitted[3]!)] } });
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
  const lateral = shape.solid !== undefined || op.across === "front" ? undefined : op.view === "plan" ? rangeOf(pointsOf(shape), 1).map((z) => z - frame.o[2]) : edgeOn ? rangeOf(pointsOf(shape), 0).map((z) => z - frame.o[2]) : undefined;
  if (op.taper === false && shape.solid === undefined) {
    const span = rangeOf(pointsOf(shape), op.view === "plan" ? 1 : 0);
    for (const key of GEOMETRY_KEYS) if (key !== "closed") delete shape[key];
    Object.assign(shape, { kind: "poly", points: structuredClone(profile), closed: true, solid: { kind: "extruded", sweep: "w", z0: round(span[0]), z1: round(span[1]) } });
    return `extruded: the elevation outline swept over the lateral range the shape had, span ${JSON.stringify(shape.solid)}`;
  }
  if (op.taper === false) return "unchanged: already a solid";
  const frontShape = op.across === "front" ? find(op.scale, "front", op.id, shape.bone) : undefined;
  const across = frontShape !== undefined ? footprint(frontShape, "front", frame, length, 2) : shape.solid?.kind === "extruded" && shape.solid.sweep === "w" ? undefined : lateral !== undefined ? [a, b].flatMap((u) => lateral.map((w) => [u, w] as const)) : footprint(shape, op.view ?? "front", frame, length, 2);
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
  if (twin.solid?.kind === "revolved" && op.rebuild !== true) {
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
      const values = [half(pointsOf(twin), u), across === undefined ? undefined : half(frontTwin.solid?.kind === "revolved" ? pointsOf(frontTwin) : across, u)].filter((value): value is number => value !== undefined);
      return [i === 0 ? range[0] : i === n - 1 ? range[1] : u, values.length === 0 ? 0 : values.reduce((x, y) => x + y, 0) / values.length] as const;
    });
  }
  const band = (centre: number, scale: number): P2[] => radial.flatMap(([u, r]) => [[u, centre - r * scale] as const, [u, centre + r * scale] as const]);
  const mid = (points: readonly P2[]): number => (rangeOf(points, 1)[0] + rangeOf(points, 1)[1]) / 2;
  const cost = (p: readonly number[]): number => Math.max(mismatch(band(p[0]!, p[2]!), side), across === undefined ? 0 : mismatch(band(p[1]!, p[2]!), across)) + (p[2]! < 0.2 ? 1 : 0);
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
      if (op.create !== true && op.create !== "only") throw new Error(`1:${op.scale} ${view} has no shape "${op.as ?? op.id}" on ${source.bone} (state "create": true to add it)`);
      target = { ...structuredClone(source), id: op.as ?? op.id, ...(op.paint ?? {}) };
      shapesOf(op.scale, view).push(target);
      done.push(`${view} created`);
      continue;
    }
    if (op.create === "only") {
      done.push(`${view} unchanged`);
      continue;
    }
    const before = JSON.stringify(target);
    copyGeometry(source, target);
    done.push(`${view} ${JSON.stringify(target) === before ? "unchanged" : "copied"}`);
  }
  return done.join(", ");
}

type Axis = "u" | "v" | "w";
const VIEW_DIR: Record<string, V3> = { elevation: [0, 0, 1], front: [1, 0, 0], plan: [0, 1, 0] };
const PROFILE_AXES: Record<Axis, readonly [Axis, Axis]> = { u: ["v", "w"], v: ["u", "w"], w: ["u", "v"] };

/** 🧊️ A flat front or plan shape as a prism that keeps its outline (the op `prism`). */
function prism(op: any): string {
  const view = op.view as "front" | "plan";
  const shape = need(op.scale, view, op.id, op.bone);
  if (shape.solid !== undefined) return `unchanged: already a ${shape.solid.kind} solid`;
  const frame = frames[shape.bone]!.frame;
  const length = lengthOf(shape.bone);
  const d = VIEW_DIR[view]!;
  const axis: Axis = op.axis ?? (["u", "v", "w"] as const).reduce((best, k) => (Math.abs(dot(frame[k], d)) > Math.abs(dot(frame[best], d)) ? k : best), "u" as Axis);
  const along = Math.abs(dot(frame[axis], d));
  if (op.axis === undefined && along < 0.906) throw new Error(`1:${op.scale} ${view} "${op.id}": no axis of bone "${shape.bone}" points at the viewer at rest (nearest ${axis}, ${((Math.acos(along) * 180) / Math.PI).toFixed(0)} degrees off): state "axis"`);
  const twin = find(op.scale, "elevation", op.twin ?? op.id, shape.bone);
  if (twin === undefined && op.span === undefined) throw new Error(`1:${op.scale} "${op.id}" has no elevation twin on ${shape.bone}`);
  const [pa, pb] = PROFILE_AXES[axis];
  const s = frame[axis];
  let profile = pointsOf(shape).map(([a, b]): P2 => {
    const x0: V3 = view === "front" ? [frame.o[0], b, a] : [a, frame.o[1], b];
    const lambda = -dot(sub(x0, frame.o), s) / dot(d, s);
    const rel: V3 = [x0[0] + lambda * d[0] - frame.o[0], x0[1] + lambda * d[1] - frame.o[1], x0[2] + lambda * d[2] - frame.o[2]];
    return [dot(rel, frame[pa]), dot(rel, frame[pb])] as const;
  });
  const side = twin === undefined ? undefined : footprint(twin, "elevation", frame, length, 1);
  let solid: any;
  let note = "";
  if (axis === "v") {
    if (side === undefined) throw new Error(`1:${op.scale} "${op.id}": the elevation twin gives no (u, v) footprint`);
    if (op.range === "clip") profile = clipStretch(profile, rangeOf(side, 0));
    const [a, b] = rangeOf(profile, 0);
    const box = (p: readonly number[]): P2[] =>
      [a, Math.min(Math.max(0, a), b), Math.max(Math.min(length, b), a), b].flatMap((u) => {
        const t = length <= 0 ? 0 : Math.min(Math.max(u / length, 0), 1);
        return [[u, p[0]! + (p[2]! - p[0]!) * t] as const, [u, p[1]! + (p[3]! - p[1]!) * t] as const];
      });
    const [v0, v1] = rangeOf(side, 1);
    const fitted = op.span !== undefined ? [op.span[0], op.span[1], op.span[0], op.span[1]] : search([v0, v1, v0, v1], (p) => mismatch(box(p), side) + (p[0]! >= p[1]! - 0.002 || p[2]! >= p[3]! - 0.002 ? 1 : 0));
    solid = { kind: "extruded", sweep: "v", z0: round(fitted[0]!), z1: round(fitted[1]!), zTip: [round(fitted[2]!), round(fitted[3]!)] };
    note = `footprint mismatch ${(mismatch(box(fitted), side) * 1000).toFixed(1)} mm`;
  } else if (axis === "u") {
    let range = op.span ?? (side === undefined ? undefined : rangeOf(side, 0));
    if (range === undefined) throw new Error(`1:${op.scale} "${op.id}": the elevation twin gives no u-range`);
    if (op.span === undefined && side !== undefined) {
      const across = rangeOf(profile, 0);
      const box = (p: readonly number[]): P2[] => [p[0]!, p[1]!].flatMap((u) => across.map((v) => [u, v] as const));
      const fitted = search(range, (p) => mismatch(box(p), side) + (p[0]! >= p[1]! - 0.002 ? 1 : 0));
      range = [fitted[0]!, fitted[1]!];
      note = `footprint mismatch ${(mismatch(box(fitted), side) * 1000).toFixed(1)} mm (u-range of the twin [${rangeOf(side, 0).map((value) => value.toFixed(3))}])`;
    }
    solid = { kind: "extruded", sweep: "u", z0: round(range[0]), z1: round(range[1]) };
  } else {
    const other = find(op.scale, view === "front" ? "plan" : "front", op.id, shape.bone);
    const lateral = op.span ?? (twin?.solid?.kind === "extruded" && twin.solid.sweep === "w" ? [twin.solid.z0, twin.solid.z1] : other !== undefined && other.solid === undefined ? rangeOf(pointsOf(other), view === "front" ? 1 : 0) : undefined);
    if (lateral === undefined) throw new Error(`1:${op.scale} "${op.id}": no lateral span for a sweep along w: state "span"`);
    solid = { kind: "extruded", sweep: "w", z0: round(lateral[0]), z1: round(lateral[1]) };
  }
  const tilt = Math.sqrt(Math.max(0, 1 - along * along)) * Math.max(Math.abs(solid.z0 - (axis === "w" ? frame.o[2] : 0)), Math.abs(solid.z1 - (axis === "w" ? frame.o[2] : 0)));
  for (const key of GEOMETRY_KEYS) if (key !== "closed") delete shape[key];
  Object.assign(shape, { kind: "poly", points: (axis === "v" ? unlevel(profile) : profile).map(([p, q]) => [round(p, 4), round(q, 4)]), closed: true, solid });
  return `prism along ${axis}: span ${JSON.stringify(solid)} ${note} rest shift up to ${(tilt * 1000).toFixed(1)} mm`;
}

/** 🧱️ A flat elevation shape as a solid swept laterally (the op `elevation-solid`). */
function elevationSolid(op: any): string {
  const shape = need(op.scale, "elevation", op.id, op.bone);
  if (shape.solid !== undefined && !(op.respan === true && shape.solid.kind === "extruded" && shape.solid.sweep === "w")) return `unchanged: already a ${shape.solid.kind} solid`;
  const was = snapshot(shape);
  const frame = frames[shape.bone]!.frame;
  const length = lengthOf(shape.bone);
  if (frame.w[2] < 0.99) throw new Error(`1:${op.scale} elevation "${op.id}": the lateral axis of bone "${shape.bone}" is not the world z at rest`);
  const profile = pointsOf(shape);
  const [a, b] = rangeOf(profile, 0);
  const frontTwin = find(op.scale, "front", op.frontTwin ?? op.id, shape.bone);
  const planTwin = find(op.scale, "plan", op.planTwin ?? op.id, shape.bone);
  let across: P2[] | undefined;
  let source = "span";
  if (op.span !== undefined) across = [a, b].flatMap((u) => (op.span as number[]).map((z) => [u, z - frame.o[2]] as const));
  else {
    try {
      across = frontTwin === undefined || op.from === "plan" ? undefined : footprint(frontTwin, "front", frame, length, 2);
      source = "front twin";
    } catch {
      across = undefined;
    }
    if (across === undefined && op.from !== "plan" && frontTwin !== undefined && frontTwin.solid === undefined) (across = [a, b].flatMap((u) => rangeOf(pointsOf(frontTwin), 0).map((z) => [u, z - frame.o[2]] as const))), (source = "lateral range of the front twin");
    if (across === undefined && planTwin !== undefined) {
      across = footprint(planTwin, "plan", frame, length, 2);
      source = "plan twin";
      if (across === undefined && planTwin.solid === undefined) (across = [a, b].flatMap((u) => rangeOf(pointsOf(planTwin), 1).map((z) => [u, z - frame.o[2]] as const))), (source = "lateral range of the plan twin");
    }
  }
  if (across === undefined) throw new Error(`1:${op.scale} elevation "${op.id}": no front or plan twin gives a lateral span: state "span"`);
  const box = (p: readonly number[]): P2[] =>
    [a, Math.min(Math.max(0, a), b), Math.max(Math.min(length, b), a), b].flatMap((u) => {
      const t = length <= 0 ? 0 : Math.min(Math.max(u / length, 0), 1);
      return [[u, p[0]! + (p[2]! - p[0]!) * t] as const, [u, p[1]! + (p[3]! - p[1]!) * t] as const];
    });
  const [w0, w1] = rangeOf(across, 1);
  const fitted = search([w0, w1, w0, w1], (p) => mismatch(box(p), across!) + (p[0]! >= p[1]! - 0.002 || p[2]! >= p[3]! - 0.002 ? 1 : 0));
  const z = frame.o[2];
  const taper = Math.abs(fitted[0]! - fitted[2]!) > 0.001 || Math.abs(fitted[1]! - fitted[3]!) > 0.001;
  const points = structuredClone(profile);
  for (const key of GEOMETRY_KEYS) if (key !== "closed") delete shape[key];
  Object.assign(shape, { kind: "poly", points, closed: true, solid: { kind: "extruded", sweep: "w", z0: round(fitted[0]! + z), z1: round(fitted[1]! + z), ...(taper ? { zTip: [round(fitted[2]! + z), round(fitted[3]! + z)] } : {}) } });
  return `${settle(shape, was) ? "solid" : "unchanged"} from the ${source}: span ${JSON.stringify(shape.solid)} footprint mismatch ${(mismatch(box(fitted), across) * 1000).toFixed(1)} mm`;
}

function apply(op: any): string {
  switch (op.op) {
    case "mirror": {
      const done: string[] = [];
      for (const view of (op.views ?? ["elevation", "front", "plan"]) as string[]) {
        const shape = find(op.scale, view, op.id, op.bone);
        if (shape === undefined) continue;
        if (shape.solid?.kind !== "revolved") throw new Error(`1:${op.scale} ${view} "${op.id}" is not a revolved solid`);
        done.push(`${view} ${shape.solid.mirror === true ? "unchanged" : "mirrored"}`);
        shape.solid.mirror = true;
      }
      return done.join(", ");
    }
    case "prism": return prism(op);
    case "elevation-solid": return elevationSolid(op);
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
