/** 🐐️ Round 3 goat, 1:100, 1:200 and 1:500: each scale stated anew with its own merged masses and fewer sections (smallest feature 15, 30 and 75 mm). World metres, default pose, near side z < 0.
 * Usage: bun r3-goat-coarse.ts <100|200|500> [--write]
 * @see ./r3-goat-50.ts
 * @see ./r3-animals-goat.md */
import { A, AO, Bench, CLAW, COAT, HEAD_DEG, HORN, PALE, SKIN, X, Y, along, dome, far, near, smooth, span, vec, type Look, type St } from "./r3-goat-kit.ts";
import { BONE, hoop, outlets, pin, plumePatch, thermalRefs } from "./r3-goat-50-lists.ts";
import { DECISIONS } from "./r3-goat-decisions.ts";
import { type Json, type V3 } from "./volumes-lib.ts";

const H = (s: number, off: number, ra: number, rb: number, e = 2.3): St => A(AO, HEAD_DEG, s, off, ra, rb, 0, e);
const SIDES = [["n", -1], ["f", 1]] as const;
const side = (sg: number, st: St[]): St[] => (sg < 0 ? near(st) : far(near(st)));
const ST: [number, number] = [0.118, 0.462];
const HK: [number, number] = [0.014, 0.25];
const SC: [number, number] = [0.636, 0.745];
const SH: [number, number] = [0.728, 0.596];
const EL: [number, number] = [0.655, 0.438];
const CA: [number, number] = [0.664, 0.226];
const HIP: [number, number] = [0, 0.64];
const FH: [number, number] = [0.029, 0.079];
const FF: [number, number] = [0.659, 0.079];

function ears(B: Bench, rows: readonly (readonly [number, number, number])[], per: number): void {
  for (const [s, sg] of SIDES) {
    const e0: V3 = [1.0215, 0.9026, 0.046];
    const d = vec.unit([0.25, 0.22, 0.943]);
    const m = vec.unit(vec.sub([1, -0.3, 0], vec.mul(d, vec.dot([1, -0.3, 0], d))));
    const q = vec.cross(d, m);
    const ear: St[] = rows.map(([t, rq, rm]) => ({ c: vec.add(e0, vec.mul(d, t)), a: vec.mul(q, rq), b: vec.mul(m, rm), e: 2 }));
    B.loft(`ear-${s}`, `ear-${s}`, smooth(sg < 0 ? near(ear).map((x) => ({ ...x, a: [x.a[0], x.a[1], -x.a[2]], b: [x.b[0], x.b[1], -x.b[2]] })) : ear, per), SKIN);
  }
}

function horns(B: Bench, pts: readonly (readonly [number, number, number, number, number])[], per: number): void {
  for (const [s, sg] of SIDES) B.loft(`horn-${s}`, "head", smooth(side(sg, along(pts)), per), HORN);
}

/** 🦴️ A straight bone bar between two world points. */
const bar = (B: Bench, id: string, bone: string, p0: V3, p1: V3, r0: number, r1 = r0): void => {
  B.list("structure", { member: bone, part: "member" });
  B.cap(id, bone, p0, p1, r0, r1, BONE);
};

/** 🥢️ A plain round rod between two world points (two sections, flat ends): the bar of the coarsest scale. */
const rod = (B: Bench, id: string, bone: string, p0: V3, p1: V3, r: number): void => {
  B.list("structure", { member: bone, part: "member" });
  B.loft(id, bone, along([[p0[0], p0[1], Math.abs(p0[2]), r, r], [p1[0], p1[1], Math.abs(p1[2]), r, r]]).map((st) => (p0[2] < 0 ? { ...st, c: [st.c[0], st.c[1], -st.c[2]] as V3, a: [st.a[0], st.a[1], -st.a[2]] as V3, b: [st.b[0], st.b[1], -st.b[2]] as V3 } : st)), BONE);
};

function masses(B: Bench, r: number): void {
  for (const [bone, k] of [["thorax", 1.25], ["lumbar", 1]] as const) {
    const m = (B.asset.rig.bones as Json[]).find((b) => b.id === bone).mass;
    B.list("structure", { member: bone, part: "mass" });
    B.ell(`mass-${bone}`, bone, B.world(bone, [m.comX, m.comY, 0]), [r * k, r * k, r * k], { material: "bone", tone: 0.3 });
  }
}

function body100(B: Bench): void {
  B.list("body");
  B.loft("chest", "thorax", smooth([X(0.36, 0.6, 0, 0.13, 0.12, 2.1), X(0.42, 0.585, 0, 0.168, 0.142, 2.1), X(0.5, 0.585, 0, 0.168, 0.127, 2.1), X(0.58, 0.587, 0, 0.171, 0.113, 2.1), X(0.65, 0.593, 0, 0.167, 0.1, 2.1), X(0.71, 0.6, 0, 0.15, 0.085, 2), X(0.76, 0.6, 0, 0.115, 0.065, 2), X(0.787, 0.585, 0, 0.05, 0.035, 2)], 1), COAT);
  B.loft("hindbody", "lumbar", smooth([X(0.02, 0.63, 0, 0.1, 0.08), X(0.1, 0.603, 0, 0.145, 0.118), X(0.17, 0.588, 0, 0.166, 0.142, 2.1), X(0.24, 0.58, 0, 0.176, 0.154, 2.1), X(0.31, 0.576, 0, 0.176, 0.155, 2.1), X(0.38, 0.578, 0, 0.172, 0.148, 2.1), X(0.45, 0.583, 0, 0.166, 0.133, 2.1), X(0.5, 0.585, 0, 0.165, 0.124, 2.1), X(0.54, 0.587, 0, 0.12, 0.09, 2.1)], 1), COAT);
  B.loft("croup", "pelvis", smooth([X(0.16, 0.665, 0, 0.085, 0.07, 2.4), X(0.12, 0.663, 0, 0.096, 0.088, 2.6), X(0.03, 0.66, 0, 0.086, 0.083, 2.4), X(-0.05, 0.65, 0, 0.074, 0.068, 2.2), X(-0.095, 0.64, 0, 0.052, 0.048, 2), X(-0.114, 0.635, 0, 0.016, 0.016, 2)], 1), COAT);
  B.ell("udder", "pelvis", [0.072, 0.418, 0], [0.056, 0.052, 0.05], PALE);
  B.loft("neck", "neck", smooth([span([0.63, 0.745], [0.745, 0.49], 0.056), span([0.69, 0.762], [0.785, 0.548], 0.06), span([0.793, 0.825], [0.858, 0.688], 0.043), span([0.905, 0.908], [0.945, 0.795], 0.038), span([0.99, 0.962], [1.01, 0.866], 0.04)], 1), COAT);
  B.loft("head", "head", smooth([H(-0.03, 0.005, 0.035, 0.034), H(0, -0.004, 0.062, 0.048, 2.6), H(0.04, -0.008, 0.07, 0.057, 2.8), H(0.08, -0.008, 0.064, 0.055, 2.6), H(0.12, -0.006, 0.052, 0.042), H(0.16, -0.004, 0.04, 0.033), H(0.2, -0.004, 0.031, 0.027), H(0.235, -0.004, 0.024, 0.023), H(0.246, -0.003, 0.012, 0.014)], 1), COAT);
  const chin = B.world("head", [0.172, -0.04, 0]);
  B.loft("beard", "head", [Y(chin[1], chin[0], 0, 0.014, 0.011), Y(chin[1] - 0.04, chin[0] - 0.007, 0, 0.017, 0.012), Y(chin[1] - 0.085, chin[0] - 0.017, 0, 0.0075, 0.0075)], PALE);
  horns(B, [[1.041, 0.905, 0.024, 0.0155, 0.0115], [1.045, 0.975, 0.034, 0.012, 0.009], [1.016, 1.045, 0.055, 0.008, 0.0062], [0.957, 1.086, 0.083, 0.003, 0.003]], 1);
  ears(B, [[0, 0.012, 0.0085], [0.035, 0.03, 0.0078], [0.09, 0.027, 0.0078], [0.128, 0.007, 0.0075]], 1);
  B.loft("tail-root", "tail-1", [A([-0.048, 0.722], 70, -0.03, 0, 0.016, 0.02), A([-0.048, 0.722], 70, 0.078, 0, 0.013, 0.019)], COAT);
  B.loft("tail-tip", "tail-2", [A([-0.024, 0.7878], 60, -0.008, 0, 0.013, 0.019), A([-0.024, 0.7878], 60, 0.035, 0, 0.0105, 0.015), A([-0.024, 0.7878], 60, 0.064, 0, 0.0075, 0.0075)], COAT);
  for (const [s, sg] of SIDES) {
    B.loft(`thigh-${s}`, `thigh-h${s}`, smooth(dome(side(sg, [Y(0.7, 0.006, 0.066, 0.085, 0.034, 2.2), Y(0.63, 0.015, 0.072, 0.118, 0.045, 2.2), Y(0.55, 0.045, 0.08, 0.103, 0.045, 2.2), Y(0.48, 0.085, 0.087, 0.064, 0.039, 2.1), Y(0.44, 0.106, 0.089, 0.04, 0.032, 2)]), 0.02, 0.02), 1), COAT);
    B.loft(`gaskin-${s}`, `shank-h${s}`, smooth(dome(side(sg, [A(ST, -116.13, -0.02, -0.005, 0.045, 0.034, 0.088), A(ST, -116.13, 0.04, -0.009, 0.05, 0.035, 0.086), A(ST, -116.13, 0.12, -0.01, 0.034, 0.026, 0.08), A(ST, -116.13, 0.19, -0.016, 0.031, 0.019, 0.074), A(ST, -116.13, 0.238, -0.012, 0.027, 0.018, 0.071)]), 0.03, 0.012), 1), COAT);
    B.loft(`cannon-h${s}`, `metatarsus-h${s}`, smooth(dome(side(sg, [A(HK, -84.99, -0.038, -0.016, 0.012, 0.012, 0.071), A(HK, -84.99, -0.004, -0.004, 0.024, 0.019, 0.071), A(HK, -84.99, 0.03, 0, 0.0165, 0.014, 0.071), A(HK, -84.99, 0.1, 0, 0.015, 0.013, 0.072), A(HK, -84.99, 0.168, 0, 0.019, 0.017, 0.072), A(HK, -84.99, 0.185, 0, 0.016, 0.015, 0.072)]), 0.006, 0.01), 1), COAT);
    B.loft(`shoulder-${s}`, `scapula-f${s}`, smooth(dome(side(sg, [A(SC, -58.31, 0.022, -0.003, 0.04, 0.016, 0.038), A(SC, -58.31, 0.09, -0.004, 0.056, 0.027, 0.068), A(SC, -58.31, 0.15, -0.005, 0.052, 0.032, 0.082), A(SC, -58.31, 0.195, 0, 0.03, 0.027, 0.086)]), 0.02, 0.014), 1), COAT);
    B.loft(`arm-${s}`, `humerus-f${s}`, smooth(dome(side(sg, [A(SH, -114.8, -0.01, -0.005, 0.04, 0.03, 0.086), A(SH, -114.8, 0.06, -0.02, 0.055, 0.034, 0.088), A(SH, -114.8, 0.13, -0.019, 0.05, 0.031, 0.089), A(SH, -114.8, 0.19, -0.009, 0.032, 0.025, 0.088)]), 0.02, 0.014), 1), COAT);
    B.loft(`forearm-${s}`, `forearm-f${s}`, smooth(dome(side(sg, [A(EL, -87.57, -0.02, -0.004, 0.036, 0.027, 0.088), A(EL, -87.57, 0.04, -0.001, 0.035, 0.027, 0.086), A(EL, -87.57, 0.12, 0, 0.025, 0.02, 0.08), A(EL, -87.57, 0.19, 0, 0.021, 0.019, 0.076), A(EL, -87.57, 0.215, 0.002, 0.023, 0.022, 0.075), A(EL, -87.57, 0.232, 0.001, 0.02, 0.02, 0.075)]), 0.02, 0.01), 1), COAT);
    B.loft(`cannon-f${s}`, `metacarpus-f${s}`, smooth(dome(side(sg, [A(CA, -91.95, -0.008, 0, 0.02, 0.02, 0.075), A(CA, -91.95, 0.035, 0, 0.0155, 0.0145, 0.075), A(CA, -91.95, 0.11, 0, 0.0148, 0.014, 0.075), A(CA, -91.95, 0.147, 0, 0.019, 0.018, 0.075), A(CA, -91.95, 0.162, 0, 0.016, 0.016, 0.075)]), 0.01, 0.01), 1), COAT);
    for (const [leg, F, zc] of [["h", FH, 0.072], ["f", FF, 0.075]] as const) {
      B.loft(`pastern-${leg}${s}`, `hoof-${leg}${s}`, dome(side(sg, [A(F, -52.33, -0.006, 0, 0.017, 0.016, zc), A(F, -52.33, 0.03, 0, 0.0155, 0.017, zc), A(F, -52.33, 0.06, 0, 0.014, 0.02, zc)]), 0.01, 0.008), COAT);
      const toe = F[0] + 0.061;
      B.loft(`hoof-${leg}${s}`, `hoof-${leg}${s}`, side(sg, [Y(0, toe - 0.0285, zc, 0.0285, 0.023, 2.6), Y(0.016, toe - 0.03, zc, 0.027, 0.024, 2.6), Y(0.034, toe - 0.038, zc, 0.015, 0.02, 2.3)]), CLAW);
    }
  }
}

function structure100(B: Bench): void {
  const M = (bone: string): void => void B.list("structure", { member: bone, part: "member" });
  M("head");
  B.loft("skull", "head", smooth([H(-0.022, 0.008, 0.026, 0.028), H(0.03, 0.012, 0.043, 0.048, 2.6), H(0.08, 0.01, 0.038, 0.05, 2.5), H(0.15, 0.006, 0.025, 0.026), H(0.232, 0.004, 0.011, 0.013)], 1), BONE);
  M("jaw");
  B.loft("mandible", "jaw", [H(0.02, -0.03, 0.02, 0.032), H(0.06, -0.046, 0.02, 0.03), H(0.14, -0.033, 0.011, 0.02), H(0.215, -0.019, 0.0075, 0.01)], BONE);
  M("neck");
  B.loft("cervical", "neck", smooth(along([[0.982, 0.893, 0, 0.017, 0.021], [0.878, 0.778, 0, 0.019, 0.023], [0.77, 0.67, 0, 0.022, 0.026], [0.7, 0.655, 0, 0.022, 0.026]]), 2), BONE);
  M("thorax");
  B.loft("thoracic", "thorax", smooth([span([0.715, 0.69], [0.705, 0.633], 0.013), span([0.65, 0.75], [0.645, 0.645], 0.012), span([0.5, 0.742], [0.5, 0.672], 0.012), span([0.365, 0.738], [0.365, 0.692], 0.014)], 1), BONE);
  ([[0.43, 0.567, 0.153, 0.139], [0.495, 0.57, 0.152, 0.127], [0.56, 0.572, 0.154, 0.116], [0.62, 0.575, 0.154, 0.106], [0.675, 0.582, 0.147, 0.094]] as const).forEach(([x, yc, ry, rz], i) => hoop(B, `rib-${i + 1}`, x, yc, ry, rz, 0.016, 0.016, { member: "thorax", part: "member" }));
  B.loft("sternum", "thorax", [X(0.5, 0.43, 0, 0.01, 0.015), X(0.66, 0.442, 0, 0.012, 0.016), X(0.76, 0.515, 0, 0.011, 0.011)], BONE);
  M("lumbar");
  B.loft("lumbar-column", "lumbar", [X(0.385, 0.712, 0, 0.024, 0.05, 3), X(0.24, 0.714, 0, 0.025, 0.058, 3), X(0.1, 0.713, 0, 0.024, 0.05, 3)], BONE);
  M("pelvis");
  B.loft("pelvis-girdle", "pelvis", smooth([X(0.132, 0.708, 0, 0.016, 0.078, 3), X(0.07, 0.7, 0, 0.03, 0.07, 2.6), X(0, 0.668, 0, 0.04, 0.075, 2.6), X(-0.05, 0.662, 0, 0.026, 0.064, 2.6), X(-0.09, 0.664, 0, 0.016, 0.058, 2.4)], 1), BONE);
  bar(B, "caudal-1", "tail-1", [-0.05, 0.716, 0], [-0.0245, 0.7865, 0], 0.0085, 0.0078);
  bar(B, "caudal-2", "tail-2", [-0.024, 0.7878, 0], [0.002, 0.833, 0], 0.0078, 0.0075);
  for (const [s, sg] of SIDES) {
    M(`ear-${s}`);
    B.ref(`ear-cartilage-${s}`, `ear-${s}`);
    M(`scapula-f${s}`);
    B.loft(`scapula-${s}`, `scapula-f${s}`, side(sg, [A(SC, -58.31, 0.026, -0.004, 0.032, 0.0078, 0.043, 2.6), A(SC, -58.31, 0.1, -0.002, 0.026, 0.0078, 0.067, 2.2), A(SC, -58.31, 0.172, 0, 0.017, 0.012, 0.085)]), BONE);
    const z = (v: number): number => sg * v;
    bar(B, `humerus-${s}`, `humerus-f${s}`, [0.728, 0.596, z(0.086)], [0.655, 0.438, z(0.088)], 0.014, 0.013);
    bar(B, `radius-ulna-${s}`, `forearm-f${s}`, [0.653, 0.47, z(0.088)], [0.664, 0.226, z(0.075)], 0.013, 0.014);
    bar(B, `metacarpal-${s}`, `metacarpus-f${s}`, [0.664, 0.226, z(0.075)], [0.659, 0.079, z(0.075)], 0.011, 0.0125);
    bar(B, `femur-${s}`, `thigh-h${s}`, [0, 0.64, z(0.07)], [0.118, 0.462, z(0.088)], 0.016, 0.015);
    bar(B, `tibia-${s}`, `shank-h${s}`, [0.118, 0.462, z(0.088)], [0.014, 0.25, z(0.071)], 0.015, 0.012);
    bar(B, `metatarsal-${s}`, `metatarsus-h${s}`, [0.003, 0.28, z(0.071)], [0.029, 0.079, z(0.072)], 0.011, 0.0125);
    bar(B, `phalanges-h${s}`, `hoof-h${s}`, [0.031, 0.076, z(0.072)], [0.076, 0.014, z(0.072)], 0.0095, 0.0085);
    bar(B, `phalanges-f${s}`, `hoof-f${s}`, [0.661, 0.076, z(0.075)], [0.706, 0.014, z(0.075)], 0.0095, 0.0085);
  }
  for (const [bone, at, h] of [["lumbar", [0.108, 0.712], 0.02], ["thorax", [0.375, 0.712], 0.02], ["neck", [0.7, 0.655], 0.024], ["head", AO, 0.024], ["jaw", [1.0077, 0.8679], 0.036]] as const) pin(B, bone, at, 0, h, 0.009);
  for (const [s, sg] of SIDES) for (const [bone, at, z] of [["thigh-h", HIP, 0.07], ["shank-h", ST, 0.088], ["metatarsus-h", HK, 0.071], ["hoof-h", FH, 0.072], ["humerus-f", SH, 0.086], ["forearm-f", EL, 0.088], ["metacarpus-f", CA, 0.075], ["hoof-f", FF, 0.075]] as const) pin(B, bone + s, at, sg * z, /^(meta|hoof)/.test(bone) ? 0.0085 : 0.017, 0.008);
  masses(B, 0.027);
}

function body200(B: Bench): void {
  B.list("body");
  B.loft("chest", "thorax", smooth([X(0.38, 0.59, 0, 0.15, 0.13, 2.1), X(0.47, 0.585, 0, 0.168, 0.132, 2.1), X(0.58, 0.587, 0, 0.171, 0.113, 2.1), X(0.68, 0.597, 0, 0.16, 0.092, 2), X(0.77, 0.595, 0, 0.1, 0.058, 2), X(0.79, 0.585, 0, 0.04, 0.03, 2)], 1), COAT);
  B.loft("hindbody", "lumbar", smooth([X(-0.112, 0.64, 0, 0.03, 0.03), X(-0.07, 0.645, 0, 0.075, 0.062), X(0.02, 0.64, 0, 0.1, 0.085, 2.2), X(0.12, 0.6, 0, 0.152, 0.122, 2.2), X(0.24, 0.58, 0, 0.176, 0.154, 2.1), X(0.36, 0.578, 0, 0.174, 0.15, 2.1), X(0.44, 0.585, 0, 0.14, 0.115, 2.1)], 1), COAT);
  B.loft("neck", "neck", smooth([span([0.64, 0.748], [0.75, 0.5], 0.056), span([0.72, 0.778], [0.805, 0.585], 0.055), span([0.85, 0.866], [0.9, 0.748], 0.04), span([0.99, 0.962], [1.01, 0.866], 0.04)], 1), COAT);
  B.loft("head", "head", smooth([H(-0.03, 0.005, 0.035, 0.036), H(0.03, -0.007, 0.07, 0.056, 2.7), H(0.1, -0.007, 0.058, 0.048, 2.5), H(0.18, -0.004, 0.036, 0.03), H(0.243, -0.004, 0.02, 0.02)], 1), COAT);
  horns(B, [[1.041, 0.905, 0.026, 0.016, 0.0155], [1.04, 0.99, 0.038, 0.014, 0.013], [0.957, 1.082, 0.083, 0.008, 0.008]], 2);
  ears(B, [[0, 0.016, 0.015], [0.05, 0.03, 0.015], [0.126, 0.012, 0.012]], 1);
  B.loft("tail", "tail-1", [A([-0.048, 0.722], 70, -0.03, 0, 0.018, 0.02), A([-0.048, 0.722], 70, 0.06, -0.002, 0.017, 0.019), A([-0.048, 0.722], 70, 0.127, -0.012, 0.015, 0.015)], COAT);
  for (const [s, sg] of SIDES) {
    B.loft(`thigh-${s}`, `thigh-h${s}`, dome(side(sg, [Y(0.69, 0.008, 0.068, 0.095, 0.036, 2.2), Y(0.6, 0.022, 0.075, 0.116, 0.046, 2.2), Y(0.5, 0.072, 0.085, 0.076, 0.041, 2.1), Y(0.44, 0.106, 0.089, 0.04, 0.032, 2)]), 0.02, 0.02), COAT);
    B.loft(`gaskin-${s}`, `shank-h${s}`, dome(side(sg, [A(ST, -116.13, -0.02, -0.006, 0.046, 0.034, 0.088), A(ST, -116.13, 0.1, -0.01, 0.038, 0.028, 0.082), A(ST, -116.13, 0.236, -0.006, 0.022, 0.018, 0.071)]), 0.03, 0.012), COAT);
    B.loft(`cannon-h${s}`, `metatarsus-h${s}`, dome(side(sg, [A(HK, -84.99, -0.025, -0.01, 0.018, 0.016, 0.071), A(HK, -84.99, 0.06, 0, 0.0155, 0.015, 0.072), A(HK, -84.99, 0.172, 0, 0.018, 0.017, 0.072)]), 0.008, 0.008), COAT);
    B.loft(`shoulder-${s}`, `humerus-f${s}`, smooth(dome(side(sg, along([[0.655, 0.712, 0.046, 0.036, 0.016], [0.7, 0.64, 0.07, 0.058, 0.028], [0.712, 0.57, 0.085, 0.056, 0.033], [0.675, 0.49, 0.088, 0.048, 0.03], [0.655, 0.44, 0.088, 0.034, 0.026]])), 0.016, 0.014), 1), COAT);
    B.loft(`forearm-${s}`, `forearm-f${s}`, dome(side(sg, [A(EL, -87.57, -0.02, -0.003, 0.036, 0.027, 0.088), A(EL, -87.57, 0.1, 0, 0.027, 0.022, 0.082), A(EL, -87.57, 0.22, 0.001, 0.022, 0.021, 0.075)]), 0.02, 0.008), COAT);
    B.loft(`cannon-f${s}`, `metacarpus-f${s}`, dome(side(sg, [A(CA, -91.95, -0.004, 0, 0.019, 0.019, 0.075), A(CA, -91.95, 0.07, 0, 0.0155, 0.015, 0.075), A(CA, -91.95, 0.147, 0, 0.018, 0.017, 0.075)]), 0.008, 0.008), COAT);
    for (const [leg, F, zc] of [["h", FH, 0.072], ["f", FF, 0.075]] as const) B.loft(`hoof-${leg}${s}`, `hoof-${leg}${s}`, side(sg, [Y(0.085, F[0] - 0.002, zc, 0.016, 0.016), Y(0.04, F[0] + 0.022, zc, 0.019, 0.02, 2.2), Y(0.02, F[0] + 0.032, zc, 0.027, 0.023, 2.4), Y(0, F[0] + 0.032, zc, 0.029, 0.023, 2.6)]), CLAW);
  }
}

function structure200(B: Bench): void {
  const M = (bone: string): void => void B.list("structure", { member: bone, part: "member" });
  M("head");
  B.loft("skull", "head", [H(-0.02, 0.008, 0.028, 0.03), H(0.05, 0.01, 0.042, 0.048, 2.5), H(0.228, 0.004, 0.015, 0.015)], BONE);
  bar(B, "mandible", "jaw", [0.9772, 0.8415, 0], [1.1128, 0.753, 0], 0.016, 0.015);
  M("neck");
  B.loft("cervical", "neck", smooth(along([[0.982, 0.893, 0, 0.017, 0.021], [0.86, 0.757, 0, 0.02, 0.024], [0.7, 0.655, 0, 0.022, 0.026]]), 2), BONE);
  M("thorax");
  B.loft("thoracic", "thorax", [span([0.71, 0.7], [0.705, 0.633], 0.016), span([0.645, 0.75], [0.64, 0.645], 0.015), span([0.365, 0.738], [0.365, 0.69], 0.016)], BONE);
  ([[0.45, 0.568, 0.152, 0.135], [0.56, 0.572, 0.153, 0.116], [0.66, 0.58, 0.148, 0.097]] as const).forEach(([x, yc, ry, rz], i) => hoop(B, `rib-${i + 1}`, x, yc, ry, rz, 0.03, 0.024, { member: "thorax", part: "member" }));
  bar(B, "lumbar-column", "lumbar", [0.385, 0.712, 0], [0.1, 0.713, 0], 0.024);
  M("pelvis");
  B.loft("pelvis-girdle", "pelvis", [X(0.132, 0.706, 0, 0.017, 0.076, 3), X(0, 0.672, 0, 0.04, 0.074, 2.6), X(-0.085, 0.662, 0, 0.016, 0.04, 2.4)], BONE);
  bar(B, "caudal-1", "tail-1", [-0.048, 0.722, 0], [-0.026, 0.782, 0], 0.015);
  bar(B, "caudal-2", "tail-2", [-0.022, 0.79, 0], [-0.006, 0.822, 0], 0.015);
  for (const [s, sg] of SIDES) {
    const z = (v: number): number => sg * v;
    M(`ear-${s}`);
    B.ref(`ear-cartilage-${s}`, `ear-${s}`);
    bar(B, `scapula-${s}`, `scapula-f${s}`, [0.656, 0.708, z(0.052)], [0.728, 0.596, z(0.085)], 0.016, 0.015);
    bar(B, `humerus-${s}`, `humerus-f${s}`, [0.728, 0.596, z(0.086)], [0.655, 0.438, z(0.088)], 0.016, 0.015);
    bar(B, `radius-ulna-${s}`, `forearm-f${s}`, [0.655, 0.438, z(0.088)], [0.664, 0.226, z(0.075)], 0.015);
    bar(B, `metacarpal-${s}`, `metacarpus-f${s}`, [0.664, 0.226, z(0.075)], [0.659, 0.079, z(0.075)], 0.015);
    bar(B, `femur-${s}`, `thigh-h${s}`, [0, 0.64, z(0.07)], [0.118, 0.462, z(0.088)], 0.017, 0.016);
    bar(B, `tibia-${s}`, `shank-h${s}`, [0.118, 0.462, z(0.088)], [0.014, 0.25, z(0.071)], 0.016, 0.015);
    bar(B, `metatarsal-${s}`, `metatarsus-h${s}`, [0.014, 0.25, z(0.071)], [0.029, 0.079, z(0.072)], 0.015);
    bar(B, `phalanges-h${s}`, `hoof-h${s}`, [0.031, 0.076, z(0.072)], [0.07, 0.02, z(0.072)], 0.015);
    bar(B, `phalanges-f${s}`, `hoof-f${s}`, [0.661, 0.076, z(0.075)], [0.7, 0.02, z(0.075)], 0.015);
  }
  masses(B, 0.026);
}

function body500(B: Bench): void {
  B.list("body");
  B.loft("chest", "thorax", [X(0.4, 0.59, 0, 0.15, 0.125, 2.1), X(0.52, 0.587, 0, 0.17, 0.125, 2.1), X(0.64, 0.592, 0, 0.168, 0.108, 2.1), X(0.75, 0.6, 0, 0.12, 0.072, 2), X(0.79, 0.59, 0, 0.045, 0.04, 2)], COAT);
  B.loft("hindbody", "lumbar", smooth([X(-0.113, 0.636, 0, 0.025, 0.025), X(-0.095, 0.64, 0, 0.062, 0.055), X(-0.04, 0.642, 0, 0.098, 0.085, 2.2), X(0.07, 0.616, 0, 0.14, 0.118, 2.2), X(0.24, 0.58, 0, 0.176, 0.152, 2.1), X(0.4, 0.582, 0, 0.168, 0.14, 2.1), X(0.46, 0.587, 0, 0.13, 0.11, 2.1)], 1), COAT);
  B.loft("neck", "neck", [span([0.64, 0.748], [0.76, 0.51], 0.056), span([0.8, 0.83], [0.86, 0.69], 0.044), span([0.99, 0.962], [1.01, 0.866], 0.04)], COAT);
  B.loft("head", "head", [H(-0.03, 0.002, 0.04, 0.04), H(0.04, -0.007, 0.07, 0.056, 2.6), H(0.15, -0.005, 0.045, 0.04), H(0.244, -0.004, 0.022, 0.022)], COAT);
  horns(B, [[1.041, 0.9, 0.034, 0.0375, 0.0375], [1.03, 1.0, 0.05, 0.026, 0.026], [0.96, 1.078, 0.085, 0.012, 0.012]], 0);
  for (const [s, sg] of SIDES) B.loft(`ear-${s}`, `ear-${s}`, side(sg, along([[1.02, 0.9, 0.03, 0.038, 0.038], [1.036, 0.916, 0.1, 0.038, 0.038], [1.052, 0.932, 0.168, 0.016, 0.016]])), SKIN);
  B.loft("tail", "tail-1", [A([-0.048, 0.722], 70, -0.035, 0, 0.0375, 0.0375), A([-0.048, 0.722], 70, 0.06, -0.003, 0.036, 0.036), A([-0.048, 0.722], 70, 0.122, -0.012, 0.02, 0.02)], COAT);
  for (const [s, sg] of SIDES) {
    B.loft(`hind-upper-${s}`, `shank-h${s}`, dome(side(sg, along([[0.04, 0.585, 0.075, 0.08, 0.045], [0.105, 0.47, 0.088, 0.058, 0.042], [0.06, 0.345, 0.08, 0.042, 0.0375], [0.012, 0.245, 0.072, 0.0375, 0.0375]])), 0.05, 0), COAT);
    B.loft(`hind-lower-${s}`, `metatarsus-h${s}`, side(sg, [Y(0.275, 0.01, 0.072, 0.0375, 0.0375), Y(0.09, 0.03, 0.072, 0.0375, 0.0375), Y(0, 0.05, 0.072, 0.042, 0.0375, 2.4)]), COAT);
    B.loft(`fore-upper-${s}`, `forearm-f${s}`, dome(side(sg, along([[0.695, 0.6, 0.075, 0.06, 0.04], [0.668, 0.47, 0.088, 0.048, 0.0375], [0.664, 0.215, 0.075, 0.0375, 0.0375]])), 0.05, 0), COAT);
    B.loft(`fore-lower-${s}`, `metacarpus-f${s}`, side(sg, [Y(0.25, 0.664, 0.075, 0.0375, 0.0375), Y(0.09, 0.662, 0.075, 0.0375, 0.0375), Y(0, 0.68, 0.075, 0.042, 0.0375, 2.4)]), COAT);
  }
}

function thermal500(B: Bench): void {
  thermalRefs(B);
  B.list("thermal", { region: "hooves" });
  for (const [s, sg] of SIDES) for (const [leg, x] of [["h", 0.05], ["f", 0.68]] as const) B.loft(`th-hoof-${leg}${s}`, leg === "h" ? `metatarsus-h${s}` : `metacarpus-f${s}`, side(sg, [Y(0, x, leg === "h" ? 0.072 : 0.075, 0.042, 0.0375, 2.4), Y(0.075, x - 0.012, leg === "h" ? 0.072 : 0.075, 0.0375, 0.0375, 2.2)]), CLAW);
  const cap = (id: string, tag: string, st: St[]): void => void B.list("thermal", { region: "face" }).loft(id, "head", st, { material: "bare-skin", tone: 0.3, role: "detail" }, { tags: ["emitter", tag] });
  cap("th-breath-nose", "breath-nose", [H(0.16, -0.005, 0.0455, 0.0415), H(0.2, -0.0045, 0.0355, 0.033), H(0.247, -0.004, 0.025, 0.025)]);
  cap("th-breath-mouth", "breath-mouth", [H(0.06, -0.033, 0.038, 0.054, 2.4), H(0.12, -0.021, 0.038, 0.05, 2.4), H(0.175, -0.008, 0.038, 0.039)]);
  plumePatch(B, 0.004);
}

function structure500(B: Bench): void {
  const M = (bone: string): void => void B.list("structure", { member: bone, part: "member" });
  const R = 0.0375;
  M("head");
  B.loft("skull", "head", [H(-0.01, 0.006, 0.0375, 0.0375), H(0.06, 0.006, 0.045, 0.046, 2.4), H(0.215, 0.002, 0.0375, 0.0375)], BONE);
  rod(B, "mandible", "jaw", [1.025, 0.85, 0], [1.09, 0.776, 0], R);
  rod(B, "cervical", "neck", [0.975, 0.887, 0], [0.7, 0.662, 0], R);
  rod(B, "thoracic", "thorax", [0.7, 0.672, 0], [0.375, 0.706, 0], R);
  hoop(B, "rib-girth", 0.57, 0.575, 0.153, 0.118, 0.075, 0.045, { member: "thorax", part: "member" });
  rod(B, "lumbar-column", "lumbar", [0.375, 0.706, 0], [0.108, 0.706, 0], R);
  rod(B, "pelvis-girdle", "pelvis", [0.108, 0.706, 0], [-0.055, 0.668, 0], R);
  rod(B, "caudal-1", "tail-1", [-0.058, 0.694, 0], [-0.032, 0.766, 0], R);
  rod(B, "caudal-2", "tail-2", [-0.032, 0.766, 0], [-0.002, 0.835, 0], R);
  for (const [s, sg] of SIDES) {
    const z = (v: number): number => sg * v;
    M(`ear-${s}`);
    B.ref(`ear-cartilage-${s}`, `ear-${s}`);
    rod(B, `scapula-${s}`, `scapula-f${s}`, [0.655, 0.705, z(0.052)], [0.728, 0.596, z(0.078)], R);
    rod(B, `humerus-${s}`, `humerus-f${s}`, [0.728, 0.596, z(0.078)], [0.655, 0.438, z(0.082)], R);
    rod(B, `radius-ulna-${s}`, `forearm-f${s}`, [0.655, 0.438, z(0.082)], [0.664, 0.226, z(0.075)], R);
    rod(B, `metacarpal-${s}`, `metacarpus-f${s}`, [0.664, 0.226, z(0.075)], [0.659, 0.079, z(0.075)], R);
    rod(B, `phalanges-f${s}`, `hoof-f${s}`, [0.659, 0.079, z(0.075)], [0.7, 0.012, z(0.075)], R);
    rod(B, `femur-${s}`, `thigh-h${s}`, [0, 0.64, z(0.07)], [0.118, 0.462, z(0.084)], R);
    rod(B, `tibia-${s}`, `shank-h${s}`, [0.118, 0.462, z(0.084)], [0.014, 0.25, z(0.074)], R);
    rod(B, `metatarsal-${s}`, `metatarsus-h${s}`, [0.014, 0.25, z(0.074)], [0.029, 0.079, z(0.072)], R);
    rod(B, `phalanges-h${s}`, `hoof-h${s}`, [0.029, 0.079, z(0.072)], [0.07, 0.012, z(0.072)], R);
  }
  masses(B, 0.0375);
}

if (import.meta.main) {
  const scale = Number(process.argv[2]);
  const B = new Bench(scale);
  if (scale === 100) { body100(B); thermalRefs(B); outlets(B, { nose: [0.239, 0.006, 0, 0.0077, 0.014], mouth: [0.2345, -0.0205, 0.0077, 0.011] }); structure100(B); }
  else if (scale === 200) { body200(B); thermalRefs(B); outlets(B, { nose: [0.2295, 0.003, 0, 0.0152, 0.018], mouth: [0.2, -0.0215, 0.0152, 0.019] }); structure200(B); }
  else if (scale === 500) { body500(B); thermal500(B); structure500(B); }
  else throw new Error("scale 100, 200 or 500");
  const ps = B.probe();
  const min = [0, 1, 2].map((d) => Math.min(...ps.map((p) => p.min[d]!)));
  const max = [0, 1, 2].map((d) => Math.max(...ps.map((p) => p.max[d]!)));
  console.log(`1:${scale} body ${B.lists.body.length} thermal ${B.lists.thermal.length} structure ${B.lists.structure.length}`, "size", max.map((v, i) => (v - min[i]!).toFixed(4)).join(" "), "from", min.map((v) => v.toFixed(3)).join(" "));
  if (process.argv.includes("--write")) B.save(DECISIONS[scale]!);
}
