/** 🔭️ The 1:100, 1:200 and 1:500 bodies of the round 3 seated reader, each stated anew for its scale (smallest feature 15, 30 and 75 mm): fewer and longer sections, parts merged as the scale coarsens, the same woman on the same bench at first sight.
 * Rows are `[s, cv, cw, rx, ry, e?]` as in the 1:50 body.
 * @see ./r3-seated-reader-s50.ts */
import { Kit, aboutW, type P2, type Row } from "./r3-seated-reader-kit.ts";
import { BENCH, BENCH_LENGTH, BENCH_ZC, CUP, PAD, backFace, seatTop } from "./r3-seated-reader-ref.ts";
import { BACK_SLATS, FOOT_DROP, FRONT_LEG, POST, SEAT_RAIL, STRETCHER, TONE, inFrame } from "./r3-seated-reader-s50.ts";

const SIDES = [["l", 1], ["r", -1]] as const;
const FRAMES = [["l", "frame-l"], ["r", "frame-r"]] as const;
const sided = (rows: readonly Row[], side: number): Row[] => rows.map(([a, cv, cw, rx, ry, e]) => (e === undefined ? [a, cv, side * cw, rx, ry] : [a, cv, side * cw, rx, ry, e]) as Row);
const page = (k: Kit): void => { k.box("page-sheet", "page", "paper", TONE.paper + 0.04, [PAD.page / 2 + 0.004, PAD.thick / 2 + 0.0008, 0], [PAD.page - 0.008, 0.0008, PAD.width - 0.006], 0.0002, "notebook", { texture: true, role: "texture" }); };

/* ---------------------------------------------------------------- 1:100 */

export function body100(k: Kit): void {
  const t = BENCH.frameT / 2;
  for (const [s, bone] of FRAMES) {
    k.ext(`bench-post-${s}`, bone, "wood-bench", TONE.timber - 0.06, "xy", inFrame(bone, [POST[0]!, POST[1]!, [backFace(BENCH.top), BENCH.top], POST[4]!, POST[5]!, POST[6]!]), -t, t, "bench");
    k.ext(`bench-leg-${s}`, bone, "wood-bench", TONE.timber - 0.06, "xy", inFrame(bone, FRONT_LEG), -t, t, "bench");
    k.ext(`bench-rail-${s}`, bone, "wood-bench", TONE.timber - 0.1, "xy", inFrame(bone, SEAT_RAIL), -t + 0.0025, t - 0.0025, "bench");
    k.ext(`bench-stretcher-${s}`, bone, "wood-bench", TONE.timber - 0.1, "xy", inFrame(bone, STRETCHER), -0.0175, 0.0175, "bench");
  }
  k.ext("bench-tie", "frame-l", "wood-bench", TONE.timber - 0.12, "xy", inFrame("frame-l", [[-0.03, 0.14], [0.005, 0.14], [0.005, 0.2], [-0.03, 0.2]]), BENCH.frameZ[1] - BENCH.frameZ[0] + 0.0175, -0.0175, "bench");
  k.box("bench-seat", "seat", "wood-bench", TONE.timber, [BENCH.seatDepth / 2, -BENCH.slat[1] / 2, BENCH_ZC], [BENCH.seatDepth, BENCH.slat[1], BENCH_LENGTH], 0.004, "bench");
  BACK_SLATS.forEach((u, i) => k.box(`bench-back-slat-${i + 1}`, "backrest", "wood-bench", TONE.timber, [u, -BENCH.backSlat[1] / 2, BENCH_ZC], [BENCH.backSlat[0], BENCH.backSlat[1], BENCH_LENGTH], 0.003, "bench"));
  k.rev("cup-body", "cup", "paper-cup", TONE.cup, [[0, 0], [0, CUP.r0], [CUP.height - 0.003, CUP.r1], [CUP.height - 0.003, 0]], [1, 0, 0], [0, 0, 0], "cup");
  k.rev("cup-lid", "cup", "paper-cup", TONE.lid, [[CUP.height - 0.004, 0], [CUP.height - 0.004, CUP.r1 + 0.0025], [CUP.height + 0.005, CUP.r1 + 0.0025], [CUP.height + CUP.lid, CUP.r1 - 0.008], [CUP.height + CUP.lid, 0]], [1, 0, 0], [0, 0, 0], "cup");
  k.box("pad-board", "pad", "paper", TONE.paper - 0.1, [PAD.length / 2, 0, 0], [PAD.length, PAD.thick, PAD.width], 0.001, "notebook");
  page(k);
  k.loft("jeans-seat", "pelvis", "denim", TONE.denim, [[-0.087, 0.024, 0, 0.14, 0.07, 3], [-0.066, 0.027, 0, 0.19, 0.094, 2.7], [-0.02, 0.021, 0, 0.203, 0.108, 2.5], [0.04, 0.01, 0, 0.187, 0.113, 2.4], [0.1, -0.01, 0, 0.154, 0.102, 2.4], [0.126, -0.014, 0, 0.145, 0.096, 2.4]], "jeans");
  k.loft("sweater", "torso", "knit-wool", TONE.knit, [[-0.06, -0.02, 0, 0.19, 0.12, 2.5], [-0.04, -0.021, 0, 0.198, 0.127, 2.5], [0.03, -0.03, 0, 0.175, 0.118, 2.4], [0.12, -0.04, 0, 0.151, 0.11, 2.4], [0.21, -0.04, 0, 0.152, 0.12, 2.4], [0.29, -0.042, 0, 0.16, 0.134, 2.4], [0.37, -0.027, 0, 0.176, 0.117, 2.6], [0.425, -0.016, 0, 0.186, 0.097, 2.7], [0.455, -0.005, 0, 0.158, 0.076, 2.5], [0.477, 0.004, 0, 0.09, 0.06, 2.2]], "sweater");
  k.loft("cowl", "neck", "knit-wool", TONE.knit + 0.04, [[-0.048, 0.003, 0, 0.094, 0.08, 2.2], [-0.02, -0.004, 0, 0.1, 0.09, 2.2], [0.02, -0.009, 0, 0.089, 0.084, 2.1], [0.047, -0.011, 0, 0.06, 0.062, 2.1]], "sweater");
  k.loft("neck", "neck", "skin", TONE.skin - 0.04, [[-0.01, -0.007, 0, 0.053, 0.055], [0.06, -0.012, 0, 0.047, 0.051], [0.116, -0.008, 0, 0.048, 0.054]], "face-skin");
  k.loft("head", "head", "skin", TONE.skin, [[-0.093, -0.075, 0, 0.016, 0.012, 2], [-0.0837, -0.066, 0, 0.035, 0.028, 2.3], [-0.0558, -0.037, 0, 0.053, 0.06, 2.2], [-0.0186, -0.012, 0, 0.064, 0.084, 2.1], [0.03, -0.002, 0, 0.071, 0.0935, 2.1], [0.075, 0.002, 0, 0.068, 0.087, 2.1], [0.108, 0.006, 0, 0.047, 0.059, 2], [0.122, 0.0075, 0, 0.014, 0.018, 2]], "face-skin");
  k.loft("hair", "head", "hair", TONE.hair, [[-0.05, 0.032, 0, 0.034, 0.014, 2.2], [-0.005, 0.03, 0, 0.062, 0.057, 2.2], [0.03, 0.014, 0, 0.074, 0.085, 2.1], [0.06, 0.007, 0, 0.0765, 0.086, 2.1], [0.08, 0.0065, 0, 0.071, 0.092, 2.1], [0.095, 0.006, 0, 0.063, 0.08, 2.1], [0.118, 0.007, 0, 0.038, 0.047, 2], [0.1295, 0.0075, 0, 0.01, 0.012, 2]], "hair");
  k.ell("hair-bun", "head", "hair", TONE.hair - 0.04, [0.112, 0.068, 0], [0.038, 0.043, 0.045], "hair");
  k.loft("nose", "head", "skin", TONE.skin, [[0.018, -0.0935, 0, 0.0075, 0.0075, 2], [-0.02, -0.106, 0, 0.012, 0.011, 2], [-0.037, -0.096, 0, 0.014, 0.01, 2]], "face-skin");
  for (const [s, side] of SIDES) {
    k.ell(`ear-${s}`, "head", "skin", TONE.skin - 0.05, [0.002, 0.011, side * 0.068], [0.029, 0.017, 0.0085], "face-skin");
    k.loft(`sleeve-${s}`, `arm-${s}`, "knit-wool", TONE.knit - 0.03, sided([[-0.046, 0, -0.028, 0.012, 0.026], [-0.036, 0, -0.016, 0.032, 0.043], [-0.017, 0, -0.004, 0.046, 0.052], [0.012, 0, 0.002, 0.052, 0.055], [0.1, 0.004, 0.003, 0.047, 0.05], [0.22, 0.003, 0, 0.042, 0.045], [0.3115, 0, 0, 0.041, 0.041], [0.34, 0, 0, 0.032, 0.032], [0.351, 0, 0, 0.015, 0.015]], side), "sweater");
    k.loft(`forearm-${s}`, `forearm-${s}`, "knit-wool", TONE.knit - 0.03, [[-0.037, 0, 0, 0.013, 0.013], [-0.026, 0, 0, 0.029, 0.028], [-0.004, 0, 0, 0.034, 0.033], [0, 0, 0, 0.041, 0.04], [0.06, 0.002, 0, 0.043, 0.041], [0.15, 0, 0, 0.036, 0.033], [0.2, 0, 0, 0.035, 0.031], [0.241, 0, 0, 0.033, 0.029]], "sweater");
    const t = s === "l" ? -1 : 1;
    k.loft(`palm-${s}`, `hand-${s}`, "skin", TONE.skin, [[-0.012, 0, 0, 0.026, 0.018, 2.4], [0.03, -0.001, 0, 0.036, 0.015, 2.6], [0.085, -0.002, 0, 0.04, 0.012, 2.8], [0.13, -0.002, 0, 0.038, 0.0095, 2.8], [0.165, -0.003, 0, 0.032, 0.0085, 2.6], [0.176, -0.003, 0, 0.02, 0.0075, 2.4]], "hand-skin");
    k.bar(`thumb-${s}`, `hand-${s}`, "skin", TONE.skin, [[0.018, -0.004, t * 0.024, 0.0115], [0.045, -0.012, t * 0.047, 0.0105], [0.07, -0.032, t * 0.048, 0.009], [0.087, -0.038, t * 0.035, 0.008]], "hand-skin");
    k.loft(`thigh-${s}`, `thigh-${s}`, "denim", TONE.denim, sided([[-0.064, 0, 0.008, 0.03, 0.028], [-0.045, 0, 0.008, 0.072, 0.064], [0, -0.004, 0.008, 0.097, 0.086, 2.4], [0.12, -0.003, 0.005, 0.088, 0.08, 2.2], [0.26, 0.003, 0, 0.072, 0.068], [0.38, 0.008, 0, 0.057, 0.059], [0.43, 0.009, 0, 0.054, 0.057], [0.46, 0.009, 0, 0.042, 0.044], [0.471, 0.009, 0, 0.018, 0.02]], side), "jeans");
    k.loft(`shank-${s}`, `shank-${s}`, "denim", TONE.denim - 0.02, [[-0.051, 0.003, 0, 0.02, 0.022], [-0.036, 0.003, 0, 0.042, 0.046], [0, 0.003, 0, 0.055, 0.06], [0.1, -0.004, 0, 0.058, 0.064], [0.25, -0.003, 0, 0.054, 0.059], [0.385, 0.002, 0, 0.053, 0.057], [0.398, 0.002, 0, 0.05, 0.054]], "jeans");
    const level = { rotation: aboutW(FOOT_DROP) };
    k.loft(`shoe-${s}`, `foot-${s}`, "canvas-sneaker", TONE.canvas, [[-0.069, -0.046, 0, 0.024, 0.032, 3], [-0.058, -0.042, 0, 0.036, 0.036, 3], [-0.02, -0.036, 0, 0.041, 0.042, 3], [0.03, -0.045, 0, 0.043, 0.033, 3], [0.09, -0.054, 0, 0.047, 0.024, 3], [0.14, -0.057, 0, 0.048, 0.021, 3]], "shoes", level);
    k.loft(`shoe-toe-${s}`, `toe-${s}`, "canvas-sneaker", TONE.canvas, [[-0.012, 0.0215, 0, 0.048, 0.0215, 3], [0.03, 0.021, 0, 0.046, 0.021, 3], [0.056, 0.019, 0, 0.037, 0.019, 2.8], [0.069, 0.017, 0, 0.018, 0.012, 2.4]], "shoes");
  }
}

/* ---------------------------------------------------------------- 1:200 */

export function body200(k: Kit): void {
  const t = BENCH.frameT / 2;
  for (const [s, bone] of FRAMES) {
    k.ext(`bench-post-${s}`, bone, "wood-bench", TONE.timber - 0.06, "xy", inFrame(bone, [[-0.185, 0], [BENCH.backX, BENCH.seatRear[1]], [backFace(BENCH.top), BENCH.top], [backFace(BENCH.top) - 0.06, BENCH.top], [BENCH.backX - 0.06, BENCH.seatRear[1]], [-0.245, 0]]), -t, t, "bench");
    k.ext(`bench-leg-${s}`, bone, "wood-bench", TONE.timber - 0.06, "xy", inFrame(bone, [[0.18, 0], [0.25, 0], [0.25, seatTop(0.25) - 0.04], [-0.2, seatTop(-0.2) - 0.04], [-0.2, seatTop(-0.2) - 0.12], [0.18, seatTop(0.18) - 0.12]]), -t, t, "bench");
  }
  k.box("bench-seat", "seat", "wood-bench", TONE.timber, [BENCH.seatDepth / 2, -BENCH.slat[1] / 2, BENCH_ZC], [BENCH.seatDepth, BENCH.slat[1], BENCH_LENGTH], 0, "bench");
  k.box("bench-back", "backrest", "wood-bench", TONE.timber, [BENCH.backLength / 2, -BENCH.backSlat[1] / 2, BENCH_ZC], [BENCH.backLength, BENCH.backSlat[1], BENCH_LENGTH], 0, "bench");
  k.rev("cup-body", "cup", "paper-cup", TONE.cup, [[0, 0], [0, CUP.r0], [CUP.height, CUP.r1 + 0.001], [CUP.height + CUP.lid, CUP.r1 - 0.008], [CUP.height + CUP.lid, 0]], [1, 0, 0], [0, 0, 0], "cup");
  k.box("pad-board", "pad", "paper", TONE.paper - 0.1, [PAD.length / 2, 0, 0], [PAD.length, 0.03, PAD.width], 0, "notebook");
  page(k);
  k.loft("jeans-seat", "pelvis", "denim", TONE.denim, [[-0.086, 0.024, 0, 0.15, 0.075, 3], [-0.05, 0.026, 0, 0.2, 0.1, 2.6], [0.02, 0.016, 0, 0.196, 0.114, 2.4], [0.125, -0.014, 0, 0.146, 0.097, 2.4]], "jeans");
  k.loft("sweater", "torso", "knit-wool", TONE.knit, [[-0.06, -0.02, 0, 0.192, 0.122, 2.5], [-0.01, -0.024, 0, 0.192, 0.125, 2.5], [0.13, -0.04, 0, 0.151, 0.111, 2.4], [0.29, -0.042, 0, 0.16, 0.133, 2.4], [0.41, -0.02, 0, 0.186, 0.103, 2.7], [0.455, -0.005, 0, 0.158, 0.076, 2.5], [0.477, 0.004, 0, 0.09, 0.06, 2.2]], "sweater");
  k.loft("cowl", "neck", "knit-wool", TONE.knit + 0.04, [[-0.046, 0.002, 0, 0.095, 0.082, 2.2], [0.0, -0.007, 0, 0.097, 0.089, 2.1], [0.046, -0.011, 0, 0.062, 0.064, 2.1]], "sweater");
  k.loft("neck", "neck", "skin", TONE.skin - 0.04, [[0.0, -0.008, 0, 0.052, 0.054], [0.115, -0.008, 0, 0.048, 0.054]], "face-skin");
  k.loft("head", "head", "skin", TONE.skin, [[-0.093, -0.072, 0, 0.02, 0.016, 2], [-0.075, -0.056, 0, 0.044, 0.04, 2.3], [-0.028, -0.018, 0, 0.062, 0.08, 2.1], [0.035, -0.002, 0, 0.071, 0.093, 2.1], [0.09, 0.004, 0, 0.062, 0.078, 2.1], [0.121, 0.0075, 0, 0.018, 0.023, 2]], "face-skin");
  k.loft("hair", "head", "hair", TONE.hair, [[-0.045, 0.034, 0, 0.038, 0.018, 2.2], [0.01, 0.024, 0, 0.068, 0.072, 2.1], [0.06, 0.007, 0, 0.0765, 0.08, 2.1], [0.09, 0.0065, 0, 0.064, 0.084, 2.1], [0.105, 0.0065, 0, 0.056, 0.07, 2], [0.1295, 0.0075, 0, 0.016, 0.02, 2]], "hair");
  k.ell("hair-bun", "head", "hair", TONE.hair - 0.04, [0.11, 0.068, 0], [0.04, 0.044, 0.045], "hair");
  for (const [s, side] of SIDES) {
    k.loft(`sleeve-${s}`, `arm-${s}`, "knit-wool", TONE.knit - 0.03, sided([[-0.044, 0, -0.024, 0.016, 0.028], [-0.026, 0, -0.008, 0.042, 0.049], [-0.008, 0, 0.0, 0.049, 0.052], [0.03, 0.001, 0.003, 0.052, 0.054], [0.2, 0.003, 0, 0.043, 0.046], [0.32, 0, 0, 0.04, 0.04], [0.35, 0, 0, 0.02, 0.02]], side), "sweater");
    k.loft(`forearm-${s}`, `forearm-${s}`, "knit-wool", TONE.knit - 0.03, [[-0.036, 0, 0, 0.018, 0.018], [-0.01, 0, 0, 0.034, 0.033], [0.01, 0, 0, 0.041, 0.04], [0.06, 0.002, 0, 0.043, 0.041], [0.18, 0, 0, 0.035, 0.032], [0.24, 0, 0, 0.033, 0.029]], "sweater");
    k.loft(`palm-${s}`, `hand-${s}`, "skin", TONE.skin, [[-0.012, -0.001, 0, 0.027, 0.017, 2.6], [0.05, -0.002, 0, 0.04, 0.0155, 2.8], [0.13, -0.002, 0, 0.04, 0.015, 2.8], [0.176, -0.003, 0, 0.026, 0.015, 2.6]], "hand-skin");
    k.loft(`thigh-${s}`, `thigh-${s}`, "denim", TONE.denim, sided([[-0.062, 0, 0.01, 0.04, 0.036], [-0.03, -0.002, 0.01, 0.084, 0.077], [0.03, -0.004, 0.01, 0.093, 0.086, 2.3], [0.26, 0.003, 0, 0.072, 0.068], [0.42, 0.009, 0, 0.056, 0.058], [0.45, 0.009, 0, 0.052, 0.054], [0.47, 0.009, 0, 0.036, 0.038]], side), "jeans");
    k.loft(`shank-${s}`, `shank-${s}`, "denim", TONE.denim - 0.02, [[-0.04, 0.003, 0, 0.02, 0.02], [-0.012, 0.003, 0, 0.044, 0.048], [0.02, 0.003, 0, 0.052, 0.057], [0.1, -0.004, 0, 0.058, 0.064], [0.3, -0.001, 0, 0.053, 0.058], [0.398, 0.002, 0, 0.051, 0.055]], "jeans");
    k.loft(`shoe-${s}`, `foot-${s}`, "canvas-sneaker", TONE.canvas, [[-0.069, -0.046, 0, 0.026, 0.032, 3], [-0.05, -0.04, 0, 0.038, 0.038, 3], [0.0, -0.04, 0, 0.042, 0.038, 3], [0.08, -0.053, 0, 0.046, 0.025, 3], [0.15, -0.058, 0, 0.047, 0.02, 3], [0.185, -0.06, 0, 0.036, 0.018, 2.8], [0.198, -0.062, 0, 0.018, 0.015, 2.4]], "shoes", { rotation: aboutW(FOOT_DROP) });
  }
}

/* ---------------------------------------------------------------- 1:500 */

/** 🪵️ An end frame as one outline: rear post, seat rail and front leg in one board. */
const END: P2[] = [[-0.185, 0], [-0.168, 0.29], [0.175, 0.32], [0.175, 0], [0.25, 0], [0.25, seatTop(0.25) - 0.04], [BENCH.backX, BENCH.seatRear[1] - 0.04], [backFace(BENCH.top), BENCH.top], [backFace(BENCH.top) - 0.075, BENCH.top], [-0.24, 0.36], [-0.26, 0]];

export function body500(k: Kit): void {
  for (const [s, bone] of FRAMES) k.ext(`bench-post-${s}`, bone, "wood-bench", TONE.timber - 0.06, "xy", inFrame(bone, END), -0.0375, 0.0375, "bench");
  k.box("bench-seat", "seat", "wood-bench", TONE.timber, [BENCH.seatDepth / 2, -0.03, BENCH_ZC], [BENCH.seatDepth, 0.06, BENCH_LENGTH], 0, "bench");
  k.box("bench-back", "backrest", "wood-bench", TONE.timber, [BENCH.backLength / 2, -0.025, BENCH_ZC], [BENCH.backLength, 0.05, BENCH_LENGTH], 0, "bench");
  k.rev("cup-body", "cup", "paper-cup", TONE.cup, [[0, 0], [0, 0.038], [CUP.height + CUP.lid, 0.042], [CUP.height + CUP.lid, 0]], [1, 0, 0], [0, 0, 0], "cup");
  k.box("pad-board", "pad", "paper", TONE.paper - 0.1, [PAD.length / 2, 0, 0], [PAD.length, 0.078, PAD.width], 0, "notebook");
  page(k);
  k.loft("jeans-seat", "pelvis", "denim", TONE.denim, [[-0.085, 0.024, 0, 0.16, 0.082, 3], [-0.02, 0.021, 0, 0.2, 0.107, 2.5], [0.12, -0.012, 0, 0.15, 0.1, 2.4]], "jeans");
  k.loft("sweater", "torso", "knit-wool", TONE.knit, [[-0.055, -0.02, 0, 0.192, 0.123, 2.5], [0.13, -0.04, 0, 0.153, 0.112, 2.4], [0.3, -0.04, 0, 0.162, 0.132, 2.4], [0.42, -0.018, 0, 0.182, 0.1, 2.6], [0.456, -0.006, 0, 0.16, 0.086, 2.5], [0.49, 0.0, 0, 0.095, 0.085, 2.2]], "sweater");
  k.loft("cowl", "neck", "knit-wool", TONE.knit + 0.04, [[-0.04, 0.0, 0, 0.09, 0.08, 2.2], [0.05, -0.01, 0, 0.066, 0.068, 2.1], [0.112, -0.008, 0, 0.05, 0.055, 2]], "sweater");
  k.loft("head", "head", "skin", TONE.skin, [[-0.091, -0.066, 0, 0.038, 0.038, 2], [-0.037, -0.022, 0, 0.06, 0.076, 2.1], [0.04, -0.001, 0, 0.071, 0.092, 2.1], [0.118, 0.006, 0, 0.038, 0.045, 2]], "face-skin");
  k.loft("hair", "head", "hair", TONE.hair, [[-0.02, 0.048, 0, 0.058, 0.04, 2.2], [0.04, 0.02, 0, 0.0765, 0.082, 2.1], [0.09, 0.007, 0, 0.072, 0.09, 2.1], [0.1295, 0.008, 0, 0.0375, 0.0375, 2]], "hair");
  k.ell("hair-bun", "head", "hair", TONE.hair - 0.04, [0.11, 0.066, 0], [0.04, 0.045, 0.046], "hair");
  for (const [s, side] of SIDES) {
    k.loft(`sleeve-${s}`, `arm-${s}`, "knit-wool", TONE.knit - 0.03, sided([[-0.03, 0, -0.012, 0.0375, 0.0375], [0.03, 0.001, 0.004, 0.05, 0.052], [0.3, 0.001, 0, 0.041, 0.042], [0.345, 0, 0, 0.038, 0.038]], side), "sweater");
    k.loft(`forearm-${s}`, `forearm-${s}`, "knit-wool", TONE.knit - 0.03, [[-0.038, 0, 0, 0.038, 0.038], [0.06, 0.001, 0, 0.042, 0.04], [0.24, 0, 0, 0.038, 0.038]], "sweater");
    k.loft(`palm-${s}`, `hand-${s}`, "skin", TONE.skin, [[-0.01, -0.002, 0, 0.038, 0.038, 2.6], [0.09, -0.004, 0, 0.042, 0.038, 2.6], [0.17, -0.004, 0, 0.038, 0.038, 2.6]], "hand-skin");
    k.loft(`thigh-${s}`, `thigh-${s}`, "denim", TONE.denim, sided([[-0.06, -0.002, 0.01, 0.06, 0.055], [0.02, -0.004, 0.01, 0.092, 0.086, 2.3], [0.3, 0.004, 0.002, 0.07, 0.065], [0.43, 0.009, 0, 0.054, 0.055], [0.47, 0.009, 0, 0.0375, 0.0375]], side), "jeans");
    k.loft(`shank-${s}`, `shank-${s}`, "denim", TONE.denim - 0.02, [[-0.03, 0.003, 0, 0.0375, 0.0375], [0.03, 0.003, 0, 0.054, 0.058], [0.1, -0.003, 0, 0.058, 0.063], [0.398, 0.002, 0, 0.051, 0.055]], "jeans");
    k.loft(`shoe-${s}`, `foot-${s}`, "canvas-sneaker", TONE.canvas, [[-0.068, -0.0405, 0, 0.04, 0.0375, 2.4], [-0.02, -0.036, 0, 0.044, 0.042, 2.4], [0.06, -0.0405, 0, 0.046, 0.0375, 2.4], [0.196, -0.0405, 0, 0.04, 0.0375, 2.4]], "shoes", { rotation: aboutW(FOOT_DROP) });
  }
}
