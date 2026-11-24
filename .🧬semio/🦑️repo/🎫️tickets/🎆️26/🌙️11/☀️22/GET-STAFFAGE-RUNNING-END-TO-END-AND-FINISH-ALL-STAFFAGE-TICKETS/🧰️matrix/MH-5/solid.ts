/** 🧊️ The model the MH-5 repair measures with: the body every drawn shape implies (the engine's own placement rules of `evaluate`, written as a rigid point cloud in the frame of the bone) and the bone frames of every subject of the matrix (poses, clip instants, transitions), so the extents the matrix compares are known in milliseconds instead of minutes. The real gate (`../MR/gate.ts`) stays the proof; `mgate.ts` prints both tiers from this model.
 *
 * Bodies: a flat elevation shape is a plate in the (u, v) plane of its bone; a flat front or plan shape with an elevation twin is a plate at the middle depth of that twin at rest; `extruded` sweeps u, v, w and `revolved` about the u axis of the own bone are the prisms and bodies of revolution of `solid3`. Everything else (a flat shape without a twin, `followTip` growth, a foreign axis bone, skinned solids) is `undefined`: not modelled, only the real gate judges it.
 * @see ../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts */
import { join } from "node:path";
import { S } from "../MR/lib.ts";

const T = join(S, "🧪️tests", "🔭️multiview-coverage");
const engine = await import(join(T, "🧰️.ts"));
const { evaluate, flattenGeometry } = await import(join(S, "🔨️modules", "🧮️evaluation", "🟦️.ts"));
const { restFrames } = await import(join(S, "🔨️modules", "🦴️rig", "🟦️.ts"));

export type V3 = readonly [number, number, number];
export type P2 = readonly [number, number];
export type Frame = { o: V3; u: V3; v: V3; w: V3 };
export type Subject = { id: string; isDefault: boolean; frames: Record<string, Frame> };
export type View = "elevation" | "front" | "plan";
export type Body = { local: V3[] } | { world: (frame: Frame) => V3[] };

export const VIEWS: readonly View[] = ["elevation", "front", "plan"];
/** 🧭️ The world axis a view looks along (elevation z, front x, plan y). */
export const LOOK: Record<View, 0 | 1 | 2> = { elevation: 2, front: 0, plan: 1 };
/** 🤝️ The pairs of views the matrix compares and the world axis they share. */
export const PAIRS: readonly (readonly [string, View, View, 0 | 1 | 2])[] = [["height", "elevation", "front", 1], ["forward", "elevation", "plan", 0], ["lateral", "front", "plan", 2]];

const vec = (p: any): V3 => (Array.isArray(p) ? (p as unknown as V3) : [p.x, p.y, p.z]);
const frameOf = (f: any): Frame => ({ o: vec(f.o), u: vec(f.u), v: vec(f.v), w: vec(f.w) });
const dot = (a: V3, b: V3): number => a[0] * b[0] + a[1] * b[1] + a[2] * b[2];

/** 📏️ The tolerance of the matrix for an asset in metres. */
export const toleranceOf = (asset: any): number => engine.toleranceM(asset);

/** 🦴️ The rest frames of the bones. */
export function restOf(asset: any): Record<string, Frame> {
  return Object.fromEntries(Object.entries(restFrames(asset.rig) as Record<string, { frame: any }>).map(([bone, value]) => [bone, frameOf(value.frame)]));
}

/** 🎞️ The bone frames of every subject of the matrix plus the 49 probe instants of every clip (the matrix picks its extreme instants among them) and denser transitions, evaluated on a copy of the asset without drawn shapes (the frames depend on rig, poses and clips only). */
export function subjectsOf(source: any): Subject[] {
  const asset = structuredClone(source);
  const keys = Object.keys(asset.representations).filter((key) => engine.authoredOf(asset, key) !== undefined);
  for (const key of keys) for (const view of VIEWS) asset.representations[key].geometry[view] = asset.representations[key].geometry[view].filter((shape: any) => shape.role === "ground").slice(0, 1);
  const selector = engine.scaleSelector(Number(keys[0]));
  const queries: { id: string; isDefault: boolean; query: unknown }[] = engine.subjects(asset, Number(keys[0])).map((subject: any) => ({ id: subject.id, isDefault: subject.isDefault, query: subject.query }));
  for (const clip of asset.clips ?? []) for (let i = 0; i <= 48; i++) queries.push({ id: `clip ${clip.id} probe ${i}`, isDefault: false, query: { kind: "clip", clip: clip.id, timeS: (clip.durationS * 0.999 * i) / 48 } });
  for (const [from, to] of engine.transitionPairs(asset) as [string, string][]) for (const fraction of [0.125, 0.375, 0.625, 0.875]) queries.push({ id: `transition ${from}>${to} @${fraction}`, isDefault: false, query: { kind: "transition", from, to, durationS: 1, timeS: fraction } });
  const out: Subject[] = [];
  for (const { id, isDefault, query } of queries) {
    let state: any;
    try {
      state = evaluate(asset, query, selector);
    } catch {
      continue;
    }
    out.push({ id, isDefault, frames: Object.fromEntries(Object.entries(state.bones).map(([bone, value]: [string, any]) => [bone, frameOf(value.frame)])) });
  }
  return out;
}

/** 🖊️ The rings of a shape as point lists. */
export const ringsOf = (shape: any): P2[][] => (shape.kind === "poly" ? [shape.points as P2[]] : flattenGeometry(shape).rings.map((ring: any[]) => ring.map((p) => [p.x, p.y] as const)));
export const pointsOf = (shape: any): P2[] => ringsOf(shape).flat();

/** 📍️ A world point of the rest pose in the local coordinates (u, v, w) of a rest frame. */
export const toLocal = (rest: Frame, p: V3): V3 => {
  const d: V3 = [p[0] - rest.o[0], p[1] - rest.o[1], p[2] - rest.o[2]];
  return [dot(d, rest.u), dot(d, rest.v), dot(d, rest.w)];
};
export const toWorld = (f: Frame, p: V3): V3 => [f.o[0] + p[0] * f.u[0] + p[1] * f.v[0] + p[2] * f.w[0], f.o[1] + p[0] * f.u[1] + p[1] * f.v[1] + p[2] * f.w[1], f.o[2] + p[0] * f.u[2] + p[1] * f.v[2] + p[2] * f.w[2]];

const CIRCLE = Array.from({ length: 24 }, (_, i) => [Math.cos((2 * Math.PI * i) / 24), Math.sin((2 * Math.PI * i) / 24)] as const);

/** 🧱️ The local points of an extruded solid from its profile. */
export function prismPoints(profile: readonly P2[], solid: any, rest: Frame, length: number): V3[] {
  const base = solid.sweep === "w" ? rest.o[2] : 0;
  const out: V3[] = [];
  for (const [a, b] of profile) {
    const t = solid.zTip === undefined || length <= 0 ? 0 : Math.min(Math.max(a / length, 0), 1);
    for (const c of [solid.z0 + ((solid.zTip?.[0] ?? solid.z0) - solid.z0) * t - base, solid.z1 + ((solid.zTip?.[1] ?? solid.z1) - solid.z1) * t - base]) out.push(solid.sweep === "u" ? [c, a, b] : solid.sweep === "w" ? [a, b, c] : [a, c, b]);
  }
  return out;
}

/** 🗿️ The body a shape implies in a view; `twin` is the elevation shape of the same id and bone (the depth of a flat front or plan shape). */
export function bodyOf(asset: any, rest: Record<string, Frame>, view: View, shape: any, twin: any | undefined): Body | undefined {
  const r = rest[shape.bone];
  if (r === undefined || shape.kind === "member") return undefined;
  const points = pointsOf(shape);
  if (points.length === 0) return undefined;
  const solid = shape.solid;
  const length = asset.rig.bones.find((bone: any) => bone.id === shape.bone).lengthM;
  if (solid === undefined) {
    if (view === "elevation") return { local: points.map(([u, v]) => [u, v, 0] as const) };
    if (shape.followTip === true || twin === undefined) return undefined;
    const placed = pointsOf(twin).map(([u, v]) => toWorld(r, [u, v, 0]));
    const k = view === "plan" ? 1 : 0;
    const middle = (Math.min(...placed.map((p) => p[k])) + Math.max(...placed.map((p) => p[k]))) / 2;
    return { local: points.map(([a, b]) => toLocal(r, view === "plan" ? [a, middle, b] : [middle, b, a])) };
  }
  if (solid.kind === "extruded") {
    if (solid.weights?.length) return undefined;
    return { local: prismPoints(points, solid, r, length) };
  }
  if (solid.kind !== "revolved" || solid.axisBone !== shape.bone || solid.axis !== "u") return undefined;
  const offset: V3 = [solid.axisX, 0, solid.axisZ ?? 0];
  const a: V3 = [dot(offset, r.u), dot(offset, r.v), dot(offset, r.w)];
  const pz = solid.axisZ ?? 0;
  const polar = points.map(([x, y]) => ({ h: x - a[0], dv: y - a[1], dw: pz - a[2] }));
  if (solid.mirror === true || view === "plan") return { local: polar.flatMap(({ h, dv, dw }) => CIRCLE.map(([c, s]) => [a[0] + h, a[1] + Math.hypot(dv, dw) * c, a[2] + Math.hypot(dv, dw) * s] as const)) };
  const m0: V3 = [0, r.v[0], r.w[0]];
  const ml = Math.hypot(m0[1], m0[2]);
  const m: V3 = ml > 1e-6 ? [0, m0[1] / ml, m0[2] / ml] : [0, 1, 0];
  if (view === "elevation") return { local: polar.map(({ h, dv, dw }) => [a[0] + h, a[1] + (dv * m[1] + dw * m[2]) * m[1], a[2] + (dv * m[1] + dw * m[2]) * m[2]] as const) };
  return {
    world: (f) => {
      const origin = toWorld(f, a);
      const k = f.u[2];
      const l0: V3 = [-k * f.u[0], -k * f.u[1], 1 - k * f.u[2]];
      const n = Math.hypot(...l0) || 1;
      return polar.map(({ h, dv, dw }) => {
        const rho = dv * m[1] + dw * m[2];
        return [origin[0] + h * f.u[0] + (rho * l0[0]) / n, origin[1] + h * f.u[1] + (rho * l0[1]) / n, origin[2] + h * f.u[2] + (rho * l0[2]) / n] as const;
      });
    },
  };
}

/** ↔️ The extent of a body along a world axis in a frame. */
export function extent(body: Body, f: Frame, k: 0 | 1 | 2): [number, number] {
  let lo = Number.POSITIVE_INFINITY;
  let hi = Number.NEGATIVE_INFINITY;
  if ("local" in body) {
    const [a, b, c] = [f.u[k], f.v[k], f.w[k]];
    for (const p of body.local) {
      const value = p[0] * a + p[1] * b + p[2] * c;
      if (value < lo) lo = value;
      if (value > hi) hi = value;
    }
    return [lo + f.o[k], hi + f.o[k]];
  }
  for (const p of body.world(f)) {
    if (p[k] < lo) lo = p[k];
    if (p[k] > hi) hi = p[k];
  }
  return [lo, hi];
}

/** 📐️ The largest disagreement of two bodies of one bone along a world axis over the subjects, with the subject and the two ranges. */
export function worst(a: Body, b: Body, bone: string, k: 0 | 1 | 2, subjects: readonly Subject[]): { delta: number; mean: number; subject: string; a: [number, number]; b: [number, number] } {
  let best = { delta: -1, mean: 0, subject: "", a: [0, 0] as [number, number], b: [0, 0] as [number, number] };
  let sum = 0;
  for (const subject of subjects) {
    const f = subject.frames[bone];
    if (f === undefined) continue;
    const [ra, rb] = [extent(a, f, k), extent(b, f, k)];
    const delta = Math.max(Math.abs(ra[0] - rb[0]), Math.abs(ra[1] - rb[1]));
    sum += delta;
    if (delta > best.delta) best = { delta, mean: 0, subject: subject.id, a: ra, b: rb };
  }
  best.mean = sum / Math.max(1, subjects.length);
  return best;
}
