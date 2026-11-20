/** 🦌️ Round 3 roe deer: hand-stated section tables placed in the world of the default pose (x ahead, y up, z lateral, near side z < 0) and bound to their bones.
 * Every row of every table is written by hand in the model files; this kit only places rows (bone-relative or world) and densifies a table by a Catmull-Rom pass so that few hand rows give a smooth organic surface.
 * @see ./r3-roe-deer-50.ts
 * @see ./r3-animals-roe-deer.md */
import { Fauna, type Look, type PP } from "./fauna-lib.ts";
import { type Json, type V3 } from "./kit-lib.ts";

export { type Look, type PP, type Json, type V3 };
export const SPEC = "🗑️generated/r3-roe-deer/🔣️.json";
export const OUT = "🗑️generated/r3-roe-deer";
/** Row of a limb table: distance along the bone, sagittal offset (bone normal, forward for a hanging bone), sagittal half extent, lateral half extent, lateral position (optional), exponent (optional). */
export type Row = readonly [s: number, v: number, rs: number, rl: number, z?: number, e?: number];
/** Row of a trunk table: x, centre height, half height, half width, exponent (optional), lateral centre (optional). */
export type XRow = readonly [x: number, y: number, rs: number, rl: number, e?: number, z?: number];

/** 🧵️ Catmull-Rom densification of a table: `k - 1` rows between every two hand rows, every column interpolated.
 * @example dense([[0, 0, 1, 1], [1, 0, 2, 2]], 2).length // 3 */
export function dense<T extends readonly number[]>(rows: readonly T[], k: number): number[][] {
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

export class Roe extends Fauna {
  readonly shapes: Record<string, string> = {};

  /** 📌️ World point of bone-relative coordinates: `s` along the bone, `v` along its sagittal normal, `z` lateral. */
  at(bone: string, s: number, v: number, z = 0): V3 {
    const f = this.frame(bone);
    return [f.o[0]! + s * f.u[0]! + v * f.v[0]!, f.o[1]! + s * f.u[1]! + v * f.v[1]!, z];
  }

  tag(solid: Json, shape?: string): Json {
    if (shape !== undefined) this.shapes[solid.id] = shape;
    return solid;
  }

  /** 🦵️ Loft along a bone from a hand table of rows in the frame of `frameBone` (default the bound bone), mirrored to side `sd` (-1 near, +1 far, 0 on the midline). */
  limb(id: string, bone: string, rows: readonly (readonly number[])[], look: Look, o: { sd?: -1 | 0 | 1; frame?: string; shape?: string; e?: number; floor?: number } = {}): Json {
    const fb = o.frame ?? bone;
    const f = this.frame(fb);
    const sd = o.sd ?? 0;
    const floor = o.floor ?? 0.55 * this.feature;
    const st = rows.map((r) => ({
      c: this.at(fb, r[0]!, r[1]!, sd * (r[4] ?? 0)),
      a: [0, 0, Math.max(floor, r[3]!)] as V3,
      b: [f.v[0]! * Math.max(floor, r[2]!), f.v[1]! * Math.max(floor, r[2]!), 0] as V3,
      e: r[5] ?? o.e ?? 2.2,
    }));
    return this.tag(this.loft(id, bone, st, look), o.shape);
  }

  /** 🐏️ Loft in planes x = const from a hand table (trunk, claw). `x0` shifts the table, `sd` mirrors its lateral centre. */
  hull(id: string, bone: string, rows: readonly (readonly number[])[], look: Look, o: { x0?: number; sd?: -1 | 0 | 1; shape?: string; e?: number; floor?: number } = {}): Json {
    const floor = o.floor ?? 0.55 * this.feature;
    const st = rows.map((r) => ({ c: [(o.x0 ?? 0) + r[0]!, r[1]!, (o.sd ?? 1) * (r[5] ?? 0)] as V3, a: [0, 0, Math.max(floor, r[3]!)] as V3, b: [0, Math.max(floor, r[2]!), 0] as V3, e: r[4] ?? o.e ?? 2.2 }));
    return this.tag(this.loft(id, bone, st, look), o.shape);
  }

  /** 🪢️ Path loft through world points (ribs, ears, tail, processes). */
  run(id: string, bone: string, pts: readonly PP[], look: Look, o: { lat?: V3; shape?: string; floor?: number } = {}): Json {
    return this.tag(this.path(id, bone, pts, look, { ...(o.lat === undefined ? {} : { lat: o.lat }), ...(o.floor === undefined ? {} : { floor: o.floor }) }), o.shape);
  }

  ball(id: string, bone: string, at: V3, r: V3, look: Look, dir: V3 = [1, 0, 0], shape?: string): Json {
    return this.tag(this.blob(id, bone, at, r, look, dir), shape);
  }

  /** 📦️ The finished list. */
  list(extra: (s: Json) => Json = (s) => s): Json[] {
    return this.body.map(extra);
  }
}

export const side = (sd: -1 | 1): string => (sd < 0 ? "near" : "far");
export const sx = (sd: -1 | 1): string => (sd < 0 ? "n" : "f");
