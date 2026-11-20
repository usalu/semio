/** 🧥 The 1:50 model of `humans/walking-commuter`, stated section by section: a man of 1.78 m and 72 kg (1.80 m in his shoes) in a knee-length single-breasted wool overcoat with a scarf, wool trousers and derby shoes, a leather shoulder bag hanging from the right shoulder at the hip and the right hand closed on its strap in front of the chest.
 *
 * Figure coordinates (`H`, `X`, `L`): x ahead, y up, z to the left, the default pose `standing`. Local stations (`st`): u along the bone, v its sagittal normal, w lateral.
 * @see ./r3-walking-commuter-kit.ts
 * @see ./r3-humans-walking-commuter.md */
import { axial, type Fit } from "./r4-g06-skeleton.ts";
import type { Json } from "./volumes-lib.ts";
import { ASSET, FRAMES, H, L, X, aboutU, aboutV, aboutW, box, boneDeg, capsule, compose, dense, dome, ellipsoid, extrude, loft, mirrored, part, revolve, st, thOf, toRight, tube, type St, type V3 } from "./r3-walking-commuter-kit.ts";

export const TONE = { coat: 0.3, coatFar: 0.25, trousers: 0.42, trousersFar: 0.36, shoe: 0.2, shoeFar: 0.15, skin: 0.8, hand: 0.78, handFar: 0.71, hair: 0.14, scarf: 0.58, bag: 0.46, strap: 0.36, shirt: 0.9, sole: 0.1 } as const;
export const near = (t: number): number => (t === TONE.coatFar ? TONE.coat : t === TONE.trousersFar ? TONE.trousers : t === TONE.shoeFar ? TONE.shoe : t === TONE.handFar ? TONE.hand : t);
export type Row = readonly [number, number, number, number, number];

/* ---------------------------------------------------------------- stations shared by body, thermal and the drawings */

/** 🧥 The overcoat on the trunk as horizontal slices from the hem to the collar seam: [y, back, front, half-width, exponent]. Shoulder blades and seat shape the back, chest the front; the skirt flares a little to the hem at the knee. */
export const COAT: readonly Row[] = [
  [0.556, -0.15, 0.15, 0.212, 2.5],
  [0.64, -0.15, 0.148, 0.21, 2.5],
  [0.76, -0.148, 0.143, 0.205, 2.5],
  [0.88, -0.146, 0.138, 0.199, 2.45],
  [0.96, -0.142, 0.136, 0.194, 2.4],
  [1.05, -0.128, 0.134, 0.183, 2.4],
  [1.12, -0.116, 0.133, 0.174, 2.4],
  [1.2, -0.114, 0.137, 0.171, 2.4],
  [1.28, -0.118, 0.143, 0.172, 2.4],
  [1.36, -0.122, 0.143, 0.178, 2.35],
  [1.42, -0.12, 0.136, 0.2, 2.3],
  [1.462, -0.112, 0.126, 0.218, 2.3],
  [1.488, -0.102, 0.114, 0.221, 2.25],
  [1.508, -0.09, 0.102, 0.212, 2.2],
  [1.526, -0.076, 0.09, 0.184, 2.15],
  [1.543, -0.06, 0.078, 0.136, 2.1],
  [1.556, -0.048, 0.068, 0.092, 2],
];
export const coatStations = (table: readonly Row[] = COAT): St[] => table.map(([y, b, f, rw, e]) => H("torso", y, b, f, rw, e));
/** 📏 The coat surface at the height `y` and the lateral position `z`: x of the front (+1) or of the back (-1). */
export function coatX(y: number, z: number, side: 1 | -1): number {
  let i = 0;
  while (i + 2 < COAT.length && COAT[i + 1]![0] < y) i++;
  const a = COAT[i]!;
  const b = COAT[i + 1]!;
  const t = Math.min(1, Math.max(0, (y - a[0]) / (b[0] - a[0])));
  const m = (k: number): number => a[k]! + (b[k]! - a[k]!) * t;
  const [back, front, rw, e] = [m(1), m(2), m(3), m(4)];
  const k = Math.pow(Math.max(0, 1 - Math.pow(Math.min(1, Math.abs(z) / rw), e)), 1 / e);
  return (back + front) / 2 + (side * k * (front - back)) / 2;
}
/** 🧭 The outward direction of the coat front at (y, z) as an angle about the vertical, degrees from straight ahead towards the left. */
export const coatYaw = (y: number, z: number): number => (Math.atan2(coatX(y, z - 0.004, 1) - coatX(y, z + 0.004, 1), 0.008) * 180) / Math.PI;

/** 👔 The lapel of one coat front from the top button to the collar notch: [y, inner edge z, outer edge z]. */
export const LAPEL: readonly [number, number, number][] = [[1.365, 0.003, 0.018], [1.39, 0.014, 0.046], [1.415, 0.025, 0.07], [1.435, 0.034, 0.088], [1.455, 0.043, 0.104], [1.476, 0.052, 0.12], [1.496, 0.055, 0.106], [1.516, 0.058, 0.093]];
/** 👖 The seat of the trousers under the coat. */
export const SEAT: readonly Row[] = [
  [0.842, -0.084, 0.07, 0.128, 2.3],
  [0.9, -0.118, 0.1, 0.168, 2.4],
  [0.96, -0.124, 0.112, 0.172, 2.4],
  [1.02, -0.112, 0.116, 0.164, 2.3],
  [1.085, -0.1, 0.116, 0.156, 2.3],
];
/** 👖 The left trouser leg on the thigh, bone-local: quadriceps in front, hamstrings behind, the inner seam clear of the other leg. */
export const THIGH_L: St[] = [st(0.0, -0.005, 0.008, 0.09, 0.1, 2.1), st(0.1, -0.003, 0.008, 0.088, 0.097, 2.1), st(0.2, 0.003, 0.004, 0.08, 0.089, 2), st(0.3, 0.005, 0, 0.071, 0.079, 2), st(0.38, 0.004, 0, 0.064, 0.071, 2), st(0.435, 0.004, 0, 0.061, 0.068, 2)];
/** 🦵 The left trouser leg on the shank: knee, the calf pushing the cloth back, the break over the instep and the hem cut lower at the heel. */
export const SHANK_L: St[] = [st(0.0, 0.004, 0, 0.061, 0.068, 2), st(0.05, 0.002, 0, 0.06, 0.066, 2), st(0.13, -0.008, 0, 0.059, 0.068, 2), st(0.23, -0.008, 0, 0.056, 0.064, 2), st(0.33, -0.004, 0, 0.054, 0.062, 2), st(0.398, 0.003, 0, 0.0545, 0.0655, 2, -5), st(0.428, 0.004, 0, 0.053, 0.0625, 2, -10)];

/** 🧥 The left sleeve on the upper arm: the sleeve head is set into the shoulder inwards of the bone, the deltoid rounds it, the elbow hangs a little outwards (carrying angle). */
export const SLEEVE_L: St[] = [st(-0.022, 0.0, -0.03, 0.04, 0.056), st(-0.004, 0.0, -0.012, 0.054, 0.066), st(0.045, 0.002, 0.004, 0.061, 0.069), st(0.12, 0.004, 0.008, 0.059, 0.066), st(0.2, 0.002, 0.01, 0.056, 0.062), st(0.28, -0.002, 0.012, 0.054, 0.059), st(0.33, -0.004, 0.012, 0.053, 0.058)];
/** 💪 The left sleeve on the forearm down to the cuff, running outwards to the wrist. */
export const CUFF_L: St[] = [st(0.0, -0.004, 0.012, 0.053, 0.058), st(0.06, 0.0, 0.016, 0.052, 0.056), st(0.14, 0.0, 0.022, 0.048, 0.05), st(0.21, 0, 0.027, 0.046, 0.046), st(0.246, 0, 0.03, 0.0455, 0.0445)];
const WRIST_W = 0.031;

/** 🥾 The left shoe upper as cross slices: [x, bottom, top, half-width, exponent]; quarters and vamp on the foot bone, toe box on the toe bone, overlapping at the ball. */
export const SHOE_REAR: readonly Row[] = [
  [-0.061, 0.03, 0.066, 0.018, 2],
  [-0.048, 0.014, 0.088, 0.032, 2.4],
  [-0.02, 0.009, 0.094, 0.039, 2.4],
  [0.02, 0.008, 0.097, 0.043, 2.4],
  [0.06, 0.008, 0.09, 0.047, 2.4],
  [0.1, 0.008, 0.074, 0.05, 2.5],
  [0.14, 0.008, 0.061, 0.052, 2.6],
  [0.174, 0.008, 0.053, 0.052, 2.6],
];
export const SHOE_TOE: readonly [number, number, number, number, number, number][] = [
  [0.15, 0.008, 0.058, 0.052, 2.6, 0],
  [0.185, 0.008, 0.05, 0.05, 2.6, -0.002],
  [0.212, 0.008, 0.042, 0.042, 2.4, -0.005],
  [0.228, 0.009, 0.033, 0.028, 2.2, -0.008],
  [0.2345, 0.011, 0.024, 0.013, 2, -0.009],
];
export const HEEL_PLAN: [number, number][] = [[0, -0.064], [0.02, -0.059], [0.031, -0.044], [0.036, -0.016], [0.038, 0.014], [-0.038, 0.014], [-0.036, -0.016], [-0.031, -0.044], [-0.02, -0.059]];
export const SOLE_PLAN: [number, number][] = [[0.04, 0.008], [0.044, 0.05], [0.05, 0.095], [0.054, 0.135], [0.054, 0.174], [-0.054, 0.174], [-0.053, 0.135], [-0.048, 0.095], [-0.042, 0.05], [-0.04, 0.008]];
export const TOE_PLAN: [number, number][] = [[0.0545, -0.014], [0.053, 0.018], [0.044, 0.042], [0.027, 0.058], [0.006, 0.066], [-0.02, 0.0655], [-0.04, 0.056], [-0.051, 0.04], [-0.054, 0.018], [-0.0545, -0.014]];

/** 🗣️ The head as horizontal slices from the chin to the crown of the skull: [y, back, front, half-width, exponent]. */
export const HEAD: readonly Row[] = [
  [1.57, 0.056, 0.098, 0.02, 2],
  [1.58, 0.03, 0.104, 0.034, 2],
  [1.595, -0.006, 0.107, 0.045, 2],
  [1.615, -0.038, 0.108, 0.054, 2],
  [1.64, -0.06, 0.109, 0.061, 2],
  [1.665, -0.074, 0.11, 0.0665, 2],
  [1.69, -0.08, 0.107, 0.0705, 2],
  [1.718, -0.082, 0.117, 0.074, 2],
  [1.745, -0.079, 0.112, 0.071, 2],
  [1.765, -0.068, 0.098, 0.062, 2],
  [1.78, -0.052, 0.079, 0.049, 2],
  [1.79, -0.032, 0.056, 0.033, 2],
  [1.795, -0.012, 0.034, 0.016, 2],
];
/** 💇 The short hair as slices from the nape to the crown: it lies behind the ears and the face below the hairline and closes over the skull above it. */
export const HAIR: readonly Row[] = [
  [1.622, -0.07, -0.03, 0.044, 2],
  [1.642, -0.081, -0.02, 0.06, 2],
  [1.668, -0.088, -0.012, 0.0695, 2],
  [1.7, -0.091, 0.03, 0.0765, 2],
  [1.73, -0.091, 0.084, 0.08, 2],
  [1.752, -0.086, 0.098, 0.077, 2],
  [1.77, -0.075, 0.104, 0.068, 2],
  [1.785, -0.058, 0.087, 0.055, 2],
  [1.796, -0.036, 0.061, 0.037, 2],
  [1.8025, -0.012, 0.035, 0.018, 2],
];
export const NECK: readonly Row[] = [
  [1.5, -0.06, 0.058, 0.064, 2],
  [1.555, -0.056, 0.06, 0.059, 2],
  [1.61, -0.05, 0.064, 0.056, 2],
  [1.66, -0.044, 0.066, 0.055, 2],
];
/** 🧣 The scarf wound once round the neck above the collar. */
export const SCARF: readonly Row[] = [
  [1.528, -0.078, 0.084, 0.086, 2.2],
  [1.552, -0.087, 0.096, 0.092, 2.2],
  [1.582, -0.088, 0.099, 0.093, 2.2],
  [1.606, -0.08, 0.094, 0.087, 2.2],
  [1.622, -0.066, 0.082, 0.074, 2.1],
];

/* ---------------------------------------------------------------- the bag and its strap */

export const BAG = { x: -0.01, y: 0.945, z: -0.243, height: 0.225, length: 0.31, depth: 0.085 } as const;
/** 👜 A point of the standing figure in the frame of the bag bone. */
export const B = (x: number, y: number, z: number): V3 => L("bag", [x, y, z]);
/** 🎗️ The centre line of the strap in figure coordinates: from the front lug of the bag up the chest, through the right fist, over the shoulder and down the back to the rear lug. */
export const STRAP: readonly V3[] = [
  [0.13, 1.066, -0.243],
  [0.13, 1.15, -0.2],
  [0.131, 1.25, -0.156],
  [0.131, 1.32, -0.138],
  [0.129, 1.38, -0.135],
  [0.118, 1.44, -0.136],
  [0.094, 1.492, -0.138],
  [0.052, 1.528, -0.14],
  [0.0, 1.541, -0.14],
  [-0.048, 1.528, -0.14],
  [-0.085, 1.496, -0.142],
  [-0.107, 1.44, -0.145],
  [-0.112, 1.36, -0.15],
  [-0.118, 1.25, -0.172],
  [-0.14, 1.15, -0.208],
  [-0.15, 1.066, -0.243],
];
/** 🎀 A flat band along a path of the standing figure, bone-local: sections across the path in the sagittal plane, `halfWidth` lateral and `halfThick` across. */
export function ribbon(bone: string, path: readonly V3[], halfWidth: number, halfThick: number): St[] {
  const q = path.map((p) => L(bone, p));
  let last = 0;
  return q.map((at, i) => {
    const a = q[Math.max(0, i - 1)]!;
    const b = q[Math.min(q.length - 1, i + 1)]!;
    let n = (Math.atan2(b[1] - a[1], b[0] - a[0]) * 180) / Math.PI;
    while (n - last > 180) n -= 360;
    while (n - last < -180) n += 360;
    last = n;
    return { at, rw: halfWidth, rv: halfThick, e: 4, n };
  });
}

/* ---------------------------------------------------------------- hands */

const HAND_SCALE = 1.07;
/** ✋ The hanging left hand: palm towards the thigh, thumb ahead, fingers of three phalanges curling inwards. */
function leftHand(tone: number): Json[] {
  const k0 = HAND_SCALE;
  const c = (id: string): { id: string; bone: string; material: string; tone: number } => ({ id, bone: "hand-l", material: "skin", tone });
  const w0 = WRIST_W + 0.002;
  const out: Json[] = [];
  const palm: St[] = [st(-0.01, 0, w0, 0.0205, 0.029), st(0.022, 0.002, w0, 0.018, 0.036), st(0.054, 0.002, w0 - 0.001, 0.0155, 0.043), st(0.088, 0.0, w0 - 0.003, 0.0135, 0.044), st(0.1, 0, w0 - 0.004, 0.012, 0.041)];
  out.push(loft(c("hand-l"), dome(dense(palm, 3), 1, 0.008, 2)));
  out.push(ellipsoid(c("hand-l-thenar"), [0.032, 0.017, 0.012], [0.043, 0.03, w0 - 0.011]));
  const fingers: [string, number, number, number][] = [["1", 0.096, 0.032, 1], ["2", 0.1, 0.011, 1.08], ["3", 0.096, -0.0105, 1], ["4", 0.088, -0.031, 0.8]];
  for (const [name, u, v, k] of fingers) {
    const dir = (deg: number, len: number, from: V3): V3 => [from[0] + len * Math.cos((deg * Math.PI) / 180), from[1], from[2] - len * Math.sin((deg * Math.PI) / 180)];
    const p0: V3 = [u, v, w0 - 0.003];
    const p1 = dir(20, 0.045 * k * k0 * 0.94, p0);
    const p2 = dir(48, 0.028 * k * k0 * 0.94, p1);
    const p3 = dir(72, 0.021 * k * k0 * 0.94, p2);
    const r = k < 0.9 ? 0.9 : 1;
    out.push(capsule(c(`hand-l-finger-${name}`), p0, p1, 0.0102 * r, 0.0095 * r));
    out.push(capsule(c(`hand-l-finger-${name}-mid`), p1, p2, 0.0093 * r, 0.0086 * r));
    out.push(capsule(c(`hand-l-finger-${name}-tip`), p2, p3, 0.0086 * r, 0.0079 * r));
  }
  out.push(capsule(c("hand-l-thumb"), [0.021, 0.032, w0 - 0.01], [0.06, 0.055, w0 - 0.017], 0.0125, 0.0112));
  out.push(capsule(c("hand-l-thumb-mid"), [0.06, 0.055, w0 - 0.017], [0.092, 0.065, w0 - 0.022], 0.011, 0.0099));
  out.push(capsule(c("hand-l-thumb-tip"), [0.092, 0.065, w0 - 0.022], [0.113, 0.067, w0 - 0.027], 0.0099, 0.0086));
  return out;
}

/** 🤜 The wrist of the right arm in figure coordinates: the end of the forearm bone, moved outwards with the sleeve. */
export const WRIST_R: V3 = [FRAMES["hand-r"]!.o[0], FRAMES["hand-r"]!.o[1], FRAMES["hand-r"]!.o[2] - WRIST_W];
/** 🫱 A point given from the right wrist (x ahead, y up, z towards the body) in the frame of the right hand. */
export const R = (dx: number, dy: number, dz: number): V3 => L("hand-r", [WRIST_R[0] + dx, WRIST_R[1] + dy, WRIST_R[2] + dz]);
/** ✊ The right hand closed on the strap: the palm turned to the chest, the metacarpals running inwards from the wrist, four fingers wrapped round the inner edge of the strap and the thumb under it. */
function rightHand(tone: number): Json[] {
  const c = (id: string): { id: string; bone: string; material: string; tone: number } => ({ id, bone: "hand-r", material: "skin", tone });
  const out: Json[] = [];
  const turn = compose(aboutW(-boneDeg("hand-r")), aboutV(8));
  out.push(ellipsoid(c("hand-r"), [0.0165, 0.041, 0.05], R(0.012, 0.022, 0.046), turn));
  const fingers: [string, number, number][] = [["1", 0.052, 1], ["2", 0.031, 1.05], ["3", 0.01, 1], ["4", -0.01, 0.84]];
  for (const [name, dy, k] of fingers) {
    const p0 = R(0.017, dy, 0.09);
    const p1 = R(0.014, dy - 0.002, 0.09 + 0.034 * k);
    const p2 = R(-0.012, dy - 0.004, 0.092 + 0.04 * k);
    const p3 = R(-0.028, dy - 0.006, 0.086 + 0.022 * k);
    out.push(capsule(c(`hand-r-finger-${name}`), p0, p1, 0.0102 * (k < 0.9 ? 0.9 : 1), 0.0095 * (k < 0.9 ? 0.9 : 1)));
    out.push(capsule(c(`hand-r-finger-${name}-mid`), p1, p2, 0.0093 * (k < 0.9 ? 0.9 : 1), 0.0086 * (k < 0.9 ? 0.9 : 1)));
    out.push(capsule(c(`hand-r-finger-${name}-tip`), p2, p3, 0.0086 * (k < 0.9 ? 0.9 : 1), 0.0079 * (k < 0.9 ? 0.9 : 1)));
  }
  out.push(capsule(c("hand-r-thumb"), R(0.006, 0.05, 0.022), R(-0.004, 0.068, 0.058), 0.0125, 0.0112));
  out.push(capsule(c("hand-r-thumb-tip"), R(-0.004, 0.068, 0.058), R(-0.016, 0.066, 0.09), 0.011, 0.0092));
  return out;
}

/* ---------------------------------------------------------------- body */

const LEVEL = { transform: { rotation: aboutW(-boneDeg("foot-l")) } };
export const ANKLE_Y = FRAMES["foot-l"]!.o[1];

export function body50(): Json[] {
  const out: Json[] = [];
  const both = (solids: Json[]): void => {
    for (const s of solids) out.push(s);
    for (const s of solids) out.push(mirrored(s, toRight, near));
  };
  const coat = (id: string, bone = "torso", tone: number = TONE.coat, role = "body"): { id: string; bone: string; material: string; tone: number; role: string } => ({ id, bone, material: "wool-coat", tone, role });

  out.push(loft({ id: "pelvis-trousers", bone: "pelvis", material: "wool-trouser", tone: TONE.trousers }, dense(SEAT.map(([y, b, f, rw, e]) => H("pelvis", y, b, f, rw, e)), 3)));
  out.push(loft(coat("coat"), dense(coatStations(), 4)));

  const leg: Json[] = [];
  leg.push(loft({ id: "thigh-l", bone: "thigh-l", material: "wool-trouser", tone: TONE.trousersFar }, dome(dense(THIGH_L, 3), 1, 0.04)));
  leg.push(loft({ id: "shank-l", bone: "shank-l", material: "wool-trouser", tone: TONE.trousersFar }, dome(dense(SHANK_L, 3), -1, 0.04)));
  leg.push(loft({ id: "sock-l", bone: "shank-l", material: "cotton-sock", tone: 0.22 }, [H("shank-l", 0.16, -0.047, 0.04, 0.037), H("shank-l", 0.12, -0.048, 0.043, 0.0375), H("shank-l", 0.085, -0.05, 0.048, 0.039)].reverse()));
  leg.push(loft({ id: "shoe-rear-l", bone: "foot-l", material: "leather-shoe", tone: TONE.shoeFar }, dense(SHOE_REAR.map(([x, b, t, rw, e]) => X("foot-l", x, b, t, rw, e)), 3)));
  leg.push(loft({ id: "shoe-toe-l", bone: "toe-l", material: "leather-shoe", tone: TONE.shoeFar }, dense(SHOE_TOE.map(([x, b, t, rw, e, z]) => X("toe-l", x, b, t, rw, e, z)), 3)));
  leg.push(extrude({ id: "shoe-heel-l", bone: "foot-l", material: "leather-shoe", tone: TONE.sole }, "zx", HEEL_PLAN, -ANKLE_Y, -ANKLE_Y + 0.025, LEVEL));
  leg.push(extrude({ id: "shoe-sole-l", bone: "foot-l", material: "leather-shoe", tone: TONE.sole }, "zx", SOLE_PLAN, -ANKLE_Y, -ANKLE_Y + 0.011, LEVEL));
  leg.push(extrude({ id: "shoe-toe-sole-l", bone: "toe-l", material: "leather-shoe", tone: TONE.sole }, "zx", TOE_PLAN, 0, 0.011));
  both(leg);

  const arm: Json[] = [];
  arm.push(loft(coat("sleeve-l", "arm-l", TONE.coatFar), dome(dome(dense(SLEEVE_L, 3), 1, 0.044), -1, 0.01, 2)));
  arm.push(loft(coat("forearm-l", "forearm-l", TONE.coatFar), dome(dense(CUFF_L, 3), -1, 0.044)));
  arm.push(loft({ id: "shirt-cuff-l", bone: "forearm-l", material: "cotton-shirt", tone: TONE.shirt }, [st(0.22, 0, 0.028, 0.0285, 0.0355), st(0.262, 0, WRIST_W, 0.027, 0.0345)]));
  arm.push(loft({ id: "wrist-l", bone: "forearm-l", material: "skin", tone: TONE.handFar }, [st(0.228, 0, 0.029, 0.0225, 0.03), st(0.258, 0, WRIST_W, 0.021, 0.0295), st(0.278, 0, WRIST_W + 0.002, 0.0205, 0.029)]));
  both(arm);
  for (const s of out) if (s.id === "wrist-r") s.tone = TONE.hand;
  out.push(...leftHand(TONE.handFar));
  out.push(...rightHand(TONE.hand));

  const skin = (id: string, bone = "head", tone: number = TONE.skin, role = "body"): { id: string; bone: string; material: string; tone: number; role: string } => ({ id, bone, material: "skin", tone, role });
  const Hd = (x: number, y: number, z = 0): V3 => L("head", [x, y, z]);
  const level = aboutW(90 - boneDeg("head"));
  out.push(loft(skin("neck", "neck", 0.76), NECK.map(([y, b, f, rw, e]) => H("neck", y, b, f, rw, e))));
  out.push(loft({ id: "scarf", bone: "neck", material: "wool-scarf", tone: TONE.scarf }, dense(SCARF.map(([y, b, f, rw, e]) => H("neck", y, b, f, rw, e)), 3)));
  out.push(loft(coat("collar", "neck", 0.26), [st(-0.048, 0.004, 0, 0.112, 0.1, 2.1, -20), st(-0.022, -0.004, 0, 0.11, 0.1, 2.1, -20), st(0.004, -0.012, 0, 0.103, 0.095, 2.1, -20)]));
  out.push(loft(skin("head"), dome(dome(dense(HEAD.map(([y, b, f, rw, e]) => H("head", y, b, f, rw, e)), 3), -1, 0.005, 2), 1, 0.003, 2)));
  out.push(loft({ id: "hair", bone: "head", material: "hair", tone: TONE.hair }, dome(dome(dense(HAIR.map(([y, b, f, rw, e]) => H("head", y, b, f, rw, e)), 3), -1, 0.008, 2), 1, 0.003, 2)));
  const nose: [number, number, number][] = [[1.706, 0.1185, 0.0075], [1.69, 0.1265, 0.0095], [1.675, 0.1345, 0.0125], [1.664, 0.1395, 0.0165], [1.656, 0.131, 0.0165]];
  out.push(loft(skin("head-nose"), dense(nose.map(([y, f, rw]) => H("head", y, 0.1, f, rw, 2)), 2).reverse()));
  out.push(ellipsoid(skin("head-brow"), [0.0075, 0.008, 0.043], Hd(0.1085, 1.714), level));
  out.push(ellipsoid(skin("head-lips", "head", 0.66), [0.0068, 0.0075, 0.023], Hd(0.1065, 1.6255), level));
  out.push(ellipsoid(skin("head-chin"), [0.015, 0.014, 0.022], Hd(0.092, 1.583), level));
  for (const side of [1, -1]) {
    const n = side > 0 ? "l" : "r";
    out.push(ellipsoid(skin(`head-ear-${n}`, "head", 0.74), [0.031, 0.0155, 0.0062], Hd(0.008, 1.684, side * 0.0745), compose(level, aboutW(-10), aboutU(side * 16))));
    out.push(ellipsoid(skin(`head-eye-${n}`, "head", 0.15, "detail"), [0.0052, 0.0052, 0.0115], Hd(0.1045, 1.694, side * 0.032), level));
  }

  for (const side of [1, -1]) {
    const n = side > 0 ? "l" : "r";
    const py = 0.985;
    const pz = 0.116;
    out.push(box(coat(`coat-pocket-${n}`, "torso", 0.24, "detail"), [0.05, 0.009, 0.135], L("torso", [coatX(py, pz, 1) + 0.001, py, side * pz]), compose(aboutW(90 - boneDeg("torso")), aboutU(-side * coatYaw(py, pz))), 0.003));
  }
  for (const side of [1, -1]) {
    const rows = LAPEL.map(([y, zin, zout]) => H("torso", y, coatX(y, zout, 1) - 0.01, coatX(y, zin, 1) + 0.0055, (zout - zin) / 2, 5, (side * (zin + zout)) / 2));
    out.push(loft(coat(`lapel-${side > 0 ? "l" : "r"}`, "torso", 0.26, "detail"), rows));
  }
  const vee: [number, number][] = [[1.372, 0.012], [1.4, 0.028], [1.44, 0.05], [1.48, 0.064], [1.515, 0.072], [1.54, 0.074]];
  out.push(loft({ id: "scarf-front", bone: "torso", material: "wool-scarf", tone: TONE.scarf, role: "detail" }, vee.map(([y, rw]) => H("torso", y, coatX(y, 0, 1) - 0.027, coatX(y, 0, 1) + 0.003, rw, 2))));
  [1.325, 1.19, 1.055].forEach((y, i) => out.push(ellipsoid(coat(`coat-button-${i + 1}`, "torso", 0.1, "detail"), [0.011, 0.0045, 0.011], L("torso", [coatX(y, 0.014, 1) + 0.002, y, 0.014]), aboutW(90 - boneDeg("torso")))));

  const leather = (id: string, tone: number = TONE.bag, role = "body"): { id: string; bone: string; material: string; tone: number; role: string } => ({ id, bone: "bag", material: "leather-bag", tone, role });
  const upright = aboutW(-90 - boneDeg("bag"));
  out.push(box(leather("bag-body"), [BAG.height, BAG.length, BAG.depth], B(BAG.x, BAG.y, BAG.z), upright, 0.02));
  const top = BAG.y + BAG.height / 2;
  const outer = BAG.z - BAG.depth / 2;
  out.push(box(leather("bag-flap-top", 0.4), [0.009, BAG.length - 0.006, BAG.depth + 0.008], B(BAG.x, top + 0.002, BAG.z - 0.004), upright, 0.004));
  const c = B(BAG.x, top - 0.07, 0);
  const hl = BAG.length / 2 - 0.003;
  const flap: [number, number][] = [[-0.078, -hl], [0.05, -hl], [0.066, -hl + 0.006], [0.078, -hl + 0.024], [0.08, -hl + 0.05], [0.08, hl - 0.05], [0.078, hl - 0.024], [0.066, hl - 0.006], [0.05, hl], [-0.078, hl]];
  out.push(extrude(leather("bag-flap", 0.4), "xy", flap.map(([u, v]) => [c[0] + u, c[1] + v] as [number, number]), L("bag", [0, 0, outer - 0.0085])[2], L("bag", [0, 0, outer + 0.003])[2]));
  out.push(box(leather("bag-clasp", 0.2, "detail"), [0.034, 0.022, 0.009], B(BAG.x, top - 0.135, outer - 0.0105), upright, 0.003));
  for (const [n, x] of [["front", 0.13], ["back", -0.15]] as const) out.push(box(leather(`bag-lug-${n}`, 0.3), [0.03, 0.014, 0.034], B(x, top + 0.004, BAG.z), upright, 0.004));
  out.push(loft({ ...leather("bag-strap", TONE.strap), }, dense(ribbon("bag", STRAP, 0.018, 0.003), 2), [true, true], { slender: true }));
  return out;
}

/* ---------------------------------------------------------------- thermal */

const LAYERS = {
  coat: [["jacket", 0.006], ["shirt", 0.009], ["tee", 0.0102]],
  trousers: [["briefs", 0.0025]],
  shoes: [["socks", 0.004]],
} as const;
export const OUTER = { coat: "overcoat", trousers: "trousers", shoes: "shoe", scarf: "scarf", hair: "hair" } as const;
type Layered = keyof typeof LAYERS;
/** 🧅 The shells of a garment loft under its outer layer: the same sections moved inwards by the thickness of the layers outside. */
function shells(solid: Json, region: Layered): Json[] {
  return LAYERS[region].map(([layer, t], i) => ({
    ...solid,
    id: `th-${solid.id}-${layer}`,
    tone: Math.max(0.05, Math.round((solid.tone + 0.12 * (i + 1)) * 100) / 100),
    sections: solid.sections.map((q: Json) => ({ ...q, rx: Math.max(0.003, Math.round((q.rx - t) * 1e5) / 1e5), ry: Math.max(0.003, Math.round((q.ry - t) * 1e5) / 1e5) })),
    region,
    layer,
  }));
}
/** 👖 A trouser leg as the open tube it is: the sections of the body loft between the crotch and the knee (thigh) or the knee and the hem (shank), without the rounded joint ends, and at 1:50 the briefs shell inside it. */
export function legTubes(solid: Json, withShell: boolean): Json[] {
  const thigh = solid.bone.startsWith("thigh");
  const below = (from: number): Json[] => solid.sections.filter((q: Json) => (thigh ? q.at[0] >= from && q.at[0] <= 0.4351 : q.at[0] >= 0));
  const sections = below(0.1).length >= 2 ? below(0.1) : below(0);
  const tube: Json = { ...solid, id: `th-${solid.id}`, sections, capStart: false, capEnd: false, region: "trousers", ...(withShell ? { layer: "trousers" } : {}) };
  if (!withShell) return [tube];
  return [tube, { ...tube, id: `th-${solid.id}-briefs`, tone: Math.round((solid.tone + 0.12) * 100) / 100, sections: sections.map((q: Json) => ({ ...q, rx: Math.round((q.rx - 0.0025) * 1e5) / 1e5, ry: Math.round((q.ry - 0.0025) * 1e5) / 1e5 })), layer: "briefs" }];
}
/** 🙂 The rows of the head that carry bare skin: chin to the hairline of the forehead. */
export const FACE_TOP = 1.748;
export const breath = (scale: number): Json[] => {
  const k = scale === 50 ? 1 : scale === 100 ? 1.9 : scale === 200 ? 3.8 : 9.4;
  const skin = { bone: "head", material: "skin", tone: TONE.skin };
  const level = aboutW(90 - boneDeg("head"));
  return [
    { ...ellipsoid({ id: "th-nose-breath", ...skin }, [0.004 * k, 0.0048 * k, Math.max(0.0095, 0.004 * k)], L("head", [0.127 - 0.002 * k, 1.6545, 0]), level), region: "face-skin", tags: ["emitter", "nose-breath"] },
    { ...ellipsoid({ id: "th-mouth-breath", ...skin }, [0.004 * k, 0.004 * k, Math.max(0.0062, 0.004 * k)], L("head", [0.111 - 0.002 * k, 1.6255, 0]), level), region: "face-skin", tags: ["emitter", "mouth-breath"] },
  ];
};
export const plume = (thick: number): Json => ({ ...ellipsoid({ id: "th-body-plume", bone: "head", material: "hair", tone: 0.5 }, [thick, 0.062, 0.05], L("head", [0.012, 1.8005 - thick * 0.4, 0]), aboutW(90 - boneDeg("head"))), region: "coat", tags: ["emitter", "body-plume"] });

export function thermal50(body: readonly Json[]): Json[] {
  const out: Json[] = [];
  const by = (id: string): Json => { const s = body.find((b) => b.id === id); if (!s) throw new Error(`no body solid ${id}`); return s; };
  const layered = (id: string, region: Layered): void => { out.push(thOf(id, region, OUTER[region])); out.push(...shells(by(id), region)); };
  const ref = (ids: readonly string[], region: string, layer?: string, extra: Json = {}): void => { for (const id of ids) { by(id); out.push(thOf(id, region, layer, extra)); } };
  const sides = (ids: readonly string[]): string[] => ids.flatMap((id) => [id, toRight(id)]);

  for (const id of ["coat", "collar", ...sides(["sleeve-l", "forearm-l"])]) layered(id, "coat");
  ref(["lapel-l", "lapel-r", "coat-pocket-l", "coat-pocket-r", "coat-button-1", "coat-button-2", "coat-button-3"], "coat", "overcoat");
  ref(sides(["shirt-cuff-l"]), "coat", "shirt");
  for (const id of sides(["thigh-l", "shank-l"])) out.push(...legTubes(by(id), true));
  for (const id of sides(["shoe-rear-l", "shoe-toe-l"])) layered(id, "shoes");
  ref(sides(["shoe-heel-l", "shoe-sole-l", "shoe-toe-sole-l"]), "shoes", "shoe");
  const scarf = by("scarf");
  out.push({ ...scarf, id: "th-scarf", sections: scarf.sections.filter((q: Json, i: number) => i >= 6), capStart: false, capEnd: false, region: "scarf", layer: "scarf" });
  ref(["scarf-front"], "scarf", "scarf", { sectionFaces: ["+v"] });
  ref(["hair"], "hair", "hair");

  const head = by("head");
  const face = head.sections.filter((q: Json) => L0(q) <= FACE_TOP);
  out.push({ ...head, id: "th-head", sections: face, capStart: true, capEnd: false, region: "face-skin" });
  ref(["head-nose", "head-brow", "head-lips", "head-chin", ...["ear", "eye"].flatMap((k) => [`head-${k}-l`, `head-${k}-r`])], "face-skin");
  out.push({ ...loft({ id: "th-neck", bone: "neck", material: "skin", tone: 0.76 }, [H("neck", 1.612, -0.05, 0.064, 0.056), H("neck", 1.66, -0.044, 0.066, 0.055)], [false, false]), region: "face-skin" });
  ref(body.filter((b) => b.material === "skin" && /^(hand|wrist)/.test(b.id)).map((b) => b.id), "hand-skin");
  ref(body.filter((b) => b.bone === "bag").map((b) => b.id), "bag-leather");
  out.push(...breath(50), plume(0.004));
  return out;
}
/** 📐 The height of a head section in the standing figure. */
function L0(q: Json): number {
  const f = FRAMES.head!;
  return f.o[1] + q.at[0] * f.u[1] + q.at[1] * f.v[1];
}

/* ---------------------------------------------------------------- structure */

export const BONE = { material: "bone", tone: 0.62 };
/** 📐 The fit of the shared adult skeleton table to the commuter: trunk 0.97 of the reference man, the thorax 8 mm further forward under the coat, shoulder joints 0.42 m up the torso bone. */
export const FIT: Fit = { ...BONE, ks: 0.9706, kv: 1, kw: 1, dv: -0.008, kp: 1, kpv: 1, drop: 1, shoulder: 0.42, torso: "torso", pelvis: "pelvis" };
export const kgOf = (bone: string): Json => (ASSET.rig.bones as Json[]).find((b) => b.id === bone).mass;
export const T = (x: number, y: number, z = 0): V3 => L("torso", [x, y, z]);
export const P = (x: number, y: number, z = 0): V3 => L("pelvis", [x, y, z]);
export const N = (x: number, y: number, z = 0): V3 => L("neck", [x, y, z]);
export const Hd = (x: number, y: number, z = 0): V3 => L("head", [x, y, z]);
export const ring = (rd: number, r: number): [number, number][] => Array.from({ length: 9 }, (_, k) => [r * Math.cos((k * Math.PI) / 4), rd + r * Math.sin((k * Math.PI) / 4)] as [number, number]);
/** 🦴 The spine from the sacral base to the first thoracic vertebra: lumbar lordosis, thoracic kyphosis; [x, y, radius]. */
export const SPINE: readonly [number, number, number][] = [[-0.036, 1.064, 0.02], [-0.014, 1.115, 0.0195], [-0.006, 1.17, 0.019], [-0.018, 1.24, 0.0175], [-0.044, 1.32, 0.016], [-0.058, 1.4, 0.015], [-0.052, 1.47, 0.014], [-0.034, 1.528, 0.013], [-0.022, 1.556, 0.0125]];
export const CERVICAL: readonly [number, number, number][] = [[-0.022, 1.556, 0.0125], [-0.008, 1.59, 0.012], [0.004, 1.624, 0.0115], [0.013, 1.652, 0.011]];
/** 🫁 The rib hoops from the tenth to the second rib: [y, x of the centre, depth radius, half-width]. */
export const RIBS: readonly [number, number, number, number][] = [[1.2, 0.012, 0.088, 0.122], [1.235, 0.012, 0.094, 0.131], [1.27, 0.011, 0.098, 0.136], [1.305, 0.01, 0.099, 0.138], [1.34, 0.008, 0.097, 0.137], [1.375, 0.004, 0.092, 0.131], [1.41, 0.0, 0.083, 0.12], [1.44, -0.006, 0.071, 0.102], [1.468, -0.012, 0.056, 0.078]];
export const massBall = (member: string, scale = 1): Json => {
  const k = kgOf(member);
  const r = 0.0125 * Math.cbrt(k.kg) * scale;
  return part(ellipsoid({ id: `mass-${member}`, bone: member, material: "bone", tone: 0.2, role: "detail" }, [r, r, r], [k.comX, k.comY, 0]), member, "mass");
};
export const jointBall = (member: string, at: V3, r: number): Json => part(ellipsoid({ id: `joint-${member}`, bone: member, material: "bone", tone: 0.4 }, [r, r, r], at), member, "joint");

export function structure50(body: readonly Json[]): Json[] {
  const out: Json[] = [];
  const m = (solid: Json, member: string): void => { out.push(part(solid, member, "member")); };
  for (const s of axial(FIT, 50)) m(s, s.bone);
  const level = aboutW(90 - boneDeg("head"));
  m(ellipsoid({ id: "cranium", bone: "head", ...BONE }, [0.066, 0.094, 0.071], Hd(0.014, 1.727), level), "head");
  m(ellipsoid({ id: "face-bones", bone: "head", ...BONE }, [0.044, 0.034, 0.054], Hd(0.074, 1.662), level), "head");
  for (const side of [1, -1]) {
    const n = side > 0 ? "l" : "r";
    m(capsule({ id: `mandible-ramus-${n}`, bone: "head", ...BONE }, Hd(0.026, 1.664, side * 0.052), Hd(0.02, 1.606, side * 0.048), 0.0075, 0.008), "head");
    m(capsule({ id: `mandible-body-${n}`, bone: "head", ...BONE }, Hd(0.02, 1.606, side * 0.048), Hd(0.09, 1.582, side * 0.01), 0.008, 0.009), "head");
  }
  m(tube({ id: "cervical-spine", bone: "neck", ...BONE }, CERVICAL.map(([x, y]) => N(x, y)), CERVICAL.map((q) => q[2])), "neck");

  const limb: Json[] = [];
  const lm = (solid: Json, member: string): void => { limb.push(part(solid, member, "member")); };
  lm(capsule({ id: "femur-neck-l", bone: "thigh-l", ...BONE }, [0, 0, 0], [0.036, -0.006, 0.058], 0.0165, 0.0175), "thigh-l");
  lm(ellipsoid({ id: "femur-trochanter-l", bone: "thigh-l", ...BONE }, [0.024, 0.018, 0.015], [0.032, -0.008, 0.066]), "thigh-l");
  lm(tube({ id: "femur-l", bone: "thigh-l", ...BONE }, [[0.048, -0.005, 0.062], [0.13, 0.006, 0.048], [0.23, 0.012, 0.032], [0.33, 0.008, 0.016], [0.416, 0.0, 0.004]], [0.0155, 0.0142, 0.0135, 0.0142, 0.017]), "thigh-l");
  lm(ellipsoid({ id: "femur-condyles-l", bone: "thigh-l", ...BONE }, [0.022, 0.031, 0.041], [0.428, -0.006, 0]), "thigh-l");
  lm(ellipsoid({ id: "patella-l", bone: "thigh-l", ...BONE }, [0.022, 0.0095, 0.021], [0.426, 0.042, 0]), "thigh-l");
  lm(ellipsoid({ id: "tibia-plateau-l", bone: "shank-l", ...BONE }, [0.015, 0.027, 0.039], [0.021, -0.002, 0]), "shank-l");
  lm(tube({ id: "tibia-l", bone: "shank-l", ...BONE }, [[0.032, 0.004, -0.003], [0.13, 0.012, -0.003], [0.27, 0.01, -0.003], [0.404, 0.002, -0.002]], [0.017, 0.0135, 0.012, 0.0135]), "shank-l");
  lm(ellipsoid({ id: "malleolus-l", bone: "shank-l", ...BONE }, [0.014, 0.012, 0.0095], [0.418, 0.0, -0.027]), "shank-l");
  lm(capsule({ id: "fibula-l", bone: "shank-l", ...BONE }, [0.042, -0.018, 0.034], [0.428, -0.012, 0.03], 0.0068, 0.0072), "shank-l");
  const fz = FRAMES["foot-l"]!.o[2];
  lm(capsule({ id: "calcaneus-l", bone: "foot-l", ...BONE }, L("foot-l", [-0.042, 0.05, fz]), L("foot-l", [0.006, 0.064, fz]), 0.021, 0.019), "foot-l");
  lm(loft({ id: "midfoot-l", bone: "foot-l", ...BONE }, [X("foot-l", -0.004, 0.052, 0.094, 0.024), X("foot-l", 0.06, 0.04, 0.078, 0.033), X("foot-l", 0.11, 0.027, 0.055, 0.039), X("foot-l", 0.166, 0.016, 0.036, 0.042)]), "foot-l");
  lm(loft({ id: "phalanges-l", bone: "toe-l", ...BONE }, [X("toe-l", 0.156, 0.016, 0.036, 0.042), X("toe-l", 0.198, 0.015, 0.031, 0.038, 2, -0.003), X("toe-l", 0.224, 0.015, 0.027, 0.026, 2, -0.006)]), "toe-l");
  lm(ellipsoid({ id: "humerus-head-l", bone: "arm-l", ...BONE }, [0.023, 0.023, 0.023], [0.002, 0, 0]), "arm-l");
  lm(tube({ id: "humerus-l", bone: "arm-l", ...BONE }, [[0.014, 0, 0.004], [0.11, 0.004, 0.008], [0.22, 0.003, 0.01], [0.316, 0, 0.012]], [0.012, 0.0108, 0.0104, 0.0125]), "arm-l");
  lm(ellipsoid({ id: "humerus-condyles-l", bone: "arm-l", ...BONE }, [0.012, 0.014, 0.031], [0.325, 0, 0.012]), "arm-l");
  lm(capsule({ id: "ulna-l", bone: "forearm-l", ...BONE }, [-0.014, -0.017, 0.012], [0.252, -0.014, WRIST_W], 0.0082, 0.0052), "forearm-l");
  lm(capsule({ id: "radius-l", bone: "forearm-l", ...BONE }, [0.01, 0.015, 0.013], [0.256, 0.011, WRIST_W], 0.006, 0.0088), "forearm-l");
  for (const s of limb) out.push(s);
  for (const s of limb) out.push(mirrored(s, toRight));

  for (const hand of ["hand-l", "hand-r"]) {
    const rays = body.filter((b) => b.bone === hand && b.kind === "capsule");
    const carpus: V3 = hand === "hand-l" ? [0.014, 0, WRIST_W + 0.002] : R(0.006, 0.02, 0.014);
    m(ellipsoid({ id: `carpals-${hand.slice(-1)}`, bone: hand, ...BONE }, [0.014, 0.014, 0.014], carpus), hand);
    for (const ray of rays) {
      m(capsule({ id: ray.id.replace("hand-", "bones-"), bone: hand, ...BONE }, ray.p0, ray.p1, 0.0044, 0.004), hand);
      if (/finger-\d$|thumb$/.test(ray.id)) m(capsule({ id: `${ray.id.replace("hand-", "bones-")}-metacarpal`, bone: hand, ...BONE }, [0, 1, 2].map((k) => carpus[k]! + (ray.p0[k] - carpus[k]!) * 0.25) as V3, ray.p0, 0.0048, 0.0046), hand);
    }
  }
  out.push({ id: "member-bag-strap", of: "bag-strap", member: "bag", part: "member" });
  for (const n of ["front", "back"]) out.push({ id: `member-bag-lug-${n}`, of: `bag-lug-${n}`, member: "bag", part: "member" });
  out.push(part(box({ id: "mass-bag", bone: "bag", material: "leather-bag", tone: 0.2, role: "detail" }, [0.07, 0.11, 0.05], B(BAG.x, BAG.y, BAG.z), aboutW(-90 - boneDeg("bag")), 0.01), "bag", "mass"));

  out.push(jointBall("torso", T(-0.036, 1.064), 0.017));
  out.push(jointBall("neck", N(-0.022, 1.556), 0.0135));
  out.push(jointBall("head", Hd(0.013, 1.652), 0.0125));
  for (const side of ["l", "r"]) {
    out.push(jointBall(`thigh-${side}`, [0, 0, 0], 0.0245));
    out.push(jointBall(`shank-${side}`, [0, 0, 0], 0.019));
    out.push(jointBall(`foot-${side}`, [0, 0.002, 0], 0.016));
    out.push(jointBall(`toe-${side}`, [-0.004, 0.024, 0], 0.01));
    out.push(jointBall(`arm-${side}`, [0.002, 0, 0], 0.0245));
    out.push(jointBall(`forearm-${side}`, [0, 0, side === "l" ? 0.012 : -0.012], 0.0155));
    out.push(jointBall(`hand-${side}`, [0, 0, side === "l" ? WRIST_W : -WRIST_W], 0.012));
  }
  out.push(jointBall("bag", B(0, 1.541, -0.14), 0.01));
  for (const b of ["pelvis", "torso", "head", "thigh-l", "thigh-r", "shank-l", "shank-r", "arm-l", "arm-r", "forearm-l", "forearm-r", "foot-l", "foot-r"]) out.push(massBall(b));
  return out;
}

export const DECISIONS_50: Json[] = [{ en: "1:50 body: draft.", de: "Körper 1:50: Entwurf." }];
export { WRIST_W, LEVEL };
