/** 🤰 The 1:50 body of `humans/pregnant-person`, stated section by section: a woman of 1.66 m in week 36 (71 kg), standing in leather ankle boots with a 25 mm block heel, in an oversized wool-blend knit sweater with dropped shoulders, rib hem, rib cuffs and a crew neck, the knit stretched over the abdomen, black maternity leggings, and a shoulder-length bob tucked behind the ears.
 *
 * Every station is anatomical (`sa`, `pa`): `a` along the bone from its joint, `f` ahead of the bone line (up for the abdomen bone and the foot), `l` to the left. The trunk, seat and abdomen tables are the slices of `r3-pregnant-person-forms.ts`.
 * @see ./r3-pregnant-person-kit.ts
 * @see ./r3-humans-pregnant-person.md */
import type { Json } from "./volumes-lib.ts";
import { D, FRAMES, L, W, aboutU, aboutV, aboutW, capsule, compose, dense, dome, ellipsoid, extrude, loft, mirrored, pa, sa, toRight, tube, type St, type V3 } from "./r3-pregnant-person-kit.ts";
import { BELLY, HEM_TILT, SEAT, SWEATER, SWEATER_SKIRT, bellyRow, pelvisRow, torsoAF, torsoRow, type Slice } from "./r3-pregnant-person-forms.ts";
import { RIG } from "./r3-pregnant-person-data.ts";

export const TONE = { sweater: 0.6, rib: 0.5, belly: 0.56, leggings: 0.85, boot: 0.7, sole: 0.9, heel: 0.82, zip: 0.3, skin: 0.25, hand: 0.3, lips: 0.42, eye: 0.9, brow: 0.6, hair: 0.72, bone: 0.92, joint: 0.6, mass: 0.5, fetus: 0.8 } as const;
export const MAT = { knit: "knit-wool-sweater", leggings: "stretch-leggings", boot: "leather-ankle-boot", skin: "skin", hair: "hair", sock: "cotton-sock", cotton: "cotton-underwear", bone: "bone" } as const;
export const C = (id: string, bone: string, material: string, tone: number, role = "body"): Json => ({ id, bone, material, tone, role });
export const G = -RIG.ankle;
const rad = (d: number): number => (d * Math.PI) / 180;
const deg = (r: number): number => (r * 180) / Math.PI;

/* ================================================================ trunk */

/** 🧶 A trunk slice as a torso station. */
export const torsoSt = (s: Slice, grow = 0): St => { const [a, f, hw, hd, e] = torsoRow(s); return sa("torso", a, f, 0, hw + grow, hd + grow, e); };
/** 🪢 A slice that hangs from the pelvis (the sweater skirt over the seat) as a torso station: centre and section plane carried by the pelvis of the default pose, tilted `tilt` degrees (negative: higher at the front). */
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
/** 🧥 The whole sweater body: skirt over the seat, trunk, shoulders, neckline. */
export const sweaterStations = (grow = 0): St[] => [...SWEATER_SKIRT.map((s, i) => skirtSt(s, i === 0 ? HEM_TILT : i === 1 ? HEM_TILT / 2 : 0, grow)), ...SWEATER.map((s) => torsoSt(s, grow))];
/** 🫃 The abdomen cover as belly stations. */
export const bellyStations = (grow = 0): St[] => BELLY.map((c) => { const [a, f, hw, hh, e] = bellyRow(c); return sa("belly", a, f, 0, hw + grow, hh + grow, e); });
/** 🩳 The leggings seat as pelvis stations. */
export const seatStations = (grow = 0): St[] => SEAT.map((s) => { const [a, f, hw, hd, e] = pelvisRow(s); return sa("pelvis", a, f, 0, hw + grow, hd + grow, e); });

/** 🔘 A point on the sweater front (+1) or back (-1) of the upright woman at height `y`, `l` to the left, `off` proud of the knit, torso-local. */
export function onSweater(y: number, l: number, side: 1 | -1, off = 0): V3 {
  let i = 0;
  while (i + 2 < SWEATER.length && SWEATER[i + 1]![0] < y) i++;
  const a = SWEATER[i]!;
  const b = SWEATER[i + 1]!;
  const t = Math.min(1, Math.max(0, (y - a[0]) / (b[0] - a[0])));
  const m = (k: number): number => a[k]! + (b[k]! - a[k]!) * t;
  const [back, front, rw, e] = [m(1), m(2), m(3), m(4)];
  const k = Math.pow(Math.max(0, 1 - Math.pow(Math.min(1, Math.abs(l) / rw), e)), 1 / e);
  const x = (back + front) / 2 + side * (k * (front - back) / 2 + off);
  const [ta, tf] = torsoAF(x, y);
  return pa("torso", ta, tf, l);
}

/* ================================================================ legs (thigh, shank: a down the bone, f ahead, l left) */

export const THIGH_L: St[] = [
  sa("thigh-l", -0.06, -0.01, 0.008, 0.088, 0.095, 2.2),
  sa("thigh-l", 0.03, -0.004, 0.002, 0.089, 0.092, 2.2),
  sa("thigh-l", 0.1, 0.004, -0.004, 0.085, 0.088, 2.1),
  sa("thigh-l", 0.18, 0.008, -0.006, 0.078, 0.082, 2.05),
  sa("thigh-l", 0.26, 0.008, -0.005, 0.069, 0.074, 2),
  sa("thigh-l", 0.33, 0.006, -0.003, 0.059, 0.064, 2),
  sa("thigh-l", 0.38, 0.007, -0.002, 0.053, 0.057, 2),
  sa("thigh-l", 0.418, 0.005, 0, 0.0515, 0.0545, 2),
];
export const SHANK_L: St[] = [
  sa("shank-l", -0.035, 0.002, 0, 0.04, 0.042, 2),
  sa("shank-l", -0.004, 0.003, 0, 0.0455, 0.0475, 2),
  sa("shank-l", 0.04, -0.002, 0, 0.048, 0.0505, 2),
  sa("shank-l", 0.1, -0.012, 0.002, 0.052, 0.057, 2),
  sa("shank-l", 0.15, -0.014, 0.002, 0.053, 0.058, 2),
  sa("shank-l", 0.2, -0.01, 0.001, 0.048, 0.052, 2),
  sa("shank-l", 0.26, -0.005, 0, 0.04, 0.044, 2),
  sa("shank-l", 0.31, -0.002, 0, 0.034, 0.038, 2),
  sa("shank-l", 0.365, 0, 0, 0.0305, 0.034, 2),
];

/* ================================================================ ankle boot (foot bone horizontal: a ahead of the ankle, f up, ground at f = G) */

/** 👢 The shaft as horizontal slices round the ankle: [height above the ground, x centre, half-length, half-width]. */
export const SHAFT: readonly [number, number, number, number][] = [
  [0.028, -0.012, 0.057, 0.036],
  [0.07, -0.01, 0.055, 0.0375],
  [0.1, -0.008, 0.05, 0.0365],
  [0.13, -0.006, 0.047, 0.037],
  [0.152, -0.005, 0.0475, 0.0385],
];
/** 👞 The vamp over instep and forefoot as cross slices: [x ahead of the ankle, bottom, top (above the ground), half-width, exponent]. */
export const VAMP: readonly [number, number, number, number, number][] = [
  [0.0, 0.02, 0.12, 0.039, 2.4],
  [0.03, 0.016, 0.11, 0.042, 2.5],
  [0.06, 0.013, 0.092, 0.045, 2.6],
  [0.09, 0.012, 0.077, 0.047, 2.7],
  [0.115, 0.012, 0.068, 0.048, 2.8],
  [0.135, 0.012, 0.064, 0.048, 2.8],
];
/** 🥾 The toe box on the toe bone (origin on the ground under the ball): [x, bottom, top, half-width, exponent, drift towards the big toe]. */
export const TOE_BOX: readonly [number, number, number, number, number, number][] = [
  [-0.012, 0.012, 0.064, 0.048, 2.8, 0],
  [0.02, 0.012, 0.058, 0.047, 2.7, -0.003],
  [0.045, 0.012, 0.05, 0.042, 2.5, -0.006],
  [0.062, 0.013, 0.042, 0.033, 2.3, -0.009],
  [0.072, 0.016, 0.034, 0.02, 2.1, -0.011],
];
/** 🛞 The outsole of the forefoot under the toe bone, plan outline [x, l] of the left boot. */
const TOE_SOLE: [number, number][] = [[-0.012, 0.05], [0.02, 0.049], [0.045, 0.044], [0.062, 0.035], [0.074, 0.02], [0.078, 0.0], [0.075, -0.015], [0.066, -0.03], [0.05, -0.043], [0.025, -0.05], [-0.012, -0.051]];
/** 👠 The block heel in plan [x, l]: back rounded, breast straight. */
const HEEL_PLAN: [number, number][] = [[-0.004, 0.03], [-0.035, 0.031], [-0.055, 0.028], [-0.066, 0.018], [-0.069, 0.0], [-0.066, -0.018], [-0.055, -0.028], [-0.035, -0.031], [-0.004, -0.03]];

const xSlice = (bone: string, x: number, lo: number, hi: number, hw: number, e: number, l = 0, ground = G): St => ({ at: [x, ground + (lo + hi) / 2, l], rw: hw, rv: (hi - lo) / 2, e, n: 0 });
const hSlice = (bone: string, y: number, x: number, hx: number, hw: number, e = 2.2): St => ({ ...sa(bone, x, G + y, 0, hw, hx, e, 90) });

function boot(): Json[] {
  const f = "foot-l";
  const t = "toe-l";
  const out: Json[] = [];
  out.push(extrude(C("boot-heel-l", f, MAT.boot, TONE.heel), "zx", HEEL_PLAN.map(([x, l]) => [l, x]), G, G + RIG.heel));
  const waist: St[] = [xSlice(f, -0.006, 0.016, 0.03, 0.031, 2.6), xSlice(f, 0.03, 0.007, 0.02, 0.036, 2.8), xSlice(f, 0.07, 0.0, 0.013, 0.044, 3), xSlice(f, 0.135, 0.0, 0.012, 0.0475, 3)];
  out.push(loft(C("boot-sole-l", f, MAT.boot, TONE.sole), dense(waist, 2)));
  out.push(extrude(C("boot-toe-sole-l", t, MAT.boot, TONE.sole), "zx", TOE_SOLE.map(([x, l]) => [l, x]), 0, RIG.ball));
  out.push(loft(C("boot-l", f, MAT.boot, TONE.boot), dome(dense(SHAFT.map(([y, x, hx, hw]) => hSlice(f, y, x, hx, hw, 2.4)), 2), -1, 0.004, 1), [true, false]));
  out.push(loft(C("boot-vamp-l", f, MAT.boot, TONE.boot), dense(VAMP.map(([x, lo, hi, hw, e]) => xSlice(f, x, lo, hi, hw, e)), 2)));
  out.push(loft(C("boot-toe-l", t, MAT.boot, TONE.boot), dome(TOE_BOX.map(([x, lo, hi, hw, e, d]) => xSlice(t, x, lo, hi, hw, e, d, 0)), 1, 0.005, 2)));
  out.push(loft(C("boot-rim-l", f, MAT.boot, TONE.zip, "detail"), [hSlice(f, 0.146, -0.005, 0.0495, 0.0405, 2.4), hSlice(f, 0.155, -0.005, 0.0495, 0.0405, 2.4)]));
  const zip: V3[] = [[-0.012, G + 0.035, -0.0372], [-0.011, G + 0.09, -0.0378], [-0.009, G + 0.148, -0.0392]];
  out.push({ ...tube(C("boot-zip-l", f, MAT.boot, TONE.zip, "detail"), zip, [0.0022, 0.0022, 0.0022]), slender: true });
  out.push(ellipsoid(C("boot-zip-pull-l", f, MAT.boot, TONE.zip, "detail"), [0.004, 0.009, 0.0025], [-0.009, G + 0.138, -0.042], undefined, { texture: true }));
  return out;
}

/* ================================================================ arms (a down the bone, f ahead, l left) */

/** 🧶 The left sleeve on the upper arm: dropped shoulder seam on the deltoid, the roomy oversized sleeve easing to the elbow. */
export const SLEEVE_L: St[] = [
  sa("arm-l", -0.006, -0.003, -0.004, 0.05, 0.057, 2.2),
  sa("arm-l", 0.04, -0.002, 0.004, 0.064, 0.066, 2.1),
  sa("arm-l", 0.11, 0.0, 0.004, 0.062, 0.064, 2),
  sa("arm-l", 0.19, 0.0, 0.003, 0.058, 0.06, 2),
  sa("arm-l", 0.26, -0.003, 0.002, 0.055, 0.057, 2),
  sa("arm-l", 0.315, -0.006, 0.002, 0.053, 0.055, 2),
];
/** 🧶 The left sleeve on the forearm, pushed up into soft folds above the rib cuff. */
export const FORESLEEVE_L: St[] = [
  sa("forearm-l", -0.035, -0.008, 0.002, 0.045, 0.047, 2),
  sa("forearm-l", 0.03, -0.003, 0.002, 0.05, 0.052, 2),
  sa("forearm-l", 0.09, 0, 0.002, 0.048, 0.05, 2),
  sa("forearm-l", 0.14, 0, 0.001, 0.047, 0.048, 2),
  sa("forearm-l", 0.158, 0, 0.001, 0.043, 0.044, 2),
  sa("forearm-l", 0.172, 0, 0.001, 0.046, 0.047, 2),
  sa("forearm-l", 0.188, 0, 0.001, 0.041, 0.041, 2),
  sa("forearm-l", 0.2, 0, 0.001, 0.042, 0.042, 2),
  sa("forearm-l", 0.21, 0, 0.001, 0.037, 0.035, 2),
];
/** 🧶 The rib cuff round the wrist. */
export const CUFF_L: St[] = [sa("forearm-l", 0.204, 0, 0.001, 0.034, 0.031, 2.2), sa("forearm-l", 0.226, 0, 0, 0.0315, 0.028, 2.2), sa("forearm-l", 0.248, 0, 0, 0.03, 0.026, 2.2)];

/* ================================================================ hands (a from the wrist, f towards the thumb, l left; the left palm faces -l) */

export type Curl = { mcp: number; pip: number; dip: number; spread: number; thumb: number };
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
  const dir = (len: number, drop: number): V3 => [len * Math.cos(ta), len * Math.sin(ta), -drop];
  const t1: V3 = [t0[0] + dir(0.036, 0.009)[0], t0[1] + dir(0.036, 0.009)[1], t0[2] - 0.009];
  const t2: V3 = [t1[0] + dir(0.031, 0.006)[0] * 1.05, t1[1] + dir(0.031, 0.006)[1] * 0.85, t1[2] - 0.006];
  const t3: V3 = [t2[0] + dir(0.025, 0.004)[0] * 1.1, t2[1] + dir(0.025, 0.004)[1] * 0.7, t2[2] - 0.004];
  out.push(capsule(c(`${prefix}-thumb-1`), t0, t1, 0.0115, 0.0098));
  out.push(capsule(c(`${prefix}-thumb-2`), t1, t2, 0.0092, 0.0085));
  out.push(capsule(c(`${prefix}-thumb-3`), t2, t3, 0.0083, 0.0074));
  return out;
}
/** 🤚 The right hand flat on the lower back; 🤲 the left hand cupped under the abdomen. */
export const CURL_R: Curl = { mcp: 6, pip: 9, dip: 6, spread: 3, thumb: 32 };
export const CURL_L: Curl = { mcp: 12, pip: 18, dip: 10, spread: 2.5, thumb: 30 };

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
const headSt = ([a, b, f, hw, e]: readonly number[]): St => sa("head", a!, (b! + f!) / 2, 0, hw!, (f! - b!) / 2, e!);
/** 💇 The shoulder-length bob: full over the crown and the back of the head, a side-swept fringe at the hairline, the sides tucked behind the ears, the ends turning in at the collar. */
export const HAIR: readonly [number, number, number, number, number][] = [
  [0.157, -0.03, 0.028, 0.028, 2],
  [0.15, -0.062, 0.056, 0.052, 2],
  [0.138, -0.085, 0.08, 0.066, 2.1],
  [0.12, -0.095, 0.093, 0.077, 2.2],
  [0.1, -0.099, 0.096, 0.083, 2.2],
  [0.08, -0.1, 0.064, 0.086, 2.25],
  [0.055, -0.1, 0.03, 0.088, 2.3],
  [0.028, -0.099, -0.012, 0.089, 2.35],
  [0.0, -0.097, -0.016, 0.089, 2.35],
  [-0.03, -0.093, -0.022, 0.087, 2.3],
  [-0.056, -0.086, -0.03, 0.082, 2.2],
  [-0.07, -0.078, -0.036, 0.072, 2.1],
];
/** 🧣 The neck on the neck bone (a up from C7 towards the skull, f ahead). */
export const NECK: St[] = [sa("neck", -0.02, 0.008, 0, 0.056, 0.058, 2.1), sa("neck", 0.03, 0.012, 0, 0.051, 0.053, 2), sa("neck", 0.07, 0.013, 0, 0.0495, 0.052, 2), sa("neck", 0.105, 0.004, 0, 0.051, 0.05, 2)];

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

/* ================================================================ the body list */

export function body50(): Json[] {
  const out: Json[] = [];
  out.push(loft(C("sweater", "torso", MAT.knit, TONE.sweater), dense(sweaterStations(), 2), [true, true]));
  const hem: St[] = [skirtSt([0.818, -0.166, 0.097, 0.188, 2.4], HEM_TILT, 0.003), skirtSt([0.836, -0.172, 0.102, 0.194, 2.4], HEM_TILT, 0.005), skirtSt([0.858, -0.172, 0.106, 0.198, 2.4], HEM_TILT, 0.005)];
  out.push(loft(C("sweater-hem-rib", "torso", MAT.knit, TONE.rib, "detail"), hem));
  const collar: St[] = [torsoSt([1.432, -0.091, 0.046, 0.072, 2.1], 0.0), torsoSt([1.446, -0.086, 0.038, 0.065, 2.0], 0.0), torsoSt([1.462, -0.081, 0.03, 0.06, 2.0], 0.0)].map((q) => ({ ...q, n: q.n - 14 }));
  out.push(loft(C("sweater-collar", "torso", MAT.knit, TONE.rib, "detail"), collar));
  out.push(loft(C("belly-cover", "belly", MAT.knit, TONE.belly), dome(dense(bellyStations(), 2), 1, 0.004, 2), [false, true]));
  out.push(loft(C("leggings-seat", "pelvis", MAT.leggings, TONE.leggings), dense(seatStations(), 2), [true, true], { sectionFaces: ["+u"] }));
  out.push(loft(C("thigh-l", "thigh-l", MAT.leggings, TONE.leggings), dense(THIGH_L, 2)));
  out.push(ellipsoid(C("knee-l", "shank-l", MAT.leggings, TONE.leggings, "detail"), [0.024, 0.026, 0.023], pa("shank-l", -0.012, 0.034, 0)));
  out.push(loft(C("shank-l", "shank-l", MAT.leggings, TONE.leggings), dense(SHANK_L, 2)));
  out.push(loft(C("sleeve-l", "arm-l", MAT.knit, TONE.sweater), dome(dense(SLEEVE_L, 2), -1, 0.008, 2)));
  out.push(loft(C("forearm-l", "forearm-l", MAT.knit, TONE.sweater), dome(dense(FORESLEEVE_L, 1), -1, 0.024, 2)));
  out.push(loft(C("cuff-rib-l", "forearm-l", MAT.knit, TONE.rib, "detail"), CUFF_L));
  out.push(...boot());
  const leftOnly = out.filter((q) => /-l(-|$)/.test(q.id));
  for (const s of leftOnly) out.push(mirrored(s, toRight));
  out.push(...handL(CURL_L));
  out.push(...handL(CURL_R).map((s) => mirrored(s, toRight)));
  out.push(...head());
  return out;
}
void aboutV;
void capsule;
