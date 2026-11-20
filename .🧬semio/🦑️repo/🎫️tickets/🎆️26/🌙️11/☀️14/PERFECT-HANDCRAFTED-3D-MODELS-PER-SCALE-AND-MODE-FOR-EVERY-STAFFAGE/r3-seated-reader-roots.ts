/** 🦴️ Root fixes of `humans/seated-reader` (round 3), stated as final values and solved again on every run (idempotent): the rig (a ground root with the two end frames, the raked seat and the backrest of a real park bench as fixed bones; hip and shoulder joint spacing, lumbosacral joint behind the hip joints, C7 and the atlanto-occipital joint at their heights, shank length, ankle in a sneaker with a sole; the notebook lying in the left palm with its page turning over the top towards the reader; a cup that weighs something), the anchors, every pose and every baked clip key solved in three dimensions with the product's own chain (`solveChain`), the dimensions and the texts that named the old construction.
 * Usage: `bun r3-seated-reader-roots.ts [--print]` (as module: `rooted()` gives the catalog document with these fixes).
 * @see ./r3-seated-reader-ref.ts */
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";

import { solveChain } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts";
import { BENCH, CUP, HIP_Y, PAD, WOMAN, backFace, type V3 } from "./r3-seated-reader-ref.ts";

export type Json = Record<string, any>;
export const ASSET = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/seated-reader/🔣️.json";
export type Frame = { o: V3; u: V3; v: V3; w: V3 };
type Offsets = Record<string, Record<string, number>>;
/** 🗃️ The solved poses and clip keys of the last run, reused while rig and goals are unchanged (the solve takes about a minute). */
const CACHE = join(import.meta.dir, "🗑️generated", "r3-seated-reader", "solved.json");

const rad = (d: number): number => (d * Math.PI) / 180;
const deg = (r: number): number => (r * 180) / Math.PI;
const add = (a: V3, b: V3, s = 1): V3 => [a[0] + s * b[0], a[1] + s * b[1], a[2] + s * b[2]];
const unit = (a: V3): V3 => { const l = Math.hypot(...a); return [a[0] / l, a[1] / l, a[2] / l]; };
const r3 = (x: number): number => Math.round(x * 1e3) / 1e3;
const r4 = (x: number): number => Math.round(x * 1e4) / 1e4;
export const at = (f: Frame, p: V3): V3 => add(add(add(f.o, f.u, p[0]), f.v, p[1]), f.w, p[2]);

export function framesOf(rig: Json, offsets: Offsets): Record<string, Frame> {
  return Object.fromEntries(Object.entries(solveChain(rig as never, offsets as never, { x: 0, y: 0, angleDeg: 0 })).map(([id, b]) => [id, (b as Json).frame as Frame]));
}

/* ---------------------------------------------------------------- rig */

export const THIGH_DROP = deg(Math.asin((HIP_Y - WOMAN.ankleY - WOMAN.shank) / WOMAN.thigh));
export const FOOT_DROP = deg(Math.atan2(WOMAN.ankleY, WOMAN.ballAhead));
const FOOT_LENGTH = r4(Math.hypot(WOMAN.ankleY, WOMAN.ballAhead));
export const ANKLE_X = r4(WOMAN.thigh * Math.cos(rad(THIGH_DROP)));
const T = (en: string, de: string): Json => ({ en, de });
const FIXED = { type: "fixed" };

/** 🧱️ The bones of the bench, new or restated: a ground root, the two end frames as columns from the rear foot to the top of the back post, the seat from its rear to its front edge on the rake, the backrest up the posts. */
const BENCH_BONES: Json[] = [
  { id: "ground", parent: null, title: T("Ground under the bench", "Boden unter der Bank"), x: 0, y: 0, angleDeg: 0, lengthM: 0, joint: FIXED },
  { id: "frame-l", parent: "ground", title: T("Left end frame", "Linke Wange"), x: BENCH.footX, y: 0, angleDeg: 90, lengthM: BENCH.top, joint: FIXED, z: BENCH.frameZ[0] },
  { id: "frame-r", parent: "ground", title: T("Right end frame", "Rechte Wange"), x: BENCH.footX, y: 0, angleDeg: 90, lengthM: BENCH.top, joint: FIXED, z: BENCH.frameZ[1] },
  { id: "seat", parent: "ground", title: T("Seat slats", "Sitzlatten"), x: BENCH.seatRear[0], y: BENCH.seatRear[1], angleDeg: BENCH.rakeDeg, lengthM: BENCH.seatDepth, joint: FIXED },
  { id: "backrest", parent: "ground", title: T("Back slats", "Rückenlatten"), x: r4(backFace(BENCH.backFrom)), y: BENCH.backFrom, angleDeg: 90 + BENCH.leanDeg, lengthM: BENCH.backLength, joint: FIXED },
];
const BONES: Record<string, Json> = {
  "hip-lift": { parent: "ground", x: 0, y: HIP_Y },
  pelvis: { lengthM: WOMAN.pelvis },
  torso: { x: WOMAN.pelvis, y: 0.033, angleDeg: -2, lengthM: WOMAN.torso, "mass.comX": r4(WOMAN.torso * 0.4557), "mass.comY": 0 },
  neck: { x: WOMAN.torso, angleDeg: -16, lengthM: WOMAN.neck, "mass.comX": r4(WOMAN.neck / 2) },
  head: { x: WOMAN.neck, angleDeg: 18, lengthM: WOMAN.head, "mass.comX": 0.0245, "mass.comY": -0.015 },
  "arm-l": { x: WOMAN.shoulderOnTorso, y: -WOMAN.shoulderAhead, angleDeg: 182, z: WOMAN.shoulderZ },
  "arm-r": { x: WOMAN.shoulderOnTorso, y: -WOMAN.shoulderAhead, angleDeg: 182, z: -WOMAN.shoulderZ },
  "thigh-l": { angleDeg: r4(-90 - THIGH_DROP), z: WOMAN.hipZ },
  "thigh-r": { angleDeg: r4(-90 - THIGH_DROP), z: -WOMAN.hipZ },
  pad: { x: PAD.inHand[0], y: PAD.inHand[1], z: PAD.inHand[2], angleDeg: 0, lengthM: PAD.length, "joint.minDeg": -30, "joint.maxDeg": 30, "mass.kg": 0.25, "mass.comX": r4(PAD.length / 2), title: T("Top-bound A5 notebook in the left palm", "Oben gebundenes A5-Notizbuch in der linken Handfläche") },
  page: { x: PAD.length, angleDeg: 180, lengthM: PAD.page, "joint.minDeg": -5, "joint.maxDeg": 360, "mass.kg": 0.0025, "mass.comX": r4(PAD.page / 2), "mass.source": "authored-estimate" },
  cup: { x: CUP.alongSeat, y: 0, z: CUP.z, angleDeg: 90, lengthM: r4(CUP.height + CUP.lid), "mass.kg": 0.27, "mass.comX": 0.05, title: T("Paper cup on the seat beside her", "Pappbecher auf der Sitzfläche neben ihr") },
};
for (const s of ["l", "r"]) {
  BONES[`shank-${s}`] = { angleDeg: r4(-90 + THIGH_DROP), lengthM: WOMAN.shank, "mass.comX": 0.1843 };
  BONES[`foot-${s}`] = { x: WOMAN.shank, angleDeg: r4(90 - FOOT_DROP), lengthM: FOOT_LENGTH, "mass.comX": r4(0.0747 * (FOOT_LENGTH / 0.1754)), "mass.comY": -0.003 };
  BONES[`toe-${s}`] = { x: FOOT_LENGTH, angleDeg: r4(FOOT_DROP) };
}

/* ---------------------------------------------------------------- anchors */

const HEEL_LOCAL = ((): [number, number] => {
  const [c, s] = [Math.cos(rad(-FOOT_DROP)), Math.sin(rad(-FOOT_DROP))];
  const d: [number, number] = [-0.042, -WOMAN.ankleY];
  return [r4(d[0] * c + d[1] * s), r4(-d[0] * s + d[1] * c)];
})();
const ANCHORS: Record<string, Json> = {
  seat: { x: -0.09, y: 0.02, footprintLengthM: 0.16, footprintWidthM: 0.3 },
  "thigh-support": { x: 0.17, y: -0.07, z: WOMAN.hipZ, footprintLengthM: 0.2, footprintWidthM: 0.34 },
  backrest: { x: 0.3, y: 0.09, footprintLengthM: 0.24, footprintWidthM: 0.34 },
  "heel-l": { x: HEEL_LOCAL[0], y: HEEL_LOCAL[1], footprintLengthM: 0.07, footprintWidthM: 0.065 },
  "heel-r": { x: HEEL_LOCAL[0], y: HEEL_LOCAL[1], footprintLengthM: 0.07, footprintWidthM: 0.065 },
  "ball-l": { x: FOOT_LENGTH, y: 0, footprintLengthM: 0.095, footprintWidthM: 0.09 },
  "ball-r": { x: FOOT_LENGTH, y: 0, footprintLengthM: 0.095, footprintWidthM: 0.09 },
  mouth: { x: -0.056, y: -0.097 },
  nostrils: { x: -0.034, y: -0.094 },
  crown: { x: 0.131, y: 0.008 },
  "pad-grip": { x: r4(PAD.length / 2), y: 0 },
};

/* ---------------------------------------------------------------- solver */

type Param = [bone: string, field: "angleDeg" | "abductionDeg" | "twistDeg", min: number, max: number];
/** 🧮️ Levenberg-Marquardt over a few joint offsets: `residual` reads the solved frames, the offsets stay inside their limits. */
function solve(rig: Json, offsets: Offsets, params: readonly Param[], residual: (f: Record<string, Frame>) => number[], label: string, report: string[]): void {
  const get = (): number[] => params.map(([b, k]) => offsets[b]?.[k] ?? 0);
  const set = (x: readonly number[]): void => params.forEach(([b, k, lo, hi], i) => { (offsets[b] ??= {})[k] = Math.min(Math.max(x[i]!, lo), hi); });
  const cost = (r: readonly number[]): number => r.reduce((s, e) => s + e * e, 0);
  let x = get();
  set(x);
  let r = residual(framesOf(rig, offsets));
  let lambda = 1e-2;
  for (let step = 0; step < 120 && cost(r) > 1e-13; step++) {
    const J = params.map((_, i) => {
      const y = [...x];
      y[i]! += 1e-3;
      set(y);
      const q = residual(framesOf(rig, offsets));
      return q.map((e, k) => (e - r[k]!) / 1e-3);
    });
    const n = params.length;
    const A = Array.from({ length: n }, (_, i) => Array.from({ length: n }, (_, j) => J[i]!.reduce((s, e, k) => s + e * J[j]![k]!, 0)));
    const g = J.map((row) => -row.reduce((s, e, k) => s + e * r[k]!, 0));
    let better = false;
    for (let attempt = 0; attempt < 12 && !better; attempt++) {
      const M = A.map((row, i) => [...row.map((e, j) => (i === j ? e * (1 + lambda) + 1e-12 : e)), g[i]!]);
      for (let i = 0; i < n; i++) {
        let p = i;
        for (let k = i + 1; k < n; k++) if (Math.abs(M[k]![i]!) > Math.abs(M[p]![i]!)) p = k;
        [M[i], M[p]] = [M[p]!, M[i]!];
        for (let k = i + 1; k < n; k++) {
          const f = M[k]![i]! / M[i]![i]!;
          for (let j = i; j <= n; j++) M[k]![j]! -= f * M[i]![j]!;
        }
      }
      const d = new Array<number>(n).fill(0);
      for (let i = n - 1; i >= 0; i--) d[i] = (M[i]![n]! - M[i]!.slice(i + 1, n).reduce((s, e, j) => s + e * d[i + 1 + j]!, 0)) / M[i]![i]!;
      const y = x.map((e, i) => e + Math.max(-25, Math.min(25, d[i]!)));
      set(y);
      const q = residual(framesOf(rig, offsets));
      if (cost(q) < cost(r)) {
        x = get();
        r = q;
        lambda = Math.max(lambda / 3, 1e-7);
        better = true;
      } else lambda *= 4;
    }
    if (!better) break;
  }
  set(x);
  params.forEach(([b, k], i) => { offsets[b]![k] = r3(x[i]!); });
  report.push(`${label}: rms ${(Math.sqrt(cost(residual(framesOf(rig, offsets))) / r.length) * 1000).toFixed(2)} mm  ${params.map(([b, k], i) => `${b}.${k.replace("Deg", "")} ${x[i]!.toFixed(1)}`).join("  ")}`);
}


/* ---------------------------------------------------------------- poses */

/** 📓️ Where the notebook is: its middle relative to the hip joints (ahead, up), the rise of its long axis from the horizontal, its lateral position; the reading face looks back up at the reader. */
type PadSpec = { c: readonly [number, number]; tilt: number; z: number };
/** ✋️ Where a free hand goes: the wrist, the direction of the hand and (optionally) of the back of the hand, in the world. */
type HandGoal = { wrist: V3; u: V3; v?: V3; k?: number };
type Spec = { hipShift: number; hipLift: number; torso: number; neck: number; head: number; pad: PadSpec; page?: number; right?: (f: Record<string, Frame>) => HandGoal };

const STAND = { lift: r4(WOMAN.ankleY + Math.sqrt((WOMAN.thigh + WOMAN.shank) ** 2 - 0.02 ** 2) - 0.004 - HIP_Y), shift: r4(ANKLE_X + 0.02) };
const LEAN_SHIFT = 0.08;
const POSES: Record<string, Spec> = {
  "seated-upright": { hipShift: 0, hipLift: 0, torso: -4, neck: -10, head: -18, pad: { c: [0.33, 0.296], tilt: 35, z: 0 } },
  "reclined-reading": { hipShift: -0.03, hipLift: 0, torso: 15.5, neck: -12, head: -8, pad: { c: [0.22, 0.6], tilt: 75, z: 0 } },
  "reaching-for-cup": { hipShift: 0, hipLift: 0, torso: -6, neck: -8, head: -12, pad: { c: [0.31, 0.165], tilt: 40, z: 0.07 }, right: (f) => ({ wrist: at(f.cup!, [CUP.height + 0.072, 0.014, 0.055]), u: unit([0.12, -0.78, -0.55]), v: unit([0.5, 0.45, -0.75]) }) },
  "leaning-forward": { hipShift: LEAN_SHIFT, hipLift: 0, torso: -38, neck: -6, head: -4, pad: { c: [0.33, 0.16], tilt: 15, z: 0 } },
  rising: { hipShift: r4(LEAN_SHIFT + 0.576 * (STAND.shift - LEAN_SHIFT)), hipLift: r4(0.429 * STAND.lift), torso: -35, neck: -6, head: -4, pad: { c: [0.29, 0.14], tilt: 25, z: 0 } },
  standing: { hipShift: STAND.shift, hipLift: STAND.lift, torso: -4, neck: -8, head: -10, pad: { c: [0.25, 0.17], tilt: 35, z: 0 } },
};
const DESCRIPTIONS: Record<string, Json> = {
  "seated-upright": T("Upright on the bench a hand's breadth off the backrest, head bowed to the notebook that lies in both open palms in front of the chest, thumbs on its edges, feet flat.", "Aufrecht auf der Bank, eine Handbreit vor der Lehne, der Kopf zum Notizbuch geneigt, das vor der Brust in beiden offenen Handflächen liegt, die Daumen an den Kanten, die Füße flach."),
  "reclined-reading": T("Slid back and leaning against the back slats with the notebook raised in both hands to eye level.", "Zurückgerutscht und an die Rückenlatten gelehnt, das Notizbuch in beiden Händen auf Augenhöhe gehoben."),
  "reaching-for-cup": T("Notebook lowered onto the left thigh in the left hand, the right hand reaching down beside her for the paper cup on the seat.", "Notizbuch in der linken Hand auf den linken Oberschenkel gesenkt, die rechte Hand greift neben ihr nach dem Pappbecher auf der Sitzfläche."),
  "leaning-forward": T("Slid to the front edge with the weight shifted forward over the feet, ready to stand up, the notebook in both hands above the thighs.", "An die Vorderkante gerutscht, das Gewicht nach vorn über die Füße verlagert, bereit zum Aufstehen, das Notizbuch in beiden Händen über den Oberschenkeln."),
  rising: T("Pelvis lifted off the seat, trunk still inclined, the legs extending over the planted feet.", "Becken von der Sitzfläche abgehoben, Rumpf noch geneigt, die Beine strecken sich über den stehenden Füßen."),
  standing: T("Upright on both feet in front of the bench, the notebook held in both hands at waist height.", "Aufrecht auf beiden Füßen vor der Bank, das Notizbuch in beiden Händen auf Hüfthöhe."),
};

/** 🤲️ The wrist and the axes of a hand that carries the notebook from below (`side` +1 left, -1 right). */
function holding(hip: V3, pad: PadSpec, side: number): HandGoal {
  const u: V3 = [Math.cos(rad(pad.tilt)), Math.sin(rad(pad.tilt)), 0];
  const v: V3 = [Math.sin(rad(pad.tilt)), -Math.cos(rad(pad.tilt)), 0];
  const origin = add([hip[0] + pad.c[0], hip[1] + pad.c[1], pad.z], u, -PAD.length / 2);
  const palm = add(add(add(origin, u, PAD.up), [0, 0, 1], side * PAD.off), v, PAD.below);
  const hand = add([u[0] * Math.cos(rad(PAD.turnDeg)), u[1] * Math.cos(rad(PAD.turnDeg)), 0], [0, 0, 1], -side * Math.sin(rad(PAD.turnDeg)));
  return { wrist: add(palm, hand, -PAD.palm), u: hand, v };
}

function arm(rig: Json, offsets: Offsets, s: "l" | "r", goal: HandGoal, label: string, report: string[], seed?: Offsets): void {
  const side = s === "l" ? 1 : -1;
  const params: Param[] = [[`arm-${s}`, "angleDeg", -60, 170], [`arm-${s}`, "abductionDeg", -150, 150], [`arm-${s}`, "twistDeg", -90, 90], [`forearm-${s}`, "angleDeg", 0, 150], [`forearm-${s}`, "abductionDeg", -8, 8], [`forearm-${s}`, "twistDeg", -180, 180], [`hand-${s}`, "angleDeg", -60, 60], [`hand-${s}`, "abductionDeg", -30, 30]];
  const residual = (f: Record<string, Frame>): number[] => {
    const [hand, fore, upper] = [f[`hand-${s}`]!, f[`forearm-${s}`]!, f[`arm-${s}`]!];
    const k = goal.k ?? 0.06;
    const reach = unit(add(goal.wrist, upper.o, -1));
    const hint = add(upper.o, unit([reach[0] * 0.5 - 0.1, reach[1] * 0.5 - 1, reach[2] * 0.5 + side * 0.45]), WOMAN.arm);
    const out = [hand.o[0] - goal.wrist[0], hand.o[1] - goal.wrist[1], hand.o[2] - goal.wrist[2], (fore.o[0] - hint[0]) * 0.08, (fore.o[1] - hint[1]) * 0.08, (fore.o[2] - hint[2]) * 0.08, (hand.u[0] - goal.u[0]) * k, (hand.u[1] - goal.u[1]) * k, (hand.u[2] - goal.u[2]) * k];
    if (goal.v !== undefined) out.push((hand.v[0] - goal.v[0]) * 0.06, (hand.v[1] - goal.v[1]) * 0.06, (hand.v[2] - goal.v[2]) * 0.06);
    return out;
  };
  const starts: number[][] = seed === undefined || goal.k !== undefined ? [[30, side * 15, 0, 80, side * 165], [30, side * 15, 0, 80, -side * 165], [40, side * 30, -side * 40, 70, side * 90], [20, side * 10, 0, 70, 0], [10, side * 20, side * 30, 90, -side * 120]] : [];
  if (goal.k !== undefined) starts.push([60, -side * 20, -side * 60, 120, side * 120], [80, side * 10, 0, 130, side * 90], [50, -side * 30, -side * 30, 110, -side * 150]);
  let best: { cost: number; set: Offsets; line: string } | null = null;
  const run = (): void => {
    const lines: string[] = [];
    solve(rig, offsets, params, residual, label, lines);
    const cost = residual(framesOf(rig, offsets)).reduce((q, e) => q + e * e, 0);
    if (best === null || cost < best.cost) best = { cost, set: { arm: { ...offsets[`arm-${s}`] }, fore: { ...offsets[`forearm-${s}`] }, hand: { ...offsets[`hand-${s}`] } }, line: lines[0]! };
  };
  if (seed !== undefined) {
    for (const b of ["arm", "forearm", "hand"]) offsets[`${b}-${s}`] = { angleDeg: 0, abductionDeg: 0, twistDeg: 0, ...seed[`${b}-${s}`] };
    delete offsets[`hand-${s}`]!.twistDeg;
    run();
  }
  for (const [a, b, t, c, p] of starts) {
    offsets[`arm-${s}`] = { angleDeg: a!, abductionDeg: b!, twistDeg: t! };
    offsets[`forearm-${s}`] = { angleDeg: c!, abductionDeg: 0, twistDeg: p! };
    offsets[`hand-${s}`] = { angleDeg: 0, abductionDeg: 0 };
    run();
  }
  offsets[`arm-${s}`] = best!.set.arm!;
  offsets[`forearm-${s}`] = best!.set.fore!;
  offsets[`hand-${s}`] = best!.set.hand!;
  report.push(best!.line);
}

function legs(rig: Json, offsets: Offsets, label: string, report: string[]): void {
  for (const s of ["l", "r"]) {
    let best: { cost: number; set: Json[] } | null = null;
    for (const [a, b] of [[0, 0], [-40, 30], [-80, 70]]) {
      offsets[`thigh-${s}`] = { angleDeg: a! };
      offsets[`shank-${s}`] = { angleDeg: b! };
      offsets[`foot-${s}`] = { angleDeg: 0 };
      offsets[`toe-${s}`] = { angleDeg: 0 };
      const lines: string[] = [];
      const residual = (f: Record<string, Frame>): number[] => {
        const g = f[`foot-${s}`]!;
        return [g.o[0] - ANKLE_X, g.o[1] - WOMAN.ankleY, (g.u[1] + Math.sin(rad(FOOT_DROP))) * 0.2];
      };
      solve(rig, offsets, [[`thigh-${s}`, "angleDeg", -125, 25], [`shank-${s}`, "angleDeg", -65, 90], [`foot-${s}`, "angleDeg", -60, 30]], residual, `${label} leg ${s}`, lines);
      const cost = residual(framesOf(rig, offsets)).reduce((q, e) => q + e * e, 0);
      if (best === null || cost < best.cost - 1e-12) best = { cost, set: [{ ...offsets[`thigh-${s}`] }, { ...offsets[`shank-${s}`] }, { ...offsets[`foot-${s}`] }] };
    }
    [offsets[`thigh-${s}`], offsets[`shank-${s}`], offsets[`foot-${s}`]] = best!.set as [Json, Json, Json];
    if (best!.cost > 1e-7) report.push(`${label} leg ${s}: residual ${(Math.sqrt(best!.cost) * 1000).toFixed(1)} mm`);
  }
}

function tidy(offsets: Offsets): Offsets {
  for (const b of Object.values(offsets)) for (const k of Object.keys(b)) if ((k === "abductionDeg" || k === "twistDeg") && b[k] === 0) delete b[k];
  return offsets;
}

function posed(rig: Json, id: string, spec: Spec, report: string[]): Offsets {
  const offsets: Offsets = { "hip-lift": { dy: spec.hipLift }, "hip-shift": { dx: spec.hipShift }, pelvis: { angleDeg: 0 }, torso: { angleDeg: spec.torso }, neck: { angleDeg: spec.neck }, head: { angleDeg: spec.head }, pad: { angleDeg: 0, abductionDeg: -PAD.turnDeg }, page: { angleDeg: spec.page ?? 0 } };
  legs(rig, offsets, id, report);
  const hip = framesOf(rig, offsets).pelvis!.o;
  arm(rig, offsets, "l", holding(hip, spec.pad, 1), `${id} left arm`, report);
  arm(rig, offsets, "r", spec.right === undefined ? holding(hip, spec.pad, -1) : spec.right(framesOf(rig, offsets)), `${id} right arm`, report);
  return tidy(offsets);
}

/* ---------------------------------------------------------------- clips */

/** 🪜️ The baked keys of the sit-to-stand between the key poses: time, the base pose, the shares of the pelvis lift and of its travel from the front edge to the standing place, and the trunk angle (the measured course of the old clip, carried to the new leg lengths). */
const RISE: readonly (readonly [t: number, pose: string, lift: number, shift: number, torso: number])[] = [
  [1.1, "leaning-forward", 0.144, 0.193, -39.7], [1.2, "leaning-forward", 0.204, 0.267, -39.2], [1.3, "leaning-forward", 0.265, 0.345, -38.2], [1.4, "leaning-forward", 0.322, 0.423, -37.2],
  [1.7, "rising", 0.503, 0.631, -32.7], [1.8, "rising", 0.591, 0.675, -29.4], [1.9, "rising", 0.673, 0.719, -26], [2, "rising", 0.748, 0.763, -22.6], [2.1, "rising", 0.815, 0.807, -19.1], [2.2, "rising", 0.872, 0.85, -15.7], [2.3, "rising", 0.921, 0.894, -12.3], [2.4, "rising", 0.959, 0.937, -9], [2.5, "rising", 0.987, 0.976, -5.9],
];
const mix = (a: number, b: number, k: number): number => a + (b - a) * k;
function riseSpec(t: number, lift: number, shift: number, torso: number): Spec {
  const [from, to, k] = t < 1.6 ? [POSES["leaning-forward"]!, POSES.rising!, (t - 0.8) / 0.8] : [POSES.rising!, POSES.standing!, (t - 1.6) / 1];
  return { hipShift: r4(LEAN_SHIFT + shift * (STAND.shift - LEAN_SHIFT)), hipLift: r4(lift * STAND.lift), torso, neck: mix(from.neck, to.neck, k), head: mix(from.head, to.head, k), pad: { c: [mix(from.pad.c[0], to.pad.c[0], k), mix(from.pad.c[1], to.pad.c[1], k)], tilt: mix(from.pad.tilt, to.pad.tilt, k), z: 0 } };
}

/** 📄️ The right hand of the page turn: the finger tips at the free lower edge of the sheet while it swings by `turn` degrees over the binding; the wrist stays below and to the right of them. */
const PAGE_TURN: readonly (readonly [t: number, page: number, hand: number | null])[] = [[0.7, 0, 0], [1.2, 50, 50], [1.7, 110, 110], [2.3, 215, null], [3, 352, null], [3.6, 352, null]];
function turning(f: Record<string, Frame>, turn: number): HandGoal {
  const pad = f.pad!;
  const hinge = at(pad, [PAD.length, 0, 0]);
  const edge = add(add(add(hinge, pad.u, -Math.cos(rad(turn)) * PAD.page), pad.v, -Math.sin(rad(turn)) * PAD.page), pad.w, 0.045);
  const back = unit([-0.4, -0.5, -0.76]);
  return { wrist: add(edge, back, 0.15), u: [-back[0], -back[1], -back[2]], k: 0.012 };
}

/* ---------------------------------------------------------------- document */

function setPath(target: Json, path: string, value: unknown): void {
  const keys = path.split(".");
  let o = target;
  for (const k of keys.slice(0, -1)) o = o[k];
  o[keys[keys.length - 1]!] = value;
}

const SUMMARY = T("A woman reading a top-bound notebook on a timber park bench: pelvis and thighs carried by the seat slats, feet flat, with a page-turn, a sit-to-stand transition and a reach for a paper cup that stands on the seat beside her.", "Eine Frau liest in einem oben gebundenen Notizbuch auf einer hölzernen Parkbank: Becken und Oberschenkel liegen auf den Sitzlatten, die Füße stehen flach; mit Umblättern, Aufstehen und Griff zum Pappbecher, der neben ihr auf der Sitzfläche steht.");
const ASSUMPTIONS: Json[] = [
  T("Seat, thigh support and backrest are unilateral contacts; they release when they would have to pull. The bench (two end frames, seat and back slats) is rigid and fixed to the ground and carries no mass of its own in this model.", "Sitz, Oberschenkelauflage und Lehne sind einseitige Kontakte; sie lösen sich, wenn sie ziehen müssten. Die Bank (zwei Wangen, Sitz- und Rückenlatten) ist starr, fest mit dem Boden verbunden und trägt in diesem Modell keine eigene Masse."),
  T("Segments are point masses at the de Leva centres; the notebook is a 0.25 kg bone mass in the left hand, its turning sheet 2.5 g, the cup with its drink 0.27 kg on the seat.", "Segmente sind Punktmassen in den Schwerpunkten nach de Leva; das Notizbuch ist eine Knochenmasse von 0,25 kg in der linken Hand, das umblätternde Blatt 2,5 g, der Becher mit Getränk 0,27 kg auf der Sitzfläche."),
];
const CLIP_TEXT: Record<string, Json> = {
  "page-turn": T("The right hand leaves the notebook, takes the free lower edge of the top-bound sheet, lifts it over the wire binding, lets it fall round behind the board and returns under the notebook while the left hand keeps carrying it.", "Die rechte Hand löst sich vom Notizbuch, fasst die freie untere Kante des oben gebundenen Blatts, hebt es über die Drahtbindung, lässt es hinter die Pappe fallen und kehrt unter das Notizbuch zurück, während die linke Hand es weiter trägt."),
  "stand-up": T(`Sit-to-stand: she slides to the front edge, the trunk leans forward over the feet, the pelvis leaves the seat and travels ${STAND.shift.toFixed(2)} m forward and ${STAND.lift.toFixed(2)} m up while both feet stay planted; the activity rises from 1 to about 2.2 met and falls again.`, `Aufstehen aus dem Sitz: Sie rutscht an die Vorderkante, der Rumpf neigt sich über die Füße, das Becken verlässt die Sitzfläche und bewegt sich ${STAND.shift.toFixed(2).replace(".", ",")} m nach vorn und ${STAND.lift.toFixed(2).replace(".", ",")} m nach oben, während beide Füße stehen bleiben; die Aktivität steigt von 1 auf etwa 2,2 met und fällt wieder.`),
  "lean-back": T("From upright reading she slides back against the back slats, reads with the notebook raised, then sits up, lowers the notebook onto her thigh and reaches with the right hand for the paper cup beside her.", "Aus aufrechtem Lesen rutscht sie zurück an die Rückenlatten, liest mit angehobenem Notizbuch, richtet sich dann auf, senkt das Notizbuch auf den Oberschenkel und greift mit der rechten Hand nach dem Pappbecher neben ihr."),
};

/** 📄️ The catalog document with the root fixes applied (everything except `volumes` and the 2D geometry, which the build adds). */
export function rooted(report: string[] = []): Json {
  const document = JSON.parse(readFileSync(ASSET, "utf8")) as Json;
  const asset = document.asset as Json;
  const have = new Map((asset.rig.bones as Json[]).map((b) => [b.id, b]));
  const bench = BENCH_BONES.map((b) => ({ ...b }));
  asset.rig.bones = [...bench, ...(asset.rig.bones as Json[]).filter((b) => !bench.some((q) => q.id === b.id))];
  asset.rig.root = "ground";
  for (const bone of asset.rig.bones as Json[]) for (const [path, value] of Object.entries(BONES[bone.id] ?? {})) setPath(bone, path, value);
  if (!have.has("ground")) for (const symbol of asset.symbols as Json[]) for (const shapes of Object.values(symbol.geometry as Json)) for (const shape of shapes as Json[]) if (shape.bone === "seat") shape.bone = "ground";
  for (const anchor of asset.anchors as Json[]) Object.assign(anchor, ANCHORS[anchor.id] ?? {});
  asset.summary = SUMMARY;
  asset.structure.assumptions = ASSUMPTIONS;
  const rig = asset.rig as Json;
  const key = JSON.stringify([rig.bones, POSES, RISE, PAGE_TURN, PAD, STAND, CUP]);
  const cached = existsSync(CACHE) ? (JSON.parse(readFileSync(CACHE, "utf8")) as Json) : null;
  if (cached !== null && cached.key === key) {
    for (const pose of asset.poses as Json[]) [pose.bones, pose.description] = [cached.poses[pose.id], DESCRIPTIONS[pose.id]];
    for (const clip of asset.clips as Json[]) {
      clip.description = CLIP_TEXT[clip.id];
      if (cached.clips[clip.id] !== undefined) clip.keys = cached.clips[clip.id];
    }
    report.push(...cached.report);
    asset.dimensions.massKg = r4((rig.bones as Json[]).reduce((s, b) => s + (b.mass?.kg ?? 0), 0));
    return document;
  }
  for (const pose of asset.poses as Json[]) {
    pose.bones = posed(rig, pose.id, POSES[pose.id]!, report);
    pose.description = DESCRIPTIONS[pose.id];
  }
  const now = Object.fromEntries((asset.poses as Json[]).map((p) => [p.id, p.bones as Offsets]));
  for (const clip of asset.clips as Json[]) {
    clip.description = CLIP_TEXT[clip.id];
    if (clip.id === "page-turn") {
      let seed: Offsets = now["seated-upright"]!;
      clip.keys = [{ t: 0, pose: "seated-upright", ease: "smooth" }, ...PAGE_TURN.map(([t, page, hand]) => {
        const offsets: Offsets = JSON.parse(JSON.stringify(now["seated-upright"]));
        offsets.page = { angleDeg: page };
        if (hand !== null) {
          arm(rig, offsets, "r", turning(framesOf(rig, offsets), hand), `page-turn ${t}`, report, seed);
          seed = offsets;
        }
        tidy(offsets);
        return { t, pose: "seated-upright", ease: "smooth", bones: { "arm-r": offsets["arm-r"], "forearm-r": offsets["forearm-r"], "hand-r": offsets["hand-r"], page: offsets.page } };
      })];
    }
    if (clip.id === "stand-up") {
      const baked = new Map(RISE.map(([t, pose, lift, shift, torso]) => [t, { t, pose, ease: "smooth", bones: posed(rig, `stand-up ${t}`, riseSpec(t, lift, shift, torso), report) }]));
      clip.keys = [{ t: 0, pose: "seated-upright", ease: "smooth" }, { t: 0.8, pose: "leaning-forward", ease: "smooth" }, ...RISE.filter(([t]) => t < 1.6).map(([t]) => baked.get(t)), { t: 1.6, pose: "rising", ease: "smooth" }, ...RISE.filter(([t]) => t > 1.6).map(([t]) => baked.get(t)), { t: 2.6, pose: "standing", ease: "smooth" }, { t: 4, pose: "standing", ease: "smooth" }];
    }
  }
  asset.dimensions.massKg = r4((rig.bones as Json[]).reduce((s, b) => s + (b.mass?.kg ?? 0), 0));
  mkdirSync(dirname(CACHE), { recursive: true });
  writeFileSync(CACHE, JSON.stringify({ key, report, poses: Object.fromEntries((asset.poses as Json[]).map((p) => [p.id, p.bones])), clips: Object.fromEntries((asset.clips as Json[]).filter((c) => c.id !== "lean-back").map((c) => [c.id, c.keys])) }));
  return document;
}

if (import.meta.main) {
  const report: string[] = [];
  const document = rooted(report);
  console.log(report.join("\n"));
  const rig = document.asset.rig as Json;
  for (const pose of document.asset.poses as Json[]) {
    const f = framesOf(rig, pose.bones);
    console.log(pose.id, Object.entries(f).filter(([id]) => /^(pelvis|torso|neck|head|arm|forearm|hand|thigh|shank|foot|toe|pad)/.test(id)).map(([id, g]) => `${id} ${g.o.map((x) => x.toFixed(3)).join(",")}`).join(" | "));
  }
  if (process.argv.includes("--print")) console.log(JSON.stringify(document.asset.rig.bones, null, 1));
}
