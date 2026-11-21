/** 🦴️ Round 5 goat: the thirteen rib pairs of a goat as they lie, stated by hand in world metres of the standing pose. Each rib leaves its thoracic vertebra (T1 above the point of the shoulder, T13 at the thoracolumbar joint), bows out to the ribcage wall, runs caudoventrally (the sweep grows from the first to the last rib) and turns forward in its costal cartilage: ribs 1 to 8 reach the sternum, ribs 9 to 13 end in the costal arch that runs back up from the xiphoid. Replaces the round-3 hoops that stood upright like barrel rings.
 * Reference: Nickel/Schummer/Seiferle, small ruminants: 13 thoracic vertebrae, 8 sternal and 5 asternal ribs, ribcage 0.33 m long at 0.76 m withers.
 * @see ./r3-animals-goat.md */
import { type Bench } from "./r3-goat-kit.ts";
import { type Json, type V3 } from "./volumes-lib.ts";

const lerp = (t: readonly (readonly [number, number])[], x: number): number => {
  if (x <= t[0]![0]) return t[0]![1];
  for (let i = 1; i < t.length; i++) if (x <= t[i]![0]) { const [x0, y0] = t[i - 1]!; const [x1, y1] = t[i]!; return y0 + ((x - x0) / (x1 - x0)) * (y1 - y0); }
  return t[t.length - 1]![1];
};
/** Height of the vertebral bodies along the thoracic column (world x, y). */
const SPINE: readonly (readonly [number, number])[] = [[0.37, 0.712], [0.5, 0.705], [0.635, 0.694], [0.715, 0.664]];
/** Half width of the ribcage wall inside the coat and the thoracic muscles (world x, |z|). */
const WALL: readonly (readonly [number, number])[] = [[0.38, 0.122], [0.44, 0.12], [0.5, 0.11], [0.58, 0.098], [0.65, 0.086], [0.71, 0.072], [0.75, 0.06]];
/** Sternum from the manubrium to the xiphoid (world x, y). */
export const STERNUM_R: readonly (readonly [number, number])[] = [[0.5, 0.437], [0.58, 0.437], [0.66, 0.45], [0.72, 0.478], [0.757, 0.515]];

/** 🦴️ World points of rib `i` (0 = first rib) on side `sg` (-1 near, +1 far): head at the vertebra, angle, widest bow, lowest point of the bony rib, end of the costal cartilage. */
export function ribPoints(i: number, sg: number): V3[] {
  const xd = 0.705 - i * 0.0255;
  const yd = lerp(SPINE, xd) - 0.004;
  const kd = 0.015 + 0.0105 * i;
  const w = (x: number): number => lerp(WALL, x);
  const p = (x: number, y: number, z: number): V3 => [x, y, sg * z];
  const head = p(xd, yd, 0.012);
  const angle = p(xd - 0.006, yd - 0.032, 0.68 * w(xd));
  const bow = p(xd - 0.45 * kd, 0.6, w(xd - 0.45 * kd));
  if (i < 8) {
    const xs = 0.745 - i * 0.032;
    const low = p(xd - kd, 0.49, 0.8 * w(xd - kd));
    return [head, angle, bow, low, p(xs, lerp([...STERNUM_R], xs) + 0.006, 0.018)];
  }
  const low = p(xd - kd, 0.505 + (i - 8) * 0.032, 0.85 * w(xd - kd));
  return [head, angle, bow, low, p(xd - kd - 0.045, 0.47 + (i - 8) * 0.045, 0.72 * w(xd - kd - 0.045))];
}

/** 🦴️ The chosen rib pairs as chains of capsules (bony rib `r`, cartilage `rc`), the costal arch from the xiphoid along the ends of the asternal ribs when rib 9 or later is chosen, and the sternum. */
export function ribCage(B: Bench, ribs: readonly number[], r: number, rc: number, extra: Json, sternum = true): void {
  B.list("structure", extra);
  const look = { material: "bone", tone: 0.9 };
  for (const [side, sg] of [["n", -1], ["f", 1]] as const) {
    for (const i of ribs) {
      const q = ribPoints(i, sg);
      for (let k = 0; k < q.length - 1; k++) B.cap(`rib-${i + 1}-${side}-${k + 1}`, "thorax", q[k]!, q[k + 1]!, k === q.length - 2 ? rc : r, k === q.length - 2 ? rc : r, look);
    }
    const ends = ribs.filter((i) => i >= 8).map((i) => ribPoints(i, sg).at(-1)!);
    if (ends.length > 0) {
      const arch: V3[] = [[0.52, 0.44, sg * 0.022], ...ends];
      for (let k = 0; k < arch.length - 1; k++) B.cap(`costal-arch-${side}-${k + 1}`, "thorax", arch[k]!, arch[k + 1]!, rc, rc, look);
    }
  }
  if (sternum) for (let k = 0; k < STERNUM_R.length - 1; k++) {
    const [a, b] = [STERNUM_R[k]!, STERNUM_R[k + 1]!];
    B.cap(`sternum-${k + 1}`, "thorax", [a[0], a[1], 0], [b[0], b[1], 0], Math.max(r * 1.6, 0.011), Math.max(r * 1.6, 0.011), look);
  }
}
