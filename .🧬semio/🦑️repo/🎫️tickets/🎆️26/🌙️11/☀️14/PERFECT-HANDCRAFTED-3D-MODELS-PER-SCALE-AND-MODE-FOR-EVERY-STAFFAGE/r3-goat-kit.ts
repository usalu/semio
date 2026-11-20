/** 🐐️ Round 3 goat: the modelling bench. Solids are stated by hand in world metres of the default pose (x ahead, y up, z lateral with the near side at z < 0, hip joint at x = 0, ground y = 0) and written into the rest frame of their bone; `smooth` only rounds the surface between the stated sections (Catmull-Rom), it invents no form.
 * @see ./r3-animals-goat.md
 * @see ./author-29-lib.ts */
import { existsSync, readFileSync, renameSync, writeFileSync } from "node:fs";
import { quatOf } from "./author-29-lib.ts";
import { entryOf, featureM, meshesAt, poseState, sourcePath, type Json, type V3 } from "./volumes-lib.ts";

export type St = { c: V3; a: V3; b: V3; e?: number };
export type Look = { material: string; tone: number; role?: string };
export const SPEC = "animals/goat";

const sub = (a: V3, b: V3): V3 => [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
const add = (a: V3, b: V3): V3 => [a[0] + b[0], a[1] + b[1], a[2] + b[2]];
const mul = (a: V3, k: number): V3 => [a[0] * k, a[1] * k, a[2] * k];
const dot = (a: V3, b: V3): number => a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
const cross = (a: V3, b: V3): V3 => [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
const len = (a: V3): number => Math.hypot(a[0], a[1], a[2]);
const unit = (a: V3): V3 => mul(a, 1 / (len(a) || 1));
export const r5 = (x: number): number => Math.round(x * 1e5) / 1e5 + 0;
export const vec = { sub, add, mul, dot, cross, len, unit };

/** 🪞️ The far-side twin of near-side stations (z mirrored, section handedness kept). */
export const far = (st: readonly St[]): St[] => st.map((s) => ({ c: [s.c[0], s.c[1], -s.c[2]], a: [-s.a[0], -s.a[1], s.a[2]], b: [s.b[0], s.b[1], -s.b[2]], e: s.e }));

/** 🧵️ Rounds a chain of stated sections: `per` interpolated sections between each pair (Catmull-Rom on centre, both half-axes and exponent). */
export function smooth(keys: readonly St[], per: number): St[] {
  if (per <= 0 || keys.length < 3) return [...keys];
  const out: St[] = [];
  const cr = (p0: number, p1: number, p2: number, p3: number, t: number): number => 0.5 * (2 * p1 + (-p0 + p2) * t + (2 * p0 - 5 * p1 + 4 * p2 - p3) * t * t + (-p0 + 3 * p1 - 3 * p2 + p3) * t * t * t);
  const v3 = (f: (s: St) => V3, i: number, t: number): V3 => {
    const k = (j: number) => f(keys[Math.max(0, Math.min(keys.length - 1, j))]!);
    return [0, 1, 2].map((d) => cr(k(i - 1)[d]!, k(i)[d]!, k(i + 1)[d]!, k(i + 2)[d]!, t)) as V3;
  };
  for (let i = 0; i < keys.length - 1; i++) {
    out.push(keys[i]!);
    for (let j = 1; j <= per; j++) {
      const t = j / (per + 1);
      const e0 = keys[i]!.e ?? 2;
      const e1 = keys[i + 1]!.e ?? 2;
      out.push({ c: v3((s) => s.c, i, t), a: v3((s) => s.a, i, t), b: v3((s) => s.b, i, t), e: e0 + (e1 - e0) * t });
    }
  }
  out.push(keys[keys.length - 1]!);
  return out;
}

/** 🥚️ Closes both ends of a chain with a dome instead of a flat cap: two shrinking sections `d0` before the first and `d1` after the last station (0 leaves that end flat). */
export function dome(st: readonly St[], d0: number, d1: number): St[] {
  const end = (s: St, from: St, d: number): St[] => {
    const t = unit(sub(s.c, from.c));
    return [[0.55, 0.84], [0.9, 0.5], [1, 0.2]].map(([k, f]) => ({ c: add(s.c, mul(t, d * k!)), a: mul(s.a, f!), b: mul(s.b, f!), e: s.e }));
  };
  const head = d0 > 0 ? end(st[0]!, st[1]!, d0).reverse() : [];
  const tail = d1 > 0 ? end(st[st.length - 1]!, st[st.length - 2]!, d1) : [];
  return [...head, ...st, ...tail];
}

/** 📐️ A section in the plane x = const: half-height `ry`, half-width `rz`. */
export const X = (x: number, yc: number, zc: number, ry: number, rz: number, e = 2): St => ({ c: [x, yc, zc], a: [0, 0, rz], b: [0, ry, 0], e });
/** 📐️ A section in the plane y = const: half-length `rx`, half-width `rz`. */
export const Y = (y: number, xc: number, zc: number, rx: number, rz: number, e = 2): St => ({ c: [xc, y, zc], a: [rx, 0, 0], b: [0, 0, rz], e });
/** 📐️ A section across an axis of the sagittal plane: at `s` metres from `p0` along the direction of `deg`, shifted `off` along the left normal, half-depth `ra` in the sagittal plane, half-width `rb` lateral, centre at z = `zc`. */
export const A = (p0: readonly [number, number], deg: number, s: number, off: number, ra: number, rb: number, zc = 0, e = 2): St => {
  const d: [number, number] = [Math.cos((deg * Math.PI) / 180), Math.sin((deg * Math.PI) / 180)];
  const n: [number, number] = [-d[1], d[0]];
  return { c: [p0[0] + d[0] * s + n[0] * off, p0[1] + d[1] * s + n[1] * off, zc], a: [n[0] * ra, n[1] * ra, 0], b: [0, 0, rb], e };
};

export const COAT: Look = { material: "short-coat", tone: 0.72 };
export const PALE: Look = { material: "short-coat", tone: 0.82 };
export const HORN: Look = { material: "horn", tone: 0.5 };
export const CLAW: Look = { material: "horn", tone: 0.36 };
export const SKIN: Look = { material: "bare-skin", tone: 0.64 };
export const EYE: Look = { material: "bare-skin", tone: 0.12, role: "detail" };

export const AO: [number, number] = [0.985, 0.895];
export const HEAD_DEG = -42;
/** 🔀️ Near-side stations with |z| written positive become z < 0. */
export const near = (st: readonly St[]): St[] => st.map((s) => ({ ...s, c: [s.c[0], s.c[1], -s.c[2]] }));
/** ↔️ A section spanned between a dorsal and a ventral point of the sagittal plane, half-width `rb`. */
export const span = (top: readonly [number, number], bottom: readonly [number, number], rb: number, e = 2): St => ({ c: [(top[0] + bottom[0]) / 2, (top[1] + bottom[1]) / 2, 0], a: [(top[0] - bottom[0]) / 2, (top[1] - bottom[1]) / 2, 0], b: [0, 0, rb], e });
/** 🦌️ Sections across a curved centre line of the sagittal plane that drifts sideways: `[x, y, |z|, sagittal half, lateral half]`. */
export function along(points: readonly (readonly [number, number, number, number, number])[], e = 2): St[] {
  return points.map((p, i) => {
    const q0 = points[Math.max(0, i - 1)]!;
    const q1 = points[Math.min(points.length - 1, i + 1)]!;
    const t = unit([q1[0] - q0[0], q1[1] - q0[1], q1[2] - q0[2]]);
    const lat = unit(sub([0, 0, 1], mul(t, t[2])));
    const sag = cross(lat, t);
    return { c: [p[0], p[1], p[2]], a: mul(sag, p[3]), b: mul(lat, p[4]), e };
  });
}


export class Bench {
  readonly asset: Json;
  readonly frames: Record<string, { o: V3; u: V3; v: V3; w: V3 }>;
  readonly feature: number;
  readonly lists: Record<"body" | "thermal" | "structure", Json[]> = { body: [], thermal: [], structure: [] };
  private into: "body" | "thermal" | "structure" = "body";
  private extra: Json = {};
  constructor(readonly scale: number) {
    this.asset = entryOf(SPEC).asset;
    this.frames = poseState(this.asset, scale).frames as never;
    this.feature = featureM(scale, this.asset);
  }

  /** 🗂️ Directs the following solids into a list, each carrying the given extra fields (`region`, `layer`, `member`, `part`, `tags`). */
  list(name: "body" | "thermal" | "structure", extra: Json = {}): this {
    this.into = name;
    this.extra = extra;
    return this;
  }

  frame(bone: string) {
    const f = this.frames[bone];
    if (f === undefined) throw new Error(`no bone ${bone}`);
    return f;
  }

  /** 🌍️ A point of a bone frame (u along the bone, v its sagittal normal, w lateral) in the world. */
  world(bone: string, p: V3): V3 {
    const f = this.frame(bone);
    return add(add(add(f.o, mul(f.u, p[0])), mul(f.v, p[1])), mul(f.w, p[2]));
  }

  local(bone: string, p: V3): V3 {
    const f = this.frame(bone);
    const d = sub(p, f.o);
    return [r5(dot(d, f.u)), r5(dot(d, f.v)), r5(dot(d, f.w))];
  }

  private dir(bone: string, d: V3): V3 {
    const f = this.frame(bone);
    return [dot(d, f.u), dot(d, f.v), dot(d, f.w)];
  }

  private rot(bone: string, ax: V3, ay: V3) {
    const x = unit(this.dir(bone, ax));
    let y = this.dir(bone, ay);
    y = unit(sub(y, mul(x, dot(y, x))));
    return quatOf(x, y, cross(x, y));
  }

  private push(s: Json, extra: Json): Json {
    const full = { ...s, ...this.extra, ...extra };
    if (this.lists[this.into].some((q) => q.id === full.id)) throw new Error(`duplicate ${this.into} solid ${full.id}`);
    this.lists[this.into].push(full);
    return full;
  }

  private base(id: string, kind: string, bone: string, look: Look): Json {
    return { id, kind, bone, material: look.material, role: look.role ?? "body", tone: look.tone };
  }

  loft(id: string, bone: string, stations: readonly St[], look: Look, extra: Json = {}): Json {
    const s = this.base(id, "loft", bone, look);
    s.sections = stations.map((st) => ({ at: this.local(bone, st.c), rx: r5(Math.max(len(st.a), 1e-4)), ry: r5(Math.max(len(st.b), 1e-4)), exponent: r5(st.e ?? 2), rotation: this.rot(bone, st.a, st.b) }));
    s.capStart = true;
    s.capEnd = true;
    return this.push(s, extra);
  }

  /** 🫧️ An ellipsoid with semi-axes `r` along the world directions `ax`, `ay` and their normal. */
  ell(id: string, bone: string, c: V3, r: V3, look: Look, axes: [V3, V3] = [[1, 0, 0], [0, 1, 0]], extra: Json = {}): Json {
    const s = this.base(id, "ellipsoid", bone, look);
    s.radii = r.map(r5);
    s.transform = { translation: this.local(bone, c), rotation: this.rot(bone, axes[0], axes[1]) };
    return this.push(s, extra);
  }

  cap(id: string, bone: string, p0: V3, p1: V3, r0: number, r1: number, look: Look, extra: Json = {}): Json {
    const s = this.base(id, "capsule", bone, look);
    s.p0 = this.local(bone, p0);
    s.p1 = this.local(bone, p1);
    s.r0 = r5(r0);
    s.r1 = r5(r1);
    return this.push(s, extra);
  }

  /** 🔗️ A reference to a body solid of this scale. */
  ref(id: string, of: string, extra: Json = {}): Json {
    if (!this.lists.body.some((q) => q.id === of)) throw new Error(`no body solid ${of}`);
    return this.push({ id, of }, extra);
  }

  /** 🧪️ Signed volume and world extent of every solid of a list (a negative volume is an inside-out solid). */
  probe(name: "body" | "thermal" | "structure" = "body"): { id: string; volume: number; min: V3; max: V3 }[] {
    const own = this.lists[name].filter((s) => s.of === undefined);
    return meshesAt(this.asset, this.scale, own).map((m) => {
      let v = 0;
      const min: V3 = [1e9, 1e9, 1e9];
      const max: V3 = [-1e9, -1e9, -1e9];
      const p = (i: number): V3 => [m.world[3 * i]!, m.world[3 * i + 1]!, m.world[3 * i + 2]!];
      for (let i = 0; i < m.indices.length; i += 3) v += dot(p(m.indices[i]!), cross(p(m.indices[i + 1]!), p(m.indices[i + 2]!))) / 6;
      for (let i = 0; i < m.world.length / 3; i++) for (let d = 0; d < 3; d++) { min[d] = Math.min(min[d]!, m.world[3 * i + d]!); max[d] = Math.max(max[d]!, m.world[3 * i + d]!); }
      return { id: m.id, volume: v, min, max };
    });
  }

  /** 💾️ Writes the lists and decisions of this scale into the author's source (re-read right before, other scales untouched). */
  save(decisions: Json[]): void {
    const file = sourcePath(entryOf(SPEC).entry);
    const fresh: Json = existsSync(file) ? JSON.parse(readFileSync(file, "utf8")) : { volumes: {} };
    fresh.volumes[String(this.scale)] = { status: "authored", denominator: this.scale, body: this.lists.body, thermal: this.lists.thermal, structure: this.lists.structure, decisions };
    const tmp = `${file}.r3-goat.tmp`;
    writeFileSync(tmp, JSON.stringify(fresh));
    renameSync(tmp, file);
  }
}
