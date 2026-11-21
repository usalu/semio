/** 👴 The 1:50 model of `humans/elderly-with-cane`, stated section by section: a 74-year-old man of 1.65 m and 74 kg with a rounded upper back, flat cap, glasses, tweed jacket over waistcoat and shirt, wool trousers, leather walking shoes and a crook cane cut to his wrist crease.
 *
 * Figure coordinates (`H`, `X`, `L`): x ahead, y up, z to the left, the default pose `stooped-standing`. Local stations (`st`): u along the bone, v its sagittal normal, w lateral.
 * @see ./r3-elderly-with-cane-kit.ts
 * @see ./r3-humans-elderly-with-cane.md */
import type { Json } from "./volumes-lib.ts";
import { ASSET, D, H, L, X, aboutU, aboutV, aboutW, box, boneDeg, capsule, compose, dense, dome, ellipsoid, extrude, flip, inset, loft, mirrored, part, revolve, st, thOf, toRight, tube, type Quat, type St, type V3 } from "./r3-elderly-with-cane-kit.ts";

const TONE = { jacket: 0.5, jacketFar: 0.43, trousers: 0.4, trousersFar: 0.33, shoe: 0.3, shoeFar: 0.23, skin: 0.8, hand: 0.78, handFar: 0.71, hair: 0.92, cap: 0.32, wood: 0.45, shirt: 0.86, waistcoat: 0.34 } as const;
const far = (t: number): number => (t === TONE.jacket ? TONE.jacketFar : t === TONE.trousers ? TONE.trousersFar : t === TONE.shoe ? TONE.shoeFar : t === TONE.hand ? TONE.handFar : t);

/* ---------------------------------------------------------------- stations shared by body, thermal and the coarser scales */

/** 🧥 The jacket on the trunk as horizontal slices: [y, back, front, half-width, exponent]. The back rounds over the thoracic kyphosis (apex at T7, 1.22 m) and the cloth hangs taut from that hump to the buttocks, bridging the flattened lumbar curve; the shoulder line falls about 22 degrees from the collar to the shoulder point; a slight waist under the chest; the front hangs from chest and belly. */
export const JACKET: readonly [number, number, number, number, number][] = [
  [0.779, -0.128, 0.146, 0.183, 2.4],
  [0.834, -0.137, 0.156, 0.187, 2.4],
  [0.914, -0.14, 0.166, 0.186, 2.35],
  [1.044, -0.125, 0.181, 0.173, 2.3],
  [1.154, -0.1125, 0.19, 0.175, 2.3],
  [1.22, -0.105, 0.192, 0.181, 2.3],
  [1.289, -0.098, 0.19, 0.19, 2.3],
  [1.33, -0.082, 0.186, 0.2, 2.3],
  [1.356, -0.064, 0.18, 0.203, 2.25],
  [1.377, -0.044, 0.171, 0.178, 2.2],
  [1.398, -0.018, 0.158, 0.13, 2.1],
  [1.414, 0.006, 0.142, 0.086, 2],
];
export const jacketStations = (): St[] => JACKET.map(([y, b, f, rw, e]) => H("torso", y, b, f, rw, e));
/** 📏 The jacket surface at the height `y` and the lateral position `z`: x of the front (+1) or of the back (-1). */
export function jacketX(y: number, z: number, side: 1 | -1): number {
  let i = 0;
  while (i + 2 < JACKET.length && JACKET[i + 1]![0] < y) i++;
  const a = JACKET[i]!;
  const b = JACKET[i + 1]!;
  const t = Math.min(1, Math.max(0, (y - a[0]) / (b[0] - a[0])));
  const m = (k: number): number => a[k]! + (b[k]! - a[k]!) * t;
  const [back, front, rw, e] = [m(1), m(2), m(3), m(4)];
  const k = Math.pow(Math.max(0, 1 - Math.pow(Math.min(1, Math.abs(z) / rw), e)), 1 / e);
  return (back + front) / 2 + (side * k * (front - back)) / 2;
}
/** 🧭 The outward direction of the jacket front at (y, z) as an angle about the vertical, degrees from straight ahead towards the left. */
export const jacketYaw = (y: number, z: number): number => (Math.atan2(jacketX(y, z - 0.004, 1) - jacketX(y, z + 0.004, 1), 0.008) * 180) / Math.PI;

export const SEAT: readonly [number, number, number, number, number][] = [
  [0.806, -0.1, 0.07, 0.15, 2.3],
  [0.834, -0.126, 0.098, 0.176, 2.4],
  [0.914, -0.135, 0.115, 0.178, 2.4],
  [0.984, -0.112, 0.146, 0.168, 2.3],
  [1.054, -0.098, 0.168, 0.158, 2.3],
  [1.076, -0.095, 0.166, 0.155, 2.3],
];
export const seatStations = (): St[] => SEAT.map(([y, b, f, rw, e]) => H("pelvis", y, b, f, rw, e));

/** 👖 The left trouser leg on the thigh, bone-local: the tube hangs from the seat, quadriceps in front. */
export const THIGH_L: St[] = [st(0.0, -0.01, 0, 0.085, 0.1, 2.1), st(0.08, -0.008, 0, 0.086, 0.097, 2.1), st(0.16, -0.002, 0, 0.08, 0.089, 2), st(0.26, 0.0, 0, 0.073, 0.08, 1.95), st(0.34, 0.002, 0, 0.068, 0.074, 1.9), st(0.4046, 0.004, 0, 0.066, 0.072, 1.9)];
/** 🦵 The left trouser leg on the shank: knee (the first station repeats the thigh end, so the cloth runs on without a step), the calf pushing the cloth back, the break over the instep and the hem cut lower at the heel. */
export const SHANK_L: St[] = [st(-0.02, 0.004, 0, 0.066, 0.072, 1.9), st(0.04, 0.002, 0, 0.0645, 0.0705, 1.9), st(0.14, -0.006, 0, 0.064, 0.071, 1.9), st(0.24, -0.006, 0, 0.0625, 0.069, 1.9), st(0.31, -0.002, 0, 0.0625, 0.07, 1.9), st(0.35, 0.004, 0, 0.0635, 0.0745, 1.95, -4), st(0.376, 0.004, 0, 0.062, 0.072, 2, -10)];

/** 🧥 The left sleeve on the upper arm: the sleeve head sits inwards of the bone (the bone hangs from the acromion, the deltoid lies under it), biceps in front, the elbow at the end. */
export const SLEEVE_L: St[] = [st(-0.022, 0.0, -0.036, 0.034, 0.05), st(0.0, 0.0, -0.022, 0.05, 0.061), st(0.04, 0.002, -0.01, 0.055, 0.064), st(0.1, 0.004, -0.005, 0.052, 0.061), st(0.19, 0.002, 0, 0.048, 0.055), st(0.27, -0.002, 0, 0.0455, 0.0515), st(0.3069, -0.004, 0, 0.0455, 0.051)];
/** 💪 The left sleeve on the forearm down to the cuff. */
export const CUFF_L: St[] = [st(0.0, -0.004, 0, 0.0452, 0.0505), st(0.05, 0.0, 0, 0.042, 0.0465), st(0.13, 0.0, 0, 0.0405, 0.043), st(0.19, 0, 0, 0.038, 0.0395), st(0.213, 0, 0, 0.0375, 0.039)];

/** 🥾 The left shoe upper as cross slices: [x, bottom, top, half-width, exponent]; rear part on the foot bone, toe box on the toe bone, overlapping at the ball. */
export const SHOE_REAR: readonly [number, number, number, number, number][] = [
  [-0.066, 0.02, 0.062, 0.017, 2],
  [-0.05, 0.007, 0.083, 0.031, 2.4],
  [-0.02, 0.006, 0.09, 0.038, 2.4],
  [0.02, 0.006, 0.094, 0.043, 2.4],
  [0.06, 0.006, 0.084, 0.047, 2.4],
  [0.1, 0.006, 0.066, 0.05, 2.5],
  [0.14, 0.006, 0.054, 0.052, 2.6],
  [0.166, 0.006, 0.048, 0.051, 2.6],
];
export const SHOE_TOE: readonly [number, number, number, number, number][] = [
  [0.146, 0.006, 0.053, 0.0515, 2.6],
  [0.175, 0.006, 0.046, 0.049, 2.6],
  [0.2, 0.006, 0.038, 0.04, 2.4],
  [0.214, 0.007, 0.03, 0.026, 2.2],
  [0.2195, 0.009, 0.022, 0.012, 2],
];

/** 🗣️ The head below the cap line, bone-local (v is the back of the head): chin, jaw, cheeks, zygoma, brow; the last sections tilt with the rim of the cap, which sits lower on the occiput. */
export const HEAD: St[] = [st(-0.045, -0.057, 0, 0.02, 0.018), st(-0.032, -0.045, 0, 0.039, 0.037), st(-0.012, -0.03, 0, 0.053, 0.058), st(0.01, -0.013, 0, 0.062, 0.076), st(0.035, -0.004, 0, 0.068, 0.087), st(0.06, 0.004, 0, 0.0715, 0.092), st(0.08, 0.007, 0, 0.0745, 0.0955, 2, 3), st(0.092, 0.009, 0, 0.0755, 0.0955, 2, 7.4)];
/** 🧠 The skull above the cap line, only as the inside of the cap shells. */
export const SCALP: St[] = [st(0.092, 0.009, 0, 0.0755, 0.0955, 2, 7.4), st(0.115, 0.01, 0, 0.0755, 0.094), st(0.138, 0.011, 0, 0.069, 0.082), st(0.156, 0.012, 0, 0.05, 0.057), st(0.1674, 0.012, 0, 0.014, 0.016)];
/** 🧢 The crown of the flat cap as slices from the front edge to the back: [centre u, v, half-width, half-height]. */
export const CROWN: readonly [number, number, number, number][] = [
  [0.117, -0.123, 0.044, 0.0055],
  [0.121, -0.105, 0.067, 0.014],
  [0.126, -0.075, 0.087, 0.025],
  [0.131, -0.04, 0.096, 0.036],
  [0.1335, 0.0, 0.099, 0.047],
  [0.13, 0.04, 0.096, 0.052],
  [0.12, 0.08, 0.083, 0.047],
  [0.108, 0.105, 0.059, 0.032],
  [0.1, 0.118, 0.03, 0.014],
];
export const crownStations = (): St[] => CROWN.map(([u, v, rw, rh]) => st(u, v, 0, rw, rh, 2.3, 90));
export const NECK: St[] = [st(-0.035, 0.004, 0, 0.066, 0.068), st(0.02, 0.0, 0, 0.059, 0.061), st(0.06, -0.005, 0, 0.056, 0.057), st(0.112, -0.01, 0, 0.055, 0.054)];

const CROOK_R = 0.045;
/** 🩼 The cane from the shaft top under the crook to the ground, cut so that the crook stands at the wrist crease of the stooped stance. */
export const CANE_L = 0.748;
/** 🩼 The centre line of the crook in the cane frame (u down the shaft, v ahead): from the shaft top over the apex to the nose of the hook. */
export const crookPath = (n = 12): V3[] => [...Array.from({ length: n + 1 }, (_, i) => [-CROOK_R * Math.sin((Math.PI * i) / n), -CROOK_R + CROOK_R * Math.cos((Math.PI * i) / n), 0] as V3), [0.016, -2 * CROOK_R - 0.001, 0], [0.03, -2 * CROOK_R - 0.003, 0]];

/* ---------------------------------------------------------------- the right hand on the crook */

const HAND_SLOPE = -50;
/** ✋ A point given beside the gripped handle (x ahead, y up, z left, the cane upright, the origin at the wrist) in the frame of the right hand. */
const hr = (p: V3): V3 => [p[0] * Math.cos((HAND_SLOPE * Math.PI) / 180) + p[1] * Math.sin((HAND_SLOPE * Math.PI) / 180), -p[0] * Math.sin((HAND_SLOPE * Math.PI) / 180) + p[1] * Math.cos((HAND_SLOPE * Math.PI) / 180), p[2]];
const GRIP: V3 = [0.03, -0.045, -0.04];
/** 🪝 The centre of the handle tube at the distance `s` ahead of the apex, and the point at the angle `phi` (degrees above the outward horizontal) and the distance `r` around it. */
const around = (s: number, phi: number, r: number): V3 => {
  const drop = CROOK_R - Math.sqrt(CROOK_R * CROOK_R - s * s);
  const a = (phi * Math.PI) / 180;
  return hr([GRIP[0] + s, GRIP[1] - drop + r * Math.sin(a), GRIP[2] - r * Math.cos(a)]);
};
function quatOf(ex: V3, ey: V3, ez: V3): Quat {
  const [m00, m10, m20, m01, m11, m21, m02, m12, m22] = [ex[0], ex[1], ex[2], ey[0], ey[1], ey[2], ez[0], ez[1], ez[2]];
  const tr = m00 + m11 + m22;
  let q: Quat;
  if (tr > 0) {
    const s = Math.sqrt(tr + 1) * 2;
    q = [(m21 - m12) / s, (m02 - m20) / s, (m10 - m01) / s, s / 4];
  } else if (m00 > m11 && m00 > m22) {
    const s = Math.sqrt(1 + m00 - m11 - m22) * 2;
    q = [s / 4, (m01 + m10) / s, (m02 + m20) / s, (m21 - m12) / s];
  } else if (m11 > m22) {
    const s = Math.sqrt(1 + m11 - m00 - m22) * 2;
    q = [(m01 + m10) / s, s / 4, (m12 + m21) / s, (m02 - m20) / s];
  } else {
    const s = Math.sqrt(1 + m22 - m00 - m11) * 2;
    q = [(m02 + m20) / s, (m12 + m21) / s, s / 4, (m10 - m01) / s];
  }
  return q.map((x) => Math.round(x * 1e5) / 1e5) as Quat;
}
const unit = (a: V3): V3 => { const l = Math.hypot(...a); return [a[0] / l, a[1] / l, a[2] / l]; };
const cross = (a: V3, b: V3): V3 => [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
const sub = (a: V3, b: V3): V3 => [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
const mid = (a: V3, b: V3, t = 0.5): V3 => [a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t, a[2] + (b[2] - a[2]) * t];

function rightHand(tone: number): Json[] {
  const c = (id: string, role = "body"): { id: string; bone: string; material: string; tone: number; role: string } => ({ id, bone: "hand-r", material: "skin", tone, role });
  const out: Json[] = [];
  const knuckle = around(0, 50, 0.03);
  const e1 = unit(knuckle);
  const e2 = unit(sub(hr([1, 0, 0]), e1.map((x) => x * (hr([1, 0, 0])[0] * e1[0] + hr([1, 0, 0])[1] * e1[1] + hr([1, 0, 0])[2] * e1[2])) as V3));
  const e3 = cross(e1, e2);
  out.push(ellipsoid(c("hand-r"), [0.047, 0.044, 0.0145], mid([0, 0, 0], knuckle, 0.5), quatOf(e1, e2, e3)));
  const fingers: [string, number, number][] = [["1", 0.033, 1], ["2", 0.011, 1.04], ["3", -0.011, 1], ["4", -0.031, 0.86]];
  for (const [name, s, k] of fingers) {
    const p = [around(s, 50, 0.03), around(s, -35, 0.0245 * (0.9 + 0.1 * k)), around(s, -110, 0.022), around(s, -165, 0.021)];
    out.push(capsule(c(`hand-r-finger-${name}`), p[0]!, p[1]!, 0.0098 * k, 0.0092 * k));
    out.push(capsule(c(`hand-r-finger-${name}-mid`), p[1]!, p[2]!, 0.009 * k, 0.0084 * k));
    out.push(capsule(c(`hand-r-finger-${name}-tip`), p[2]!, p[3]!, 0.0084 * k, 0.0076 * k));
  }
  const t0 = hr([0.006, -0.012, 0.004]);
  const t1 = hr([0.034, -0.035, -0.013]);
  const t2 = hr([0.06, -0.053, -0.019]);
  out.push(capsule(c("hand-r-thumb"), t0, t1, 0.0128, 0.011));
  out.push(capsule(c("hand-r-thumb-tip"), t1, t2, 0.0108, 0.009));
  return out;
}

function leftHand(tone: number): Json[] {
  const c = (id: string): { id: string; bone: string; material: string; tone: number } => ({ id, bone: "hand-l", material: "skin", tone });
  const out: Json[] = [];
  const palm: St[] = [st(-0.01, 0, 0, 0.0195, 0.0275), st(0.02, 0.002, 0, 0.017, 0.034), st(0.05, 0.002, -0.001, 0.0145, 0.041), st(0.082, 0.0, -0.003, 0.0125, 0.0415), st(0.094, 0, -0.004, 0.011, 0.039)];
  out.push(loft(c("hand-l"), dome(dense(palm, 3), 1, 0.008, 2)));
  out.push(ellipsoid(c("hand-l-thenar"), [0.03, 0.016, 0.0115], [0.04, 0.028, -0.011]));
  const fingers: [string, number, number, number][] = [["1", 0.09, 0.03, 1], ["2", 0.094, 0.0105, 1.08], ["3", 0.09, -0.0095, 1], ["4", 0.082, -0.029, 0.8]];
  for (const [name, u, v, k] of fingers) {
    const dir = (deg: number, len: number, from: V3): V3 => [from[0] + len * Math.cos((deg * Math.PI) / 180), from[1], from[2] - len * Math.sin((deg * Math.PI) / 180)];
    const p0: V3 = [u, v, -0.003];
    const p1 = dir(22, 0.042 * k, p0);
    const p2 = dir(52, 0.026 * k, p1);
    const p3 = dir(78, 0.02 * k, p2);
    const r = k < 0.9 ? 0.9 : 1;
    out.push(capsule(c(`hand-l-finger-${name}`), p0, p1, 0.0096 * r, 0.009 * r));
    out.push(capsule(c(`hand-l-finger-${name}-mid`), p1, p2, 0.0088 * r, 0.0082 * r));
    out.push(capsule(c(`hand-l-finger-${name}-tip`), p2, p3, 0.0082 * r, 0.0076 * r));
  }
  out.push(capsule(c("hand-l-thumb"), [0.02, 0.03, -0.01], [0.056, 0.052, -0.017], 0.0118, 0.0106));
  out.push(capsule(c("hand-l-thumb-mid"), [0.056, 0.052, -0.017], [0.086, 0.061, -0.022], 0.0104, 0.0094));
  out.push(capsule(c("hand-l-thumb-tip"), [0.086, 0.061, -0.022], [0.106, 0.063, -0.027], 0.0094, 0.0082));
  return out;
}

/* ---------------------------------------------------------------- body */

export function body50(): Json[] {
  const out: Json[] = [];
  const both = (solids: Json[]): void => {
    for (const s of solids) out.push(s);
    for (const s of solids) out.push(mirrored(s, toRight, (t) => (t === TONE.jacketFar ? TONE.jacket : t === TONE.trousersFar ? TONE.trousers : t === TONE.shoeFar ? TONE.shoe : t)));
  };

  out.push(loft({ id: "pelvis-trousers", bone: "pelvis", material: "wool-trouser", tone: TONE.trousers }, dense(seatStations(), 3)));
  out.push(loft({ id: "jacket", bone: "torso", material: "tweed", tone: TONE.jacket }, dense(jacketStations(), 4)));

  const leg: Json[] = [];
  leg.push(loft({ id: "thigh-l", bone: "thigh-l", material: "wool-trouser", tone: TONE.trousersFar }, dome(dense(THIGH_L, 3), 1, 0.014)));
  leg.push(loft({ id: "shank-l", bone: "shank-l", material: "wool-trouser", tone: TONE.trousersFar }, dome(dense(SHANK_L, 3), -1, 0.04)));
  leg.push(loft({ id: "sock-l", bone: "shank-l", material: "wool-sock", tone: 0.25 }, [H("shank-l", 0.164, -0.046, 0.04, 0.037), H("shank-l", 0.119, -0.047, 0.043, 0.0375), H("shank-l", 0.086, -0.05, 0.048, 0.039)].reverse()));
  leg.push(loft({ id: "shoe-rear-l", bone: "foot-l", material: "leather-shoe", tone: TONE.shoeFar }, dense(SHOE_REAR.map(([x, b, t, rw, e]) => X("foot-l", x, b, t, rw, e)), 3)));
  leg.push(loft({ id: "shoe-toe-l", bone: "toe-l", material: "leather-shoe", tone: TONE.shoeFar }, dense(SHOE_TOE.map(([x, b, t, rw, e]) => X("toe-l", x, b, t, rw, e)), 3)));
  const heelPlan: [number, number][] = [[0, -0.072], [0.02, -0.066], [0.031, -0.05], [0.036, -0.02], [0.038, 0.012], [-0.038, 0.012], [-0.036, -0.02], [-0.031, -0.05], [-0.02, -0.066]];
  const solePlan: [number, number][] = [[0.04, 0.012], [0.044, 0.05], [0.05, 0.095], [0.054, 0.135], [0.053, 0.166], [-0.053, 0.166], [-0.054, 0.135], [-0.05, 0.095], [-0.044, 0.05], [-0.04, 0.012]];
  const level = { transform: { rotation: aboutW(-boneDeg("foot-l")) } };
  const ankleY = 0.084;
  leg.push(extrude({ id: "shoe-heel-l", bone: "foot-l", material: "leather-shoe", tone: 0.12 }, "zx", heelPlan, -ankleY, -ankleY + 0.025, level));
  leg.push(extrude({ id: "shoe-sole-l", bone: "foot-l", material: "leather-shoe", tone: 0.12 }, "zx", solePlan, -ankleY, -ankleY + 0.012, level));
  const toePlan: [number, number][] = [[0.0535, -0.012], [0.052, 0.02], [0.044, 0.044], [0.03, 0.058], [0.012, 0.0635], [-0.012, 0.0635], [-0.03, 0.058], [-0.044, 0.044], [-0.052, 0.02], [-0.0535, -0.012]];
  leg.push(extrude({ id: "shoe-toe-sole-l", bone: "toe-l", material: "leather-shoe", tone: 0.12 }, "zx", toePlan, 0, 0.012));
  both(leg);

  const arm: Json[] = [];
  arm.push(loft({ id: "sleeve-l", bone: "arm-l", material: "tweed", tone: TONE.jacketFar }, dome(dome(dense(SLEEVE_L, 3), 1, 0.012), -1, 0.016, 2)));
  arm.push(loft({ id: "forearm-l", bone: "forearm-l", material: "tweed", tone: TONE.jacketFar }, dome(dense(CUFF_L, 3), -1, 0.042)));
  arm.push(loft({ id: "shirt-cuff-l", bone: "forearm-l", material: "cotton-flannel", tone: TONE.shirt }, [st(0.19, 0, 0, 0.031, 0.0345), st(0.228, 0, 0, 0.0295, 0.0335)]));
  arm.push(loft({ id: "wrist-l", bone: "forearm-l", material: "skin", tone: TONE.handFar }, [st(0.2, 0, 0, 0.0235, 0.0295), st(0.232, 0, 0, 0.021, 0.0285), st(0.252, 0, 0, 0.0198, 0.0278)]));
  both(arm);
  for (const s of out) if (s.id === "wrist-r") s.tone = TONE.hand;
  out.push(...leftHand(TONE.handFar));
  out.push(...rightHand(TONE.hand));

  const skin = (id: string, bone = "head", tone: number = TONE.skin, role = "body"): { id: string; bone: string; material: string; tone: number; role: string } => ({ id, bone, material: "skin", tone, role });
  out.push(loft(skin("neck", "neck", 0.78), dense(NECK, 2)));
  out.push(loft({ id: "shirt-collar", bone: "neck", material: "cotton-flannel", tone: TONE.shirt }, [st(-0.012, 0.002, 0, 0.068, 0.072, 2, 14), st(0.012, 0.003, 0, 0.0655, 0.0695, 2, 14), st(0.034, 0.004, 0, 0.0625, 0.0665, 2, 14)]));
  out.push(loft({ id: "collar", bone: "neck", material: "tweed", tone: 0.45 }, [st(-0.03, 0.02, 0, 0.088, 0.08, 2.1, 24), st(-0.004, 0.022, 0, 0.081, 0.073, 2.1, 24), st(0.016, 0.024, 0, 0.073, 0.066, 2.1, 24)]));
  out.push(loft(skin("head"), dome(dense(HEAD, 3), -1, 0.006, 2)));
  out.push(loft(skin("head-nose"), dome(dense([st(0.076, -0.088, 0, 0.0075, 0.0065, 2, -148), st(0.062, -0.0975, 0, 0.0085, 0.0075, 2, -148), st(0.047, -0.107, 0, 0.0105, 0.009, 2, -148), st(0.034, -0.114, 0, 0.014, 0.0105, 2, -140)], 3), 1, 0.008, 2)));
  out.push(ellipsoid(skin("head-nostrils"), [0.009, 0.012, 0.0165], [0.027, -0.0985, 0]));
  out.push(ellipsoid(skin("head-brow"), [0.008, 0.01, 0.046], [0.0815, -0.084, 0]));
  out.push(ellipsoid(skin("head-lips", "head", 0.68), [0.0062, 0.0065, 0.022], [-0.004, -0.083, 0]));
  out.push(ellipsoid(skin("head-chin"), [0.015, 0.014, 0.021], [-0.034, -0.068, 0]));
  for (const side of [1, -1]) {
    const n = side > 0 ? "l" : "r";
    out.push(ellipsoid(skin(`head-cheek-${n}`), [0.018, 0.0085, 0.015], [0.042, -0.052, side * 0.05]));
    out.push(ellipsoid(skin(`head-ear-${n}`, "head", 0.76), [0.034, 0.0175, 0.0065], [0.046, 0.016, side * 0.079], compose(aboutW(-8), aboutU(side * 24))));
    out.push(ellipsoid(skin(`head-eye-${n}`, "head", 0.15, "detail"), [0.0055, 0.005, 0.0115], [0.064, -0.0825, side * 0.031]));
    out.push(ellipsoid({ id: `hair-temple-${n}`, bone: "head", material: "hair", tone: TONE.hair }, [0.024, 0.036, 0.012], [0.074, 0.034, side * 0.068]));
  }
  out.push(ellipsoid({ id: "hair-white", bone: "head", material: "hair", tone: TONE.hair }, [0.04, 0.02, 0.066], [0.05, 0.078, 0], aboutW(-6)));

  const cap = { bone: "head", material: "cap-tweed", tone: TONE.cap };
  out.push(loft({ id: "cap", ...cap }, dense(crownStations(), 3)));
  out.push(loft({ id: "cap-brim", ...cap }, dense([st(0.109, -0.08, 0, 0.076, 0.0042, 2, 90), st(0.105, -0.108, 0, 0.071, 0.0042, 2, 90), st(0.1, -0.134, 0, 0.056, 0.004, 2, 90), st(0.0975, -0.148, 0, 0.03, 0.0038, 2, 90)], 3)));

  const frame = { bone: "head", material: "spectacle-frame", tone: 0.1, role: "detail" };
  const ring: [number, number][] = Array.from({ length: 9 }, (_, i) => [0.0016 * Math.cos((i * Math.PI) / 4), 0.021 + 0.0016 * Math.sin((i * Math.PI) / 4)] as [number, number]);
  for (const side of [1, -1]) {
    const n = side > 0 ? "l" : "r";
    out.push(revolve({ id: `glasses-rim-${n}`, ...frame }, ring, [0.062, -0.0965, side * 0.0315], [0, 1, 0], { slender: true }));
    out.push(capsule({ id: `glasses-temple-${n}`, ...frame }, [0.068, -0.094, side * 0.0555], [0.071, 0.012, side * 0.0795], 0.0015, 0.0015, { slender: true }));
  }
  out.push(capsule({ id: "glasses-bar", ...frame }, [0.0705, -0.0975, -0.056], [0.0705, -0.0975, 0.056], 0.0016, 0.0016, { slender: true }));

  const tweed = (id: string, tone = 0.45, role = "detail"): { id: string; bone: string; material: string; tone: number; role: string } => ({ id, bone: "torso", material: "tweed", tone, role });
  for (const side of [1, -1]) {
    const n = side > 0 ? "l" : "r";
    const lapel: [number, number, number][] = [[1.386, 0.062, 0.022], [1.344, 0.07, 0.03], [1.274, 0.062, 0.034], [1.204, 0.04, 0.03], [1.144, 0.016, 0.014]];
    out.push(loft(tweed(`lapel-${n}`), dense(lapel.map(([y, z, rw]) => { const x = jacketX(y, z, 1); return H("torso", y, x - 0.006, x + 0.0045, rw, 2, side * z); }), 2).reverse()));
    const py = 0.919;
    const pz = 0.118;
    const px = jacketX(py, pz, 1);
    out.push(box(tweed(`jacket-pocket-${n}`, 0.38), [0.05, 0.009, 0.14], L("torso", [px + 0.001, py, side * pz]), compose(aboutW(90 - boneDeg("torso")), aboutU(-side * jacketYaw(py, pz))), 0.003));
  }
  const bx = jacketX(1.259, 0.105, 1);
  out.push(box(tweed("jacket-breast-pocket", 0.4), [0.014, 0.009, 0.09], L("torso", [bx + 0.003, 1.259, 0.105]), compose(aboutW(90 - boneDeg("torso")), aboutU(-jacketYaw(1.259, 0.105))), 0.003));
  const vee: [number, number][] = [[1.149, 0.006], [1.204, 0.022], [1.264, 0.036], [1.314, 0.042]];
  out.push(loft({ id: "waistcoat", bone: "torso", material: "wool-knit", tone: TONE.waistcoat, role: "detail" }, vee.map(([y, rw]) => H("torso", y, jacketX(y, 0, 1) - 0.012, jacketX(y, 0, 1) + 0.0015, rw, 2))));
  const shirt: [number, number][] = [[1.306, 0.04], [1.344, 0.043], [1.379, 0.04], [1.404, 0.034]];
  out.push(loft({ id: "shirt-front", bone: "torso", material: "cotton-flannel", tone: TONE.shirt, role: "detail" }, shirt.map(([y, rw]) => H("torso", y, jacketX(y, 0, 1) - 0.014, jacketX(y, 0, 1) + 0.002, rw, 2))));
  [0.944, 1.034, 1.124].forEach((y, i) => out.push(ellipsoid(tweed(`jacket-button-${i}`, 0.1), [0.01, 0.0045, 0.01], L("torso", [jacketX(y, 0.012, 1) + 0.002, y, 0.012]), aboutW(90 - boneDeg("torso")))));

  const wood = { bone: "cane", material: "ash-wood", tone: TONE.wood };
  out.push(tube({ id: "cane-shaft", ...wood }, [[-0.004, 0, 0], [0.25, 0, 0], [0.5, 0, 0], [CANE_L - 0.034, 0, 0]], [0.011, 0.0105, 0.0098, 0.009]));
  out.push(tube({ id: "cane-crook", ...wood, tone: 0.4 }, crookPath(), [...Array.from({ length: 13 }, () => 0.011), 0.0108, 0.0102]));
  out.push(ellipsoid({ id: "cane-crook-nose", ...wood, tone: 0.4 }, [0.008, 0.0102, 0.0102], [0.03, -2 * CROOK_R - 0.003, 0]));
  out.push(revolve({ id: "cane-ferrule", bone: "cane", material: "rubber", tone: 0.05 }, [[CANE_L - 0.042, 0], [CANE_L - 0.042, 0.0118], [CANE_L - 0.022, 0.0125], [CANE_L - 0.006, 0.014], [CANE_L, 0.0132], [CANE_L, 0]], [0, 0, 0], [1, 0, 0]));
  return out;
}

/* ---------------------------------------------------------------- thermal */

const LAYERS = {
  jacket: [["jk-waistcoat", 0.004], ["jk-shirt", 0.007], ["jk-air", 0.0082]],
  trousers: [["tr-underwear", 0.003], ["tr-air", 0.004]],
  shoes: [["sh-air", 0.004], ["sh-sock", 0.008]],
  cap: [["cap-air", 0.003], ["cap-hair", 0.009]],
} as const;
const OUTER = { jacket: "jk-tweed", trousers: "tr-wool", shoes: "sh-leather", cap: "cap-tweed" } as const;
type Layered = keyof typeof LAYERS;
/** 🧅 The shells of a garment loft under its outer layer: the same sections moved inwards by the thickness of the layers outside. */
function shells(solid: Json, region: Layered): Json[] {
  return LAYERS[region].map(([layer, t], i) => ({
    ...solid,
    id: `th-${solid.id}-${layer}`,
    tone: Math.max(0.05, Math.round((solid.tone - 0.1 * (i + 1)) * 100) / 100),
    sections: solid.sections.map((q: Json) => ({ ...q, rx: Math.max(0.003, Math.round((q.rx - t) * 1e5) / 1e5), ry: Math.max(0.003, Math.round((q.ry - t) * 1e5) / 1e5) })),
    region,
    layer,
  }));
}

export function thermal50(body: readonly Json[]): Json[] {
  const out: Json[] = [];
  const by = (id: string): Json => { const s = body.find((b) => b.id === id); if (!s) throw new Error(`no body solid ${id}`); return s; };
  const layered = (id: string, region: Layered): void => { out.push(thOf(id, region, OUTER[region], id === "cap" ? { sectionFaces: ["-u"] } : {})); out.push(...shells(by(id), region)); };
  const ref = (ids: readonly string[], region: string, layer?: string): void => { for (const id of ids) { by(id); out.push(thOf(id, region, layer)); } };
  const sides = (ids: readonly string[]): string[] => ids.flatMap((id) => [id, toRight(id)]);

  for (const id of ["jacket", "collar", ...sides(["sleeve-l", "forearm-l"])]) layered(id, "jacket");
  ref(["lapel-l", "lapel-r", "jacket-pocket-l", "jacket-pocket-r", "jacket-breast-pocket", "jacket-button-0", "jacket-button-1", "jacket-button-2"], "jacket", "jk-tweed");
  ref(["waistcoat"], "jacket", "jk-waistcoat");
  ref(["shirt-front", "shirt-collar", "shirt-cuff-l", "shirt-cuff-r"], "jacket", "jk-shirt");
  for (const id of sides(["thigh-l", "shank-l"])) layered(id, "trousers");
  for (const id of sides(["shoe-rear-l", "shoe-toe-l"])) layered(id, "shoes");
  ref(sides(["shoe-heel-l", "shoe-sole-l", "shoe-toe-sole-l"]), "shoes", "sh-leather");
  ref(sides(["sock-l"]), "shoes", "sh-sock");
  layered("cap", "cap");
  ref(["cap-brim"], "cap", "cap-tweed");
  ref(["hair-white", "hair-temple-l", "hair-temple-r"], "hair", "hair-pelt");

  const skin = { bone: "head", material: "skin", tone: TONE.skin };
  out.push({ ...loft({ id: "th-head", ...skin }, dome(dense(HEAD, 3), -1, 0.006, 2), [true, false]), region: "face-skin" });
  out.push({ ...loft({ id: "th-neck", bone: "neck", material: "skin", tone: 0.78 }, [st(0.03, -0.001, 0, 0.0582, 0.06), st(0.06, -0.005, 0, 0.056, 0.057), st(0.088, -0.008, 0, 0.0555, 0.0555)], [false, false]), region: "face-skin" });
  ref(["head-nose", "head-nostrils", "head-brow", "head-lips", "head-chin", ...["cheek", "ear", "eye"].flatMap((k) => [`head-${k}-l`, `head-${k}-r`]), "glasses-rim-l", "glasses-rim-r", "glasses-temple-l", "glasses-temple-r", "glasses-bar"], "face-skin");
  ref(body.filter((b) => b.material === "skin" && /^(hand|wrist)/.test(b.id)).map((b) => b.id), "hand-skin");
  ref(["cane-shaft", "cane-crook", "cane-crook-nose", "cane-ferrule"], "cane-wood");

  out.push({ ...ellipsoid({ id: "th-breath-nose", ...skin }, [0.004, 0.0048, 0.0095], [0.0195, -0.1, 0]), region: "face-skin", tags: ["emitter", "breath-nose"] });
  out.push({ ...ellipsoid({ id: "th-breath-mouth", ...skin }, [0.004, 0.004, 0.0062], [-0.004, -0.0875, 0]), region: "face-skin", tags: ["emitter", "breath-mouth"] });
  out.push({ ...ellipsoid({ id: "th-plume", bone: "head", material: "cap-tweed", tone: 0.5 }, [0.005, 0.07, 0.07], [0.1785, 0.025, 0], aboutW(-5)), region: "jacket", tags: ["emitter", "plume"] });
  return out;
}

/* ---------------------------------------------------------------- structure */

const BONE = { material: "bone", tone: 0.62 };
const kgOf = (bone: string): Json => (ASSET.rig.bones as Json[]).find((b) => b.id === bone).mass;

export function structure50(body: readonly Json[]): Json[] {
  const out: Json[] = [];
  const m = (solid: Json, member: string): void => { out.push(part(solid, member, "member")); };
  const joint = (member: string, at: V3, r: number): void => { out.push(part(ellipsoid({ id: `joint-${member}`, bone: member, material: "bone", tone: 0.4 }, [r, r, r], at), member, "joint")); };
  const mass = (member: string): void => {
    const k = kgOf(member);
    const r = 0.0125 * Math.cbrt(k.kg);
    out.push(part(ellipsoid({ id: `mass-${member}`, bone: member, material: "bone", tone: 0.2, role: "detail" }, [r, r, r], [k.comX, k.comY, 0]), member, "mass"));
  };
  const T = (x: number, y: number, z = 0): V3 => L("torso", [x, y, z]);
  const spine: [number, number, number][] = [[-0.03, 0.989, 0.0195], [-0.026, 1.044, 0.019], [-0.03, 1.104, 0.018], [-0.034, 1.174, 0.0165], [-0.026, 1.249, 0.0155], [0.0, 1.314, 0.0145], [0.04, 1.369, 0.0135], [0.08, 1.406, 0.0125]];
  m(tube({ id: "spine", bone: "torso", ...BONE }, spine.map(([x, y]) => T(x, y)), spine.map((q) => q[2])), "torso");
  const hoops: [number, number, number, number][] = [[1.114, 0.044, 0.09, 0.122], [1.149, 0.046, 0.098, 0.132], [1.184, 0.048, 0.103, 0.138], [1.219, 0.05, 0.105, 0.14], [1.254, 0.052, 0.103, 0.138], [1.289, 0.054, 0.097, 0.132], [1.319, 0.057, 0.088, 0.122], [1.346, 0.061, 0.075, 0.104], [1.37, 0.066, 0.058, 0.078]];
  hoops.forEach(([y, x, rd, rw], i) => {
    const ring: [number, number][] = Array.from({ length: 9 }, (_, k) => [0.006 * Math.cos((k * Math.PI) / 4), rd + 0.006 * Math.sin((k * Math.PI) / 4)] as [number, number]);
    m(revolve({ id: `rib-${10 - i}`, bone: "torso", ...BONE }, ring, T(x, y), [1, 0, 0], { lateralScale: Math.round((rw / rd) * 1000) / 1000 }), "torso");
  });
  m(loft({ id: "sternum", bone: "torso", ...BONE }, [H("torso", 1.209, 0.144, 0.156, 0.016), H("torso", 1.274, 0.15, 0.162, 0.02), H("torso", 1.336, 0.142, 0.154, 0.024)]), "torso");
  for (const side of [1, -1]) {
    const n = side > 0 ? "l" : "r";
    m(capsule({ id: `clavicle-${n}`, bone: "torso", ...BONE }, T(0.146, 1.346, side * 0.022), T(0.098, 1.372, side * 0.168), 0.008, 0.0075), "torso");
    m(ellipsoid({ id: `scapula-${n}`, bone: "torso", ...BONE }, [0.075, 0.007, 0.048], T(-0.03, 1.289, side * 0.105), compose(aboutW(8), aboutU(side * 28))), "torso");
  }
  const P = (x: number, y: number, z = 0): V3 => L("pelvis", [x, y, z]);
  m(tube({ id: "sacrum", bone: "pelvis", ...BONE }, [P(-0.03, 0.989), P(-0.052, 0.944), P(-0.066, 0.904), P(-0.062, 0.876)], [0.028, 0.026, 0.017, 0.008]), "pelvis");
  for (const side of [1, -1]) {
    const n = side > 0 ? "l" : "r";
    m(ellipsoid({ id: `ilium-${n}`, bone: "pelvis", ...BONE }, [0.062, 0.07, 0.012], P(-0.022, 0.954, side * 0.092), compose(aboutU(side * 32), aboutV(side * -22))), "pelvis");
    m(capsule({ id: `ilium-body-${n}`, bone: "pelvis", ...BONE }, P(-0.05, 0.954, side * 0.034), P(0.0, 0.892, side * 0.084), 0.017, 0.019), "pelvis");
    m(capsule({ id: `pubis-${n}`, bone: "pelvis", ...BONE }, P(0.004, 0.886, side * 0.082), P(0.05, 0.866, side * 0.012), 0.012, 0.011), "pelvis");
    m(capsule({ id: `ischium-${n}`, bone: "pelvis", ...BONE }, P(-0.008, 0.882, side * 0.08), P(-0.04, 0.819, side * 0.056), 0.013, 0.014), "pelvis");
  }
  m(ellipsoid({ id: "cranium", bone: "head", ...BONE }, [0.07, 0.09, 0.0685], [0.086, 0.011, 0]), "head");
  m(ellipsoid({ id: "face-bones", bone: "head", ...BONE }, [0.04, 0.036, 0.052], [0.032, -0.046, 0]), "head");
  for (const side of [1, -1]) {
    const n = side > 0 ? "l" : "r";
    m(capsule({ id: `mandible-ramus-${n}`, bone: "head", ...BONE }, [0.03, -0.008, side * 0.05], [-0.018, -0.012, side * 0.046], 0.0075, 0.008), "head");
    m(capsule({ id: `mandible-body-${n}`, bone: "head", ...BONE }, [-0.018, -0.012, side * 0.046], [-0.035, -0.062, side * 0.008], 0.008, 0.009), "head");
  }
  const N = (x: number, y: number): V3 => L("neck", [x, y, 0]);
  m(tube({ id: "cervical-spine", bone: "neck", ...BONE }, [N(0.08, 1.406), N(0.106, 1.428), N(0.134, 1.451), N(0.158, 1.471)], [0.0125, 0.012, 0.0115, 0.011]), "neck");

  const leg: Json[] = [];
  const lm = (solid: Json, member: string): void => { leg.push(part(solid, member, "member")); };
  lm(capsule({ id: "femur-neck-l", bone: "thigh-l", ...BONE }, [0, 0, 0], [0.034, -0.006, 0.058], 0.0165, 0.0175), "thigh-l");
  lm(ellipsoid({ id: "femur-trochanter-l", bone: "thigh-l", ...BONE }, [0.023, 0.018, 0.015], [0.03, -0.008, 0.066]), "thigh-l");
  lm(tube({ id: "femur-l", bone: "thigh-l", ...BONE }, [[0.045, -0.005, 0.062], [0.12, 0.006, 0.048], [0.21, 0.012, 0.032], [0.3, 0.008, 0.016], [0.385, 0.0, 0.004]], [0.0155, 0.0142, 0.0135, 0.0142, 0.017]), "thigh-l");
  lm(ellipsoid({ id: "femur-condyles-l", bone: "thigh-l", ...BONE }, [0.021, 0.03, 0.041], [0.398, -0.006, 0]), "thigh-l");
  lm(ellipsoid({ id: "patella-l", bone: "thigh-l", ...BONE }, [0.021, 0.0095, 0.02], [0.396, 0.04, 0]), "thigh-l");
  lm(ellipsoid({ id: "tibia-plateau-l", bone: "shank-l", ...BONE }, [0.015, 0.026, 0.038], [0.02, -0.002, 0]), "shank-l");
  lm(loft({ id: "tibia-l", bone: "shank-l", ...BONE }, [st(0.026, 0.002, 0.002, 0.021, 0.019, 2), st(0.06, 0.008, 0.003, 0.016, 0.0165, 1.8), st(0.13, 0.011, 0.003, 0.0125, 0.0145, 1.7), st(0.25, 0.008, 0.003, 0.0115, 0.0125, 1.8), st(0.34, 0.004, 0.002, 0.0125, 0.0125, 2), st(0.38, 0.002, 0.0, 0.017, 0.0145, 2)]), "shank-l");
  lm(ellipsoid({ id: "malleolus-l", bone: "shank-l", ...BONE }, [0.014, 0.012, 0.0095], [0.39, 0.0, -0.027]), "shank-l");
  lm(capsule({ id: "fibula-l", bone: "shank-l", ...BONE }, [0.04, -0.018, 0.034], [0.398, -0.012, 0.03], 0.0068, 0.0072), "shank-l");
  lm(capsule({ id: "calcaneus-l", bone: "foot-l", ...BONE }, L("foot-l", [-0.05, 0.034, 0.09]), L("foot-l", [0.006, 0.046, 0.09]), 0.021, 0.019), "foot-l");
  lm(loft({ id: "midfoot-l", bone: "foot-l", ...BONE }, [X("foot-l", -0.004, 0.034, 0.074, 0.024), X("foot-l", 0.06, 0.028, 0.064, 0.033), X("foot-l", 0.11, 0.02, 0.046, 0.039), X("foot-l", 0.16, 0.013, 0.031, 0.042)]), "foot-l");
  lm(loft({ id: "phalanges-l", bone: "toe-l", ...BONE }, [X("toe-l", 0.15, 0.013, 0.031, 0.042), X("toe-l", 0.19, 0.012, 0.027, 0.038), X("toe-l", 0.211, 0.012, 0.023, 0.027)]), "toe-l");
  lm(ellipsoid({ id: "humerus-head-l", bone: "arm-l", ...BONE }, [0.022, 0.022, 0.022], [0.002, 0, -0.022]), "arm-l");
  lm(tube({ id: "humerus-l", bone: "arm-l", ...BONE }, [[0.012, 0, -0.018], [0.1, 0.004, -0.008], [0.2, 0.003, -0.002], [0.294, 0, 0]], [0.012, 0.0108, 0.0104, 0.0125]), "arm-l");
  lm(ellipsoid({ id: "humerus-condyles-l", bone: "arm-l", ...BONE }, [0.012, 0.014, 0.031], [0.302, 0, 0]), "arm-l");
  lm(capsule({ id: "ulna-l", bone: "forearm-l", ...BONE }, [-0.014, -0.017, 0], [0.236, -0.014, 0], 0.0082, 0.0052), "forearm-l");
  lm(capsule({ id: "radius-l", bone: "forearm-l", ...BONE }, [0.01, 0.015, 0], [0.24, 0.011, 0], 0.006, 0.0088), "forearm-l");
  for (const s of leg) out.push(s);
  for (const s of leg) out.push(mirrored(s, toRight));

  for (const hand of ["hand-l", "hand-r"]) {
    const rays = body.filter((b) => b.bone === hand && b.kind === "capsule");
    const carpus: V3 = hand === "hand-l" ? [0.014, 0, 0] : hr([0.006, -0.006, -0.008]);
    m(ellipsoid({ id: `carpals-${hand.slice(-1)}`, bone: hand, ...BONE }, [0.014, 0.014, 0.014], carpus), hand);
    for (const ray of rays) {
      m(capsule({ id: ray.id.replace("hand-", "bones-"), bone: hand, ...BONE }, ray.p0, ray.p1, 0.0044, 0.004), hand);
      if (/finger-\d$|thumb$/.test(ray.id)) m(capsule({ id: `${ray.id.replace("hand-", "bones-")}-metacarpal`, bone: hand, ...BONE }, mid(carpus, ray.p0, 0.25), ray.p0, 0.0048, 0.0046), hand);
    }
  }
  for (const id of ["cane-shaft", "cane-crook", "cane-crook-nose", "cane-ferrule"]) out.push({ id: `member-${id}`, of: id, member: "cane", part: "member" });

  joint("torso", T(-0.03, 0.989), 0.017);
  joint("neck", N(0.08, 1.406), 0.0135);
  joint("head", L("head", [0.158, 1.471, 0]), 0.0125);
  for (const side of ["l", "r"]) {
    const k = side === "l" ? 1 : -1;
    joint(`thigh-${side}`, [0, 0, 0], 0.0245);
    joint(`shank-${side}`, [0, 0, 0], 0.019);
    joint(`foot-${side}`, [0, 0.002, 0], 0.016);
    joint(`toe-${side}`, [-0.004, 0.02, 0], 0.01);
    joint(`arm-${side}`, [0.002, 0, k * -0.022], 0.0235);
    joint(`forearm-${side}`, [0, 0, 0], 0.0155);
    joint(`hand-${side}`, side === "l" ? [0, 0, 0] : [0, 0, 0], 0.012);
  }
  joint("cane", [-CROOK_R, -CROOK_R, 0], 0.0125);
  for (const b of ["pelvis", "torso", "head", "thigh-l", "thigh-r", "shank-l", "shank-r", "arm-l", "arm-r", "forearm-l", "forearm-r", "foot-l", "foot-r"]) mass(b);
  return out;
}

export const DECISIONS_50: Json[] = [
  { en: "1:50 body: a 74-year-old man of 1.65 m, modelled as garments over the body. The tweed jacket is one loft of horizontal sections that falls from the rounded upper back over the seat and hangs from chest and belly in front, with sleeve heads set inwards of the acromion, lapels, a waistcoat and shirt front between them, three buttons, flap pockets and a breast pocket; shirt collar and cuffs show at neck and wrists. Trousers are a seat and two legs per bone with the calf pushing the cloth back and a break over the instep; walking shoes have an upper per bone, a heel block and a sole. The head below the cap line has chin, jaw, cheeks, brow, nose, lips, ears and glasses; the flat cap is a low crown pulled forward onto a stiff peak; white hair shows at the nape and temples. The left hand hangs relaxed with a thumb and three-part fingers, the right hand lies on the crook with the fingers wrapped down its outer side and the thumb on the inner side. The cane is a tapered ash shaft, a half-round crook and a rubber ferrule.", de: "Körper 1:50: ein 74-jähriger Mann von 1,65 m, als Kleidung über dem Körper modelliert. Die Tweedjacke ist ein Loft aus waagrechten Schnitten, der vom runden oberen Rücken über das Gesäß fällt und vorn von Brust und Bauch hängt, mit nach innen gesetzten Ärmelkugeln, Revers, Weste und Hemdbrust dazwischen, drei Knöpfen, Pattentaschen und Brusttasche; Hemdkragen und Manschetten zeigen sich an Hals und Handgelenken. Die Hose besteht aus Sitz und zwei Beinen je Knochen, die Wade drückt den Stoff nach hinten, über dem Rist bricht er; die Wanderschuhe haben je Knochen ein Oberteil, einen Absatz und eine Sohle. Der Kopf unter der Mützenlinie hat Kinn, Kiefer, Wangen, Brauen, Nase, Lippen, Ohren und Brille; die Schiebermütze ist ein flacher Deckel, nach vorn auf den steifen Schirm gezogen; weißes Haar zeigt sich an Nacken und Schläfen. Die linke Hand hängt entspannt mit Daumen und dreigliedrigen Fingern, die rechte liegt auf dem Krückgriff, die Finger an seiner Außenseite herabgelegt, der Daumen innen. Der Stock ist ein verjüngter Eschenschaft mit halbrundem Griff und Gummizwinge." },
  { en: "1:50 thermal: every garment loft is its outer layer by reference and carries its inner layers as nested shells with the declared thicknesses (jacket: waistcoat, shirt, air; trousers: underwear, air; shoes: air, sock; cap: air, hair); details are references in their own layer (waistcoat and shirt fronts, collar and cuffs). The seat of the trousers lies under the jacket skirt, where the trunk is the jacket region, so it is no thermal surface of its own. Face and neck skin are open skins of the visible part only, so the skull under the cap and the neck inside the collar do not count twice; the underside of the cap lies on the hair and is marked as a face against the head, the underside of the peak stays exposed; the breath outlets are patches under the nose and on the lips, the plume outlet is a patch on the crown of the cap.", de: "Thermik 1:50: Jeder Kleidungs-Loft ist per Verweis seine äußere Schicht und trägt die inneren Schichten als geschachtelte Schalen mit den deklarierten Dicken (Jacke: Weste, Hemd, Luft; Hose: Unterwäsche, Luft; Schuhe: Luft, Socke; Mütze: Luft, Haar); Details sind Verweise in ihrer eigenen Schicht (Westen- und Hemdbrust, Kragen und Manschetten). Der Hosensitz liegt unter dem Jackenschoß, wo der Rumpf zur Jackenregion gehört, und ist daher keine eigene thermische Fläche. Gesichts- und Halshaut sind offene Häute nur des sichtbaren Teils, damit der Schädel unter der Mütze und der Hals im Kragen nicht doppelt zählen; die Unterseite der Mütze liegt auf dem Haar und ist als Fläche am Kopf markiert, die Unterseite des Schirms bleibt frei; die Atemauslässe sind Flächen unter der Nase und auf den Lippen, der Auslass der Wärmefahne eine Fläche auf dem Mützendeckel." },
  { en: "1:50 structure: the skeleton in its stooped form, not the body. A spine with the remaining lumbar curve and the thoracic kyphosis, nine rib hoops and a sternum, clavicles and scapulae, sacrum, iliac wings, pubic and ischial rami, cranium, facial bones and mandible, cervical spine; femur with neck, trochanter, bowed shaft and condyles, patella, tibia with plateau and malleolus, fibula, calcaneus, midfoot and phalanges; humerus with head and condyles, radius and ulna, carpals, metacarpals and phalanges inside every finger. Joints are balls at the rig joints (the shoulder ball at the glenohumeral centre, inwards of the acromion the bone hangs from), masses are balls at the centres of mass sized by the cube root of the segment mass, and the cane is its own member by reference. The asset declares no structural supports, so there is no support solid.", de: "Tragwerk 1:50: das Skelett in seiner gebeugten Form, nicht der Körper. Eine Wirbelsäule mit der verbliebenen Lendenkrümmung und der Brustkyphose, neun Rippenbögen und Brustbein, Schlüsselbeine und Schulterblätter, Kreuzbein, Darmbeinschaufeln, Scham- und Sitzbeinäste, Hirnschädel, Gesichtsschädel und Unterkiefer, Halswirbelsäule; Oberschenkelknochen mit Hals, Rollhügel, gebogenem Schaft und Kondylen, Kniescheibe, Schienbein mit Plateau und Knöchel, Wadenbein, Fersenbein, Mittelfuß und Zehenglieder; Oberarmknochen mit Kopf und Kondylen, Speiche und Elle, Handwurzel, Mittelhandknochen und Fingerglieder in jedem Finger. Gelenke sind Kugeln an den Rig-Gelenken (die Schulterkugel im Zentrum des Schultergelenks, innen vom Schulterdach, an dem der Knochen hängt), Massen sind Kugeln in den Schwerpunkten, bemessen nach der dritten Wurzel der Segmentmasse, und der Stock ist per Verweis sein eigener Stab. Das Asset deklariert keine Auflager, daher gibt es keinen Auflagerkörper." },
];

/** 🤲 The placement of the right palm on the crook: centre, rotation and the knuckle line. */
export function palmFrame(): { at: V3; rotation: Quat; knuckle: V3 } {
  const knuckle = around(0, 50, 0.03);
  const e1 = unit(knuckle);
  const ahead = hr([1, 0, 0]);
  const k = ahead[0] * e1[0] + ahead[1] * e1[1] + ahead[2] * e1[2];
  const e2 = unit(sub(ahead, e1.map((x) => x * k) as V3));
  return { at: mid([0, 0, 0], knuckle, 0.5), rotation: quatOf(e1, e2, cross(e1, e2)), knuckle };
}
/** 🧤 The four fingers of the right hand as one band wrapped over the crook: sections `halfWidth` along the handle and `halfThick` across, centred `s` ahead of the apex. */
export function fingerBand(halfWidth: number, halfThick: number, s = 0.001): Json[] {
  const stations: [number, number][] = [[50, 0.03], [-35, 0.0245], [-110, 0.022], [-165, 0.021]];
  const at = stations.map(([phi, r]) => around(s, phi, r));
  return at.map((p, i) => {
    const ez = unit(sub(at[Math.min(i + 1, at.length - 1)]!, at[Math.max(i - 1, 0)]!));
    const along = unit(sub(around(s + 0.02, stations[i]![0], stations[i]![1]), around(s - 0.02, stations[i]![0], stations[i]![1])));
    const ey = unit(cross(ez, along));
    const ex = cross(ey, ez);
    return { at: p.map((x) => Math.round(x * 1e5) / 1e5), rx: halfWidth, ry: halfThick, exponent: 2.6, rotation: quatOf(ex, ey, ez) };
  });
}
export { D, flip, inset, far, hr, around, quatOf, mid, TONE, OUTER, BONE, kgOf, CROOK_R };

