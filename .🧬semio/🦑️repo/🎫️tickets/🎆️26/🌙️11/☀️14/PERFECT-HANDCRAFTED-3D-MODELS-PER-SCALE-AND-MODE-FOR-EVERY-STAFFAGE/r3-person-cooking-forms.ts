/** 🧥 The measured forms of the clothed trunk of `humans/person-cooking` at 1:50, stated by hand as horizontal slices of the upright woman in her sneakers (x ahead of the hip joints, y above the ground, metres) and converted to the anatomical frames of the bones that carry them.
 *
 * Sources: ANSUR II female breadths, depths and girths (Gordon 2014) for a slim woman of 1.69 m and 58 kg (chest 0.27 x 0.22 m at the bust, waist 0.24 x 0.17 m, hips 0.345 x 0.21 m, bideltoid 0.415 m), a relaxed hooded fleece sweatshirt with 15 to 25 mm ease that hangs from bust and shoulder blades and is gathered by its rib hem at the hips, slim jeans 3 mm over the skin.
 * @see ./r3-humans-person-cooking.md */
import { RIG } from "./r3-person-cooking-data.ts";

/** 📏 A horizontal slice: [y, back x, front x, half-width, exponent]. */
export type Slice = readonly [number, number, number, number, number];

const TORSO_O: [number, number] = [-RIG.lumboBack, RIG.hip + RIG.pelvisLen];
const TORSO_DIR = ((90 + RIG.torsoRel) * Math.PI) / 180;
const TU: [number, number] = [Math.cos(TORSO_DIR), Math.sin(TORSO_DIR)];
const TA: [number, number] = [TU[1], -TU[0]];
/** 🧭 A point of the upright woman in the torso frame: `a` up the trunk line from the lumbosacral joint, `f` ahead of it. */
export function torsoAF(x: number, y: number): [number, number] {
  const d = [x - TORSO_O[0], y - TORSO_O[1]];
  return [d[0]! * TU[0] + d[1]! * TU[1], d[0]! * TA[0] + d[1]! * TA[1]];
}

/** 🧥 The hooded sweatshirt on the trunk: back over the shoulder blades bridging the lumbar hollow, front over the bust and hanging free below it, the shoulder slope to the neckline. */
export const HOODIE: readonly Slice[] = [
  [0.99, -0.124, 0.106, 0.186, 2.5],
  [1.04, -0.12, 0.106, 0.185, 2.5],
  [1.09, -0.119, 0.111, 0.184, 2.5],
  [1.14, -0.126, 0.119, 0.186, 2.5],
  [1.19, -0.133, 0.128, 0.188, 2.45],
  [1.24, -0.138, 0.135, 0.191, 2.4],
  [1.285, -0.14, 0.124, 0.194, 2.45],
  [1.33, -0.139, 0.104, 0.196, 2.55],
  [1.37, -0.132, 0.082, 0.192, 2.6],
  [1.4, -0.122, 0.068, 0.176, 2.55],
  [1.425, -0.11, 0.054, 0.148, 2.45],
  [1.445, -0.099, 0.04, 0.116, 2.3],
  [1.46, -0.09, 0.026, 0.084, 2.15],
];
/** 🧥 The sweatshirt below the lumbosacral joint, over the seat down to the rib hem: carried by the pelvis of the default pose. */
export const HOODIE_SKIRT: readonly Slice[] = [
  [0.85, -0.134, 0.092, 0.18, 2.45],
  [0.89, -0.14, 0.1, 0.186, 2.5],
  [0.94, -0.133, 0.104, 0.187, 2.5],
];
/** 👖 The jeans over seat, hips and fly on the pelvis bone (upright, the pelvis line is the vertical through the hip joints). */
export const SEAT: readonly Slice[] = [
  [0.778, -0.086, 0.052, 0.138, 2.2],
  [0.81, -0.118, 0.068, 0.164, 2.3],
  [0.85, -0.13, 0.08, 0.174, 2.35],
  [0.89, -0.129, 0.087, 0.176, 2.4],
  [0.93, -0.116, 0.09, 0.169, 2.4],
  [0.97, -0.102, 0.088, 0.154, 2.4],
  [1.005, -0.094, 0.085, 0.142, 2.4],
];

/** 📐 A trunk slice as a torso station row: [a, f centre, half-width, half-depth, exponent]. */
export function torsoRow([y, b, f, hw, e]: Slice): [number, number, number, number, number] {
  const [a, fb] = torsoAF(b, y);
  const [, ff] = torsoAF(f, y);
  return [a, (fb + ff) / 2, hw, (ff - fb) / 2, e];
}
/** 📐 A seat slice as a pelvis station row: [a, f centre, half-width, half-depth, exponent]. */
export const pelvisRow = ([y, b, f, hw, e]: Slice): [number, number, number, number, number] => [y - RIG.hip, (b + f) / 2, hw, (f - b) / 2, e];

/** 📏 The surface of a slice table at the height `y` and the lateral position `l`: x of the front (+1) or of the back (-1). */
export function sliceX(table: readonly Slice[], y: number, l: number, side: 1 | -1): number {
  let i = 0;
  while (i + 2 < table.length && table[i + 1]![0] < y) i++;
  const a = table[i]!;
  const b = table[i + 1]!;
  const t = Math.min(1, Math.max(0, (y - a[0]) / (b[0] - a[0])));
  const m = (k: number): number => a[k]! + (b[k]! - a[k]!) * t;
  const [back, front, rw, e] = [m(1), m(2), m(3), m(4)];
  const k = Math.pow(Math.max(0, 1 - Math.pow(Math.min(1, Math.abs(l) / rw), e)), 1 / e);
  return (back + front) / 2 + (side * k * (front - back)) / 2;
}
