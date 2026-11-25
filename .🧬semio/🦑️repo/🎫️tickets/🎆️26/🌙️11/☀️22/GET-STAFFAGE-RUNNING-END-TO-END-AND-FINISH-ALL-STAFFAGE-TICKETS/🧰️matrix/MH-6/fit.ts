/** 🧵️ The multiview repair of one asset of batch MH-6, the last step of its chain (the `fit.ts` of the fixer MR with the further op `plan-sweep`): `bun fit.ts <id|file.json> --ops <ops.json> [--write] [--out <file>]` applies the ops of the file in order and prints what each did (an op that cannot be applied is refused by name, the others still run, exit code 1); without `--write` nothing is written. Every op takes the ELEVATION twin (the shape of the same `id` and bone in `geometry.elevation`, or `twin`) as the profile of the part and rebuilds or refits the front or plan shape from it, so a second run changes nothing.
 *
 * Ops (`scale` is the representation key, `id` the shape id in the target view, `bone` only when the id is not unique):
 * - `{ "op": "front-sweep", scale, id }`: the front shape becomes (or stays) an `extruded` solid with `sweep: "v"`; its outline is kept (a flat front outline is first re-expressed in the bone-local (u, w) plane), cut along the bone to the u-range of the elevation twin (stretched to it where it falls short; `"range": "stretch"` only stretches, `"keep"` leaves it; `"lateral": "plan"` cuts it the same way across the bone to the lateral range of the plan twin), and `z0`, `z1`, `zTip` are fitted so that the (u, v) footprint of the prism has the support function of the elevation outline in every direction (so the heights agree at every sagittal angle). `"twin": [id…]` fits a front shape that draws several elevation parts as one (a head with its helmet) to their union.
 * - `{ "op": "plan-sweep", scale, id }`: the same for the plan shape of a bone that does not stand upright (a foot, a hand, a level forearm): the plan outline is kept in the bone-local (u, w) plane, cut to the u-range of the elevation twin and swept along v with the span fitted to the elevation outline.
 * - `{ "op": "extrude", scale, id, "view": "front" | "plan" }`: the exact alternative for a part whose front outline is a plain taper (and for the plan of a part that is a prism): the shape takes the elevation outline as its profile and becomes an `extruded` solid with `sweep: "w"` whose lateral span `z0`, `z1`, `zTip` is fitted to the (u, w) footprint the shape had; elevation and this view then agree at every pose by construction, and the view draws the hull of the swept outline instead of its own outline. It is also the op for a bone that points at the viewer of the front view (a foot, a toe): there no front outline can be swept along v and `front-sweep` refuses. `"across": "range"` takes the plain lateral range of a flat front shape as the span (a foot whose bone slopes: the outline seen from ahead is not an outline in the (u, w) plane).
 * - `{ "op": "plan-revolve", scale, id }`: the plan shape becomes (or stays) a `revolved` solid about the bone axis. An elevation twin that is itself revolved is copied (one body in three views). Else an existing revolved profile is kept in shape, cut or stretched to the u-range of the elevation twin, and its axis offset and radius scale are fitted to the (u, v) footprint of the elevation twin and the (u, w) footprint of the front twin; a flat plan shape gets a new profile of the mean half thickness of the two twins at nine stations (`"stations": n`).
 * - `{ "op": "copy", scale, id, "from": view, "to": [view…] }`: the geometry and the solid of the shape in `from` replace those of its twins in `to` (the same revolved or extruded body in several views); `"create": true` adds a missing twin with the paint fields of the source.
 * - `{ "op": "set", scale, view, id, "set": {…} }` and `{ "op": "remove", scale, view, id }`: a hand-made correction of single fields, or the removal of a shape.
 * - `{ "op": "anchor", id, "set": {…} }`: binds an anchor (`bone`, `x`, `y`, `z`).
 * - `{ "op": "rebind", scale, view, id, "to": bone }`: a flat front or plan shape that is bound to another bone than the one whose part it draws (a seat drawn on the thorax) is bound to `to` and takes as its points the outline the default pose drew, moved so that the default pose draws it where it was drawn before; a shape already on `to` is left alone.
 * - `{ "op": "core", scale, id, "as"?: id, "share"?: 0.6 }`: a plan part with a free outline (a trunk) on a bone that leans keeps its outline and gets below it the middle slice of the same body, the shape `as` (default `<id>-core`): the elevation outline of `id` swept laterally over `share` of the lateral range the plan shape has, so the plan reaches as far ahead and back as the elevation at every lean.
 * - `{ "op": "plan-prism", scale, id }`: the plan shape of a part on a bone that stands (a trunk, a head) keeps its outline and becomes an `extruded` solid with `sweep: "u"`: the flat plan outline is re-expressed as the transverse (v, w) section of the bone, narrowed along v by what the sweep along a bone that rests inclined adds (so the rest pose draws the extent it drew), and swept along the bone over the span whose (u, v) footprint has the support function of the elevation outline; an existing `sweep: "u"` solid is refitted in its span only, unless its span is shorter than half the elevation outline: such a slab is a flat sheet across the bone and is rebuilt from its outline like a flat shape. `"fit": "inside"` takes instead the longest span whose footprint stays inside the hull of the elevation outline (within 2 mm), for a part whose depth changes along the bone: the plan then never reaches beyond the elevation, and the op `core` adds the slice that reaches as far.
 * - `{ "op": "anchor-split", id, "into": [{ id, z, title }…] }`: a lumped contact without `z` that stands for the wheels of both sides becomes one anchor per wheel at its real lateral place (D45); every place that names the anchor (`contacts` of the poses, of the clips, any other reference) names the new ones in its stead (an event whose `target` is the anchor becomes one event per part; `event` of a part overrides its fields, such as the title).
 * - `{ "op": "thicken", scale, view, id, "from"?: view }`: an outline that a tracer laid as a thin plate across the viewing direction of the default pose (an `extruded` shape a few centimetres thick) gets the real depth of the part along its sweep axis, read from the shape of the same id in `from` (default: the elevation, else the other view): the range of that shape's profile along the axis, or axis ± largest radius of a revolved one. So that the view of the default pose draws no more than it drew, the outline is first narrowed by what the sweep over that depth adds on the sheet there (the common part of the outline moved to the two ends of the depth; `"hold": false` keeps the outline as it is; the op refuses when less than half of the outline is left, `"least"` states another share; `"share"` takes only that part of the depth about its middle, where the whole depth would narrow the outline by more than the silhouette guard allows); in every other pose the part then turns as a body.
 * - `{ "op": "keep-as", scale, view, id, "as": id, "to": bone }`: a flat blob that drew two parts under the id of one (an upper arm seen from above together with the shoulder cap it hangs from) stays as the shape `as` on the bone `to` (bound there as `rebind` does), so that the shape `id` can become the solid of its own part; nothing happens when `as` exists.
 * - `{ "op": "carry", "from": scale, scale, view, id }`: a part that a coarser band left out although an anchor stands on it (a load roller) is drawn there too: the shape of `from` is added to the view of `scale` (or replaces the geometry of the shape of that id), `"set"` overrides paint fields; `"geometry": true` gives an existing shape only the geometry and solid of the other band (a limb that a finer band drew as tracer plates takes the body of revolution of the coarser band). `{ "op": "carry-solid", "from": scale, scale, id }` does the same for the solid of that id in `volumes.<scale>.body`, so drawing and body stay one part.
 * @see ./gate.ts — the measurement that says which bones need an op */
import { join } from "node:path";
import polygonClipping from "polygon-clipping";
import { S, dot, readDoc, round, sub, support, writeDoc, type V3 } from "../MR/lib.ts";

const { restFrames } = await import(join(S, "🔨️modules", "🦴️rig", "🟦️.ts"));
const { flattenGeometry, evaluate } = await import(join(S, "🔨️modules", "🧮️evaluation", "🟦️.ts"));

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

/** 🔁️ A flat plan outline (ahead, lateral) in asset metres at rest re-expressed in the bone-local (u, w) plane through the bone origin, seen from above. */
function planLocal(points: readonly P2[], frame: Frame): P2[] {
  const det = frame.u[0] * frame.w[2] - frame.w[0] * frame.u[2];
  if (Math.abs(det) < 0.2) throw new Error("the (u, w) plane of the bone is nearly edge-on in the plan: no plan outline can be swept along v");
  return points.map(([x, z]) => {
    const [dx, dz] = [x - frame.o[0], z - frame.o[2]];
    return [(dx * frame.w[2] - dz * frame.w[0]) / det, (frame.u[0] * dz - frame.u[2] * dx) / det] as const;
  });
}

function frontSweep(op: any): string {
  const view = op.op === "plan-sweep" ? "plan" : "front";
  const shape = need(op.scale, view, op.id, op.bone);
  const frame = frames[shape.bone]!.frame;
  const length = lengthOf(shape.bone);
  const twins = [op.twin ?? op.id].flat().map((id: string) => find(op.scale, "elevation", id, shape.bone));
  const twin = twins[0];
  if (twins.includes(undefined)) throw new Error(`1:${op.scale} "${op.id}" has no elevation twin on ${shape.bone}`);
  const target = twins.some((part: any) => part.solid?.kind === "revolved") ? undefined : twins.flatMap((part: any) => footprint(part, "elevation", frame, length, 1) ?? []);
  if (target === undefined || twin.solid?.kind === "revolved") throw new Error(`1:${op.scale} "${op.id}": the elevation twin is a revolved solid, use the op "copy"`);
  if (shape.solid !== undefined && !(shape.solid.kind === "extruded" && shape.solid.sweep === "v")) throw new Error(`1:${op.scale} ${view} "${op.id}" is a ${shape.solid.kind} solid, not a sweep along v`);
  const before = snapshot(shape);
  let profile = shape.solid === undefined ? (view === "plan" ? planLocal(pointsOf(shape), frame) : frontLocal(pointsOf(shape), frame)) : pointsOf(shape);
  const range = rangeOf(target, 0);
  if (op.range !== "keep") profile = op.range === "stretch" ? stretch(profile, range) : clipStretch(profile, range);
  if (op.lateral === "plan") {
    const planTwin = find(op.scale, view === "plan" ? "front" : "plan", op.planTwin ?? op.id, shape.bone);
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
  const lateral = shape.solid?.kind === "extruded" && shape.solid.sweep === "u" ? rangeOf(pointsOf(shape), 1) : shape.solid !== undefined ? undefined : op.view === "plan" ? rangeOf(pointsOf(shape), 1).map((z) => z - frame.o[2]) : edgeOn || op.across === "range" ? rangeOf(pointsOf(shape), 0).map((z) => z - frame.o[2]) : undefined;
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
      const values = [half(pointsOf(twin), u), across === undefined ? undefined : half(across, u)].filter((value): value is number => value !== undefined);
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

function apply(op: any): string {
  switch (op.op) {
    case "front-sweep": return frontSweep(op);
    case "plan-sweep": return frontSweep(op);
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
    case "plan-prism": {
      const shape = need(op.scale, "plan", op.id, op.bone);
      const frame = frames[shape.bone]!.frame;
      const twin = find(op.scale, "elevation", op.twin ?? op.id, shape.bone);
      if (twin === undefined) throw new Error(`1:${op.scale} "${op.id}" needs an elevation twin on ${shape.bone}`);
      const target = footprint(twin, "elevation", frame, lengthOf(shape.bone), 1)!;
      const before = snapshot(shape);
      const [u0, u1] = rangeOf(target, 0);
      let section: P2[];
      const sheet = shape.solid?.kind === "extruded" && shape.solid.sweep === "u" && shape.solid.z1 - shape.solid.z0 < 0.5 * (u1 - u0);
      if (shape.solid?.kind === "extruded" && shape.solid.sweep === "u" && !sheet) section = pointsOf(shape);
      else if (shape.solid !== undefined && !sheet) throw new Error(`1:${op.scale} plan "${op.id}" is a ${shape.solid.kind} solid, not a flat outline`);
      else {
        if (Math.abs(frame.v[0]) < 0.5 || Math.abs(frame.w[2]) < 0.5) throw new Error(`1:${op.scale} plan "${op.id}": the bone ${shape.bone} does not stand, use "plan-sweep"`);
        const mid = (u0 + u1) / 2;
        const at = sheet ? (shape.solid.z0 + shape.solid.z1) / 2 : 0;
        const flat = sheet ? pointsOf(shape).map(([b, c]) => [frame.o[0] + at * frame.u[0] + b * frame.v[0], frame.o[2] + c * frame.w[2]] as const) : pointsOf(shape);
        const raw = flat.map(([x, z]) => [(x - frame.o[0] - mid * frame.u[0]) / frame.v[0], (z - frame.o[2]) / frame.w[2]] as const);
        const [b0, b1] = rangeOf(raw, 0);
        const keep = Math.max(0.2, (Math.abs(b1 - b0) * Math.abs(frame.v[0]) - (u1 - u0) * Math.abs(frame.u[0])) / (Math.abs(b1 - b0) * Math.abs(frame.v[0])));
        section = raw.map(([b, c]) => [(b0 + b1) / 2 + (b - (b0 + b1) / 2) * keep, c] as const);
      }
      const [v0, v1] = rangeOf(section, 0);
      const box = (p: readonly number[]): P2[] => [[p[0]!, v0], [p[0]!, v1], [p[1]!, v0], [p[1]!, v1]];
      const beyond = (p: readonly number[]): number => Math.max(...ANGLES.map((angle) => support(box(p), angle) - support(target, angle)));
      const inside = op.fit === "inside";
      const middle = (u0 + u1) / 2;
      const span = inside ? search([middle - 0.01, middle + 0.01], (p) => 100 * Math.max(0, beyond(p) - 0.002) - (p[1]! - p[0]!) + (p[0]! >= p[1]! - 0.002 ? 1 : 0)) : search([u0, u1], (p) => mismatch(box(p), target) + (p[0]! >= p[1]! - 0.002 ? 1 : 0));
      for (const key of GEOMETRY_KEYS) if (key !== "closed") delete shape[key];
      Object.assign(shape, { kind: "poly", points: section.map(([b, c]) => [round(b, 4), round(c, 4)]), closed: true, solid: { kind: "extruded", sweep: "u", z0: round(span[0]!), z1: round(span[1]!) } });
      return `${settle(shape, before) ? "fitted" : "unchanged"}: section v [${v0.toFixed(3)}, ${v1.toFixed(3)}], span ${JSON.stringify(shape.solid)} footprint mismatch ${(mismatch(box(span), target) * 1000).toFixed(1)} mm`;
    }
    case "anchor-split": {
      const index = asset.anchors.findIndex((candidate: any) => candidate.id === op.id);
      if (index < 0) {
        if ((op.into as any[]).every((part) => asset.anchors.some((candidate: any) => candidate.id === part.id))) return "unchanged";
        throw new Error(`no anchor "${op.id}"`);
      }
      const whole = asset.anchors[index];
      asset.anchors.splice(index, 1, ...(op.into as any[]).map(({ event: _event, ...part }) => ({ ...structuredClone(whole), ...part })));
      let places = 0;
      const visit = (node: any): void => {
        if (Array.isArray(node)) {
          for (let i = node.length - 1; i >= 0; i--) {
            const entry = node[i];
            if (entry === op.id) (node.splice(i, 1, ...(op.into as any[]).map((part) => part.id)), places++);
            else if (entry !== null && typeof entry === "object" && entry.anchor === op.id) (node.splice(i, 1, ...(op.into as any[]).map((part) => ({ ...structuredClone(entry), anchor: part.id }))), places++);
            else if (entry !== null && typeof entry === "object" && entry.target === op.id) (node.splice(i, 1, ...(op.into as any[]).map((part) => ({ ...structuredClone(entry), target: part.id, ...(part.event ?? {}) }))), places++);
            else visit(entry);
          }
          return;
        }
        if (node === null || typeof node !== "object") return;
        for (const [key, value] of Object.entries(node)) {
          if (value === op.id && key !== "id") throw new Error(`"${op.id}" is named by the single field "${key}": decide by hand which part it means`);
          visit(value);
        }
      };
      for (const [key, value] of Object.entries(asset)) if (key !== "anchors") visit(value);
      return `split into ${(op.into as any[]).map((part) => part.id).join(", ")}, ${places} references follow`;
    }
    case "thicken": {
      const shape = need(op.scale, op.view, op.id, op.bone);
      if (shape.solid?.kind !== "extruded") throw new Error(`1:${op.scale} ${op.view} "${op.id}" is not an extruded plate`);
      const frame = frames[shape.bone]!.frame;
      const axis = shape.solid.sweep as "u" | "v" | "w";
      const before = snapshot(shape);
      const spanFrom = (view: string): [number, number] | undefined => {
        const other = find(op.scale, view, op.twin ?? op.id, shape.bone);
        if (other === undefined || other === shape) return undefined;
        const points = pointsOf(other);
        if (other.solid?.kind === "revolved") {
          const { av, aw, radial } = revolvedOf(other, frame);
          const reach = Math.max(...radial.map(([, r]) => r));
          return axis === "u" ? rangeOf(radial, 0) : axis === "v" ? [av - reach, av + reach] : [aw - reach, aw + reach];
        }
        const sweep = other.solid?.kind === "extruded" ? other.solid.sweep : view === "elevation" ? "w" : undefined;
        if (sweep === undefined || sweep === axis) return undefined;
        const coordinates = sweep === "w" ? ["u", "v"] : sweep === "v" ? ["u", "w"] : ["v", "w"];
        return rangeOf(points, coordinates.indexOf(axis) as 0 | 1);
      };
      const whole = (op.from ? [op.from] : ["elevation", "front", "plan"]).map(spanFrom).find((range) => range !== undefined);
      if (whole === undefined) throw new Error(`1:${op.scale} ${op.view} "${op.id}": no other view gives the depth along ${axis}`);
      const local: [number, number] = [(whole[0] + whole[1]) / 2 - ((whole[1] - whole[0]) / 2) * (op.share ?? 1), (whole[0] + whole[1]) / 2 + ((whole[1] - whole[0]) / 2) * (op.share ?? 1)];
      const base = axis === "w" ? frame.o[2] : 0;
      if (Math.abs(shape.solid.z0 - local[0] - base) < 0.0003 && Math.abs(shape.solid.z1 - local[1] - base) < 0.0003) return "unchanged";
      if (op.hold !== false) {
        const posed = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: Number(op.scale) } }).bones[shape.bone].frame;
        const paper = (vector: { x: number; y: number; z: number }): P2 => (op.view === "elevation" ? [vector.x, vector.y] : op.view === "front" ? [vector.z, vector.y] : [vector.x, vector.z]);
        const [first, second] = axis === "w" ? ["u", "v"] : axis === "v" ? ["u", "w"] : ["v", "w"];
        const [P, Q, A] = [paper(posed[first]), paper(posed[second]), paper(posed[axis])];
        const det = P[0] * Q[1] - Q[0] * P[1];
        if (Math.abs(det) < 0.3) throw new Error(`1:${op.scale} ${op.view} "${op.id}": the plate does not face the view in the default pose (the view sees ${Math.abs(det).toFixed(2)} of its area)`);
        const c: P2 = [(A[0] * Q[1] - Q[0] * A[1]) / det, (P[0] * A[1] - A[0] * P[1]) / det];
        const middle = (shape.solid.z0 + shape.solid.z1) / 2 - base;
        const moved = (to: number): [number, number][] => pointsOf(shape).map(([x, y]) => [x + c[0] * (middle - to), y + c[1] * (middle - to)]);
        const common = polygonClipping.intersection([moved(local[0])], [moved(local[1])]);
        const area = (ring: readonly (readonly [number, number])[]): number => Math.abs(ring.reduce((sum, point, i) => sum + point[0] * ring[(i + 1) % ring.length]![1] - ring[(i + 1) % ring.length]![0] * point[1], 0)) / 2;
        const ring = common.map((polygon) => polygon[0]!).sort((x, y) => area(y) - area(x))[0];
        if (ring === undefined || area(ring) < (op.least ?? 0.5) * area(pointsOf(shape))) throw new Error(`1:${op.scale} ${op.view} "${op.id}": the plate is too oblique to the view to keep its drawing at the depth [${local.map((value) => value.toFixed(3))}] (the sweep moves it ${(Math.hypot(c[0], c[1]) * (local[1] - local[0]) * 1000).toFixed(0)} mm on the sheet)`);
        for (const key of GEOMETRY_KEYS) if (key !== "closed" && key !== "solid") delete shape[key];
        Object.assign(shape, { kind: "poly", points: ring.slice(0, -1).map(([x, y]) => [round(x, 4), round(y, 4)]), closed: true });
      }
      shape.solid = { kind: "extruded", sweep: axis, z0: round(local[0] + base), z1: round(local[1] + base) };
      return `${settle(shape, before) ? "thickened" : "unchanged"}: ${axis} [${shape.solid.z0}, ${shape.solid.z1}]`;
    }
    case "keep-as": {
      if (shapesOf(op.scale, op.view).some((shape) => shape.id === op.as)) return "unchanged";
      const shape = need(op.scale, op.view, op.id, op.bone);
      const list = shapesOf(op.scale, op.view);
      list.splice(list.indexOf(shape), 0, { ...structuredClone(shape), id: op.as });
      return `kept as ${op.as}: ${apply({ op: "rebind", scale: op.scale, view: op.view, id: op.as, to: op.to })}`;
    }
    case "rebind": {
      const shape = need(op.scale, op.view, op.id, op.bone);
      if (shape.bone === op.to) return "unchanged";
      if (shape.solid !== undefined || shape.kind !== "poly") throw new Error(`1:${op.scale} ${op.view} "${op.id}": rebind moves a flat poly only`);
      const drawn = (): P2[] => {
        const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: Number(op.scale) } });
        return state.shapes[op.view].find((entry: any) => entry.shape === shape || (entry.shape.id === shape.id && entry.shape.bone === shape.bone)).rings[0].map((p: any) => [p.x, p.y] as const);
      };
      const centre = (points: readonly P2[]): P2 => [(rangeOf(points, 0)[0] + rangeOf(points, 0)[1]) / 2, (rangeOf(points, 1)[0] + rangeOf(points, 1)[1]) / 2];
      const ring = drawn();
      const before = centre(ring);
      shape.bone = op.to;
      shape.points = ring.map(([x, y]) => [round(x, 4), round(y, 4)]);
      for (let pass = 0; pass < 3; pass++) {
        const now = centre(drawn());
        shape.points = (shape.points as P2[]).map(([x, y]) => [round(x + before[0] - now[0], 4), round(y + before[1] - now[1], 4)]);
      }
      const left = centre(drawn());
      return `rebound to ${op.to}, default pose within ${(Math.hypot(left[0] - before[0], left[1] - before[1]) * 1000).toFixed(1)} mm of where it was`;
    }
    case "core": {
      const twin = need(op.scale, "elevation", op.id, op.bone);
      const plan = need(op.scale, "plan", op.id, twin.bone);
      const name = op.as ?? `${op.id}-core`;
      const list = shapesOf(op.scale, "plan");
      let core = find(op.scale, "plan", name, twin.bone);
      const before = core === undefined ? undefined : JSON.stringify(core);
      const lateral = plan.solid === undefined ? rangeOf(pointsOf(plan), 1) : plan.solid.kind === "extruded" && plan.solid.sweep === "u" ? rangeOf(pointsOf(plan), 1).map((w) => w + frames[twin.bone]!.frame.o[2]) : undefined;
      if (lateral === undefined) throw new Error(`1:${op.scale} plan "${op.id}" is a solid: its lateral range is not read, state the core by hand`);
      const mid = (lateral[0]! + lateral[1]!) / 2;
      const half = ((lateral[1]! - lateral[0]!) / 2) * (op.share ?? 0.6);
      const next = { ...structuredClone(plan), id: name, kind: "poly", points: structuredClone(pointsOf(twin)), closed: true, solid: { kind: "extruded", sweep: "w", z0: round(mid - half), z1: round(mid + half) } };
      for (const key of ["x", "y", "w", "h", "d", "followTip"]) delete (next as any)[key];
      if (core === undefined) list.splice(list.indexOf(plan), 0, next);
      else {
        for (const key of Object.keys(core)) delete core[key];
        Object.assign(core, next);
      }
      return before === JSON.stringify(next) ? "unchanged" : `core ${name}: the elevation outline swept over [${next.solid.z0}, ${next.solid.z1}]`;
    }
    case "carry": {
      const source = need(op.from, op.view, op.id, op.bone);
      const target = find(op.scale, op.view, op.id, source.bone);
      if (target !== undefined && op.geometry === true) {
        const was = JSON.stringify(target);
        copyGeometry(source, target);
        return JSON.stringify(target) === was ? "unchanged" : "geometry carried";
      }
      const next = { ...structuredClone(source), ...(op.set ?? {}) };
      if (target === undefined) {
        shapesOf(op.scale, op.view).push(next);
        return "carried";
      }
      const before = JSON.stringify(target);
      for (const key of Object.keys(target)) delete target[key];
      Object.assign(target, next);
      return JSON.stringify(target) === before ? "unchanged" : "carried";
    }
    case "carry-solid": {
      const source = asset.volumes[op.from].body.find((solid: any) => solid.id === op.id);
      if (source === undefined) throw new Error(`volume 1:${op.from} has no body solid "${op.id}"`);
      const list = asset.volumes[op.scale].body as any[];
      const index = list.findIndex((solid) => solid.id === op.id);
      const next = { ...structuredClone(source), ...(op.set ?? {}) };
      if (index >= 0 && JSON.stringify(list[index]) === JSON.stringify(next)) return "unchanged";
      if (index >= 0) list[index] = next;
      else list.push(next);
      return "carried";
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
