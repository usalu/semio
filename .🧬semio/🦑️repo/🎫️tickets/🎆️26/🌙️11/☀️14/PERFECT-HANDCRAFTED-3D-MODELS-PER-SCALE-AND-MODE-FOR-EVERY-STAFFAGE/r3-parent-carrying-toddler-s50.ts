/** 🤱 The 1:50 model of `humans/parent-carrying-toddler`, stated section by section: a parent of 1.70 m and 66 kg (1.73 m in 22 mm boot heels and a beanie) in a mid-calf wool coat with a knit scarf, jeans, leather gloves and ankle boots, carrying a 24-month toddler (0.865 m, 12.2 kg) in a padded overall, knit hat, mittens and rubber boots on the near (right) forearm; the toddler faces ahead with its back against the parent's chest, legs over the forearm, mittens on its thighs.
 *
 * Figure coordinates (`H`, `X`, `L`): x ahead, y up, z to the left, the default pose `standing-carry`. Local stations (`st`): u along the bone, v its sagittal normal, w lateral.
 * @see ./r3-parent-carrying-toddler-kit.ts
 * @see ./r3-humans-parent-carrying-toddler.md */
import type { Json } from "./volumes-lib.ts";
import { ASSET, FRAMES, H, L, X, aboutU, aboutV, aboutW, box, boneDeg, capsule, compose, dense, dome, ellipsoid, extrude, loft, mirrored, part, revolve, st, thOf, toRight, tube, type St, type V3 } from "./r3-parent-carrying-toddler-kit.ts";

export const TONE = { coat: 0.3, coatFar: 0.26, jeans: 0.45, jeansFar: 0.4, boot: 0.2, bootFar: 0.16, sole: 0.1, skin: 0.8, glove: 0.24, gloveFar: 0.2, hair: 0.18, beanie: 0.42, scarf: 0.6, suit: 0.58, suitFar: 0.52, mitten: 0.4, welly: 0.26, wellyFar: 0.22, hat: 0.36, tskin: 0.84 } as const;
export const near = (t: number): number => (t === TONE.coatFar ? TONE.coat : t === TONE.jeansFar ? TONE.jeans : t === TONE.bootFar ? TONE.boot : t === TONE.gloveFar ? TONE.glove : t === TONE.suitFar ? TONE.suit : t === TONE.wellyFar ? TONE.welly : t);
export type Row = readonly [number, number, number, number, number];
type C = { id: string; bone: string; material: string; tone: number; role?: string };

/* ---------------------------------------------------------------- the parent's trunk: rows stated upright and leaned with the trunk */

const L5 = FRAMES.torso!.o;
const TORSO_TILT = FRAMES.torso!.u[0] / FRAMES.torso!.u[1];
/** 📐 The forward shift of a trunk row at the height `y` from the 3 degree backward lean of the default pose. */
export const lean = (y: number): number => (y > L5[1] ? (y - L5[1]) * TORSO_TILT : 0);
/** 🧥 The coat on the trunk as horizontal slices, upright: [y, back, front, half-width, exponent]: seat, waist, bust, shoulder blades, sloping shoulders to the collar seam. */
export const COAT: readonly Row[] = [
  [0.94, -0.162, 0.118, 0.19, 2.5],
  [0.98, -0.164, 0.122, 0.192, 2.5],
  [1.04, -0.15, 0.123, 0.181, 2.45],
  [1.1, -0.135, 0.124, 0.17, 2.4],
  [1.17, -0.128, 0.129, 0.169, 2.4],
  [1.24, -0.13, 0.138, 0.175, 2.4],
  [1.31, -0.134, 0.143, 0.181, 2.35],
  [1.37, -0.135, 0.138, 0.192, 2.3],
  [1.41, -0.13, 0.126, 0.21, 2.3],
  [1.435, -0.122, 0.114, 0.221, 2.3],
  [1.455, -0.11, 0.1, 0.221, 2.25],
  [1.47, -0.097, 0.087, 0.204, 2.2],
  [1.482, -0.083, 0.075, 0.17, 2.15],
  [1.493, -0.068, 0.064, 0.125, 2.1],
  [1.502, -0.056, 0.055, 0.085, 2],
];
export const coatRows = (table: readonly Row[] = COAT): St[] => table.map(([y, b, f, rw, e]) => H("torso", y, b + lean(y), f + lean(y), rw, e));
/** 📏 The coat surface at the height `y` and lateral `z`: x of the front (+1) or the back (-1), leaned. */
export function coatX(y: number, z: number, side: 1 | -1): number {
  let i = 0;
  while (i + 2 < COAT.length && COAT[i + 1]![0] < y) i++;
  const a = COAT[i]!;
  const b = COAT[i + 1]!;
  const t = Math.min(1, Math.max(0, (y - a[0]) / (b[0] - a[0])));
  const m = (k: number): number => a[k]! + (b[k]! - a[k]!) * t;
  const [back, front, rw, e] = [m(1), m(2), m(3), m(4)];
  const k = Math.pow(Math.max(0, 1 - Math.pow(Math.min(1, Math.abs(z) / rw), e)), 1 / e);
  return (back + front) / 2 + (side * k * (front - back)) / 2 + lean(y);
}
/** 🧥 The coat skirt below the hip, one half per thigh so that it opens with the stride: [y, back, front, half-width of the whole skirt, exponent]. */
export const SKIRT: readonly Row[] = [
  [1.0, -0.153, 0.113, 0.181, 2.8],
  [0.95, -0.168, 0.122, 0.197, 2.8],
  [0.9, -0.17, 0.12, 0.198, 2.8],
  [0.8, -0.166, 0.124, 0.201, 2.8],
  [0.7, -0.162, 0.128, 0.205, 2.8],
  [0.6, -0.158, 0.132, 0.209, 2.8],
  [0.578, -0.156, 0.133, 0.21, 2.8],
];
const SKIRT_SHIFT = 0.07;
/** 📏 The front of the skirt half on the side of `z` at the height `y`. */
export function skirtFront(y: number, z: number): number {
  let i = 0;
  while (i + 2 < SKIRT.length && SKIRT[i + 1]![0] > y) i++;
  const a = SKIRT[i]!;
  const b = SKIRT[i + 1]!;
  const t = Math.min(1, Math.max(0, (y - a[0]) / (b[0] - a[0])));
  const m = (k: number): number => a[k]! + (b[k]! - a[k]!) * t;
  const rw = m(3) - SKIRT_SHIFT + 0.004;
  const c = Math.sign(z) * SKIRT_SHIFT;
  const k = Math.pow(Math.max(0, 1 - Math.pow(Math.min(1, Math.abs(z - c) / rw), m(4))), 1 / m(4));
  return (m(1) + m(2)) / 2 + (k * (m(2) - m(1))) / 2;
}
/** 👖 The jeans seat under the coat. */
export const SEAT: readonly Row[] = [
  [0.79, -0.1, 0.07, 0.15, 2.3],
  [0.84, -0.13, 0.088, 0.172, 2.4],
  [0.9, -0.146, 0.094, 0.178, 2.4],
  [0.96, -0.138, 0.096, 0.17, 2.3],
  [1.03, -0.116, 0.096, 0.155, 2.3],
];
/** 👖 The left jeans leg on the thigh, bone-local: quadriceps in front, hamstrings behind, inner seam clear of the other leg. */
export const THIGH_L: St[] = [st(0.0, -0.006, 0.012, 0.086, 0.095, 2.1), st(0.08, 0.0, 0.01, 0.084, 0.092, 2.1), st(0.18, 0.006, 0.006, 0.076, 0.083), st(0.28, 0.008, 0.002, 0.066, 0.072), st(0.36, 0.006, 0, 0.058, 0.063), st(0.4, 0.006, 0, 0.056, 0.06)];
/** 🦵 The left jeans leg on the shank: knee, calf, straight leg falling over the boot shaft with a small break. */
export const SHANK_L: St[] = [st(0.0, 0.006, 0, 0.056, 0.06), st(0.06, 0.003, 0, 0.053, 0.057), st(0.14, -0.008, 0, 0.055, 0.062), st(0.24, -0.006, 0, 0.052, 0.058), st(0.32, -0.002, 0, 0.05, 0.055), st(0.37, 0.0, 0, 0.051, 0.058), st(0.395, 0.004, 0, 0.052, 0.062, 2, -6)];

/** 🥾 The left ankle boot upper as cross slices from the ankle: [dx, bottom, top, half-width, exponent]; shaft and quarters on the foot bone, toe box on the toe bone. */
export const BOOT_REAR: readonly Row[] = [
  [-0.072, 0.034, 0.07, 0.02, 2],
  [-0.066, 0.025, 0.14, 0.033, 2.3],
  [-0.045, 0.025, 0.15, 0.042, 2.3],
  [-0.015, 0.024, 0.152, 0.045, 2.3],
  [0.02, 0.022, 0.15, 0.046, 2.4],
  [0.05, 0.018, 0.112, 0.048, 2.4],
  [0.09, 0.013, 0.085, 0.05, 2.5],
  [0.135, 0.012, 0.068, 0.051, 2.6],
];
export const BOOT_TOE: readonly Row[] = [
  [0.115, 0.012, 0.07, 0.051, 2.6],
  [0.155, 0.012, 0.06, 0.049, 2.6],
  [0.185, 0.013, 0.052, 0.042, 2.4],
  [0.203, 0.016, 0.044, 0.03, 2.2],
  [0.212, 0.022, 0.035, 0.014, 2],
];
export const HEEL_PLAN: [number, number][] = [[0, -0.074], [0.02, -0.07], [0.031, -0.058], [0.036, -0.035], [0.037, -0.004], [-0.037, -0.004], [-0.036, -0.035], [-0.031, -0.058], [-0.02, -0.07]];
export const SOLE_PLAN: [number, number][] = [[0.038, -0.01], [0.044, 0.04], [0.05, 0.09], [0.053, 0.136], [-0.053, 0.136], [-0.05, 0.09], [-0.044, 0.04], [-0.038, -0.01]];
export const TOE_PLAN: [number, number][] = [[0.053, -0.02], [0.052, 0.02], [0.045, 0.045], [0.03, 0.068], [0.008, 0.08], [-0.014, 0.08], [-0.035, 0.07], [-0.048, 0.05], [-0.053, 0.02], [-0.053, -0.02]];

/* ---------------------------------------------------------------- the parent's head */

/** 🗣️ The head as horizontal slices from the chin to the vertex: [y, back, front, half-width, exponent]. */
export const HEAD: readonly Row[] = [
  [1.496, 0.01, 0.052, 0.02, 2],
  [1.506, -0.016, 0.058, 0.036, 2.1],
  [1.521, -0.052, 0.061, 0.049, 2.2],
  [1.541, -0.084, 0.062, 0.057, 2.2],
  [1.566, -0.106, 0.063, 0.063, 2.2],
  [1.591, -0.12, 0.064, 0.065, 2.1],
  [1.616, -0.126, 0.062, 0.069, 2.1],
  [1.644, -0.128, 0.071, 0.072, 2],
  [1.671, -0.125, 0.066, 0.07, 2],
  [1.691, -0.114, 0.052, 0.061, 2],
  [1.706, -0.098, 0.033, 0.048, 2],
  [1.716, -0.078, 0.01, 0.032, 2],
  [1.721, -0.058, -0.012, 0.016, 2],
];
export const NECK: readonly Row[] = [[1.43, -0.106, 0.012, 0.062, 2], [1.481, -0.102, 0.014, 0.058, 2], [1.536, -0.096, 0.018, 0.055, 2], [1.586, -0.09, 0.02, 0.054, 2]];
/** 🧣 The scarf wound twice round the neck above the collar. */
export const SCARF: readonly Row[] = [[1.452, -0.124, 0.036, 0.088, 2.2], [1.476, -0.134, 0.05, 0.094, 2.2], [1.506, -0.135, 0.054, 0.095, 2.2], [1.53, -0.127, 0.049, 0.089, 2.2], [1.547, -0.113, 0.037, 0.076, 2.1]];
const BEANIE_EDGE = 1.655;
/** 📐 A head row interpolated at the height `y`. */
export function rowAt(table: readonly Row[], y: number): Row {
  let i = 0;
  while (i + 2 < table.length && table[i + 1]![0] < y) i++;
  const a = table[i]!;
  const b = table[i + 1]!;
  const t = (y - a[0]) / (b[0] - a[0]);
  return a.map((v, k) => v + (b[k]! - v) * t) as unknown as Row;
}
/** 🧢 A knit cap over a head table: the crown as the head rows from `edge` up, standing off by `base` at the edge and `top` at the crown (the slack of the knit), closed by a round dome; and the turned-up cuff of height `cuff`, 6 mm proud of the cap. */
export function knitCap(table: readonly Row[], edge: number, base: number, top: number, cuff: number, bone: string): [St[], St[]] {
  const yTop = table[table.length - 1]![0];
  const ys = Array.from({ length: 9 }, (_, i) => edge + ((yTop - 0.012 - edge) * i) / 8);
  const off = (y: number): number => base + ((top - base) * (y - edge)) / (yTop - edge);
  const crown = ys.map((y) => { const [, b, f, rw, e] = rowAt(table, y); const o = off(y); return H(bone, y + o * 0.6, b - o, f + o, rw + o, e); });
  const band = ([[edge - 0.004, 0.002], [edge, 0.007], [edge + cuff * 0.5, 0.008], [edge + cuff, 0.007], [edge + cuff + 0.004, 0.002]] as [number, number][]).map(([y, d]) => { const [, b, f, rw, e] = rowAt(table, y); const o = off(y) + d; return H(bone, y, b - o, f + o, rw + o, e); });
  return [dome(crown, 1, top + 0.004, 4), band];
}
/** 🧶 The head rows above the beanie edge, interpolated so the beanie starts exactly at its edge. */
function headAbove(y0: number): Row[] {
  let i = 0;
  while (HEAD[i + 1]![0] < y0) i++;
  const a = HEAD[i]!;
  const b = HEAD[i + 1]!;
  const t = (y0 - a[0]) / (b[0] - a[0]);
  const first = a.map((v, k) => v + (b[k]! - v) * t) as unknown as Row;
  return [first, ...HEAD.filter((r) => r[0] > y0)];
}

/* ---------------------------------------------------------------- the toddler */

const TH = FRAMES["toddler-torso"]!;
const TT = TH.u[0] / TH.u[1];
/** 📐 The spine line of the toddler: x of the line from its hip-joint centre to C7 at the height `y`. */
export const spine = (y: number): number => TH.o[0] + (y - TH.o[1]) * TT;
/** 🧸 The padded overall on the toddler's trunk: [y, back, front, half-width, exponent] relative to the spine line; seat, pot belly, chest, shoulders, collar. */
export const SUIT: readonly Row[] = [
  [1.152, -0.04, 0.035, 0.07, 2.2],
  [1.165, -0.056, 0.058, 0.098, 2.3],
  [1.19, -0.064, 0.078, 0.108, 2.3],
  [1.23, -0.064, 0.092, 0.11, 2.3],
  [1.275, -0.058, 0.098, 0.108, 2.2],
  [1.32, -0.046, 0.099, 0.106, 2.2],
  [1.37, -0.04, 0.096, 0.107, 2.2],
  [1.41, -0.036, 0.09, 0.112, 2.3],
  [1.44, -0.033, 0.078, 0.114, 2.3],
  [1.46, -0.03, 0.064, 0.1, 2.2],
  [1.475, -0.027, 0.05, 0.074, 2.1],
  [1.488, -0.024, 0.04, 0.05, 2],
];
export const suitRows = (): St[] => SUIT.map(([y, b, f, rw, e]) => H("toddler-torso", y, spine(y) + b, spine(y) + f, rw, e));
/** 👶 The toddler's head: [y, back, front, half-width, exponent]; large cranium, full cheeks, small chin. */
export const THEAD: readonly Row[] = [
  [1.5, 0.208, 0.238, 0.024, 2],
  [1.51, 0.176, 0.244, 0.04, 2],
  [1.525, 0.142, 0.25, 0.056, 2.2],
  [1.545, 0.116, 0.256, 0.065, 2.2],
  [1.565, 0.101, 0.257, 0.068, 2.1],
  [1.59, 0.093, 0.254, 0.067, 2.1],
  [1.615, 0.09, 0.253, 0.0675, 2],
  [1.64, 0.092, 0.247, 0.066, 2],
  [1.66, 0.099, 0.235, 0.06, 2],
  [1.675, 0.113, 0.217, 0.048, 2],
  [1.684, 0.136, 0.194, 0.025, 2],
];
const HAT_EDGE = 1.605;
function theadAbove(y0: number): Row[] {
  let i = 0;
  while (THEAD[i + 1]![0] < y0) i++;
  const a = THEAD[i]!;
  const b = THEAD[i + 1]!;
  const t = (y0 - a[0]) / (b[0] - a[0]);
  return [a.map((v, k) => v + (b[k]! - v) * t) as unknown as Row, ...THEAD.filter((r) => r[0] > y0)];
}
/** 🦵 The toddler's left suit leg on the thigh (u ahead, v up): full nappy-padded thigh to a round knee. */
export const TTHIGH_L: St[] = [st(-0.01, 0.004, 0, 0.0616, 0.0638), st(0.05, 0.002, 0, 0.0583, 0.0594), st(0.11, 0.0, 0, 0.0528, 0.0528), st(0.16, -0.002, 0, 0.0473, 0.0473), st(0.185, -0.002, 0, 0.0451, 0.0451)];
/** 🦶 The suit leg on the shank (u down, v ahead), ending over the boot shaft. */
export const TSHANK_L: St[] = [st(-0.01, 0.0, 0, 0.0451, 0.0451), st(0.04, -0.003, 0, 0.044, 0.0451), st(0.09, -0.006, 0, 0.0429, 0.0451), st(0.125, -0.002, 0, 0.0418, 0.0429), st(0.158, 0, 0, 0.0457, 0.0468)];
/** 🥾 The toddler's rubber boot on the foot (u ahead and a little down, v up): heel, instep, round toe. */
export const WELLY_L: St[] = [st(-0.043, -0.024, 0, 0.022, 0.026), st(-0.034, -0.016, 0, 0.032, 0.036), st(-0.012, -0.012, 0, 0.036, 0.042), st(0.02, -0.02, 0, 0.038, 0.034), st(0.055, -0.028, 0, 0.038, 0.026), st(0.085, -0.032, 0, 0.035, 0.021), st(0.103, -0.034, 0, 0.026, 0.017), st(0.113, -0.035, 0, 0.012, 0.011)];
export const TSLEEVE_L: St[] = [st(-0.012, 0.0, -0.012, 0.0354, 0.0401), st(0.01, 0.0, -0.002, 0.0437, 0.046), st(0.06, 0.002, 0.01, 0.0425, 0.0437), st(0.11, 0.0, 0.023, 0.0389, 0.0401), st(0.135, 0.0, 0.03, 0.0378, 0.0389)];
export const TFORE_L: St[] = [st(-0.01, 0.0, 0, 0.0378, 0.0389), st(0.04, 0.0, -0.012, 0.0366, 0.0366), st(0.085, 0.0, -0.026, 0.033, 0.033), st(0.1, 0.0, -0.03, 0.0342, 0.0342)];

/* ---------------------------------------------------------------- the parent's arms and hands */

/** 🧥 The left sleeve on the upper arm: set-in sleeve head inside the shoulder, deltoid, the elbow drifting outwards to the forearm joint (`drift` = lateral offset of the elbow). */
export const sleeve = (drift: number, side: 1 | -1): St[] => [st(-0.025, 0, -0.02 * side, 0.04, 0.05), st(-0.005, 0, -0.005 * side, 0.054, 0.062), st(0.045, 0.002, 0.006 * side + drift * 0.14, 0.06, 0.066), st(0.12, 0.004, drift * 0.38, 0.056, 0.062), st(0.2, 0.002, drift * 0.63, 0.052, 0.058), st(0.27, -0.002, drift * 0.85, 0.05, 0.055), st(0.316, -0.004, drift, 0.049, 0.054)];
/** 💪 The sleeve on the forearm down to the cuff, drifting laterally by `drift` to the wrist. */
export const foreSleeve = (drift: number): St[] => [st(-0.01, -0.004, 0, 0.049, 0.054), st(0.06, 0, drift * 0.24, 0.048, 0.052), st(0.14, 0, drift * 0.56, 0.044, 0.046), st(0.2, 0, drift * 0.8, 0.041, 0.042), st(0.232, 0, drift * 0.93, 0.042, 0.043)];
const glove = (id: string, bone: string, tone: number, role = "body"): C => ({ id, bone, material: "leather-glove", tone, role });

/** 🧤 The hanging left hand in its glove: palm towards the thigh, thumb ahead, fingers of three phalanges curling in. */
function leftHand(tone: number): Json[] {
  const c = (id: string): C => glove(id, "hand-l", tone);
  const out: Json[] = [];
  const palm: St[] = [st(-0.012, 0, 0, 0.022, 0.03), st(0.02, 0.002, 0, 0.019, 0.037), st(0.052, 0.002, -0.001, 0.016, 0.043), st(0.084, 0.0, -0.003, 0.014, 0.044), st(0.096, 0, -0.004, 0.0125, 0.041)];
  out.push(loft(c("glove-l"), dome(dense(palm, 3), 1, 0.008, 2)));
  out.push(ellipsoid(c("glove-l-thenar"), [0.031, 0.017, 0.012], [0.041, 0.029, -0.011]));
  const fingers: [string, number, number, number][] = [["1", 0.092, 0.031, 1], ["2", 0.096, 0.0105, 1.08], ["3", 0.092, -0.0105, 1], ["4", 0.085, -0.03, 0.8]];
  for (const [name, u, v, k] of fingers) {
    const dir = (deg: number, len: number, from: V3): V3 => [from[0] + len * Math.cos((deg * Math.PI) / 180), from[1], from[2] - len * Math.sin((deg * Math.PI) / 180)];
    const p0: V3 = [u, v, -0.003];
    const p1 = dir(20, 0.044 * k, p0);
    const p2 = dir(48, 0.027 * k, p1);
    const p3 = dir(72, 0.02 * k, p2);
    const r = k < 0.9 ? 0.9 : 1;
    out.push(capsule(c(`glove-l-finger-${name}`), p0, p1, 0.0104 * r, 0.0097 * r));
    out.push(capsule(c(`glove-l-finger-${name}-mid`), p1, p2, 0.0095 * r, 0.0088 * r));
    out.push(capsule(c(`glove-l-finger-${name}-tip`), p2, p3, 0.0088 * r, 0.0081 * r));
  }
  out.push(capsule(c("glove-l-thumb"), [0.02, 0.031, -0.01], [0.058, 0.053, -0.017], 0.0127, 0.0114));
  out.push(capsule(c("glove-l-thumb-mid"), [0.058, 0.053, -0.017], [0.089, 0.063, -0.022], 0.0112, 0.0101));
  out.push(capsule(c("glove-l-thumb-tip"), [0.089, 0.063, -0.022], [0.109, 0.065, -0.027], 0.0101, 0.0088));
  return out;
}

/** 🫴 The right hand palm up under the toddler's thighs: metacarpals flat, the four fingers along the underside of the thighs and bending up a little at the knees, the thumb along the outer thigh. */
function rightHand(tone: number): Json[] {
  const c = (id: string): C => glove(id, "hand-r", tone);
  const out: Json[] = [];
  const palm: St[] = [st(-0.012, 0.004, 0, 0.03, 0.019), st(0.02, 0.008, 0, 0.038, 0.017), st(0.055, 0.009, 0, 0.042, 0.0155), st(0.09, 0.009, 0, 0.042, 0.0135), st(0.1, 0.009, 0, 0.04, 0.0125)];
  out.push(loft(c("glove-r"), dome(dense(palm, 3), 1, 0.008, 2)));
  out.push(ellipsoid(c("glove-r-thenar"), [0.03, 0.013, 0.016], [0.035, 0.004, -0.03]));
  const fingers: [string, number, number][] = [["1", -0.0285, 1], ["2", -0.0095, 1.08], ["3", 0.0095, 1], ["4", 0.0285, 0.8]];
  for (const [name, w, k] of fingers) {
    const up = (deg: number, len: number, from: V3): V3 => [from[0] + len * Math.cos((deg * Math.PI) / 180), from[1] + len * Math.sin((deg * Math.PI) / 180), from[2]];
    const p0: V3 = [0.097, 0.009, w];
    const p1 = up(3, 0.043 * k, p0);
    const p2 = up(7, 0.026 * k, p1);
    const p3 = up(12, 0.019 * k, p2);
    const r = k < 0.9 ? 0.9 : 1;
    out.push(capsule(c(`glove-r-finger-${name}`), p0, p1, 0.0104 * r, 0.0097 * r));
    out.push(capsule(c(`glove-r-finger-${name}-mid`), p1, p2, 0.0095 * r, 0.0088 * r));
    out.push(capsule(c(`glove-r-finger-${name}-tip`), p2, p3, 0.0088 * r, 0.0081 * r));
  }
  out.push(capsule(c("glove-r-thumb"), [0.022, 0.006, -0.038], [0.055, 0.016, -0.055], 0.0127, 0.0114));
  out.push(capsule(c("glove-r-thumb-tip"), [0.055, 0.016, -0.055], [0.086, 0.03, -0.062], 0.0108, 0.0092));
  return out;
}

/* ---------------------------------------------------------------- body */

export const ANKLE_Y = FRAMES["foot-l"]!.o[1];
const FOOT_X = (side: "l" | "r"): number => FRAMES[`foot-${side}`]!.o[0];
const LEVEL = { transform: { rotation: aboutW(-boneDeg("foot-l")) } };

export function body50(): Json[] {
  const out: Json[] = [];
  const both = (solids: Json[]): void => {
    for (const s of solids) out.push(s);
    for (const s of solids) out.push(mirrored(s, toRight, near));
  };
  const wool = (id: string, bone: string, tone: number = TONE.coat, role = "body"): C => ({ id, bone, material: "wool", tone, role });
  const denim = (id: string, bone: string, tone: number = TONE.jeans): C => ({ id, bone, material: "denim", tone });
  const leather = (id: string, bone: string, tone: number = TONE.boot, role = "body"): C => ({ id, bone, material: "leather", tone, role });

  out.push(loft(denim("jeans-seat", "pelvis"), dense(SEAT.map(([y, b, f, rw, e]) => H("pelvis", y, b, f, rw, e)), 3)));
  out.push(loft(wool("coat", "torso"), dense(coatRows(), 3)));
  for (const side of ["l", "r"] as const) {
    const s = side === "l" ? 1 : -1;
    const rows = SKIRT.map(([y, b, f, rw, e]) => H(`thigh-${side}`, y, b, f, rw - SKIRT_SHIFT + 0.004, e, s * SKIRT_SHIFT - FRAMES[`thigh-${side}`]!.o[2]));
    out.push(loft(wool(`coat-skirt-${side}`, `thigh-${side}`, side === "l" ? TONE.coatFar : TONE.coat), dense(rows, 2)));
  }

  const leg: Json[] = [];
  const lx = FOOT_X("l");
  leg.push(loft(denim("thigh-l", "thigh-l", TONE.jeansFar), dome(dense(THIGH_L, 3), 1, 0.035)));
  leg.push(loft(denim("shank-l", "shank-l", TONE.jeansFar), dome(dense(SHANK_L, 3), -1, 0.035)));
  leg.push(loft(leather("boot-l", "foot-l", TONE.bootFar), dense(BOOT_REAR.map(([x, b, t, rw, e]) => X("foot-l", lx + x, b, t, rw, e)), 3)));
  leg.push(loft(leather("boot-toe-l", "toe-l", TONE.bootFar), dense(BOOT_TOE.map(([x, b, t, rw, e]) => X("toe-l", lx + x, b, t, rw, e)), 3)));
  leg.push(extrude(leather("boot-heel-l", "foot-l", TONE.sole), "zx", HEEL_PLAN, -ANKLE_Y, -ANKLE_Y + 0.025, LEVEL));
  leg.push(extrude(leather("boot-sole-l", "foot-l", TONE.sole), "zx", SOLE_PLAN, -ANKLE_Y, -ANKLE_Y + 0.012, LEVEL));
  leg.push(extrude(leather("boot-toe-sole-l", "toe-l", TONE.sole), "zx", TOE_PLAN.map(([z, x]) => [z, x + 0.005] as [number, number]), 0, 0.012));
  both(leg);

  out.push(loft(wool("sleeve-l", "upper-arm-l", TONE.coatFar), dome(dense(sleeve(0.04, 1), 3), 1, 0.04)));
  out.push(loft(wool("forearm-l", "forearm-l", TONE.coatFar), dome(dense(foreSleeve(0.03), 3), -1, 0.04)));
  out.push(loft(glove("glove-cuff-l", "forearm-l", TONE.gloveFar), [st(0.205, 0, 0.026, 0.035, 0.034), st(0.248, 0, 0.03, 0.031, 0.031), st(0.262, 0, 0.03, 0.029, 0.03)]));
  out.push(...leftHand(TONE.gloveFar));
  out.push(loft(wool("sleeve-r", "upper-arm-r"), dome(dense(sleeve(-0.03, -1), 3), 1, 0.04)));
  out.push(loft(wool("forearm-r", "forearm-r"), dome(dense(foreSleeve(0.05), 3), -1, 0.04)));
  out.push(loft(glove("glove-cuff-r", "forearm-r", TONE.glove), [st(0.205, 0, 0.043, 0.035, 0.034), st(0.248, 0, 0.05, 0.031, 0.031), st(0.262, 0.002, 0.05, 0.029, 0.03)]));
  out.push(...rightHand(TONE.glove));

  const skin = (id: string, bone = "head", tone: number = TONE.skin, role = "body"): C => ({ id, bone, material: "skin", tone, role });
  const Hd = (x: number, y: number, z = 0): V3 => L("head", [x, y, z]);
  const level = aboutW(90 - boneDeg("head"));
  out.push(loft(skin("neck", "head", 0.76), NECK.map(([y, b, f, rw, e]) => H("head", y, b, f, rw, e))));
  out.push(loft({ id: "scarf", bone: "torso", material: "knit-hat", tone: TONE.scarf }, dense(SCARF.map(([y, b, f, rw, e]) => H("torso", y, b, f, rw, e)), 3)));
  for (const [k, [z, bottom, w]] of ([[0.028, 1.29, 0.032], [0.078, 1.335, 0.03]] as [number, number, number][]).entries()) {
    const ys = [1.475, 1.43, 1.38, bottom + 0.03, bottom];
    out.push(loft({ id: `scarf-end-${k + 1}`, bone: "torso", material: "knit-hat", tone: TONE.scarf - 0.03 * k, role: "detail" }, ys.map((y, j) => H("torso", y, coatX(y, z, 1) - 0.006 + 0.004 * k, coatX(y, z, 1) + 0.014 + 0.004 * k, j === ys.length - 1 ? w * 0.9 : w, 2.3, z + (1.475 - y) * 0.05))));
  }
  out.push(loft(wool("collar", "torso", 0.27), [H("torso", 1.465, -0.108 + lean(1.465), 0.084 + lean(1.465), 0.105, 2.2), H("torso", 1.49, -0.11 + lean(1.49), 0.07 + lean(1.49), 0.1, 2.2), H("torso", 1.505, -0.108 + lean(1.505), 0.058 + lean(1.505), 0.096, 2.2)]));
  out.push(loft(skin("head"), dome(dome(dense(HEAD.map(([y, b, f, rw, e]) => H("head", y, b, f, rw, e)), 3), -1, 0.004, 2), 1, 0.003, 2)));

  const [cap, capCuff] = knitCap(HEAD, BEANIE_EDGE, 0.01, 0.012, 0.04, "head");
  out.push(loft({ id: "beanie", bone: "head", material: "knit-hat", tone: TONE.beanie }, cap));
  out.push(loft({ id: "beanie-cuff", bone: "head", material: "knit-hat", tone: TONE.beanie - 0.04 }, capCuff));
  const hair: Row[] = [[1.57, -0.112, -0.06, 0.056, 2], [1.6, -0.128, -0.04, 0.069, 2], [1.63, -0.134, -0.03, 0.075, 2], [1.66, -0.132, -0.04, 0.074, 2]];
  out.push(loft({ id: "hair", bone: "head", material: "skin", tone: TONE.hair }, dome(hair.map(([y, b, f, rw, e]) => H("head", y, b, f, rw, e)), -1, 0.012, 2)));
  const nose: [number, number, number][] = [[1.632, 0.0725, 0.0075], [1.616, 0.0805, 0.0092], [1.601, 0.0885, 0.012], [1.59, 0.0935, 0.0158], [1.582, 0.085, 0.0158]];
  out.push(loft(skin("head-nose"), dense(nose.map(([y, f, rw]) => H("head", y, 0.054, f, rw, 2)), 2).reverse()));
  out.push(ellipsoid(skin("head-brow"), [0.0075, 0.008, 0.042], Hd(0.0625, 1.64), level));
  out.push(ellipsoid(skin("head-lips", "head", 0.66), [0.0066, 0.0072, 0.022], Hd(0.0605, 1.5515), level));
  for (const side of [1, -1]) {
    const n = side > 0 ? "l" : "r";
    out.push(ellipsoid(skin(`head-ear-${n}`, "head", 0.74), [0.03, 0.015, 0.006], Hd(-0.038, 1.612, side * 0.071), compose(level, aboutW(-10), aboutU(side * 16))));
    out.push(ellipsoid(skin(`head-eye-${n}`, "head", 0.15, "detail"), [0.005, 0.005, 0.011], Hd(0.0585, 1.62, side * 0.031), level));
  }
  for (const side of [1, -1]) {
    const n = side > 0 ? "l" : "r";
    for (const [i, y] of [1.33, 1.21, 1.09].entries()) out.push(ellipsoid(wool(`coat-button-${n}-${i + 1}`, "torso", 0.12, "detail"), [0.011, 0.0045, 0.011], L("torso", [coatX(y, 0.06, 1) + 0.002, y, side * 0.06]), aboutW(90 - boneDeg("torso"))));
  }
  for (const side of ["l", "r"] as const) {
    const s = side === "l" ? 1 : -1;
    const y = 0.86;
    const z = s * 0.13;
    const f = skirtFront(y, z);
    const yaw = (Math.atan2(skirtFront(y, z - 0.01) - skirtFront(y, z + 0.01), 0.02) * 180) / Math.PI;
    out.push(box(wool(`coat-pocket-${side}`, `thigh-${side}`, 0.24, "detail"), [0.045, 0.008, 0.14], L(`thigh-${side}`, [f + 0.002, y, z]), aboutU(-yaw), 0.003));
  }
  out.push(...toddler50());
  return out;
}

/* ---------------------------------------------------------------- the toddler's body */

function toddler50(): Json[] {
  const out: Json[] = [];
  const suit = (id: string, bone: string, tone: number = TONE.suit, role = "body"): C => ({ id, bone, material: "padded-suit", tone, role });
  const both = (solids: Json[], farTone = true): void => {
    for (const s of solids) out.push(farTone ? s : s);
    for (const s of solids) out.push(mirrored(s, toRight, near));
  };
  out.push(loft(suit("toddler-suit", "toddler-torso"), dome(dense(suitRows(), 3), -1, 0.012, 2)));
  out.push(loft(suit("toddler-collar", "toddler-torso", 0.5), [H("toddler-torso", 1.47, spine(1.47) - 0.034, spine(1.47) + 0.062, 0.07, 2.2), H("toddler-torso", 1.492, spine(1.492) - 0.03, spine(1.492) + 0.052, 0.058, 2.2), H("toddler-torso", 1.508, spine(1.508) - 0.024, spine(1.508) + 0.044, 0.05, 2.2)]));
  const zip: V3[] = [1.2, 1.28, 1.36, 1.44, 1.5].map((y) => [spine(y) + (y < 1.25 ? 0.092 : y < 1.4 ? 0.099 : y < 1.47 ? 0.08 : 0.045) + 0.003, y, FRAMES["toddler-torso"]!.o[2]] as V3);
  out.push(tube(suit("toddler-zip", "toddler-torso", 0.4, "detail"), zip.map((p) => L("toddler-torso", p)), [0.003], { texture: true }));

  const leg: Json[] = [];
  leg.push(loft(suit("toddler-thigh-l", "toddler-thigh-l", TONE.suitFar), dome(dense(TTHIGH_L, 2), 1, 0.03)));
  leg.push(loft(suit("toddler-shank-l", "toddler-shank-l", TONE.suitFar), dome(dense(TSHANK_L, 2), -1, 0.03)));
  leg.push(loft({ id: "toddler-boot-shaft-l", bone: "toddler-shank-l", material: "rubber", tone: TONE.wellyFar }, [st(0.12, 0, 0, 0.0365, 0.0385), st(0.145, 0, 0, 0.0365, 0.0385), st(0.165, 0, 0, 0.036, 0.039)]));
  leg.push(loft({ id: "toddler-boot-l", bone: "toddler-foot-l", material: "rubber", tone: TONE.wellyFar }, dense(WELLY_L, 2)));
  leg.push(extrude({ id: "toddler-boot-sole-l", bone: "toddler-foot-l", material: "rubber", tone: TONE.sole }, "zx", [[0.024, -0.045], [0.034, -0.02], [0.038, 0.03], [0.034, 0.08], [0.022, 0.108], [0, 0.116], [-0.022, 0.108], [-0.034, 0.08], [-0.038, 0.03], [-0.034, -0.02], [-0.024, -0.045], [0, -0.05]], -0.0605, -0.052));
  const arm: Json[] = [];
  arm.push(loft(suit("toddler-sleeve-l", "toddler-arm-l", TONE.suitFar), dome(dense(TSLEEVE_L, 2), -1, 0.02)));
  arm.push(loft(suit("toddler-forearm-l", "toddler-forearm-l", TONE.suitFar), dome(dense(TFORE_L, 2), -1, 0.02)));
  const mit = (id: string): C => ({ id, bone: "toddler-forearm-l", material: "knit-hat", tone: TONE.mitten });
  arm.push(loft(mit("toddler-mitten-l"), [st(0.098, 0, -0.03, 0.026, 0.024), st(0.115, 0.002, -0.03, 0.028, 0.019), st(0.145, 0.004, -0.031, 0.029, 0.017), st(0.165, 0.004, -0.032, 0.024, 0.016), st(0.175, 0.004, -0.032, 0.013, 0.012)]));
  arm.push(capsule(mit("toddler-mitten-thumb-l"), [0.112, 0.004, -0.05], [0.14, 0.008, -0.062], 0.0105, 0.009));
  both([...leg, ...arm]);
  return [...out, ...toddlerHead()];
}

function toddlerHead(): Json[] {
  const out: Json[] = [];
  const skin = (id: string, tone: number = TONE.tskin, role = "body"): C => ({ id, bone: "toddler-head", material: "skin", tone, role });
  const z0 = FRAMES["toddler-head"]!.o[2];
  const P = (x: number, y: number, z = 0): V3 => L("toddler-head", [x, y, z0 + z]);
  const level = aboutW(90 - boneDeg("toddler-head"));
  out.push(loft(skin("toddler-neck", 0.8), [H("toddler-head", 1.47, 0.12, 0.2, 0.04, 2), H("toddler-head", 1.51, 0.122, 0.205, 0.04, 2)]));
  out.push(loft(skin("toddler-head"), dome(dome(dense(THEAD.map(([y, b, f, rw, e]) => H("toddler-head", y, b, f, rw, e)), 3), -1, 0.004, 2), 1, 0.003, 2)));
  const [hat, hatCuff] = knitCap(THEAD, HAT_EDGE, 0.011, 0.016, 0.028, "toddler-head");
  out.push(loft({ id: "toddler-hat", bone: "toddler-head", material: "knit-hat", tone: TONE.hat }, hat));

  out.push(loft({ id: "toddler-hat-cuff", bone: "toddler-head", material: "knit-hat", tone: TONE.hat - 0.05 }, hatCuff));
  out.push(ellipsoid({ id: "toddler-pompom", bone: "toddler-head", material: "knit-hat", tone: TONE.hat + 0.1 }, [0.021, 0.019, 0.021], P(0.163, 1.705), level));
  out.push(ellipsoid(skin("toddler-nose"), [0.009, 0.008, 0.0095], P(0.258, 1.566), level));
  out.push(ellipsoid(skin("toddler-lips", 0.72), [0.005, 0.005, 0.014], P(0.247, 1.537), level));
  for (const side of [1, -1]) {
    const n = side > 0 ? "l" : "r";
    out.push(ellipsoid(skin(`toddler-ear-${n}`, 0.8), [0.021, 0.012, 0.005], P(0.16, 1.585, side * 0.067), compose(level, aboutU(side * 14))));
    out.push(ellipsoid(skin(`toddler-eye-${n}`, 0.12, "detail"), [0.0042, 0.0046, 0.0085], P(0.249, 1.59, side * 0.026), level));
  }
  return out;
}

export { leftHand, rightHand, LEVEL };
export const DECISIONS_50: Json[] = [{ en: "1:50: draft.", de: "1:50: Entwurf." }];
export const kgOf = (bone: string): Json => (ASSET.rig.bones as Json[]).find((b) => b.id === bone).mass;
export { part, revolve, thOf, capsule, ellipsoid, tube, box, extrude, loft, X, H, L, st, dome, dense };
