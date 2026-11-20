/** 🚩 The 1:50 model of `humans/tour-guide`, stated section by section: an adult of 1.75 m (1.765 m in walking shoes) and 79 kg in a baseball cap, a hip-length softshell jacket zipped to the stand collar with a lanyard and badge, chinos and walking shoes; the left fist holds a 24 mm flag pole with a ball finial and a 0.46 x 0.30 m flag in the wind, the right hand points a telescopic pointer along its stretched index finger.
 *
 * Figure coordinates (`HA`, `X`, `L`): x ahead, y up, z to the left, the default pose `standing-pointing`; `HA` measures back and front from the bone axis at that height. Local stations (`st`): u along the bone, v its sagittal normal (ahead for a hanging limb, the thumb side of a hand), w lateral (+z).
 * @see ./r3-tour-guide-kit.ts
 * @see ./r3-humans-tour-guide.md */
import type { Json } from "./volumes-lib.ts";
import { ASSET, FRAMES, H, L, X, aboutU, aboutW, box, capsule, dense, dome, ellipsoid, flip, freeLoft, inset, loft, mirrored, part, revolve, st, thOf, toRight, tube, type St, type V3 } from "./r3-tour-guide-kit.ts";

export const TONE = { jacket: 0.34, jacketFar: 0.29, trousers: 0.5, trousersFar: 0.44, shoe: 0.14, shoeFar: 0.11, skin: 0.82, hand: 0.8, handFar: 0.74, hair: 0.22, cap: 0.24, metal: 0.62, flag: 0.45, badge: 0.9, bone: 0.9 } as const;
export const far = (t: number): number => (t === TONE.jacket ? TONE.jacketFar : t === TONE.trousers ? TONE.trousersFar : t === TONE.shoe ? TONE.shoeFar : t === TONE.hand ? TONE.handFar : t);
export const JACKET = { material: "softshell", tone: TONE.jacket } as const;
export const SKIN = { material: "skin", tone: TONE.skin } as const;
export const CLOTH = { material: "cotton-clothing", tone: TONE.trousers } as const;
export const SHOE = { material: "leather-shoe", tone: TONE.shoe } as const;
export const METAL = { material: "pointer-aluminium", tone: TONE.metal } as const;
export const BONE = { material: "bone", tone: TONE.bone } as const;
export type Slice = readonly [number, number, number, number, number];
export type Cross = readonly [number, number, number, number];

/** 📐 The world x of a bone's axis at the height `y` in the default pose. */
export const axisX = (bone: string, y: number): number => FRAMES[bone]!.o[0] + ((y - FRAMES[bone]!.o[1]) * FRAMES[bone]!.u[0]) / FRAMES[bone]!.u[1];
/** 🥞 A horizontal slice at the height `y` whose back and front are measured from the bone axis at that height. */
export const HA = (bone: string, y: number, back: number, front: number, rw: number, e = 2, z = 0): St => H(bone, y, axisX(bone, y) + back, axisX(bone, y) + front, rw, e, z);
export const slices = (bone: string, t: readonly Slice[]): St[] => t.map(([y, b, f, rw, e]) => HA(bone, y, b, f, rw, e));
/** 📏 The surface of a sliced garment at the height `y` and the lateral position `z`: world x of its front (+1) or back (-1). */
export function surfaceX(bone: string, t: readonly Slice[], y: number, z: number, side: 1 | -1): number {
  const s = [...t].sort((a, b) => a[0] - b[0]);
  let i = 0;
  while (i + 2 < s.length && s[i + 1]![0] < y) i++;
  const a = s[i]!;
  const b = s[i + 1]!;
  const k = Math.min(1, Math.max(0, (y - a[0]) / (b[0] - a[0])));
  const m = (j: number): number => a[j]! + (b[j]! - a[j]!) * k;
  const [back, front, rw, e] = [m(1), m(2), m(3), m(4)];
  const c = Math.pow(Math.max(0, 1 - Math.pow(Math.min(1, Math.abs(z) / rw), e)), 1 / e);
  return axisX(bone, y) + (back + front) / 2 + (side * c * (front - back)) / 2;
}
/** 🩹 A strip lying on a sliced garment: at every row [y, z, half-width] a thin section `proud` of the surface and `sunk` into it. */
export const onSurface = (bone: string, t: readonly Slice[], side: 1 | -1, rows: readonly (readonly [number, number, number])[], proud: number, sunk = 0.006): St[] =>
  rows.map(([y, z, half]) => {
    const x = surfaceX(bone, t, y, z, side);
    return H(bone, y, side > 0 ? x - sunk : x - proud, side > 0 ? x + proud : x + sunk, half, 3, z);
  });
/** 🍞 Cross slices [x from the bone axis at the slice, bottom, top, half-width] of a head part. */
export const crossA = (bone: string, t: readonly Cross[], e: number): St[] => t.map(([x, b, top, rw]) => X(bone, axisX(bone, (b + top) / 2) + x, b, top, rw, e));

/* ---------------------------------------------------------------- stations shared by body, thermal and the coarser scales */

const HEAD_O = FRAMES.head!;
/** ⛰️ The vertex of the skull in the default pose (head bone tip, 1.762 m). */
export const YV = HEAD_O.o[1] + HEAD_O.u[1] * 0.2826;

/** 🗣️ Head and face as horizontal slices [depth below the vertex, back, front, half-width, exponent], back and front from the head axis (the ear canal): chin, mandible, upper lip, cheekbones, eyes and brow, forehead, vault; 0.233 m chin to vertex, 0.206 m long at the brow, 0.155 m broad. */
export const HEAD_T: readonly Slice[] = [
  [0.233, 0.058, 0.094, 0.021, 2],
  [0.222, 0.028, 0.103, 0.038, 2],
  [0.2, -0.008, 0.105, 0.051, 2.1],
  [0.178, -0.046, 0.106, 0.06, 2.1],
  [0.155, -0.074, 0.104, 0.0665, 2.1],
  [0.13, -0.088, 0.104, 0.072, 2.1],
  [0.105, -0.095, 0.108, 0.0765, 2.1],
  [0.08, -0.098, 0.108, 0.0775, 2.1],
  [0.055, -0.096, 0.102, 0.0765, 2.05],
  [0.03, -0.085, 0.086, 0.068, 2],
  [0.012, -0.06, 0.058, 0.05, 2],
  [0.002, -0.026, 0.024, 0.022, 2],
];
export const headSlices = (t: readonly Slice[]): St[] => t.map(([d, b, f, rw, e]) => HA("head", YV - d, b, f, rw, e));
/** 🦒 The neck from inside the collar to the skull base [y, back, front, half-width]. */
export const NECK_T: readonly Slice[] = [
  [1.45, -0.066, 0.05, 0.062, 2],
  [1.49, -0.064, 0.05, 0.058, 2],
  [1.53, -0.06, 0.048, 0.056, 2],
  [1.565, -0.062, 0.04, 0.057, 2],
  [1.59, -0.064, 0.02, 0.056, 2],
];
/** 💇 The short hair under the cap as horizontal slices [depth below the vertex, back, front, half-width, exponent]: 8 mm over the skull under the crown, below the sweatband only behind the temples, over and behind the ears and down to the nape. */
export const HAIR_T: readonly Slice[] = [
  [0.165, -0.08, -0.05, 0.048, 2],
  [0.15, -0.088, -0.04, 0.056, 2],
  [0.125, -0.098, -0.02, 0.064, 2.1],
  [0.1, -0.105, 0.03, 0.071, 2.1],
  [0.08, -0.106, 0.1, 0.0855, 2.1],
  [0.055, -0.104, 0.104, 0.0845, 2.05],
  [0.03, -0.093, 0.094, 0.076, 2],
  [0.012, -0.068, 0.066, 0.058, 2],
  [-0.006, -0.03, 0.03, 0.026, 2],
];
/** 🧢 The crown of the cap as horizontal slices [depth below the vertex, back, front, half-width, exponent]: six-panel crown from the sweatband to the button. */
export const CAP_T: readonly Slice[] = [
  [0.09, -0.114, 0.117, 0.091, 2.2],
  [0.06, -0.116, 0.118, 0.093, 2.2],
  [0.035, -0.109, 0.112, 0.0905, 2.1],
  [0.012, -0.093, 0.095, 0.08, 2],
  [-0.006, -0.066, 0.067, 0.058, 2],
  [-0.0145, -0.032, 0.034, 0.028, 2],
];
/** 🧢 The brim as cross slices [x from the head axis, y of its middle, half-width]: curved down at the sides and the tip. */
export const BRIM_T: readonly (readonly [number, number, number])[] = [
  [0.1, YV - 0.084, 0.084],
  [0.125, YV - 0.088, 0.088],
  [0.15, YV - 0.093, 0.083],
  [0.172, YV - 0.098, 0.064],
  [0.186, YV - 0.101, 0.035],
];

/** 🧥 The softshell jacket above the waist as horizontal slices [y, back, front, half-width, exponent] from the trunk axis: waist, chest, shoulder blades, the shoulder line falling from the neck to the sleeve heads. */
export const JACKET_UP: readonly Slice[] = [
  [1.03, -0.125, 0.115, 0.171, 2.3],
  [1.1, -0.12, 0.115, 0.165, 2.3],
  [1.18, -0.122, 0.124, 0.168, 2.3],
  [1.25, -0.126, 0.134, 0.174, 2.3],
  [1.32, -0.13, 0.134, 0.182, 2.4],
  [1.38, -0.126, 0.122, 0.194, 2.5],
  [1.42, -0.112, 0.104, 0.2, 2.5],
  [1.45, -0.094, 0.084, 0.19, 2.4],
  [1.47, -0.08, 0.068, 0.165, 2.4],
  [1.487, -0.072, 0.06, 0.115, 2.2],
  [1.497, -0.068, 0.056, 0.08, 2],
];
/** 🧥 The jacket from the waist to the hem over the seat [y, back, front, half-width, exponent] from the hip joints. */
export const JACKET_LOW: readonly Slice[] = [
  [1.065, -0.117, 0.109, 0.163, 2.3],
  [1.0, -0.126, 0.112, 0.172, 2.3],
  [0.95, -0.134, 0.112, 0.18, 2.3],
  [0.9, -0.14, 0.114, 0.186, 2.35],
  [0.86, -0.141, 0.115, 0.187, 2.35],
  [0.835, -0.14, 0.115, 0.186, 2.35],
];
/** 🧥 The stand collar round the neck [y, back, front, half-width, exponent] from the head axis. */
export const COLLAR_T: readonly Slice[] = [
  [1.46, -0.082, 0.07, 0.078, 2.1],
  [1.495, -0.08, 0.066, 0.074, 2.05],
  [1.53, -0.078, 0.06, 0.071, 2],
];
/** 👖 The seat of the chinos under the jacket down to the crotch [y, back, front, half-width, exponent] from the hip joints. */
export const SEAT_T: readonly Slice[] = [
  [0.95, -0.128, 0.104, 0.172, 2.3],
  [0.88, -0.134, 0.102, 0.178, 2.3],
  [0.83, -0.126, 0.09, 0.168, 2.3],
  [0.805, -0.1, 0.06, 0.13, 2.2],
  [0.79, -0.06, 0.03, 0.06, 2],
];

/** 🧥 The left jacket sleeve on the upper arm: sleeve head inside the shoulder, deltoid, biceps and triceps under the cloth, the elbow. */
export const SLEEVE_T: St[] = [st(-0.03, 0, -0.026, 0.04, 0.05, 2.2), st(0.0, 0.002, -0.006, 0.06, 0.064, 2.2), st(0.05, 0.004, 0.004, 0.062, 0.064, 2.1), st(0.12, 0.004, 0.006, 0.057, 0.059), st(0.2, 0.002, 0.006, 0.053, 0.055), st(0.28, 0, 0.004, 0.05, 0.052), st(0.3255, -0.002, 0.004, 0.049, 0.051)];
/** 💪 The jacket sleeve on the forearm, the muscle mass near the elbow tapering to the elastic cuff; `end` = where the cuff ends (0.235 hanging, 0.2 on the raised arm whose sleeve slides back). */
export const foreT = (end: number): St[] => [st(0.0, 0, 0, 0.05, 0.052), st(0.05, 0.002, 0.002, 0.049, 0.05), st(0.12, 0, 0.003, 0.044, 0.045), st(end - 0.045, 0, 0.003, 0.04, 0.041), st(end - 0.02, 0, 0.003, 0.038, 0.039), st(end, 0, 0.003, 0.036, 0.037)];
/** 👖 The left trouser leg on the thigh: seat and quadriceps, the cloth tapering to the knee. */
export const THIGH_T: St[] = [st(-0.01, -0.012, 0.0, 0.076, 0.094, 2.1), st(0.1, -0.006, 0.006, 0.089, 0.095, 2.1), st(0.2, 0, 0.002, 0.08, 0.086, 2.05), st(0.3, 0.003, -0.002, 0.072, 0.078), st(0.37, 0.004, -0.004, 0.066, 0.07), st(0.411, 0.004, -0.004, 0.063, 0.067)];
/** 👖 The trouser leg on the shank: knee, the calf pushing the cloth back, the hem breaking on the instep. */
export const SHANK_T: St[] = [st(-0.025, 0.004, -0.004, 0.062, 0.066), st(0.02, 0.003, -0.004, 0.062, 0.066), st(0.07, 0, -0.004, 0.059, 0.065), st(0.14, -0.006, -0.004, 0.058, 0.066), st(0.24, -0.004, -0.003, 0.055, 0.062), st(0.33, 0, -0.002, 0.053, 0.06), st(0.395, 0.004, -0.002, 0.054, 0.063, 2, -5), st(0.42, 0.006, -0.002, 0.054, 0.064, 2, -9)];

/** 👟 The walking shoe (EU 43, 0.30 m) as cross slices [distance ahead of the ankle, bottom, top above the ground, half-width]: heel counter, collar, tongue, vamp on the foot bone, toe cap on the forefoot; the upper sits in a rubber sole that rises to 26 mm under the heel. */
export const SHOE_REAR_T: readonly Cross[] = [[-0.072, 0.024, 0.086, 0.024], [-0.062, 0.02, 0.092, 0.035], [-0.032, 0.018, 0.098, 0.041], [0.01, 0.016, 0.1, 0.044], [0.05, 0.013, 0.09, 0.047], [0.1, 0.011, 0.074, 0.05], [0.14, 0.01, 0.062, 0.052], [0.16, 0.01, 0.058, 0.0515]];
export const SHOE_TOE_T: readonly Cross[] = [[0.13, 0.01, 0.063, 0.0515], [0.17, 0.01, 0.054, 0.05], [0.2, 0.01, 0.046, 0.045], [0.22, 0.011, 0.037, 0.034], [0.233, 0.013, 0.027, 0.017]];
export const SOLE_REAR_T: readonly Cross[] = [[-0.075, 0.003, 0.026, 0.03], [-0.06, 0.0, 0.026, 0.038], [-0.02, 0.0, 0.024, 0.043], [0.04, 0.0, 0.018, 0.046], [0.1, 0.0, 0.014, 0.051], [0.15, 0.0, 0.013, 0.0535]];
export const SOLE_TOE_T: readonly Cross[] = [[0.13, 0.0, 0.013, 0.0535], [0.19, 0.0, 0.013, 0.049], [0.222, 0.002, 0.014, 0.037], [0.238, 0.007, 0.016, 0.017]];
const ANKLE = 0.083;
const BALL = 0.14;
/** 👣 Shoe slices in the frame of the foot (`toe` false) or of the forefoot (`toe` true), from heights above the sole. */
export const shoeSlices = (t: readonly Cross[], e: number, toe: boolean): St[] => t.map(([a, b, top, rw]) => st(toe ? a - BALL : a, (b + top) / 2 - (toe ? 0 : ANKLE), 0, rw, (top - b) / 2, e));

/* ---------------------------------------------------------------- the pointer, the flag pole and the flag */

/** 🪄 The telescopic pointer along the pointer bone: rubber grip in the hand, three drawn-out aluminium sections, the ball tip at 0.55 m. */
export const POINTER = { grip: [-0.05, 0.08, 0.0075], s1: [0.07, 0.22, 0.0055], s2: [0.21, 0.36, 0.0045], s3: [0.35, 0.54, 0.0035], ball: 0.006 } as const;
/** 🚩 The flag pole from below the fist to the ball finial, 24 mm, and the flag cloth 0.46 x 0.30 m with a hoist sleeve. */
export const POLE = { from: -0.33, to: 1.0, r: 0.012, ball: 0.02, flagLen: 0.46, flagH: 0.3, sleeveFrom: 0.665, sleeveTo: 0.975, sleeveR: 0.0165 } as const;
/** 🌊 The flag cloth as stations along the fly [u from the hoist, centre of the height below the top, lateral wave]: it flies back from the pole, rippled by the wind, the fly end sagging a little. */
export const FLAG_T: readonly (readonly [number, number, number])[] = [[0.004, 0.15, 0], [0.06, 0.151, 0.016], [0.12, 0.153, 0.022], [0.18, 0.155, 0.006], [0.24, 0.158, -0.016], [0.3, 0.161, -0.022], [0.36, 0.164, -0.006], [0.41, 0.167, 0.014], [0.458, 0.17, 0.02]];

function props(): Json[] {
  const out: Json[] = [];
  const p = { bone: "pointer", ...METAL } as const;
  out.push(capsule({ id: "pointer-grip", ...p, material: "pointer-aluminium", tone: 0.18 }, [POINTER.grip[0], 0, 0], [POINTER.grip[1], 0, 0], POINTER.grip[2], POINTER.grip[2]));
  out.push(capsule({ id: "pointer-1", ...p }, [POINTER.s1[0], 0, 0], [POINTER.s1[1], 0, 0], POINTER.s1[2], POINTER.s1[2]));
  out.push(capsule({ id: "pointer-2", ...p }, [POINTER.s2[0], 0, 0], [POINTER.s2[1], 0, 0], POINTER.s2[2], POINTER.s2[2]));
  out.push(capsule({ id: "pointer-3", ...p }, [POINTER.s3[0], 0, 0], [POINTER.s3[1], 0, 0], POINTER.s3[2], POINTER.s3[2], { slender: true }));
  out.push(ellipsoid({ id: "pointer-tip", ...p, role: "detail" }, [POINTER.ball, POINTER.ball, POINTER.ball], [0.55 - POINTER.ball, 0, 0]));
  const q = { bone: "flag-pole", ...METAL } as const;
  out.push(capsule({ id: "pole", ...q }, [POLE.from + POLE.r, 0, 0], [POLE.to, 0, 0], POLE.r, POLE.r, { slender: true }));
  out.push(ellipsoid({ id: "pole-finial", ...q }, [POLE.ball, POLE.ball, POLE.ball], [POLE.to + POLE.ball * 0.8, 0, 0]));
  out.push(revolve({ id: "pole-ferrule", ...q, tone: 0.15, role: "detail" }, [[POLE.from, 0], [POLE.from, 0.0135], [POLE.from + 0.004, 0.0145], [POLE.from + 0.035, 0.0145], [POLE.from + 0.035, 0]], [0, 0, 0], [1, 0, 0]));
  out.push(revolve({ id: "flag-sleeve", bone: "flag-pole", material: "flag-textile", tone: TONE.flag }, [[POLE.sleeveFrom, 0], [POLE.sleeveFrom, POLE.sleeveR - 0.002], [POLE.sleeveFrom + 0.004, POLE.sleeveR], [POLE.sleeveTo - 0.004, POLE.sleeveR], [POLE.sleeveTo, POLE.sleeveR - 0.002], [POLE.sleeveTo, 0]], [0, 0, 0], [1, 0, 0]));
  out.push(loft({ id: "flag", bone: "flag", material: "flag-textile", tone: TONE.flag }, FLAG_T.map(([u, v, w]) => st(u, v, w, 0.0038, POLE.flagH / 2, 6))));
  return out;
}

/* ---------------------------------------------------------------- the hands */

/** ✊ A fist round a cylinder of radius `r` lying in the palm at `C` (bone-local, palm on +w: a right hand; the left one is its mirror), the cylinder axis at `deg` from the hand axis towards the thumb side; four fingers wrap it from the knuckles, the thumb closes over the index finger. */
export function fist(bone: string, id: string, deg: number, C: V3, r: number, tone: number): Json[] {
  const p = { bone, material: "skin", tone } as const;
  const out: Json[] = [];
  out.push(loft({ id, ...p }, dome([st(-0.025, 0, -0.002, 0.018, 0.026, 2.4), st(0.0, 0, 0, 0.019, 0.029, 2.4), st(0.03, 0.002, -0.002, 0.016, 0.039, 2.6), st(0.065, 0.004, -0.003, 0.014, 0.044, 2.6), st(0.09, 0.005, -0.003, 0.013, 0.043, 2.6)], 1, 0.008, 2)));
  out.push(ellipsoid({ id: `${id}-thenar`, ...p }, [0.03, 0.018, 0.013], [0.036, 0.022, 0.006]));
  const s: V3 = [Math.cos((deg * Math.PI) / 180), Math.sin((deg * Math.PI) / 180), 0];
  const q: V3 = [s[1], -s[0], 0];
  const wrap = (t: number, rr: number, from: number, to: number): V3[] => [from, from + (to - from) * 0.2, from + (to - from) * 0.4, from + (to - from) * 0.6, from + (to - from) * 0.8, to].map((phi) => { const a = (phi * Math.PI) / 180; return [C[0] + s[0] * t + q[0] * rr * Math.cos(a), C[1] + s[1] * t + q[1] * rr * Math.cos(a), C[2] + rr * Math.sin(a)] as V3; });
  const finger = (name: string, t: number, fr: number, to: number): void => { out.push(tube({ id: `${id}-${name}`, ...p }, wrap(t, r + fr, -62, to), [fr * 1.05, fr, fr, fr, fr * 0.95, fr * 0.85])); };
  finger("index", 0.0335, 0.0088, 168);
  finger("middle", 0.0125, 0.009, 176);
  finger("ring", -0.0085, 0.0086, 172);
  finger("little", -0.0285, 0.0076, 164);
  out.push(tube({ id: `${id}-thumb`, ...p }, [[0.03, 0.03, 0.006], [0.05, 0.047, 0.028], [0.07, 0.055, 0.046], [0.09, 0.055, 0.054]], [0.012, 0.011, 0.0098, 0.0085]));
  return out;
}
/** 👉 The right hand holding the pointer like a baton: the grip in the hollow of the palm, the index finger stretched along the pointer, the thumb pressing it from the other side, middle, ring and little finger curled into the palm. */
function pointingHand(): Json[] {
  const p = { bone: "hand-r", material: "skin", tone: TONE.hand } as const;
  const out: Json[] = [];
  out.push(loft({ id: "hand-r", ...p }, dome([st(-0.025, 0, -0.002, 0.018, 0.026, 2.4), st(0.0, 0, 0, 0.019, 0.029, 2.4), st(0.03, 0.002, -0.002, 0.016, 0.039, 2.6), st(0.065, 0.004, -0.003, 0.014, 0.044, 2.6), st(0.09, 0.005, -0.003, 0.013, 0.043, 2.6)], 1, 0.008, 2)));
  out.push(ellipsoid({ id: "hand-r-thenar", ...p }, [0.03, 0.018, 0.013], [0.036, 0.022, 0.006]));
  out.push(tube({ id: "hand-r-index", ...p }, [[0.088, 0.03, -0.002], [0.115, 0.031, 0.004], [0.142, 0.031, 0.009], [0.168, 0.03, 0.013]], [0.0092, 0.0088, 0.0082, 0.0074]));
  const curl = (name: string, v: number, length: number, r: number): void => {
    const pts: V3[] = [[0.088, v, -0.002]];
    let a = 0;
    for (const [share, bend] of [[0.45, 55], [0.3, 75], [0.25, 45]] as const) {
      a += bend;
      const q = pts[pts.length - 1]!;
      pts.push([q[0] + length * share * Math.cos((a * Math.PI) / 180), v, q[2] + length * share * Math.sin((a * Math.PI) / 180)]);
    }
    out.push(tube({ id: `hand-r-${name}`, ...p }, pts, [r, r * 0.96, r * 0.9, r * 0.8]));
  };
  curl("middle", 0.0105, 0.08, 0.009);
  curl("ring", -0.009, 0.075, 0.0086);
  curl("little", -0.0275, 0.06, 0.0076);
  out.push(tube({ id: "hand-r-thumb", ...p }, [[0.03, 0.03, 0.008], [0.058, 0.045, 0.024], [0.085, 0.042, 0.034], [0.104, 0.034, 0.037]], [0.012, 0.011, 0.0096, 0.0084]));
  return out;
}

/* ---------------------------------------------------------------- body */

const pairs = (left: Json[]): Json[] => left.flatMap((s) => [mirrored(s, toRight), { ...s, tone: far(s.tone) }]);

/** 🪪 The lanyard on one side (+1 left): from behind the collar over the neck point and down the chest to the clip of the badge. */
export function lanyardPath(side: 1 | -1): V3[] {
  const front = (y: number, z: number, d = 0.003): V3 => [surfaceX("torso", JACKET_UP, y, z, 1) + d, y, z];
  const c = (y: number): number => axisX("head", y);
  return [[c(1.505) - 0.082, 1.505, side * 0.03], [c(1.51) - 0.06, 1.515, side * 0.062], [c(1.505) + 0.0, 1.5, side * 0.079], [c(1.48) + 0.055, 1.462, side * 0.074], front(1.42, side * 0.064), front(1.36, side * 0.05), front(1.31, side * 0.032), front(1.285, side * 0.014)];
}
export const BADGE = { y: 1.235, h: 0.086, w: 0.054, t: 0.0076 } as const;

export function body50(): Json[] {
  const out: Json[] = [];
  const head = { bone: "head", ...SKIN } as const;
  out.push(loft({ id: "neck", ...head }, dense(slices("head", NECK_T), 2)));
  out.push(loft({ id: "head", ...head }, dome(dense(headSlices(HEAD_T), 3), 1, 0.003, 2)));
  const hx = (d: number): number => axisX("head", YV - d);
  out.push(loft({ id: "head-nose", ...head }, [H("head", YV - 0.168, hx(0.168) + 0.09, hx(0.168) + 0.124, 0.017), H("head", YV - 0.158, hx(0.158) + 0.09, hx(0.158) + 0.134, 0.0145), H("head", YV - 0.135, hx(0.135) + 0.09, hx(0.135) + 0.119, 0.0095), H("head", YV - 0.108, hx(0.108) + 0.092, hx(0.108) + 0.112, 0.008)]));
  out.push(ellipsoid({ id: "head-lips", ...head, tone: 0.68, role: "detail" }, [0.0075, 0.0065, 0.024], L("head", [hx(0.19) + 0.103, YV - 0.19, 0])));
  for (const side of [1, -1]) {
    const n = side > 0 ? "l" : "r";
    out.push(ellipsoid({ id: `head-ear-${n}`, ...head }, [0.03, 0.016, 0.0085], L("head", [hx(0.12) - 0.01, YV - 0.12, side * 0.0745]), aboutU(side * -12)));
    out.push(ellipsoid({ id: `head-eye-${n}`, bone: "head", material: "skin", tone: 0.2, role: "detail" }, [0.0055, 0.0045, 0.013], L("head", [hx(0.115) + 0.0985, YV - 0.115, side * 0.0325])));
  }
  out.push(loft({ id: "hair", bone: "head", material: "hair", tone: TONE.hair }, dome(dense(headSlices(HAIR_T), 2), -1, 0.006, 2)));
  const cap = { bone: "head", material: "cap-fabric", tone: TONE.cap } as const;
  out.push(loft({ id: "cap", ...cap }, dense(headSlices(CAP_T), 2)));
  out.push(loft({ id: "cap-brim", ...cap }, BRIM_T.map(([x, y, rw]) => X("head", hx(YV - y) + x, y - 0.0042, y + 0.0042, rw, 2.6))));
  out.push(ellipsoid({ id: "cap-button", ...cap, role: "detail" }, [0.0095, 0.0055, 0.0095], L("head", [hx(-0.0125), YV + 0.0125, 0])));

  const jk = (id: string, bone: string, extra: Partial<{ tone: number; role: string }> = {}) => ({ id, bone, ...JACKET, ...extra });
  out.push(loft(jk("jacket", "torso"), dense(slices("torso", JACKET_UP), 2)));
  out.push(loft(jk("jacket-skirt", "pelvis"), dense(slices("pelvis", JACKET_LOW), 2)));
  out.push(loft(jk("jacket-hem", "pelvis", { tone: 0.3 }), [HA("pelvis", 0.831, -0.147, 0.121, 0.193, 2.35), HA("pelvis", 0.849, -0.1475, 0.1215, 0.1935, 2.35), HA("pelvis", 0.867, -0.1475, 0.1215, 0.1935, 2.35)]));
  out.push(loft(jk("jacket-collar", "head", { tone: 0.3 }), slices("head", COLLAR_T)));
  out.push(loft(jk("jacket-zip", "torso", { tone: 0.2, role: "detail" }), onSurface("torso", JACKET_UP, 1, [[1.03, 0, 0.006], [1.15, 0, 0.006], [1.3, 0, 0.006], [1.42, 0, 0.006], [1.46, 0, 0.006]], 0.003)));
  out.push(loft(jk("jacket-zip-skirt", "pelvis", { tone: 0.2, role: "detail" }), onSurface("pelvis", JACKET_LOW, 1, [[0.836, 0, 0.006], [0.9, 0, 0.006], [1.0, 0, 0.006], [1.065, 0, 0.006]], 0.003)));
  out.push(loft(jk("jacket-zip-collar", "head", { tone: 0.2, role: "detail" }), onSurface("head", COLLAR_T, 1, [[1.46, 0, 0.006], [1.495, 0, 0.006], [1.528, 0, 0.006]], 0.003)));
  out.push(box({ id: "jacket-zip-puller", bone: "head", material: "softshell", tone: 0.15, role: "detail" }, [0.026, 0.008, 0.012], L("head", [surfaceX("head", COLLAR_T, 1.512, 0, 1) + 0.006, 1.512, 0]), undefined, 0.003));
  for (const side of [1, -1]) {
    const n = side > 0 ? "l" : "r";
    out.push(loft(jk(`jacket-pocket-${n}`, "pelvis", { tone: 0.2, role: "detail" }), onSurface("pelvis", JACKET_LOW, 1, [[0.92, side * 0.14, 0.0055], [0.98, side * 0.122, 0.0055], [1.045, side * 0.104, 0.0055]], 0.003)));
  }
  out.push(loft(jk("jacket-chest-pocket", "torso", { tone: 0.2, role: "detail" }), onSurface("torso", JACKET_UP, 1, [[1.28, 0.09, 0.0055], [1.33, 0.096, 0.0055], [1.375, 0.102, 0.0055]], 0.003)));
  for (const side of [1, -1] as const) {
    out.push(tube({ id: `lanyard-${side > 0 ? "l" : "r"}`, bone: "torso", material: "cotton-clothing", tone: 0.1, role: "detail" }, lanyardPath(side).map((p) => L("torso", p)), [0.0035], { slender: true }));
  }
  const by = BADGE.y;
  out.push(box({ id: "badge", bone: "torso", material: "cotton-clothing", tone: TONE.badge, role: "detail" }, [BADGE.h, BADGE.t, BADGE.w], L("torso", [surfaceX("torso", JACKET_UP, by, 0, 1) + BADGE.t / 2 + 0.002, by, 0]), undefined, 0.0035));
  out.push(box({ id: "badge-clip", bone: "torso", material: "pointer-aluminium", tone: 0.5, role: "detail" }, [0.012, 0.0076, 0.0115], L("torso", [surfaceX("torso", JACKET_UP, by + 0.048, 0, 1) + 0.0055, by + 0.048, 0]), undefined, 0.002));

  const arm: Json[] = [];
  arm.push(loft({ id: "sleeve-l", bone: "upper-arm-l", ...JACKET }, dome(dense(SLEEVE_T, 2), 1, 0.04, 3)));
  out.push(...pairs(arm));
  out.push({ ...loft({ id: "forearm-l", bone: "forearm-l", ...JACKET }, dome(dense(foreT(0.235), 2), -1, 0.04, 3)), tone: TONE.jacketFar });
  out.push({ ...loft({ id: "cuff-l", bone: "forearm-l", ...JACKET, tone: 0.24, role: "detail" }, [st(0.214, 0, 0.003, 0.0395, 0.0405), st(0.236, 0, 0.003, 0.0375, 0.0385)]) });
  out.push(mirrored(loft({ id: "forearm-l", bone: "forearm-l", ...JACKET }, dome(dense(foreT(0.2), 2), -1, 0.04, 3)), toRight));
  out.push(mirrored(loft({ id: "cuff-l", bone: "forearm-l", ...JACKET, tone: 0.24, role: "detail" }, [st(0.179, 0, 0.003, 0.0405, 0.0415), st(0.201, 0, 0.003, 0.0385, 0.0395)]), toRight));
  out.push(mirrored(loft({ id: "wrist-l", bone: "forearm-l", ...SKIN }, [st(0.185, 0, 0.003, 0.024, 0.031), st(0.225, 0, 0.002, 0.021, 0.029), st(0.262, 0, 0.0, 0.019, 0.028, 2.3)]), toRight));
  out.push(...fist("hand-r", "hand-l", 65, [0.08, 0, 0.026], POLE.r, TONE.hand).map((s) => ({ ...mirrored(s, (x) => (x === "hand-r" ? "hand-l" : x)), tone: TONE.handFar })));
  out.push(...pointingHand());

  const leg: Json[] = [];
  leg.push(loft({ id: "thigh-l", bone: "thigh-l", ...CLOTH }, dense(THIGH_T, 2)));
  leg.push(loft({ id: "shank-l", bone: "shank-l", ...CLOTH }, dense(SHANK_T, 2)));
  leg.push(loft({ id: "shoe-l", bone: "foot-l", ...SHOE }, dense(shoeSlices(SHOE_REAR_T, 2.5, false), 2)));
  leg.push(loft({ id: "shoe-toe-l", bone: "toe-l", ...SHOE }, dense(shoeSlices(SHOE_TOE_T, 2.5, true), 2)));
  leg.push(loft({ id: "shoe-sole-l", bone: "foot-l", ...SHOE, tone: 0.32 }, shoeSlices(SOLE_REAR_T, 5, false)));
  leg.push(loft({ id: "shoe-toe-sole-l", bone: "toe-l", ...SHOE, tone: 0.32 }, shoeSlices(SOLE_TOE_T, 5, true)));
  leg.push(loft({ id: "shoe-lacing-l", bone: "foot-l", ...SHOE, tone: 0.05, role: "detail" }, [st(0.0, 0.1 - ANKLE + 0.001, 0, 0.016, 0.004, 3, -8), st(0.05, 0.091 - ANKLE + 0.001, 0, 0.017, 0.004, 3, -18), st(0.1, 0.076 - ANKLE + 0.001, 0, 0.015, 0.004, 3, -22)]));
  out.push(...pairs(leg));
  out.push(loft({ id: "trousers-seat", bone: "pelvis", ...CLOTH }, slices("pelvis", SEAT_T)));
  out.push(...props());
  return out;
}

/* ---------------------------------------------------------------- thermal */

export function thermal50(body: readonly Json[]): Json[] {
  const out: Json[] = [];
  const has = (id: string): boolean => body.some((s) => s.id === id);
  const of = (id: string, region: string, layer?: string): void => { if (!has(id)) throw new Error(`no body solid ${id}`); out.push(thOf(id, region, layer)); };
  for (const id of ["neck", "head", "head-nose", "head-lips", "head-ear-l", "head-ear-r", "head-eye-l", "head-eye-r"]) of(id, "head-skin");
  of("hair", "hair", "hair-under");
  for (const id of ["cap", "cap-brim", "cap-button"]) of(id, "hair", "cap");
  for (const s of body) if (s.material === "softshell" && /^(jacket|lanyard|badge)/.test(s.id)) of(s.id, "torso-clothing", "jacket");
  for (const id of ["lanyard-l", "lanyard-r", "badge", "badge-clip"]) of(id, "torso-clothing", "jacket");
  const air = { material: "softshell", tone: 0.6 } as const;
  const shirt = { material: "cotton-clothing", tone: 0.85 } as const;
  out.push({ ...loft({ id: "th-jacket-air", bone: "torso", ...air }, inset(dense(slices("torso", JACKET_UP), 2), 0.002)), region: "torso-clothing", layer: "jacket-air" });
  out.push({ ...loft({ id: "th-jacket-shirt", bone: "torso", ...shirt }, inset(dense(slices("torso", JACKET_UP), 2), 0.006)), region: "torso-clothing", layer: "shirt-under" });
  out.push({ ...loft({ id: "th-jacket-skirt-air", bone: "pelvis", ...air }, inset(dense(slices("pelvis", JACKET_LOW), 2), 0.002)), region: "torso-clothing", layer: "jacket-air" });
  out.push({ ...loft({ id: "th-jacket-skirt-shirt", bone: "pelvis", ...shirt }, inset(slices("pelvis", JACKET_LOW.slice(0, 4)), 0.006)), region: "torso-clothing", layer: "shirt-under" });
  for (const side of ["l", "r"] as const) {
    const m = (s: Json): Json => (side === "r" ? mirrored(s, toRight) : s);
    of(`sleeve-${side}`, "arm-clothing", "jacket-sleeve");
    of(`forearm-${side}`, "arm-clothing", "jacket-sleeve");
    of(`cuff-${side}`, "arm-clothing", "jacket-sleeve");
    out.push(m({ ...loft({ id: "th-sleeve-l-air", bone: "upper-arm-l", ...air }, dome(inset(dense(SLEEVE_T, 2), 0.002), 1, 0.038, 3)), region: "arm-clothing", layer: "sleeve-air" }));
    out.push(m({ ...loft({ id: "th-sleeve-l-shirt", bone: "upper-arm-l", ...shirt }, dome(inset(dense(SLEEVE_T, 2), 0.006), 1, 0.034, 3)), region: "arm-clothing", layer: "shirt-sleeve" }));
    const end = side === "l" ? 0.235 : 0.2;
    out.push(m({ ...loft({ id: "th-forearm-l-air", bone: "forearm-l", ...air }, dome(inset(dense(foreT(end), 2), 0.002), -1, 0.038, 3)), region: "arm-clothing", layer: "sleeve-air" }));
    out.push(m({ ...loft({ id: "th-forearm-l-shirt", bone: "forearm-l", ...shirt }, dome(inset(dense(foreT(end), 2), 0.006), -1, 0.034, 3)), region: "arm-clothing", layer: "shirt-sleeve" }));
  }
  of("wrist-r", "arm-skin");
  for (const s of body) if (s.material === "skin" && /^hand-/.test(s.id)) of(s.id, "hand-skin");
  for (const side of ["l", "r"] as const) {
    const m = (s: Json): Json => (side === "r" ? mirrored(s, toRight) : s);
    of(`thigh-${side}`, "leg-clothing", "trousers");
    of(`shank-${side}`, "leg-clothing", "trousers");
    out.push(m({ ...loft({ id: "th-thigh-l-air", bone: "thigh-l", ...CLOTH, tone: 0.7 }, inset(dense(THIGH_T, 2), 0.0012)), region: "leg-clothing", layer: "trouser-air" }));
    out.push(m({ ...loft({ id: "th-shank-l-air", bone: "shank-l", ...CLOTH, tone: 0.7 }, inset(dense(SHANK_T, 2), 0.0012)), region: "leg-clothing", layer: "trouser-air" }));
    for (const id of ["shoe", "shoe-toe", "shoe-sole", "shoe-toe-sole", "shoe-lacing"]) of(`${id}-${side}`, "shoes", "shoe-upper");
    const leather = { material: "leather-shoe", tone: 0.4 } as const;
    out.push(m({ ...loft({ id: "th-shoe-l-air", bone: "foot-l", ...leather }, inset(dense(shoeSlices(SHOE_REAR_T, 2.5, false), 2), 0.003)), region: "shoes", layer: "shoe-air" }));
    out.push(m({ ...loft({ id: "th-shoe-l-sock", bone: "foot-l", material: "cotton-clothing", tone: 0.8 }, inset(dense(shoeSlices(SHOE_REAR_T, 2.5, false), 2), 0.011)), region: "shoes", layer: "sock-layer" }));
    out.push(m({ ...loft({ id: "th-shoe-toe-l-air", bone: "toe-l", ...leather }, inset(dense(shoeSlices(SHOE_TOE_T, 2.5, true), 2), 0.003)), region: "shoes", layer: "shoe-air" }));
    out.push(m({ ...loft({ id: "th-shoe-toe-l-sock", bone: "toe-l", material: "cotton-clothing", tone: 0.8 }, inset(dense(shoeSlices(SHOE_TOE_T.slice(0, 4), 2.5, true), 2), 0.011)), region: "shoes", layer: "sock-layer" }));
  }
  of("trousers-seat", "leg-clothing", "trousers");
  out.push({ ...loft({ id: "th-seat-air", bone: "pelvis", ...CLOTH, tone: 0.7 }, inset(slices("pelvis", SEAT_T.slice(0, 4)), 0.0012)), region: "leg-clothing", layer: "trouser-air" });
  for (const id of ["pointer-grip", "pointer-1", "pointer-2", "pointer-3", "pointer-tip"]) of(id, "pointer-metal");
  for (const id of ["pole", "pole-finial", "pole-ferrule"]) of(id, "pole-metal");
  for (const id of ["flag", "flag-sleeve"]) of(id, "flag-cloth");
  const hx = (d: number): number => axisX("head", YV - d);
  out.push({ ...ellipsoid({ id: "th-nose-breath", bone: "head", ...SKIN }, [0.0045, 0.004, 0.0105], L("head", [hx(0.168) + 0.118, YV - 0.172, 0])), region: "head-skin", tags: ["emitter", "nose-breath"] });
  out.push({ ...ellipsoid({ id: "th-mouth-breath", bone: "head", ...SKIN }, [0.0045, 0.004, 0.009], L("head", [hx(0.19) + 0.108, YV - 0.19, 0])), region: "head-skin", tags: ["emitter", "mouth-breath"] });
  out.push({ ...loft({ id: "th-plume", bone: "torso", ...air }, [HA("torso", 1.43, -0.114, -0.02, 0.18, 2.6), HA("torso", 1.455, -0.093, -0.02, 0.17, 2.6), HA("torso", 1.475, -0.079, -0.02, 0.135, 2.4)]), region: "torso-clothing", tags: ["emitter", "plume"] });
  return out;
}

/* ---------------------------------------------------------------- structure */

export const kgOf = (bone: string): { kg: number; comX: number; comY: number } => (ASSET.rig.bones.find((b: Json) => b.id === bone) as Json).mass;
/** 🦴 A point of the trunk skeleton: `x` from the trunk axis, `y` world height. */
export const T = (x: number, y: number, z = 0): V3 => L("torso", [axisX("torso", y) + x, y, z]);
export const P = (x: number, y: number, z = 0): V3 => L("pelvis", [axisX("pelvis", y) + x, y, z]);
/** 💀 A point of the skull: `d` below the vertex, `x` from the head axis. */
export const HD = (x: number, d: number, z = 0): V3 => L("head", [axisX("head", YV - d) + x, YV - d, z]);

export function structure50(body: readonly Json[]): Json[] {
  const out: Json[] = [];
  const m = (solid: Json, member: string): void => { out.push(part(solid, member, "member")); };
  const joint = (list: Json[], member: string, at: V3, r: number): void => { list.push(part(ellipsoid({ id: `joint-${member}`, bone: member, material: "bone", tone: 0.4 }, [r, r, r], at), member, "joint")); };
  const mass = (list: Json[], member: string): void => {
    const k = kgOf(member);
    const r = 0.0125 * Math.cbrt(k.kg);
    list.push(part(ellipsoid({ id: `mass-${member}`, bone: member, material: /^(pointer|flag)/.test(member) ? "pointer-aluminium" : "bone", tone: 0.2, role: "detail" }, [r, r, r], [k.comX, k.comY, 0]), member, "mass"));
  };
  const spine: [number, number, number][] = [[-0.042, 1.035, 0.021], [-0.03, 1.08, 0.0205], [-0.026, 1.13, 0.02], [-0.034, 1.18, 0.019], [-0.052, 1.24, 0.0175], [-0.066, 1.3, 0.016], [-0.069, 1.36, 0.015], [-0.061, 1.415, 0.014], [-0.048, 1.465, 0.013]];
  m(tube({ id: "spine", bone: "torso", ...BONE }, spine.map(([x, y]) => T(x, y)), spine.map((q) => q[2])), "torso");
  const hoops: [number, number, number, number][] = [[1.168, 0.014, 0.09, 0.122], [1.2, 0.014, 0.1, 0.134], [1.232, 0.012, 0.107, 0.142], [1.264, 0.01, 0.111, 0.147], [1.296, 0.006, 0.112, 0.148], [1.326, 0.002, 0.109, 0.145], [1.356, -0.004, 0.102, 0.136], [1.384, -0.01, 0.092, 0.122], [1.41, -0.016, 0.078, 0.104], [1.432, -0.022, 0.061, 0.081]];
  hoops.forEach(([y, x, rd, rw], i) => {
    const ring: [number, number][] = Array.from({ length: 8 }, (_, k) => [0.006 * Math.cos((k * Math.PI) / 4), rd + 0.006 * Math.sin((k * Math.PI) / 4)] as [number, number]);
    m(revolve({ id: `rib-cage-${10 - i}`, bone: "torso", ...BONE }, ring, T(x, y), [1, 0, 0], { lateralScale: Math.round((rw / rd) * 1000) / 1000 }), "torso");
  });
  m(loft({ id: "sternum", bone: "torso", ...BONE }, [HA("torso", 1.27, 0.111, 0.124, 0.013), HA("torso", 1.315, 0.108, 0.121, 0.019), HA("torso", 1.385, 0.088, 0.101, 0.021), HA("torso", 1.437, 0.054, 0.068, 0.028)]), "torso");
  for (const side of [1, -1]) {
    const n = side > 0 ? "l" : "r";
    m(capsule({ id: `clavicle-${n}`, bone: "torso", ...BONE }, T(0.06, 1.447, side * 0.022), T(-0.004, 1.47, side * 0.168), 0.0085, 0.008), "torso");
    m(ellipsoid({ id: `scapula-${n}`, bone: "torso", ...BONE }, [0.082, 0.0076, 0.053], T(-0.088, 1.37, side * 0.11), aboutU(side * 26)), "torso");
  }
  joint(out, "torso", [0, 0, 0], 0.021);
  mass(out, "torso");
  m(tube({ id: "sacrum", bone: "pelvis", ...BONE }, [P(-0.044, 1.03), P(-0.066, 0.985), P(-0.08, 0.945), P(-0.076, 0.915)], [0.029, 0.027, 0.018, 0.0085]), "pelvis");
  for (const side of [1, -1]) {
    const n = side > 0 ? "l" : "r";
    m(ellipsoid({ id: `ilium-${n}`, bone: "pelvis", ...BONE }, [0.07, 0.076, 0.0125], P(-0.03, 1.01, side * 0.104), aboutU(side * 32)), "pelvis");
    m(capsule({ id: `pubis-${n}`, bone: "pelvis", ...BONE }, P(0.004, 0.94, side * 0.086), P(0.054, 0.92, side * 0.012), 0.0125, 0.0115), "pelvis");
    m(capsule({ id: `ischium-${n}`, bone: "pelvis", ...BONE }, P(-0.01, 0.935, side * 0.084), P(-0.045, 0.87, side * 0.06), 0.0135, 0.0145), "pelvis");
  }
  mass(out, "pelvis");
  m(ellipsoid({ id: "cranium", bone: "head", ...BONE }, [0.069, 0.093, 0.071], HD(0.004, 0.072)), "head");
  m(ellipsoid({ id: "face-bones", bone: "head", ...BONE }, [0.042, 0.036, 0.052], HD(0.06, 0.14)), "head");
  for (const side of [1, -1]) {
    const n = side > 0 ? "l" : "r";
    m(capsule({ id: `mandible-ramus-${n}`, bone: "head", ...BONE }, HD(0.004, 0.137, side * 0.052), HD(0.0, 0.19, side * 0.048), 0.0078, 0.0082), "head");
    m(capsule({ id: `mandible-body-${n}`, bone: "head", ...BONE }, HD(0.0, 0.19, side * 0.048), HD(0.074, 0.218, side * 0.008), 0.0082, 0.0092), "head");
  }
  m(tube({ id: "cervical-spine", bone: "head", ...BONE }, [HD(-0.046, 0.3), HD(-0.032, 0.258), HD(-0.022, 0.214), HD(-0.016, 0.17)], [0.0128, 0.0122, 0.0118, 0.0112]), "head");
  joint(out, "head", [0, 0, 0], 0.016);
  mass(out, "head");

  const side: Json[] = [];
  const lm = (solid: Json, member: string): void => { side.push(part(solid, member, "member")); };
  lm(capsule({ id: "femur-neck-l", bone: "thigh-l", ...BONE }, [0, 0, 0], [0.034, -0.006, 0.06], 0.017, 0.018), "thigh-l");
  lm(ellipsoid({ id: "femur-head-l", bone: "thigh-l", ...BONE }, [0.025, 0.025, 0.025], [0, 0, 0]), "thigh-l");
  lm(ellipsoid({ id: "femur-trochanter-l", bone: "thigh-l", ...BONE }, [0.025, 0.019, 0.016], [0.03, -0.008, 0.068]), "thigh-l");
  lm(tube({ id: "femur-l", bone: "thigh-l", ...BONE }, [[0.045, -0.005, 0.064], [0.13, 0.008, 0.047], [0.215, 0.013, 0.031], [0.3, 0.008, 0.014], [0.39, 0.0, 0.002]], [0.016, 0.0146, 0.014, 0.0146, 0.0175]), "thigh-l");
  lm(ellipsoid({ id: "femur-condyles-l", bone: "thigh-l", ...BONE }, [0.022, 0.032, 0.042], [0.402, -0.006, 0]), "thigh-l");
  lm(ellipsoid({ id: "patella-l", bone: "thigh-l", ...BONE }, [0.022, 0.0098, 0.021], [0.4, 0.043, 0]), "thigh-l");
  mass(side, "thigh-l");
  lm(ellipsoid({ id: "tibia-plateau-l", bone: "shank-l", ...BONE }, [0.016, 0.028, 0.039], [0.02, -0.002, 0]), "shank-l");
  lm(tube({ id: "tibia-l", bone: "shank-l", ...BONE }, [[0.03, 0.004, -0.003], [0.134, 0.012, -0.003], [0.28, 0.009, -0.003], [0.416, 0.002, -0.002]], [0.0175, 0.014, 0.0125, 0.014]), "shank-l");
  lm(ellipsoid({ id: "malleolus-l", bone: "shank-l", ...BONE }, [0.0145, 0.0125, 0.0098], [0.428, 0.0, -0.031]), "shank-l");
  lm(capsule({ id: "fibula-l", bone: "shank-l", ...BONE }, [0.042, -0.019, 0.033], [0.436, -0.012, 0.029], 0.007, 0.0075), "shank-l");
  joint(side, "shank-l", [0, 0, 0], 0.021);
  mass(side, "shank-l");
  lm(capsule({ id: "calcaneus-l", bone: "foot-l", ...BONE }, [-0.052, 0.05 - ANKLE, 0], [0.004, 0.062 - ANKLE, 0], 0.022, 0.02), "foot-l");
  lm(loft({ id: "midfoot-l", bone: "foot-l", ...BONE }, [st(-0.004, 0.066 - ANKLE, 0, 0.025, 0.022), st(0.05, 0.057 - ANKLE, 0.0, 0.033, 0.022), st(0.1, 0.04 - ANKLE, 0, 0.039, 0.015), st(0.145, 0.028 - ANKLE, 0, 0.043, 0.011)]), "foot-l");
  joint(side, "foot-l", [0, 0, 0], 0.017);
  lm(loft({ id: "phalanges-l", bone: "toe-l", ...BONE }, [st(-0.005, 0.028, 0, 0.043, 0.011), st(0.04, 0.023, 0, 0.039, 0.009), st(0.075, 0.02, 0, 0.028, 0.006)]), "toe-l");
  lm(ellipsoid({ id: "humerus-head-l", bone: "upper-arm-l", ...BONE }, [0.024, 0.024, 0.024], [0.002, 0, -0.012]), "upper-arm-l");
  lm(tube({ id: "humerus-l", bone: "upper-arm-l", ...BONE }, [[0.014, 0, -0.008], [0.103, 0.004, 0.003], [0.206, 0.003, 0.006], [0.31, 0, 0.004]], [0.0124, 0.0111, 0.0107, 0.0128]), "upper-arm-l");
  lm(ellipsoid({ id: "humerus-condyles-l", bone: "upper-arm-l", ...BONE }, [0.0124, 0.0144, 0.032], [0.319, 0, 0.004]), "upper-arm-l");
  joint(side, "upper-arm-l", [0, 0, 0], 0.0145);
  lm(capsule({ id: "ulna-l", bone: "forearm-l", ...BONE }, [-0.017, -0.014, -0.006], [0.243, -0.004, -0.012], 0.0088, 0.0054), "forearm-l");
  lm(capsule({ id: "radius-l", bone: "forearm-l", ...BONE }, [0.012, 0.012, 0.012], [0.249, 0.012, 0.006], 0.0062, 0.0093), "forearm-l");
  joint(side, "forearm-l", [0, 0, 0], 0.0145);
  for (const s of side) out.push(s, mirrored(s, toRight));
  const hand = (n: "l" | "r", bodyIds: string[]): void => {
    const b = `hand-${n}`;
    m(loft({ id: `carpus-${n}`, bone: b, ...BONE }, [st(0.004, 0, 0, 0.0092, 0.023, 2.4), st(0.031, 0.001, 0, 0.0082, 0.027, 2.4)]), b);
    for (const [k, v] of [0.028, 0.0095, -0.0095, -0.027].entries()) m(capsule({ id: `metacarpal-${n}-${k + 2}`, bone: b, ...BONE }, [0.033, v * 0.75, 0], [0.088, v, 0], 0.005, 0.0057), b);
    m(capsule({ id: `metacarpal-${n}-1`, bone: b, ...BONE }, [0.02, 0.022, n === "r" ? 0.004 : -0.004], [0.052, 0.04, n === "r" ? 0.012 : -0.012], 0.0057, 0.0057), b);
    for (const id of bodyIds) {
      const f = body.find((s) => s.id === id)!;
      out.push(part({ ...f, id: id.replace("hand-", "phalanges-"), material: "bone", tone: TONE.bone, sections: f.sections.map((q: Json) => ({ ...q, rx: 0.0042, ry: 0.0042 })) }, b, "member"));
    }
    joint(out, b, [0, 0, 0], 0.0115);
  };
  hand("l", ["hand-l-index", "hand-l-middle", "hand-l-ring", "hand-l-little", "hand-l-thumb"]);
  hand("r", ["hand-r-index", "hand-r-middle", "hand-r-ring", "hand-r-little", "hand-r-thumb"]);
  for (const id of ["pointer-grip", "pointer-1", "pointer-2", "pointer-3"]) out.push({ id: `member-${id}`, of: id, member: "pointer", part: "member" });
  out.push({ id: "member-pole", of: "pole", member: "flag-pole", part: "member" });
  out.push({ id: "member-flag", of: "flag", member: "flag", part: "member" });
  out.push(part(ellipsoid({ id: "joint-flag", bone: "flag", material: "pointer-aluminium", tone: 0.4 }, [0.0165, 0.012, 0.0165], [0.0, 0.03, 0]), "flag", "joint"));
  mass(out, "pointer");
  mass(out, "flag-pole");
  return out;
}

export const DECISIONS_50: Json[] = [
  { en: "1:50 body: every part is a loft through hand-stated sections: head with jaw, cheekbones, nose, lips, ears and eyes under a six-panel baseball cap with a curved brim and a button, short hair at the temples and the nape; the neck in the stand collar of a hip-length softshell jacket with its front zip and puller, two slanted zipped hand pockets, a chest pocket, the back yoke and a drawcord hem band over the seat; a lanyard round the collar down to a badge with its clip; jacket sleeves with deltoid, elbow and elastic cuffs, the right sleeve pulled back by the raised arm so the wrist shows; the left fist round the flag pole with four fingers and the thumb, the right hand holding the pointer like a baton, index finger stretched along it, thumb against it, three fingers curled; chinos with seat, knees, calves and hems breaking on walking shoes with heel counter, collar, lacing, toe cap and a rubber sole; the telescopic pointer with grip, three sections and ball tip; the 24 mm flag pole with rubber ferrule and ball finial, the flag cloth rippled in the wind on its hoist sleeve.", de: "1:50 Körper: Jedes Teil ist ein Loft durch von Hand gesetzte Schnitte: Kopf mit Kiefer, Wangenknochen, Nase, Lippen, Ohren und Augen unter einer sechsteiligen Baseballmütze mit gebogenem Schirm und Knopf, kurzes Haar an Schläfen und Nacken; der Hals im Stehkragen einer hüftlangen Softshelljacke mit Frontreißverschluss und Zipper, zwei schrägen Reißverschluss-Handtaschen, einer Brusttasche, der Rückenpasse und einem Saumbund mit Kordelzug über dem Gesäß; ein Umhängeband um den Kragen bis zu einem Ausweis mit Clip; Jackenärmel mit Deltamuskel, Ellbogen und Bündchen, der rechte Ärmel vom gehobenen Arm zurückgezogen, sodass das Handgelenk frei liegt; die linke Faust mit vier Fingern und Daumen um die Fahnenstange, die rechte Hand hält den Zeigestab wie einen Taktstock, der Zeigefinger daran gestreckt, der Daumen dagegen, drei Finger eingerollt; Chinohose mit Gesäß, Knien, Waden und auf den Wanderschuhen aufstoßenden Säumen; Schuhe mit Fersenkappe, Schaftrand, Schnürung, Zehenkappe und Gummisohle; der Teleskop-Zeigestab mit Griff, drei Auszügen und Kugelspitze; die 24-mm-Fahnenstange mit Gummikappe und Kugelknauf, das Fahnentuch im Wind gewellt an seinem Hohlsaum." },
  { en: "1:50 thermal: the jacket shell is the outer layer by reference; the air under the jacket and the shirt lie inside it as nested lofts on trunk, seat and sleeves (the shirt ends at the hip); the chinos and their air layer are modelled on seat, thighs and shanks; the shoes carry air and sock inside the upper; the hair is its own 8 mm shell under the cap layer; face and neck, the exposed right wrist and the hands are their own surfaces; pointer, pole and flag are their real parts; breath leaves at nostril and mouth patches, the plume from a patch on the jacket shoulders.", de: "1:50 thermisch: Die Jackenhülle ist als Verweis die äußere Schicht; die Luft unter der Jacke und das Hemd liegen als geschachtelte Lofts darin an Rumpf, Gesäß und Ärmeln (das Hemd endet an der Hüfte); die Chinohose und ihre Luftschicht sind an Gesäß, Oberschenkeln und Unterschenkeln modelliert; die Schuhe tragen Luft und Socke im Schaft; das Haar ist eine eigene 8-mm-Schale unter der Mützenschicht; Gesicht und Hals, das freie rechte Handgelenk und die Hände sind eigene Flächen; Zeigestab, Stange und Fahne sind ihre wirklichen Teile; der Atem tritt an Nasen- und Mundfeld aus, die Warmluftfahne an einem Feld auf den Jackenschultern." },
  { en: "1:50 structure: the skeleton carries the body (skull with jaw, cervical and thoracolumbar spine with their curves, ten rib hoops, sternum, clavicles, scapulae, sacrum, ilia, pubic and ischial rami, femur with head, neck, trochanter and condyles, patella, tibia, fibula, malleolus, calcaneus, midfoot and toes, humerus, ulna, radius, carpus, metacarpals and finger rays), joints at the rig joints and masses at the segment centres; the pointer is carried by its grip and sections, the flag by the pole and the cloth on its hoist.", de: "1:50 Tragwerk: Das Skelett trägt den Körper (Schädel mit Unterkiefer, Hals- und Brust-Lenden-Wirbelsäule mit ihren Krümmungen, zehn Rippenbögen, Brustbein, Schlüsselbeine, Schulterblätter, Kreuzbein, Darmbeine, Scham- und Sitzbeinäste, Oberschenkelknochen mit Kopf, Hals, Rollhügel und Kondylen, Kniescheibe, Schien- und Wadenbein, Knöchel, Fersenbein, Mittelfuß und Zehen, Oberarmknochen, Elle, Speiche, Handwurzel, Mittelhand und Fingerstrahlen), Gelenke an den Rig-Gelenken und Massen in den Segmentschwerpunkten; der Zeigestab trägt über Griff und Auszüge, die Fahne über die Stange und das Tuch an seinem Hohlsaum." },
];
