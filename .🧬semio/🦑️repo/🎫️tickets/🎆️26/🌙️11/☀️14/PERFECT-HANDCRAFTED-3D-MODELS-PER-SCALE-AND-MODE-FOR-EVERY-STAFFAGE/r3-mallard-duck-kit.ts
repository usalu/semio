/** 🦆️ Round 3 mallard: the sculpting bench. World metres in `presentation.defaultPose` (x ahead, y up, z left; the near side of the elevation is z < 0), converted to the rest frame of each solid's bone by `Sculpt`. The rig is the one of the trial document (`🗑️generated/r3-mallard-duck/🔣️.json`) when it exists, so that bodies can be modelled on corrected roots before the catalog file is written. Every section of every solid is stated by hand in the model scripts; this file only carries the frames, the body-surface lookup that lets a folded wing lie on the flank, and the bookkeeping of the three lists.
 * @see ./r3-mallard-duck-model.ts
 * @see ./author-29-lib.ts */
import { existsSync, readFileSync } from "node:fs";
import { Sculpt, type Look, type Station } from "./author-29-lib.ts";
import { saveLists, type Ctx, type Json, type V3 } from "./kit-lib.ts";
import { entryOf, featureM, loadSource, merged, poseState, sourcePath } from "./volumes-lib.ts";

export type { Look, Station, Json, V3 };
export type Side = -1 | 1;
export type HullStation = readonly [s: number, dorsal: number, ventral: number, half: number];

export const SPEC = "animals/mallard-duck";
export const CANDIDATE = decodeURIComponent(new URL("./🗑️generated/r3-mallard-duck/🔣️.json", import.meta.url).pathname);
export const add = (a: V3, b: V3): V3 => [a[0] + b[0], a[1] + b[1], a[2] + b[2]];
export const sub = (a: V3, b: V3): V3 => [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
export const mul = (a: V3, k: number): V3 => [a[0] * k, a[1] * k, a[2] * k];
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

/** 🧵️ A station of a path loft: centre, half extent `rl` along the lateral reference and `rs` across it, exponent. */
export type PP = { p: V3; rl: number; rs: number; e?: number };

/** 📂️ The asset the bench models on: the trial document when it exists, else the catalog asset; the authoring source merged over it. */
export function openDuck(scale: number): Ctx {
  const { entry, asset: catalog } = entryOf(SPEC);
  const raw: Json = existsSync(CANDIDATE) ? JSON.parse(readFileSync(CANDIDATE, "utf8")).asset : catalog;
  const source = loadSource(entry)!;
  const m = merged(raw, source);
  return { spec: SPEC, raw, source, m, scale, vol: source.volumes[String(scale)], feature: featureM(scale, raw), state: poseState(m, scale), file: sourcePath(entry), seeded: false };
}

export class Bench extends Sculpt {
  readonly structure: Json[] = [];
  readonly thermal: Json[] = [];
  /** 📏️ Trunk frame in the world: origin at the hips, `axis` along the column to the neck base, `dorsal` towards the back. */
  readonly hip: V3;
  readonly axis: V3;
  readonly dorsal: V3;
  hull: readonly HullStation[] = [];
  hullExponent = 2.2;

  constructor(scale: number) {
    super(SPEC, scale);
    Object.assign(this, { c: openDuck(scale) });
    const f = this.frame("trunk");
    this.hip = [(f.o as V3)[0], (f.o as V3)[1], 0];
    this.axis = f.u as V3;
    this.dorsal = f.v as V3;
  }

  /** 📍️ Joint (bone origin) in the world, optionally moved along the bone (`s`), its normal (`t`) and laterally (`w`). */
  J(bone: string, s = 0, t = 0, w = 0): V3 {
    const f = this.frame(bone);
    return [0, 1, 2].map((k) => (f.o as V3)[k]! + s * (f.u as V3)[k]! + t * (f.v as V3)[k]! + w * (f.w as V3)[k]!) as V3;
  }

  /** 🧭️ Point of the trunk frame: `s` along the column from the hips, `t` towards the back, `z` lateral. */
  T(s: number, t: number, z = 0): V3 {
    return [this.hip[0] + s * this.axis[0] + t * this.dorsal[0], this.hip[1] + s * this.axis[1] + t * this.dorsal[1], z];
  }

  /** 🫱️ Half width of the plumage hull at the trunk point (s, t): where the flank is, so that wings and thighs can lie on it; 0 outside the hull. */
  flank(s: number, t: number): number {
    const st = this.hull;
    if (st.length < 2 || s <= st[0]![0] || s >= st[st.length - 1]![0]) return 0;
    const i = st.findIndex((q) => q[0] >= s);
    const [a, b] = [st[i - 1]!, st[i]!];
    const k = (s - a[0]) / (b[0] - a[0]);
    const [td, tv, half] = [lerp(a[1], b[1], k), lerp(a[2], b[2], k), lerp(a[3], b[3], k)];
    const q = Math.abs((t - (td + tv) / 2) / ((td - tv) / 2));
    return q >= 1 ? 0 : half * Math.pow(1 - Math.pow(q, this.hullExponent), 1 / this.hullExponent);
  }

  /** 🥚️ Loft of hull stations between two indices (the whole hull, or one of its tone zones). */
  hullLoft(id: string, st: readonly HullStation[], look: Look, shrink = 0): Json {
    return this.loft(id, "trunk", st.map(([s, td, tv, half]): Station => ({ c: this.T(s, (td + tv) / 2), a: [0, 0, half - shrink], b: mul(this.dorsal, (td - tv) / 2 - shrink), e: this.hullExponent })), look);
  }

  /** 🧶️ Loft along a path of hand-stated stations: every section is perpendicular to the local path tangent, `rl` along the lateral reference `lat` (default world z), `rs` across both. */
  path(id: string, bone: string, pts: readonly PP[], look: Look, lat: V3 = [0, 0, 1], caps = true): Json {
    const st: Station[] = pts.map((q, i) => {
      const t = unit(sub(pts[Math.min(pts.length - 1, i + 1)]!.p, pts[Math.max(0, i - 1)]!.p));
      const l = unit(sub(lat, mul(t, dot(lat, t))));
      return { c: q.p, a: mul(l, q.rl), b: mul(cross(t, l), q.rs), e: q.e };
    });
    return this.loft(id, bone, st, look, caps);
  }

  /** 🪞️ Moves the solids staged since `from` out of the body list into the structure list as `part` of `member`. */
  toStructure(from: number, member: (s: Json) => string, part: "member" | "joint" | "support" | "mass" = "member"): void {
    for (const s of this.body.splice(from)) this.structure.push({ ...s, member: member(s), part });
  }

  /** 💨️ Moves the solids staged since `from` into the thermal list as own solids of `region` (emitter outlets carry their tags). */
  toThermal(from: number, region: string, tags?: string[]): void {
    for (const s of this.body.splice(from)) this.thermal.push({ ...s, region, ...(tags === undefined ? {} : { tags }) });
  }

  finish(decisions: Json[]): void {
    saveLists(this.c, { body: this.body, thermal: this.thermal, structure: this.structure, decisions });
  }
}
