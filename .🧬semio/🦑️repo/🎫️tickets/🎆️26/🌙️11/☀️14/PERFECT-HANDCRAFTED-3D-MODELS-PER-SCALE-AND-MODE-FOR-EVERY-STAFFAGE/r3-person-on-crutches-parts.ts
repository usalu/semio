/** 🧩 The measured forms that every scale of `humans/person-on-crutches` shares: tones, the frames in which hand, feet, boot and crutch are stated, and the hand-stated section tables of trunk, limbs, head and crutch (a 24-year-old man of 1.805 m barefoot and 78 kg in a hooded sweatshirt and sports shorts, a sneaker on the right foot, a tall walking boot on the left, two forearm crutches).
 *
 * Figure coordinates (`H`, `X`, `L`): x ahead, y up, z to the left, the default pose `tripod-stand`. Local stations (`st`): u along the bone, v its sagittal normal, w lateral.
 * @see ./r3-person-on-crutches-kit.ts
 * @see ./r3-humans-person-on-crutches.md */
import { FRAMES, H, L, st, type St, type V3 } from "./r3-person-on-crutches-kit.ts";
import { CRUTCH_AT, CRUTCH_L, CRUTCH_REST, GRIP_AT, SPREAD, TIP_Z } from "./r3-person-on-crutches-roots.ts";

export const TONE = { hoodie: 0.42, hoodieFar: 0.34, shorts: 0.3, shortsFar: 0.24, leg: 0.76, legFar: 0.66, boot: 0.18, sneaker: 0.85, sock: 0.92, hand: 0.82, handFar: 0.7, face: 0.8, hair: 0.15, frame: 0.62, frameFar: 0.5, tip: 0.1, grip: 0.12, cuff: 0.15, sole: 0.55 } as const;
/** 🌗 The tone of the far (left) side for a near-side tone. */
export const nearOf = (t: number): number => (t === TONE.hoodieFar ? TONE.hoodie : t === TONE.shortsFar ? TONE.shorts : t === TONE.legFar ? TONE.leg : t === TONE.handFar ? TONE.hand : t === TONE.frameFar ? TONE.frame : t);

/** 🦶 The ankle joint centre 95 mm above the sole plane in the sneaker (0.039 H = 70 mm barefoot for H 1.805 m, Winter 2009, plus the 25 mm heel stack), the foot bone running down to the ball contact 149 mm ahead, and the shank from the knee to that ankle. */
export const ANKLE_H = 0.095;
export const FOOT_PITCH = 32.4766;
export const SHANK_L = 0.4357;
const COS = Math.cos((FOOT_PITCH * Math.PI) / 180);
const SIN = Math.sin((FOOT_PITCH * Math.PI) / 180);
const BALL = 0.14926;
/** 🦶 A cross slice of a foot standing flat (x ahead of the ankle, heights above the sole plane of the rig) as a station of the foot bone or of the toe bone. */
export function footSt(bone: "foot" | "toe", x: number, bottom: number, top: number, rw: number, e = 2.4, z = 0): St {
  const y = (bottom + top) / 2 - ANKLE_H;
  const at: V3 = bone === "foot" ? [x * COS - y * SIN, x * SIN + y * COS, z] : [x - BALL, y + ANKLE_H, z];
  return { at, rw, rv: (top - bottom) / 2, e, n: bone === "foot" ? FOOT_PITCH : 0 };
}
/** 🦶 A point of the flat-standing foot in the frame of the foot bone or of the toe bone. */
export function footAt(bone: "foot" | "toe", x: number, y: number, z = 0): V3 {
  const h = y - ANKLE_H;
  return bone === "foot" ? [x * COS - h * SIN, x * SIN + h * COS, z] : [x - BALL, y, z];
}
/** 🥾 A horizontal slice of the boot leg with the shank upright (height above the sole plane, x ahead of the ankle) as a station of the shank bone. */
export const bootSt = (y: number, back: number, front: number, rw: number, e = 2.5): St => ({ at: [SHANK_L - (y - ANKLE_H), (back + front) / 2, 0], rw, rv: (front - back) / 2, e, n: 0 });

const HD: [number, number] = [GRIP_AT[0] - CRUTCH_AT[0], GRIP_AT[1] - CRUTCH_AT[1]];
const HL = Math.hypot(HD[0], HD[1]);
/** ✊ A point given at the gripped handle (a ahead along the handle from the middle of the grip, b upwards across it, c outwards) in the frame of the left hand. */
export const grip = (a: number, b: number, c: number): V3 => [GRIP_AT[0] + (a * HD[0] - b * HD[1]) / HL, GRIP_AT[1] + (a * HD[1] + b * HD[0]) / HL, c];
/** 🌀 The point at the angle `phi` (degrees from straight up towards the outside) and the distance `r` around the handle, `a` ahead of the middle of the grip. */
export const around = (a: number, phi: number, r: number): V3 => grip(a, r * Math.cos((phi * Math.PI) / 180), r * Math.sin((phi * Math.PI) / 180));
/** 🩼 A point of the hand frame in the frame of the crutch of that side (the crutch is rigid in the hand). */
export function handToCrutch(p: V3): V3 {
  const a = (CRUTCH_REST * Math.PI) / 180;
  const d: [number, number] = [p[0] - CRUTCH_AT[0], p[1] - CRUTCH_AT[1]];
  return [d[0] * Math.cos(a) + d[1] * Math.sin(a), -d[0] * Math.sin(a) + d[1] * Math.cos(a), p[2]];
}
/** 🩼 The direction of the handle in the crutch frame and its length from the junction to the front end. */
export const HANDLE_DIR: V3 = ((): V3 => { const a = handToCrutch(grip(0, 0, 0)); const l = Math.hypot(a[0], a[1]); return [a[0] / l, a[1] / l, 0]; })();
export const HANDLE_L = 0.118;
/** 💪 A point of the left forearm frame carried into the frame of the left crutch through the default pose (forearm, hand and crutch keep their angles in every pose). */
export function forearmToCrutch(p: V3): V3 {
  const f = FRAMES["forearm-l"]!;
  return L("crutch-l", [0, 1, 2].map((k) => f.o[k]! + p[0] * f.u[k]! + p[1] * f.v[k]! + p[2] * f.w[k]!) as V3);
}
/** 🩼 A point of the left crutch frame in the left forearm frame. */
export function crutchToForearm(p: V3): V3 {
  const f = FRAMES["crutch-l"]!;
  return L("forearm-l", [0, 1, 2].map((k) => f.o[k]! + p[0] * f.u[k]! + p[1] * f.v[k]! + p[2] * f.w[k]!) as V3);
}

/** 🪽 Stations leaning outwards by `k` metres per metre along the bone: the spread of the arm plane written into the solids. */
export const lean = (s: readonly St[], k: number): St[] => s.map((q) => ({ ...q, at: [q.at[0], q.at[1], q.at[2] + k * Math.max(0, q.at[0])] }));
export const leanAt = (p: V3, k: number): V3 => [p[0], p[1], p[2] + k * Math.max(0, p[0])];
/** 🪽 The outward lean per metre of the upper arm, of the forearm and of the crutch shaft in the default stance, and the unit axis of the shaft in the crutch frame. */
export const ARM_LEAN = SPREAD;
export const FORE_LEAN = 0.0772;
export const SHAFT: V3 = [Math.cos(Math.atan(TIP_Z / CRUTCH_L)), 0, Math.sin(Math.atan(TIP_Z / CRUTCH_L))];
/** 🩼 The point at the distance `u` down the leaning shaft, `v` across it. */
export const shaft = (u: number, v = 0): V3 => [u * SHAFT[0], v, u * SHAFT[2]];
/** 📏 The middle line of the trunk at the height y: the line through hip joints and shoulder joints, leaning 7 degrees. */
export const mid = (y: number): number => (y <= 1.0655 ? 0 : 0.1228 * (y - 1.0655));
type Row = readonly [number, number, number, number, number];
/** 🧥 The sweatshirt on the trunk as horizontal slices [y, back, front, half-width, exponent], back and front measured from the middle line: ribbed hem band on the hips, cloth blousing over it, hanging straight from chest and shoulder blades, the shoulder slope up to the neck. */
export const HOODIE: readonly Row[] = [
  [0.918, -0.134, 0.116, 0.188, 2.5],
  [0.94, -0.138, 0.122, 0.192, 2.5],
  [0.972, -0.139, 0.126, 0.193, 2.5],
  [1.005, -0.139, 0.134, 0.194, 2.4],
  [1.07, -0.124, 0.134, 0.188, 2.4],
  [1.16, -0.118, 0.132, 0.184, 2.4],
  [1.26, -0.126, 0.134, 0.186, 2.4],
  [1.35, -0.134, 0.137, 0.189, 2.4],
  [1.43, -0.132, 0.128, 0.191, 2.4],
  [1.485, -0.122, 0.11, 0.19, 2.35],
  [1.52, -0.108, 0.088, 0.178, 2.3],
  [1.545, -0.09, 0.068, 0.146, 2.2],
  [1.565, -0.074, 0.056, 0.102, 2.1],
  [1.58, -0.062, 0.05, 0.078, 2],
];
export const rowsAt = (rows: readonly Row[], bone: string): St[] => rows.map(([y, b, f, rw, e]) => H(bone, y, mid(y) + b, mid(y) + f, rw, e));
/** 📍 The front (+1) or back (-1) surface of a slice table at the height y and the lateral position z. */
export function surfaceX(rows: readonly Row[], y: number, z: number, side: 1 | -1): number {
  let i = 0;
  while (i + 2 < rows.length && rows[i + 1]![0] < y) i++;
  const [a, b] = [rows[i]!, rows[i + 1]!];
  const t = Math.min(1, Math.max(0, (y - a[0]) / (b[0] - a[0])));
  const m = (k: number): number => a[k]! + (b[k]! - a[k]!) * t;
  const [back, front, rw, e] = [mid(y) + m(1), mid(y) + m(2), m(3), m(4)];
  const k = Math.pow(Math.max(0, 1 - Math.pow(Math.min(1, Math.abs(z) / rw), e)), 1 / e);
  return (back + front) / 2 + (side * k * (front - back)) / 2;
}
/** 🩳 The shorts on the pelvis: waistband under the sweatshirt, seat over the buttocks, the crotch between the leg tubes. */
export const SEAT: readonly Row[] = [
  [0.812, -0.07, 0.05, 0.15, 2.3],
  [0.84, -0.094, 0.07, 0.164, 2.4],
  [0.885, -0.106, 0.086, 0.169, 2.4],
  [0.935, -0.106, 0.09, 0.17, 2.4],
  [0.99, -0.114, 0.104, 0.171, 2.4],
  [1.045, -0.104, 0.11, 0.168, 2.4],
];

/** 🦵 The bare left thigh, bone-local (v ahead: quadriceps, -v: hamstrings, w outwards): from inside the shorts over the vastus bulge to the knee. */
export const THIGH: St[] = [st(0.21, 0.006, 0, 0.076, 0.086, 2.1), st(0.27, 0.007, 0, 0.07, 0.079, 2.1), st(0.34, 0.006, 0.001, 0.062, 0.07, 2.1), st(0.395, 0.006, 0.001, 0.058, 0.064, 2.15), st(0.43, 0.006, 0, 0.0565, 0.062, 2.2), st(0.4459, 0.005, 0, 0.056, 0.0615, 2.2)];
/** 🩳 The left leg of the shorts on the thigh: a loose tube from the hip to a hem at mid-thigh. */
export const SHORTS_LEG: St[] = [st(-0.02, -0.006, -0.01, 0.09, 0.102, 2.3), st(0.05, -0.008, -0.008, 0.091, 0.108, 2.3), st(0.12, -0.004, -0.004, 0.094, 0.11, 2.3), st(0.2, 0.002, 0.001, 0.096, 0.107, 2.3), st(0.245, 0.003, 0.003, 0.098, 0.108, 2.3)];
/** 🦵 The bare left shank, bone-local (v ahead: shin, -v: calf): knee, tibial head, the calf belly behind and inwards, the taper to the ankle. */
export const SHANK: St[] = [st(-0.02, 0.005, 0, 0.0555, 0.061, 2.2), st(0.03, 0.003, 0, 0.0535, 0.0585, 2.15), st(0.085, -0.005, -0.002, 0.054, 0.059, 2.05), st(0.15, -0.011, -0.004, 0.057, 0.064, 2), st(0.22, -0.01, -0.003, 0.05, 0.057, 2), st(0.282, -0.006, -0.001, 0.04, 0.047, 2), st(0.34, -0.004, 0, 0.034, 0.041, 2), st(0.39, -0.003, 0, 0.0325, 0.039, 2)];

/** 💪 The left sleeve on the upper arm, bone-local (v ahead, w outwards): the sleeve head sits inwards over the shoulder, the deltoid below it, biceps ahead, the elbow. */
export const SLEEVE: St[] = lean([st(-0.03, 0, -0.03, 0.038, 0.052), st(0, 0, -0.014, 0.049, 0.063), st(0.045, 0.002, -0.003, 0.053, 0.068), st(0.11, 0.004, 0, 0.053, 0.066), st(0.19, 0.004, 0, 0.052, 0.062), st(0.27, 0, 0, 0.049, 0.056), st(0.3385, -0.003, 0, 0.0475, 0.053)], ARM_LEAN);
/** 💪 The left sleeve on the forearm down to the ribbed cuff. */
export const FORE: St[] = lean([st(-0.012, -0.004, 0, 0.047, 0.0525), st(0.05, 0, 0, 0.0445, 0.049), st(0.12, 0, 0, 0.0445, 0.0465), st(0.185, 0, 0, 0.039, 0.04), st(0.222, 0, 0, 0.037, 0.0375)], FORE_LEAN);
export const CUFF: St[] = lean([st(0.218, 0, 0, 0.0335, 0.031), st(0.24, 0, 0, 0.0325, 0.029), st(0.26, 0, 0, 0.0325, 0.0285)], FORE_LEAN);
export const WRIST: St[] = lean([st(0.236, 0, 0, 0.03, 0.0235), st(0.262, 0, 0, 0.0305, 0.023), st(0.285, 0.001, 0, 0.032, 0.0235)], FORE_LEAN);

/** 🗣️ The head, bone-local (u up from the atlanto-occipital joint, -v the face, v the back of the head): chin, jaw, mouth, cheeks, eyes, forehead, crown. */
export const HEAD: St[] = [st(-0.08, -0.064, 0, 0.021, 0.019), st(-0.066, -0.053, 0, 0.039, 0.036), st(-0.045, -0.036, 0, 0.0535, 0.056), st(-0.02, -0.02, 0, 0.0635, 0.074), st(0.005, -0.008, 0, 0.0695, 0.087), st(0.035, -0.004, 0, 0.0735, 0.096), st(0.065, -0.002, 0, 0.0765, 0.099), st(0.095, 0, 0, 0.0765, 0.097), st(0.125, 0.002, 0, 0.068, 0.085), st(0.145, 0.004, 0, 0.05, 0.06), st(0.156, 0.004, 0, 0.02, 0.024)];
/** 💇 The short hair: from the nape hairline round the back and sides, over the temples to the forehead hairline and the crown. */
export const HAIR: St[] = [st(-0.012, 0.05, 0, 0.05, 0.036), st(0.01, 0.045, 0, 0.068, 0.051), st(0.04, 0.036, 0, 0.0795, 0.066), st(0.07, 0.02, 0, 0.0825, 0.084), st(0.096, 0.012, 0, 0.0825, 0.092), st(0.12, 0.005, 0, 0.078, 0.095), st(0.142, 0.004, 0, 0.063, 0.075), st(0.156, 0.004, 0, 0.04, 0.047), st(0.162, 0.004, 0, 0.02, 0.024)];
export const NECK: St[] = [st(-0.035, 0.008, 0, 0.07, 0.07), st(0.015, 0.005, 0, 0.061, 0.063), st(0.06, 0.001, 0, 0.0575, 0.06), st(0.118, -0.004, 0, 0.056, 0.058)];
