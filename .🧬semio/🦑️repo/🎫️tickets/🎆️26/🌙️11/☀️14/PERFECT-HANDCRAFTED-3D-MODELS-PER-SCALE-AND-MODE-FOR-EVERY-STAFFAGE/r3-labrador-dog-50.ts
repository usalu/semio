/** 🦮️ Round 3 Labrador, 1:50 (smallest feature 7.5 mm): body stated section by section from the reference table (adult Labrador Retriever, 0.57 m at the withers, 30 kg, standing alert). World metres, default pose, x ahead of the hip joint, near side z < 0.
 * Usage: bun r3-labrador-dog-50.ts [--write]
 * @see ./r3-animals-labrador-dog.md */
import { A, AO, Bench, COAT, EAR, EYE, HEAD_DEG, NOSE, PAD, TAIL, TONGUE, X, Y, dome, far, near, smooth, span, vec, type St } from "./r3-labrador-dog-kit.ts";
import { DECISIONS_50, structure50, thermal50 } from "./r3-labrador-dog-50-lists.ts";

export const J = { hip: [0, 0.485], stifle: [0.0394, 0.291], hock: [-0.11, 0.155], mtp: [-0.1, 0.034], scapula: [0.4073, 0.552], shoulder: [0.498, 0.4226], elbow: [0.3896, 0.2839], carpus: [0.395, 0.105], mcp: [0.41, 0.032], tail1: [-0.072, 0.522], tail2: [-0.2017, 0.5129], tail3: [-0.3217, 0.5129], c7: [0.47, 0.448], t13: [0.245, 0.502], ls: [0.055, 0.508] } as const satisfies Record<string, readonly [number, number]>;
export const DEG = { thigh: -78.52, shank: -137.7, metatarsus: -85.28, scapula: -55, humerus: -128.01, forearm: -88.27, metacarpus: -78.39, tail1: 184, tail2: 180, tail3: 176 } as const;
export const Z = { hip: 0.055, stifle: 0.08, hock: 0.068, shoulderBlade: 0.035, shoulder: 0.075, elbow: 0.08, carpus: 0.075, ear: 0.06 } as const;
export const H = (s: number, off: number, ra: number, rb: number, e = 2.3): St => A(AO, HEAD_DEG, s, off, ra, rb, 0, e);
export const HU: [number, number, number] = [Math.cos((HEAD_DEG * Math.PI) / 180), Math.sin((HEAD_DEG * Math.PI) / 180), 0];
export const HN: [number, number, number] = [-HU[1], HU[0], 0];
export const EAR_DEG = -73;

/** 🐾️ One paw: the foot from the pastern joint to the toe bases, four arched toes ending in their claws (the two middle ones longer), the large pad and four digital pads. `m` = world x of the pastern joint, `zc` = lateral centre, `k` = size (hind 1, fore 1.1). */
export function paw(B: Bench, leg: "h" | "f", side: "n" | "f", m: number, zc: number, k: number): void {
  const sg = side === "n" ? -1 : 1;
  const S = (st: St[]): St[] => (sg < 0 ? near(st) : far(near(st)));
  const bone = `paw-${leg}${side}`;
  B.loft(`foot-${leg}${side}`, bone, smooth(dome(S([X(m - 0.016, 0.037, zc, 0.019, 0.019 * k), X(m + 0.002, 0.031, zc, 0.022, 0.026 * k, 2.3), X(m + 0.022, 0.026, zc, 0.02, 0.031 * k, 2.5), X(m + 0.04, 0.026, zc, 0.021, 0.0335 * k, 2.6), X(m + 0.055 * k, 0.0225, zc, 0.0185, 0.0325 * k, 2.8)]), 0.008, 0.006), 1), COAT);
  ([["o", 0.0245, 0.064], ["p", 0.0082, 0.077], ["q", -0.0082, 0.077], ["i", -0.0245, 0.064]] as const).forEach(([toe, dz, reach]) => {
    const z = zc + dz * k;
    const t = m + reach * k;
    B.loft(`toe-${leg}${side}-${toe}`, bone, smooth(S([X(t - 0.05 * k, 0.027, z, 0.014, 0.0088 * k), X(t - 0.036 * k, 0.0275, z, 0.017, 0.0096 * k), X(t - 0.022 * k, 0.0215, z, 0.016, 0.0096 * k), X(t - 0.01 * k, 0.014, z, 0.011, 0.0082 * k), X(t - 0.003 * k, 0.009, z, 0.0065, 0.0055), X(t, 0.006, z, 0.004, 0.004)]), 1), COAT);
    B.ell(`toe-pad-${leg}${side}-${toe}`, bone, [t - 0.02 * k, 0.0062, sg * z], [0.009 * k, 0.0062, 0.0066 * k], PAD);
  });
  B.ell(`pad-${leg}${side}`, bone, [m + 0.014, 0.0085, sg * zc], [0.0155 * k, 0.0085, 0.0175 * k], PAD);
}

export function body50(B: Bench): void {
  B.list("body");
  B.loft("chest", "thorax", smooth(dome([X(0.185, 0.4345, 0, 0.112, 0.101, 2.1), X(0.22, 0.4295, 0, 0.1265, 0.118, 2.15), X(0.25, 0.426, 0, 0.13, 0.127, 2.2), X(0.3, 0.422, 0, 0.135, 0.13, 2.3), X(0.35, 0.423, 0, 0.139, 0.13, 2.3), X(0.4, 0.426, 0, 0.142, 0.124, 2.3), X(0.44, 0.43, 0, 0.136, 0.108, 2.2), X(0.475, 0.438, 0, 0.116, 0.092, 2.1), X(0.5, 0.447, 0, 0.09, 0.074, 2), X(0.516, 0.452, 0, 0.058, 0.05, 2), X(0.526, 0.454, 0, 0.022, 0.02, 2)], 0.03, 0), 2), COAT);
  B.loft("hindbody", "lumbar", smooth(dome([X(-0.01, 0.468, 0, 0.078, 0.078, 2), X(0.04, 0.4525, 0, 0.1005, 0.092, 2.1), X(0.09, 0.4435, 0, 0.1115, 0.097, 2.1), X(0.14, 0.437, 0, 0.119, 0.102, 2.2), X(0.19, 0.4315, 0, 0.1245, 0.112, 2.2), X(0.22, 0.4295, 0, 0.1265, 0.118, 2.15), X(0.255, 0.4265, 0, 0.118, 0.112, 2.2)], 0.02, 0.03), 2), COAT);
  B.loft("croup", "pelvis", smooth([X(0.09, 0.468, 0, 0.086, 0.094, 2.2), X(0.04, 0.474, 0, 0.079, 0.1, 2.3), X(-0.01, 0.478, 0, 0.072, 0.097, 2.3), X(-0.05, 0.483, 0, 0.062, 0.084, 2.2), X(-0.08, 0.486, 0, 0.05, 0.064, 2.1), X(-0.097, 0.487, 0, 0.034, 0.04, 2), X(-0.104, 0.487, 0, 0.014, 0.016, 2)], 1), COAT);

  B.loft("neck", "neck", smooth(dome([span([0.4, 0.566], [0.498, 0.372], 0.076), span([0.447, 0.582], [0.538, 0.42], 0.074), span([0.49, 0.618], [0.58, 0.484], 0.067), span([0.532, 0.66], [0.617, 0.546], 0.061), span([0.571, 0.692], [0.648, 0.592], 0.057), span([0.603, 0.706], [0.668, 0.622], 0.052)], 0.03, 0), 1), COAT);

  B.loft("head", "head", smooth([H(-0.03, 0.012, 0.026, 0.034), H(-0.005, 0.012, 0.044, 0.054, 2.4), H(0.03, 0.013, 0.051, 0.066, 2.6), H(0.06, 0.013, 0.053, 0.069, 2.6), H(0.09, 0.011, 0.049, 0.063, 2.5), H(0.11, 0.004, 0.044, 0.052, 2.4), H(0.126, -0.004, 0.039, 0.043, 2.5), H(0.15, -0.007, 0.0375, 0.04, 2.6), H(0.19, -0.008, 0.0355, 0.038, 2.6), H(0.22, -0.006, 0.0325, 0.036, 2.5), H(0.235, -0.003, 0.026, 0.03, 2.3), H(0.242, 0, 0.013, 0.017, 2)], 1), COAT);
  B.loft("jaw", "jaw", smooth([H(0.03, -0.038, 0.016, 0.036), H(0.055, -0.046, 0.021, 0.044, 2.4), H(0.09, -0.049, 0.02, 0.04, 2.4), H(0.13, -0.049, 0.017, 0.034, 2.4), H(0.17, -0.048, 0.015, 0.03, 2.4), H(0.2, -0.046, 0.013, 0.027, 2.3), H(0.216, -0.044, 0.008, 0.019, 2)], 1), COAT);
  B.loft("tongue", "jaw", [H(0.06, -0.029, 0.0045, 0.017, 2), H(0.11, -0.029, 0.0045, 0.019, 2), H(0.17, -0.029, 0.0045, 0.018, 2), H(0.2, -0.029, 0.0042, 0.015, 2), H(0.212, -0.029, 0.004, 0.009, 2)], TONGUE);
  B.ell("nose-leather", "head", B.world("head", [0.2365, 0.006, 0]), [0.0125, 0.0165, 0.02], NOSE, [HU, HN]);
  for (const [side, sg] of [["n", -1], ["f", 1]] as const) {
    B.ell(`eye-${side}`, "head", B.world("head", [0.1, 0.026, sg * 0.0485]), [0.0095, 0.0072, 0.0062], EYE, [HU, HN]);
    const e0 = B.world(`ear-${side}`, [0, 0, 0]);
    const d: [number, number, number] = [Math.cos((EAR_DEG * Math.PI) / 180), Math.sin((EAR_DEG * Math.PI) / 180), 0];
    const q: [number, number, number] = [-d[1], d[0], 0];
    const ear: St[] = ([[-0.01, 0.018, 0.0065, 0.064], [0.008, 0.031, 0.0055, 0.071], [0.032, 0.037, 0.0045, 0.0745], [0.06, 0.034, 0.004, 0.0735], [0.083, 0.026, 0.004, 0.071], [0.099, 0.016, 0.004, 0.0685], [0.107, 0.007, 0.004, 0.067]] as const).map(([t, rq, rm, z]) => ({ c: [e0[0] + d[0] * t, e0[1] + d[1] * t, sg * z], a: vec.mul(q, rq), b: [0, 0, rm], e: 2 }));
    B.loft(`ear-${side}`, `ear-${side}`, smooth(ear, 1), EAR);
  }

  B.loft("tail-root", "tail-1", smooth([A(J.tail1, DEG.tail1, -0.04, 0.004, 0.024, 0.03), A(J.tail1, DEG.tail1, 0.0, 0.004, 0.028, 0.031), A(J.tail1, DEG.tail1, 0.05, 0.001, 0.029, 0.029), A(J.tail1, DEG.tail1, 0.1, 0, 0.027, 0.0265), A(J.tail1, DEG.tail1, 0.14, 0, 0.025, 0.0245)], 1), TAIL);
  B.loft("tail-mid", "tail-2", smooth(dome([A(J.tail2, DEG.tail2, -0.004, 0, 0.0255, 0.025), A(J.tail2, DEG.tail2, 0.06, 0, 0.022, 0.0215), A(J.tail2, DEG.tail2, 0.124, 0, 0.018, 0.0175)], 0.012, 0.01), 1), TAIL);
  B.loft("tail-tip", "tail-3", smooth(dome([A(J.tail3, DEG.tail3, -0.004, 0, 0.019, 0.0175), A(J.tail3, DEG.tail3, 0.045, 0, 0.0155, 0.014), A(J.tail3, DEG.tail3, 0.085, 0, 0.011, 0.01), A(J.tail3, DEG.tail3, 0.104, 0, 0.0065, 0.006)], 0.01, 0.007), 1), TAIL);

  for (const [side, sg] of [["n", -1], ["f", 1]] as const) {
    const S = (st: St[]): St[] => (sg < 0 ? near(st) : far(near(st)));
    B.loft(`thigh-${side}`, `thigh-h${side}`, smooth(dome(S([Y(0.53, -0.012, 0.06, 0.066, 0.032, 2.2), Y(0.49, -0.008, 0.072, 0.09, 0.042, 2.2), Y(0.45, -0.002, 0.077, 0.098, 0.047, 2.2), Y(0.41, 0.006, 0.08, 0.096, 0.048, 2.2), Y(0.37, 0.014, 0.083, 0.086, 0.046, 2.2), Y(0.33, 0.022, 0.086, 0.071, 0.042, 2.2), Y(0.295, 0.03, 0.088, 0.054, 0.037, 2.1), Y(0.272, 0.034, 0.087, 0.04, 0.03, 2)]), 0.018, 0.016), 1), COAT);
    B.loft(`shank-${side}`, `shank-h${side}`, smooth(dome(S([A(J.stifle, DEG.shank, -0.02, -0.005, 0.043, 0.035, 0.087), A(J.stifle, DEG.shank, 0.03, -0.009, 0.046, 0.036, 0.085), A(J.stifle, DEG.shank, 0.07, -0.013, 0.038, 0.031, 0.081), A(J.stifle, DEG.shank, 0.11, -0.012, 0.031, 0.025, 0.077), A(J.stifle, DEG.shank, 0.15, -0.013, 0.027, 0.02, 0.073), A(J.stifle, DEG.shank, 0.183, -0.014, 0.0275, 0.018, 0.07), A(J.stifle, DEG.shank, 0.205, -0.011, 0.0265, 0.018, 0.068)]), 0.02, 0.01), 1), COAT);
    const TC: [number, number] = [-0.138, 0.184];
    B.loft(`hock-${side}`, `metatarsus-h${side}`, smooth(dome(S([A(TC, -58, 0, 0, 0.011, 0.011, Z.hock), A(TC, -58, 0.018, 0.002, 0.015, 0.014, Z.hock), A(TC, -58, 0.04, 0.004, 0.02, 0.017, Z.hock), A(TC, -58, 0.064, 0.002, 0.017, 0.016, Z.hock)]), 0.006, 0.008), 1), COAT);
    B.loft(`metatarsus-${side}`, `metatarsus-h${side}`, smooth(dome(S([A(J.hock, DEG.metatarsus, -0.012, 0, 0.023, 0.02, Z.hock), A(J.hock, DEG.metatarsus, 0.03, 0, 0.0205, 0.018, Z.hock), A(J.hock, DEG.metatarsus, 0.07, 0, 0.019, 0.0175, Z.hock), A(J.hock, DEG.metatarsus, 0.1, 0, 0.0195, 0.019, Z.hock), A(J.hock, DEG.metatarsus, 0.1214, 0, 0.021, 0.022, Z.hock)]), 0.01, 0.01), 1), COAT);
    paw(B, "h", side, J.mtp[0], Z.hock, 1.04);

    B.loft(`shoulder-${side}`, `scapula-f${side}`, smooth(dome(S([A(J.scapula, DEG.scapula, 0.012, 0, 0.036, 0.028, 0.03), A(J.scapula, DEG.scapula, 0.05, 0, 0.052, 0.04, 0.048), A(J.scapula, DEG.scapula, 0.09, -0.002, 0.058, 0.046, 0.062), A(J.scapula, DEG.scapula, 0.13, -0.004, 0.054, 0.05, 0.075), A(J.scapula, DEG.scapula, 0.158, 0, 0.044, 0.048, 0.083), A(J.scapula, DEG.scapula, 0.176, 0, 0.03, 0.04, 0.085)]), 0.016, 0.012), 1), COAT);
    B.loft(`upper-arm-${side}`, `humerus-f${side}`, smooth(dome(S([A(J.shoulder, DEG.humerus, -0.012, -0.008, 0.04, 0.03, 0.1), A(J.shoulder, DEG.humerus, 0.04, -0.02, 0.054, 0.035, 0.103), A(J.shoulder, DEG.humerus, 0.09, -0.024, 0.057, 0.035, 0.103), A(J.shoulder, DEG.humerus, 0.14, -0.018, 0.048, 0.03, 0.098), A(J.shoulder, DEG.humerus, 0.176, -0.01, 0.038, 0.027, 0.093), A(J.shoulder, DEG.humerus, 0.196, -0.006, 0.028, 0.023, 0.09)]), 0.016, 0.012), 1), COAT);
    B.loft(`forearm-${side}`, `forearm-f${side}`, smooth(dome(S([A(J.elbow, DEG.forearm, -0.026, -0.012, 0.034, 0.026, 0.088), A(J.elbow, DEG.forearm, 0.01, -0.006, 0.038, 0.029, 0.086), A(J.elbow, DEG.forearm, 0.05, -0.002, 0.034, 0.027, 0.083), A(J.elbow, DEG.forearm, 0.1, 0, 0.029, 0.025, 0.08), A(J.elbow, DEG.forearm, 0.145, 0, 0.0255, 0.0235, 0.077), A(J.elbow, DEG.forearm, 0.172, 0.001, 0.026, 0.025, 0.076), A(J.elbow, DEG.forearm, 0.19, 0.002, 0.024, 0.024, 0.075)]), 0.016, 0.008), 1), COAT);
    B.loft(`pastern-${side}`, `metacarpus-f${side}`, smooth(dome(S([A(J.carpus, DEG.metacarpus, -0.01, 0, 0.024, 0.024, Z.carpus), A(J.carpus, DEG.metacarpus, 0.03, 0, 0.022, 0.0225, Z.carpus), A(J.carpus, DEG.metacarpus, 0.06, 0, 0.022, 0.024, Z.carpus), A(J.carpus, DEG.metacarpus, 0.0745, 0, 0.023, 0.026, Z.carpus)]), 0.008, 0.008), 1), COAT);
    B.ell(`carpal-pad-${side}`, `metacarpus-f${side}`, [0.3755, 0.088, sg * Z.carpus], [0.0075, 0.01, 0.0085], PAD);
    paw(B, "f", side, J.mcp[0], Z.carpus, 1.15);
  }
}

if (import.meta.main) {
  const B = new Bench(50);
  body50(B);
  thermal50(B);
  structure50(B);
  const ps = B.probe();
  const min = [0, 1, 2].map((d) => Math.min(...ps.map((p) => p.min[d]!)));
  const max = [0, 1, 2].map((d) => Math.max(...ps.map((p) => p.max[d]!)));
  console.log(`1:50 body ${B.lists.body.length} thermal ${B.lists.thermal.length} structure ${B.lists.structure.length}`);
  console.log("extent", min.map((v) => v.toFixed(4)).join(" "), "..", max.map((v) => v.toFixed(4)).join(" "), "size", max.map((v, i) => (v - min[i]!).toFixed(4)).join(" "));
  for (const p of ps) if (p.volume <= 0) console.log("INSIDE OUT", p.id, p.volume);
  if (process.argv.includes("--volumes")) for (const p of ps) console.log(p.id.padEnd(22), (p.volume * 1000).toFixed(2), "L");
  if (process.argv.includes("--write")) B.save(DECISIONS_50);
}
