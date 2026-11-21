/** 🦮️ Round 5 Labrador, 1:100, 1:200 and 1:500: each scale stated anew with its own merged masses and fewer sections (smallest feature 15, 30 and 75 mm). World metres, default pose, near side z < 0.
 * Usage: bun r5-labrador-dog-coarse.ts <100|200|500> [--write]
 * @see ./r5-labrador-dog-50.ts
 * @see ./r3-animals-labrador-dog.md */
import { A, Bench, COAT, EAR, NOSE, PAD, TAIL, TONGUE, X, Y, along, dome, far, near, smooth, span, vec, type St } from "./r5-labrador-dog-kit.ts";
import { DEG, EAR_DEG, H, HN, HU, J, Z } from "./r5-labrador-dog-50.ts";
import { BONE, CERVICAL, OUTLET, SPINE_Y, THORACIC, hoop, masses, outlets, pin, thermalRefs } from "./r5-labrador-dog-50-lists.ts";
import { DECISIONS } from "./r5-labrador-dog-decisions.ts";
import { type V3 } from "./volumes-lib.ts";

const SIDES = [["n", -1], ["f", 1]] as const;
const side = (sg: number, st: St[]): St[] => (sg < 0 ? near(st) : far(near(st)));
const AO: readonly [number, number] = [0.612, 0.648];

/** 👂️ The hanging ear flaps: rows `[t along the ear, half-width, half-thickness, |z|]`. */
function ears(B: Bench, rows: readonly (readonly [number, number, number, number])[], per: number): void {
  for (const [s, sg] of SIDES) {
    const e0 = B.world(`ear-${s}`, [0, 0, 0]);
    const d: V3 = [Math.cos((EAR_DEG * Math.PI) / 180), Math.sin((EAR_DEG * Math.PI) / 180), 0];
    const q: V3 = [-d[1], d[0], 0];
    B.loft(`ear-${s}`, `ear-${s}`, smooth(rows.map(([t, rq, rm, z]) => ({ c: [e0[0] + d[0] * t, e0[1] + d[1] * t, sg * z] as V3, a: vec.mul(q, rq), b: [0, 0, rm] as V3, e: 2 })), per), EAR);
  }
}

/** 🦴️ A straight bone bar between two world points. */
const bar = (B: Bench, id: string, bone: string, p0: V3, p1: V3, r0: number, r1 = r0): void => {
  B.list("structure", { member: bone, part: "member" });
  B.cap(id, bone, p0, p1, r0, r1, BONE);
};

/** 🥢️ A plain round rod between two world points (two sections, flat ends): the bar of the coarsest scale. */
const rod = (B: Bench, id: string, bone: string, p0: V3, p1: V3, r: number): void => {
  B.list("structure", { member: bone, part: "member" });
  const d = vec.unit(vec.sub(p1, p0));
  const n = vec.unit(vec.cross(d, Math.abs(d[2]) > 0.9 ? [1, 0, 0] : [0, 0, 1]));
  const m = vec.cross(d, n);
  B.loft(id, bone, [p0, p1].map((c) => ({ c, a: vec.mul(n, r), b: vec.mul(m, r), e: 2 })), BONE);
};

/** 🩻️ Rib hoops of a coarse scale: `[x, ventral end height, lateral half-width]`, the dorsal end on the vertebral body line, sloping back by `slope` degrees. */
function ribs(B: Bench, rows: readonly (readonly [number, number, number])[], t: number, slope: number, id?: string): void {
  rows.forEach(([x, yb, rz], i) => {
    const yt = SPINE_Y(x) - 0.002;
    hoop(B, id ?? `rib-${i + 1}`, x, (yt + yb) / 2, (yt - yb) / 2, rz, t, 0, { member: "thorax", part: "member" }, slope);
  });
}

/** 🦴️ The caudal vertebrae of a coarse scale as one bar per tail bone from the sacrum to the tip, `r` = start radius of each and end radius of the last. */
function caudals(B: Bench, r: readonly [number, number, number, number]): void {
  bar(B, "caudal-1", "tail-1", [-0.066, 0.5216, 0], [J.tail2[0], J.tail2[1], 0], r[0], r[1]);
  bar(B, "caudal-2", "tail-2", [J.tail2[0], J.tail2[1], 0], [J.tail3[0], J.tail3[1], 0], r[1], r[2]);
  bar(B, "caudal-3", "tail-3", [J.tail3[0], J.tail3[1], 0], [J.tailTip[0] + 0.01, J.tailTip[1] - 0.0005, 0], r[2], r[3]);
}

/** 🧱️ The bars of the four legs between the rig joints, each `r` thick, with one bar for the digits of a paw. */
function limbBars(B: Bench, r: number, withScapula: boolean): void {
  for (const [s, sg] of SIDES) {
    const z = (v: number): number => sg * v;
    if (withScapula) bar(B, `scapula-${s}`, `scapula-f${s}`, [J.scapula[0] + 0.012, J.scapula[1] - 0.017, z(0.05)], [J.shoulder[0], J.shoulder[1], z(0.077)], r);
    bar(B, `humerus-${s}`, `humerus-f${s}`, [J.shoulder[0], J.shoulder[1], z(0.078)], [J.elbow[0], J.elbow[1], z(Z.elbow)], r);
    bar(B, `radius-ulna-${s}`, `forearm-f${s}`, [J.elbow[0], J.elbow[1], z(Z.elbow)], [J.carpus[0], J.carpus[1], z(Z.carpus)], r);
    bar(B, `metacarpals-${s}`, `metacarpus-f${s}`, [J.carpus[0], J.carpus[1], z(Z.carpus)], [J.mcp[0], J.mcp[1], z(Z.carpus)], r);
    bar(B, `phalanges-f${s}`, `paw-f${s}`, [J.mcp[0], J.mcp[1], z(Z.carpus)], [J.mcp[0] + 0.06, Math.max(0.016, r + 0.002), z(Z.carpus)], r);
    bar(B, `femur-${s}`, `thigh-h${s}`, [J.hip[0], J.hip[1], z(0.06)], [J.stifle[0], J.stifle[1], z(Z.stifle)], r);
    bar(B, `tibia-${s}`, `shank-h${s}`, [J.stifle[0], J.stifle[1], z(Z.stifle)], [J.hock[0], J.hock[1], z(Z.hock)], r);
    bar(B, `metatarsals-${s}`, `metatarsus-h${s}`, [J.hock[0], J.hock[1], z(Z.hock)], [J.mtp[0], J.mtp[1], z(Z.hock)], r);
    bar(B, `phalanges-h${s}`, `paw-h${s}`, [J.mtp[0], J.mtp[1], z(Z.hock)], [J.mtp[0] + 0.055, Math.max(0.016, r + 0.002), z(Z.hock)], r);
  }
}

function body100(B: Bench): void {
  B.list("body");
  B.loft("chest", "thorax", smooth(dome([X(0.185, 0.4345, 0, 0.112, 0.101, 2.1), X(0.22, 0.4295, 0, 0.1265, 0.118, 2.15), X(0.27, 0.424, 0, 0.132, 0.128, 2.2), X(0.34, 0.4225, 0, 0.138, 0.13, 2.3), X(0.4, 0.426, 0, 0.142, 0.124, 2.3), X(0.45, 0.432, 0, 0.132, 0.105, 2.2), X(0.49, 0.443, 0, 0.1, 0.082, 2), X(0.52, 0.453, 0, 0.05, 0.042, 2)], 0.03, 0.006), 1), COAT);
  B.loft("hindbody", "lumbar", smooth(dome([X(-0.097, 0.487, 0, 0.034, 0.04, 2), X(-0.07, 0.485, 0, 0.056, 0.07, 2.1), X(-0.02, 0.478, 0, 0.072, 0.095, 2.2), X(0.04, 0.4565, 0, 0.0965, 0.098, 2.2), X(0.1, 0.4425, 0, 0.1125, 0.098, 2.1), X(0.17, 0.4335, 0, 0.1225, 0.108, 2.2), X(0.22, 0.4295, 0, 0.1265, 0.118, 2.15), X(0.255, 0.4265, 0, 0.118, 0.112, 2.2)], 0.008, 0.03), 1), COAT);
  B.loft("neck", "neck", smooth(dome([span([0.4, 0.567], [0.498, 0.372], 0.084), span([0.46, 0.603], [0.562, 0.43], 0.082), span([0.51, 0.652], [0.607, 0.5], 0.075), span([0.56, 0.69], [0.645, 0.572], 0.066), span([0.603, 0.708], [0.668, 0.62], 0.056)], 0.03, 0), 1), COAT);
  B.loft("head", "head", smooth([H(-0.03, 0.012, 0.026, 0.034), H(0, 0.012, 0.046, 0.056, 2.5), H(0.045, 0.013, 0.053, 0.069, 2.6), H(0.09, 0.011, 0.049, 0.063, 2.5), H(0.118, 0.001, 0.041, 0.047, 2.5), H(0.15, -0.007, 0.0375, 0.04, 2.6), H(0.21, -0.007, 0.034, 0.037, 2.5), H(0.236, -0.003, 0.025, 0.03, 2.3), H(0.243, 0, 0.013, 0.017, 2)], 1), COAT);
  B.loft("jaw", "jaw", smooth([H(0.03, -0.038, 0.016, 0.036), H(0.07, -0.048, 0.021, 0.043, 2.4), H(0.14, -0.049, 0.017, 0.033, 2.4), H(0.2, -0.046, 0.013, 0.027, 2.3), H(0.216, -0.044, 0.008, 0.019, 2)], 1), COAT);
  B.ell("nose-leather", "head", B.world("head", [0.236, 0.006, 0]), [0.013, 0.016, 0.0195], NOSE, [HU, HN]);
  ears(B, [[-0.01, 0.018, 0.0078, 0.066], [0.02, 0.035, 0.0078, 0.074], [0.06, 0.034, 0.0078, 0.075], [0.095, 0.02, 0.0078, 0.071], [0.107, 0.008, 0.0076, 0.069]], 1);
  B.loft("tail-root", "tail-1", [A(J.tail1, DEG.tail1, -0.045, 0.012, 0.026, 0.034), A(J.tail1, DEG.tail1, 0.02, 0.003, 0.0335, 0.035), A(J.tail1, DEG.tail1, 0.14, 0, 0.0285, 0.028)], TAIL);
  B.loft("tail-mid", "tail-2", dome([A(J.tail2, DEG.tail2, -0.01, 0, 0.029, 0.0285), A(J.tail2, DEG.tail2, 0.126, 0, 0.0215, 0.021)], 0.012, 0.01), TAIL);
  B.loft("tail-tip", "tail-3", dome([A(J.tail3, DEG.tail3, -0.008, 0, 0.0225, 0.0215), A(J.tail3, DEG.tail3, 0.06, 0, 0.016, 0.0152), A(J.tail3, DEG.tail3, 0.098, 0, 0.0095, 0.009)], 0.01, 0.008), TAIL);
  for (const [s, sg] of SIDES) {
    B.loft(`thigh-${s}`, `thigh-h${s}`, smooth(dome(side(sg, [Y(0.53, -0.012, 0.06, 0.066, 0.032, 2.2), Y(0.47, -0.005, 0.075, 0.095, 0.045, 2.2), Y(0.4, 0.008, 0.081, 0.094, 0.048, 2.2), Y(0.33, 0.022, 0.086, 0.071, 0.042, 2.2), Y(0.28, 0.033, 0.087, 0.045, 0.033, 2)]), 0.018, 0.016), 1), COAT);
    B.loft(`shank-${s}`, `shank-h${s}`, smooth(dome(side(sg, [A(J.stifle, DEG.shank, -0.02, -0.005, 0.043, 0.035, 0.087), A(J.stifle, DEG.shank, 0.04, -0.01, 0.045, 0.035, 0.084), A(J.stifle, DEG.shank, 0.11, -0.012, 0.031, 0.025, 0.077), A(J.stifle, DEG.shank, 0.17, -0.014, 0.027, 0.019, 0.071), A(J.stifle, DEG.shank, 0.205, -0.011, 0.0265, 0.018, 0.068)]), 0.02, 0.01), 1), COAT);
    B.loft(`metatarsus-${s}`, `metatarsus-h${s}`, smooth(dome(side(sg, [A(J.hock, DEG.metatarsus, -0.034, -0.022, 0.012, 0.012, Z.hock), A(J.hock, DEG.metatarsus, -0.008, -0.007, 0.026, 0.019, Z.hock), A(J.hock, DEG.metatarsus, 0.03, 0, 0.0205, 0.018, Z.hock), A(J.hock, DEG.metatarsus, 0.08, 0, 0.019, 0.0175, Z.hock), A(J.hock, DEG.metatarsus, 0.1214, 0, 0.021, 0.022, Z.hock)]), 0.006, 0.01), 1), COAT);
    B.loft(`shoulder-${s}`, `scapula-f${s}`, smooth(dome(side(sg, [A(J.scapula, DEG.scapula, 0.012, 0, 0.036, 0.028, 0.03), A(J.scapula, DEG.scapula, 0.07, -0.001, 0.056, 0.043, 0.055), A(J.scapula, DEG.scapula, 0.13, -0.004, 0.054, 0.05, 0.075), A(J.scapula, DEG.scapula, 0.172, 0, 0.034, 0.042, 0.085)]), 0.016, 0.012), 1), COAT);
    B.loft(`upper-arm-${s}`, `humerus-f${s}`, smooth(dome(side(sg, [A(J.shoulder, DEG.humerus, -0.012, -0.008, 0.04, 0.03, 0.1), A(J.shoulder, DEG.humerus, 0.06, -0.022, 0.056, 0.034, 0.102), A(J.shoulder, DEG.humerus, 0.13, -0.02, 0.05, 0.031, 0.099), A(J.shoulder, DEG.humerus, 0.19, -0.007, 0.031, 0.024, 0.091)]), 0.016, 0.012), 1), COAT);
    B.loft(`forearm-${s}`, `forearm-f${s}`, smooth(dome(side(sg, [A(J.elbow, DEG.forearm, -0.026, -0.012, 0.034, 0.026, 0.088), A(J.elbow, DEG.forearm, 0.02, -0.005, 0.038, 0.029, 0.086), A(J.elbow, DEG.forearm, 0.09, 0, 0.03, 0.025, 0.081), A(J.elbow, DEG.forearm, 0.15, 0, 0.0255, 0.0235, 0.077), A(J.elbow, DEG.forearm, 0.19, 0.002, 0.025, 0.024, 0.075)]), 0.016, 0.008), 1), COAT);
    B.loft(`pastern-${s}`, `metacarpus-f${s}`, dome(side(sg, [A(J.carpus, DEG.metacarpus, -0.01, 0, 0.024, 0.024, Z.carpus), A(J.carpus, DEG.metacarpus, 0.035, 0, 0.022, 0.0225, Z.carpus), A(J.carpus, DEG.metacarpus, 0.0745, 0, 0.023, 0.026, Z.carpus)]), 0.008, 0.008), COAT);
    for (const [leg, m, zc, k] of [["h", J.mtp[0], Z.hock, 1.04], ["f", J.mcp[0], Z.carpus, 1.15]] as const) {
      B.loft(`paw-${leg}${s}`, `paw-${leg}${s}`, smooth(dome(side(sg, [X(m - 0.016, 0.036, zc, 0.02, 0.02 * k), X(m + 0.005, 0.03, zc, 0.024, 0.029 * k, 2.4), X(m + 0.035 * k, 0.025, zc, 0.0225, 0.033 * k, 2.6), X(m + 0.06 * k, 0.0165, zc, 0.0155, 0.032 * k, 2.6), X(m + 0.076 * k, 0.0095, zc, 0.0085, 0.025 * k, 2.4)]), 0.008, 0.003), 1), COAT);
      B.ell(`pad-${leg}${s}`, `paw-${leg}${s}`, [m + 0.03 * k, 0.0077, sg * zc], [0.03 * k, 0.0077, 0.024 * k], PAD);
    }
  }
}

function thermal100(B: Bench): void {
  thermalRefs(B);
  B.list("thermal", { region: "tongue" });
  B.loft("th-tongue", "jaw", [H(0.07, -0.031, 0.0076, 0.015, 2), H(0.13, -0.031, 0.0076, 0.019, 2), H(0.19, -0.031, 0.0076, 0.017, 2), H(0.207, -0.031, 0.0075, 0.011, 2)], TONGUE);
  outlets(B, { nose: [0.2414, 0.006, 0, 0.0085, 0.012], mouth: [0.222, -0.031, 0.0077, 0.014], plume: [[0.22, 0.5545, 0.04], [0.3, 0.557, 0.04], [0.38, 0.565, 0.03]] });
}

function structure100(B: Bench): void {
  const M = (bone: string): void => void B.list("structure", { member: bone, part: "member" });
  M("head");
  B.loft("skull", "head", smooth([H(-0.014, 0.01, 0.02, 0.022), H(0.03, 0.014, 0.039, 0.045, 2.5), H(0.075, 0.012, 0.038, 0.054, 2.4), H(0.125, 0, 0.026, 0.03), H(0.2, -0.002, 0.02, 0.022), H(0.232, 0, 0.009, 0.011)], 1), BONE);
  M("jaw");
  B.loft("mandible", "jaw", [H(0.03, -0.022, 0.014, 0.034), H(0.06, -0.046, 0.017, 0.035), H(0.14, -0.049, 0.011, 0.025), H(0.212, -0.045, 0.0076, 0.012)], BONE);
  M("neck");
  B.loft("cervical", "neck", smooth(along([CERVICAL[0]!, CERVICAL[2]!, CERVICAL[4]!, CERVICAL[5]!].map(([x, y, z, a, b]) => [x, y, z, Math.max(a, 0.016), b] as const)), 2), BONE);
  M("thorax");
  B.loft("thoracic", "thorax", smooth([THORACIC[0]!, THORACIC[1]!, THORACIC[3]!, THORACIC[5]!].map(([top, bottom]) => span(top, bottom, 0.0078)), 1), BONE);
  ribs(B, [[0.265, 0.311, 0.104], [0.31, 0.303, 0.106], [0.355, 0.3, 0.105], [0.4, 0.302, 0.1], [0.44, 0.318, 0.086]], 0.016, 9);
  B.loft("sternum", "thorax", [X(0.27, 0.307, 0, 0.009, 0.013), X(0.39, 0.302, 0, 0.01, 0.014), X(0.45, 0.325, 0, 0.01, 0.013), X(0.495, 0.39, 0, 0.009, 0.011)], BONE);
  M("lumbar");
  B.loft("lumbar-column", "lumbar", [X(0.25, 0.511, 0, 0.026, 0.04, 3), X(0.15, 0.514, 0, 0.028, 0.05, 3), X(0.05, 0.514, 0, 0.026, 0.042, 3)], BONE);
  M("pelvis");
  B.loft("pelvis-girdle", "pelvis", smooth([X(0.09, 0.528, 0, 0.02, 0.064, 3), X(0.045, 0.512, 0, 0.03, 0.06, 2.6), X(0, 0.492, 0, 0.034, 0.066, 2.6), X(-0.045, 0.476, 0, 0.022, 0.058, 2.6), X(-0.084, 0.466, 0, 0.014, 0.056, 2.4)], 1), BONE);
  caudals(B, [0.0095, 0.0085, 0.008, 0.0076]);
  for (const [s, sg] of SIDES) {
    M(`ear-${s}`);
    B.ref(`ear-cartilage-${s}`, `ear-${s}`);
    M(`scapula-f${s}`);
    B.loft(`scapula-${s}`, `scapula-f${s}`, side(sg, [A(J.scapula, DEG.scapula, 0.01, -0.002, 0.03, 0.0077, 0.042), A(J.scapula, DEG.scapula, 0.085, -0.001, 0.027, 0.0077, 0.064), A(J.scapula, DEG.scapula, 0.155, 0, 0.016, 0.012, 0.077)]), BONE);
    const z = (v: number): number => sg * v;
    bar(B, `humerus-${s}`, `humerus-f${s}`, [J.shoulder[0], J.shoulder[1], z(0.078)], [J.elbow[0], J.elbow[1], z(Z.elbow)], 0.013, 0.0125);
    bar(B, `radius-ulna-${s}`, `forearm-f${s}`, [J.elbow[0] - 0.012, J.elbow[1] + 0.03, z(Z.elbow)], [J.carpus[0], J.carpus[1], z(Z.carpus)], 0.012, 0.013);
    bar(B, `metacarpals-${s}`, `metacarpus-f${s}`, [J.carpus[0], J.carpus[1], z(Z.carpus)], [J.mcp[0], J.mcp[1], z(Z.carpus)], 0.012, 0.013);
    bar(B, `phalanges-f${s}`, `paw-f${s}`, [J.mcp[0], J.mcp[1], z(Z.carpus)], [J.mcp[0] + 0.066, 0.014, z(Z.carpus)], 0.011, 0.008);
    bar(B, `femur-${s}`, `thigh-h${s}`, [J.hip[0], J.hip[1], z(0.06)], [J.stifle[0], J.stifle[1], z(Z.stifle)], 0.015, 0.014);
    bar(B, `tibia-${s}`, `shank-h${s}`, [J.stifle[0], J.stifle[1], z(Z.stifle)], [J.hock[0], J.hock[1], z(Z.hock)], 0.014, 0.012);
    bar(B, `metatarsals-${s}`, `metatarsus-h${s}`, [J.hock[0] - 0.018, J.hock[1] + 0.02, z(Z.hock)], [J.mtp[0], J.mtp[1], z(Z.hock)], 0.011, 0.012);
    bar(B, `phalanges-h${s}`, `paw-h${s}`, [J.mtp[0], J.mtp[1], z(Z.hock)], [J.mtp[0] + 0.06, 0.014, z(Z.hock)], 0.011, 0.008);
  }
  for (const [bone, at, h] of [["lumbar", J.ls, 0.018], ["thorax", J.t13, 0.018], ["neck", J.c7, 0.022], ["head", AO, 0.022], ["jaw", [0.64, 0.6319], 0.034]] as const) pin(B, bone, at, 0, h, 0.008);
  for (const [s, sg] of SIDES) for (const [bone, at, z] of [["thigh-h", J.hip, 0.058], ["shank-h", J.stifle, Z.stifle], ["metatarsus-h", J.hock, Z.hock], ["paw-h", J.mtp, Z.hock], ["humerus-f", J.shoulder, 0.078], ["forearm-f", J.elbow, Z.elbow], ["metacarpus-f", J.carpus, Z.carpus], ["paw-f", J.mcp, Z.carpus]] as const) pin(B, bone + s, at, sg * z, /^(meta|paw)/.test(bone) ? 0.012 : 0.016, 0.0078);
  masses(B, [["thorax", 0.03], ["lumbar", 0.023], ["pelvis", 0.021]]);
}

function body200(B: Bench): void {
  B.list("body");
  B.loft("chest", "thorax", smooth(dome([X(0.19, 0.434, 0, 0.113, 0.103, 2.2), X(0.225, 0.429, 0, 0.127, 0.118, 2.2), X(0.31, 0.4225, 0, 0.136, 0.13, 2.3), X(0.4, 0.426, 0, 0.142, 0.124, 2.3), X(0.47, 0.437, 0, 0.12, 0.095, 2.1), X(0.52, 0.453, 0, 0.05, 0.042, 2)], 0.03, 0.006), 1), COAT);
  B.loft("hindbody", "lumbar", smooth(dome([X(-0.097, 0.487, 0, 0.034, 0.04, 2), X(-0.05, 0.483, 0, 0.064, 0.085, 2.2), X(0.04, 0.4565, 0, 0.0965, 0.098, 2.2), X(0.14, 0.437, 0, 0.119, 0.103, 2.2), X(0.225, 0.429, 0, 0.127, 0.118, 2.2), X(0.26, 0.426, 0, 0.118, 0.111, 2.2)], 0.008, 0.03), 1), COAT);
  B.loft("neck", "neck", smooth(dome([span([0.4, 0.567], [0.498, 0.372], 0.084), span([0.475, 0.618], [0.575, 0.452], 0.08), span([0.55, 0.682], [0.638, 0.56], 0.068), span([0.603, 0.708], [0.668, 0.62], 0.056)], 0.03, 0), 1), COAT);
  B.loft("head", "head", smooth([H(-0.03, 0.012, 0.026, 0.034), H(0.02, 0.013, 0.05, 0.064, 2.6), H(0.085, 0.011, 0.05, 0.064, 2.5), H(0.125, -0.004, 0.039, 0.043, 2.5), H(0.215, -0.007, 0.034, 0.037, 2.5), H(0.244, -0.002, 0.017, 0.02, 2)], 1), COAT);
  B.loft("jaw", "jaw", [H(0.03, -0.04, 0.017, 0.036), H(0.1, -0.049, 0.02, 0.04, 2.4), H(0.19, -0.047, 0.0155, 0.029, 2.3), H(0.216, -0.045, 0.015, 0.02, 2)], COAT);
  ears(B, [[-0.008, 0.02, 0.015, 0.07], [0.04, 0.036, 0.015, 0.078], [0.105, 0.016, 0.015, 0.074]], 1);
  B.loft("tail", "tail-2", dome(smooth(along([[-0.04, 0.522, 0, 0.026, 0.034], [-0.13, 0.5045, 0, 0.032, 0.032], [-0.2, 0.4865, 0, 0.029, 0.0285], [-0.26, 0.4795, 0, 0.0255, 0.025], [-0.32, 0.474, 0, 0.0215, 0.021], [-0.372, 0.4765, 0, 0.0175, 0.017], [-0.414, 0.4805, 0, 0.0152, 0.0152]]), 1), 0, 0.012), TAIL);
  for (const [s, sg] of SIDES) {
    B.loft(`thigh-${s}`, `thigh-h${s}`, dome(side(sg, [Y(0.53, -0.012, 0.06, 0.066, 0.032, 2.2), Y(0.45, -0.002, 0.077, 0.098, 0.047, 2.2), Y(0.36, 0.016, 0.084, 0.083, 0.045, 2.2), Y(0.28, 0.033, 0.087, 0.045, 0.033, 2)]), 0.018, 0.016), COAT);
    B.loft(`shank-${s}`, `shank-h${s}`, dome(side(sg, [A(J.stifle, DEG.shank, -0.02, -0.005, 0.043, 0.035, 0.087), A(J.stifle, DEG.shank, 0.08, -0.011, 0.037, 0.03, 0.08), A(J.stifle, DEG.shank, 0.205, -0.014, 0.028, 0.019, 0.068)]), 0.02, 0.01), COAT);
    B.loft(`hind-pastern-${s}`, `metatarsus-h${s}`, side(sg, along([[-0.113, 0.178, Z.hock, 0.026, 0.02], [-0.105, 0.09, Z.hock, 0.02, 0.019], [-0.098, 0.046, Z.hock, 0.021, 0.022], [-0.094, 0.03, Z.hock, 0.016, 0.018]])), COAT);
    B.loft(`paw-h${s}`, `paw-h${s}`, side(sg, along([[-0.103, 0.054, Z.hock, 0.016, 0.018], [-0.096, 0.04, Z.hock, 0.023, 0.026], [-0.07, 0.023, Z.hock, 0.022, 0.032], [-0.038, 0.016, Z.hock, 0.016, 0.033], [-0.02, 0.013, Z.hock, 0.012, 0.027]])), COAT);
    B.loft(`fore-upper-${s}`, `humerus-f${s}`, smooth(dome(side(sg, along([[0.43, 0.535, 0.032, 0.04, 0.03], [0.47, 0.47, 0.066, 0.06, 0.046], [0.47, 0.41, 0.091, 0.06, 0.046], [0.43, 0.34, 0.1, 0.05, 0.032], [0.392, 0.286, 0.092, 0.034, 0.026]])), 0.016, 0.012), 1), COAT);
    B.loft(`forearm-${s}`, `forearm-f${s}`, dome(side(sg, [A(J.elbow, DEG.forearm, -0.026, -0.012, 0.034, 0.026, 0.088), A(J.elbow, DEG.forearm, 0.08, 0, 0.031, 0.026, 0.082), A(J.elbow, DEG.forearm, 0.19, 0.002, 0.025, 0.024, 0.075)]), 0.016, 0.008), COAT);
    B.loft(`fore-pastern-${s}`, `metacarpus-f${s}`, side(sg, along([[0.393, 0.115, Z.carpus, 0.024, 0.024], [0.401, 0.075, Z.carpus, 0.023, 0.024], [0.407, 0.046, Z.carpus, 0.021, 0.024], [0.41, 0.032, Z.carpus, 0.016, 0.018]])), COAT);
    B.loft(`paw-f${s}`, `paw-f${s}`, side(sg, along([[0.4, 0.058, Z.carpus, 0.016, 0.019], [0.407, 0.044, Z.carpus, 0.024, 0.028], [0.432, 0.025, Z.carpus, 0.024, 0.036], [0.47, 0.017, Z.carpus, 0.017, 0.037], [0.495, 0.013, Z.carpus, 0.012, 0.03]])), COAT);
  }
}

/** 🌡️ Regions without a body solid at the coarse scales, each at the size floor of the scale: the nose pad, the tongue in the mouth and one pad slab under each paw. */
function smallRegions(B: Bench, nose: boolean, f: number, onPaws = false): void {
  if (nose) B.list("thermal", { region: "nose-skin" }).ell("th-nose", "head", B.world("head", [0.243 - f / 2, 0.004, 0]), [f / 2, Math.max(0.016, f / 2), Math.max(0.02, f / 2)], NOSE, [HU, HN]);
  B.list("thermal", { region: "tongue" }).loft("th-tongue", "jaw", [H(0.07, -0.033, f / 2, 0.016, 2), H(0.14, -0.033, f / 2, Math.max(0.019, f / 2), 2), H(0.205, -0.033, f / 2, Math.max(0.014, f / 2), 2)], TONGUE);
  B.list("thermal", { region: "pads" });
  for (const [s, sg] of SIDES) for (const [leg, m, zc, bone] of [["h", J.mtp[0], Z.hock, onPaws ? "paw-h" : "metatarsus-h"], ["f", J.mcp[0], Z.carpus, onPaws ? "paw-f" : "metacarpus-f"]] as const) B.ell(`th-pad-${leg}${s}`, bone + s, [m + 0.034, f / 2, sg * zc], [Math.max(0.026, f / 2), f / 2, Math.max(0.022, f / 2)], PAD);
}

function thermal200(B: Bench): void {
  thermalRefs(B);
  smallRegions(B, true, 0.0302, true);
  outlets(B, { nose: [0.2325, 0.004, 0, 0.0152, 0.0152], mouth: [0.21, -0.033, 0.0151, 0.0151], plume: [[0.22, 0.5545, 0.04], [0.38, 0.565, 0.03]] });
}

function structure200(B: Bench): void {
  const M = (bone: string): void => void B.list("structure", { member: bone, part: "member" });
  M("head");
  B.loft("skull", "head", [H(-0.012, 0.01, 0.022, 0.024), H(0.05, 0.013, 0.039, 0.048, 2.5), H(0.125, 0, 0.026, 0.03), H(0.228, -0.002, 0.015, 0.016)], BONE);
  bar(B, "mandible", "jaw", [...B.world("head", [0.045, -0.04, 0])] as V3, [...B.world("head", [0.198, -0.047, 0])] as V3, 0.0152);
  M("neck");
  B.loft("cervical", "neck", smooth(along([[0.61, 0.645, 0, 0.0152, 0.0152], [0.545, 0.548, 0, 0.0152, 0.0152], [0.462, 0.487, 0, 0.0152, 0.0152]]), 2), BONE);
  M("thorax");
  B.loft("thoracic", "thorax", [THORACIC[0]!, THORACIC[1]!, THORACIC[5]!].map(([top, bottom]) => span(top, bottom, 0.0152)), BONE);
  ribs(B, [[0.29, 0.318, 0.096], [0.36, 0.312, 0.096], [0.43, 0.33, 0.082]], 0.03, 10);
  bar(B, "lumbar-column", "lumbar", [0.245, 0.508, 0], [0.055, 0.511, 0], 0.022);
  M("pelvis");
  B.loft("pelvis-girdle", "pelvis", [X(0.09, 0.526, 0, 0.02, 0.062, 3), X(0, 0.492, 0, 0.034, 0.066, 2.6), X(-0.082, 0.468, 0, 0.0155, 0.054, 2.4)], BONE);
  caudals(B, [0.0152, 0.0152, 0.0152, 0.0152]);
  for (const [s] of SIDES) {
    M(`ear-${s}`);
    B.ref(`ear-cartilage-${s}`, `ear-${s}`);
  }
  limbBars(B, 0.0152, true);
  masses(B, [["thorax", 0.03], ["lumbar", 0.023]]);
}

function body500(B: Bench): void {
  B.list("body");
  B.loft("chest", "thorax", smooth(dome([X(0.19, 0.434, 0, 0.113, 0.103, 2.2), X(0.225, 0.429, 0, 0.127, 0.118, 2.2), X(0.33, 0.4225, 0, 0.137, 0.13, 2.3), X(0.43, 0.428, 0, 0.137, 0.115, 2.2), X(0.5, 0.445, 0, 0.09, 0.075, 2)], 0.03, 0.026), 1), COAT);
  B.loft("hindbody", "lumbar", smooth(dome([X(-0.085, 0.486, 0, 0.05, 0.06, 2), X(-0.03, 0.478, 0, 0.07, 0.09, 2.2), X(0.1, 0.4425, 0, 0.1125, 0.1, 2.2), X(0.225, 0.429, 0, 0.127, 0.118, 2.2), X(0.26, 0.426, 0, 0.118, 0.111, 2.2)], 0.018, 0.03), 1), COAT);
  B.loft("neck", "neck", smooth(dome([span([0.4, 0.567], [0.498, 0.372], 0.084), span([0.51, 0.652], [0.607, 0.5], 0.075), span([0.598, 0.702], [0.668, 0.62], 0.056)], 0.03, 0), 1), COAT);
  B.loft("head", "head", smooth(dome([H(-0.02, 0.004, 0.045, 0.045), H(0.045, 0.001, 0.068, 0.068, 2.6), H(0.125, -0.014, 0.05, 0.046, 2.5), H(0.228, -0.014, 0.0385, 0.0385, 2.4)], 0.012, 0.018), 1), COAT);
  B.loft("tail", "tail-2", dome(along([[-0.045, 0.512, 0, 0.0377, 0.0377], [-0.2, 0.4865, 0, 0.032, 0.032], [-0.405, 0.4795, 0, 0.021, 0.021]]), 0, 0.02), TAIL);
  for (const [s, sg] of SIDES) {
    B.loft(`hind-thigh-${s}`, `thigh-h${s}`, dome(side(sg, [Y(0.525, -0.012, 0.064, 0.066, 0.0375, 2.2), Y(0.45, -0.002, 0.077, 0.097, 0.046, 2.2), Y(0.38, 0.012, 0.082, 0.088, 0.046, 2.2), Y(0.28, 0.033, 0.085, 0.046, 0.0375, 2)]), 0.016, 0.016), COAT);
    B.loft(`hind-leg-${s}`, `shank-h${s}`, dome(side(sg, [A(J.stifle, DEG.shank, -0.015, -0.005, 0.042, 0.038, 0.084), A(J.stifle, DEG.shank, 0.1, -0.01, 0.0385, 0.0377, 0.078), A(J.stifle, DEG.shank, 0.205, -0.01, 0.0377, 0.0377, 0.071)]), 0.02, 0.016), COAT);
    B.loft(`hind-foot-${s}`, `metatarsus-h${s}`, dome(side(sg, along([[-0.112, 0.165, 0.071, 0.0377, 0.0377], [-0.104, 0.085, 0.071, 0.0377, 0.0377], [-0.086, 0.04, 0.071, 0.036, 0.038], [-0.036, 0.025, 0.071, 0.025, 0.038]])), 0.014, 0.012), COAT);
    B.loft(`fore-upper-${s}`, `humerus-f${s}`, smooth(dome(side(sg, along([[0.43, 0.53, 0.04, 0.042, 0.04], [0.475, 0.44, 0.088, 0.062, 0.048], [0.395, 0.29, 0.094, 0.04, 0.038]])), 0.016, 0.014), 1), COAT);
    B.loft(`fore-leg-${s}`, `forearm-f${s}`, dome(side(sg, [A(J.elbow, DEG.forearm, -0.02, -0.008, 0.039, 0.0377, 0.086), A(J.elbow, DEG.forearm, 0.185, 0.002, 0.0377, 0.0377, 0.076)]), 0.016, 0.012), COAT);
    B.loft(`fore-foot-${s}`, `metacarpus-f${s}`, dome(side(sg, along([[0.393, 0.112, 0.076, 0.0377, 0.0377], [0.403, 0.06, 0.076, 0.0377, 0.0377], [0.432, 0.034, 0.076, 0.034, 0.039], [0.478, 0.024, 0.076, 0.024, 0.039]])), 0.014, 0.012), COAT);
  }
}

function thermal500(B: Bench): void {
  thermalRefs(B);
  smallRegions(B, false, 0.0752);
  B.list("thermal", { region: "ears" });
  for (const [s, sg] of SIDES) B.ell(`th-ear-${s}`, `ear-${s}`, [0.672, 0.632, sg * 0.034], [0.04, 0.055, 0.0376], EAR);
  const capOf = (id: string, region: string, tag: string, st: St[]): void => void B.list("thermal", { region }).loft(id, "head", st, OUTLET, { tags: ["emitter", tag] });
  B.list("thermal", { region: "nose-skin" }).loft("th-nose", "head", [H(0.172, -0.014, 0.0383, 0.0383, 2.4), H(0.248, -0.014, 0.0383, 0.0383, 2.4)], NOSE);
  capOf("th-breath-nose", "nose-skin", "breath-nose", [H(0.174, -0.012, 0.0386, 0.0386, 2.4), H(0.25, -0.012, 0.0386, 0.0386, 2.4)]);
  capOf("th-breath-mouth", "tongue", "breath-mouth", [H(0.17, -0.017, 0.0386, 0.0386, 2.4), H(0.2485, -0.017, 0.0386, 0.0386, 2.4)]);
  const hp = 0.0376;
  B.list("thermal", { region: "coat-trunk" }).loft("th-plume-back", "thorax", dome([X(0.22, 0.5545 - hp + 0.004, 0, hp, 0.04, 2), X(0.38, 0.565 - hp + 0.004, 0, hp, 0.0376, 2)], 0.02, 0.02), OUTLET, { tags: ["emitter", "plume-back"] });
}

/** 🥢️ The skeleton of the coarsest scale: one bar per bone from joint to joint, each as thick as the scale can show (75 mm, or the drawn width of its limb where that is less). */
function structure500(B: Bench): void {
  const M = (bone: string): void => void B.list("structure", { member: bone, part: "member" });
  const r = 0.0376;
  M("head");
  B.loft("skull", "head", [H(-0.01, 0.005, 0.038, 0.038), H(0.05, 0.004, 0.05, 0.052, 2.5), H(0.228, -0.008, 0.0376, 0.0376, 2.2)], BONE);
  rod(B, "mandible", "jaw", [...B.world("head", [0.05, -0.022, 0])] as V3, [...B.world("head", [0.19, -0.024, 0])] as V3, r);
  rod(B, "cervical", "neck", [AO[0] - 0.012, AO[1] - 0.004, 0], [J.c7[0] + 0.006, J.c7[1] + 0.02, 0], r);
  rod(B, "thoracic", "thorax", [J.c7[0] - 0.01, J.c7[1] + 0.012, 0], [J.t13[0], J.t13[1] + 0.004, 0], r);
  ribs(B, [[0.35, 0.335, 0.085]], 0.0752, 0, "rib-girth");
  rod(B, "lumbar-column", "lumbar", [J.t13[0], J.t13[1], 0], [J.ls[0], J.ls[1], 0], r);
  rod(B, "pelvis-girdle", "pelvis", [0.055, 0.508, 0], [-0.05, 0.482, 0], r);
  rod(B, "caudal-1", "tail-1", [-0.05, 0.528, 0], [J.tail2[0], J.tail2[1], 0], r);
  rod(B, "caudal-2", "tail-2", [J.tail2[0], J.tail2[1], 0], [J.tail3[0] + 0.02, J.tail3[1] + 0.002, 0], 0.0315);
  rod(B, "caudal-3", "tail-3", [J.tail3[0] + 0.02, J.tail3[1] + 0.002, 0], [J.tailTip[0] + 0.05, J.tailTip[1] - 0.002, 0], 0.0376);
  for (const [s, sg] of SIDES) {
    const z = (v: number): number => sg * v;
    rod(B, `ear-cartilage-${s}`, `ear-${s}`, [0.662, 0.676, z(0.03)], [0.684, 0.602, z(0.03)], r);
    rod(B, `scapula-${s}`, `scapula-f${s}`, [J.scapula[0] + 0.02, J.scapula[1] - 0.03, z(0.06)], [J.shoulder[0] - 0.012, J.shoulder[1] + 0.012, z(0.084)], r);
    rod(B, `humerus-${s}`, `humerus-f${s}`, [J.shoulder[0] - 0.012, J.shoulder[1] + 0.005, z(0.088)], [J.elbow[0] + 0.008, J.elbow[1] + 0.012, z(0.09)], r);
    rod(B, `radius-ulna-${s}`, `forearm-f${s}`, [J.elbow[0], J.elbow[1], z(0.086)], [J.carpus[0], J.carpus[1], z(0.076)], r);
    rod(B, `metacarpals-${s}`, `metacarpus-f${s}`, [J.carpus[0] - 0.002, 0.12, z(0.076)], [J.mcp[0] - 0.004, 0.042, z(0.076)], r);
    rod(B, `phalanges-f${s}`, `paw-f${s}`, [J.mcp[0] - 0.004, 0.039, z(0.076)], [J.mcp[0] + 0.073, 0.039, z(0.076)], r);
    rod(B, `femur-${s}`, `thigh-h${s}`, [J.hip[0], J.hip[1], z(0.07)], [J.stifle[0] - 0.004, J.stifle[1] + 0.012, z(0.082)], r);
    rod(B, `tibia-${s}`, `shank-h${s}`, [J.stifle[0] - 0.012, J.stifle[1] - 0.008, z(0.082)], [J.hock[0] + 0.004, J.hock[1] + 0.008, z(0.071)], r);
    rod(B, `metatarsals-${s}`, `metatarsus-h${s}`, [J.hock[0], J.hock[1], z(0.07)], [J.mtp[0] - 0.004, 0.05, z(0.071)], r);
    rod(B, `phalanges-h${s}`, `paw-h${s}`, [J.mtp[0] - 0.004, 0.039, z(0.071)], [J.mtp[0] + 0.073, 0.039, z(0.071)], r);
  }
  masses(B, [["thorax", 0.0376], ["lumbar", 0.0376]]);
}

if (import.meta.main) {
  const scale = Number(process.argv[2]);
  const B = new Bench(scale);
  ({ 100: body100, 200: body200, 500: body500 } as Record<number, (b: Bench) => void>)[scale]!(B);
  ({ 100: thermal100, 200: thermal200, 500: thermal500 } as Record<number, (b: Bench) => void>)[scale]!(B);
  ({ 100: structure100, 200: structure200, 500: structure500 } as Record<number, (b: Bench) => void>)[scale]!(B);
  const ps = B.probe();
  const min = [0, 1, 2].map((d) => Math.min(...ps.map((p) => p.min[d]!)));
  const max = [0, 1, 2].map((d) => Math.max(...ps.map((p) => p.max[d]!)));
  console.log(`1:${scale} body ${B.lists.body.length} thermal ${B.lists.thermal.length} structure ${B.lists.structure.length}`);
  console.log("extent", min.map((v) => v.toFixed(4)).join(" "), "..", max.map((v) => v.toFixed(4)).join(" "), "size", max.map((v, i) => (v - min[i]!).toFixed(4)).join(" "));
  for (const l of ["body", "thermal", "structure"] as const) for (const p of B.probe(l)) if (p.volume <= 0) console.log("INSIDE OUT", l, p.id, p.volume);
  if (process.argv.includes("--write")) B.save(DECISIONS[scale]!);
}
