/** 🐦️ Author 29: world-space sculpting helpers for animals. Solids are placed in the default pose in world metres (x forward, y up, z lateral, near side z < 0) and converted to the bone frame of the solid.
 * Every loft section carries an explicit `rotation` (the kernel frame of a section without one flips with the path slope), so `rx` runs along the first and `ry` along the second given axis.
 * @see ./kit-lib.ts
 * @see ./author-29.md */
import { openCtx, drawnSpan, saveLists, type Ctx, type Json, type V3 } from "./kit-lib.ts";

export type Q = [number, number, number, number];
export type Look = { material: string; tone?: number; role?: string };
export type Station = { c: V3; a: V3; b: V3; e?: number };

const sub = (a: V3, b: V3): V3 => [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
const dot = (a: V3, b: V3): number => a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
const len = (a: V3): number => Math.hypot(a[0], a[1], a[2]);
const nrm = (a: V3): V3 => {
  const l = len(a) || 1;
  return [a[0] / l, a[1] / l, a[2] / l];
};
const cross = (a: V3, b: V3): V3 => [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
const r5 = (x: number): number => Math.round(x * 1e5) / 1e5;

/** 🔄️ Quaternion [x, y, z, w] of the rotation whose columns are the orthonormal axes `x`, `y`, `z`. */
export function quatOf(x: V3, y: V3, z: V3): Q {
  const m = [[x[0], y[0], z[0]], [x[1], y[1], z[1]], [x[2], y[2], z[2]]];
  const tr = m[0]![0]! + m[1]![1]! + m[2]![2]!;
  let q: Q;
  if (tr > 0) {
    const s = Math.sqrt(tr + 1) * 2;
    q = [(m[2]![1]! - m[1]![2]!) / s, (m[0]![2]! - m[2]![0]!) / s, (m[1]![0]! - m[0]![1]!) / s, s / 4];
  } else if (m[0]![0]! > m[1]![1]! && m[0]![0]! > m[2]![2]!) {
    const s = Math.sqrt(1 + m[0]![0]! - m[1]![1]! - m[2]![2]!) * 2;
    q = [s / 4, (m[0]![1]! + m[1]![0]!) / s, (m[0]![2]! + m[2]![0]!) / s, (m[2]![1]! - m[1]![2]!) / s];
  } else if (m[1]![1]! > m[2]![2]!) {
    const s = Math.sqrt(1 + m[1]![1]! - m[0]![0]! - m[2]![2]!) * 2;
    q = [(m[0]![1]! + m[1]![0]!) / s, s / 4, (m[1]![2]! + m[2]![1]!) / s, (m[0]![2]! - m[2]![0]!) / s];
  } else {
    const s = Math.sqrt(1 + m[2]![2]! - m[0]![0]! - m[1]![1]!) * 2;
    q = [(m[0]![2]! + m[2]![0]!) / s, (m[1]![2]! + m[2]![1]!) / s, s / 4, (m[1]![0]! - m[0]![1]!) / s];
  }
  const l = Math.hypot(...q);
  return q.map((v) => r5(v / l)) as Q;
}

export class Sculpt {
  readonly c: Ctx;
  readonly feature: number;
  readonly body: Json[] = [];
  constructor(readonly spec: string, readonly scale: number) {
    this.c = openCtx(spec, scale);
    this.feature = this.c.feature;
  }

  /** 📥️ Starts from the body already in the source and drops the solids `ids` (their replacements keep the same ids). */
  keep(drop: readonly string[]): void {
    for (const s of this.c.vol.body as Json[]) if (!drop.includes(s.id)) this.body.push(s);
  }

  frame(bone: string) {
    const f = this.c.state.frames[bone];
    if (f === undefined) throw new Error(`no bone ${bone}`);
    return f;
  }

  local(bone: string, p: V3): V3 {
    const f = this.frame(bone);
    const d = sub(p, f.o as V3);
    return [r5(dot(d, f.u as V3)), r5(dot(d, f.v as V3)), r5(dot(d, f.w as V3))];
  }

  dir(bone: string, d: V3): V3 {
    const f = this.frame(bone);
    return [dot(d, f.u as V3), dot(d, f.v as V3), dot(d, f.w as V3)];
  }

  add(s: Json): Json {
    if (this.body.some((q) => q.id === s.id)) throw new Error(`duplicate solid ${s.id}`);
    this.body.push(s);
    return s;
  }

  private base(id: string, kind: string, bone: string, look: Look): Json {
    return { id, kind, bone, material: look.material, role: look.role ?? "body", tone: look.tone ?? 0.5 };
  }

  ell(id: string, bone: string, at: V3, radii: V3, look: Look, axes?: [V3, V3]): Json {
    const s = this.base(id, "ellipsoid", bone, look);
    s.radii = radii.map(r5);
    s.transform = { translation: this.local(bone, at), ...(axes === undefined ? {} : { rotation: this.rot(bone, axes[0], axes[1]) }) };
    return this.add(s);
  }

  cap(id: string, bone: string, a: V3, b: V3, r0: number, r1: number | undefined, look: Look): Json {
    const s = this.base(id, "capsule", bone, look);
    s.p0 = this.local(bone, a);
    s.p1 = this.local(bone, b);
    s.r0 = r5(r0);
    s.r1 = r5(r1 ?? r0);
    return this.add(s);
  }

  /** 🪈️ A capped two-section tube between two world points (a segment of a bent limb), `lat` the lateral axis the first section axis follows. */
  tube(id: string, bone: string, a: V3, b: V3, r0: number, r1: number, look: Look): Json {
    const d = nrm(sub(b, a));
    const lat: V3 = [0, 0, 1];
    const p1 = nrm(sub(lat, d.map((v) => v * dot(lat, d)) as V3));
    const p2 = cross(d, p1);
    const ext = 0.4 * Math.min(r0, r1);
    const a2 = a.map((v, i) => v - d[i]! * ext) as V3;
    const b2 = b.map((v, i) => v + d[i]! * ext) as V3;
    return this.loft(id, bone, [{ c: a2, a: p1.map((v) => v * r0) as V3, b: p2.map((v) => v * r0) as V3 }, { c: b2, a: p1.map((v) => v * r1) as V3, b: p2.map((v) => v * r1) as V3 }], look);
  }

  rot(bone: string, ax: V3, ay: V3): Q {
    const x = nrm(this.dir(bone, ax));
    let y = this.dir(bone, ay);
    y = nrm(sub(y, x.map((v) => v * dot(y, x)) as V3));
    return quatOf(x, y, cross(x, y));
  }

  loft(id: string, bone: string, stations: readonly Station[], look: Look, caps = true): Json {
    const s = this.base(id, "loft", bone, look);
    s.sections = stations.map((st) => {
      const a = st.a;
      const b = st.b;
      const rx = len(a);
      const ry = len(b);
      return { at: this.local(bone, st.c), rx: r5(Math.max(rx, 1e-4)), ry: r5(Math.max(ry, 1e-4)), exponent: st.e ?? 2, rotation: this.rot(bone, a, b) };
    });
    s.capStart = caps;
    s.capEnd = caps;
    return this.add(s);
  }

  span(view: "elevation" | "front" | "plan", ids: readonly string[], along: "x" | "y", at: number): [number, number] | null {
    const set = new Set(ids);
    return drawnSpan(this.c, view, undefined, along, at, (s) => set.has(s.id));
  }

  /** 🏗️ Loft in vertical planes x = const: stations `xs`, vertical extent from the elevation shapes `elev`, lateral extent from the plan shapes `plan` (or `half` when given). */
  hullX(id: string, bone: string, o: { elev: readonly string[]; plan?: readonly string[]; xs: readonly number[]; look: Look; e?: number; half?: (x: number, plan: number) => number; vfix?: (x: number, lo: number, hi: number) => [number, number]; floor?: number; zc?: number; caps?: boolean }): Json {
    const floor = o.floor ?? 0.55 * this.feature;
    const st: Station[] = [];
    for (const x of o.xs) {
      let sp = this.span("elevation", o.elev, "x", x);
      if (sp === null) continue;
      if (o.vfix !== undefined) sp = o.vfix(x, sp[0], sp[1]);
      const pl = o.plan === undefined ? null : this.span("plan", o.plan, "x", x);
      const zh = pl === null ? floor : (pl[1] - pl[0]) / 2;
      const zc = pl === null ? (o.zc ?? 0) : (pl[0] + pl[1]) / 2;
      const rz = Math.max(floor, o.half === undefined ? zh : o.half(x, zh));
      st.push({ c: [x, (sp[0] + sp[1]) / 2, zc], a: [0, 0, rz], b: [0, Math.max(floor, (sp[1] - sp[0]) / 2), 0], e: o.e });
    }
    if (st.length < 2) throw new Error(`${id}: no stations`);
    return this.loft(id, bone, st, o.look, o.caps ?? true);
  }

  /** 🏗️ Loft in horizontal planes y = const: stations `ys`, extent in x from the elevation shapes, lateral extent from the front shapes (or plan). */
  hullY(id: string, bone: string, o: { elev: readonly string[]; front?: readonly string[]; ys: readonly number[]; look: Look; e?: number; floor?: number; zc?: number }): Json {
    const floor = o.floor ?? 0.55 * this.feature;
    const st: Station[] = [];
    for (const y of o.ys) {
      const sp = this.span("elevation", o.elev, "y", y);
      if (sp === null) continue;
      const fr = o.front === undefined ? null : this.span("front", o.front, "y", y);
      const zh = fr === null ? floor : (fr[1] - fr[0]) / 2;
      const zc = fr === null ? (o.zc ?? 0) : (fr[0] + fr[1]) / 2;
      st.push({ c: [(sp[0] + sp[1]) / 2, y, zc], a: [Math.max(floor, (sp[1] - sp[0]) / 2), 0, 0], b: [0, 0, Math.max(floor, zh)], e: o.e });
    }
    if (st.length < 2) throw new Error(`${id}: no stations`);
    return this.loft(id, bone, st, o.look);
  }

  save(): void {
    saveLists(this.c, { body: this.body });
  }

  /** 🌡️ Replaces every thermal solid of the given regions by own solids (built with `ell`/`cap`/`loft` into `this.body` first, then moved here by `takeThermal`). */
  takeThermal(regions: Record<string, string[]>, keep: (t: Json) => boolean = () => false): void {
    const th: Json[] = (this.c.vol.thermal as Json[]).filter((t) => !(t.region in regions) || keep(t));
    for (const [region, ids] of Object.entries(regions)) {
      for (const id of ids) {
        const i = this.body.findIndex((s) => s.id === id);
        if (i < 0) throw new Error(`no staged solid ${id}`);
        const [s] = this.body.splice(i, 1);
        th.push({ ...s, id: `th-${region}-${id}`, region });
      }
    }
    saveLists(this.c, { thermal: th });
  }
}
