/** 🌦️ The 1:100, 1:200 and 1:500 models of `humans/person-with-umbrella`, each stated anew for its smallest feature (15 / 30 / 75 mm): fewer and plainer sections of the same figure, merged masses, the umbrella with its eight ribs down to 1:200 and eight whole gores at 1:500, a skeleton that turns from merged bones into hoops and bars and finally into sticks.
 *
 * Figure coordinates (`H`, `X`, `L`): x ahead, y up, z to the left, the default pose `umbrella-up-still`. Local stations (`st`): u along the bone, v its sagittal normal, w lateral.
 * @see ./r5-person-with-umbrella-s50.ts
 * @see ./r3-humans-person-with-umbrella.md */
import { ribCage } from "./hu-thorax.ts";
import type { Json } from "./volumes-lib.ts";
import { FRAMES, H, L, X, aboutW, capsule, dome, ellipsoid, freeLoft, loft, mirrored, part, revolve, st, thOf, toRight, tube, type St, type V3 } from "./r5-person-with-umbrella-kit.ts";
import { AZIMUTH, RIB, STRETCHER } from "./r5-person-with-umbrella-roots.ts";
import { BONE, COAT, SKIN, TONE, canopyPoint, crookPath, far, halfGore, kgOf, slices, crossSlices } from "./r5-person-with-umbrella-s50.ts";

type Scale = 100 | 200 | 500;
type Slice = readonly [number, number, number, number, number];
const F: Record<Scale, number> = { 100: 0.015, 200: 0.03, 500: 0.075 };
const add = (a: V3, b: readonly number[], k: number): V3 => [a[0] + b[0]! * k, a[1] + b[1]! * k, a[2] + b[2]! * k];
const sub = (a: V3, b: V3): V3 => [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
const len = (a: V3): number => Math.hypot(a[0], a[1], a[2]);
const unit = (a: V3): V3 => [a[0] / len(a), a[1] / len(a), a[2] / len(a)];
const cross = (a: V3, b: V3): V3 => [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
const dot = (a: V3, b: V3): number => a[0] * b[0] + a[1] * b[1] + a[2] * b[2];

/* ---------------------------------------------------------------- tables per scale */

/** 🧥 Raincoat above the waist per scale [y, back, front, half-width, exponent]. */
const COAT_UP: Record<Scale, Slice[]> = {
  100: [[0.995, -0.134, 0.137, 0.173, 2.3], [1.06, -0.131, 0.14, 0.177, 2.3], [1.15, -0.132, 0.148, 0.184, 2.3], [1.3, -0.14, 0.156, 0.194, 2.3], [1.385, -0.132, 0.14, 0.205, 2.4], [1.425, -0.112, 0.11, 0.19, 2.4], [1.462, -0.075, 0.07, 0.1, 2.1]],
  200: [[0.995, -0.134, 0.137, 0.173, 2.3], [1.25, -0.138, 0.155, 0.19, 2.3], [1.39, -0.13, 0.138, 0.205, 2.4], [1.44, -0.1, 0.095, 0.165, 2.3], [1.47, -0.07, 0.065, 0.09, 2]],
  500: [[0.985, -0.152, 0.154, 0.197, 2.3], [1.03, -0.14, 0.144, 0.184, 2.3], [1.12, -0.137, 0.148, 0.186, 2.3], [1.3, -0.14, 0.156, 0.194, 2.3], [1.42, -0.116, 0.116, 0.2, 2.5], [1.47, -0.075, 0.07, 0.1, 2]],
};
/** 🧥 Skirt of the coat per scale, waist to hem. */
const COAT_DOWN: Record<Scale, Slice[]> = {
  100: [[1.05, -0.13, 0.139, 0.174, 2.3], [0.98, -0.143, 0.14, 0.182, 2.3], [0.92, -0.159, 0.145, 0.196, 2.3], [0.7, -0.172, 0.156, 0.212, 2.4], [0.535, -0.182, 0.166, 0.226, 2.4]],
  200: [[1.05, -0.13, 0.139, 0.174, 2.3], [0.88, -0.164, 0.148, 0.201, 2.3], [0.535, -0.182, 0.166, 0.226, 2.4]],
  500: [[1.06, -0.137, 0.139, 0.183, 2.3], [0.84, -0.166, 0.15, 0.204, 2.35], [0.535, -0.182, 0.166, 0.226, 2.4]],
};
const HEAD_S: Record<Scale, Slice[]> = {
  100: [[1.493, 0.05, 0.086, 0.02, 2], [1.51, 0.012, 0.099, 0.044, 2], [1.545, -0.046, 0.1, 0.0605, 2.1], [1.59, -0.082, 0.098, 0.071, 2.1], [1.64, -0.09, 0.106, 0.0755, 2.1], [1.69, -0.078, 0.09, 0.069, 2], [1.7175, -0.03, 0.036, 0.032, 2]],
  200: [[1.495, 0.04, 0.09, 0.03, 2], [1.54, -0.04, 0.104, 0.06, 2.1], [1.61, -0.088, 0.106, 0.074, 2.1], [1.68, -0.084, 0.098, 0.072, 2], [1.718, -0.03, 0.036, 0.032, 2]],
  500: [[1.495, 0.03, 0.09, 0.035, 2], [1.56, -0.07, 0.106, 0.066, 2], [1.66, -0.09, 0.104, 0.075, 2], [1.72, -0.04, 0.04, 0.04, 2]],
};
const NECK_S: Record<Scale, Slice[]> = {
  100: [[1.47, -0.057, 0.051, 0.055, 2], [1.52, -0.052, 0.046, 0.054, 2], [1.56, -0.05, 0.042, 0.055, 2]],
  200: [[1.465, -0.058, 0.052, 0.056, 2], [1.56, -0.05, 0.042, 0.055, 2]],
  500: [[1.4, -0.06, 0.06, 0.058, 2], [1.54, -0.054, 0.05, 0.055, 2]],
};
const HAIR_S: Record<Scale, (readonly [number, number, number, number])[]> = {
  100: [[0.097, 1.668, 1.697, 0.05], [0.04, 1.643, 1.726, 0.079], [-0.03, 1.612, 1.727, 0.082], [-0.08, 1.562, 1.702, 0.068], [-0.097, 1.568, 1.666, 0.045]],
  200: [[0.095, 1.664, 1.7, 0.055], [0.0, 1.63, 1.73, 0.083], [-0.085, 1.565, 1.7, 0.066], [-0.1, 1.575, 1.655, 0.04]],
  500: [[0.088, 1.672, 1.7, 0.048], [0.04, 1.645, 1.727, 0.078], [-0.03, 1.612, 1.729, 0.082], [-0.08, 1.565, 1.703, 0.068], [-0.097, 1.57, 1.666, 0.045]],
};
const SLEEVE: Record<Scale, St[]> = {
  100: [st(-0.03, 0, -0.03, 0.042, 0.056), st(0.03, 0.002, 0.0, 0.06, 0.067), st(0.16, 0.003, 0.017, 0.056, 0.062), st(0.316, -0.002, 0.035, 0.051, 0.055)],
  200: [st(-0.02, 0, -0.025, 0.046, 0.058), st(0.06, 0.002, 0.005, 0.061, 0.067), st(0.316, -0.002, 0.035, 0.052, 0.056)],
  500: [st(-0.01, 0, -0.02, 0.05, 0.06), st(0.05, 0.002, 0.004, 0.062, 0.067), st(0.316, 0, 0.035, 0.056, 0.06)],
};
const FORE: Record<Scale, St[]> = {
  100: [st(0.0, 0, 0, 0.051, 0.054), st(0.11, 0, 0.009, 0.045, 0.047), st(0.222, 0, 0.018, 0.041, 0.042)],
  200: [st(0.0, 0, 0, 0.051, 0.054), st(0.222, 0, 0.018, 0.042, 0.043)],
  500: [st(0.0, 0, 0, 0.054, 0.056), st(0.2, 0, 0.016, 0.046, 0.046)],
};
const CUFF: Record<Scale, St[]> = {
  100: [st(0.208, 0, 0.017, 0.031, 0.034), st(0.25, 0, 0.02, 0.029, 0.032)],
  200: [st(0.2, 0, 0.017, 0.032, 0.034), st(0.255, 0, 0.02, 0.03, 0.032)],
  500: [st(0.19, 0, 0.016, 0.04, 0.041), st(0.268, 0, 0.02, 0.038, 0.039)],
};
const THIGH: Record<Scale, St[]> = {
  100: [st(0.0, -0.01, 0, 0.086, 0.098, 2.1), st(0.15, -0.003, -0.001, 0.087, 0.088, 2.05), st(0.32, 0.003, -0.004, 0.069, 0.073, 2), st(0.4202, 0.004, -0.004, 0.058, 0.065, 2)],
  200: [st(0.0, -0.01, 0, 0.086, 0.098, 2.1), st(0.22, 0, -0.002, 0.082, 0.083, 2), st(0.4202, 0.004, -0.004, 0.058, 0.065, 2)],
  500: [st(0.0, -0.008, 0, 0.085, 0.095, 2), st(0.2, 0, -0.002, 0.082, 0.085, 2), st(0.4202, 0.004, -0.004, 0.06, 0.066, 2)],
};
const SHANK: Record<Scale, St[]> = {
  100: [st(-0.02, 0.004, -0.004, 0.058, 0.065), st(0.16, -0.004, -0.004, 0.056, 0.064), st(0.33, 0, -0.002, 0.053, 0.062), st(0.395, 0.006, -0.002, 0.053, 0.065, 2, -9)],
  200: [st(-0.02, 0.004, -0.004, 0.058, 0.065), st(0.2, -0.004, -0.004, 0.055, 0.063), st(0.395, 0.006, -0.002, 0.053, 0.065, 2, -9)],
  500: [st(-0.03, 0.004, -0.004, 0.058, 0.064), st(0.395, 0.004, -0.002, 0.054, 0.065, 2, -9)],
};
/** 🥾 Shoe per scale as cross slices [x, bottom, top, half-width]: sole and heel merged into the upper. */
const SHOE_R: Record<Scale, (readonly [number, number, number, number])[]> = {
  100: [[-0.073, 0.0, 0.08, 0.0291], [-0.058, 0.0, 0.097, 0.0349], [-0.02, 0.0, 0.106, 0.0398], [0.03, 0.008, 0.1, 0.0436], [0.09, 0.0, 0.072, 0.0485], [0.132, 0.0, 0.058, 0.0504]],
  200: [[-0.073, 0.0, 0.085, 0.031], [-0.03, 0.0, 0.106, 0.0398], [0.06, 0.004, 0.084, 0.0466], [0.132, 0.0, 0.058, 0.0504]],
  500: [[-0.074, 0.0, 0.09, 0.034], [-0.02, 0.0, 0.106, 0.0407], [0.1, 0.0, 0.07, 0.051], [0.168, 0.0, 0.05, 0.049], [0.2016, 0.0, 0.028, 0.0194]],
};
const SHOE_T: Record<Scale, (readonly [number, number, number, number])[]> = {
  100: [[0.12, 0.0, 0.061, 0.0504], [0.16, 0.0, 0.051, 0.0475], [0.184, 0.0, 0.043, 0.0388], [0.2012, 0.003, 0.026, 0.0155]],
  200: [[0.12, 0.0, 0.061, 0.0504], [0.172, 0.0, 0.048, 0.0436], [0.2016, 0.0, 0.03, 0.0175]],
  500: [],
};

/* ---------------------------------------------------------------- shared pieces */

/** 🪢 A loft along a path bent in the plane of u and w, its width across along v (a curled hand). */
function bent(p: { id: string; bone: string; material: string; tone: number }, pts: readonly V3[], widths: readonly number[], thicks: readonly number[]): Json {
  const ex: V3 = [0, 1, 0];
  return freeLoft(p, pts.map((at, i) => {
    const t = unit(sub(pts[Math.min(i + 1, pts.length - 1)]!, pts[Math.max(i - 1, 0)]!));
    return { at, rx: widths[i]!, ry: thicks[i]!, ex, ey: unit(cross(t, ex)) };
  }));
}
const GRIP_DEG = 75;
const S_AX: V3 = [Math.cos((GRIP_DEG * Math.PI) / 180), Math.sin((GRIP_DEG * Math.PI) / 180), 0];
const Q_AX: V3 = [S_AX[1], -S_AX[0], 0];
const GRIP_C: V3 = [0.075, 0, 0.026];
/** ✊ The four fingers of the right fist as one band round the handle: sections `wide` along the handle, `thick` across. */
function fingerBand(id: string, wide: number, thick: number, n = 6): Json {
  const r = 0.012 + thick;
  const pts: V3[] = Array.from({ length: n }, (_, i) => {
    const a = ((-62 + (234 * i) / (n - 1)) * Math.PI) / 180;
    return [GRIP_C[0] + Q_AX[0] * r * Math.cos(a), GRIP_C[1] + Q_AX[1] * r * Math.cos(a), GRIP_C[2] + r * Math.sin(a)];
  });
  return freeLoft({ id, bone: "hand-r", material: "skin", tone: TONE.hand }, pts.map((at, i) => {
    const t = unit(sub(pts[Math.min(i + 1, n - 1)]!, pts[Math.max(i - 1, 0)]!));
    const ey = unit(cross(t, S_AX));
    return { at: add(at, S_AX, 0.002), rx: wide, ry: thick, ex: S_AX, ey };
  }));
}

/** 🌂 A whole gore at 1:500: the cloth from the middle of one panel over the rib `key` to the middle of the next, one elliptic section per station lying through the rib and both panel middles, `t` thick at the hem. */
function fullGore(key: string, stations: readonly number[], t: number): Json {
  const az = AZIMUTH[key]!;
  const bone = `rib-${key}`;
  const f = FRAMES[bone]!;
  const local = (d: V3): V3 => [dot(d, f.u), dot(d, f.v), dot(d, f.w)];
  return freeLoft({ id: `canopy-${key}`, bone, material: "umbrella-fabric", tone: TONE.canopy }, stations.map((s) => {
    const rib = canopyPoint(az, s);
    const a = canopyPoint(az - 22.5, s, Math.cos((22.5 * Math.PI) / 180));
    const b = canopyPoint(az + 22.5, s, Math.cos((22.5 * Math.PI) / 180));
    const mid: V3 = [(a[0] + b[0]) / 2, (a[1] + b[1]) / 2, (a[2] + b[2]) / 2];
    const sag = len(sub(rib, mid));
    const ry = Math.max(t / 2, sag / 2);
    const centre = add(mid, unit(sub(rib, mid)), sag / 2);
    const ex = unit(sub(b, a));
    const ey = sag > 1e-6 ? unit(sub(rib, mid)) : unit(cross(ex, f.u as V3));
    return { at: L(bone, centre), rx: Math.max(0.56 * len(sub(b, a)), 0.02), ry, ex: local(ex), ey: local(ey), e: 2.6 };
  }));
}

/* ---------------------------------------------------------------- body */

const pairs = (left: Json[]): Json[] => left.flatMap((s) => [mirrored(s, toRight), { ...s, tone: far(s.tone) }]);
const rnd = (x: number): number => Math.round(x * 1e5) / 1e5;

export function body(scale: Scale): Json[] {
  const out: Json[] = [];
  const f = F[scale];
  const head = { bone: "head", ...SKIN } as const;
  out.push(loft({ id: "neck", ...head }, slices("head", NECK_S[scale])));
  out.push(loft({ id: "head", ...head }, dome(slices("head", HEAD_S[scale]), 1, 0.004, 2)));
  if (scale === 100) {
    out.push(loft({ id: "head-nose", ...head }, [H("head", 1.556, 0.088, 0.122, 0.0165), H("head", 1.5675, 0.088, 0.1285, 0.0135), H("head", 1.6, 0.088, 0.11, 0.0085)]));
    for (const side of [1, -1]) out.push(ellipsoid({ id: `head-ear-${side > 0 ? "l" : "r"}`, ...head }, [0.031, 0.016, 0.0085], L("head", [-0.006, 1.597, side * 0.0765])));
  }
  if (scale === 200) out.push(loft({ id: "head-nose", ...head }, [H("head", 1.552, 0.07, 0.122, 0.016), H("head", 1.6, 0.07, 0.11, 0.009)]));
  const hairBand = HAIR_S[scale].map(([x, b, t, rw]) => X("head", x, b, t, rw, 2.3));
  out.push(loft({ id: "hair", bone: "head", material: "hair", tone: TONE.hair }, scale === 500 ? dome(dome(hairBand, -1, 0.008, 2), 1, 0.01, 2) : dome(hairBand, 1, 0.008, 2)));
  out.push(ellipsoid({ id: "hair-bun", bone: "head", material: "hair", tone: TONE.hair }, scale === 100 ? [0.034, 0.03, 0.036] : scale === 200 ? [0.036, 0.032, 0.036] : [0.0385, 0.0385, 0.0385], L("head", scale === 500 ? [-0.108, 1.6, 0] : [-0.112, 1.6, 0])));
  out.push(loft({ id: "shirt-collar", bone: "head", material: "cotton-clothing", tone: TONE.shirt }, scale === 500 ? [H("head", 1.445, -0.068, 0.066, 0.068, 2), H("head", 1.52, -0.062, 0.056, 0.062, 2)] : [H("head", 1.458, -0.066, 0.064, 0.066, 2), { ...H("head", 1.502, -0.062, 0.053, 0.061, 2), n: 7 }]));
  out.push(loft({ id: "coat", bone: "torso", ...COAT }, slices("torso", COAT_UP[scale])));
  out.push(loft({ id: "coat-skirt", bone: "pelvis", ...COAT }, slices("pelvis", COAT_DOWN[scale])));
  if (scale !== 500) {
    out.push(loft({ id: "coat-belt", bone: "pelvis", ...COAT, tone: 0.5 }, [H("pelvis", 0.99, -0.147, 0.146, 0.1875, 2.3), H("pelvis", 1.045, -0.135, 0.144, 0.1795, 2.3)]));
    out.push(loft({ id: "coat-collar", bone: "torso", ...COAT, tone: 0.5 }, [H("torso", 1.448, -0.09, 0.086, 0.11, 2.2), { ...H("torso", 1.494, -0.078, 0.054, 0.076, 2), n: 9 }]));
  }
  if (scale === 100) for (const side of [1, -1]) out.push(loft({ id: `coat-lapel-${side > 0 ? "l" : "r"}`, bone: "torso", ...COAT, tone: 0.5, role: "detail" }, [H("torso", 1.456, 0.02, 0.09, 0.032, 2.6, side * 0.076), H("torso", 1.42, 0.08, 0.15, 0.034, 2.6, side * 0.064), H("torso", 1.386, 0.128, 0.156, 0.016, 2.4, side * 0.05)]));
  const arm: Json[] = [];
  arm.push(loft({ id: "sleeve-l", bone: "upper-arm-l", ...COAT }, dome(SLEEVE[scale], 1, 0.04, scale === 500 ? 2 : 3)));
  arm.push(loft({ id: "forearm-l", bone: "forearm-l", ...COAT }, dome(FORE[scale], -1, 0.04, scale === 500 ? 2 : 3)));
  arm.push(loft({ id: "shirt-cuff-l", bone: "forearm-l", material: "cotton-clothing", tone: TONE.shirt }, CUFF[scale], scale === 500 ? [true, true] : [false, false]));
  out.push(...pairs(arm));
  const hand = { material: "skin", tone: TONE.hand } as const;
  const th = Math.max(f / 2, 0.0076);
  if (scale === 500) {
    out.push({ ...loft({ id: "hand-l", bone: "hand-l", ...hand }, [st(0.0, 0, 0, 0.0375, 0.03), st(0.07, 0.004, -0.004, 0.034, 0.045), st(0.135, 0.002, -0.012, 0.03, 0.04), st(0.172, 0.0, -0.024, 0.02, 0.026)]), tone: TONE.handFar });
    out.push(loft({ id: "hand-r", bone: "hand-r", ...hand }, [st(0.0, 0, 0, 0.0375, 0.03), st(0.06, 0.004, 0.012, 0.042, 0.044), st(0.1, 0.002, 0.026, 0.04, 0.042), st(0.12, 0.0, 0.028, 0.024, 0.03)]));
  } else {
    out.push({ ...bent({ id: "hand-l", bone: "hand-l", ...hand }, [[0.0, 0.002, 0], [0.05, 0.002, 0], [0.09, 0.002, -0.001], [0.128, 0.0, -0.008], [0.153, -0.002, -0.02], [0.168, -0.002, -0.033]], [0.028, 0.041, 0.043, 0.04, 0.037, 0.03], [0.019, Math.max(0.016, th), Math.max(0.0135, th), Math.max(0.01, th), Math.max(0.0088, th), Math.max(0.0078, th)]), tone: TONE.handFar });
    out.push({ ...tube({ id: "hand-l-thumb", bone: "hand-l", ...hand }, [[0.03, 0.03, -0.006], [0.062, 0.047, -0.013], [0.1, 0.054, -0.023]], scale === 100 ? [0.0115, 0.0098, 0.0082] : [0.015, 0.015, 0.015]), tone: TONE.handFar });
    out.push(loft({ id: "hand-r", bone: "hand-r", ...hand }, dome([st(0.0, 0, 0, 0.019, 0.028, 2.4), st(0.035, 0.002, -0.002, Math.max(0.016, th), 0.04, 2.6), st(0.075, 0.004, -0.003, Math.max(0.014, th), 0.044, 2.6)], 1, 0.008, 2)));
    out.push(fingerBand("hand-r-fingers", scale === 100 ? 0.037 : 0.039, Math.max(0.0095, th)));
    out.push(tube({ id: "hand-r-thumb", bone: "hand-r", ...hand }, [[0.03, 0.03, 0.006], [0.055, 0.05, 0.034], [0.085, 0.055, 0.052]], scale === 100 ? [0.012, 0.0105, 0.0088] : [0.015, 0.015, 0.015]));
  }
  const leg: Json[] = [];
  const cloth = { material: "cotton-clothing", tone: TONE.trousers } as const;
  const shoe = { material: "leather-shoe", tone: TONE.shoe } as const;
  leg.push(loft({ id: "thigh-l", bone: "thigh-l", ...cloth }, THIGH[scale]));
  leg.push(loft({ id: "shank-l", bone: "shank-l", ...cloth }, SHANK[scale]));
  leg.push(loft({ id: "shoe-l", bone: "foot-l", ...shoe }, crossSlices("foot-l", SHOE_R[scale], 2.5)));
  if (SHOE_T[scale].length > 0) leg.push(loft({ id: "shoe-toe-l", bone: "toe-l", ...shoe }, crossSlices("toe-l", SHOE_T[scale], 2.5)));
  out.push(...pairs(leg));
  out.push(...umbrella(scale));
  return out;
}

function umbrella(scale: Scale): Json[] {
  const out: Json[] = [];
  const metal = { material: "umbrella-metal", tone: TONE.metal } as const;
  const f = F[scale];
  out.push(capsule({ id: "shaft", bone: "umbrella", ...metal }, [0.05, 0, 0], [scale === 500 ? 0.84 : 0.8, 0, 0], 0.005, 0.005, { slender: true }));
  if (scale !== 500) {
    const r = scale === 100 ? 0.012 : 0.015;
    out.push(tube({ id: "handle", bone: "umbrella", material: "umbrella-handle", tone: TONE.wood }, crookPath(scale === 100 ? 6 : 4), [r]));
    out.push(revolve({ id: "ferrule", bone: "umbrella", ...metal, role: "detail" }, scale === 100 ? [[0.768, 0], [0.768, 0.0095], [0.776, 0.015], [0.79, 0.016], [0.81, 0.0085], [0.84, 0.0055], [0.84, 0]] : [[0.765, 0], [0.765, 0.015], [0.79, 0.016], [0.845, 0.0065], [0.845, 0]], [0, 0, 0], [1, 0, 0]));
  }
  if (scale === 100) out.push(revolve({ id: "runner", bone: "runner", ...metal, role: "detail" }, [[-0.034, 0], [-0.034, 0.0085], [-0.006, 0.0085], [-0.006, 0.0135], [0.012, 0.0135], [0.012, 0]], [0, 0, 0], [1, 0, 0]));
  for (const key of Object.keys(AZIMUTH)) {
    const bone = `rib-${key}`;
    if (scale !== 500) {
      out.push(tube({ id: `rib-${key}`, bone, ...metal, role: "detail" }, [0.014, 0.14, 0.28, 0.42, 0.556].map((s) => L(bone, canopyPoint(AZIMUTH[key]!, s, 0.99))), [0.0022], { slender: true, ...(key === "front" || key === "back" ? { texture: true } : {}) }));
      out.push(capsule({ id: `stretcher-${key}`, bone: `stretcher-${key}`, ...metal, role: "detail" }, [0, 0, 0], [STRETCHER, 0, 0], 0.0018, 0.0018, { slender: true }));
      const stations = scale === 100 ? [0.02, 0.14, 0.28, 0.42, 0.556] : [0.03, 0.29, 0.556];
      out.push(halfGore(key, 1, stations, rnd(f + 0.0001), `canopy-${key}-a`));
      out.push(halfGore(key, -1, stations, rnd(f + 0.0001), `canopy-${key}-b`));
    } else out.push(fullGore(key, [0.04, 0.3, 0.556], 0.0751));
  }
  return out;
}

/* ---------------------------------------------------------------- thermal */

const REGION: Record<string, string> = { "rain-shell": "coat", "umbrella-fabric": "canopy", "umbrella-metal": "umbrella-frame", "umbrella-handle": "umbrella-frame", "leather-shoe": "shoes", hair: "hair" };
export function thermal(scale: Scale, solids: readonly Json[]): Json[] {
  const f = F[scale];
  const out: Json[] = [];
  for (const s of solids) {
    let region = REGION[s.material];
    if (s.material === "skin") region = /^hand/.test(s.id) ? "hand-skin" : "head-skin";
    if (s.material === "cotton-clothing") region = s.id === "shirt-collar" ? "torso-clothing" : /^shirt-cuff/.test(s.id) ? "arm-clothing" : /^shank/.test(s.id) ? "leg-clothing" : undefined;
    if (region !== undefined) out.push(thOf(s.id, region));
  }
  const r = Math.max(f / 2, 0.0076);
  const nose: V3 = scale === 100 ? [0.1185, 1.5565, 0] : scale === 200 ? [0.112, 1.556, 0] : [0.098, 1.556, 0];
  out.push({ ...ellipsoid({ id: "th-nose-breath", bone: "head", ...SKIN }, [r, r, Math.max(r, 0.0105)], L("head", nose)), region: "head-skin", tags: ["emitter", "nose-breath"] });
  out.push({ ...ellipsoid({ id: "th-mouth-breath", bone: "head", ...SKIN }, [r, r, Math.max(r, 0.0075)], L("head", [scale === 500 ? 0.088 : 0.1, 1.5345, 0])), region: "head-skin", tags: ["emitter", "mouth-breath"] });
  const lo = scale === 500 ? 1.37 : 1.4;
  out.push({ ...loft({ id: "th-plume", bone: "torso", material: "rain-shell", tone: 0.7 }, [H("torso", lo, -0.145, -0.02, 0.17, 2.6), H("torso", 1.448, -0.1, -0.02, 0.155, 2.4)]), region: "coat", tags: ["emitter", "plume"] });
  return out;
}

/* ---------------------------------------------------------------- structure */

export function structure(scale: Scale, solids: readonly Json[]): Json[] {
  const f = F[scale];
  const out: Json[] = [];
  const m = (solid: Json, member: string, kind: "member" | "joint" | "mass" = "member"): void => { out.push(part(solid, member, kind)); };
  const T = (x: number, y: number, z = 0): V3 => L("torso", [x, y, z]);
  const P = (x: number, y: number, z = 0): V3 => L("pelvis", [x, y, z]);
  const HD = (x: number, y: number, z = 0): V3 => L("head", [x, y, z]);
  const rr = (r: number): number => Math.max(r, f / 2);
  const joint = (bone: string, at: V3, r: number): void => m(ellipsoid({ id: `joint-${bone}`, bone, material: "bone", tone: 0.4 }, [rr(r), rr(r), rr(r)], at), bone, "joint");
  const mass = (bone: string): void => { const k = kgOf(bone); const r = rr(0.0125 * Math.cbrt(k.kg)); m(ellipsoid({ id: `mass-${bone}`, bone, material: "bone", tone: 0.2, role: "detail" }, [r, r, r], [k.comX, k.comY, 0]), bone, "mass"); };
  const spine: [number, number][] = scale === 500 ? [[-0.03, 1.03], [-0.05, 1.25], [-0.045, 1.44]] : [[-0.036, 1.03], [-0.022, 1.115], [-0.048, 1.225], [-0.064, 1.345], [-0.044, 1.45]];
  m(tube({ id: "spine", bone: "torso", ...BONE }, spine.map(([x, y]) => T(x, y)), [rr(0.02), rr(0.018), rr(0.016), rr(0.0145), rr(0.0125)]), "torso");
  m(tube({ id: "cervical-spine", bone: "head", ...BONE }, [HD(-0.044, 1.45), HD(-0.02, 1.535), HD(-0.014, 1.578)], [rr(0.0125)]), "head");
  if (scale === 500) {
    m(ellipsoid({ id: "skull", bone: "head", ...BONE }, [0.07, 0.095, 0.0685], HD(0.012, 1.63)), "head");
    m(capsule({ id: "shoulder-girdle", bone: "torso", ...BONE }, T(-0.03, 1.41, 0.17), T(-0.03, 1.41, -0.17), 0.0375, 0.0375), "torso");
    out.push(...ribCage("person-with-umbrella", scale));
    m(capsule({ id: "pelvic-girdle", bone: "pelvis", ...BONE }, P(-0.02, 0.95, 0.09), P(-0.02, 0.95, -0.09), 0.0375, 0.0375), "pelvis");
  } else {
    m(ellipsoid({ id: "cranium", bone: "head", ...BONE }, [0.066, 0.09, 0.0685], HD(0.004, 1.648)), "head");
    m(ellipsoid({ id: "face-bones", bone: "head", ...BONE }, [0.04, scale === 100 ? 0.034 : 0.04, 0.05], HD(0.058, 1.575)), "head");
    out.push(...ribCage("person-with-umbrella", scale));
    m(loft({ id: "sternum", bone: "torso", ...BONE }, [H("torso", 1.228, 0.1 - rr(0.006), 0.123, rr(0.012)), H("torso", 1.395, 0.064, 0.064 + 2 * rr(0.006), rr(0.027))]), "torso");
    if (scale === 100) {
      for (const side of [1, -1]) {
        const n = side > 0 ? "l" : "r";
        m(capsule({ id: `clavicle-${n}`, bone: "torso", ...BONE }, T(0.066, 1.403, side * 0.022), T(-0.012, 1.432, side * 0.178), 0.0078, 0.0078), "torso");
        m(ellipsoid({ id: `scapula-${n}`, bone: "torso", ...BONE }, [0.078, 0.0078, 0.05], T(-0.078, 1.335, side * 0.104)), "torso");
        m(ellipsoid({ id: `ilium-${n}`, bone: "pelvis", ...BONE }, [0.066, 0.072, 0.0078], P(-0.026, 0.99, side * 0.098)), "pelvis");
        m(capsule({ id: `ilium-body-${n}`, bone: "pelvis", ...BONE }, P(-0.056, 0.992, side * 0.034), P(-0.002, 0.927, side * 0.084), 0.017, 0.019), "pelvis");
        m(capsule({ id: `ischiopubic-${n}`, bone: "pelvis", ...BONE }, P(0.052, 0.905, side * 0.012), P(-0.042, 0.858, side * 0.058), 0.012, 0.013), "pelvis");
        m(capsule({ id: `mandible-${n}`, bone: "head", ...BONE }, HD(0.004, 1.585, side * 0.05), HD(0.07, 1.508, side * 0.008), 0.0078, 0.0085), "head");
      }
      m(tube({ id: "sacrum", bone: "pelvis", ...BONE }, [P(-0.038, 1.03), P(-0.066, 0.97), P(-0.07, 0.915)], [0.028, 0.022, 0.0078]), "pelvis");
    } else {
      m(capsule({ id: "shoulder-girdle", bone: "torso", ...BONE }, T(0.02, 1.42, 0.17), T(0.02, 1.42, -0.17), 0.015, 0.015), "torso");
      m(revolve({ id: "pelvic-ring", bone: "pelvis", ...BONE }, Array.from({ length: 8 }, (_, k) => [0.03 * Math.cos((k * Math.PI) / 4), 0.075 + 0.016 * Math.sin((k * Math.PI) / 4)] as [number, number]), P(-0.01, 0.955), [0.97, 0.24, 0], { lateralScale: 1.3 }), "pelvis");
    }
  }
  joint("torso", [0, 0, 0], 0.02);
  joint("head", [0, 0, 0], 0.016);
  mass("torso");
  if (scale !== 500) { mass("pelvis"); mass("head"); }
  const side: Json[] = [];
  const sm = (solid: Json, member: string, kind: "member" | "joint" | "mass" = "member"): void => { side.push(part(solid, member, kind)); };
  const sj = (bone: string, at: V3, r: number): void => sm(ellipsoid({ id: `joint-${bone}`, bone, material: "bone", tone: 0.4 }, [rr(r), rr(r), rr(r)], at), bone, "joint");
  if (scale === 500) {
    sm(capsule({ id: "femur-l", bone: "thigh-l", ...BONE }, [0.03, 0, 0.02], [0.4, 0, 0], 0.0375, 0.0375), "thigh-l");
    sm(capsule({ id: "tibia-l", bone: "shank-l", ...BONE }, [0.03, 0, 0], [0.4, 0, -0.005], 0.0375, 0.0375), "shank-l");
    sm(capsule({ id: "foot-bones-l", bone: "foot-l", ...BONE }, L("foot-l", [-0.03, 0.045, FRAMES["foot-l"]!.o[2]]), L("foot-l", [0.17, 0.0375, FRAMES["foot-l"]!.o[2]]), 0.0375, 0.0375), "foot-l");
    sm(capsule({ id: "humerus-l", bone: "upper-arm-l", ...BONE }, [0.02, 0, 0], [0.3, 0, 0.033], 0.0375, 0.0375), "upper-arm-l");
    sm(capsule({ id: "forearm-bones-l", bone: "forearm-l", ...BONE }, [0.02, 0, 0.004], [0.23, 0, 0.018], 0.0375, 0.0375), "forearm-l");
    sj("thigh-l", [0, 0, 0], 0.0375);
    sj("shank-l", [0, 0, 0], 0.0375);
  } else {
    const r = (x: number): number => rr(x);
    sm(tube({ id: "femur-l", bone: "thigh-l", ...BONE }, [[0.0, 0, 0], [0.034, -0.006, 0.058], scale === 100 ? [0.13, 0.008, 0.046] : [0.2, 0.012, 0.03], [0.398, 0.0, 0.002]], [r(0.022), r(0.0175), r(0.0145), r(0.017)]), "thigh-l");
    sm(ellipsoid({ id: "femur-condyles-l", bone: "thigh-l", ...BONE }, [r(0.021), 0.031, 0.041], [0.412, -0.006, 0]), "thigh-l");
    sm(loft({ id: "tibia-l", bone: "shank-l", ...BONE }, [st(0.02, -0.002, 0, r(0.026), r(0.022)), st(0.07, 0.007, -0.002, r(0.017), r(0.017)), st(0.15, 0.011, -0.003, r(0.0135), r(0.0145)), st(0.3, 0.008, -0.003, r(0.0125), r(0.013)), st(0.404, 0.002, -0.002, r(0.0155), r(0.014))]), "shank-l");
    if (scale === 100) {
      sm(ellipsoid({ id: "patella-l", bone: "thigh-l", ...BONE }, [0.021, 0.0095, 0.02], [0.41, 0.042, 0]), "thigh-l");
      sm(capsule({ id: "fibula-l", bone: "shank-l", ...BONE }, [0.04, -0.018, 0.032], [0.424, -0.012, 0.028], 0.0076, 0.0076), "shank-l");
    }
    sm(loft({ id: "foot-bones-l", bone: "foot-l", ...BONE }, [X("foot-l", -0.06, 0.03, 0.07, r(0.02)), X("foot-l", 0.0, 0.03, 0.088, 0.026), X("foot-l", 0.08, 0.026, 0.06, 0.036), X("foot-l", 0.142, 0.018, 0.018 + 2 * r(0.01), 0.04)]), "foot-l");
    sm(loft({ id: "phalanges-l", bone: "toe-l", ...BONE }, [X("toe-l", 0.13, 0.016, 0.016 + 2 * r(0.011), 0.04), X("toe-l", 0.188, 0.014, 0.014 + 2 * r(0.0075), r(0.024))]), "toe-l");
    sm(tube({ id: "humerus-l", bone: "upper-arm-l", ...BONE }, [[0.002, 0, -0.012], [0.1, 0.004, 0.008], [0.3, 0, 0.033]], [r(0.022), r(0.0108), r(0.0125)]), "upper-arm-l");
    sm(ellipsoid({ id: "humerus-condyles-l", bone: "upper-arm-l", ...BONE }, [r(0.012), r(0.014), 0.031], [0.31, 0, 0.035]), "upper-arm-l");
    if (scale === 100) {
      sm(capsule({ id: "ulna-l", bone: "forearm-l", ...BONE }, [-0.016, -0.014, 0.004], [0.236, -0.004, 0.01], 0.0085, 0.0076), "forearm-l");
      sm(capsule({ id: "radius-l", bone: "forearm-l", ...BONE }, [0.012, 0.004, 0.022], [0.242, 0.006, 0.03], 0.0076, 0.009), "forearm-l");
    } else sm(capsule({ id: "forearm-bones-l", bone: "forearm-l", ...BONE }, [0.0, -0.004, 0.012], [0.24, 0.0, 0.02], 0.015, 0.015), "forearm-l");
    sj("thigh-l", [0, 0, 0], 0.024);
    sj("shank-l", [0, 0, 0], 0.02);
    sj("upper-arm-l", [0, 0, 0], 0.016);
    if (scale === 100) { sj("foot-l", [0, 0, 0], 0.016); sj("forearm-l", [0, 0, 0], 0.014); }
    side.push(part(ellipsoid({ id: "mass-thigh-l", bone: "thigh-l", material: "bone", tone: 0.2, role: "detail" }, [rr(0.027), rr(0.027), rr(0.027)], [kgOf("thigh-l").comX, 0, 0]), "thigh-l", "mass"));
  }
  for (const s of side) out.push(s, mirrored(s, toRight));
  for (const n of ["l", "r"] as const) {
    const b = `hand-${n}`;
    if (scale === 500) m(capsule({ id: `hand-bones-${n}`, bone: b, ...BONE }, [0.0, 0, 0], [0.09, 0, n === "r" ? 0.02 : -0.01], 0.0375, 0.0375), b);
    else {
      m(loft({ id: `metacarpus-${n}`, bone: b, ...BONE }, [st(0.004, 0, 0, rr(0.009), 0.022, 2.4), st(0.09, 0.0, n === "r" ? 0.006 : 0, rr(0.0076), 0.034, 2.4)]), b);
      if (scale === 100) {
        const body = solids.find((s) => s.id === (n === "r" ? "hand-r-fingers" : "hand-l"))!;
        if (n === "r") m({ ...body, id: "finger-rays-r", material: "bone", tone: TONE.bone, sections: body.sections.map((q: Json) => ({ ...q, rx: rnd(q.rx * 0.82), ry: 0.0076 })) }, b);
        else m(bent({ id: "finger-rays-l", bone: b, material: "bone", tone: TONE.bone }, [[0.09, 0.002, -0.001], [0.122, 0.0, -0.011], [0.136, -0.002, -0.028], [0.14, -0.002, -0.042]], [0.032, 0.03, 0.028, 0.024], [0.0076, 0.0076, 0.0076, 0.0076]), b);
        m(tube({ id: `thumb-ray-${n}`, bone: b, ...BONE }, n === "r" ? [[0.03, 0.03, 0.006], [0.055, 0.05, 0.034], [0.08, 0.054, 0.05]] : [[0.03, 0.03, -0.006], [0.062, 0.047, -0.013], [0.095, 0.053, -0.022]], [0.0076]), b);
      }
    }
  }
  const has = (id: string): boolean => solids.some((s) => s.id === id);
  if (has("shaft")) out.push({ id: "member-shaft", of: "shaft", member: "umbrella", part: "member" });
  if (has("handle")) out.push({ id: "member-handle", of: "handle", member: "umbrella", part: "member" });
  if (scale !== 100) m(revolve({ id: "member-notch", bone: "umbrella", material: "umbrella-metal", tone: 0.4 }, [[0.78 - f / 2, 0], [0.78 - f / 2, f / 2], [0.78 + f / 2, f / 2], [0.78 + f / 2, 0]], [0, 0, 0], [1, 0, 0]), "umbrella", "joint");
  else m(revolve({ id: "member-notch", bone: "umbrella", material: "umbrella-metal", tone: 0.4 }, [[0.77, 0], [0.77, 0.014], [0.786, 0.014], [0.786, 0]], [0, 0, 0], [1, 0, 0]), "umbrella", "joint");
  if (has("runner")) out.push({ id: "member-runner", of: "runner", member: "umbrella", part: "joint" });
  for (const key of Object.keys(AZIMUTH)) {
    const rib = `rib-${key}`;
    const str = `stretcher-${key}`;
    if (has(rib)) out.push({ id: `member-${rib}`, of: rib, member: rib, part: "member" });
    else m(tube({ id: `member-${rib}`, bone: rib, material: "umbrella-metal", tone: TONE.metal }, [0.014, 0.28, 0.556].map((s) => L(rib, canopyPoint(AZIMUTH[key]!, s, 0.97))), [0.004], { slender: true }), rib);
    if (has(str)) out.push({ id: `member-${str}`, of: str, member: str, part: "member" });
    else m(capsule({ id: `member-${str}`, bone: str, material: "umbrella-metal", tone: TONE.metal }, [0, 0, 0], [STRETCHER, 0, 0], 0.003, 0.003, { slender: true }), str);
  }
  return out;
}

export const DECISIONS: Record<Scale, Json[]> = {
  100: [
    { en: "1:100 body (smallest feature 15 mm): the same figure in fewer sections: head with nose and ears, hair swept into the bun, neck in the shirt collar; raincoat upper part and flaring skirt, belt, collar and lapels (buttons, pockets, placket, yoke and folds are below the feature and left out); sleeves with the elbow carried outwards and shirt cuffs; the left hand as palm and curled fingers in one form with the thumb, the right fist as palm, one finger band round the handle and the thumb; trouser legs, shoes with sole and heel merged into the upper; the umbrella with crook handle, shaft, runner, ferrule, eight ribs, eight stretchers and sixteen half panels 15 mm thick.", de: "1:100 Körper (kleinstes Merkmal 15 mm): dieselbe Figur mit weniger Schnitten: Kopf mit Nase und Ohren, das Haar in den Knoten genommen, Hals im Hemdkragen; Regenmantel mit Oberteil und ausgestelltem Schoß, Gürtel, Kragen und Revers (Knöpfe, Taschen, Knopfleiste, Passe und Falten liegen unter dem Merkmal und entfallen); Ärmel mit nach außen getragenem Ellbogen und Hemdmanschetten; die linke Hand als Handfläche und gekrümmte Finger in einer Form mit dem Daumen, die rechte Faust als Handfläche, ein Fingerband um den Griff und der Daumen; Hosenbeine, Schuhe mit in den Schaft gezogener Sohle und Absatz; der Schirm mit Rundhakengriff, Stock, Schieber, Zwinge, acht Rippen, acht Streben und sechzehn 15 mm dicken halben Stoffbahnen." },
    { en: "1:100 thermal: every region is the outer surface of its body solids by reference (the layers of coat, trousers and shoes are merged into their outer shell, B2); breath leaves at nostril and mouth patches of the feature size, the plume from a patch on the coat shoulders.", de: "1:100 thermisch: Jede Region ist per Verweis die Außenfläche ihrer Körper (die Schichten von Mantel, Hose und Schuhen sind in ihre äußere Hülle zusammengefasst, B2); der Atem tritt an Nasen- und Mundfeldern in Merkmalsgröße aus, die Warmluftfahne an einem Feld auf den Mantelschultern." },
    { en: "1:100 structure: a skeleton of merged bones: skull, face and jaw, cervical and thoracolumbar spine, six bilateral rib courses including a floating pair, sternum, clavicles, scapulae, ilia, sacrum and ischiopubic bars, femur with neck and condyles, patella, tibia, fibula, one foot and one toe form, humerus, ulna, radius, metacarpus and finger rays; joints, masses; the umbrella frame by reference (shaft, handle, runner, ribs, stretchers) with the notch as its joint.", de: "1:100 Tragwerk: ein Skelett aus zusammengefassten Knochen: Schädel, Gesicht und Kiefer, Hals- und Brust-Lenden-Wirbelsäule, sechs beidseitige Rippenzüge mit einem freien Paar, Brustbein, Schlüsselbeine, Schulterblätter, Darmbeine, Kreuzbein und Sitz-Scham-Stege, Oberschenkelknochen mit Hals und Kondylen, Kniescheibe, Schien- und Wadenbein, je eine Fuß- und Zehenform, Oberarmknochen, Elle, Speiche, Mittelhand und Fingerstrahlen; Gelenke, Massen; das Schirmgestell per Verweis (Stock, Griff, Schieber, Rippen, Streben) mit der Krone als Gelenk." },
  ],
  200: [
    { en: "1:200 body (smallest feature 30 mm): head with the nose as one form, hair and bun, neck and shirt collar; coat upper part, skirt, belt and collar; sleeves, shirt cuffs; the left hand curled in one form with a thumb, the right fist with a finger band and thumb round the handle; trouser legs in three sections, shoe and toe cap; the umbrella with a 30 mm crook, ferrule, shaft, eight ribs and stretchers as true-size slender rods and sixteen half panels 30 mm thick (the runner merges into the shaft, ears and face details are below the feature).", de: "1:200 Körper (kleinstes Merkmal 30 mm): Kopf mit der Nase als eine Form, Haar und Knoten, Hals und Hemdkragen; Mantel-Oberteil, Schoß, Gürtel und Kragen; Ärmel, Hemdmanschetten; die linke Hand gekrümmt in einer Form mit Daumen, die rechte Faust mit Fingerband und Daumen um den Griff; Hosenbeine in drei Schnitten, Schuh und Zehenkappe; der Schirm mit 30 mm dickem Rundhaken, Zwinge, Stock, acht Rippen und Streben als schlanke Stäbe in wahrer Größe und sechzehn 30 mm dicken halben Stoffbahnen (der Schieber geht im Stock auf, Ohren und Gesichtsdetails liegen unter dem Merkmal)." },
    { en: "1:200 thermal: every region by reference to its outer body solids, layers merged (B2); breath and plume patches of the feature size.", de: "1:200 thermisch: Jede Region per Verweis auf ihre äußeren Körper, Schichten zusammengefasst (B2); Atem- und Fahnenfelder in Merkmalsgröße." },
    { en: "1:200 structure: skull and face, cervical bar, spine, three bilateral rib courses with the lower costal arch, sternum, a shoulder girdle bar, a pelvic ring, one bone per limb segment with knee and elbow condyles, foot and toe forms, metacarpal plates; joints at hips, knees, shoulders, trunk and neck, masses of trunk, pelvis, head and thighs; ribs and stretchers by reference, a notch ring as the joint of the umbrella.", de: "1:200 Tragwerk: Schädel und Gesicht, Halsstab, Wirbelsäule, drei beidseitige Rippenzüge mit dem unteren Rippenbogen, Brustbein, ein Schultergürtelstab, ein Beckenring, ein Knochen je Gliedabschnitt mit Knie- und Ellbogenkondylen, Fuß- und Zehenformen, Mittelhandplatten; Gelenke an Hüften, Knien, Schultern, Rumpf und Hals, Massen von Rumpf, Becken, Kopf und Oberschenkeln; Rippen und Streben per Verweis, ein Kronenring als Gelenk des Schirms." },
  ],
  500: [
    { en: "1:500 body (smallest feature 75 mm): head, hair swept back into the low bun (both at the 75 mm floor), neck in the collar; coat upper part and skirt; sleeves, forearms with their cuffs, the hands as single forms (the right one closed round the shaft, the crook handle and the ferrule are below the feature); trouser legs in two sections, one shoe per foot; the canopy as eight whole gores, each from the middle of one panel over its rib to the next, 75 mm thick, on the true-size shaft.", de: "1:500 Körper (kleinstes Merkmal 75 mm): Kopf, das Haar zurück in den tiefen Knoten genommen (beide an der 75-mm-Grenze), Hals im Kragen; Mantel-Oberteil und Schoß; Ärmel, Unterarme mit ihren Manschetten, die Hände als je eine Form (die rechte um den Stock geschlossen, Rundhakengriff und Zwinge liegen unter dem Merkmal); Hosenbeine in zwei Schnitten, ein Schuh je Fuß; das Schirmdach als acht ganze, 75 mm dicke Bahnen, jede von der Mitte eines Felds über ihre Rippe zum nächsten, auf dem Stock in wahrer Größe." },
    { en: "1:500 thermal: every region by reference to its body solids, layers merged (B2); breath and plume patches of the feature size.", de: "1:500 thermisch: Jede Region per Verweis auf ihre Körper, Schichten zusammengefasst (B2); Atem- und Fahnenfelder in Merkmalsgröße." },
    { en: "1:500 structure: a stick skeleton at the 75 mm floor: skull, cervical bar, spine, two open paired rib courses and a sternal bar, shoulder and pelvic bars, one stick per limb segment and foot, hand sticks; joints at hips, knees, trunk and neck, the trunk mass; the umbrella as shaft by reference (it runs into the fist, the crook is below the feature), slender ribs and stretchers of its own (the body draws only the canopy over them) and a notch ring.", de: "1:500 Tragwerk: ein Stabskelett an der Grenze von 75 mm: Schädel, Halsstab, Wirbelsäule, zwei offene Rippenpaare und ein Brustbeinstab, Schulter- und Beckenstab, ein Stab je Gliedabschnitt und Fuß, Handstäbe; Gelenke an Hüften, Knien, Rumpf und Hals, die Rumpfmasse; der Schirm als Stock per Verweis (er läuft in die Faust, der Haken liegt unter dem Merkmal), eigene schlanke Rippen und Streben (der Körper zeigt nur das Dach darüber) und einem Kronenring." },
  ],
};
