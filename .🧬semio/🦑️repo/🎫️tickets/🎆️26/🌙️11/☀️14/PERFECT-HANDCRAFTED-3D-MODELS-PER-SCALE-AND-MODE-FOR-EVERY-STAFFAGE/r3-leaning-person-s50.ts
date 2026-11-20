/** 🧍 The 1:50 model of `humans/leaning-person`, stated section by section: a slim man of 1.72 m and 66 kg (1.74 m in his sneakers) in a regular-fit cotton fleece hoodie with the hood folded down on the upper back, straight jeans, a knitted beanie with a turned-up cuff and white low-top sneakers, holding a phone in the right hand.
 *
 * Every station is anatomical (`sa`, `pa`): `a` along the bone from its joint, `f` ahead of the bone line, `l` to the left. Trunk heights `h` are measured up the trunk line from the hip joints (pelvis bone 0.0977 m, then the torso bone to C7 at 0.5732 m).
 * @see ./r3-leaning-person-kit.ts
 * @see ./r3-humans-leaning-person.md */
import type { Json } from "./volumes-lib.ts";
import { aboutU, aboutV, aboutW, box, capsule, compose, dense, dome, ellipsoid, extrude, inset, loft, mirrored, pa, part, sa, thOf, toRight, tube, type St, type V3 } from "./r3-leaning-person-kit.ts";
import { PHONE } from "./r3-leaning-person-data.ts";

export const TONE = { hoodie: 0.34, hood: 0.38, rib: 0.42, pocket: 0.3, string: 0.5, jeans: 0.26, seam: 0.2, shoe: 0.9, sole: 0.96, sock: 0.25, toeCap: 0.86, lace: 0.8, skin: 0.8, hand: 0.78, lips: 0.62, eye: 0.22, brow: 0.3, hair: 0.2, beanie: 0.22, cuff: 0.18, phone: 0.12, screen: 0.4, wall: 0.6, bone: 0.92, joint: 0.6, mass: 0.5 } as const;
export const PELVIS_LEN = 0.0977;
const C = (id: string, bone: string, material: string, tone: number, role = "body") => ({ id, bone, material, tone, role });
type Row = readonly [number, number, number, number, number];

/* ================================================================ trunk: hoodie over the chest, waist and seat */

/** 👕 The hoodie on the trunk as slices square to the trunk line: [h, back, front, half-width, exponent]. The hem rib gathers over the seat, the body hangs straight over the waist, the chest fills it, the shoulders slope from the acromion to the neckline. */
export const HOODIE: readonly Row[] = [
  [-0.05, -0.127, 0.117, 0.174, 2.5],
  [-0.02, -0.13, 0.122, 0.178, 2.5],
  [0.02, -0.128, 0.124, 0.177, 2.5],
  [0.1, -0.121, 0.126, 0.171, 2.5],
  [0.17, -0.114, 0.13, 0.167, 2.5],
  [0.25, -0.113, 0.138, 0.168, 2.5],
  [0.33, -0.115, 0.146, 0.172, 2.45],
  [0.4, -0.116, 0.142, 0.18, 2.4],
  [0.455, -0.113, 0.129, 0.19, 2.5],
  [0.5, -0.104, 0.112, 0.194, 2.7],
  [0.532, -0.09, 0.096, 0.184, 2.8],
  [0.556, -0.078, 0.081, 0.152, 2.6],
  [0.578, -0.067, 0.069, 0.108, 2.3],
  [0.594, -0.059, 0.063, 0.078, 2.1],
];
const torsoSt = ([h, b, f, hw, e]: Row, n = 0): St => sa("torso", h - PELVIS_LEN, (b + f) / 2, 0, hw, (f - b) / 2, e, n);
/** 📏 The surface of a trunk table at the height `h` and the lateral position `l`: f of the front (+1) or of the back (-1). */
export function trunkF(table: readonly Row[], h: number, l: number, side: 1 | -1): number {
  let i = 0;
  while (i + 2 < table.length && table[i + 1]![0] < h) i++;
  const a = table[i]!;
  const b = table[i + 1]!;
  const t = Math.min(1, Math.max(0, (h - a[0]) / (b[0] - a[0])));
  const m = (k: number): number => a[k]! + (b[k]! - a[k]!) * t;
  const [back, front, rw, e] = [m(1), m(2), m(3), m(4)];
  const k = Math.pow(Math.max(0, 1 - Math.pow(Math.min(1, Math.abs(l) / rw), e)), 1 / e);
  return (back + front) / 2 + (side * k * (front - back)) / 2;
}
/** 🧷 A point on the hoodie front (+1) or back (-1) at the height `h`, `l` to the left, `off` proud of the cloth, torso-local. */
export const onHoodie = (h: number, l: number, side: 1 | -1, off = 0): V3 => pa("torso", h - PELVIS_LEN, trunkF(HOODIE, h, l, side) + side * off, l);

/** 👖 The seat of the jeans on the pelvis bone, below and inside the hoodie hem: [a, back, front, half-width, exponent]. */
export const SEAT: readonly Row[] = [
  [-0.098, -0.075, 0.058, 0.118, 2.2],
  [-0.075, -0.118, 0.085, 0.165, 2.3],
  [-0.03, -0.127, 0.098, 0.174, 2.4],
  [0.03, -0.117, 0.104, 0.169, 2.4],
  [0.1, -0.1, 0.106, 0.158, 2.4],
];
const seatSt = ([a, b, f, hw, e]: Row): St => sa("pelvis", a, (b + f) / 2, 0, hw, (f - b) / 2, e);

/* ================================================================ legs */

/** 🦵 The left jeans leg on the thigh: the trochanter widens it at the top, quadriceps ahead, the inner seams meet just below the crotch. */
export const THIGH_L: St[] = [
  sa("thigh-l", -0.05, -0.006, -0.002, 0.086, 0.098, 2.2),
  sa("thigh-l", 0.04, -0.002, -0.006, 0.088, 0.094, 2.2),
  sa("thigh-l", 0.13, 0.004, -0.008, 0.083, 0.089, 2.1),
  sa("thigh-l", 0.23, 0.008, -0.007, 0.076, 0.082, 2.1),
  sa("thigh-l", 0.32, 0.006, -0.005, 0.067, 0.073, 2),
  sa("thigh-l", 0.385, 0.004, -0.004, 0.06, 0.065, 2),
  sa("thigh-l", 0.425, 0.003, -0.002, 0.0565, 0.0605, 2),
];
/** 🦿 The left jeans leg on the shank: kneecap, the calf pushing the denim back, the straight fall and the hem resting on the shoe, lower at the heel. */
export const SHANK_L: St[] = [
  sa("shank-l", -0.012, 0.004, -0.002, 0.0575, 0.0615, 2),
  sa("shank-l", 0.02, 0.003, 0, 0.0565, 0.0598, 2),
  sa("shank-l", 0.08, -0.006, 0, 0.056, 0.06, 2),
  sa("shank-l", 0.14, -0.011, 0, 0.058, 0.062, 2),
  sa("shank-l", 0.22, -0.009, 0, 0.056, 0.059, 2),
  sa("shank-l", 0.3, -0.006, 0, 0.054, 0.057, 2),
  sa("shank-l", 0.36, -0.006, 0, 0.055, 0.058, 2),
  sa("shank-l", 0.392, -0.01, 0, 0.056, 0.06, 2, -12),
];

/* ================================================================ sneaker (foot bone: a ahead of the ankle, f up, l left; ground at f = -0.088) */

const G = -0.088;
/** 👟 The left sneaker upper as cross slices [x ahead of the ankle, bottom, top, half-width, exponent]: heel counter, padded collar below the malleoli, tongue over the instep, vamp. */
export const UPPER: readonly Row[] = [
  [-0.071, G + 0.03, G + 0.07, 0.016, 2],
  [-0.064, G + 0.02, G + 0.077, 0.03, 2.4],
  [-0.045, G + 0.019, G + 0.078, 0.038, 2.6],
  [-0.015, G + 0.019, G + 0.071, 0.041, 2.6],
  [0.02, G + 0.018, G + 0.086, 0.044, 2.6],
  [0.055, G + 0.018, G + 0.099, 0.047, 2.6],
  [0.09, G + 0.018, G + 0.079, 0.05, 2.7],
  [0.125, G + 0.018, G + 0.062, 0.052, 2.8],
  [0.15, G + 0.018, G + 0.056, 0.052, 2.8],
];
const shoeSt = (_bone: string, [x, lo, hi, hw, e]: Row, l = 0): St => ({ at: [x, (lo + hi) / 2, l], rw: hw, rv: (hi - lo) / 2, e, n: 0 });
/** 🥿 The toe box on the toe bone (origin on the ground under the ball of the foot): [x, bottom, top, half-width, exponent], drifting inwards to the big toe. */
export const TOE_BOX: readonly Row[] = [
  [-0.012, 0.018, 0.058, 0.052, 2.8],
  [0.02, 0.018, 0.052, 0.051, 2.7],
  [0.045, 0.018, 0.046, 0.046, 2.5],
  [0.068, 0.019, 0.04, 0.036, 2.3],
  [0.08, 0.021, 0.033, 0.022, 2],
];
const TOE_DRIFT = [0, -0.002, -0.005, -0.008, -0.009];
/** 🛞 The sole outline in plan, foot-local [x, l] of the left shoe, heel to the ball; the toe part follows on the toe bone. */
const SOLE_REAR: [number, number][] = [[-0.074, 0.0], [-0.07, 0.022], [-0.058, 0.035], [-0.035, 0.041], [0.0, 0.042], [0.04, 0.044], [0.08, 0.049], [0.115, 0.053], [0.135, 0.054], [0.135, -0.054], [0.115, -0.054], [0.08, -0.047], [0.04, -0.038], [0.0, -0.036], [-0.035, -0.038], [-0.058, -0.033], [-0.07, -0.021]];
const SOLE_TOE: [number, number][] = [[-0.012, 0.054], [0.02, 0.053], [0.048, 0.045], [0.07, 0.03], [0.085, 0.012], [0.089, -0.008], [0.084, -0.025], [0.07, -0.04], [0.048, -0.05], [0.02, -0.055], [-0.012, -0.055]];

/* ================================================================ arms */

/** 🧥 The left hoodie sleeve on the upper arm: the dropped sleeve head over the deltoid, a slight swell over the biceps, easing to the elbow. */
export const SLEEVE_L: St[] = [
  sa("upper-arm-l", -0.03, -0.004, -0.012, 0.04, 0.047, 2.2),
  sa("upper-arm-l", -0.006, -0.002, 0.002, 0.054, 0.057, 2.2),
  sa("upper-arm-l", 0.045, 0.0, 0.005, 0.0555, 0.058, 2.1),
  sa("upper-arm-l", 0.12, 0.003, 0.003, 0.052, 0.055, 2),
  sa("upper-arm-l", 0.2, 0.004, 0.002, 0.049, 0.053, 2),
  sa("upper-arm-l", 0.27, 0.0, 0.002, 0.047, 0.05, 2),
  sa("upper-arm-l", 0.335, -0.004, 0.002, 0.045, 0.047, 2),
];
/** 💪 The left sleeve on the forearm: the cloth over the forearm muscles, bunching above the rib cuff. */
export const FORESLEEVE_L: St[] = [
  sa("forearm-l", -0.035, -0.006, 0.002, 0.045, 0.047, 2),
  sa("forearm-l", 0.02, -0.002, 0.002, 0.044, 0.046, 2),
  sa("forearm-l", 0.08, 0.0, 0.002, 0.042, 0.043, 2),
  sa("forearm-l", 0.15, 0.0, 0.001, 0.039, 0.039, 2),
  sa("forearm-l", 0.192, 0.0, 0.001, 0.039, 0.038, 2),
  sa("forearm-l", 0.206, 0.0, 0.001, 0.036, 0.034, 2),
];
/** 🧶 The rib cuff, tight above the wrist. */
export const CUFF_L: St[] = [sa("forearm-l", 0.2, 0, 0.001, 0.034, 0.03, 2), sa("forearm-l", 0.236, 0, 0.0, 0.032, 0.026, 2)];
/** ⌚ The bare wrist between cuff and hand (ulna and radius ends). */
export const WRIST_L: St[] = [sa("forearm-l", 0.228, 0, 0, 0.0275, 0.021, 2.2), sa("forearm-l", 0.262, 0, 0, 0.0285, 0.019, 2.3)];

/* ================================================================ hands */

const fingerPath = (from: V3, dirs: [number, number][], lens: number[], bend: "l" | "-l" | "f" | "-f"): V3[] => {
  const out: V3[] = [from];
  let p = from;
  for (let i = 0; i < lens.length; i++) {
    const [degA, len] = [dirs[i]![0], lens[i]!];
    const c = Math.cos((degA * Math.PI) / 180);
    const s = Math.sin((degA * Math.PI) / 180);
    const d: V3 = bend === "l" ? [c, 0, s] : bend === "-l" ? [c, 0, -s] : bend === "f" ? [c, s, 0] : [c, -s, 0];
    p = [p[0] + d[0] * len, p[1] + d[1] * len, p[2] + d[2] * len];
    out.push(p);
  }
  return out;
};
/** ✋ The relaxed hanging left hand: palm turned to the thigh (-l), thumb ahead, the fingers curling gently towards the palm. */
function leftHand(): Json[] {
  const c = (id: string): Json => C(id, "hand-l", "skin", TONE.hand);
  const out: Json[] = [];
  const palm: St[] = [sa("hand-l", -0.012, 0.0, 0.0, 0.0175, 0.028, 2.3), sa("hand-l", 0.025, 0.002, -0.001, 0.016, 0.037, 2.4), sa("hand-l", 0.065, 0.001, -0.002, 0.0145, 0.042, 2.5), sa("hand-l", 0.093, -0.001, -0.003, 0.0125, 0.041, 2.5)];
  out.push(loft(c("hand-l"), dome(dense(palm, 2), 1, 0.008, 2)));
  out.push(ellipsoid(c("hand-l-thenar"), [0.03, 0.017, 0.012], pa("hand-l", 0.038, 0.022, -0.008), aboutW(-12)));
  const fingers: [string, number, number, number][] = [["index", 0.094, 0.028, 0.95], ["middle", 0.098, 0.009, 1.03], ["ring", 0.095, -0.01, 0.97], ["little", 0.087, -0.027, 0.78]];
  for (const [name, a, f, k] of fingers) {
    const pts = fingerPath(pa("hand-l", a, f, -0.003), [[14, 0], [36, 0], [58, 0]], [0.044 * k, 0.027 * k, 0.021 * k], "-l");
    const r = name === "little" ? 0.9 : 1;
    out.push(capsule(c(`hand-l-${name}-1`), pts[0]!, pts[1]!, 0.0098 * r, 0.0091 * r));
    out.push(capsule(c(`hand-l-${name}-2`), pts[1]!, pts[2]!, 0.009 * r, 0.0083 * r));
    out.push(capsule(c(`hand-l-${name}-3`), pts[2]!, pts[3]!, 0.0082 * r, 0.0076 * r));
  }
  const t0 = pa("hand-l", 0.012, 0.026, -0.006);
  const t1: V3 = [t0[0] + 0.036, t0[1] + 0.021, t0[2] - 0.011];
  const t2: V3 = [t1[0] + 0.03, t1[1] + 0.008, t1[2] - 0.011];
  const t3: V3 = [t2[0] + 0.024, t2[1] + 0.001, t2[2] - 0.01];
  out.push(capsule(c("hand-l-thumb-1"), t0, t1, 0.0125, 0.0108));
  out.push(capsule(c("hand-l-thumb-2"), t1, t2, 0.0105, 0.0095));
  out.push(capsule(c("hand-l-thumb-3"), t2, t3, 0.0094, 0.0084));
  return out;
}
/** 📱 The right hand holding the phone: palm turned to +l (towards the body in the neutral hang), thumb ahead; the phone lies on palm and fingers with its long axis towards the thumb, the four fingers run under its back and curl round its far long edge onto the glass, the thumb rests on the near edge. */
function rightHand(): Json[] {
  const c = (id: string): Json => C(id, "hand-r", "skin", TONE.hand);
  const out: Json[] = [];
  const palm: St[] = [sa("hand-r", -0.012, 0.0, 0.0, 0.0175, 0.028, 2.3), sa("hand-r", 0.025, 0.002, 0.0, 0.016, 0.037, 2.4), sa("hand-r", 0.065, 0.001, 0.0, 0.0145, 0.042, 2.5), sa("hand-r", 0.093, -0.001, 0.0, 0.0125, 0.041, 2.5)];
  out.push(loft(c("hand-r"), dome(dense(palm, 2), 1, 0.008, 2)));
  out.push(ellipsoid(c("hand-r-thenar"), [0.03, 0.017, 0.012], pa("hand-r", 0.038, 0.022, 0.008), aboutW(-12)));
  const back = PHONE.w - PHONE.thick / 2;
  const fingers: [string, number, number, number][] = [["index", 0.094, 0.028, 0.95], ["middle", 0.098, 0.009, 1.03], ["ring", 0.095, -0.01, 0.97], ["little", 0.087, -0.027, 0.78]];
  for (const [name, a, f, k] of fingers) {
    const edge = PHONE.u + PHONE.width / 2;
    const p0 = pa("hand-r", a, f, 0.006);
    const p1: V3 = [edge - 0.004, p0[1], back - 0.0095];
    const p2: V3 = [edge + 0.012, p0[1], PHONE.w + 0.004];
    const p3: V3 = [edge - 0.006, p0[1], PHONE.w + PHONE.thick / 2 + 0.0075];
    const r = name === "little" ? 0.9 : 1;
    void k;
    out.push(capsule(c(`hand-r-${name}-1`), p0, p1, 0.0098 * r, 0.0091 * r));
    out.push(capsule(c(`hand-r-${name}-2`), p1, p2, 0.009 * r, 0.0083 * r));
    out.push(capsule(c(`hand-r-${name}-3`), p2, p3, 0.0082 * r, 0.0076 * r));
  }
  const t0 = pa("hand-r", 0.012, 0.026, 0.006);
  const t1: V3 = [t0[0] + 0.034, t0[1] + 0.02, t0[2] + 0.012];
  const t2: V3 = [PHONE.u - PHONE.width / 2 + 0.006, t1[1] + 0.012, PHONE.w + PHONE.thick / 2 + 0.006];
  const t3: V3 = [t2[0] + 0.022, t2[1] + 0.006, t2[2] + 0.001];
  out.push(capsule(c("hand-r-thumb-1"), t0, t1, 0.0125, 0.0108));
  out.push(capsule(c("hand-r-thumb-2"), t1, t2, 0.0105, 0.0095));
  out.push(capsule(c("hand-r-thumb-3"), t2, t3, 0.0094, 0.0084));
  out.push(box(C("phone", "hand-r", "phone-glass", TONE.phone), [PHONE.width, PHONE.length, PHONE.thick], [PHONE.u, PHONE.v, PHONE.w], undefined, 0.0035));
  out.push(box(C("phone-screen", "hand-r", "phone-glass", TONE.screen, "glazing"), [PHONE.width - 0.006, PHONE.length - 0.012, 0.0012], [PHONE.u, PHONE.v, PHONE.w + PHONE.thick / 2 + 0.0002], undefined, 0.0003, { texture: true }));
  return out;
}

/* ================================================================ head (head bone: a up from the neck base at C7 level, f ahead, l left) */

/** 🗣️ Skull and face as slices square to the head bone from the chin to the vertex: [a, back, front, half-width, exponent]. */
export const HEAD: readonly Row[] = [
  [0.02, 0.036, 0.086, 0.024, 2],
  [0.032, 0.0, 0.096, 0.042, 2.1],
  [0.046, -0.018, 0.098, 0.05, 2.2],
  [0.062, -0.03, 0.1, 0.055, 2.2],
  [0.08, -0.045, 0.102, 0.06, 2.2],
  [0.1, -0.068, 0.102, 0.066, 2.3],
  [0.12, -0.085, 0.099, 0.07, 2.3],
  [0.14, -0.093, 0.096, 0.072, 2.3],
  [0.16, -0.097, 0.099, 0.075, 2.3],
  [0.18, -0.098, 0.095, 0.076, 2.3],
  [0.2, -0.095, 0.087, 0.074, 2.2],
  [0.22, -0.085, 0.071, 0.066, 2.1],
  [0.235, -0.066, 0.05, 0.052, 2],
  [0.245, -0.038, 0.022, 0.03, 2],
];
const headSt = ([a, b, f, hw, e]: Row): St => sa("head", a, (b + f) / 2, 0, hw, (f - b) / 2, e);
/** 🧣 The neck from inside the collar to under the skull. */
export const NECK: St[] = [sa("head", -0.06, -0.006, 0, 0.066, 0.064, 2.1), sa("head", 0.0, 0.0, 0, 0.06, 0.06, 2), sa("head", 0.045, 0.006, 0, 0.056, 0.055, 2), sa("head", 0.085, 0.0, 0, 0.055, 0.054, 2), sa("head", 0.115, -0.012, 0, 0.052, 0.05, 2)];
/** 💇 Short hair where the beanie leaves it: nape, behind the ears and sideburns. */
export const HAIR: St[] = [sa("head", 0.1, -0.05, 0, 0.058, 0.03, 2), sa("head", 0.115, -0.03, 0, 0.068, 0.058, 2), sa("head", 0.135, -0.02, 0, 0.076, 0.079, 2.2), sa("head", 0.16, -0.017, 0, 0.08, 0.087, 2.2), sa("head", 0.185, -0.022, 0, 0.0775, 0.079, 2.2)];
/** 🧢 The beanie, tilted to sit lower at the back: cuff, knitted body and a soft crown that slouches back a little. */
export const BEANIE: St[] = [sa("head", 0.164, -0.004, 0, 0.083, 0.106, 2.2, -13), sa("head", 0.19, -0.005, 0, 0.0825, 0.105, 2.2, -11), sa("head", 0.214, -0.009, 0, 0.077, 0.097, 2.1, -7), sa("head", 0.236, -0.014, 0, 0.065, 0.082, 2, -3), sa("head", 0.25, -0.02, 0, 0.046, 0.058, 2, 0), sa("head", 0.258, -0.023, 0, 0.024, 0.03, 2, 0)];
/** 🧶 The turned-up cuff of the beanie, one knit thickness proud of it. */
export const BEANIE_CUFF: St[] = [sa("head", 0.165, -0.004, 0, 0.0855, 0.1085, 2.2, -13), sa("head", 0.19, -0.005, 0, 0.0852, 0.1078, 2.2, -11)];

/* ================================================================ the wall */

export const wall = (): Json => ({ ...box(C("wall", "wall", "masonry", TONE.wall, "ground"), [0.38, 1.9, 1.2], [-0.19, 0.95, 0]), sectionFaces: ["+v"], texture: true });

/* ================================================================ body */

export function body50(): Json[] {
  const out: Json[] = [];
  const hoodie = C("hoodie", "torso", "cotton-clothing", TONE.hoodie);
  out.push(loft(hoodie, dense(HOODIE.map((r) => torsoSt(r)), 2), [true, true]));
  out.push(loft(C("hoodie-hem", "torso", "cotton-clothing", TONE.rib, "detail"), [torsoSt([-0.094, -0.121, 0.111, 0.167, 2.4]), torsoSt([-0.07, -0.123, 0.113, 0.169, 2.4]), torsoSt([-0.04, -0.124, 0.114, 0.17, 2.4])]));
  const pocketAt = (l: number, a0: number, a1: number): St => { const h = (a0 + a1) / 2; const p = onHoodie(h, l, 1, 0.0035); return { at: p, rw: (a1 - a0) / 2, rv: 0.0045, e: 4, n: 0 }; };
  const pocket: St[] = [];
  for (const l of [-0.15, -0.13, -0.105, -0.07, 0, 0.07, 0.105, 0.13, 0.15]) {
    const top = Math.abs(l) <= 0.105 ? 0.135 : 0.135 - ((Math.abs(l) - 0.105) / 0.045) * 0.1;
    pocket.push(pocketAt(l, -0.018, top));
  }
  out.push(lateral(C("hoodie-pocket", "torso", "cotton-clothing", TONE.pocket, "detail"), pocket));
  const rollPath: V3[] = [[0.468, 0.068, 0.012], [0.476, 0.052, 0.058], [0.49, 0.02, 0.09], [0.5, -0.03, 0.095], [0.505, -0.072, 0.07], [0.508, -0.092, 0.03], [0.509, -0.096, 0]].map(([a, f, l]) => pa("torso", a, f, l));
  const roll = [...rollPath, ...rollPath.slice(0, -1).reverse().map((p) => [p[0], p[1], -p[2]] as V3)];
  out.push({ ...loft(C("hood-roll", "torso", "cotton-clothing", TONE.hood), []), sections: roll.map((at) => { const k = 1 - Math.abs(at[2]) / 0.1; return { at, rx: 0.012 + 0.004 * k, ry: 0.02 + 0.008 * k, exponent: 2.2 }; }) });
  const hood: St[] = ([[0.594, 0.07, 0.014], [0.572, 0.112, 0.013], [0.535, 0.13, 0.012], [0.49, 0.132, 0.011], [0.44, 0.112, 0.01], [0.405, 0.07, 0.0085], [0.385, 0.026, 0.007]] as const).map(([h, hw, hf]) => sa("torso", h - PELVIS_LEN, trunkF(HOODIE, h, 0, -1) - hf + 0.004, 0, hw, hf, 2.5));
  out.push(loft(C("hood", "torso", "cotton-clothing", TONE.hood), dense(hood, 2)));
  for (const side of [1, -1]) {
    const s = side > 0 ? "l" : "r";
    const top = onHoodie(0.565, side * 0.03, 1, 0.006);
    const mid = onHoodie(0.47, side * 0.042, 1, 0.005);
    const end = onHoodie(0.38, side * 0.046, 1, 0.005);
    out.push({ ...tube(C(`hood-string-${s}`, "torso", "cotton-clothing", TONE.string, "detail"), [top, mid, end], [0.0035, 0.0035, 0.0035]), slender: true });
    out.push(capsule(C(`hood-aglet-${s}`, "torso", "cotton-clothing", TONE.string, "detail"), end, onHoodie(0.355, side * 0.047, 1, 0.005), 0.0042, 0.0042));
  }
  out.push(loft(C("jeans-seat", "pelvis", "cotton-clothing", TONE.jeans), dense(SEAT.map(seatSt), 2)));
  out.push(loft(C("thigh-l", "thigh-l", "cotton-clothing", TONE.jeans), dome(dense(THIGH_L, 2), 1, 0.03, 3)));
  out.push(loft(C("shank-l", "shank-l", "cotton-clothing", TONE.jeans), dense(SHANK_L, 2)));
  out.push(loft(C("upper-arm-l", "upper-arm-l", "cotton-clothing", TONE.hoodie), dome(dome(dense(SLEEVE_L, 2), -1, 0.012, 2), 1, 0.03, 3)));
  out.push(loft(C("forearm-l", "forearm-l", "cotton-clothing", TONE.hoodie), dome(dense(FORESLEEVE_L, 2), -1, 0.022, 2)));
  out.push(loft(C("cuff-l", "forearm-l", "cotton-clothing", TONE.rib, "detail"), CUFF_L));
  out.push(loft(C("wrist-l", "forearm-l", "skin", TONE.hand), WRIST_L));
  out.push(loft(C("sock-l", "shank-l", "cotton-clothing", TONE.sock), [sa("shank-l", 0.33, -0.004, 0, 0.036, 0.041, 2), sa("shank-l", 0.385, -0.006, 0, 0.035, 0.042, 2), sa("shank-l", 0.42, -0.004, 0, 0.034, 0.043, 2.2)]));
  out.push(...shoe("l"));
  out.push(...leftHand());
  for (const s of out.filter((q) => /-l(-|$)/.test(q.id) && !/^(hood-|hand-l|hoodie)/.test(q.id))) {
    if (/^(thigh|shank|upper-arm|forearm|cuff|wrist|sock|shoe|toe|sole|lace)/.test(s.id)) out.push(mirrored(s, toRight));
  }
  out.push(...rightHand());
  out.push(loft(C("neck", "head", "skin", TONE.skin), NECK));
  out.push(loft(C("head", "head", "skin", TONE.skin), dome(dense(HEAD.map(headSt), 2), 1, 0.004, 2)));
  out.push(loft(C("nose", "head", "skin", TONE.skin), [sa("head", 0.146, 0.094, 0, 0.0075, 0.006, 2, 0), sa("head", 0.125, 0.104, 0, 0.0085, 0.0085, 2, 0), sa("head", 0.106, 0.112, 0, 0.0115, 0.0105, 2, 0), sa("head", 0.096, 0.108, 0, 0.0165, 0.009, 2, 0)]));
  out.push(ellipsoid(C("lips", "head", "skin", TONE.lips, "detail"), [0.0095, 0.0075, 0.024], pa("head", 0.068, 0.097, 0)));
  out.push(tube(C("brow", "head", "skin", TONE.brow, "detail"), [pa("head", 0.15, 0.08, 0.044), pa("head", 0.154, 0.091, 0.024), pa("head", 0.153, 0.095, 0), pa("head", 0.154, 0.091, -0.024), pa("head", 0.15, 0.08, -0.044)], [0.004, 0.0055, 0.005, 0.0055, 0.004]));
  for (const side of [1, -1]) {
    const s = side > 0 ? "l" : "r";
    out.push(ellipsoid(C(`eye-${s}`, "head", "skin", TONE.eye, "detail"), [0.0065, 0.0055, 0.0098], pa("head", 0.139, 0.084, side * 0.031)));
    out.push(ellipsoid(C(`ear-${s}`, "head", "skin", TONE.skin), [0.031, 0.017, 0.0085], pa("head", 0.126, -0.013, side * 0.073), compose(aboutU(side * -12), aboutW(16))));
  }
  out.push(loft(C("hair", "head", "hair", TONE.hair), HAIR));
  out.push(loft(C("beanie", "head", "wool-hat", TONE.beanie), dome(dense(BEANIE, 2), 1, 0.004, 2)));
  out.push(loft(C("beanie-cuff", "head", "wool-hat", TONE.cuff, "detail"), BEANIE_CUFF));
  out.push(wall());
  return out;
}

/** ↔️ A loft whose path runs to the left across a bone: sections in the (along, sagittal) plane, `rw` along the bone, `rv` across. */
function lateral(p: Json, s: readonly St[]): Json {
  return { ...loft(p, []), sections: s.map((q) => ({ at: q.at.map((x) => Math.round(x * 1e5) / 1e5), rx: q.rw, ry: q.rv, exponent: q.e })), capStart: true, capEnd: true };
}

/** 👟 One sneaker: sole (rear on the foot bone, front on the toe bone), upper with collar and tongue, toe box, toe cap and laces. */
function shoe(s: "l"): Json[] {
  const foot = `foot-${s}`;
  const toe = `toe-${s}`;
  const out: Json[] = [];
  out.push(extrude(C(`sole-${s}`, foot, "leather-shoe", TONE.sole), "zx", SOLE_REAR.map(([x, l]) => [l, x]), G, G + 0.024));
  out.push(extrude(C(`sole-toe-${s}`, toe, "leather-shoe", TONE.sole), "zx", SOLE_TOE.map(([x, l]) => [l, x]), 0.0, 0.019));
  out.push(loft(C(`shoe-${s}`, foot, "leather-shoe", TONE.shoe), dense(UPPER.map((r) => shoeSt(foot, r)), 2)));
  out.push(loft(C(`shoe-toe-${s}`, toe, "leather-shoe", TONE.shoe), dome(TOE_BOX.map((r, i) => shoeSt(toe, r, TOE_DRIFT[i])), 1, 0.006, 2)));
  out.push(loft(C(`toe-cap-${s}`, toe, "leather-shoe", TONE.toeCap, "detail"), [shoeSt(toe, [0.05, 0.016, 0.034, 0.048, 2.4], -0.006), shoeSt(toe, [0.072, 0.017, 0.032, 0.036, 2.2], -0.008), shoeSt(toe, [0.085, 0.019, 0.03, 0.022, 2], -0.009)]));
  const lace: V3[] = [[0.088, G + 0.083, 0], [0.065, G + 0.094, 0], [0.04, G + 0.099, 0]];
  out.push(loft(C(`laces-${s}`, foot, "leather-shoe", TONE.lace, "detail"), lace.map((at) => ({ at, rw: 0.017, rv: 0.0045, e: 3, n: 0 }))));
  return out;
}

/* ================================================================ thermal */

const LAYER: Record<string, string> = { "torso-clothing": "hoodie", "arm-clothing": "sleeve", "leg-clothing": "denim", shoes: "shoe-upper" };
const INNER: Record<string, [string, number]> = { "torso-clothing": ["torso-air", 0.004], "arm-clothing": ["sleeve-air", 0.004], "leg-clothing": ["jeans-air", 0.0012] };

export function thermal50(b: Json[]): Json[] {
  const out: Json[] = [];
  const byId = new Map(b.map((s) => [s.id, s]));
  const shell = (id: string, region: string, layer: string, src: Json, by: number, extra: Json = {}): Json => ({ ...src, id, region, layer, sections: src.sections.map((q: Json) => ({ ...q, rx: Math.max(0.003, q.rx - by), ry: Math.max(0.003, q.ry - by) })), ...extra });
  for (const id of ["hoodie", "hoodie-hem", "hood", "hood-roll", "hoodie-pocket", "hood-string-l", "hood-string-r", "hood-aglet-l", "hood-aglet-r"]) out.push(thOf(id, "torso-clothing", "hoodie"));
  out.push(shell("th-torso-air", "torso-clothing", "torso-air", byId.get("hoodie"), 0.004));
  for (const s of ["l", "r"]) {
    const up = byId.get(`upper-arm-${s}`);
    out.push({ ...up, id: `th-upper-arm-${s}`, region: "arm-clothing", layer: "sleeve", sections: up.sections.filter((q: Json) => q.at[0] >= -0.007) });
    out.push(thOf(`forearm-${s}`, "arm-clothing", "sleeve"));
    out.push(thOf(`cuff-${s}`, "arm-clothing", "sleeve"));
    out.push(shell(`th-sleeve-air-upper-${s}`, "arm-clothing", "sleeve-air", byId.get(`upper-arm-${s}`), 0.004));
    out.push(shell(`th-sleeve-air-fore-${s}`, "arm-clothing", "sleeve-air", byId.get(`forearm-${s}`), 0.004));
    const wl = (WRIST_L.map((q) => sa("forearm-l", q.at[0], 0, 0, q.rw, q.rv, q.e)));
    const wrist = loft(C(`th-wrist-${s}`, `forearm-${s}`, "skin", TONE.hand), [{ ...wl[0]!, at: [0.233, 0, 0], rw: 0.0272, rv: 0.0205 }, { ...wl[1]!, at: [0.256, 0, 0], rw: 0.0282, rv: 0.0195 }], [false, false]);
    out.push({ ...wrist, region: "arm-skin" });
    out.push(thOf(`sock-${s}`, "shoes", "sock-layer"));
    out.push(thOf(`thigh-${s}`, "leg-clothing", "denim"));
    out.push(thOf(`shank-${s}`, "leg-clothing", "denim"));
    out.push(shell(`th-jeans-air-thigh-${s}`, "leg-clothing", "jeans-air", byId.get(`thigh-${s}`), 0.0012));
    out.push(shell(`th-jeans-air-shank-${s}`, "leg-clothing", "jeans-air", byId.get(`shank-${s}`), 0.0012));
    for (const id of [`shoe-${s}`, `shoe-toe-${s}`, `sole-${s}`, `sole-toe-${s}`, `toe-cap-${s}`, `laces-${s}`]) out.push(thOf(id, "shoes", "shoe-upper"));
    out.push(shell(`th-shoe-air-${s}`, "shoes", "shoe-air", byId.get(`shoe-${s}`), 0.004));
    out.push(shell(`th-shoe-air-toe-${s}`, "shoes", "shoe-air", byId.get(`shoe-toe-${s}`), 0.004));
    out.push(shell(`th-sock-shell-${s}`, "shoes", "sock-layer", byId.get(`shoe-${s}`), 0.013));
    out.push(shell(`th-sock-shell-toe-${s}`, "shoes", "sock-layer", byId.get(`shoe-toe-${s}`), 0.013));
  }
  out.push(thOf("jeans-seat", "leg-clothing", "denim"));
  out.push(shell("th-jeans-air-seat", "leg-clothing", "jeans-air", byId.get("jeans-seat"), 0.0012));
  for (const id of b.filter((s) => /^hand-[lr]/.test(s.id)).map((s) => s.id)) out.push(thOf(id, "hand-skin"));
  const head = byId.get("head");
  const faceTop = 0.168;
  out.push({ ...head, id: "th-face", region: "head-skin", sections: head.sections.filter((q: Json) => q.at[0] <= faceTop), capEnd: false });
  for (const id of ["nose", "lips", "ear-l", "ear-r", "eye-l", "eye-r", "brow"]) out.push(thOf(id, "head-skin"));
  out.push(thOf("hair", "hair", "hair-under"));
  out.push(thOf("beanie", "hair", "beanie"));
  out.push(thOf("beanie-cuff", "hair", "beanie"));
  out.push(shell("th-hair-under-beanie", "hair", "hair-under", byId.get("beanie"), 0.004));
  out.push(thOf("phone", "phone-surface"));
  out.push(thOf("phone-screen", "phone-surface"));
  out.push({ ...box(C("th-wall-brick", "wall", "masonry", TONE.wall, "ground"), [0.365, 1.9, 1.2], [-0.1825, 0.95, 0]), region: "wall-face", layer: "brick", sectionFaces: ["+v", "-v", "-u"], texture: true });
  out.push({ ...box(C("th-wall-plaster", "wall", "masonry", TONE.wall, "ground"), [0.015, 1.9, 1.2], [-0.3725, 0.95, 0]), region: "wall-face", layer: "plaster", sectionFaces: ["+v", "-v", "-u"], texture: true });
  out.push({ ...ellipsoid(C("th-nose-breath", "head", "skin", TONE.skin), [0.0038, 0.0048, 0.0075], pa("head", 0.0905, 0.103, 0)), region: "head-skin", tags: ["emitter", "nose-breath"] });
  out.push({ ...ellipsoid(C("th-mouth-breath", "head", "skin", TONE.lips), [0.0062, 0.0038, 0.0105], pa("head", 0.068, 0.102, 0)), region: "head-skin", tags: ["emitter", "mouth-breath"] });
  out.push({ ...ellipsoid(C("th-plume", "head", "wool-hat", TONE.beanie), [0.004, 0.05, 0.045], pa("head", 0.262, -0.02, 0)), region: "torso-clothing", tags: ["emitter", "plume"] });
  return out;
}

/* ================================================================ structure: the skeleton */

const BONE_M = "bone";
const sk = (id: string, bone: string, member: string, kind: "member" | "joint" | "support" | "mass" = "member"): Json => ({ ...C(id, bone, BONE_M, kind === "member" ? TONE.bone : kind === "joint" ? TONE.joint : TONE.mass), member, part: kind });
const bar = (id: string, bone: string, pts: V3[], r: number[], member = bone): Json => ({ ...tube(C(id, bone, BONE_M, TONE.bone), pts, r), member, part: "member" });

export function structure50(masses: Record<string, [number, number, number]>): Json[] {
  const out: Json[] = [];
  const spine: V3[] = [[-0.035, -0.07, 0], [0.0, -0.042, 0], [0.05, -0.03, 0], [0.1, -0.03, 0], [0.15, -0.04, 0], [0.22, -0.058, 0], [0.3, -0.07, 0], [0.38, -0.065, 0], [0.45, -0.048, 0], [0.48, -0.04, 0]].map(([a, f]) => pa("torso", a, f, 0));
  out.push(bar("spine", "torso", spine, [0.024, 0.024, 0.023, 0.022, 0.02, 0.017, 0.016, 0.015, 0.014, 0.013]));
  const spineF = (a: number): number => { const q = spine.map((p) => [p[0], -p[1]] as const); let i = 0; while (i + 2 < q.length && q[i + 1]![0] < a) i++; const t = (a - q[i]![0]) / (q[i + 1]![0] - q[i]![0]); return q[i]![1] + (q[i + 1]![1] - q[i]![1]) * t; };
  for (const a of [0.03, 0.09, 0.15, 0.21, 0.27, 0.33, 0.39]) {
    const f = spineF(a);
    out.push({ ...capsule(C(`spinous-${Math.round(a * 100)}`, "torso", BONE_M, TONE.bone), pa("torso", a, f - 0.018, 0), pa("torso", a - 0.022, f - 0.042, 0), 0.006, 0.0045), member: "torso", part: "member" });
  }
  const RIBS: [number, number, number][] = [[0.43, 0.058, 0.07], [0.405, 0.088, 0.09], [0.38, 0.104, 0.1], [0.355, 0.114, 0.11], [0.33, 0.121, 0.115], [0.305, 0.126, 0.118], [0.28, 0.13, 0.12], [0.255, 0.132, 0.118], [0.23, 0.131, 0.112], [0.205, 0.127, 0.1], [0.18, 0.118, 0.07], [0.16, 0.1, 0.04]];
  RIBS.forEach(([a, w, drop], i) => {
    const sp = spine.length;
    void sp;
    const back = -0.07 - 0.012 * Math.sin((Math.PI * i) / 12);
    const free = i >= 10;
    const ptsL: V3[] = [[a, back + 0.02, 0.022], [a + 0.004, back - 0.022, w * 0.42], [a - drop * 0.25, back + 0.005, w * 0.88], [a - drop * 0.55, 0.0, w], [a - drop * 0.85, 0.05, w * 0.8]];
    if (!free) ptsL.push([a - drop + (i >= 7 ? 0.0 : 0.012), 0.084, w * 0.38], [Math.max(0.215, a - drop + 0.02), 0.092, 0.018]);
    const pts = ptsL.map(([aa, f, l]) => pa("torso", aa, f, l));
    const r = i === 0 ? 0.0065 : 0.0055;
    const rib = bar(`rib-${i + 1}-l`, "torso", pts, pts.map(() => r));
    out.push(rib, mirrored(rib, toRight));
  });
  out.push({ ...loft(C("sternum", "torso", BONE_M, TONE.bone), [sa("torso", 0.42, 0.083, 0, 0.026, 0.008, 2.4), sa("torso", 0.38, 0.087, 0, 0.018, 0.008, 2.4), sa("torso", 0.3, 0.093, 0, 0.016, 0.008, 2.4), sa("torso", 0.24, 0.096, 0, 0.01, 0.006, 2)]), member: "torso", part: "member" });
  const clav = bar("clavicle-l", "torso", [[0.412, 0.078, 0.022], [0.426, 0.074, 0.075], [0.44, 0.045, 0.13], [0.448, 0.008, 0.168]].map(([a, f, l]) => pa("torso", a, f, l)), [0.0075, 0.007, 0.0065, 0.0075]);
  const scap = { ...loft(C("scapula-l", "torso", BONE_M, TONE.bone), [sa("torso", 0.405, -0.088, 0.105, 0.05, 0.007, 2.6), sa("torso", 0.37, -0.096, 0.102, 0.045, 0.007, 2.6), sa("torso", 0.32, -0.099, 0.095, 0.029, 0.006, 2.4), sa("torso", 0.28, -0.097, 0.09, 0.01, 0.005, 2)].map((q) => ({ ...q, yaw: 18 }))), member: "torso", part: "member" };
  const spineScap = bar("scapular-spine-l", "torso", [[0.39, -0.095, 0.065], [0.415, -0.092, 0.13], [0.44, -0.05, 0.175], [0.445, -0.012, 0.18]].map(([a, f, l]) => pa("torso", a, f, l)), [0.006, 0.007, 0.008, 0.008]);
  out.push(clav, mirrored(clav, toRight), scap, mirrored(scap, toRight), spineScap, mirrored(spineScap, toRight));
  out.push({ ...loft(C("sacrum", "pelvis", BONE_M, TONE.bone), [sa("pelvis", 0.125, -0.052, 0, 0.05, 0.022, 2.2, -25), sa("pelvis", 0.08, -0.068, 0, 0.042, 0.018, 2.2, -32), sa("pelvis", 0.035, -0.085, 0, 0.026, 0.013, 2, -40), sa("pelvis", 0.0, -0.095, 0, 0.012, 0.009, 2, -50)]), member: "pelvis", part: "member" });
  const ilium = { ...loft(C("ilium-l", "pelvis", BONE_M, TONE.bone), [sa("pelvis", 0.138, -0.002, 0.098, 0.074, 0.007, 2.2), sa("pelvis", 0.105, 0.0, 0.1, 0.074, 0.008, 2.2), sa("pelvis", 0.06, 0.004, 0.094, 0.05, 0.012, 2.2), sa("pelvis", 0.02, 0.0, 0.086, 0.028, 0.02, 2), sa("pelvis", -0.025, -0.008, 0.074, 0.022, 0.016, 2)].map((q, i) => ({ ...q, yaw: [52, 50, 40, 18, 0][i] }))), member: "pelvis", part: "member" };
  const pubis = bar("pubis-l", "pelvis", [[-0.01, 0.012, 0.075], [-0.022, 0.045, 0.04], [-0.03, 0.058, 0.006]].map(([a, f, l]) => pa("pelvis", a, f, l)), [0.011, 0.01, 0.01]);
  const ischium = bar("ischium-l", "pelvis", [[-0.02, -0.022, 0.074], [-0.065, -0.038, 0.062], [-0.072, -0.012, 0.048], [-0.05, 0.04, 0.01]].map(([a, f, l]) => pa("pelvis", a, f, l)), [0.012, 0.014, 0.01, 0.009]);
  out.push(ilium, mirrored(ilium, toRight), pubis, mirrored(pubis, toRight), ischium, mirrored(ischium, toRight));
  out.push({ ...ellipsoid(C("cranium", "head", BONE_M, TONE.bone), [0.077, 0.088, 0.066], pa("head", 0.164, -0.003, 0)), member: "head", part: "member" });
  out.push({ ...loft(C("face-skeleton", "head", BONE_M, TONE.bone), [sa("head", 0.15, 0.06, 0, 0.06, 0.03, 2.2), sa("head", 0.115, 0.07, 0, 0.062, 0.027, 2.2), sa("head", 0.085, 0.072, 0, 0.042, 0.022, 2)]), member: "head", part: "member" });
  out.push({ ...tube(C("mandible", "head", BONE_M, TONE.bone), [[0.115, -0.012, 0.055], [0.06, -0.006, 0.05], [0.04, 0.035, 0.04], [0.03, 0.082, 0.0], [0.04, 0.035, -0.04], [0.06, -0.006, -0.05], [0.115, -0.012, -0.055]].map(([a, f, l]) => pa("head", a, f, l)), [0.008, 0.009, 0.009, 0.011, 0.009, 0.009, 0.008]), member: "head", part: "member" });
  out.push(bar("cervical-spine", "head", [[-0.04, -0.04, 0], [0.0, -0.03, 0], [0.04, -0.018, 0], [0.08, -0.012, 0], [0.11, -0.01, 0]].map(([a, f, l]) => pa("head", a, f, l)), [0.013, 0.012, 0.011, 0.011, 0.012]));
  for (const s of ["l"]) {
    const th = `thigh-${s}`;
    const femur = [bar(`femur-neck-${s}`, th, [pa(th, 0, 0, 0), pa(th, 0.03, -0.004, 0.045), pa(th, 0.05, -0.008, 0.06)], [0.016, 0.014, 0.017]),
      { ...ellipsoid(C(`femur-head-${s}`, th, BONE_M, TONE.bone), [0.024, 0.024, 0.024], pa(th, 0, 0, 0)), member: th, part: "member" },
      { ...ellipsoid(C(`trochanter-${s}`, th, BONE_M, TONE.bone), [0.03, 0.02, 0.017], pa(th, 0.035, -0.012, 0.068)), member: th, part: "member" },
      bar(`femur-${s}`, th, [pa(th, 0.05, -0.006, 0.058), pa(th, 0.15, 0.006, 0.042), pa(th, 0.27, 0.01, 0.022), pa(th, 0.37, 0.002, 0.004)], [0.015, 0.0135, 0.0135, 0.016]),
      { ...ellipsoid(C(`femur-condyles-${s}`, th, BONE_M, TONE.bone), [0.032, 0.034, 0.042], pa(th, 0.395, -0.006, 0.0)), member: th, part: "member" }];
    const sh = `shank-${s}`;
    const tib = [{ ...ellipsoid(C(`tibia-plateau-${s}`, sh, BONE_M, TONE.bone), [0.025, 0.03, 0.039], pa(sh, 0.025, 0.0, 0)), member: sh, part: "member" },
      bar(`tibia-${s}`, sh, [pa(sh, 0.04, 0.006, -0.004), pa(sh, 0.2, 0.01, -0.006), pa(sh, 0.37, 0.004, -0.008)], [0.018, 0.0125, 0.015]),
      { ...ellipsoid(C(`malleolus-medial-${s}`, sh, BONE_M, TONE.bone), [0.022, 0.016, 0.014], pa(sh, 0.392, 0.004, -0.018)), member: sh, part: "member" },
      bar(`fibula-${s}`, sh, [pa(sh, 0.035, -0.012, 0.033), pa(sh, 0.22, -0.008, 0.028), pa(sh, 0.405, -0.008, 0.028)], [0.009, 0.0075, 0.01]),
      { ...ellipsoid(C(`patella-${s}`, sh, BONE_M, TONE.bone), [0.023, 0.011, 0.021], pa(sh, -0.012, 0.044, 0)), member: sh, part: "member" }];
    const ft = `foot-${s}`;
    const foot = [{ ...ellipsoid(C(`talus-${s}`, ft, BONE_M, TONE.bone), [0.028, 0.018, 0.02], [0.006, -0.006, 0]), member: ft, part: "member" },
      bar(`calcaneus-${s}`, ft, [[0.0, -0.024, 0.002], [-0.03, -0.042, 0.004], [-0.05, -0.052, 0.004]], [0.019, 0.02, 0.018], ft),
      { ...ellipsoid(C(`midfoot-${s}`, ft, BONE_M, TONE.bone), [0.03, 0.017, 0.03], [0.045, -0.024, -0.002]), member: ft, part: "member" }];
    for (const [k, l0, l1] of [[1, -0.018, -0.026], [2, -0.006, -0.008], [3, 0.006, 0.008], [4, 0.016, 0.022], [5, 0.025, 0.036]] as const) foot.push(bar(`metatarsal-${k}-${s}`, ft, [[0.06, -0.028, l0], [0.124, -0.06, l1]], [k === 1 ? 0.0075 : 0.006, k === 1 ? 0.0085 : 0.0065], ft));
    const toe = `toe-${s}`;
    const toes: Json[] = [];
    for (const [k, l, len] of [[1, -0.028, 0.052], [2, -0.009, 0.046], [3, 0.008, 0.04], [4, 0.022, 0.034], [5, 0.035, 0.028]] as const) toes.push(bar(`phalanges-${k}-${s}`, toe, [[0.0, 0.03, l], [len * 0.6, 0.029, l - 0.002], [len, 0.026, l - 0.004]], [k === 1 ? 0.0085 : 0.006, 0.0055, 0.005], toe));
    const ua = `upper-arm-${s}`;
    const hum = [{ ...ellipsoid(C(`humerus-head-${s}`, ua, BONE_M, TONE.bone), [0.024, 0.024, 0.024], pa(ua, 0.0, 0.0, -0.004)), member: ua, part: "member" },
      bar(`humerus-${s}`, ua, [pa(ua, 0.02, 0.004, 0.006), pa(ua, 0.15, 0.002, 0.004), pa(ua, 0.285, -0.004, 0.0)], [0.0155, 0.011, 0.012]),
      { ...ellipsoid(C(`humerus-condyles-${s}`, ua, BONE_M, TONE.bone), [0.015, 0.016, 0.031], pa(ua, 0.303, -0.004, 0.0)), member: ua, part: "member" }];
    const fa = `forearm-${s}`;
    const fore = [bar(`ulna-${s}`, fa, [pa(fa, -0.022, -0.012, -0.012), pa(fa, 0.03, -0.006, -0.012), pa(fa, 0.15, 0.0, -0.014), pa(fa, 0.245, -0.004, -0.017)], [0.011, 0.009, 0.007, 0.0075]),
      bar(`radius-${s}`, fa, [pa(fa, 0.01, 0.004, 0.012), pa(fa, 0.12, 0.006, 0.012), pa(fa, 0.24, 0.006, 0.014)], [0.008, 0.0075, 0.011])];
    out.push(...femur, ...tib, ...foot, ...toes, ...hum, ...fore);
    for (const q of [...femur, ...tib, ...foot, ...toes, ...hum, ...fore]) out.push(mirrored(q, toRight));
  }
  const handL = "hand-l";
  const hl: Json[] = [{ ...ellipsoid(C("carpus-l", handL, BONE_M, TONE.bone), [0.018, 0.026, 0.009], pa(handL, 0.016, 0.002, -0.001)), member: handL, part: "member" }];
  for (const [name, f, k] of [["index", 0.024, 0.95], ["middle", 0.008, 1.03], ["ring", -0.009, 0.97], ["little", -0.024, 0.78]] as const) {
    const pts = fingerPath(pa(handL, 0.032, f * 0.8, -0.002), [[0, 0], [14, 0], [36, 0], [58, 0]], [0.062, 0.042 * k, 0.025 * k, 0.018 * k], "-l");
    hl.push(bar(`hand-bones-${name}-l`, handL, pts, [0.0045, 0.0045, 0.004, 0.0038, 0.0035], handL));
  }
  hl.push(bar("thumb-bones-l", handL, [pa(handL, 0.012, 0.022, -0.006), [0.048, 0.042, -0.016], [0.078, 0.05, -0.027], [0.098, 0.051, -0.036]], [0.005, 0.0045, 0.004, 0.0038], handL));
  out.push(...hl);
  const handR = "hand-r";
  out.push({ ...ellipsoid(C("carpus-r", handR, BONE_M, TONE.bone), [0.018, 0.026, 0.009], pa(handR, 0.016, 0.002, 0.001)), member: handR, part: "member" });
  for (const [name, f] of [["index", 0.024], ["middle", 0.008], ["ring", -0.009], ["little", -0.024]] as const) {
    const edge = PHONE.u + PHONE.width / 2;
    out.push(bar(`hand-bones-${name}-r`, handR, [pa(handR, 0.032, f * 0.8, 0.002), pa(handR, 0.094, f, 0.004), [edge - 0.004, f, PHONE.w - PHONE.thick / 2 - 0.0095], [edge + 0.011, f, PHONE.w + 0.004]], [0.0045, 0.0045, 0.004, 0.0036], handR));
  }
  out.push(bar("thumb-bones-r", handR, [pa(handR, 0.012, 0.022, 0.006), [0.046, 0.04, 0.018], [PHONE.u - PHONE.width / 2 + 0.006, 0.054, PHONE.w + PHONE.thick / 2 + 0.006]], [0.005, 0.0045, 0.004], handR));
  const J = (id: string, bone: string, at: V3, r: V3, member = bone): Json => ({ ...ellipsoid(C(id, bone, BONE_M, TONE.joint), r, at), member, part: "joint" });
  out.push(J("joint-lumbosacral", "torso", pa("torso", 0, -0.042, 0), [0.014, 0.026, 0.026]));
  out.push(J("joint-neck", "head", pa("head", 0, -0.03, 0), [0.01, 0.017, 0.017]));
  out.push(J("joint-atlas", "head", pa("head", 0.115, -0.01, 0), [0.009, 0.016, 0.022]));
  for (const s of ["l", "r"]) {
    const g = s === "l" ? 1 : -1;
    out.push(J(`joint-hip-${s}`, `thigh-${s}`, [0, 0, 0], [0.027, 0.027, 0.027]));
    out.push(J(`joint-knee-${s}`, `shank-${s}`, [0, 0, 0], [0.012, 0.026, 0.038]));
    out.push(J(`joint-ankle-${s}`, `foot-${s}`, [0, 0, 0], [0.016, 0.014, 0.024]));
    out.push(J(`joint-toe-${s}`, `toe-${s}`, [0.0, 0.026, -0.0], [0.009, 0.01, 0.038]));
    out.push(J(`joint-shoulder-${s}`, `upper-arm-${s}`, [0, 0, 0], [0.026, 0.026, 0.026]));
    out.push(J(`joint-elbow-${s}`, `forearm-${s}`, [0, 0, 0], [0.013, 0.015, 0.026]));
    out.push(J(`joint-wrist-${s}`, `hand-${s}`, [0, 0, 0], [0.009, 0.017, 0.011]));
    void g;
  }
  for (const member of ["wall", "wall-x", "wall-frame"]) out.push({ id: `member-${member}`, of: "wall", member, part: "member" });
  for (const [bone, [x, y, r]] of Object.entries(masses)) out.push({ ...ellipsoid(C(`mass-${bone}`, bone, BONE_M, TONE.mass), [r, r, r], [x, y, 0]), member: bone, part: "mass" });
  return out;
}

export const DECISIONS_50: Json[] = [];
