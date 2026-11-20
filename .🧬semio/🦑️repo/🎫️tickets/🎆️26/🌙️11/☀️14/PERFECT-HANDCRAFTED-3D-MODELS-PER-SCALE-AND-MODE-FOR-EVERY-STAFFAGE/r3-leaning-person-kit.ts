/** 🧰 Authoring helpers of the round 3 model of `humans/leaning-person`: stations of superelliptic sections stated by hand in the anatomical frame of a bone (along, ahead, left), smooth densification between them, rounded ends, mirrored pairs and nested layer shells. Nothing here guesses a form: every helper only writes down the sections it is given.
 *
 * @see ./r3-leaning-person-build.ts
 * @see ./design.md */
import { rigAt, type Frame } from "./r3-lib.ts";
import { entryOf, type Json } from "./volumes-lib.ts";
import { SPEC as LEANING, candidate } from "./r3-leaning-person-io.ts";

export type V3 = [number, number, number];
export type Quat = [number, number, number, number];
/** 🍰 One section: centre `at` (u, v, w), half-width `rw` along w, half-depth `rv` in the sagittal plane, superellipse exponent `e`, and `n`, the angle in degrees of the section normal from the bone axis u towards v. */
export type St = { at: V3; rw: number; rv: number; e: number; n: number; yaw?: number };

export const SPEC = LEANING;
export const { entry: ENTRY } = entryOf(SPEC);
/** 🧪 The candidate asset (root fixes applied) without its volumes. */
export const ASSET: Json = candidate(false);
export const FRAMES: Record<string, Frame> = rigAt(ASSET, 50, { kind: "pose", pose: ASSET.presentation.defaultPose }).frames;
export const r5 = (x: number): number => Math.round(x * 1e5) / 1e5;
const rad = (d: number): number => (d * Math.PI) / 180;
const dot = (a: readonly number[], b: readonly number[]): number => a[0]! * b[0]! + a[1]! * b[1]! + a[2]! * b[2]!;

/** 🧭 The world direction of a bone axis in the default pose, degrees in the side elevation. */
export const boneDeg = (bone: string): number => (Math.atan2(FRAMES[bone]!.u[1], FRAMES[bone]!.u[0]) * 180) / Math.PI;
/** 📌 A point of the standing figure (x ahead, y up, z left, default pose) in the local frame of a bone. */
export function L(bone: string, p: V3): V3 {
  const f = FRAMES[bone]!;
  const d = [p[0] - f.o[0], p[1] - f.o[1], p[2] - f.o[2]];
  return [dot(d, f.u), dot(d, f.v), dot(d, f.w)];
}
/** 🧮 A direction of the standing figure in the local frame of a bone. */
export function D(bone: string, d: V3): V3 {
  const f = FRAMES[bone]!;
  return [dot(d, f.u), dot(d, f.v), dot(d, f.w)];
}

const qmul = (a: Quat, b: Quat): Quat => [a[3] * b[0] + a[0] * b[3] + a[1] * b[2] - a[2] * b[1], a[3] * b[1] - a[0] * b[2] + a[1] * b[3] + a[2] * b[0], a[3] * b[2] + a[0] * b[1] - a[1] * b[0] + a[2] * b[3], a[3] * b[3] - a[0] * b[0] - a[1] * b[1] - a[2] * b[2]];
const ROT: Quat = [0, -Math.SQRT1_2, 0, Math.SQRT1_2];
/** 🌀 The rotation of a section whose normal lies `n` degrees from u towards v: section x = w, section y in the sagittal plane. */
export const sectionRotation = (n: number): Quat => qmul([0, 0, Math.sin(rad(n) / 2), Math.cos(rad(n) / 2)], ROT).map(r5) as Quat;
/** 🔁 A rotation about the lateral axis w by `deg` (u towards v). */
export const aboutW = (deg: number): Quat => [0, 0, r5(Math.sin(rad(deg) / 2)), r5(Math.cos(rad(deg) / 2))];
/** 🔂 A rotation about the bone axis u by `deg` (v towards w). */
export const aboutU = (deg: number): Quat => [r5(Math.sin(rad(deg) / 2)), 0, 0, r5(Math.cos(rad(deg) / 2))];
/** 🔃 A rotation about the sagittal normal v by `deg` (w towards u). */
export const aboutV = (deg: number): Quat => [0, r5(Math.sin(rad(deg) / 2)), 0, r5(Math.cos(rad(deg) / 2))];
export const compose = (...qs: Quat[]): Quat => qs.reduce((a, b) => qmul(a, b)).map(r5) as Quat;

/** ✍️ A bone-local station. */
export const st = (u: number, v: number, w: number, rw: number, rv: number, e = 2, n = 0): St => ({ at: [u, v, w], rw, rv, e, n });

/** 🧭 +1 when the bone's sagittal normal v points ahead in the standing figure (limbs hanging, feet), -1 when it points back (pelvis, trunk, head). */
export const AHEAD: Record<string, 1 | -1> = { pelvis: -1, torso: -1, head: -1 };
const ahead = (bone: string): 1 | -1 => AHEAD[bone] ?? 1;
/** 🫀 An anatomical station of a bone: `a` along the bone from its joint, `f` ahead of the bone line, `l` to the left; `rw` half-width, `rf` half-depth, exponent `e`, normal tilted `n` degrees from the bone axis towards ahead. */
export const sa = (bone: string, a: number, f: number, l: number, rw: number, rf: number, e = 2, n = 0): St => ({ at: [a, ahead(bone) * f, l], rw, rv: rf, e, n: ahead(bone) * n });
/** 📍 A point of a bone in anatomical terms (along, ahead, left), bone-local. */
export const pa = (bone: string, a: number, f: number, l: number): V3 => [a, ahead(bone) * f, l];
/** 📌 A point of a bone (along, ahead, left) in the world of the default pose. */
export function W(bone: string, a: number, f: number, l: number): V3 {
  const fr = FRAMES[bone]!;
  const p = pa(bone, a, f, l);
  return [0, 1, 2].map((k) => fr.o[k]! + p[0] * fr.u[k]! + p[1] * fr.v[k]! + p[2] * fr.w[k]!) as V3;
}
/** 🥞 A horizontal slice of the standing figure at the height `y`: from `back` to `front` (x), half-width `rw` about the lateral centre `z`. */
export function H(bone: string, y: number, back: number, front: number, rw: number, e = 2, z = 0): St {
  return { at: L(bone, [(back + front) / 2, y, FRAMES[bone]!.o[2] + z]), rw, rv: (front - back) / 2, e, n: 90 - boneDeg(bone) };
}
/** 🍞 A cross slice of the standing figure at the position `x` ahead: from `bottom` to `top` (y), half-width `rw` about the lateral centre `z`. */
export function X(bone: string, x: number, bottom: number, top: number, rw: number, e = 2, z = 0): St {
  return { at: L(bone, [x, (bottom + top) / 2, FRAMES[bone]!.o[2] + z]), rw, rv: (top - bottom) / 2, e, n: -boneDeg(bone) };
}

const lerpAngle = (a: number, b: number, t: number): number => a + (b - a) * t;
/** 🪢 Catmull-Rom densification: `k` sections per span, through every stated station. */
export function dense(s: readonly St[], k: number): St[] {
  if (k <= 1 || s.length < 3) return [...s];
  const out: St[] = [];
  const cr = (p0: number, p1: number, p2: number, p3: number, t: number): number => 0.5 * (2 * p1 + (-p0 + p2) * t + (2 * p0 - 5 * p1 + 4 * p2 - p3) * t * t + (-p0 + 3 * p1 - 3 * p2 + p3) * t * t * t);
  for (let i = 0; i + 1 < s.length; i++) {
    const a = s[Math.max(0, i - 1)]!;
    const b = s[i]!;
    const c = s[i + 1]!;
    const d = s[Math.min(s.length - 1, i + 2)]!;
    for (let j = 0; j < k; j++) {
      const t = j / k;
      if (j === 0) {
        out.push(b);
        continue;
      }
      const f = (g: (q: St) => number): number => cr(g(a), g(b), g(c), g(d), t);
      const lo = (g: (q: St) => number): number => Math.max(Math.min(g(b), g(c)) * 0.97, f(g));
      out.push({ at: [f((q) => q.at[0]), f((q) => q.at[1]), f((q) => q.at[2])], rw: lo((q) => q.rw), rv: lo((q) => q.rv), e: b.e + (c.e - b.e) * t, n: lerpAngle(b.n, c.n, t), ...(b.yaw !== undefined || c.yaw !== undefined ? { yaw: lerpAngle(b.yaw ?? 0, c.yaw ?? 0, t) } : {}) });
    }
  }
  out.push(s[s.length - 1]!);
  return out;
}
/** 🥚 Rounds an end of a station list: sections shrinking on a quarter ellipse of the length `len` beyond the end station, along its normal (`end` = -1 before the first, +1 after the last). */
export function dome(s: readonly St[], end: -1 | 1, len: number, steps = 3): St[] {
  const base = end < 0 ? s[0]! : s[s.length - 1]!;
  const dir = end * (travel(s) >= 0 ? 1 : -1);
  const nu = Math.cos(rad(base.n)) * dir;
  const nv = Math.sin(rad(base.n)) * dir;
  const extra: St[] = [];
  for (let i = 1; i <= steps; i++) {
    const a = ((i / (steps + 0.35)) * Math.PI) / 2;
    const k = Math.cos(a);
    extra.push({ at: [base.at[0] + nu * len * Math.sin(a), base.at[1] + nv * len * Math.sin(a), base.at[2]], rw: base.rw * k, rv: base.rv * k, e: base.e + (2 - base.e) * (i / steps), n: base.n });
  }
  return end < 0 ? [...extra.reverse(), ...s] : [...s, ...extra];
}
function travel(s: readonly St[]): number {
  const a = s[0]!;
  const b = s[s.length - 1]!;
  return (b.at[0] - a.at[0]) * Math.cos(rad(a.n)) + (b.at[1] - a.at[1]) * Math.sin(rad(a.n));
}
/** 🧅 The stations moved inwards by `t` (a layer shell under a garment), never thinner than `floor`. */
export const inset = (s: readonly St[], t: number, floor = 0.003): St[] => s.map((q) => ({ ...q, rw: Math.max(q.rw - t, floor), rv: Math.max(q.rv - t, floor) }));
/** 🪞 The stations mirrored to the other side of the body. */
export const flip = (s: readonly St[]): St[] => s.map((q) => ({ ...q, at: [q.at[0], q.at[1], -q.at[2]] }));

export type Common = { id: string; bone: string; material: string; tone: number; role?: string };
const head = (p: Common, kind: string): Json => ({ id: p.id, kind, bone: p.bone, material: p.material, role: p.role ?? "body", tone: p.tone });
/** 🧵 A loft through stations. */
export function loft(p: Common, s: readonly St[], caps: [boolean, boolean] = [true, true], extra: Json = {}): Json {
  return { ...head(p, "loft"), sections: s.map((q) => ({ at: q.at.map(r5), rx: r5(q.rw), ry: r5(q.rv), exponent: r5(q.e), rotation: q.yaw ? compose(aboutU(q.yaw), sectionRotation(q.n)) : sectionRotation(q.n) })), capStart: caps[0], capEnd: caps[1], ...extra };
}
/** 🪈 A loft of round sections along a free path (sections perpendicular to the path). */
export function tube(p: Common, path: readonly V3[], radii: readonly number[], extra: Json = {}): Json {
  return { ...head(p, "loft"), sections: path.map((at, i) => ({ at: at.map(r5), rx: r5(radii[Math.min(i, radii.length - 1)]!), ry: r5(radii[Math.min(i, radii.length - 1)]!) })), capStart: true, capEnd: true, ...extra };
}
export function ellipsoid(p: Common, radii: V3, at: V3, rotation?: Quat, extra: Json = {}): Json {
  return { ...head(p, "ellipsoid"), radii: radii.map(r5), transform: rotation ? { translation: at.map(r5), rotation } : { translation: at.map(r5) }, ...extra };
}
export function capsule(p: Common, p0: V3, p1: V3, r0: number, r1: number, extra: Json = {}): Json {
  return { ...head(p, "capsule"), p0: p0.map(r5), p1: p1.map(r5), r0: r5(r0), r1: r5(r1), ...extra };
}
export function box(p: Common, size: V3, at: V3, rotation?: Quat, cornerRadius = 0, extra: Json = {}): Json {
  return { ...head(p, "box"), size: size.map(r5), ...(cornerRadius > 0 ? { cornerRadius: r5(cornerRadius) } : {}), transform: rotation ? { translation: at.map(r5), rotation } : { translation: at.map(r5) }, ...extra };
}
export function extrude(p: Common, plane: "xy" | "yz" | "zx", profile: readonly [number, number][], from: number, to: number, extra: Json = {}): Json {
  return { ...head(p, "extrude"), profile: profile.map((q) => q.map(r5)), plane, from: r5(from), to: r5(to), ...extra };
}
export function revolve(p: Common, profile: readonly [number, number][], origin: V3, axis: V3, extra: Json = {}): Json {
  return { ...head(p, "revolve"), profile: profile.map((q) => q.map(r5)), axis: axis.map(r5), origin: origin.map(r5), ...extra };
}

/** 🪞 A solid mirrored to the other side: lateral coordinates negated, ids and bones renamed from `-l` to `-r`. */
export function mirrored(solid: Json, rename: (s: string) => string, tone?: (t: number) => number): Json {
  const m = JSON.parse(JSON.stringify(solid)) as Json;
  m.id = rename(m.id);
  m.bone = rename(m.bone);
  if (tone) m.tone = r5(tone(m.tone));
  const fq = (q: Quat): Quat => [-q[0], -q[1], q[2], q[3]];
  if (m.sections) for (const s of m.sections) { s.at[2] = -s.at[2]; if (s.rotation) s.rotation = fq(s.rotation); }
  if (m.p0) { m.p0[2] = -m.p0[2]; m.p1[2] = -m.p1[2]; }
  if (m.transform?.translation) m.transform.translation[2] = -m.transform.translation[2];
  if (m.transform?.rotation) m.transform.rotation = fq(m.transform.rotation);
  if (m.origin) { m.origin[2] = -m.origin[2]; m.axis[2] = -m.axis[2]; }
  if (m.kind === "extrude") {
    if (m.plane === "xy") [m.from, m.to] = [-m.to, -m.from];
    if (m.plane === "yz") m.profile = m.profile.map((q: number[]) => [q[0], -q[1]]).reverse();
    if (m.plane === "zx") m.profile = m.profile.map((q: number[]) => [-q[0], q[1]]).reverse();
  }
  if (m.of) m.of = rename(m.of);
  if (m.member) m.member = rename(m.member);
  return m;
}
export const toRight = (s: string): string => s.replace(/-l(?=$|-)/g, "-r");

/** 🌡️ A thermal reference to a body solid. */
export const thOf = (of: string, region: string, layer?: string, extra: Json = {}): Json => ({ id: `th-${of}`, of, region, ...(layer ? { layer } : {}), ...extra });
/** 🏗️ A structural solid from a plain solid. */
export const part = (solid: Json, member: string, kind: "member" | "joint" | "support" | "mass"): Json => ({ ...solid, member, part: kind });
