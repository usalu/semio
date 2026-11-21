/** 🏃‍♂️ The round 5 body of `humans/jogger` at 1:50, 1:100, 1:200 and 1:500, every key section stated by hand from the reference table of `r3-humans-jogger.md` (round 5 section): a lean 1.82 m (shod) runner of 72 kg in a loose short-sleeved running shirt, running shorts, ankle socks and running shoes. The lofts run through a smooth spline of the stated key sections, so every limb is one continuous anatomical form. Trunk bones (pelvis, torso, head): u up, v to the back, w to the left. Limb bones: u distal, v to the front, w to the left. Foot: u ahead, v up from the ankle (ground at v = -0.0827). Toe: u ahead from the ball, v up from the ground.
 * @see ./r5-runners-kit.ts
 * @see ./r5-jogger-figure.ts */
import { sloft, tube, type Kit, type Row, type V3 } from "./r5-runners-kit.ts";

const SKIN = "skin";
const SHIRT = "nylon-clothing";
const SOCK = "cotton-clothing";
const SHOE = "leather-shoe";
const HAIR = "hair";
const T = { skin: 0.72, shirt: 0.88, shorts: 0.22, hair: 0.15, band: 0.9, shoe: 0.92, sole: 0.55, sock: 0.95, arm: 0.25, eye: 0.1 };
const SIDES = [["l", 1], ["r", -1]] as const;
const m = (rows: readonly Row[], g: number): Row[] => rows.map(([s, cv, cw, rx, ry, e]) => [s, cv, cw * g, rx, ry, e] as Row);

/** 👕 Shirt over the trunk of a lean man: hem over the shorts, waist 0.30 x 0.21 m, ribcage flaring to the chest 0.34 x 0.25 m, the shoulder girdle 0.36 m wide at the armpits and the trapezius slope rising at 24 degrees from the acromion to the collar. */
const SHIRT50: Row[] = [[-0.062, 0.006, 0, 0.18, 0.134, 2.3], [-0.03, 0.004, 0, 0.169, 0.123, 2.3], [0.05, 0, 0, 0.151, 0.105, 2.2], [0.15, -0.005, 0, 0.155, 0.111, 2.2], [0.24, -0.009, 0, 0.164, 0.123, 2.2], [0.31, -0.008, 0, 0.168, 0.125, 2.2], [0.37, -0.003, 0, 0.172, 0.118, 2.3], [0.42, 0.004, 0, 0.178, 0.104, 2.4], [0.45, 0.008, 0, 0.174, 0.09, 2.4], [0.475, 0.011, 0, 0.152, 0.078, 2.3], [0.497, 0.012, 0, 0.115, 0.07, 2.1], [0.517, 0.01, 0, 0.078, 0.066, 2], [0.527, 0.009, 0, 0.068, 0.063, 2]];
/** 🩳 Running shorts on the pelvis: waistband at 1.10 m, hips 0.35 m over the trochanters, the gluteal mass 0.25 m deep behind, crotch at 0.86 m. */
const SHORTS50: Row[] = [[0.135, 0.0, 0, 0.152, 0.104, 2.3], [0.09, 0.004, 0, 0.157, 0.108, 2.3], [0.04, 0.01, 0, 0.167, 0.115, 2.4], [-0.01, 0.018, 0, 0.175, 0.123, 2.4], [-0.05, 0.022, 0, 0.171, 0.126, 2.4], [-0.085, 0.02, 0, 0.15, 0.111, 2.4], [-0.105, 0.014, 0, 0.11, 0.086, 2.3], [-0.116, 0.01, 0, 0.06, 0.05, 2.2]];
/** 🗿 Head from the chin to the vertex (head length 0.196 m, breadth 0.155 m), jaw, chin, cheeks and the occiput. */
const HEAD50: Row[] = [[0.008, -0.076, 0, 0.026, 0.016, 2.2], [0.028, -0.06, 0, 0.05, 0.04, 2.2], [0.055, -0.04, 0, 0.061, 0.062, 2.2], [0.085, -0.018, 0, 0.068, 0.085], [0.125, -0.008, 0, 0.074, 0.095], [0.165, -0.006, 0, 0.0775, 0.098], [0.2, -0.002, 0, 0.072, 0.09], [0.228, 0, 0, 0.052, 0.066], [0.2449, 0, 0, 0.018, 0.024]];
const HAIR50: Row[] = [[0.088, 0.044, 0, 0.058, 0.034], [0.125, 0.032, 0, 0.079, 0.062], [0.165, 0.012, 0, 0.082, 0.088], [0.195, 0.001, 0, 0.08, 0.097], [0.225, 0, 0, 0.063, 0.078], [0.243, 0, 0, 0.042, 0.052], [0.2529, 0, 0, 0.02, 0.026]];
const NECK50: Row[] = [[-0.035, 0.014, 0, 0.068, 0.066], [0.0, 0.009, 0, 0.061, 0.062], [0.04, 0.002, 0, 0.057, 0.06], [0.085, -0.006, 0, 0.056, 0.062]];

/** 💪 Deltoid cap riding on the upper arm, from the acromion over the humeral head to the sleeve seam; bideltoid breadth 0.486 m with the shirt. */
const SHOULDER50: Row[] = [[-0.042, 0.0, -0.05, 0.012, 0.024], [-0.034, 0.0, -0.032, 0.03, 0.04], [-0.02, 0.0, -0.012, 0.047, 0.052], [0.004, 0.001, 0.003, 0.056, 0.058], [0.035, 0.002, 0.007, 0.059, 0.061], [0.072, 0.003, 0.006, 0.0575, 0.0605]];
const SLEEVE50: Row[] = [[0.058, 0.003, 0.007, 0.0595, 0.0625], [0.12, 0.003, 0.004, 0.056, 0.059], [0.17, 0.004, 0.003, 0.058, 0.061]];
/** 💪 Upper arm (girth 0.30 m): biceps belly to the front, triceps behind, narrowing to the epicondyles at the elbow (0.336 m). */
const UPPER50: Row[] = [[0.14, 0.004, 0.003, 0.046, 0.052], [0.19, 0.006, 0.002, 0.045, 0.051], [0.25, 0.004, 0, 0.041, 0.046], [0.3, 0.0, 0, 0.038, 0.04], [0.336, -0.004, 0, 0.04, 0.038], [0.358, -0.004, 0, 0.033, 0.031], [0.371, -0.004, 0, 0.02, 0.02]];
/** 🦾 Forearm (girth 0.28 m below the elbow, wrist 0.058 x 0.041 m, thumb side up): flexor and extensor mass near the elbow, flat towards the wrist. */
const FOREARM50: Row[] = [[-0.036, -0.006, 0, 0.022, 0.022], [-0.016, -0.004, 0, 0.038, 0.037], [0.04, 0.002, 0, 0.044, 0.044], [0.1, 0.002, 0, 0.038, 0.04], [0.17, 0.001, 0, 0.029, 0.034], [0.24, 0, 0, 0.0225, 0.031], [0.2792, 0, 0, 0.022, 0.031], [0.293, 0, 0, 0.0205, 0.029]];
/** ✋ Palm of the loosely closed running hand (breadth 0.086 m, thickness 0.028 m), the thenar to the thumb side. */
const PALM50: Row[] = [[-0.012, 0.0, 0, 0.019, 0.026], [0.0, 0.001, 0, 0.0195, 0.03], [0.032, 0.003, -0.002, 0.017, 0.037], [0.075, 0.001, -0.004, 0.0145, 0.041], [0.102, -0.001, -0.005, 0.0135, 0.04], [0.112, -0.002, -0.007, 0.011, 0.037]];
/** 🤜 The four fingers grouped and curled into the palm (path in the u-w plane; rx finger thickness, ry the breadth of the four fingers). */
const FINGERS50: Row[] = [[0.098, -0.004, -0.005, 0.0125, 0.039, 2.4], [0.126, -0.004, -0.011, 0.012, 0.038, 2.4], [0.141, -0.004, -0.028, 0.0115, 0.036, 2.4], [0.138, -0.004, -0.048, 0.0105, 0.033, 2.4], [0.124, -0.004, -0.06, 0.0095, 0.029, 2.4]];
const THUMB50: Row[] = [[0.012, 0.02, -0.008, 0.016, 0.015], [0.048, 0.038, -0.014, 0.014, 0.0125], [0.08, 0.046, -0.024, 0.011, 0.0105], [0.105, 0.044, -0.034, 0.0095, 0.009], [0.114, 0.04, -0.039, 0.007, 0.007]];

/** 🦵 Thigh (girth 0.54 / 0.51 / 0.37 m at the top, middle and above the knee): quadriceps to the front, the teardrop of vastus medialis low and inside, knee 0.105 m wide. */
const THIGH50: Row[] = [[0.13, 0.01, -0.002, 0.08, 0.088], [0.2, 0.012, -0.001, 0.077, 0.085], [0.27, 0.012, 0, 0.071, 0.078], [0.34, 0.01, -0.002, 0.063, 0.068], [0.39, 0.008, -0.004, 0.058, 0.06], [0.43, 0.006, -0.002, 0.054, 0.056], [0.46, 0.002, 0, 0.05, 0.052], [0.486, 0.0, 0, 0.033, 0.035]];
/** 🦵 Shank: tibial plateau, gastrocnemius belly behind (girth 0.37 m, medial head lower), Achilles taper, malleoli (girth 0.22 m). */
const SHANK50: Row[] = [[-0.046, 0.004, 0, 0.032, 0.034], [-0.02, 0.006, 0, 0.05, 0.054], [0.01, 0.004, 0, 0.052, 0.055], [0.06, -0.008, -0.002, 0.053, 0.059], [0.12, -0.018, -0.003, 0.055, 0.065], [0.19, -0.015, -0.002, 0.048, 0.056], [0.26, -0.006, 0, 0.039, 0.043], [0.32, -0.002, 0, 0.032, 0.036], [0.38, -0.002, 0, 0.032, 0.037], [0.43, -0.001, 0, 0.034, 0.04], [0.456, -0.001, 0, 0.03, 0.036]];
const SOCK50: Row[] = [[0.385, -0.002, 0, 0.0345, 0.0405], [0.41, -0.002, 0, 0.035, 0.042], [0.4343, -0.001, 0, 0.0375, 0.0435], [0.452, -0.001, 0, 0.036, 0.042]];
/** 👟 Midsole of an EU 44 running shoe: heel stack 31 mm with a flared, rounded heel, narrow waist under the arch, 106 mm at the ball. */
const SOLE50: Row[] = [[-0.079, -0.067, 0, 0.03, 0.012, 4], [-0.07, -0.0672, 0, 0.04, 0.0155, 4], [-0.03, -0.0675, 0, 0.042, 0.015, 4], [0.03, -0.0695, 0.002, 0.039, 0.013, 4], [0.09, -0.0712, 0.002, 0.05, 0.0115, 4], [0.1344, -0.0715, 0.001, 0.053, 0.011, 4], [0.147, -0.0715, 0.001, 0.052, 0.011, 4]];
/** 👟 Upper from the heel counter and the padded collar over the tongue and instep to the forefoot. */
const UPPER_SHOE50: Row[] = [[-0.074, -0.03, 0, 0.026, 0.022, 2.4], [-0.062, -0.026, 0, 0.036, 0.03, 2.4], [-0.03, -0.029, 0, 0.04, 0.031, 2.4], [0.01, -0.027, 0.001, 0.041, 0.033, 2.4], [0.05, -0.03, 0.002, 0.044, 0.031, 2.4], [0.09, -0.037, 0.002, 0.048, 0.027, 2.4], [0.1344, -0.044, 0.001, 0.052, 0.022, 2.4], [0.147, -0.046, 0.001, 0.051, 0.0215, 2.4]];
const SOLE_TOE50: Row[] = [[-0.004, 0.0115, 0.001, 0.053, 0.0115, 4], [0.04, 0.0125, -0.002, 0.05, 0.0105, 4], [0.07, 0.016, -0.006, 0.04, 0.0095, 4], [0.088, 0.021, -0.01, 0.026, 0.008, 4], [0.097, 0.026, -0.012, 0.012, 0.006, 4]];
const UPPER_TOE50: Row[] = [[-0.004, 0.037, 0.001, 0.051, 0.0225, 2.4], [0.035, 0.035, -0.002, 0.048, 0.0205, 2.4], [0.065, 0.033, -0.006, 0.04, 0.017, 2.4], [0.085, 0.032, -0.01, 0.027, 0.013, 2.4], [0.096, 0.031, -0.012, 0.013, 0.009, 2.4]];

/* ------------------------------------------------------------------ 1:50 */

export function body50(k: Kit): void {
  sloft(k, "shirt", "torso", SHIRT, T.shirt, SHIRT50, "torso-clothing", "body", 2.2, 2);
  k.loft("shirt-collar", "torso", SHIRT, 0.6, [[0.515, 0.01, 0, 0.082, 0.068], [0.531, 0.009, 0, 0.07, 0.065]], "torso-clothing", "detail");
  sloft(k, "neck", "head", SKIN, T.skin, NECK50, "head-skin", "body", 2, 2);
  sloft(k, "head", "head", SKIN, T.skin, HEAD50, "head-skin", "body", 2, 2);
  tube(k, "nose", "head", SKIN, T.skin, [[0.152, -0.091, 0, 0.006, 0.007], [0.128, -0.105, 0, 0.008, 0.0095], [0.107, -0.116, 0, 0.0095, 0.012], [0.095, -0.112, 0, 0.007, 0.013]], "head-skin");
  for (const [sfx, g] of SIDES) {
    k.ell(`ear-${sfx}`, "head", SKIN, T.skin, [0.11, 0.004, 0.076 * g], [0.028, 0.016, 0.0075], "head-skin", "detail", [0.1305 * g, 0, 0, 0.9914]);
    k.ell(`eye-${sfx}`, "head", SKIN, T.eye, [0.128, -0.0925, 0.031 * g], [0.005, 0.004, 0.011], "head-skin", "detail");
  }
  sloft(k, "hair", "head", HAIR, T.hair, HAIR50, "hair", "body", 2, 2);
  k.loft("headband", "head", SHIRT, T.band, [[0.172, -0.001, 0, 0.0855, 0.1035], [0.198, 0.001, 0, 0.0835, 0.1005]], "hair", "detail");
  sloft(k, "shorts", "pelvis", SHIRT, T.shorts, SHORTS50, "leg-clothing", "body", 2.4, 2);
  for (const [sfx, g] of SIDES) {
    sloft(k, `shorts-leg-${sfx}`, `thigh-${sfx}`, SHIRT, T.shorts, m([[-0.04, 0.006, -0.006, 0.076, 0.09], [0.02, 0.007, -0.003, 0.085, 0.097], [0.1, 0.008, 0.0, 0.086, 0.096], [0.17, 0.01, 0.003, 0.087, 0.096]], g), "leg-clothing", "body", 2.3, 1);
    sloft(k, `thigh-${sfx}`, `thigh-${sfx}`, SKIN, T.skin, m(THIGH50, g), "leg-skin");
    k.ell(`patella-${sfx}`, `shank-${sfx}`, SKIN, T.skin, [-0.008, 0.044, 0], [0.028, 0.017, 0.026], "leg-skin");
    sloft(k, `shank-${sfx}`, `shank-${sfx}`, SKIN, T.skin, m(SHANK50, g), "leg-skin");
    sloft(k, `sock-${sfx}`, `shank-${sfx}`, SOCK, T.sock, SOCK50, "leg-skin", "body", 2, 1);
    sloft(k, `sole-${sfx}`, `foot-${sfx}`, SHOE, T.sole, m(SOLE50, g), "shoes", "body", 4, 1);
    sloft(k, `shoe-${sfx}`, `foot-${sfx}`, SHOE, T.shoe, m(UPPER_SHOE50, g), "shoes", "body", 2.4, 1);
    sloft(k, `sole-toe-${sfx}`, `toe-${sfx}`, SHOE, T.sole, m(SOLE_TOE50, g), "shoes", "body", 4, 1);
    sloft(k, `shoe-toe-${sfx}`, `toe-${sfx}`, SHOE, T.shoe, m(UPPER_TOE50, g), "shoes", "body", 2.4, 1);
    sloft(k, `shoulder-${sfx}`, `upper-arm-${sfx}`, SHIRT, T.shirt, m(SHOULDER50, g), "torso-clothing", "body", 2, 2);
    sloft(k, `sleeve-${sfx}`, `upper-arm-${sfx}`, SHIRT, T.shirt, m(SLEEVE50, g), "arm-clothing", "body", 2, 1);
    sloft(k, `upper-arm-${sfx}`, `upper-arm-${sfx}`, SKIN, T.skin, m(UPPER50, g), "arm-skin");
    sloft(k, `forearm-${sfx}`, `forearm-${sfx}`, SKIN, T.skin, FOREARM50, "arm-skin");
    sloft(k, `hand-${sfx}`, `hand-${sfx}`, SKIN, T.skin, m(PALM50, g), "hand-skin", "body", 2.6, 1);
    tube(k, `finger-${sfx}`, `hand-${sfx}`, SKIN, T.skin, m(FINGERS50, g), "hand-skin");
    tube(k, `thumb-${sfx}`, `hand-${sfx}`, SKIN, T.skin, m(THUMB50, g), "hand-skin");
  }
  k.loft("armband", "upper-arm-l", SHIRT, T.arm, [[0.2, 0.004, 0.002, 0.0485, 0.0545], [0.262, 0.003, 0.001, 0.0455, 0.0505]], "arm-clothing");
  k.box("armband-pouch", "upper-arm-l", SHIRT, T.arm, [0.231, 0.002, 0.05], [0.125, 0.068, 0.014], 0.005, "arm-clothing");
}

/* ------------------------------------------------------------------ 1:100 */

/** 🧤 1:100: the same runner drawn anew with fewer, longer key sections (smallest feature 15 mm). Nose, ears, eyes, knee caps and the phone pouch fall below the feature and are left out, the knee cap is part of the shank front; shoulder cap and sleeve are one garment; midsole and upper are one shoe form; palm and curled fingers are one mitten swept along the curl, the thumb stays its own mass. */
export function body100(k: Kit): void {
  sloft(k, "shirt", "torso", SHIRT, T.shirt, [[-0.062, 0.006, 0, 0.18, 0.134, 2.3], [0.05, 0, 0, 0.151, 0.105, 2.2], [0.24, -0.009, 0, 0.164, 0.123, 2.2], [0.37, -0.003, 0, 0.172, 0.118, 2.3], [0.45, 0.008, 0, 0.174, 0.09, 2.4], [0.497, 0.012, 0, 0.115, 0.07, 2.1], [0.527, 0.009, 0, 0.068, 0.063, 2]], "torso-clothing", "body", 2.2, 2);
  sloft(k, "neck", "head", SKIN, T.skin, [[-0.035, 0.014, 0, 0.068, 0.066], [0.04, 0.002, 0, 0.057, 0.06], [0.085, -0.006, 0, 0.056, 0.062]], "head-skin", "body", 2, 1);
  sloft(k, "head", "head", SKIN, T.skin, [[0.008, -0.074, 0, 0.03, 0.018, 2.2], [0.04, -0.05, 0, 0.058, 0.054, 2.2], [0.085, -0.018, 0, 0.068, 0.085], [0.165, -0.006, 0, 0.0775, 0.098], [0.228, 0, 0, 0.052, 0.066], [0.2449, 0, 0, 0.02, 0.026]], "head-skin", "body", 2, 2);
  sloft(k, "hair", "head", HAIR, T.hair, [[0.09, 0.044, 0, 0.058, 0.036], [0.165, 0.012, 0, 0.082, 0.088], [0.225, 0, 0, 0.063, 0.078], [0.2529, 0, 0, 0.022, 0.03]], "hair", "body", 2, 2);
  k.loft("headband", "head", SHIRT, T.band, [[0.17, -0.001, 0, 0.086, 0.104], [0.2, 0.001, 0, 0.0835, 0.1005]], "hair", "detail");
  sloft(k, "shorts", "pelvis", SHIRT, T.shorts, [[0.135, 0.0, 0, 0.152, 0.104, 2.3], [0.04, 0.01, 0, 0.167, 0.115, 2.4], [-0.05, 0.022, 0, 0.171, 0.126, 2.4], [-0.105, 0.014, 0, 0.11, 0.086, 2.3], [-0.116, 0.01, 0, 0.06, 0.05, 2.2]], "leg-clothing", "body", 2.4, 2);
  for (const [sfx, g] of SIDES) {
    k.loft(`shorts-leg-${sfx}`, `thigh-${sfx}`, SHIRT, T.shorts, m([[-0.04, 0.006, -0.006, 0.076, 0.09], [0.08, 0.008, 0, 0.086, 0.096], [0.17, 0.01, 0.003, 0.087, 0.096]], g), "leg-clothing", "body", 2.3);
    sloft(k, `thigh-${sfx}`, `thigh-${sfx}`, SKIN, T.skin, m([[0.13, 0.01, -0.002, 0.08, 0.088], [0.27, 0.012, 0, 0.071, 0.078], [0.39, 0.008, -0.004, 0.058, 0.06], [0.46, 0.002, 0, 0.05, 0.052], [0.486, 0, 0, 0.033, 0.035]], g), "leg-skin", "body", 2, 2);
    sloft(k, `shank-${sfx}`, `shank-${sfx}`, SKIN, T.skin, m([[-0.046, 0.004, 0, 0.032, 0.034], [-0.01, 0.01, 0, 0.052, 0.058], [0.12, -0.018, -0.003, 0.055, 0.065], [0.26, -0.006, 0, 0.039, 0.043], [0.38, -0.002, 0, 0.032, 0.037], [0.43, -0.001, 0, 0.034, 0.04], [0.456, -0.001, 0, 0.03, 0.036]], g), "leg-skin", "body", 2, 2);
    k.loft(`sock-${sfx}`, `shank-${sfx}`, SOCK, T.sock, [[0.385, -0.002, 0, 0.0345, 0.0405], [0.4343, -0.001, 0, 0.0375, 0.0435], [0.452, -0.001, 0, 0.036, 0.042]], "leg-skin");
    sloft(k, `shoe-${sfx}`, `foot-${sfx}`, SHOE, T.shoe, m([[-0.08, -0.046, 0, 0.03, 0.034, 3], [-0.065, -0.04, 0, 0.039, 0.043, 3], [-0.02, -0.04, 0, 0.041, 0.043, 3], [0.03, -0.043, 0.002, 0.041, 0.04, 3], [0.09, -0.052, 0.002, 0.049, 0.031, 3], [0.1344, -0.058, 0.001, 0.053, 0.025, 3], [0.147, -0.059, 0.001, 0.052, 0.024, 3]], g), "shoes", "body", 3, 1);
    sloft(k, `shoe-toe-${sfx}`, `toe-${sfx}`, SHOE, T.shoe, m([[-0.004, 0.0245, 0.001, 0.053, 0.0245, 3], [0.04, 0.023, -0.002, 0.05, 0.023, 3], [0.07, 0.024, -0.006, 0.04, 0.019, 3], [0.09, 0.026, -0.01, 0.026, 0.014, 3], [0.097, 0.027, -0.012, 0.014, 0.01, 3]], g), "shoes", "body", 3, 1);
    sloft(k, `sleeve-${sfx}`, `upper-arm-${sfx}`, SHIRT, T.shirt, m(SLEEVE100, g), "arm-clothing", "body", 2, 2);
    sloft(k, `upper-arm-${sfx}`, `upper-arm-${sfx}`, SKIN, T.skin, m([[0.14, 0.004, 0.003, 0.046, 0.052], [0.25, 0.004, 0, 0.041, 0.046], [0.336, -0.004, 0, 0.04, 0.038], [0.371, -0.004, 0, 0.02, 0.02]], g), "arm-skin", "body", 2, 2);
    sloft(k, `forearm-${sfx}`, `forearm-${sfx}`, SKIN, T.skin, [[-0.036, -0.006, 0, 0.022, 0.022], [0.04, 0.002, 0, 0.044, 0.044], [0.17, 0.001, 0, 0.03, 0.035], [0.2792, 0, 0, 0.022, 0.031], [0.293, 0, 0, 0.0205, 0.029]], "arm-skin", "body", 2, 2);
    tube(k, `hand-${sfx}`, `hand-${sfx}`, SKIN, T.skin, m([[-0.012, 0, 0, 0.019, 0.026, 2.4], [0.03, 0, -0.002, 0.017, 0.037, 2.4], [0.095, 0, -0.005, 0.016, 0.04, 2.4], [0.128, 0, -0.012, 0.015, 0.038, 2.4], [0.142, 0, -0.03, 0.0135, 0.034, 2.4], [0.134, 0, -0.052, 0.012, 0.03, 2.4]], g), "hand-skin");
    tube(k, `thumb-${sfx}`, `hand-${sfx}`, SKIN, T.skin, m([[0.012, 0.02, -0.008, 0.016, 0.015], [0.062, 0.042, -0.018, 0.0125, 0.0115], [0.108, 0.044, -0.035, 0.0095, 0.0095]], g), "hand-skin");
  }
  k.loft("armband", "upper-arm-l", SHIRT, T.arm, [[0.19, 0.004, 0.003, 0.0505, 0.0555], [0.272, 0.002, 0.002, 0.0465, 0.0505]], "arm-clothing");
}
/** 👕 Shoulder cap and sleeve of 1:100 as one garment; the thermal model splits it at the sleeve seam (u 0.066). */
export const SLEEVE100: Row[] = [[-0.042, 0.0, -0.05, 0.012, 0.024], [-0.02, 0.0, -0.012, 0.047, 0.052], [0.035, 0.002, 0.007, 0.059, 0.061], [0.1, 0.003, 0.005, 0.057, 0.06], [0.17, 0.004, 0.003, 0.058, 0.061]];

/* ------------------------------------------------------------------ 1:200 */

/** 🧍 1:200: masses of at least 30 mm. Head and neck are one form under the hair cap, the head-band and armband are gone, the sock is part of the shank, hand and thumb are one mitten; shirt, shorts, sleeves and shoes keep their own forms because their tones carry the figure. */
export function body200(k: Kit): void {
  sloft(k, "shirt", "torso", SHIRT, T.shirt, [[-0.062, 0.006, 0, 0.18, 0.134, 2.3], [0.06, 0, 0, 0.152, 0.106, 2.2], [0.3, -0.008, 0, 0.168, 0.124, 2.2], [0.44, 0.007, 0, 0.176, 0.096, 2.4], [0.5, 0.012, 0, 0.11, 0.07, 2.1], [0.527, 0.009, 0, 0.07, 0.064, 2]], "torso-clothing", "body", 2.2, 1);
  sloft(k, "head", "head", SKIN, T.skin, [[-0.035, 0.014, 0, 0.068, 0.066], [0.02, 0.004, 0, 0.058, 0.062], [0.05, -0.034, 0, 0.06, 0.07], [0.11, -0.012, 0, 0.073, 0.093], [0.17, -0.006, 0, 0.0775, 0.097], [0.225, 0, 0, 0.056, 0.07], [0.2449, 0, 0, 0.024, 0.03]], "head-skin", "body", 2, 1);
  sloft(k, "hair", "head", HAIR, T.hair, [[0.095, 0.042, 0, 0.062, 0.04], [0.16, 0.014, 0, 0.083, 0.086], [0.215, 0.001, 0, 0.073, 0.088], [0.2529, 0, 0, 0.026, 0.032]], "hair", "body", 2, 1);
  sloft(k, "shorts", "pelvis", SHIRT, T.shorts, [[0.135, 0.0, 0, 0.152, 0.104, 2.3], [0.03, 0.012, 0, 0.169, 0.117, 2.4], [-0.06, 0.022, 0, 0.168, 0.124, 2.4], [-0.116, 0.012, 0, 0.08, 0.064, 2.3]], "leg-clothing", "body", 2.4, 1);
  for (const [sfx, g] of SIDES) {
    k.loft(`shorts-leg-${sfx}`, `thigh-${sfx}`, SHIRT, T.shorts, m([[-0.04, 0.006, -0.006, 0.076, 0.09], [0.17, 0.01, 0.003, 0.087, 0.096]], g), "leg-clothing", "body", 2.3);
    sloft(k, `thigh-${sfx}`, `thigh-${sfx}`, SKIN, T.skin, m([[0.13, 0.01, -0.002, 0.08, 0.088], [0.3, 0.011, -0.001, 0.068, 0.075], [0.44, 0.005, -0.002, 0.053, 0.055], [0.484, 0, 0, 0.035, 0.037]], g), "leg-skin", "body", 2, 1);
    sloft(k, `shank-${sfx}`, `shank-${sfx}`, SKIN, T.skin, m([[-0.044, 0.004, 0, 0.034, 0.036], [0.0, 0.008, 0, 0.053, 0.058], [0.13, -0.017, -0.003, 0.055, 0.064], [0.3, -0.004, 0, 0.036, 0.04], [0.42, -0.001, 0, 0.036, 0.042], [0.455, -0.001, 0, 0.034, 0.04]], g), "leg-skin", "body", 2, 1);
    k.loft(`shoe-${sfx}`, `foot-${sfx}`, SHOE, T.shoe, m([[-0.08, -0.046, 0, 0.032, 0.036, 3], [-0.05, -0.04, 0, 0.041, 0.043, 3], [0.03, -0.043, 0.002, 0.041, 0.04, 3], [0.1344, -0.058, 0.001, 0.053, 0.025, 3], [0.147, -0.059, 0.001, 0.052, 0.024, 3]], g), "shoes", "body", 3);
    k.loft(`shoe-toe-${sfx}`, `toe-${sfx}`, SHOE, T.shoe, m([[-0.004, 0.0245, 0.001, 0.053, 0.0245, 3], [0.06, 0.024, -0.005, 0.045, 0.021, 3], [0.097, 0.026, -0.012, 0.02, 0.016, 3]], g), "shoes", "body", 3);
    sloft(k, `sleeve-${sfx}`, `upper-arm-${sfx}`, SHIRT, T.shirt, m([[-0.042, 0.0, -0.046, 0.016, 0.026], [-0.016, 0.0, -0.01, 0.05, 0.054], [0.05, 0.002, 0.007, 0.059, 0.061], [0.17, 0.004, 0.003, 0.058, 0.061]], g), "arm-clothing", "body", 2, 1);
    k.loft(`upper-arm-${sfx}`, `upper-arm-${sfx}`, SKIN, T.skin, m([[0.14, 0.004, 0.003, 0.046, 0.052], [0.24, 0.004, 0.001, 0.042, 0.047], [0.336, -0.004, 0, 0.04, 0.038], [0.368, -0.004, 0, 0.026, 0.026]], g), "arm-skin");
    k.loft(`forearm-${sfx}`, `forearm-${sfx}`, SKIN, T.skin, [[-0.034, -0.006, 0, 0.026, 0.026], [0.04, 0.002, 0, 0.044, 0.044], [0.17, 0.001, 0, 0.03, 0.034], [0.293, 0, 0, 0.0195, 0.028]], "arm-skin");
    tube(k, `hand-${sfx}`, `hand-${sfx}`, SKIN, T.skin, m([[-0.01, 0, 0, 0.0195, 0.028, 2.4], [0.06, 0, -0.004, 0.0175, 0.044, 2.4], [0.124, 0, -0.012, 0.0165, 0.042, 2.4], [0.14, 0, -0.036, 0.0155, 0.036, 2.4]], g), "hand-skin");
  }
}

/* ------------------------------------------------------------------ 1:500 */

/** 🧱 1:500: the figure in its fewest masses (smallest feature 75 mm; limbs as thin as their own 2D drawing): head with neck under a blunt hair dome, shirt with the shoulder caps on the sleeves, shorts, and per side shorts leg, thigh, shank with sock, shoe, toe cap, sleeve, upper arm, forearm and mitten hand. */
export function body500(k: Kit): void {
  k.loft("shirt", "torso", SHIRT, T.shirt, [[-0.062, 0.006, 0, 0.18, 0.134, 2.3], [0.08, -0.002, 0, 0.154, 0.108, 2.3], [0.3, -0.006, 0, 0.169, 0.123, 2.3], [0.45, 0.008, 0, 0.172, 0.088, 2.3], [0.527, 0.008, 0, 0.078, 0.066, 2]], "torso-clothing");
  k.loft("head", "head", SKIN, T.skin, [[-0.035, 0.014, 0, 0.068, 0.066], [0.03, -0.012, 0, 0.06, 0.072], [0.1, -0.012, 0, 0.073, 0.094], [0.18, -0.006, 0, 0.076, 0.096], [0.236, 0, 0, 0.05, 0.06]], "head-skin");
  k.loft("hair", "head", HAIR, T.hair, [[0.1, 0.04, 0, 0.066, 0.046], [0.17, 0.01, 0, 0.082, 0.094], [0.226, 0.001, 0, 0.068, 0.08], [0.2529, 0, 0, 0.04, 0.046]], "hair");
  k.loft("shorts", "pelvis", SHIRT, T.shorts, [[0.135, 0, 0, 0.153, 0.106, 2.3], [0.0, 0.02, 0, 0.174, 0.13, 2.4], [-0.135, 0.018, 0, 0.135, 0.108, 2.4]], "leg-clothing");
  for (const [sfx, g] of SIDES) {
    k.loft(`shorts-leg-${sfx}`, `thigh-${sfx}`, SHIRT, T.shorts, m([[-0.03, 0.006, -0.004, 0.078, 0.092], [0.17, 0.01, 0.002, 0.087, 0.096]], g), "leg-clothing", "body", 2.3);
    k.loft(`thigh-${sfx}`, `thigh-${sfx}`, SKIN, T.skin, m([[0.15, 0.01, 0.001, 0.081, 0.089], [0.44, 0.006, 0, 0.054, 0.058], [0.482, 0.003, 0, 0.038, 0.04]], g), "leg-skin");
    k.loft(`shank-${sfx}`, `shank-${sfx}`, SKIN, T.skin, [[-0.04, 0.004, 0, 0.038, 0.04], [0.12, -0.014, 0, 0.054, 0.063], [0.36, -0.002, 0, 0.034, 0.04], [0.455, -0.001, 0, 0.037, 0.043]], "leg-skin");
    k.loft(`shoe-${sfx}`, `foot-${sfx}`, SHOE, T.shoe, m([[-0.08, -0.044, 0, 0.035, 0.038, 3], [0.02, -0.042, 0.001, 0.042, 0.041, 3], [0.147, -0.057, 0.001, 0.052, 0.026, 3]], g), "shoes", "body", 3);
    k.loft(`shoe-toe-${sfx}`, `toe-${sfx}`, SHOE, T.shoe, m([[-0.004, 0.0255, 0.001, 0.052, 0.0255, 3], [0.097, 0.026, -0.008, 0.026, 0.017, 3]], g), "shoes", "body", 3);
    k.loft(`sleeve-${sfx}`, `upper-arm-${sfx}`, SHIRT, T.shirt, m(SLEEVE500, g), "arm-clothing");
    k.loft(`upper-arm-${sfx}`, `upper-arm-${sfx}`, SKIN, T.skin, m([[0.14, 0.004, 0.003, 0.047, 0.053], [0.366, -0.003, 0, 0.034, 0.034]], g), "arm-skin");
    k.loft(`forearm-${sfx}`, `forearm-${sfx}`, SKIN, T.skin, [[-0.03, -0.004, 0, 0.03, 0.03], [0.04, 0.002, 0, 0.044, 0.042], [0.29, 0, 0, 0.021, 0.029]], "arm-skin");
    k.loft(`hand-${sfx}`, `hand-${sfx}`, SKIN, T.skin, m([[-0.008, 0.003, 0, 0.02, 0.03, 2.6], [0.15, 0.002, -0.02, 0.018, 0.038, 2.6]], g), "hand-skin");
  }
}
/** 👕 Shoulder cap and sleeve of 1:500 as one garment; the thermal model splits it at the sleeve seam (u 0.066). */
export const SLEEVE500: Row[] = [[-0.036, 0, -0.03, 0.03, 0.036], [0.02, 0.001, 0.004, 0.058, 0.06], [0.17, 0.004, 0.003, 0.057, 0.06]];
