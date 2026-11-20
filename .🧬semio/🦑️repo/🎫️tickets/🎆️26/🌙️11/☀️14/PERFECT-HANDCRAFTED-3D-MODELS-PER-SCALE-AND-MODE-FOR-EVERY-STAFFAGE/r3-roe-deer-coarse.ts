/** 🦌️ Round 3 roe deer at 1:100, 1:200 and 1:500: each scale has its own hand tables (merged masses, fewer and simpler sections), drawn for what that scale can show (smallest feature 15 / 30 / 75 mm). 1:100 keeps every segment of the buck with one hoof per foot and simplified antlers; 1:200 merges pastern and hoof forms, keeps a plain antler spike pair; 1:500 is trunk, neck, head with ear stubs and three segments per leg at the size floor.
 * Usage: bun r3-roe-deer-coarse.ts <100|200|500> [--write]
 * @see ./r3-roe-deer-50.ts
 * @see ./r3-roe-deer-bones.ts */
import { writeFileSync } from "node:fs";
import { saveLists } from "./kit-lib.ts";
import { OUT, SPEC, Roe, dense, side, sx, type Json, type V3 } from "./r3-roe-deer-kit.ts";
import { ANTLER, COAT, EARL, EYE, FACE, HORN, LEG, NOSE, REGION, SIDES, WHITE, beamAt, domed, ear, lines, piece } from "./r3-roe-deer-50.ts";
import { structure } from "./r3-roe-deer-bones.ts";

type Rows = readonly (readonly number[])[];
type Scale = 100 | 200 | 500;

const TRUNK: Record<Scale, Rows> = {
  100: [[-0.108, 0.69, 0.61, 0.035, 2.0], [-0.085, 0.735, 0.575, 0.066, 2.1], [-0.03, 0.765, 0.552, 0.085, 2.2], [0.05, 0.776, 0.541, 0.088, 2.3], [0.13, 0.771, 0.533, 0.08, 2.3], [0.2, 0.762, 0.519, 0.078, 2.3], [0.28, 0.753, 0.486, 0.088, 2.4], [0.38, 0.748, 0.455, 0.096, 2.4], [0.47, 0.753, 0.446, 0.088, 2.3], [0.55, 0.742, 0.45, 0.071, 2.2], [0.6, 0.708, 0.474, 0.056, 2.1], [0.64, 0.656, 0.524, 0.034, 2.0]],
  200: [[-0.105, 0.7, 0.6, 0.04, 2.0], [-0.06, 0.755, 0.56, 0.078, 2.2], [0.05, 0.775, 0.541, 0.088, 2.3], [0.18, 0.765, 0.523, 0.079, 2.3], [0.3, 0.751, 0.478, 0.091, 2.4], [0.42, 0.75, 0.449, 0.094, 2.4], [0.54, 0.745, 0.449, 0.074, 2.2], [0.62, 0.686, 0.494, 0.047, 2.0], [0.645, 0.635, 0.555, 0.03, 2.0]],
  500: [[-0.1, 0.71, 0.6, 0.05, 2.0], [-0.03, 0.765, 0.55, 0.085, 2.2], [0.1, 0.773, 0.535, 0.085, 2.3], [0.26, 0.755, 0.495, 0.088, 2.3], [0.42, 0.75, 0.449, 0.093, 2.3], [0.56, 0.74, 0.452, 0.07, 2.1], [0.64, 0.665, 0.52, 0.04, 2.0]],
};
/** Trunk pieces of a scale: bone, range of the densified table, the range the piece owns. Neighbours share the bound of their owned ranges and that bound is a hand station, so two pieces meet in one clean ring: beyond it each piece runs on inside its neighbour, tucked, so that the trunk stays closed when the spine bends and the drawing shows no sliver; the loin starts at its ring with the croup. */
const PIECES: Record<Scale, readonly (readonly [string, string, number, number, number, number])[]> = {
  100: [["croup", "pelvis", -1, 0.2, -1, 0.13], ["hindbody", "lumbar", 0.13, 0.38, 0.13, 0.28], ["chest", "thorax", 0.2, 1, 0.28, 1]],
  200: [["croup", "pelvis", -1, 0.18, -1, 0.05], ["hindbody", "lumbar", 0.05, 0.42, 0.05, 0.3], ["chest", "thorax", 0.18, 1, 0.3, 1]],
  500: [["hindbody", "lumbar", -1, 0.35, -1, 0.26], ["chest", "thorax", 0.17, 1, 0.26, 1]],
};
const NECK: Record<Scale, Rows> = {
  100: [[-0.07, 0.022, 0.11, 0.058], [0.02, 0.018, 0.094, 0.055], [0.12, 0.007, 0.062, 0.043], [0.22, 0.001, 0.045, 0.036], [0.3, 0, 0.04, 0.033], [0.33, -0.004, 0.032, 0.028]],
  200: [[-0.07, 0.022, 0.11, 0.058], [0.05, 0.014, 0.084, 0.052], [0.17, 0.004, 0.05, 0.04], [0.3, 0, 0.04, 0.034], [0.33, -0.003, 0.033, 0.03]],
  500: [[-0.06, 0.02, 0.105, 0.058], [0.1, 0.008, 0.07, 0.048], [0.32, 0, 0.042, 0.04]],
};
const HEAD: Record<Scale, Rows> = {
  100: [[-0.02, 0.014, 0.024, 0.024], [0.02, 0.012, 0.047, 0.04], [0.07, 0.008, 0.052, 0.046], [0.12, 0.0075, 0.0355, 0.035], [0.18, 0.004, 0.023, 0.0225], [0.214, 0.002, 0.013, 0.015]],
  200: [[-0.015, 0.014, 0.028, 0.027], [0.04, 0.01, 0.05, 0.044], [0.1, 0.008, 0.042, 0.04], [0.17, 0.005, 0.025, 0.024], [0.212, 0.003, 0.0165, 0.0165]],
  500: [[-0.02, 0.004, 0.04, 0.04], [0.05, 0.004, 0.054, 0.046], [0.13, -0.004, 0.042, 0.04], [0.222, -0.008, 0.038, 0.038]],
};
const JAW: Record<100 | 200, Rows> = {
  100: [[0.03, -0.03, 0.016, 0.026], [0.07, -0.036, 0.014, 0.027], [0.13, -0.0285, 0.011, 0.019], [0.19, -0.0225, 0.009, 0.013], [0.208, -0.021, 0.0083, 0.0085]],
  200: [[0.03, -0.027, 0.0165, 0.026], [0.08, -0.029, 0.0165, 0.027], [0.15, -0.019, 0.0165, 0.018], [0.205, -0.013, 0.0165, 0.0165]],
};
const EAR: Record<100 | 200, Rows> = {
  100: [[0, 0.012, 0.01], [0.3, 0.03, 0.0083], [0.6, 0.032, 0.0083], [0.9, 0.014, 0.0083], [1, 0.0083, 0.0083]],
  200: [[0, 0.0165, 0.0165], [0.35, 0.03, 0.0165], [0.7, 0.028, 0.0165], [1, 0.0165, 0.0165]],
};
const THIGH: Record<Scale, Rows> = {
  100: [[-0.05, -0.014, 0.068, 0.03, 0.047], [0.0, -0.024, 0.096, 0.043, 0.052], [0.07, -0.027, 0.102, 0.045, 0.055], [0.14, -0.018, 0.073, 0.037, 0.062], [0.2, -0.003, 0.038, 0.027, 0.068]],
  200: [[-0.045, -0.016, 0.072, 0.032, 0.048], [0.04, -0.027, 0.103, 0.045, 0.054], [0.13, -0.02, 0.078, 0.038, 0.061], [0.2, -0.003, 0.038, 0.027, 0.068]],
  500: [[-0.03, -0.02, 0.085, 0.042, 0.056], [0.06, -0.026, 0.1, 0.045, 0.06], [0.19, -0.004, 0.045, 0.038, 0.066]],
};
const SHANK: Record<Scale, Rows> = {
  100: [[-0.03, 0.002, 0.04, 0.029, 0.067], [0.03, -0.004, 0.048, 0.031, 0.07], [0.1, -0.009, 0.036, 0.025, 0.07], [0.18, -0.011, 0.022, 0.016, 0.069], [0.23, -0.016, 0.021, 0.014, 0.068]],
  200: [[-0.03, 0.002, 0.04, 0.029, 0.067], [0.04, -0.005, 0.047, 0.03, 0.07], [0.14, -0.009, 0.028, 0.02, 0.069], [0.23, -0.012, 0.02, 0.0165, 0.068]],
  500: [[-0.02, 0, 0.044, 0.038, 0.067], [0.08, -0.006, 0.042, 0.038, 0.068], [0.23, -0.004, 0.038, 0.038, 0.066]],
};
const HIND_CANNON: Record<100 | 200, Rows> = {
  100: [[-0.022, -0.005, 0.023, 0.0155, 0.068], [0.015, -0.003, 0.021, 0.015, 0.068], [0.045, 0, 0.0125, 0.0105, 0.067], [0.17, 0, 0.011, 0.0098, 0.063], [0.225, -0.002, 0.0155, 0.014, 0.061], [0.244, -0.001, 0.0125, 0.0125, 0.061]],
  200: [[-0.03, -0.012, 0.025, 0.0165, 0.068], [0.03, -0.002, 0.02, 0.0165, 0.067], [0.12, 0, 0.0165, 0.0165, 0.064], [0.24, 0, 0.0165, 0.0165, 0.061]],
};
const SCAPULA: Record<100 | 200, Rows> = {
  100: [[-0.005, 0, 0.034, 0.013, 0.044], [0.04, 0.002, 0.048, 0.018, 0.052], [0.11, 0.001, 0.047, 0.021, 0.058], [0.165, -0.004, 0.04, 0.021, 0.061], [0.19, -0.008, 0.03, 0.018, 0.062]],
  200: [[0.0, 0, 0.036, 0.0165, 0.046], [0.08, 0.002, 0.05, 0.02, 0.055], [0.16, -0.004, 0.04, 0.021, 0.061], [0.19, -0.008, 0.03, 0.018, 0.062]],
};
const UPPER_ARM: Record<Scale, Rows> = {
  100: [[-0.01, -0.008, 0.03, 0.018, 0.058], [0.03, -0.016, 0.048, 0.023, 0.063], [0.09, -0.019, 0.05, 0.025, 0.067], [0.15, -0.008, 0.033, 0.022, 0.07], [0.18, -0.002, 0.023, 0.018, 0.071]],
  200: [[-0.01, -0.008, 0.03, 0.018, 0.058], [0.06, -0.019, 0.052, 0.025, 0.065], [0.14, -0.01, 0.036, 0.022, 0.07], [0.18, -0.002, 0.023, 0.018, 0.071]],
  500: [[-0.05, -0.004, 0.05, 0.038, 0.062], [0.04, -0.016, 0.064, 0.04, 0.064], [0.12, -0.012, 0.058, 0.04, 0.066], [0.18, -0.002, 0.048, 0.04, 0.067]],
};
const FOREARM: Record<Scale, Rows> = {
  100: [[-0.04, -0.016, 0.022, 0.016, 0.072], [-0.005, -0.008, 0.031, 0.021, 0.072], [0.05, -0.002, 0.026, 0.019, 0.07], [0.12, 0, 0.017, 0.014, 0.068], [0.18, 0, 0.015, 0.0135, 0.066], [0.205, 0, 0.0155, 0.014, 0.065]],
  200: [[-0.035, -0.014, 0.024, 0.0165, 0.072], [0.01, -0.006, 0.031, 0.021, 0.072], [0.1, 0, 0.02, 0.0165, 0.069], [0.205, 0, 0.0165, 0.0165, 0.065]],
  500: [[-0.02, -0.004, 0.04, 0.038, 0.07], [0.09, 0, 0.038, 0.038, 0.068], [0.2, 0, 0.038, 0.038, 0.065]],
};
const FORE_CANNON: Record<100 | 200, Rows> = {
  100: [[-0.02, 0.001, 0.0165, 0.015, 0.065], [0.01, 0.001, 0.016, 0.0145, 0.065], [0.035, 0, 0.011, 0.0098, 0.064], [0.13, 0, 0.011, 0.0102, 0.062], [0.162, -0.001, 0.015, 0.0135, 0.061], [0.182, -0.001, 0.0125, 0.0125, 0.061]],
  200: [[-0.02, 0, 0.0175, 0.0165, 0.065], [0.06, 0, 0.0165, 0.0165, 0.063], [0.18, 0, 0.0165, 0.0165, 0.061]],
};
const PASTERN: Record<100 | 200, Rows> = {
  100: [[-0.012, 0, 0.0135, 0.013], [0.025, 0, 0.012, 0.0135], [0.056, 0, 0.0115, 0.014]],
  200: [[-0.01, 0, 0.0165, 0.0165], [0.068, 0.004, 0.0165, 0.0165]],
};
/** Hoof of both claws at 1:100 in planes x = const behind the toe tip: x, centre height, half height, half width. */
const HOOF_100 = [[-0.045, 0.0083, 0.0083, 0.0115], [-0.036, 0.0105, 0.0105, 0.014], [-0.027, 0.012, 0.012, 0.015], [-0.016, 0.0095, 0.0095, 0.0135], [-0.006, 0.0083, 0.0083, 0.0105], [-0.001, 0.0083, 0.0083, 0.0085]] as const;
const HIND_Z = 0.059;

function toe(S: Roe, hoof: string): V3 {
  const f = S.frame(hoof);
  const len = S.c.raw.rig.bones.find((b: Json) => b.id === hoof).lengthM as number;
  return [f.o[0]! + f.u[0]! * len, f.o[1]! + f.u[1]! * len, 0];
}

function trunk(S: Roe, scale: Scale): void {
  const T = dense(lines(TRUNK[scale]), 2);
  for (const [id, bone, lo, hi, own0, own1] of PIECES[scale]) S.hull(id, bone, piece(T, lo, hi, own0, own1, scale === 100 ? 0.012 : scale === 200 ? 0.016 : 0.008), COAT, { shape: id });
}

function body100(S: Roe): void {
  trunk(S, 100);
  S.limb("neck", "neck", dense(NECK[100], 2), COAT, { shape: "neck", e: 2.2 });
  S.limb("head", "head", dense(HEAD[100], 2), FACE, { shape: "head", e: 2.5 });
  S.limb("jaw", "jaw", dense(JAW[100], 2), FACE, { frame: "head", shape: "jaw", e: 2.3 });
  const hu = S.frame("head").u;
  S.ball("nose-leather", "head", S.at("head", 0.2085, -0.003, 0), [0.013, 0.018, 0.0168], NOSE, [hu[0]!, hu[1]!, 0], "nose-leather");
  S.ball("rump-patch", "pelvis", [-0.06, 0.672, 0], [0.056, 0.07, 0.058], WHITE, [1, 0, 0], "rump-patch");
  for (const sd of SIDES) {
    const n = side(sd);
    const k = sx(sd);
    S.ball(`${n}-eye`, "head", S.at("head", 0.089, 0.03, sd * 0.0365), [0.0083, 0.0108, 0.0108], EYE, [0, 0, sd], sd < 0 ? "eye" : "eye-f");
    ear(S, `${n}-ear`, sd, EARL, 0.135, EAR[100], { shape: `${n}-ear` });
    const shape = `${n}-antler`;
    S.run(`${n}-antler-beam`, "head", ([[-0.06, 0.0115], [0.06, 0.0125], [0.14, 0.0098], [0.4, 0.009], [0.7, 0.0083], [1, 0.0083]] as const).map(([t, r]) => ({ p: beamAt(S, sd, t, 0.2), rl: r, rs: r })), ANTLER, { shape });
    const f = S.frame("head");
    const tine = (id: string, t: number, a: number, b: number, l: number): void => {
      const p = beamAt(S, sd, t, 0.2);
      S.run(id, "head", [{ p, rl: 0.0083, rs: 0.0083 }, { p: [p[0] + (a * f.u[0]! + b * f.v[0]!) * l, p[1] + (a * f.u[1]! + b * f.v[1]!) * l, p[2] + sd * 0.003], rl: 0.0083, rs: 0.0083 }], ANTLER, { shape });
    };
    tine(`${n}-antler-brow`, 0.5, 0.7, 0.71, 0.05);
    tine(`${n}-antler-back`, 0.76, -0.68, 0.73, 0.042);
    S.limb(`${n}-thigh`, `thigh-h${k}`, dense(domed(THIGH[100]), 2), COAT, { sd, shape: `${n}-thigh`, e: 2.0 });
    S.limb(`${n}-shank`, `shank-h${k}`, dense(domed(SHANK[100], true, false), 2), LEG, { sd, shape: `${n}-shank` });
    const hc = `cannon-h${k}`;
    S.run(`${n}-hock-point`, hc, [{ p: S.at(hc, 0.03, -0.005, sd * 0.068), rl: 0.01, rs: 0.011 }, { p: S.at(hc, -0.028, -0.023, sd * 0.068), rl: 0.0083, rs: 0.0095 }], LEG, { shape: `${n}-hind-cannon` });
    S.limb(`${n}-hind-cannon`, hc, domed(HIND_CANNON[100]), LEG, { sd, shape: `${n}-hind-cannon` });
    S.limb(`${n}-scapula`, `scapula-f${k}`, dense(domed(SCAPULA[100]), 2), COAT, { sd, shape: `${n}-scapula`, e: 2.0 });
    S.limb(`${n}-humerus`, `humerus-f${k}`, dense(domed(UPPER_ARM[100]), 2), COAT, { sd, shape: `${n}-humerus`, e: 2.0 });
    S.limb(`${n}-forearm`, `forearm-f${k}`, dense(domed(FOREARM[100]), 2), LEG, { sd, shape: `${n}-forearm` });
    S.limb(`${n}-fore-cannon`, `cannon-f${k}`, domed(FORE_CANNON[100]), LEG, { sd, shape: `${n}-fore-cannon` });
    for (const [leg, h] of [["hind", "h"], ["fore", "f"]] as const) {
      const hoof = `hoof-${h}${k}`;
      S.limb(`${n}-${leg}-pastern`, hoof, domed(PASTERN[100].map((r) => [...r, HIND_Z])), LEG, { sd, shape: `${n}-${leg}-hoof` });
      const t = toe(S, hoof);
      S.hull(`${n}-${leg}-hoof`, hoof, HOOF_100.map((r) => [r[0], r[1] + t[1], r[2], r[3], 2.6, sd * HIND_Z]), HORN, { x0: t[0], sd: 1, shape: `${n}-${leg}-hoof` });
    }
  }
}

function body200(S: Roe): void {
  trunk(S, 200);
  S.limb("neck", "neck", dense(NECK[200], 2), COAT, { shape: "neck", e: 2.2 });
  S.limb("head", "head", dense(HEAD[200], 2), FACE, { shape: "head", e: 2.4 });
  S.limb("jaw", "jaw", JAW[200], FACE, { frame: "head", shape: "jaw", e: 2.3 });
  S.ball("rump-patch", "pelvis", [-0.06, 0.672, 0], [0.056, 0.07, 0.058], WHITE, [1, 0, 0], "rump-patch");
  for (const sd of SIDES) {
    const n = side(sd);
    const k = sx(sd);
    ear(S, `${n}-ear`, sd, EARL, 0.118, EAR[200], { shape: `${n}-ear` });
    S.run(`${n}-antler`, "head", ([-0.08, 0.3, 0.65, 0.92] as const).map((t) => ({ p: beamAt(S, sd, t, 0.19), rl: 0.0165, rs: 0.0165 })), ANTLER, { shape: `${n}-antler` });
    S.ball(`${n}-antler-tip`, "head", beamAt(S, sd, 0.92, 0.19), [0.0165, 0.0165, 0.0165], ANTLER, [0, 1, 0], `${n}-antler`);
    const ef = S.frame(`ear-${k}`);
    S.ball(`${n}-ear-tip`, `ear-${k}`, [ef.o[0]! + (ef.u[0]! / 1.0847) * 0.118, ef.o[1]! + (ef.u[1]! / 1.0847) * 0.118, sd * (0.03 + (0.42 / 1.0847) * 0.118)], [0.0165, 0.0165, 0.0165], EARL, [0, 1, 0], `${n}-ear`);
    S.limb(`${n}-thigh`, `thigh-h${k}`, dense(domed(THIGH[200]), 2), COAT, { sd, shape: `${n}-thigh`, e: 2.0 });
    S.limb(`${n}-shank`, `shank-h${k}`, domed(SHANK[200], true, false), LEG, { sd, shape: `${n}-shank` });
    S.limb(`${n}-hind-cannon`, `cannon-h${k}`, domed(HIND_CANNON[200]), LEG, { sd, shape: `${n}-hind-cannon` });
    S.limb(`${n}-scapula`, `scapula-f${k}`, dense(domed(SCAPULA[200]), 2), COAT, { sd, shape: `${n}-scapula`, e: 2.0 });
    S.limb(`${n}-humerus`, `humerus-f${k}`, dense(domed(UPPER_ARM[200]), 2), COAT, { sd, shape: `${n}-humerus`, e: 2.0 });
    S.limb(`${n}-forearm`, `forearm-f${k}`, domed(FOREARM[200]), LEG, { sd, shape: `${n}-forearm` });
    S.limb(`${n}-fore-cannon`, `cannon-f${k}`, domed(FORE_CANNON[200]), LEG, { sd, shape: `${n}-fore-cannon` });
    for (const [leg, h] of [["hind", "h"], ["fore", "f"]] as const) {
      const hoof = `hoof-${h}${k}`;
      S.limb(`${n}-${leg}-pastern`, hoof, PASTERN[200].map((r) => [...r, HIND_Z]), LEG, { sd, shape: `${n}-${leg}-hoof` });
      const t = toe(S, hoof);
      S.ball(`${n}-${leg}-hoof`, hoof, [t[0] - 0.022, t[1] + 0.0165, sd * HIND_Z], [0.0225, 0.0165, 0.0165], HORN, [1, 0, 0], `${n}-${leg}-hoof`);
    }
  }
}

function body500(S: Roe): void {
  const fl = 0.038;
  trunk(S, 500);
  S.limb("neck", "neck", dense(NECK[500], 2), COAT, { shape: "neck", e: 2.2, floor: fl });
  S.limb("head", "head", dense(HEAD[500], 2), FACE, { shape: "head", e: 2.3, floor: fl });
  for (const sd of SIDES) {
    const n = side(sd);
    const k = sx(sd);
    const ef = S.frame(`ear-${k}`);
    S.ball(`${n}-ear`, `ear-${k}`, [ef.o[0]! + ef.u[0]! * 0.05, ef.o[1]! + ef.u[1]! * 0.05, sd * 0.045], [0.072, 0.04, 0.038], EARL, [ef.u[0]!, ef.u[1]!, sd * 0.5], sd < 0 ? "ears" : "far-ear");
    S.run(`${n}-antler`, "head", ([-0.1, 0.5, 1.02] as const).map((t) => ({ p: beamAt(S, sd, t, 0.2), rl: fl, rs: fl })), ANTLER, { shape: `${n}-antler`, floor: fl });
    S.ball(`${n}-antler-tip`, "head", beamAt(S, sd, 1.02, 0.2), [fl, fl, fl], ANTLER, [0, 1, 0], `${n}-antler`);
    S.limb(`${n}-thigh`, `thigh-h${k}`, dense(THIGH[500], 2), COAT, { sd, shape: `${n}-thigh`, e: 2.0, floor: fl });
    S.limb(`${n}-hind-leg`, `shank-h${k}`, SHANK[500], LEG, { sd, shape: `${n}-hind-leg`, floor: fl });
    S.limb(`${n}-fore-upper`, `humerus-f${k}`, dense(UPPER_ARM[500], 2), COAT, { sd, shape: `${n}-fore-upper`, e: 2.0, floor: fl });
    S.limb(`${n}-fore-leg`, `forearm-f${k}`, FOREARM[500], LEG, { sd, shape: `${n}-fore-leg`, floor: fl });
    for (const [leg, h] of [["hind", "h"], ["fore", "f"]] as const) {
      const cannon = `cannon-${h}${k}`;
      const f = S.frame(cannon);
      const t = toe(S, `hoof-${h}${k}`);
      const z = sd * 0.064;
      S.ball(`${n}-${leg === "hind" ? "hock" : "carpus"}`, cannon, [f.o[0]!, f.o[1]!, z], [0.04, 0.04, 0.04], LEG, [1, 0, 0], `${n}-${leg}-lower`);
      S.run(`${n}-${leg}-lower`, cannon, [{ p: [f.o[0]! - f.u[0]! * 0.02, f.o[1]! - f.u[1]! * 0.02, z], rl: fl, rs: fl }, { p: [(f.o[0]! + t[0]) / 2 - 0.012, (f.o[1]! + t[1]) / 2, z], rl: fl, rs: fl }, { p: [t[0] - 0.03, fl, z], rl: fl, rs: fl }], LEG, { shape: `${n}-${leg}-lower`, floor: fl });
    }
  }
}

type Extra = readonly [id: string, bone: string, at: V3, r: V3, region: string, tag?: string];

/** 🌡️ Thermal list of a coarse scale: references to the body solids by region, plus own small solids at the size floor for the regions the body of that scale no longer shows (eyes, nose leather, hooves, rump patch) and the outlets. */
function thermal(S: Roe, T: Roe, extras: readonly Extra[]): Json[] {
  const out: Json[] = [];
  for (const b of S.body) {
    if (/-antler(-(beam|brow|back|tip))?$/.test(b.id)) continue;
    const hit = REGION.find(([re]) => re.test(b.id));
    if (hit === undefined) throw new Error(`no region for ${b.id}`);
    out.push({ id: `th-${b.id}`, of: b.id, region: hit[1], ...(hit[2] === undefined ? {} : { layer: hit[2] }) });
  }
  for (const [id, bone, at, r, region, tag] of extras) {
    const s = T.ball(id, bone, at, r, tag === undefined ? { material: region === "hooves" ? "hoof-horn" : region === "rump-patch" ? "white-coat" : "bare-skin", tone: 0.1 } : { material: region === "coat-trunk" ? "summer-coat" : "bare-skin", tone: 0.2 }, bone === "head" ? [T.frame("head").u[0]!, T.frame("head").u[1]!, 0] : [1, 0, 0]);
    out.push({ ...s, region, ...(tag === undefined ? {} : { tags: ["emitter", tag] }) });
  }
  return out;
}

function extras(S: Roe, scale: Scale): Extra[] {
  const H = (s: number, v: number, z = 0): V3 => S.at("head", s, v, z);
  if (scale === 100) return [["th-breath-nose", "head", H(0.2175, 0.0035), [0.0083, 0.0105, 0.0083], "nose-skin", "breath-nose"], ["th-plume-back", "thorax", [0.36, 0.741, 0], [0.11, 0.05, 0.012], "coat-trunk", "plume-back"]];
  const feet = (r: number): Extra[] => SIDES.flatMap((sd) => (["h", "f"] as const).map((h) => { const t = toe(S, `hoof-${h}${sx(sd)}`); return [`th-${side(sd)}-${h === "h" ? "hind" : "fore"}-hoof`, `hoof-${h}${sx(sd)}`, [t[0] - 0.03, t[1] + r, sd * 0.064] as V3, [r, r, r] as V3, "hooves"] as Extra; }));
  if (scale === 200) return [
    ["th-nose-leather", "head", H(0.2, 0), [0.0165, 0.0165, 0.0165], "nose-skin"], ["th-near-eye", "head", H(0.089, 0.028, -0.03), [0.0165, 0.0165, 0.0165], "eyes"], ["th-far-eye", "head", H(0.089, 0.028, 0.03), [0.0165, 0.0165, 0.0165], "eyes"],
    ["th-breath-nose", "head", H(0.207, 0.003), [0.0165, 0.0165, 0.0165], "nose-skin", "breath-nose"], ["th-plume-back", "thorax", [0.36, 0.737, 0], [0.11, 0.05, 0.0165], "coat-trunk", "plume-back"],
  ];
  return [
    ["th-rump-patch", "lumbar", [-0.055, 0.672, 0], [0.05, 0.066, 0.056], "rump-patch"], ["th-nose-leather", "head", H(0.175, -0.006), [0.042, 0.042, 0.042], "nose-skin"], ["th-eyes", "head", H(0.08, 0.008), [0.042, 0.048, 0.042], "eyes"], ...feet(0.042),
    ["th-breath-nose", "head", H(0.178, -0.004), [0.042, 0.042, 0.042], "nose-skin", "breath-nose"], ["th-plume-back", "thorax", [0.36, 0.716, 0], [0.12, 0.06, 0.042], "coat-trunk", "plume-back"],
  ];
}

const DECISIONS: Record<Scale, { en: string; de: string }[]> = {
  100: [
    { en: "Body at 1:100 (smallest feature 15 mm): authored anew with its own tables. Croup, loin and chest keep the high croup, the tucked flank and the deep chest in twelve hand sections; neck, skull, mandible, nose leather and eyes stay separate. The two claws of a foot merge into one hoof of 45 x 30 mm, the dewclaws (12 mm) are below the feature and left out, hock point and cannon stay; the antlers are a beam with burr swelling, a brow tine and a back tine at the 16.5 mm floor, the ears leaves of 135 mm.", de: "Körper bei 1:100 (kleinstes Merkmal 15 mm): mit eigenen Tabellen neu gesetzt. Kruppe, Lende und Brust halten die hohe Kruppe, die aufgezogene Flanke und die tiefe Brust in zwölf Handschnitten; Hals, Schädel, Unterkiefer, Nasenspiegel und Augen bleiben getrennt. Die zwei Klauen eines Fußes verschmelzen zu einer Schale von 45 x 30 mm, die Afterklauen (12 mm) liegen unter dem Merkmal und entfallen, Fersenhöcker und Röhre bleiben; das Gehörn ist eine Stange mit Rosenverdickung, einer Vordersprosse und einer Hintersprosse an der Untergrenze von 16,5 mm, die Lauscher Blätter von 135 mm." },
    { en: "Thermal at 1:100: each region is the body surface it covers; the nostrils merge into one outlet patch on the nose leather, the plume leaves from a patch of the back. The antlers stay outside the thermal model.", de: "Thermik bei 1:100: jede Region ist die Körperoberfläche, die sie bedeckt; die Nasenlöcher verschmelzen zu einem Auslassfeld auf dem Nasenspiegel, die Warmluftfahne verlässt ein Rückenfeld. Das Gehörn bleibt außerhalb des Wärmemodells." },
    { en: "Structure at 1:100: the skeleton simplified. Skull and mandible, cervical, thoracic and lumbar spine with a spinous process on every second vertebra (the long ones carry the withers), six rib hoops per side, the sternal ones closed onto the sternum by their cartilages, sacrum, ilium and ischium, long bones with their epiphyses, calcaneus and olecranon as the prolonged ends of metatarsal and radius, one digit per foot, ear cartilages; joints at the limb and spine joints, the masses of chest, loin and pelvis.", de: "Tragwerk bei 1:100: das Skelett vereinfacht. Schädel und Unterkiefer, Hals-, Brust- und Lendenwirbelsäule mit einem Dornfortsatz an jedem zweiten Wirbel (die langen tragen den Widerrist), sechs Rippenbögen je Seite, die sternalen über ihre Knorpel am Brustbein geschlossen, Kreuzbein, Darmbein und Sitzbein, Röhrenknochen mit ihren Gelenkenden, Fersenbein und Ellbogenhöcker als verlängerte Enden von Röhrbein und Speiche, eine Zehe je Fuß, Ohrknorpel; Gelenke an den Glied- und Wirbelsäulengelenken, die Massen von Brust, Lende und Becken." },
  ],
  200: [
    { en: "Body at 1:200 (smallest feature 30 mm): authored anew. The trunk is nine hand sections in three pieces, the head a wedge with a jaw bar; eyes, nose leather and dewclaws are below the feature and are not modelled in the body. Shank, cannons and pasterns stand at the 33 mm floor, the hoof is one rounded form per foot, the hock keeps its backward point as the swollen head of the cannon; each antler is one plain spike, each ear a thick leaf; the white rump patch stays.", de: "Körper bei 1:200 (kleinstes Merkmal 30 mm): neu gesetzt. Der Rumpf besteht aus neun Handschnitten in drei Teilen, der Kopf ist ein Keil mit Kieferstab; Augen, Nasenspiegel und Afterklauen liegen unter dem Merkmal und werden im Körper nicht modelliert. Unterschenkel, Röhren und Fesseln stehen an der Untergrenze von 33 mm, die Schale ist eine gerundete Form je Fuß, das Sprunggelenk behält seinen Fersenhöcker als verdickten Kopf der Röhre; jede Stange ist ein schlichter Spieß, jeder Lauscher ein dickes Blatt; der weiße Spiegel bleibt." },
    { en: "Thermal at 1:200: the regions follow the merged body; eyes and nose leather are small solids at the size floor inside the head surface, the hoof form of each foot is the horn region, breath and plume leave from one patch each.", de: "Thermik bei 1:200: die Regionen folgen dem zusammengefassten Körper; Augen und Nasenspiegel sind kleine Körper an der Größenuntergrenze in der Kopfoberfläche, die Schalenform jedes Fußes ist die Hornregion, Atem und Warmluftfahne verlassen je ein Feld." },
    { en: "Structure at 1:200: plain shafts. Skull with jaw bar, cervical, thoracic and lumbar members, one pelvic girdle, five rib hoops per side on a sternum, one bar per limb bone with calcaneus and olecranon as prolonged ends, ear cartilages; joints at the large joints, the masses of chest and pelvis.", de: "Tragwerk bei 1:200: schlichte Schäfte. Schädel mit Kieferstab, Hals-, Brust- und Lendenglied, ein Beckengürtel, fünf Rippenbögen je Seite auf einem Brustbein, ein Stab je Gliedmaßenknochen mit Fersenbein und Ellbogenhöcker als verlängerten Enden, Ohrknorpel; Gelenke an den großen Gelenken, die Massen von Brust und Becken." },
  ],
  500: [
    { en: "Body at 1:500 (smallest feature 75 mm): the buck as a symbol that is still a roe deer: one loin-and-croup mass and one chest mass with the high croup and the tucked flank, a tapering neck, a head wedge with two ear stubs and two antler spikes, and per leg three segments at the 76 mm floor (thigh, shank and cannon with the foot; shoulder and upper arm in one, forearm and cannon with the foot) so that every gait still bends at stifle, hock, elbow and carpus. The antler pair stays as two plain spikes at the floor because it sets the height of the buck; jaw, eyes, nose, hooves and rump patch are below the feature.", de: "Körper bei 1:500 (kleinstes Merkmal 75 mm): der Bock als Symbol, das noch ein Reh ist: eine Lenden-Kruppen-Masse und eine Brustmasse mit hoher Kruppe und aufgezogener Flanke, ein sich verjüngender Hals, ein Kopfkeil mit zwei Lauscherstummeln und zwei Spießen und je Lauf drei Segmente an der Untergrenze von 76 mm (Keule, Unterschenkel und Röhre mit Fuß; Schulter und Oberarm in einem, Unterarm und Röhre mit Fuß), damit jede Gangart noch an Knie, Sprunggelenk, Ellbogen und Vorderfußwurzel beugt. Das Gehörn bleibt als zwei schlichte Spieße an der Untergrenze, weil es die Höhe des Bocks bestimmt; Kiefer, Augen, Nase, Schalen und Spiegel liegen unter dem Merkmal." },
    { en: "Thermal at 1:500: the regions follow the merged body; rump patch, eyes, nose leather and hooves are solids at the size floor so that every region keeps a place, breath and plume leave from one patch each.", de: "Thermik bei 1:500: die Regionen folgen dem zusammengefassten Körper; Spiegel, Augen, Nasenspiegel und Schalen sind Körper an der Größenuntergrenze, damit jede Region ihren Ort behält, Atem und Warmluftfahne verlassen je ein Feld." },
    { en: "Structure at 1:500: one bar per bone at the size floor: skull with the two antler spikes, jaw, neck, back in two members, pelvis bar, ear cartilages and four limb chains of scapula or femur down to the digit; the chest mass at its centre of mass.", de: "Tragwerk bei 1:500: ein Stab je Knochen an der Größenuntergrenze: Schädel mit den zwei Spießen, Kiefer, Hals, Rücken in zwei Gliedern, Beckenstab, Ohrknorpel und vier Gliedmaßenketten von Schulterblatt oder Oberschenkelbein bis zur Zehe; die Brustmasse in ihrem Schwerpunkt." },
  ],
};

if (import.meta.main) {
  const scale = Number(process.argv[2]) as Scale;
  const S = new Roe(SPEC, scale);
  (scale === 100 ? body100 : scale === 200 ? body200 : body500)(S);
  const th = thermal(S, new Roe(SPEC, scale), extras(S, scale));
  const st = structure(new Roe(SPEC, scale), scale, (id) => S.body.some((b) => b.id === id));
  console.log(`1:${scale} body ${S.body.length} thermal ${th.length} structure ${st.length}`);
  if (process.argv.includes("--write")) {
    saveLists(S.c, { body: S.list(), thermal: th, structure: st, decisions: DECISIONS[scale] });
    writeFileSync(`${OUT}/shapes.${scale}.json`, JSON.stringify(S.shapes));
  }
}
