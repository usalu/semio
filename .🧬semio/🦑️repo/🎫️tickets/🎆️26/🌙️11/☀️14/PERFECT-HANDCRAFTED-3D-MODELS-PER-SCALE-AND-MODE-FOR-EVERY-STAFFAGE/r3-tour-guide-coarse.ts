/** 🧭 The 1:100, 1:200 and 1:500 models of `humans/tour-guide`, each stated anew for its smallest feature (15 / 30 / 75 mm): fewer and plainer sections of the same figure, merged masses (cap and hair, hands, shoe and sole), the flag, pole and pointer kept at their true size where they print, and a skeleton that turns from merged bones into hoops and bars and finally into sticks.
 *
 * Figure coordinates (`HA`, `L`): x ahead, y up, z to the left, the default pose `standing-pointing`. Local stations (`st`): u along the bone, v its sagittal normal, w lateral.
 * @see ./r3-tour-guide-s50.ts
 * @see ./r3-humans-tour-guide.md */
import type { Json } from "./volumes-lib.ts";
import { FRAMES, H, L, X, capsule, dome, ellipsoid, freeLoft, loft, mirrored, part, revolve, st, thOf, toRight, tube, type St, type V3 } from "./r3-tour-guide-kit.ts";
import { BONE, CLOTH, HA, HD, JACKET, METAL, P, POLE, SHOE, SKIN, T, TONE, YV, axisX, far, fist, headSlices, kgOf, lanyardPath, shoeSlices, slices, surfaceX, type Cross, type Slice } from "./r3-tour-guide-s50.ts";

type Scale = 100 | 200 | 500;
const F: Record<Scale, number> = { 100: 0.015, 200: 0.03, 500: 0.075 };
const sub = (a: V3, b: V3): V3 => [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
const len = (a: V3): number => Math.hypot(a[0], a[1], a[2]);
const unit = (a: V3): V3 => [a[0] / len(a), a[1] / len(a), a[2] / len(a)];
const cross = (a: V3, b: V3): V3 => [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];

/* ---------------------------------------------------------------- tables per scale */

const HEAD_S: Record<Scale, Slice[]> = {
  100: [[0.233, 0.056, 0.094, 0.022, 2], [0.215, 0.012, 0.104, 0.044, 2], [0.178, -0.046, 0.106, 0.06, 2.1], [0.13, -0.088, 0.104, 0.072, 2.1], [0.08, -0.098, 0.108, 0.0775, 2.1], [0.03, -0.085, 0.086, 0.068, 2], [0.002, -0.026, 0.024, 0.022, 2]],
  200: [[0.232, 0.048, 0.096, 0.03, 2], [0.19, -0.03, 0.106, 0.056, 2.1], [0.12, -0.09, 0.106, 0.074, 2.1], [0.05, -0.096, 0.1, 0.075, 2], [0.004, -0.03, 0.03, 0.026, 2]],
  500: [[0.232, 0.04, 0.096, 0.038, 2], [0.16, -0.075, 0.106, 0.068, 2], [0.07, -0.098, 0.108, 0.078, 2], [0.0, -0.04, 0.04, 0.038, 2]],
};
const NECK_S: Record<Scale, Slice[]> = {
  100: [[1.45, -0.066, 0.05, 0.062, 2], [1.53, -0.06, 0.048, 0.056, 2], [1.59, -0.064, 0.02, 0.056, 2]],
  200: [[1.45, -0.066, 0.05, 0.062, 2], [1.585, -0.062, 0.03, 0.056, 2]],
  500: [[1.44, -0.068, 0.054, 0.064, 2], [1.57, -0.062, 0.04, 0.058, 2]],
};
const HAIR_S: Record<Scale, Slice[]> = {
  100: [[0.165, -0.08, -0.05, 0.048, 2], [0.125, -0.098, -0.02, 0.064, 2.1], [0.1, -0.105, 0.03, 0.071, 2.1], [0.08, -0.106, 0.1, 0.0855, 2.1], [0.03, -0.093, 0.094, 0.076, 2], [-0.006, -0.03, 0.03, 0.026, 2]],
  200: [[0.16, -0.085, -0.045, 0.052, 2], [0.1, -0.106, 0.03, 0.072, 2.1], [0.07, -0.106, 0.1, 0.085, 2.1], [0.02, -0.085, 0.085, 0.07, 2], [-0.006, -0.03, 0.03, 0.026, 2]],
  500: [],
};
const CAP_S: Record<Scale, Slice[]> = {
  100: [[0.09, -0.114, 0.117, 0.091, 2.2], [0.05, -0.115, 0.117, 0.093, 2.2], [0.012, -0.093, 0.095, 0.08, 2], [-0.0145, -0.032, 0.034, 0.028, 2]],
  200: [[0.09, -0.114, 0.117, 0.092, 2.2], [0.04, -0.112, 0.114, 0.092, 2.1], [-0.015, -0.035, 0.036, 0.03, 2]],
  500: [[0.1, -0.112, 0.118, 0.092, 2.2], [0.03, -0.106, 0.11, 0.09, 2.1], [-0.02, -0.04, 0.04, 0.038, 2]],
};
/** 🧢 Brim per scale [x from the head axis, y of its middle, half-width]; its thickness is the feature of the scale. */
const BRIM_S: Record<Scale, (readonly [number, number, number])[]> = {
  100: [[0.1, YV - 0.084, 0.084], [0.14, YV - 0.091, 0.086], [0.172, YV - 0.098, 0.064], [0.186, YV - 0.101, 0.035]],
  200: [[0.1, YV - 0.085, 0.085], [0.15, YV - 0.093, 0.083], [0.184, YV - 0.1, 0.04]],
  500: [[0.08, YV - 0.08, 0.09], [0.15, YV - 0.095, 0.08], [0.185, YV - 0.1, 0.045]],
};
const JACKET_UP_S: Record<Scale, Slice[]> = {
  100: [[1.03, -0.125, 0.115, 0.171, 2.3], [1.18, -0.122, 0.124, 0.168, 2.3], [1.32, -0.13, 0.134, 0.182, 2.4], [1.42, -0.112, 0.104, 0.2, 2.5], [1.47, -0.08, 0.068, 0.165, 2.4], [1.497, -0.068, 0.056, 0.08, 2]],
  200: [[1.03, -0.125, 0.115, 0.171, 2.3], [1.28, -0.128, 0.134, 0.178, 2.35], [1.41, -0.116, 0.11, 0.2, 2.5], [1.465, -0.084, 0.072, 0.17, 2.4], [1.497, -0.068, 0.056, 0.085, 2]],
  500: [[0.835, -0.14, 0.115, 0.186, 2.35], [0.95, -0.134, 0.112, 0.18, 2.3], [1.1, -0.12, 0.115, 0.165, 2.3], [1.3, -0.13, 0.134, 0.18, 2.4], [1.42, -0.112, 0.104, 0.2, 2.5], [1.48, -0.076, 0.064, 0.14, 2.2], [1.52, -0.07, 0.056, 0.075, 2]],
};
const JACKET_LOW_S: Record<Scale, Slice[]> = {
  100: [[1.065, -0.117, 0.109, 0.163, 2.3], [0.95, -0.134, 0.112, 0.18, 2.3], [0.86, -0.141, 0.115, 0.187, 2.35], [0.835, -0.14, 0.115, 0.186, 2.35]],
  200: [[1.065, -0.117, 0.109, 0.163, 2.3], [0.9, -0.14, 0.114, 0.186, 2.35], [0.835, -0.142, 0.116, 0.188, 2.35]],
  500: [],
};
const COLLAR_S: Record<Scale, Slice[]> = {
  100: [[1.46, -0.082, 0.07, 0.078, 2.1], [1.53, -0.078, 0.06, 0.071, 2]],
  200: [[1.46, -0.082, 0.07, 0.078, 2.1], [1.53, -0.078, 0.062, 0.072, 2]],
  500: [],
};
const SEAT_S: Record<Scale, Slice[]> = {
  100: [[0.95, -0.128, 0.104, 0.172, 2.3], [0.85, -0.13, 0.096, 0.174, 2.3], [0.805, -0.1, 0.06, 0.13, 2.2], [0.79, -0.06, 0.03, 0.06, 2]],
  200: [[0.95, -0.128, 0.104, 0.172, 2.3], [0.83, -0.126, 0.09, 0.168, 2.3], [0.79, -0.07, 0.04, 0.08, 2]],
  500: [[0.9, -0.13, 0.102, 0.172, 2.3], [0.8, -0.1, 0.07, 0.13, 2.2]],
};
const SLEEVE_S: Record<Scale, St[]> = {
  100: [st(-0.03, 0, -0.026, 0.04, 0.05, 2.2), st(0.02, 0.003, 0.0, 0.062, 0.065, 2.1), st(0.16, 0.003, 0.006, 0.055, 0.057), st(0.3255, -0.002, 0.004, 0.049, 0.051)],
  200: [st(-0.02, 0, -0.022, 0.046, 0.054, 2.2), st(0.06, 0.003, 0.004, 0.061, 0.063), st(0.3255, -0.002, 0.004, 0.05, 0.052)],
  500: [st(-0.01, 0, -0.016, 0.056, 0.058), st(0.3255, 0, 0.004, 0.056, 0.058)],
};
const foreS = (scale: Scale, end: number): St[] => scale === 100 ? [st(0.0, 0, 0, 0.05, 0.052), st(0.1, 0.001, 0.003, 0.046, 0.047), st(end, 0, 0.003, 0.037, 0.038)] : scale === 200 ? [st(0.0, 0, 0, 0.05, 0.052), st(end, 0, 0.003, 0.039, 0.04)] : [st(0.0, 0, 0, 0.054, 0.056), st(end, 0, 0.003, 0.046, 0.046)];
const THIGH_S: Record<Scale, St[]> = {
  100: [st(-0.01, -0.012, 0.0, 0.076, 0.094, 2.1), st(0.1, -0.006, 0.006, 0.089, 0.095, 2.1), st(0.25, 0.002, 0.0, 0.076, 0.082), st(0.411, 0.004, -0.004, 0.063, 0.067)],
  200: [st(-0.01, -0.012, 0.0, 0.076, 0.094, 2.1), st(0.12, -0.005, 0.005, 0.088, 0.093, 2.1), st(0.411, 0.004, -0.004, 0.063, 0.067)],
  500: [st(0.06, -0.008, 0.004, 0.084, 0.092, 2), st(0.411, 0.004, -0.004, 0.064, 0.068)],
};
const SHANK_S: Record<Scale, St[]> = {
  100: [st(-0.025, 0.004, -0.004, 0.062, 0.066), st(0.03, 0.003, -0.004, 0.062, 0.066), st(0.14, -0.006, -0.004, 0.058, 0.066), st(0.33, 0, -0.002, 0.053, 0.06), st(0.42, 0.006, -0.002, 0.054, 0.064, 2, -9)],
  200: [st(-0.025, 0.004, -0.004, 0.062, 0.066), st(0.16, -0.005, -0.004, 0.058, 0.065), st(0.42, 0.006, -0.002, 0.054, 0.064, 2, -9)],
  500: [st(-0.03, 0.004, -0.004, 0.063, 0.067), st(0.42, 0.004, -0.002, 0.055, 0.064, 2, -9)],
};
/** 👟 Shoe per scale as cross slices [ahead of the ankle, bottom, top above the ground, half-width]: sole and heel merged into the upper; at 1:500 the forefoot belongs to the same form. */
const SHOE_S: Record<Scale, Cross[]> = {
  100: [[-0.075, 0.0, 0.086, 0.03], [-0.04, 0, 0.098, 0.043], [0.03, 0.0, 0.096, 0.046], [0.1, 0.0, 0.074, 0.051], [0.15, 0, 0.061, 0.0535]],
  200: [[-0.075, 0.0, 0.088, 0.032], [-0.02, 0, 0.1, 0.045], [0.1, 0.0, 0.074, 0.052], [0.15, 0, 0.062, 0.0535]],
  500: [[-0.075, 0.0, 0.09, 0.036], [0.0, 0, 0.1, 0.046], [0.12, 0, 0.07, 0.053], [0.2, 0, 0.05, 0.048], [0.24, 0.0, 0.03, 0.022]],
};
const TOE_S: Record<Scale, Cross[]> = {
  100: [[0.13, 0, 0.063, 0.0535], [0.19, 0, 0.05, 0.048], [0.224, 0.002, 0.037, 0.036], [0.238, 0.006, 0.024, 0.018]],
  200: [[0.13, 0, 0.063, 0.0535], [0.2, 0, 0.047, 0.046], [0.238, 0.004, 0.03, 0.02]],
  500: [],
};

/* ---------------------------------------------------------------- body */

const pairs = (left: Json[]): Json[] => left.flatMap((s) => [mirrored(s, toRight), { ...s, tone: far(s.tone) }]);
const toLeftHand = (s: Json): Json => ({ ...mirrored(s, (x) => (x === "hand-r" ? "hand-l" : x)), tone: TONE.handFar });

/** ✋ A simplified hand of a coarse scale: palm as one loft, the fingers of the fist as one band round the pole or the index finger along the pointer, the thumb. */
function coarseHands(scale: Scale): Json[] {
  const out: Json[] = [];
  const hand = { material: "skin", tone: TONE.hand } as const;
  const th = Math.max(F[scale] / 2, 0.0076);
  if (scale === 500) {
    out.push(toLeftHand(loft({ id: "hand-l", bone: "hand-r", ...hand }, [st(-0.03, 0, 0, 0.0375, 0.04), st(0.05, 0.004, 0.012, 0.0375, 0.046), st(0.11, 0.004, 0.02, 0.0375, 0.044)])));
    out.push(loft({ id: "hand-r", bone: "hand-r", ...hand }, [st(-0.03, 0, 0, 0.0375, 0.04), st(0.06, 0.006, 0.006, 0.0375, 0.046), st(0.15, 0.02, 0.008, 0.0375, 0.0375)]));
    return out;
  }
  const palm = (id: string): Json => loft({ id, bone: "hand-r", ...hand }, dome([st(-0.025, 0, -0.002, Math.max(0.018, th), 0.026, 2.4), st(0.035, 0.002, -0.002, Math.max(0.016, th), 0.04, 2.6), st(0.075, 0.004, -0.003, Math.max(0.014, th), 0.044, 2.6)], 1, 0.008, 2));
  out.push(toLeftHand(palm("hand-l")));
  const C: V3 = [0.08, 0, 0.026];
  const s: V3 = [Math.cos((65 * Math.PI) / 180), Math.sin((65 * Math.PI) / 180), 0];
  const q: V3 = [s[1], -s[0], 0];
  const r = POLE.r + th;
  const pts: V3[] = Array.from({ length: 6 }, (_, i) => { const a = ((-62 + (234 * i) / 5) * Math.PI) / 180; return [C[0] + q[0] * r * Math.cos(a), C[1] + q[1] * r * Math.cos(a), C[2] + r * Math.sin(a)]; });
  out.push(toLeftHand(freeLoft({ id: "hand-l-fingers", bone: "hand-r", ...hand }, pts.map((at, i) => { const t = unit(sub(pts[Math.min(i + 1, 5)]!, pts[Math.max(i - 1, 0)]!)); return { at, rx: 0.037, ry: th, ex: s, ey: unit(cross(t, s)) }; }))));
  out.push(toLeftHand(tube({ id: "hand-l-thumb", bone: "hand-r", ...hand }, [[0.03, 0.03, 0.006], [0.05, 0.047, 0.028], [0.08, 0.055, 0.05]], [Math.max(0.011, th)])));
  out.push(palm("hand-r"));
  out.push(tube({ id: "hand-r-index", bone: "hand-r", ...hand }, [[0.085, 0.03, -0.002], [0.13, 0.031, 0.006], [0.165, 0.03, 0.012]], [Math.max(0.0088, th)]));
  out.push(ellipsoid({ id: "hand-r-fist", bone: "hand-r", ...hand }, [0.03, 0.03, Math.max(0.018, th)], [0.098, -0.008, 0.018]));
  out.push(tube({ id: "hand-r-thumb", bone: "hand-r", ...hand }, [[0.03, 0.03, 0.008], [0.06, 0.045, 0.024], [0.1, 0.036, 0.036]], [Math.max(0.011, th)]));
  return out;
}

export function body(scale: Scale): Json[] {
  const out: Json[] = [];
  const f = F[scale];
  const head = { bone: "head", ...SKIN } as const;
  const hx = (d: number): number => axisX("head", YV - d);
  out.push(loft({ id: "neck", ...head }, slices("head", NECK_S[scale])));
  out.push(loft({ id: "head", ...head }, dome(headSlices(HEAD_S[scale]), 1, 0.003, 2)));
  if (scale !== 500) out.push(loft({ id: "head-nose", ...head }, [H("head", YV - 0.168, hx(0.168) + 0.09, hx(0.168) + 0.126, 0.017), H("head", YV - 0.155, hx(0.155) + 0.09, hx(0.155) + 0.13, 0.015), H("head", YV - 0.11, hx(0.11) + 0.092, hx(0.11) + 0.112, 0.009)]));
  if (scale === 100) for (const side of [1, -1]) out.push(ellipsoid({ id: `head-ear-${side > 0 ? "l" : "r"}`, ...head }, [0.03, 0.016, 0.0085], L("head", [hx(0.12) - 0.01, YV - 0.12, side * 0.0745])));
  if (HAIR_S[scale].length > 0) out.push(loft({ id: "hair", bone: "head", material: "hair", tone: TONE.hair }, dome(headSlices(HAIR_S[scale]), -1, 0.006, 2)));
  const cap = { bone: "head", material: "cap-fabric", tone: TONE.cap } as const;
  out.push(loft({ id: "cap", ...cap }, headSlices(CAP_S[scale])));
  const bt = Math.max(f, 0.0084) / 2;
  out.push(loft({ id: "cap-brim", ...cap }, BRIM_S[scale].map(([x, y, rw]) => X("head", hx(YV - y) + x, y - 0.0042, y - 0.0042 + 2 * bt, rw, 2.6))));

  out.push(loft({ id: "jacket", bone: "torso", ...JACKET }, slices("torso", JACKET_UP_S[scale])));
  if (JACKET_LOW_S[scale].length > 0) out.push(loft({ id: "jacket-skirt", bone: "pelvis", ...JACKET }, slices("pelvis", JACKET_LOW_S[scale])));
  if (scale === 100) out.push(loft({ id: "jacket-hem", bone: "pelvis", ...JACKET, tone: 0.3 }, [HA("pelvis", 0.831, -0.148, 0.122, 0.194, 2.35), HA("pelvis", 0.866, -0.148, 0.122, 0.194, 2.35)]));
  if (COLLAR_S[scale].length > 0) out.push(loft({ id: "jacket-collar", bone: "head", ...JACKET, tone: 0.3 }, slices("head", COLLAR_S[scale])));
  if (scale !== 500) {
    const by = 1.235;
    out.push(box({ id: "badge", bone: "torso", material: "cotton-clothing", tone: TONE.badge, role: "detail" }, [0.086, f, 0.054], L("torso", [surfaceX("torso", JACKET_UP_S[scale], by, 0, 1) + f / 2 + 0.001, by, 0])));
    if (scale === 100) for (const side of [1, -1] as const) out.push(tube({ id: `lanyard-${side > 0 ? "l" : "r"}`, bone: "torso", material: "cotton-clothing", tone: 0.1, role: "detail" }, lanyardPath(side).map((p) => L("torso", p)), [0.0035], { slender: true }));
  }
  out.push(...pairs([loft({ id: "sleeve-l", bone: "upper-arm-l", ...JACKET }, dome(SLEEVE_S[scale], 1, 0.04, scale === 500 ? 2 : 3))]));
  const fe = scale === 500 ? 0.22 : 0.235;
  out.push({ ...loft({ id: "forearm-l", bone: "forearm-l", ...JACKET }, dome(foreS(scale, fe), -1, 0.04, scale === 500 ? 2 : 3)), tone: TONE.jacketFar });
  out.push(mirrored(loft({ id: "forearm-l", bone: "forearm-l", ...JACKET }, dome(foreS(scale, scale === 500 ? 0.19 : 0.2), -1, 0.04, scale === 500 ? 2 : 3)), toRight));
  const wr = scale === 500 ? [st(0.16, 0, 0.002, 0.0375, 0.0375), st(0.27, 0, 0.0, 0.0375, 0.0375)] : [st(0.185, 0, 0.003, Math.max(0.024, f / 2), Math.max(0.031, f / 2)), st(0.262, 0, 0.0, Math.max(0.019, f / 2), Math.max(0.028, f / 2), 2.3)];
  out.push(mirrored(loft({ id: "wrist-l", bone: "forearm-l", ...SKIN }, wr), toRight));
  out.push(...coarseHands(scale));
  const leg: Json[] = [];
  leg.push(loft({ id: "thigh-l", bone: "thigh-l", ...CLOTH }, THIGH_S[scale]));
  leg.push(loft({ id: "shank-l", bone: "shank-l", ...CLOTH }, SHANK_S[scale]));
  leg.push(loft({ id: "shoe-l", bone: "foot-l", ...SHOE }, shoeSlices(SHOE_S[scale], 2.5, false)));
  if (TOE_S[scale].length > 0) leg.push(loft({ id: "shoe-toe-l", bone: "toe-l", ...SHOE }, shoeSlices(TOE_S[scale], 2.5, true)));
  out.push(...pairs(leg));
  out.push(loft({ id: "trousers-seat", bone: "pelvis", ...CLOTH }, slices("pelvis", SEAT_S[scale])));
  out.push(...props(scale));
  return out;
}

function box(p: { id: string; bone: string; material: string; tone: number; role?: string }, size: V3, at: V3): Json {
  return { id: p.id, kind: "box", bone: p.bone, material: p.material, role: p.role ?? "body", tone: p.tone, size: size.map((x) => Math.round(x * 1e5) / 1e5), transform: { translation: at.map((x) => Math.round(x * 1e5) / 1e5) } };
}

/** 🚩 Pointer, pole and flag per scale: true-size slender rods where they would print thinner than the feature, the finial and the hoist sleeve while they print, the flag cloth a feature thick. */
function props(scale: Scale): Json[] {
  const out: Json[] = [];
  const f = F[scale];
  const p = { bone: "pointer", ...METAL } as const;
  if (scale === 100) {
    out.push(capsule({ id: "pointer-grip", ...p, tone: 0.18 }, [-0.05, 0, 0], [0.08, 0, 0], 0.0076, 0.0076));
    out.push(capsule({ id: "pointer-rod", ...p }, [0.07, 0, 0], [0.55, 0, 0], 0.0045, 0.0035, { slender: true }));
  } else out.push(capsule({ id: "pointer-rod", ...p }, [-0.05, 0, 0], [0.55, 0, 0], 0.0055, 0.0035, { slender: true }));
  const q = { bone: "flag-pole", ...METAL } as const;
  out.push(capsule({ id: "pole", ...q }, [POLE.from + POLE.r, 0, 0], [POLE.to, 0, 0], POLE.r, POLE.r, { slender: true }));
  if (scale !== 500) out.push(ellipsoid({ id: "pole-finial", ...q }, [Math.max(POLE.ball, f / 2), Math.max(POLE.ball, f / 2), Math.max(POLE.ball, f / 2)], [POLE.to + POLE.ball * 0.8, 0, 0]));
  if (scale === 100) out.push(revolve({ id: "flag-sleeve", bone: "flag-pole", material: "flag-textile", tone: TONE.flag }, [[POLE.sleeveFrom, 0], [POLE.sleeveFrom, POLE.sleeveR], [POLE.sleeveTo, POLE.sleeveR], [POLE.sleeveTo, 0]], [0, 0, 0], [1, 0, 0]));
  const t = f / 2 + 0.0001;
  const flag: (readonly [number, number, number])[] = scale === 100 ? [[0.004, 0.15, 0], [0.12, 0.153, 0.02], [0.24, 0.158, -0.016], [0.36, 0.164, -0.006], [0.458, 0.17, 0.018]] : scale === 200 ? [[0.004, 0.15, 0], [0.15, 0.154, 0.015], [0.3, 0.161, -0.015], [0.458, 0.17, 0.01]] : [[0.004, 0.15, 0], [0.23, 0.157, 0], [0.458, 0.17, 0]];
  out.push(loft({ id: "flag", bone: "flag", material: "flag-textile", tone: TONE.flag }, flag.map(([u, v, w]) => st(u, v, w, t, POLE.flagH / 2, 6))));
  return out;
}

/* ---------------------------------------------------------------- thermal */

export function thermal(scale: Scale, solids: readonly Json[]): Json[] {
  const f = F[scale];
  const out: Json[] = [];
  for (const s of solids) {
    let region: string | undefined;
    if (s.material === "skin") region = /^hand/.test(s.id) ? "hand-skin" : /^wrist/.test(s.id) ? "arm-skin" : "head-skin";
    if (s.material === "hair" || s.material === "cap-fabric") region = "hair";
    if (s.material === "softshell") region = /^(sleeve|forearm)/.test(s.id) ? "arm-clothing" : "torso-clothing";
    if (s.material === "cotton-clothing") region = /^(badge|lanyard)/.test(s.id) ? "torso-clothing" : "leg-clothing";
    if (s.material === "leather-shoe") region = "shoes";
    if (s.material === "pointer-aluminium") region = /^pointer/.test(s.id) ? "pointer-metal" : "pole-metal";
    if (s.material === "flag-textile") region = "flag-cloth";
    if (region === undefined) throw new Error(`no region for ${s.id}`);
    out.push(thOf(s.id, region));
  }
  const r = Math.max(f / 2, 0.0076);
  const hx = (d: number): number => axisX("head", YV - d);
  const sink = scale === 500 ? 0.02 : 0;
  out.push({ ...ellipsoid({ id: "th-nose-breath", bone: "head", ...SKIN }, [r, r, Math.max(r, 0.0105)], L("head", [hx(0.17) + (scale === 500 ? 0.1 : 0.122) - sink, YV - 0.17, 0])), region: "head-skin", tags: ["emitter", "nose-breath"] });
  out.push({ ...ellipsoid({ id: "th-mouth-breath", bone: "head", ...SKIN }, [r, r, Math.max(r, 0.009)], L("head", [hx(0.19) + (scale === 500 ? 0.1 : 0.106) - sink, YV - 0.19, 0])), region: "head-skin", tags: ["emitter", "mouth-breath"] });
  out.push({ ...loft({ id: "th-plume", bone: "torso", material: "softshell", tone: 0.6 }, [HA("torso", scale === 500 ? 1.385 : 1.42, scale === 500 ? -0.122 : -0.118, -0.02, 0.185, 2.6), HA("torso", 1.47, -0.08 - (scale === 500 ? 0.0 : 0.002), -0.02, 0.15, 2.4)]), region: "torso-clothing", tags: ["emitter", "plume"] });
  return out;
}

/* ---------------------------------------------------------------- structure */

export function structure(scale: Scale, solids: readonly Json[]): Json[] {
  const f = F[scale];
  const out: Json[] = [];
  const m = (solid: Json, member: string, kind: "member" | "joint" | "mass" = "member"): void => { out.push(part(solid, member, kind)); };
  const rr = (r: number): number => Math.max(r, f / 2);
  const joint = (bone: string, at: V3, r: number): void => m(ellipsoid({ id: `joint-${bone}`, bone, material: "bone", tone: 0.4 }, [rr(r), rr(r), rr(r)], at), bone, "joint");
  const mass = (bone: string): void => { const k = kgOf(bone); const r = rr(0.0125 * Math.cbrt(k.kg)); m(ellipsoid({ id: `mass-${bone}`, bone, material: /^(pointer|flag)/.test(bone) ? "pointer-aluminium" : "bone", tone: 0.2, role: "detail" }, [r, r, r], [k.comX, k.comY, 0]), bone, "mass"); };
  const spine: [number, number][] = scale === 500 ? [[-0.035, 1.035], [-0.06, 1.26], [-0.05, 1.46]] : [[-0.042, 1.035], [-0.026, 1.13], [-0.052, 1.24], [-0.069, 1.36], [-0.048, 1.465]];
  m(tube({ id: "spine", bone: "torso", ...BONE }, spine.map(([x, y]) => T(x, y)), [rr(0.021), rr(0.02), rr(0.0175), rr(0.015), rr(0.013)]), "torso");
  m(tube({ id: "cervical-spine", bone: "head", ...BONE }, [HD(-0.046, 0.3), HD(-0.022, 0.214), HD(-0.016, 0.17)], [rr(0.0125)]), "head");
  if (scale === 500) {
    m(ellipsoid({ id: "skull", bone: "head", ...BONE }, [0.072, 0.1, 0.0715], HD(0.014, 0.1)), "head");
    m(capsule({ id: "shoulder-girdle", bone: "torso", ...BONE }, T(-0.03, 1.455, 0.165), T(-0.03, 1.455, -0.165), 0.0375, 0.0375), "torso");
    m(ellipsoid({ id: "rib-cage", bone: "torso", ...BONE }, [0.125, 0.1, 0.14], T(0.004, 1.3)), "torso");
    m(capsule({ id: "pelvic-girdle", bone: "pelvis", ...BONE }, P(-0.02, 0.97, 0.09), P(-0.02, 0.97, -0.09), 0.0375, 0.0375), "pelvis");
  } else {
    m(ellipsoid({ id: "cranium", bone: "head", ...BONE }, [0.069, 0.093, 0.071], HD(0.004, 0.072)), "head");
    m(ellipsoid({ id: "face-bones", bone: "head", ...BONE }, [0.042, scale === 100 ? 0.036 : 0.042, 0.052], HD(0.06, 0.15)), "head");
    const hoops: [number, number, number, number][] = scale === 100 ? [[1.18, 0.014, 0.094, 0.128], [1.24, 0.012, 0.108, 0.144], [1.3, 0.006, 0.112, 0.148], [1.36, -0.004, 0.102, 0.136], [1.415, -0.016, 0.076, 0.1]] : [[1.2, 0.014, 0.1, 0.134], [1.3, 0.006, 0.112, 0.148], [1.395, -0.01, 0.088, 0.116]];
    const rw = rr(0.006);
    hoops.forEach(([y, x, rd, w], i) => {
      const ring: [number, number][] = Array.from({ length: 8 }, (_, k) => [rw * Math.cos((k * Math.PI) / 4), rd + rw * Math.sin((k * Math.PI) / 4)] as [number, number]);
      m(revolve({ id: `rib-cage-${i + 1}`, bone: "torso", ...BONE }, ring, T(x, y), [1, 0, 0], { lateralScale: Math.round((w / rd) * 1000) / 1000 }), "torso");
    });
    m(loft({ id: "sternum", bone: "torso", ...BONE }, [HA("torso", 1.27, 0.124 - 2 * rr(0.0065), 0.124, rr(0.013)), HA("torso", 1.437, 0.068 - 2 * rr(0.007), 0.068, rr(0.028))]), "torso");
    if (scale === 100) {
      for (const side of [1, -1]) {
        const n = side > 0 ? "l" : "r";
        m(capsule({ id: `clavicle-${n}`, bone: "torso", ...BONE }, T(0.06, 1.447, side * 0.022), T(-0.004, 1.47, side * 0.168), 0.0078, 0.0078), "torso");
        m(ellipsoid({ id: `scapula-${n}`, bone: "torso", ...BONE }, [0.082, 0.0078, 0.053], T(-0.088, 1.37, side * 0.11)), "torso");
        m(ellipsoid({ id: `ilium-${n}`, bone: "pelvis", ...BONE }, [0.07, 0.076, 0.0078], P(-0.03, 1.01, side * 0.104)), "pelvis");
        m(capsule({ id: `ischiopubic-${n}`, bone: "pelvis", ...BONE }, P(0.054, 0.92, side * 0.012), P(-0.045, 0.87, side * 0.06), 0.0125, 0.0135), "pelvis");
        m(capsule({ id: `mandible-${n}`, bone: "head", ...BONE }, HD(0.004, 0.137, side * 0.052), HD(0.074, 0.218, side * 0.008), 0.0078, 0.0088), "head");
      }
      m(tube({ id: "sacrum", bone: "pelvis", ...BONE }, [P(-0.044, 1.03), P(-0.072, 0.965), P(-0.076, 0.915)], [0.029, 0.022, 0.0078]), "pelvis");
    } else {
      m(capsule({ id: "shoulder-girdle", bone: "torso", ...BONE }, T(0.02, 1.46, 0.168), T(0.02, 1.46, -0.168), 0.015, 0.015), "torso");
      m(revolve({ id: "pelvic-ring", bone: "pelvis", ...BONE }, Array.from({ length: 8 }, (_, k) => [0.03 * Math.cos((k * Math.PI) / 4), 0.078 + 0.016 * Math.sin((k * Math.PI) / 4)] as [number, number]), P(-0.012, 0.97), [0.97, 0.24, 0], { lateralScale: 1.3 }), "pelvis");
    }
  }
  joint("torso", [0, 0, 0], 0.021);
  joint("head", [0, 0, 0], 0.016);
  mass("torso");
  if (scale !== 500) { mass("pelvis"); mass("head"); }
  const side: Json[] = [];
  const sm = (solid: Json, member: string, kind: "member" | "joint" | "mass" = "member"): void => { side.push(part(solid, member, kind)); };
  const sj = (bone: string, at: V3, r: number): void => sm(ellipsoid({ id: `joint-${bone}`, bone, material: "bone", tone: 0.4 }, [rr(r), rr(r), rr(r)], at), bone, "joint");
  const A = 0.083;
  if (scale === 500) {
    sm(capsule({ id: "femur-l", bone: "thigh-l", ...BONE }, [0.03, 0, 0.02], [0.39, 0, 0], 0.0375, 0.0375), "thigh-l");
    sm(capsule({ id: "tibia-l", bone: "shank-l", ...BONE }, [0.03, 0, 0], [0.41, 0, -0.005], 0.0375, 0.0375), "shank-l");
    sm(capsule({ id: "foot-bones-l", bone: "foot-l", ...BONE }, [-0.03, 0.045 - A, 0], [0.17, 0.0375 - A, 0], 0.0375, 0.0375), "foot-l");
    sm(capsule({ id: "humerus-l", bone: "upper-arm-l", ...BONE }, [0.02, 0, 0], [0.31, 0, 0.004], 0.0375, 0.0375), "upper-arm-l");
    sm(capsule({ id: "forearm-bones-l", bone: "forearm-l", ...BONE }, [0.02, 0, 0.0], [0.235, 0, 0.0], 0.0375, 0.0375), "forearm-l");
    sj("thigh-l", [0, 0, 0], 0.0375);
    sj("shank-l", [0, 0, 0], 0.0375);
  } else {
    sm(tube({ id: "femur-l", bone: "thigh-l", ...BONE }, [[0.0, 0, 0], [0.034, -0.006, 0.06], scale === 100 ? [0.13, 0.008, 0.047] : [0.2, 0.012, 0.03], [0.39, 0.0, 0.002]], [rr(0.023), rr(0.018), rr(0.0146), rr(0.0175)]), "thigh-l");
    sm(ellipsoid({ id: "femur-condyles-l", bone: "thigh-l", ...BONE }, [rr(0.022), 0.032, 0.042], [0.402, -0.006, 0]), "thigh-l");
    sm(tube({ id: "tibia-l", bone: "shank-l", ...BONE }, [[0.02, -0.002, 0], [0.134, 0.012, -0.003], [0.416, 0.002, -0.002]], [rr(0.027), rr(0.014), rr(0.014)]), "shank-l");
    if (scale === 100) {
      sm(ellipsoid({ id: "patella-l", bone: "thigh-l", ...BONE }, [0.022, 0.0098, 0.021], [0.4, 0.043, 0]), "thigh-l");
      sm(capsule({ id: "fibula-l", bone: "shank-l", ...BONE }, [0.042, -0.019, 0.033], [0.436, -0.012, 0.029], 0.0076, 0.0076), "shank-l");
    }
    sm(loft({ id: "foot-bones-l", bone: "foot-l", ...BONE }, [st(-0.065, 0.05 - A, 0, rr(0.02), 0.02), st(0.0, 0.06 - A, 0, 0.026, 0.028), st(0.08, 0.044 - A, 0, 0.036, rr(0.017)), st(0.142, 0.03 - A, 0, 0.042, rr(0.011))]), "foot-l");
    sm(loft({ id: "phalanges-l", bone: "toe-l", ...BONE }, [st(-0.01, rr(0.011) + 0.016, 0, 0.042, rr(0.011)), st(0.075, rr(0.0075) + 0.014, 0, rr(0.026), rr(0.0075))]), "toe-l");
    sm(tube({ id: "humerus-l", bone: "upper-arm-l", ...BONE }, [[0.002, 0, -0.012], [0.103, 0.004, 0.003], [0.31, 0, 0.004]], [rr(0.023), rr(0.0111), rr(0.0128)]), "upper-arm-l");
    sm(ellipsoid({ id: "humerus-condyles-l", bone: "upper-arm-l", ...BONE }, [rr(0.0124), rr(0.0144), 0.032], [0.319, 0, 0.004]), "upper-arm-l");
    if (scale === 100) {
      sm(capsule({ id: "ulna-l", bone: "forearm-l", ...BONE }, [-0.017, -0.014, -0.006], [0.243, -0.004, -0.012], 0.0088, 0.0076), "forearm-l");
      sm(capsule({ id: "radius-l", bone: "forearm-l", ...BONE }, [0.012, 0.012, 0.012], [0.249, 0.012, 0.006], 0.0076, 0.0093), "forearm-l");
    } else sm(capsule({ id: "forearm-bones-l", bone: "forearm-l", ...BONE }, [0.0, 0.0, 0.0], [0.248, 0.004, 0.0], 0.015, 0.015), "forearm-l");
    sj("thigh-l", [0, 0, 0], 0.025);
    sj("shank-l", [0, 0, 0], 0.021);
    sj("upper-arm-l", [0, 0, 0], 0.0165);
    if (scale === 100) { sj("foot-l", [0, 0, 0], 0.017); sj("forearm-l", [0, 0, 0], 0.0145); }
    side.push(part(ellipsoid({ id: "mass-thigh-l", bone: "thigh-l", material: "bone", tone: 0.2, role: "detail" }, [rr(0.028), rr(0.028), rr(0.028)], [kgOf("thigh-l").comX, 0, 0]), "thigh-l", "mass"));
  }
  for (const s of side) out.push(s, mirrored(s, toRight));
  for (const n of ["l", "r"] as const) {
    const b = `hand-${n}`;
    if (scale === 500) m(capsule({ id: `hand-bones-${n}`, bone: b, ...BONE }, [0.0, 0, 0], [0.09, n === "r" ? 0.01 : 0, n === "r" ? 0.01 : -0.012], 0.0375, 0.0375), b);
    else {
      m(loft({ id: `metacarpus-${n}`, bone: b, ...BONE }, [st(0.004, 0, 0, rr(0.0092), 0.023, 2.4), st(0.088, 0.0, 0, rr(0.0076), 0.034, 2.4)]), b);
      if (scale === 100) {
        if (n === "l") {
          const band = solids.find((s) => s.id === "hand-l-fingers")!;
          m({ ...band, id: "finger-rays-l", material: "bone", tone: TONE.bone, sections: band.sections.map((q: Json) => ({ ...q, rx: Math.round(q.rx * 0.82 * 1e5) / 1e5, ry: 0.0076 })) }, b);
        } else m(tube({ id: "finger-rays-r", bone: b, ...BONE }, [[0.088, 0.03, -0.002], [0.13, 0.031, 0.006], [0.162, 0.03, 0.012]], [0.0076]), b);
        m(tube({ id: `thumb-ray-${n}`, bone: b, ...BONE }, n === "r" ? [[0.03, 0.03, 0.008], [0.06, 0.045, 0.024], [0.095, 0.036, 0.034]] : [[0.03, 0.03, -0.006], [0.05, 0.047, -0.028], [0.078, 0.054, -0.048]], [0.0076]), b);
      }
    }
  }
  const has = (id: string): boolean => solids.some((s) => s.id === id);
  for (const id of ["pointer-grip", "pointer-rod"]) if (has(id)) out.push({ id: `member-${id}`, of: id, member: "pointer", part: "member" });
  out.push({ id: "member-pole", of: "pole", member: "flag-pole", part: "member" });
  out.push({ id: "member-flag", of: "flag", member: "flag", part: "member" });
  m(ellipsoid({ id: "joint-flag", bone: "flag", material: "pointer-aluminium", tone: 0.4 }, [rr(0.0165), rr(0.012), rr(0.0165)], [0.0, 0.03, 0]), "flag", "joint");
  if (scale !== 500) { mass("pointer"); mass("flag-pole"); }
  return out;
}

export const DECISIONS: Record<Scale, Json[]> = {
  100: [
    { en: "1:100 body (smallest feature 15 mm): the same figure in fewer sections: head with nose and ears, the cap with its brim a feature thick, the short hair at temples and nape, neck in the stand collar; the jacket as upper part and skirt with the hem band, the badge on its true-size lanyard (zip, pockets, lips, eyes and the lacing are below the feature and left out); sleeves, forearms and the bare right wrist; the left fist as palm, one finger band round the pole and the thumb, the right hand as palm, index finger along the pointer, the curled fingers as one mass and the thumb; chinos with seat, thighs and shanks, shoes with the sole merged into the upper; the pointer as grip and one true-size rod, the pole with finial and hoist sleeve, the flag 15 mm thick.", de: "1:100 Körper (kleinstes Merkmal 15 mm): dieselbe Figur mit weniger Schnitten: Kopf mit Nase und Ohren, die Mütze mit merkmaldickem Schirm, das kurze Haar an Schläfen und Nacken, der Hals im Stehkragen; die Jacke als Oberteil und Schoß mit Saumbund, der Ausweis am Umhängeband in wahrer Größe (Reißverschluss, Taschen, Lippen, Augen und Schnürung liegen unter dem Merkmal und entfallen); Ärmel, Unterarme und das freie rechte Handgelenk; die linke Faust als Handfläche, ein Fingerband um die Stange und der Daumen, die rechte Hand als Handfläche, Zeigefinger am Zeigestab, die eingerollten Finger als eine Masse und der Daumen; Chinohose mit Gesäß, Ober- und Unterschenkeln, Schuhe mit in den Schaft gezogener Sohle; der Zeigestab als Griff und ein Stab in wahrer Größe, die Stange mit Knauf und Hohlsaum, die Fahne 15 mm dick." },
    { en: "1:100 thermal: every region is the outer surface of its body solids by reference (the layers of jacket, sleeves, trousers, shoes and the hair under the cap are merged into their outer shell, B2); breath leaves at nostril and mouth patches of the feature size, the plume from a patch on the jacket shoulders.", de: "1:100 thermisch: Jede Region ist per Verweis die Außenfläche ihrer Körper (die Schichten von Jacke, Ärmeln, Hose, Schuhen und das Haar unter der Mütze sind in ihre äußere Hülle zusammengefasst, B2); der Atem tritt an Nasen- und Mundfeldern in Merkmalsgröße aus, die Warmluftfahne an einem Feld auf den Jackenschultern." },
    { en: "1:100 structure: a skeleton of merged bones: skull, face and jaw, cervical and thoracolumbar spine, five rib hoops, sternum, clavicles, scapulae, ilia, sacrum and ischiopubic bars, femur with neck and condyles, patella, tibia, fibula, one foot and one toe form, humerus, ulna, radius, metacarpus, finger and thumb rays; joints, masses; the pointer by its grip and rod, the pole and the flag cloth by reference.", de: "1:100 Tragwerk: ein Skelett aus zusammengefassten Knochen: Schädel, Gesicht und Kiefer, Hals- und Brust-Lenden-Wirbelsäule, fünf Rippenbögen, Brustbein, Schlüsselbeine, Schulterblätter, Darmbeine, Kreuzbein und Sitz-Scham-Stege, Oberschenkelknochen mit Hals und Kondylen, Kniescheibe, Schien- und Wadenbein, je eine Fuß- und Zehenform, Oberarmknochen, Elle, Speiche, Mittelhand, Finger- und Daumenstrahlen; Gelenke, Massen; der Zeigestab über Griff und Stab, Stange und Fahnentuch per Verweis." },
  ],
  200: [
    { en: "1:200 body (smallest feature 30 mm): head with the nose as one form, hair at the nape, the cap with a 30 mm brim, neck and collar; jacket upper part and skirt, the badge (the lanyard and the hem band are below the feature); sleeves, forearms and the bare right wrist; the left fist with one finger band and the thumb round the pole, the right hand with index finger, curled fingers and thumb; trouser legs and seat, shoe and toe cap; the pointer as one true-size rod, the pole with its finial, the flag 30 mm thick (the hoist sleeve merges into the pole).", de: "1:200 Körper (kleinstes Merkmal 30 mm): Kopf mit der Nase als eine Form, Haar im Nacken, die Mütze mit 30 mm dickem Schirm, Hals und Kragen; Jacken-Oberteil und Schoß, der Ausweis (Umhängeband und Saumbund liegen unter dem Merkmal); Ärmel, Unterarme und das freie rechte Handgelenk; die linke Faust mit einem Fingerband und dem Daumen um die Stange, die rechte Hand mit Zeigefinger, eingerollten Fingern und Daumen; Hosenbeine und Gesäß, Schuh und Zehenkappe; der Zeigestab als ein Stab in wahrer Größe, die Stange mit ihrem Knauf, die Fahne 30 mm dick (der Hohlsaum geht in der Stange auf)." },
    { en: "1:200 thermal: every region by reference to its outer body solids, layers merged (B2); breath and plume patches of the feature size.", de: "1:200 thermisch: Jede Region per Verweis auf ihre äußeren Körper, Schichten zusammengefasst (B2); Atem- und Fahnenfelder in Merkmalsgröße." },
    { en: "1:200 structure: skull and face, cervical bar, spine, three rib hoops, sternum, a shoulder girdle bar, a pelvic ring, one bone per limb segment with knee and elbow condyles, foot and toe forms, metacarpal plates; joints at hips, knees, shoulders, trunk and neck, masses of trunk, pelvis, head, thighs, pointer and pole; pointer, pole and flag by reference.", de: "1:200 Tragwerk: Schädel und Gesicht, Halsstab, Wirbelsäule, drei Rippenbögen, Brustbein, ein Schultergürtelstab, ein Beckenring, ein Knochen je Gliedabschnitt mit Knie- und Ellbogenkondylen, Fuß- und Zehenformen, Mittelhandplatten; Gelenke an Hüften, Knien, Schultern, Rumpf und Hals, Massen von Rumpf, Becken, Kopf, Oberschenkeln, Zeigestab und Stange; Zeigestab, Stange und Fahne per Verweis." },
  ],
  500: [
    { en: "1:500 body (smallest feature 75 mm): head, the cap as crown and brim over it (the hair under it is below the feature), neck; the jacket as one form from hem to shoulders (the collar merges into it), sleeves and forearms, the bare right wrist, the hands as single forms; seat, trouser legs in two sections, one shoe per foot; pointer and pole as true-size rods, the flag 75 mm thick.", de: "1:500 Körper (kleinstes Merkmal 75 mm): Kopf, die Mütze als Kopfteil und Schirm darüber (das Haar darunter liegt unter dem Merkmal), Hals; die Jacke als eine Form vom Saum bis zu den Schultern (der Kragen geht darin auf), Ärmel und Unterarme, das freie rechte Handgelenk, die Hände als je eine Form; Gesäß, Hosenbeine in zwei Schnitten, ein Schuh je Fuß; Zeigestab und Stange als Stäbe in wahrer Größe, die Fahne 75 mm dick." },
    { en: "1:500 thermal: every region by reference to its body solids, layers merged (B2), the hair region on the cap; breath and plume patches of the feature size.", de: "1:500 thermisch: Jede Region per Verweis auf ihre Körper, Schichten zusammengefasst (B2), die Haarregion auf der Mütze; Atem- und Fahnenfelder in Merkmalsgröße." },
    { en: "1:500 structure: a stick skeleton at the 75 mm floor: skull, cervical bar, spine, rib-cage mass, shoulder and pelvic bars, one stick per limb segment and foot, hand sticks; joints at hips, knees, trunk, neck and the flag hoist, the trunk mass; pointer, pole and flag by reference.", de: "1:500 Tragwerk: ein Stabskelett an der Grenze von 75 mm: Schädel, Halsstab, Wirbelsäule, Brustkorbmasse, Schulter- und Beckenstab, ein Stab je Gliedabschnitt und Fuß, Handstäbe; Gelenke an Hüften, Knien, Rumpf, Hals und am Fahnensaum, die Rumpfmasse; Zeigestab, Stange und Fahne per Verweis." },
  ],
};
