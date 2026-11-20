/** 🌳 Round 3 london plane (pollarded): the handcrafted model of every scale, stated solid by solid in world metres (x east, y up, z left) and written bone-local into `volumes/vegetation/london-plane-pollarded.json`.
 *
 * Usage: `bun r3-london-plane-pollarded-model.ts` (all four scales; prints counts and the body extents). Every section, shoot tip and leaf cluster is a number in the tables below; nothing is fitted to a drawing.
 * @see ./round3-brief.md
 * @see ./r3-vegetation-london-plane-pollarded.md */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { TICKET, entryOf, frames, meshesAt, toLocal, type Json, type V3 } from "./volumes-lib.ts";

const SPEC = "vegetation/london-plane-pollarded";
export const FILE = join(TICKET, "volumes", `${SPEC}.json`);
export const { asset } = entryOf(SPEC);
const F = frames(asset);
const RAD = Math.PI / 180;
const r5 = (x: number): number => Math.round(x * 1e5) / 1e5;
const add = (a: V3, b: V3, k = 1): V3 => [a[0] + b[0] * k, a[1] + b[1] * k, a[2] + b[2] * k];
const sub = (a: V3, b: V3): V3 => [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
const len = (a: V3): number => Math.hypot(a[0], a[1], a[2]);
const unit = (a: V3): V3 => { const l = len(a) || 1; return [a[0] / l, a[1] / l, a[2] / l]; };
const cross = (a: V3, b: V3): V3 => [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
const lerp = (a: V3, b: V3, t: number): V3 => add(a, sub(b, a), t);
const UP: V3 = [0, 1, 0];
/** 🧭 A point at radius `r`, azimuth `az` degrees (0 = east, 90 = left) and height `y`; the crown is 4 % narrower in depth than in width. */
const ZS = 0.985;
const polar = (r: number, az: number, y: number, zs = ZS): V3 => [r * Math.cos(az * RAD), y, r * Math.sin(az * RAD) * zs];
const radial = (az: number): V3 => [Math.cos(az * RAD), 0, Math.sin(az * RAD)];
const loc = (bone: string, p: V3): V3 => toLocal(F[bone]!, p).map(r5) as V3;
const dirLoc = (bone: string, d: V3): V3 => { const f = F[bone]!; return [d[0] * f.u[0] + d[1] * f.u[1] + d[2] * f.u[2], d[0] * f.v[0] + d[1] * f.v[1] + d[2] * f.v[2], d[0] * f.w[0] + d[1] * f.w[1] + d[2] * f.w[2]]; };
const origin = (bone: string): V3 => [...F[bone]!.o] as V3;
const tipOf = (bone: string): V3 => add(origin(bone), F[bone]!.u as V3, asset.rig.bones.find((b: Json) => b.id === bone).lengthM);

/** 🔄 The unit quaternion [x, y, z, w] of the rotation whose matrix columns are the bone-local axes `x`, `y`, `z`. */
function quat(x: V3, y: V3, z: V3): [number, number, number, number] {
  const [m00, m10, m20, m01, m11, m21, m02, m12, m22] = [x[0], x[1], x[2], y[0], y[1], y[2], z[0], z[1], z[2]];
  const t = m00 + m11 + m22;
  let q: [number, number, number, number];
  if (t > 0) { const s = Math.sqrt(t + 1) * 2; q = [(m21 - m12) / s, (m02 - m20) / s, (m10 - m01) / s, s / 4]; }
  else if (m00 > m11 && m00 > m22) { const s = Math.sqrt(1 + m00 - m11 - m22) * 2; q = [s / 4, (m01 + m10) / s, (m02 + m20) / s, (m21 - m12) / s]; }
  else if (m11 > m22) { const s = Math.sqrt(1 + m11 - m00 - m22) * 2; q = [(m01 + m10) / s, s / 4, (m12 + m21) / s, (m02 - m20) / s]; }
  else { const s = Math.sqrt(1 + m22 - m00 - m11) * 2; q = [(m02 + m20) / s, (m12 + m21) / s, s / 4, (m10 - m01) / s]; }
  const n = Math.hypot(...q);
  return q.map((v) => r5(v / n)) as [number, number, number, number];
}

const BARK = { material: "plane-bark", tone: 0.33 }, SCAR = { material: "plane-bark-scarred", tone: 0.35 }, TWIG = { material: "plane-twig", tone: 0.35 };
const SUN = { material: "plane-leaf-sun", tone: 0.6 }, SHADE = { material: "plane-leaf-shade", tone: 0.45 }, WOOD = { material: "plane-wood", tone: 0.45 };
type Look = { material: string; tone: number };
type Sec = { c: V3; r: number; rz?: number; e?: number; a?: V3 };

/** 🧵 A loft through world sections. Round sections follow the path; a section with `rz` is a horizontal ellipse with the semi-axis `r` along world x and `rz` along world z; a section with the reference direction `a` is a circle perpendicular to the smoothed path whose first axis is `a` projected into its plane (no twist where the path runs along the bone). */
function loft(id: string, bone: string, look: Look, secs: readonly Sec[], extra: Json = {}): Json {
  const f = F[bone]!;
  const rot = quat(dirLoc(bone, [1, 0, 0]), dirLoc(bone, [0, 0, 1]), dirLoc(bone, [0, -1, 0]));
  void f;
  const { slender, ...rest } = extra;
  const run = secs.slice(1).reduce((n, s, i) => n + len(sub(s.c, secs[i]!.c)), 0), thick = 2 * Math.max(...secs.map((s) => Math.max(s.r, s.rz ?? 0)));
  return { id, kind: "loft", bone, material: look.material, role: "body", tone: look.tone, sections: secs.map((s, i) => ({ at: loc(bone, s.c), rx: r5(s.r), ry: r5(s.rz ?? s.r), exponent: s.e ?? 2, ...(s.rz !== undefined ? { rotation: rot } : s.a !== undefined ? { rotation: across(bone, unit(sub(secs[Math.min(secs.length - 1, i + 1)]!.c, secs[Math.max(0, i - 1)]!.c)), s.a) } : {}) })), capStart: true, capEnd: true, ...rest, ...(slender === true && run >= 20.5 * thick ? { slender: true } : {}) };
}
/** 🧭 The rotation of a section perpendicular to the unit tangent `n` whose first axis is `a` projected into the section plane. */
function across(bone: string, n: V3, a: V3): [number, number, number, number] {
  const x = unit(add(a, n, -(a[0] * n[0] + a[1] * n[1] + a[2] * n[2]))), y = cross(x, n);
  return quat(dirLoc(bone, x), dirLoc(bone, y), dirLoc(bone, cross(x, y)));
}
const ball = (id: string, bone: string, look: Look, c: V3, r: number, extra: Json = {}): Json => ({ id, kind: "ellipsoid", bone, material: look.material, role: "body", tone: look.tone, radii: [r5(r), r5(r), r5(r)], transform: { translation: loc(bone, c) }, ...extra });
/** 🥚 An ellipsoid with world semi-axes `a` along the unit direction `ax`, `b` along `bx` and `c` along their cross product. */
function egg(id: string, bone: string, look: Look, at: V3, ax: V3, bx: V3, radii: V3, extra: Json = {}): Json {
  const x = unit(ax), z = unit(cross(x, bx)), y = cross(z, x);
  return { id, kind: "ellipsoid", bone, material: look.material, role: "body", tone: look.tone, radii: radii.map(r5), transform: { rotation: quat(dirLoc(bone, x), dirLoc(bone, y), dirLoc(bone, z)), translation: loc(bone, at) }, ...extra };
}
const bezier = (a: V3, b: V3, c: V3, t: number): V3 => add(add([0, 0, 0], a, (1 - t) * (1 - t)), add([0, 0, 0], b, 2 * t * (1 - t)), 1).map((v, i) => v + c[i]! * t * t) as V3;

/** 🪵 The five knuckle limbs: bone, azimuth of the limb round the head. */
const KNUCKLES: Record<string, { bone: string; shoot: string; az: number }> = {
  e2: { bone: "k-e2", shoot: "s-e2a", az: -10 }, e1: { bone: "k-e1", shoot: "s-e1a", az: 60 }, w1: { bone: "k-w1", shoot: "s-w1a", az: 125 }, w2: { bone: "k-w2", shoot: "s-w2a", az: 190 }, c: { bone: "k-c", shoot: "s-ca", az: 270 },
};
const limbAt = (k: string, t: number): V3 => lerp(origin(KNUCKLES[k]!.bone), origin(KNUCKLES[k]!.shoot), t);
const fist = (k: string): V3 => limbAt(k, 0.93);

/** 🍃 The twenty foliage groups of the crown: id (the 2D front and plan shape ids are `f-<id>` and `p-<id>`), bone, ring and azimuth. */
export const GROUPS: readonly { id: string; bone: string; ring: "core" | "top" | "outer" | "diag" | "middle" | "low"; az: number; region: string }[] = [
  { id: "core", bone: "fol-core", ring: "core", az: 0, region: "leaf-shade" }, { id: "top", bone: "fol-top", ring: "top", az: 0, region: "leaf-sun" },
  { id: "o-0", bone: "fol-e2", ring: "outer", az: 0, region: "leaf-sun" }, { id: "o-180", bone: "fol-w2", ring: "outer", az: 180, region: "leaf-sun" },
  { id: "ol-90", bone: "fol-e2", ring: "outer", az: 90, region: "leaf-sun" }, { id: "ol-270", bone: "fol-w2", ring: "outer", az: 270, region: "leaf-sun" },
  { id: "od-45", bone: "fol-e2", ring: "diag", az: 45, region: "leaf-sun" }, { id: "od-135", bone: "fol-w2", ring: "diag", az: 135, region: "leaf-sun" },
  { id: "od-225", bone: "fol-w2", ring: "diag", az: 225, region: "leaf-sun" }, { id: "od-315", bone: "fol-e2", ring: "diag", az: 315, region: "leaf-sun" },
  { id: "m-0", bone: "fol-e1", ring: "middle", az: 0, region: "leaf-sun" }, { id: "m-180", bone: "fol-w1", ring: "middle", az: 180, region: "leaf-sun" },
  { id: "m-60", bone: "fol-e1", ring: "middle", az: 60, region: "leaf-sun" }, { id: "m-120", bone: "fol-w1", ring: "middle", az: 120, region: "leaf-sun" },
  { id: "m-240", bone: "fol-w1", ring: "middle", az: 240, region: "leaf-sun" }, { id: "m-300", bone: "fol-e1", ring: "middle", az: 300, region: "leaf-sun" },
  { id: "lo-0", bone: "fol-low-e", ring: "low", az: 0, region: "leaf-sun" }, { id: "lo-180", bone: "fol-low-w", ring: "low", az: 180, region: "leaf-sun" },
  { id: "lo-90", bone: "fol-low-e", ring: "low", az: 90, region: "leaf-sun" }, { id: "lo-270", bone: "fol-low-w", ring: "low", az: 270, region: "leaf-sun" },
];
const groupOf = (id: string) => GROUPS.find((g) => g.id === id)!;

/** 🌿 The 36 secondary water shoots of 1:50: knuckle, foliage group, tip (radius, azimuth, height), first lean from the vertical in degrees. The five leading shoots are the rig bones. */
const SHOOTS: readonly [string, string, number, number, number, number][] = [
  ["e2", "o-0", 3.225, 13, 7.25, 58], ["e2", "o-0", 3.285, 346, 6.7, 64],
  ["e1", "od-45", 3.225, 37, 7.1, 60], ["e1", "od-45", 3.105, 56, 7.4, 52],
  ["e1", "ol-90", 3.205, 80, 7.2, 56], ["w1", "ol-90", 3.205, 101, 6.95, 60],
  ["w1", "od-135", 3.155, 124, 7.35, 52], ["w1", "od-135", 3.225, 145, 7.0, 60],
  ["w2", "o-180", 3.285, 167, 6.75, 64], ["w2", "o-180", 3.225, 196, 7.2, 58],
  ["w2", "od-225", 3.155, 214, 7.35, 54], ["c", "od-225", 3.155, 235, 7.0, 60],
  ["c", "ol-270", 3.205, 259, 7.25, 56], ["c", "ol-270", 3.205, 282, 6.95, 60],
  ["c", "od-315", 3.105, 304, 7.35, 54], ["e2", "od-315", 3.225, 325, 7.0, 60],
  ["e2", "m-0", 1.5, 350, 8.3, 30],
  ["e1", "m-60", 1.55, 50, 8.47, 26], ["e1", "m-60", 1.45, 76, 8.2, 32],
  ["w1", "m-120", 1.5, 108, 8.42, 26], ["w1", "m-120", 1.55, 133, 8.15, 32],
  ["w2", "m-180", 1.5, 186, 8.35, 30],
  ["c", "m-240", 1.5, 228, 8.2, 32], ["c", "m-240", 1.55, 251, 8.45, 26],
  ["c", "m-300", 1.5, 290, 8.4, 26], ["e2", "m-300", 1.5, 318, 8.15, 34],
  ["e1", "top", 0.42, 40, 8.777, 12], ["w1", "top", 0.5, 150, 8.5, 14],
  ["e2", "lo-0", 2.6, 356, 5.45, 78], ["e2", "lo-0", 2.45, 21, 5.25, 82],
  ["e1", "lo-90", 2.5, 77, 5.4, 78], ["w1", "lo-90", 2.45, 106, 5.25, 82],
  ["w2", "lo-180", 2.6, 178, 5.45, 78], ["w2", "lo-180", 2.45, 201, 5.25, 82],
  ["c", "lo-270", 2.5, 262, 5.4, 78], ["c", "lo-270", 2.45, 286, 5.25, 82],
];
/** 🌱 The leading shoots (rig bones): knuckle, foliage group. */
const LEADERS: readonly [string, string][] = [["e2", "o-0"], ["e1", "m-0"], ["c", "top"], ["w1", "m-180"], ["w2", "o-180"]];
/** 🍂 Leaf clusters along a shoot per ring: position along the shoot (0 base, 1 tip), horizontal radius, half height. The tip plume is a teardrop of the last radius. */
const CLUSTERS: Record<string, { lumps: readonly [number, number, number][]; tip: number }> = {
  outer: { lumps: [[0.42, 0.5, 0.44], [0.6, 0.6, 0.52], [0.79, 0.5, 0.46]], tip: 0.36 },
  diag: { lumps: [[0.46, 0.46, 0.4], [0.64, 0.56, 0.5], [0.81, 0.52, 0.46]], tip: 0.34 },
  middle: { lumps: [[0.48, 0.46, 0.42], [0.66, 0.54, 0.48], [0.83, 0.46, 0.42]], tip: 0.33 },
  top: { lumps: [[0.55, 0.44, 0.4], [0.72, 0.5, 0.44], [0.86, 0.42, 0.38]], tip: 0.31 },
  low: { lumps: [[0.5, 0.36, 0.3], [0.7, 0.42, 0.34], [0.87, 0.38, 0.3]], tip: 0.26 },
};
/** 🌑 The shade clusters of the inner crown (1:50): radius from the axis, azimuth, height, horizontal radius, half height. */
const CORE: readonly [number, number, number, number, number][] = [[0, 0, 6.75, 1.0, 0.95], [0.95, 30, 5.85, 0.8, 0.62], [0.95, 150, 5.85, 0.8, 0.62], [0.95, 270, 5.85, 0.8, 0.62], [0.85, 90, 6.9, 0.8, 0.7], [0.85, 210, 6.9, 0.8, 0.7], [0.85, 330, 6.9, 0.8, 0.7]];

const CL7: readonly [number, number][] = [[-1, 0.2], [-0.8, 0.66], [-0.42, 0.96], [0.05, 1], [0.5, 0.84], [0.82, 0.52], [1, 0.16]];
const CL5: readonly [number, number][] = [[-1, 0.28], [-0.55, 0.9], [0, 1], [0.6, 0.76], [1, 0.2]];
const TEAR: readonly [number, number][] = [[0, 0.5], [0.18, 0.9], [0.45, 1], [0.72, 0.8], [0.92, 0.45], [1, 0.16]];
/** 🍃 A leaf cluster: a body of revolution about the vertical with a flat underside and a rounded top. */
const REG: Record<string, { bone: string; look: Look; secs: Sec[]; group?: string }> = {};
/** 🗃️ Per scale: the foliage solids (world sections, group) the 2D redraw reads. */
export const REGS: Record<string, typeof REG> = {};
const tag = (s: Json, group: string): Json => { REG[s.id]!.group = group; return s; };
const keep = (id: string, bone: string, look: Look, secs: Sec[]): Json => { REG[id] = { bone, look, secs }; return loft(id, bone, look, secs); };
const cluster = (id: string, bone: string, look: Look, c: V3, r: number, h: number, profile = CL7): Json => keep(id, bone, look, profile.map(([t, k]) => ({ c: add(c, UP, t * h), r: r * k })));
/** 🥚 A foliage mass of a coarse scale: horizontal elliptic sections (semi-axes `rx` along x, `rz` along z) stacked over the half height `h`. */
const mass = (id: string, bone: string, look: Look, c: V3, rx: number, rz: number, h: number, profile: readonly [number, number][], lean = 0, e = 2): Json => { const out = unit([c[0], 0, c[2]]); return keep(id, bone, look, profile.map(([t, k]) => ({ c: add(add(c, UP, t * h), out, lean * (t - 0.65)), r: rx * k, rz: rz * k, e }))); };
/** 💨 The outlet patch of an emitter: the upper (or lower) cap of a foliage solid, 3 % proud of it. */
function outlet(L: Lists, id: string, emitter: string, of: string, region: string, top: boolean, n: number): V3 {
  const g = REG[of]!, secs = top ? g.secs.slice(-n) : g.secs.slice(0, n);
  L.thermal.push({ ...loft(id, g.bone, { material: g.look.material, tone: 0.5 }, secs.map((s) => ({ ...s, c: add(s.c, UP, top ? 0.012 : -0.012), r: s.r * 1.03, ...(s.rz === undefined ? {} : { rz: s.rz * 1.03 }) }))), region, tags: [emitter, "emitter"] });
  const end = top ? g.secs[g.secs.length - 1]! : g.secs[0]!;
  return loc(g.bone, end.c);
}
/** 🪶 The leafy tip of a shoot: a teardrop along the shoot from `a` to `b`. */
const plume = (id: string, bone: string, look: Look, a: V3, b: V3, r: number): Json => keep(id, bone, look, TEAR.map(([t, k]) => ({ c: lerp(a, b, t), r: r * k })));

type Lists = { body: Json[]; thermal: Json[]; structure: Json[]; decisions: Json[] };
const th = (L: Lists, s: Json, region: string): void => void L.thermal.push({ id: `th-${s.id}`, of: s.id, region });
const put = (L: Lists, s: Json, region?: string): Json => { L.body.push(s); if (region !== undefined) th(L, s, region); return s; };
const text = (en: string, de: string): Json => ({ en, de });

/** 🌰 Root flare, buttress roots, stem and pollard head; `detail` 3 = 1:50 … 0 = 1:500. */
function trunk(L: Lists, detail: number): void {
  const ax = (bone: string, u: number): V3 => add(origin(bone), F[bone]!.u as V3, u);
  const along = (id: string, bone: string, look: Look, rows: readonly [number, number][], oval = 1): Json => ({ id, kind: "loft", bone, material: look.material, role: "body", tone: look.tone, sections: rows.map(([u, r]) => ({ at: [u, 0, 0], rx: r5(r * oval), ry: r5(r / oval), exponent: 2 })), capStart: true, capEnd: true });
  const flare: [number, number][] = detail >= 2 ? [[0, 0.6], [0.08, 0.54], [0.2, 0.455], [0.34, 0.4], [0.52, 0.37]] : detail === 1 ? [[0, 0.8], [0.12, 0.56], [0.3, 0.42], [0.52, 0.37]] : [[0, 0.64], [0.2, 0.44], [0.52, 0.37]];
  put(L, along("root-flare", "root-plate", BARK, flare), "trunk-bark");
  if (detail >= 2) {
    const roots: readonly [number, number][] = detail === 3 ? [[20, 1], [95, 0.92], [160, 1.04], [235, 0.95], [300, 1]] : [[20, 1], [140, 1], [260, 1]];
    roots.forEach(([az, k], i) => put(L, loft(`buttress-${i + 1}`, "root-plate", BARK, detail === 3 ? [{ c: polar(0.27, az, 0.66), r: 0.085 }, { c: polar(0.4 * k, az, 0.44), r: 0.115 }, { c: polar(0.56 * k, az, 0.24), r: 0.125 }, { c: polar(0.72 * k, az, 0.12), r: 0.105 }, { c: polar(0.86 * k, az, 0.05), r: 0.06 }] : [{ c: polar(0.28, az, 0.6), r: 0.1 }, { c: polar(0.55, az, 0.25), r: 0.13 }, { c: polar(0.84, az, 0.06), r: 0.07 }]), "trunk-bark"));
  }
  const stem: [number, number][] = detail === 3 ? [[-0.05, 0.372], [0.35, 0.345], [0.85, 0.325], [1.7, 0.315], [2.6, 0.308], [3.1, 0.315], [3.38, 0.345]] : detail === 2 ? [[-0.05, 0.372], [0.5, 0.338], [1.7, 0.315], [2.9, 0.31], [3.38, 0.345]] : detail === 1 ? [[-0.05, 0.37], [0.9, 0.325], [2.8, 0.31], [3.38, 0.345]] : [[-0.05, 0.37], [1.2, 0.32], [3.38, 0.335]];
  put(L, along("stem", "trunk", BARK, stem, detail >= 2 ? 1.02 : 1), "trunk-bark");
  const head: [number, number][] = detail === 3 ? [[-0.14, 0.335], [0.08, 0.4], [0.3, 0.462], [0.52, 0.475], [0.7, 0.41], [0.82, 0.27], [0.88, 0.11]] : detail === 2 ? [[-0.14, 0.335], [0.12, 0.415], [0.42, 0.475], [0.68, 0.42], [0.86, 0.16]] : detail === 1 ? [[-0.14, 0.335], [0.3, 0.46], [0.62, 0.44], [0.86, 0.16]] : [[-0.14, 0.335], [0.25, 0.48], [0.62, 0.72], [0.95, 0.66], [1.22, 0.3]];
  put(L, along("head", "head", SCAR, head), "pollard-bark");
  if (detail === 3) {
    ([[3.97, 40, 0.39, 0.115], [4.2, 200, 0.44, 0.13], [4.06, 300, 0.42, 0.1], [4.3, 110, 0.43, 0.11]] as const).forEach(([y, az, r, k], i) => put(L, ball(`head-burr-${i + 1}`, "head", SCAR, add(ax("head", y - 3.75), radial(az), r), k), "pollard-bark"));
    const drawn = (asset.representations["50"].geometry.elevation as Json[]).filter((s) => /^bark-patch-/.test(s.id));
    const patch = (id: string, u: number, deg: number, a: number, b: number): void => {
      const r = 0.345 + (0.308 - 0.345) * Math.min(1, Math.max(0, (u - 0.35) / 2.25));
      const n: V3 = [0, Math.cos(deg * RAD), Math.sin(deg * RAD)];
      L.body.push({ id, kind: "ellipsoid", bone: "trunk", material: BARK.material, role: "texture", tone: 0.15, radii: [r5(a), 0.008, r5(b)], transform: { rotation: [r5(Math.sin((deg * RAD) / 2)), 0, 0, r5(Math.cos((deg * RAD) / 2))], translation: [r5(u), r5(n[1] * (r + 0.001)), r5(n[2] * (r + 0.001))] }, texture: true });
    };
    for (const s of drawn) {
      const nums = (s.d as string).match(/-?\d+(\.\d+)?/g)!.map(Number);
      const us = nums.filter((_, i) => i % 2 === 0), vs = nums.filter((_, i) => i % 2 === 1);
      const u = (Math.min(...us) + Math.max(...us)) / 2, v = (Math.min(...vs) + Math.max(...vs)) / 2;
      patch(s.id, u, (Math.atan2(-Math.sqrt(Math.max(0, 0.1 - v * v)), v) / RAD), (Math.max(...us) - Math.min(...us)) / 2, (Math.max(...vs) - Math.min(...vs)) / 2 + 0.01);
    }
    ([[0.5, 60, 0.09, 0.06], [0.95, 150, 0.1, 0.07], [1.3, 95, 0.08, 0.055], [1.75, 35, 0.11, 0.07], [2.1, 120, 0.09, 0.06], [2.45, 170, 0.1, 0.065], [2.8, 70, 0.085, 0.06], [3.05, 140, 0.09, 0.055]] as const).forEach(([u, deg, a, b], i) => patch(`bark-patch-back-${i + 1}`, u, deg, a, b));
  }
  for (const [k, K] of Object.entries(KNUCKLES)) {
    const a = add(lerp(origin(K.bone), [origin("head")[0], origin(K.bone)[1] - 0.1, 0], 0.4), [0, 0, 0]);
    if (detail === 0) continue;
    const rows: [number, number][] = detail === 3 ? [[0.22, 0.14], [0.52, 0.128], [0.7, 0.15], [0.83, 0.2], [0.94, 0.218], [1.04, 0.18], [1.1, 0.09]] : detail === 2 ? [[0.3, 0.135], [0.66, 0.14], [0.86, 0.21], [1.0, 0.2], [1.09, 0.1]] : [[0.4, 0.15], [0.9, 0.21], [1.08, 0.11]];
    put(L, loft(`knuckle-${k}`, K.bone, SCAR, [{ c: a, r: detail === 1 ? 0.17 : 0.155 }, ...rows.map(([t, r]) => ({ c: limbAt(k, t), r }))]), "pollard-bark");
    if (detail === 3) {
      const rad = radial(K.az), tan: V3 = [-rad[2], 0, rad[0]], c = fist(k);
      const knobs: readonly [V3, number][] = [[add(add(c, UP, 0.085), tan, 0.135), 0.085], [add(add(c, UP, 0.05), tan, -0.155), 0.075], [add(add(c, rad, 0.15), UP, -0.06), 0.08]];
      knobs.forEach(([p, r], i) => put(L, ball(`knuckle-${k}-knob-${i + 1}`, K.bone, SCAR, p, r), "pollard-bark"));
      const stubs: readonly [V3, V3][] = [[add(add(c, UP, 0.14), rad, -0.08), unit(add(UP, rad, -0.35))], [add(add(c, UP, 0.1), tan, 0.02), unit(add(add(UP, tan, 0.5), rad, 0.3))]];
      stubs.forEach(([p, d], i) => put(L, loft(`knuckle-${k}-stub-${i + 1}`, K.bone, SCAR, [{ c: p, r: 0.042 }, { c: add(p, d, 0.11), r: 0.036 }, { c: add(p, d, 0.15), r: 0.03 }]), "pollard-bark"));
    }
  }
}

/** 🌱 The path of a secondary shoot: from the fist of its knuckle, first leaning out, then turning up to its tip. */
function shootPath(k: string, tipR: number, az: number, y: number, lean: number, n: number): V3[] {
  const K = KNUCKLES[k]!, tip = polar(tipR, az, y), c = fist(k);
  const out = unit([tip[0] - c[0], 0, tip[2] - c[2]]);
  const d0 = unit(add([0, Math.cos(lean * RAD), 0], out, Math.sin(lean * RAD)));
  const p0 = add(c, d0, 0.1), p1 = add(p0, d0, 0.46 * len(sub(tip, p0)));
  void K;
  return Array.from({ length: n }, (_, i) => bezier(p0, p1, tip, i / (n - 1)));
}
const along = (path: readonly V3[], t: number): V3 => { const x = t * (path.length - 1), i = Math.min(path.length - 2, Math.floor(x)); return lerp(path[i]!, path[i + 1]!, x - i); };

/** ↔️ The horizontal direction across a shoot (perpendicular to the vertical plane through its foot and its tip). */
const sideOf = (path: readonly V3[]): V3 => { const d = sub(path[path.length - 1]!, path[0]!), h = Math.hypot(d[0], d[2]); return h < 0.05 ? [0, 0, 1] : [-d[2] / h, 0, d[0] / h]; };
type Shoot = { key: string; k: string; group: string; path: V3[]; seed: number; big: number; leader: boolean };
/** 🌾 All 41 shoots: the five leaders along their two rig bones and the 36 stated shoots, each with `n` path points for its wood. */
function allShoots(n: number): Shoot[] {
  const out: Shoot[] = LEADERS.map(([k, group], i) => ({ key: `${k}-lead`, k, group, path: allShootPath(`${k}-lead`), seed: i + 3, big: 1.06, leader: true }));
  const count: Record<string, number> = {};
  SHOOTS.forEach(([k, group, r, az, y, lean], i) => { const c = (count[k] = (count[k] ?? 0) + 1); out.push({ key: `${k}-${c}`, k, group, path: shootPath(k, r, az, y, lean, n), seed: i, big: 1, leader: false }); });
  return out;
}
/** 🛤️ The centre line of a shoot by key, finely sampled (the foliage is strung along it). */
function allShootPath(key: string): V3[] {
  const [k, n] = key.split("-") as [string, string];
  if (n === "lead") {
    const a = KNUCKLES[k]!.shoot, b = a.replace(/a$/, "b"), p0 = add(fist(k), unit(sub(tipOf(a), origin(a))), -0.02), p1 = tipOf(a), p2 = tipOf(b);
    return [...[0, 0.25, 0.5, 0.75].map((t) => lerp(p0, p1, t)), ...[0, 0.25, 0.5, 0.75, 1].map((t) => lerp(p1, p2, t))];
  }
  const row = SHOOTS.filter((s) => s[0] === k)[Number(n) - 1]!;
  return shootPath(k, row[2], row[3], row[4], row[5], 7);
}

/** 🍀 The centre of leaf cluster `j` of a shoot: on the shoot, `off` metres to one side, the side turning by 140 degrees from cluster to cluster. */
const lumpAt = (ring: string, path: readonly V3[], seed: number, j: number, off = 0.13): V3 => { const turn = (seed * 97 + j * 140) * RAD; return add(along(path, CLUSTERS[ring]!.lumps[j]![0]), [Math.cos(turn), 0, Math.sin(turn)], off); };

/** 🌳 1:50: every shoot and every leaf cluster. */
function scale50(): Lists {
  const L: Lists = { body: [], thermal: [], structure: [], decisions: [] };
  trunk(L, 3);
  const foliage = (key: string, g: (typeof GROUPS)[number], path: readonly V3[], seed: number, big: number): void => {
    const C = CLUSTERS[g.ring]!;
    C.lumps.forEach(([, r, h], j) => put(L, tag(cluster(`leaf-${key}-${j + 1}`, g.bone, SUN, lumpAt(g.ring, path, seed, j, 0.22), r * big, h * big), g.id), g.region));
    const a = along(path, 0.88), b = path[path.length - 1]!;
    put(L, tag(plume(`leaf-${key}-tip`, g.bone, SUN, add(a, sub(a, b), 0.35), add(b, unit(sub(b, a)), C.tip * 0.4), C.tip * big), g.id), g.region);
  };
  LEADERS.forEach(([k]) => {
    const a = KNUCKLES[k]!.shoot, b = a.replace(/a$/, "b");
    const p0 = add(fist(k), unit(sub(tipOf(a), origin(a))), -0.02), p1 = tipOf(a), p2 = tipOf(b);
    put(L, loft(`shoot-${k}-a`, a, TWIG, [0, 0.3, 0.65, 1].map((t) => ({ c: lerp(p0, p1, t), r: 0.055 - 0.015 * t })), { slender: true }), "shoot-bark");
    put(L, loft(`shoot-${k}-b`, b, TWIG, [0, 0.35, 0.7, 1].map((t) => ({ c: lerp(p1, p2, t), r: 0.036 - 0.024 * t })), { slender: true }), "shoot-bark");
    put(L, ball(`shoot-${k}-node`, a, TWIG, p1, 0.046), "shoot-bark");
  });
  for (const s of allShoots(7)) {
    if (!s.leader) L.body.push(loft(`shoot-${s.key}`, KNUCKLES[s.k]!.shoot, TWIG, s.path.map((c, j) => ({ c, r: 0.03 - 0.021 * (j / 6) })), { slender: true }));
    foliage(s.key, groupOf(s.group), allShootPath(s.key), s.seed, s.big);
  }
  CORE.forEach(([r, az, y, R, H], i) => put(L, tag(cluster(`leaf-core-${i + 1}`, "fol-core", SHADE, polar(r, az, y), R, H), "core"), "leaf-shade"));
  const sun = ["leaf-w2-lead-2", "leaf-w1-lead-2", "leaf-c-lead-3", "leaf-e1-lead-2", "leaf-e2-lead-2", "leaf-w2-5-2", "leaf-e2-6-2"];
  sun.forEach((of, i) => (ANCHORS[`tp-sun-${i + 1}`] = outlet(L, `emit-transpiration-sun-${i + 1}`, "transpiration-sun", of, "leaf-sun", true, 3)));
  [2, 3, 4, 5, 6].forEach((n, i) => (ANCHORS[`tp-shade-${i + 1}`] = outlet(L, `emit-transpiration-shade-${i + 1}`, "transpiration-shade", `leaf-core-${n}`, "leaf-shade", false, 3)));
  structure(L, 3);
  L.decisions = DECISIONS["50"]!;
  return L;
}

/** ⚓ Emitter anchors moved onto the outlet patches of the 1:50 crown (bone-local x, y, z). */
export const ANCHORS: Record<string, V3> = {};

/** 🏗️ The load path: the wood under the bark along the real limb axes, joints at the rig joints, the fixed root plate, masses at the centres of mass; `detail` 3 = 1:50 … 0 = 1:500. */
function structure(L: Lists, detail: number): void {
  const S = (s: Json, member: string, part: string, extra: Json = {}): void => void L.structure.push({ ...s, role: part === "member" ? "body" : "detail", member, part, ...extra });
  const k = [0.86, 0.9, 0.95, 1][detail]!;
  const axial = (id: string, bone: string, rows: readonly [number, number][]): Json => ({ id, kind: "loft", bone, material: WOOD.material, role: "body", tone: WOOD.tone, sections: rows.map(([u, r]) => ({ at: [u, 0, 0], rx: r5(r), ry: r5(r), exponent: 2 })), capStart: true, capEnd: true });
  S(axial("member-root-plate", "root-plate", detail >= 2 ? [[0, 0.5], [0.2, 0.43], [0.45, 0.36]] : [[0, 0.5 * k], [0.45, 0.36 * k]]), "root-plate", "member");
  S(axial("member-trunk", "trunk", detail === 3 ? [[0, 0.355], [0.85, 0.312], [2.6, 0.296], [3.3, 0.31]] : detail === 2 ? [[0, 0.35], [1.2, 0.305], [3.3, 0.3]] : [[0, 0.34 * k], [3.3, 0.3 * k]]), "trunk", "member");
  S(axial("member-head", "head", detail >= 2 ? [[0, 0.31], [0.4, 0.42 * k], [0.8, 0.25]] : [[0, 0.31 * k], [0.8, 0.3 * k]]), "head", "member");
  for (const [n, K] of Object.entries(KNUCKLES)) {
    const a = K.shoot, b = a.replace(/a$/, "b");
    S(loft(`member-${K.bone}`, K.bone, WOOD, [{ c: limbAt(n, 0), r: 0.12 * k }, { c: limbAt(n, 1), r: 0.12 * k }]), K.bone, "member");
    S(loft(`member-${a}`, a, WOOD, detail === 3 ? [{ c: origin(a), r: 0.048 }, { c: lerp(origin(a), tipOf(a), 0.5), r: 0.043 }, { c: tipOf(a), r: 0.036 }] : [{ c: origin(a), r: detail === 2 ? 0.046 : 0.042 * k }, { c: tipOf(a), r: detail === 2 ? 0.038 : 0.042 * k }], { slender: true }), a, "member");
    S(loft(`member-${b}`, b, WOOD, detail === 3 ? [{ c: tipOf(a), r: 0.033 }, { c: lerp(tipOf(a), tipOf(b), 0.5), r: 0.023 }, { c: tipOf(b), r: 0.012 }] : [{ c: tipOf(a), r: detail === 2 ? 0.032 : 0.024 * k }, { c: tipOf(b), r: detail === 2 ? 0.015 : 0.024 * k }], { slender: true }), b, "member");
  }
  if (detail >= 2) {
    const J = (bone: string, r: number): void => S(ball(`joint-${bone}`, bone, { material: WOOD.material, tone: 0.35 }, origin(bone), r), bone, "joint");
    J("trunk", 0.34 * (detail === 3 ? 1 : 0.94)); J("head", 0.325 * (detail === 3 ? 1 : 0.94));
    for (const K of Object.values(KNUCKLES)) { J(K.bone, detail === 3 ? 0.135 : 0.125); J(K.shoot, detail === 3 ? 0.062 : 0.058); J(K.shoot.replace(/a$/, "b"), detail === 3 ? 0.042 : 0.038); }
    if (detail === 3) for (const g of ["fol-core", "fol-w2", "fol-w1", "fol-top", "fol-e1", "fol-e2", "fol-low-w", "fol-low-e"]) J(g, 0.05);
  }
  S({ id: "support-base-support", kind: "box", bone: "root-plate", material: WOOD.material, tone: 0.25, size: [[0.15, 0.1, 0.08, 0.06][detail]!, [0.9, 1.0, 1.2, 1.2][detail]!, [0.9, 1.0, 1.2, 1.2][detail]!], cornerRadius: 0.02, transform: { translation: [[0.075, 0.05, 0.04, 0.03][detail]!, 0, 0] } }, "root-plate", "support", { anchor: "base-support", tags: ["base-support"] });
  const M = (bone: string, f: number): void => {
    const m = asset.rig.bones.find((x: Json) => x.id === bone).mass, fr = F[bone]!;
    const c = add(add(origin(bone), fr.u as V3, m.comX), fr.v as V3, m.comY);
    S(ball(`mass-${bone}`, bone, { material: /^fol/.test(bone) ? SUN.material : WOOD.material, tone: 0.3 }, c, f * Math.cbrt(m.kg)), bone, "mass");
  };
  M("trunk", 0.029 * k);
  if (detail >= 1) { M("root-plate", 0.03 * k); M("head", 0.03 * k); }
  if (detail >= 2) for (const g of detail === 3 ? ["fol-core", "fol-w2", "fol-w1", "fol-top", "fol-e1", "fol-e2", "fol-low-w", "fol-low-e"] : ["fol-core", "fol-w2", "fol-e2"]) M(g, detail === 3 ? 0.05 : 0.046);
}

/** 🧺 Which drawn mass a foliage group belongs to at the coarser scales: the diagonal groups go to their neighbours (1:100), and so do the side groups of the middle and low rings (1:200). */
const MERGE100: Record<string, string> = { "od-45": "o-0", "od-315": "o-0", "od-135": "o-180", "od-225": "o-180" };
const MERGE200: Record<string, string> = { ...MERGE100, "m-60": "m-0", "m-300": "m-0", "m-120": "m-180", "m-240": "m-180", "lo-90": "lo-0", "lo-270": "lo-180" };

/** 🌿 1:100: every shoot's leaf clusters merged into one plume along the shoot; the leaders and one further shoot per drawn group stay as wood. */
function scale100(): Lists {
  const L: Lists = { body: [], thermal: [], structure: [], decisions: DECISIONS["100"]! };
  trunk(L, 2);
  for (const [k] of LEADERS) {
    const a = KNUCKLES[k]!.shoot, b = a.replace(/a$/, "b");
    put(L, loft(`shoot-${k}-a`, a, TWIG, [{ c: fist(k), r: 0.054 }, { c: tipOf(a), r: 0.041 }], { slender: true }), "shoot-bark");
    put(L, loft(`shoot-${k}-b`, b, TWIG, [{ c: tipOf(a), r: 0.038 }, { c: tipOf(b), r: 0.016 }], { slender: true }), "shoot-bark");
  }
  const wood = new Set(["e1-3", "c-2", "e1-4", "w1-4", "c-5", "c-7", "e2-6", "e1-7", "w2-5", "c-8"]);
  for (const s of allShoots(5)) {
    const g = groupOf(s.group), C = CLUSTERS[g.ring]!, t0 = C.lumps[0]![0] - 0.1;
    if (wood.has(s.key)) L.body.push(loft(`shoot-${s.key}`, KNUCKLES[s.k]!.shoot, TWIG, s.path.map((c, j) => ({ c, r: 0.032 - 0.018 * (j / 4) })), { slender: true }));
    const full = allShootPath(s.key), end = full[full.length - 1]!, dir = unit(sub(end, along(full, 0.88)));
    const secs: Sec[] = [{ c: add(along(full, t0), UP, -0.08), r: 0.45 * C.lumps[0]![1] * s.big }, ...C.lumps.map(([, r], j) => ({ c: lumpAt(g.ring, full, s.seed, j, 0.22), r: r * s.big })), { c: add(end, dir, -0.22), r: C.tip * s.big }, { c: add(end, dir, C.tip * 0.02), r: C.tip * s.big * 0.8 }, { c: add(end, dir, C.tip * 0.28), r: C.tip * s.big * 0.46 }, { c: add(end, dir, C.tip * 0.4), r: C.tip * s.big * 0.16 }];
    const side = sideOf(full);
    put(L, tag(keep(`leaf-${s.key}`, g.bone, SUN, secs.map((x) => ({ ...x, a: side }))), MERGE100[g.id] ?? g.id), g.region);
  }
  ([[0, 0, 6.8, 1.35, 0.95], [0.95, 30, 5.85, 0.8, 0.62], [0.95, 150, 5.85, 0.8, 0.62], [0.95, 270, 5.85, 0.8, 0.62]] as const).forEach(([r, az, y, R, H], i) => put(L, tag(cluster(`leaf-core-${i + 1}`, "fol-core", SHADE, polar(r, az, y), R, H, CL5), "core"), "leaf-shade"));
  ["w2-lead", "w1-lead", "c-lead", "e1-lead", "e2-lead", "w2-5", "e2-6"].forEach((key, i) => void outlet(L, `emit-transpiration-sun-${i + 1}`, "transpiration-sun", `leaf-${key}`, "leaf-sun", true, 2));
  [2, 3, 4].forEach((n, i) => void outlet(L, `emit-transpiration-shade-${i + 1}`, "transpiration-shade", `leaf-core-${n}`, "leaf-shade", false, 2));
  structure(L, 2);
  return L;
}

/** 🌲 1:200: every shoot with its leaves as one spindle of five sections (foot, lower girth, upper girth, rounded tip) on the straightened shoot; one shade mass inside; the head with five knuckle stubs, the two outer shoots. */
const EGG: readonly [number, number][] = [[-1, 0.2], [-0.78, 0.68], [-0.35, 0.95], [0.15, 1], [0.6, 0.8], [0.9, 0.45], [1, 0.14]];
function scale200(): Lists {
  const L: Lists = { body: [], thermal: [], structure: [], decisions: DECISIONS["200"]! };
  trunk(L, 1);
  for (const k of ["w2", "e2"]) put(L, loft(`shoot-${k}`, KNUCKLES[k]!.shoot, TWIG, [{ c: fist(k), r: 0.052 }, { c: tipOf(KNUCKLES[k]!.shoot), r: 0.04 }], { slender: true }), "shoot-bark");
  for (const s of allShoots(5)) {
    const g = groupOf(s.group), C = CLUSTERS[g.ring]!, full = allShootPath(s.key), end = full[full.length - 1]!, dir = unit(sub(end, along(full, 0.88)));
    const secs: Sec[] = [{ c: along(full, C.lumps[0]![0] - 0.12), r: 0.42 * C.lumps[0]![1] * s.big }, { c: along(full, C.lumps[0]![0] + 0.04), r: (C.lumps[0]![1] + 0.03) * s.big }, { c: along(full, C.lumps[2]![0]), r: (C.lumps[2]![1] + 0.04) * s.big }, { c: add(end, dir, -0.15), r: C.tip * s.big * 0.72 }, { c: add(end, dir, C.tip * 0.52), r: C.tip * s.big * 0.2 }];
    const side = sideOf(full);
    put(L, tag(keep(`leaf-${s.key}`, g.bone, SUN, secs.map((x) => ({ ...x, a: side }))), MERGE200[g.id] ?? g.id), g.region);
  }
  put(L, tag(mass("leaf-core", "fol-core", SHADE, [0, 6.465, -0.04], 1.62, 1.68, 1.235, EGG), "core"), "leaf-shade");
  ["w2-lead", "w1-lead", "c-lead", "e1-lead", "e2-lead", "w2-5", "e2-6"].forEach((key, i) => void outlet(L, `emit-transpiration-sun-${i + 1}`, "transpiration-sun", `leaf-${key}`, "leaf-sun", true, 2));
  outlet(L, "emit-transpiration-shade-1", "transpiration-shade", "leaf-core", "leaf-shade", false, 2);
  structure(L, 1);
  return L;
}

/** 🌐 1:500: stem, the head with its knuckles as one swelling, and the crown as one mass per foliage group, entered from the extents of that group at 1:50 (centre x, y, z, semi-axes along x and z, half height). */
const FAN: readonly [number, number][] = [[-1, 0.3], [-0.6, 0.62], [-0.15, 0.86], [0.3, 0.98], [0.65, 1], [0.9, 0.8], [1, 0.42]];
const SPIRE: readonly [number, number][] = [[-1, 0.2], [-0.6, 0.55], [-0.1, 0.85], [0.35, 1], [0.65, 0.88], [0.88, 0.58], [1, 0.24]];
const MASS500: Record<string, [number, number, number, number, number, number]> = {
  core: [0, 6.465, -0.04, 1.62, 1.68, 1.235], top: [-0.045, 7.68, -0.21, 0.945, 1.05, 1.22],
  "o-0": [2.395, 6.03, -0.08, 0.995, 1.18, 1.37], "o-180": [-2.37, 6.01, -0.16, 1.03, 1.18, 1.35], "ol-90": [-0.02, 6.33, 2.26, 1.14, 1.03, 1.05], "ol-270": [0.015, 6.365, -2.165, 1.085, 1.105, 1.055],
  "od-45": [1.63, 6.5, 1.63, 1.0, 1.0, 1.07], "od-135": [-1.63, 6.45, 1.63, 1.0, 1.0, 1.06], "od-225": [-1.63, 6.35, -1.63, 1.0, 1.0, 1.15], "od-315": [1.63, 6.27, -1.63, 1.0, 1.0, 1.24],
  "m-0": [1.41, 7.085, 0.115, 0.68, 1.035, 1.345], "m-180": [-1.435, 7.125, 0.19, 0.695, 0.91, 1.365], "m-60": [0.7, 7.305, 1.2, 0.88, 0.75, 1.305], "m-120": [-0.71, 7.255, 1.075, 0.8, 0.695, 1.295],
  "m-240": [-0.61, 7.31, -1.095, 0.79, 0.765, 1.28], "m-300": [0.735, 7.095, -1.075, 0.995, 0.775, 1.445],
  "lo-0": [2.0, 5.03, 0.245, 0.72, 0.875, 0.53], "lo-180": [-1.995, 5.03, -0.375, 0.865, 0.895, 0.53], "lo-90": [-0.015, 5.22, 1.86, 1.035, 0.69, 0.42], "lo-270": [0.1, 5.245, -1.835, 0.89, 0.785, 0.405],
};
function scale500(): Lists {
  const L: Lists = { body: [], thermal: [], structure: [], decisions: DECISIONS["500"]! };
  trunk(L, 0);
  for (const [id, [x, y, z, rx, rz, h]] of Object.entries(MASS500)) {
    const g = groupOf(id), fan = g.ring === "outer" || g.ring === "diag";
    put(L, tag(mass(`crown-${id}`, "fol-core", g.ring === "core" ? SHADE : { material: SUN.material, tone: 0.55 }, [x, y, z], rx, rz, h, g.ring === "middle" ? SPIRE : fan ? FAN : EGG, g.ring === "middle" ? 0.2 : fan ? 0.35 : 0, fan ? 2.7 : 2), "crown"), g.region);
  }
  for (const k of ["w2", "e2"]) L.body.push(loft(`outer-shoot-${k}`, KNUCKLES[k]!.shoot, TWIG, [{ c: limbAt(k, 0.5), r: 0.055 }, { c: origin(KNUCKLES[k]!.shoot), r: 0.05 }, { c: tipOf(KNUCKLES[k]!.shoot), r: 0.04 }]));
  for (const [k, K] of Object.entries(KNUCKLES)) L.thermal.push({ ...loft(`shoot-${k}`, K.shoot, TWIG, [{ c: origin(K.shoot), r: 0.05 }, { c: tipOf(K.shoot), r: 0.038 }, { c: tipOf(K.shoot.replace(/a$/, "b")), r: 0.014 }], { slender: true }), region: "shoot-bark" });
  outlet(L, "emit-transpiration-sun-1", "transpiration-sun", "crown-top", "leaf-sun", true, 2);
  outlet(L, "emit-transpiration-shade-1", "transpiration-shade", "crown-core", "leaf-shade", false, 2);
  structure(L, 0);
  return L;
}

const DECISIONS: Record<string, Json[]> = {
  "50": [
    text("Body: a pollarded street plane as it stands in its second summer after the cut. The stem is one tapered loft (0.74 m at the collar, 0.63 m at breast height, swelling again under the head) on a concave root flare with five buttress roots; the bark plates the elevation draws are thin texture lenses on the stem, eight more lie on the far sides. The pollard head is a swollen boll with four burrs; five short framework limbs leave it in a ring (azimuths -10, 60, 125, 190 and 270 degrees), each ending in a fist with three callus knobs and two old cut stubs.", "Körper: eine gekappte Strassenplatane im zweiten Sommer nach dem Schnitt. Der Stamm ist ein verjüngter Loft (0,74 m am Wurzelhals, 0,63 m in Brusthöhe, unter dem Kopf wieder anschwellend) auf einem konkaven Wurzelanlauf mit fünf Wurzelrippen; die Rindenschuppen der Ansicht sind dünne Texturlinsen auf dem Stamm, acht weitere liegen auf den abgewandten Seiten. Der Kopf ist eine verdickte Kappstelle mit vier Maserknollen; fünf kurze Gerüstäste verlassen ihn im Ring (Azimute -10, 60, 125, 190 und 270 Grad), jeder endet in einer Faust mit drei Kallusknoten und zwei alten Schnittstummeln."),
    text("From the fists rise 41 water shoots: the five leading shoots follow the rig bones with a growth node between the two annual increments, 36 further shoots are stated one by one (knuckle, tip, first lean) and arch out before they turn upright. Each shoot carries three leaf clusters (bodies of revolution with a flat underside) and a rounded tip plume, 164 in all, grouped into the twenty masses the drawings name; seven shade clusters fill the inner crown. Single leaves (0.2 m, 4 mm on paper) are not modelled.", "Aus den Fäusten steigen 41 Wasserreiser: Die fünf Leittriebe folgen den Rig-Knochen mit einem Zuwachsknoten zwischen den zwei Jahrestrieben, 36 weitere Triebe sind einzeln angegeben (Knolle, Spitze, Anfangsneigung) und bogen nach aussen, bevor sie sich aufrichten. Jeder Trieb trägt drei Blattbüschel (Rotationskörper mit flacher Unterseite) und eine gerundete Spitzenfahne, zusammen 164, gruppiert in die zwanzig Massen der Zeichnungen; sieben Schattenbüschel füllen die Innenkrone. Einzelne Blätter (0,2 m, 4 mm auf dem Papier) sind nicht modelliert."),
    text("Thermal: bark, head, leading shoots and every leaf cluster by reference to the body; the secondary shoots are not part of the shoot-bark region, whose declared area is that of the ten leader sections. The twelve transpiration outlets are the upper caps of seven sun clusters and the undersides of five shade clusters, 3 % proud of the cluster. Structure: the wood under the bark along the real limb axes (stem, head, five leaning limbs, ten shoot sections), a joint at every rig joint, the fixed root plate and eleven masses at the centres of mass.", "Thermik: Rinde, Kopf, Leittriebe und jedes Blattbüschel als Verweis auf den Körper; die Nebentriebe gehören nicht zur Region Triebrinde, deren Fläche die der zehn Leittriebabschnitte ist. Die zwölf Transpirationsauslässe sind die oberen Kappen von sieben Sonnenbüscheln und die Unterseiten von fünf Schattenbüscheln, 3 % über dem Büschel. Tragwerk: das Holz unter der Rinde entlang der wirklichen Astachsen (Stamm, Kopf, fünf geneigte Äste, zehn Triebabschnitte), ein Gelenk an jedem Rig-Gelenk, der eingespannte Wurzelteller und elf Massen in den Schwerpunkten."),
  ],
  "100": [
    text("Body, authored anew for 1:100: stem with three buttress roots, head without burrs, five limbs with plain fists, the five leading shoots in two straight sections and one further shoot for each of ten groups; the other shoots run inside the foliage and are left out. The three leaf clusters and the tip of every shoot are merged into one plume along the shoot (41 plumes), the seven shade clusters into four; the four diagonal groups are drawn with their neighbours; bark plates, knobs, stubs and growth nodes are left out.", "Körper, für 1:100 neu gebaut: Stamm mit drei Wurzelrippen, Kopf ohne Maserknollen, fünf Äste mit glatten Fäusten, die fünf Leittriebe in zwei geraden Abschnitten und je ein weiterer Trieb für zehn Gruppen; die übrigen Triebe laufen im Laub und entfallen. Die drei Blattbüschel und die Spitze jedes Triebs sind zu einer Fahne entlang des Triebs vereint (41 Fahnen), die sieben Schattenbüschel zu vier; die vier Diagonalgruppen werden mit ihren Nachbarn gezeichnet; Rindenschuppen, Knoten, Stummel und Zuwachsknoten entfallen."),
    text("Thermal: references to this scale's solids, seven sun outlets on the upper caps of the main masses and three shade outlets under the core. Structure: members with simplified sections, joints of the woody bones only, the root plate and six masses.", "Thermik: Verweise auf die Körper dieses Massstabs, sieben Sonnenauslässe auf den oberen Kappen der Hauptmassen und drei Schattenauslässe unter dem Kern. Tragwerk: Stäbe mit vereinfachten Querschnitten, nur die Gelenke der Holzknochen, der Wurzelteller und sechs Massen."),
  ],
  "200": [
    text("Body, authored anew for 1:200: stem and a plain flare without buttress roots, head, five knuckle stubs and the two outer shoots that show under the crown. Each of the 41 shoots is one leafy spindle of five sections on the straightened shoot (narrow foot, lower girth, upper girth, rounded tip) (the leaf clusters and their sideways offsets are merged into it), the shade leaves are one egg-shaped core. The spindles are drawn as the ten masses of this scale (core, top, four outer, two middle, two low), the diagonal and side groups with their neighbours.", "Körper, für 1:200 neu gebaut: Stamm und glatter Anlauf ohne Wurzelrippen, Kopf, fünf Knollenstummel und die zwei äusseren Triebe, die unter der Krone sichtbar sind. Jeder der 41 Triebe ist eine belaubte Spindel aus fünf Schnitten auf dem begradigten Trieb (schmaler Fuss, unterer Umfang, oberer Umfang, gerundete Spitze) (die Blattbüschel und ihr seitlicher Versatz gehen darin auf), die Schattenblätter sind ein eiförmiger Kern. Gezeichnet werden die Spindeln als die zehn Massen dieses Massstabs (Kern, Spitze, vier äussere, zwei mittlere, zwei untere), die Diagonal- und Seitengruppen mit ihren Nachbarn."),
    text("Thermal: references, the shoot-bark region by the two drawn shoots, outlets on the tips of seven spindles and under the core. Structure: prismatic members for all eighteen load-bearing bones, the root plate and the three wood masses; joints are below this scale.", "Thermik: Verweise, die Region Triebrinde durch die zwei gezeichneten Triebe, Auslässe auf den Spitzen von sieben Spindeln und unter dem Kern. Tragwerk: prismatische Stäbe für alle achtzehn tragenden Knochen, der Wurzelteller und die drei Holzmassen; Gelenke liegen unter diesem Massstab."),
  ],
  "500": [
    text("Body, authored anew for 1:500 and minimal: the stem with its flare, the head with its five knuckles as one swelling of 1.4 m, and the crown as one mass per foliage group (twenty, entered from the extents of the group: fans for the splayed outer groups, spires for the upright middle groups, eggs for top, low groups and the shade core), which together give the lobed outline with the low underside and the widest girth high up. Of the shoots only the two outer leaders are drawn, where they bridge the open air between the head and the low masses (0.2 mm on paper); all others run inside the masses.", "Körper, für 1:500 neu gebaut und minimal: der Stamm mit Anlauf, der Kopf mit seinen fünf Knollen als eine Verdickung von 1,4 m und die Krone als je eine Masse pro Laubgruppe (zwanzig, aus der Ausdehnung der Gruppe eingetragen: Fächer für die gespreizten Aussengruppen, Kegel für die aufrechten Mittelgruppen, Eiformen für Spitze, untere Gruppen und Schattenkern), die zusammen den gelappten Umriss mit tiefer Unterseite und dem grössten Umfang weit oben ergeben. Von den Trieben sind nur die zwei äusseren Leittriebe gezeichnet, wo sie die freie Luft zwischen Kopf und unteren Massen überbrücken (0,2 mm auf dem Papier); alle anderen laufen in den Massen."),
    text("Thermal: stem, head and the crown masses by reference (the core is the shade region), the shoot bark as the five leading shoots inside the crown, one outlet on the top mass and one under the core. Structure: the eighteen members as slender prismatic bars, the root plate and the stem mass.", "Thermik: Stamm, Kopf und Kronenmassen als Verweis (der Kern ist die Schattenregion), die Triebrinde als die fünf Leittriebe in der Krone, ein Auslass auf der Spitzenmasse und einer unter dem Kern. Tragwerk: die achtzehn Stäbe als schlanke prismatische Stäbe, der Wurzelteller und die Stammmasse."),
  ],
};

export function build(): Record<string, Lists> {
  const out: Record<string, Lists> = {};
  for (const [scale, make] of [["50", scale50], ["100", scale100], ["200", scale200], ["500", scale500]] as const) {
    for (const k of Object.keys(REG)) delete REG[k];
    out[scale] = make();
    REGS[scale] = { ...REG };
  }
  return out;
}

if (import.meta.main) {
  const src = JSON.parse(readFileSync(FILE, "utf8"));
  for (const [scale, L] of Object.entries(build())) {
    src.volumes[scale] = { status: "authored", denominator: Number(scale), body: L.body, thermal: L.thermal, structure: L.structure, decisions: L.decisions };
    const ms = meshesAt({ ...asset, volumes: src.volumes }, Number(scale), L.body);
    const lo = [Infinity, Infinity, Infinity], hi = [-Infinity, -Infinity, -Infinity];
    for (const m of ms) for (let i = 0; i < m.world.length; i += 3) for (let a = 0; a < 3; a++) { lo[a] = Math.min(lo[a]!, m.world[i + a]!); hi[a] = Math.max(hi[a]!, m.world[i + a]!); }
    console.log(`1:${scale} body ${L.body.length} thermal ${L.thermal.length} structure ${L.structure.length} | x ${lo[0]!.toFixed(3)}..${hi[0]!.toFixed(3)} y ${lo[1]!.toFixed(3)}..${hi[1]!.toFixed(3)} z ${lo[2]!.toFixed(3)}..${hi[2]!.toFixed(3)} | target x ±3.40 y 0..8.901 z ±3.269`);
  }
  writeFileSync(FILE, `${JSON.stringify(src, null, 1)}\n`);
  writeFileSync(join(TICKET, "🗑️generated", "r3-london-plane-pollarded", "anchors.json"), `${JSON.stringify(Object.fromEntries(Object.entries(ANCHORS).map(([id, p]) => [id, { x: Math.round(p[0] * 1000) / 1000, y: Math.round(p[1] * 1000) / 1000, z: Math.round(p[2] * 1000) / 1000 }])), null, 1)}\n`);
  void egg;
}
