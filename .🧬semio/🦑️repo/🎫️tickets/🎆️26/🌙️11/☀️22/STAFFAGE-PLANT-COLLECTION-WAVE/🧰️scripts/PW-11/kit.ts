/** 🧰️ PW-11 desert plants: the shared chisel of the three generators (saguaro, prickly pear, yucca). It converts hand-stated world forms into bone-local rig placements, 2D shapes with `solid` placements and volume solids; every number of a plant lives in its own generator. Vector maths, quaternions and bone placements are imported from the agave generators of round 3, never copied.
 * @see ../../📋️brief.md
 * @see ../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/r3-agave-kit.ts
 * @see ../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/r3-vegetation-agave.md */
import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { D2R, add, cross, dot, len, lin, loftOf, mul, quaternion, r4, r5, solveOffsets, sub, table, unit, type Section } from "../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/r3-agave-kit.ts";
import { STAFFAGE, toLocal, type Frame, type Json, type V3 } from "../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/volumes-lib.ts";

export { D2R, add, cross, dot, len, lin, loftOf, mul, quaternion, r4, r5, sub, table, unit, toLocal, STAFFAGE };
export type { Frame, Json, V3, Section };

const { solveChain } = (await import(join(STAFFAGE, "🔨️modules/🦴️rig/🟦️.ts"))) as { solveChain: (rig: Json, offsets: Json, root: Json) => Record<string, { frame: Frame }> };

export const HERE = dirname(fileURLToPath(import.meta.url));
export const WAVE = join(HERE, "../..");
export const GENERATED = join(WAVE, "🗑️generated", "PW-11");
export const CATALOG_VEG = join(STAFFAGE, "🗂️catalog", "🌳️vegetation");

export type Scale = 50 | 100 | 200 | 500;
export const SCALES: Scale[] = [50, 100, 200, 500];
export const FEATURE: Record<Scale, number> = { 50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075 };
export const RANGES: Record<Scale, { minDenominator: number; maxDenominator: number }> = { 50: { minDenominator: 20, maxDenominator: 75 }, 100: { minDenominator: 75, maxDenominator: 150 }, 200: { minDenominator: 150, maxDenominator: 350 }, 500: { minDenominator: 350, maxDenominator: 1000 } };
export const PENS: Record<Scale, Json> = {
  50: { body: 0.25, detail: 0.18, texture: 0.13, glazing: null, ground: 0.35 },
  100: { body: 0.2, detail: 0.13, texture: 0.1, glazing: null, ground: 0.3 },
  200: { body: 0.18, detail: 0.13, texture: null, glazing: null, ground: 0.25 },
  500: { body: 0.15, detail: null, texture: null, glazing: null, ground: 0.25 },
};
export const EXPORTS = ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"];
export const LIMB_ROTATION: [number, number, number, number] = [0.5, 0.5, 0.5, 0.5];

/** 🌐️ English and German text, English first, no default language. */
export const T = (en: string, de: string) => ({ en, de });

/** 🧭️ Unit direction of azimuth `az` (degrees, counter-clockwise seen from above: 0 ahead (+x), 90 to the asset's left (+z)) and elevation `el` above the horizon. */
export const dir = (az: number, el: number): V3 => [Math.cos(el * D2R) * Math.cos(az * D2R), Math.sin(el * D2R), Math.cos(el * D2R) * Math.sin(az * D2R)];

/** ⬆️ The upward normal of {@link dir} in its vertical plane. */
export const upNormal = (az: number, el: number): V3 => [-Math.sin(el * D2R) * Math.cos(az * D2R), Math.cos(el * D2R), -Math.sin(el * D2R) * Math.sin(az * D2R)];

/** 🔄️ Rodrigues rotation of `p` about the unit axis `k` by `deg`. */
export function rotate(p: V3, k: V3, deg: number): V3 {
  const c = Math.cos(deg * D2R), s = Math.sin(deg * D2R);
  return add(add(mul(p, c), mul(cross(k, p), s)), mul(k, dot(k, p) * (1 - c)));
}

/** 🦴️ A rig authored in world space: every bone is stated by its world origin, its direction and its normal in the default pose (which is the rest pose of these plants), and placed in its parent by the product convention (`solveOffsets` of the agave kit). */
export class RigBuilder {
  readonly bones: Json[] = [];
  readonly frames: Record<string, Frame> = {};
  constructor(root: Json) {
    this.bones.push({ ...root, parent: null, x: 0, y: 0, angleDeg: 0 });
    this.frames[root.id] = { o: [0, 0, 0], u: [1, 0, 0], v: [0, 1, 0], w: [0, 0, 1] };
  }
  /** ➕️ Adds a bone at world origin `o` along `u` with its normal as near `vHint` as the direction allows. */
  add(id: string, parent: string, title: Json, o: V3, u: V3, vHint: V3, lengthM: number, joint: Json, extra: Json = {}): Frame {
    const P = this.frames[parent];
    if (P === undefined) throw new Error(`unknown parent ${parent} of ${id}`);
    const uu = unit(u);
    const v = unit(sub(vHint, mul(uu, dot(vHint, uu))));
    const w = cross(uu, v);
    const d = sub(o, P.o);
    const rot = solveOffsets(P, uu, v, 0);
    const z = r5(dot(d, P.w));
    this.bones.push({
      id, parent, title, x: r5(dot(d, P.u)), y: r5(dot(d, P.v)), ...(Math.abs(z) > 1e-6 ? { z } : {}), angleDeg: rot.angleDeg,
      ...(Math.abs(rot.abductionDeg) > 1e-4 ? { abductionDeg: rot.abductionDeg } : {}), ...(Math.abs(rot.twistDeg) > 1e-4 ? { twistDeg: rot.twistDeg } : {}),
      lengthM: r5(lengthM), joint, ...extra,
    });
    const f: Frame = { o, u: uu, v, w };
    this.frames[id] = f;
    return f;
  }
  rig(): Json {
    return { kind: "botanical", root: this.bones[0].id, bones: this.bones };
  }
  /** 🧾️ Largest difference between the product's solved rest frames and the stated frames. */
  verify(): number {
    const solved = solveChain(this.rig(), {}, { x: 0, y: 0, angleDeg: 0 });
    let worst = 0;
    for (const b of this.bones) {
      const a = this.frames[b.id]!, s = solved[b.id]!.frame;
      for (const k of ["o", "u", "v", "w"] as const) for (let i = 0; i < 3; i++) worst = Math.max(worst, Math.abs(a[k][i]! - s[k][i]!));
    }
    return worst;
  }
}

/** 🌍️ The world frames of a rig in a pose (product solver). */
export const poseFrames = (rig: Json, offsets: Json): Record<string, Frame> => Object.fromEntries(Object.entries(solveChain(rig, offsets, { x: 0, y: 0, angleDeg: 0 })).map(([k, v]) => [k, v.frame]));

/** 🗺️ World point of a bone-local point. */
export const world = (f: Frame, p: V3): V3 => add(add(add(f.o, mul(f.u, p[0])), mul(f.v, p[1])), mul(f.w, p[2]));

// 2D shapes ----------------------------------------------------------------------------------------------------

/** ✏️ A shape of one scale with the world point that orders it per view (painter's order by nearness to the viewer). */
export type Drawn = { shape: Json; at: V3; views?: ("elevation" | "front" | "plan")[]; rank?: number };

/** 🏺️ A revolved part about the bone axis: closed half profile [u, r] from the axis, mirrored (agave caudex and spear). */
export function revolvedShape(id: string, bone: string, profile: readonly (readonly [number, number])[], fields: Json): Json {
  const first = profile[0]!, last = profile[profile.length - 1]!;
  const pts = [...(first[1] > 0 ? [[r5(first[0]), 0]] : []), ...profile.map(([u, r]) => [r5(u), r5(r)]), ...(last[1] > 0 ? [[r5(last[0]), 0]] : [])];
  return { id, bone, role: "body", filled: true, ...fields, kind: "poly", points: pts, closed: true, solid: { kind: "revolved", axisBone: bone, axis: "u", axisX: 0, mirror: true } };
}

/** 🍃️ A flat part in the (u, w) plane of its bone swept along the normal v from `z0` to `z1` (at the bone tip `zTip`): a pad, a leaf blade. */
export function plateShape(id: string, bone: string, ring: readonly (readonly [number, number])[], span: [number, number], tip: [number, number] | undefined, fields: Json, closed = true): Json {
  return { id, bone, role: "body", filled: closed, ...fields, kind: "poly", points: ring.map(([u, w]) => [r5(u), r5(w)]), closed, solid: { kind: "extruded", sweep: "v", z0: r5(span[0]), z1: r5(span[1]), ...(tip ? { zTip: [r5(tip[0]), r5(tip[1])] } : {}) } };
}

/** 🍪️ The solid of {@link plateShape} without a tip span: the same ring in the (u, w) plane of the bone pushed along the normal v from `span[0]` to `span[1]`, so the drawing is the true view of this solid from every side (a lens or a loft would leave the slots between neighbouring plates narrower in the drawing than in the body). */
export function plateSolid(id: string, bone: string, ring: readonly (readonly [number, number])[], span: [number, number], fields: Json): Json {
  return { id, kind: "extrude", bone, ...fields, plane: "zx", profile: ring.map(([u, w]) => [r5(w), r5(u)]), from: r5(span[0]), to: r5(span[1]) };
}

/** 🎨️ The three view lists of one scale: the same placed shapes in every view (their `solid` decides the projection), each list in painter's order from far to near. */
export function views(drawn: readonly Drawn[]): Record<"elevation" | "front" | "plan", Json[]> {
  const toward: Record<string, V3> = { elevation: [0, 0, -1], front: [1, 0, 0], plan: [0, 1, 0] };
  const out: Json = {};
  for (const view of ["elevation", "front", "plan"] as const) {
    const list = drawn.filter((d) => d.views === undefined || d.views.includes(view)).map((d, i) => ({ d, i, n: dot(d.at, toward[view]!) + (d.rank ?? 0) * 1e-4 }));
    list.sort((a, b) => a.n - b.n || a.i - b.i);
    out[view] = list.map((x, k) => ({ ...x.d.shape, z: r4(2 + k / 1000) }));
  }
  return out;
}

// Volume solids -------------------------------------------------------------------------------------------------

/** 🏺️ A revolved solid about the bone axis u from its half profile [u, r]. */
export const revolveSolid = (id: string, bone: string, profile: readonly (readonly [number, number])[], fields: Json): Json => ({ id, kind: "revolve", bone, ...fields, axis: [1, 0, 0], origin: [0, 0, 0], profile: profile.map(([u, r]) => [r5(u), r5(Math.max(0, r))]) });

/** 🪵️ A loft along the bone axis u: sections at bone-local centres with half depth `rx` (along v) and half width `ry` (along w), limb rotation on every section. */
export function loftU(id: string, bone: string, sections: readonly { at: V3; rx: number; ry: number; n?: number }[], fields: Json): Json {
  return { id, kind: "loft", bone, ...fields, sections: sections.map((s) => ({ at: s.at.map(r5), rx: r5(Math.max(s.rx, 2e-4)), ry: r5(Math.max(s.ry, 2e-4)), ...(s.n !== undefined && s.n !== 2 ? { exponent: s.n } : {}), rotation: LIMB_ROTATION })), capStart: true, capEnd: true };
}

/** ⭐️ A ribbed (fluted) column: the star profile of `ribs` crests at radius `r` and valleys `depth` deeper in the (v, w) plane, extruded along u from `from` to `to` and tapered about the axis by `taper` [scale at from, scale at to]; `phaseDeg` turns the crests, `points` is the number of points per rib (crest, flanks, valley). */
export function ribbedColumn(id: string, bone: string, ribs: number, r: number, depth: number, from: number, to: number, taper: [number, number], phaseDeg: number, points: number, fields: Json): Json {
  return { id, kind: "extrude", bone, ...fields, plane: "yz", profile: ribbedProfile(ribs, r, depth, phaseDeg, points), from: r5(from), to: r5(to), ...(Math.abs(taper[0] - taper[1]) > 1e-4 ? { taperTo: taper.map(r5) } : {}) };
}

/** ⭐️ The star profile of a ribbed column in the (v, w) plane: `ribs` crests at radius `r`, valleys `depth` deeper, `points` points per rib, crests turned by `phaseDeg`. */
export function ribbedProfile(ribs: number, r: number, depth: number, phaseDeg: number, points: number): [number, number][] {
  const profile: [number, number][] = [];
  const n = ribs * points;
  for (let i = 0; i < n; i++) {
    const a = phaseDeg * D2R + (2 * Math.PI * i) / n;
    const phase = (i % points) / points;
    const rr = r - depth * (0.5 - 0.5 * Math.cos(2 * Math.PI * phase)) ** 1.4;
    profile.push([r5(rr * Math.cos(a)), r5(rr * Math.sin(a))]);
  }
  return profile;
}

/** ⭐️ The drawing of a ribbed column without taper: the star profile of {@link ribbedColumn} swept along the bone from `from` to `to`, so every view is the true view of that solid, valleys included (seen end-on a fluted stem is a star, not a disc). */
export function ribbedShape(id: string, bone: string, ribs: number, r: number, depth: number, from: number, to: number, phaseDeg: number, points: number, fields: Json): Json {
  return { id, bone, role: "body", filled: true, ...fields, kind: "poly", points: ribbedProfile(ribs, r, depth, phaseDeg, points), closed: true, solid: { kind: "extruded", sweep: "u", z0: r5(from), z1: r5(to) } };
}

/** 🥚️ A bone-local ellipsoid. */
export const ellipsoidSolid = (id: string, bone: string, centre: V3, radii: V3, fields: Json, rotation?: [number, number, number, number]): Json => ({ id, kind: "ellipsoid", bone, ...fields, radii: radii.map(r5), transform: { translation: centre.map(r5), ...(rotation ? { rotation } : {}) } });

/** 🧮️ Outer area of a revolved profile [u, r] (lateral surface of the frustums). */
export function revolvedArea(profile: readonly (readonly [number, number])[]): number {
  let a = 0;
  for (let i = 1; i < profile.length; i++) {
    const [u0, r0] = profile[i - 1]!, [u1, r1] = profile[i]!;
    a += Math.PI * (r0 + r1) * Math.hypot(u1 - u0, r1 - r0);
  }
  return a;
}

/** 📏️ Perimeter of the ribbed star of {@link ribbedColumn} against the circle of its crests. */
export function ribbedPerimeterRatio(ribs: number, r: number, depth: number, points = 24): number {
  let p = 0, prev: [number, number] | null = null;
  const n = ribs * points;
  for (let i = 0; i <= n; i++) {
    const a = (2 * Math.PI * i) / n, phase = (i % points) / points;
    const rr = r - depth * (0.5 - 0.5 * Math.cos(2 * Math.PI * phase)) ** 1.4;
    const q: [number, number] = [rr * Math.cos(a), rr * Math.sin(a)];
    if (prev) p += Math.hypot(q[0] - prev[0], q[1] - prev[1]);
    prev = q;
  }
  return p / (2 * Math.PI * r);
}

/** 💾️ Writes JSON with a trailing newline, creating the folder. */
export function writeJson(path: string, value: unknown, pretty = false): void {
  mkdirSync(dirname(path), { recursive: true });
  writeFileSync(path, `${JSON.stringify(value, null, pretty ? 2 : undefined)}\n`);
}

/** 🧾️ The asset document around an asset. */
export const documentOf = (asset: Json): Json => ({ $schema: "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", version: "semio.staffage/v2", asset });

/** 📥️ The manifest row of an evaluated asset, the shape of the existing `entries` rows. */
export function manifestRow(asset: Json, distinction: Json): Json {
  return {
    id: asset.id, category: asset.category, title: asset.title, heightM: asset.dimensions.heightM, widthM: asset.dimensions.widthM, powerW: 0,
    poses: asset.poses.filter((p: Json) => !p.key).map((p: Json) => p.id), clips: asset.clips.map((c: Json) => c.id),
    representations: Object.fromEntries(Object.entries(asset.representations).map(([k, v]) => [k, (v as Json).status])), symbols: (asset.symbols ?? []).map((s: Json) => s.id),
    analytics: ["thermal", "transport", "structural"], exports: asset.exports, distinction, completion: asset.completion,
  };
}

/** 🌬️ Drag force F = ½ ρ cd A v² (EN 1991-1-4, ρ = 1.25 kg/m³). */
export const drag = (cd: number, areaM2: number, vMS: number): number => 0.5 * 1.25 * cd * areaM2 * vMS * vMS;

/** 🎚️ Clip keys of a periodic sway: `n` keys over `duration`, first equal to last, angle offsets of each bone `amp · sin(2π t / T + phase)` laid over a base pose. */
export function swayKeys(pose: string, duration: number, n: number, bones: readonly { id: string; base: Json; amp: number; phase: number; field?: "angleDeg" }[]): Json[] {
  const keys: Json[] = [];
  for (let i = 0; i <= n; i++) {
    const t = (duration * i) / n;
    const offsets: Json = {};
    for (const b of bones) {
      const s = i === n ? Math.sin(b.phase) : Math.sin((2 * Math.PI * i) / n + b.phase);
      offsets[b.id] = { ...b.base, angleDeg: r4((b.base.angleDeg ?? 0) + b.amp * s) };
    }
    keys.push({ t: r4(t), pose, ease: "smooth", bones: offsets });
  }
  return keys;
}

const { unionPolygons } = (await import(join(STAFFAGE, "🔨️modules/🔺️polygon/🟦️.ts"))) as { unionPolygons: (input: readonly (readonly { x: number; y: number }[])[]) => { x: number; y: number }[][] };

/** 🫧️ The union outline of round-ended bars [x0, y0, x1, y1, r0, r1] in one drawing plane (a symbol's true-extent silhouette), simplified to points at least `step` apart. */
export function barOutline(bars: readonly [number, number, number, number, number, number][], step: number): [number, number][][] {
  const polys = bars.map(([x0, y0, x1, y1, r0, r1]) => {
    const pts: { x: number; y: number }[] = [];
    const a = Math.atan2(y1 - y0, x1 - x0);
    for (let i = 0; i <= 12; i++) { const t = a + Math.PI / 2 + (Math.PI * i) / 12; pts.push({ x: x0 + r0 * Math.cos(t), y: y0 + r0 * Math.sin(t) }); }
    for (let i = 0; i <= 12; i++) { const t = a - Math.PI / 2 + (Math.PI * i) / 12; pts.push({ x: x1 + r1 * Math.cos(t), y: y1 + r1 * Math.sin(t) }); }
    return pts;
  });
  return unionPolygons(polys).map((ring) => {
    const out: [number, number][] = [];
    for (const p of ring) if (out.length === 0 || Math.hypot(p.x - out[out.length - 1]![0], p.y - out[out.length - 1]![1]) >= step) out.push([r4(p.x), r4(p.y)]);
    return out;
  }).filter((r) => r.length >= 3);
}

/** 🌬️ Pose offsets that turn the listed bones (parent first) by `deg` about the world axis `axis` relative to where their parents put them, on top of `base` offsets: an exact lean of a limb chain, solved with the product rig. */
export function leanOffsets(rig: Json, base: Json, leans: readonly { id: string; deg: number }[], axis: V3 = [0, 0, 1]): Json {
  const offsets: Json = JSON.parse(JSON.stringify(base));
  for (const { id, deg } of leans) {
    const rest = { ...offsets };
    delete rest[id];
    const P = poseFrames(rig, rest)[id]!;
    const now = offsets[id] === undefined ? P : poseFrames(rig, offsets)[id]!;
    const o = solveOffsets(P, rotate(now.u, axis, deg), rotate(now.v, axis, deg), 0);
    offsets[id] = { angleDeg: o.angleDeg, ...(Math.abs(o.abductionDeg) > 1e-4 ? { abductionDeg: o.abductionDeg } : {}), ...(Math.abs(o.twistDeg) > 1e-4 ? { twistDeg: o.twistDeg } : {}) };
  }
  return offsets;
}

export { lin as mix };
