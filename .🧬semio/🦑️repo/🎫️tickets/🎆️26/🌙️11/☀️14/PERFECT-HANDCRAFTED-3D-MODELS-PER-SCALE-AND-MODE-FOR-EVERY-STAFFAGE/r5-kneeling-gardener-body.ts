/** 👩‍🌾️ The body of the round 5 kneeling gardener (round 3 body, remodelled: kyphotic shirt with bust, slimmer sleeves, a closed fist), stated by hand: every section of a 1.66 m, 64 kg woman of 58 in straw hat, linen shirt with rolled sleeves, canvas trousers, gloves and garden clogs, her kneeling mat, hand trowel and flexible tub with weeds, at 1:50 and 1:100. Trunk bones (pelvis, torso, head): u up, v to the back, w to the left. Limb bones: u distal, v to the front, w to the left. Foot and toe: u ahead, v up. Pad and trug: u ahead, v up. Trowel: u from the fist to the tip, v towards the hollow face of the blade.
 * @see ./r3-kneeling-gardener-kit.ts
 * @see ./r5-kneeling-gardener-figure.ts */
import { Kit, aboutU, aboutV, aboutW, quat, type P2, type Row, type V3 } from "./r3-kneeling-gardener-kit.ts";

export const SKIN = "skin";
export const STRAW = "straw";
export const HAIR = "hair";
export const LINEN = "linen";
export const CANVAS = "canvas";
export const GLOVE = "glove";
export const CLOG = "rubber-clog";
export const FOAM = "eva-foam";
export const STEEL = "steel";
export const PE = "polyethylene";
export const T = { skin: 0.86, shirt: 0.88, cuff: 0.8, trousers: 0.45, patch: 0.36, glove: 0.6, clog: 0.3, sole: 0.2, sock: 0.7, hat: 0.8, band: 0.35, hair: 0.6, eye: 0.12, pad: 0.55, tub: 0.72, weeds: 0.42, leaf: 0.3, steel: 0.7, handle: 0.3, bone: 0.85 };
/** 👚️ The 1:50 shirt as `[s, cv, cw, half width, half depth, exponent]`: stations of back (+v) and front (-v) lines of a woman of 58 in a loose linen shirt: the loose shirt blouses over the trouser waistband at the back (0.12 m behind the axis) and ends at the hem over the hips, belly, bust (front 0.151 m at s 0.265), thoracic kyphosis (back 0.138 m over the shoulder blades), sloping shoulders into the collar. */
export const SHIRT50: Row[] = [[-0.075, -0.01, 0, 0.17, 0.122, 2.5], [-0.05, -0.008, 0, 0.171, 0.129, 2.5], [-0.02, -0.0095, 0, 0.168, 0.1295, 2.5], [0.03, -0.014, 0, 0.156, 0.127, 2.4], [0.09, -0.0145, 0, 0.15, 0.1255, 2.4], [0.15, -0.0105, 0, 0.152, 0.1265, 2.4], [0.21, -0.0095, 0, 0.158, 0.1335, 2.3], [0.265, -0.009, 0, 0.163, 0.142, 2.3], [0.32, 0.0055, 0, 0.166, 0.1325, 2.3], [0.37, 0.015, 0, 0.169, 0.116, 2.2], [0.402, 0.0175, 0, 0.17, 0.1005, 2.2], [0.426, 0.0175, 0, 0.152, 0.0855, 2.1], [0.444, 0.0165, 0, 0.112, 0.0745, 2], [0.458, 0.014, 0, 0.074, 0.066, 2], [0.466, 0.011, 0, 0.062, 0.061, 2]];
/** 💪️ The 1:50 sleeve on the upper arm: deltoid cap from 54 mm above the shoulder joint, then the loose sleeve of an upper arm of 0.29 m girth (ISO 7250 women P50) plus 25 mm ease; the cap rows end at s = 0.04, where the sleeve region starts. */
export const SLEEVE50: Row[] = [[-0.054, 0, -0.012, 0.012, 0.013], [-0.047, 0, -0.01, 0.026, 0.028], [-0.034, 0, -0.007, 0.038, 0.041], [-0.014, 0, -0.003, 0.047, 0.05], [0.01, 0.001, 0.001, 0.051, 0.054], [0.04, 0.001, 0.003, 0.051, 0.054], [0.075, 0.002, 0.004, 0.049, 0.053], [0.13, 0.003, 0.003, 0.047, 0.051], [0.19, 0.003, 0.002, 0.046, 0.05], [0.205, 0.003, 0.002, 0.046, 0.05]];
export const SIDES = [["l", 1], ["r", -1]] as const;
export const m = (rows: readonly Row[], g: number): Row[] => rows.map(([s, cv, cw, rx, ry, e]) => [s, cv, cw * g, rx, ry, e] as Row);

/** ✊️ A point of the right fist stated in the frame of the trowel handle (a along the handle towards the blade, b across it in the sagittal plane towards the wrist side, lat to the outside of the body) as hand coordinates; the handle crosses the hand bone at 50 degrees, 70 mm from the wrist. */
export const GRIP_DEG = 50;
const [gc, gs] = [Math.cos((GRIP_DEG * Math.PI) / 180), Math.sin((GRIP_DEG * Math.PI) / 180)];
export const hf = (a: number, b: number, lat: number): V3 => [0.07 + a * gc - b * gs, a * gs + b * gc, -lat];

/** ➰️ The handle axis in the hand frame (the grip direction of `hf`). */
export const GRIP_AXIS: V3 = [gc, gs, 0];
const unit = (a: readonly number[]): number[] => {
  const n = Math.hypot(a[0]!, a[1]!, a[2]!);
  return [a[0]! / n, a[1]! / n, a[2]! / n];
};
const crossV = (a: readonly number[], b: readonly number[]): number[] => [a[1]! * b[2]! - a[2]! * b[1]!, a[2]! * b[0]! - a[0]! * b[2]!, a[0]! * b[1]! - a[1]! * b[0]!];
/** 🧮️ The unit quaternion that carries local x to `X` and local y to `Y` (orthonormal). */
function basisQuat(X: number[], Y: number[]): number[] {
  const Z = crossV(X, Y);
  const [m00, m01, m02, m10, m11, m12, m20, m21, m22] = [X[0]!, Y[0]!, Z[0]!, X[1]!, Y[1]!, Z[1]!, X[2]!, Y[2]!, Z[2]!];
  const tr = m00 + m11 + m22;
  let q: number[];
  if (tr > 0) {
    const t = Math.sqrt(tr + 1) * 2;
    q = [(m21 - m12) / t, (m02 - m20) / t, (m10 - m01) / t, t / 4];
  } else if (m00 > m11 && m00 > m22) {
    const t = Math.sqrt(1 + m00 - m11 - m22) * 2;
    q = [t / 4, (m01 + m10) / t, (m02 + m20) / t, (m21 - m12) / t];
  } else if (m11 > m22) {
    const t = Math.sqrt(1 + m11 - m00 - m22) * 2;
    q = [(m01 + m10) / t, t / 4, (m12 + m21) / t, (m02 - m20) / t];
  } else {
    const t = Math.sqrt(1 + m22 - m00 - m11) * 2;
    q = [(m02 + m20) / t, (m12 + m21) / t, t / 4, (m10 - m01) / t];
  }
  const n = Math.hypot(...q);
  return q.map((c) => Math.round((c / n) * 1e6) / 1e6);
}
/** 🪱️ A loft along a free path in the bone frame: every section perpendicular to the path, its first semi-axis `rx` along `hint` projected into the section plane, the second `ry` across it. */
export function path(k: Kit, id: string, bone: string, material: string, tone: number, points: readonly (readonly [V3, number, number])[], hint: V3, region: string, role = "body"): void {
  const sections = points.map(([p, rx, ry], i) => {
    const a = points[Math.max(0, i - 1)]![0];
    const b = points[Math.min(points.length - 1, i + 1)]![0];
    const t = unit([b[0] - a[0], b[1] - a[1], b[2] - a[2]]);
    const d = hint[0] * t[0]! + hint[1] * t[1]! + hint[2] * t[2]!;
    const X = unit([hint[0] - d * t[0]!, hint[1] - d * t[1]!, hint[2] - d * t[2]!]);
    const Y = crossV(t, X);
    const r = (x: number): number => Math.round(x * 1e5) / 1e5;
    return { at: [r(p[0]), r(p[1]), r(p[2])], rx: r(rx), ry: r(ry), rotation: basisQuat(X, Y) };
  });
  k.solids.push({ id, kind: "loft", bone, material, role, tone, sections, capStart: true, capEnd: true });
  k.region.set(id, region);
}

/** 🥄️ The hollow section of the trowel blade on the plane (v, w): a 2.5 mm sheet 80 mm wide bent to a 12 mm deep scoop, its keel 22 mm below the handle axis. */
export const BLADE: P2[] = (() => {
  const ws = [-0.04, -0.032, -0.022, -0.011, 0, 0.011, 0.022, 0.032, 0.04];
  const keel = (w: number): number => -0.022 + 0.012 * (w / 0.04) ** 2;
  return [...ws.map((w) => [keel(w), w] as P2), ...[...ws].reverse().map((w) => [keel(w) + 0.0025, w] as P2)];
})();

/** 🪣️ The wall of the flexible tub as `[height, radius]`: 4 mm polyethylene, flat base of 0.35 m, top of 0.5 m with a rolled rim, 0.21 m high. */
export const TUB: P2[] = [[0, 0], [0, 0.168], [0.004, 0.18], [0.014, 0.188], [0.2, 0.2445], [0.206, 0.25], [0.21, 0.2495], [0.2115, 0.2455], [0.207, 0.2415], [0.2, 0.2405], [0.016, 0.1845], [0.008, 0.178], [0.004, 0.166], [0.004, 0]];

export function equipment50(k: Kit): void {
  k.box("pad", "pad", FOAM, T.pad, [-0.18, 0.02, 0], [0.76, 0.04, 0.4], 0.016, "pad");
  k.rev("trug", "trug", PE, T.tub, TUB, [0, 1, 0], [0, 0, 0], 1, "trug");
  for (const [sfx, g] of SIDES) k.bar(`trug-handle-${sfx}`, "trug", PE, T.tub, [[-0.062, 0.198, 0.24 * g, 0.0075], [-0.058, 0.222, 0.2415 * g, 0.0075], [-0.04, 0.242, 0.2425 * g, 0.0075], [0, 0.25, 0.2425 * g, 0.0075], [0.04, 0.242, 0.2425 * g, 0.0075], [0.058, 0.222, 0.2415 * g, 0.0075], [0.062, 0.198, 0.24 * g, 0.0075]], "trug", "detail");
  k.ell("weeds-bed", "trug", PE, T.weeds, [0.005, 0.105, 0], [0.205, 0.1, 0.2], "trug");
  k.ell("weeds-heap-1", "trug", PE, T.weeds, [-0.07, 0.19, 0.05], [0.12, 0.062, 0.11], "trug", "body", aboutW(12));
  k.ell("weeds-heap-2", "trug", PE, T.weeds, [0.075, 0.18, -0.06], [0.11, 0.055, 0.1], "trug", "body", aboutW(-14));
  k.ell("weeds-heap-3", "trug", PE, T.weeds, [0.02, 0.215, 0.02], [0.085, 0.05, 0.075], "trug", "body", aboutU(15));
  k.ell("weeds-heap-4", "trug", PE, T.weeds, [0.1, 0.17, 0.09], [0.075, 0.045, 0.07], "trug");
  k.ell("weeds-heap-5", "trug", PE, T.weeds, [-0.1, 0.165, -0.085], [0.08, 0.045, 0.075], "trug");
  const leaf = (id: string, deg: number, pts: readonly (readonly [r: number, y: number, rx: number])[]): void => {
    const [c, n] = [Math.cos((deg * Math.PI) / 180), Math.sin((deg * Math.PI) / 180)];
    path(k, id, "trug", PE, T.leaf, pts.map(([r, y, rx]) => [[r * c, y, r * n], rx, 0.004] as const), [-n, 0, c], "trug", "detail");
  };
  [8, 52, 96, 140, 184, 228, 272, 316].forEach((deg, i) => leaf(`weed-leaf-${i}`, deg + (i % 2) * 9, [[0.04, 0.214, 0.008], [0.09, 0.238, 0.022], [0.14, 0.247, 0.028], [0.19, 0.24, 0.024], [0.226, 0.224, 0.014], [0.243, 0.215, 0.004]]));
  [30, 150, 260].forEach((deg, i) => leaf(`weed-leaf-${8 + i}`, deg, [[0.0, 0.235, 0.008], [0.03, 0.272, 0.02], [0.06, 0.292, 0.02], [0.09, 0.298, 0.004]]));
  const tufts: [V3, V3][] = [[[-0.1, 0.22, 0.09], [-0.135, 0.31, 0.09]], [[-0.02, 0.24, -0.1], [-0.03, 0.335, -0.1]], [[0.1, 0.22, 0.1], [0.14, 0.305, 0.1]]];
  tufts.forEach(([p0, p1], i) => {
    k.cap(`weed-stalk-${i}`, "trug", PE, T.leaf, p0, p1, 0.005, 0.004, "trug", "detail");
    k.ell(`weed-head-${i}`, "trug", PE, T.weeds, [p1[0], p1[1] + 0.008, p1[2]], [0.014, 0.012, 0.014], "trug", "detail");
  });

  k.rev("trowel-handle", "trowel", STEEL, T.handle, [[-0.056, 0], [-0.056, 0.01], [-0.05, 0.0148], [-0.03, 0.0162], [-0.012, 0.016], [0.02, 0.0138], [0.05, 0.0148], [0.07, 0.0152], [0.076, 0.011], [0.076, 0]], [1, 0, 0], [0, 0, 0], 1, "trowel");
  k.bar("trowel-neck", "trowel", STEEL, T.steel, [[0.07, 0, 0, 0.0055], [0.086, -0.001, 0, 0.0055], [0.1, -0.011, 0, 0.0055], [0.112, -0.019, 0, 0.0065, 0.004]], "trowel", "detail");
  k.ext("trowel-blade-shoulder", "trowel", STEEL, T.steel, BLADE, 0.106, 0.15, [0.5, 1], "trowel");
  k.ext("trowel-blade", "trowel", STEEL, T.steel, BLADE, 0.15, 0.21, undefined, "trowel");
  k.ext("trowel-blade-point", "trowel", STEEL, T.steel, BLADE, 0.21, 0.26, [1, 0.14], "trowel");
}

/** 🤚️ Lateral place of the left hand in its bone frame: the bracing arm turns in from the shoulder so the palm lies on the thigh. */
export const HL = -0.036;

export function body50(k: Kit): void {
  equipment50(k);
  k.loft("trousers-seat", "pelvis", CANVAS, T.trousers, [[-0.1, 0.012, 0, 0.03, 0.026, 2.2], [-0.088, 0.012, 0, 0.075, 0.06, 2.3], [-0.055, 0.016, 0, 0.148, 0.102, 2.4], [-0.012, 0.02, 0, 0.172, 0.116, 2.4], [0.04, 0.014, 0, 0.168, 0.111, 2.4], [0.09, 0.006, 0, 0.156, 0.104, 2.3], [0.128, 0, 0, 0.147, 0.099, 2.3]], "trousers");
  k.loft("shirt", "torso", LINEN, T.shirt, SHIRT50, "shirt-torso");
  k.loft("shirt-collar", "torso", LINEN, T.cuff, [[0.45, 0.012, 0, 0.068, 0.065], [0.474, 0.009, 0, 0.059, 0.058]], "shirt-torso", "detail");

  k.loft("neck", "head", SKIN, T.skin, [[-0.035, 0.016, 0, 0.054, 0.052], [0, 0.012, 0, 0.049, 0.05], [0.04, 0.006, 0, 0.047, 0.05], [0.08, 0, 0, 0.048, 0.055]], "face");
  k.loft("head", "head", SKIN, T.skin, [[0.0175, -0.068, 0, 0.016, 0.013, 2.2], [0.032, -0.054, 0, 0.036, 0.034, 2.2], [0.055, -0.036, 0, 0.05, 0.056, 2.2], [0.085, -0.014, 0, 0.06, 0.078], [0.12, -0.006, 0, 0.067, 0.088], [0.155, -0.003, 0, 0.0725, 0.092], [0.19, 0, 0, 0.069, 0.087], [0.215, 0.002, 0, 0.052, 0.066], [0.2325, 0.002, 0, 0.018, 0.024]], "face");
  k.cap("nose", "head", SKIN, T.skin, [0.138, -0.091, 0], [0.097, -0.106, 0], 0.006, 0.0105, "face");
  k.ell("lips", "head", SKIN, 0.7, [0.0655, -0.083, 0], [0.007, 0.0075, 0.021], "face", "detail");
  for (const [sfx, g] of SIDES) {
    k.ell(`ear-${sfx}`, "head", SKIN, T.skin, [0.122, 0.01, 0.07 * g], [0.028, 0.015, 0.0075], "face", "detail", aboutV(-14 * g));
    k.ell(`eye-${sfx}`, "head", SKIN, T.eye, [0.134, -0.0835, 0.03 * g], [0.005, 0.004, 0.0105], "face", "detail");
  }
  k.loft("hair", "head", HAIR, T.hair, [[0.07, 0.05, 0, 0.04, 0.022], [0.1, 0.042, 0, 0.066, 0.046], [0.135, 0.024, 0, 0.076, 0.072], [0.168, 0.008, 0, 0.0765, 0.088], [0.176, 0.006, 0, 0.074, 0.086]], "hat");
  k.ell("hair-bun", "head", HAIR, T.hair, [0.09, 0.09, 0], [0.03, 0.03, 0.034], "hat");
  k.rev("hat-crown", "head", STRAW, T.hat, [[0.166, 0], [0.166, 0.1], [0.2, 0.0985], [0.235, 0.094], [0.254, 0.081], [0.264, 0.055], [0.267, 0]], [1, 0, 0], [0, 0.002, 0], 0.82, "hat");
  k.rev("hat-brim", "head", STRAW, T.hat, [[0.171, 0.074], [0.1675, 0.15], [0.1595, 0.198], [0.152, 0.1975], [0.16, 0.15], [0.1635, 0.074], [0.171, 0.074]], [1, 0, 0], [0, 0.002, 0], 1, "hat");
  k.loft("hat-band", "head", STRAW, T.band, [[0.1715, 0.002, 0, 0.0845, 0.1028], [0.196, 0.002, 0, 0.0835, 0.1012]], "hat", "detail");

  for (const [sfx, g] of SIDES) {
    k.ell(`seat-${sfx}`, "pelvis", CANVAS, T.trousers, [-0.036, 0.07, 0.074 * g], [0.072, 0.058, 0.07], "trousers", "body", aboutU(-8 * g));
    k.loft(`thigh-${sfx}`, `thigh-${sfx}`, CANVAS, T.trousers, m([[-0.05, 0, -0.004, 0.06, 0.07, 2.2], [0, 0.004, -0.002, 0.084, 0.096, 2.2], [0.06, 0.008, 0, 0.088, 0.098, 2.2], [0.14, 0.01, 0, 0.083, 0.091], [0.22, 0.01, 0, 0.075, 0.082], [0.3, 0.008, 0, 0.066, 0.072], [0.36, 0.006, 0, 0.061, 0.066], [0.4055, 0.006, 0, 0.059, 0.064], [0.437, 0.004, 0, 0.052, 0.056], [0.455, 0.003, 0, 0.042, 0.045], [0.4625, 0.002, 0, 0.03, 0.032]], g), "trousers");
    k.loft(`knee-patch-${sfx}`, `thigh-${sfx}`, CANVAS, T.patch, [[0.325, 0.017, 0, 0.05, 0.062, 2.6], [0.4, 0.016, 0, 0.05, 0.062, 2.6], [0.444, 0.013, 0, 0.045, 0.05, 2.6]], "trousers", "detail");
    k.loft(`shank-${sfx}`, `shank-${sfx}`, CANVAS, T.trousers, [[-0.052, 0.004, 0, 0.032, 0.032], [-0.028, 0.004, 0, 0.05, 0.053], [0, 0.004, 0, 0.058, 0.062], [0.05, 0, 0, 0.055, 0.06], [0.12, -0.008, 0, 0.056, 0.064], [0.2, -0.008, 0, 0.051, 0.058], [0.28, -0.004, 0, 0.047, 0.053], [0.34, -0.002, 0, 0.046, 0.051], [0.352, -0.002, 0, 0.0455, 0.0505]], "trousers");
    k.loft(`sock-${sfx}`, `shank-${sfx}`, CLOG, T.sock, [[0.325, -0.002, 0, 0.034, 0.039], [0.365, -0.002, 0, 0.031, 0.037], [0.3961, -0.001, 0, 0.034, 0.039], [0.42, -0.001, 0, 0.03, 0.035]], "clogs");
    k.loft(`sock-foot-${sfx}`, `foot-${sfx}`, CLOG, T.sock, [[-0.056, -0.03, 0, 0.022, 0.026], [-0.032, -0.024, 0, 0.03, 0.036], [0, -0.018, 0, 0.033, 0.04], [0.04, -0.02, 0, 0.036, 0.03], [0.075, -0.026, 0, 0.038, 0.022]], "clogs");
    k.loft(`clog-sole-${sfx}`, `foot-${sfx}`, CLOG, T.sole, m([[-0.072, -0.0625, 0, 0.024, 0.0115], [-0.052, -0.063, 0, 0.036, 0.0124], [0, -0.064, 0.001, 0.04, 0.0114], [0.06, -0.0655, 0.002, 0.044, 0.0099], [0.1225, -0.0664, 0.001, 0.05, 0.009], [0.13, -0.0664, 0.001, 0.05, 0.009]], g), "clogs", "body", 4);
    k.loft(`clog-${sfx}`, `foot-${sfx}`, CLOG, T.clog, m([[-0.069, -0.043, 0, 0.025, 0.013], [-0.05, -0.04, 0, 0.036, 0.018], [-0.012, -0.038, 0.001, 0.04, 0.02], [0.025, -0.032, 0.002, 0.042, 0.028], [0.075, -0.031, 0.002, 0.046, 0.0305], [0.1225, -0.0364, 0.001, 0.051, 0.027], [0.13, -0.0364, 0.001, 0.051, 0.027]], g), "clogs", "body", 2.5);
    k.loft(`clog-strap-${sfx}`, `foot-${sfx}`, CLOG, T.sole, m([[-0.07, -0.022, 0, 0.03, 0.0085], [-0.05, -0.02, 0, 0.0395, 0.0085]], g), "clogs", "detail", 2.5);
    k.loft(`clog-toe-sole-${sfx}`, `toe-${sfx}`, CLOG, T.sole, m([[-0.004, 0.009, 0.001, 0.05, 0.009], [0.04, 0.0095, -0.002, 0.047, 0.0095], [0.07, 0.012, -0.005, 0.037, 0.009], [0.0868, 0.016, -0.007, 0.02, 0.007]], g), "clogs", "body", 4);
    k.loft(`clog-toe-${sfx}`, `toe-${sfx}`, CLOG, T.clog, m([[-0.004, 0.039, 0.001, 0.051, 0.027], [0.03, 0.038, -0.001, 0.049, 0.026], [0.058, 0.034, -0.004, 0.041, 0.022], [0.078, 0.029, -0.006, 0.027, 0.015], [0.0868, 0.026, -0.007, 0.013, 0.008]], g), "clogs", "body", 2.5);

    k.loft(`sleeve-${sfx}`, `upper-arm-${sfx}`, LINEN, T.shirt, m(SLEEVE50, g), "shirt-sleeves");
    k.loft(`sleeve-roll-${sfx}`, `upper-arm-${sfx}`, LINEN, T.cuff, m([[0.192, 0.003, 0.002, 0.048, 0.052], [0.2, 0.003, 0.002, 0.0505, 0.0545], [0.226, 0.002, 0.002, 0.0502, 0.054], [0.234, 0.002, 0.002, 0.047, 0.051]], g), "shirt-sleeves", "detail");
    k.loft(`upper-arm-${sfx}`, `upper-arm-${sfx}`, SKIN, T.skin, m([[0.222, 0.002, 0.002, 0.043, 0.047], [0.258, 0, 0.001, 0.04, 0.042], [0.2925, -0.003, 0, 0.04, 0.039], [0.315, -0.003, 0, 0.035, 0.034], [0.33, -0.002, 0, 0.022, 0.022]], g), "forearms");
  }
  k.loft("forearm-l", "forearm-l", SKIN, T.skin, [[-0.032, 0, 0, 0.022, 0.022], [-0.016, 0, 0, 0.035, 0.034], [0, 0, -0.001, 0.039, 0.037], [0.045, 0.003, -0.006, 0.042, 0.041], [0.11, 0.002, -0.015, 0.038, 0.037], [0.175, 0.001, -0.024, 0.031, 0.027], [0.22, 0, -0.03, 0.0275, 0.021]], "forearms");
  k.loft("glove-cuff-l", "forearm-l", GLOVE, T.glove, [[0.205, 0, -0.028, 0.0315, 0.0255, 2.3], [0.2546, 0, -0.035, 0.031, 0.0225, 2.3], [0.266, 0, -0.0365, 0.029, 0.021, 2.3]], "gloves");
  k.loft("hand-l", "hand-l", GLOVE, T.glove, [[-0.012, 0, HL, 0.029, 0.021, 2.4], [0, 0, HL, 0.03, 0.021, 2.4], [0.03, 0.001, HL - 0.002, 0.041, 0.019, 2.6], [0.07, 0.001, HL - 0.001, 0.045, 0.017, 2.6], [0.098, 0, HL, 0.044, 0.0145, 2.6], [0.106, -0.001, HL, 0.041, 0.012, 2.6]], "gloves");
  for (const [i, w, len] of [[1, -0.031, 0.072], [2, -0.0105, 0.08], [3, 0.0105, 0.074], [4, 0.0305, 0.058]] as const) {
    const p0: V3 = [0.098, 0, HL + w];
    const p1: V3 = [0.098 + 0.55 * len, -0.004, HL + w * 1.12];
    const p2: V3 = [0.098 + len - 0.004, -0.02, HL + w * 1.2];
    k.cap(`finger-${i}-l`, "hand-l", GLOVE, T.glove, p0, p1, 0.0102, 0.0096, "gloves");
    k.cap(`fingertip-${i}-l`, "hand-l", GLOVE, T.glove, p1, p2, 0.0096, 0.0088, "gloves");
  }
  k.cap("thumb-l", "hand-l", GLOVE, T.glove, [0.028, -0.002, HL - 0.034], [0.07, -0.008, HL - 0.064], 0.0125, 0.011, "gloves");
  k.cap("thumbtip-l", "hand-l", GLOVE, T.glove, [0.07, -0.008, HL - 0.064], [0.1, -0.016, HL - 0.07], 0.011, 0.0095, "gloves");

  k.loft("forearm-r", "forearm-r", SKIN, T.skin, [[-0.032, 0, 0, 0.022, 0.022], [-0.016, 0, 0, 0.035, 0.034], [0, 0, 0, 0.039, 0.037], [0.045, 0.003, 0, 0.041, 0.042], [0.11, 0.002, 0, 0.036, 0.039], [0.175, 0.001, 0, 0.027, 0.032], [0.22, 0, 0, 0.021, 0.028]], "forearms");
  k.loft("glove-cuff-r", "forearm-r", GLOVE, T.glove, [[0.205, 0, 0, 0.0255, 0.0315, 2.3], [0.2546, 0, 0, 0.0225, 0.031, 2.3], [0.266, 0, 0, 0.021, 0.029, 2.3]], "gloves");
  k.loft("hand-r", "hand-r", GLOVE, T.glove, [[-0.014, 0, -0.012, 0.017, 0.026, 2.3], [0.015, -0.002, -0.016, 0.019, 0.035, 2.4], [0.045, -0.004, -0.019, 0.018, 0.04, 2.5], [0.07, -0.004, -0.022, 0.016, 0.039, 2.4], [0.086, -0.003, -0.023, 0.013, 0.035, 2.2], [0.098, -0.002, -0.023, 0.007, 0.024, 2]], "gloves");
  k.ell("palm-r", "hand-r", GLOVE, T.glove, [0.03, 0.01, 0.002], [0.03, 0.026, 0.015], "gloves");
  for (const [i, a, r, end] of [[1, 0.0335, 0.0255, 238], [2, 0.0115, 0.0255, 240], [3, -0.0105, 0.0255, 236], [4, -0.0315, 0.0245, 226]] as const) {
    const ring = (deg: number): V3 => hf(a, -r * Math.sin((deg * Math.PI) / 180), r * Math.cos((deg * Math.PI) / 180));
    const w = i === 4 ? 0.0104 : 0.0114;
    path(k, `finger-${i}-r`, "hand-r", GLOVE, T.glove, [[ring(18), w, 0.0094], [ring(55), w, 0.0093], [ring(95), w, 0.0092], [ring(135), w * 0.97, 0.009], [ring(175), w * 0.94, 0.0088], [ring(210), w * 0.9, 0.0085], [ring(end - 8), w * 0.8, 0.0078], [ring(end), w * 0.45, 0.0042]], GRIP_AXIS, "gloves");
  }
  path(k, "thumb-r", "hand-r", GLOVE, T.glove, [[hf(-0.012, 0.04, -0.004), 0.015, 0.013], [hf(-0.004, 0.034, -0.012), 0.0128, 0.0118], [hf(0.016, 0.026, -0.026), 0.0115, 0.0105], [hf(0.036, 0.012, -0.032), 0.0105, 0.0098], [hf(0.05, -0.002, -0.031), 0.0088, 0.0082], [hf(0.058, -0.008, -0.03), 0.004, 0.004]], [0, 0, 1], "gloves");
}
