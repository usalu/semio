/** 🐈‍⬛️ Round 3 domestic cat at 1:100 and 1:200: each scale restated by hand for its own smallest feature (15 mm, 30 mm), not the 1:50 list with parts removed. 1:100 keeps every limb segment with simpler sections, one paw and one sole per foot and a head without separate whisker pads; 1:200 merges the trunk into two masses, each leg into upper, lower and foot, the tail into two pieces.
 * Usage: bun r3-domestic-cat-coarse.ts <100|200> [--write]
 * @see ./r3-domestic-cat-50.ts
 * @see ./r3-animals-domestic-cat.md */
import { saveLists } from "./kit-lib.ts";
import { BONE, COAT, EAR, EYE, FACE, LIMB, MASS, NOSE, PAD, TONGUE, ear, joints, mem, ribPath, spineY, thermal } from "./r3-domestic-cat-50.ts";

import { Cat, SIDES, dense, side, sx, type Json, type Look, type PP, type Row, type V3 } from "./r3-domestic-cat-kit.ts";

type Tables = Record<string, readonly Row[]>;
const B100: Tables = {
  trunk: [[-0.046, 0.226, 0.014, 0.016, 2], [-0.03, 0.222, 0.036, 0.037, 2.1], [0.0, 0.214, 0.049, 0.045, 2.2], [0.05, 0.2065, 0.06, 0.05, 2.2], [0.11, 0.198, 0.0685, 0.06, 2.2], [0.17, 0.192, 0.0715, 0.0635, 2.2], [0.225, 0.188, 0.07, 0.056, 2.3], [0.265, 0.188, 0.064, 0.048, 2.4], [0.295, 0.196, 0.052, 0.041, 2.2], [0.317, 0.21, 0.022, 0.016, 2]],
  neck: [[-0.035, 0.016, 0.02, 0.02], [0.0, 0.014, 0.045, 0.041], [0.035, 0.009, 0.044, 0.036], [0.065, 0.005, 0.035, 0.032], [0.088, 0.002, 0.012, 0.012]],
  head: [[-0.012, 0.014, 0.01, 0.012], [0.0, 0.0115, 0.03, 0.032], [0.022, 0.0075, 0.0395, 0.0405], [0.045, 0.003, 0.0395, 0.04], [0.06, -0.002, 0.031, 0.033], [0.072, -0.0065, 0.02, 0.022], [0.084, -0.009, 0.0135, 0.0155], [0.09, -0.01, 0.006, 0.008]],
  jaw: [[0.008, -0.022, 0.006, 0.016], [0.03, -0.0255, 0.009, 0.023], [0.058, -0.027, 0.008, 0.016], [0.079, -0.0265, 0.004, 0.006]],
  ear: [[0, 0.0215, 0.004], [0.5, 0.016, 0.004], [1, 0.003, 0.004]],
  thigh: [[-0.026, -0.008, 0.008, 0.006, 0.031], [-0.004, -0.009, 0.036, 0.017, 0.033], [0.03, -0.011, 0.043, 0.02, 0.036], [0.07, -0.009, 0.034, 0.0185, 0.039], [0.1, -0.002, 0.022, 0.015, 0.042], [0.115, 0.002, 0.006, 0.006, 0.042]],
  shank: [[-0.014, 0.003, 0.007, 0.007, 0.042], [0.004, -0.003, 0.0225, 0.0155, 0.042], [0.03, -0.007, 0.0225, 0.0155, 0.041], [0.07, -0.006, 0.0145, 0.0115, 0.037], [0.104, -0.005, 0.013, 0.0115, 0.034], [0.115, -0.004, 0.006, 0.006, 0.034]],
  hock: [[-0.021, -0.0105, 0.004, 0.005, 0.034], [-0.006, -0.0055, 0.0125, 0.011, 0.034], [0.02, 0, 0.0108, 0.012, 0.034], [0.066, 0.001, 0.0108, 0.014, 0.034], [0.077, 0.002, 0.006, 0.01, 0.034]],
  shoulder: [[-0.008, 0, 0.006, 0.004, 0.017], [0.012, -0.002, 0.024, 0.011, 0.025], [0.04, -0.003, 0.027, 0.013, 0.033], [0.066, 0.0, 0.019, 0.012, 0.037], [0.078, 0.003, 0.005, 0.005, 0.037]],
  arm: [[-0.015, -0.003, 0.006, 0.005, 0.037], [0.01, -0.008, 0.024, 0.0135, 0.038], [0.045, -0.0095, 0.0265, 0.0145, 0.04], [0.085, -0.004, 0.0175, 0.012, 0.04], [0.1, -0.003, 0.006, 0.006, 0.04]],
  forearm: [[-0.016, -0.0105, 0.006, 0.007, 0.04], [0.005, -0.003, 0.0175, 0.0135, 0.0395], [0.04, 0, 0.015, 0.013, 0.038], [0.085, -0.0015, 0.012, 0.0128, 0.0345], [0.095, -0.002, 0.005, 0.007, 0.034]],
  wrist: [[-0.008, -0.003, 0.008, 0.01, 0.034], [0.004, -0.0035, 0.0125, 0.0132, 0.034], [0.036, 0.001, 0.011, 0.0158, 0.034], [0.047, 0.002, 0.006, 0.011, 0.034]],
  pawH: [[-0.047, 0.013, 0.005, 0.007, 2.3], [-0.036, 0.014, 0.0125, 0.0145, 2.4], [-0.018, 0.0115, 0.011, 0.0165, 2.6], [-0.004, 0.0085, 0.008, 0.015, 2.6], [0.002, 0.007, 0.004, 0.009, 2.4]],
  pawF: [[-0.043, 0.013, 0.005, 0.008, 2.3], [-0.033, 0.014, 0.0125, 0.016, 2.4], [-0.017, 0.0115, 0.011, 0.018, 2.6], [-0.004, 0.0085, 0.008, 0.0165, 2.6], [0.002, 0.007, 0.004, 0.01, 2.4]],
};
const B200: Tables = {
  trunk: [[-0.045, 0.225, 0.02, 0.022, 2], [-0.02, 0.219, 0.042, 0.042, 2.1], [0.03, 0.21, 0.056, 0.05, 2.2], [0.1, 0.199, 0.067, 0.059, 2.2], [0.17, 0.192, 0.071, 0.0635, 2.2], [0.235, 0.188, 0.068, 0.055, 2.3], [0.285, 0.192, 0.057, 0.045, 2.3], [0.316, 0.208, 0.026, 0.02, 2]],
  neck: [[-0.03, 0.015, 0.025, 0.025], [0.005, 0.013, 0.045, 0.04], [0.05, 0.007, 0.04, 0.034], [0.085, 0.003, 0.016, 0.016]],
  head: [[-0.011, 0.013, 0.012, 0.014], [0.008, 0.0105, 0.034, 0.037], [0.035, 0.005, 0.04, 0.0405], [0.06, -0.003, 0.031, 0.032], [0.08, -0.01, 0.015, 0.02], [0.09, -0.011, 0.007, 0.011]],
  jaw: [[0.01, -0.0225, 0.007, 0.018], [0.04, -0.026, 0.009, 0.021], [0.078, -0.0265, 0.004, 0.006]],
  ear: [[0, 0.0215, 0.005], [1, 0.004, 0.005]],
  thigh: [[-0.024, -0.008, 0.012, 0.008, 0.031], [0.0, -0.009, 0.037, 0.018, 0.034], [0.04, -0.011, 0.042, 0.02, 0.037], [0.09, -0.004, 0.027, 0.0165, 0.041], [0.112, 0.002, 0.008, 0.008, 0.042]],
  shank: [[-0.012, 0.002, 0.01, 0.009, 0.042], [0.01, -0.004, 0.0225, 0.0155, 0.042], [0.06, -0.0065, 0.016, 0.0125, 0.038], [0.106, -0.005, 0.013, 0.0115, 0.034], [0.115, -0.004, 0.006, 0.006, 0.034]],
  shoulder: [[-0.007, 0, 0.007, 0.006, 0.02], [0.02, -0.002, 0.026, 0.0152, 0.027], [0.055, -0.001, 0.023, 0.0152, 0.034], [0.077, 0.003, 0.006, 0.006, 0.036]],
  arm: [[-0.013, -0.003, 0.008, 0.007, 0.037], [0.02, -0.009, 0.026, 0.0152, 0.038], [0.07, -0.006, 0.021, 0.0152, 0.039], [0.099, -0.003, 0.007, 0.007, 0.04]],
  forearm: [[-0.015, -0.01, 0.007, 0.008, 0.04], [0.008, -0.002, 0.0175, 0.0135, 0.0395], [0.06, 0, 0.0125, 0.0125, 0.037], [0.094, -0.002, 0.006, 0.008, 0.034]],
};

/** 🫁️ Trunk pieces of one table: each piece is full between its seams and continues 3 mm smaller inside its neighbours. */
function trunk(S: Cat, table: readonly Row[], pieces: readonly (readonly [id: string, bone: string, lo: number, hi: number, from: number, to: number])[]): void {
  const T = dense(table, 3);
  const rowAt = (x: number): number[] => {
    const i = Math.max(1, T.findIndex((r) => r[0]! >= x));
    const [p, q] = [T[i - 1]!, T[i]!];
    const k = (x - p[0]!) / (q[0]! - p[0]!);
    return p.map((v, c) => v + (q[c]! - v) * k);
  };
  const tuck = (r: readonly number[]): number[] => [r[0]!, r[1]!, r[2]! - 0.003, r[3]! - 0.003, r[4]!];
  for (const [id, bone, lo, hi, from, to] of pieces) {
    S.hull(id, bone, [
      ...T.filter((r) => r[0]! >= lo - 1e-9 && r[0]! < from - 0.006).map(tuck),
      ...(from > lo ? [tuck(rowAt(from - 0.004)), rowAt(from)] : []),
      ...T.filter((r) => r[0]! > from + 0.002 && r[0]! < to - 0.002 && r[0]! >= lo - 1e-9 && r[0]! <= hi + 1e-9),
      ...(to < hi ? [rowAt(to), tuck(rowAt(to + 0.004))] : []),
      ...T.filter((r) => r[0]! > to + 0.006 && r[0]! <= hi + 1e-9).map(tuck),
    ], COAT);
  }
}

/** 🪢️ One round tapering piece through world points with a dome at each end (tail pieces of several bones). */
function chain(S: Cat, id: string, bone: string, pts: readonly V3[], r: readonly number[], look: Look): Json {
  const dir = (a: V3, b: V3): V3 => { const d: V3 = [b[0] - a[0], b[1] - a[1], b[2] - a[2]]; const l = Math.hypot(...d) || 1; return [d[0] / l, d[1] / l, d[2] / l]; };
  const n = pts.length - 1;
  const t0 = dir(pts[1]!, pts[0]!);
  const t1 = dir(pts[n - 1]!, pts[n]!);
  const cap = (p: V3, t: V3, rr: number, c: number): PP => ({ p: [p[0] + t[0] * rr * c, p[1] + t[1] * rr * c, p[2] + t[2] * rr * c], rl: rr * Math.sqrt(Math.max(0.02, 1 - c * c)), rs: rr * Math.sqrt(Math.max(0.02, 1 - c * c)), e: 2 });
  return S.run(id, bone, [...[1, 0.85, 0.5].map((c) => cap(pts[0]!, t0, r[0]!, c)), ...pts.map((p, i) => ({ p, rl: r[i]!, rs: r[i]!, e: 2 })), ...[0.5, 0.85, 1].map((c) => cap(pts[n]!, t1, r[n]!, c))], look, { floor: 0.0004 });
}

function paw(S: Cat, id: string, bone: string, pawBone: string, rows: readonly Row[], sole: V3 | null): void {
  const tip = S.tipOf(pawBone);
  const zc = S.frame(pawBone).o[2]!;
  S.hull(id, bone, rows.map((r) => [r[0]!, r[1]!, r[2]!, r[3]!, r[4]!, zc]), LIMB, { x0: tip[0] + 0.0025, sd: 1 });
  if (sole !== null) S.ball(id.replace("paw", "sole"), bone, [tip[0] - 0.0155, sole[2], zc], sole, PAD, [1, 0, 0]);
}

/** 👅️ The tongue at the size floor of the coarse scales (15 mm thick), inside the closed mouth. */
function tongue(S: Cat): void {
  const hu = S.frame("head").u;
  S.ball("tongue", "jaw", S.at("head", 0.045, -0.0185, 0), [0.017, 0.008, 0.0076], TONGUE, [hu[0]!, hu[1]!, 0]);
}

/** 🥾️ Lower end of a leg at 1:200 as one round piece: metapodium from the joint (with the heel `heel` behind it) down to the ground and forward to the toes. */
function boot(S: Cat, id: string, bone: string, pawBone: string, sd: -1 | 1, heel: readonly [number, number, number] | null, r: number): Json {
  const z = S.frame(bone).o[2]!;
  const len = S.bone(bone).lengthM as number;
  const tip = S.tipOf(pawBone);
  const mtp = S.frame(pawBone).o as V3;
  const pts: PP[] = [
    ...(heel === null ? [] : [{ p: S.at(bone, heel[0], heel[1], z), rl: heel[2], rs: heel[2] }]),
    { p: S.at(bone, -0.004, -0.003, z), rl: r * 0.75, rs: r * 0.75 },
    { p: S.at(bone, 0.006, -0.002, z), rl: r, rs: r },
    { p: S.at(bone, len * 0.5, 0, z), rl: r, rs: r * 0.9 },
    { p: S.at(bone, len * 0.86, 0.003, z), rl: r, rs: r * 0.95 },
    { p: [mtp[0] + 0.011, 0.0138, z], rl: r, rs: r * 0.98 },
    { p: [tip[0] - 0.01, 0.012, z], rl: r, rs: r * 0.88 },
    { p: [tip[0] + 0.001, 0.0095, z], rl: r * 0.62, rs: r * 0.55 },
    { p: [tip[0] + 0.004, 0.0085, z], rl: r * 0.25, rs: r * 0.22 },
  ];
  void sd;
  return S.run(id, bone, pts, LIMB, { floor: 0.002 });
}

function head(S: Cat, B: Tables, o: { eyes: boolean; nose: boolean }): void {
  S.limb("neck", "neck", dense(B.neck!, 2), COAT, { e: 2.1 });
  S.limb("head", "head", dense(B.head!, 2), FACE, { e: 2.5 });
  S.limb("jaw", "jaw", dense(B.jaw!, 2), FACE, { frame: "head", e: 2.3 });
  const hu = S.frame("head").u;
  const hv = S.frame("head").v;
  if (o.nose) S.ball("nose", "head", S.at("head", 0.0885, -0.009, 0), [0.0052, 0.0066, 0.0056], NOSE, [hu[0]!, hu[1]!, 0]);
  for (const sd of SIDES) {
    if (o.eyes) S.ball(`eye-${sx(sd)}`, "head", S.at("head", 0.0665, 0.012, sd * 0.0195), [0.004, 0.0051, 0.0066], EYE, [hu[0]! * 0.62 + hv[0]! * 0.45, hu[1]! * 0.62 + hv[1]! * 0.45, sd * 0.64], [hv[0]!, hv[1]!, 0]);
    ear(S, `ear-${sx(sd)}`, sd, EAR, { rows: B.ear!, flat: true });
  }
}

function body100(S: Cat): void {
  const B = B100;
  trunk(S, B.trunk!, [["rump", "pelvis", -0.05, 0.08, -1, 0.04], ["hindbody", "lumbar", 0.0, 0.23, 0.04, 0.19], ["midbody", "thorax-a", 0.15, 0.29, 0.19, 0.25], ["chest", "thorax-b", 0.21, 0.32, 0.25, 1]]);
  head(S, B, { eyes: true, nose: true });
  tongue(S);
  const R = [0.015, 0.0132, 0.0122, 0.0112, 0.0104, 0.0096];
  ["tail-1", "tail-2", "tail-3", "tail-4", "tail-5"].forEach((b, i) => { const o = S.frame(b).o as V3; S.tubeDome(b, b, i === 0 ? [o[0] + 0.012, o[1] - 0.002, 0] : [o[0], o[1], 0], S.tipOf(b), R[i]!, R[i + 1]!, COAT, { n: 1 }); });
  for (const sd of SIDES) {
    const k = sx(sd);
    S.limb(`thigh-h${k}`, `thigh-h${k}`, dense(B.thigh!, 2), LIMB, { sd });
    S.limb(`shank-h${k}`, `shank-h${k}`, dense(B.shank!, 2), LIMB, { sd });
    S.limb(`hock-h${k}`, `metatarsus-h${k}`, dense(B.hock!, 2), LIMB, { sd, e: 2.4 });
    paw(S, `paw-h${k}`, `paw-h${k}`, `paw-h${k}`, B.pawH!, [0.019, 0.0155, 0.004]);
    S.limb(`shoulder-f${k}`, `scapula-f${k}`, dense(B.shoulder!, 2), LIMB, { sd });
    S.limb(`arm-f${k}`, `humerus-f${k}`, dense(B.arm!, 2), LIMB, { sd });
    S.limb(`forearm-f${k}`, `forearm-f${k}`, dense(B.forearm!, 2), LIMB, { sd });
    S.limb(`wrist-f${k}`, `metacarpus-f${k}`, dense(B.wrist!, 2), LIMB, { sd, e: 2.4 });
    paw(S, `paw-f${k}`, `paw-f${k}`, `paw-f${k}`, B.pawF!, [0.018, 0.0165, 0.004]);
  }
}

function body200(S: Cat): void {
  const B = B200;
  trunk(S, B.trunk!, [["hindbody", "lumbar", -0.05, 0.23, -1, 0.19], ["chest", "thorax-b", 0.15, 0.32, 0.19, 1]]);
  head(S, B, { eyes: false, nose: false });
  const o = (b: string): V3 => { const p = S.frame(b).o as V3; return [p[0], p[1], 0]; };
  const t1 = o("tail-1");
  chain(S, "tail-a", "tail-2", [[t1[0] + 0.014, t1[1] - 0.002, 0], o("tail-2"), o("tail-3"), o("tail-4")], [0.016, 0.0155, 0.015, 0.015], COAT);
  chain(S, "tail-b", "tail-4", [o("tail-4"), o("tail-5"), S.tipOf("tail-5")], [0.015, 0.015, 0.0145], COAT);
  for (const sd of SIDES) {
    const k = sx(sd);
    S.limb(`thigh-h${k}`, `thigh-h${k}`, dense(B.thigh!, 2), LIMB, { sd });
    S.limb(`lower-h${k}`, `shank-h${k}`, dense(B.shank!, 2), LIMB, { sd });
    boot(S, `foot-h${k}`, `metatarsus-h${k}`, `paw-h${k}`, sd, [-0.019, -0.01, 0.006], 0.013);
    S.limb(`shoulder-f${k}`, `humerus-f${k}`, dense(B.shoulder!, 2), LIMB, { sd, frame: `scapula-f${k}` });
    S.limb(`upper-f${k}`, `humerus-f${k}`, dense(B.arm!, 2), LIMB, { sd });
    S.limb(`lower-f${k}`, `forearm-f${k}`, dense(B.forearm!, 2), LIMB, { sd });
    boot(S, `foot-f${k}`, `metacarpus-f${k}`, `paw-f${k}`, sd, null, 0.013);
  }
}

/** Regions the body of a scale does not show, as own small thermal solids at their real size. */
function extras(scale: number): (T: Cat) => void {
  return (T) => {
    if (scale < 200) return;
    tongue(T);
    const hu = T.frame("head").u;
    const hv = T.frame("head").v;
    T.ball("nose", "head", T.at("head", 0.071, -0.009, 0), [0.0151, 0.0151, 0.0151], NOSE, [hu[0]!, hu[1]!, 0]);
    for (const sd of SIDES) {
      const k = sx(sd);
      T.ball(`eye-${k}`, "head", T.at("head", 0.047, 0.01, sd * 0.019), [0.0151, 0.0151, 0.0151], EYE, [hu[0]!, hu[1]!, 0], [hv[0]!, hv[1]!, 0]);
      for (const [leg, bone] of [["h", `metatarsus-h${k}`], ["f", `metacarpus-f${k}`]] as const) {
        const tip = T.tipOf(`paw-${leg}${k}`);
        T.ball(`sole-${leg}${k}`, bone, [tip[0] - 0.016, 0.0132, T.frame(bone).o[2]!], [0.013, 0.013, 0.013], PAD, [1, 0, 0]);
      }
    }
  };
}

const S100: Tables = {
  cranium: [[-0.008, 0.016, 0.012, 0.014], [0.012, 0.0155, 0.021, 0.023], [0.038, 0.012, 0.02, 0.028], [0.06, 0.003, 0.014, 0.018], [0.086, -0.006, 0.005, 0.006]],
  jaw: [[0.012, -0.007, 0.009, 0.021], [0.03, -0.02, 0.007, 0.019], [0.06, -0.024, 0.005, 0.01], [0.0765, -0.024, 0.003, 0.004]],
  scapula: [[-0.004, 0, 0.016, 0.003, 0.018], [0.015, 0, 0.019, 0.003, 0.025], [0.056, 0, 0.008, 0.0055, 0.034], [0.066, 0, 0.008, 0.0078, 0.037]],
  humerus: [[0, 0, 0.009, 0.008, 0.037], [0.03, 0, 0.0055, 0.0055, 0.039], [0.07, 0, 0.0055, 0.0055, 0.04], [0.091, 0, 0.008, 0.009, 0.04]],
  radius: [[-0.014, -0.009, 0.004, 0.004, 0.04], [0, -0.002, 0.008, 0.007, 0.04], [0.03, 0, 0.0055, 0.006, 0.039], [0.085, 0, 0.0065, 0.008, 0.034]],
  metacarpals: [[-0.003, -0.001, 0.0078, 0.009, 0.034], [0.02, 0, 0.0045, 0.0105, 0.034], [0.041, 0, 0.005, 0.012, 0.034]],
  femur: [[-0.004, 0, 0.008, 0.008, 0.03], [0.02, 0, 0.006, 0.006, 0.036], [0.075, 0, 0.006, 0.006, 0.041], [0.099, 0.001, 0.009, 0.0095, 0.042]],
  tibia: [[0, 0, 0.009, 0.0095, 0.042], [0.035, 0, 0.0052, 0.0055, 0.04], [0.09, 0, 0.005, 0.0052, 0.0355], [0.105, 0, 0.006, 0.007, 0.034]],
  metatarsals: [[-0.019, -0.0105, 0.004, 0.004, 0.034], [0.0, -0.001, 0.0075, 0.008, 0.034], [0.03, 0, 0.0045, 0.008, 0.034], [0.07, 0, 0.0045, 0.0105, 0.034]],
  digits: [[0.0, 0, 0.004, 0.0115, 0.034], [0.027, 0, 0.003, 0.012, 0.034]],
};
const S200: Tables = {
  cranium: [[-0.008, 0.016, 0.013, 0.015], [0.025, 0.014, 0.021, 0.025], [0.06, 0.003, 0.014, 0.018], [0.086, -0.006, 0.005, 0.006]],
  jaw: [[0.014, -0.012, 0.008, 0.02], [0.045, -0.023, 0.006, 0.014], [0.0765, -0.024, 0.003, 0.004]],
  scapula: [[-0.004, 0, 0.017, 0.003, 0.018], [0.064, 0, 0.007, 0.006, 0.037]],
  humerus: [[0, 0, 0.0085, 0.008, 0.037], [0.045, 0, 0.006, 0.006, 0.039], [0.091, 0, 0.008, 0.0085, 0.04]],
  radius: [[-0.01, -0.006, 0.006, 0.006, 0.04], [0.04, 0, 0.006, 0.0065, 0.038], [0.085, 0, 0.0065, 0.008, 0.034]],
  metacarpals: [[-0.003, -0.001, 0.006, 0.009, 0.034], [0.041, 0, 0.005, 0.0115, 0.034]],
  femur: [[-0.004, 0, 0.008, 0.008, 0.03], [0.05, 0, 0.0062, 0.0062, 0.039], [0.099, 0.001, 0.009, 0.0095, 0.042]],
  tibia: [[0, 0, 0.009, 0.0095, 0.042], [0.055, 0, 0.0055, 0.006, 0.038], [0.105, 0, 0.006, 0.007, 0.034]],
  metatarsals: [[-0.016, -0.009, 0.0045, 0.0045, 0.034], [0.003, -0.001, 0.0075, 0.008, 0.034], [0.07, 0, 0.0048, 0.0105, 0.034]],
  digits: [[0.0, 0, 0.004, 0.0115, 0.034], [0.027, 0, 0.0032, 0.012, 0.034]],
};
/** Rib hoops per side at 1:100: x of the spine end, half width, depth, caudal sweep. Each hoop is slender at its true 5 mm. */
const HOOPS: readonly Row[] = [[0.268, 0.032, 0.074, 0.008], [0.247, 0.039, 0.086, 0.017], [0.226, 0.044, 0.088, 0.024], [0.205, 0.047, 0.078, 0.029]];

function structure100(K: Cat): Json[] {
  const T = S100;
  mem(K.run("pelvis", "pelvis", [{ p: [0.05, 0.254, 0], rl: 0.027, rs: 0.008 }, { p: [0.02, 0.247, 0], rl: 0.028, rs: 0.009 }, { p: [0.0, 0.237, 0], rl: 0.033, rs: 0.009 }, { p: [-0.034, 0.23, 0], rl: 0.03, rs: 0.007 }], BONE, { floor: 0.002 }), "pelvis");
  mem(K.run("lumbar-spine", "lumbar", ([[0.045, 0.25], [0.11, 0.2535], [0.178, 0.246]] as const).map(([x, y]) => ({ p: [x, y, 0] as V3, rl: 0.012, rs: 0.0085 })), BONE), "lumbar");
  mem(K.run("thoracic-spine-a", "thorax-a", ([[0.178, 0.246], [0.212, 0.243], [0.245, 0.236]] as const).map(([x, y]) => ({ p: [x, y, 0] as V3, rl: 0.0085, rs: 0.0085 })), BONE), "thorax-a");
  mem(K.run("thoracic-spine-b", "thorax-b", ([[0.245, 0.236], [0.28, 0.2245], [0.305, 0.208]] as const).map(([x, y], i) => ({ p: [x, y + (i === 1 ? 0.003 : 0), 0] as V3, rl: 0.0085, rs: i === 1 ? 0.012 : 0.0085 })), BONE), "thorax-b");
  mem(K.run("cervical-spine", "neck", ([[0.305, 0.208], [0.325, 0.226], [0.335, 0.252], [0.3365, 0.27]] as const).map(([x, y]) => ({ p: [x, y, 0] as V3, rl: 0.011, rs: 0.0085 })), BONE), "neck");
  mem(K.run("sternum", "thorax-b", [{ p: [0.304, 0.168, 0], rl: 0.0078, rs: 0.0078 }, { p: [0.262, 0.142, 0], rl: 0.0078, rs: 0.006 }, { p: [0.212, 0.136, 0], rl: 0.006, rs: 0.005 }], BONE), "thorax-b");
  mem(K.limb("skull", "head", T.cranium!, BONE, { e: 2.4 }), "head");
  mem(K.limb("mandible", "jaw", T.jaw!, BONE, { frame: "head", e: 2.3 }), "jaw");
  ["tail-1", "tail-2", "tail-3", "tail-4", "tail-5"].forEach((b) => { const o = K.frame(b).o as V3; mem(K.tubeDome(`caudal-${b.slice(5)}`, b, [o[0], o[1], 0], K.tipOf(b), 0.0077, 0.0077, BONE, { n: 1 }), b); });
  for (const sd of SIDES) {
    const n = side(sd);
    const k = sx(sd);
    HOOPS.forEach(([x, w, d, sweep], i) => {
      const y = spineY(x!) - 0.004;
      const at = (r: number): PP[] => ([[0, 0, 0.22], [0.25, 0.14, 0.82], [0.6, 0.52, 1], [1, 1, 0.6]] as const).map(([a, b, c]) => ({ p: [x! - a * sweep!, y - b * d!, sd * c * w!] as V3, rl: r, rs: r }));
      const len = at(0).slice(1).reduce((s, q, j) => s + Math.hypot(q.p[0] - at(0)[j]!.p[0], q.p[1] - at(0)[j]!.p[1], q.p[2] - at(0)[j]!.p[2]), 0);
      mem(K.run(`${n}-rib-${i + 1}`, x! > 0.245 ? "thorax-b" : "thorax-a", at(Math.min(0.0025, len / 44)), BONE, { floor: 0.001 }), x! > 0.245 ? "thorax-b" : "thorax-a").slender = true;
    });
    mem(ear(K, `${n}-ear-cartilage`, sd, BONE, { len: 0.048, rows: [[0.04, 0.014, 0.004], [1, 0.003, 0.004]], flat: true }), `ear-${k}`);
    mem(K.limb(`${n}-scapula`, `scapula-f${k}`, T.scapula!, BONE, { sd, e: 2.6 }), `scapula-f${k}`);
    mem(K.limb(`${n}-humerus`, `humerus-f${k}`, T.humerus!, BONE, { sd }), `humerus-f${k}`);
    mem(K.limb(`${n}-radius-ulna`, `forearm-f${k}`, T.radius!, BONE, { sd }), `forearm-f${k}`);
    mem(K.limb(`${n}-metacarpals`, `metacarpus-f${k}`, T.metacarpals!, BONE, { sd, e: 2.6 }), `metacarpus-f${k}`);
    mem(K.limb(`${n}-fore-digits`, `paw-f${k}`, T.digits!, BONE, { sd, e: 2.8 }), `paw-f${k}`);
    mem(K.limb(`${n}-femur`, `thigh-h${k}`, T.femur!, BONE, { sd }), `thigh-h${k}`);
    mem(K.limb(`${n}-tibia`, `shank-h${k}`, T.tibia!, BONE, { sd }), `shank-h${k}`);
    mem(K.limb(`${n}-metatarsals`, `metatarsus-h${k}`, T.metatarsals!, BONE, { sd, e: 2.6 }), `metatarsus-h${k}`);
    mem(K.limb(`${n}-hind-digits`, `paw-h${k}`, T.digits!, BONE, { sd, e: 2.8 }), `paw-h${k}`);
  }
  joints(K, 1.1, () => true, 0.0077);
  for (const [b, at, r] of [["pelvis", [0.005, 0.232, 0], 0.0135], ["lumbar", [0.112, 0.214, 0], 0.0135], ["thorax-a", [0.21, 0.198, 0], 0.014], ["thorax-b", [0.272, 0.196, 0], 0.014]] as const) mem(K.ball(`mass-${b}`, b, [...at] as V3, [r, r, r], MASS), b, "mass");
  return K.body;
}

/** Member radius per bone at 1:200: the 30 mm print floor, or the thickness the drawing gives the limb where that is less. */
const R200: Record<string, number> = { thigh: 0.0152, shank: 0.0152, metatarsus: 0.013, paw: 0.0152, scapula: 0.0152, humerus: 0.0152, forearm: 0.0136, metacarpus: 0.013, tail: 0.0152 };

function structure200(K: Cat): Json[] {
  const rod = (id: string, bone: string, a: V3, b: V3, r: number, part = "member"): Json => mem(K.tubeDome(id, bone, a, b, r, r, BONE, { n: 1 }), bone, part);
  const o = (b: string): V3 => K.frame(b).o as V3;
  mem(K.limb("skull", "head", [[-0.006, 0.014, 0.014, 0.016], [0.02, 0.013, 0.022, 0.026], [0.055, 0.004, 0.017, 0.02], [0.082, -0.006, 0.008, 0.01]], BONE, { e: 2.4 }), "head");
  mem(K.limb("mandible", "jaw", [[0.012, -0.014, 0.008, 0.02], [0.045, -0.0235, 0.0078, 0.014], [0.0765, -0.024, 0.004, 0.005]], BONE, { frame: "head", e: 2.3 }), "jaw");
  rod("cervical-spine", "neck", [0.309, 0.213, 0], [0.334, 0.262, 0], 0.0152);
  rod("thoracic-spine-b", "thorax-b", [0.25, 0.232, 0], [0.3, 0.211, 0], 0.0152);
  rod("thoracic-spine-a", "thorax-a", [0.183, 0.243, 0], [0.24, 0.235, 0], 0.0152);
  rod("lumbar-spine", "lumbar", [0.05, 0.247, 0], [0.173, 0.244, 0], 0.0152);
  mem(K.run("pelvis", "pelvis", [{ p: [0.046, 0.25, 0], rl: 0.028, rs: 0.0152 }, { p: [0.0, 0.238, 0], rl: 0.033, rs: 0.0152 }, { p: [-0.03, 0.231, 0], rl: 0.029, rs: 0.0152 }], BONE), "pelvis");
  mem(K.run("sternum", "thorax-b", [{ p: [0.304, 0.168, 0], rl: 0.0025, rs: 0.0025 }, { p: [0.262, 0.142, 0], rl: 0.0025, rs: 0.0025 }, { p: [0.207, 0.136, 0], rl: 0.0025, rs: 0.0025 }], BONE, { floor: 0.001 }), "thorax-b").slender = true;
  ["tail-1", "tail-2", "tail-3", "tail-4", "tail-5"].forEach((b) => rod(`caudal-${b.slice(5)}`, b, [o(b)[0], o(b)[1], 0], K.tipOf(b), R200.tail!));
  for (const sd of SIDES) {
    const n = side(sd);
    const k = sx(sd);
    mem(ear(K, `${n}-ear-cartilage`, sd, BONE, { len: 0.048, rows: [[0.04, 0.014, 0.005], [1, 0.004, 0.005]], flat: true }), `ear-${k}`);
    for (const i of [2, 6, 10]) mem(K.run(`${n}-rib-${i + 1}`, i < 6 ? "thorax-b" : "thorax-a", ribPath(i, sd, 0.002), BONE, { floor: 0.001 }), i < 6 ? "thorax-b" : "thorax-a").slender = true;
    for (const [name, bone] of [["scapula", `scapula-f${k}`], ["humerus", `humerus-f${k}`], ["radius-ulna", `forearm-f${k}`], ["metacarpals", `metacarpus-f${k}`], ["fore-digits", `paw-f${k}`], ["femur", `thigh-h${k}`], ["tibia", `shank-h${k}`], ["metatarsals", `metatarsus-h${k}`], ["hind-digits", `paw-h${k}`]] as const) {
      const r = R200[bone.split("-")[0]!]!;
      const tip = K.tipOf(bone);
      const from = o(bone);
      const digit = bone.startsWith("paw");
      rod(`${n}-${name}`, bone, digit ? [from[0], r, from[2]] : from, digit ? [tip[0] - r * 0.6, r, tip[2]] : tip, r);
    }
  }
  for (const b of K.c.raw.rig.bones as Json[]) {
    const stem = (b.id as string).split("-")[0]!;
    if (!/^(neck|head|thigh|shank|metatarsus|humerus|forearm|metacarpus)$/.test(stem)) continue;
    const r = (R200[stem] ?? 0.0152) + 0.0008;
    mem(K.ball(`joint-${b.id}`, b.id, [...o(b.id)] as V3, [r, r, r], { material: "bone", tone: 0.5 }), b.id, "joint");
  }
  for (const [b, at] of [["pelvis", [0.005, 0.222, 0]], ["lumbar", [0.112, 0.21, 0]], ["thorax-a", [0.21, 0.198, 0]], ["thorax-b", [0.272, 0.19, 0]]] as const) mem(K.ball(`mass-${b}`, b, [...at] as V3, [0.0155, 0.0155, 0.0155], MASS), b, "mass");
  return K.body;
}

const DECISIONS: Record<number, { en: string; de: string }[]> = {
  100: [
    {
      en: "Body at 1:100 (smallest feature 15 mm), restated for this scale: the trunk keeps its four pieces on pelvis, loin and the two halves of the ribcage, with fewer and simpler sections; neck, skull and mandible are single lofts, the whisker pads are merged into the muzzle, nose leather, eyes and ears stay because the drawing shows them. Every limb segment keeps its own solid with the muscle mass in a five-row table, each paw is one rounded body on one sole pad (toes and single digital pads are 9 mm and merged). The tail keeps one round piece per bone. Legs, tail and ears are thinner than 15 mm in life and stay at their true size, as the drawing draws them.",
      de: "Körper bei 1:100 (kleinstes Merkmal 15 mm), für diesen Maßstab neu gesetzt: der Rumpf behält seine vier Teile an Becken, Lende und den beiden Brustkorbhälften mit weniger und einfacheren Schnitten; Hals, Schädel und Unterkiefer sind einzelne Lofts, die Schnurrhaarkissen sind in die Schnauze eingegangen, Nasenspiegel, Augen und Ohren bleiben, weil die Zeichnung sie zeigt. Jedes Gliedmaßensegment behält seinen eigenen Körper mit der Muskelmasse in einer fünfzeiligen Tabelle, jede Pfote ist ein gerundeter Körper auf einem Sohlenballen (Zehen und einzelne Zehenballen messen 9 mm und sind zusammengefasst). Der Schwanz behält ein rundes Stück je Knochen. Läufe, Schwanz und Ohren sind in Wirklichkeit dünner als 15 mm und bleiben in wahrer Größe, wie die Zeichnung sie zeichnet.",
    },
    {
      en: "Thermal at 1:100: each fur region is the body surface it covers (references), nose leather, corneas and the four soles are the body solids of their regions, the tongue is an own solid inside the closed mouth. Outlets: one nostril patch on the nose leather, the lips, a patch of the back for the plume.",
      de: "Thermik bei 1:100: jede Fellregion ist die Körperoberfläche, die sie bedeckt (Verweise), Nasenspiegel, Hornhäute und die vier Sohlen sind die Körper ihrer Regionen, die Zunge ist ein eigener Körper im geschlossenen Maul. Auslässe: ein Nasenlochfeld im Nasenspiegel, die Lippen, ein Rückenfeld für die Warmluftfahne.",
    },
    {
      en: "Structure at 1:100: the skeleton simplified for 15 mm. Skull, mandible, cervical, two thoracic and the lumbar spine pieces, the pelvis with sacrum as one plate, seven rib hoops per side for thirteen ribs, sternum, five caudal rods, ear cartilages; scapula, humerus, radius with ulna, metacarpal bundle and digits per foreleg; femur, tibia, metatarsal bundle with the heel and digits per hind leg. Joints at every rig joint, four trunk masses at their centres of mass. Bones stay at their true thickness (5 to 12 mm), below the feature size, because the 15 mm floor would fill the leg.",
      de: "Tragwerk bei 1:100: das Skelett, für 15 mm vereinfacht. Schädel, Unterkiefer, Hals-, zwei Brust- und Lendenwirbelsäulenstücke, das Becken mit Kreuzbein als eine Platte, sieben Rippenbögen je Seite für dreizehn Rippen, Brustbein, fünf Schwanzwirbelstäbe, Ohrknorpel; Schulterblatt, Oberarmbein, Speiche mit Elle, Mittelhandbündel und Zehen je Vorderbein; Oberschenkelbein, Schienbein, Mittelfußbündel mit Ferse und Zehen je Hinterbein. Gelenke an jedem Gelenk des Skeletts, vier Rumpfmassen in ihren Schwerpunkten. Die Knochen bleiben in wahrer Dicke (5 bis 12 mm) unter der Merkmalgröße, weil 15 mm den Lauf ausfüllen würden.",
    },
  ],
  200: [
    {
      en: "Body at 1:200 (smallest feature 30 mm; the cat is 3.8 mm long on paper), generalised: the trunk is two masses, hindquarters with loin on the lumbar bone and ribcage with chest on the fore thorax; neck, head and jaw are three coarse lofts, ears two leaves; eyes, nose and pads are not shown. Each hind leg is thigh, lower leg and foot with its paw; each foreleg shoulder, upper arm, lower leg and foot with its paw, bound to the bones the 1:200 drawing draws. The tail is two pieces over its five bones. Legs and tail are thinner than 30 mm in life and stay at true size.",
      de: "Körper bei 1:200 (kleinstes Merkmal 30 mm; die Katze ist auf dem Papier 3,8 mm lang), verallgemeinert: der Rumpf besteht aus zwei Massen, Hinterhand mit Lende am Lendenknochen und Brustkorb mit Brust am vorderen Brustknochen; Hals, Kopf und Unterkiefer sind drei grobe Lofts, die Ohren zwei Blätter; Augen, Nase und Ballen werden nicht gezeigt. Jedes Hinterbein besteht aus Oberschenkel, Unterschenkel und Fuß mit Pfote; jedes Vorderbein aus Schulter, Oberarm, Unterarm und Fuß mit Pfote, gebunden an die Knochen, die die Zeichnung 1:200 zeichnet. Der Schwanz besteht aus zwei Stücken über seinen fünf Knochen. Läufe und Schwanz sind in Wirklichkeit dünner als 30 mm und bleiben in wahrer Größe.",
    },
    {
      en: "Thermal at 1:200: the fur regions are the body surfaces (references); nose leather, corneas, tongue and the four soles are too small for the body of this scale and are own solids at their real size, so every region keeps a solid. Outlets: nostril patch, lips, back patch.",
      de: "Thermik bei 1:200: die Fellregionen sind die Körperoberflächen (Verweise); Nasenspiegel, Hornhäute, Zunge und die vier Sohlen sind für den Körper dieses Maßstabs zu klein und sind eigene Körper in wahrer Größe, damit jede Region einen Körper behält. Auslässe: Nasenlochfeld, Lippen, Rückenfeld.",
    },
    {
      en: "Structure at 1:200: one member per rig bone at the 30 mm print floor (or the thickness the drawing gives the limb where that is less): skull, mandible, four spine rods under the topline, pelvis plate, five caudal rods, ear cartilages and the long bones of the four legs with metapodial and digit rods; three ribs per side and the sternum at their real size as slender solids. Joints only at neck, head and the large limb joints, four trunk masses.",
      de: "Tragwerk bei 1:200: ein Stab je Rig-Knochen an der 30-mm-Druckgrenze (oder in der Dicke, die die Zeichnung dem Bein gibt, wo diese kleiner ist): Schädel, Unterkiefer, vier Wirbelsäulenstäbe unter der Rückenlinie, Beckenplatte, fünf Schwanzstäbe, Ohrknorpel und die Röhrenknochen der vier Beine mit Mittelfuß- und Zehenstäben; drei Rippen je Seite und das Brustbein in wirklicher Größe als schlanke Körper. Gelenke nur an Hals, Kopf und den großen Gliedmaßengelenken, vier Rumpfmassen.",
    },
  ],
};

if (import.meta.main) {
  const scale = Number(process.argv[2]) as 100 | 200;
  const S = new Cat(scale);
  if (scale === 100) body100(S);
  else body200(S);
  const th = thermal(S, new Cat(scale), { extra: extras(scale) });
  const st = scale === 100 ? structure100(new Cat(scale)) : structure200(new Cat(scale));
  console.log(`1:${scale} body ${S.body.length} thermal ${th.length} structure ${st.length}`);
  if (process.argv.includes("--write")) saveLists(S.c, { body: S.body, thermal: th, structure: st, decisions: DECISIONS[scale]! });
}
