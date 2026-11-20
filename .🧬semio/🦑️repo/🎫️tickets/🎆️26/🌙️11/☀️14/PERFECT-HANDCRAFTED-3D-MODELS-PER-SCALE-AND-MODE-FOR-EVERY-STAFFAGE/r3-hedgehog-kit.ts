/** 🦔️ Round 3 hedgehog: the modelling bench. Hand-stated section tables are placed in the world of the default pose of the working asset (x ahead, y up, z lateral, near side z < 0, ground y = 0) and bound to their bones; nothing is derived from the drawings.
 * @see ./r3-hedgehog-work.ts
 * @see ./r3-animals-hedgehog.md */
import { Fauna, type Look, type PP } from "./fauna-lib.ts";
import { type Json, type V3 } from "./kit-lib.ts";
import { poseState } from "./volumes-lib.ts";
import { OUT, SPEC, working } from "./r3-hedgehog-work.ts";

export { OUT, SPEC, type Look, type PP, type Json, type V3 };
/** Row of a limb table: distance along the bone, offset along the bone normal, half extent along the normal, lateral half extent, lateral centre (optional), exponent (optional). */
export type Row = readonly number[];
export const FLOOR = 0.0008;
const unit = (a: V3): V3 => { const l = Math.hypot(...a) || 1; return [a[0] / l, a[1] / l, a[2] / l]; };
const cross = (a: V3, b: V3): V3 => [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];

export class Hog extends Fauna {
  constructor(scale: number) {
    super(SPEC, scale);
    const w = working();
    Object.assign(this.c, { raw: w, m: w, state: poseState(w, scale) });
  }

  /** 📌️ World point of bone-relative coordinates: `s` along the bone, `v` along its sagittal normal, `z` world lateral. */
  at(bone: string, s: number, v: number, z = 0): V3 {
    const f = this.frame(bone);
    return [f.o[0]! + s * f.u[0]! + v * f.v[0]!, f.o[1]! + s * f.u[1]! + v * f.v[1]!, z];
  }

  /** 🦵️ Loft along a bone from a hand table of rows in the frame of `frame` (default the bound bone), mirrored to side `sd` (-1 near, +1 far, 0 midline). */
  limb(id: string, bone: string, rows: readonly Row[], look: Look, o: { sd?: -1 | 0 | 1; frame?: string; e?: number } = {}): Json {
    const fb = o.frame ?? bone;
    const f = this.frame(fb);
    const sd = o.sd ?? 0;
    const st = rows.map((r) => ({
      c: this.at(fb, r[0]!, r[1]!, sd * (r[4] ?? 0)),
      a: [0, 0, Math.max(FLOOR, r[3]!)] as V3,
      b: [f.v[0]! * Math.max(FLOOR, r[2]!), f.v[1]! * Math.max(FLOOR, r[2]!), 0] as V3,
      e: r[5] ?? o.e ?? 2.2,
    }));
    return this.loft(id, bone, st, look);
  }

  /** 🫁️ Loft in planes x = const from a hand table of rows [x, centre height, half height, half width, exponent?]. */
  hull(id: string, bone: string, rows: readonly Row[], look: Look, o: { e?: number } = {}): Json {
    const st = rows.map((r) => ({ c: [r[0]!, r[1]!, 0] as V3, a: [0, 0, Math.max(FLOOR, r[3]!)] as V3, b: [0, Math.max(FLOOR, r[2]!), 0] as V3, e: r[4] ?? o.e ?? 2.2 }));
    return this.loft(id, bone, st, look);
  }

  /** 🪢️ Path loft through world stations. */
  run(id: string, bone: string, pts: readonly PP[], look: Look, o: { lat?: V3; floor?: number } = {}): Json {
    return this.path(id, bone, pts, look, { ...(o.lat === undefined ? {} : { lat: o.lat }), floor: o.floor ?? FLOOR });
  }

  /** 🔘️ Ellipsoid at a world point with its first axis along `dir`, radii [along, lateral, sagittal]. */
  ball(id: string, bone: string, at: V3, r: V3, look: Look, dir: V3 = [1, 0, 0], lat: V3 = [0, 0, 1]): Json {
    const d = unit(dir);
    const k = lat[0] * d[0] + lat[1] * d[1] + lat[2] * d[2];
    let l: V3 = [lat[0] - d[0] * k, lat[1] - d[1] * k, lat[2] - d[2] * k];
    if (Math.hypot(...l) < 0.2) l = [0 - d[0] * d[1], 1 - d[1] * d[1], 0 - d[2] * d[1]];
    return this.ell(id, bone, at, r, look, [d, l]);
  }

  /** 🥣️ Hemispherical bowl of radius `r` about the origin of `bone` (moved `dz` sideways), its pole along the bone: circular sections from the rim plane through the centre up to the pole, as many as `polar` states (degrees from the pole, descending to 0). */
  bowl(id: string, bone: string, r: number, polar: readonly number[], look: Look, dz = 0): Json {
    const f = this.frame(bone);
    const u: V3 = [f.u[0]!, f.u[1]!, f.u[2]!];
    const v: V3 = [f.v[0]!, f.v[1]!, f.v[2]!];
    const st = polar.map((deg) => {
      const t = (deg * Math.PI) / 180;
      const rr = Math.max(FLOOR, r * Math.sin(t));
      const h = r * Math.cos(t);
      return { c: [f.o[0]! + u[0] * h, f.o[1]! + u[1] * h, f.o[2]! + dz] as V3, a: [0, 0, rr] as V3, b: [v[0] * rr, v[1] * rr, 0] as V3, e: 2 };
    });
    return this.loft(id, bone, st, look);
  }

  /** 🌵️ Spine tuft: a cone from `base` (world) along `dir` with base radius `r0`, tip radius `r1` and length `len`. */
  cone(id: string, bone: string, base: V3, dir: V3, len: number, r0: number, r1: number, look: Look): Json {
    const d = unit(dir);
    let l = cross(d, [0, 0, 1]);
    if (Math.hypot(...l) < 0.2) l = cross(d, [0, 1, 0]);
    l = unit(l);
    const s = cross(d, l);
    const st = [0, 0.55, 1].map((t) => { const rr = r0 + (r1 - r0) * t; return { c: [base[0] + d[0] * len * t, base[1] + d[1] * len * t, base[2] + d[2] * len * t] as V3, a: [l[0] * rr, l[1] * rr, l[2] * rr] as V3, b: [s[0] * rr, s[1] * rr, s[2] * rr] as V3, e: 2 }; });
    return this.loft(id, bone, st, look);
  }
}

export const SIDES = [-1, 1] as const;
export const sx = (sd: -1 | 1): string => (sd < 0 ? "n" : "f");
