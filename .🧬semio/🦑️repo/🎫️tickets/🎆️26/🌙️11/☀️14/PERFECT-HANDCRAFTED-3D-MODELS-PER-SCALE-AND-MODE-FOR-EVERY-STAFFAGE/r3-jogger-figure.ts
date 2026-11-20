/** 🏃️ The round 3 jogger, stated by hand: every section of the body, the thermal model and the skeleton at 1:50, 1:100, 1:200 and 1:500 for a lean 1.82 m (shod) male runner of 72 kg. Trunk bones: u up, v to the back, w to the left. Limb bones: u distal, v to the front, w to the left. Foot and toe: u ahead, v up.
 * Usage: `bun r3-jogger-figure.ts` writes `volumes/humans/jogger.json`.
 * @see ./r3-jogger-kit.ts
 * @see ./r3-humans-jogger.md */
import { writeFileSync } from "node:fs";
import { join } from "node:path";

import { Kit, parts, thermalOf, type Json, type Row, type V3 } from "./r3-jogger-kit.ts";

import { axial, face, rod, type Fit } from "./r4-g06-skeleton.ts";
import { structureText } from "./r4-g06-texts.ts";
const SKIN = "skin";
const SHIRT = "nylon-clothing";
const SOCK = "cotton-clothing";
const SHOE = "leather-shoe";
const HAIR = "hair";
const T = { skin: 0.72, shirt: 0.88, shorts: 0.22, hair: 0.15, band: 0.9, shoe: 0.92, sole: 0.55, sock: 0.95, arm: 0.25, eye: 0.1, bone: 0.85 };
const SIDES = [["l", 1], ["r", -1]] as const;
const m = (rows: readonly Row[], g: number): Row[] => rows.map(([s, cv, cw, rx, ry, e]) => [s, cv, cw * g, rx, ry, e] as Row);

/* ------------------------------------------------------------------ body 1:50 */

function body50(k: Kit): void {
  k.loft("shirt", "torso", SHIRT, T.shirt, [[-0.085, 0.004, 0, 0.184, 0.142, 2.4], [-0.03, 0.002, 0, 0.172, 0.126, 2.4], [0.05, -0.006, 0, 0.152, 0.104, 2.3], [0.14, -0.006, 0, 0.155, 0.11, 2.3], [0.24, -0.004, 0, 0.165, 0.122, 2.3], [0.33, 0.002, 0, 0.166, 0.118, 2.3], [0.4, 0.006, 0, 0.168, 0.1, 2.2], [0.44, 0.008, 0, 0.178, 0.084, 2.2], [0.46, 0.008, 0, 0.168, 0.075, 2.1], [0.477, 0.008, 0, 0.128, 0.069, 2], [0.491, 0.007, 0, 0.094, 0.066, 2], [0.503, 0.005, 0, 0.076, 0.064, 2]], "torso-clothing");
  k.loft("shirt-collar", "torso", SHIRT, 0.6, [[0.496, 0.005, 0, 0.08, 0.068], [0.511, 0.004, 0, 0.071, 0.066]], "torso-clothing", "detail");
  k.loft("neck", "head", SKIN, T.skin, [[-0.03, 0.012, 0, 0.066, 0.064], [0.0, 0.008, 0, 0.061, 0.062], [0.04, 0.002, 0, 0.057, 0.06], [0.085, -0.006, 0, 0.056, 0.062]], "head-skin");
  k.loft("head", "head", SKIN, T.skin, [[0.008, -0.076, 0, 0.02, 0.016, 2.2], [0.028, -0.06, 0, 0.044, 0.04, 2.2], [0.055, -0.04, 0, 0.058, 0.062, 2.2], [0.085, -0.018, 0, 0.068, 0.085], [0.125, -0.008, 0, 0.074, 0.095], [0.165, -0.006, 0, 0.0775, 0.098], [0.2, -0.002, 0, 0.072, 0.09], [0.228, 0, 0, 0.052, 0.066], [0.2449, 0, 0, 0.018, 0.024]], "head-skin");
  k.cap("nose", "head", SKIN, T.skin, [0.13, -0.1, 0], [0.093, -0.118, 0], 0.007, 0.0115, "head-skin");
  for (const [sfx, g] of SIDES) {
    k.ell(`ear-${sfx}`, "head", SKIN, T.skin, [0.11, 0.004, 0.076 * g], [0.028, 0.016, 0.0075], "head-skin", "detail", [0.1305 * g, 0, 0, 0.9914]);
    k.ell(`eye-${sfx}`, "head", SKIN, T.eye, [0.128, -0.0925, 0.031 * g], [0.005, 0.004, 0.011], "head-skin", "detail");
  }
  k.loft("hair", "head", HAIR, T.hair, [[0.088, 0.044, 0, 0.058, 0.034], [0.125, 0.032, 0, 0.079, 0.062], [0.165, 0.012, 0, 0.082, 0.088], [0.195, 0.001, 0, 0.08, 0.097], [0.225, 0, 0, 0.063, 0.078], [0.243, 0, 0, 0.042, 0.052], [0.2529, 0, 0, 0.02, 0.026]], "hair");
  k.loft("headband", "head", SHIRT, T.band, [[0.172, -0.001, 0, 0.0855, 0.1035], [0.198, 0.001, 0, 0.0835, 0.1005]], "hair", "detail");

  k.loft("shorts", "pelvis", SHIRT, T.shorts, [[-0.092, 0.01, 0, 0.12, 0.08, 2.4], [-0.06, 0.012, 0, 0.165, 0.108, 2.4], [-0.02, 0.013, 0, 0.174, 0.118, 2.4], [0.03, 0.008, 0, 0.17, 0.114, 2.4], [0.08, 0.002, 0, 0.158, 0.106, 2.3], [0.125, -0.002, 0, 0.148, 0.098, 2.3]], "leg-clothing");
  for (const [sfx, g] of SIDES) {
    k.ell(`seat-${sfx}`, "pelvis", SHIRT, T.shorts, [-0.02, 0.072, 0.075 * g], [0.085, 0.06, 0.078], "leg-clothing");
    k.loft(`shorts-leg-${sfx}`, `thigh-${sfx}`, SHIRT, T.shorts, m([[-0.03, 0.004, -0.004, 0.08, 0.095], [0.03, 0.006, -0.003, 0.09, 0.103], [0.1, 0.008, -0.001, 0.089, 0.1], [0.16, 0.01, 0.001, 0.089, 0.099]], g), "leg-clothing", "body", 2.3);
    k.loft(`thigh-${sfx}`, `thigh-${sfx}`, SKIN, T.skin, m([[0.135, 0.009, -0.002, 0.08, 0.089], [0.18, 0.01, -0.001, 0.078, 0.086], [0.26, 0.01, 0, 0.071, 0.078], [0.34, 0.008, 0, 0.062, 0.068], [0.4, 0.006, 0, 0.0555, 0.061], [0.4447, 0.006, 0, 0.053, 0.059], [0.468, 0.005, 0, 0.047, 0.051], [0.487, 0.004, 0, 0.03, 0.033]], g), "leg-skin");
    k.ell(`patella-${sfx}`, `shank-${sfx}`, SKIN, T.skin, [-0.006, 0.043, 0], [0.03, 0.022, 0.027], "leg-skin");
    k.loft(`shank-${sfx}`, `shank-${sfx}`, SKIN, T.skin, [[-0.045, 0.004, 0, 0.03, 0.032], [-0.025, 0.004, 0, 0.047, 0.052], [0, 0.004, 0, 0.053, 0.058], [0.05, -0.002, 0, 0.05, 0.054], [0.12, -0.01, 0, 0.054, 0.062], [0.2, -0.008, 0, 0.049, 0.056], [0.28, -0.004, 0, 0.04, 0.045], [0.35, -0.002, 0, 0.032, 0.038], [0.4, -0.002, 0, 0.031, 0.039], [0.4343, -0.001, 0, 0.034, 0.04], [0.458, -0.001, 0, 0.03, 0.036]], "leg-skin");
    k.loft(`sock-${sfx}`, `shank-${sfx}`, SOCK, T.sock, [[0.384, -0.002, 0, 0.0345, 0.042], [0.41, -0.002, 0, 0.0345, 0.0425], [0.4343, -0.001, 0, 0.0375, 0.0435], [0.452, -0.001, 0, 0.036, 0.042]], "leg-skin");
    k.loft(`sole-${sfx}`, `foot-${sfx}`, SHOE, T.sole, m([[-0.074, -0.0685, 0, 0.02, 0.01], [-0.06, -0.0672, 0, 0.034, 0.0155], [-0.03, -0.0667, 0, 0.04, 0.016], [0.03, -0.0682, 0.002, 0.038, 0.0145], [0.09, -0.0702, 0.002, 0.046, 0.0125], [0.1344, -0.0712, 0.001, 0.051, 0.0115], [0.146, -0.0712, 0.001, 0.05, 0.0115]], g), "shoes", "body", 4);
    k.loft(`shoe-${sfx}`, `foot-${sfx}`, SHOE, T.shoe, m([[-0.069, -0.03, 0, 0.022, 0.024], [-0.045, -0.026, 0, 0.033, 0.029], [-0.01, -0.028, 0, 0.037, 0.027], [0.03, -0.026, 0.001, 0.037, 0.03], [0.07, -0.03, 0.002, 0.041, 0.028], [0.11, -0.037, 0.002, 0.046, 0.023], [0.1344, -0.041, 0.001, 0.048, 0.02], [0.146, -0.042, 0.001, 0.047, 0.0195]], g), "shoes", "body", 2.4);
    k.loft(`sole-toe-${sfx}`, `toe-${sfx}`, SHOE, T.sole, m([[-0.004, 0.0115, 0.001, 0.051, 0.0115], [0.04, 0.0125, -0.002, 0.047, 0.0105], [0.075, 0.016, -0.006, 0.036, 0.009], [0.0952, 0.02, -0.008, 0.016, 0.006]], g), "shoes", "body", 4);
    k.loft(`shoe-toe-${sfx}`, `toe-${sfx}`, SHOE, T.shoe, m([[-0.004, 0.0407, 0.001, 0.048, 0.021], [0.035, 0.039, -0.002, 0.045, 0.0195], [0.07, 0.034, -0.006, 0.034, 0.016], [0.092, 0.027, -0.008, 0.015, 0.009]], g), "shoes", "body", 2.4);

    k.loft(`shoulder-${sfx}`, `upper-arm-${sfx}`, SHIRT, T.shirt, m([[-0.03, 0, -0.012, 0.02, 0.028], [-0.022, 0, -0.008, 0.038, 0.044], [-0.004, 0, -0.002, 0.052, 0.055], [0.025, 0.001, 0.002, 0.057, 0.059], [0.05, 0.002, 0.004, 0.0575, 0.0595], [0.075, 0.002, 0.005, 0.057, 0.059]], g), "torso-clothing");
    k.loft(`sleeve-${sfx}`, `upper-arm-${sfx}`, SHIRT, T.shirt, m([[0.065, 0.002, 0.005, 0.0575, 0.0595], [0.11, 0.003, 0.004, 0.0555, 0.058], [0.15, 0.003, 0.003, 0.057, 0.0595], [0.158, 0.003, 0.003, 0.0575, 0.06]], g), "arm-clothing");
    k.loft(`upper-arm-${sfx}`, `upper-arm-${sfx}`, SKIN, T.skin, m([[0.135, 0.004, 0.003, 0.048, 0.053], [0.17, 0.004, 0.003, 0.047, 0.053], [0.22, 0.003, 0.002, 0.044, 0.049], [0.28, 0, 0.001, 0.041, 0.044], [0.3206, -0.002, 0, 0.042, 0.041], [0.342, -0.002, 0, 0.037, 0.036], [0.356, -0.002, 0, 0.024, 0.024]], g), "arm-skin");
    k.loft(`forearm-${sfx}`, `forearm-${sfx}`, SKIN, T.skin, [[-0.034, 0, 0, 0.024, 0.024], [-0.02, 0, 0, 0.037, 0.036], [0, 0, 0, 0.042, 0.039], [0.05, 0.002, 0, 0.044, 0.043], [0.12, 0.002, 0, 0.038, 0.039], [0.2, 0.001, 0, 0.028, 0.033], [0.26, 0, 0, 0.022, 0.0295], [0.2792, 0, 0, 0.0205, 0.029], [0.295, 0, 0, 0.019, 0.027]], "arm-skin");
    k.loft(`hand-${sfx}`, `hand-${sfx}`, SKIN, T.skin, m([[-0.012, 0.002, 0, 0.019, 0.026], [0, 0.003, 0, 0.0205, 0.029], [0.035, 0.006, -0.002, 0.016, 0.038], [0.075, 0.004, -0.004, 0.014, 0.042], [0.1, 0.002, -0.006, 0.012, 0.04], [0.108, 0.002, -0.007, 0.008, 0.036]], g), "hand-skin", "body", 2.6);
    for (const [i, kv, ku, len] of [[1, 0.03, 0.098, 1], [2, 0.01, 0.1, 1.08], [3, -0.01, 0.097, 1], [4, -0.028, 0.09, 0.8]] as const) {
      const p0: V3 = [ku, kv, -0.006 * g];
      const p1: V3 = [ku + 0.04 * len, kv, -0.024 * g];
      const p2: V3 = [ku + 0.05 * len, kv - 0.001, -0.06 * len * g - 0.004 * g];
      k.cap(`finger-${i}-${sfx}`, `hand-${sfx}`, SKIN, T.skin, p0, p1, 0.0092, 0.0085, "hand-skin");
      k.cap(`fingertip-${i}-${sfx}`, `hand-${sfx}`, SKIN, T.skin, p1, p2, 0.0085, 0.0075, "hand-skin");
    }
    k.cap(`thumb-${sfx}`, `hand-${sfx}`, SKIN, T.skin, [0.025, 0.03, -0.006 * g], [0.07, 0.05, -0.018 * g], 0.011, 0.0095, "hand-skin");
    k.cap(`thumbtip-${sfx}`, `hand-${sfx}`, SKIN, T.skin, [0.07, 0.05, -0.018 * g], [0.1, 0.047, -0.03 * g], 0.0095, 0.008, "hand-skin");
  }
  k.loft("armband", "upper-arm-l", SHIRT, T.arm, [[0.19, 0.003, 0.002, 0.0495, 0.055], [0.25, 0.002, 0.001, 0.0465, 0.051]], "arm-clothing");
  k.box("armband-pouch", "upper-arm-l", SHIRT, T.arm, [0.22, 0.002, 0.05], [0.125, 0.068, 0.014], 0.005, "arm-clothing");
}

/* ------------------------------------------------------------------ body 1:100 */

function body100(k: Kit): void {
  k.loft("shirt", "torso", SHIRT, T.shirt, [[-0.085, 0.004, 0, 0.184, 0.142, 2.4], [-0.02, 0.002, 0, 0.17, 0.124, 2.4], [0.05, -0.006, 0, 0.152, 0.104, 2.3], [0.24, -0.004, 0, 0.165, 0.122, 2.3], [0.34, 0.003, 0, 0.167, 0.116, 2.4], [0.44, 0.008, 0, 0.178, 0.084, 2.2], [0.468, 0.008, 0, 0.15, 0.072, 2], [0.503, 0.005, 0, 0.076, 0.064, 2]], "torso-clothing");
  k.loft("neck", "head", SKIN, T.skin, [[-0.03, 0.012, 0, 0.066, 0.064], [0.03, 0.004, 0, 0.058, 0.06], [0.085, -0.006, 0, 0.056, 0.062]], "head-skin");
  k.loft("head", "head", SKIN, T.skin, [[0.01, -0.076, 0, 0.024, 0.02, 2.2], [0.045, -0.05, 0, 0.054, 0.056, 2.2], [0.085, -0.02, 0, 0.068, 0.087], [0.13, -0.01, 0, 0.075, 0.098], [0.18, -0.006, 0, 0.076, 0.097], [0.222, 0, 0, 0.058, 0.074], [0.2449, 0, 0, 0.02, 0.026]], "head-skin");
  k.loft("hair", "head", HAIR, T.hair, [[0.09, 0.044, 0, 0.058, 0.036], [0.13, 0.032, 0, 0.08, 0.064], [0.17, 0.011, 0, 0.083, 0.09], [0.2, 0.001, 0, 0.08, 0.096], [0.232, 0, 0, 0.058, 0.072], [0.2529, 0, 0, 0.022, 0.03]], "hair");
  k.loft("headband", "head", SHIRT, T.band, [[0.17, -0.001, 0, 0.086, 0.104], [0.2, 0.001, 0, 0.0835, 0.1005]], "hair", "detail");
  k.loft("shorts", "pelvis", SHIRT, T.shorts, [[-0.092, 0.012, 0, 0.12, 0.085, 2.4], [-0.055, 0.017, 0, 0.168, 0.117, 2.4], [-0.01, 0.019, 0, 0.175, 0.125, 2.4], [0.06, 0.006, 0, 0.162, 0.11, 2.3], [0.125, -0.002, 0, 0.148, 0.098, 2.3]], "leg-clothing");
  for (const [sfx, g] of SIDES) {
    k.loft(`shorts-leg-${sfx}`, `thigh-${sfx}`, SHIRT, T.shorts, m([[-0.03, 0.004, -0.004, 0.08, 0.095], [0.06, 0.007, -0.002, 0.09, 0.102], [0.16, 0.01, 0.001, 0.089, 0.099]], g), "leg-clothing", "body", 2.3);
    k.loft(`thigh-${sfx}`, `thigh-${sfx}`, SKIN, T.skin, m([[0.135, 0.009, -0.002, 0.08, 0.089], [0.21, 0.01, -0.001, 0.076, 0.084], [0.32, 0.008, 0, 0.064, 0.071], [0.41, 0.007, 0, 0.055, 0.062], [0.4447, 0.008, 0, 0.053, 0.061], [0.485, 0.004, 0, 0.032, 0.035]], g), "leg-skin");
    k.loft(`shank-${sfx}`, `shank-${sfx}`, SKIN, T.skin, [[-0.045, 0.004, 0, 0.03, 0.032], [-0.01, 0.006, 0, 0.052, 0.06], [0.05, -0.002, 0, 0.05, 0.054], [0.12, -0.01, 0, 0.054, 0.062], [0.24, -0.006, 0, 0.045, 0.05], [0.36, -0.002, 0, 0.032, 0.038], [0.4343, -0.001, 0, 0.034, 0.04], [0.458, -0.001, 0, 0.03, 0.036]], "leg-skin");
    k.loft(`sock-${sfx}`, `shank-${sfx}`, SOCK, T.sock, [[0.384, -0.002, 0, 0.035, 0.0425], [0.4343, -0.001, 0, 0.038, 0.044], [0.452, -0.001, 0, 0.036, 0.042]], "leg-skin");
    k.loft(`shoe-${sfx}`, `foot-${sfx}`, SHOE, T.shoe, m([[-0.074, -0.05, 0, 0.018, 0.022], [-0.058, -0.042, 0, 0.034, 0.04], [-0.02, -0.04, 0, 0.04, 0.0425], [0.03, -0.04, 0.002, 0.038, 0.0425], [0.08, -0.048, 0.002, 0.044, 0.0345], [0.1344, -0.056, 0.001, 0.05, 0.0265], [0.146, -0.057, 0.001, 0.049, 0.0255]], g), "shoes", "body", 3);
    k.loft(`shoe-toe-${sfx}`, `toe-${sfx}`, SHOE, T.shoe, m([[-0.004, 0.0265, 0.001, 0.05, 0.0265], [0.04, 0.026, -0.003, 0.046, 0.024], [0.075, 0.024, -0.006, 0.034, 0.017], [0.0952, 0.022, -0.008, 0.015, 0.008]], g), "shoes", "body", 3);
    k.loft(`sleeve-${sfx}`, `upper-arm-${sfx}`, SHIRT, T.shirt, m([[-0.03, 0, -0.012, 0.021, 0.029], [-0.008, 0, -0.003, 0.05, 0.054], [0.03, 0.001, 0.003, 0.0575, 0.0595], [0.1, 0.003, 0.004, 0.056, 0.058], [0.157, 0.003, 0.003, 0.0575, 0.06]], g), "arm-clothing");
    k.loft(`upper-arm-${sfx}`, `upper-arm-${sfx}`, SKIN, T.skin, m([[0.135, 0.004, 0.003, 0.048, 0.053], [0.19, 0.004, 0.003, 0.046, 0.052], [0.27, 0.001, 0.001, 0.041, 0.045], [0.3206, -0.002, 0, 0.042, 0.041], [0.354, -0.002, 0, 0.026, 0.026]], g), "arm-skin");
    k.loft(`forearm-${sfx}`, `forearm-${sfx}`, SKIN, T.skin, [[-0.034, 0, 0, 0.024, 0.024], [-0.005, 0, 0, 0.041, 0.039], [0.05, 0.002, 0, 0.044, 0.043], [0.15, 0.002, 0, 0.035, 0.037], [0.2792, 0, 0, 0.0205, 0.029], [0.295, 0, 0, 0.019, 0.027]], "arm-skin");
    k.loft(`hand-${sfx}`, `hand-${sfx}`, SKIN, T.skin, m([[-0.012, 0.002, 0, 0.019, 0.026], [0, 0.003, 0, 0.0205, 0.029], [0.05, 0.005, -0.003, 0.015, 0.04], [0.1, 0.002, -0.008, 0.014, 0.041], [0.135, 0.001, -0.02, 0.016, 0.038], [0.155, 0, -0.034, 0.014, 0.032]], g), "hand-skin", "body", 2.6);
    k.cap(`thumb-${sfx}`, `hand-${sfx}`, SKIN, T.skin, [0.03, 0.032, -0.006 * g], [0.09, 0.05, -0.022 * g], 0.0105, 0.0085, "hand-skin");
  }
  k.loft("armband", "upper-arm-l", SHIRT, T.arm, [[0.165, 0.003, 0.004, 0.052, 0.057], [0.275, 0.001, 0.003, 0.048, 0.05]], "arm-clothing");
}

/* ------------------------------------------------------------------ body 1:200 */

function body200(k: Kit): void {
  k.loft("shirt", "torso", SHIRT, T.shirt, [[-0.085, 0.004, 0, 0.184, 0.142, 2.4], [0.05, -0.006, 0, 0.153, 0.105, 2.3], [0.25, -0.004, 0, 0.166, 0.122, 2.3], [0.44, 0.008, 0, 0.178, 0.085, 2.2], [0.472, 0.008, 0, 0.14, 0.07, 2], [0.505, 0.005, 0, 0.074, 0.064, 2]], "torso-clothing");
  k.loft("head", "head", SKIN, T.skin, [[-0.03, 0.012, 0, 0.066, 0.064], [0.02, 0.002, 0, 0.058, 0.062], [0.05, -0.034, 0, 0.058, 0.07], [0.1, -0.014, 0, 0.072, 0.092], [0.17, -0.006, 0, 0.077, 0.097], [0.225, 0, 0, 0.056, 0.07], [0.2449, 0, 0, 0.02, 0.026]], "head-skin");
  k.loft("hair", "head", HAIR, T.hair, [[0.095, 0.042, 0, 0.062, 0.04], [0.15, 0.02, 0, 0.083, 0.078], [0.2, 0.001, 0, 0.081, 0.097], [0.235, 0, 0, 0.056, 0.07], [0.2529, 0, 0, 0.024, 0.03]], "hair");
  k.loft("shorts", "pelvis", SHIRT, T.shorts, [[-0.092, 0.012, 0, 0.125, 0.088, 2.4], [-0.03, 0.019, 0, 0.175, 0.124, 2.4], [0.05, 0.008, 0, 0.165, 0.112, 2.3], [0.125, -0.002, 0, 0.148, 0.098, 2.3]], "leg-clothing");
  for (const [sfx, g] of SIDES) {
    k.loft(`shorts-leg-${sfx}`, `thigh-${sfx}`, SHIRT, T.shorts, m([[-0.03, 0.004, -0.004, 0.08, 0.095], [0.05, 0.007, -0.002, 0.09, 0.102], [0.16, 0.01, 0.001, 0.0885, 0.0985]], g), "leg-clothing", "body", 2.3);
    k.loft(`thigh-${sfx}`, `thigh-${sfx}`, SKIN, T.skin, m([[0.135, 0.009, -0.002, 0.08, 0.089], [0.26, 0.01, 0, 0.072, 0.08], [0.4, 0.007, 0, 0.056, 0.062], [0.4447, 0.008, 0, 0.053, 0.061], [0.483, 0.004, 0, 0.034, 0.037]], g), "leg-skin");
    k.loft(`shank-${sfx}`, `shank-${sfx}`, SKIN, T.skin, [[-0.043, 0.004, 0, 0.032, 0.034], [-0.005, 0.006, 0, 0.053, 0.06], [0.12, -0.01, 0, 0.054, 0.062], [0.3, -0.004, 0, 0.038, 0.043], [0.38, -0.002, 0, 0.033, 0.04], [0.455, -0.001, 0, 0.036, 0.043]], "leg-skin");
    k.loft(`shoe-${sfx}`, `foot-${sfx}`, SHOE, T.shoe, m([[-0.074, -0.048, 0, 0.02, 0.026], [-0.05, -0.04, 0, 0.038, 0.0425], [0.03, -0.04, 0.002, 0.039, 0.0425], [0.1344, -0.056, 0.001, 0.05, 0.0265], [0.146, -0.057, 0.001, 0.049, 0.0255]], g), "shoes", "body", 3);
    k.loft(`shoe-toe-${sfx}`, `toe-${sfx}`, SHOE, T.shoe, m([[-0.004, 0.0265, 0.001, 0.05, 0.0265], [0.05, 0.026, -0.004, 0.044, 0.022], [0.0952, 0.022, -0.008, 0.018, 0.011]], g), "shoes", "body", 3);
    k.loft(`sleeve-${sfx}`, `upper-arm-${sfx}`, SHIRT, T.shirt, m([[-0.03, 0, -0.012, 0.024, 0.032], [0.0, 0, -0.001, 0.054, 0.057], [0.06, 0.002, 0.004, 0.0575, 0.0595], [0.157, 0.003, 0.003, 0.0575, 0.06]], g), "arm-clothing");
    k.loft(`upper-arm-${sfx}`, `upper-arm-${sfx}`, SKIN, T.skin, m([[0.135, 0.004, 0.003, 0.048, 0.053], [0.21, 0.004, 0.002, 0.045, 0.05], [0.3206, -0.002, 0, 0.042, 0.042], [0.352, -0.002, 0, 0.028, 0.028]], g), "arm-skin");
    k.loft(`forearm-${sfx}`, `forearm-${sfx}`, SKIN, T.skin, [[-0.032, 0, 0, 0.026, 0.026], [0.03, 0.002, 0, 0.044, 0.042], [0.16, 0.002, 0, 0.034, 0.036], [0.292, 0, 0, 0.02, 0.028]], "arm-skin");
    k.loft(`hand-${sfx}`, `hand-${sfx}`, SKIN, T.skin, m([[-0.01, 0.003, 0, 0.02, 0.028], [0.08, 0.006, -0.006, 0.017, 0.043], [0.15, 0.002, -0.025, 0.017, 0.034]], g), "hand-skin", "body", 2.6);
  }
}

/* ------------------------------------------------------------------ body 1:500 */

function body500(k: Kit): void {
  k.loft("shirt", "torso", SHIRT, T.shirt, [[-0.085, 0.004, 0, 0.184, 0.142, 2.4], [0.06, -0.006, 0, 0.154, 0.106, 2.3], [0.27, -0.003, 0, 0.167, 0.122, 2.3], [0.45, 0.008, 0, 0.174, 0.08, 2.2], [0.505, 0.005, 0, 0.078, 0.064, 2]], "torso-clothing");
  k.loft("head", "head", SKIN, T.skin, [[-0.03, 0.012, 0, 0.066, 0.064], [0.03, -0.012, 0, 0.058, 0.07], [0.1, -0.012, 0, 0.073, 0.094], [0.18, -0.006, 0, 0.076, 0.096], [0.2449, 0, 0, 0.03, 0.038]], "head-skin");
  k.loft("hair", "head", HAIR, T.hair, [[0.1, 0.04, 0, 0.066, 0.046], [0.19, 0.004, 0, 0.082, 0.096], [0.2529, 0, 0, 0.034, 0.042]], "hair");
  k.loft("shorts", "pelvis", SHIRT, T.shorts, [[-0.092, 0.012, 0, 0.13, 0.09, 2.4], [-0.02, 0.019, 0, 0.175, 0.124, 2.4], [0.125, -0.002, 0, 0.15, 0.1, 2.3]], "leg-clothing");
  for (const [sfx, g] of SIDES) {
    k.loft(`thigh-${sfx}`, `thigh-${sfx}`, SKIN, T.skin, m([[-0.02, 0.006, -0.003, 0.082, 0.096], [0.16, 0.01, 0, 0.086, 0.094], [0.4447, 0.007, 0, 0.054, 0.061], [0.48, 0.004, 0, 0.038, 0.04]], g), "leg-skin");
    k.loft(`shank-${sfx}`, `shank-${sfx}`, SKIN, T.skin, [[-0.04, 0.004, 0, 0.036, 0.038], [0.1, -0.008, 0, 0.054, 0.062], [0.36, -0.002, 0, 0.034, 0.04], [0.455, -0.001, 0, 0.036, 0.043]], "leg-skin");
    k.loft(`shoe-${sfx}`, `foot-${sfx}`, SHOE, T.shoe, m([[-0.074, -0.046, 0, 0.026, 0.03], [0.0, -0.04, 0.001, 0.04, 0.0425], [0.146, -0.056, 0.001, 0.05, 0.0265]], g), "shoes", "body", 3);
    k.loft(`shoe-toe-${sfx}`, `toe-${sfx}`, SHOE, T.shoe, m([[-0.004, 0.0265, 0.001, 0.05, 0.0265], [0.0952, 0.022, -0.006, 0.024, 0.014]], g), "shoes", "body", 3);
    k.loft(`sleeve-${sfx}`, `upper-arm-${sfx}`, SHIRT, T.shirt, m([[-0.03, 0, -0.01, 0.03, 0.036], [0.02, 0.001, 0.002, 0.057, 0.059], [0.157, 0.003, 0.003, 0.057, 0.06]], g), "arm-clothing");
    k.loft(`upper-arm-${sfx}`, `upper-arm-${sfx}`, SKIN, T.skin, m([[0.135, 0.004, 0.003, 0.048, 0.053], [0.35, -0.002, 0, 0.034, 0.034]], g), "arm-skin");
    k.loft(`forearm-${sfx}`, `forearm-${sfx}`, SKIN, T.skin, [[-0.03, 0, 0, 0.03, 0.03], [0.04, 0.002, 0, 0.044, 0.042], [0.29, 0, 0, 0.021, 0.029]], "arm-skin");
    k.loft(`hand-${sfx}`, `hand-${sfx}`, SKIN, T.skin, m([[-0.008, 0.003, 0, 0.02, 0.03], [0.15, 0.002, -0.02, 0.018, 0.038]], g), "hand-skin", "body", 2.6);
  }
}

/* ------------------------------------------------------------------ thermal */

/** 🕳️ Body solids whose thermal copy drops flat end faces (start, end); none: the validator measures open shells as more area, not less, so every thermal solid stays closed. */
const OPEN: [RegExp, boolean, boolean][] = [];

function thermal(asset: Json, k: Kit, scale: number): Json[] {
  const regions = new Map<string, Json>(asset.thermal.regions.map((r: Json) => [r.id, r]));
  const out: Json[] = [];
  const t = new Kit();
  const fine = true;
  const own = scale === 500 ? /^(sleeve|thigh)-/ : scale >= 100 ? /^sleeve-/ : /^$/;
  for (const s of k.solids) {
    const region = regions.get(k.region.get(s.id)!)!;
    if ((fine && (s.id === "head" || s.id === "neck")) || own.test(s.id)) continue;
    const open = OPEN.find(([re]) => re.test(s.id));
    if (open === undefined) out.push(...thermalOf(s, region, scale === 50));
    else out.push(...thermalOf({ ...s, capStart: open[1], capEnd: open[2] }, region, scale === 50).map((x) => (x.of === undefined ? x : { ...s, capStart: open[1], capEnd: open[2], id: x.id, region: x.region, ...(x.layer === undefined ? {} : { layer: x.layer }) })));
  }
  if (fine) {
    out.push({ ...t.loft("th-face", "head", SKIN, T.skin, [[0.008, -0.076, 0, 0.02, 0.016, 2.2], [0.028, -0.06, 0, 0.044, 0.04, 2.2], [0.055, -0.04, 0, 0.058, 0.062, 2.2], [0.085, -0.022, 0, 0.068, 0.081], [0.125, -0.032, 0, 0.073, 0.071], [0.165, -0.064, 0, 0.068, 0.04], [0.196, -0.08, 0, 0.05, 0.012]]), region: "head-skin" });
    out.push({ ...t.loft("th-neck", "head", SKIN, T.skin, [[0.0, 0.008, 0, 0.061, 0.062], [0.04, 0.002, 0, 0.057, 0.06], [0.085, -0.006, 0, 0.056, 0.062]]), region: "head-skin" });
  }
  if (scale >= 100)
    for (const [sfx, g] of SIDES) {
      const c = scale === 500;
      out.push({ ...t.loft(`th-shoulder-${sfx}`, `upper-arm-${sfx}`, SHIRT, T.shirt, m(c ? [[-0.03, 0, -0.01, 0.03, 0.036], [0.02, 0.001, 0.002, 0.057, 0.059], [0.066, 0.002, 0.003, 0.056, 0.059]] : [[-0.03, 0, -0.012, 0.022, 0.03], [-0.006, 0, -0.003, 0.051, 0.055], [0.03, 0.001, 0.003, 0.0575, 0.0595], [0.066, 0.002, 0.004, 0.057, 0.059]], g)), region: "torso-clothing" });
      out.push({ ...t.loft(`th-sleeve-${sfx}`, `upper-arm-${sfx}`, SHIRT, T.shirt, m(c ? [[0.066, 0.002, 0.003, 0.056, 0.059], [0.157, 0.003, 0.003, 0.0525, 0.0565]] : [[0.066, 0.002, 0.004, 0.057, 0.059], [0.157, 0.003, 0.003, 0.0575, 0.06]], g)), region: "arm-clothing" });
      if (!c) continue;
      out.push({ ...t.loft(`th-shorts-leg-${sfx}`, `thigh-${sfx}`, SHIRT, T.shorts, m([[-0.02, 0.006, -0.003, 0.082, 0.096], [0.16, 0.01, 0, 0.086, 0.094]], g)), region: "leg-clothing" });
      out.push({ ...t.loft(`th-thigh-${sfx}`, `thigh-${sfx}`, SKIN, T.skin, m([[0.14, 0.01, 0, 0.084, 0.092], [0.4447, 0.007, 0, 0.054, 0.061], [0.48, 0.004, 0, 0.038, 0.04]], g)), region: "leg-skin" });
    }
  const h = (0.0077 * scale) / 100;
  const e = (id: string, at: V3, radii: V3, region: string, emitter: string): Json => ({ ...t.ell(id, "head", region === "torso-clothing" ? HAIR : SKIN, 0.3, at, radii), region, tags: ["emitter", emitter] });
  out.push(e("th-nose-breath", [0.081, -0.115 + h, 0], [h, h, Math.max(h, 0.008)], "head-skin", "nose-breath"));
  out.push(e("th-mouth-breath", [0.052, -0.1035 + h, 0], [h, h, Math.max(h, 0.0105)], "head-skin", "mouth-breath"));
  out.push(e("th-plume", [0.2555 - h, 0, 0], [h, Math.max(0.05, h), Math.max(0.042, h)], "torso-clothing", "plume"));
  return out;
}

/* ------------------------------------------------------------------ structure */

const B = SKIN;
function masses(asset: Json, least = 0): Json[] {
  const k = new Kit();
  const out: Json[] = [];
  for (const b of asset.rig.bones as Json[]) {
    if (b.mass === undefined || b.mass.kg < 0.1 * asset.dimensions.massKg) continue;
    const r = Math.max(0.011 * Math.cbrt(b.mass.kg), least);
    out.push(...parts([k.ell(`mass-${b.id}`, b.id, B, 0.4, [b.mass.comX, b.mass.comY ?? 0, 0], [r, r, r])], b.id, "mass"));
  }
  return out;
}
/** 🔩️ Joint solids at the real joints: balls at hip and shoulder, hinge pins through knee, elbow, ankle and the toe joints, discs between the vertebrae at L5/S1 and C7/T1; `f` enlarges them for a coarser scale. */
const JOINT: Record<string, [at: V3, radii: V3]> = { torso: [[0, 0.04, 0], [0.007, 0.022, 0.029]], head: [[0, 0.031, 0], [0.006, 0.017, 0.02]], thigh: [[0, 0, 0], [0.027, 0.027, 0.027]], shank: [[0, 0, 0], [0.014, 0.014, 0.046]], foot: [[0, 0, 0], [0.011, 0.011, 0.031]], toe: [[0, 0.03, 0], [0.008, 0.008, 0.046]], "upper-arm": [[0, 0, 0], [0.026, 0.026, 0.026]], forearm: [[0, 0, 0], [0.01, 0.01, 0.037]], hand: [[0, 0, 0], [0.009, 0.026, 0.013]] };
function joints(ids: readonly string[], f: number): Json[] {
  const k = new Kit();
  return ids.flatMap((id) => {
    const [at, r] = JOINT[id.replace(/-[lr]$/, "")]!;
    return parts([k.ell(`joint-${id}`, id, B, 0.5, at, [r[0] * f, r[1] * f, Math.max(r[2], r[2] * (1 + (f - 1) / 3))])], id, "joint");
  });
}
const LIMB_JOINTS = ["thigh", "shank", "foot", "toe", "upper-arm", "forearm", "hand"].flatMap((b) => SIDES.map(([s]) => `${b}-${s}`));
const SPINE: Row[] = [[-0.01, 0.04, 0, 0.025, 0.018], [0.08, 0.03, 0, 0.024, 0.018], [0.17, 0.04, 0, 0.021, 0.017], [0.3, 0.062, 0, 0.018, 0.016], [0.42, 0.05, 0, 0.016, 0.014], [0.515, 0.032, 0, 0.015, 0.013]];

/** 📐 The fit of the shared adult skeleton table to the jogger: it is the reference figure itself. */
const FIT: Fit = { material: B, tone: T.bone, ks: 1, kv: 1, kw: 1, dv: 0, kp: 1, kpv: 1, drop: 1, shoulder: 0.4447, torso: "torso", pelvis: "pelvis" };
/** 💀 Face mass `[centre, radii]`, jaw joint, jaw angle and chin of the left side in the head bone (the bone starts at the base of the neck). */
const FACE: readonly [readonly [V3, V3], V3, V3, V3] = [[[0.096, -0.054, 0], [0.038, 0.032, 0.05]], [0.106, -0.008, 0.052], [0.05, -0.012, 0.047], [0.024, -0.076, 0.012]];
const byBone = (solids: readonly Json[]): Json[] => solids.map((s) => ({ ...s, member: s.bone, part: "member" }));

function structure50(asset: Json): Json[] {
  const out: Json[] = [];
  const put = (member: string, f: (k: Kit) => void): void => {
    const k = new Kit();
    f(k);
    out.push(...parts(k.solids, member, "member"));
  };
  put("head", (k) => {
    k.ell("sk-cranium", "head", B, T.bone, [0.162, -0.002, 0], [0.076, 0.09, 0.07]);
    k.loft("sk-cervical", "head", B, T.bone, [[-0.012, 0.031, 0, 0.016, 0.014], [0.03, 0.02, 0, 0.015, 0.013], [0.06, 0.014, 0, 0.014, 0.013], [0.095, 0.016, 0, 0.017, 0.014]]);
  });
  out.push(...parts(face(FIT, "head", 50, FACE[0], FACE[1], FACE[2], FACE[3], 0.008), "head", "member"), ...byBone(axial(FIT, 50)));
  for (const [sfx, g] of SIDES) {
    put(`thigh-${sfx}`, (k) => {
      k.ell(`sk-femur-head-${sfx}`, `thigh-${sfx}`, B, T.bone, [0, 0, 0], [0.024, 0.024, 0.024]);
      k.cap(`sk-femur-neck-${sfx}`, `thigh-${sfx}`, B, T.bone, [0, 0, 0], [0.032, 0, 0.048 * g], 0.015, 0.014);
      k.ell(`sk-trochanter-${sfx}`, `thigh-${sfx}`, B, T.bone, [0.03, -0.004, 0.055 * g], [0.03, 0.02, 0.014]);
      k.loft(`sk-femur-${sfx}`, `thigh-${sfx}`, B, T.bone, m([[0.04, 0, 0.05, 0.016, 0.016], [0.12, 0.006, 0.04, 0.0145, 0.0145], [0.25, 0.01, 0.022, 0.014, 0.014], [0.36, 0.006, 0.008, 0.016, 0.015], [0.41, 0.002, 0.002, 0.028, 0.022], [0.4447, 0, 0, 0.041, 0.032], [0.462, 0, 0, 0.034, 0.024]], g));
      k.ell(`sk-patella-${sfx}`, `thigh-${sfx}`, B, T.bone, [0.436, 0.044, 0], [0.022, 0.01, 0.02]);
    });
    put(`shank-${sfx}`, (k) => {
      k.loft(`sk-tibia-${sfx}`, `shank-${sfx}`, B, T.bone, m([[-0.012, 0, 0, 0.04, 0.03], [0.02, 0.004, -0.002, 0.03, 0.026], [0.07, 0.01, -0.004, 0.017, 0.019], [0.2, 0.01, -0.004, 0.013, 0.015], [0.36, 0.004, -0.004, 0.013, 0.013], [0.415, 0, -0.004, 0.02, 0.017], [0.44, 0, -0.008, 0.021, 0.016]], g));
      k.cap(`sk-fibula-${sfx}`, `shank-${sfx}`, B, T.bone, [0.035, -0.012, 0.034 * g], [0.442, -0.008, 0.0255 * g], 0.0075, 0.0065);
    });
    put(`foot-${sfx}`, (k) => {
      k.ell(`sk-talus-${sfx}`, `foot-${sfx}`, B, T.bone, [0.005, -0.014, 0], [0.028, 0.016, 0.02]);
      k.ell(`sk-calcaneus-${sfx}`, `foot-${sfx}`, B, T.bone, [-0.034, -0.035, 0.003 * g], [0.034, 0.017, 0.016]);
      k.loft(`sk-metatarsus-${sfx}`, `foot-${sfx}`, B, T.bone, [[0.02, -0.028, 0, 0.026, 0.016], [0.07, -0.04, 0, 0.032, 0.011], [0.115, -0.05, 0, 0.04, 0.008], [0.138, -0.054, 0, 0.042, 0.008]], undefined, "body", 2.6);
    });
    put(`toe-${sfx}`, (k) => k.loft(`sk-phalanges-${sfx}`, `toe-${sfx}`, B, T.bone, m([[0, 0.0285, 0, 0.04, 0.008], [0.035, 0.0275, -0.004, 0.034, 0.007], [0.07, 0.0265, -0.007, 0.02, 0.006]], g), undefined, "body", 2.6));
    put(`upper-arm-${sfx}`, (k) => {
      k.ell(`sk-humerus-head-${sfx}`, `upper-arm-${sfx}`, B, T.bone, [0, 0, -0.003 * g], [0.023, 0.023, 0.023]);
      k.loft(`sk-humerus-${sfx}`, `upper-arm-${sfx}`, B, T.bone, [[0.015, 0, 0, 0.017, 0.017], [0.08, 0.002, 0, 0.012, 0.012], [0.24, 0, 0, 0.0105, 0.0105], [0.295, 0, 0, 0.02, 0.011], [0.3206, 0, 0, 0.032, 0.014], [0.332, 0, 0, 0.026, 0.012]]);
    });
    put(`forearm-${sfx}`, (k) => {
      k.cap(`sk-ulna-${sfx}`, `forearm-${sfx}`, B, T.bone, [-0.012, -0.012, -0.008 * g], [0.272, -0.016, -0.002 * g], 0.011, 0.006);
      k.cap(`sk-radius-${sfx}`, `forearm-${sfx}`, B, T.bone, [0.012, 0.004, 0.018 * g], [0.272, 0.012, 0.002 * g], 0.008, 0.0105);
    });
    put(`hand-${sfx}`, (k) => {
      k.ell(`sk-carpus-${sfx}`, `hand-${sfx}`, B, T.bone, [0.014, 0.003, 0], [0.017, 0.024, 0.01]);
      k.loft(`sk-metacarpus-${sfx}`, `hand-${sfx}`, B, T.bone, m([[0.03, 0.004, -0.002, 0.007, 0.026], [0.097, 0.002, -0.005, 0.006, 0.035]], g), undefined, "body", 3);
      for (const [i, kv, ku, len] of [[1, 0.03, 0.098, 1], [2, 0.01, 0.1, 1.08], [3, -0.01, 0.097, 1], [4, -0.028, 0.09, 0.8]] as const) {
        const p1: V3 = [ku + 0.04 * len, kv, -0.024 * g];
        k.cap(`sk-phalanx-${i}-${sfx}`, `hand-${sfx}`, B, T.bone, [ku, kv, -0.006 * g], p1, 0.0045, 0.004);
        k.cap(`sk-phalanx-tip-${i}-${sfx}`, `hand-${sfx}`, B, T.bone, p1, [ku + 0.05 * len, kv - 0.001, -0.06 * len * g - 0.004 * g], 0.004, 0.004);
      }
      k.cap(`sk-thumb-${sfx}`, `hand-${sfx}`, B, T.bone, [0.025, 0.03, -0.006 * g], [0.07, 0.05, -0.018 * g], 0.005, 0.0045);
      k.cap(`sk-thumb-tip-${sfx}`, `hand-${sfx}`, B, T.bone, [0.07, 0.05, -0.018 * g], [0.098, 0.047, -0.03 * g], 0.0045, 0.004);
    });
  }
  out.push(...joints(["torso", "head", ...LIMB_JOINTS], 1), ...masses(asset));
  return out;
}

function structure100(asset: Json): Json[] {
  const out: Json[] = [];
  const put = (member: string, f: (k: Kit) => void): void => {
    const k = new Kit();
    f(k);
    out.push(...parts(k.solids, member, "member"));
  };
  put("head", (k) => {
    k.ell("sk-cranium", "head", B, T.bone, [0.16, -0.002, 0], [0.078, 0.09, 0.07]);
    k.loft("sk-cervical", "head", B, T.bone, [[-0.012, 0.031, 0, 0.016, 0.015], [0.045, 0.016, 0, 0.015, 0.015], [0.095, 0.016, 0, 0.017, 0.015]]);
  });
  out.push(...parts(face(FIT, "head", 100, FACE[0], FACE[1], FACE[2], FACE[3], 0.008), "head", "member"), ...byBone(axial(FIT, 100)));
  for (const [sfx, g] of SIDES) {
    put(`thigh-${sfx}`, (k) => k.loft(`sk-femur-${sfx}`, `thigh-${sfx}`, B, T.bone, m([[-0.022, 0, 0, 0.014, 0.014], [0, 0, 0.004, 0.024, 0.024], [0.03, 0, 0.038, 0.024, 0.02], [0.075, 0.004, 0.046, 0.016, 0.016], [0.25, 0.01, 0.022, 0.0145, 0.0145], [0.4, 0.003, 0.003, 0.024, 0.02], [0.4447, 0, 0, 0.041, 0.032], [0.462, 0, 0, 0.034, 0.024]], g)));
    put(`shank-${sfx}`, (k) => {
      k.loft(`sk-tibia-${sfx}`, `shank-${sfx}`, B, T.bone, m([[-0.012, 0, 0, 0.04, 0.03], [0.05, 0.008, -0.003, 0.02, 0.021], [0.3, 0.006, -0.004, 0.0135, 0.014], [0.44, 0, -0.007, 0.022, 0.017]], g));
      k.cap(`sk-fibula-${sfx}`, `shank-${sfx}`, B, T.bone, [0.035, -0.012, 0.033 * g], [0.442, -0.008, 0.025 * g], 0.008, 0.0075);
    });
    put(`foot-${sfx}`, (k) => k.loft(`sk-foot-${sfx}`, `foot-${sfx}`, B, T.bone, [[-0.066, -0.036, 0, 0.012, 0.012], [-0.035, -0.034, 0, 0.022, 0.019], [0.005, -0.02, 0, 0.026, 0.022], [0.07, -0.04, 0, 0.032, 0.011], [0.138, -0.054, 0, 0.042, 0.008]], undefined, "body", 2.6));
    put(`toe-${sfx}`, (k) => k.loft(`sk-phalanges-${sfx}`, `toe-${sfx}`, B, T.bone, m([[0, 0.0285, 0, 0.04, 0.008], [0.07, 0.0265, -0.007, 0.022, 0.0075]], g), undefined, "body", 2.6));
    put(`upper-arm-${sfx}`, (k) => k.loft(`sk-humerus-${sfx}`, `upper-arm-${sfx}`, B, T.bone, [[-0.02, 0, 0, 0.013, 0.013], [0, 0, 0, 0.023, 0.023], [0.035, 0, 0, 0.016, 0.016], [0.24, 0, 0, 0.011, 0.011], [0.3206, 0, 0, 0.032, 0.014], [0.332, 0, 0, 0.026, 0.012]]));
    put(`forearm-${sfx}`, (k) => {
      k.cap(`sk-ulna-${sfx}`, `forearm-${sfx}`, B, T.bone, [-0.012, -0.012, -0.008 * g], [0.272, -0.016, -0.002 * g], 0.011, 0.0075);
      k.cap(`sk-radius-${sfx}`, `forearm-${sfx}`, B, T.bone, [0.012, 0.004, 0.018 * g], [0.272, 0.012, 0.002 * g], 0.008, 0.0105);
    });
    put(`hand-${sfx}`, (k) => {
      k.loft(`sk-hand-${sfx}`, `hand-${sfx}`, B, T.bone, m([[0, 0.003, 0, 0.011, 0.022], [0.03, 0.004, -0.002, 0.008, 0.026], [0.098, 0.002, -0.005, 0.0075, 0.035]], g), undefined, "body", 3);
      k.loft(`sk-fingers-${sfx}`, `hand-${sfx}`, B, T.bone, m([[0.1, 0.002, -0.008, 0.0075, 0.034], [0.135, 0.001, -0.02, 0.0075, 0.032], [0.152, 0, -0.034, 0.0075, 0.028]], g), undefined, "body", 3);
    });
  }
  out.push(...joints(["torso", "head", ...LIMB_JOINTS], 1.25), ...masses(asset));
  return out;
}

function structureCoarse(asset: Json, scale: 200 | 500): Json[] {
  const out: Json[] = [];
  const c = scale === 500;
  const put = (member: string, f: (k: Kit) => void): void => {
    const k = new Kit();
    f(k);
    out.push(...parts(k.solids, member, "member"));
  };
  const r = 0.017;
  put("head", (k) => k.loft("sk-skull", "head", B, T.bone, c ? [[0.02, -0.04, 0, 0.0375, 0.0375], [0.1, -0.02, 0, 0.06, 0.07], [0.17, -0.004, 0, 0.07, 0.088], [0.238, 0, 0, 0.0375, 0.04]] : [[0.018, -0.06, 0, 0.02, 0.018], [0.05, -0.036, 0, 0.044, 0.046], [0.11, -0.022, 0, 0.062, 0.072], [0.17, -0.004, 0, 0.072, 0.09], [0.215, 0, 0, 0.052, 0.066], [0.238, 0, 0, 0.022, 0.03]]));
  if (c) out.push(...byBone([rod(FIT, "sk-cervical", "head", [-0.01, 0.034, 0], [0.11, 0.012, 0], 0.012)]));
  else put("head", (k) => k.loft("sk-cervical", "head", B, T.bone, [[-0.012, 0.032, 0, 0.017, 0.016], [0.04, 0.018, 0, 0.016, 0.0155], [0.09, 0.014, 0, 0.017, 0.016]]));
  out.push(...byBone(axial(FIT, scale)));
  for (const [sfx, g] of SIDES) {
    if (c) {
      out.push(
        ...byBone([
          rod(FIT, `sk-femur-${sfx}`, `thigh-${sfx}`, [0, 0, 0.04 * g], [0.4447, 0, 0], 0.014),
          rod(FIT, `sk-tibia-${sfx}`, `shank-${sfx}`, [0, 0.002, 0], [0.4343, 0, 0], 0.0125),
          rod(FIT, `sk-foot-${sfx}`, `foot-${sfx}`, [-0.05, -0.04, 0], [0.138, -0.052, 0], 0.009),
          rod(FIT, `sk-phalanges-${sfx}`, `toe-${sfx}`, [0, 0.0285, 0], [0.072, 0.0265, -0.004 * g], 0.004),
          rod(FIT, `sk-humerus-${sfx}`, `upper-arm-${sfx}`, [0, 0, 0], [0.3206, 0, 0], 0.0105),
          rod(FIT, `sk-forearm-${sfx}`, `forearm-${sfx}`, [0, 0, 0], [0.2792, 0, 0], 0.008),
          rod(FIT, `sk-hand-${sfx}`, `hand-${sfx}`, [0, 0.003, 0], [0.16, 0.002, -0.022 * g], 0.006),
        ]),
      );
      continue;
    }
    put(`thigh-${sfx}`, (k) => k.loft(`sk-femur-${sfx}`, `thigh-${sfx}`, B, T.bone, m([[-0.022, 0, 0, 0.016, 0.016], [0.005, 0, 0.008, 0.024, 0.024], [0.06, 0.003, 0.044, r, r], [0.3, 0.009, 0.016, r - 0.001, r - 0.001], [0.4447, 0, 0, 0.04, 0.03], [0.462, 0, 0, 0.034, 0.024]], g)));
    put(`shank-${sfx}`, (k) => k.loft(`sk-tibia-${sfx}`, `shank-${sfx}`, B, T.bone, [[-0.012, 0, 0, 0.04, 0.03], [0.06, 0.006, 0, 0.022, 0.022], [0.3, 0.004, 0, r - 0.001, r - 0.001], [0.44, 0, 0, 0.026, 0.018]]));
    put(`foot-${sfx}`, (k) => k.loft(`sk-foot-${sfx}`, `foot-${sfx}`, B, T.bone, [[-0.064, -0.036, 0, 0.014, 0.014], [-0.02, -0.03, 0, 0.024, 0.02], [0.01, -0.02, 0, 0.026, 0.022], [0.138, -0.053, 0, 0.042, 0.01]], undefined, "body", 2.6));
    put(`toe-${sfx}`, (k) => k.loft(`sk-phalanges-${sfx}`, `toe-${sfx}`, B, T.bone, m([[0, 0.029, 0, 0.04, 0.0155], [0.072, 0.027, -0.006, 0.024, 0.0155]], g), undefined, "body", 2.6));
    put(`upper-arm-${sfx}`, (k) => k.loft(`sk-humerus-${sfx}`, `upper-arm-${sfx}`, B, T.bone, [[-0.02, 0, 0, 0.014, 0.014], [0.0, 0, 0, 0.023, 0.023], [0.05, 0, 0, r - 0.002, r - 0.002], [0.25, 0, 0, r - 0.004, r - 0.004], [0.325, 0, 0, 0.03, 0.015]]));
    put(`forearm-${sfx}`, (k) => k.loft(`sk-forearm-${sfx}`, `forearm-${sfx}`, B, T.bone, [[-0.014, -0.004, 0, 0.022, 0.016], [0.08, -0.002, 0, 0.018, 0.017], [0.276, 0, 0, 0.012, 0.022]]));
    put(`hand-${sfx}`, (k) => k.loft(`sk-hand-${sfx}`, `hand-${sfx}`, B, T.bone, m([[0, 0.003, 0, 0.0155, 0.022], [0.09, 0.004, -0.006, 0.0155, 0.034], [0.148, 0.002, -0.025, 0.0155, 0.028]], g), undefined, "body", 3));
  }
  if (!c) out.push(...joints(["thigh", "shank", "foot", "upper-arm", "forearm"].flatMap((b) => SIDES.map(([s]) => `${b}-${s}`)), 1.6));
  out.push(...masses(asset, c ? 0.038 : 0));
  return out;
}

/* ------------------------------------------------------------------ decisions */

const DECISIONS: Record<number, Json[]> = {
  50: [
    { en: "Body 1:50: a lean 1.82 m runner modelled from anthropometric sections. The loose running shirt is one garment loft from the hem at hip height over waist, chest and the sloping shoulder line to the collar rib; the shoulder caps and the short sleeves ride on the upper arms so the shoulder stays closed in every arm swing. Head with jaw, chin, nose ridge, ears and eyes, a short hair cap and the head-band; shorts with the gluteal masses and two flared legs; thighs with the quadriceps bulge, knee caps, calves that bulge to the back over a straight shin, ankle socks; running shoes with a separate midsole, rocker toe and an upper that rises to the heel collar; forearms that turn from the wide elbow to the upright wrist, hands with a cupped palm, four curled fingers and the thumb; an armband with phone pouch on the left arm.", de: "Körper 1:50: ein schlanker, 1,82 m grosser Läufer aus anthropometrischen Querschnitten. Das weite Laufshirt ist ein Kleidungsstück vom Saum auf Hüfthöhe über Taille, Brust und die abfallende Schulterlinie bis zum Kragenbündchen; Schulterkappen und kurze Ärmel sitzen auf den Oberarmen, sodass die Schulter in jedem Armschwung geschlossen bleibt. Kopf mit Kiefer, Kinn, Nasenrücken, Ohren und Augen, kurzer Haarkappe und Stirnband; Shorts mit den Gesässmassen und zwei ausgestellten Beinen; Oberschenkel mit dem Quadrizepsbauch, Kniescheiben, Waden, die über dem geraden Schienbein nach hinten ausladen, Knöchelsocken; Laufschuhe mit eigener Zwischensohle, aufgebogener Spitze und einem Schaft bis zum Fersenkragen; Unterarme, die sich vom breiten Ellbogen zum hochkant stehenden Handgelenk drehen, Hände mit hohler Handfläche, vier gekrümmten Fingern und Daumen; ein Armband mit Telefontasche am linken Arm." },
    { en: "Thermal 1:50: every region is the body surface it names; shirt, sleeves and shorts are two nested shells (air gap under the fabric), the shoes three (sock, air, upper), each inset by the real layer thickness. Skin runs only a few centimetres under a garment. The breath outlets are small patches under the nose tip and on the lips, the plume leaves from a patch on the crown.", de: "Thermik 1:50: Jede Region ist die Körperoberfläche, die sie benennt; Shirt, Ärmel und Shorts sind zwei ineinanderliegende Schalen (Luftspalt unter dem Stoff), die Schuhe drei (Socke, Luft, Schaft), jeweils um die echte Schichtdicke eingerückt. Haut läuft nur wenige Zentimeter unter ein Kleidungsstück. Die Atemauslässe sind kleine Flächen unter der Nasenspitze und auf den Lippen, die Warmluftfahne verlässt den Körper an einer Fläche auf dem Scheitel." },
    structureText(50, { en: "the runner", de: "des Läufers" }, { en: "The structure declares no supports (inertial model).", de: "Das Tragwerk nennt keine Auflager (Trägheitsmodell)." }),
  ],
  100: [
    { en: "Body 1:100: the same runner drawn anew with fewer, longer sections. Nose, ears, eyes, knee caps and the phone pouch fall below 15 mm and are left out; shoulder cap and sleeve are one garment, the gluteal masses are part of the shorts, sole and upper are one shoe form, the fingers are one curled mitten with a separate thumb.", de: "Körper 1:100: derselbe Läufer, neu gezeichnet mit weniger und längeren Querschnitten. Nase, Ohren, Augen, Kniescheiben und Telefontasche liegen unter 15 mm und entfallen; Schulterkappe und Ärmel sind ein Kleidungsstück, die Gesässmassen gehören zu den Shorts, Sohle und Schaft sind eine Schuhform, die Finger ein gekrümmter Fäustling mit eigenem Daumen." },
    { en: "Thermal 1:100: one surface per region and body part, layers merged into it; the outlets are enlarged to the scale.", de: "Thermik 1:100: eine Oberfläche je Region und Körperteil, die Schichten sind darin zusammengefasst; die Auslässe sind auf den Massstab vergrössert." },
    structureText(100, { en: "the runner", de: "des Läufers" }, { en: "The structure declares no supports (inertial model).", de: "Das Tragwerk nennt keine Auflager (Trägheitsmodell)." }),
  ],
  200: [
    { en: "Body 1:200: masses of at least 30 mm. Head and neck are one form under the hair cap, the head-band and armband are gone, the sock is part of the shank, the hand is a mitten; shirt, shorts, sleeves and shoes keep their own forms because their tones carry the figure.", de: "Körper 1:200: Massen ab 30 mm. Kopf und Hals sind eine Form unter der Haarkappe, Stirnband und Armband entfallen, die Socke gehört zum Unterschenkel, die Hand ist ein Fäustling; Shirt, Shorts, Ärmel und Schuhe behalten eigene Formen, weil ihre Tonwerte die Figur tragen." },
    { en: "Thermal 1:200: one surface per region and body part.", de: "Thermik 1:200: eine Oberfläche je Region und Körperteil." },
    structureText(200, { en: "the runner", de: "des Läufers" }, { en: "The structure declares no supports (inertial model).", de: "Das Tragwerk nennt keine Auflager (Trägheitsmodell)." }),
  ],
  500: [
    { en: "Body 1:500: the figure in its fewest masses: head with neck, hair, shirt, shorts, and per side thigh, shank, shoe, toe cap, sleeve, upper arm, forearm and hand. Proportions and the running pose still read at a glance.", de: "Körper 1:500: die Figur in ihren wenigsten Massen: Kopf mit Hals, Haar, Shirt, Shorts und je Seite Oberschenkel, Unterschenkel, Schuh, Zehenkappe, Ärmel, Oberarm, Unterarm und Hand. Proportionen und Laufhaltung bleiben auf einen Blick lesbar." },
    { en: "Thermal 1:500: one surface per region; shoulder cap and sleeve, one form in the body, are split at the sleeve seam, and the thigh, one form in the body, is split at the shorts hem.", de: "Thermik 1:500: eine Oberfläche je Region; Schulterkappe und Ärmel, im Körper eine Form, sind an der Ärmelnaht geteilt, und der Oberschenkel, im Körper eine Form, ist am Hosensaum geteilt." },
    structureText(500, { en: "the runner", de: "des Läufers" }, { en: "The structure declares no supports (inertial model).", de: "Das Tragwerk nennt keine Auflager (Trägheitsmodell)." }),
  ],
};

/* ------------------------------------------------------------------ assembly */

export const SCALES = [50, 100, 200, 500] as const;
export type Built = { volume: Json; kit: Kit };

/** 🏗️ The three lists and the decisions of one scale; `kit` keeps the thermal region of every body solid for the drawing. */
export function build(asset: Json, scale: (typeof SCALES)[number]): Built {
  const kit = new Kit();
  ({ 50: body50, 100: body100, 200: body200, 500: body500 })[scale](kit);
  const structure = scale === 50 ? structure50(asset) : scale === 100 ? structure100(asset) : structureCoarse(asset, scale);
  return { kit, volume: { status: "authored", denominator: scale, body: kit.solids, thermal: thermal(asset, kit, scale), structure, decisions: DECISIONS[scale]! } };
}

export const ASSET_FILE = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/jogger/🔣️.json";

if (import.meta.main) {
  const asset = JSON.parse(await Bun.file(ASSET_FILE).text()).asset as Json;
  const volumes = Object.fromEntries(SCALES.map((s) => [String(s), build(asset, s).volume]));
  writeFileSync(join(import.meta.dir, "volumes", "humans", "jogger.json"), `${JSON.stringify({ volumes })}\n`);
  for (const s of SCALES) console.log(s, "body", volumes[s].body.length, "thermal", volumes[s].thermal.length, "structure", volumes[s].structure.length);
}
