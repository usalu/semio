/** 🕊️ Round 3 pigeon: the sculpting bench. World metres in `presentation.defaultPose` (x ahead, y up, z left; the near side of the elevation is z < 0), converted to the rest frame of each solid's bone by `Sculpt`. Every section of every solid is stated by hand in the model scripts; this file only carries the frames, the body-surface lookup that lets a folded wing lie on the flank, and the bookkeeping of the three lists.
 * @see ./r3-pigeon-model.ts
 * @see ./author-29-lib.ts */
import { Sculpt, type Look, type Station } from "./author-29-lib.ts";
import { saveLists, type Json, type V3 } from "./kit-lib.ts";

export type { Look, Station, Json, V3 };
export type Side = -1 | 1;
export type TrunkStation = readonly [s: number, dorsal: number, ventral: number, half: number];
export type Draw = { id: string; bone: string; role: string; z: number; tone: number; region?: string; solids: string[]; views?: string[]; filled?: boolean };

export const add = (a: V3, b: V3): V3 => [a[0] + b[0], a[1] + b[1], a[2] + b[2]];
export const sub = (a: V3, b: V3): V3 => [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
export const mul = (a: V3, k: number): V3 => [a[0] * k, a[1] * k, a[2] * k];
export const mix = (a: V3, b: V3, t: number): V3 => [a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t, a[2] + (b[2] - a[2]) * t];
export const unit = (a: V3): V3 => { const l = Math.hypot(...a) || 1; return [a[0] / l, a[1] / l, a[2] / l]; };
export const cross = (a: V3, b: V3): V3 => [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
export const dot = (a: V3, b: V3): number => a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
export const lerp = (a: number, b: number, t: number): number => a + (b - a) * t;
export const sideName = (sd: Side): string => (sd < 0 ? "near" : "far");
export const sideBone = (sd: Side): string => (sd < 0 ? "n" : "f");

/** 🪢️ Rows of hand-stated numbers (stations of a loft) with `n - 1` rows inserted between each pair on the Catmull-Rom curve through them: the stated sections stay exactly, the surface between them bends instead of breaking. */
export function smooth<T extends readonly number[]>(rows: readonly T[], n: number): T[] {
  const out: T[] = [];
  for (let i = 0; i + 1 < rows.length; i++) {
    const [p0, p1, p2, p3] = [rows[Math.max(0, i - 1)]!, rows[i]!, rows[i + 1]!, rows[Math.min(rows.length - 1, i + 2)]!];
    for (let k = 0; k < n; k++) {
      const t = k / n;
      out.push(p1.map((_, c) => 0.5 * (2 * p1[c]! + (p2[c]! - p0[c]!) * t + (2 * p0[c]! - 5 * p1[c]! + 4 * p2[c]! - p3[c]!) * t * t + (3 * p1[c]! - p0[c]! - 3 * p2[c]! + p3[c]!) * t * t * t)) as unknown as T);
    }
  }
  out.push(rows[rows.length - 1]!);
  return out;
}

/** 🧵️ A station of a path loft: centre, half extent `rl` along the lateral reference and `rs` across it, exponent, roll about the path (rad). */
export type PP = { p: V3; rl: number; rs: number; e?: number; roll?: number };

export class Bench extends Sculpt {
  readonly structure: Json[] = [];
  readonly thermal: Json[] = [];
  readonly draws: Draw[] = [];
  /** 📏️ Axis of the trunk envelope: from the tail base along the vertebral column to the lower neck. */
  readonly tailBase: V3;
  readonly axis: V3;
  readonly dorsal: V3;
  trunkStations: readonly TrunkStation[] = [];
  trunkExponent = 2.1;

  constructor(scale: number) {
    super("animals/pigeon", scale);
    this.tailBase = this.J("tail");
    const neck = this.J("neck-1");
    this.axis = unit(sub(neck, this.tailBase));
    this.dorsal = [-this.axis[1], this.axis[0], 0];
  }

  /** 📍️ Joint (bone origin) in the world, optionally moved along the bone (`s`), its normal (`t`) and laterally (`w`). */
  J(bone: string, s = 0, t = 0, w = 0): V3 {
    const f = this.frame(bone);
    return [0, 1, 2].map((k) => (f.o as V3)[k]! + s * (f.u as V3)[k]! + t * (f.v as V3)[k]! + w * (f.w as V3)[k]!) as V3;
  }

  tip(bone: string): V3 {
    return this.J(bone, (this.c.m.rig.bones as Json[]).find((b) => b.id === bone)!.lengthM);
  }

  /** 🧭️ Point of the trunk envelope frame: `s` along the column from the tail base, `t` towards the back, `z` lateral. */
  T(s: number, t: number, z = 0): V3 {
    return [this.tailBase[0] + s * this.axis[0] + t * this.dorsal[0], this.tailBase[1] + s * this.axis[1] + t * this.dorsal[1], z];
  }

  /** 🫱️ Half width of the trunk envelope at the world point (x, y): where the flank is, so that wings and thighs can lie on it; 0 outside the trunk. */
  flank(x: number, y: number): number {
    const d: V3 = [x - this.tailBase[0], y - this.tailBase[1], 0];
    const s = dot(d, this.axis);
    const t = dot(d, this.dorsal);
    const st = this.trunkStations;
    if (st.length < 2 || s <= st[0]![0] || s >= st[st.length - 1]![0]) return 0;
    const i = st.findIndex((q) => q[0] >= s);
    const [a, b] = [st[i - 1]!, st[i]!];
    const k = (s - a[0]) / (b[0] - a[0]);
    const [td, tv, half] = [lerp(a[1], b[1], k), lerp(a[2], b[2], k), lerp(a[3], b[3], k)];
    const q = Math.abs((t - (td + tv) / 2) / ((td - tv) / 2));
    return q >= 1 ? 0 : half * Math.pow(1 - Math.pow(q, this.trunkExponent), 1 / this.trunkExponent);
  }

  /** 🧶️ Loft along a path of hand-stated stations: every section is perpendicular to the local path tangent, `rl` along the lateral reference `lat` (default world z), `rs` across both. */
  path(id: string, bone: string, pts: readonly PP[], look: Look, lat: V3 = [0, 0, 1], caps = true): Json {
    const st: Station[] = pts.map((q, i) => {
      const t = unit(sub(pts[Math.min(pts.length - 1, i + 1)]!.p, pts[Math.max(0, i - 1)]!.p));
      let l = unit(sub(lat, mul(t, dot(lat, t))));
      let s = cross(t, l);
      if (q.roll) {
        const [c, n] = [Math.cos(q.roll), Math.sin(q.roll)];
        [l, s] = [add(mul(l, c), mul(s, n)), add(mul(s, c), mul(l, -n))];
      }
      return { c: q.p, a: mul(l, q.rl), b: mul(s, q.rs), e: q.e };
    });
    return this.loft(id, bone, st, look, caps);
  }

  /** 🪞️ Moves the solids staged since `from` out of the body list into the structure list as `part` of `member`. */
  toStructure(from: number, member: (s: Json) => string, part: "member" | "joint" | "support" | "mass" = "member", extra: Json = {}): void {
    for (const s of this.body.splice(from)) this.structure.push({ ...s, member: member(s), part, ...extra });
  }

  /** 🌡️ Thermal reference to a body solid. */
  th(region: string, ids: readonly string[]): void {
    for (const id of ids) {
      if (!this.body.some((s) => s.id === id)) throw new Error(`thermal: no body solid ${id}`);
      this.thermal.push({ id: `th-${id}`, of: id, region });
    }
  }

  /** 💨️ Moves the solids staged since `from` into the thermal list as own solids of `region` (emitter outlets carry their tags). */
  toThermal(from: number, region: string, tags?: string[]): void {
    for (const s of this.body.splice(from)) this.thermal.push({ ...s, region, ...(tags === undefined ? {} : { tags }) });
  }

  draw(d: Draw): void {
    for (const id of d.solids) if (!this.body.some((s) => s.id === id)) throw new Error(`draw ${d.id}: no body solid ${id}`);
    this.draws.push(d);
  }

  finish(decisions: Json[]): void {
    saveLists(this.c, { body: this.body, thermal: this.thermal, structure: this.structure, decisions });
  }
}
