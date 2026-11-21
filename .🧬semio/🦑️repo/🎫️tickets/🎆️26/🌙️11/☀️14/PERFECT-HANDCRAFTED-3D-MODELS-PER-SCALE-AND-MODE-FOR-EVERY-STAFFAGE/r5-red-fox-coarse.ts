/** 🦊️ Round 5 red fox at 1:100, 1:200 and 1:500: each scale its own hand tables (merged masses, fewer sections, parts at the size floor of the scale), with the corrections of the 1:50 remodel carried into every abstraction: the brush grows out of the croup instead of sitting on it as a knob, the gaskin is red like the thigh and only the cannons and paws are dark, the brush tip is white, the trunk lofts taper inside each other where they overlap so the bent spine stays closed, the shoulder is one rounded mass, the throat bib and the white lower jaw are kept at 1:100.
 * Usage: bun r5-red-fox-coarse.ts <100|200|500> [--write]
 * @see ./r5-red-fox-50.ts
 * @see ./r3-animals-red-fox.md */
import { writeFileSync } from "node:fs";
import { saveLists } from "./kit-lib.ts";
import { OUT, SPEC, Fox, dense, side, sx, type Json, type Look, type V3 } from "./r3-red-fox-kit.ts";
import { BONE, DARK, FORE_Z, HIND_Z, JOINT, JOINT_Z, MASS, RED, SIDES, SPINE_C, SPINE_L, SPINE_T, STERNUM, mem, ribPath } from "./r3-red-fox-50.ts";
import { structure as coarseStructure } from "./r3-red-fox-coarse.ts";
import { WHITE, ear, thermal } from "./r5-red-fox-50.ts";

type Rows = readonly (readonly number[])[];
type Tables = {
  trunk: Rows; split: readonly [number, number]; taper: number; neck: Rows; bib?: Rows; head: Rows; jaw?: Rows; ear: Rows; root: Rows; mid: Rows; tip: Rows;
  thigh: Rows; gaskin: Rows; hindCannon: Rows; paw: Rows; forearm: Rows; foreCannon: Rows; shoulder?: Rows; brush: readonly [string, string, string]; ears: readonly [string, string]; dense: boolean; floor?: number;
};

const T100: Tables = {
  trunk: [[-0.056, 0.39, 0.018, 0.018, 2], [-0.03, 0.381, 0.055, 0.054, 2.1], [0.0, 0.371, 0.065, 0.064, 2.2], [0.06, 0.363, 0.073, 0.063, 2.2], [0.14, 0.359, 0.078, 0.059, 2.2], [0.21, 0.347, 0.091, 0.067, 2.3], [0.28, 0.336, 0.101, 0.074, 2.3], [0.37, 0.329, 0.112, 0.07, 2.25], [0.42, 0.337, 0.097, 0.055, 2.15], [0.455, 0.348, 0.066, 0.036, 2.05], [0.471, 0.356, 0.03, 0.017, 2]],
  split: [0.17, 0.3], taper: 0.004,
  neck: [[-0.06, -0.002, 0.064, 0.047], [0.0, -0.004, 0.07, 0.056], [0.06, -0.004, 0.056, 0.049], [0.12, -0.001, 0.044, 0.042], [0.162, 0.004, 0.032, 0.035]],
  bib: [[-0.06, -0.047, 0.024, 0.032], [0.0, -0.048, 0.03, 0.04], [0.08, -0.034, 0.028, 0.034], [0.15, -0.023, 0.02, 0.026]],
  head: [[-0.014, 0.014, 0.022, 0.028, 2.2], [0.01, 0.013, 0.033, 0.048, 2.4], [0.04, 0.012, 0.032, 0.05, 2.4], [0.07, 0.009, 0.025, 0.032, 2.3], [0.1, 0.003, 0.018, 0.02, 2.3], [0.14, -0.001, 0.0125, 0.0135, 2.2], [0.155, -0.002, 0.0083, 0.0083, 2.1]],
  jaw: [[0.0, -0.014, 0.016, 0.036], [0.035, -0.02, 0.019, 0.042], [0.075, -0.02, 0.0135, 0.024], [0.115, -0.018, 0.0095, 0.014], [0.145, -0.0145, 0.0083, 0.0083]],
  ear: [[0, 0.021, 0.0095], [0.3, 0.027, 0.0085], [0.7, 0.016, 0.0083], [1, 0.0083, 0.0083]],
  root: [[-0.024, 0.014, 0.016, 0.018], [0.03, 0.006, 0.031, 0.031], [0.1, -0.002, 0.047, 0.046], [0.165, -0.004, 0.056, 0.054]],
  mid: [[-0.012, -0.004, 0.055, 0.053], [0.06, -0.003, 0.06, 0.058], [0.152, -0.002, 0.05, 0.048]],
  tip: [[-0.01, -0.002, 0.051, 0.049], [0.05, 0, 0.042, 0.04], [0.105, 0, 0.022, 0.021], [0.134, 0, 0.0083, 0.0083]],
  thigh: [[-0.03, -0.014, 0.034, 0.022, 0.034], [0.02, -0.014, 0.056, 0.033, 0.041], [0.08, -0.008, 0.047, 0.031, 0.043], [0.142, 0.003, 0.026, 0.02, 0.045]],
  gaskin: [[-0.016, -0.004, 0.021, 0.016, 0.045], [0.02, -0.008, 0.027, 0.019, 0.045], [0.085, -0.007, 0.018, 0.0135, 0.041], [0.142, -0.005, 0.0138, 0.0115, 0.038]],
  hindCannon: [[-0.016, -0.009, 0.013, 0.0105, 0.038], [0.02, -0.001, 0.0115, 0.0105, 0.038], [0.092, 0.002, 0.0118, 0.0135, 0.036]],
  paw: [[-0.01, -0.001, 0.0122, 0.0115], [0.015, -0.005, 0.017, 0.016], [0.035, -0.003, 0.0135, 0.0165], [0.049, 0.002, 0.0083, 0.012]],
  forearm: [[-0.02, -0.01, 0.015, 0.011, 0.039], [0.005, -0.004, 0.02, 0.0145, 0.039], [0.06, 0.0, 0.0145, 0.012, 0.036], [0.124, 0.0, 0.0125, 0.012, 0.033]],
  foreCannon: [[-0.01, 0.0, 0.0118, 0.0112, 0.033], [0.035, 0, 0.0112, 0.012, 0.032], [0.07, 0.002, 0.0118, 0.0145, 0.032]],
  shoulder: [[-0.008, -0.004, 0.026, 0.01, 0.016], [0.03, -0.002, 0.034, 0.02, 0.03], [0.004, 0.002, 0.039, 0.024, 0.035], [0.065, -0.007, 0.04, 0.021, 0.031], [0.122, -0.006, 0.038, 0.0135, 0.0175], [0.134, -0.005, 0.038, 0.009, 0.01]],
  brush: ["brush", "brush-2", "brush-tip"], ears: ["ear-n", "ear-f"], dense: true,
};

const T200: Tables = {
  trunk: [[-0.054, 0.388, 0.03, 0.03, 2], [-0.02, 0.377, 0.059, 0.058, 2.2], [0.04, 0.365, 0.071, 0.064, 2.2], [0.14, 0.358, 0.079, 0.06, 2.2], [0.23, 0.344, 0.094, 0.07, 2.3], [0.33, 0.331, 0.108, 0.074, 2.3], [0.4, 0.333, 0.105, 0.062, 2.2], [0.45, 0.346, 0.074, 0.04, 2.05], [0.47, 0.355, 0.034, 0.02, 2]],
  split: [0.17, 0.3], taper: 0.006,
  neck: [[-0.06, -0.003, 0.064, 0.048], [0.0, -0.004, 0.068, 0.055], [0.08, -0.004, 0.052, 0.046], [0.16, 0.002, 0.034, 0.036]],
  head: [[-0.012, 0.01, 0.024, 0.028, 2.2], [0.02, 0.006, 0.042, 0.05, 2.4], [0.06, 0.0, 0.04, 0.042, 2.3], [0.1, -0.008, 0.027, 0.022, 2.3], [0.152, -0.01, 0.0175, 0.0165, 2.1]],
  ear: [[0, 0.024, 0.0165], [0.35, 0.027, 0.0165], [0.75, 0.016, 0.011], [1, 0.005, 0.005]],
  root: [[-0.022, 0.014, 0.0165, 0.018], [0.06, 0.0, 0.038, 0.037], [0.165, -0.004, 0.056, 0.054]],
  mid: [[-0.012, -0.004, 0.055, 0.053], [0.07, -0.003, 0.06, 0.058], [0.152, -0.002, 0.05, 0.048]],
  tip: [[-0.01, -0.002, 0.051, 0.049], [0.06, 0, 0.04, 0.038], [0.134, 0, 0.0165, 0.0165]],
  thigh: [[-0.026, -0.013, 0.038, 0.022, 0.036], [0.03, -0.013, 0.055, 0.032, 0.042], [0.09, -0.006, 0.042, 0.028, 0.044], [0.142, 0.003, 0.026, 0.0185, 0.045]],
  gaskin: [[-0.018, -0.006, 0.024, 0.0175, 0.045], [0.03, -0.008, 0.026, 0.019, 0.045], [0.09, -0.006, 0.0175, 0.0165, 0.041], [0.142, -0.005, 0.0165, 0.0165, 0.038]],
  hindCannon: [[-0.015, -0.006, 0.0165, 0.0165, 0.038], [0.09, 0.002, 0.0165, 0.0165, 0.036]],
  paw: [[-0.008, -0.002, 0.019, 0.018], [0.02, -0.005, 0.022, 0.019], [0.05, 0.002, 0.019, 0.0175]],
  forearm: [[-0.02, -0.008, 0.021, 0.0165, 0.039], [0.03, -0.002, 0.02, 0.0165, 0.038], [0.124, 0, 0.0175, 0.0165, 0.034]],
  foreCannon: [[-0.01, 0, 0.0165, 0.0165, 0.034], [0.068, 0.002, 0.0165, 0.0165, 0.033]],
  shoulder: [[-0.006, -0.004, 0.026, 0.0165, 0.0165], [0.045, -0.002, 0.034, 0.021, 0.031], [0.004, 0.002, 0.039, 0.023, 0.034], [0.07, -0.007, 0.04, 0.0195, 0.028], [0.13, -0.005, 0.038, 0.0165, 0.0165]],
  brush: ["brush", "brush-2", "brush-tip"], ears: ["ear", "ear-far"], dense: false,
};

const T500: Tables = {
  trunk: [[-0.052, 0.387, 0.024, 0.024, 2], [-0.015, 0.375, 0.061, 0.06, 2.2], [0.06, 0.364, 0.074, 0.065, 2.2], [0.14, 0.358, 0.079, 0.06, 2.2], [0.23, 0.344, 0.094, 0.07, 2.3], [0.33, 0.331, 0.108, 0.075, 2.3], [0.4, 0.333, 0.104, 0.064, 2.2], [0.445, 0.343, 0.083, 0.046, 2.1], [0.47, 0.355, 0.032, 0.018, 2]],
  split: [0.17, 0.3], taper: 0.005,
  neck: [[-0.058, -0.003, 0.062, 0.046], [0.0, -0.004, 0.069, 0.055], [0.08, -0.004, 0.05, 0.045], [0.16, 0.002, 0.032, 0.034]],
  head: [[-0.014, 0.012, 0.021, 0.026], [0.02, 0.008, 0.042, 0.05], [0.06, 0.006, 0.034, 0.042], [0.1, -0.004, 0.021, 0.02], [0.154, -0.007, 0.008, 0.009]],
  ear: [[0, 0.021, 0.008], [0.35, 0.027, 0.0065], [1, 0.004, 0.004]],
  root: [[-0.022, 0.014, 0.016, 0.018], [0.07, 0.0, 0.04, 0.039], [0.165, -0.004, 0.056, 0.054]],
  mid: [[-0.012, -0.004, 0.055, 0.053], [0.07, -0.003, 0.06, 0.058], [0.152, -0.002, 0.05, 0.048]],
  tip: [[-0.01, -0.002, 0.051, 0.049], [0.06, 0, 0.04, 0.038], [0.134, 0, 0.006, 0.006]],
  thigh: [[-0.028, -0.013, 0.034, 0.021, 0.035], [0.02, -0.013, 0.056, 0.033, 0.041], [0.09, -0.006, 0.042, 0.028, 0.044], [0.142, 0.003, 0.026, 0.019, 0.045]],
  gaskin: [[-0.016, -0.004, 0.021, 0.016, 0.045], [0.03, -0.008, 0.026, 0.019, 0.044], [0.095, -0.006, 0.016, 0.0125, 0.04], [0.142, -0.005, 0.0135, 0.0115, 0.038]],
  hindCannon: [[-0.016, -0.008, 0.012, 0.01, 0.038], [0.02, -0.001, 0.0115, 0.0105, 0.038], [0.092, 0.002, 0.0115, 0.0135, 0.036]],
  paw: [[-0.008, -0.002, 0.012, 0.0115], [0.018, -0.005, 0.017, 0.016], [0.048, 0.003, 0.006, 0.012]],
  forearm: [[-0.02, -0.01, 0.015, 0.011, 0.039], [0.005, -0.004, 0.02, 0.0145, 0.039], [0.06, 0, 0.014, 0.012, 0.036], [0.124, 0, 0.0122, 0.0115, 0.033]],
  foreCannon: [[-0.012, 0, 0.011, 0.0105, 0.033], [0.03, 0, 0.0115, 0.012, 0.032], [0.071, 0.002, 0.012, 0.015, 0.032]],
  brush: ["brush", "brush-2", "brush-tip"], ears: ["ear-n", "ear-f"], dense: false, floor: 0.004,
};

/** 🦊️ Body of a coarse scale. The shoulder rows are [s, v, z, lateral half, sagittal half]: the first two on the scapula, the rest on the humerus. */
function body(S: Fox, t: Tables): void {
  const sm = (rows: Rows): Rows => (t.dense ? dense(rows, 2) : rows);
  const o = t.floor === undefined ? {} : { floor: t.floor };
  const T = sm(t.trunk).map((r) => [...r]);
  const hind = T.filter((r) => r[0]! <= t.split[1] + 1e-9).map((r) => [...r]);
  const fore = T.filter((r) => r[0]! >= t.split[0] - 1e-9).map((r) => [...r]);
  const nh = hind.filter((r) => r[0]! > t.split[0]).length;
  hind.slice(-nh).forEach((r, i) => { r[2]! -= t.taper * (i + 1); r[3]! -= t.taper * (i + 1); });
  const nf = fore.filter((r) => r[0]! < t.split[1]).length;
  fore.slice(0, nf).forEach((r, i) => { r[2]! -= t.taper * (nf - i); r[3]! -= t.taper * (nf - i); });
  S.hull("hindbody", "lumbar", hind, RED, { ...o, shape: "hindbody" });
  S.hull("chest", "thorax", fore, RED, { ...o, shape: "chest" });
  S.limb("neck", "neck", sm(t.neck), RED, { ...o, shape: "neck", e: 2.2 });
  if (t.bib !== undefined) S.limb("throat-bib", "neck", sm(t.bib), WHITE, { ...o, shape: "neck", e: 2.2 });
  S.limb("head", "head", sm(t.head.map((r) => [r[0]!, r[1]!, r[2]!, r[3]!, 0, r[4] ?? 2.4])), RED, { ...o, shape: "head" });
  if (t.jaw !== undefined) S.limb("jaw", "jaw", sm(t.jaw), WHITE, { ...o, frame: "head", shape: "head", e: 2.3 });
  S.limb("brush-root", "tail-1", sm(t.root), RED, { ...o, shape: t.brush[0], e: 2.1 });
  S.limb("brush-mid", "tail-2", sm(t.mid), RED, { ...o, shape: t.brush[1], e: 2.1 });
  S.limb("brush-tip", "tail-3", sm(t.tip), WHITE, { ...o, shape: t.brush[2], e: 2.1 });
  for (const sd of SIDES) {
    const n = side(sd);
    const k = sx(sd);
    ear(S, `${n}-ear`, sd, DARK, 0.092, t.ear, t.ears[sd < 0 ? 0 : 1]);
    S.limb(`${n}-thigh`, `thigh-h${k}`, sm(t.thigh), RED, { ...o, sd, shape: sd < 0 ? "thigh-n" : "hindbody" });
    S.limb(`${n}-gaskin`, `shank-h${k}`, sm(t.gaskin), RED, { ...o, sd, shape: `gaskin-${k}` });
    S.limb(`${n}-hind-cannon`, `metatarsus-h${k}`, t.hindCannon, DARK, { ...o, sd, shape: `cannon-h-${k}` });
    S.limb(`${n}-hind-paw`, `paw-h${k}`, t.paw.map((r) => [...r, HIND_Z]), DARK, { ...o, sd, shape: `paw-h-${k}`, e: 2.5 });
    if (t.shoulder !== undefined) {
      const sc = `scapula-f${k}`;
      const hm = `humerus-f${k}`;
      S.run(`${n}-shoulder`, hm, t.shoulder.map((r, i) => ({ p: S.at(i < 2 ? sc : hm, r[0]!, r[1]!, sd * r[2]!), rl: r[3]!, rs: r[4]!, e: 2.1 })), RED, { ...o, shape: "chest" });
    }
    S.limb(`${n}-forearm`, `forearm-f${k}`, sm(t.forearm), DARK, { ...o, sd, shape: `forearm-${k}` });
    S.limb(`${n}-fore-cannon`, `metacarpus-f${k}`, t.foreCannon, DARK, { ...o, sd, shape: `cannon-f-${k}` });
    S.limb(`${n}-fore-paw`, `paw-f${k}`, t.paw.map((r) => [r[0]!, r[1]!, r[2]!, r[3]! * 1.05, FORE_Z]), DARK, { ...o, sd, shape: `paw-f-${k}`, e: 2.5 });
  }
}

/** 🦴️ Skeleton at 1:500 (digital, no print floor): every bone one member at its real thickness, the column as one curved member with the fin of the withers, five ribs per side and the costal arch, joints at the rig joints, the two trunk masses. */
function structure500(K: Fox): Json[] {
  const o = { floor: 0.003 };
  const len = (bone: string): number => K.c.raw.rig.bones.find((b: Json) => b.id === bone).lengthM as number;
  const bar = (id: string, bone: string, z: number, sd: number, r0: number, rm: number, r1: number, short = 0): Json => mem(K.run(id, bone, [{ p: K.at(bone, 0, 0, sd * z), rl: r0, rs: r0 }, { p: K.at(bone, len(bone) * 0.5, 0, sd * z), rl: rm, rs: rm }, { p: K.at(bone, len(bone) - short, 0, sd * z), rl: r1, rs: r1 }], BONE, o), bone);
  const knob = (id: string, bone: string, c: V3, r: number, look: Look, part: string): Json => mem(K.run(id, bone, ([[-1, 0.3], [-0.7, 0.72], [0, 1], [0.7, 0.72], [1, 0.3]] as const).map(([t, k]) => ({ p: [c[0] + t * r, c[1], c[2]] as V3, rl: k * r, rs: k * r, e: 2 })), look, { floor: 0.002 }), bone, part);
  mem(K.limb("skull", "head", [[-0.002, 0.012, 0.016, 0.018], [0.03, 0.015, 0.0225, 0.0245], [0.075, 0.008, 0.016, 0.016], [0.148, -0.001, 0.007, 0.008]], BONE, { e: 2.3, ...o }), "head");
  mem(K.limb("mandible", "jaw", [[0.03, -0.012, 0.008, 0.026], [0.075, -0.021, 0.006, 0.019], [0.136, -0.018, 0.004, 0.006]], BONE, { frame: "head", ...o }), "jaw");
  mem(K.run("cervical-spine", "neck", [SPINE_C[0]!, SPINE_C[2]!, SPINE_C[4]!, SPINE_C[5]!].map((p, i) => ({ p, rl: i === 0 ? 0.014 : 0.009, rs: 0.008 })), BONE, o), "neck");
  mem(K.run("thoracic-spine", "thorax", [{ p: SPINE_T[0]!, rl: 0.007, rs: 0.008 }, { p: [0.38, 0.386, 0] as V3, rl: 0.0045, rs: 0.02 }, { p: [0.3, 0.399, 0] as V3, rl: 0.0045, rs: 0.013 }, { p: SPINE_T[3]!, rl: 0.007, rs: 0.0085 }], BONE, o), "thorax");
  mem(K.run("lumbar-spine", "lumbar", SPINE_L.map((p) => ({ p, rl: 0.018, rs: 0.008 })), BONE, o), "lumbar");
  mem(K.run("pelvis-girdle", "pelvis", [{ p: [0.072, 0.408, 0], rl: 0.035, rs: 0.012 }, { p: [0.03, 0.398, 0], rl: 0.03, rs: 0.01 }, { p: [0.0, 0.378, 0], rl: 0.034, rs: 0.009 }, { p: [-0.048, 0.374, 0], rl: 0.036, rs: 0.009 }], BONE, o), "pelvis");
  mem(K.run("sternum", "thorax", [STERNUM[0]!, STERNUM[1]!, STERNUM[2]!, STERNUM[4]!], BONE, o), "thorax");
  for (const [i, b] of ["tail-1", "tail-2", "tail-3"].entries()) mem(K.run(`caudal-${i + 1}`, b, [{ p: K.at(b, -0.004, 0), rl: 0.006 - i * 0.001, rs: 0.006 - i * 0.001 }, { p: K.at(b, len(b), 0), rl: 0.005 - i * 0.001, rs: 0.005 - i * 0.001 }], BONE, o), b);
  for (const sd of SIDES) {
    const n = side(sd);
    const k = sx(sd);
    mem(ear(K, `${n}-ear-cartilage`, sd, BONE, 0.086, [[0.02, 0.014, 0.004], [0.35, 0.02, 0.004], [1, 0.004, 0.004]]), `ear-${k}`);
    for (const i of [2, 4, 6, 8, 10]) mem(Object.assign(K.run(`${n}-rib-${i + 1}`, "thorax", ribPath(i, sd, 0.003), BONE, { floor: 0.003 }), { slender: true }), "thorax");
    mem(Object.assign(K.run(`${n}-costal-arch`, "thorax", [{ p: [0.3, 0.236, sd * 0.01], rl: 0.003, rs: 0.003 }, ...[9, 10, 11, 12].map((i) => ({ p: ribPath(i, sd, 0.003)[4]!.p, rl: 0.003, rs: 0.003 }))], BONE, { floor: 0.003 }), { slender: true }), "thorax");
    bar(`${n}-femur`, `thigh-h${k}`, 0.04, sd, 0.01, 0.0065, 0.0095);
    bar(`${n}-tibia`, `shank-h${k}`, 0.042, sd, 0.0095, 0.0055, 0.007);
    bar(`${n}-metatarsals`, `metatarsus-h${k}`, 0.037, sd, 0.0075, 0.006, 0.007);
    bar(`${n}-hind-digits`, `paw-h${k}`, HIND_Z, sd, 0.006, 0.005, 0.004, 0.004);
    mem(K.limb(`${n}-scapula`, `scapula-f${k}`, [[-0.003, 0, 0.024, 0.004, 0.03], [0.05, 0, 0.017, 0.004, 0.04], [0.094, 0, 0.009, 0.006, 0.045]], BONE, { sd, e: 2.6, ...o }), `scapula-f${k}`);
    bar(`${n}-humerus`, `humerus-f${k}`, 0.044, sd, 0.0095, 0.006, 0.009);
    bar(`${n}-radius-ulna`, `forearm-f${k}`, 0.037, sd, 0.008, 0.0058, 0.0072);
    bar(`${n}-metacarpals`, `metacarpus-f${k}`, 0.033, sd, 0.007, 0.0055, 0.0062);
    bar(`${n}-fore-digits`, `paw-f${k}`, FORE_Z, sd, 0.006, 0.005, 0.004, 0.004);
  }
  const R: Record<string, number> = { head: 0.008, thigh: 0.009, shank: 0.0095, metatarsus: 0.0075, humerus: 0.009, forearm: 0.008, metacarpus: 0.007 };
  for (const b of K.c.raw.rig.bones as Json[]) {
    const id = b.id as string;
    const stem = id.replace(/-[hf]?[nf]$/, "");
    if (R[stem] === undefined) continue;
    const sd = /-(h|f)?n$/.test(id) ? -1 : /-(h|f)?f$/.test(id) ? 1 : 0;
    const at = K.frame(id).o;
    knob(`joint-${id}`, id, [at[0]!, at[1]!, sd * (JOINT_Z[stem] ?? 0)], R[stem]!, JOINT, "joint");
  }
  knob("mass-thorax", "thorax", [0.33, 0.325, 0], 0.024, MASS, "mass");
  knob("mass-lumbar", "lumbar", [0.14, 0.355, 0], 0.019, MASS, "mass");
  return K.body;
}

const DECISIONS: Record<number, { en: string; de: string }[]> = {
  100: [
    { en: "Body at 1:100: 27 solids with their own sections for this scale (smallest feature 15 mm). Trunk in two lofts that taper inside each other where they overlap: level back, deep brisket, tucked flank; thigh per side wrapping the stifle; shoulder blade and upper arm as one rounded shoulder mass per side; neck with ruff and a white throat bib; red head with cheek ruffs, stop and long muzzle, and the white lower jaw along the lips, without eyes or nose pad; ears as cupped leaves at the 15 mm floor; the brush grows out of the croup in three links, the white tip last; per leg a red gaskin or a dark forearm, a dark cannon with the point of hock merged in, and a compact paw without toes.", de: "Körper bei 1:100: 27 Körper mit eigenen Schnitten für diesen Maßstab (kleinstes Merkmal 15 mm). Rumpf in zwei Lofts, die sich im Überlappungsbereich ineinander verjüngen: gerader Rücken, tiefe Brust, aufgezogene Flanke; je Seite eine Keule, die das Knie umschließt; Schulterblatt und Oberarm als eine gerundete Schultermasse je Seite; Hals mit Krause und weißem Kehllatz; roter Kopf mit Backenbart, Stop und langem Fang, der weiße Unterkiefer an den Lefzen, ohne Augen und Nasenspiegel; Ohren als gewölbte Blätter an der 15-mm-Grenze; die Rute wächst in drei Gliedern aus der Kruppe, zuletzt die weiße Spitze; je Bein ein roter Unterschenkel oder dunkler Unterarm, eine dunkle Röhre mit eingeschmolzenem Fersenhöcker und eine kompakte Pfote ohne Zehen." },
    { en: "Thermal at 1:100: each region is the coat surface of its body solids (single fur layer, references): trunk coat with neck, throat bib, thighs and shoulders; leg coat; face with the lower jaw; ears; brush; paws. Outlets on the surface: one nostril patch at the nose, the lips, the back strip for the plume.", de: "Thermik bei 1:100: jede Region ist die Felloberfläche ihrer Körper (eine Fellschicht, Verweise): Rumpffell mit Hals, Kehllatz, Keulen und Schultern; Beinfell; Gesicht mit Unterkiefer; Ohren; Rute; Pfoten. Auslässe auf der Oberfläche: ein Nasenlochfeld an der Nase, die Lippen, der Rückenstreifen für die Warmluftfahne." },
    { en: "Structure at 1:100: the skeleton simplified: skull, mandible, cervical, thoracic and lumbar spine, the fin of the withers, one pelvic girdle, sternum, seven of the thirteen ribs per side as slender solids at real size, caudal vertebrae, ear cartilages, scapula plate and one member per long bone, metapodial bundle and digit bundle of every leg; joints at the rig joints, masses of thorax and loin at their centres of mass. No supports are declared.", de: "Tragwerk bei 1:100: das Skelett vereinfacht: Schädel, Unterkiefer, Hals-, Brust- und Lendenwirbelsäule, der Kamm des Widerrists, ein Beckengürtel, Brustbein, sieben der dreizehn Rippen je Seite als schlanke Körper in wirklicher Größe, Schwanzwirbel, Ohrknorpel, Schulterblattplatte und je ein Stab pro Röhrenknochen, Mittelfuß- und Zehenbündel jedes Beins; Gelenke an den Gelenken des Skeletts, Massen von Brust und Lende in ihren Schwerpunkten. Auflager sind nicht deklariert." },
  ],
  200: [
    { en: "Body at 1:200: 25 solids with their own sections (smallest feature 30 mm). Trunk in two lofts that taper inside each other where they overlap, with level back, deep chest and tucked flank; one thigh mass and one rounded shoulder mass per side lie on the trunk and turn with their bones, so the trot stays closed; gaskin, forearm, cannons and paws stand at the 30 mm floor, so the legs are drawn stouter than the real 22 mm; head as one wedge with the pointed muzzle; ears as leaves 33 mm thick at the base tapering to the tip; the brush grows out of the croup in three links, the white tip last. Throat bib, jaw, eyes and toes are below this scale.", de: "Körper bei 1:200: 25 Körper mit eigenen Schnitten (kleinstes Merkmal 30 mm). Rumpf in zwei Lofts, die sich im Überlappungsbereich ineinander verjüngen, mit geradem Rücken, tiefer Brust und aufgezogener Flanke; je Seite liegen eine Keule und eine gerundete Schultermasse auf dem Rumpf und drehen mit ihren Knochen, der Trab bleibt so geschlossen; Unterschenkel, Unterarm, Röhren und Pfoten stehen an der 30-mm-Grenze, die Beine sind daher kräftiger als die wirklichen 22 mm gezeichnet; Kopf als ein Keil mit spitzem Fang; Ohren als Blätter, am Ansatz 33 mm dick und zur Spitze verjüngt; die Rute wächst in drei Gliedern aus der Kruppe, zuletzt die weiße Spitze. Kehllatz, Unterkiefer, Augen und Zehen liegen unter diesem Maßstab." },
    { en: "Thermal at 1:200: each region is the coat surface of its body solids (single fur layer, references); trunk coat is trunk, neck, thighs and shoulders, leg coat is gaskins, forearms and cannons. Outlets on the surface at the size floor: nose, lips, back strip.", de: "Thermik bei 1:200: jede Region ist die Felloberfläche ihrer Körper (eine Fellschicht, Verweise); Rumpffell sind Rumpf, Hals, Keulen und Schultern, Beinfell sind Unterschenkel, Unterarme und Röhren. Auslässe auf der Oberfläche an der Größengrenze: Nase, Lippen, Rückenstreifen." },
    { en: "Structure at 1:200: the load path as beams at the 30 mm floor: skull, mandible, cervical, thoracic and lumbar spine, one pelvic girdle, the sternum and four ribs per side at their real size as slender solids, caudal vertebrae, ear cartilages, scapula and one bar per leg bone; joints and the two trunk masses. No supports are declared.", de: "Tragwerk bei 1:200: der Lastpfad als Stäbe an der 30-mm-Grenze: Schädel, Unterkiefer, Hals-, Brust- und Lendenwirbelsäule, ein Beckengürtel, Brustbein, vier Rippen je Seite als schlanke Körper, Schwanzwirbel, Ohrknorpel, Schulterblatt und je ein Stab pro Beinknochen; Gelenke und die beiden Rumpfmassen. Auflager sind nicht deklariert." },
  ],
  500: [
    { en: "Body at 1:500 (digital only, the print drawing of this scale is unavailable): 23 solids at their real size with the fewest hand sections that still give the fox. Trunk in two lofts of nine sections that taper inside each other where they overlap, with level back, deep narrow chest and tucked flank, the shoulders merged into the chest; one thigh mass per side; neck with ruff; head as one wedge with the pointed muzzle; ears as thin leaves; red gaskin, dark forearm, cannon and paw of every leg at their real 22 to 30 mm; the brush grows out of the croup in three links, the white tip last. No eyes, nose pad, toes, bib or jaw.", de: "Körper bei 1:500 (nur digital, die Druckzeichnung dieses Maßstabs entfällt): 23 Körper in wirklicher Größe mit den wenigsten von Hand gesetzten Schnitten, die den Fuchs noch ergeben. Rumpf in zwei Lofts aus neun Schnitten, die sich im Überlappungsbereich ineinander verjüngen, mit geradem Rücken, tiefer schmaler Brust und aufgezogener Flanke, die Schultern in die Brust eingeschmolzen; je Seite eine Keule; Hals mit Krause; Kopf als ein Keil mit spitzem Fang; Ohren als dünne Blätter; roter Unterschenkel, dunkler Unterarm, Röhre und Pfote jedes Beins in ihren wirklichen 22 bis 30 mm; die Rute wächst in drei Gliedern aus der Kruppe, zuletzt die weiße Spitze. Keine Augen, kein Nasenspiegel, keine Zehen, kein Kehllatz, kein Unterkiefer." },
    { en: "Thermal at 1:500: each region is the coat surface of its body solids (single fur layer, references): trunk coat with neck and thighs; leg coat; face; ears; brush; paws. The outlets lie on the surface as one nostril patch, the lips and the back strip for the plume.", de: "Thermik bei 1:500: jede Region ist die Felloberfläche ihrer Körper (eine Fellschicht, Verweise): Rumpffell mit Hals und Keulen; Beinfell; Gesicht; Ohren; Rute; Pfoten. Die Auslässe liegen auf der Oberfläche als ein Nasenfleck, die Lippen und der Rückenstreifen für die Fahne." },
    { en: "Structure at 1:500: the skeleton as real slender bones, since no print floor applies: skull, mandible, cervical spine entering the chest low, thoracic spine under the topline with the fin of the withers, lumbar spine, pelvic girdle, sternum, five ribs per side and the costal arch as slender solids, caudal vertebrae, ear cartilages, scapula plate and one tapered member per long bone, metapodial and digit bundle of every leg; joints at head, hip, stifle, hock, shoulder, elbow and carpus, the masses of thorax and loin at their centres. No supports are declared.", de: "Tragwerk bei 1:500: das Skelett als wirklich schlanke Knochen, da keine Druckgrenze gilt: Schädel, Unterkiefer, tief in die Brust eintretende Halswirbelsäule, Brustwirbelsäule unter der Rückenlinie mit dem Kamm des Widerrists, Lendenwirbelsäule, Beckengürtel, Brustbein, fünf Rippen je Seite und der Rippenbogen als schlanke Körper, Schwanzwirbel, Ohrknorpel, Schulterblattplatte und ein verjüngter Stab je Röhrenknochen, Mittelfuß- und Zehenbündel jedes Beins; Gelenke an Kopf, Hüfte, Knie, Sprunggelenk, Schulter, Ellbogen und Vorderfußwurzel, die Massen von Brust und Lende in ihren Schwerpunkten. Es sind keine Auflager deklariert." },
  ],
};

if (import.meta.main) {
  const scale = Number(process.argv[2]);
  const t = scale === 100 ? T100 : scale === 200 ? T200 : T500;
  const S = new Fox(SPEC, scale);
  body(S, t);
  const fl = scale === 500 ? 0.004 : 0.56 * S.feature;
  const plume: readonly [V3, V3] = [[0.235, scale === 200 ? 0.428 : 0.432, 0], [0.1, 0.03, scale === 100 ? 0.009 : scale === 200 ? 0.0168 : 0.005]];
  const th = thermal(S, new Fox(SPEC, scale), { nostrilR: fl, mouthR: fl, plume, pair: false });
  const K = new Fox(SPEC, scale);
  const st = scale === 500 ? structure500(K) : coarseStructure(K, fl, scale === 100 ? [0, 2, 4, 6, 8, 10, 12] : [1, 4, 7, 10], scale === 100);
  if (scale === 200) {
    st.splice(st.findIndex((x) => x.id === "sternum"), 1);
    mem(Object.assign(K.run("sternum", "thorax", STERNUM.filter((_, j) => j !== 3).map((q) => ({ p: [q.p[0], q.p[1] + 0.024, 0] as V3, rl: 0.0035, rs: 0.0035 })), BONE, { floor: 0.0025 }), { slender: true }), "thorax");
  }
  console.log(`1:${scale} body ${S.body.length} thermal ${th.length} structure ${st.length}`);
  if (process.argv.includes("--write")) {
    saveLists(S.c, { body: S.list(), thermal: th, structure: st, decisions: DECISIONS[scale]! });
    writeFileSync(`${OUT}/shapes.${scale}.json`, JSON.stringify(S.shapes));
  }
}
