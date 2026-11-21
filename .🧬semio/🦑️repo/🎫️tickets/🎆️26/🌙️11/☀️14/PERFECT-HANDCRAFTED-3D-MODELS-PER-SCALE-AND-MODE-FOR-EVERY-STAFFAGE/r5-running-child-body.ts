/** 🧒 The round 5 body of `humans/running-child` at 1:50, 1:100, 1:200 and 1:500, every key section stated by hand from the reference table of `r3-humans-running-child.md` (round 5 section): a slim six-year-old of 1.125 m barefoot (1.137 m in sneakers) in a loose T-shirt, short shorts, ankle socks, canvas sneakers and a small backpack. The lofts run through a smooth spline of the stated key sections, so every limb is one continuous form; a child's proportions: head 1/6 of the stature, short neck, narrow sloping shoulders, a round belly that stands forward of the chest, soft knees, slim limbs without muscle relief. Trunk bones (pelvis, torso, neck, head, pack): u up, v to the back, w to the left. Limb bones: u distal, v to the front, w to the left. Foot rows are stated in upright axes (x ahead, y up from the ankle), toe rows in the toe frame (u ahead, v up from the ground).
 * @see ./r5-runners-kit.ts
 * @see ./r5-running-child-figure.ts */
import { FOOT, HAIR, PACK, SHIRT, SHOE, SHORTS, SIDES, SKIN, SOCK, STRAP, T, m } from "./r3-running-child-body.ts";
import type { Kit, V3 } from "./r3-running-child-kit.ts";
import { STRAP_COARSE } from "./r5-running-child-lists.ts";
import { smooth, tube, type Row } from "./r5-runners-kit.ts";

const mp = (path: readonly V3[], g: number): V3[] => path.map(([u, v, w]) => [u, v, w * g] as V3);
/** 〰️ A strap path through the stated points, smoothed by the same spline as the lofts. */
const curve = (path: readonly V3[], n = 2): V3[] => smooth(path.map(([u, v, w]) => [u, v, w, 0.01, 0.01] as Row), n).map(([u, v, w]) => [u, v, w] as V3);
const sl = (k: Kit, id: string, bone: string, material: string, tone: number, keys: readonly Row[], region: string, role = "body", e = 2, n = 2) => k.loft(id, bone, material, tone, smooth(keys, n), region, role, e);
const st = (k: Kit, id: string, bone: string, material: string, tone: number, keys: readonly Row[], region: string, role = "body", e = 2, n = 1) => k.turned(id, bone, material, tone, FOOT, smooth(keys, n), region, role, e);

/** 👕 T-shirt (size 116) over a child's trunk: loose hem, the round belly standing forward of the chest (waist 0.21 x 0.18 m with the shirt), chest 0.22 x 0.16 m, narrow shoulders (biacromial 0.243 m) and the trapezius slope rising to the collar. */
const SHIRT50: Row[] = [[-0.072, 0.008, 0, 0.118, 0.096, 2.3], [-0.03, 0.004, 0, 0.112, 0.091, 2.3], [0.04, -0.005, 0, 0.106, 0.089, 2.2], [0.11, -0.005, 0, 0.105, 0.085, 2.2], [0.17, -0.003, 0, 0.108, 0.082, 2.2], [0.21, 0.0, 0, 0.113, 0.075, 2.3], [0.235, 0.003, 0, 0.114, 0.066, 2.3], [0.255, 0.005, 0, 0.097, 0.057, 2.2], [0.272, 0.006, 0, 0.073, 0.05, 2.1], [0.287, 0.006, 0, 0.054, 0.047, 2], [0.297, 0.006, 0, 0.049, 0.046, 2]];
/** 🗿 A child's head (length 0.18 m, breadth 0.138 m, chin to vertex 0.19 m): small jaw and chin, round cheeks, a high forehead and the full occiput. */
const HEAD50: Row[] = [[-0.051, -0.056, 0, 0.021, 0.014, 2.2], [-0.04, -0.05, 0, 0.037, 0.029, 2.2], [-0.022, -0.036, 0, 0.051, 0.046, 2.2], [0, -0.016, 0, 0.059, 0.067, 2.1], [0.03, -0.004, 0, 0.063, 0.083], [0.06, 0.002, 0, 0.069, 0.089], [0.09, 0.004, 0, 0.0635, 0.082], [0.112, 0.005, 0, 0.05, 0.064], [0.127, 0.006, 0, 0.03, 0.038], [0.1335, 0.006, 0, 0.01, 0.013]];
const HAIR50: Row[] = [[-0.03, 0.05, 0, 0.026, 0.012], [-0.008, 0.052, 0, 0.036, 0.02], [0.015, 0.04, 0, 0.056, 0.034], [0.04, 0.03, 0, 0.0685, 0.062], [0.066, 0.012, 0, 0.076, 0.088], [0.09, 0.004, 0, 0.0715, 0.0905], [0.112, 0.005, 0, 0.058, 0.072], [0.128, 0.006, 0, 0.04, 0.05], [0.141, 0.006, 0, 0.016, 0.02]];
/** 🧣 A short child's neck (girth 0.26 m) widening into the trapezius at its base. */
const NECK50: Row[] = [[-0.026, 0.006, 0, 0.05, 0.046], [0.0, 0.004, 0, 0.045, 0.044], [0.03, 0.002, 0, 0.042, 0.042], [0.06, 0, 0, 0.041, 0.042], [0.088, 0, 0, 0.041, 0.043]];
/** 🩳 Shorts on the pelvis: waistband at 0.65 m, hips 0.21 m, the small gluteal mass behind, crotch at 0.51 m. */
const SHORTS50: Row[] = [[0.078, 0.001, 0, 0.092, 0.068, 2.3], [0.05, 0.004, 0, 0.096, 0.072, 2.3], [0.02, 0.01, 0, 0.102, 0.078, 2.4], [-0.01, 0.016, 0, 0.106, 0.084, 2.4], [-0.035, 0.019, 0, 0.102, 0.085, 2.4], [-0.052, 0.015, 0, 0.085, 0.072, 2.3], [-0.064, 0.01, 0, 0.055, 0.048, 2.2]];
const SHORTS_LEG50: Row[] = [[-0.046, -0.004, -0.005, 0.026, 0.034], [-0.03, -0.004, -0.004, 0.043, 0.053], [-0.005, -0.003, -0.004, 0.051, 0.062], [0.03, -0.002, -0.003, 0.054, 0.065], [0.075, 0.0, -0.001, 0.055, 0.064], [0.115, 0.002, 0, 0.056, 0.064]];
/** 🦵 Thigh (girth 0.33 / 0.29 / 0.24 m at the top, middle and above the knee), knee 0.071 m wide, no muscle relief. */
const THIGH50: Row[] = [[0.095, 0.001, -0.001, 0.047, 0.052], [0.14, 0.003, 0, 0.044, 0.049], [0.19, 0.004, 0, 0.0395, 0.044], [0.23, 0.004, 0, 0.036, 0.041], [0.2565, 0.004, 0, 0.0355, 0.04], [0.272, 0.002, 0, 0.031, 0.034], [0.284, 0.001, 0, 0.02, 0.022]];
/** 🦵 Shank: soft knee, the slim calf set back (girth 0.235 m), ankle 0.155 m. */
const SHANK50: Row[] = [[-0.028, 0.002, 0, 0.02, 0.022], [-0.012, 0.004, 0, 0.032, 0.036], [0.005, 0.004, 0, 0.0355, 0.039], [0.04, -0.003, 0, 0.0348, 0.039], [0.085, -0.009, 0, 0.0352, 0.041], [0.13, -0.007, 0, 0.031, 0.036], [0.18, -0.004, 0, 0.025, 0.03], [0.225, -0.002, 0, 0.0215, 0.027], [0.25, -0.001, 0, 0.0235, 0.028], [0.275, -0.001, 0, 0.022, 0.026]];
/** 👟 Sneaker EU 28 (0.19 x 0.075 m): sole with rounded heel, upper with heel counter and collar, the instep rising to the tongue. */
const SOLE50: Row[] = [[-0.0505, -0.048, 0, 0.018, 0.008, 4], [-0.04, -0.048, 0, 0.027, 0.0085, 4], [-0.006, -0.048, 0, 0.03, 0.0085, 4], [0.029, -0.048, 0.001, 0.032, 0.0085, 4], [0.059, -0.048, 0.002, 0.035, 0.0085, 4], [0.084, -0.048, 0.002, 0.0365, 0.0085, 4]];
const UPPER50: Row[] = [[-0.051, -0.024, 0, 0.018, 0.019, 2.6], [-0.038, -0.019, 0, 0.027, 0.026, 2.6], [-0.005, -0.019, 0, 0.031, 0.026, 2.6], [0.03, -0.02, 0.001, 0.033, 0.0245, 2.6], [0.06, -0.025, 0.002, 0.036, 0.0195, 2.6], [0.083, -0.028, 0.002, 0.0375, 0.016, 2.6]];
const SOLE_TOE50: Row[] = [[-0.001, 0.008, 0.002, 0.0365, 0.008, 4], [0.019, 0.008, 0.001, 0.035, 0.008, 4], [0.039, 0.0085, -0.001, 0.029, 0.0075, 4], [0.051, 0.0095, -0.002, 0.018, 0.0065, 4], [0.0555, 0.011, -0.002, 0.008, 0.005, 4]];
const UPPER_TOE50: Row[] = [[-0.003, 0.028, 0.002, 0.0375, 0.0155, 2.6], [0.02, 0.027, 0.001, 0.036, 0.0148, 2.6], [0.04, 0.025, -0.001, 0.03, 0.0125, 2.6], [0.052, 0.023, -0.002, 0.019, 0.0095, 2.6], [0.056, 0.022, -0.002, 0.008, 0.0055, 2.6]];
/** 👕 Sleeve with the shoulder cap: from the acromion it continues the trapezius slope over the humeral head into the short sleeve (hem 0.10 m below the shoulder joint). */
const SLEEVE50: Row[] = [[-0.026, 0, -0.032, 0.008, 0.016], [-0.02, 0, -0.019, 0.019, 0.027], [-0.01, 0, -0.007, 0.029, 0.034], [0.006, 0.0, 0.002, 0.035, 0.038], [0.03, 0.001, 0.003, 0.0362, 0.04], [0.065, 0.001, 0.003, 0.0365, 0.0405], [0.1, 0.001, 0.003, 0.038, 0.042]];
const UPPER_ARM50: Row[] = [[0.088, 0.002, 0.002, 0.026, 0.03], [0.118, 0.003, 0.002, 0.0255, 0.0295], [0.15, 0.001, 0.001, 0.0235, 0.026], [0.1848, -0.002, 0, 0.0235, 0.0235], [0.198, -0.002, 0, 0.02, 0.02], [0.207, -0.002, 0, 0.012, 0.012]];
const FOREARM50: Row[] = [[-0.021, -0.002, 0, 0.012, 0.012], [-0.011, -0.001, 0, 0.0205, 0.0205], [0.0, 0, 0, 0.0235, 0.023], [0.03, 0.001, 0, 0.0255, 0.0255], [0.07, 0.001, 0, 0.023, 0.0235], [0.11, 0, 0, 0.0185, 0.021], [0.1456, 0, 0, 0.0155, 0.0205], [0.156, 0, 0, 0.0138, 0.019]];
/** ✋ A child's hand (length 0.12 m, breadth 0.056 m, thickness 0.019 m) held loosely while running: palm, the four fingers grouped and curled, the thumb along the index finger. */
const PALM50: Row[] = [[-0.008, 0, 0, 0.012, 0.0175], [0.0, 0.001, 0, 0.0125, 0.019], [0.02, 0.002, -0.001, 0.011, 0.024], [0.046, 0.001, -0.002, 0.0095, 0.028], [0.063, 0.0, -0.003, 0.0088, 0.027], [0.069, -0.001, -0.004, 0.0075, 0.024]];
const FINGERS50: Row[] = [[0.06, -0.002, -0.003, 0.0085, 0.025, 2.4], [0.078, -0.002, -0.007, 0.008, 0.024, 2.4], [0.088, -0.002, -0.017, 0.0078, 0.023, 2.4], [0.087, -0.002, -0.029, 0.0075, 0.021, 2.4], [0.079, -0.002, -0.037, 0.0072, 0.018, 2.4]];
const THUMB50: Row[] = [[0.008, 0.013, -0.005, 0.0095, 0.009], [0.03, 0.024, -0.009, 0.0085, 0.0078], [0.05, 0.029, -0.015, 0.0072, 0.0066], [0.064, 0.027, -0.021, 0.006, 0.0055], [0.069, 0.025, -0.024, 0.0045, 0.0045]];
/** 🎒 Small child's backpack 0.20 x 0.26 x 0.09 m. */
const PACK50: Row[] = [[-0.085, 0.075, 0, 0.08, 0.03, 4], [-0.07, 0.079, 0, 0.096, 0.042, 4.5], [0.0, 0.08, 0, 0.1, 0.045, 4.5], [0.1, 0.079, 0, 0.098, 0.043, 4.5], [0.15, 0.075, 0, 0.09, 0.038, 4], [0.168, 0.07, 0, 0.075, 0.03, 3.5], [0.175, 0.066, 0, 0.05, 0.02, 3]];

/* ------------------------------------------------------------------ 1:50 */

export function body50(k: Kit): void {
  sl(k, "shirt", "torso", SHIRT, T.shirt, SHIRT50, "tshirt", "body", 2.2);
  k.loft("collar", "torso", SHIRT, T.collar, [[0.286, 0.006, 0, 0.055, 0.05], [0.298, 0.005, 0, 0.05, 0.0475]], "tshirt", "detail");
  sl(k, "neck", "neck", SKIN, T.skin, NECK50, "face-skin");
  sl(k, "head", "head", SKIN, T.skin, HEAD50, "face-skin");
  tube(k, "nose", "head", SKIN, T.skin, [[0.032, -0.084, 0, 0.004, 0.0045], [0.017, -0.09, 0, 0.0052, 0.0065], [0.005, -0.0935, 0, 0.0058, 0.0082], [0.0, -0.0925, 0, 0.0042, 0.007]], "face-skin");
  for (const [sfx, g] of SIDES) {
    k.ell(`ear-${sfx}`, "head", SKIN, T.skin, [0.017, 0.008, 0.063 * g], [0.024, 0.013, 0.006], "face-skin", "detail", [0.1305 * g, 0, 0, 0.9914]);
    k.ell(`eye-${sfx}`, "head", SKIN, T.eye, [0.032, -0.0795, 0.027 * g], [0.0045, 0.004, 0.009], "face-skin", "detail");
  }
  sl(k, "hair", "head", HAIR, T.hair, HAIR50, "hair");
  sl(k, "shorts", "pelvis", SHORTS, T.shorts, SHORTS50, "shorts", "body", 2.3);
  sl(k, "pack", "pack", PACK, T.pack, PACK50, "backpack", "body", 4, 1);
  k.box("pack-pocket", "pack", PACK, T.pocket, [0.005, 0.128, 0], [0.11, 0.02, 0.14], 0.008, "backpack", "detail");
  k.sweep("pack-handle", "pack", PACK, T.strap, [[0.168, 0.05, 0], [0.184, 0.054, 0], [0.19, 0.066, 0], [0.184, 0.078, 0], [0.168, 0.082, 0]], 0.011, 0.004, "backpack", "detail", 3);
  for (const [sfx, g] of SIDES) {
    k.sweep(`strap-${sfx}`, "torso", PACK, T.strap, mp(curve(STRAP), g), 0.011, 0.004, "backpack", "detail", 3).slender = true;
    sl(k, `shorts-leg-${sfx}`, `thigh-${sfx}`, SHORTS, T.shorts, m(SHORTS_LEG50, g), "shorts", "body", 2.3, 1);
    sl(k, `thigh-${sfx}`, `thigh-${sfx}`, SKIN, T.skin, m(THIGH50, g), "leg-skin");
    k.ell(`patella-${sfx}`, `shank-${sfx}`, SKIN, T.skin, [-0.003, 0.03, 0], [0.019, 0.011, 0.017], "leg-skin");
    sl(k, `shank-${sfx}`, `shank-${sfx}`, SKIN, T.skin, SHANK50, "leg-skin");
    sl(k, `sock-${sfx}`, `shank-${sfx}`, SOCK, T.sock, [[0.222, -0.002, 0, 0.0245, 0.03], [0.245, -0.001, 0, 0.026, 0.031], [0.262, -0.001, 0, 0.027, 0.0315], [0.272, -0.001, 0, 0.026, 0.03]], "socks", "body", 2, 1);
    st(k, `sole-${sfx}`, `foot-${sfx}`, SHOE, T.sole, m(SOLE50, g), "shoes", "body", 4);
    st(k, `shoe-${sfx}`, `foot-${sfx}`, SHOE, T.shoe, m(UPPER50, g), "shoes", "body", 2.6);
    sl(k, `sole-toe-${sfx}`, `toe-${sfx}`, SHOE, T.sole, m(SOLE_TOE50, g), "shoes", "body", 4, 1);
    sl(k, `shoe-toe-${sfx}`, `toe-${sfx}`, SHOE, T.shoe, m(UPPER_TOE50, g), "shoes", "body", 2.6, 1);
    sl(k, `sleeve-${sfx}`, `arm-${sfx}`, SHIRT, T.shirt, m(SLEEVE50, g), "tshirt");
    sl(k, `upper-arm-${sfx}`, `arm-${sfx}`, SKIN, T.skin, m(UPPER_ARM50, g), "arm-skin");
    sl(k, `forearm-${sfx}`, `forearm-${sfx}`, SKIN, T.skin, FOREARM50, "arm-skin");
    sl(k, `hand-${sfx}`, `hand-${sfx}`, SKIN, T.skin, m(PALM50, g), "arm-skin", "body", 2.6, 1);
    tube(k, `finger-${sfx}`, `hand-${sfx}`, SKIN, T.skin, m(FINGERS50, g), "arm-skin");
    tube(k, `thumb-${sfx}`, `hand-${sfx}`, SKIN, T.skin, m(THUMB50, g), "arm-skin");
  }
}

/* ------------------------------------------------------------------ 1:100 */

/** 🧤 1:100 (smallest feature 15 mm): nose, ears, eyes, knee caps, collar and the pack handle are left out; sole and upper are one shoe form; palm, fingers and thumb are one curled mitten; straps stay slender bands. */
function body100(k: Kit): void {
  sl(k, "shirt", "torso", SHIRT, T.shirt, [[-0.072, 0.008, 0, 0.118, 0.096, 2.3], [0.04, -0.005, 0, 0.106, 0.089, 2.2], [0.17, -0.003, 0, 0.108, 0.082, 2.2], [0.235, 0.003, 0, 0.114, 0.066, 2.3], [0.272, 0.006, 0, 0.073, 0.05, 2.1], [0.297, 0.006, 0, 0.049, 0.046, 2]], "tshirt", "body", 2.2);
  sl(k, "neck", "neck", SKIN, T.skin, [[-0.026, 0.006, 0, 0.05, 0.046], [0.03, 0.002, 0, 0.042, 0.042], [0.088, 0, 0, 0.041, 0.043]], "face-skin", "body", 2, 1);
  sl(k, "head", "head", SKIN, T.skin, [[-0.051, -0.056, 0, 0.023, 0.016, 2.2], [-0.03, -0.042, 0, 0.045, 0.04, 2.2], [0.002, -0.017, 0, 0.059, 0.069, 2.1], [0.045, -0.001, 0, 0.066, 0.086], [0.085, 0.004, 0, 0.065, 0.084], [0.118, 0.005, 0, 0.044, 0.056], [0.1335, 0.006, 0, 0.012, 0.016]], "face-skin");
  sl(k, "hair", "head", HAIR, T.hair, [[-0.008, 0.05, 0, 0.035, 0.019], [0.02, 0.038, 0, 0.059, 0.04], [0.05, 0.022, 0, 0.073, 0.074], [0.085, 0.004, 0, 0.073, 0.091], [0.12, 0.006, 0, 0.05, 0.062], [0.141, 0.006, 0, 0.017, 0.021]], "hair");
  sl(k, "shorts", "pelvis", SHORTS, T.shorts, [[0.078, 0.001, 0, 0.092, 0.068, 2.3], [0.02, 0.01, 0, 0.102, 0.078, 2.4], [-0.03, 0.019, 0, 0.104, 0.085, 2.4], [-0.064, 0.01, 0, 0.058, 0.05, 2.2]], "shorts", "body", 2.3, 1);
  k.loft("pack", "pack", PACK, T.pack, [[-0.085, 0.076, 0, 0.084, 0.033, 4], [-0.06, 0.08, 0, 0.099, 0.045, 4.5], [0.11, 0.079, 0, 0.098, 0.043, 4.5], [0.16, 0.073, 0, 0.084, 0.034, 3.5], [0.175, 0.067, 0, 0.055, 0.022, 3]], "backpack");
  k.box("pack-pocket", "pack", PACK, T.pocket, [0.005, 0.1275, 0], [0.11, 0.021, 0.14], 0.008, "backpack", "detail");
  for (const [sfx, g] of SIDES) {
    k.sweep(`strap-${sfx}`, "torso", PACK, T.strap, mp(STRAP_COARSE, g), 0.011, 0.004, "backpack", "detail", 3).slender = true;
    k.loft(`shorts-leg-${sfx}`, `thigh-${sfx}`, SHORTS, T.shorts, m([[-0.045, -0.004, -0.005, 0.028, 0.036], [-0.02, -0.003, -0.004, 0.048, 0.059], [0.04, -0.001, -0.002, 0.055, 0.065], [0.115, 0.002, 0, 0.056, 0.064]], g), "shorts", "body", 2.3);
    sl(k, `thigh-${sfx}`, `thigh-${sfx}`, SKIN, T.skin, m([[0.095, 0.001, -0.001, 0.047, 0.052], [0.165, 0.004, 0, 0.042, 0.047], [0.235, 0.004, 0, 0.036, 0.041], [0.2565, 0.005, 0, 0.0355, 0.042], [0.284, 0.001, 0, 0.021, 0.024]], g), "leg-skin", "body", 2, 1);
    sl(k, `shank-${sfx}`, `shank-${sfx}`, SKIN, T.skin, [[-0.028, 0.002, 0, 0.021, 0.023], [-0.005, 0.006, 0, 0.035, 0.041], [0.04, -0.003, 0, 0.0348, 0.039], [0.085, -0.009, 0, 0.0352, 0.041], [0.16, -0.005, 0, 0.0275, 0.0325], [0.228, -0.002, 0, 0.0215, 0.027], [0.275, -0.001, 0, 0.023, 0.027]], "leg-skin", "body", 2, 1);
    k.loft(`sock-${sfx}`, `shank-${sfx}`, SOCK, T.sock, [[0.222, -0.002, 0, 0.025, 0.0305], [0.272, -0.001, 0, 0.027, 0.0315]], "socks");
    st(k, `shoe-${sfx}`, `foot-${sfx}`, SHOE, T.shoe, m([[-0.053, -0.031, 0, 0.018, 0.026, 3], [-0.038, -0.0265, 0, 0.028, 0.031, 3], [0.02, -0.0275, 0.001, 0.032, 0.0295, 3], [0.055, -0.032, 0.002, 0.036, 0.0245, 3], [0.083, -0.035, 0.002, 0.038, 0.021, 3]], g), "shoes", "body", 3);
    k.loft(`shoe-toe-${sfx}`, `toe-${sfx}`, SHOE, T.shoe, m([[-0.003, 0.0215, 0.002, 0.038, 0.0215, 3], [0.025, 0.0205, 0.001, 0.035, 0.0205, 3], [0.046, 0.0185, -0.001, 0.026, 0.016, 3], [0.057, 0.0165, -0.002, 0.011, 0.0095, 3]], g), "shoes", "body", 3);
    sl(k, `sleeve-${sfx}`, `arm-${sfx}`, SHIRT, T.shirt, m([[-0.03, 0, -0.028, 0.009, 0.017], [-0.012, 0, -0.006, 0.03, 0.034], [0.03, 0.001, 0.003, 0.0362, 0.04], [0.1, 0.001, 0.003, 0.038, 0.042]], g), "tshirt", "body", 2, 1);
    k.loft(`upper-arm-${sfx}`, `arm-${sfx}`, SKIN, T.skin, m([[0.088, 0.002, 0.002, 0.026, 0.03], [0.13, 0.003, 0.002, 0.025, 0.029], [0.1848, -0.002, 0, 0.0235, 0.0235], [0.207, -0.002, 0, 0.013, 0.013]], g), "arm-skin");
    sl(k, `forearm-${sfx}`, `forearm-${sfx}`, SKIN, T.skin, [[-0.021, -0.002, 0, 0.013, 0.013], [-0.004, 0, 0, 0.023, 0.0225], [0.035, 0.001, 0, 0.0255, 0.0255], [0.1, 0, 0, 0.0195, 0.0215], [0.156, 0, 0, 0.013, 0.018]], "arm-skin", "body", 2, 1);
    tube(k, `hand-${sfx}`, `hand-${sfx}`, SKIN, T.skin, m([[-0.008, 0, 0, 0.012, 0.0175, 2.4], [0.02, 0, -0.001, 0.0115, 0.025, 2.4], [0.06, 0, -0.003, 0.0105, 0.027, 2.4], [0.081, 0, -0.009, 0.0098, 0.025, 2.4], [0.088, 0, -0.022, 0.0092, 0.022, 2.4], [0.081, 0, -0.035, 0.0085, 0.019, 2.4]], g), "arm-skin");
  }
}

/* ------------------------------------------------------------------ 1:200 */

/** 🧍 1:200 (30 mm): head and neck one form, hair cap, shirt with the shoulder caps on the sleeves, shorts, pack, and per side shorts leg, thigh, shank, sock cuff, shoe, toe cap, sleeve, upper arm, forearm and mitten hand; the straps keep their true size as slender bands. */
function body200(k: Kit): void {
  sl(k, "shirt", "torso", SHIRT, T.shirt, [[-0.062, 0.008, 0, 0.117, 0.095, 2.3], [0.06, -0.005, 0, 0.106, 0.087, 2.2], [0.2, -0.002, 0, 0.111, 0.078, 2.3], [0.25, 0.004, 0, 0.11, 0.06, 2.2], [0.297, 0.006, 0, 0.05, 0.046, 2]], "tshirt", "body", 2.2, 1);
  sl(k, "head", "head", SKIN, T.skin, [[-0.09, 0.012, 0, 0.047, 0.045], [-0.05, -0.006, 0, 0.043, 0.047], [-0.025, -0.036, 0, 0.044, 0.044, 2.2], [0.005, -0.016, 0, 0.058, 0.07], [0.05, 0, 0, 0.067, 0.087], [0.1, 0.004, 0, 0.058, 0.075], [0.1335, 0.006, 0, 0.016, 0.021]], "face-skin", "body", 2, 1);
  k.loft("hair", "head", HAIR, T.hair, [[-0.008, 0.05, 0, 0.036, 0.02], [0.03, 0.033, 0, 0.065, 0.052], [0.08, 0.005, 0, 0.074, 0.091], [0.118, 0.006, 0, 0.052, 0.065], [0.141, 0.006, 0, 0.018, 0.023]], "hair");
  sl(k, "shorts", "pelvis", SHORTS, T.shorts, [[0.078, 0.001, 0, 0.092, 0.068, 2.3], [0.02, 0.01, 0, 0.102, 0.079, 2.4], [-0.03, 0.019, 0, 0.104, 0.085, 2.4], [-0.064, 0.012, 0, 0.065, 0.06, 2.3]], "shorts", "body", 2.3, 1);
  k.loft("pack", "pack", PACK, T.pack, [[-0.085, 0.078, 0, 0.088, 0.038, 4], [-0.05, 0.081, 0, 0.099, 0.046, 4.5], [0.12, 0.08, 0, 0.097, 0.044, 4.5], [0.163, 0.072, 0, 0.07, 0.03, 3]], "backpack");
  for (const [sfx, g] of SIDES) {
    k.loft(`shorts-leg-${sfx}`, `thigh-${sfx}`, SHORTS, T.shorts, m([[-0.044, -0.004, -0.005, 0.03, 0.038], [-0.01, -0.003, -0.004, 0.052, 0.063], [0.115, 0.002, 0, 0.057, 0.065]], g), "shorts", "body", 2.3);
    k.loft(`thigh-${sfx}`, `thigh-${sfx}`, SKIN, T.skin, m([[0.095, 0.001, -0.001, 0.047, 0.052], [0.2, 0.004, 0, 0.039, 0.044], [0.2565, 0.005, 0, 0.0355, 0.042], [0.283, 0.001, 0, 0.022, 0.025]], g), "leg-skin");
    k.loft(`shank-${sfx}`, `shank-${sfx}`, SKIN, T.skin, [[-0.027, 0.002, 0, 0.022, 0.024], [0, 0.005, 0, 0.0355, 0.041], [0.085, -0.009, 0, 0.0352, 0.041], [0.21, -0.003, 0, 0.0225, 0.028], [0.274, -0.001, 0, 0.0255, 0.03]], "leg-skin");
    k.loft(`sock-${sfx}`, `shank-${sfx}`, SOCK, T.sock, [[0.228, -0.002, 0, 0.026, 0.031], [0.272, -0.001, 0, 0.0275, 0.032]], "socks");
    k.turned(`shoe-${sfx}`, `foot-${sfx}`, SHOE, T.shoe, FOOT, m([[-0.053, -0.03, 0, 0.02, 0.027], [-0.03, -0.0265, 0, 0.03, 0.031], [0.035, -0.029, 0.001, 0.034, 0.028], [0.083, -0.035, 0.002, 0.038, 0.021]], g), "shoes", "body", 3);
    k.loft(`shoe-toe-${sfx}`, `toe-${sfx}`, SHOE, T.shoe, m([[-0.003, 0.0215, 0.002, 0.038, 0.0215], [0.035, 0.02, 0, 0.032, 0.0185], [0.057, 0.0165, -0.002, 0.015, 0.0155]], g), "shoes", "body", 3);
    sl(k, `sleeve-${sfx}`, `arm-${sfx}`, SHIRT, T.shirt, m([[-0.03, 0, -0.026, 0.015, 0.022], [-0.008, 0, -0.004, 0.033, 0.036], [0.1, 0.001, 0.003, 0.038, 0.042]], g), "tshirt", "body", 2, 1);
    k.loft(`upper-arm-${sfx}`, `arm-${sfx}`, SKIN, T.skin, m([[0.088, 0.002, 0.002, 0.026, 0.03], [0.1848, -0.002, 0, 0.0235, 0.024], [0.206, -0.002, 0, 0.0155, 0.0155]], g), "arm-skin");
    k.loft(`forearm-${sfx}`, `forearm-${sfx}`, SKIN, T.skin, [[-0.02, -0.001, 0, 0.0155, 0.0155], [0.02, 0.001, 0, 0.0255, 0.025], [0.155, 0, 0, 0.0155, 0.019]], "arm-skin");
    tube(k, `hand-${sfx}`, `hand-${sfx}`, SKIN, T.skin, m([[-0.006, 0, 0, 0.0155, 0.018, 2.6], [0.05, 0, -0.002, 0.0155, 0.029, 2.6], [0.083, 0, -0.012, 0.0155, 0.025, 2.6], [0.086, 0, -0.03, 0.0155, 0.02, 2.6]], g), "arm-skin");
  }
}

/* ------------------------------------------------------------------ 1:500 */

/** 🧱 1:500 (75 mm; limbs as thin as their own 2D drawing): head with neck under a blunt hair dome, shirt, shorts, pack, and per side shorts leg, thigh, shank, sock cuff, shoe, toe cap, sleeve with shoulder cap, upper arm and forearm with the hand. */
function body500(k: Kit): void {
  k.loft("shirt", "torso", SHIRT, T.shirt, [[-0.035, 0.006, 0, 0.115, 0.093, 2.3], [0.2, -0.002, 0, 0.11, 0.079, 2.3], [0.252, 0.004, 0, 0.11, 0.06, 2.2], [0.297, 0.006, 0, 0.054, 0.047, 2]], "tshirt");
  k.loft("head", "head", SKIN, T.skin, [[-0.09, 0.012, 0, 0.047, 0.045], [-0.04, -0.02, 0, 0.044, 0.053], [0.01, -0.012, 0, 0.058, 0.074], [0.07, 0.002, 0, 0.067, 0.087], [0.13, 0.006, 0, 0.03, 0.036]], "face-skin");
  k.loft("hair", "head", HAIR, T.hair, [[0, 0.046, 0, 0.044, 0.028], [0.075, 0.006, 0, 0.074, 0.09], [0.125, 0.006, 0, 0.05, 0.06], [0.141, 0.006, 0, 0.03, 0.036]], "hair");
  k.loft("shorts", "pelvis", SHORTS, T.shorts, [[-0.09, 0.012, 0, 0.078, 0.066, 2.3], [-0.01, 0.019, 0, 0.103, 0.085, 2.4], [0.078, 0.001, 0, 0.092, 0.069, 2.3]], "shorts");
  k.loft("pack", "pack", PACK, T.pack, [[-0.085, 0.079, 0, 0.092, 0.042, 4], [0.12, 0.08, 0, 0.097, 0.044, 4], [0.163, 0.073, 0, 0.074, 0.034, 3]], "backpack");
  for (const [sfx, g] of SIDES) {
    k.loft(`shorts-leg-${sfx}`, `thigh-${sfx}`, SHORTS, T.shorts, m([[-0.042, -0.003, -0.005, 0.036, 0.044], [-0.008, -0.003, -0.004, 0.052, 0.062], [0.115, 0.002, -0.001, 0.054, 0.062]], g), "shorts", "body", 2.3);
    k.loft(`thigh-${sfx}`, `thigh-${sfx}`, SKIN, T.skin, m([[0.1, 0.002, -0.001, 0.049, 0.055], [0.2565, 0.005, 0, 0.036, 0.042], [0.282, 0.001, 0, 0.025, 0.028]], g), "leg-skin");
    k.loft(`shank-${sfx}`, `shank-${sfx}`, SKIN, T.skin, [[-0.025, 0.003, 0, 0.026, 0.029], [0.07, -0.007, 0, 0.0355, 0.041], [0.22, -0.002, 0, 0.023, 0.028], [0.273, -0.001, 0, 0.026, 0.03]], "leg-skin");
    k.loft(`sock-${sfx}`, `shank-${sfx}`, SOCK, T.sock, [[0.198, -0.003, 0, 0.0265, 0.0315], [0.273, -0.001, 0, 0.028, 0.032]], "socks");
    k.turned(`shoe-${sfx}`, `foot-${sfx}`, SHOE, T.shoe, FOOT, m([[-0.053, -0.028, 0, 0.022, 0.028], [0, -0.0242, 0, 0.032, 0.0318], [0.083, -0.0355, 0.002, 0.038, 0.0205]], g), "shoes", "body", 3);
    k.loft(`shoe-toe-${sfx}`, `toe-${sfx}`, SHOE, T.shoe, m([[-0.002, 0.0205, 0.002, 0.038, 0.0205], [0.057, 0.0165, -0.001, 0.02, 0.0165]], g), "shoes", "body", 3);
    k.loft(`sleeve-${sfx}`, `arm-${sfx}`, SHIRT, T.shirt, m([[-0.028, 0, -0.016, 0.022, 0.028], [0.02, 0.001, 0.002, 0.036, 0.04], [0.1, 0.001, 0.002, 0.037, 0.041]], g), "tshirt");
    k.loft(`upper-arm-${sfx}`, `arm-${sfx}`, SKIN, T.skin, m([[0.09, 0.001, 0.001, 0.029, 0.032], [0.1848, -0.001, 0, 0.024, 0.025], [0.205, -0.001, 0, 0.017, 0.017]], g), "arm-skin");
    k.loft(`forearm-${sfx}`, `forearm-${sfx}`, SKIN, T.skin, m([[-0.019, 0, 0, 0.017, 0.017], [0.02, 0.001, 0, 0.0255, 0.025], [0.15, 0.001, 0, 0.015, 0.02], [0.2, 0.004, -0.002, 0.014, 0.029], [0.25, 0.001, -0.014, 0.014, 0.022]], g), "arm-skin");
  }
}

export function bodyCoarse(k: Kit, scale: number): void {
  ({ 100: body100, 200: body200, 500: body500 })[scale as 100 | 200 | 500](k);
}
