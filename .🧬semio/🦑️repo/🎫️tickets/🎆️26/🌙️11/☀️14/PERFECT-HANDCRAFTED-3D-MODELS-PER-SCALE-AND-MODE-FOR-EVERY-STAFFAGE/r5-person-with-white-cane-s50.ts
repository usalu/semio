/** 🦯 The 1:50 model of `humans/person-with-white-cane`, stated section by section: a blind woman of 1.68 m and 62 kg (1.70 m in her ankle boots) in a buttoned denim jacket, slim jeans, Chelsea boots and sunglasses, a chin-length bob, a small leather bag worn across the body at the right hip, and a folding long cane of 1.3 m held in the right hand at the midline, index finger along the grip, the roller tip on the ground a good step ahead.
 *
 * Figure coordinates (`H`, `X`, `L`): x ahead, y up, z to the left, the default pose `constant-contact-stance`. Local stations (`st`): u along the bone, v its sagittal normal, w lateral.
 * @see ./r5-person-with-white-cane-kit.ts
 * @see ./r3-humans-person-with-white-cane.md */
import type { Json } from "./volumes-lib.ts";
import { ASSET, D, FRAMES, H, L, X, aboutU, aboutV, aboutW, box, boneDeg, capsule, compose, dense, dome, ellipsoid, extrude, freeLoft, loft, mirrored, part, revolve, st, thOf, toRight, tube, type St, type V3 } from "./r5-person-with-white-cane-kit.ts";

export const TONE = { jacket: 0.46, jacketFar: 0.4, jeans: 0.26, jeansFar: 0.21, boot: 0.13, bootFar: 0.09, sole: 0.06, skin: 0.8, hand: 0.78, handFar: 0.71, hair: 0.24, lens: 0.05, bag: 0.38, strap: 0.3, shirt: 0.92, cane: 0.97, band: 0.5, grip: 0.14, roller: 0.86 } as const;
export const near = (t: number): number => (t === TONE.jacketFar ? TONE.jacket : t === TONE.jeansFar ? TONE.jeans : t === TONE.bootFar ? TONE.boot : t === TONE.handFar ? TONE.hand : t);
export type Row = readonly [number, number, number, number, number];
export const MAT = { skin: "skin-medium", hair: "hair-brown", jacket: "denim-jacket", jeans: "denim-jeans", boot: "leather-boot", shirt: "cotton-shirt", lens: "sunglass-polymer", bag: "bag-leather", cane: "cane-alu-white", grip: "grip-eva", roller: "nylon-roller", bone: "bone" } as const;
export type Common = { id: string; bone: string; material: string; tone: number; role?: string };
export const c = (id: string, bone: string, material: string, tone: number, role = "body"): Common => ({ id, bone, material, tone, role });
export const hs = (bone: string, table: readonly Row[]): St[] => table.map(([y, b, f, rw, e]) => H(bone, y, b, f, rw, e));

/* ---------------------------------------------------------------- stations shared by body, thermal and the drawings */

/** 🧥 The denim jacket on the trunk as horizontal slices from the waistband to the collar seam: [y, back, front, half-width, exponent]. Boxy and short: the band sits on the hip bones, the back is shaped by the shoulder blades, the front by the bust. */
export const JACKET: readonly Row[] = [
  [0.953, -0.12, 0.112, 0.179, 2.5],
  [0.975, -0.121, 0.113, 0.18, 2.5],
  [1.0, -0.118, 0.111, 0.177, 2.5],
  [1.04, -0.108, 0.111, 0.169, 2.45],
  [1.09, -0.1, 0.11, 0.158, 2.4],
  [1.15, -0.1, 0.121, 0.158, 2.4],
  [1.21, -0.105, 0.148, 0.164, 2.4],
  [1.25, -0.109, 0.159, 0.168, 2.4],
  [1.3, -0.112, 0.15, 0.173, 2.35],
  [1.345, -0.112, 0.124, 0.186, 2.3],
  [1.38, -0.104, 0.11, 0.197, 2.3],
  [1.405, -0.094, 0.098, 0.194, 2.25],
  [1.425, -0.08, 0.086, 0.17, 2.2],
  [1.442, -0.064, 0.074, 0.13, 2.1],
  [1.456, -0.052, 0.064, 0.09, 2],
];
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

/** 👖 The hips in the jeans from the crotch to the waistband under the jacket: wide female pelvis, the seat behind. */
export const HIPS: readonly Row[] = [
  [0.822, -0.07, 0.058, 0.108, 2.3],
  [0.85, -0.104, 0.08, 0.166, 2.4],
  [0.88, -0.122, 0.094, 0.184, 2.4],
  [0.92, -0.128, 0.103, 0.186, 2.4],
  [0.96, -0.116, 0.105, 0.176, 2.35],
  [1.0, -0.104, 0.103, 0.163, 2.3],
  [1.04, -0.094, 0.103, 0.153, 2.3],
];
/** 🦵 The left leg of the jeans on the thigh, bone-local: gluteal fold and hamstrings behind, quadriceps in front, tapering to the knee which lies a little inwards of the hip joint. */
export const THIGH_L: St[] = [st(0.0, -0.008, 0.005, 0.088, 0.1, 2.1), st(0.09, -0.004, 0.005, 0.091, 0.097, 2.1), st(0.19, 0.003, 0.002, 0.083, 0.087, 2), st(0.29, 0.006, -0.002, 0.07, 0.074, 2), st(0.36, 0.005, -0.004, 0.06, 0.066, 2), st(0.4116, 0.004, -0.005, 0.057, 0.062, 2)];
/** 🦵 The left leg of the jeans on the shank: knee, calf behind, the slim leg down to the hem over the boot shaft. */
export const SHANK_L: St[] = [st(0.0, 0.004, -0.005, 0.0565, 0.0615, 2), st(0.05, 0.0, -0.005, 0.0555, 0.0605, 2), st(0.12, -0.007, -0.004, 0.0555, 0.0585, 2), st(0.22, -0.006, -0.002, 0.047, 0.0515, 2), st(0.31, -0.003, 0, 0.041, 0.046, 2), st(0.362, 0.0, 0, 0.04, 0.045, 2)];
/** 🥾 The shaft of the left Chelsea boot on the shank, from its top under the hem to below the ankle joint. */
export const SHAFT_L: St[] = [st(0.33, 0.0, 0, 0.036, 0.041, 2), st(0.37, 0.0, 0, 0.035, 0.04, 2), st(0.405, 0.002, 0, 0.036, 0.043, 2), st(0.432, 0.004, 0, 0.035, 0.044, 2)];

/** 🧥 The left sleeve on the upper arm: the sleeve head set into the shoulder inwards of the joint, the deltoid rounding it, the elbow carried a little outwards. */
export const SLEEVE_L: St[] = [st(-0.02, 0.0, -0.028, 0.036, 0.05), st(-0.004, 0.0, -0.012, 0.048, 0.058), st(0.04, 0.002, 0, 0.054, 0.06), st(0.12, 0.003, 0, 0.052, 0.057), st(0.2, 0.001, 0, 0.048, 0.053), st(0.28, -0.002, 0, 0.045, 0.05), st(0.3125, -0.003, 0, 0.044, 0.049)];
/** 💪 The left sleeve on the forearm down to the buttoned cuff, running outwards to the wrist. */
export const CUFF_L: St[] = [st(0.0, -0.003, 0.0, 0.044, 0.049), st(0.06, 0.0, 0, 0.043, 0.047), st(0.14, 0.0, 0, 0.039, 0.041), st(0.2, 0, 0, 0.0355, 0.0365), st(0.234, 0, 0, 0.035, 0.0355)];
export const WRIST_W = 0;

const footX = (bone: string): number => FRAMES[bone.replace("toe", "foot")]!.o[0];
/** 👢 A cross slice of a foot given from its ankle joint: [x ahead of the ankle, bottom, top, half-width, exponent, lateral shift]. */
export const xa = (bone: string, table: readonly (readonly number[])[]): St[] => table.map(([x, b, t, rw, e, z]) => X(bone, footX(bone) + x!, b!, t!, rw!, e ?? 2, z ?? 0));
/** 🥾 The left boot round the foot from the heel counter to the ball: quarters rising to the ankle, the elastic gusset region, the vamp falling to the toe box. */
export const BOOT_REAR: readonly (readonly number[])[] = [
  [-0.0635, 0.03, 0.104, 0.019, 2],
  [-0.055, 0.012, 0.118, 0.03, 2.3],
  [-0.03, 0.008, 0.126, 0.036, 2.3],
  [0.0, 0.008, 0.128, 0.0375, 2.3],
  [0.028, 0.008, 0.12, 0.038, 2.3],
  [0.05, 0.008, 0.099, 0.04, 2.3],
  [0.075, 0.008, 0.081, 0.043, 2.4],
  [0.105, 0.008, 0.067, 0.046, 2.5],
  [0.146, 0.008, 0.055, 0.047, 2.6],
];
export const BOOT_TOE: readonly (readonly number[])[] = [
  [0.126, 0.008, 0.06, 0.047, 2.6, 0],
  [0.162, 0.008, 0.051, 0.045, 2.6, -0.002],
  [0.185, 0.008, 0.042, 0.038, 2.4, -0.004],
  [0.199, 0.009, 0.033, 0.026, 2.2, -0.006],
  [0.2055, 0.011, 0.024, 0.012, 2, -0.007],
];
export const HEEL_PLAN: [number, number][] = [[0, -0.064], [0.018, -0.06], [0.028, -0.046], [0.0325, -0.02], [0.034, 0.012], [-0.034, 0.012], [-0.0325, -0.02], [-0.028, -0.046], [-0.018, -0.06]];
export const SOLE_PLAN: [number, number][] = [[0.036, 0.006], [0.04, 0.045], [0.045, 0.085], [0.049, 0.12], [0.049, 0.146], [-0.049, 0.146], [-0.048, 0.12], [-0.043, 0.085], [-0.038, 0.045], [-0.036, 0.006]];
export const TOE_PLAN: [number, number][] = [[0.049, -0.012], [0.048, 0.016], [0.04, 0.038], [0.025, 0.052], [0.005, 0.0585], [-0.018, 0.058], [-0.036, 0.05], [-0.046, 0.035], [-0.049, 0.016], [-0.049, -0.012]];

/** 🗣️ The head as horizontal slices from the chin to the crown of the scalp: [y, back, front, half-width, exponent]. */
export const HEAD: readonly Row[] = [
  [1.483, 0.064, 0.101, 0.017, 2],
  [1.492, 0.04, 0.107, 0.03, 2],
  [1.505, 0.008, 0.11, 0.04, 2],
  [1.522, -0.022, 0.111, 0.049, 2],
  [1.545, -0.046, 0.111, 0.056, 2],
  [1.568, -0.06, 0.112, 0.0615, 2],
  [1.592, -0.068, 0.109, 0.066, 2],
  [1.618, -0.071, 0.115, 0.07, 2],
  [1.645, -0.069, 0.112, 0.0715, 2],
  [1.668, -0.06, 0.1, 0.064, 2],
  [1.684, -0.045, 0.08, 0.05, 2],
  [1.693, -0.026, 0.055, 0.033, 2],
  [1.698, -0.008, 0.03, 0.015, 2],
];
/** 💇 The bob: hair falling from the crown to the level of the chin, covering the ears, the nape and the temples, parted so that the forehead shows under a side-swept fringe. */
export const HAIR: readonly Row[] = [
  [1.495, -0.083, 0.02, 0.07, 2.05],
  [1.503, -0.091, 0.03, 0.081, 2.05],
  [1.525, -0.095, 0.035, 0.087, 2.0],
  [1.565, -0.096, 0.04, 0.089, 2.0],
  [1.605, -0.093, 0.046, 0.088, 2.0],
  [1.632, -0.09, 0.056, 0.0865, 2.0],
  [1.647, -0.087, 0.098, 0.085, 2.0],
  [1.664, -0.081, 0.112, 0.081, 2.0],
  [1.682, -0.07, 0.099, 0.0735, 2.0],
  [1.695, -0.054, 0.079, 0.062, 2.0],
  [1.703, -0.034, 0.055, 0.046, 2.0],
  [1.708, -0.013, 0.028, 0.026, 2.0],
]
export const NECK: readonly Row[] = [
  [1.425, -0.056, 0.044, 0.056, 2],
  [1.47, -0.05, 0.05, 0.052, 2],
  [1.52, -0.044, 0.056, 0.05, 2],
  [1.565, -0.038, 0.06, 0.05, 2],
];
/** 👃 Where the breath leaves and where the plume rises, in figure coordinates (the emitter anchors of the rig are these points). */
export const OUTLET = { nose: [0.1255, 1.5555, 0] as V3, mouth: [0.1135, 1.531, 0] as V3, crown: [0.008, 1.707, 0] as V3 };

/* ---------------------------------------------------------------- the bag and its strap */

export const BAG = { x: -0.02, y: 0.9, z: -0.226, height: 0.2, length: 0.25, depth: 0.08 } as const;
/** 👜 A point of the standing figure in the frame of the trunk, which carries the bag. */
export const B = (x: number, y: number, z: number): V3 => L("torso", [x, y, z]);
/** 🎗️ The centre line of the strap in figure coordinates: from the front lug of the bag at the right hip up across the chest, over the left shoulder beside the neck and down across the back to the rear lug. */
export const strapPath = (lift: number): V3[] => {
  const front: [number, number][] = [[1.004, -0.228], [1.06, -0.176], [1.13, -0.128], [1.2, -0.076], [1.27, -0.022], [1.33, 0.026], [1.385, 0.066], [1.42, 0.09]];
  const back: [number, number][] = [[1.42, 0.092], [1.385, 0.068], [1.33, 0.03], [1.27, -0.016], [1.2, -0.07], [1.13, -0.124], [1.06, -0.174], [1.004, -0.228]];
  const f = front.map(([y, z], i): V3 => [i === 0 ? 0.075 : jacketX(y, z, 1) + lift, y, z]);
  const b = back.map(([y, z], i): V3 => [i === back.length - 1 ? -0.115 : jacketX(y, z, -1) - lift, y, z]);
  const top: V3[] = [[0.052, 1.441, 0.103], [0.004, 1.4515, 0.106], [-0.044, 1.441, 0.104]];
  return [...f, ...top.map(([x, y, z]): V3 => [x, y + lift - 0.0035, z]), ...b];
};
export const STRAP: readonly V3[] = strapPath(0.0035);
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

/* ---------------------------------------------------------------- the cane */

const CB = ASSET.rig.bones.find((b: Json) => b.id === "cane") as Json;
/** 🦯 The cane along its bone: the bone starts at the centre of the fist, `top` of the grip lies behind it, the roller ends at the bone tip. */
export const CANE = { top: -0.078, gripEnd: 0.15, joints: [0.402, 0.656, 0.91], band: 0.915, shaftEnd: 1.172, rollerFrom: 1.181, dome: CB.lengthM - 0.0135, end: CB.lengthM as number, shaft: 0.006, grip: 0.012, roller: 0.0135 } as const;
const GRIP_PROFILE: [number, number][] = [[-0.078, 0.0065], [-0.0745, 0.0105], [-0.066, 0.0122], [-0.02, 0.0122], [0.06, 0.0118], [0.13, 0.0108], [0.146, 0.0095], [0.15, 0.0072]];
const ROLLER_PROFILE: [number, number][] = [[1.184, 0.007], [1.1865, 0.0118], [1.192, 0.0135], [1.2085, 0.0135], [1.2145, 0.0121], [1.2192, 0.0084], [1.222, 0.0036]].map(([u, r]) => [u! + (CB.lengthM - 1.222), r!] as [number, number]);
/** 🪢 The elastic wrist loop hanging from the top of the grip, as a closed path in the frame of the cane (it hangs plumb in the default pose). */
export function loopPath(): V3[] {
  const down = D("cane", [0, -1, 0]);
  const side = D("cane", [0, 0, -1]);
  const at = (a: number, b: number): V3 => [CANE.top + 0.004 + down[0] * a + side[0] * b, down[1] * a + side[1] * b, down[2] * a + side[2] * b];
  return [at(0, 0), at(0.022, 0.009), at(0.05, 0.0165), at(0.078, 0.0165), at(0.094, 0.009), at(0.099, 0), at(0.094, -0.009), at(0.078, -0.0165), at(0.05, -0.0165), at(0.022, -0.009), at(0, 0)];
}
export function caneBody(scale: 50 | 100): Json[] {
  const out: Json[] = [];
  const on = (id: string, material: string, tone: number, role = "body"): Common => c(id, "cane", material, tone, role);
  out.push(tube(on("cane-grip", MAT.grip, TONE.grip), GRIP_PROFILE.map(([u]) => [u, 0, 0] as V3), GRIP_PROFILE.map((q) => q[1])));
  out.push(capsule(on("cane-shaft", MAT.cane, TONE.cane), [CANE.gripEnd - 0.004, 0, 0], [CANE.band, 0, 0], CANE.shaft, CANE.shaft, { slender: true }));
  out.push(capsule(on("cane-band", MAT.cane, TONE.band), [CANE.band, 0, 0], [CANE.shaftEnd, 0, 0], CANE.shaft, CANE.shaft, { slender: true }));
  if (scale === 50) {
    CANE.joints.forEach((u, i) => out.push(capsule(on(`cane-joint-${i + 1}`, MAT.cane, 0.8, "detail"), [u - 0.008, 0, 0], [u + 0.008, 0, 0], 0.0076, 0.0076)));
    out.push(tube(on("cane-loop", MAT.grip, 0.2, "detail"), loopPath(), [0.002], { slender: true }));
    out.push(capsule(on("cane-hook", MAT.cane, 0.6, "detail"), [CANE.shaftEnd - 0.002, 0, 0], [CANE.rollerFrom + 0.004, 0, 0], 0.0055, 0.0055));
  }
  out.push(tube(on("cane-roller", MAT.roller, TONE.roller), ROLLER_PROFILE.map(([u]) => [u, 0, 0] as V3), ROLLER_PROFILE.map((q) => q[1])));
  return out;
}

/* ---------------------------------------------------------------- hands */

/** ✋ The hanging left hand: palm towards the thigh, thumb ahead, fingers of three phalanges curling inwards. */
function leftHand(tone: number): Json[] {
  const k0 = 0.88;
  const on = (id: string): Common => c(id, "hand-l", MAT.skin, tone);
  const out: Json[] = [];
  const palm: St[] = [st(-0.01, 0, 0, 0.0185, 0.0265), st(0.02, 0.002, 0, 0.0165, 0.033), st(0.05, 0.002, -0.001, 0.0143, 0.039), st(0.082, 0.0, -0.003, 0.0125, 0.04), st(0.093, 0, -0.004, 0.011, 0.037)];
  out.push(loft(on("hand-l"), dome(dense(palm, 3), 1, 0.007, 2)));
  out.push(ellipsoid(on("hand-l-thenar"), [0.029, 0.0155, 0.011], [0.04, 0.027, -0.009]));
  const fingers: [string, number, number, number][] = [["1", 0.089, 0.029, 1], ["2", 0.093, 0.01, 1.08], ["3", 0.089, -0.0095, 1], ["4", 0.082, -0.028, 0.8]];
  for (const [name, u, v, k] of fingers) {
    const dir = (deg: number, len: number, from: V3): V3 => [from[0] + len * Math.cos((deg * Math.PI) / 180), from[1], from[2] - len * Math.sin((deg * Math.PI) / 180)];
    const p0: V3 = [u, v * 0.96, -0.003];
    const p1 = dir(10, 0.043 * k * k0, p0);
    const p2 = dir(28, 0.027 * k * k0, p1);
    const p3 = dir(46, 0.02 * k * k0, p2);
    const r = k < 0.9 ? 0.9 : 1;
    out.push(capsule(on(`hand-l-finger-${name}`), p0, p1, 0.0092 * r, 0.0086 * r));
    out.push(capsule(on(`hand-l-finger-${name}-mid`), p1, p2, 0.0084 * r, 0.0078 * r));
    out.push(capsule(on(`hand-l-finger-${name}-tip`), p2, p3, 0.0078 * r, 0.0072 * r));
  }
  out.push(capsule(on("hand-l-thumb"), [0.02, 0.029, -0.009], [0.055, 0.05, -0.015], 0.0113, 0.0101));
  out.push(capsule(on("hand-l-thumb-mid"), [0.055, 0.05, -0.015], [0.084, 0.059, -0.02], 0.01, 0.009));
  out.push(capsule(on("hand-l-thumb-tip"), [0.084, 0.059, -0.02], [0.103, 0.061, -0.024], 0.009, 0.0078));
  return out;
}
/** 🧷 A point on the axis of the cane in the frame of the right hand, `s` metres from the centre of the fist towards the tip, moved by (`dv`, `dw`) across. */
export function onCane(s: number, dv = 0, dw = 0): V3 {
  const a = (CB.angleDeg * Math.PI) / 180;
  return [CB.x + s * Math.cos(a) - dv * Math.sin(a), CB.y + s * Math.sin(a) + dv * Math.cos(a), CB.z + dw];
}
/** ✊ The right hand on the grip: the grip lies obliquely in the palm from the heel of the hand to the base of the index finger, the index finger lies stretched along it, the thumb on top, the other three fingers wrap it from below. */
function rightHand(tone: number): Json[] {
  const on = (id: string): Common => c(id, "hand-r", MAT.skin, tone);
  const out: Json[] = [];
  const palm: St[] = [st(-0.01, 0, 0, 0.0185, 0.0265), st(0.02, 0.002, 0, 0.0155, 0.033), st(0.05, 0.002, 0, 0.013, 0.039), st(0.082, 0.0, 0.001, 0.012, 0.04), st(0.093, 0, 0.002, 0.0105, 0.037)];
  out.push(loft(on("hand-r"), dome(dense(palm, 3), 1, 0.007, 2)));
  out.push(ellipsoid(on("hand-r-thenar"), [0.028, 0.015, 0.012], [0.036, 0.029, 0.008]));
  const i0: V3 = [0.088, 0.03, 0.005];
  const i1 = onCane(0.088, 0.0185, -0.0095);
  const i2 = onCane(0.114, 0.0165, -0.011);
  const i3 = onCane(0.134, 0.015, -0.0115);
  out.push(capsule(on("hand-r-finger-1"), i0, i1, 0.0092, 0.0086));
  out.push(capsule(on("hand-r-finger-1-mid"), i1, i2, 0.0084, 0.0078));
  out.push(capsule(on("hand-r-finger-1-tip"), i2, i3, 0.0078, 0.0072));
  const wrap: [string, V3, number, number][] = [["2", [0.091, 0.008, 0.005], 0.05, 1], ["3", [0.087, -0.0115, 0.005], 0.028, 0.96], ["4", [0.08, -0.03, 0.004], 0.006, 0.86]];
  for (const [name, p0, s, k] of wrap) {
    const p1: V3 = [p0[0] + 0.012 * k, p0[1] + 0.002, p0[2] + 0.034 * k];
    const p2 = onCane(s + 0.012, -0.004, 0.0205);
    const p3 = onCane(s - 0.004, 0.012, 0.014);
    out.push(capsule(on(`hand-r-finger-${name}`), p0, p1, 0.0092 * k, 0.0086 * k));
    out.push(capsule(on(`hand-r-finger-${name}-mid`), p1, p2, 0.0084 * k, 0.0078 * k));
    out.push(capsule(on(`hand-r-finger-${name}-tip`), p2, p3, 0.0078 * k, 0.0072 * k));
  }
  const t1 = onCane(0.0, 0.021, 0.004);
  const t2 = onCane(0.034, 0.0215, 0.003);
  out.push(capsule(on("hand-r-thumb"), [0.018, 0.03, 0.006], t1, 0.0113, 0.0101));
  out.push(capsule(on("hand-r-thumb-tip"), t1, t2, 0.01, 0.0082));
  return out;
}

/* ---------------------------------------------------------------- body */

const LEVEL = { transform: { rotation: aboutW(-boneDeg("foot-l")) } };
export const ANKLE_Y = FRAMES["foot-l"]!.o[1];
export const Hd = (x: number, y: number, z = 0): V3 => L("head", [x, y, z]);
export const level = aboutW(90 - boneDeg("head"));

/** 📏 The half-width of the jacket at the height `y`. */
export function jacketHalf(y: number): number {
  let i = 0;
  while (i + 2 < JACKET.length && JACKET[i + 1]![0] < y) i++;
  const [a, b] = [JACKET[i]!, JACKET[i + 1]!];
  const t = Math.min(1, Math.max(0, (y - a[0]) / (b[0] - a[0])));
  return a[3] + (b[3] - a[3]) * t;
}
/** 🔖 A pointed flap of a chest pocket lying on the jacket front: straight top edge, the point 21 mm lower in the middle, 9 mm thick, following the curve of the chest across its 100 mm width; with the press stud at the point. */
export const POCKET = { y: 1.287, drop: 0.032, point: 0.053, z: 0.094, half: 0.05, thick: 0.0092 } as const;
function pocketFlap(side: 1 | -1): Json[] {
  const n = side > 0 ? "l" : "r";
  const zs = [-1, -0.75, -0.5, -0.25, 0, 0.25, 0.5, 0.75, 1].map((k) => side * POCKET.z + k * POCKET.half);
  const sections = zs.map((z) => {
    const k = Math.abs(z - side * POCKET.z) / POCKET.half;
    const bottom = POCKET.y - POCKET.drop - (POCKET.point - POCKET.drop) * (1 - k);
    const yc = (POCKET.y + bottom) / 2;
    const dx = (jacketX(yc, z + 0.003, 1) - jacketX(yc, z - 0.003, 1)) / 0.006;
    const t = Math.hypot(dx, 1);
    const out: V3 = [1 / t, 0, -dx / t];
    const at: V3 = [jacketX(yc, z, 1) + out[0] * (POCKET.thick / 2 - 0.0015), yc, z + out[2] * (POCKET.thick / 2 - 0.0015)];
    return { at: L("torso", at), rx: (POCKET.y - bottom) / 2, ry: POCKET.thick / 2, e: 4, ex: D("torso", [0, 1, 0]), ey: D("torso", [-out[0], 0, -out[2]]) };
  });
  const yb = POCKET.y - POCKET.point + 0.006;
  return [
    freeLoft(c(`jacket-pocket-${n}`, "torso", MAT.jacket, 0.37, "detail"), sections),
    ellipsoid(c(`jacket-pocket-stud-${n}`, "torso", MAT.jacket, 0.72, "detail"), [0.0075, 0.0042, 0.0075], L("torso", [jacketX(yb, side * POCKET.z, 1) + POCKET.thick - 0.001, yb, side * POCKET.z]), aboutW(90 - boneDeg("torso"))),
  ];
}

export function body50(): Json[] {
  const out: Json[] = [];
  const both = (solids: Json[]): void => {
    for (const s of solids) out.push(s);
    for (const s of solids) out.push(mirrored(s, toRight, near));
  };
  const denim = (id: string, bone = "torso", tone: number = TONE.jacket, role = "body"): Common => c(id, bone, MAT.jacket, tone, role);

  out.push(loft(c("hips", "pelvis", MAT.jeans, TONE.jeans), dense(hs("pelvis", HIPS), 3)));
  out.push(loft(denim("jacket"), dense(hs("torso", JACKET), 4)));

  const leg: Json[] = [];
  leg.push(loft(c("thigh-l", "thigh-l", MAT.jeans, TONE.jeansFar), dome([...dense(THIGH_L, 3), st(0.426, 0.004, -0.005, 0.0565, 0.0615, 2)], 1, 0.012)));
  leg.push(loft(c("shank-l", "shank-l", MAT.jeans, TONE.jeansFar), dome([st(-0.02, 0.002, -0.005, 0.052, 0.054, 2), ...dense(SHANK_L, 3)], -1, 0.012)));
  leg.push(loft(c("boot-shaft-l", "shank-l", MAT.boot, TONE.bootFar), SHAFT_L));
  leg.push(loft(c("boot-l", "foot-l", MAT.boot, TONE.bootFar), dense(xa("foot-l", BOOT_REAR), 3)));
  leg.push(loft(c("boot-toe-l", "toe-l", MAT.boot, TONE.bootFar), dense(xa("toe-l", BOOT_TOE), 3)));
  leg.push(extrude(c("boot-heel-l", "foot-l", MAT.boot, TONE.sole), "zx", HEEL_PLAN, -ANKLE_Y, -ANKLE_Y + 0.025, LEVEL));
  leg.push(extrude(c("boot-sole-l", "foot-l", MAT.boot, TONE.sole), "zx", SOLE_PLAN, -ANKLE_Y, -ANKLE_Y + 0.01, LEVEL));
  leg.push(extrude(c("boot-toe-sole-l", "toe-l", MAT.boot, TONE.sole), "zx", TOE_PLAN, 0, 0.01));
  both(leg);

  const arm: Json[] = [];
  arm.push(loft(denim("sleeve-l", "arm-l", TONE.jacketFar), dome(dome([...dense(SLEEVE_L, 3), st(0.326, -0.003, 0, 0.0435, 0.0485)], 1, 0.014), -1, 0.009, 2)));
  arm.push(loft(denim("forearm-l", "forearm-l", TONE.jacketFar), dome([st(-0.026, -0.003, 0, 0.0425, 0.0475), ...dense(CUFF_L, 3)], -1, 0.022)));
  arm.push(loft(denim("cuff-l", "forearm-l", 0.36, "detail"), [st(0.19, 0, 0, 0.0375, 0.0385, 2.2), st(0.236, 0, 0, 0.037, 0.0375, 2.2)]));
  arm.push(loft(c("wrist-l", "forearm-l", MAT.skin, TONE.handFar), [st(0.21, 0, 0, 0.02, 0.0275), st(0.244, 0, 0, 0.019, 0.027), st(0.262, 0, 0, 0.0185, 0.0265)]));
  both(arm);
  for (const s of out) if (s.id === "wrist-r") s.tone = TONE.hand;
  out.push(...leftHand(TONE.handFar));
  out.push(...rightHand(TONE.hand));

  const skin = (id: string, bone = "head", tone: number = TONE.skin, role = "body"): Common => c(id, bone, MAT.skin, tone, role);
  out.push(loft(skin("neck", "neck", 0.76), hs("neck", NECK)));
  out.push(loft(denim("collar", "neck", 0.4), [st(-0.034, 0.004, 0, 0.094, 0.086, 2.1, -18), st(-0.012, -0.002, 0, 0.09, 0.082, 2.1, -18), st(0.012, -0.008, 0, 0.081, 0.074, 2.1, -18)]));
  out.push(loft(skin("head"), dome(dome(dense(hs("head", HEAD), 3), -1, 0.005, 2), 1, 0.003, 2)));
  out.push(loft(c("hair", "head", MAT.hair, TONE.hair), dome(dome(dense(hs("head", HAIR), 3), -1, 0.006, 2), 1, 0.003, 2)));
  const nose: [number, number, number][] = [[1.6, 0.111, 0.007], [1.585, 0.1185, 0.0088], [1.571, 0.126, 0.0115], [1.561, 0.1305, 0.0152], [1.5535, 0.1225, 0.0152]];
  out.push(loft(skin("head-nose"), dense(nose.map(([y, f, rw]) => H("head", y, 0.094, f, rw, 2)), 2).reverse()));
  out.push(ellipsoid(skin("head-lips", "head", 0.62), [0.0063, 0.007, 0.0215], Hd(0.1085, 1.531), level));
  out.push(ellipsoid(skin("head-chin"), [0.014, 0.013, 0.02], Hd(0.094, 1.4955), level));
  const shades = (id: string, role = "body"): Common => c(id, "head", MAT.lens, TONE.lens, role);
  for (const side of [1, -1]) {
    const n = side > 0 ? "l" : "r";
    out.push(ellipsoid(shades(`lens-${n}`, "glazing"), [0.0195, 0.0042, 0.0265], Hd(0.1135, 1.596, side * 0.0325), compose(level, aboutU(side * 9))));
    out.push(capsule(shades(`temple-${n}`, "detail"), Hd(0.106, 1.602, side * 0.064), Hd(0.03, 1.598, side * 0.0765), 0.0018, 0.0018, { slender: true }));
  }
  out.push(tube(shades("glasses-front", "detail"), [-0.064, -0.034, 0, 0.034, 0.064].map((z) => Hd(0.1165 - Math.abs(z) * 0.14, 1.6135 - (z === 0 ? 0.004 : 0), z)), [0.003], { slender: true }));

  for (const side of [1, -1]) out.push(...pocketFlap(side as 1 | -1));
  out.push(loft(denim("jacket-waistband", "torso", 0.4, "detail"), [0.951, 0.972, 0.996].map((y) => H("torso", y, jacketX(y, 0, -1) - 0.0045, jacketX(y, 0, 1) + 0.0045, jacketHalf(y) + 0.0045, 2.5))));
  [1.385, 1.3, 1.215, 1.13, 1.045, 0.976].forEach((y, i) => out.push(ellipsoid(denim(`jacket-button-${i + 1}`, "torso", 0.72, "detail"), [0.0085, 0.0042, 0.0085], L("torso", [jacketX(y, 0.008, 1) + 0.002, y, 0.008]), aboutW(90 - boneDeg("torso")))));
  const vee: [number, number][] = [[1.4, 0.008], [1.418, 0.022], [1.436, 0.036], [1.452, 0.044]];
  out.push(loft(c("shirt-front", "torso", MAT.shirt, TONE.shirt, "detail"), vee.map(([y, rw]) => H("torso", y, jacketX(y, 0, 1) - 0.02, jacketX(y, 0, 1) + 0.003, rw, 2))));

  const leather = (id: string, tone: number = TONE.bag, role = "body"): Common => c(id, "torso", MAT.bag, tone, role);
  const upright = aboutW(90 - boneDeg("torso"));
  out.push(box(leather("bag-body"), [BAG.height, BAG.length, BAG.depth], B(BAG.x, BAG.y, BAG.z), upright, 0.018));
  const top = BAG.y + BAG.height / 2;
  const outer = BAG.z - BAG.depth / 2;
  const hl = BAG.length / 2 - 0.003;
  const o = L("torso", [BAG.x, top - 0.06, 0]);
  const tu = FRAMES.torso!.u;
  const flapFig: [number, number][] = [[hl, 0.062], [hl, -0.045], [hl - 0.006, -0.06], [hl - 0.024, -0.071], [hl - 0.05, -0.074], [-hl + 0.05, -0.074], [-hl + 0.024, -0.071], [-hl + 0.006, -0.06], [-hl, -0.045], [-hl, 0.062]];
  out.push(extrude(leather("bag-flap", 0.44), "xy", flapFig.map(([dx, dy]) => [o[0] + dx * tu[0] + dy * tu[1], o[1] - dx * tu[1] + dy * tu[0]] as [number, number]), outer - 0.0085, outer + 0.003));
  out.push(box(leather("bag-flap-top", 0.44), [0.009, BAG.length - 0.006, BAG.depth + 0.008], B(BAG.x, top + 0.002, BAG.z - 0.004), upright, 0.004));
  out.push(box(leather("bag-clasp", 0.75, "detail"), [0.026, 0.02, 0.009], B(BAG.x, top - 0.128, outer - 0.0105), upright, 0.003));
  for (const [n, x] of [["front", 0.075], ["back", -0.115]] as const) out.push(box(leather(`bag-lug-${n}`, 0.3), [0.026, 0.012, 0.03], B(x, top + 0.004, BAG.z), upright, 0.004));
  out.push(loft(leather("bag-strap", TONE.strap), dense(ribbon("torso", STRAP, 0.0145, 0.003), 2), [true, true], { slender: true }));

  out.push(...caneBody(50));
  return out;
}

/* ---------------------------------------------------------------- thermal */

const LAYERS = {
  jacket: [["jacket-shirt", 0.002], ["jacket-air", 0.003]],
  jeans: [["jeans-air", 0.002]],
  boots: [["boots-sock", 0.003], ["boots-air", 0.005]],
} as const;
export const OUTER = { jacket: "jacket-denim", jeans: "jeans-denim", boots: "boots-leather", hair: "hair-pelt" } as const;
type Layered = keyof typeof LAYERS;
/** 🧅 The shells of a garment loft under its outer layer: the same sections moved inwards by the thickness of the layers outside. */
function shells(solid: Json, region: Layered): Json[] {
  return LAYERS[region].map(([layer, t], i) => ({
    ...solid,
    id: `th-${solid.id}-${layer.split("-").pop()}`,
    tone: Math.min(0.95, Math.round((solid.tone + 0.14 * (i + 1)) * 100) / 100),
    sections: solid.sections.map((q: Json) => ({ ...q, rx: Math.max(0.003, Math.round((q.rx - t) * 1e5) / 1e5), ry: Math.max(0.003, Math.round((q.ry - t) * 1e5) / 1e5) })),
    region,
    layer,
  }));
}
/** 🧥 The height of the jacket hem above the pelvis joint, where the jeans come out from under the jacket. */
export const HEM = 0.05;
/** ✂️ The sections of a loft between two stations along its bone, with a section interpolated at each station that falls between two authored ones, so a cut garment ends exactly where the next one begins. */
export function between(sections: readonly Json[], lo: number, hi: number): Json[] {
  const mix = (a: Json, b: Json, x: number): Json => {
    const k = (x - a.at[0]) / (b.at[0] - a.at[0]);
    const m = (p: number, q: number): number => Math.round((p + (q - p) * k) * 1e5) / 1e5;
    return { ...a, at: [x, m(a.at[1], b.at[1]), m(a.at[2], b.at[2])], rx: m(a.rx, b.rx), ry: m(a.ry, b.ry), ...(a.exponent === undefined ? {} : { exponent: m(a.exponent, b.exponent ?? a.exponent) }) };
  };
  const out: Json[] = [];
  for (let i = 0; i < sections.length; i++) {
    const [a, b] = [sections[i]!, sections[i + 1]];
    if (a.at[0] >= lo - 1e-6 && a.at[0] <= hi + 1e-6) out.push(a);
    if (b === undefined) continue;
    for (const x of [lo, hi]) if (x > a.at[0] + 1e-6 && x < b.at[0] - 1e-6) out.push(mix(a, b, x));
  }
  return out.sort((a, b) => a.at[0] - b.at[0]);
}
/** 👖 A leg of the jeans as the open tube it is: the sections of the body loft without the rounded joint end, the thigh from the crotch line down; at 1:50 with the air shell inside it. */
export function legTubes(solid: Json, withShell: boolean): Json[] {
  const sections = solid.bone.startsWith("thigh") ? between(solid.sections, 0.085, 0.4117) : between(solid.sections, 0, 0.426);
  const open: Json = { ...solid, id: `th-${solid.id}`, sections, capStart: false, capEnd: false, region: "jeans", ...(withShell ? { layer: OUTER.jeans } : {}) };
  if (!withShell) return [open];
  return [open, { ...open, id: `th-${solid.id}-air`, tone: Math.round((solid.tone + 0.14) * 100) / 100, sections: sections.map((q: Json) => ({ ...q, rx: Math.round((q.rx - 0.002) * 1e5) / 1e5, ry: Math.round((q.ry - 0.002) * 1e5) / 1e5 })), layer: "jeans-air" }];
}
/** 🙂 The rows of the head that carry bare skin: chin to the hairline of the forehead. */
export const FACE_TOP = 1.658;
export const breath = (scale: number): Json[] => {
  const k = scale === 50 ? 1 : scale === 100 ? 1.9 : scale === 200 ? 3.8 : 9.4;
  const skin = { bone: "head", material: MAT.skin, tone: TONE.skin };
  return [
    { ...ellipsoid({ id: "th-breath-nose", ...skin }, [0.004 * k, 0.0045 * k, Math.max(0.0085, 0.004 * k)], L("head", [OUTLET.nose[0] - 0.002 * k, OUTLET.nose[1], 0]), level), region: "face-skin", tags: ["emitter", "breath-nose"] },
    { ...ellipsoid({ id: "th-breath-mouth", ...skin }, [0.004 * k, 0.004 * k, Math.max(0.0062, 0.004 * k)], L("head", [OUTLET.mouth[0] - 0.0035 - 0.002 * k, OUTLET.mouth[1], 0]), level), region: "face-skin", tags: ["emitter", "breath-mouth"] },
  ];
};
export const plume = (thick: number): Json => ({ ...ellipsoid({ id: "th-plume", bone: "head", material: MAT.hair, tone: 0.55 }, [thick, 0.058, 0.047], L("head", [OUTLET.crown[0] + 0.006, OUTLET.crown[1] - thick * 0.45, 0]), level), region: "jacket", tags: ["emitter", "plume"] });
/** 📐 The height of a head section in the standing figure. */
export function headY(q: Json): number {
  const f = FRAMES.head!;
  return f.o[1] + q.at[0] * f.u[1] + q.at[1] * f.v[1];
}
export const sides = (ids: readonly string[]): string[] => ids.flatMap((id) => [id, toRight(id)]);
/** 🦯 The thermal solids of the cane and the glasses, by reference. */
export function passive(body: readonly Json[]): Json[] {
  const out: Json[] = [];
  for (const s of body) {
    if (s.material === MAT.lens) out.push(thOf(s.id, "sunglasses"));
    else if (s.material === MAT.bag) out.push(thOf(s.id, "bag"));
    else if (s.bone === "cane") out.push(thOf(s.id, s.material === MAT.grip ? "cane-grip" : s.material === MAT.roller ? "cane-tip" : "cane-shaft"));
  }
  return out;
}

export function thermal50(body: readonly Json[]): Json[] {
  const out: Json[] = [];
  const by = (id: string): Json => { const s = body.find((b) => b.id === id); if (!s) throw new Error(`no body solid ${id}`); return s; };
  const layered = (id: string, region: Layered): void => { out.push(thOf(id, region, OUTER[region], id.startsWith("boot-shaft") ? { sectionFaces: ["-u"] } : {})); out.push(...shells(by(id), region)); };
  const ref = (ids: readonly string[], region: string, layer?: string, extra: Json = {}): void => { for (const id of ids) { by(id); out.push(thOf(id, region, layer, extra)); } };

  for (const id of ["jacket", "collar", ...sides(["sleeve-l", "forearm-l"])]) layered(id, "jacket");
  ref([...sides(["cuff-l", "jacket-pocket-l", "jacket-pocket-stud-l"]), "jacket-waistband", ...[1, 2, 3, 4, 5, 6].map((i) => `jacket-button-${i}`)], "jacket", OUTER.jacket);
  ref(["shirt-front"], "jacket", "jacket-shirt", { sectionFaces: ["+v"] });
  out.push({ ...by("hips"), id: "th-hips", sections: between(by("hips").sections, -1, HEM), capEnd: false, region: "jeans", layer: OUTER.jeans });
  for (const id of sides(["thigh-l", "shank-l"])) out.push(...legTubes(by(id), true));
  for (const id of sides(["boot-shaft-l", "boot-l", "boot-toe-l"])) layered(id, "boots");
  ref(sides(["boot-heel-l", "boot-sole-l", "boot-toe-sole-l"]), "boots", OUTER.boots);
  ref(["hair"], "hair", OUTER.hair);

  const head = by("head");
  out.push({ ...head, id: "th-head", sections: head.sections.filter((q: Json) => headY(q) <= FACE_TOP), capStart: true, capEnd: false, sectionFaces: ["+v"], region: "face-skin" });
  ref(["head-nose", "head-lips", "head-chin"], "face-skin");
  out.push({ ...loft({ id: "th-neck", bone: "neck", material: MAT.skin, tone: 0.76 }, hs("neck", [[1.478, -0.049, 0.051, 0.0515, 2], [1.52, -0.044, 0.056, 0.05, 2]]), [false, false]), region: "face-skin" });
  ref(body.filter((b) => b.material === MAT.skin && /^(hand|wrist)/.test(b.id)).map((b) => b.id), "hands");
  out.push(...passive(body));
  out.push(...breath(50), plume(0.004));
  return out;
}

/* ---------------------------------------------------------------- structure */

export const BONE = { material: MAT.bone, tone: 0.62 };
export const kgOf = (bone: string): Json => (ASSET.rig.bones as Json[]).find((b) => b.id === bone).mass;
export const T = (x: number, y: number, z = 0): V3 => L("torso", [x, y, z]);
export const P = (x: number, y: number, z = 0): V3 => L("pelvis", [x, y, z]);
export const N = (x: number, y: number, z = 0): V3 => L("neck", [x, y, z]);
export const ring = (rd: number, r: number): [number, number][] => Array.from({ length: 9 }, (_, k) => [r * Math.cos((k * Math.PI) / 4), rd + r * Math.sin((k * Math.PI) / 4)] as [number, number]);
/** 🦴 The spine from the sacral base to the seventh cervical vertebra: lumbar lordosis, thoracic kyphosis; [x, y, radius]. */
export const SPINE: readonly [number, number, number][] = [[-0.032, 1.0, 0.019], [-0.012, 1.045, 0.0185], [-0.004, 1.095, 0.018], [-0.016, 1.16, 0.0165], [-0.04, 1.235, 0.015], [-0.052, 1.31, 0.014], [-0.046, 1.375, 0.013], [-0.03, 1.43, 0.012], [-0.02, 1.464, 0.0115]];
export const CERVICAL: readonly [number, number, number][] = [[-0.02, 1.464, 0.0115], [-0.008, 1.495, 0.011], [0.004, 1.525, 0.0105], [0.012, 1.548, 0.01]];
/** 🫁 The rib hoops from the tenth to the second rib: [y, x of the centre, depth radius, half-width]. */
export const RIBS: readonly [number, number, number, number][] = [[1.13, 0.01, 0.078, 0.108], [1.163, 0.01, 0.084, 0.117], [1.196, 0.009, 0.088, 0.122], [1.229, 0.008, 0.089, 0.124], [1.262, 0.006, 0.087, 0.123], [1.295, 0.003, 0.082, 0.118], [1.328, -0.002, 0.074, 0.108], [1.358, -0.007, 0.063, 0.092], [1.386, -0.012, 0.05, 0.07]];
export const massBall = (member: string, scale = 1): Json => {
  const k = kgOf(member);
  const r = 0.0125 * Math.cbrt(k.kg) * scale;
  return part(ellipsoid({ id: `mass-${member}`, bone: member, material: MAT.bone, tone: 0.2, role: "detail" }, [r, r, r], [k.comX, k.comY, 0]), member, "mass");
};
export const jointBall = (member: string, at: V3, r: number): Json => part(ellipsoid({ id: `joint-${member}`, bone: member, material: MAT.bone, tone: 0.4 }, [r, r, r], at), member, "joint");
/** 🟢 The roller support at the anchor of the cane tip. */
export function support(r: number): Json {
  const a = (ASSET.anchors as Json[]).find((q) => q.id === "cane-tip");
  return { ...part(ellipsoid({ id: "support-cane-tip", bone: "cane", material: MAT.roller, tone: 0.2, role: "detail" }, [r, r, r], [a.x, a.y, a.z ?? 0]), "cane", "support"), anchor: "cane-tip" };
}
/** 🦯 The cane as a member: the aluminium tube from the top of the grip to the hook of the tip, and its joint in the fist. */
export function caneMember(body: readonly Json[], jointR: number): Json[] {
  const out: Json[] = [part(capsule({ id: "member-cane-core", bone: "cane", ...BONE, material: MAT.cane, tone: 0.9 }, [CANE.top + 0.006, 0, 0], [CANE.gripEnd + 0.03, 0, 0], CANE.shaft, CANE.shaft, { slender: true }), "cane", "member")];
  for (const id of ["cane-shaft", "cane-band"]) if (body.some((b) => b.id === id)) out.push({ id: `member-${id}`, of: id, member: "cane", part: "member" });
  out.push(jointBall("cane", [0, 0, 0], jointR));
  return out;
}

export function structure50(body: readonly Json[]): Json[] {
  const out: Json[] = [];
  const m = (solid: Json, member: string): void => { out.push(part(solid, member, "member")); };
  m(tube({ id: "spine", bone: "torso", ...BONE }, SPINE.map(([x, y]) => T(x, y)), SPINE.map((q) => q[2])), "torso");
  RIBS.forEach(([y, x, rd, rw], i) => m(revolve({ id: `rib-${10 - i}`, bone: "torso", ...BONE }, ring(rd, 0.0055), T(x, y), [1, 0, 0], { lateralScale: Math.round((rw / rd) * 1000) / 1000 }), "torso"));
  m(loft({ id: "sternum", bone: "torso", ...BONE }, [H("torso", 1.235, 0.086, 0.097, 0.012), H("torso", 1.31, 0.081, 0.092, 0.017), H("torso", 1.365, 0.062, 0.073, 0.022), H("torso", 1.386, 0.048, 0.059, 0.024)]), "torso");
  for (const side of [1, -1]) {
    const n = side > 0 ? "l" : "r";
    m(capsule({ id: `clavicle-${n}`, bone: "torso", ...BONE }, T(0.05, 1.39, side * 0.02), T(-0.004, 1.414, side * 0.168), 0.007, 0.0066), "torso");
    m(ellipsoid({ id: `scapula-${n}`, bone: "torso", ...BONE }, [0.066, 0.0062, 0.044], T(-0.076, 1.332, side * 0.096), compose(aboutW(4), aboutU(side * 28))), "torso");
  }
  m(tube({ id: "sacrum", bone: "pelvis", ...BONE }, [P(-0.032, 1.0), P(-0.06, 0.958), P(-0.076, 0.918), P(-0.07, 0.888)], [0.027, 0.025, 0.016, 0.0075]), "pelvis");
  for (const side of [1, -1]) {
    const n = side > 0 ? "l" : "r";
    m(ellipsoid({ id: `ilium-${n}`, bone: "pelvis", ...BONE }, [0.056, 0.07, 0.011], P(-0.02, 0.972, side * 0.116), compose(aboutU(side * 36), aboutV(side * -24))), "pelvis");
    m(capsule({ id: `pubis-${n}`, bone: "pelvis", ...BONE }, P(0.006, 0.906, side * 0.078), P(0.052, 0.884, side * 0.012), 0.011, 0.01), "pelvis");
    m(capsule({ id: `ischium-${n}`, bone: "pelvis", ...BONE }, P(-0.006, 0.9, side * 0.078), P(-0.036, 0.846, side * 0.06), 0.012, 0.013), "pelvis");
  }
  m(ellipsoid({ id: "cranium", bone: "head", ...BONE }, [0.061, 0.087, 0.066], Hd(0.02, 1.632), level), "head");
  m(ellipsoid({ id: "face-bones", bone: "head", ...BONE }, [0.041, 0.031, 0.05], Hd(0.078, 1.567), level), "head");
  for (const side of [1, -1]) {
    const n = side > 0 ? "l" : "r";
    m(capsule({ id: `mandible-ramus-${n}`, bone: "head", ...BONE }, Hd(0.03, 1.57, side * 0.048), Hd(0.024, 1.516, side * 0.044), 0.007, 0.0075), "head");
    m(capsule({ id: `mandible-body-${n}`, bone: "head", ...BONE }, Hd(0.024, 1.516, side * 0.044), Hd(0.09, 1.4945, side * 0.01), 0.0075, 0.0085), "head");
  }
  m(tube({ id: "cervical-spine", bone: "neck", ...BONE }, CERVICAL.map(([x, y]) => N(x, y)), CERVICAL.map((q) => q[2])), "neck");

  const limb: Json[] = [];
  const lm = (solid: Json, member: string): void => { limb.push(part(solid, member, "member")); };
  lm(capsule({ id: "femur-neck-l", bone: "thigh-l", ...BONE }, [0, 0, 0], [0.034, -0.006, 0.052], 0.0155, 0.0165), "thigh-l");
  lm(ellipsoid({ id: "femur-trochanter-l", bone: "thigh-l", ...BONE }, [0.022, 0.017, 0.014], [0.03, -0.008, 0.06]), "thigh-l");
  lm(tube({ id: "femur-l", bone: "thigh-l", ...BONE }, [[0.045, -0.005, 0.056], [0.123, 0.006, 0.042], [0.218, 0.012, 0.026], [0.312, 0.008, 0.01], [0.394, 0.0, -0.002]], [0.0145, 0.0132, 0.0126, 0.0132, 0.016]), "thigh-l");
  lm(ellipsoid({ id: "femur-condyles-l", bone: "thigh-l", ...BONE }, [0.02, 0.029, 0.038], [0.405, -0.006, -0.005]), "thigh-l");
  lm(ellipsoid({ id: "patella-l", bone: "thigh-l", ...BONE }, [0.02, 0.009, 0.019], [0.403, 0.039, -0.005]), "thigh-l");
  lm(ellipsoid({ id: "tibia-plateau-l", bone: "shank-l", ...BONE }, [0.014, 0.025, 0.036], [0.02, -0.002, -0.005]), "shank-l");
  lm(tube({ id: "tibia-l", bone: "shank-l", ...BONE }, [[0.03, 0.004, -0.007], [0.126, 0.011, -0.005], [0.262, 0.009, -0.003], [0.392, 0.002, -0.002]], [0.016, 0.0127, 0.0113, 0.0127]), "shank-l");
  lm(ellipsoid({ id: "malleolus-l", bone: "shank-l", ...BONE }, [0.013, 0.011, 0.009], [0.406, 0.0, -0.025]), "shank-l");
  lm(capsule({ id: "fibula-l", bone: "shank-l", ...BONE }, [0.04, -0.017, 0.027], [0.415, -0.011, 0.027], 0.0064, 0.0068), "shank-l");
  lm(capsule({ id: "calcaneus-l", bone: "foot-l", ...BONE }, xa("foot-l", [[-0.04, 0.04, 0.044, 0.01]])[0]!.at, xa("foot-l", [[0.004, 0.052, 0.058, 0.01]])[0]!.at, 0.0195, 0.0175), "foot-l");
  lm(loft({ id: "midfoot-l", bone: "foot-l", ...BONE }, xa("foot-l", [[-0.004, 0.044, 0.084, 0.022], [0.05, 0.035, 0.071, 0.03], [0.095, 0.025, 0.051, 0.036], [0.134, 0.016, 0.035, 0.039]])), "foot-l");
  lm(loft({ id: "phalanges-l", bone: "toe-l", ...BONE }, xa("toe-l", [[0.13, 0.016, 0.035, 0.039], [0.17, 0.015, 0.03, 0.035, 2, -0.003], [0.196, 0.015, 0.026, 0.024, 2, -0.005]])), "toe-l");
  lm(ellipsoid({ id: "humerus-head-l", bone: "arm-l", ...BONE }, [0.0215, 0.0215, 0.0215], [0.002, 0, 0]), "arm-l");
  lm(tube({ id: "humerus-l", bone: "arm-l", ...BONE }, [[0.013, 0, 0], [0.104, 0.004, 0], [0.208, 0.003, 0], [0.299, 0, 0]], [0.0112, 0.01, 0.0097, 0.0117]), "arm-l");
  lm(ellipsoid({ id: "humerus-condyles-l", bone: "arm-l", ...BONE }, [0.0115, 0.013, 0.029], [0.308, 0, 0]), "arm-l");
  lm(capsule({ id: "ulna-l", bone: "forearm-l", ...BONE }, [-0.013, -0.016, 0], [0.238, -0.013, 0], 0.0077, 0.0049), "forearm-l");
  lm(capsule({ id: "radius-l", bone: "forearm-l", ...BONE }, [0.009, 0.014, 0], [0.242, 0.01, 0], 0.0056, 0.0082), "forearm-l");
  for (const s of limb) out.push(s);
  for (const s of limb) out.push(mirrored(s, toRight));

  for (const hand of ["hand-l", "hand-r"]) {
    const rays = body.filter((b) => b.bone === hand && b.kind === "capsule");
    const carpus: V3 = [0.013, 0, 0];
    m(ellipsoid({ id: `carpals-${hand.slice(-1)}`, bone: hand, ...BONE }, [0.013, 0.013, 0.012], carpus), hand);
    for (const ray of rays) {
      m(capsule({ id: ray.id.replace("hand-", "bones-"), bone: hand, ...BONE }, ray.p0, ray.p1, 0.004, 0.0038), hand);
      if (/finger-\d$|thumb$/.test(ray.id)) m(capsule({ id: `${ray.id.replace("hand-", "bones-")}-metacarpal`, bone: hand, ...BONE }, [0, 1, 2].map((k) => carpus[k]! + (ray.p0[k] - carpus[k]!) * 0.25) as V3, ray.p0, 0.0044, 0.0042), hand);
    }
  }
  out.push(...caneMember(body, 0.0085));
  out.push(support(0.0085));

  out.push(jointBall("torso", T(-0.032, 1.0), 0.016));
  out.push(jointBall("neck", N(-0.02, 1.464), 0.0125));
  out.push(jointBall("head", Hd(0.012, 1.548), 0.0115));
  for (const side of ["l", "r"]) {
    out.push(jointBall(`thigh-${side}`, [0, 0, 0], 0.023));
    out.push(jointBall(`shank-${side}`, [0, 0, side === "l" ? -0.005 : 0.005], 0.018));
    out.push(jointBall(`foot-${side}`, [0, 0.002, 0], 0.015));
    out.push(jointBall(`toe-${side}`, [-0.004, 0.022, 0], 0.0095));
    out.push(jointBall(`arm-${side}`, [0.002, 0, 0], 0.023));
    out.push(jointBall(`forearm-${side}`, [0, 0, 0], 0.0145));
    out.push(jointBall(`hand-${side}`, [0, 0, 0], 0.0115));
  }
  for (const b of ["pelvis", "torso", "head", "thigh-l", "thigh-r", "shank-l", "shank-r", "arm-l", "arm-r", "forearm-l", "forearm-r", "foot-l", "foot-r", "hand-l", "hand-r", "cane"]) out.push(massBall(b));
  return out;
}

export const DECISIONS_50: Json[] = [{ en: "1:50 body: draft.", de: "Körper 1:50: Entwurf." }];
export { LEVEL };
