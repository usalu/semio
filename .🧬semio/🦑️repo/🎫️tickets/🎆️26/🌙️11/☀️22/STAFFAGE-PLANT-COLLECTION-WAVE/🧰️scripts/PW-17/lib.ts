/** 🪴 PW-17 potted-plant kit: vectors and bone frames, the rest placement of a bone from a wanted frame (checked against the product's own `restFrames`), the 2D shapes and 3D solids of pots, stalks and leaf plates (one definition for drawing and model), the merged coarse-scale outlines (projection of the scale's own model, closed and opened on a raster) and the document writer.
 * Every number of a plant is stated by hand in its species file; this kit only formats, converts frames and projects.
 * @see ../../📋️brief.md
 * @see ../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/r3-tools.md
 * @see ../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/README.md */
import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { restFrames } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts";
import { unionPolygons } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🔺️polygon/🟦️.ts";
import { meshesIn, rigAt } from "../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/r3-lib.ts";
import { contours, projectMask } from "../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/r3-project2d.ts";
import { area as ringArea, pointInRing, simplifyRing } from "../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/volumes-lib.ts";

export type Json = any;
export type V3 = [number, number, number];
export type P = [number, number];
export type Text = { en: string; de: string };
export type Scale = 50 | 100 | 200 | 500;
export type View = "elevation" | "front" | "plan";
export const SCALES: readonly Scale[] = [50, 100, 200, 500];
export const VIEWS: readonly View[] = ["elevation", "front", "plan"];
export const FEATURE: Record<Scale, number> = { 50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075 };
export const BAND: Record<Scale, [number, number]> = { 50: [20, 75], 100: [75, 150], 200: [150, 350], 500: [350, 1000] };
export const REPO = join(import.meta.dir, "../../../../../../../../..");
export const STAFFAGE = join(REPO, "🎓️teaching/🏛️architecture/🧍️staffage");
export const WAVE = join(import.meta.dir, "../..");
export const GENERATED = join(WAVE, "🗑️generated/PW-17");
export const EXPORTS = ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"];
export const D2R = Math.PI / 180;

export const T = (en: string, de: string): Text => ({ en, de });
export const r5 = (x: number): number => Math.round(x * 1e5) / 1e5;
export const r4 = (x: number): number => Math.round(x * 1e4) / 1e4;
export const r3 = (x: number): number => Math.round(x * 1e3) / 1e3;
export const add = (a: V3, b: V3): V3 => [a[0] + b[0], a[1] + b[1], a[2] + b[2]];
export const sub = (a: V3, b: V3): V3 => [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
export const mul = (a: V3, k: number): V3 => [a[0] * k, a[1] * k, a[2] * k];
export const lin = (a: V3, s: number, b: V3, t: number): V3 => [a[0] * s + b[0] * t, a[1] * s + b[1] * t, a[2] * s + b[2] * t];
export const dot = (a: V3, b: V3): number => a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
export const cross = (a: V3, b: V3): V3 => [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
export const len = (a: V3): number => Math.hypot(a[0], a[1], a[2]);
export const unit = (a: V3): V3 => mul(a, 1 / (len(a) || 1));
export const UP: V3 = [0, 1, 0];

/** 🧭️ The horizontal direction of an azimuth in degrees: 0 ahead (+x), 90 to the left of the asset (+z). */
export const azimuth = (deg: number): V3 => [Math.cos(deg * D2R), 0, Math.sin(deg * D2R)];

/** 📉️ Linear interpolation in a table of [x, y] rows (clamped at the ends). */
export function table(rows: readonly (readonly [number, number])[], x: number): number {
  if (x <= rows[0]![0]) return rows[0]![1];
  for (let i = 1; i < rows.length; i++) if (x <= rows[i]![0]) {
    const [x0, y0] = rows[i - 1]!, [x1, y1] = rows[i]!;
    return y0 + ((y1 - y0) * (x - x0)) / (x1 - x0);
  }
  return rows[rows.length - 1]![1];
}

/** 🎲️ A small deterministic pseudo-random sequence (for phases of the sway keys only, never for geometry). */
export function phase(i: number, salt = 0): number {
  const s = Math.sin((i + 1) * 12.9898 + salt * 78.233) * 43758.5453;
  return s - Math.floor(s);
}

export type Frame = { o: V3; u: V3; v: V3; w: V3 };
export const WORLD: Frame = { o: [0, 0, 0], u: [1, 0, 0], v: [0, 1, 0], w: [0, 0, 1] };

/** 🧱️ An orthonormal bone frame from its origin, its axis `u` and a hint for its normal `v` (made orthogonal to `u`); `w = u × v` as in the product's rig. */
export function frameOf(o: V3, u: V3, vHint: V3): Frame {
  const uu = unit(u);
  const vv = unit(sub(vHint, mul(uu, dot(vHint, uu))));
  return { o, u: uu, v: vv, w: cross(uu, vv) };
}

export const toLocal = (f: Frame, p: V3): V3 => { const d = sub(p, f.o); return [dot(d, f.u), dot(d, f.v), dot(d, f.w)]; };
export const toWorld = (f: Frame, l: readonly number[]): V3 => [f.o[0] + l[0]! * f.u[0] + l[1]! * f.v[0] + (l[2] ?? 0) * f.w[0], f.o[1] + l[0]! * f.u[1] + l[1]! * f.v[1] + (l[2] ?? 0) * f.w[1], f.o[2] + l[0]! * f.u[2] + l[1]! * f.v[2] + (l[2] ?? 0) * f.w[2]];

/** 🔩️ The rest placement of a child frame in its parent: offset (x, y, z) along the parent's axes and `angleDeg`, `abductionDeg`, `twistDeg` in the product's order (about w, then the turned v, then the turned u). */
export function placement(parent: Frame, child: Frame): { x: number; y: number; z: number; angleDeg: number; abductionDeg: number; twistDeg: number } {
  const d = sub(child.o, parent.o);
  const ul: V3 = [dot(child.u, parent.u), dot(child.u, parent.v), dot(child.u, parent.w)];
  const vl: V3 = [dot(child.v, parent.u), dot(child.v, parent.v), dot(child.v, parent.w)];
  const b = Math.asin(Math.max(-1, Math.min(1, ul[2])));
  const a = Math.atan2(ul[1], ul[0]);
  const v1: V3 = [-Math.sin(a), Math.cos(a), 0];
  const w2: V3 = [-Math.sin(b) * Math.cos(a), -Math.sin(b) * Math.sin(a), Math.cos(b)];
  const c = Math.atan2(-dot(vl, w2), dot(vl, v1));
  const deg = (x: number) => { const v = r4(x / D2R); return Math.abs(v) < 1e-4 ? 0 : v; };
  return { x: r5(dot(d, parent.u)), y: r5(dot(d, parent.v)), z: r5(dot(d, parent.w)), angleDeg: deg(a), abductionDeg: deg(b), twistDeg: deg(c) };
}

export type BoneSpec = { id: string; parent: string | null; title: Text; f: Frame; length: number; joint: Json; mass?: Json; section?: Json };

/** 🌳️ A rig assembled from wanted frames; `frames` keeps the wanted rest frame of every bone for the solids, shapes and anchors. */
export class RigBuilder {
  bones: Json[] = [];
  frames: Record<string, Frame> = {};
  root = "";
  add(spec: BoneSpec): Frame {
    const parent = spec.parent === null ? WORLD : this.frames[spec.parent]!;
    if (spec.parent === null) this.root = spec.id;
    const p = placement(parent, spec.f);
    const bone: Json = { id: spec.id, parent: spec.parent, title: spec.title, x: p.x, y: p.y, ...(Math.abs(p.z) > 0 ? { z: p.z } : {}), angleDeg: p.angleDeg, ...(p.abductionDeg !== 0 ? { abductionDeg: p.abductionDeg } : {}), ...(p.twistDeg !== 0 ? { twistDeg: p.twistDeg } : {}), lengthM: r4(spec.length), joint: spec.joint };
    if (spec.mass) bone.mass = spec.mass;
    if (spec.section) bone.section = spec.section;
    this.bones.push(bone);
    this.frames[spec.id] = spec.f;
    return spec.f;
  }
  rig(): Json { return { kind: "botanical", root: this.root, bones: this.bones }; }

  /** ✅️ The largest difference between the wanted frames and the product's rest frames of the written rig (rounding of the angles to 1e-4 degrees). */
  verify(): number {
    const solved = restFrames(this.rig() as never) as Record<string, { frame: { o: V3; u: V3; v: V3; w: V3 } }>;
    let worst = 0;
    for (const [id, f] of Object.entries(this.frames)) {
      const s = solved[id]!.frame;
      for (const k of ["o", "u", "v", "w"] as const) worst = Math.max(worst, len(sub(s[k] as V3, f[k])));
    }
    return worst;
  }
}

export const revolute = (minDeg: number, maxDeg: number): Json => ({ type: "revolute", minDeg, maxDeg });
export const FIXED: Json = { type: "fixed" };
export const massOf = (kg: number, comX: number, comY: number, distribution: "point" | "uniform", source: string, reference: string): Json => ({ kg: r4(kg), comX: r4(comX), comY: r4(comY), distribution, source, reference });

/** 🟢️ A solid circular section for stress: area, second moment and extreme fibre of a stalk of diameter `d`. */
export const roundSection = (d: number, material: string, title: Text): Json => ({ areaM2: Number((Math.PI * d * d / 4).toPrecision(5)), secondMomentM4: Number((Math.PI * d ** 4 / 64).toPrecision(5)), extremeFibreM: r5(d / 2), material, title });

export type Meta = { role?: string; tone: number; region?: string; regions?: string[]; material?: string; filled?: boolean; penMm?: number; hatch?: string };
const metaOf = (m: Meta): Json => ({ role: m.role ?? "body", filled: m.filled ?? true, tone: r3(m.tone), ...(m.region ? { region: m.region } : {}), ...(m.regions ? { regions: m.regions } : {}), ...(m.material ? { material: m.material } : {}), ...(m.penMm ? { penMm: m.penMm } : {}), ...(m.hatch ? { hatch: m.hatch } : {}) });

/** 🏺️ A turned part about the bone axis u, drawn from its half profile [u, r] in every view (`revolved`, mirrored), with its 3D twin {@link revolveSolid}. */
export function revolvedShape(id: string, bone: string, profile: readonly P[], m: Meta, lateralScale?: number): Json {
  const first = profile[0]!, last = profile[profile.length - 1]!;
  const points = [...(first[1] > 0 ? [[r5(first[0]), 0]] : []), ...profile.map(([u, r]) => [r5(u), r5(r)]), ...(last[1] > 0 ? [[r5(last[0]), 0]] : [])];
  return { id, bone, ...metaOf(m), z: 0, kind: "poly", points, closed: true, solid: { kind: "revolved", axisBone: bone, axis: "u", axisX: 0, mirror: true, ...(lateralScale !== undefined ? { lateralScale: r4(lateralScale) } : {}) } };
}

/** 🍃️ A plate in the (u, w) plane of its bone between v0 and v1 (a leaf blade, a sheet), outline and holes as [u, w] rings, drawn by its `extruded` solid (sweep v) in every view, with its 3D twin {@link plateSolid}. */
export function plateShape(id: string, bone: string, outer: readonly P[], holes: readonly (readonly P[])[], v0: number, v1: number, m: Meta): Json {
  const ring = (r: readonly P[]) => r.map(([u, w]) => [r5(u), r5(w)]);
  const solid = { kind: "extruded", sweep: "v", z0: r5(v0), z1: r5(v1) };
  if (holes.length === 0) return { id, bone, ...metaOf(m), z: 0, kind: "poly", points: ring(outer), closed: true, solid };
  const d = [outer, ...holes].map((r) => r.map(([u, w], i) => `${i === 0 ? "M" : "L"}${r5(u)} ${r5(w)}`).join(" ") + " Z").join(" ");
  return { id, bone, ...metaOf(m), z: 0, kind: "path", d, solid };
}

/** ✒️ An open line on the face of a plate (vein, band), placed like the plate. */
export function plateStroke(id: string, bone: string, points: readonly P[], v: number, m: Meta): Json {
  return { id, bone, ...metaOf({ ...m, filled: false }), z: 0, kind: "poly", points: points.map(([u, w]) => [r5(u), r5(w)]), closed: false, solid: { kind: "extruded", sweep: "v", z0: r5(v - 0.0002), z1: r5(v + 0.0002) } };
}

/** 🖊️ A plain shape of one view (merged coarse masses): rings in the coordinates of that view's shapes, one ring a `poly`, several rings a `path` (even-odd). */
export function plainShape(id: string, bone: string, rings: readonly (readonly P[])[], m: Meta): Json {
  if (rings.length === 1) return { id, bone, ...metaOf(m), z: 0, kind: "poly", points: rings[0]!.map(([x, y]) => [r5(x), r5(y)]), closed: true };
  const d = rings.map((r) => r.map(([x, y], i) => `${i === 0 ? "M" : "L"}${r5(x)} ${r5(y)}`).join(" ") + " Z").join(" ");
  return { id, bone, ...metaOf(m), z: 0, kind: "path", d };
}

export type SolidMeta = { material: string; tone: number; role?: string; slender?: boolean; sheet?: boolean; tags?: string[] };
const solidMeta = (m: SolidMeta): Json => ({ material: m.material, role: m.role ?? "body", tone: r3(m.tone), ...(m.slender ? { slender: true } : {}), ...(m.sheet ? { sheet: true } : {}), ...(m.tags ? { tags: m.tags } : {}) });

/** 🏺️ The 3D twin of {@link revolvedShape}: the half profile [u, r] revolved about the bone axis u. */
export function revolveSolid(id: string, bone: string, profile: readonly P[], m: SolidMeta, lateralScale?: number): Json {
  return { id, kind: "revolve", bone, ...solidMeta(m), axis: [1, 0, 0], origin: [0, 0, 0], profile: profile.filter(([, r]) => r > 0).map(([u, r]) => [r5(u), r5(r)]), ...(lateralScale !== undefined ? { lateralScale: r4(lateralScale) } : {}) };
}

/** 🍃️ The 3D twin of {@link plateShape}: the [u, w] outline (and holes) on the `zx` plane, pushed along v from v0 to v1. */
export function plateSolid(id: string, bone: string, outer: readonly P[], holes: readonly (readonly P[])[], v0: number, v1: number, m: SolidMeta): Json {
  const ring = (r: readonly P[]) => r.map(([u, w]) => [r5(w), r5(u)]);
  return { id, kind: "extrude", bone, ...solidMeta(m), plane: "zx", profile: ring(outer), ...(holes.length ? { holes: holes.map(ring) } : {}), from: r5(v0), to: r5(v1) };
}

export const ellipsoidSolid = (id: string, bone: string, centre: V3, radii: V3, m: SolidMeta): Json => ({ id, kind: "ellipsoid", bone, ...solidMeta(m), radii: radii.map(r5), transform: { translation: centre.map(r5) } });

/** 🌀️ A closed ring as a polygon of `n` points: an ellipse of half axes a (u) and b (w) about (cu, cw). */
export const ellipseRing = (cu: number, cw: number, a: number, b: number, n = 14, rot = 0): P[] => Array.from({ length: n }, (_, i) => { const t = (2 * Math.PI * i) / n; const [x, y] = [a * Math.cos(t), b * Math.sin(t)]; return [cu + x * Math.cos(rot) - y * Math.sin(rot), cw + x * Math.sin(rot) + y * Math.cos(rot)] as P; });

/** 🗂️ Shapes of one representation with the world point that orders them per view (nearer to the viewer drawn later, with a higher z). */
export type Placed = { shape: Json; at: V3; rank?: number; view?: View };
const TOWARD: Record<View, V3> = { elevation: [0, 0, -1], front: [1, 0, 0], plan: [0, 1, 0] };

/** 🔢️ The three view lists of shapes with z set by nearness of their world point (rank breaks ties within one group). */
export function ordered(items: readonly Placed[], perView?: Partial<Record<View, readonly Placed[]>>): Record<View, Json[]> {
  const out = {} as Record<View, Json[]>;
  for (const view of VIEWS) {
    const list = (perView?.[view] ?? items).filter((p) => p.view === undefined || p.view === view);
    const sorted = [...list].sort((a, b) => dot(a.at, TOWARD[view]) + (a.rank ?? 0) * 1e-6 - (dot(b.at, TOWARD[view]) + (b.rank ?? 0) * 1e-6));
    out[view] = sorted.map((p, i) => ({ ...p.shape, z: r4(1 + i / 1000) }));
  }
  return out;
}

/** 🧽️ Squared Euclidean distance (in cells) of every cell to the nearest cell whose value is `target` (Felzenszwalb and Huttenlocher, separable). */
function edt(cells: Uint8Array, width: number, height: number, target: 0 | 1): Float64Array {
  const INF = 1e20;
  const f = new Float64Array(Math.max(width, height));
  const d = new Float64Array(Math.max(width, height));
  const v = new Int32Array(Math.max(width, height));
  const z = new Float64Array(Math.max(width, height) + 1);
  const grid = new Float64Array(width * height);
  for (let i = 0; i < cells.length; i++) grid[i] = cells[i] === target ? 0 : INF;
  const pass = (n: number) => {
    let k = 0;
    v[0] = 0; z[0] = -INF; z[1] = INF;
    for (let q = 1; q < n; q++) {
      let s = ((f[q]! + q * q) - (f[v[k]!]! + v[k]! * v[k]!)) / (2 * q - 2 * v[k]!);
      while (s <= z[k]!) { k--; s = ((f[q]! + q * q) - (f[v[k]!]! + v[k]! * v[k]!)) / (2 * q - 2 * v[k]!); }
      k++; v[k] = q; z[k] = s; z[k + 1] = INF;
    }
    k = 0;
    for (let q = 0; q < n; q++) { while (z[k + 1]! < q) k++; d[q] = (q - v[k]!) ** 2 + f[v[k]!]!; }
  };
  for (let x = 0; x < width; x++) { for (let y = 0; y < height; y++) f[y] = grid[y * width + x]!; pass(height); for (let y = 0; y < height; y++) grid[y * width + x] = d[y]!; }
  for (let y = 0; y < height; y++) { for (let x = 0; x < width; x++) f[x] = grid[y * width + x]!; pass(width); for (let x = 0; x < width; x++) grid[y * width + x] = d[x]!; }
  return grid;
}

/** 🫧️ Morphological dilation (`grow` > 0) or erosion (`grow` < 0) by a disc of |grow| cells. */
export function morph(cells: Uint8Array, width: number, height: number, grow: number): Uint8Array {
  if (grow === 0) return cells;
  const r2 = grow * grow;
  const out = new Uint8Array(cells.length);
  if (grow > 0) { const d = edt(cells, width, height, 1); for (let i = 0; i < cells.length; i++) out[i] = d[i]! <= r2 ? 1 : 0; }
  else { const d = edt(cells, width, height, 0); for (let i = 0; i < cells.length; i++) out[i] = d[i]! > r2 ? 1 : 0; }
  return out;
}

export type Ring2 = { x: number; y: number }[];
export type Outline = { outer: P[]; holes: P[][] };

/** 🫥️ The outline of the union of world meshes in one view: projected on a raster of `cell`, closed by `closeM` (gaps narrower than twice it disappear), opened by `openM` (parts thinner than twice it go), holes under `minHoleM2` filled, simplified within `toleranceM`; rings in that view's paper coordinates. */
export function outlineOf(view: View, meshes: Json[], cell: number, closeM: number, openM: number, toleranceM: number, minHoleM2: number, minPartM2: number): Outline[] {
  const margin = Math.ceil((closeM + openM) / cell) + 4;
  const mask = projectMask(view as never, meshes as never, cell, margin);
  if (mask === null) return [];
  const { width, height } = mask;
  const c = Math.round(closeM / cell), o = Math.round(openM / cell);
  let cells = morph(morph(mask.cells, width, height, c), width, height, -c);
  if (o > 0) cells = morph(morph(cells, width, height, -o), width, height, o);
  const rings = (contours(cells, width, height, mask.box, cell) as Ring2[]).map((r) => simplifyRing(r as never, toleranceM) as unknown as Ring2).filter((r) => r.length >= 3);
  const outers = rings.filter((r) => ringArea(r as never) > minPartM2);
  const holes = rings.filter((r) => ringArea(r as never) < -minHoleM2);
  const groups: Outline[] = outers.sort((a, b) => ringArea(b as never) - ringArea(a as never)).map((r) => ({ outer: r.map((p) => [p.x, p.y] as P), holes: [] }));
  for (const h of holes) {
    const host = groups.find((g) => pointInRing(h[0] as never, g.outer.map(([x, y]) => ({ x, y })) as never));
    if (host) host.holes.push(h.map((p) => [p.x, p.y] as P));
  }
  return groups;
}

/** 🧊️ The world meshes of solids in the default pose at the tessellation quality of the scale. */
export function worldMeshes(asset: Json, scale: Scale, solids: readonly Json[]): Json[] {
  const rig = rigAt(asset, scale, { kind: "pose", pose: asset.presentation.defaultPose });
  return meshesIn(asset, rig.frames, solids, scale);
}

/** 🧊️ The world meshes of solids carried by the given bone frames (the rest frames of a builder) at the tessellation quality of the scale. */
export const meshesOnFrames = (asset: Json, frames: Record<string, Frame>, solids: readonly Json[], scale: Scale): Json[] => meshesIn(asset, frames as never, solids, scale);

/** 📦️ The world box of meshes: [min, max]. */
export function boxOf(meshes: readonly Json[]): { min: V3; max: V3 } {
  const min: V3 = [Infinity, Infinity, Infinity], max: V3 = [-Infinity, -Infinity, -Infinity];
  for (const m of meshes) for (let i = 0; i < m.world.length; i += 3) for (let k = 0; k < 3; k++) { min[k] = Math.min(min[k]!, m.world[i + k]); max[k] = Math.max(max[k]!, m.world[i + k]); }
  return { min, max };
}

/** ↔️ A view outline converted to the coordinates of a shape bound to a sagittal bone of rest frame `f`: elevation in the bone's (u, v), front (z, y) and plan (x, z) in rest world coordinates. */
export function toShapeCoords(view: View, f: Frame, ring: readonly P[]): P[] {
  if (view !== "elevation") return ring.map(([x, y]) => [x, y] as P);
  return ring.map(([x, y]) => { const l = toLocal(f, [x, y, f.o[2]]); return [l[0], l[1]] as P; });
}

/** 💾️ Writes a JSON document (pretty with two spaces, LF) and creates its folder. */
export function writeJson(path: string, value: unknown): void {
  mkdirSync(dirname(path), { recursive: true });
  writeFileSync(path, JSON.stringify(value, null, 1) + "\n");
}

/** 📋️ The manifest row of an evaluated asset, in the shape of the existing `entries`. */
export function manifestRow(asset: Json, distinction: Text): Json {
  return {
    id: asset.id, category: asset.category, title: asset.title, heightM: asset.dimensions.heightM, widthM: asset.dimensions.widthM, powerW: 0,
    poses: asset.poses.filter((p: Json) => !p.key).map((p: Json) => p.id), clips: asset.clips.map((c: Json) => c.id),
    representations: Object.fromEntries(["50", "100", "200", "500"].map((k) => [k, asset.representations[k].status])), symbols: asset.symbols.map((s: Json) => s.id),
    analytics: ["thermal", "transport", "structural"], exports: asset.exports, distinction, completion: asset.completion,
  };
}

/** 📝️ Bilingual decisions of one representation. */
export const decisions = (d: Record<"silhouette" | "retained" | "merged" | "omitted" | "thinElements" | "hatching" | "styles" | "overlays", Text>): Json => d;

/** 🌬️ EN 1991-1-4 drag of a part of projected area A at air speed v: F = cf · ½ ρ v² · A with ρ = 1.25 kg/m³. */
export const drag = (cf: number, v: number, areaM2: number): number => cf * 0.5 * 1.25 * v * v * areaM2;

/** 💎️ The convex hull of points in 3D (incremental, with a tiny tolerance): its vertices and outward triangles, for `polyhedron` solids of merged masses. */
export function hull3(points: readonly V3[]): { vertices: V3[]; triangles: [number, number, number][] } {
  const pts = points.filter((p, i) => points.findIndex((q) => len(sub(p, q)) < 1e-7) === i);
  const eps = 1e-9;
  const far = (from: V3, dir?: V3) => pts.reduce((best, p, i) => { const d = dir ? Math.abs(dot(sub(p, from), dir)) : len(sub(p, from)); return d > best.d ? { i, d } : best; }, { i: -1, d: -1 }).i;
  const a = 0, b = far(pts[0]!);
  const ab = unit(sub(pts[b]!, pts[a]!));
  const c = pts.reduce((best, p, i) => { const d = len(cross(sub(p, pts[a]!), ab)); return d > best.d ? { i, d } : best; }, { i: -1, d: -1 }).i;
  const n0 = unit(cross(sub(pts[b]!, pts[a]!), sub(pts[c]!, pts[a]!)));
  const d = far(pts[a]!, n0);
  let faces: [number, number, number][] = [[a, b, c], [a, c, d], [a, d, b], [b, d, c]];
  const centre = mul(add(add(pts[a]!, pts[b]!), add(pts[c]!, pts[d]!)), 0.25);
  const normal = (f: [number, number, number]) => cross(sub(pts[f[1]]!, pts[f[0]]!), sub(pts[f[2]]!, pts[f[0]]!));
  faces = faces.map((f) => (dot(normal(f), sub(pts[f[0]]!, centre)) < 0 ? [f[0], f[2], f[1]] : f));
  for (let i = 0; i < pts.length; i++) {
    if ([a, b, c, d].includes(i)) continue;
    const p = pts[i]!;
    const visible = faces.filter((f) => dot(unit(normal(f)), sub(p, pts[f[0]]!)) > eps);
    if (visible.length === 0) continue;
    const edges = new Map<string, [number, number]>();
    for (const f of visible) for (const [x, y] of [[f[0], f[1]], [f[1], f[2]], [f[2], f[0]]] as [number, number][]) {
      const back = `${y},${x}`;
      if (edges.has(back)) edges.delete(back); else edges.set(`${x},${y}`, [x, y]);
    }
    faces = faces.filter((f) => !visible.includes(f));
    for (const [x, y] of edges.values()) faces.push([x, y, i]);
  }
  const used = [...new Set(faces.flat())].sort((x, y) => x - y);
  const index = new Map(used.map((v, k) => [v, k] as const));
  return { vertices: used.map((v) => pts[v]!), triangles: faces.map((f) => f.map((v) => index.get(v)!) as [number, number, number]) };
}

/** 📐️ The 2D convex hull of points (monotone chain), counter-clockwise. */
export function hull2(points: readonly P[]): P[] {
  const s = [...points].sort((p, q) => p[0] - q[0] || p[1] - q[1]);
  const turn = (o: P, a: P, b: P) => (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0]);
  const chain = (list: P[]) => { const out: P[] = []; for (const p of list) { while (out.length >= 2 && turn(out[out.length - 2]!, out[out.length - 1]!, p) <= 1e-12) out.pop(); out.push(p); } out.pop(); return out; };
  return s.length < 3 ? s : [...chain(s), ...chain([...s].reverse())];
}

/** 🧩️ The union of simple polygons (the product's own union) as outlines with holes. */
export function union2(polys: readonly (readonly P[])[]): Outline[] {
  const rings = unionPolygons(polys.map((r) => r.map(([x, y]) => ({ x, y })))) as { x: number; y: number }[][];
  const signed = (r: { x: number; y: number }[]) => ringArea(r as never);
  const groups: Outline[] = rings.filter((r) => signed(r) > 0).sort((a, b) => signed(b) - signed(a)).map((r) => ({ outer: r.map((p) => [p.x, p.y] as P), holes: [] }));
  for (const h of rings.filter((r) => signed(r) < 0)) groups.find((g) => pointInRing(h[0] as never, g.outer.map(([x, y]) => ({ x, y })) as never))?.holes.push(h.map((p) => [p.x, p.y] as P));
  return groups;
}

/** 🗺️ A world point projected to a view's paper coordinates (elevation x,y; front z,y; plan x,z). */
export const project = (view: View, p: V3): P => (view === "elevation" ? [p[0], p[1]] : view === "front" ? [p[2], p[1]] : [p[0], p[2]]);

/** 📐️ The area of a [u, w] ring (absolute). */
export const polyArea = (r: readonly P[]): number => Math.abs(r.reduce((s, [x, y], i) => { const [x2, y2] = r[(i + 1) % r.length]!; return s + x * y2 - x2 * y; }, 0) / 2);
