/** 🏃️ Body 1:50 (round 5: V-shaped trunk with chest, scapular back and sloping trapezius line, deltoid cap and biceps, quadriceps with vastus medialis, medial calf head) of the runner with blade prosthesis, every section stated by hand for a lean 1.80 m man of 68.5 kg with a right transtibial amputation. Trunk, neck and head bones: s up, cv to the back, cw to the left. Limb bones: s distal, cv to the front, cw to the left. Shoe and blade are stated in the sagittal rest plane (x ahead of the leg axis, y above the ground).
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
/** 〰️ A smooth centre line through hand-stated knots `[x, y, thickness]`: Catmull-Rom with `n` points per span, the knots kept. */
export const smooth = (knots: readonly (readonly [number, number, number])[], n = 3): [number, number, number][] => {
  const out: [number, number, number][] = [];
  for (let i = 0; i < knots.length - 1; i++) {
    const [p0, p1, p2, p3] = [knots[Math.max(0, i - 1)]!, knots[i]!, knots[i + 1]!, knots[Math.min(knots.length - 1, i + 2)]!];
    for (let j = 0; j < n; j++) {
      const t = j / n;
      const c = (a: number, b: number, d: number, e: number): number => Math.round(0.5 * (2 * b + (-a + d) * t + (2 * a - 5 * b + 4 * d - e) * t * t + (-a + 3 * b - 3 * d + e) * t * t * t) * 1e5) / 1e5;
      out.push([c(p0[0], p1[0], p2[0], p3[0]), c(p0[1], p1[1], p2[1], p3[1]), c(p0[2], p1[2], p2[2], p3[2])]);
    }
  }
  out.push([...knots[knots.length - 1]!] as [number, number, number]);
  return out;
};
export const BLADE_UPPER: [number, number, number][] = [[0.036, 0.313, 0.011], [-0.02, 0.313, 0.011], [-0.055, 0.305, 0.011], [-0.084, 0.284, 0.011], [-0.104, 0.254, 0.011], [-0.113, 0.221, 0.011], [RIG.bladeHinge[0], RIG.bladeHinge[1], 0.011]];
export const BLADE_LOWER: [number, number, number][] = [[RIG.bladeHinge[0], RIG.bladeHinge[1], 0.011], [-0.1, 0.153, 0.0105], [-0.078, 0.113, 0.01], [-0.043, 0.074, 0.0095], [0.004, 0.044, 0.009], [0.055, 0.0235, 0.008], [0.1, 0.0115, 0.007], [0.14, 0.008, 0.006], [0.153, 0.0105, 0.006]];
export const BLADE_PAD: [number, number, number][] = [[0.058, 0.0165, 0.005], [0.1, 0.0055, 0.005], [0.14, 0.0025, 0.005], [0.152, 0.005, 0.005]];
export const BLADE_W = 0.031;

export function body50(k: Kit): void {
  k.loft("singlet", "torso", M.singlet, T.singlet, [[-0.078, 0.01, 0, 0.17, 0.122, 2.3], [-0.03, 0.006, 0, 0.158, 0.112, 2.3], [0.03, -0.003, 0, 0.146, 0.102, 2.3], [0.09, -0.006, 0, 0.14, 0.1, 2.3], [0.16, -0.0055, 0, 0.147, 0.105, 2.3], [0.23, -0.0055, 0, 0.157, 0.1135, 2.3], [0.29, -0.003, 0, 0.164, 0.119, 2.3], [0.35, -0.002, 0, 0.171, 0.123, 2.3], [0.4, 0.001, 0, 0.181, 0.117, 2.3], [0.44, 0.002, 0, 0.188, 0.104, 2.2], [0.465, 0.0025, 0, 0.174, 0.0895, 2.2], [0.485, 0.0045, 0, 0.136, 0.0765, 2.1], [0.5, 0.0055, 0, 0.098, 0.0675, 2], [0.512, 0.0055, 0, 0.076, 0.0635]], "singlet");
  k.loft("singlet-collar", "torso", M.singlet, T.trim, [[0.506, 0.0055, 0, 0.089, 0.069], [0.517, 0.0055, 0, 0.074, 0.065]], "singlet", "detail");
  k.loft("neck", "neck", M.skin, T.skin, [[-0.03, 0.004, 0, 0.066, 0.065], [0.0, 0.003, 0, 0.061, 0.062], [0.04, 0.001, 0, 0.058, 0.061], [0.085, -0.001, 0, 0.057, 0.061], [0.13, -0.004, 0, 0.054, 0.06]], "face-skin");
  k.loft("head", "head", M.skin, T.skin, [[-0.088, -0.084, 0, 0.02, 0.016, 2.2], [-0.068, -0.068, 0, 0.044, 0.04, 2.2], [-0.041, -0.048, 0, 0.058, 0.062, 2.2], [-0.011, -0.026, 0, 0.068, 0.085], [0.029, -0.016, 0, 0.074, 0.095], [0.069, -0.014, 0, 0.0775, 0.098], [0.104, -0.01, 0, 0.072, 0.09], [0.134, -0.008, 0, 0.052, 0.066], [0.151, -0.008, 0, 0.018, 0.024]], "face-skin");
  k.cap("nose", "head", M.skin, T.skin, [0.034, -0.108, 0], [-0.003, -0.126, 0], 0.007, 0.0115, "face-skin");
  for (const [sfx, g] of SIDES) {
    k.ell(`ear-${sfx}`, "head", M.skin, T.skin, [0.014, -0.004, 0.076 * g], [0.028, 0.016, 0.0075], "face-skin", "detail", [0.1305 * g, 0, 0, 0.9914]);
    k.ell(`eye-${sfx}`, "head", M.skin, T.eye, [0.032, -0.1005, 0.031 * g], [0.005, 0.004, 0.011], "face-skin", "detail");
  }
  k.loft("hair", "head", M.hair, T.hair, [[-0.008, 0.038, 0, 0.056, 0.034], [0.029, 0.026, 0, 0.077, 0.062], [0.069, 0.006, 0, 0.0805, 0.086], [0.099, -0.006, 0, 0.078, 0.095], [0.129, -0.008, 0, 0.061, 0.076], [0.147, -0.008, 0, 0.04, 0.05], [0.157, -0.008, 0, 0.02, 0.026]], "hair");

  k.loft("shorts", "pelvis", M.shorts, T.shorts, [[-0.088, 0.012, 0, 0.1, 0.07, 2.4], [-0.06, 0.014, 0, 0.158, 0.103, 2.4], [-0.02, 0.016, 0, 0.168, 0.113, 2.4], [0.02, 0.012, 0, 0.162, 0.106, 2.4], [0.05, 0.01, 0, 0.152, 0.097, 2.3]], "shorts");
  for (const [sfx, g] of SIDES) {
    k.ell(`seat-${sfx}`, "pelvis", M.shorts, T.shorts, [-0.03, 0.07, 0.072 * g], [0.085, 0.064, 0.078], "shorts");
    k.loft(`shorts-leg-${sfx}`, `thigh-${sfx}`, M.shorts, T.shorts, m([[-0.03, 0.004, -0.004, 0.078, 0.092], [0.03, 0.006, -0.003, 0.087, 0.0995], [0.08, 0.008, -0.002, 0.0875, 0.0985], [0.125, 0.0095, -0.001, 0.0865, 0.0965]], g), "shorts", "body", 2.2);
  }
  k.loft("thigh-l", "thigh-l", M.skin, T.skin, [[0.117, 0.0095, 0.001, 0.0825, 0.0925], [0.16, 0.011, 0, 0.081, 0.091], [0.21, 0.013, 0, 0.078, 0.088], [0.26, 0.013, -0.001, 0.074, 0.083], [0.31, 0.011, -0.002, 0.068, 0.076], [0.36, 0.008, -0.004, 0.062, 0.068], [0.4, 0.006, -0.003, 0.056, 0.062], [0.435, 0.006, 0, 0.053, 0.059], [0.458, 0.005, 0, 0.047, 0.051], [0.477, 0.004, 0, 0.03, 0.033]], "leg-skin");
  k.ell("patella-l", "shank-l", M.skin, T.skin, [-0.006, 0.043, 0], [0.03, 0.022, 0.027], "leg-skin");
  k.loft("shank-l", "shank-l", M.skin, T.skin, [[-0.045, 0.004, 0, 0.03, 0.032], [-0.025, 0.004, 0, 0.047, 0.052], [0, 0.004, 0, 0.053, 0.058], [0.05, -0.004, -0.002, 0.051, 0.057], [0.11, -0.012, -0.004, 0.055, 0.064], [0.17, -0.011, -0.004, 0.052, 0.06], [0.23, -0.006, -0.002, 0.044, 0.05], [0.29, -0.003, 0, 0.036, 0.042], [0.345, -0.002, 0, 0.032, 0.038], [0.39, -0.002, 0, 0.0305, 0.0385], [0.425, -0.001, 0, 0.034, 0.04], [0.448, -0.001, 0, 0.03, 0.036]], "leg-skin");
  k.loft("sock-l", "shank-l", M.sock, T.sock, [[0.378, -0.002, 0, 0.0335, 0.0415], [0.4, -0.002, 0, 0.0335, 0.0418], [0.425, -0.001, 0, 0.037, 0.043], [0.444, -0.001, 0, 0.035, 0.041]], "running-shoe");
  k.loftPlane("sole-l", "foot-l", FOOT, M.shoe, T.sole, [[-0.076, 0.018, 0, 0.018, 0.01], [-0.064, 0.015, 0, 0.033, 0.0145], [-0.03, 0.014, 0, 0.039, 0.014], [0.03, 0.0125, 0.002, 0.037, 0.0125], [0.09, 0.011, 0.002, 0.046, 0.011], [0.135, 0.0105, 0.001, 0.051, 0.0105], [0.147, 0.0105, 0.001, 0.05, 0.0105]], "running-shoe", "body", 4);
  k.loftPlane("shoe-l", "foot-l", FOOT, M.shoe, T.shoe, [[-0.071, 0.058, 0, 0.022, 0.033], [-0.05, 0.058, 0, 0.034, 0.036], [-0.01, 0.053, 0, 0.038, 0.029], [0.03, 0.057, 0.001, 0.0385, 0.034], [0.07, 0.048, 0.002, 0.042, 0.027], [0.11, 0.038, 0.002, 0.047, 0.018], [0.135, 0.0345, 0.001, 0.049, 0.0155], [0.147, 0.034, 0.001, 0.048, 0.015]], "running-shoe", "body", 2.4);
  k.loftPlane("sole-toe-l", "toe-l", TOE, M.shoe, T.sole, [[0.131, 0.0105, 0.001, 0.051, 0.0105], [0.175, 0.011, -0.002, 0.047, 0.01], [0.21, 0.014, -0.006, 0.036, 0.0085], [0.228, 0.018, -0.008, 0.016, 0.006]], "running-shoe", "body", 4);
  k.loftPlane("shoe-toe-l", "toe-l", TOE, M.shoe, T.shoe, [[0.131, 0.0345, 0.001, 0.049, 0.0155], [0.17, 0.033, -0.002, 0.045, 0.0145], [0.205, 0.029, -0.006, 0.034, 0.012], [0.225, 0.024, -0.008, 0.015, 0.007]], "running-shoe", "body", 2.4);

  k.loft("thigh-r", "thigh-r", M.skin, T.skin, [[0.117, 0.0095, -0.001, 0.0825, 0.0925], [0.16, 0.011, 0, 0.081, 0.091], [0.21, 0.013, 0, 0.078, 0.088], [0.26, 0.013, 0.001, 0.074, 0.083], [0.31, 0.011, 0.002, 0.068, 0.076], [0.365, 0.008, 0.003, 0.0615, 0.0675]], "leg-skin");
  k.loft("liner-thigh", "thigh-r", M.liner, T.liner, [[0.34, 0.008, 0.002, 0.0675, 0.0745], [0.37, 0.007, 0.002, 0.064, 0.0695], [0.41, 0.006, 0, 0.058, 0.0635], [0.435, 0.006, 0, 0.0565, 0.0625], [0.458, 0.005, 0, 0.0505, 0.0545], [0.478, 0.004, 0, 0.033, 0.036]], "socket");
  k.loft("liner-knee", "shank-r", M.liner, T.liner, [[-0.045, 0.004, 0, 0.033, 0.035], [-0.025, 0.004, 0, 0.0505, 0.0555], [0, 0.004, 0, 0.0565, 0.0615], [0.035, 0.002, 0, 0.055, 0.059]], "socket");
  k.ell("liner-patella", "shank-r", M.liner, T.liner, [-0.006, 0.045, 0], [0.03, 0.022, 0.027], "socket");
  k.loft("socket", "shank-r", M.socket, T.socket, [[0.02, 0.001, 0, 0.0605, 0.066, 2, 13], [0.05, -0.002, 0, 0.0585, 0.064, 2, 6], [0.09, -0.004, 0, 0.054, 0.058], [0.125, -0.003, 0, 0.048, 0.05], [0.147, -0.001, 0, 0.04, 0.041], [0.157, 0, 0, 0.029, 0.029]], "socket");
  k.loft("socket-trim", "shank-r", M.socket, T.rim, [[0.016, 0.001, 0, 0.0625, 0.068, 2, 13], [0.03, 0, 0, 0.0625, 0.068, 2, 13]], "socket", "detail");
  k.loft("socket-adapter", "shank-r", M.socket, T.metal, [[0.152, 0, 0, 0.027, 0.027], [0.163, 0, 0, 0.027, 0.027]], "socket", "detail");
  k.box("socket-pyramid", "shank-r", M.socket, T.metal, [0.1715, 0, 0], [0.019, 0.042, 0.042], 0.004, "socket", "detail");
  k.strip("blade-upper", "blade-r", BLADE, M.blade, T.blade, smooth(BLADE_UPPER), -BLADE_W, BLADE_W, { end: true }, "blade");
  k.strip("blade-curl", "blade-tip-r", TIP, M.blade, T.blade, smooth(BLADE_LOWER), -BLADE_W, BLADE_W, { start: true, end: true }, "blade");
  k.strip("blade-toe", "blade-tip-r", TIP, M.blade, T.pad, BLADE_PAD, -BLADE_W, BLADE_W, {}, "blade");

  for (const [sfx, g] of SIDES) {
    k.loft(`arm-${sfx}`, `arm-${sfx}`, M.skin, T.skin, m([[-0.048, 0, -0.012, 0.012, 0.016], [-0.04, 0.002, -0.004, 0.034, 0.038], [-0.025, 0.003, 0, 0.049, 0.051], [0, 0.004, 0, 0.056, 0.056], [0.05, 0.006, 0, 0.055, 0.054], [0.1, 0.006, 0, 0.049, 0.05], [0.15, 0.004, 0, 0.041, 0.047], [0.21, 0.007, 0, 0.0385, 0.049], [0.26, 0.005, 0, 0.0375, 0.045], [0.3, 0, 0, 0.037, 0.039], [0.33, -0.003, 0, 0.04, 0.037], [0.352, -0.003, 0, 0.034, 0.032], [0.366, -0.002, 0, 0.022, 0.022]], g), "arm-skin");
    k.loft(`forearm-${sfx}`, `forearm-${sfx}`, M.skin, T.skin, [[-0.034, 0, 0, 0.022, 0.022], [-0.02, 0, 0, 0.035, 0.034], [0, 0, 0, 0.041, 0.038], [0.045, 0.003, 0, 0.041, 0.039], [0.11, 0.003, 0, 0.034, 0.037], [0.18, 0.001, 0, 0.027, 0.032], [0.235, 0, 0, 0.0215, 0.0292], [0.26, 0, 0, 0.021, 0.029], [0.275, 0, 0, 0.018, 0.026]], "arm-skin");
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
