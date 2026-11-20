/** 🦴️ Root fixes of `humans/seated-office-worker` (round 3), stated as final values and solved again on every run (idempotent): the rig (hip and shoulder joint spacing, lumbosacral joint behind the hip joints, C7 and atlanto-occipital joint at their heights, shank length, ankle in a shoe with a sole, a five-star base whose arms slope to castors that stand beside the symmetry plane, a seat on a gas lift, a compact desk of 800 x 600 mm and a notebook of real depth), the anchors, the masses of desk and frame, every pose solved in three dimensions with the product's own chain (`solveChain`), the clips, the dimensions and the texts that named the old construction.
 * Usage: `bun r3-seated-office-worker-roots.ts [--print]` (as module: `rooted()` gives the catalog document with these fixes).
 * @see ./r3-seated-office-worker-ref.ts */
import { readFileSync } from "node:fs";

import { solveChain } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts";
import { CHAIR, DESK, LAPTOP, MAN, STAR, type V3 } from "./r3-seated-office-worker-ref.ts";

export type Json = Record<string, any>;
export const ASSET = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/seated-office-worker/🔣️.json";
export type Frame = { o: V3; u: V3; v: V3; w: V3 };
type Offsets = Record<string, Record<string, number>>;

const rad = (d: number): number => (d * Math.PI) / 180;
const deg = (r: number): number => (r * 180) / Math.PI;
const add = (a: V3, b: V3, s = 1): V3 => [a[0] + s * b[0], a[1] + s * b[1], a[2] + s * b[2]];
const r3 = (x: number): number => Math.round(x * 1e3) / 1e3;
const r4 = (x: number): number => Math.round(x * 1e4) / 1e4;
export const at = (f: Frame, p: V3): V3 => add(add(add(f.o, f.u, p[0]), f.v, p[1]), f.w, p[2]);

export function framesOf(rig: Json, offsets: Offsets): Record<string, Frame> {
  return Object.fromEntries(Object.entries(solveChain(rig as never, offsets as never, { x: 0, y: 0, angleDeg: 0 })).map(([id, b]) => [id, (b as Json).frame as Frame]));
}

/* ---------------------------------------------------------------- rig */

const HIP_Y = CHAIR.seatY + MAN.hipAboveSeatY;
const THIGH_DROP = deg(Math.asin((HIP_Y - MAN.ankleY - MAN.shank) / MAN.thigh));
const FOOT_DROP = deg(Math.atan2(MAN.ankleY, MAN.ballAhead));
const FOOT_LENGTH = r4(Math.hypot(MAN.ankleY, MAN.ballAhead));
const SHOE_SCALE = FOOT_LENGTH / 0.1858;

function starBones(): Record<string, Json> {
  const out: Record<string, Json> = {};
  for (const [id, yaw] of STAR) {
    const [c, s] = [Math.cos(rad(yaw)), Math.sin(rad(yaw))];
    const [dx, dy] = [CHAIR.armReach * c, CHAIR.armTipY - CHAIR.hubY];
    const angle = deg(Math.atan2(dy, dx));
    const [uc, us] = [Math.cos(rad(angle)), Math.sin(rad(angle))];
    const [px, py] = [(CHAIR.armReach - CHAIR.castorTrail) * c, CHAIR.wheelR - CHAIR.hubY];
    out[`chair-arm-${id}`] = { angleDeg: r4(angle), lengthM: r4(Math.hypot(dx, dy)) };
    out[`chair-castor-${id}`] = { x: r4(px * uc + py * us), y: r4(-px * us + py * uc), angleDeg: r4(-angle), lengthM: CHAIR.wheelR, ...(yaw === 0 ? {} : { z: r4((CHAIR.armReach - CHAIR.castorTrail) * s) }) };
  }
  return out;
}

const BONES: Record<string, Json> = {
  "chair-base": { x: CHAIR.columnX, y: CHAIR.hubY, lengthM: 0.05 },
  ...starBones(),
  "chair-column-sleeve": { lengthM: 0.2 },
  "chair-column": { lengthM: r4(CHAIR.seatY - CHAIR.hubY), "mass.comX": 0.16 },
  "chair-seat": { x: r4(CHAIR.seatY - CHAIR.hubY), lengthM: CHAIR.seatHalfDepth, "mass.comX": 0, "mass.comY": 0.035 },
  "chair-backrest": { x: CHAIR.backPivot[0], y: CHAIR.backPivot[1], lengthM: CHAIR.backLength, "mass.comX": 0.36, "mass.comY": 0 },
  "hip-lift": { y: MAN.hipAboveSeatY },
  pelvis: { lengthM: 0.1 },
  torso: { x: 0.1, y: 0.035, angleDeg: -2, lengthM: MAN.torso, "mass.comX": r4(MAN.torso * 0.4557), "mass.comY": 0 },
  neck: { x: MAN.torso, angleDeg: -16, lengthM: MAN.neck, "mass.comX": r4(MAN.neck / 2) },
  head: { x: MAN.neck, angleDeg: 18, lengthM: MAN.head, "mass.comX": 0.026, "mass.comY": -0.016 },
  "arm-l": { x: MAN.shoulderOnTorso, y: -MAN.shoulderAhead, angleDeg: 182, z: MAN.shoulderZ },
  "arm-r": { x: MAN.shoulderOnTorso, y: -MAN.shoulderAhead, angleDeg: 182, z: -MAN.shoulderZ },
  "thigh-l": { angleDeg: r4(-90 - THIGH_DROP), z: MAN.hipZ },
  "thigh-r": { angleDeg: r4(-90 - THIGH_DROP), z: -MAN.hipZ },
  laptop: { x: LAPTOP.hingeX, y: LAPTOP.hingeY, lengthM: LAPTOP.depth, "mass.kg": 1.3, "mass.comX": 0.11, "mass.comY": 0.004 },
  screen: { angleDeg: -LAPTOP.openDeg, lengthM: LAPTOP.lidLength, "mass.kg": 0.5, "mass.comX": 0.11 },
  "desk-top": { x: DESK.x0, y: DESK.top - DESK.board, lengthM: DESK.depth, "mass.kg": 7.6, "mass.comX": DESK.depth / 2, "mass.comY": DESK.board / 2 },
  "desk-leg": { x: DESK.x0 + DESK.depth / 2, lengthM: DESK.top - DESK.board, "mass.kg": 6.2, "mass.comX": 0.44, "mass.distribution": "uniform" },
};
for (const s of ["l", "r"]) {
  BONES[`shank-${s}`] = { angleDeg: r4(-90 + THIGH_DROP), lengthM: MAN.shank, "mass.comX": 0.1953 };
  BONES[`foot-${s}`] = { x: MAN.shank, angleDeg: r4(90 - FOOT_DROP), lengthM: FOOT_LENGTH, "mass.comX": r4(0.0791 * SHOE_SCALE), "mass.comY": -0.003 };
  BONES[`toe-${s}`] = { x: FOOT_LENGTH, angleDeg: r4(FOOT_DROP) };
}
const TITLES: Record<string, Json> = {
  "chair-arm-side-a": { en: "Left side arm (drawn in the side plane, the castor stands beside it)", de: "Linker Seitenarm (in der Seitenebene gezeichnet, die Rolle steht daneben)" },
  "chair-arm-side-b": { en: "Right side arm (drawn in the side plane, the castor stands beside it)", de: "Rechter Seitenarm (in der Seitenebene gezeichnet, die Rolle steht daneben)" },
  "chair-arm-back-a": { en: "Left back arm (drawn in the side plane, the castor stands beside it)", de: "Linker hinterer Arm (in der Seitenebene gezeichnet, die Rolle steht daneben)" },
  "chair-arm-back-b": { en: "Right back arm (drawn in the side plane, the castor stands beside it)", de: "Rechter hinterer Arm (in der Seitenebene gezeichnet, die Rolle steht daneben)" },
  "chair-castor-side-a": { en: "Left side castor", de: "Linke Seitenrolle" },
  "chair-castor-side-b": { en: "Right side castor", de: "Rechte Seitenrolle" },
  "chair-castor-back-a": { en: "Left back castor", de: "Linke hintere Rolle" },
  "chair-castor-back-b": { en: "Right back castor", de: "Rechte hintere Rolle" },
  "chair-column-sleeve": { en: "Gas-lift outer tube", de: "Standrohr der Gasfeder" },
  "chair-seat": { en: "Seat on its mechanism", de: "Sitz auf seiner Mechanik" },
  "desk-top": { en: "Desk top, 780 x 600 mm", de: "Schreibtischplatte, 780 x 600 mm" },
  laptop: { en: "Notebook base with keyboard (15.6 inch)", de: "Notebook-Unterteil mit Tastatur (15,6 Zoll)" },
  screen: { en: "Notebook lid with display", de: "Notebook-Deckel mit Display" },
  "desk-leg": { en: "Desk frame of four steel legs and rails", de: "Tischgestell aus vier Stahlbeinen und Zargen" },
};

/* ---------------------------------------------------------------- anchors */

const HEEL_LOCAL = ((): [number, number] => {
  const [c, s] = [Math.cos(rad(-FOOT_DROP)), Math.sin(rad(-FOOT_DROP))];
  const d: [number, number] = [-0.048, -MAN.ankleY];
  return [r4(d[0] * c + d[1] * s), r4(-d[0] * s + d[1] * c)];
})();
const ANCHORS: Record<string, Json> = {
  seat: { x: -0.095, y: 0.025, footprintLengthM: 0.17, footprintWidthM: 0.32 },
  "thigh-support": { x: 0.2, y: -0.08, z: MAN.hipZ, footprintLengthM: 0.21, footprintWidthM: 0.34 },
  backrest: { x: 0.2, y: 0.092, footprintLengthM: 0.3, footprintWidthM: 0.36 },
  "heel-l": { x: HEEL_LOCAL[0], y: HEEL_LOCAL[1], footprintLengthM: 0.075, footprintWidthM: 0.07 },
  "heel-r": { x: HEEL_LOCAL[0], y: HEEL_LOCAL[1], footprintLengthM: 0.075, footprintWidthM: 0.07 },
  "ball-l": { x: FOOT_LENGTH, y: 0, footprintLengthM: 0.1, footprintWidthM: 0.1 },
  "ball-r": { x: FOOT_LENGTH, y: 0, footprintLengthM: 0.1, footprintWidthM: 0.1 },
  mouth: { x: -0.06, y: -0.104 },
  nostrils: { x: -0.036, y: -0.1 },
  crown: { x: 0.142, y: 0.008 },
  "laptop-vent": { x: -0.008, y: 0.004 },
};

/* ---------------------------------------------------------------- poses */

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
  for (let step = 0; step < 300 && cost(r) > 1e-14; step++) {
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

/** ✋️ Where an arm goes: the wrist, a point the elbow is drawn towards (weakly) and the direction of the hand; all in the world, or in the frame of `bone` when one is named. */
type ArmGoal = { wrist: V3; elbow: V3; hand: V3; bone?: string; start: [arm: number, abduction: number, forearm: number]; search?: boolean };
type PoseSpec = { hipShift: number; torso: number; neck: number; head: number; backrest: number; l: ArmGoal; r: ArmGoal };

export const ANKLE_X = r4(CHAIR.columnX - 0.04 + MAN.thigh * Math.cos(rad(THIGH_DROP)));
const KEYS_Y = LAPTOP.hingeY + 0.006;
const POSES: Record<string, PoseSpec> = {
  typing: {
    hipShift: -0.04, torso: -6, neck: -8, head: -14, backrest: 0,
    l: { wrist: [0.438, KEYS_Y + 0.03, 0.098], elbow: [0.17, 0.8, 0.215], hand: [0.95, 0.25, -0.18], start: [35, 8, 70] },
    r: { wrist: [0.432, KEYS_Y + 0.031, -0.098], elbow: [0.165, 0.8, -0.215], hand: [0.95, 0.25, 0.18], start: [35, -8, 70] },
  },
  "reading-screen": {
    hipShift: -0.085, torso: 3, neck: -11, head: -9, backrest: 4,
    l: { wrist: [0.3, 0.672, 0.112], elbow: [0.02, 0.76, 0.21], hand: [0.96, -0.22, -0.16], start: [20, 6, 60] },
    r: { wrist: [0.4, KEYS_Y + 0.028, -0.05], elbow: [0.12, 0.81, -0.2], hand: [0.95, 0.2, 0.2], start: [40, -8, 70] },
  },
  "leaning-back": {
    hipShift: -0.115, torso: 12, neck: -4, head: -4, backrest: 12,
    l: { wrist: [-0.265, 1.31, 0.135], elbow: [-0.21, 1.36, 0.4], hand: [-0.3, 0.05, -0.95], start: [100, 75, 130], search: true },
    r: { wrist: [0.215, 0.678, -0.09], elbow: [-0.05, 0.76, -0.21], hand: [0.96, -0.22, 0.16], start: [10, -8, 70] },
  },
};
const DESCRIPTIONS: Record<string, Json> = {
  typing: { en: "Upright and slightly forward at the notebook, the heels of both hands over the palm rest and the curled fingers on the home row, the elbows a hand's breadth off the trunk, head bowed to the screen, feet flat under the desk.", de: "Aufrecht und leicht vorgebeugt am Notebook, beide Handballen über der Handauflage und die gekrümmten Finger auf der Grundreihe, die Ellbogen eine Handbreit vom Rumpf, der Kopf zum Bildschirm geneigt, die Füße flach unter dem Tisch." },
  "reading-screen": { en: "Sitting further back and upright, head pushed forward, the right hand on the trackpad and the left forearm lying on the left thigh.", de: "Weiter hinten und aufrecht sitzend, der Kopf nach vorn geschoben, die rechte Hand am Trackpad und der linke Unterarm auf dem linken Oberschenkel." },
  "leaning-back": { en: "The backrest tilts with the trunk, the left hand rests behind the head with the elbow out to the side and the right forearm lies in the lap.", de: "Die Lehne neigt sich mit dem Rumpf, die linke Hand ruht hinter dem Kopf mit dem Ellbogen zur Seite und der rechte Unterarm liegt im Schoß." },
};

function arm(rig: Json, offsets: Offsets, s: "l" | "r", goal: ArmGoal, label: string, report: string[]): void {
  const n = Math.hypot(...goal.hand);
  const params: Param[] = [[`arm-${s}`, "angleDeg", -60, 170], [`arm-${s}`, "abductionDeg", -150, 150], [`arm-${s}`, "twistDeg", -90, 90], [`forearm-${s}`, "angleDeg", 0, 150], [`forearm-${s}`, "abductionDeg", -8, 8], [`hand-${s}`, "angleDeg", -60, 60], [`hand-${s}`, "abductionDeg", -30, 30]];
  const residual = (f: Record<string, Frame>): number[] => {
    const to = (p: V3): V3 => (goal.bone === undefined ? p : at(f[goal.bone]!, p));
    const dir = (p: V3): V3 => (goal.bone === undefined ? p : add(at(f[goal.bone]!, p), f[goal.bone]!.o, -1));
    const [w, e, h] = [to(goal.wrist), to(goal.elbow), dir(goal.hand)];
    const [hand, fore] = [f[`hand-${s}`]!, f[`forearm-${s}`]!];
    return [hand.o[0] - w[0], hand.o[1] - w[1], hand.o[2] - w[2], (fore.o[0] - e[0]) * 0.12, (fore.o[1] - e[1]) * 0.12, (fore.o[2] - e[2]) * 0.12, (hand.u[0] - h[0] / n) * 0.03, (hand.u[1] - h[1] / n) * 0.03, (hand.u[2] - h[2] / n) * 0.03];
  };
  const starts = goal.search === true ? [0, 50, 100, 150].flatMap((a) => [20, 70, 120].flatMap((b) => [-60, 0, 60].flatMap((t) => [80, 125].map((c) => [a, b, t, c])))) : [[goal.start[0], goal.start[1], 0, goal.start[2]]];
  let best: { cost: number; arm: Json; fore: Json; hand: Json; line: string } | null = null;
  for (const [a, b, t, c] of starts) {
    offsets[`arm-${s}`] = { angleDeg: a!, abductionDeg: b!, twistDeg: t! };
    offsets[`forearm-${s}`] = { angleDeg: c!, abductionDeg: 0 };
    offsets[`hand-${s}`] = { angleDeg: 0, abductionDeg: 0 };
    const lines: string[] = [];
    solve(rig, offsets, params, residual, label, lines);
    const cost = residual(framesOf(rig, offsets)).reduce((q, e) => q + e * e, 0);
    if (best === null || cost < best.cost) best = { cost, arm: { ...offsets[`arm-${s}`] }, fore: { ...offsets[`forearm-${s}`] }, hand: { ...offsets[`hand-${s}`] }, line: lines[0]! };
  }
  offsets[`arm-${s}`] = best!.arm;
  offsets[`forearm-${s}`] = best!.fore;
  offsets[`hand-${s}`] = best!.hand;
  report.push(best!.line);
}

function legs(rig: Json, offsets: Offsets, label: string, report: string[]): void {
  for (const s of ["l", "r"]) {
    offsets[`thigh-${s}`] = { angleDeg: 0 };
    offsets[`shank-${s}`] = { angleDeg: 0 };
    offsets[`foot-${s}`] = { angleDeg: 0 };
    offsets[`toe-${s}`] = { angleDeg: 0 };
    solve(rig, offsets, [[`thigh-${s}`, "angleDeg", -30, 25], [`shank-${s}`, "angleDeg", -65, 90], [`foot-${s}`, "angleDeg", -60, 30]], (f) => {
      const g = f[`foot-${s}`]!;
      return [g.o[0] - ANKLE_X, g.o[1] - MAN.ankleY, (g.u[1] + Math.sin(rad(FOOT_DROP))) * 0.2];
    }, `${label} leg ${s}`, report);
  }
}

function posed(rig: Json, id: string, spec: PoseSpec, report: string[]): Offsets {
  const offsets: Offsets = { "hip-shift": { dx: spec.hipShift }, "hip-lift": { dy: 0 }, pelvis: { angleDeg: 0 }, torso: { angleDeg: spec.torso }, neck: { angleDeg: spec.neck }, head: { angleDeg: spec.head }, "chair-backrest": { angleDeg: spec.backrest } };
  legs(rig, offsets, id, report);
  arm(rig, offsets, "l", spec.l, `${id} left arm`, report);
  arm(rig, offsets, "r", spec.r, `${id} right arm`, report);
  for (const b of Object.values(offsets)) for (const k of Object.keys(b)) if ((k === "abductionDeg" || k === "twistDeg") && b[k] === 0) delete b[k];
  return offsets;
}

/* ---------------------------------------------------------------- document */

function setPath(target: Json, path: string, value: unknown): void {
  const keys = path.split(".");
  let o = target;
  for (const k of keys.slice(0, -1)) o = o[k];
  o[keys[keys.length - 1]!] = value;
}

const TEXTS: readonly (readonly [RegExp, string])[] = [
  [/The chair is rigid and fixed\./, "The chair is rigid and fixed."],
  [/Die Stuhl ist starr und fest\./, "Der Stuhl ist starr und fest."],
  [/Segments are point masses at the de Leva centres; the notebook is a 0\.6 kg bone mass on the far hand\./, "Segments are point masses at the de Leva centres; the notebook (1.7 kg) and the desk (14.6 kg) stand on their own and carry no load of the sitter except the weight of the hands."],
  [/Segmente sind Punktmassen in den Schwerpunkten nach de Leva; das Notizbuch ist eine Knochenmasse von 0,6 kg an der fernen Hand\./, "Segmente sind Punktmassen in den Schwerpunkten nach de Leva; das Notebook (1,7 kg) und der Tisch (14,6 kg) stehen für sich und tragen von der sitzenden Person nur das Gewicht der Hände."],
  [/From typing she reads/, "From typing he reads"],
];

/** 📄️ The catalog document with the root fixes applied (everything except `volumes` and the 2D geometry, which the build adds). */
export function rooted(report: string[] = []): Json {
  const document = JSON.parse(readFileSync(ASSET, "utf8")) as Json;
  const asset = document.asset as Json;
  const before = JSON.parse(JSON.stringify(asset)) as Json;
  for (const bone of asset.rig.bones as Json[]) {
    for (const [path, value] of Object.entries(BONES[bone.id] ?? {})) setPath(bone, path, value);
    if (TITLES[bone.id] !== undefined) bone.title = TITLES[bone.id];
  }
  for (const anchor of asset.anchors as Json[]) Object.assign(anchor, ANCHORS[anchor.id] ?? {});
  const text = (value: Json): void => {
    for (const lang of ["en", "de"]) for (const [from, to] of TEXTS) value[lang] = (value[lang] as string).replace(from, to);
  };
  for (const a of asset.structure.assumptions as Json[]) text(a);
  for (const c of asset.clips as Json[]) text(c.description);
  const rig = asset.rig as Json;
  const old = Object.fromEntries((before.poses as Json[]).map((p) => [p.id, p.bones as Offsets]));
  for (const pose of asset.poses as Json[]) {
    pose.bones = posed(rig, pose.id, POSES[pose.id]!, report);
    pose.description = DESCRIPTIONS[pose.id];
  }
  const now = Object.fromEntries((asset.poses as Json[]).map((p) => [p.id, p.bones as Offsets]));
  for (const clip of asset.clips as Json[])
    for (const key of clip.keys as Json[])
      for (const [bone, offset] of Object.entries((key.bones ?? {}) as Offsets))
        for (const k of Object.keys(offset)) offset[k] = r3((now[key.pose]![bone]?.[k] ?? 0) + (offset[k]! - (old[key.pose]![bone]?.[k] ?? 0)));
  asset.dimensions.massKg = r4((rig.bones as Json[]).reduce((s, b) => s + (b.mass?.kg ?? 0), 0));
  return document;
}

if (import.meta.main) {
  const report: string[] = [];
  const document = rooted(report);
  console.log(report.join("\n"));
  const rig = document.asset.rig as Json;
  for (const pose of document.asset.poses as Json[]) {
    const f = framesOf(rig, pose.bones);
    console.log(pose.id, Object.entries(f).filter(([id]) => /^(pelvis|torso|neck|head|arm|forearm|hand|thigh|shank|foot|toe|chair-backrest)/.test(id)).map(([id, g]) => `${id} ${g.o.map((x) => x.toFixed(3)).join(",")}`).join(" | "));
  }
  if (process.argv.includes("--print")) console.log(JSON.stringify(document.asset.rig.bones, null, 1));
}
