/** 🦴️ Root fixes of `humans/wheelchair-user` (round 3), stated as final values and solved again on every run (idempotent): the rig (hip joints ahead of the rear axle so the buttocks meet the back sling, lumbosacral joint behind the hips, C7, shoulder and atlanto-occipital joints at their seated heights, shoulder joints 0.35 m apart, a shank of 0.246 H, a foot from ankle to ball flat on the footplate), the anchors, every pose and every clip key solved in three dimensions with the product's own chain (`solveChain`) so that the gloved hands lie in the lap or grip the hand rims outboard of the wheels and move with them through the push, and the texts that named the old construction.
 * Usage: `bun r3-wheelchair-user-roots.ts [--print]` (as module: `rooted()` gives the catalog document with these fixes).
 * @see ./r3-wheelchair-user-ref.ts */
import { createHash } from "node:crypto";
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

import { solveChain } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts";
import { CHAIR, HEAD_ANCHORS, MAN, type V3 } from "./r3-wheelchair-user-ref.ts";

export type Json = Record<string, any>;
export const ASSET = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/wheelchair-user/🔣️.json";
export type Frame = { o: V3; u: V3; v: V3; w: V3 };
type Offsets = Record<string, Record<string, number>>;

const rad = (d: number): number => (d * Math.PI) / 180;
const deg = (r: number): number => (r * 180) / Math.PI;
const add = (a: V3, b: V3, s = 1): V3 => [a[0] + s * b[0], a[1] + s * b[1], a[2] + s * b[2]];
const norm = (a: V3): V3 => {
  const l = Math.hypot(...a);
  return [a[0] / l, a[1] / l, a[2] / l];
};
const r3 = (x: number): number => Math.round(x * 1e3) / 1e3;
const r4 = (x: number): number => Math.round(x * 1e4) / 1e4;
export const at = (f: Frame, p: V3): V3 => add(add(add(f.o, f.u, p[0]), f.v, p[1]), f.w, p[2]);

export function framesOf(rig: Json, offsets: Offsets, rootDeg = 0): Record<string, Frame> {
  return Object.fromEntries(Object.entries(solveChain(rig as never, offsets as never, { x: 0, y: 0, angleDeg: rootDeg })).map(([id, b]) => [id, (b as Json).frame as Frame]));
}

/* ---------------------------------------------------------------- rig */

const THIGH_ABS = deg(Math.atan2(MAN.knee[1] - MAN.hip[1], MAN.knee[0] - MAN.hip[0]));
export const ANKLE: [number, number] = [MAN.knee[0] + MAN.shank * Math.cos(rad(-80)), MAN.knee[1] + MAN.shank * Math.sin(rad(-80))];
const FOOT_ABS = -deg(Math.atan2(ANKLE[1] - (CHAIR.footplate.top + 0.035), MAN.ballAhead));
export const FOOT_LENGTH = r4(Math.hypot(ANKLE[1] - (CHAIR.footplate.top + 0.035), MAN.ballAhead));
export const FOOT_DROP = -FOOT_ABS;

const BONES: Record<string, Json> = {
  pelvis: { x: MAN.hip[0], y: r4(MAN.hip[1] - CHAIR.wheelR), lengthM: MAN.pelvis },
  torso: { x: MAN.pelvis, y: MAN.lumbosacralBehind, angleDeg: MAN.torsoLean, lengthM: MAN.torso, "mass.comX": r4(MAN.torso * 0.44), "mass.comY": 0.01 },
  neck: { x: MAN.torso, angleDeg: MAN.neckAngle, lengthM: MAN.neck, "mass.comX": r4(MAN.neck / 2) },
  head: { x: MAN.neck, angleDeg: MAN.headAngle, lengthM: MAN.head, "mass.comX": 0.03, "mass.comY": -0.012 },
  "arm-l": { x: MAN.shoulderOnTorso, y: -MAN.shoulderAhead, z: MAN.shoulderZ },
  "arm-r": { x: MAN.shoulderOnTorso, y: -MAN.shoulderAhead, z: -MAN.shoulderZ },
  "thigh-l": { angleDeg: r4(THIGH_ABS - 90), z: MAN.hipZ },
  "thigh-r": { angleDeg: r4(THIGH_ABS - 90), z: -MAN.hipZ },
};
for (const s of ["l", "r"]) {
  BONES[`shank-${s}`] = { angleDeg: r4(-80 - THIGH_ABS), lengthM: MAN.shank, "mass.comX": r4(0.4459 * MAN.shank) };
  BONES[`foot-${s}`] = { x: MAN.shank, angleDeg: r4(FOOT_ABS + 80), lengthM: FOOT_LENGTH, "mass.comX": 0.052, "mass.comY": -0.022 };
  BONES[`toe-${s}`] = { x: FOOT_LENGTH, angleDeg: r4(-FOOT_ABS), lengthM: 0.066, "mass.comX": 0.03, "mass.comY": 0.006 };
}
const TITLES: Record<string, Json> = {
  frame: { en: "Rigid wheelchair frame with seat, backrest and footplate", de: "Starrer Rollstuhlrahmen mit Sitz, Rückenlehne und Fußplatte" },
  "rear-wheel": { en: "Rear wheel pair with hand rims (24 inch)", de: "Hinterradpaar mit Greifreifen (24 Zoll)" },
  "castor": { en: "Front castor wheel pair (135 mm)", de: "Vorderes Lenkrollenpaar (135 mm)" },
};

/* ---------------------------------------------------------------- anchors */

/** 🖐️ The grip point of the hand in its bone frame: the middle of the palm over the knuckles, where the hand rim lies when the fingers close round it. */
export const GRIP: V3 = [0.082, -0.026, 0];
const ANCHORS: Record<string, Json> = {
  "rim-grip": { x: GRIP[0], y: GRIP[1] },
  "rim-grip-l": { x: GRIP[0], y: GRIP[1] },
  ...HEAD_ANCHORS,
};

/* ---------------------------------------------------------------- solver */

type Param = [bone: string, field: "angleDeg" | "abductionDeg" | "twistDeg", min: number, max: number];
/** 🧮️ Levenberg-Marquardt over a few joint offsets: `residual` reads the solved frames, the offsets stay inside their limits. */
function solve(rig: Json, offsets: Offsets, params: readonly Param[], residual: (f: Record<string, Frame>) => number[], rootDeg: number): number {
  const get = (): number[] => params.map(([b, k]) => offsets[b]?.[k] ?? 0);
  const set = (x: readonly number[]): void => params.forEach(([b, k, lo, hi], i) => { (offsets[b] ??= {})[k] = Math.min(Math.max(x[i]!, lo), hi); });
  const cost = (r: readonly number[]): number => r.reduce((s, e) => s + e * e, 0);
  let x = get();
  set(x);
  let r = residual(framesOf(rig, offsets, rootDeg));
  let lambda = 1e-2;
  for (let step = 0; step < 300 && cost(r) > 1e-14; step++) {
    const J = params.map((_, i) => {
      const y = [...x];
      y[i]! += 1e-3;
      set(y);
      const q = residual(framesOf(rig, offsets, rootDeg));
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
      const q = residual(framesOf(rig, offsets, rootDeg));
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
  const q = residual(framesOf(rig, offsets, rootDeg));
  return Math.sqrt(q.slice(0, 3).reduce((s, e) => s + e * e, 0));
}

/** ✋️ Where a hand goes, in the frame of `bone` (`frame` for the hand rims, `world` for the lap): the grip point, the direction of the fingers, the direction the palm faces and a point the elbow is drawn towards. */
type HandGoal = { grip: V3; fingers: V3; palm: V3; elbow: V3; bone: "frame" | "world" };

function arm(rig: Json, offsets: Offsets, s: "l" | "r", goal: HandGoal, rootDeg: number): number {
  const params: Param[] = [[`arm-${s}`, "angleDeg", -60, 170], [`arm-${s}`, "abductionDeg", -90, 90], [`arm-${s}`, "twistDeg", -90, 90], [`forearm-${s}`, "angleDeg", 0, 150], [`forearm-${s}`, "abductionDeg", -8, 8], [`forearm-${s}`, "twistDeg", -90, 90], [`hand-${s}`, "angleDeg", -70, 70], [`hand-${s}`, "abductionDeg", -30, 30]];
  const residual = (f: Record<string, Frame>): number[] => {
    const base = goal.bone === "world" ? null : f.frame!;
    const to = (p: V3): V3 => (base === null ? p : at(base, p));
    const dir = (p: V3): V3 => (base === null ? norm(p) : norm(add(at(base, p), base.o, -1)));
    const [g, e, u, n] = [to(goal.grip), to(goal.elbow), dir(goal.fingers), dir(goal.palm)];
    const hand = f[`hand-${s}`]!;
    const fore = f[`forearm-${s}`]!;
    const p = at(hand, GRIP);
    return [p[0] - g[0], p[1] - g[1], p[2] - g[2], (fore.o[0] - e[0]) * 0.08, (fore.o[1] - e[1]) * 0.08, (fore.o[2] - e[2]) * 0.08, (hand.u[0] - u[0]) * 0.04, (hand.u[1] - u[1]) * 0.04, (hand.u[2] - u[2]) * 0.04, (-hand.v[0] - n[0]) * 0.02, (-hand.v[1] - n[1]) * 0.02, (-hand.v[2] - n[2]) * 0.02];
  };
  return solve(rig, offsets, params, residual, rootDeg);
}

/* ---------------------------------------------------------------- push kinematics */

/** 🛞️ A hand goal on the hand rim at angle `theta` (degrees from ahead, counter-clockwise seen from the left, in the frame of the chair), lifted `lift` metres radially above the rim. */
function rimGoal(s: "l" | "r", theta: number, lift: number): HandGoal {
  const side = s === "l" ? 1 : -1;
  const [c, n] = [Math.cos(rad(theta)), Math.sin(rad(theta))];
  const r = CHAIR.handRim.r + lift;
  const grip: V3 = [r * c, r * n, side * CHAIR.handRim.z];
  const fingers: V3 = norm([n * 0.906 - c * 0.423, -c * 0.906 - n * 0.423, side * 0.25]);
  const palm: V3 = norm([-c, -n, -side * 0.55]);
  const elbow: V3 = [-0.14 + 0.25 * Math.max(0, c), 0.5 + 0.2 * n, side * 0.27];
  return { grip, fingers, palm, elbow, bone: "frame" };
}

/** ♻️ The recovery of a stroke: grip angle and lift over the recovery share `s` (0 at release, 1 at the next catch), the hands following through ahead of the release, rising over the rim and coming back down behind the top. */
const RECOVERY: readonly (readonly [s: number, dTheta: number, lift: number])[] = [[0, 0, 0], [0.08, -6, 0.015], [0.2, -9, 0.05], [0.4, 8, 0.11], [0.6, 30, 0.13], [0.8, 46, 0.07], [0.92, 49, 0.02], [1, 45, 0]];
function recovery(sh: number): [number, number] {
  let i = 1;
  while (i < RECOVERY.length - 1 && RECOVERY[i]![0] < sh) i++;
  const p = (k: number): readonly [number, number, number] => RECOVERY[Math.max(0, Math.min(RECOVERY.length - 1, k))]!;
  const [a, b, c, d] = [p(i - 2), p(i - 1), p(i), p(i + 1)];
  const t = (sh - b[0]) / (c[0] - b[0]);
  const cr = (k: 1 | 2): number => {
    const m1 = ((c[k] - a[k]) / (c[0] - a[0] || 1)) * (c[0] - b[0]);
    const m2 = ((d[k] - b[k]) / (d[0] - b[0] || 1)) * (c[0] - b[0]);
    const t2 = t * t;
    const t3 = t2 * t;
    return (2 * t3 - 3 * t2 + 1) * b[k] + (t3 - 2 * t2 + t) * m1 + (-2 * t3 + 3 * t2) * c[k] + (t3 - t2) * m2;
  };
  return [cr(1), Math.max(0, cr(2))];
}

/** 🎬️ The hand goal of a clip time: on the rim through the push (it turns with the wheel from the catch angle), then the recovery. */
function cycleGoal(s: "l" | "r", t: number, period: number, push: number, catchDeg: number, wheelDeg: (t: number) => number): HandGoal {
  const phase = t - Math.floor(t / period + 1e-9) * period;
  const start = t - phase;
  if (phase <= push + 1e-9) return rimGoal(s, catchDeg + (wheelDeg(t) - wheelDeg(start)), 0);
  const release = catchDeg + (wheelDeg(start + push) - wheelDeg(start));
  const [d, lift] = recovery((phase - push) / (period - push));
  return rimGoal(s, release + d, lift);
}

/* ---------------------------------------------------------------- poses */

const LAP: Record<"l" | "r", HandGoal> = {
  l: { grip: [0.31, 0.685, 0.112], fingers: [0.93, -0.12, -0.3], palm: [0.05, -1, -0.12], elbow: [0.0, 0.84, 0.19], bone: "world" },
  r: { grip: [0.31, 0.685, -0.112], fingers: [0.93, -0.12, 0.3], palm: [0.05, -1, 0.12], elbow: [0.0, 0.84, -0.19], bone: "world" },
};
const CATCH = 100;
const POSE_GOALS: Record<string, (s: "l" | "r") => HandGoal> = {
  "seated-level": (s) => LAP[s],
  "hands-on-rims": (s) => rimGoal(s, CATCH, 0),
  "push-end": (s) => rimGoal(s, CATCH - 45, 0),
  "ramp-ascent": (s) => rimGoal(s, 98 - 24, 0),
};
const DESCRIPTIONS: Record<string, Json> = {
  "seated-level": { en: "Hands resting palm down on the thighs, back against the back sling, castors and rear wheels on the ground.", de: "Hände ruhen mit der Handfläche nach unten auf den Oberschenkeln, Rücken an der Rückenbespannung, Lenkrollen und Hinterräder auf dem Boden." },
  "hands-on-rims": { en: "The gloved hands grip the hand rims outboard of the wheels just behind the top (100 degrees from ahead), elbows back and out, trunk upright, ready to push.", de: "Die behandschuhten Hände greifen die außen an den Rädern sitzenden Greifreifen kurz hinter dem Scheitel (100 Grad von vorn), Ellbogen nach hinten und außen, Rumpf aufrecht, bereit zum Antrieb." },
  "push-end": { en: "End of the 45 degree push arc: the hands have turned with the rims to 55 degrees ahead of the axle, elbows nearly straight, trunk leaning forward.", de: "Ende des 45-Grad-Schubbogens: Die Hände haben sich mit den Greifreifen bis 55 Grad vor der Achse mitgedreht, Ellbogen fast gestreckt, Rumpf nach vorn geneigt." },
  "ramp-ascent": { en: "The whole chair is tilted 4.8 degrees about the rear axle onto a 1:12 ramp, the trunk leans forward and the hands push the rims half way through the stroke.", de: "Der ganze Rollstuhl ist um 4,8 Grad um die Hinterachse auf eine Rampe 1:12 gekippt, der Rumpf neigt sich vor und die Hände schieben die Greifreifen in der Mitte des Schubs." },
};

/* ---------------------------------------------------------------- document */

function setPath(target: Json, path: string, value: unknown): void {
  const keys = path.split(".");
  let o = target;
  for (const k of keys.slice(0, -1)) o = o[k];
  o[keys[keys.length - 1]!] = value;
}

const ARM_BONES = ["arm-l", "forearm-l", "hand-l", "arm-r", "forearm-r", "hand-r"];
const clean = (o: Record<string, number>): Record<string, number> => Object.fromEntries(Object.entries(o).filter(([k, v]) => k === "angleDeg" || v !== 0));

/** 🖐️ Solves both arms for one set of offsets (warm-started from `warm`) and returns the worst grip miss in metres. */
function hands(rig: Json, offsets: Offsets, goal: (s: "l" | "r") => HandGoal, rootDeg: number, warm?: Offsets): number {
  let worst = 0;
  for (const s of ["l", "r"] as const) {
    for (const b of [`arm-${s}`, `forearm-${s}`, `hand-${s}`]) offsets[b] = { ...(warm?.[b] ?? { angleDeg: b.startsWith("arm") ? 20 : b.startsWith("forearm") ? 60 : 0 }) };
    let miss = arm(rig, offsets, s, goal(s), rootDeg);
    if (miss > 0.004 && warm !== undefined) {
      for (const b of [`arm-${s}`, `forearm-${s}`, `hand-${s}`]) offsets[b] = { angleDeg: b.startsWith("arm") ? 0 : b.startsWith("forearm") ? 70 : 0 };
      miss = Math.min(miss, arm(rig, offsets, s, goal(s), rootDeg));
    }
    worst = Math.max(worst, miss);
  }
  return worst;
}

/** 📄️ The catalog document with the root fixes applied (everything except `volumes` and the 2D geometry, which the build adds). */
export function rooted(report: string[] = []): Json {
  const document = JSON.parse(readFileSync(ASSET, "utf8")) as Json;
  const asset = document.asset as Json;
  const key = createHash("sha256").update(readFileSync(import.meta.path)).update(readFileSync(join(import.meta.dir, "r3-wheelchair-user-ref.ts"))).update(JSON.stringify(["rig", "anchors", "poses", "clips", "dimensions"].map((k) => asset[k]))).digest("hex").slice(0, 16);
  const cache = join(import.meta.dir, "🗑️generated", "r3-wheelchair-user", `rooted-${key}.json`);
  if (existsSync(cache)) {
    const saved = JSON.parse(readFileSync(cache, "utf8")) as Json;
    Object.assign(asset, saved.members);
    report.push(...(saved.report as string[]));
    return document;
  }
  fix(asset, report);
  mkdirSync(join(import.meta.dir, "🗑️generated", "r3-wheelchair-user"), { recursive: true });
  writeFileSync(cache, JSON.stringify({ members: Object.fromEntries(["rig", "anchors", "poses", "clips", "dimensions"].map((k) => [k, asset[k]])), report }));
  return document;
}

function fix(asset: Json, report: string[]): void {
  for (const bone of asset.rig.bones as Json[]) {
    for (const [path, value] of Object.entries(BONES[bone.id] ?? {})) setPath(bone, path, value);
    if (TITLES[bone.id] !== undefined) bone.title = TITLES[bone.id];
  }
  for (const anchor of asset.anchors as Json[]) Object.assign(anchor, ANCHORS[anchor.id] ?? {});
  const rig = asset.rig as Json;
  for (const pose of asset.poses as Json[]) {
    const offsets: Offsets = JSON.parse(JSON.stringify(pose.bones));
    const miss = hands(rig, offsets, POSE_GOALS[pose.id]!, pose.root?.angleDeg ?? 0);
    for (const b of ARM_BONES) offsets[b] = clean(offsets[b]!);
    pose.bones = offsets;
    pose.description = DESCRIPTIONS[pose.id];
    report.push(`pose ${pose.id}: grip miss ${(miss * 1000).toFixed(2)} mm  ${ARM_BONES.slice(3).map((b) => `${b} ${JSON.stringify(offsets[b])}`).join(" ")}`);
  }
  for (const clip of asset.clips as Json[]) {
    if (clip.id === "rest-in-chair") continue;
    const ramp = clip.id === "ramp-roll";
    const keys = clip.keys as Json[];
    const wheel = (t: number): number => {
      let best = keys[0]!;
      for (const k of keys) if (Math.abs(k.t - t) < Math.abs(best.t - t)) best = k;
      return best.bones["rear-wheel"].angleDeg;
    };
    let warm: Offsets | undefined;
    let worst = 0;
    for (const key of keys) {
      const pose = (asset.poses as Json[]).find((p) => p.id === key.pose)!;
      const offsets: Offsets = { ...JSON.parse(JSON.stringify(pose.bones)), ...JSON.parse(JSON.stringify(key.bones)) };
      worst = Math.max(worst, hands(rig, offsets, (s) => cycleGoal(s, key.t, 1, ramp ? 0.25 : 0.1875, ramp ? 98 : CATCH, wheel), pose.root?.angleDeg ?? 0, warm));
      warm = JSON.parse(JSON.stringify(offsets));
      for (const b of ARM_BONES) key.bones[b] = clean(offsets[b]!);
    }
    report.push(`clip ${clip.id}: ${keys.length} keys, worst grip miss ${(worst * 1000).toFixed(2)} mm`);
  }
  asset.dimensions.massKg = r4((rig.bones as Json[]).reduce((s, b) => s + (b.mass?.kg ?? 0), 0));
}

if (import.meta.main) {
  const report: string[] = [];
  const document = rooted(report);
  console.log(report.join("\n"));
  const rig = document.asset.rig as Json;
  console.log("ankle", ANKLE.map(r4), "foot", FOOT_LENGTH, "drop", r3(FOOT_DROP));
  for (const pose of document.asset.poses as Json[]) {
    const f = framesOf(rig, pose.bones, pose.root?.angleDeg ?? 0);
    console.log(pose.id, Object.entries(f).filter(([id]) => /^(pelvis|torso|neck|head|arm-r|forearm-r|hand-r|thigh-r|shank-r|foot-r|toe-r)$/.test(id)).map(([id, g]) => `${id} ${g.o.map((x) => x.toFixed(3)).join(",")}`).join(" | "));
  }
  if (process.argv.includes("--print")) console.log(JSON.stringify(document.asset.rig.bones, null, 1));
}
