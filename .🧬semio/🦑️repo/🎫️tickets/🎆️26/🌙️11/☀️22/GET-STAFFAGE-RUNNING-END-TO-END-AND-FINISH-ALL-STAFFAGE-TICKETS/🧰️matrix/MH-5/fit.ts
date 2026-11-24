/** 🪡️ The multiview repair of one asset of batch MH-5, the last step of its chain: `bun fit.ts <id|file.json> --ops <ops.json> [--write] [--out <file>]` (from `T`). It is `../MR/fit.ts` (ops `front-sweep`, `plan-revolve`, `extrude`, `copy`, `set`, `remove`, `anchor`, unchanged) plus the op the people of this batch need:
 *
 * - `{ "op": "sync", scale, id, bone?, "convert"?: [view…], "sweep"?: { view: "u" | "v" | "w" }, "lock"?: [view…], "slack"?: metres, "remap"?: false, "zTip"?: false }`: the shape `id` of the bone in the three views becomes three pictures of one solid. A flat shape of a view named in `convert` first becomes a prism: its outline, projected along the viewing direction onto the plane across the bone axis that points most at the viewer (or the axis `sweep` names), is the profile, and the sweep runs along that axis over the extent the elevation twin has there (a flat ELEVATION shape becomes its own outline swept laterally: the solid the twisted heads lacked). Then the span `z0`, `z1`, `zTip` and the ranges of the two profile coordinates of every extruded shape of the three are fitted together: the fit minimises the largest disagreement of the extents the matrix compares (elevation and front: height, elevation and plan: forward, front and plan: lateral) over every pose, the 49 probe instants of every clip and nine instants of every transition (`solid.ts`), under the condition that no shape moves in its OWN view in the default pose by more than `slack` (default 0.3 mm, the pose the silhouette guard measures): only what a view does not show there is free. `lock` keeps a view as it is, `remap: false` fits the spans only. The search runs on a 0.1 mm grid from the present values and a refit is kept only when it gains more than 0.5 mm, so a second run changes nothing.
 * - `{ "op": "clip", scale, view, id, "axis"?: "u" | "v" | "w", "range"?: [lo, hi], "side"?: "low" | "high" }`: cuts the profile of an extruded shape to the range of the elevation twin along a bone axis (hidden length inside the neighbouring part).
 * - `{ "op": "anchor-split", id, "into": [{ id, title, z, … }] }`: one anchor per physical contact (D45), references followed.
 * @see ./mgate.ts — the measurement in seconds; ../MR/gate.ts — the proof */
import { join } from "node:path";
import { S, dot, readDoc, round, sub, support, writeDoc, type V3 } from "../MR/lib.ts";
import { LOOK, PAIRS, VIEWS, bodyOf, extent, pointsOf as flatPoints, prismPoints, restOf, ringsOf, subjectsOf, toLocal, toleranceOf, worst, type Body, type Frame as F3, type Subject, type View } from "./solid.ts";

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
let asset = read.doc.asset;
const ops = (flag("ops") === undefined ? [] : JSON.parse(await Bun.file(flag("ops")!).text())) as any[];
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
  const lateral = shape.solid !== undefined ? undefined : op.view === "plan" ? rangeOf(pointsOf(shape), 1).map((z) => z - frame.o[2]) : edgeOn ? rangeOf(pointsOf(shape), 0).map((z) => z - frame.o[2]) : undefined;
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

/** 🥚️ The convex hull of points in the plane (the extents of a prism depend on the hull of its profile only). */
function hull2(points: readonly P2[]): P2[] {
  const sorted = [...points].sort((a, b) => a[0] - b[0] || a[1] - b[1]);
  if (sorted.length < 4) return sorted;
  const cross = (o: P2, a: P2, b: P2): number => (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0]);
  const half = (list: readonly P2[]): P2[] => {
    const out: P2[] = [];
    for (const p of list) {
      while (out.length >= 2 && cross(out[out.length - 2]!, out[out.length - 1]!, p) <= 0) out.pop();
      out.push(p);
    }
    out.pop();
    return out;
  };
  return [...half(sorted), ...half([...sorted].reverse())];
}

/** 🧲️ The op `sync`: see the header. */
let SUBJECTS: Subject[] | undefined;
function sync(op: any): string {
  const subjects = (SUBJECTS ??= subjectsOf(asset));
  const rest = restOf(asset);
  const tolerance = toleranceOf(asset);
  const bone: string = op.bone ?? VIEWS.map((view) => find(op.scale, view, op.id)).find((shape) => shape !== undefined)?.bone;
  if (bone === undefined) throw new Error(`1:${op.scale} has no shape "${op.id}"`);
  const r = rest[bone]!;
  const length = lengthOf(bone);
  const shapes = Object.fromEntries(VIEWS.map((view) => [view, find(op.scale, view, op.id, bone)])) as Record<View, any>;
  const before = Object.fromEntries(VIEWS.map((view) => [view, shapes[view] === undefined ? undefined : JSON.stringify(shapes[view])]));
  const bodies = (): Record<View, Body | undefined> => Object.fromEntries(VIEWS.map((view) => [view, shapes[view] === undefined ? undefined : bodyOf(asset, rest, view, shapes[view], shapes.elevation)])) as Record<View, Body | undefined>;
  const measure = (set: Record<View, Body | undefined>): string => PAIRS.map(([name, a, b, k]) => (set[a] === undefined || set[b] === undefined ? `${name} -` : `${name} ${(worst(set[a]!, set[b]!, bone, k, subjects).delta * 1000).toFixed(0)}`)).join(", ");
  const was = measure(bodies());
  if (op.op === "measure") {
    const set = bodies();
    const values = PAIRS.map(([, a, b, kk]) => (set[a] === undefined || set[b] === undefined ? (shapes[a] === undefined || shapes[b] === undefined ? 0 : -1) : worst(set[a]!, set[b]!, bone, kk, subjects).delta));
    return `unchanged MEASURE ${JSON.stringify(values.map((value) => Math.round(value * 10000) / 10))} tolerance ${(tolerance * 1000).toFixed(1)}`;
  }
  const axes = { u: r.u, v: r.v, w: r.w } as const;
  const notes: string[] = [];
  for (const view of (op.convert ?? []) as View[]) {
    const shape = shapes[view];
    if (shape === undefined) throw new Error(`1:${op.scale} ${view} has no shape "${op.id}" on ${bone}`);
    if (shape.solid !== undefined) continue;
    const rings = ringsOf(shape);
    if (rings.length !== 1) throw new Error(`1:${op.scale} ${view} "${op.id}" has ${rings.length} rings: only a single outline becomes a prism`);
    const reference = view === "elevation" ? (shapes.front ?? shapes.plan) : shapes.elevation;
    const referenceBody = reference === undefined ? undefined : bodyOf(asset, rest, view === "elevation" ? (shapes.front !== undefined ? "front" : "plan") : "elevation", reference, shapes.elevation);
    const rangeAlong = (i: number): [number, number] | undefined => {
      if (referenceBody === undefined || !("local" in referenceBody)) return undefined;
      const values = referenceBody.local.map((p) => p[i]!);
      return Math.max(...values) - Math.min(...values) < 0.004 ? undefined : [Math.min(...values), Math.max(...values)];
    };
    if (view === "elevation") {
      const span = rangeAlong(2) ?? [-0.02, 0.02];
      const points = rings[0]!.map(([a, b]) => [round(a, 4), round(b, 4)]);
      for (const key of GEOMETRY_KEYS) if (key !== "closed") delete shape[key];
      Object.assign(shape, { kind: "poly", points, closed: true, solid: { kind: "extruded", sweep: "w", z0: round(span[0] + r.o[2]), z1: round(span[1] + r.o[2]) } });
      notes.push("elevation flat → sweep w");
      continue;
    }
    const look = LOOK[view];
    const sweep: "u" | "v" | "w" = op.sweep?.[view] ?? (["u", "v", "w"] as const).reduce((best, name) => (Math.abs(axes[name][look]) > Math.abs(axes[best][look]) ? name : best), "u" as "u" | "v" | "w");
    const a = axes[sweep];
    if (Math.abs(a[look]) < 0.3) throw new Error(`1:${op.scale} ${view} "${op.id}": the ${sweep} axis of ${bone} lies across the viewing direction`);
    const index = { u: 0, v: 1, w: 2 }[sweep];
    const span = rangeAlong(index) ?? [-0.02, 0.02];
    const middle = (span[0] + span[1]) / 2;
    const profile = rings[0]!.map(([p, q]) => {
      const x: [number, number, number] = view === "plan" ? [p, r.o[1], q] : [r.o[0], q, p];
      const d = (x[0] - r.o[0]) * a[0] + (x[1] - r.o[1]) * a[1] + (x[2] - r.o[2]) * a[2];
      x[look] += (middle - d) / a[look];
      const local = toLocal(r, x);
      return (sweep === "u" ? [local[1], local[2]] : sweep === "v" ? [local[0], local[2]] : [local[0], local[1]]).map((value) => round(value, 4));
    });
    const base = sweep === "w" ? r.o[2] : 0;
    for (const key of GEOMETRY_KEYS) if (key !== "closed") delete shape[key];
    Object.assign(shape, { kind: "poly", points: profile, closed: true, solid: { kind: "extruded", sweep, z0: round(span[0] + base), z1: round(span[1] + base) } });
    notes.push(`${view} flat → sweep ${sweep}`);
  }
  const defaultFrame = (subjects.find((subject) => subject.isDefault) ?? subjects[0]!).frames[bone]!;
  const slack: number = op.slack ?? 0.0003;
  type Param = { view: View; kind: "span" | "remap"; coordinate: 0 | 1; from: [number, number] };
  const groups: Param[] = [];
  for (const view of VIEWS) {
    const shape = shapes[view];
    if (shape?.solid?.kind !== "extruded" || shape.solid.weights?.length || (op.lock ?? []).includes(view)) continue;
    groups.push({ view, kind: "span", coordinate: 0, from: [0, 0] });
    if (ringsOf(shape).length !== 1 || op.remap === false) continue;
    for (const i of [0, 1] as const) {
      const values = flatPoints(shape).map((p) => p[i]!);
      if (Math.max(...values) - Math.min(...values) > 0.002) groups.push({ view, kind: "remap", coordinate: i, from: [Math.min(...values), Math.max(...values)] });
    }
  }
  if (groups.length > 0) {
    const fixed = bodies();
    const base = Object.fromEntries(VIEWS.map((view) => [view, shapes[view] === undefined ? [] : hull2(flatPoints(shapes[view]))])) as Record<View, P2[]>;
    const tipOf = (view: View): boolean => op.zTip !== false && shapes[view].solid.sweep !== "u";
    const start: number[] = [];
    const slots = groups.map((group) => {
      const at = start.length;
      const solid = shapes[group.view].solid;
      if (group.kind === "span") start.push(solid.z0, solid.z1, ...(tipOf(group.view) ? [solid.zTip?.[0] ?? solid.z0, solid.zTip?.[1] ?? solid.z1] : []));
      else start.push(group.from[0], group.from[1]);
      return at;
    });
    const freeViews = [...new Set(groups.map((group) => group.view))];
    const build = (p: readonly number[]): Record<View, Body | undefined> => {
      const out = { ...fixed };
      for (const view of freeViews) {
        let profile = base[view];
        let solid = shapes[view].solid;
        groups.forEach((group, g) => {
          if (group.view !== view) return;
          const at = slots[g]!;
          if (group.kind === "span") solid = { ...solid, z0: p[at]!, z1: p[at + 1]!, ...(tipOf(view) ? { zTip: [p[at + 2]!, p[at + 3]!] } : {}) };
          else {
            const k = (p[at + 1]! - p[at]!) / (group.from[1] - group.from[0]);
            profile = profile.map((q) => (group.coordinate === 0 ? [p[at]! + (q[0] - group.from[0]) * k, q[1]] : [q[0], p[at]! + (q[1] - group.from[0]) * k]) as P2);
          }
        });
        out[view] = { local: prismPoints(profile, solid, r, length) };
      }
      return out;
    };
    const used = subjects.filter((subject) => subject.frames[bone] !== undefined);
    const cache: Record<string, [number, number][]> = {};
    const shown = (set: Record<View, Body | undefined>): number[] => freeViews.flatMap((view) => ([0, 1, 2] as const).filter((k) => k !== LOOK[view]).flatMap((k) => extent(set[view]!, defaultFrame, k)));
    const shownStart = shown(build(start));
    const valid = (p: readonly number[]): boolean => groups.every((group, g) => p[slots[g]!]! < p[slots[g]! + 1]! - 0.002 && (group.kind !== "span" || !tipOf(group.view) || p[slots[g]! + 2]! < p[slots[g]! + 3]! - 0.002));
    const cost = (p: readonly number[]): number => {
      if (!valid(p)) return 9;
      const set = build(p);
      let top = 0;
      let mean = 0;
      for (const [, a, b, k] of PAIRS) {
        if (set[a] === undefined || set[b] === undefined) continue;
        let sum = 0;
        for (let i = 0; i < used.length; i++) {
          const f = used[i]!.frames[bone]!;
          const ra = freeViews.includes(a) ? extent(set[a]!, f, k) : (cache[`${a}${k}`] ??= used.map((subject) => extent(fixed[a]!, subject.frames[bone]!, k)))[i]!;
          const rb = freeViews.includes(b) ? extent(set[b]!, f, k) : (cache[`${b}${k}`] ??= used.map((subject) => extent(fixed[b]!, subject.frames[bone]!, k)))[i]!;
          const delta = Math.max(Math.abs(ra[0] - rb[0]), Math.abs(ra[1] - rb[1]));
          sum += delta;
          if (delta > top) top = delta;
        }
        mean += sum / Math.max(1, used.length);
      }
      const drift = Math.max(0, ...shown(set).map((value, i) => Math.abs(value - shownStart[i]!) - slack));
      return top + 0.05 * mean + 20 * drift + (0.001 * p.reduce((sum, value, i) => sum + Math.abs(value - start[i]!), 0)) / p.length;
    };
    const p = start.map((value) => round(value, 4));
    let best = cost(p);
    for (let step = 0.0128; step >= 0.00009; step /= 2)
      for (let moved = true, guard = 0; moved && guard < 400; guard++) {
        moved = false;
        for (let i = 0; i < p.length; i++)
          for (const d of [step, -step]) {
            const old = p[i]!;
            p[i] = round(old + d, 4);
            const c = cost(p);
            if (c < best - 1e-7) (best = c), (moved = true);
            else p[i] = old;
          }
      }
    if (cost(start) - best > 0.0005) {
      const done: string[] = [];
      groups.forEach((group, g) => {
        const shape = shapes[group.view];
        const at = slots[g]!;
        if (group.kind === "span") {
          if (Math.abs(p[at]! - start[at]!) + Math.abs(p[at + 1]! - start[at + 1]!) + (tipOf(group.view) ? Math.abs(p[at + 2]! - start[at + 2]!) + Math.abs(p[at + 3]! - start[at + 3]!) : 0) < 0.00005) return;
          const tip = tipOf(group.view) && (shape.solid.zTip !== undefined || Math.abs(p[at + 2]! - p[at]!) > 0.00045 || Math.abs(p[at + 3]! - p[at + 1]!) > 0.00045);
          shape.solid = { ...shape.solid, z0: p[at]!, z1: p[at + 1]!, ...(tip ? { zTip: [p[at + 2]!, p[at + 3]!] } : {}) };
          if (!tip) delete shape.solid.zTip;
          done.push(`${group.view} span`);
        } else {
          if (Math.abs(p[at]! - group.from[0]) + Math.abs(p[at + 1]! - group.from[1]) < 0.00005) return;
          const k = (p[at + 1]! - p[at]!) / (group.from[1] - group.from[0]);
          const points = flatPoints(shape).map((q) => (group.coordinate === 0 ? [round(p[at]! + (q[0] - group.from[0]) * k, 4), round(q[1], 4)] : [round(q[0], 4), round(p[at]! + (q[1] - group.from[0]) * k, 4)]));
          const solid = shape.solid;
          for (const key of GEOMETRY_KEYS) if (key !== "closed") delete shape[key];
          Object.assign(shape, { kind: "poly", points, closed: true, solid });
          done.push(`${group.view} profile ${group.coordinate} [${group.from.map((v) => v.toFixed(3))}]→[${p[at]!.toFixed(3)},${p[at + 1]!.toFixed(3)}]`);
        }
      });
      notes.push(`fitted ${done.join(", ")}`);
    }
  }
  const now = measure(bodies());
  const changed = VIEWS.some((view) => shapes[view] !== undefined && JSON.stringify(shapes[view]) !== before[view]);
  const over = PAIRS.some(([, a, b, k]) => { const set = bodies(); return set[a] !== undefined && set[b] !== undefined && worst(set[a]!, set[b]!, bone, k, subjects).delta > tolerance; });
  return `${changed ? "synced" : "unchanged"}${over ? " OVER" : ""}: ${bone} [${was}] → [${now}] mm${notes.length ? `; ${notes.join("; ")}` : ""}`;
}

/** ✂️ The op `clip`: the profile of a solid shape is cut to the range the elevation twin has along a bone axis (`axis`, default u; `range: [lo, hi]` states it instead; `side`: "low" or "high" cuts one end only), ring by ring: what lies beyond is length hidden inside the neighbouring part, and it is what makes the part taller, longer or wider than its twin when the bone turns. Several rings stay a `path`. */
function clipOp(op: any): string {
  const shape = need(op.scale, op.view, op.id, op.bone);
  if (shape.solid?.kind !== "extruded") throw new Error(`1:${op.scale} ${op.view} "${op.id}" is not an extruded solid`);
  const axis: "u" | "v" | "w" = op.axis ?? "u";
  const names = shape.solid.sweep === "u" ? ["v", "w"] : shape.solid.sweep === "v" ? ["u", "w"] : ["u", "v"];
  const k = names.indexOf(axis);
  if (k < 0) throw new Error(`1:${op.scale} ${op.view} "${op.id}": the profile of a sweep along ${shape.solid.sweep} has no ${axis} coordinate`);
  let range: [number, number] | undefined = op.range;
  if (range === undefined) {
    const twin = find(op.scale, "elevation", op.twin ?? op.id, shape.bone);
    const body = twin === undefined ? undefined : bodyOf(asset, restOf(asset), "elevation", twin, twin);
    if (body === undefined || !("local" in body)) throw new Error(`1:${op.scale} "${op.id}" has no elevation twin to take the range from`);
    const values = body.local.map((p) => p[{ u: 0, v: 1, w: 2 }[axis]]!);
    range = [Math.min(...values), Math.max(...values)];
  }
  if (op.side === "low") range = [range[0], Number.POSITIVE_INFINITY];
  if (op.side === "high") range = [Number.NEGATIVE_INFINITY, range[1]];
  const flip = (points: readonly P2[]): P2[] => (k === 0 ? [...points] : points.map(([x, y]) => [y, x] as const));
  const cut = (points: readonly P2[]): P2[] => {
    let ring = [...points];
    for (const [limit, keep] of [[range![0], (u: number) => u >= range![0] - 0.00015], [range![1], (u: number) => u <= range![1] + 0.00015]] as const) {
      if (!Number.isFinite(limit)) continue;
      const next: P2[] = [];
      for (let i = 0; i < ring.length; i++) {
        const [p, q] = [ring[i]!, ring[(i + 1) % ring.length]!];
        if (keep(p[0])) next.push(p);
        if (keep(p[0]) !== keep(q[0])) next.push([limit, p[1] + ((q[1] - p[1]) * (limit - p[0])) / (q[0] - p[0])] as const);
      }
      ring = next;
    }
    return ring.filter((p, i) => Math.hypot(p[0] - ring[(i + 1) % ring.length]![0], p[1] - ring[(i + 1) % ring.length]![1]) > 1e-6);
  };
  const before = ringsOf(shape);
  const rings = before.map((ring) => flip(cut(flip(ring)))).filter((ring) => ring.length >= 3);
  const span = (list: readonly (readonly P2[])[]): [number, number] => [Math.min(...list.flat().map((p) => p[k]!)), Math.max(...list.flat().map((p) => p[k]!))];
  const [was, now] = [span(before), span(rings)];
  if (rings.length === 0) throw new Error(`1:${op.scale} ${op.view} "${op.id}": nothing of the profile lies inside [${range}]`);
  if (Math.abs(was[0] - now[0]) < 0.0003 && Math.abs(was[1] - now[1]) < 0.0003) return "unchanged";
  const solid = shape.solid;
  for (const key of GEOMETRY_KEYS) delete shape[key];
  if (rings.length === 1) Object.assign(shape, { kind: "poly", points: rings[0]!.map(([x, y]) => [round(x, 4), round(y, 4)]), closed: true, solid });
  else Object.assign(shape, { kind: "path", d: rings.map((ring) => `M ${ring.map(([x, y]) => `${round(x, 4)} ${round(y, 4)}`).join(" L ")} Z`).join(" "), solid });
  return `clipped: ${axis} [${was.map((v) => v.toFixed(3))}] → [${now.map((v) => v.toFixed(3))}], ${rings.length} of ${before.length} rings`;
}

/** 🎈️ The op `scale-profile`: `{ "op": "scale-profile", scale, view, id, bone?, "coordinate": 0 | 1, "factor" }` multiplies one profile coordinate of a solid shape (the radius of a revolved profile about an axis through the bone: coordinate 1), for a body of revolution that is a few millimetres fuller than the silhouette guard allows. Not idempotent by itself: `auto.py` records its result as `set` ops. */
function scaleProfile(op: any): string {
  const shape = need(op.scale, op.view, op.id, op.bone);
  if (shape.kind !== "poly" || shape.solid === undefined) throw new Error(`1:${op.scale} ${op.view} "${op.id}" is not a solid poly`);
  shape.points = shape.points.map((p: number[]) => p.map((value, i) => (i === op.coordinate ? round(value * op.factor, 4) : value)));
  return `scaled: coordinate ${op.coordinate} × ${op.factor}`;
}

/** 🪓️ The op `anchor-split` (D45, one anchor per physical contact): `{ "op": "anchor-split", id, "into": [{ id, title, x?, y?, z?, footprintLengthM?, footprintWidthM?, bone? }…] }` replaces a lumped contact anchor by the listed anchors (each a copy of the lumped one with the stated members) and follows every reference in the document: a list of anchor ids (`poses[].contacts`) gets all new ids in its place, a list entry that names the anchor (`clips[].contacts[].anchor` with its interval) is repeated per new anchor, a single reference (`events[].target`) names the first. */
function anchorSplit(op: any): string {
  const index = asset.anchors.findIndex((candidate: any) => candidate.id === op.id);
  if (index < 0) {
    if ((op.into as any[]).every((entry) => asset.anchors.some((candidate: any) => candidate.id === entry.id))) return "unchanged";
    throw new Error(`no anchor "${op.id}"`);
  }
  const source = asset.anchors[index];
  const made = (op.into as any[]).map((entry) => ({ ...structuredClone(source), ...structuredClone(entry) }));
  asset.anchors.splice(index, 1, ...made);
  const ids = made.map((anchor) => anchor.id as string);
  let references = 0;
  const walk = (node: any): void => {
    if (Array.isArray(node)) {
      for (let i = node.length - 1; i >= 0; i--) {
        const item = node[i];
        if (item === op.id) (node.splice(i, 1, ...ids), references++);
        else if (item !== null && typeof item === "object" && !Array.isArray(item) && item.anchor === op.id && node !== asset.anchors) (node.splice(i, 1, ...ids.map((id) => ({ ...structuredClone(item), anchor: id }))), references++);
        else walk(item);
      }
      return;
    }
    if (node === null || typeof node !== "object") return;
    for (const [key, value] of Object.entries(node)) {
      if (value === op.id && key !== "id") ((node[key] = ids[0]), references++);
      else walk(value);
    }
  };
  for (const [key, value] of Object.entries(asset)) if (key !== "anchors") walk(value);
  return `split into ${ids.join(", ")}; ${references} references followed`;
}

/** 🪜️ The op `copy-scale`: `{ "op": "copy-scale", scale, id, bone?, "from": scale }` gives the shape `id` of the bone in the three views the geometry and solid it has in another band (the last resort for a part whose three pictures in this band cannot be made one solid without leaving the silhouette guard: the neighbouring band has the part as a solid). */
function copyScale(op: any): string {
  const done: string[] = [];
  for (const view of VIEWS) {
    const target = find(op.scale, view, op.id, op.bone);
    const source = find(op.from, view, op.id, op.bone ?? target?.bone);
    if (target === undefined || source === undefined) continue;
    const before = JSON.stringify(target);
    copyGeometry(source, target);
    done.push(`${view} ${JSON.stringify(target) === before ? "unchanged" : "copied"}`);
  }
  if (done.length === 0) throw new Error(`1:${op.from} has no shape "${op.id}"`);
  return done.join(", ");
}

function apply(op: any): string {
  switch (op.op) {
    case "front-sweep": return frontSweep(op);
    case "plan-revolve": return planRevolve(op);
    case "extrude": return frontExtrude(op);
    case "copy": return copy(op);
    case "sync": case "measure": return sync(op);
    case "clip": return clipOp(op);
    case "copy-scale": return copyScale(op);
    case "anchor-split": return anchorSplit(op);
    case "scale-profile": return scaleProfile(op);
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

/** 🧫️ `--try <candidates.json> --prefix <path>`: every entry `{ name: ops[] }` of the file is applied to a fresh copy of the document and written to `<path>.<name>.json`; each line is printed with the name in front, and an op `{ "op": "measure", scale, id, bone? }` prints the three disagreements of that shape without changing anything. Nothing is written to the asset itself. */
if (flag("try") !== undefined) {
  const candidates = JSON.parse(await Bun.file(flag("try")!).text()) as Record<string, any[]>;
  for (const [name, list] of Object.entries(candidates)) {
    read.doc = JSON.parse(read.text);
    asset = read.doc.asset;
    let failed = 0;
    for (const op of list) {
      let result: string;
      try {
        result = apply(op);
      } catch (error) {
        failed++;
        result = `unchanged, REFUSED: ${(error as Error).message}`;
      }
      console.log(`${name}\t${op.op}\t${result}`);
    }
    if (failed === 0) writeDoc(read, `${flag("prefix")}.${name}.json`);
    console.log(`${name}\tdone\t${failed === 0 ? "written" : "refused"}`);
  }
  process.exit(0);
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
