/** 🏃️ Body 1:50 of the runner with blade prosthesis, every section stated by hand for a lean 1.80 m man of 68.5 kg with a right transtibial amputation. Trunk, neck and head bones: s up, cv to the back, cw to the left. Limb bones: s distal, cv to the front, cw to the left. Shoe and blade are stated in the sagittal rest plane (x ahead of the leg axis, y above the ground).
 * @see ./r3-runner-figure.ts */
import { RIG } from "./r3-runner-rig.ts";
import type { Kit, Plane, Row, V3 } from "./r3-runner-kit.ts";

export const M = { skin: "skin", hair: "hair", sock: "cotton-sock", singlet: "running-singlet", shorts: "compression-shorts", shoe: "running-shoe", liner: "silicone-liner", socket: "carbon-socket", blade: "carbon-blade" } as const;
export const T = { skin: 0.72, singlet: 0.9, trim: 0.55, shorts: 0.2, hair: 0.15, shoe: 0.82, sole: 0.96, outsole: 0.3, sock: 0.95, liner: 0.56, socket: 0.16, rim: 0.34, metal: 0.62, blade: 0.1, pad: 0.36, eye: 0.1, bone: 0.85 };
export const SIDES = [["l", 1], ["r", -1]] as const;
export const m = (rows: readonly Row[], g: number): Row[] => rows.map(([s, cv, cw, rx, ry, e, t]) => [s, cv, cw * g, rx, ry, e, t] as Row);

const KNEE_Y = RIG.hipY - RIG.thigh;
export const ADAPTER_Y = KNEE_Y - RIG.socket;
const deg = (dx: number, dy: number): number => (Math.atan2(dy, dx) * 180) / Math.PI;
export const FOOT: Plane = { ox: 0, oy: RIG.ankleY, deg: deg(RIG.ballX, -RIG.ankleY) };
export const TOE: Plane = { ox: RIG.ballX, oy: 0, deg: 0 };
export const BLADE: Plane = { ox: 0, oy: ADAPTER_Y, deg: deg(RIG.bladeHinge[0], RIG.bladeHinge[1] - ADAPTER_Y) };
export const TIP: Plane = { ox: RIG.bladeHinge[0], oy: RIG.bladeHinge[1], deg: deg(RIG.bladeToe[0] - RIG.bladeHinge[0], RIG.bladeToe[1] - RIG.bladeHinge[1]) };

/** 🌀️ Centre line of the C-shaped running blade `[x, y, thickness]`: clamp plate under the adapter, rear curve, hinge of the rig at the apex, forefoot lever to the upturned toe. */
export const BLADE_UPPER: [number, number, number][] = [[0.036, 0.313, 0.011], [-0.02, 0.313, 0.011], [-0.055, 0.305, 0.011], [-0.084, 0.284, 0.011], [-0.104, 0.254, 0.011], [-0.113, 0.221, 0.011], [RIG.bladeHinge[0], RIG.bladeHinge[1], 0.011]];
export const BLADE_LOWER: [number, number, number][] = [[RIG.bladeHinge[0], RIG.bladeHinge[1], 0.011], [-0.1, 0.153, 0.0105], [-0.078, 0.113, 0.01], [-0.043, 0.074, 0.0095], [0.004, 0.044, 0.009], [0.055, 0.0235, 0.008], [0.1, 0.0115, 0.007], [0.14, 0.008, 0.006], [0.153, 0.0105, 0.006]];
export const BLADE_PAD: [number, number, number][] = [[0.058, 0.0165, 0.005], [0.1, 0.0055, 0.005], [0.14, 0.0025, 0.005], [0.152, 0.005, 0.005]];
export const BLADE_W = 0.031;

export function body50(k: Kit): void {
  k.loft("singlet", "torso", M.singlet, T.singlet, [[-0.078, 0.014, 0, 0.174, 0.127, 2.3], [-0.03, 0.009, 0, 0.163, 0.116, 2.3], [0.03, 0.003, 0, 0.149, 0.104, 2.3], [0.09, -0.002, 0, 0.142, 0.099, 2.3], [0.16, -0.005, 0, 0.148, 0.105, 2.3], [0.23, -0.008, 0, 0.157, 0.114, 2.3], [0.3, -0.008, 0, 0.162, 0.117, 2.3], [0.36, -0.003, 0, 0.164, 0.11, 2.3], [0.405, 0.003, 0, 0.164, 0.099, 2.2], [0.44, 0.007, 0, 0.166, 0.088, 2.2], [0.462, 0.009, 0, 0.152, 0.079, 2.1], [0.48, 0.01, 0, 0.122, 0.072, 2], [0.495, 0.009, 0, 0.094, 0.068], [0.507, 0.008, 0, 0.076, 0.066]], "singlet");
  k.loft("singlet-collar", "torso", M.singlet, T.trim, [[0.499, 0.008, 0, 0.083, 0.071], [0.512, 0.007, 0, 0.072, 0.0685]], "singlet", "detail");
  k.loft("neck", "neck", M.skin, T.skin, [[-0.03, 0.004, 0, 0.066, 0.065], [0.0, 0.003, 0, 0.061, 0.062], [0.04, 0.001, 0, 0.057, 0.059], [0.085, -0.001, 0, 0.055, 0.059], [0.13, -0.004, 0, 0.054, 0.06]], "face-skin");
  k.loft("head", "head", M.skin, T.skin, [[-0.088, -0.084, 0, 0.02, 0.016, 2.2], [-0.068, -0.068, 0, 0.044, 0.04, 2.2], [-0.041, -0.048, 0, 0.058, 0.062, 2.2], [-0.011, -0.026, 0, 0.068, 0.085], [0.029, -0.016, 0, 0.074, 0.095], [0.069, -0.014, 0, 0.0775, 0.098], [0.104, -0.01, 0, 0.072, 0.09], [0.134, -0.008, 0, 0.052, 0.066], [0.151, -0.008, 0, 0.018, 0.024]], "face-skin");
  k.cap("nose", "head", M.skin, T.skin, [0.034, -0.108, 0], [-0.003, -0.126, 0], 0.007, 0.0115, "face-skin");
  for (const [sfx, g] of SIDES) {
    k.ell(`ear-${sfx}`, "head", M.skin, T.skin, [0.014, -0.004, 0.076 * g], [0.028, 0.016, 0.0075], "face-skin", "detail", [0.1305 * g, 0, 0, 0.9914]);
    k.ell(`eye-${sfx}`, "head", M.skin, T.eye, [0.032, -0.1005, 0.031 * g], [0.005, 0.004, 0.011], "face-skin", "detail");
  }
  k.loft("hair", "head", M.hair, T.hair, [[-0.008, 0.038, 0, 0.056, 0.034], [0.029, 0.026, 0, 0.077, 0.062], [0.069, 0.006, 0, 0.0805, 0.086], [0.099, -0.006, 0, 0.078, 0.095], [0.129, -0.008, 0, 0.061, 0.076], [0.147, -0.008, 0, 0.04, 0.05], [0.157, -0.008, 0, 0.02, 0.026]], "hair");

  k.loft("shorts", "pelvis", M.shorts, T.shorts, [[-0.088, 0.012, 0, 0.1, 0.07, 2.4], [-0.06, 0.014, 0, 0.158, 0.103, 2.4], [-0.02, 0.016, 0, 0.168, 0.113, 2.4], [0.03, 0.012, 0, 0.165, 0.11, 2.4], [0.07, 0.008, 0, 0.158, 0.106, 2.3]], "shorts");
  for (const [sfx, g] of SIDES) {
    k.ell(`seat-${sfx}`, "pelvis", M.shorts, T.shorts, [-0.012, 0.07, 0.072 * g], [0.092, 0.066, 0.08], "shorts");
    k.loft(`shorts-leg-${sfx}`, `thigh-${sfx}`, M.shorts, T.shorts, m([[-0.03, 0.004, -0.004, 0.078, 0.092], [0.03, 0.006, -0.003, 0.086, 0.099], [0.08, 0.008, -0.002, 0.0855, 0.097], [0.125, 0.0095, -0.001, 0.0835, 0.0945]], g), "shorts", "body", 2.2);
  }
  k.loft("thigh-l", "thigh-l", M.skin, T.skin, [[0.117, 0.0095, 0.001, 0.0815, 0.0925], [0.16, 0.011, 0, 0.079, 0.09], [0.205, 0.011, 0, 0.076, 0.086], [0.25, 0.011, 0, 0.072, 0.081], [0.31, 0.009, 0, 0.065, 0.073], [0.37, 0.007, 0, 0.058, 0.064], [0.41, 0.006, 0, 0.0545, 0.06], [0.435, 0.006, 0, 0.053, 0.059], [0.458, 0.005, 0, 0.047, 0.051], [0.477, 0.004, 0, 0.03, 0.033]], "leg-skin");
  k.ell("patella-l", "shank-l", M.skin, T.skin, [-0.006, 0.043, 0], [0.03, 0.022, 0.027], "leg-skin");
  k.loft("shank-l", "shank-l", M.skin, T.skin, [[-0.045, 0.004, 0, 0.03, 0.032], [-0.025, 0.004, 0, 0.047, 0.052], [0, 0.004, 0, 0.053, 0.058], [0.05, -0.002, 0, 0.05, 0.054], [0.12, -0.01, 0, 0.054, 0.062], [0.2, -0.008, 0, 0.049, 0.056], [0.275, -0.004, 0, 0.04, 0.045], [0.345, -0.002, 0, 0.032, 0.038], [0.39, -0.002, 0, 0.0305, 0.0385], [0.425, -0.001, 0, 0.034, 0.04], [0.448, -0.001, 0, 0.03, 0.036]], "leg-skin");
  k.loft("sock-l", "shank-l", M.sock, T.sock, [[0.378, -0.002, 0, 0.0335, 0.0415], [0.4, -0.002, 0, 0.0335, 0.0418], [0.425, -0.001, 0, 0.037, 0.043], [0.444, -0.001, 0, 0.035, 0.041]], "running-shoe");
  k.loftPlane("sole-l", "foot-l", FOOT, M.shoe, T.sole, [[-0.076, 0.018, 0, 0.018, 0.01], [-0.064, 0.015, 0, 0.033, 0.0145], [-0.03, 0.014, 0, 0.039, 0.014], [0.03, 0.0125, 0.002, 0.037, 0.0125], [0.09, 0.011, 0.002, 0.046, 0.011], [0.135, 0.0105, 0.001, 0.051, 0.0105], [0.147, 0.0105, 0.001, 0.05, 0.0105]], "running-shoe", "body", 4);
  k.loftPlane("shoe-l", "foot-l", FOOT, M.shoe, T.shoe, [[-0.071, 0.058, 0, 0.022, 0.033], [-0.05, 0.058, 0, 0.034, 0.036], [-0.01, 0.053, 0, 0.038, 0.029], [0.03, 0.057, 0.001, 0.0385, 0.034], [0.07, 0.048, 0.002, 0.042, 0.027], [0.11, 0.038, 0.002, 0.047, 0.018], [0.135, 0.0345, 0.001, 0.049, 0.0155], [0.147, 0.034, 0.001, 0.048, 0.015]], "running-shoe", "body", 2.4);
  k.loftPlane("sole-toe-l", "toe-l", TOE, M.shoe, T.sole, [[0.131, 0.0105, 0.001, 0.051, 0.0105], [0.175, 0.011, -0.002, 0.047, 0.01], [0.21, 0.014, -0.006, 0.036, 0.0085], [0.228, 0.018, -0.008, 0.016, 0.006]], "running-shoe", "body", 4);
  k.loftPlane("shoe-toe-l", "toe-l", TOE, M.shoe, T.shoe, [[0.131, 0.0345, 0.001, 0.049, 0.0155], [0.17, 0.033, -0.002, 0.045, 0.0145], [0.205, 0.029, -0.006, 0.034, 0.012], [0.225, 0.024, -0.008, 0.015, 0.007]], "running-shoe", "body", 2.4);

  k.loft("thigh-r", "thigh-r", M.skin, T.skin, [[0.117, 0.0095, -0.001, 0.0815, 0.0925], [0.16, 0.011, 0, 0.079, 0.09], [0.205, 0.011, 0, 0.076, 0.086], [0.25, 0.011, 0, 0.072, 0.081], [0.31, 0.009, 0, 0.065, 0.073], [0.365, 0.007, 0, 0.0585, 0.0645]], "leg-skin");
  k.loft("liner-thigh", "thigh-r", M.liner, T.liner, [[0.335, 0.008, 0, 0.066, 0.0735], [0.37, 0.007, 0, 0.0615, 0.0675], [0.41, 0.006, 0, 0.058, 0.0635], [0.435, 0.006, 0, 0.0565, 0.0625], [0.458, 0.005, 0, 0.0505, 0.0545], [0.478, 0.004, 0, 0.033, 0.036]], "socket");
  k.loft("liner-knee", "shank-r", M.liner, T.liner, [[-0.045, 0.004, 0, 0.033, 0.035], [-0.025, 0.004, 0, 0.0505, 0.0555], [0, 0.004, 0, 0.0565, 0.0615], [0.035, 0.002, 0, 0.055, 0.059]], "socket");
  k.ell("liner-patella", "shank-r", M.liner, T.liner, [-0.006, 0.045, 0], [0.03, 0.022, 0.027], "socket");
  k.loft("socket", "shank-r", M.socket, T.socket, [[0.02, 0.001, 0, 0.0605, 0.066, 2, 13], [0.05, -0.002, 0, 0.0585, 0.064, 2, 6], [0.09, -0.004, 0, 0.054, 0.058], [0.125, -0.003, 0, 0.048, 0.05], [0.147, -0.001, 0, 0.04, 0.041], [0.157, 0, 0, 0.029, 0.029]], "socket");
  k.loft("socket-trim", "shank-r", M.socket, T.rim, [[0.016, 0.001, 0, 0.0625, 0.068, 2, 13], [0.03, 0, 0, 0.0625, 0.068, 2, 13]], "socket", "detail");
  k.loft("socket-adapter", "shank-r", M.socket, T.metal, [[0.152, 0, 0, 0.027, 0.027], [0.163, 0, 0, 0.027, 0.027]], "socket", "detail");
  k.box("socket-pyramid", "shank-r", M.socket, T.metal, [0.1715, 0, 0], [0.019, 0.042, 0.042], 0.004, "socket", "detail");
  k.strip("blade-upper", "blade-r", BLADE, M.blade, T.blade, BLADE_UPPER, -BLADE_W, BLADE_W, { end: true }, "blade");
  k.strip("blade-curl", "blade-tip-r", TIP, M.blade, T.blade, BLADE_LOWER, -BLADE_W, BLADE_W, { start: true, end: true }, "blade");
  k.strip("blade-toe", "blade-tip-r", TIP, M.blade, T.pad, BLADE_PAD, -BLADE_W, BLADE_W, {}, "blade");

  for (const [sfx, g] of SIDES) {
    k.loft(`arm-${sfx}`, `arm-${sfx}`, M.skin, T.skin, m([[-0.047, 0, -0.022, 0.009, 0.013], [-0.042, 0, -0.018, 0.023, 0.03], [-0.034, 0, -0.012, 0.036, 0.043], [-0.016, 0, -0.004, 0.047, 0.052], [0.01, 0.001, 0.001, 0.0515, 0.056], [0.05, 0.002, 0.003, 0.0505, 0.055], [0.1, 0.003, 0.003, 0.045, 0.051], [0.15, 0.005, 0.002, 0.042, 0.049], [0.21, 0.004, 0.001, 0.0405, 0.0465], [0.27, 0.001, 0, 0.037, 0.041], [0.305, -0.001, 0, 0.037, 0.039], [0.33, -0.002, 0, 0.039, 0.038], [0.352, -0.002, 0, 0.034, 0.033], [0.366, -0.002, 0, 0.022, 0.022]], g), "arm-skin");
    k.loft(`forearm-${sfx}`, `forearm-${sfx}`, M.skin, T.skin, [[-0.034, 0, 0, 0.022, 0.022], [-0.02, 0, 0, 0.034, 0.033], [0, 0, 0, 0.039, 0.036], [0.045, 0.003, 0, 0.0405, 0.04], [0.11, 0.003, 0, 0.036, 0.037], [0.18, 0.001, 0, 0.028, 0.0325], [0.235, 0, 0, 0.0215, 0.029], [0.26, 0, 0, 0.0195, 0.028], [0.275, 0, 0, 0.018, 0.026]], "arm-skin");
    k.loft(`hand-${sfx}`, `hand-${sfx}`, M.skin, T.skin, m([[-0.012, 0.002, 0, 0.0185, 0.026], [0, 0.003, 0, 0.02, 0.029], [0.03, 0.006, -0.002, 0.0165, 0.037], [0.07, 0.004, -0.004, 0.0145, 0.042], [0.095, 0.002, -0.006, 0.0125, 0.04], [0.104, 0.002, -0.007, 0.008, 0.036]], g), "hand-skin", "body", 2.6);
    for (const [i, kv, ku, len] of [[1, 0.03, 0.094, 1], [2, 0.01, 0.097, 1.08], [3, -0.01, 0.094, 1], [4, -0.028, 0.087, 0.8]] as const) {
      const p0: V3 = [ku, kv, -0.006 * g];
      const p1: V3 = [ku + 0.036 * len, kv, -0.02 * g];
      const p2: V3 = [ku + 0.026 * len, kv - 0.001, -0.05 * len * g - 0.004 * g];
      k.cap(`finger-${i}-${sfx}`, `hand-${sfx}`, M.skin, T.skin, p0, p1, 0.0092, 0.0085, "hand-skin");
      k.cap(`fingertip-${i}-${sfx}`, `hand-${sfx}`, M.skin, T.skin, p1, p2, 0.0085, 0.0075, "hand-skin");
    }
    k.cap(`thumb-${sfx}`, `hand-${sfx}`, M.skin, T.skin, [0.022, 0.03, -0.006 * g], [0.067, 0.048, -0.018 * g], 0.011, 0.0095, "hand-skin");
    k.cap(`thumbtip-${sfx}`, `hand-${sfx}`, M.skin, T.skin, [0.067, 0.048, -0.018 * g], [0.1, 0.04, -0.03 * g], 0.0095, 0.008, "hand-skin");
  }
}
