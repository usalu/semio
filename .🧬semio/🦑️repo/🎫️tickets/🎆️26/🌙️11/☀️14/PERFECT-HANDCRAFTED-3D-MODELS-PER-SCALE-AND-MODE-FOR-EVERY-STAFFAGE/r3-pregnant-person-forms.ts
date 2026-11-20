/** 🫃 The measured forms of the clothed trunk of `humans/pregnant-person` at 1:50, stated by hand as horizontal slices of the upright woman in her boots (x ahead of the hip joints, y above the ground, metres) and converted to the anatomical frames of the bones that carry them. Used by the model (`r3-pregnant-person-s50.ts`) and by the hand placement of the poses (`r3-pregnant-person-roots.ts`), so the palms land on the modelled cloth.
 *
 * Sources: ANSUR II female breadths, depths and girths (Gordon 2014), third-trimester abdominal girth about 1.0 to 1.05 m at the umbilicus and fundus at the xiphoid in week 36 (fundal height 36 cm), knit 8 mm on the trunk and 5 mm stretched over the abdomen (asset thermal layers).
 * @see ./r3-humans-pregnant-person.md */
import { RIG } from "./r3-pregnant-person-data.ts";

/** 📏 A horizontal slice: [y, back x, front x, half-width, exponent]. */
export type Slice = readonly [number, number, number, number, number];
/** 🍞 A cross slice of the abdomen: [x, bottom y, top y, half-width, exponent]. */
export type Cross = readonly [number, number, number, number, number];

const TORSO_O: [number, number] = [-RIG.lumboBack, RIG.hip + RIG.pelvisLen];
const TORSO_DIR = ((90 + RIG.torsoRel) * Math.PI) / 180;
const TU: [number, number] = [Math.cos(TORSO_DIR), Math.sin(TORSO_DIR)];
const TA: [number, number] = [TU[1], -TU[0]];
/** 🧭 A point of the upright woman in the torso frame: `a` up the trunk line from the lumbosacral joint, `f` ahead of it. */
export function torsoAF(x: number, y: number): [number, number] {
  const d = [x - TORSO_O[0], y - TORSO_O[1]];
  return [d[0]! * TU[0] + d[1]! * TU[1], d[0]! * TA[0] + d[1]! * TA[1]];
}
/** 🫃 The origin of the abdomen bone (on the trunk line, `RIG.bellyAt` above the lumbosacral joint, pointing straight ahead in the upright woman). */
export const BELLY_O: [number, number] = [TORSO_O[0] + TU[0] * RIG.bellyAt, TORSO_O[1] + TU[1] * RIG.bellyAt];

/** 🧶 The oversized knit sweater on the trunk: back over the shoulder blades and the lumbar hollow, front over the bust, the dropped shoulders and the shoulder slope to the crew neck. */
export const SWEATER: readonly Slice[] = [
  [0.98, -0.123, 0.125, 0.188, 2.4],
  [1.04, -0.116, 0.128, 0.184, 2.4],
  [1.1, -0.117, 0.13, 0.183, 2.4],
  [1.16, -0.127, 0.135, 0.185, 2.4],
  [1.21, -0.135, 0.15, 0.188, 2.3],
  [1.245, -0.139, 0.158, 0.192, 2.3],
  [1.28, -0.141, 0.147, 0.198, 2.4],
  [1.32, -0.139, 0.126, 0.204, 2.5],
  [1.355, -0.133, 0.103, 0.206, 2.6],
  [1.39, -0.121, 0.078, 0.198, 2.7],
  [1.415, -0.106, 0.056, 0.168, 2.6],
  [1.435, -0.094, 0.038, 0.122, 2.3],
  [1.452, -0.085, 0.025, 0.082, 2.1],
];
/** 🧶 The sweater below the lumbosacral joint, hanging over the seat: these slices are carried by the pelvis (they hang where the seat is in every pose). */
export const SWEATER_SKIRT: readonly Slice[] = [
  [0.842, -0.17, 0.102, 0.197, 2.4],
  [0.875, -0.17, 0.11, 0.199, 2.4],
  [0.925, -0.152, 0.118, 0.194, 2.4],
];
/** 🧷 The rib hem: the lowest skirt slice, tilted 12 degrees so that it rides up at the front under the abdomen (back 0.80 m, front 0.865 m). */
export const HEM_TILT = -12;

/** 🤰 The knit stretched over the abdomen as cross slices square to the abdomen bone: from inside the trunk to the apex 0.263 m ahead of the hip joints at 1.02 m, rounder and fuller in its lower half, the fundus tucked under the bust. */
export const BELLY: readonly Cross[] = [
  [0.1, 0.864, 1.208, 0.181, 2.3],
  [0.13, 0.869, 1.205, 0.176, 2.25],
  [0.16, 0.879, 1.19, 0.165, 2.15],
  [0.19, 0.893, 1.165, 0.147, 2.05],
  [0.215, 0.909, 1.135, 0.126, 2.0],
  [0.235, 0.928, 1.106, 0.1, 2.0],
  [0.25, 0.952, 1.08, 0.072, 2.0],
  [0.258, 0.974, 1.057, 0.047, 2.0],
  [0.263, 0.998, 1.036, 0.022, 2.0],
];

/** 🩳 The leggings over seat, hips and mons on the pelvis bone (upright, the pelvis line is the vertical through the hip joints). */
export const SEAT: readonly Slice[] = [
  [0.785, -0.1, 0.06, 0.15, 2.2],
  [0.82, -0.148, 0.075, 0.176, 2.3],
  [0.855, -0.158, 0.092, 0.184, 2.4],
  [0.89, -0.152, 0.1, 0.183, 2.4],
  [0.93, -0.13, 0.105, 0.172, 2.4],
  [0.97, -0.11, 0.105, 0.16, 2.4],
  [1.0, -0.098, 0.1, 0.152, 2.4],
];

/** 📐 A trunk slice as a torso station row: [a, f centre, half-width, half-depth, exponent]. */
export function torsoRow([y, b, f, hw, e]: Slice): [number, number, number, number, number] {
  const [a, fb] = torsoAF(b, y);
  const [, ff] = torsoAF(f, y);
  return [a, (fb + ff) / 2, hw, (ff - fb) / 2, e];
}
/** 📐 A seat slice as a pelvis station row: [a, f centre, half-width, half-depth, exponent]. */
export const pelvisRow = ([y, b, f, hw, e]: Slice): [number, number, number, number, number] => [y - RIG.hip, (b + f) / 2, hw, (f - b) / 2, e];
/** 📐 An abdomen cross slice as a belly station row: [a ahead, centre up, half-width, half-height, exponent]. */
export const bellyRow = ([x, lo, hi, hw, e]: Cross): [number, number, number, number, number] => [x - BELLY_O[0], (lo + hi) / 2 - BELLY_O[1], hw, (hi - lo) / 2, e];

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
/** 📏 The underside (-1) or top (+1) of the abdomen at `x` ahead and `l` to the left (upright woman). */
export function bellyY(x: number, l: number, side: 1 | -1): number {
  let i = 0;
  while (i + 2 < BELLY.length && BELLY[i + 1]![0] < x) i++;
  const a = BELLY[i]!;
  const b = BELLY[i + 1]!;
  const t = Math.min(1, Math.max(0, (x - a[0]) / (b[0] - a[0])));
  const m = (k: number): number => a[k]! + (b[k]! - a[k]!) * t;
  const [lo, hi, rw, e] = [m(1), m(2), m(3), m(4)];
  const k = Math.pow(Math.max(0, 1 - Math.pow(Math.min(1, Math.abs(l) / rw), e)), 1 / e);
  return (lo + hi) / 2 + (side * k * (hi - lo)) / 2;
}
