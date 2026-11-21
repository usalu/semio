/** 🧑‍🎤️ The 1:50 body of the round 3 seated guitarist, every section stated by hand: a man of 1.76 m in a flannel shirt over a T-shirt, jeans and leather boots (rest frames of the rig: limbs hang, the pose seats him), his two hands shaped for the neck and for the strings, a dreadnought guitar and a 24 inch wooden stool.
 * Rows are `[s, cv, cw, rx, ry, e?]`: s along the bone, cv along its normal (trunk and head: backwards; limbs: forwards; foot: upwards), cw to the left, rx the lateral and ry the normal semi-axis.
 * @see ./r3-seated-guitarist-ref.ts
 * @see ./r3-humans-seated-guitarist.md */
import { ACROSS_V, Kit, aboutU, aboutV, aboutW, dome, quat, type Row, type V3 } from "./r3-seated-guitarist-kit.ts";
import { BOUT, GUITAR, STOOL, depthAt, fretA, halfWidth, legAt, onGuitar } from "./r3-seated-guitarist-ref.ts";

export const TONE = { skin: 0.74, hair: 0.2, flannel: 0.52, tee: 0.86, denim: 0.32, leather: 0.2, sole: 0.1, spruce: 0.84, mahogany: 0.42, rosewood: 0.24, pine: 0.66, steel: 0.7, dark: 0.07 } as const;

/* ---------------------------------------------------------------- stool */

export const SEAT_PROFILE: readonly (readonly [number, number])[] = [[STOOL.seatTop - STOOL.seatT, 0], [STOOL.seatTop - STOOL.seatT, 0.148], [STOOL.seatTop - 0.033, 0.16], [STOOL.seatTop - 0.02, 0.165], [STOOL.seatTop - 0.008, 0.165], [STOOL.seatTop, 0.157], [STOOL.seatTop - 0.001, 0.12], [STOOL.seatTop - 0.007, 0]];
export const LEGS: readonly (readonly [id: string, sx: number, sz: number])[] = [["fl", 1, 1], ["fr", 1, -1], ["bl", -1, 1], ["br", -1, -1]];
export function stool(k: Kit): void {
  k.rev("stool-seat", "stool", "pine", TONE.pine, SEAT_PROFILE, [1, 0, 0], [0, 0, 0], "stool");
  for (const [id, sx, sz] of LEGS) {
    const rows: Row[] = [0, 0.012, 0.15, 0.3, 0.45, 0.585].map((y, i) => [y, -sx * legAt(y), sz * legAt(y), [0.0145, 0.0155, 0.017, 0.0185, 0.019, 0.0175][i]!, [0.0145, 0.0155, 0.017, 0.0185, 0.019, 0.0175][i]!]);
    k.loft(`stool-leg-${id}`, "stool", "pine", TONE.pine - 0.06, rows, "stool");
  }
  const rung = (id: string, y: number, a: readonly [number, number], b: readonly [number, number]): void => {
    const o = legAt(y);
    k.cap(`stool-rung-${id}`, "stool", "pine", TONE.pine - 0.1, [y, -a[0] * o, a[1] * o], [y, -b[0] * o, b[1] * o], STOOL.rungR, STOOL.rungR, "stool");
  };
  rung("front", STOOL.rungFront, [1, 1], [1, -1]);
  rung("back", STOOL.rungFront, [-1, 1], [-1, -1]);
  rung("left", STOOL.rungSide, [1, 1], [-1, 1]);
  rung("right", STOOL.rungSide, [1, -1], [-1, -1]);
}

/* ---------------------------------------------------------------- guitar */

export const NECK_HALF = (a: number): number => 0.0215 + ((0.028 - 0.0215) * (GUITAR.nut - a)) / (GUITAR.nut - GUITAR.body);
export const HEAD_DROP = (a: number): number => -Math.tan((14 * Math.PI) / 180) * (a - GUITAR.nut);
export const TUNERS: readonly number[] = [0.9, 0.94, 0.98];
export const STRING_B: readonly number[] = [0.0265, 0.0159, 0.0053, -0.0053, -0.0159, -0.0265];
export const HEAD_HALF = (a: number): number => (a < GUITAR.nut + 0.03 ? 0.0225 + ((a - GUITAR.nut) / 0.03) * 0.0125 : 0.035 + ((a - GUITAR.nut - 0.03) / 0.134) * 0.004);
export function guitar(k: Kit): void {
  const body = (id: string, material: string, tone: number, inset: number, top: number, bottom: (a: number) => number, o: object = {}): void => {
    const rows: Row[] = BOUT.map(([a, half]) => {
      const [c1, c0] = [top, bottom(a)];
      const p = onGuitar(a, 0, (c0 + c1) / 2);
      return [p[0], p[1], p[2], Math.max(half - inset, 0.02), (c1 - c0) / 2, 9];
    });
    k.loft(id, "guitar", material, tone, rows, "guitar", o);
  };
  body("guitar-body", "mahogany", TONE.mahogany, 0, 0, (a) => -depthAt(a));
  body("guitar-top", "spruce", TONE.spruce, 0.004, 0.0025, () => -0.002, { texture: true });
  k.ell("guitar-soundhole", "guitar", "rosewood", TONE.dark, onGuitar(GUITAR.soundhole, 0, 0.0028), [GUITAR.soundholeR, 0.0012, GUITAR.soundholeR], "guitar", { texture: true, role: "detail" });
  k.ell("guitar-pickguard", "guitar", "rosewood", 0.14, onGuitar(0.335, -0.082, 0.0028), [0.062, 0.001, 0.036], "guitar", { texture: true, role: "detail", rotation: aboutV(-18) });
  k.box("guitar-bridge", "guitar", "rosewood", TONE.rosewood, onGuitar(GUITAR.saddle, 0, 0.007), [0.034, 0.0095, 0.152], 0.003, "guitar", { role: "detail" });
  const neck: Row[] = [0.5, 0.56, 0.68, 0.8, GUITAR.nut].map((a) => {
    const t = 0.0155 + ((0.0185 - 0.0155) * (GUITAR.nut - a)) / (GUITAR.nut - 0.5);
    const p = onGuitar(a, 0, 0.0005 - t / 2);
    return [p[0], p[1], p[2], NECK_HALF(a), t / 2 + 0.002, 2.6];
  });
  k.loft("guitar-neck", "guitar", "mahogany", TONE.mahogany + 0.06, neck, "guitar");
  const board: Row[] = [GUITAR.fretboardEnd, 0.5, 0.68, GUITAR.nut + 0.004].map((a) => {
    const p = onGuitar(a, 0, 0.0028);
    return [p[0], p[1], p[2], NECK_HALF(Math.min(a, GUITAR.nut)) + 0.0003, 0.0039, 8];
  });
  k.loft("guitar-fretboard", "guitar", "rosewood", TONE.rosewood, board, "guitar");
  const heel = onGuitar(0.5225, 0, 0);
  k.loft("guitar-heel", "guitar", "mahogany", TONE.mahogany + 0.06, [[heel[1] + 0.012, heel[0], heel[2], 0.019, 0.027, 3], [heel[1] + 0.05, heel[0] - 0.004, heel[2], 0.013, 0.02, 2.6], [heel[1] + 0.088, heel[0] - 0.008, heel[2], 0.008, 0.012, 2.4]], "guitar", { across: ACROSS_V });
  const head: Row[] = [GUITAR.nut - 0.002, GUITAR.nut + 0.03, GUITAR.nut + 0.09, GUITAR.nut + 0.15, GUITAR.length].map((a, i) => {
    const p = onGuitar(a, 0, HEAD_DROP(a) - 0.008);
    return [p[0], p[1], p[2], i === 4 ? 0.033 : HEAD_HALF(a), 0.0075, 7];
  });
  k.loft("guitar-head", "guitar", "mahogany", TONE.mahogany - 0.08, head, "guitar");
  TUNERS.forEach((a, i) => {
    for (const side of [1, -1]) {
      const p = onGuitar(a, side * (HEAD_HALF(a) + 0.0105), HEAD_DROP(a) - 0.012);
      k.ell(`guitar-tuner-${side > 0 ? "b" : "t"}${i + 1}`, "guitar", "steel", TONE.steel, p, [0.0095, 0.0055, 0.0135], "guitar", { role: "detail" });
    }
  });
  STRING_B.forEach((b, i) => {
    const post = TUNERS[i < 3 ? i : 5 - i]!;
    const side = i < 3 ? 1 : -1;
    const r = [0.00065, 0.00055, 0.00045, 0.00035, 0.0003, 0.0003][i]!;
    const pts = [onGuitar(GUITAR.saddle, b * 1.28, 0.0125), onGuitar(GUITAR.nut, b * 0.68, 0.0085), onGuitar(post, side * 0.019, HEAD_DROP(post) + 0.004)].map((p) => [...p, r]);
    k.bar(`guitar-string-${i + 1}`, "guitar", "steel", TONE.steel, pts, "guitar", { slender: true, role: "texture" });
  });
}

/* ---------------------------------------------------------------- trunk and head */

export const PELVIS: Row[] = [[-0.088, 0.018, 0, 0.128, 0.082, 2.6], [-0.07, 0.02, 0, 0.164, 0.106, 2.7], [-0.03, 0.02, 0, 0.181, 0.117, 2.5], [0.02, 0.018, 0, 0.182, 0.116, 2.4], [0.07, 0.013, 0, 0.171, 0.11, 2.3], [0.118, 0.007, 0, 0.158, 0.105, 2.3]];
export const SHIRT: Row[] = [[-0.07, 0.004, 0, 0.172, 0.114, 2.4], [-0.02, 0.004, 0, 0.169, 0.116, 2.4], [0.05, 0, 0, 0.158, 0.113, 2.3], [0.12, -0.004, 0, 0.158, 0.114, 2.3], [0.2, -0.006, 0, 0.165, 0.122, 2.3], [0.28, -0.004, 0, 0.172, 0.127, 2.3], [0.345, 0.004, 0, 0.183, 0.12, 2.4], [0.395, 0.01, 0, 0.201, 0.1, 2.6], [0.432, 0.012, 0, 0.205, 0.082, 2.6], [0.456, 0.012, 0, 0.176, 0.071, 2.4], [0.472, 0.01, 0, 0.122, 0.065, 2.3], [0.485, 0.008, 0, 0.085, 0.062, 2.2]];
/** 👕️ The front of the shirt at height `s` on the centre line (bone normal coordinate, negative is ahead). */
export function shirtFront(s: number, w = 0): number {
  for (let i = 1; i < SHIRT.length; i++) if (s <= SHIRT[i]![0]) {
    const [a, b] = [SHIRT[i - 1]!, SHIRT[i]!];
    const t = (s - a[0]) / (b[0] - a[0]);
    const [cv, rx, ry, e] = [a[1] + (b[1] - a[1]) * t, a[3] + (b[3] - a[3]) * t, a[4] + (b[4] - a[4]) * t, a[5]! + (b[5]! - a[5]!) * t];
    return cv - ry * (1 - Math.abs(w / rx) ** e) ** (1 / e);
  }
  return SHIRT[SHIRT.length - 1]![1] - SHIRT[SHIRT.length - 1]![4];
}
export const HEAD: Row[] = [[0.022, -0.086, 0, 0.021, 0.017, 2.2], [0.036, -0.074, 0, 0.04, 0.036, 2.3], [0.062, -0.054, 0, 0.055, 0.06, 2.3], [0.09, -0.037, 0, 0.066, 0.08, 2.2], [0.125, -0.03, 0, 0.0735, 0.092, 2.1], [0.16, -0.028, 0, 0.076, 0.097, 2.1], [0.2, -0.026, 0, 0.072, 0.092, 2.1], [0.232, -0.024, 0, 0.054, 0.07, 2], [0.249, -0.024, 0, 0.028, 0.036, 2]];
export const HAIR: Row[] = [[0.085, 0.03, 0, 0.066, 0.038, 2.2], [0.11, 0.022, 0, 0.0775, 0.052, 2.2], [0.145, 0.006, 0, 0.0815, 0.069, 2.1], [0.185, -0.012, 0, 0.082, 0.091, 2.1], [0.215, -0.02, 0, 0.0745, 0.092, 2.1], [0.243, -0.023, 0, 0.055, 0.07, 2], [0.2595, -0.024, 0, 0.026, 0.034, 2]];
export const NECK: Row[] = [[-0.06, -0.03, 0, 0.064, 0.062], [-0.01, -0.03, 0, 0.059, 0.059], [0.04, -0.034, 0, 0.056, 0.056], [0.09, -0.022, 0, 0.055, 0.06]];

export function trunk(k: Kit): void {
  k.loft("jeans-seat", "pelvis", "denim", TONE.denim, PELVIS, "jeans");
  k.loft("shirt", "torso", "flannel", TONE.flannel, SHIRT, "shirt-torso");
  const collar = Array.from({ length: 11 }, (_, i) => {
    const t = (-150 + 30 * i) * (Math.PI / 180);
    const front = Math.abs(i - 5) / 5;
    return [0.493 - 0.03 * front ** 2 - (front === 1 ? 0.012 : 0), 0.004 + 0.072 * Math.cos(t) - (front === 1 ? 0.004 : 0), 0.073 * Math.sin(t) * (front === 1 ? 1.12 : 1), front === 1 ? 0.012 : 0.02, 0.0042];
  });
  k.bar("shirt-collar", "torso", "flannel", TONE.flannel + 0.08, collar, "shirt-torso", { e: 3 });
  k.ell("tee-neck", "torso", "cotton-jersey", TONE.tee, [0.452, -0.05, 0], [0.03, 0.02, 0.044], "shirt-torso");
  const stations = [0.42, 0.36, 0.28, 0.2, 0.12, 0.04, -0.03, -0.074];
  k.bar("shirt-placket", "torso", "flannel", TONE.flannel + 0.07, stations.map((s) => [s, shirtFront(s) - 0.0005, 0, 0.0028, 0.016]), "shirt-torso", { texture: true, role: "detail", e: 4 });
  [0.4, 0.31, 0.22, 0.13, 0.04, -0.045].forEach((s, i) => k.ell(`shirt-button-${i + 1}`, "torso", "flannel", 0.2, [s, shirtFront(s) - 0.003, 0], [0.0055, 0.002, 0.0055], "shirt-torso", { texture: true, role: "detail" }));
  for (const side of [1, -1]) {
    const across = (s: number, half: number, lift: number): number[][] => [0.048, 0.07, 0.094, 0.118, 0.14].map((w) => [s, shirtFront(s, side * w) - lift, side * w, half, 0.0022]);
    k.bar(`shirt-pocket-${side > 0 ? "l" : "r"}`, "torso", "flannel", TONE.flannel + 0.05, across(0.262, 0.054, 0.0004), "shirt-torso", { texture: true, role: "detail", e: 6 });
    k.bar(`shirt-flap-${side > 0 ? "l" : "r"}`, "torso", "flannel", TONE.flannel - 0.1, across(0.306, 0.016, 0.0022), "shirt-torso", { texture: true, role: "detail", e: 4 });
  }
}

export function head(k: Kit): void {
  k.loft("neck", "head", "skin", TONE.skin - 0.04, NECK, "face");
  k.loft("head", "head", "skin", TONE.skin, HEAD, "face");
  k.loft("hair", "head", "hair", TONE.hair, HAIR, "hair");
  k.loft("beard", "head", "hair", TONE.hair + 0.06, [[0.016, -0.085, 0, 0.022, 0.019, 2.2], [0.034, -0.073, 0, 0.0435, 0.04, 2.3], [0.058, -0.052, 0, 0.0585, 0.061, 2.3], [0.08, -0.03, 0, 0.0665, 0.064, 2.2], [0.1, -0.012, 0, 0.0715, 0.05, 2.2]], "hair");
  k.loft("nose", "head", "skin", TONE.skin, [[0.086, -0.123, 0, 0.0165, 0.013, 2], [0.094, -0.128, 0, 0.0155, 0.017, 2], [0.11, -0.127, 0, 0.011, 0.013, 2], [0.134, -0.122, 0, 0.0085, 0.008, 2]], "face");
  k.ell("mouth", "head", "skin", TONE.skin - 0.3, [0.066, -0.1125, 0], [0.005, 0.006, 0.023], "face", { role: "detail" });
  for (const side of [1, -1]) {
    const s = side > 0 ? "l" : "r";
    k.ell(`ear-${s}`, "head", "skin", TONE.skin - 0.05, [0.118, -0.016, side * 0.0755], [0.03, 0.017, 0.009], "face", { rotation: quat(aboutU(side * -14), aboutW(-12)) });
    k.ell(`eye-${s}`, "head", "skin", TONE.dark + 0.08, [0.128, -0.112, side * 0.031], [0.0055, 0.006, 0.0125], "face", { role: "detail" });
    k.bar(`brow-${s}`, "head", "hair", TONE.hair, [[0.142, -0.117, side * 0.012, 0.0045], [0.146, -0.118, side * 0.032, 0.005], [0.141, -0.108, side * 0.052, 0.004]], "hair", { role: "detail" });
  }
}

/* ---------------------------------------------------------------- limbs */

const SIDES = [["l", 1], ["r", -1]] as const;

export const UPPER_ARM: Row[] = [...dome(-0.06, 0, 0, 0, 0.057, 0.062), [0, 0, 0, 0.057, 0.062], [0.05, 0.003, 0, 0.056, 0.059], [0.11, 0.006, 0, 0.051, 0.055], [0.165, 0.007, 0, 0.047, 0.052], [0.232, 0.004, 0, 0.0455, 0.049], [0.266, 0.001, 0, 0.0462, 0.0492], [0.283, 0.001, 0, 0.0432, 0.0458], [0.304, 0, 0, 0.0466, 0.0482], [0.327, 0, 0, 0.044, 0.045], ...dome(0.373, 0.327, 0, 0, 0.044, 0.045).reverse()];
export const FOREARM: Row[] = [...dome(-0.045, 0, 0, 0, 0.043, 0.044), [0, 0, 0, 0.043, 0.044], [0.017, 0.001, 0, 0.0452, 0.0465], [0.032, 0.002, 0, 0.0418, 0.0432], [0.05, 0.003, 0, 0.0445, 0.0472], [0.114, 0.002, 0, 0.04, 0.041], [0.181, 0, 0, 0.037, 0.033], [0.202, 0, 0, 0.036, 0.03]];
export const CUFF: Row[] = [[0.19, 0, 0, 0.036, 0.03, 2.3], [0.197, 0, 0, 0.0385, 0.0325, 2.3], [0.246, 0, 0, 0.0375, 0.0305, 2.3], [0.252, 0, 0, 0.034, 0.027, 2.3]];
export const WRIST: Row[] = [[0.233, 0, 0, 0.03, 0.0215], [0.255, 0, 0, 0.029, 0.02], [0.272, 0, 0, 0.0285, 0.0185]];
export const THIGH: Row[] = [...dome(-0.088, 0, 0, 0, 0.088, 0.093), [0, 0, 0, 0.088, 0.093], [0.03, 0.001, 0, 0.0858, 0.0905], [0.052, 0.002, 0, 0.0892, 0.0942], [0.074, 0.003, 0, 0.0855, 0.0905], [0.1, 0.004, 0, 0.087, 0.092], [0.22, 0.006, 0, 0.078, 0.084], [0.33, 0.005, 0, 0.068, 0.074], [0.4, 0.006, 0, 0.062, 0.069], [0.43, 0.008, 0, 0.06, 0.067], ...dome(0.489, 0.43, 0.008, 0, 0.06, 0.067).reverse()];
export const SHANK: Row[] = [...dome(-0.061, 0, 0.004, 0, 0.058, 0.064), [0, 0.004, 0, 0.058, 0.064], [0.022, 0.002, 0, 0.0602, 0.0662], [0.04, 0, 0, 0.0572, 0.0632], [0.07, -0.004, 0, 0.06, 0.066], [0.14, -0.008, 0, 0.06, 0.066], [0.22, -0.006, 0, 0.056, 0.06], [0.3, -0.003, 0, 0.054, 0.057], [0.355, 0, 0, 0.054, 0.056], [0.378, 0, 0, 0.058, 0.06], [0.394, 0.002, 0, 0.0535, 0.056], [0.41, 0.002, 0, 0.0575, 0.0595], [0.424, 0.002, 0, 0.052, 0.055]];
export const BOOT: Row[] = [[-0.076, -0.03, 0, 0.022, 0.032, 2.4], [-0.066, -0.026, 0, 0.034, 0.04, 2.4], [-0.04, -0.016, 0, 0.041, 0.052, 2.4], [-0.005, -0.006, 0, 0.042, 0.062, 2.4], [0.03, -0.018, 0, 0.044, 0.05, 2.4], [0.07, -0.031, 0, 0.048, 0.037, 2.4], [0.11, -0.039, 0, 0.051, 0.029, 2.4], [0.14, -0.042, 0, 0.052, 0.026, 2.4]];
export const TOE: Row[] = [[-0.006, 0.038, 0, 0.052, 0.026, 2.4], [0.026, 0.036, 0, 0.051, 0.024, 2.4], [0.053, 0.032, 0, 0.044, 0.02, 2.4], [0.07, 0.028, 0, 0.033, 0.016, 2.3], [0.08, 0.025, 0, 0.018, 0.011, 2.2]];

/** ✋️ One finger as a bent bar in the hand frame (u to the finger tips, v out of the palm, w across the hand): knuckle, two joints and the tip from the flexion of the three joints (degrees towards the palm). */
export function fingerPath(w: number, scale: number, r: number, flex: readonly [number, number, number]): number[][] {
  const lengths = [0.043, 0.026, 0.021].map((l) => l * scale);
  let [u, p, angle] = [0.1, -0.003, 0];
  const out: number[][] = [[u - 0.012, p, w, r * 1.05], [u, p, w, r * 1.08]];
  lengths.forEach((l, i) => {
    const half = (flex[i]! * Math.PI) / 360;
    const corner = Math.min(0.0085, l * 0.3);
    angle += half;
    u += corner * Math.cos(angle);
    p += corner * Math.sin(angle);
    out.push([u, p, w, r * (1 - 0.07 * i)]);
    angle += half;
    u += (l - corner) * Math.cos(angle);
    p += (l - corner) * Math.sin(angle);
    out.push([u, p, w, r * (1 - 0.07 * (i + 1)) * (i === 2 ? 0.86 : 1)]);
  });
  return out;
}
/** 🤏️ The two grips: the flexion of the four fingers and the thumb path `[u, out of the palm, towards the thumb side, radius]`. */
export const GRIPS = {
  fret: { flex: [[28, 82, 62], [38, 86, 62], [44, 86, 60], [40, 78, 54]], thumb: [[0.02, 0.004, 0.028, 0.0125], [0.046, 0.02, 0.047, 0.0118], [0.062, 0.046, 0.054, 0.0105], [0.07, 0.07, 0.052, 0.0092]] },
  strum: { flex: [[46, 74, 38], [40, 58, 32], [42, 58, 32], [46, 60, 34]], thumb: [[0.02, 0.004, 0.028, 0.0125], [0.056, 0.012, 0.05, 0.0118], [0.09, 0.03, 0.054, 0.0105], [0.116, 0.05, 0.047, 0.0092]] },
} as const;
/** 🖐️ A hand in the anatomical rest of its bone (palm ahead, thumb outwards: to the right on the right hand, to the left on the left). */
export function hand(k: Kit, s: "l" | "r", grip: keyof typeof GRIPS): void {
  const t = s === "r" ? -1 : 1;
  const bone = `hand-${s}`;
  k.loft(`palm-${s}`, bone, "skin", TONE.skin, [[-0.004, 0, 0, 0.028, 0.0185, 2.4], [0.03, 0, t * 0.003, 0.037, 0.0165, 2.6], [0.068, 0, t * 0.001, 0.042, 0.0145, 2.8], [0.097, 0, -t * 0.001, 0.041, 0.0125, 2.8], [0.108, 0, -t * 0.001, 0.039, 0.0085, 2.5]], "hands");
  k.ell(`thenar-${s}`, bone, "skin", TONE.skin, [0.038, 0.011, t * 0.028], [0.03, 0.014, 0.017], "hands");
  const g = GRIPS[grip];
  [0.03, 0.0095, -0.011, -0.03].forEach((w, i) => k.bar(`finger-${i + 1}-${s}`, bone, "skin", TONE.skin - 0.02 * (i % 2), fingerPath(t * w, [0.94, 1, 0.95, 0.78][i]!, [0.0088, 0.009, 0.0085, 0.0076][i]!, g.flex[i] as [number, number, number]), "hands"));
  k.bar(`thumb-${s}`, bone, "skin", TONE.skin, g.thumb.map(([u, p, w, r]) => [u, p, t * w, r]), "hands");
}

export function limbs(k: Kit): void {
  for (const [s, side] of SIDES) {
    k.loft(`sleeve-${s}`, `upper-arm-${s}`, "flannel", TONE.flannel - 0.03, UPPER_ARM, "shirt-sleeves");
    k.loft(`forearm-${s}`, `forearm-${s}`, "flannel", TONE.flannel - 0.03, FOREARM, "shirt-sleeves");
    k.loft(`cuff-${s}`, `forearm-${s}`, "flannel", TONE.flannel + 0.06, CUFF, "shirt-sleeves");
    k.loft(`wrist-${s}`, `forearm-${s}`, "skin", TONE.skin, WRIST, "hands");
    k.loft(`thigh-${s}`, `thigh-${s}`, "denim", TONE.denim, THIGH.map(([a, cv, cw, rx, ry]) => [a, cv, cw + side * 0.006, rx, ry] as Row), "jeans");
    k.loft(`shank-${s}`, `shank-${s}`, "denim", TONE.denim - 0.03, SHANK, "jeans");
    k.loft(`boot-shaft-${s}`, `shank-${s}`, "leather", TONE.leather, [[0.33, 0, 0, 0.045, 0.049], [0.45, -0.004, 0, 0.043, 0.05]], "boots");
    k.loft(`boot-${s}`, `foot-${s}`, "leather", TONE.leather, BOOT, "boots");
    k.loft(`boot-heel-${s}`, `foot-${s}`, "rubber", TONE.sole, [[-0.079, -0.071, 0, 0.022, 0.009, 4], [-0.066, -0.071, 0, 0.035, 0.009, 4], [-0.03, -0.071, 0, 0.042, 0.009, 4], [0, -0.071, 0, 0.0425, 0.009, 4]], "boots");
    k.loft(`boot-sole-${s}`, `foot-${s}`, "rubber", TONE.sole, [[-0.012, -0.068, 0, 0.042, 0.0045, 4], [0.03, -0.068, 0, 0.0445, 0.0045, 4], [0.075, -0.0725, 0, 0.0495, 0.0075, 4], [0.14, -0.0725, 0, 0.0535, 0.0075, 4]], "boots");
    k.loft(`boot-toe-${s}`, `toe-${s}`, "leather", TONE.leather + 0.04, TOE, "boots");
    k.loft(`boot-toe-sole-${s}`, `toe-${s}`, "rubber", TONE.sole, [[-0.006, 0.0074, 0, 0.0535, 0.0074, 4], [0.034, 0.0074, 0, 0.0525, 0.0074, 4], [0.064, 0.0074, 0, 0.042, 0.0074, 4], [0.083, 0.008, 0, 0.022, 0.0068, 3]], "boots");
  }
  hand(k, "l", "fret");
  hand(k, "r", "strum");
}

export function body50(k: Kit): void {
  stool(k);
  trunk(k);
  head(k);
  limbs(k);
  guitar(k);
}
export type { V3 };
