/** 🧰 Chisel of author 04 (people with mobility aids): stations of superelliptic sections stated by hand in figure coordinates (x ahead, y up, z left, default pose) or bone-local, carried into the frame of their bone; tubes along a hand-stated path; mirrored pairs; candidate document in and out. Nothing here invents a form: every helper only writes down what it is given.
 *
 * @see ./rework-brief.md
 * @see ./r3-tools.md */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { rigAt, type Frame } from "./r3-lib.ts";
import { GENERATED, resolveAsset, type CatalogEntry, type Json } from "./volumes-lib.ts";

export type V3 = [number, number, number];
export type Quat = [number, number, number, number];
/** 🍰 One section in the frame of its bone: centre, the directions of its two radii, the radii and the superellipse exponent. */
export type St = { at: V3; ex: V3; ey: V3; rx: number; ry: number; e: number };
export type Common = { id: string; bone: string; material: string; tone: number; role?: string };

export const r5 = (x: number): number => Math.round(x * 1e5) / 1e5;
export const rad = (d: number): number => (d * Math.PI) / 180;
export const deg = (r: number): number => (r * 180) / Math.PI;
export const dot = (a: readonly number[], b: readonly number[]): number => a[0]! * b[0]! + a[1]! * b[1]! + a[2]! * b[2]!;
export const cross = (a: readonly number[], b: readonly number[]): V3 => [a[1]! * b[2]! - a[2]! * b[1]!, a[2]! * b[0]! - a[0]! * b[2]!, a[0]! * b[1]! - a[1]! * b[0]!];
export const add = (a: readonly number[], b: readonly number[], k = 1): V3 => [a[0]! + b[0]! * k, a[1]! + b[1]! * k, a[2]! + b[2]! * k];
export const sub = (a: readonly number[], b: readonly number[]): V3 => [a[0]! - b[0]!, a[1]! - b[1]!, a[2]! - b[2]!];
export const mul = (a: readonly number[], k: number): V3 => [a[0]! * k, a[1]! * k, a[2]! * k];
export const len = (a: readonly number[]): number => Math.hypot(a[0]!, a[1]!, a[2]!);
export const unit = (a: readonly number[]): V3 => mul(a, 1 / (len(a) || 1));
export const lerp3 = (a: readonly number[], b: readonly number[], t: number): V3 => [a[0]! + (b[0]! - a[0]!) * t, a[1]! + (b[1]! - a[1]!) * t, a[2]! + (b[2]! - a[2]!) * t];

/** 🧭 The unit quaternion of the rotation whose columns are `ex`, `ey`, `ex × ey`. */
export function quatOf(ex: readonly number[], ey: readonly number[]): Quat {
  const x = unit(ex);
  const y = unit(sub(ey, mul(x, dot(ey, x))));
  const z = cross(x, y);
  const [m00, m10, m20, m01, m11, m21, m02, m12, m22] = [x[0], x[1], x[2], y[0], y[1], y[2], z[0], z[1], z[2]];
  const tr = m00 + m11 + m22;
  let q: Quat;
  if (tr > 0) {
    const s = Math.sqrt(tr + 1) * 2;
    q = [(m21 - m12) / s, (m02 - m20) / s, (m10 - m01) / s, s / 4];
  } else if (m00 > m11 && m00 > m22) {
    const s = Math.sqrt(1 + m00 - m11 - m22) * 2;
    q = [s / 4, (m01 + m10) / s, (m02 + m20) / s, (m21 - m12) / s];
  } else if (m11 > m22) {
    const s = Math.sqrt(1 + m11 - m00 - m22) * 2;
    q = [(m01 + m10) / s, s / 4, (m12 + m21) / s, (m02 - m20) / s];
  } else {
    const s = Math.sqrt(1 + m22 - m00 - m11) * 2;
    q = [(m02 + m20) / s, (m12 + m21) / s, s / 4, (m10 - m01) / s];
  }
  const n = Math.hypot(...q);
  return q.map((c) => r5(c / n)) as Quat;
}
/** 🔁 The quaternion of a rotation by `d` degrees about the axis `a`. */
export const about = (a: readonly number[], d: number): Quat => { const u = unit(a); const s = Math.sin(rad(d) / 2); return [r5(u[0] * s), r5(u[1] * s), r5(u[2] * s), r5(Math.cos(rad(d) / 2))]; };

/** 📂 The catalog document of an asset as a working copy. */
export function openCatalog(spec: string): { entry: CatalogEntry; doc: Json; asset: Json } {
  const entry = resolveAsset(spec);
  const doc = JSON.parse(readFileSync(entry.path, "utf8")) as Json;
  return { entry, doc, asset: doc.asset as Json };
}
/** 📁 Where the candidate document of an asset and its intermediate parts live. */
export const workDir = (slug: string): string => join(GENERATED, "rework-04", slug);
export const candidatePath = (slug: string): string => join(workDir(slug), "🔣️.json");
export function writeJsonFile(path: string, value: unknown): void {
  mkdirSync(dirname(path), { recursive: true });
  writeFileSync(path, `${JSON.stringify(value)}\n`);
}
export const readJsonFile = (path: string): Json => JSON.parse(readFileSync(path, "utf8")) as Json;

export type Kit = ReturnType<typeof makeKit>;
/** 🪚 The constructors bound to the bone frames of one pose of one asset. */
export function makeKit(asset: Json, pose: string = asset.presentation.defaultPose) {
  const FRAMES: Record<string, Frame> = rigAt(asset, 50, { kind: "pose", pose }).frames;
  const frame = (bone: string): Frame => { const f = FRAMES[bone]; if (!f) throw new Error(`no bone ${bone}`); return f; };
  /** 📌 A point of the figure in the frame of a bone. */
  const L = (bone: string, p: readonly number[]): V3 => { const f = frame(bone); const d = sub(p, f.o); return [dot(d, f.u), dot(d, f.v), dot(d, f.w)]; };
  /** 🧮 A direction of the figure in the frame of a bone. */
  const D = (bone: string, d: readonly number[]): V3 => { const f = frame(bone); return [dot(d, f.u), dot(d, f.v), dot(d, f.w)]; };
  /** 🌍 A bone-local point in figure coordinates. */
  const W = (bone: string, p: readonly number[]): V3 => { const f = frame(bone); return [0, 1, 2].map((k) => f.o[k]! + p[0]! * f.u[k]! + p[1]! * f.v[k]! + p[2]! * f.w[k]!) as V3; };
  /** ✍️ A bone-local station: centre (u, v, w), half-width `rw` along w, half-depth `rv` along v, the section normal turned `n` degrees from u towards v. */
  const st = (u: number, v: number, w: number, rw: number, rv: number, e = 2, n = 0): St => ({ at: [u, v, w], ex: [0, 0, 1], ey: [-Math.sin(rad(n)), Math.cos(rad(n)), 0], rx: rw, ry: rv, e });
  /** 🥞 A horizontal slice of the figure at the height `y`: from `back` to `front` (x), half-width `rw` about the lateral centre `z`. */
  const H = (bone: string, y: number, back: number, front: number, rw: number, e = 2, z = 0): St => ({ at: L(bone, [(back + front) / 2, y, z]), ex: D(bone, [0, 0, 1]), ey: D(bone, [1, 0, 0]), rx: rw, ry: (front - back) / 2, e });
  /** 🍞 A cross slice of the figure at the position `x` ahead: from `bottom` to `top` (y), half-width `rw` about the lateral centre `z`. */
  const X = (bone: string, x: number, bottom: number, top: number, rw: number, e = 2, z = 0): St => ({ at: L(bone, [x, (bottom + top) / 2, z]), ex: D(bone, [0, 0, 1]), ey: D(bone, [0, 1, 0]), rx: rw, ry: (top - bottom) / 2, e });
  /** 🔪 A free slice of the figure: centre `c`, the directions `ax` and `ay` of the two radii in figure coordinates. */
  const S = (bone: string, c: readonly number[], ax: readonly number[], ay: readonly number[], rx: number, ry: number, e = 2): St => ({ at: L(bone, c), ex: unit(D(bone, ax)), ey: unit(D(bone, ay)), rx, ry, e });
  return { FRAMES, frame, L, D, W, st, H, X, S };
}

/** 🪢 Catmull-Rom densification: `k` sections per span, through every stated station. */
export function dense(s: readonly St[], k: number): St[] {
  if (k <= 1 || s.length < 3) return [...s];
  const out: St[] = [];
  const cr = (p0: number, p1: number, p2: number, p3: number, t: number): number => 0.5 * (2 * p1 + (-p0 + p2) * t + (2 * p0 - 5 * p1 + 4 * p2 - p3) * t * t + (-p0 + 3 * p1 - 3 * p2 + p3) * t * t * t);
  for (let i = 0; i + 1 < s.length; i++) {
    const [a, b, c, d] = [s[Math.max(0, i - 1)]!, s[i]!, s[i + 1]!, s[Math.min(s.length - 1, i + 2)]!];
    for (let j = 0; j < k; j++) {
      const t = j / k;
      if (j === 0) { out.push(b); continue; }
      const f = (g: (q: St) => number): number => cr(g(a), g(b), g(c), g(d), t);
      const lo = (g: (q: St) => number): number => Math.min(Math.max(g(b), g(c)) * 1.03, Math.max(Math.min(g(b), g(c)) * 0.97, f(g)));
      out.push({ at: [f((q) => q.at[0]), f((q) => q.at[1]), f((q) => q.at[2])], ex: unit(lerp3(b.ex, c.ex, t)), ey: unit(lerp3(b.ey, c.ey, t)), rx: lo((q) => q.rx), ry: lo((q) => q.ry), e: b.e + (c.e - b.e) * t });
    }
  }
  out.push(s[s.length - 1]!);
  return out;
}
/** 🥚 Rounds an end of a station list: sections shrinking on a quarter ellipse of the length `length` beyond the end station, along the direction from the neighbouring station (`end` = -1 before the first, +1 after the last). */
export function dome(s: readonly St[], end: -1 | 1, length: number, steps = 3): St[] {
  const base = end < 0 ? s[0]! : s[s.length - 1]!;
  const next = end < 0 ? s[1]! : s[s.length - 2]!;
  const n = cross(base.ex, base.ey);
  const dir = dot(sub(base.at, next.at), n) >= 0 ? n : mul(n, -1);
  const extra: St[] = [];
  for (let i = 1; i <= steps; i++) {
    const a = ((i / (steps + 0.3)) * Math.PI) / 2;
    extra.push({ ...base, at: add(base.at, dir, length * Math.sin(a)), rx: base.rx * Math.cos(a), ry: base.ry * Math.cos(a), e: base.e + (2 - base.e) * (i / steps) });
  }
  return end < 0 ? [...extra.reverse(), ...s] : [...s, ...extra];
}
/** 🧅 The stations moved inwards by `t` (a layer shell under a garment), never thinner than `floor`. */
export const inset = (s: readonly St[], t: number, floor = 0.003): St[] => s.map((q) => ({ ...q, rx: Math.max(q.rx - t, floor), ry: Math.max(q.ry - t, floor) }));
/** 🪞 The stations mirrored to the other side of the body (bone-local w negated). */
export const flip = (s: readonly St[]): St[] => s.map((q) => ({ ...q, at: [q.at[0], q.at[1], -q.at[2]], ex: [-q.ex[0], -q.ex[1], q.ex[2]] as V3, ey: [q.ey[0], q.ey[1], -q.ey[2]] as V3 }));

const head = (p: Common, kind: string): Json => ({ id: p.id, kind, bone: p.bone, material: p.material, role: p.role ?? "body", tone: p.tone });
const sectionOf = (q: St): Json => ({ at: q.at.map(r5), rx: r5(q.rx), ry: r5(q.ry), exponent: r5(q.e), rotation: quatOf(q.ex, q.ey) });
/** 🧵 A loft through stations. */
export const loft = (p: Common, s: readonly St[], caps: [boolean, boolean] = [true, true], extra: Json = {}): Json => ({ ...head(p, "loft"), sections: s.map(sectionOf), capStart: caps[0], capEnd: caps[1], ...extra });
/** 🪈 Stations of an elliptic tube along a bone-local path: the section normal follows the path, `ry` lies as near to `up` as the path allows, radii `[rx, ry]` per point (the last pair repeats). */
export function tubeStations(path: readonly V3[], radii: readonly (number | [number, number])[], up: readonly number[] = [0, 1, 0], e = 2): St[] {
  return path.map((at, i) => {
    const t = unit(sub(path[Math.min(i + 1, path.length - 1)]!, path[Math.max(i - 1, 0)]!));
    let ey = sub(up, mul(t, dot(up, t)));
    if (len(ey) < 1e-6) ey = sub([0, 0, 1], mul(t, t[2]));
    ey = unit(ey);
    const r = radii[Math.min(i, radii.length - 1)]!;
    const [rx, ry] = typeof r === "number" ? [r, r] : r;
    return { at, ex: cross(ey, mul(t, -1)), ey, rx, ry, e };
  });
}
export const tube = (p: Common, path: readonly V3[], radii: readonly (number | [number, number])[], up: readonly number[] = [0, 1, 0], extra: Json = {}): Json => loft(p, tubeStations(path, radii, up), [true, true], extra);
export const ellipsoid = (p: Common, radii: V3, at: readonly number[], rotation?: Quat, extra: Json = {}): Json => ({ ...head(p, "ellipsoid"), radii: radii.map(r5), transform: rotation ? { rotation, translation: at.map(r5) } : { translation: at.map(r5) }, ...extra });
export const capsule = (p: Common, p0: readonly number[], p1: readonly number[], r0: number, r1: number, extra: Json = {}): Json => ({ ...head(p, "capsule"), p0: p0.map(r5), p1: p1.map(r5), r0: r5(r0), r1: r5(r1), ...extra });
export const box = (p: Common, size: V3, at: readonly number[], rotation?: Quat, cornerRadius = 0, extra: Json = {}): Json => ({ ...head(p, "box"), size: size.map(r5), ...(cornerRadius > 0 ? { cornerRadius: r5(cornerRadius) } : {}), transform: rotation ? { rotation, translation: at.map(r5) } : { translation: at.map(r5) }, ...extra });
export const extrude = (p: Common, plane: "xy" | "yz" | "zx", profile: readonly (readonly [number, number])[], from: number, to: number, extra: Json = {}): Json => ({ ...head(p, "extrude"), profile: profile.map((q) => q.map(r5)), plane, from: r5(from), to: r5(to), ...extra });
export const revolve = (p: Common, profile: readonly (readonly [number, number])[], origin: readonly number[], axis: readonly number[], extra: Json = {}): Json => ({ ...head(p, "revolve"), profile: profile.map((q) => q.map(r5)), axis: unit(axis).map(r5), origin: origin.map(r5), ...extra });

export const toRight = (s: string): string => s.replace(/-l(?=$|-)/, "-r");
/** 🪞 A solid mirrored to the other side: lateral coordinates negated, ids, bones and members renamed from `-l` to `-r`. */
export function mirrored(solid: Json, tone?: (t: number) => number, rename: (s: string) => string = toRight): Json {
  const m = JSON.parse(JSON.stringify(solid)) as Json;
  m.id = rename(m.id);
  if (m.bone) m.bone = rename(m.bone);
  if (tone && typeof m.tone === "number") m.tone = r5(tone(m.tone));
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
/** 👯 A left solid and its mirrored right twin. */
export const pair = (solid: Json, tone?: (t: number) => number): Json[] => [solid, mirrored(solid, tone)];

/** 🌡️ A thermal reference to a body solid. */
export const thOf = (of: string, region: string, layer?: string, extra: Json = {}): Json => ({ id: `th-${of}`, of, region, ...(layer ? { layer } : {}), ...extra });
/** 🧅 A thermal shell of a loft: the same sections moved inwards by `t`. */
export const shell = (solid: Json, id: string, t: number, region: string, layer: string, tone: number): Json => ({ ...solid, id, tone, sections: solid.sections.map((q: Json) => ({ ...q, rx: r5(Math.max(0.003, q.rx - t)), ry: r5(Math.max(0.003, q.ry - t)) })), region, layer });
/** 🏗️ A structural solid from a plain solid. */
export const part = (solid: Json, member: string, kind: "member" | "joint" | "support" | "mass"): Json => ({ ...solid, member, part: kind });
