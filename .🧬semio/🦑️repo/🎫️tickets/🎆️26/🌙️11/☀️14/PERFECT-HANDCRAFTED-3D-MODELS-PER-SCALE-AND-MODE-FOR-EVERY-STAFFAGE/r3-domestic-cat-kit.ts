/** 🐾️ Round 3 domestic cat: the modelling bench. Hand-stated section tables are placed in the world of the default pose of the working asset (x ahead, y up, z lateral, near side z < 0, ground y = 0) and bound to their bones; `dense` only rounds the surface between the stated rows (Catmull-Rom), it invents no form.
 * @see ./r3-domestic-cat-work.ts
 * @see ./r3-animals-domestic-cat.md */
import { Fauna, type Look, type PP } from "./fauna-lib.ts";
import { type Json, type V3 } from "./kit-lib.ts";
import { poseState } from "./volumes-lib.ts";
import { OUT, SPEC, working } from "./r3-domestic-cat-work.ts";

export { OUT, SPEC, type Look, type PP, type Json, type V3 };
/** Row of a limb table: distance along the bone, offset along the bone normal, half extent along the normal, lateral half extent, lateral centre (optional), exponent (optional). */
export type Row = readonly number[];
export const FLOOR = 0.0012;

/** 🧵️ Catmull-Rom densification of a table: `k - 1` rows between every two hand rows, every column interpolated.
 * @example dense([[0, 0, 1, 1], [1, 0, 2, 2]], 2).length // 3 */
export function dense(rows: readonly Row[], k: number): number[][] {
  const n = rows.length;
  const width = Math.max(...rows.map((r) => r.length));
  const at = (i: number, c: number): number => rows[Math.max(0, Math.min(n - 1, i))]![c] ?? rows[0]![c] ?? NaN;
  const out: number[][] = [];
  for (let i = 0; i < n - 1; i++) {
    for (let j = 0; j < k; j++) {
      const t = j / k;
      out.push(Array.from({ length: width }, (_, c) => {
        const [p0, p1, p2, p3] = [at(i - 1, c), at(i, c), at(i + 1, c), at(i + 2, c)];
        if (Number.isNaN(p1) || Number.isNaN(p2)) return p1;
        return 0.5 * (2 * p1 + (p2 - p0) * t + (2 * p0 - 5 * p1 + 4 * p2 - p3) * t * t + (3 * p1 - p0 - 3 * p2 + p3) * t * t * t);
      }));
    }
  }
  out.push([...rows[n - 1]!]);
  return out;
}

export class Cat extends Fauna {
  constructor(scale: number) {
    super(SPEC, scale);
    const w = working();
    Object.assign(this.c, { raw: w, m: w, state: poseState(w, scale) });
  }

  bone(id: string): Json {
    return (this.c.raw.rig.bones as Json[]).find((b) => b.id === id)!;
  }

  /** 📌️ World point of bone-relative coordinates: `s` along the bone, `v` along its sagittal normal, `z` world lateral. */
  at(bone: string, s: number, v: number, z = 0): V3 {
    const f = this.frame(bone);
    return [f.o[0]! + s * f.u[0]! + v * f.v[0]!, f.o[1]! + s * f.u[1]! + v * f.v[1]!, z];
  }

  tipOf(bone: string): V3 {
    return this.at(bone, this.bone(bone).lengthM, 0, this.frame(bone).o[2]!);
  }

  /** 🦵️ Loft along a bone from a hand table of rows in the frame of `frame` (default the bound bone), mirrored to side `sd` (-1 near, +1 far, 0 midline). */
  limb(id: string, bone: string, rows: readonly Row[], look: Look, o: { sd?: -1 | 0 | 1; frame?: string; e?: number; floor?: number } = {}): Json {
    const fb = o.frame ?? bone;
    const f = this.frame(fb);
    const sd = o.sd ?? 0;
    const floor = o.floor ?? FLOOR;
    const st = rows.map((r) => ({
      c: this.at(fb, r[0]!, r[1]!, sd * (r[4] ?? 0)),
      a: [0, 0, Math.max(floor, r[3]!)] as V3,
      b: [f.v[0]! * Math.max(floor, r[2]!), f.v[1]! * Math.max(floor, r[2]!), 0] as V3,
      e: r[5] ?? o.e ?? 2.2,
    }));
    return this.loft(id, bone, st, look);
  }

  /** 🫁️ Loft in planes x = const from a hand table of rows [x, centre height, half height, half width, exponent?, lateral centre?]. */
  hull(id: string, bone: string, rows: readonly Row[], look: Look, o: { x0?: number; y0?: number; sd?: -1 | 0 | 1; e?: number; floor?: number } = {}): Json {
    const floor = o.floor ?? FLOOR;
    const st = rows.map((r) => ({ c: [(o.x0 ?? 0) + r[0]!, (o.y0 ?? 0) + r[1]!, (o.sd ?? 1) * (r[5] ?? 0)] as V3, a: [0, 0, Math.max(floor, r[3]!)] as V3, b: [0, Math.max(floor, r[2]!), 0] as V3, e: r[4] ?? o.e ?? 2.2 }));
    return this.loft(id, bone, st, look);
  }

  /** 🪢️ Path loft through world stations (tail, ears, ribs, processes). */
  run(id: string, bone: string, pts: readonly PP[], look: Look, o: { lat?: V3; floor?: number } = {}): Json {
    return this.path(id, bone, pts, look, { ...(o.lat === undefined ? {} : { lat: o.lat }), floor: o.floor ?? FLOOR });
  }

  /** 💊️ Round tube from `a` (radius `ra`) to `b` (radius `rb`) with a dome on each end, so that two tubes sharing a joint stay closed at every bend. */
  tubeDome(id: string, bone: string, a: V3, b: V3, ra: number, rb: number, look: Look, o: { flat?: number; n?: number } = {}): Json {
    const d: V3 = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
    const l = Math.hypot(...d) || 1;
    const t: V3 = [d[0] / l, d[1] / l, d[2] / l];
    const k = o.flat ?? 1;
    const pts: PP[] = [];
    for (const c of [1, 0.9, 0.62, 0.3]) pts.push({ p: [a[0] - t[0] * ra * c, a[1] - t[1] * ra * c, a[2] - t[2] * ra * c], rl: ra * k * Math.sqrt(Math.max(0.02, 1 - c * c)), rs: ra * Math.sqrt(Math.max(0.02, 1 - c * c)), e: 2 });
    const n = o.n ?? 2;
    for (let i = 0; i <= n; i++) { const q = i / n; pts.push({ p: [a[0] + d[0] * q, a[1] + d[1] * q, a[2] + d[2] * q], rl: (ra + (rb - ra) * q) * k, rs: ra + (rb - ra) * q, e: 2 }); }
    for (const c of [0.3, 0.62, 0.9, 1]) pts.push({ p: [b[0] + t[0] * rb * c, b[1] + t[1] * rb * c, b[2] + t[2] * rb * c], rl: rb * k * Math.sqrt(Math.max(0.02, 1 - c * c)), rs: rb * Math.sqrt(Math.max(0.02, 1 - c * c)), e: 2 });
    return this.path(id, bone, pts, look, { floor: 0.0004 });
  }

  /** 🔘️ Ellipsoid at a world point with its first axis along `dir`, radii [along, lateral, sagittal]; nothing is raised to a floor. */
  ball(id: string, bone: string, at: V3, r: V3, look: Look, dir: V3 = [1, 0, 0], lat: V3 = [0, 0, 1]): Json {
    const dl = Math.hypot(...dir) || 1;
    const d: V3 = [dir[0] / dl, dir[1] / dl, dir[2] / dl];
    const k = lat[0] * d[0] + lat[1] * d[1] + lat[2] * d[2];
    let l: V3 = [lat[0] - d[0] * k, lat[1] - d[1] * k, lat[2] - d[2] * k];
    if (Math.hypot(...l) < 0.2) l = [0 - d[0] * d[1], 1 - d[1] * d[1], 0 - d[2] * d[1]];
    return this.ell(id, bone, at, r, look, [d, l]);
  }
}

export const side = (sd: -1 | 1): string => (sd < 0 ? "near" : "far");
export const sx = (sd: -1 | 1): string => (sd < 0 ? "n" : "f");
export const SIDES = [-1, 1] as const;
