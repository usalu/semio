/** 🦴️ Root fixes of `humans/seated-guitarist` (round 3), stated as final values and solved again on every run (idempotent): the rig (hip and shoulder joint spacing, shank length, cervico-thoracic joint, a vertical 24 inch stool, the guitar as a real 1.03 m instrument carried by the pelvis and pivoting on the thigh it rests on), the anchors, and every pose solved in three dimensions with the product's own chain (`solveChain`): the guitar lies across the body with the neck to the left, the right foot stands on the front rung, the left foot on the floor, the right forearm lies over the lower bout and the left hand holds the neck.
 * Usage: `bun r3-seated-guitarist-roots.ts [--print]` (as module: `rooted()` gives the catalog document with these fixes).
 * @see ./r3-seated-guitarist-ref.ts */
import { readFileSync } from "node:fs";

import { solveChain } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts";
import { GUITAR, HIP_ABOVE_SEAT, STOOL, fretA, legAt, onGuitar, type V3 } from "./r3-seated-guitarist-ref.ts";

export type Json = Record<string, any>;
export const ASSET = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/seated-guitarist/🔣️.json";
type Frame = { o: V3; u: V3; v: V3; w: V3 };
type Offsets = Record<string, Record<string, number>>;

const rad = (d: number): number => (d * Math.PI) / 180;
const add = (a: V3, b: V3, s = 1): V3 => [a[0] + s * b[0], a[1] + s * b[1], a[2] + s * b[2]];
const dot = (a: V3, b: V3): number => a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
const cross = (a: V3, b: V3): V3 => [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
const unit = (a: V3): V3 => { const n = Math.hypot(...a); return [a[0] / n, a[1] / n, a[2] / n]; };
const r3 = (x: number): number => Math.round(x * 1e3) / 1e3;
const r4 = (x: number): number => Math.round(x * 1e4) / 1e4;
export const at = (f: Frame, p: V3): V3 => add(add(add(f.o, f.u, p[0]), f.v, p[1]), f.w, p[2]);

export function framesOf(rig: Json, offsets: Offsets): Record<string, Frame> {
  return Object.fromEntries(Object.entries(solveChain(rig as never, offsets as never, { x: 0, y: 0, angleDeg: 0 })).map(([id, b]) => [id, (b as Json).frame as Frame]));
}

/* ---------------------------------------------------------------- rig */

const BONES: Record<string, Json> = {
  stool: { angleDeg: 90, lengthM: STOOL.seatTop, "mass.comX": 0.45, "mass.comY": 0 },
  "hip-lift": { x: STOOL.seatTop + HIP_ABOVE_SEAT, y: 0, angleDeg: -90, "joint.axisDeg": 0 },
  torso: { lengthM: 0.475 },
  head: { x: 0.475, lengthM: 0.255, "mass.comX": 0.12, "joint.twistMinDeg": -70, "joint.twistMaxDeg": 70 },
  "upper-arm-l": { z: 0.19, lengthM: 0.327, "mass.comX": 0.1887 },
  "upper-arm-r": { z: -0.19, lengthM: 0.327, "mass.comX": 0.1887 },
  "forearm-l": { x: 0.327, lengthM: 0.257, "mass.comX": 0.1176 },
  "forearm-r": { x: 0.327, lengthM: 0.257, "mass.comX": 0.1176 },
  "hand-l": { x: 0.257, lengthM: 0.19, "mass.comX": 0.095 },
  "hand-r": { x: 0.257, lengthM: 0.19, "mass.comX": 0.095 },
  "thigh-l": { z: 0.09 },
  "thigh-r": { z: -0.09 },
  "shank-l": { lengthM: 0.435, "mass.comX": 0.194 },
  "shank-r": { lengthM: 0.435, "mass.comX": 0.194 },
  "foot-l": { x: 0.435 },
  "foot-r": { x: 0.435 },
  guitar: { parent: "pelvis", angleDeg: 0, lengthM: r4(GUITAR.length - GUITAR.pivot[0]), "joint.minDeg": -180, "joint.maxDeg": 180, "mass.kg": GUITAR.massKg, "mass.comX": r4(GUITAR.comA - GUITAR.pivot[0]), "mass.comY": 0 },
};
const TITLES: Record<string, Json> = {
  stool: { en: "Wooden stool, 24 inch, with rungs", de: "Holzhocker, 24 Zoll, mit Sprossen" },
  guitar: { en: "Dreadnought guitar, from its rest on the thigh to the head", de: "Dreadnought-Gitarre, von der Auflage auf dem Oberschenkel bis zum Kopf" },
};

/* ---------------------------------------------------------------- poses */

type HandGoal = { wrist: V3; u: V3; palm: V3 };
type PoseSpec = {
  torso: number;
  head: [angle: number, twist: number];
  guitar: [yaw: number, rise: number, tilt: number];
  left: HandGoal;
  right: HandGoal;
  elbowR: V3;
};
const PELVIS = 8;
const grip = (fret: number): HandGoal => ({ wrist: [(fretA(fret) + fretA(fret - 1)) / 2 + 0.012, -0.088, -0.066], u: [-0.08, 0.5, 0.86], palm: [0, 0.86, -0.5] });
const strum = (b: number): HandGoal => ({ wrist: [0.262, b, 0.062], u: [0.66, -0.68, -0.32], palm: [0.05, -0.2, -1] });
const POSES: Record<string, PoseSpec> = {
  strumming: { torso: -6, head: [-14, 22], guitar: [18, 14, 9], left: grip(3), right: strum(0.03), elbowR: [0.17, 0.24, 0.0] },
  "strum-up": { torso: -6, head: [-14, 22], guitar: [18, 14, 9], left: grip(3), right: strum(0.105), elbowR: [0.17, 0.24, 0.0] },
  "fretting-high": { torso: -11, head: [-20, 30], guitar: [18, 14, 9], left: grip(9), right: strum(0.03), elbowR: [0.17, 0.24, 0.0] },
  resting: { torso: -2, head: [-2, 0], guitar: [18, 20, 13], left: grip(12), right: { wrist: [0.3, 0.12, 0.05], u: [0.45, -0.8, -0.4], palm: [0.1, -0.3, -1] }, elbowR: [0.19, 0.245, 0.0] },
  tuning: { torso: -14, head: [-18, 40], guitar: [18, 14, 9], left: { wrist: [0.935, -0.03, -0.085], u: [0, 0.62, 0.78], palm: [0, 0.78, -0.62] }, right: strum(0.03), elbowR: [0.17, 0.24, 0.0] },
};
const DESCRIPTIONS: Record<string, Json> = {
  strumming: { en: "Seated on the stool, the right foot on the front rung, the left foot on the floor; the guitar lies across the body on the right thigh with the neck rising to the left, the right forearm lies over the lower bout with the hand over the strings, and the left hand holds a chord at the third fret.", de: "Auf dem Hocker sitzend, der rechte Fuß auf der vorderen Sprosse, der linke auf dem Boden; die Gitarre liegt quer vor dem Körper auf dem rechten Oberschenkel, der Hals steigt nach links an, der rechte Unterarm liegt über dem Unterbug mit der Hand über den Saiten, und die linke Hand greift einen Akkord im dritten Bund." },
  "fretting-high": { en: "The left hand slides up the neck to the ninth fret for a high chord, the trunk and head bend towards the fretboard and the player sings.", de: "Die linke Hand gleitet den Hals hinauf zum neunten Bund für einen hohen Akkord, Rumpf und Kopf neigen sich zum Griffbrett, und der Spieler singt." },
  resting: { en: "Back straight, looking ahead: the guitar stands more upright on the thigh, the right forearm rests on its upper edge and the left hand lies loosely around the neck at the twelfth fret.", de: "Gerader Rücken, Blick nach vorn: Die Gitarre steht aufrechter auf dem Oberschenkel, der rechte Unterarm ruht auf ihrer Oberkante, und die linke Hand liegt locker um den Hals am zwölften Bund." },
  tuning: { en: "Leaning towards the headstock: the left hand turns a tuning peg while the right hand plucks the string.", de: "Zur Kopfplatte geneigt: Die linke Hand dreht an einem Wirbel, während die rechte die Saite anzupft." },
  "strum-up": { en: "Key pose of the strumming loop: the right forearm has swung up from the elbow, the hand stands above the bass strings for the next down-stroke.", de: "Schlüsselpose der Anschlagschleife: Der rechte Unterarm ist aus dem Ellbogen nach oben geschwungen, die Hand steht über den Basssaiten vor dem nächsten Abschlag." },
};

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

/** 🎯️ The guitar axes of a playing position: the strings `a` (yawed forward by `yaw`, rising by `rise`), the top normal `c` (tilted back by `tilt`) and the bass side `b`. */
function guitarAxes([yaw, rise, tilt]: readonly number[]): { a: V3; b: V3; c: V3 } {
  const a: V3 = [Math.sin(rad(yaw!)) * Math.cos(rad(rise!)), Math.sin(rad(rise!)), Math.cos(rad(yaw!)) * Math.cos(rad(rise!))];
  const c0: V3 = [Math.cos(rad(yaw!)), 0, -Math.sin(rad(yaw!))];
  const c = add(([0, 0, 0] as V3), add(([c0[0] * Math.cos(rad(tilt!)), c0[1] * Math.cos(rad(tilt!)), c0[2] * Math.cos(rad(tilt!))] as V3), cross(a, c0), Math.sin(rad(tilt!))));
  return { a, b: cross(a, c), c };
}

const THIGH_REST: V3 = [0.225, 0.094, 0.035];
const SOLE = -0.0799;
const RUNG_ANCHOR: [number, number] = [0.045, -0.0725];

function legs(rig: Json, offsets: Offsets, report: string[]): void {
  offsets["thigh-r"] = { angleDeg: 80, abductionDeg: -13, twistDeg: 0 };
  offsets["shank-r"] = { angleDeg: -120 };
  offsets["foot-r"] = { angleDeg: 20 };
  const rung: V3 = [legAt(STOOL.rungFront), STOOL.rungFront + STOOL.rungR, -0.105];
  solve(rig, offsets, [["thigh-r", "angleDeg", -60, 125], ["thigh-r", "twistDeg", -60, 60], ["shank-r", "angleDeg", -150, 0], ["foot-r", "angleDeg", -65, 60]], (f) => {
    const p = at(f["foot-r"]!, [RUNG_ANCHOR[0], RUNG_ANCHOR[1], 0]);
    return [p[0] - rung[0], p[1] - rung[1], p[2] - rung[2], (f["foot-r"]!.u[1] + Math.sin(rad(10))) * 0.2];
  }, "right leg on the rung", report);
  offsets["thigh-l"] = { angleDeg: 60, abductionDeg: 12, twistDeg: 0 };
  offsets["shank-l"] = { angleDeg: -50 };
  offsets["foot-l"] = { angleDeg: 0, twistDeg: 0 };
  const ankle: V3 = [0.5, -SOLE, 0.2];
  solve(rig, offsets, [["thigh-l", "angleDeg", -60, 125], ["thigh-l", "twistDeg", -60, 60], ["shank-l", "angleDeg", -150, 0], ["foot-l", "angleDeg", -65, 60], ["foot-l", "abductionDeg", -40, 40], ["foot-l", "twistDeg", -40, 40]], (f) => {
    const g = f["foot-l"]!;
    return [g.o[0] - ankle[0], g.o[1] - ankle[1], g.o[2] - ankle[2], g.u[1] * 0.2, g.w[1] * 0.2, (g.u[2] - Math.sin(rad(12))) * 0.2];
  }, "left leg on the floor", report);
}

function arm(rig: Json, offsets: Offsets, side: "l" | "r", goal: { wrist: V3; u: V3; v: V3 }, elbow: V3, elbowWeight: number, label: string, report: string[]): void {
  const [upper, fore, hand] = [`upper-arm-${side}`, `forearm-${side}`, `hand-${side}`];
  offsets[upper] ??= { angleDeg: 30, abductionDeg: side === "l" ? 15 : -25, twistDeg: side === "l" ? -20 : 40 };
  offsets[fore] ??= { angleDeg: 90, twistDeg: 0 };
  offsets[hand] ??= { angleDeg: 0, abductionDeg: 0 };
  solve(rig, offsets, [[upper, "angleDeg", -60, 180], [upper, "abductionDeg", -100, 100], [upper, "twistDeg", -120, 120], [fore, "angleDeg", 0, 150], [fore, "twistDeg", -179, 179], [hand, "angleDeg", -65, 75], [hand, "abductionDeg", -80, 80]], (f) => {
    const h = f[hand]!;
    const e = f[fore]!.o;
    return [h.o[0] - goal.wrist[0], h.o[1] - goal.wrist[1], h.o[2] - goal.wrist[2], ...[0, 1, 2].map((k) => (h.u[k]! - goal.u[k]!) * 0.08), ...[0, 1, 2].map((k) => (h.v[k]! - goal.v[k]!) * 0.08), ...[0, 1, 2].map((k) => (e[k]! - elbow[k]!) * elbowWeight)];
  }, label, report);
}

function solvePose(rig: Json, id: string, spec: PoseSpec, report: string[], pivot?: V3): { offsets: Offsets; pivot: V3 } {
  const offsets: Offsets = { pelvis: { angleDeg: PELVIS }, torso: { angleDeg: spec.torso - PELVIS }, head: { angleDeg: spec.head[0], twistDeg: spec.head[1] } };
  legs(rig, offsets, report);
  const axes = guitarAxes(spec.guitar);
  offsets.guitar = { angleDeg: -60, abductionDeg: 60, twistDeg: 0 };
  solve(rig, offsets, [["guitar", "angleDeg", -180, 180], ["guitar", "abductionDeg", -180, 180], ["guitar", "twistDeg", -180, 180]], (f) => {
    const g = f.guitar!;
    return [...[0, 1, 2].map((k) => g.u[k]! - axes.a[k]!), ...[0, 1, 2].map((k) => g.v[k]! + axes.c[k]!)];
  }, `${id} guitar`, report);
  let f = framesOf(rig, offsets);
  const rest = at(f["thigh-r"]!, THIGH_REST);
  if (pivot === undefined) {
    const p = f.pelvis!;
    const d: V3 = [rest[0] - p.o[0], rest[1] - p.o[1], rest[2] - p.o[2]];
    pivot = [r4(dot(d, p.u)), r4(dot(d, p.v)), r4(dot(d, p.w))];
    const bone = (rig.bones as Json[]).find((b) => b.id === "guitar")!;
    [bone.x, bone.y, bone.z] = pivot;
    f = framesOf(rig, offsets);
  }
  const g = f.guitar!;
  const world = (p: V3): V3 => at(g, onGuitar(p[0], p[1], p[2]));
  const dir = (p: V3): V3 => unit(add(add(([axes.a[0] * p[0], axes.a[1] * p[0], axes.a[2] * p[0]] as V3), axes.b, p[1]), axes.c, p[2]));
  const goal = (h: HandGoal): { wrist: V3; u: V3; v: V3 } => {
    const u = dir(h.u);
    const n0 = dir(h.palm);
    return { wrist: world(h.wrist), u, v: unit(add(n0, u, -dot(n0, u))) };
  };
  const shoulderL = f["upper-arm-l"]!.o;
  arm(rig, offsets, "r", goal(spec.right), world(spec.elbowR), 0.35, `${id} right arm`, report);
  arm(rig, offsets, "l", goal(spec.left), add(shoulderL, [0.07, -0.285, 0.085]), 0.12, `${id} left arm`, report);
  return { offsets, pivot };
}

/* ---------------------------------------------------------------- the document */

const ANCHORS: Record<string, Json> = {
  "rung-support": { bone: "foot-r", x: RUNG_ANCHOR[0], y: RUNG_ANCHOR[1], title: { en: "Right boot on the front rung", de: "Rechter Stiefel auf der vorderen Sprosse" } },
  "fret-hand": { x: r4(fretA(2.5) - GUITAR.pivot[0]), y: r4(-(0.0065 - GUITAR.pivot[2])), z: r4(GUITAR.pivot[1]) },
  "strum-hand": { x: r4(0.33 - GUITAR.pivot[0]), y: r4(-(0.012 - GUITAR.pivot[2])), z: r4(GUITAR.pivot[1]) },
  "guitar-rest": { x: 0, y: 0, z: 0, normalDeg: 90 },
  seat: { x: -HIP_ABOVE_SEAT, y: 0.02 },
  mouth: { x: 0.066, y: -0.1185 },
  nostrils: { x: 0.0855, y: -0.131 },
  crown: { x: 0.2595, y: -0.024 },
};

/** 🌱️ The catalog document with the root fixes applied (rig, anchors, poses, texts); `report` receives one line per solved chain. */
export function rooted(report: string[] = []): Json {
  const document = JSON.parse(readFileSync(ASSET, "utf8")) as Json;
  const asset = document.asset as Json;
  for (const bone of asset.rig.bones as Json[]) {
    for (const [key, value] of Object.entries(BONES[bone.id] ?? {})) {
      const [group, field] = key.split(".");
      if (field === undefined) bone[key] = value;
      else bone[group!][field] = value;
    }
    if (TITLES[bone.id] !== undefined) bone.title = TITLES[bone.id];
  }
  let pivot: V3 | undefined;
  const order = [asset.presentation.defaultPose as string, ...(asset.poses as Json[]).map((p) => p.id as string).filter((id) => id !== asset.presentation.defaultPose)];
  for (const id of order) {
    const pose = (asset.poses as Json[]).find((p) => p.id === id)!;
    const solved = solvePose(asset.rig, id, POSES[id]!, report, pivot);
    pivot = solved.pivot;
    pose.bones = Object.fromEntries((asset.rig.bones as Json[]).map((b) => b.id as string).filter((b) => solved.offsets[b] !== undefined).map((b) => [b, Object.fromEntries(Object.entries(solved.offsets[b]!).filter(([k, v]) => k === "angleDeg" || v !== 0))]));
    pose.description = DESCRIPTIONS[id];
    pose.contacts = ["stool-ground", "seat", "rung-support", "contact-l", "heel-l"];
  }
  for (const anchor of asset.anchors as Json[]) Object.assign(anchor, ANCHORS[anchor.id] ?? {});
  for (const clip of asset.clips as Json[]) {
    const have = new Set((clip.contacts as Json[]).map((c) => c.anchor));
    for (const anchor of ["contact-l", "heel-l"]) if (!have.has(anchor)) clip.contacts.push({ anchor, fromS: 0, toS: clip.durationS });
  }
  return document;
}

if (import.meta.main) {
  const report: string[] = [];
  const document = rooted(report);
  console.log(report.join("\n"));
  if (process.argv.includes("--print")) {
    const asset = document.asset as Json;
    for (const pose of asset.poses as Json[]) {
      const f = framesOf(asset.rig, pose.bones);
      console.log(pose.id, JSON.stringify(pose.bones));
      for (const id of ["pelvis", "torso", "head", "upper-arm-r", "forearm-r", "hand-r", "upper-arm-l", "forearm-l", "hand-l", "thigh-r", "shank-r", "foot-r", "thigh-l", "shank-l", "foot-l", "toe-l", "guitar"]) console.log("  ", id.padEnd(12), f[id]!.o.map((x) => x.toFixed(3)).join(" "), " u", f[id]!.u.map((x) => x.toFixed(2)).join(" "), " w", f[id]!.w.map((x) => x.toFixed(2)).join(" "));
    }
  }
}
