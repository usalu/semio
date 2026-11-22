/** 👴 The 1:50 body of `humans/elderly-with-cane`, stated section by section: a man of 74, 1.65 m and 74 kg in his stooped stance (thoracic kyphosis with the apex at T7, flat lumbar curve, head carried forward, hips and knees a little bent), tweed jacket over waistcoat and shirt, wool trousers, leather walking shoes, flat cap, wire glasses, the right hand closed round the derby handle of a 78.5 cm ash cane.
 *
 * Figure coordinates (`H`, `X`, `S`, `L`): x ahead, y up, z to the left, default pose `stooped-standing`. Local stations (`st`): u along the bone, v its sagittal normal, w lateral.
 * @see ./rework-04-kit.ts
 * @see ./rework/04.md */
import { join } from "node:path";
import type { Json } from "./volumes-lib.ts";
import { about, add, box, capsule, cross, dense, dome, ellipsoid, loft, makeKit, mirrored, mul, pair, quatOf, readJsonFile, revolve, sub, tube, tubeStations, unit, workDir, type St, type V3 } from "./rework-04-kit.ts";

export const SLUG = "elderly-with-cane";
export const ASSET: Json = readJsonFile(join(workDir(SLUG), "rig.json"));
export const K = makeKit(ASSET);
export const { H, X, S, L, D, W, st } = K;

export const TONE = { jacket: 0.5, jacketFar: 0.43, trousers: 0.4, trousersFar: 0.33, shoe: 0.3, shoeFar: 0.23, sole: 0.16, skin: 0.8, hand: 0.78, handFar: 0.71, hair: 0.92, cap: 0.32, wood: 0.45, shirt: 0.86, waistcoat: 0.34, brass: 0.6, rubber: 0.05, metal: 0.1 } as const;
const FAR: Record<number, number> = { [TONE.jacket]: TONE.jacketFar, [TONE.trousers]: TONE.trousersFar, [TONE.shoe]: TONE.shoeFar, [TONE.hand]: TONE.handFar };
export const far = (t: number): number => FAR[t] ?? t;
/** 👯 The near (right) twin keeps its tone, the far (left) one is a step darker in the side elevation. */
export const sides = (leftSolid: Json): Json[] => [{ ...leftSolid, tone: far(leftSolid.tone) }, mirrored(leftSolid)];

/* ---------------------------------------------------------------- trunk */

/** 🧥 The jacket as horizontal slices [y, back, front, half-width, exponent]. The back runs straight from the buttocks to the hump of the thoracic kyphosis (apex at 1.27 m) and bends forward over it to the collar; the front hangs plumb from chest and belly; the shoulder line falls about 20 degrees. */
export const JACKET: readonly [number, number, number, number, number][] = [
  [0.782, -0.116, 0.166, 0.186, 2.5],
  [0.835, -0.124, 0.174, 0.189, 2.5],
  [0.9, -0.125, 0.182, 0.187, 2.5],
  [0.975, -0.117, 0.19, 0.181, 2.4],
  [1.055, -0.105, 0.197, 0.175, 2.4],
  [1.13, -0.094, 0.201, 0.171, 2.35],
  [1.2, -0.08, 0.204, 0.175, 2.3],
  [1.265, -0.064, 0.204, 0.184, 2.3],
  [1.315, -0.045, 0.199, 0.196, 2.3],
  [1.35, -0.021, 0.19, 0.204, 2.3],
  [1.375, 0.004, 0.177, 0.198, 2.25],
  [1.393, 0.014, 0.165, 0.158, 2.1],
  [1.41, 0.022, 0.156, 0.108, 2],
  [1.426, 0.028, 0.149, 0.076, 2],
];
/** 🪡 The jacket slices with `k` interpolated rows per stated span (Catmull-Rom through the stated rows), so that both jacket lofts are cut from one surface. */
export function jacketRows(k: number): (readonly [number, number, number, number, number])[] {
  const cr = (p0: number, p1: number, p2: number, p3: number, t: number): number => 0.5 * (2 * p1 + (-p0 + p2) * t + (2 * p0 - 5 * p1 + 4 * p2 - p3) * t * t + (-p0 + 3 * p1 - 3 * p2 + p3) * t * t * t);
  const out: (readonly [number, number, number, number, number])[] = [];
  for (let i = 0; i + 1 < JACKET.length; i++) {
    const [a, b, c, d] = [JACKET[Math.max(0, i - 1)]!, JACKET[i]!, JACKET[i + 1]!, JACKET[Math.min(JACKET.length - 1, i + 2)]!];
    for (let j = 0; j < k; j++) out.push(j === 0 ? b : ([0, 1, 2, 3, 4].map((q) => (q === 0 || q === 4 ? b[q]! + (c[q]! - b[q]!) * (j / k) : cr(a[q]!, b[q]!, c[q]!, d[q]!, j / k))) as unknown as readonly [number, number, number, number, number]));
  }
  out.push(JACKET[JACKET.length - 1]!);
  return out;
}
/** ✂️ The waist where the jacket passes from the lumbar bone to the thorax bone: slices 0..WAIST on `torso`, WAIST.. on `chest`. */
export const WAIST = 5;
export const slice = (bone: string, r: readonly [number, number, number, number, number]): St => H(bone, r[0], r[1], r[2], r[3], r[4]);
/** 📏 The jacket surface at the height `y` and the lateral position `z`: x of the front (+1) or of the back (-1). */
export function jacketX(y: number, z: number, side: 1 | -1): number {
  let i = 0;
  while (i + 2 < JACKET.length && JACKET[i + 1]![0] < y) i++;
  const [a, b] = [JACKET[i]!, JACKET[i + 1]!];
  const t = Math.min(1, Math.max(0, (y - a[0]) / (b[0] - a[0])));
  const m = (k: number): number => a[k]! + (b[k]! - a[k]!) * t;
  const [back, front, rw, e] = [m(1), m(2), m(3), m(4)];
  const k = Math.pow(Math.max(0, 1 - Math.pow(Math.min(1, Math.abs(z) / rw), e)), 1 / e);
  return (back + front) / 2 + (side * k * (front - back)) / 2;
}
/** 🧭 The outward normal of the jacket front at (y, z) in figure coordinates. */
export function jacketNormal(y: number, z: number): V3 {
  const dz: V3 = [jacketX(y, z + 0.004, 1) - jacketX(y, z - 0.004, 1), 0, 0.008];
  const dy: V3 = [jacketX(y + 0.004, z, 1) - jacketX(y - 0.004, z, 1), 0.008, 0];
  return unit(cross(dy, dz));
}
/** 🩹 A flat patch lying on the jacket front: centre (y, z), half-height `hy`, half-width `hz`, half-thickness `t`, sunk by `sink`. */
function patch(id: string, bone: string, material: string, tone: number, y: number, z: number, hy: number, hz: number, t: number, sink: number, corner = 0.003): Json {
  const n = jacketNormal(y, z);
  const across = unit(cross([0, 1, 0], n));
  const up = cross(n, across);
  const c: V3 = add([jacketX(y, z, 1), y, z], n, t - sink);
  return box({ id, bone, material, tone, role: "detail" }, [2 * hy, 2 * t, 2 * hz], L(bone, c), quatOf(D(bone, up), D(bone, n)), corner);
}

/** 👖 The trouser seat on the pelvis as horizontal slices: crotch, buttocks, hips, belly up to the waistband. */
export const SEAT: readonly [number, number, number, number, number][] = [
  [0.772, -0.075, 0.075, 0.116, 2.2],
  [0.815, -0.104, 0.096, 0.166, 2.2],
  [0.875, -0.108, 0.11, 0.17, 2.2],
  [0.945, -0.1, 0.135, 0.164, 2.2],
  [1.015, -0.09, 0.16, 0.158, 2.2],
  [1.05, -0.086, 0.164, 0.156, 2.2],
];

/* ---------------------------------------------------------------- limbs, bone-local (left side) */

/** 🦵 The left trouser leg on the thigh: buttock and hamstring behind at the top, the quadriceps in front, the cloth hanging from the seat and pushed forward by the bent knee. */
export const THIGH_L: St[] = [st(-0.012, -0.012, 0, 0.088, 0.1, 2.15), st(0.08, -0.009, 0, 0.087, 0.097, 2.1), st(0.18, -0.004, 0, 0.08, 0.088, 2.05), st(0.28, 0.001, 0, 0.072, 0.08, 2), st(0.36, 0.005, 0, 0.067, 0.075, 1.95), st(0.4046, 0.007, 0, 0.065, 0.073, 1.95)];
/** 🦿 The left trouser leg on the shank: knee, the calf pushing the cloth back, the break over the instep, the hem cut lower at the heel. */
export const SHANK_L: St[] = [st(-0.022, 0.008, 0, 0.065, 0.073, 1.95), st(0.05, 0.001, 0, 0.064, 0.071, 1.95), st(0.14, -0.008, 0, 0.0625, 0.071, 1.95), st(0.24, -0.007, 0, 0.0605, 0.069, 1.95), st(0.31, -0.003, 0, 0.059, 0.069, 1.95), st(0.355, 0.005, 0, 0.0585, 0.074, 2, -6), st(0.39, 0.007, 0, 0.0575, 0.073, 2, -12)];
/** 🧥 The left sleeve on the upper arm: sleeve head over the acromion and deltoid, biceps in front, the elbow at the end. */
export const SLEEVE_L: St[] = [st(-0.036, -0.004, -0.034, 0.03, 0.042), st(-0.022, -0.002, -0.018, 0.046, 0.056), st(0.01, 0.002, -0.004, 0.056, 0.065), st(0.07, 0.004, 0, 0.055, 0.063), st(0.15, 0.003, 0.002, 0.05, 0.057), st(0.24, 0, 0.002, 0.047, 0.053), st(0.3069, -0.003, 0.002, 0.046, 0.051)];
/** 💪 The left sleeve on the forearm down to the cuff. */
export const FORESLEEVE_L: St[] = [st(-0.02, -0.003, 0.002, 0.046, 0.051), st(0.06, 0, 0.001, 0.044, 0.047), st(0.14, 0, 0, 0.041, 0.043), st(0.2, 0, 0, 0.039, 0.0405), st(0.218, 0, 0, 0.0385, 0.0395)];

/** 🥾 The left shoe as cross slices [x from the ankle axis, bottom, top, half-width, exponent]: counter and quarters on the foot bone, vamp and toe box, sole with a 22 mm heel. */
export const ANKLE_X = K.frame("foot-l").o[0];
export const SHOE_Z = K.frame("foot-l").o[2];
export const SHOE_REAR: readonly [number, number, number, number, number][] = [
  [-0.066, 0.03, 0.078, 0.018, 2.2],
  [-0.055, 0.029, 0.099, 0.03, 2.4],
  [-0.03, 0.028, 0.109, 0.036, 2.4],
  [0.005, 0.024, 0.112, 0.039, 2.4],
  [0.04, 0.02, 0.1, 0.042, 2.4],
  [0.08, 0.014, 0.08, 0.046, 2.5],
  [0.12, 0.012, 0.064, 0.049, 2.6],
  [0.15, 0.012, 0.054, 0.05, 2.6],
  [0.165, 0.012, 0.05, 0.05, 2.6],
];
export const SHOE_TOE: readonly [number, number, number, number, number][] = [
  [0.148, 0.012, 0.0545, 0.0505, 2.6],
  [0.18, 0.012, 0.048, 0.049, 2.6],
  [0.2, 0.012, 0.04, 0.041, 2.4],
  [0.212, 0.013, 0.031, 0.028, 2.2],
];
export const SOLE: readonly [number, number, number, number, number][] = [
  [-0.07, 0.02, 0.032, 0.022, 4],
  [-0.057, 0.02, 0.032, 0.035, 5],
  [-0.02, 0.02, 0.031, 0.039, 5],
  [0.03, 0.012, 0.024, 0.04, 5],
  [0.09, 0.002, 0.014, 0.047, 5],
  [0.14, 0, 0.012, 0.052, 5],
  [0.165, 0, 0.012, 0.052, 5],
];
export const TOE_SOLE: readonly [number, number, number, number, number][] = [
  [0.15, 0, 0.012, 0.052, 5],
  [0.185, 0.001, 0.012, 0.05, 5],
  [0.205, 0.003, 0.012, 0.041, 4],
  [0.2175, 0.005, 0.012, 0.024, 3],
];
export const HEEL: readonly [number, number, number, number, number][] = [
  [-0.069, 0, 0.022, 0.021, 4],
  [-0.056, 0, 0.022, 0.034, 5],
  [-0.03, 0, 0.022, 0.0375, 5],
  [0.004, 0, 0.022, 0.0375, 5],
];
export const xs = (bone: string, rows: readonly (readonly [number, number, number, number, number])[]): St[] => rows.map(([x, b, t, rw, e]) => X(bone, ANKLE_X + x, b, t, rw, e, SHOE_Z));

/* ---------------------------------------------------------------- head, bone-local: u up from the atlanto-occipital joint, v towards the occiput, the face at negative v */

/** 🗿 The head as slices across its long axis: [u, face (distance ahead of the joint), occiput (distance behind), half-breadth, exponent]: chin, jaw angle, cheeks, zygomatic arch, temples (breadth 0.151 m), forehead, vault. */
export const HEAD: readonly [number, number, number, number, number][] = [
  [-0.084, 0.092, -0.05, 0.021, 2],
  [-0.07, 0.099, -0.022, 0.039, 2.1],
  [-0.05, 0.1, 0.004, 0.052, 2.2],
  [-0.03, 0.1, 0.03, 0.06, 2.2],
  [-0.01, 0.1, 0.06, 0.066, 2.2],
  [0.015, 0.098, 0.085, 0.071, 2.2],
  [0.045, 0.1, 0.094, 0.075, 2.2],
  [0.075, 0.096, 0.096, 0.0755, 2.2],
  [0.105, 0.081, 0.088, 0.07, 2.1],
  [0.125, 0.056, 0.066, 0.055, 2],
  [0.137, 0.027, 0.036, 0.03, 2],
];
export const headSt = (rows: readonly (readonly [number, number, number, number, number])[], grow = 0): St[] => rows.map(([u, f, b, rw, e]) => st(u, (b - f) / 2, 0, rw + grow, (f + b) / 2 + grow, e));
/** 🧢 The crown of the flat cap: low, pulled forward on to the peak, flat on top. [u, centre v, half-breadth, half-depth, exponent]. */
export const CAP: readonly [number, number, number, number, number][] = [
  [0.06, 0, 0.0785, 0.1, 2.2],
  [0.074, -0.012, 0.085, 0.113, 2.4],
  [0.094, -0.019, 0.0875, 0.119, 2.6],
  [0.116, -0.018, 0.084, 0.114, 2.7],
  [0.134, -0.014, 0.07, 0.094, 2.6],
  [0.145, -0.011, 0.044, 0.06, 2.3],
];
export const capSt = (grow = 0): St[] => CAP.map(([u, v, rw, rv, e]) => st(u, v, 0, rw + grow, rv + grow, e));
/** 🦱 The white hair left below the cap: nape and sides back of the temples. */
export const HAIR: readonly [number, number, number, number, number][] = [
  [-0.034, 0.03, 0.05, 0.036, 2],
  [-0.01, 0.034, 0.068, 0.033, 2.1],
  [0.02, 0.038, 0.075, 0.054, 2.2],
  [0.05, 0.036, 0.078, 0.065, 2.2],
  [0.072, 0.03, 0.078, 0.07, 2.2],
];

/* ---------------------------------------------------------------- hands */

const FINGER_R: Record<string, [number, number, number, number]> = { index: [0.0095, 0.0086, 0.0078, 0.0066], middle: [0.0098, 0.0088, 0.008, 0.0068], ring: [0.0092, 0.0083, 0.0075, 0.0064], little: [0.0082, 0.0074, 0.0067, 0.0058] };
const THUMB_R: [number, number, number, number] = [0.0125, 0.0108, 0.0098, 0.008];
export const boneLen = (id: string): number => (ASSET.rig.bones as Json[]).find((b) => b.id === id).lengthM;
/** ✋ One hand: carpus and metacarpus as one loft (breadth 84 mm at the knuckles, 27 mm thick), thenar and hypothenar eminences, a capsule per phalanx on its own bone. */
export function hand(side: "l" | "r", tone: number): Json[] {
  const k = side === "l" ? 1 : -1;
  const skin = (id: string, bone: string, t = tone): { id: string; bone: string; material: string; tone: number } => ({ id, bone, material: "skin", tone: t });
  const h = `hand-${side}`;
  const palm: St[] = [st(-0.012, 0, 0, 0.0185, 0.027), st(0.008, 0.001, -0.001 * k, 0.018, 0.03), st(0.03, 0.002, -0.002 * k, 0.0155, 0.038), st(0.058, 0.001, -0.002 * k, 0.0142, 0.042), st(0.08, 0, -0.001 * k, 0.0128, 0.042), st(0.09, -0.001, 0, 0.0112, 0.0395)];
  const out: Json[] = [
    loft(skin(h, h), dome(dense(palm, 2), 1, 0.006, 2)),
    ellipsoid(skin(`${h}-thenar`, h), [0.027, 0.0135, 0.012], [0.036, 0.027, -0.011 * k], about([0, 0, 1], 18)),
    ellipsoid(skin(`${h}-hypothenar`, h), [0.03, 0.0105, 0.0115], [0.042, -0.031, -0.007 * k], about([0, 0, 1], -6)),
  ];
  const seg = (bone: string, r0: number, r1: number): Json => capsule(skin(bone, bone), [0, 0, 0], [boneLen(bone), 0, 0], r0, r1);
  ["metacarpal", "proximal", "distal"].forEach((s, i) => out.push(seg(`thumb-${s}-${side}`, THUMB_R[i]!, THUMB_R[i + 1]!)));
  for (const [f, r] of Object.entries(FINGER_R)) ["proximal", "middle", "distal"].forEach((s, i) => out.push(seg(`${f}-${s}-${side}`, r[i]!, r[i + 1]!)));
  return out;
}

/* ---------------------------------------------------------------- cane, bone-local: u down the shaft from the handle axis, v ahead, w lateral */

export const CANE_L: number = boneLen("cane");
/** 🩼 The axis of the derby handle: the rear end droops, the crest lies under the palm, the nose passes over the shaft. [v, u, half-breadth, half-height]. */
export const HANDLE: readonly [number, number, number, number][] = [
  [-0.098, 0.013, 0.0085, 0.0085],
  [-0.088, 0.008, 0.011, 0.0105],
  [-0.07, 0.001, 0.0138, 0.013],
  [-0.045, -0.003, 0.015, 0.014],
  [-0.02, -0.002, 0.0145, 0.0138],
  [0, 0, 0.0135, 0.013],
  [0.02, 0.002, 0.0118, 0.0115],
  [0.031, 0.004, 0.009, 0.009],
];
export function cane(): Json[] {
  const wood = { bone: "cane", material: "ash-wood" };
  return [
    loft({ id: "cane-handle", ...wood, tone: 0.4 }, dome(dome(tubeStations(HANDLE.map(([v, u]) => [u, v, 0] as V3), HANDLE.map(([, , rw, rh]) => [rw, rh] as [number, number]), [-1, 0, 0]), -1, 0.006, 2), 1, 0.006, 2)),
    revolve({ id: "cane-collar", bone: "cane", material: "ash-wood", tone: TONE.brass, role: "detail" }, [[0.008, 0], [0.008, 0.0128], [0.012, 0.0134], [0.03, 0.0134], [0.034, 0.0128], [0.034, 0]], [0, 0, 0], [1, 0, 0]),
    revolve({ id: "cane-shaft", ...wood, tone: TONE.wood }, [[0, 0], [0, 0.0114], [0.2, 0.011], [0.45, 0.0102], [0.7, 0.0092], [0.735, 0.009], [0.735, 0]], [0, 0, 0], [1, 0, 0]),
    revolve({ id: "cane-ferrule", bone: "cane", material: "rubber", tone: TONE.rubber }, [[CANE_L - 0.042, 0], [CANE_L - 0.042, 0.0118], [CANE_L - 0.03, 0.0124], [CANE_L - 0.012, 0.0142], [CANE_L - 0.003, 0.0152], [CANE_L, 0.0138], [CANE_L, 0]], [0, 0, 0], [1, 0, 0]),
  ];
}

/* ---------------------------------------------------------------- body */

export function body50(): Json[] {
  const out: Json[] = [];
  const tweed = (id: string, bone: string, tone: number = TONE.jacket, role?: string): { id: string; bone: string; material: string; tone: number; role?: string } => ({ id, bone, material: "tweed", tone, ...(role ? { role } : {}) });

  out.push(loft({ id: "pelvis-trousers", bone: "pelvis", material: "wool-trouser", tone: TONE.trousers }, dome(dense(SEAT.map((r) => slice("pelvis", r)), 2), -1, 0.02, 2)));
  const rows = jacketRows(3);
  const waist = rows.findIndex((r) => r[0] === JACKET[WAIST]![0]);
  out.push(loft(tweed("jacket-skirt", "torso"), rows.slice(0, waist + 1).map((r) => slice("torso", r))));
  const tongue = rows.slice(waist - 3, waist).map((r) => [r[0], r[1] + 0.005, r[2] - 0.005, r[3] - 0.005, r[4]] as const);
  out.push(loft(tweed("jacket", "chest"), dome([...tongue, ...rows.slice(waist)].map((r) => slice("chest", r)), 1, 0.008, 2)));

  const neckSkin = { bone: "neck", material: "skin", tone: 0.78 };
  out.push(loft({ id: "neck", ...neckSkin }, dense([st(-0.035, -0.008, 0, 0.064, 0.068), st(0.02, -0.01, 0, 0.058, 0.061), st(0.065, -0.012, 0, 0.0555, 0.058), st(0.105, -0.012, 0, 0.0545, 0.058), st(0.128, -0.012, 0, 0.052, 0.054)], 2)));
  out.push(loft({ id: "shirt-collar", bone: "neck", material: "cotton-flannel", tone: TONE.shirt }, [S("neck", [0.09, 1.402, 0], [0, 0, 1], [1, 0.3, 0], 0.065, 0.066, 2.1), S("neck", [0.101, 1.424, 0], [0, 0, 1], [1, 0.3, 0], 0.0625, 0.064, 2.1), S("neck", [0.111, 1.444, 0], [0, 0, 1], [1, 0.3, 0], 0.06, 0.062, 2.1)]));
  const collarPath: V3[] = [[0.166, 1.372, 0.048], [0.14, 1.404, 0.067], [0.096, 1.426, 0.074], [0.05, 1.434, 0.062], [0.024, 1.435, 0.034], [0.015, 1.435, 0]];
  const collarAll = [...collarPath, ...collarPath.slice(0, -1).reverse().map((p) => [p[0], p[1], -p[2]] as V3)];
  out.push(tube(tweed("collar", "chest", 0.46), collarAll.map((p) => L("chest", p)), collarAll.map((_, i) => (i === 0 || i === collarAll.length - 1 ? [0.0075, 0.01] : [0.0075, 0.016]) as [number, number]), D("chest", [0.25, 1, 0])));

  const lapel = (k: 1 | -1): Json => {
    const rows: [number, number, number][] = [[1.372, 0.058, 0.022], [1.33, 0.066, 0.036], [1.28, 0.056, 0.036], [1.225, 0.04, 0.028], [1.175, 0.024, 0.016], [1.15, 0.016, 0.008]];
    return loft(tweed(`lapel-${k > 0 ? "l" : "r"}`, "chest", 0.45, "detail"), rows.map(([y, z, hw]) => {
      const n = jacketNormal(y, z * k);
      const across = unit(cross([0, 1, 0], n));
      return S("chest", add([jacketX(y, z * k, 1), y, z * k], n, 0.0015), across, n, hw, 0.0052, 2.6);
    }));
  };
  out.push(lapel(1), lapel(-1));
  const front = (id: string, material: string, tone: number, rows: [number, number][], sink: number): Json => loft({ id, bone: "chest", material, tone, role: "detail" }, rows.map(([y, hw]) => S("chest", [jacketX(y, 0, 1) - sink, y, 0], [0, 0, 1], [1, 0, 0], hw, 0.0055, 2.4)));
  out.push(front("shirt-front", "cotton-flannel", TONE.shirt, [[1.392, 0.05], [1.36, 0.046], [1.32, 0.034], [1.285, 0.022]], 0.004));
  out.push(front("waistcoat", "wool-knit", TONE.waistcoat, [[1.3, 0.03], [1.26, 0.03], [1.2, 0.019], [1.158, 0.008]], 0.0035));
  out.push(patch("jacket-pocket-l", "torso", "tweed", 0.38, 0.985, 0.112, 0.024, 0.07, 0.0045, 0.003));
  out.push(patch("jacket-pocket-r", "torso", "tweed", 0.38, 0.985, -0.112, 0.024, 0.07, 0.0045, 0.003));
  out.push(patch("jacket-breast-pocket", "chest", "tweed", 0.38, 1.243, 0.1, 0.011, 0.048, 0.0042, 0.003, 0.002));
  [1.13, 1.035].forEach((y, i) => {
    const n = jacketNormal(y, 0.012);
    out.push(ellipsoid({ id: `jacket-button-${i}`, bone: i === 0 ? "chest" : "torso", material: "tweed", tone: 0.1, role: "detail" }, [0.01, 0.0045, 0.01], L(i === 0 ? "chest" : "torso", add([jacketX(y, 0.012, 1), y, 0.012], n, 0.002)), quatOf(D(i === 0 ? "chest" : "torso", [0, 1, 0]), D(i === 0 ? "chest" : "torso", n))));
  });

  const wool = { material: "wool-trouser", tone: TONE.trousers };
  out.push(...sides(loft({ id: "thigh-l", bone: "thigh-l", ...wool }, dome(dense(THIGH_L, 3), 1, 0.03, 3))));
  out.push(...sides(loft({ id: "shank-l", bone: "shank-l", ...wool }, dome(dense(SHANK_L, 3), -1, 0.03, 3))));
  out.push(...sides(loft({ id: "sock-l", bone: "shank-l", material: "wool-sock", tone: 0.22 }, [st(0.3, -0.002, 0, 0.041, 0.047), st(0.35, -0.001, 0, 0.038, 0.044), st(0.4, 0.002, 0, 0.037, 0.045)])));
  const leather = { material: "leather-shoe", tone: TONE.shoe };
  out.push(...sides(loft({ id: "shoe-rear-l", bone: "foot-l", ...leather }, dome(dense(xs("foot-l", SHOE_REAR), 2), -1, 0.004, 2))));
  out.push(...sides(loft({ id: "shoe-toe-l", bone: "toe-l", ...leather }, dome(dense(xs("toe-l", SHOE_TOE), 2), 1, 0.006, 2))));
  out.push(...sides(loft({ id: "shoe-sole-l", bone: "foot-l", material: "leather-shoe", tone: TONE.sole }, dense(xs("foot-l", SOLE), 2))));
  out.push(...sides(loft({ id: "shoe-toe-sole-l", bone: "toe-l", material: "leather-shoe", tone: TONE.sole }, dome(xs("toe-l", TOE_SOLE), 1, 0.003, 2))));
  out.push(...sides(loft({ id: "shoe-heel-l", bone: "foot-l", material: "leather-shoe", tone: TONE.sole }, dome(xs("foot-l", HEEL), -1, 0.003, 2))));

  out.push(...sides(loft(tweed("sleeve-l", "arm-l"), dome(dome(dense(SLEEVE_L, 3), -1, 0.012, 2), 1, 0.03, 3))));
  out.push(...sides(loft(tweed("forearm-l", "forearm-l"), dome(dense(FORESLEEVE_L, 2), -1, 0.03, 3))));
  out.push(...sides(loft({ id: "shirt-cuff-l", bone: "forearm-l", material: "cotton-flannel", tone: TONE.shirt }, [st(0.2, 0, 0, 0.031, 0.036), st(0.231, 0, 0, 0.0285, 0.034)])));
  out.push(...sides(loft({ id: "wrist-l", bone: "forearm-l", material: "skin", tone: TONE.hand }, [st(0.2, 0, 0, 0.022, 0.03), st(0.23, 0, 0, 0.0195, 0.0285), st(0.25, 0, 0, 0.019, 0.0275)])));
  out.push(...hand("l", TONE.handFar), ...hand("r", TONE.hand));

  const skin = (id: string, tone: number = TONE.skin, role?: string): { id: string; bone: string; material: string; tone: number; role?: string } => ({ id, bone: "head", material: "skin", tone, ...(role ? { role } : {}) });
  out.push(loft(skin("head"), dome(dome(dense(headSt(HEAD), 3), -1, 0.005, 2), 1, 0.004, 2)));
  out.push(loft(skin("head-nose", 0.76), [S("head", W("head", [0.03, -0.097, 0]), [0, 0, 1], D0([0.03, -0.097, 0], [-0.016, -0.126, 0]), 0.0065, 0.0055), S("head", W("head", [0.008, -0.11, 0]), [0, 0, 1], D0([0.03, -0.097, 0], [-0.016, -0.126, 0]), 0.009, 0.008), S("head", W("head", [-0.012, -0.12, 0]), [0, 0, 1], D0([0.03, -0.097, 0], [-0.016, -0.126, 0]), 0.0145, 0.0115), S("head", W("head", [-0.021, -0.113, 0]), [0, 0, 1], D0([0.03, -0.097, 0], [-0.016, -0.126, 0]), 0.016, 0.008)]));
  out.push(ellipsoid(skin("head-brow", 0.76), [0.0085, 0.01, 0.056], [0.036, -0.095, 0]));
  out.push(ellipsoid(skin("head-chin", 0.78), [0.014, 0.012, 0.021], [-0.072, -0.09, 0]));
  out.push(ellipsoid(skin("head-lips", 0.6, "detail"), [0.0058, 0.0062, 0.023], [-0.043, -0.0965, 0]));
  out.push(...pair(ellipsoid(skin("head-cheek-l", 0.79), [0.019, 0.011, 0.02], [-0.004, -0.076, 0.047], quatOf([1, 0, 0], [0, 0.94, -0.34]))));
  out.push(...pair(ellipsoid(skin("head-eye-l", 0.15, "detail"), [0.0052, 0.0048, 0.0115], [0.017, -0.0895, 0.031])));
  out.push(...pair(ellipsoid(skin("head-ear-l", 0.76), [0.03, 0.015, 0.0065], [0.006, 0.014, 0.0745], quatOf([1, 0, 0.1], [-0.03, 0.96, 0.27]))));
  const hair = { bone: "head", material: "hair", tone: TONE.hair };
  out.push(loft({ id: "hair-white", ...hair }, dome(dense(HAIR.map(([u, v, rw, rv, e]) => st(u, v, 0, rw, rv, e)), 2), -1, 0.006, 2)));
  out.push(loft({ id: "cap", bone: "head", material: "cap-tweed", tone: TONE.cap }, dome(dense(capSt(), 3), 1, 0.006, 2)));
  out.push(loft({ id: "cap-brim", bone: "head", material: "cap-tweed", tone: 0.26 }, [st(0.058, -0.162, 0, 0.03, 0.004, 2.6, 90), st(0.059, -0.154, 0, 0.056, 0.0042, 2.6, 90), st(0.062, -0.134, 0, 0.074, 0.0048, 2.6, 90), st(0.066, -0.105, 0, 0.08, 0.0052, 2.6, 90), st(0.07, -0.075, 0, 0.078, 0.0052, 2.6, 90)].map((q) => ({ ...q, ex: [0, 0, 1] as V3, ey: [1, 0, 0] as V3 }))));
  const metal = { bone: "head", material: "spectacle-frame", tone: TONE.metal, role: "detail" };
  const ring: [number, number][] = Array.from({ length: 9 }, (_, i) => [0.0016 * Math.cos((i * Math.PI) / 4), 0.021 + 0.0016 * Math.sin((i * Math.PI) / 4)] as [number, number]);
  const rim = revolve({ id: "glasses-rim-l", ...metal }, ring, [0.017, -0.104, 0.0315], [0, 1, 0], { slender: true });
  out.push(rim, mirrored(rim));
  const temple = capsule({ id: "glasses-temple-l", ...metal }, [0.023, -0.104, 0.052], [0.026, 0.006, 0.0785], 0.0015, 0.0015, { slender: true });
  out.push(temple, mirrored(temple));
  out.push(capsule({ id: "glasses-bar", ...metal }, [0.0375, -0.105, -0.047], [0.0375, -0.105, 0.047], 0.0012, 0.0012, { slender: true }));

  out.push(...cane());
  return out;

  function D0(a: V3, b: V3): V3 {
    const t = unit(sub(W("head", b), W("head", a)));
    return unit(cross([0, 0, 1], t).map((c) => -c) as V3);
  }
}
void mul;
