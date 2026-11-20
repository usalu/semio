/** 🍳 The 1:50 body of `humans/person-cooking`, stated section by section: a slim woman of 1.69 m and 58 kg in a hooded fleece sweatshirt (hood down on the upper back, drawstrings, kangaroo pocket, rib hem and cuffs), slim jeans and low canvas sneakers, with short cropped hair, stirring with a beech cooking spoon in a 22 cm steel pot on the front right zone of a glass-ceramic hob in a 600 mm base unit with three drawer fronts.
 *
 * Every station of the figure is anatomical (`sa`, `pa`): `a` along the bone from its joint, `f` ahead of the bone line (up for the foot), `l` to the left. The trunk and seat tables are the slices of `r3-person-cooking-forms.ts`. The unit, the pot and the spoon are stated in their own bone frames (`counter`: x into the unit from the front edge of the worktop, y up from the floor, z left; `pot`: origin at the centre of the base; `spoon`: u from the grip to the tip of the bowl, the hollow facing +v).
 * @see ./r3-person-cooking-kit.ts
 * @see ./r3-humans-person-cooking.md */
import type { Json } from "./volumes-lib.ts";
import { D, FRAMES, L, W, aboutU, aboutW, box, capsule, compose, dense, dome, ellipsoid, extrude, loft, mirrored, pa, revolve, sa, st, toRight, tube, type St, type V3 } from "./r3-person-cooking-kit.ts";
import { HOODIE, HOODIE_SKIRT, SEAT, pelvisRow, sliceX, torsoAF, torsoRow, type Slice } from "./r3-person-cooking-forms.ts";
import { POT, RIG, SPOON, UNIT } from "./r3-person-cooking-data.ts";

export const TONE = { hoodie: 0.5, rib: 0.42, pocket: 0.46, cord: 0.2, jeans: 0.72, canvas: 0.3, rubber: 0.12, lace: 0.1, skin: 0.25, hand: 0.3, lips: 0.42, eye: 0.9, brow: 0.6, hair: 0.78, bone: 0.92, joint: 0.6, mass: 0.5, front: 0.3, carcass: 0.38, worktop: 0.55, plinth: 0.62, hob: 0.92, zone: 0.6, steel: 0.35, wood: 0.4, water: 0.2 } as const;
export const MAT = { fleece: "fleece-sweatshirt", denim: "denim", sneaker: "canvas-sneaker", skin: "skin", hair: "hair", sock: "cotton-sock", cotton: "cotton-underwear", bone: "bone", board: "laminate-chipboard", glass: "glass-ceramic", steel: "stainless-steel-pot", wood: "beech-wood", water: "water" } as const;
export const C = (id: string, bone: string, material: string, tone: number, role = "body"): Json => ({ id, bone, material, tone, role });
export const G = -RIG.ankle;
const rad = (d: number): number => (d * Math.PI) / 180;
const deg = (r: number): number => (r * 180) / Math.PI;

/* ================================================================ trunk */

/** 🧥 A trunk slice as a torso station. */
export const torsoSt = (s: Slice, grow = 0): St => { const [a, f, hw, hd, e] = torsoRow(s); return sa("torso", a, f, 0, hw + grow, hd + grow, e); };
/** 🪢 A slice that hangs from the pelvis (the sweatshirt over the seat) as a torso station: centre and section plane carried by the pelvis of the default pose, tilted `tilt` degrees (negative: higher at the front). */
export function skirtSt([y, b, f, hw, e]: Slice, tilt = 0, grow = 0): St {
  const a = y - RIG.hip;
  const c = W("pelvis", a, (b + f) / 2, 0);
  const pu = FRAMES.pelvis!.u;
  const pv = FRAMES.pelvis!.v;
  const t = rad(tilt);
  const nw: V3 = [pu[0] * Math.cos(t) - pv[0] * Math.sin(t), pu[1] * Math.cos(t) - pv[1] * Math.sin(t), pu[2] * Math.cos(t) - pv[2] * Math.sin(t)];
  const nl = D("torso", nw);
  return { at: L("torso", c), rw: hw + grow, rv: (f - b) / 2 + grow, e, n: deg(Math.atan2(nl[1], nl[0])) };
}
/** 🧥 The whole sweatshirt body: skirt over the seat, trunk, shoulders, neckline. */
export const hoodieStations = (grow = 0): St[] => [...HOODIE_SKIRT.map((s) => skirtSt(s, 0, grow)), ...HOODIE.map((s) => torsoSt(s, grow))];
/** 👖 The jeans seat as pelvis stations. */
export const seatStations = (grow = 0): St[] => SEAT.map((s) => { const [a, f, hw, hd, e] = pelvisRow(s); return sa("pelvis", a, f, 0, hw + grow, hd + grow, e); });

/** 🔘 A point on the sweatshirt front (+1) or back (-1) of the upright woman at height `y`, `l` to the left, `off` proud of the fleece, torso-local. */
export function onHoodie(y: number, l: number, side: 1 | -1, off = 0): V3 {
  const x = sliceX(HOODIE, y, l, side) + side * off;
  const [ta, tf] = torsoAF(x, y);
  return pa("torso", ta, tf, l);
}
/** 🎒 The hood lying on the upper back: a flattened pouch from the neckline down to between the shoulder blades, slices [y, back, front, half-width, exponent]. */
export const HOOD: readonly Slice[] = [
  [1.262, -0.156, -0.128, 0.045, 2.2],
  [1.29, -0.168, -0.128, 0.082, 2.3],
  [1.33, -0.178, -0.126, 0.112, 2.4],
  [1.375, -0.178, -0.118, 0.128, 2.5],
  [1.415, -0.166, -0.098, 0.13, 2.5],
  [1.448, -0.146, -0.07, 0.116, 2.4],
  [1.472, -0.126, -0.05, 0.096, 2.3],
];
/** 🧣 The rolled edge of the hood round the neck, falling open at the throat. */
export const HOOD_RIM: readonly Slice[] = [
  [1.44, -0.104, 0.05, 0.092, 2.2],
  [1.464, -0.1, 0.043, 0.084, 2.1],
  [1.49, -0.092, 0.028, 0.071, 2.0],
];
/** 👝 The kangaroo pocket on the lower front: [y, back, front, half-width, exponent]. */
export const POCKET: readonly Slice[] = [
  [0.905, 0.07, 0.113, 0.15, 3.2],
  [0.95, 0.072, 0.117, 0.152, 3.2],
  [1.0, 0.072, 0.117, 0.136, 3.2],
  [1.058, 0.076, 0.117, 0.104, 3.0],
];

/* ================================================================ legs (thigh, shank: a down the bone, f ahead, l left) */

export const THIGH_L: St[] = [
  sa("thigh-l", -0.06, -0.01, 0.006, 0.083, 0.09, 2.2),
  sa("thigh-l", 0.03, -0.004, 0.002, 0.084, 0.089, 2.2),
  sa("thigh-l", 0.1, 0.004, -0.003, 0.08, 0.085, 2.1),
  sa("thigh-l", 0.18, 0.008, -0.004, 0.073, 0.078, 2.05),
  sa("thigh-l", 0.26, 0.008, -0.004, 0.065, 0.07, 2),
  sa("thigh-l", 0.33, 0.006, -0.002, 0.057, 0.062, 2),
  sa("thigh-l", 0.38, 0.007, -0.001, 0.052, 0.056, 2),
  sa("thigh-l", 0.413, 0.005, 0, 0.05, 0.054, 2),
  sa("thigh-l", 0.445, 0.002, 0, 0.044, 0.046, 2),
];
export const SHANK_L: St[] = [
  sa("shank-l", -0.035, 0.002, 0, 0.043, 0.045, 2),
  sa("shank-l", -0.004, 0.003, 0, 0.049, 0.052, 2),
  sa("shank-l", 0.04, -0.002, 0, 0.046, 0.049, 2),
  sa("shank-l", 0.1, -0.011, 0.002, 0.049, 0.054, 2),
  sa("shank-l", 0.15, -0.013, 0.002, 0.05, 0.055, 2),
  sa("shank-l", 0.2, -0.01, 0.001, 0.046, 0.05, 2),
  sa("shank-l", 0.26, -0.006, 0, 0.041, 0.045, 2),
  sa("shank-l", 0.32, -0.004, 0, 0.039, 0.043, 2),
  sa("shank-l", 0.386, -0.004, 0, 0.04, 0.045, 2.1),
];
/** 👖 The turned-up hem of the jeans at the ankle. */
export const HEM_L: St[] = [sa("shank-l", 0.36, -0.004, 0, 0.0415, 0.0465, 2.1), sa("shank-l", 0.374, -0.004, 0, 0.042, 0.047, 2.1), sa("shank-l", 0.389, -0.004, 0, 0.042, 0.047, 2.1)];

/* ================================================================ sneaker (foot bone horizontal: a ahead of the ankle, f up, ground at f = G) */

/** 👟 The canvas upper as cross slices from the heel counter to the ball: [x ahead of the ankle, bottom, top (above the ground), half-width, exponent]. */
export const UPPER: readonly [number, number, number, number, number][] = [
  [-0.06, 0.02, 0.082, 0.02, 2.2],
  [-0.05, 0.02, 0.096, 0.029, 2.4],
  [-0.03, 0.02, 0.102, 0.0335, 2.5],
  [0.0, 0.02, 0.104, 0.0355, 2.5],
  [0.03, 0.019, 0.098, 0.0375, 2.6],
  [0.06, 0.018, 0.083, 0.0405, 2.7],
  [0.09, 0.017, 0.069, 0.0435, 2.8],
  [0.126, 0.016, 0.058, 0.045, 2.8],
];
/** 🛞 The rubber sole with its foxing band under the foot bone: [x, top, half-width]. */
export const SOLE: readonly [number, number, number][] = [
  [-0.066, 0.027, 0.022],
  [-0.055, 0.027, 0.0315],
  [-0.03, 0.026, 0.036],
  [0.02, 0.024, 0.0365],
  [0.07, 0.022, 0.0425],
  [0.127, 0.02, 0.0475],
];
/** 🦶 The rubber toe cap on the toe bone (origin on the ground under the ball): [x, bottom, top, half-width, exponent, drift towards the big toe]. */
export const TOE_CAP: readonly [number, number, number, number, number, number][] = [
  [-0.012, 0.014, 0.058, 0.045, 2.8, 0],
  [0.02, 0.014, 0.053, 0.044, 2.7, -0.002],
  [0.044, 0.014, 0.046, 0.04, 2.5, -0.005],
  [0.06, 0.014, 0.039, 0.032, 2.3, -0.008],
  [0.07, 0.016, 0.032, 0.02, 2.1, -0.01],
];
/** 🛞 The outsole of the forefoot under the toe bone, plan outline [x, l] of the left shoe. */
const TOE_SOLE: [number, number][] = [[-0.012, 0.0475], [0.02, 0.047], [0.044, 0.043], [0.061, 0.035], [0.073, 0.021], [0.078, 0.0], [0.075, -0.015], [0.066, -0.03], [0.05, -0.042], [0.025, -0.048], [-0.012, -0.0485]];

const xSlice = (x: number, lo: number, hi: number, hw: number, e: number, l = 0, ground = G): St => ({ at: [x, ground + (lo + hi) / 2, l], rw: hw, rv: (hi - lo) / 2, e, n: 0 });

function sneaker(): Json[] {
  const f = "foot-l";
  const t = "toe-l";
  const out: Json[] = [];
  out.push(loft(C("sole-l", f, MAT.sneaker, TONE.rubber), SOLE.map(([x, top, hw]) => xSlice(x, 0, top, hw, 3.4))));
  out.push(extrude(C("toe-sole-l", t, MAT.sneaker, TONE.rubber), "zx", TOE_SOLE.map(([x, l]) => [l, x]), 0, 0.02));
  out.push(loft(C("shoe-l", f, MAT.sneaker, TONE.canvas), dome(dense(UPPER.map(([x, lo, hi, hw, e]) => xSlice(x, lo, hi, hw, e)), 2), -1, 0.004, 1)));
  out.push(loft(C("toe-cap-l", t, MAT.sneaker, TONE.rubber), dome(TOE_CAP.map(([x, lo, hi, hw, e, d]) => xSlice(x, lo, hi, hw, e, d, 0)), 1, 0.005, 2)));
  out.push(loft(C("shoe-tongue-l", f, MAT.sneaker, TONE.canvas, "detail"), [xSlice(0.012, 0.085, 0.112, 0.019, 2.6), xSlice(0.04, 0.078, 0.099, 0.02, 2.6), xSlice(0.075, 0.062, 0.079, 0.02, 2.6), xSlice(0.1, 0.054, 0.068, 0.018, 2.4)]));
  const laces: [number, number][] = [[0.03, 0.1], [0.05, 0.091], [0.07, 0.081], [0.09, 0.072]];
  laces.forEach(([x, y], i) => out.push({ ...tube(C(`shoe-lace-${i + 1}-l`, f, MAT.sneaker, TONE.lace, "detail"), [[x - 0.004, G + y - 0.006, -0.02], [x, G + y + 0.0015, 0], [x + 0.004, G + y - 0.006, 0.02]], [0.0022, 0.0022, 0.0022]), texture: true }));
  return out;
}

/* ================================================================ arms (a down the bone, f ahead, l left) */

/** 🧥 The left sleeve on the upper arm: the seam on the deltoid, the relaxed sleeve easing to the elbow. */
export const SLEEVE_L: St[] = [
  sa("arm-l", -0.03, -0.003, -0.008, 0.038, 0.046, 2),
  sa("arm-l", 0.006, -0.003, 0, 0.05, 0.056, 2),
  sa("arm-l", 0.05, -0.002, 0.004, 0.054, 0.059, 2),
  sa("arm-l", 0.11, 0.0, 0.004, 0.053, 0.056, 2),
  sa("arm-l", 0.19, 0.0, 0.003, 0.05, 0.052, 2),
  sa("arm-l", 0.26, -0.003, 0.002, 0.048, 0.05, 2),
  sa("arm-l", 0.318, -0.006, 0.002, 0.047, 0.049, 2),
];
/** 🧥 The left sleeve on the forearm with two soft folds above the rib cuff. */
export const FORESLEEVE_L: St[] = [
  sa("forearm-l", -0.034, -0.008, 0.002, 0.046, 0.048, 2),
  sa("forearm-l", 0.03, -0.003, 0.002, 0.045, 0.047, 2),
  sa("forearm-l", 0.09, 0, 0.002, 0.043, 0.045, 2),
  sa("forearm-l", 0.14, 0, 0.001, 0.041, 0.042, 2),
  sa("forearm-l", 0.162, 0, 0.001, 0.0385, 0.039, 2),
  sa("forearm-l", 0.178, 0, 0.001, 0.041, 0.0415, 2),
  sa("forearm-l", 0.194, 0, 0.001, 0.0375, 0.0375, 2),
  sa("forearm-l", 0.206, 0, 0.001, 0.0385, 0.038, 2),
  sa("forearm-l", 0.214, 0, 0.001, 0.034, 0.032, 2),
];
/** 🧶 The rib cuff round the wrist. */
export const CUFF_L: St[] = [sa("forearm-l", 0.208, 0, 0.001, 0.0325, 0.0295, 2.2), sa("forearm-l", 0.228, 0, 0, 0.0305, 0.027, 2.2), sa("forearm-l", 0.25, 0, 0, 0.0295, 0.0255, 2.2)];

/* ================================================================ hands (a from the wrist, f towards the thumb, l left; the left palm faces -l) */

export type Curl = { mcp: number; pip: number; dip: number; spread: number; thumb: number; thumbDrop?: number };
const fingerPath = (from: V3, bends: number[], lens: number[], spread: number): V3[] => {
  const out: V3[] = [from];
  let p = from;
  let ang = 0;
  for (let i = 0; i < lens.length; i++) {
    ang += bends[i]!;
    const c = Math.cos(rad(ang));
    const s = Math.sin(rad(ang));
    const d: V3 = [c * Math.cos(rad(spread)), c * Math.sin(rad(spread)), -s];
    p = [p[0] + d[0] * lens[i]!, p[1] + d[1] * lens[i]!, p[2] + d[2] * lens[i]!];
    out.push(p);
  }
  return out;
};
/** ✋ A left hand: palm, thenar and hypothenar pads, four three-phalanx fingers and the thumb, curled by `k` towards the palm. */
const FINGERS: [string, number, number, number, number][] = [["index", 0.093, 0.026, 0.94, 0.0082], ["middle", 0.097, 0.009, 1.0, 0.0084], ["ring", 0.094, -0.008, 0.97, 0.008], ["little", 0.086, -0.024, 0.78, 0.0072]];
/** 🦴 The joint centres of the four fingers (knuckle, two finger joints, tip) and of the thumb of a left hand curled by `k`. */
export function fingerJoints(k: Curl): Record<string, V3[]> {
  const out: Record<string, V3[]> = {};
  FINGERS.forEach(([name, a, f, s], i) => { out[name] = fingerPath(pa("hand-l", a, f, -0.002), [k.mcp, k.pip, k.dip], [0.041 * s, 0.025 * s, 0.02 * s], (1.5 - i) * k.spread); });
  const t0 = pa("hand-l", 0.014, 0.022, -0.006);
  const ta = rad(k.thumb);
  const m = k.thumbDrop ?? 1;
  const t1: V3 = [t0[0] + 0.036 * Math.cos(ta), t0[1] + 0.036 * Math.sin(ta), t0[2] - 0.009 * m];
  const t2: V3 = [t1[0] + 0.031 * Math.cos(ta) * 1.05, t1[1] + 0.031 * Math.sin(ta) * 0.85, t1[2] - 0.006 * m];
  const t3: V3 = [t2[0] + 0.025 * Math.cos(ta) * 1.1, t2[1] + 0.025 * Math.sin(ta) * 0.7, t2[2] - 0.004 * m];
  out.thumb = [t0, t1, t2, t3];
  return out;
}
export function handL(k: Curl, prefix = "hand-l"): Json[] {
  const c = (id: string): Json => C(id, "hand-l", MAT.skin, TONE.hand);
  const out: Json[] = [];
  const palm: St[] = [sa("hand-l", -0.014, 0, 0, 0.0165, 0.026, 2.3), sa("hand-l", 0.02, 0.002, -0.001, 0.0145, 0.033, 2.4), sa("hand-l", 0.055, 0.002, -0.0015, 0.0135, 0.037, 2.5), sa("hand-l", 0.086, 0, -0.002, 0.0115, 0.038, 2.5), sa("hand-l", 0.096, -0.001, -0.002, 0.0105, 0.036, 2.4)];
  out.push(loft(c(prefix), dense(palm, 2)));
  out.push(ellipsoid(c(`${prefix}-thenar`), [0.028, 0.015, 0.0105], pa("hand-l", 0.034, 0.019, -0.008), aboutW(-14)));
  out.push(ellipsoid(c(`${prefix}-hypothenar`), [0.03, 0.011, 0.0085], pa("hand-l", 0.045, -0.026, -0.007)));
  const fingers: [string, number, number, number, number][] = [["index", 0.093, 0.026, 0.94, 0.0082], ["middle", 0.097, 0.009, 1.0, 0.0084], ["ring", 0.094, -0.008, 0.97, 0.008], ["little", 0.086, -0.024, 0.78, 0.0072]];
  fingers.forEach(([name, a, f, s, r], i) => {
    const pts = fingerPath(pa("hand-l", a, f, -0.002), [k.mcp, k.pip, k.dip], [0.041 * s, 0.025 * s, 0.02 * s], (1.5 - i) * k.spread);
    out.push(capsule(c(`${prefix}-${name}-1`), pts[0]!, pts[1]!, r, r * 0.93));
    out.push(capsule(c(`${prefix}-${name}-2`), pts[1]!, pts[2]!, r * 0.92, r * 0.86));
    out.push(capsule(c(`${prefix}-${name}-3`), pts[2]!, pts[3]!, r * 0.84, r * 0.76));
  });
  const t0 = pa("hand-l", 0.014, 0.022, -0.006);
  const ta = rad(k.thumb);
  const m = k.thumbDrop ?? 1;
  const t1: V3 = [t0[0] + 0.036 * Math.cos(ta), t0[1] + 0.036 * Math.sin(ta), t0[2] - 0.009 * m];
  const t2: V3 = [t1[0] + 0.031 * Math.cos(ta) * 1.05, t1[1] + 0.031 * Math.sin(ta) * 0.85, t1[2] - 0.006 * m];
  const t3: V3 = [t2[0] + 0.025 * Math.cos(ta) * 1.1, t2[1] + 0.025 * Math.sin(ta) * 0.7, t2[2] - 0.004 * m];
  out.push(capsule(c(`${prefix}-thumb-1`), t0, t1, 0.0115, 0.0098));
  out.push(capsule(c(`${prefix}-thumb-2`), t1, t2, 0.0092, 0.0085));
  out.push(capsule(c(`${prefix}-thumb-3`), t2, t3, 0.0083, 0.0074));
  return out;
}
/** 🤚 The left hand hanging relaxed; ✊ the right hand closed round the spoon handle, the thumb pressing on it. */
export const CURL_L: Curl = { mcp: 22, pip: 32, dip: 16, spread: 2, thumb: 26 };
export const CURL_R: Curl = { mcp: 62, pip: 76, dip: 42, spread: 0.5, thumb: 14, thumbDrop: 3.2 };

/* ================================================================ head (head bone: a up from the atlanto-occipital joint, f ahead, l left) */

/** 🗣️ Skull and face as slices square to the head bone from the chin to the vertex: [a, back, front, half-width, exponent]. */
export const HEAD: readonly [number, number, number, number, number][] = [
  [-0.07, 0.062, 0.088, 0.012, 2],
  [-0.06, 0.03, 0.094, 0.026, 2],
  [-0.045, -0.004, 0.096, 0.041, 2.1],
  [-0.025, -0.03, 0.1, 0.05, 2.2],
  [0.0, -0.058, 0.098, 0.058, 2.2],
  [0.025, -0.072, 0.096, 0.066, 2.3],
  [0.05, -0.08, 0.1, 0.07, 2.3],
  [0.075, -0.083, 0.098, 0.072, 2.3],
  [0.1, -0.081, 0.09, 0.071, 2.3],
  [0.12, -0.072, 0.074, 0.064, 2.2],
  [0.135, -0.052, 0.05, 0.048, 2.1],
  [0.143, -0.025, 0.022, 0.022, 2],
];
export const headSt = ([a, b, f, hw, e]: readonly number[]): St => sa("head", a!, (b! + f!) / 2, 0, hw!, (f! - b!) / 2, e!);
/** 💇 Short cropped hair: a cap 8 to 14 mm over the skull with a side-swept fringe at the hairline, cut round the ears and tapering into the nape. */
export const HAIR: readonly [number, number, number, number, number][] = [
  [0.154, -0.03, 0.026, 0.028, 2],
  [0.148, -0.062, 0.05, 0.052, 2],
  [0.137, -0.084, 0.066, 0.064, 2.1],
  [0.122, -0.094, 0.082, 0.0725, 2.2],
  [0.105, -0.097, 0.0925, 0.076, 2.25],
  [0.09, -0.096, 0.066, 0.0765, 2.3],
  [0.072, -0.094, 0.03, 0.0755, 2.3],
  [0.052, -0.091, -0.008, 0.0735, 2.3],
  [0.027, -0.084, -0.022, 0.0685, 2.25],
  [0.002, -0.073, -0.028, 0.06, 2.2],
  [-0.02, -0.06, -0.032, 0.048, 2.1],
  [-0.034, -0.052, -0.036, 0.036, 2],
];
/** 🧣 The neck on the neck bone (a up from C7 towards the skull, f ahead). */
export const NECK: St[] = [sa("neck", -0.022, 0.008, 0, 0.054, 0.056, 2.1), sa("neck", 0.03, 0.012, 0, 0.049, 0.051, 2), sa("neck", 0.07, 0.013, 0, 0.0475, 0.05, 2), sa("neck", 0.108, 0.004, 0, 0.049, 0.048, 2)];

function head(): Json[] {
  const out: Json[] = [];
  out.push(loft(C("head", "head", MAT.skin, TONE.skin), dome(dense(HEAD.map(headSt), 2), 1, 0.003, 2)));
  out.push(loft(C("nose", "head", MAT.skin, TONE.skin), [sa("head", 0.044, 0.093, 0, 0.0065, 0.006, 2), sa("head", 0.025, 0.103, 0, 0.0075, 0.0085, 2), sa("head", 0.008, 0.11, 0, 0.0095, 0.009, 2), sa("head", 0.0, 0.106, 0, 0.0145, 0.0075, 2)]));
  out.push(ellipsoid(C("lips", "head", MAT.skin, TONE.lips, "detail"), [0.0085, 0.0058, 0.0205], pa("head", -0.022, 0.0985, 0)));
  out.push(ellipsoid(C("chin", "head", MAT.skin, TONE.skin, "detail"), [0.0125, 0.0075, 0.0165], pa("head", -0.054, 0.088, 0)));
  for (const side of [1, -1]) {
    const s = side > 0 ? "l" : "r";
    out.push(ellipsoid(C(`eye-${s}`, "head", MAT.skin, TONE.eye, "detail"), [0.006, 0.0055, 0.0105], pa("head", 0.037, 0.088, side * 0.031)));
    out.push(tube(C(`brow-${s}`, "head", MAT.hair, TONE.brow, "detail"), [pa("head", 0.05, 0.094, side * 0.014), pa("head", 0.054, 0.095, side * 0.03), pa("head", 0.051, 0.088, side * 0.048)], [0.0036, 0.004, 0.0032]));
    out.push(ellipsoid(C(`ear-${s}`, "head", MAT.skin, TONE.skin), [0.029, 0.016, 0.0075], pa("head", 0.03, 0.002, side * 0.069), compose(aboutU(side * -14), aboutW(14))));
  }
  out.push(loft(C("hair", "head", MAT.hair, TONE.hair), dome(dense(HAIR.map(headSt), 2), -1, 0.006, 2)));
  out.push(loft(C("neck", "neck", MAT.skin, TONE.skin), NECK));
  return out;
}

/* ================================================================ the hob base unit (counter bone: x into the unit, y up, z left) */

const K = "counter";
const U = UNIT;
/** 🗄️ A board of the unit: from (x0, y0) to (x1, y1), lateral from z0 to z1. */
const board = (id: string, x0: number, x1: number, y0: number, y1: number, z0: number, z1: number, material: string, tone: number, role = "body", radius = 0): Json => box(C(id, K, material, tone, role), [x1 - x0, y1 - y0, z1 - z0], [(x0 + x1) / 2, (y0 + y1) / 2, (z0 + z1) / 2], undefined, radius);
/** 🚪 The three drawer fronts: [id, bottom, top, handle height]. */
export const FRONTS: readonly [string, number, number, number][] = [["drawer-front-bottom", U.plinth, 0.432, 0.392], ["drawer-front-middle", 0.432, 0.722, 0.682], ["drawer-front-top", 0.722, U.plinth + U.carcass, 0.792]];
/** 🔥 The four cooking zones: [id, x, z, radius]. */
export const ZONES: readonly [string, number, number, number][] = [["zone-front-right", U.zoneFront, -U.zoneZ, 0.105], ["zone-rear-right", U.zoneRear, -U.zoneZ, 0.0725], ["zone-front-left", U.zoneFront, U.zoneZ, 0.0725], ["zone-rear-left", U.zoneRear, U.zoneZ, 0.105]];
const HW = U.width / 2;
const CUT = { x0: 0.055, x1: 0.545, z: 0.28 } as const;

export function unit(): Json[] {
  const out: Json[] = [];
  const top = U.plinth + U.carcass;
  const c0 = U.front + U.door + 0.001;
  const j = 0.0015;
  for (const side of [1, -1]) out.push(board(`carcass-side-${side > 0 ? "l" : "r"}`, c0 - j, U.depth, U.plinth, top + j, side > 0 ? HW - 0.016 : -HW, side > 0 ? HW : -HW + 0.016, MAT.board, TONE.carcass));
  out.push(board("carcass-bottom", c0 - j, U.depth, U.plinth, U.plinth + 0.016, -HW + 0.016 - j, HW - 0.016 + j, MAT.board, TONE.carcass));
  out.push(board("carcass-back", U.depth - 0.02, U.depth - 0.012, U.plinth + 0.016 - j, top + j, -HW + 0.016 - j, HW - 0.016 + j, MAT.board, TONE.carcass));
  out.push(board("carcass-rail-front", c0, c0 + 0.1, top - 0.04, top - 0.024, -HW + 0.016, HW - 0.016, MAT.board, TONE.carcass));
  out.push(board("carcass-rail-back", U.depth - 0.12, U.depth - 0.02, top - 0.04, top - 0.024, -HW + 0.016, HW - 0.016, MAT.board, TONE.carcass));
  for (const [id, y0, y1, hy] of FRONTS) {
    out.push(board(id, U.front, c0, y0, y1, -HW, HW, MAT.board, TONE.front));
    out.push(capsule(C(`${id.replace("drawer-front", "handle")}`, K, MAT.steel, TONE.steel, "detail"), [U.front - 0.032, hy, -0.16], [U.front - 0.032, hy, 0.16], 0.006, 0.006));
    for (const side of [1, -1]) out.push(capsule(C(`${id.replace("drawer-front", "handle-post")}-${side > 0 ? "l" : "r"}`, K, MAT.steel, TONE.steel, "detail"), [U.front - 0.03, hy, side * 0.128], [U.front + 0.002, hy, side * 0.128], 0.005, 0.005));
  }
  out.push(board("plinth", U.plinthBack, U.plinthBack + 0.016, 0, U.plinth, -HW, HW, MAT.board, TONE.plinth));
  for (const side of [1, -1]) out.push(board(`plinth-side-${side > 0 ? "l" : "r"}`, U.plinthBack + 0.016, U.depth, 0, U.plinth, side > 0 ? HW - 0.016 : -HW, side > 0 ? HW : -HW + 0.016, MAT.board, TONE.plinth));
  out.push(extrude(C("worktop", K, MAT.board, TONE.worktop), "zx", [[-HW, 0], [HW, 0], [HW, U.depth], [-HW, U.depth]], top, U.top, { holes: [[[-CUT.z, CUT.x0], [CUT.z, CUT.x0], [CUT.z, CUT.x1], [-CUT.z, CUT.x1]]] }));
  out.push(board("hob-plate", U.hobX0, U.hobX0 + U.hobDepth, U.top, U.top + U.glass, -U.hobWidth / 2, U.hobWidth / 2, MAT.glass, TONE.hob, "body", 0.002));
  out.push(board("hob-tray", CUT.x0 + 0.004, CUT.x1 - 0.004, U.top - 0.046, U.top, -CUT.z + 0.004, CUT.z - 0.004, MAT.glass, TONE.steel));
  for (const [id, x, z, r] of ZONES) out.push(revolve(C(id, K, MAT.glass, TONE.zone, "detail"), [[0, r - 0.004], [0.001, r - 0.004], [0.001, r], [0, r]], [x, U.top + U.glass - 0.0002, z], [0, 1, 0], { texture: true }));
  out.push(board("hob-controls", U.hobX0 + 0.018, U.hobX0 + 0.04, U.top + U.glass, U.top + U.glass + 0.0005, -0.09, 0.09, MAT.glass, TONE.zone, "detail"));
  out[out.length - 1].texture = true;
  return out;
}

/* ================================================================ pot (pot bone: origin at the centre of the base, y up) and spoon */

/** 🍲 The wall of the pot as a profile [height, radius]: 6 mm sandwich base of 200 mm, 10 mm bottom radius, 1 mm wall, a rolled pouring rim. */
export const POT_WALL: [number, number][] = [[0, 0], [0, 0.1], [0.006, 0.1], [0.0095, 0.1068], [0.018, 0.11], [0.126, 0.11], [0.13, 0.116], [0.1312, 0.1152], [0.1275, 0.109], [0.018, 0.109], [0.0105, 0.1062], [0.007, 0.0995], [0.007, 0]];
export function pot(): Json[] {
  const out: Json[] = [];
  out.push(revolve(C("pot-shell", "pot", MAT.steel, TONE.steel), POT_WALL, [0, 0, 0], [0, 1, 0]));
  out.push(revolve(C("pot-water", "pot", MAT.water, TONE.water), [[0.007, 0], [0.007, 0.0995], [0.0105, 0.1062], [0.018, 0.109], [POT.water, 0.109], [POT.water, 0]], [0, 0, 0], [0, 1, 0]));
  for (const side of [1, -1]) {
    const pts: V3[] = [[-0.036, 0.106, side * 0.104], [-0.037, 0.107, side * 0.128], [-0.026, 0.108, side * 0.144], [0, 0.108, side * 0.149], [0.026, 0.108, side * 0.144], [0.037, 0.107, side * 0.128], [0.036, 0.106, side * 0.104]];
    out.push(tube(C(`pot-handle-${side > 0 ? "l" : "r"}`, "pot", MAT.steel, TONE.steel), pts, pts.map(() => 0.0045)));
  }
  return out;
}
export function spoon(): Json[] {
  const out: Json[] = [];
  const handle: St[] = [st(-SPOON.back + 0.004, 0, 0, 0.0056, 0.0056), st(-0.04, 0, 0, 0.006, 0.006), st(0.04, 0, 0, 0.0058, 0.0058), st(0.11, 0, 0, 0.0054, 0.005), st(0.15, 0, 0, 0.0062, 0.0042), st(0.166, 0, 0, 0.009, 0.004)];
  out.push(loft(C("spoon-handle", "spoon", MAT.wood, TONE.wood), dome(handle, -1, 0.004, 2)));
  const bowl: St[] = [st(0.156, 0, 0, 0.008, 0.004), st(0.166, 0, 0, 0.0165, 0.0046), st(0.18, 0, 0, 0.0215, 0.005), st(0.193, 0, 0, 0.021, 0.005), st(0.205, 0, 0, 0.0165, 0.0045), st(0.2125, 0, 0, 0.009, 0.004)];
  out.push(loft(C("spoon-bowl", "spoon", MAT.wood, TONE.wood), dome(bowl, 1, 0.0025, 1)));
  out.push(ellipsoid(C("spoon-hollow", "spoon", MAT.wood, 0.55, "detail"), [0.021, 0.0012, 0.0155], [0.1855, 0.0046, 0], undefined, { texture: true }));
  return out;
}

/* ================================================================ the body list */

export function figure50(): Json[] {
  const out: Json[] = [];
  out.push(loft(C("hoodie", "torso", MAT.fleece, TONE.hoodie), dense(hoodieStations(), 2), [true, true]));
  const hem: St[] = [skirtSt([0.832, -0.128, 0.086, 0.171, 2.45], 0, 0.003), skirtSt([0.848, -0.133, 0.091, 0.177, 2.45], 0, 0.004), skirtSt([0.868, -0.137, 0.096, 0.182, 2.45], 0, 0.0035)];
  out.push(loft(C("hem-rib", "torso", MAT.fleece, TONE.rib, "detail"), hem));
  out.push(loft(C("hood", "torso", MAT.fleece, TONE.hoodie), dome(dense(HOOD.map((s) => torsoSt(s)), 2), -1, 0.01, 2)));
  out.push(loft(C("hood-rim", "torso", MAT.fleece, TONE.rib, "detail"), HOOD_RIM.map((s) => torsoSt(s)).map((q) => ({ ...q, n: q.n - 12 }))));
  out.push(loft(C("kangaroo-pocket", "torso", MAT.fleece, TONE.pocket, "detail"), POCKET.map((s) => torsoSt(s))));
  for (const side of [1, -1]) {
    const cord: V3[] = [onHoodie(1.44, side * 0.03, 1, 0.006), onHoodie(1.4, side * 0.034, 1, 0.004), onHoodie(1.34, side * 0.038, 1, 0.004), onHoodie(1.285, side * 0.04, 1, 0.004)];
    out.push({ ...tube(C(`drawstring-${side > 0 ? "l" : "r"}`, "torso", MAT.fleece, TONE.cord, "detail"), cord, cord.map(() => 0.003)), slender: true });
    out.push(ellipsoid(C(`drawstring-tip-${side > 0 ? "l" : "r"}`, "torso", MAT.fleece, TONE.cord, "detail"), [0.009, 0.0042, 0.0042], onHoodie(1.275, side * 0.04, 1, 0.004), undefined, { texture: true }));
  }
  out.push(loft(C("hips", "pelvis", MAT.denim, TONE.jeans), dense(seatStations(), 2), [true, true]));
  out.push(loft(C("thigh-l", "thigh-l", MAT.denim, TONE.jeans), dense(THIGH_L, 2)));
  out.push(ellipsoid(C("knee-l", "shank-l", MAT.denim, TONE.jeans, "detail"), [0.024, 0.024, 0.022], pa("shank-l", -0.012, 0.033, 0)));
  out.push(loft(C("shank-l", "shank-l", MAT.denim, TONE.jeans), dense(SHANK_L, 2)));
  out.push(loft(C("jeans-hem-l", "shank-l", MAT.denim, TONE.jeans, "detail"), HEM_L));
  out.push(loft(C("sleeve-l", "arm-l", MAT.fleece, TONE.hoodie), dome(dome(dense(SLEEVE_L, 2), -1, 0.014, 2), 1, 0.03, 3)));
  out.push(loft(C("forearm-l", "forearm-l", MAT.fleece, TONE.hoodie), dome(dense(FORESLEEVE_L, 1), -1, 0.024, 2)));
  out.push(loft(C("cuff-rib-l", "forearm-l", MAT.fleece, TONE.rib, "detail"), CUFF_L));
  out.push(...sneaker());
  const leftOnly = out.filter((q) => /-l(-|$)/.test(q.id) && !/^drawstring/.test(q.id));
  for (const s of leftOnly) out.push(mirrored(s, toRight));
  out.push(...handL(CURL_L));
  out.push(...handL(CURL_R).map((s) => mirrored(s, toRight)));
  out.push(...head());
  return out;
}
export function body50(): Json[] {
  return [...figure50(), ...unit(), ...pot(), ...spoon()];
}
void st;
