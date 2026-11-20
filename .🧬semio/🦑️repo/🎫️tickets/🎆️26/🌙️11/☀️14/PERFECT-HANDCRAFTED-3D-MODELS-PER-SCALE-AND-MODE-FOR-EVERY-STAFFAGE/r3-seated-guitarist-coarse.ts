/** 🪆️ The coarser bodies of the round 3 seated guitarist, each stated anew for its scale: 1:100 (smallest feature 15 mm: mitten hands with thumbs, boots in one piece with a toe cap, the guitar with neck and fingerboard merged and machine heads as one bar per side), 1:200 (30 mm: one form per segment, fists, the head without features, the guitar as sound box, neck and head) and 1:500 (75 mm: trunk, limbs with the hands merged into the forearms, the guitar as sound box and one neck bar, the stool with its seat, four legs and the rung the foot stands on).
 * Rows are `[s, cv, cw, rx, ry, e?]` as in the 1:50 body.
 * @see ./r3-seated-guitarist-s50.ts */
import { ACROSS_V, Kit, aboutU, aboutW, dome, quat, type Row } from "./r3-seated-guitarist-kit.ts";
import { GUITAR, STOOL, depthAt, legAt, onGuitar } from "./r3-seated-guitarist-ref.ts";
import { TONE, fingerPath } from "./r3-seated-guitarist-s50.ts";

const SIDES = [["l", 1], ["r", -1]] as const;
const LEGS: readonly (readonly [string, number, number])[] = [["fl", 1, 1], ["fr", 1, -1], ["bl", -1, 1], ["br", -1, -1]];

function stool(k: Kit, seat: readonly (readonly [number, number])[], legs: readonly (readonly [y: number, r: number])[], rungs: readonly (readonly [id: string, y: number, r: number])[]): void {
  k.rev("stool-seat", "stool", "pine", TONE.pine, seat, [1, 0, 0], [0, 0, 0], "stool");
  for (const [id, sx, sz] of LEGS) k.loft(`stool-leg-${id}`, "stool", "pine", TONE.pine - 0.06, legs.map(([y, r]) => [y, -sx * legAt(y), sz * legAt(y), r, r] as Row), "stool");
  const ends: Record<string, [[number, number], [number, number]]> = { front: [[1, 1], [1, -1]], back: [[-1, 1], [-1, -1]], left: [[1, 1], [-1, 1]], right: [[1, -1], [-1, -1]] };
  for (const [id, y, r] of rungs) {
    const o = legAt(y);
    const [a, b] = ends[id]!;
    k.cap(`stool-rung-${id}`, "stool", "pine", TONE.pine - 0.1, [y, -a[0] * o, a[1] * o], [y, -b[0] * o, b[1] * o], r, r, "stool");
  }
}

function soundBox(k: Kit, stations: readonly (readonly [a: number, half: number])[], material: string, tone: number, e: number, depth: (a: number) => number = depthAt): void {
  k.loft("guitar-body", "guitar", material, tone, stations.map(([a, half]) => {
    const p = onGuitar(a, 0, -depth(a) / 2);
    return [p[0], p[1], p[2], half, depth(a) / 2, e] as Row;
  }), "guitar");
}
function neckBar(k: Kit, id: string, material: string, tone: number, stations: readonly (readonly [a: number, half: number, thick: number, c: number])[], e: number): void {
  k.loft(id, "guitar", material, tone, stations.map(([a, half, thick, c]) => {
    const p = onGuitar(a, 0, c);
    return [p[0], p[1], p[2], half, thick / 2, e] as Row;
  }), "guitar");
}

/* ---------------------------------------------------------------- 1:100 */

export function body100(k: Kit): void {
  stool(k, [[0.572, 0], [0.572, 0.15], [0.59, 0.165], [0.603, 0.165], [0.61, 0.156], [0.61, 0]], [[0, 0.015], [0.3, 0.0185], [0.585, 0.0178]], [["front", STOOL.rungFront, 0.0115], ["back", STOOL.rungFront, 0.0115], ["left", STOOL.rungSide, 0.0115], ["right", STOOL.rungSide, 0.0115]]);
  k.loft("jeans-seat", "pelvis", "denim", TONE.denim, [[-0.088, 0.018, 0, 0.13, 0.083, 2.6], [-0.045, 0.02, 0, 0.178, 0.115, 2.6], [0.03, 0.017, 0, 0.181, 0.115, 2.4], [0.118, 0.007, 0, 0.158, 0.105, 2.3]], "jeans");
  k.loft("shirt", "torso", "flannel", TONE.flannel, [[-0.07, 0.004, 0, 0.172, 0.115, 2.4], [0.05, 0, 0, 0.158, 0.113, 2.3], [0.16, -0.005, 0, 0.161, 0.118, 2.3], [0.28, -0.004, 0, 0.172, 0.127, 2.3], [0.36, 0.006, 0, 0.188, 0.114, 2.5], [0.43, 0.012, 0, 0.205, 0.083, 2.6], [0.462, 0.012, 0, 0.16, 0.069, 2.4], [0.485, 0.008, 0, 0.085, 0.062, 2.2]], "shirt-torso");
  k.loft("shirt-collar", "torso", "flannel", TONE.flannel + 0.08, [[0.466, 0.006, 0, 0.084, 0.071, 2.2], [0.486, 0.004, 0, 0.078, 0.07, 2.2], [0.502, 0.002, 0, 0.071, 0.068, 2.2]], "shirt-torso");
  k.loft("neck", "head", "skin", TONE.skin - 0.04, [[-0.05, -0.03, 0, 0.063, 0.061], [0.02, -0.032, 0, 0.057, 0.057], [0.09, -0.022, 0, 0.055, 0.06]], "face");
  k.loft("head", "head", "skin", TONE.skin, [[0.023, -0.084, 0, 0.024, 0.02, 2.2], [0.045, -0.066, 0, 0.047, 0.045, 2.3], [0.085, -0.04, 0, 0.065, 0.077, 2.2], [0.14, -0.029, 0, 0.0755, 0.095, 2.1], [0.2, -0.026, 0, 0.072, 0.092, 2.1], [0.235, -0.024, 0, 0.05, 0.065, 2], [0.249, -0.024, 0, 0.028, 0.036, 2]], "face");
  k.loft("hair", "head", "hair", TONE.hair, [[0.088, 0.03, 0, 0.068, 0.04, 2.2], [0.14, 0.008, 0, 0.0815, 0.067, 2.1], [0.19, -0.014, 0, 0.0815, 0.092, 2.1], [0.24, -0.023, 0, 0.058, 0.074, 2], [0.2595, -0.024, 0, 0.026, 0.034, 2]], "hair");
  k.loft("beard", "head", "hair", TONE.hair + 0.06, [[0.016, -0.084, 0, 0.025, 0.022, 2.2], [0.05, -0.058, 0, 0.055, 0.056, 2.3], [0.098, -0.014, 0, 0.0715, 0.05, 2.2]], "hair");
  k.loft("nose", "head", "skin", TONE.skin, [[0.087, -0.124, 0, 0.016, 0.014], [0.108, -0.128, 0, 0.012, 0.014], [0.132, -0.122, 0, 0.0085, 0.0085]], "face");
  for (const [s, side] of SIDES) {
    k.ell(`ear-${s}`, "head", "skin", TONE.skin - 0.05, [0.108, -0.012, side * 0.076], [0.029, 0.017, 0.0095], "face", { rotation: quat(aboutU(side * -14), aboutW(-12)) });
    k.loft(`sleeve-${s}`, `upper-arm-${s}`, "flannel", TONE.flannel - 0.03, [...dome(-0.058, 0, 0, 0, 0.054, 0.062, 3), [0, 0, 0, 0.054, 0.062], [0.14, 0.004, 0, 0.049, 0.055], [0.31, 0, 0, 0.044, 0.045], ...dome(0.356, 0.31, 0, 0, 0.044, 0.045, 3).reverse()], "shirt-sleeves");
    k.loft(`forearm-${s}`, `forearm-${s}`, "flannel", TONE.flannel - 0.03, [...dome(-0.045, 0, 0, 0, 0.043, 0.044, 3), [0, 0, 0, 0.043, 0.044], [0.055, 0.003, 0, 0.044, 0.047], [0.19, 0, 0, 0.037, 0.033], [0.264, 0, 0, 0.036, 0.029]], "shirt-sleeves");
    k.loft(`palm-${s}`, `hand-${s}`, "skin", TONE.skin, [[-0.022, 0, 0, 0.029, 0.02, 2.4], [0.03, 0.002, side * 0.008, 0.04, 0.019, 2.6], [0.075, 0, side * 0.002, 0.042, 0.015, 2.8], [0.1, 0, 0, 0.039, 0.0095, 2.5]], "hands");
    k.bar(`fingers-${s}`, `hand-${s}`, "skin", TONE.skin - 0.03, fingerPath(-side * 0.001, 0.96, 0.0095, s === "l" ? [36, 84, 60] : [42, 60, 34]).map((p) => [p[0]!, p[1]!, p[2]!, Math.max(p[3]!, 0.0078), 0.038]), "hands", { e: 3 });
    k.bar(`thumb-${s}`, `hand-${s}`, "skin", TONE.skin, (s === "l" ? [[0.02, 0.004, 0.028, 0.0125], [0.05, 0.026, 0.05, 0.011], [0.07, 0.068, 0.052, 0.0092]] : [[0.02, 0.004, 0.028, 0.0125], [0.07, 0.018, 0.053, 0.011], [0.114, 0.048, 0.047, 0.0092]]).map(([u, p, w, r]) => [u!, p!, side * w!, r!]), "hands");
    k.loft(`thigh-${s}`, `thigh-${s}`, "denim", TONE.denim, [...dome(-0.088, 0, 0, side * 0.006, 0.088, 0.093, 3), [0, 0, side * 0.006, 0.088, 0.093], [0.16, 0.005, side * 0.006, 0.083, 0.089], [0.33, 0.005, side * 0.006, 0.068, 0.074], [0.43, 0.008, side * 0.006, 0.06, 0.067], ...dome(0.489, 0.43, 0.008, side * 0.006, 0.06, 0.067, 3).reverse()], "jeans");
    k.loft(`shank-${s}`, `shank-${s}`, "denim", TONE.denim - 0.03, [...dome(-0.061, 0, 0.004, 0, 0.058, 0.064, 3), [0, 0.004, 0, 0.058, 0.064], [0.11, -0.007, 0, 0.06, 0.066], [0.3, -0.003, 0, 0.054, 0.057], [0.4, 0.002, 0, 0.057, 0.059], [0.424, 0.002, 0, 0.052, 0.055]], "jeans");
    k.loft(`boot-${s}`, `foot-${s}`, "leather", TONE.leather, [[-0.077, -0.044, 0, 0.024, 0.0359, 2.6], [-0.04, -0.03, 0, 0.041, 0.0499, 2.6], [-0.005, -0.012, 0, 0.042, 0.0679, 2.6], [0.04, -0.03, 0, 0.045, 0.0499, 2.6], [0.09, -0.045, 0, 0.05, 0.0349, 2.6], [0.14, -0.052, 0, 0.053, 0.0279, 2.6]], "boots");
    k.loft(`boot-toe-${s}`, `toe-${s}`, "leather", TONE.leather + 0.04, [[-0.006, 0.027, 0, 0.053, 0.027, 2.6], [0.04, 0.025, 0, 0.051, 0.025, 2.6], [0.08, 0.02, 0, 0.036, 0.02, 2.5], [0.096, 0.016, 0, 0.02, 0.014, 2.3]], "boots");
  }
  soundBox(k, [[0, 0.085], [0.04, 0.17], [0.14, 0.1985], [0.25, 0.156], [0.29, 0.1365], [0.38, 0.14], [0.468, 0.142], [0.508, 0.075]], "mahogany", TONE.mahogany, 8);
  k.loft("guitar-top", "guitar", "spruce", TONE.spruce, ([[0, 0.081], [0.04, 0.166], [0.14, 0.1945], [0.25, 0.152], [0.29, 0.1325], [0.38, 0.136], [0.468, 0.138], [0.508, 0.071]] as const).map(([a, half]) => {
    const p = onGuitar(a, 0, 0.0005);
    return [p[0], p[1], p[2], half, 0.0025, 8] as Row;
  }), "guitar", { texture: true });
  k.ell("guitar-soundhole", "guitar", "rosewood", TONE.dark, onGuitar(GUITAR.soundhole, 0, 0.003), [GUITAR.soundholeR, 0.0014, GUITAR.soundholeR], "guitar", { texture: true, role: "detail" });
  k.box("guitar-bridge", "guitar", "rosewood", TONE.rosewood, onGuitar(GUITAR.saddle, 0, 0.0065), [0.036, 0.009, 0.15], 0.002, "guitar", { texture: true, role: "detail" });
  neckBar(k, "guitar-neck", "rosewood", TONE.rosewood + 0.06, [[GUITAR.fretboardEnd, 0.0285, 0.0155, -0.0012], [0.5, 0.028, 0.024, -0.0055], [0.68, 0.025, 0.0225, -0.0048], [GUITAR.nut, 0.0215, 0.021, -0.004]], 3);
  const heel = onGuitar(0.522, 0, 0);
  k.loft("guitar-heel", "guitar", "mahogany", TONE.mahogany + 0.06, [[heel[1] + 0.012, heel[0], heel[2], 0.019, 0.027, 3], [heel[1] + 0.088, heel[0] - 0.008, heel[2], 0.0085, 0.013, 2.4]], "guitar", { across: ACROSS_V });
  neckBar(k, "guitar-head", "mahogany", TONE.mahogany - 0.08, [[GUITAR.nut - 0.002, 0.0225, 0.016, -0.0085], [GUITAR.nut + 0.032, 0.035, 0.016, -0.016], [GUITAR.length, 0.036, 0.016, -0.049]], 6);
  for (const side of [1, -1]) k.cap(`guitar-tuners-${side > 0 ? "b" : "t"}`, "guitar", "steel", TONE.steel, onGuitar(0.9, side * 0.0425, -0.021), onGuitar(0.98, side * 0.0445, -0.041), 0.0078, 0.0078, "guitar", { role: "detail" });
}

/* ---------------------------------------------------------------- 1:200 */

export function body200(k: Kit): void {
  stool(k, [[0.572, 0], [0.572, 0.16], [0.61, 0.16], [0.61, 0]], [[0, 0.016], [0.585, 0.0185]], [["front", STOOL.rungFront - 0.0035, 0.0152], ["back", STOOL.rungFront - 0.0035, 0.0152], ["left", STOOL.rungSide, 0.0152], ["right", STOOL.rungSide, 0.0152]]);
  k.loft("jeans-seat", "pelvis", "denim", TONE.denim, [[-0.088, 0.018, 0, 0.132, 0.085, 2.6], [-0.01, 0.02, 0, 0.181, 0.117, 2.5], [0.118, 0.007, 0, 0.158, 0.105, 2.3]], "jeans");
  k.loft("shirt", "torso", "flannel", TONE.flannel, [[-0.07, 0.004, 0, 0.172, 0.115, 2.4], [0.08, -0.002, 0, 0.158, 0.113, 2.3], [0.27, -0.004, 0, 0.172, 0.126, 2.3], [0.39, 0.009, 0, 0.2, 0.102, 2.6], [0.44, 0.012, 0, 0.2, 0.078, 2.6], [0.487, 0.008, 0, 0.088, 0.064, 2.2]], "shirt-torso");
  k.loft("neck", "head", "skin", TONE.skin - 0.04, [[-0.04, -0.03, 0, 0.062, 0.06], [0.09, -0.024, 0, 0.055, 0.06]], "face");
  k.loft("head", "head", "skin", TONE.skin, [[0.022, -0.082, 0, 0.028, 0.024, 2.2], [0.07, -0.05, 0, 0.06, 0.068, 2.3], [0.14, -0.03, 0, 0.0755, 0.096, 2.1], [0.21, -0.026, 0, 0.068, 0.088, 2.1], [0.249, -0.024, 0, 0.03, 0.038, 2]], "face");
  k.loft("hair", "head", "hair", TONE.hair, [[0.09, 0.03, 0, 0.069, 0.04, 2.2], [0.16, -0.002, 0, 0.0825, 0.078, 2.1], [0.225, -0.021, 0, 0.068, 0.085, 2], [0.26, -0.024, 0, 0.028, 0.036, 2]], "hair");
  for (const [s, side] of SIDES) {
    k.loft(`sleeve-${s}`, `upper-arm-${s}`, "flannel", TONE.flannel - 0.03, [...dome(-0.056, 0, 0, 0, 0.054, 0.061, 2), [0, 0, 0, 0.054, 0.061], [0.16, 0.003, 0, 0.048, 0.054], [0.31, 0, 0, 0.044, 0.045], ...dome(0.354, 0.31, 0, 0, 0.044, 0.045, 2).reverse()], "shirt-sleeves");
    k.loft(`forearm-${s}`, `forearm-${s}`, "flannel", TONE.flannel - 0.03, [...dome(-0.044, 0, 0, 0, 0.043, 0.044, 2), [0, 0, 0, 0.043, 0.044], [0.08, 0.002, 0, 0.043, 0.045], [0.268, 0, 0, 0.035, 0.029]], "shirt-sleeves");
    k.loft(`hand-${s}`, `hand-${s}`, "skin", TONE.skin, s === "l" ? [[-0.012, 0.002, 0, 0.03, 0.019, 2.6], [0.05, 0.008, side * 0.006, 0.042, 0.022, 2.6], [0.1, 0.022, side * 0.004, 0.042, 0.03, 2.6], [0.128, 0.03, 0, 0.034, 0.024, 2.4]] : [[-0.012, 0.002, 0, 0.03, 0.019, 2.6], [0.05, 0.006, side * 0.006, 0.042, 0.021, 2.6], [0.105, 0.018, side * 0.004, 0.042, 0.027, 2.6], [0.142, 0.03, 0, 0.034, 0.022, 2.4]], "hands");
    k.loft(`thigh-${s}`, `thigh-${s}`, "denim", TONE.denim, [...dome(-0.086, 0, 0, side * 0.006, 0.088, 0.093, 2), [0, 0, side * 0.006, 0.088, 0.093], [0.2, 0.006, side * 0.006, 0.08, 0.086], [0.43, 0.008, side * 0.006, 0.06, 0.067], ...dome(0.488, 0.43, 0.008, side * 0.006, 0.06, 0.067, 2).reverse()], "jeans");
    k.loft(`shank-${s}`, `shank-${s}`, "denim", TONE.denim - 0.03, [...dome(-0.06, 0, 0.004, 0, 0.058, 0.064, 2), [0, 0.004, 0, 0.058, 0.064], [0.12, -0.007, 0, 0.06, 0.066], [0.32, -0.002, 0, 0.054, 0.057], [0.424, 0.002, 0, 0.054, 0.057]], "jeans");
    k.loft(`boot-${s}`, `foot-${s}`, "leather", TONE.leather, [[-0.077, -0.044, 0, 0.026, 0.0359, 2.6], [-0.01, -0.014, 0, 0.043, 0.0659, 2.6], [0.06, -0.036, 0, 0.047, 0.0439, 2.6], [0.15, -0.052, 0, 0.053, 0.0279, 2.6], [0.21, -0.058, 0, 0.04, 0.0219, 2.5], [0.226, -0.062, 0, 0.022, 0.017, 2.3]], "boots");
  }
  soundBox(k, [[0, 0.09], [0.02, 0.145], [0.06, 0.182], [0.14, 0.198], [0.21, 0.18], [0.29, 0.137], [0.35, 0.134], [0.42, 0.145], [0.475, 0.136], [0.508, 0.08]], "spruce", TONE.spruce - 0.1, 6);
  neckBar(k, "guitar-neck", "rosewood", TONE.rosewood + 0.06, [[0.46, 0.0285, 0.0304, -0.0087], [GUITAR.nut, 0.0225, 0.0304, -0.0087]], 3);
  neckBar(k, "guitar-head", "mahogany", TONE.mahogany - 0.08, [[GUITAR.nut - 0.004, 0.024, 0.0304, -0.01], [GUITAR.nut + 0.035, 0.037, 0.0304, -0.018], [GUITAR.length, 0.037, 0.0304, -0.045]], 5);
}

/* ---------------------------------------------------------------- 1:500 */

export function body500(k: Kit): void {
  k.rev("stool-seat", "stool", "pine", TONE.pine, [[0.534, 0], [0.534, 0.165], [0.61, 0.165], [0.61, 0]], [1, 0, 0], [0, 0, 0], "stool");
  for (const [id, sx, sz] of LEGS) k.loft(`stool-leg-${id}`, "stool", "pine", TONE.pine - 0.06, [0, 0.55].map((y) => [y, -sx * (legAt(y) - 0.022), sz * (legAt(y) - 0.022), 0.038, 0.038] as Row), "stool");
  k.cap("stool-rung-front", "stool", "pine", TONE.pine - 0.1, [STOOL.rungFront - 0.0265, -legAt(0.22), 0.12], [STOOL.rungFront - 0.0265, -legAt(0.22), -0.12], 0.038, 0.038, "stool");
  k.loft("jeans-seat", "pelvis", "denim", TONE.denim, [[-0.088, 0.018, 0, 0.135, 0.088, 2.6], [-0.01, 0.02, 0, 0.18, 0.116, 2.5], [0.118, 0.007, 0, 0.16, 0.106, 2.3]], "jeans");
  k.loft("shirt", "torso", "flannel", TONE.flannel, [[-0.07, 0.004, 0, 0.17, 0.115, 2.4], [0.1, -0.003, 0, 0.16, 0.115, 2.3], [0.3, -0.002, 0, 0.176, 0.125, 2.3], [0.43, 0.012, 0, 0.202, 0.082, 2.6], [0.487, 0.008, 0, 0.09, 0.066, 2.2]], "shirt-torso");
  k.loft("head", "head", "skin", TONE.skin, [[-0.03, -0.03, 0, 0.06, 0.06, 2], [0.05, -0.04, 0, 0.058, 0.066, 2.2], [0.14, -0.03, 0, 0.076, 0.096, 2.1], [0.249, -0.024, 0, 0.038, 0.048, 2]], "face");
  k.loft("hair", "head", "hair", TONE.hair, [[0.095, 0.03, 0, 0.072, 0.046, 2.2], [0.17, -0.004, 0, 0.086, 0.088, 2.1], [0.272, -0.024, 0, 0.044, 0.054, 2]], "hair");
  for (const [s, side] of SIDES) {
    k.loft(`sleeve-${s}`, `upper-arm-${s}`, "flannel", TONE.flannel - 0.03, [[-0.05, 0, 0, 0.04, 0.045], [0, 0, 0, 0.054, 0.061], [0.31, 0, 0, 0.045, 0.047], [0.35, 0, 0, 0.038, 0.038]], "shirt-sleeves");
    k.loft(`forearm-${s}`, `forearm-${s}`, "flannel", TONE.flannel - 0.03, [[-0.04, 0, 0, 0.038, 0.038], [0, 0, 0, 0.044, 0.045], [0.26, 0, 0, 0.0385, 0.0385]], "shirt-sleeves");
    k.loft(`hand-${s}`, `hand-${s}`, "skin", TONE.skin, [[-0.02, 0.004, 0, 0.0385, 0.0385, 2.4], [0.06, 0.01, 0, 0.042, 0.0385, 2.6], [0.13, 0.02, 0, 0.0385, 0.0385, 2.4]], "hands");
    k.loft(`thigh-${s}`, `thigh-${s}`, "denim", TONE.denim, [[-0.08, 0, side * 0.006, 0.06, 0.065], [0, 0, side * 0.006, 0.088, 0.093], [0.43, 0.008, side * 0.006, 0.061, 0.068], [0.485, 0.008, side * 0.006, 0.042, 0.046]], "jeans");
    k.loft(`shank-${s}`, `shank-${s}`, "denim", TONE.denim - 0.03, [[-0.055, 0.004, 0, 0.042, 0.046], [0, 0.004, 0, 0.058, 0.064], [0.15, -0.006, 0, 0.059, 0.064], [0.424, 0.002, 0, 0.054, 0.057]], "jeans");
    k.loft(`boot-${s}`, `foot-${s}`, "leather", TONE.leather, [[-0.077, -0.04, 0, 0.038, 0.0399, 2.6], [0, -0.016, 0, 0.044, 0.0639, 2.6], [0.14, -0.042, 0, 0.052, 0.0379, 2.6], [0.226, -0.042, 0, 0.038, 0.0379, 2.4]], "boots");
  }
  soundBox(k, [[0.004, 0.1], [0.035, 0.155], [0.08, 0.188], [0.15, 0.197], [0.22, 0.176], [0.29, 0.14], [0.36, 0.137], [0.43, 0.145], [0.48, 0.13], [0.506, 0.09]], "spruce", TONE.spruce - 0.1, 5, () => 0.112);
  neckBar(k, "guitar-neck", "rosewood", TONE.rosewood + 0.06, [[0.46, 0.038, 0.076, -0.03], [0.9, 0.038, 0.076, -0.034], [GUITAR.length - 0.02, 0.04, 0.076, -0.046]], 4);
}
