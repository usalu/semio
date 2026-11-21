/** 👵 The 1:50 figure of `humans/person-with-rollator`, stated section by section: a 79-year-old woman of 1.58 m and 62 kg with a rounded upper back, in a knee-length loden coat over cardigan and blouse, a wool beret, glasses, wool trousers and laced leather shoes, both hands closed on the grips of her rollator.
 *
 * Figure coordinates (`H`, `X`, `L`): x ahead, y up, z to the left, the default pose `stand-at-grips`. Local stations (`st`): u along the bone, v its sagittal normal, w lateral.
 * @see ./r5-person-with-rollator-kit.ts
 * @see ./r3-humans-person-with-rollator.md */
import type { Json } from "./volumes-lib.ts";
import { H, L, X, aboutU, aboutW, boneDeg, capsule, compose, dense, dome, ellipsoid, extrude, loft, mirrored, revolve, st, toRight, type Common, type Quat, type St, type V3 } from "./r5-person-with-rollator-kit.ts";

export const TONE = { coat: 0.38, coatFar: 0.3, trousers: 0.33, trousersFar: 0.27, shoe: 0.22, shoeFar: 0.17, skin: 0.85, hand: 0.82, handFar: 0.74, hair: 0.93, beret: 0.3, knit: 0.55, blouse: 0.9 } as const;
/** 🌗 The tone of a part on the near (right) side for its tone on the far (left) side. */
export const near = (t: number): number => (t === TONE.coatFar ? TONE.coat : t === TONE.trousersFar ? TONE.trousers : t === TONE.shoeFar ? TONE.shoe : t === TONE.handFar ? TONE.hand : t);

/* ---------------------------------------------------------------- stations shared by body, thermal and the coarser scales */

type Slice = readonly [number, number, number, number, number];
/** 🧥 The coat on the trunk as horizontal slices [y, back, front, half-width, exponent]: narrow sloping shoulders, the back rounded over the thoracic spine, the front hanging from the bust. */
export const COAT: readonly Slice[] = [
  [0.815, -0.05, 0.13, 0.15, 2.4],
  [0.85, -0.086, 0.168, 0.176, 2.5],
  [0.886, -0.106, 0.187, 0.186, 2.5],
  [0.9, -0.108, 0.189, 0.186, 2.5],
  [0.94, -0.09, 0.2, 0.176, 2.5],
  [1.0, -0.068, 0.216, 0.163, 2.45],
  [1.07, -0.054, 0.234, 0.157, 2.45],
  [1.14, -0.042, 0.242, 0.158, 2.45],
  [1.2, -0.028, 0.232, 0.159, 2.45],
  [1.245, -0.009, 0.214, 0.156, 2.4],
  [1.277, 0.013, 0.197, 0.143, 2.3],
  [1.3, 0.035, 0.18, 0.118, 2.2],
  [1.318, 0.056, 0.164, 0.086, 2.1],
  [1.33, 0.07, 0.152, 0.066, 2],
];
/** 🧥 The coat over the pelvis: the seat of the coat over the buttocks and the belly, widening towards the skirt. */
export const COAT_HIP: readonly Slice[] = [
  [0.72, -0.06, 0.1, 0.14, 2.5],
  [0.76, -0.09, 0.13, 0.17, 2.6],
  [0.8, -0.118, 0.156, 0.19, 2.6],
  [0.83, -0.124, 0.166, 0.197, 2.6],
  [0.86, -0.118, 0.172, 0.194, 2.55],
  [0.92, -0.094, 0.182, 0.18, 2.5],
  [0.97, -0.072, 0.19, 0.166, 2.45],
];
export const coatStations = (): St[] => COAT.map(([y, b, f, rw, e]) => H("torso", y, b, f, rw, e));
export const hipStations = (): St[] => COAT_HIP.map(([y, b, f, rw, e]) => H("pelvis", y, b, f, rw, e));
function sliceAt(table: readonly Slice[], y: number): [number, number, number, number] {
  let i = 0;
  while (i + 2 < table.length && table[i + 1]![0] < y) i++;
  const a = table[i]!;
  const b = table[i + 1]!;
  const t = Math.min(1, Math.max(0, (y - a[0]) / (b[0] - a[0])));
  const m = (k: number): number => a[k]! + (b[k]! - a[k]!) * t;
  return [m(1), m(2), m(3), m(4)];
}
/** 📏 The coat surface at the height `y` and the lateral position `z`: x of the front (+1) or of the back (-1). */
export function coatX(y: number, z: number, side: 1 | -1): number {
  const [back, front, rw, e] = sliceAt(y < 0.9 ? COAT_HIP : COAT, y);
  const k = Math.pow(Math.max(0, 1 - Math.pow(Math.min(1, Math.abs(z) / rw), e)), 1 / e);
  return (back + front) / 2 + (side * k * (front - back)) / 2;
}
/** 🧭 The outward direction of the coat front at (y, z): degrees about the vertical from straight ahead towards the left. */
export const coatYaw = (y: number, z: number): number => (Math.atan2(coatX(y, z - 0.004, 1) - coatX(y, z + 0.004, 1), 0.008) * 180) / Math.PI;

/** 👗 The left half of the coat skirt on the thigh (bone-local, v ahead): it hangs from the seat of the coat to the hem at the knee and widens as an A-line; its centre lies 18 mm inside the thigh and its section is a flat-sided superellipse, so both halves overlap at full depth across the middle (one closed coat front, no fork between the legs) while each follows its thigh. */
export const SKIRT_L: St[] = [st(-0.01, 0.012, -0.018, 0.112, 0.128, 3.2), st(0.03, 0.008, -0.018, 0.118, 0.136, 3.2), st(0.1, 0.006, -0.018, 0.12, 0.137, 3.2), st(0.19, 0.005, -0.018, 0.123, 0.137, 3.2), st(0.28, 0.004, -0.018, 0.127, 0.138, 3), st(0.335, 0.004, -0.018, 0.131, 0.14, 2.7), st(0.352, 0.004, -0.018, 0.1325, 0.141, 2.6)];
/** 👖 The left trouser leg on the thigh, under the coat down to the knee. */
export const THIGH_L: St[] = [st(0.02, -0.004, 0, 0.078, 0.09, 2.1), st(0.12, -0.002, 0, 0.076, 0.086, 2.1), st(0.22, 0.002, 0, 0.07, 0.078, 2), st(0.31, 0.004, 0, 0.063, 0.07, 1.95), st(0.3871, 0.004, 0, 0.0585, 0.065, 1.9)];
/** 🦵 The left trouser leg on the shank: knee, the calf pushing the cloth back, the hem breaking on the shoe. */
export const SHANK_L: St[] = [st(0.0, 0.004, 0, 0.0575, 0.064, 1.9), st(0.06, 0.001, 0, 0.056, 0.062, 1.9), st(0.14, -0.006, 0, 0.0545, 0.062, 1.9), st(0.23, -0.005, 0, 0.0515, 0.058, 1.9), st(0.29, -0.002, 0, 0.0505, 0.0575, 1.9), st(0.322, 0.003, 0, 0.051, 0.0605, 1.95, -4), st(0.342, 0.004, 0, 0.05, 0.059, 2, -8)];

/** 🧥 The left coat sleeve on the upper arm, round about the bone axis: the sleeve head over the deltoid, the elbow at the end. `K` is the lateral half-width as a share of the sagittal one. */
export const K_SLEEVE = 0.9;
const ring = (u: number, rv: number, k = K_SLEEVE): St => st(u, 0, 0, rv * k, rv);
export const SLEEVE_L: St[] = [ring(0.0, 0.039), ring(0.035, 0.051), ring(0.08, 0.056), ring(0.15, 0.054), ring(0.22, 0.0515), ring(0.2939, 0.0495)];
/** 💪 The left sleeve on the forearm down to the cuff. */
export const CUFF_L: St[] = [ring(0.0, 0.0485), ring(0.05, 0.0475), ring(0.12, 0.044), ring(0.175, 0.0415), ring(0.198, 0.041)];

/** 🥾 The left shoe upper as cross slices [x from the ankle, bottom, top, half-width, exponent]: rear part on the foot bone, toe box on the toe bone, overlapping at the ball. */
export const ANKLE_X = 0.05;
export const ANKLE_Y = 0.0736;
export const SHOE_REAR: readonly Slice[] = [
  [-0.058, 0.02, 0.06, 0.016, 2],
  [-0.046, 0.008, 0.084, 0.028, 2.4],
  [-0.02, 0.007, 0.096, 0.0345, 2.4],
  [0.012, 0.007, 0.1, 0.038, 2.4],
  [0.045, 0.007, 0.086, 0.042, 2.4],
  [0.08, 0.007, 0.067, 0.045, 2.5],
  [0.115, 0.007, 0.053, 0.046, 2.6],
  [0.14, 0.007, 0.047, 0.0455, 2.6],
];
export const SHOE_TOE: readonly Slice[] = [
  [0.122, 0.007, 0.051, 0.046, 2.6],
  [0.15, 0.007, 0.044, 0.044, 2.6],
  [0.172, 0.007, 0.036, 0.036, 2.4],
  [0.185, 0.008, 0.028, 0.024, 2.2],
  [0.191, 0.01, 0.021, 0.011, 2],
];
export const shoeRear = (): St[] => SHOE_REAR.map(([x, b, t, rw, e]) => X("foot-l", ANKLE_X + x, b, t, rw, e));
export const shoeToe = (): St[] => SHOE_TOE.map(([x, b, t, rw, e]) => X("toe-l", ANKLE_X + x, b, t, rw, e));

/** 🗣️ The head (bone-local, v is the back of the head): chin, jaw, cheeks, zygoma, brow and the skull under the beret. */
export const HEAD: St[] = [st(-0.027, -0.06, 0, 0.016, 0.014), st(-0.016, -0.048, 0, 0.033, 0.033), st(0.003, -0.031, 0, 0.049, 0.054), st(0.028, -0.013, 0, 0.059, 0.072), st(0.055, -0.004, 0, 0.0655, 0.083), st(0.085, 0.004, 0, 0.07, 0.0885), st(0.11, 0.007, 0, 0.0722, 0.091), st(0.135, 0.009, 0, 0.071, 0.088), st(0.158, 0.01, 0, 0.062, 0.074), st(0.176, 0.01, 0, 0.043, 0.049), st(0.1858, 0.01, 0, 0.013, 0.015)];
/** 🧢 The beret as slices across the head axis [u, v, half-width, half-depth]: a band above the brows and ears, the soft disc pushed back and bulging over it, tilted with the band. */
export const BERET_TILT = 14;
export const BERET: readonly [number, number, number, number][] = [
  [0.108, 0.008, 0.0745, 0.0925],
  [0.12, 0.014, 0.086, 0.106],
  [0.134, 0.022, 0.105, 0.121],
  [0.149, 0.028, 0.112, 0.127],
  [0.164, 0.03, 0.101, 0.113],
  [0.176, 0.03, 0.072, 0.082],
  [0.184, 0.029, 0.03, 0.034],
];
export const beretStations = (): St[] => BERET.map(([u, v, rw, rv]) => st(u, v, 0, rw, rv, 2, BERET_TILT));
export const NECK: St[] = [st(-0.03, 0.006, 0, 0.058, 0.06), st(0.015, 0.002, 0, 0.053, 0.055), st(0.055, -0.003, 0, 0.051, 0.052), st(0.105, -0.008, 0, 0.05, 0.05)];

/* ---------------------------------------------------------------- the hands on the grips */

/** ✊ The grip tube in the frame of the closed left hand: its centre under the palm, its axis (the tube is level while the hand slopes 15 degrees down on it) and its radius. */
export const GRIP = { at: [0.055, -0.03, 0] as V3, slopeDeg: 15, r: 0.017 };
const AX: V3 = [Math.cos((GRIP.slopeDeg * Math.PI) / 180), Math.sin((GRIP.slopeDeg * Math.PI) / 180), 0];
const UP: V3 = [-AX[1], AX[0], 0];
/** 🪝 A point `s` along the grip from its centre (ahead is positive), at the angle `phi` around it (0 outer side, 90 top, -90 underneath, 180 inner side) and the distance `r` from its axis. */
export function around(s: number, phi: number, r: number): V3 {
  const a = (phi * Math.PI) / 180;
  return [GRIP.at[0] + s * AX[0] + r * Math.sin(a) * UP[0], GRIP.at[1] + s * AX[1] + r * Math.sin(a) * UP[1], r * Math.cos(a)];
}
export function quatOf(ex: V3, ey: V3, ez: V3): Quat {
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
export const unit = (a: V3): V3 => { const l = Math.hypot(...a); return [a[0] / l, a[1] / l, a[2] / l]; };
export const cross = (a: V3, b: V3): V3 => [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
export const sub = (a: V3, b: V3): V3 => [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
export const mid = (a: V3, b: V3, t = 0.5): V3 => [a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t, a[2] + (b[2] - a[2]) * t];

/** 🤜 The knuckle line of the left hand: on the outer top of the grip, ahead of its centre, so that the hand lies across the grip at the angle of a power grip. */
export const KNUCKLE = { s: 0.03, phi: 42, r: 0.031 };
/** 🖐️ The plate of the palm from the wrist to the knuckles: centre, rotation and half sizes. */
export function palmFrame(): { at: V3; rotation: Quat; radii: V3 } {
  const k = around(KNUCKLE.s, KNUCKLE.phi, KNUCKLE.r);
  const e1 = unit(k);
  const d = AX[0] * e1[0] + AX[1] * e1[1] + AX[2] * e1[2];
  const e2 = unit(sub(AX, e1.map((x) => x * d) as V3));
  return { at: mid([0, 0, 0], k, 0.52), rotation: quatOf(e1, e2, cross(e1, e2)), radii: [0.047, 0.0385, 0.0135] };
}
/** ☝️ The four fingers of the left hand [name, position of the knuckle along the grip, size]: they run from the knuckle over the outer side of the grip and close underneath it. */
export const FINGERS: readonly [string, number, number][] = [["1", 0.064, 1], ["2", 0.043, 1.05], ["3", 0.022, 1], ["4", 0.002, 0.86]];
export const WRAP: readonly [number, number][] = [[42, 0.031], [-28, 0.0265], [-98, 0.0255], [-158, 0.0245]];
function leftHand(tone: number): Json[] {
  const c = (id: string): Common => ({ id, bone: "hand-l", material: "skin-aged", tone });
  const out: Json[] = [];
  const palm = palmFrame();
  out.push(ellipsoid(c("hand-l"), palm.radii, palm.at, palm.rotation));
  for (const [name, s, k] of FINGERS) {
    const p = WRAP.map(([phi, r], i) => around(s + 0.004 * i, phi, r * (0.94 + 0.06 * k)));
    out.push(capsule(c(`hand-l-finger-${name}`), p[0]!, p[1]!, 0.0092 * k, 0.0087 * k));
    out.push(capsule(c(`hand-l-finger-${name}-mid`), p[1]!, p[2]!, 0.0085 * k, 0.008 * k));
    out.push(capsule(c(`hand-l-finger-${name}-tip`), p[2]!, p[3]!, 0.0079 * k, 0.0072 * k));
  }
  const t0: V3 = [0.014, -0.004, -0.021];
  const t1 = around(0.006, 152, 0.0275);
  const t2 = around(0.036, 178, 0.0265);
  out.push(ellipsoid(c("hand-l-thenar"), [0.026, 0.0135, 0.014], mid(t0, t1, 0.35), quatOf(unit(sub(t1, t0)), unit(cross([0, 0, 1], unit(sub(t1, t0)))), cross(unit(sub(t1, t0)), unit(cross([0, 0, 1], unit(sub(t1, t0))))))));
  out.push(capsule(c("hand-l-thumb"), t0, t1, 0.0118, 0.0102));
  out.push(capsule(c("hand-l-thumb-tip"), t1, t2, 0.01, 0.0086));
  return out;
}

/* ---------------------------------------------------------------- body */

/** 🧍‍♀️ The body solids of the figure at 1:50. */
export function figure50(): Json[] {
  const out: Json[] = [];
  const both = (solids: Json[]): void => {
    for (const s of solids) out.push(s);
    for (const s of solids) out.push(mirrored(s, toRight, near));
  };
  const loden = (id: string, bone: string, tone: number = TONE.coat, role = "body"): Common => ({ id, bone, material: "loden-wool", tone, role });

  out.push(loft(loden("coat-hip", "pelvis"), dense(hipStations(), 3)));
  out.push(loft(loden("coat", "torso"), dense(coatStations(), 4)));

  const leg: Json[] = [];
  const wool = (id: string, bone: string): Common => ({ id, bone, material: "wool-trouser", tone: TONE.trousersFar });
  const shoe = (id: string, bone: string, tone: number = TONE.shoeFar): Common => ({ id, bone, material: "leather-shoe", tone });
  leg.push(loft(loden("skirt-l", "thigh-l", TONE.coatFar), dome(dense(SKIRT_L, 3), -1, 0.03)));
  leg.push(loft(wool("thigh-l", "thigh-l"), dome(dense(THIGH_L, 3), 1, 0.038)));
  leg.push(loft(wool("shank-l", "shank-l"), dome(dense(SHANK_L, 3), -1, 0.038)));
  leg.push(loft({ id: "sock-l", bone: "shank-l", material: "wool-knit", tone: 0.5 }, [H("shank-l", 0.15, ANKLE_X - 0.043, ANKLE_X + 0.036, 0.034), H("shank-l", 0.118, ANKLE_X - 0.044, ANKLE_X + 0.038, 0.0345), H("shank-l", 0.09, ANKLE_X - 0.045, ANKLE_X + 0.04, 0.035)].reverse()));
  leg.push(loft(shoe("shoe-rear-l", "foot-l"), dense(shoeRear(), 3)));
  leg.push(loft(shoe("shoe-toe-l", "toe-l"), dense(shoeToe(), 3)));
  const heelPlan: [number, number][] = [[0, -0.062], [0.018, -0.057], [0.028, -0.043], [0.032, -0.018], [0.034, 0.008], [-0.034, 0.008], [-0.032, -0.018], [-0.028, -0.043], [-0.018, -0.057]];
  const solePlan: [number, number][] = [[0.036, 0.008], [0.04, 0.045], [0.045, 0.085], [0.0475, 0.115], [0.047, 0.14], [-0.047, 0.14], [-0.0475, 0.115], [-0.045, 0.085], [-0.04, 0.045], [-0.036, 0.008]];
  const level = { transform: { rotation: aboutW(-boneDeg("foot-l")) } };
  leg.push(extrude(shoe("shoe-heel-l", "foot-l", 0.1), "zx", heelPlan, -ANKLE_Y, -ANKLE_Y + 0.02, level));
  leg.push(extrude(shoe("shoe-sole-l", "foot-l", 0.1), "zx", solePlan, -ANKLE_Y, -ANKLE_Y + 0.009, level));
  const toePlan: [number, number][] = [[0.0475, -0.01], [0.046, 0.018], [0.039, 0.04], [0.027, 0.055], [0.011, 0.0625], [-0.011, 0.0625], [-0.027, 0.055], [-0.039, 0.04], [-0.046, 0.018], [-0.0475, -0.01]];
  leg.push(extrude(shoe("shoe-toe-sole-l", "toe-l", 0.1), "zx", toePlan, 0, 0.009));
  [0.03, 0.055, 0.08].forEach((x, i) => leg.push(capsule({ id: `shoe-lace-${i}-l`, bone: "foot-l", material: "leather-shoe", tone: 0.5, role: "detail" }, L("foot-l", [ANKLE_X + x, 0.1 - (x - 0.012) * 0.44 + 0.0005, 0.0846 - 0.018]), L("foot-l", [ANKLE_X + x, 0.1 - (x - 0.012) * 0.44 + 0.0005, 0.0846 + 0.018]), 0.004, 0.004)));
  both(leg);

  const arm: Json[] = [];
  arm.push(loft(loden("sleeve-l", "arm-l", TONE.coatFar), dome(dome(dense(SLEEVE_L, 3), 1, 0.04), -1, 0.01, 3)));
  arm.push(loft(loden("forearm-l", "forearm-l", TONE.coatFar), dome(dense(CUFF_L, 3), -1, 0.04)));
  arm.push(loft({ id: "cardigan-cuff-l", bone: "forearm-l", material: "wool-knit", tone: TONE.knit }, [ring(0.185, 0.0335, 0.92), ring(0.213, 0.032, 0.92)]));
  arm.push(loft({ id: "wrist-l", bone: "forearm-l", material: "skin-aged", tone: TONE.handFar }, [ring(0.19, 0.026, 0.92), ring(0.222, 0.0245, 0.92), ring(0.24, 0.024, 0.92)]));
  both(arm);
  for (const s of leftHand(TONE.handFar)) {
    out.push(s);
    out.push(mirrored(s, toRight, near));
  }

  const skin = (id: string, bone = "head", tone: number = TONE.skin, role = "body"): Common => ({ id, bone, material: "skin-aged", tone, role });
  out.push(loft(skin("neck", "neck", 0.8), dense(NECK, 2)));
  out.push(loft({ id: "blouse-collar", bone: "neck", material: "cotton-blouse", tone: TONE.blouse }, [st(-0.008, 0.004, 0, 0.061, 0.063, 2, 14), st(0.014, 0.004, 0, 0.0585, 0.0605, 2, 14), st(0.034, 0.003, 0, 0.056, 0.058, 2, 14)]));
  out.push(loft(loden("collar", "neck", 0.33), [st(-0.04, 0.012, 0, 0.09, 0.08, 2.1, 20), st(-0.016, 0.012, 0, 0.083, 0.076, 2.1, 20), st(0.008, 0.012, 0, 0.074, 0.07, 2.1, 20), st(0.024, 0.012, 0, 0.067, 0.066, 2.1, 20)]));
  out.push(loft(skin("head"), dome(dense(HEAD, 3), -1, 0.005, 2)));
  out.push(loft(skin("head-nose"), dome(dense([st(0.085, -0.082, 0, 0.007, 0.006, 2, -150), st(0.072, -0.091, 0, 0.008, 0.007, 2, -150), st(0.058, -0.1, 0, 0.0098, 0.0085, 2, -150), st(0.047, -0.1065, 0, 0.0128, 0.0098, 2, -142)], 3), 1, 0.007, 2)));
  out.push(ellipsoid(skin("head-nostrils"), [0.008, 0.0105, 0.015], [0.0405, -0.092, 0]));
  out.push(ellipsoid(skin("head-brow"), [0.0075, 0.009, 0.043], [0.092, -0.079, 0]));
  out.push(ellipsoid(skin("head-lips", "head", 0.66), [0.0058, 0.006, 0.02], [0.019, -0.0775, 0]));
  out.push(ellipsoid(skin("head-chin"), [0.0135, 0.0125, 0.019], [-0.016, -0.064, 0]));
  for (const side of [1, -1]) {
    const n = side > 0 ? "l" : "r";
    out.push(ellipsoid(skin(`head-cheek-${n}`), [0.017, 0.008, 0.014], [0.052, -0.05, side * 0.047]));
    out.push(ellipsoid(skin(`head-ear-${n}`, "head", 0.78), [0.03, 0.0155, 0.006], [0.068, 0.014, side * 0.0745], compose(aboutW(-8), aboutU(side * 22))));
    out.push(ellipsoid(skin(`head-eye-${n}`, "head", 0.15, "detail"), [0.005, 0.0045, 0.0105], [0.0795, -0.0775, side * 0.029]));
    out.push(ellipsoid({ id: `hair-temple-${n}`, bone: "head", material: "hair-white", tone: TONE.hair }, [0.03, 0.03, 0.0115], [0.092, 0.03, side * 0.0655]));
  }
  out.push(ellipsoid({ id: "hair-nape", bone: "head", material: "hair-white", tone: TONE.hair }, [0.052, 0.024, 0.066], [0.066, 0.072, 0], aboutW(-7)));

  const felt: Omit<Common, "id"> = { bone: "head", material: "wool-beret", tone: TONE.beret };
  out.push(loft({ id: "beret", ...felt }, dense(beretStations(), 3)));
  out.push(capsule({ id: "beret-stalk", ...felt, tone: 0.2 }, [0.183, 0.03, 0], [0.196, 0.033, 0], 0.0045, 0.004));

  const metal = { bone: "head", material: "spectacle-frame", tone: 0.1, role: "detail" };
  const rim: [number, number][] = Array.from({ length: 9 }, (_, i) => [0.0015 * Math.cos((i * Math.PI) / 4), 0.0195 + 0.0015 * Math.sin((i * Math.PI) / 4)] as [number, number]);
  for (const side of [1, -1]) {
    const n = side > 0 ? "l" : "r";
    out.push(revolve({ id: `glasses-rim-${n}`, ...metal }, rim, [0.0775, -0.0905, side * 0.0295], [0, 1, 0], { slender: true }));
    out.push(capsule({ id: `glasses-temple-${n}`, ...metal }, [0.083, -0.088, side * 0.052], [0.086, 0.012, side * 0.075], 0.0014, 0.0014, { slender: true }));
  }
  out.push(capsule({ id: "glasses-bar", ...metal }, [0.085, -0.0915, -0.0525], [0.085, -0.0915, 0.0525], 0.0015, 0.0015, { slender: true }));

  const trim = (id: string, bone: string, tone = 0.3): Common => ({ id, bone, material: "loden-wool", tone, role: "detail" });
  const onFront = (bone: string, y: number, z: number, proud: number): V3 => L(bone, [coatX(y, z, 1) + proud, y, z]);
  const facing = (bone: string, y: number, z: number): Quat => compose(aboutW(90 - boneDeg(bone)), aboutU(-coatYaw(y, z)));
  const placket: [number, number][] = [[0.72, 0.03], [0.82, 0.03], [0.93, 0.03]];
  out.push(loft(trim("coat-placket-hip", "pelvis", 0.34), placket.map(([y, rw]) => H("pelvis", y, coatX(y, 0.03, 1) - 0.008, coatX(y, 0.03, 1) + 0.003, rw, 3, 0.03))));
  const placketTop: [number, number][] = [[0.9, 0.03], [1.0, 0.03], [1.1, 0.03], [1.2, 0.03], [1.27, 0.03]];
  out.push(loft(trim("coat-placket", "torso", 0.34), placketTop.map(([y, rw]) => H("torso", y, coatX(y, 0.03, 1) - 0.008, coatX(y, 0.03, 1) + 0.003, rw, 3, 0.03))));
  [1.24, 1.13, 1.02].forEach((y, i) => out.push(ellipsoid(trim(`coat-button-${i}`, "torso", 0.12), [0.011, 0.0045, 0.011], onFront("torso", y, 0.03, 0.004), aboutW(90 - boneDeg("torso")))));
  out.push(ellipsoid(trim("coat-button-3", "pelvis", 0.12), [0.011, 0.0045, 0.011], onFront("pelvis", 0.9, 0.03, 0.004), aboutW(90 - boneDeg("pelvis"))));
  for (const side of [1, -1]) {
    const n = side > 0 ? "l" : "r";
    const lapel: [number, number, number][] = [[1.318, 0.05, 0.018], [1.29, 0.062, 0.03], [1.26, 0.06, 0.03], [1.235, 0.05, 0.02]];
    out.push(loft(trim(`coat-lapel-${n}`, "torso", 0.33), lapel.map(([y, z, rw]) => { const x = coatX(y, z, 1); return H("torso", y, x - 0.006, x + 0.004, rw, 2, side * z); }).reverse()));
    out.push({ ...ellipsoid(trim(`coat-pocket-${n}`, "pelvis", 0.28), [0.007, 0.0045, 0.062], onFront("pelvis", 0.83, side * 0.125, 0.0015), compose(facing("pelvis", 0.83, side * 0.125), aboutV2(side * 14))) });
  }
  return out;
}
/** 🔃 A rotation about the outward normal of a part lying on the coat front (its local v after `facing`). */
function aboutV2(deg: number): Quat {
  const a = (deg * Math.PI) / 360;
  return [0, Math.round(Math.sin(a) * 1e5) / 1e5, 0, Math.round(Math.cos(a) * 1e5) / 1e5];
}
