/** 🦴 Root fixes of `humans/pregnant-person` (round 3), stated as absolute values so the script can run any number of times on the original catalog document: rig landmarks after Winter 2009 and de Leva 1996 for a woman of 1.66 m in ankle boots with a 25 mm heel, the gravid abdomen bone at the uterus, anchors on the modelled body, standing legs re-solved by two-link geometry, arms placed by a small optimiser on the product's own chain (`--solve`, cached in `arms.json`), clips carried along, texts, dimensions and, when present, the redrawn 2D shapes and decisions. Writes the candidate document `🗑️generated/r3-pregnant-person/🔣️.json`; `--commit` replaces the owned members in the catalog file (re-read right before, member by member, the rest of the file untouched).
 *
 * Usage: `bun r3-pregnant-person-roots.ts [--solve] [--with-volumes] [--commit]`
 * @see ./r3-humans-pregnant-person.md */
import { fileURLToPath } from "node:url";
import { existsSync, readFileSync, writeFileSync, mkdirSync } from "node:fs";
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { sampleClip } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎞️animation/🟦️.ts";
import { type Frame } from "./r3-lib.ts";
import { dump } from "./r3-pregnant-person-fmt.ts";
import { CANDIDATE, OUT, SOURCE } from "./r3-pregnant-person-io.ts";
import { RIG } from "./r3-pregnant-person-data.ts";
import { BELLY_O, SWEATER, bellyY, sliceX, torsoAF } from "./r3-pregnant-person-forms.ts";

type Json = any;
type V3 = [number, number, number];
const CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/pregnant-person/🔣️.json";
const MANIFEST = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔣️.json";
const OWNED = ["summary", "provenance", "dimensions", "materials", "rig", "anchors", "representations", "poses", "clips", "thermal", "structure"];
const ORIGINAL = `${OUT}/original.json`;
const ARMS = `${OUT}/arms.json`;
const r4 = (x: number): number => Math.round(x * 1e4) / 1e4;
const r5 = (x: number): number => Math.round(x * 1e5) / 1e5;
const r2 = (x: number): number => Math.round(x * 100) / 100;
const deg = (r: number): number => (r * 180) / Math.PI;
const rad = (d: number): number => (d * Math.PI) / 180;
const cl = (x: number, lo: number, hi: number): number => Math.max(lo, Math.min(hi, x));

mkdirSync(OUT, { recursive: true });
if (!existsSync(ORIGINAL)) writeFileSync(ORIGINAL, readFileSync(CATALOG, "utf8"));
const doc = JSON.parse(readFileSync(ORIGINAL, "utf8")) as Json;
const asset = doc.asset as Json;
const old = JSON.parse(JSON.stringify(asset)) as Json;
const bone = (id: string): Json => asset.rig.bones.find((b: Json) => b.id === id);
const anchor = (id: string): Json => asset.anchors.find((a: Json) => a.id === id);
const pose = (id: string): Json => asset.poses.find((p: Json) => p.id === id);
const clip = (id: string): Json => asset.clips.find((c: Json) => c.id === id);

/* ------------------------------------------------------------------ rest-pose side view of a rig (x ahead, y up), for re-expressing centres of mass */
type P2 = { o: [number, number]; a: number };
function rest2(rig: Json, hip: number): Record<string, P2> {
  const out: Record<string, P2> = {};
  const by = new Map((rig.bones as Json[]).map((b) => [b.id, b]));
  const place = (id: string): P2 => {
    if (out[id]) return out[id]!;
    const b = by.get(id)!;
    if (b.parent === null) return (out[id] = { o: [b.x, hip], a: b.angleDeg });
    const p = place(b.parent);
    const c = Math.cos(rad(p.a));
    const s = Math.sin(rad(p.a));
    return (out[id] = { o: [p.o[0] + b.x * c - b.y * s, p.o[1] + b.x * s + b.y * c], a: p.a + b.angleDeg });
  };
  for (const b of rig.bones) place(b.id);
  return out;
}
const toWorld = (f: P2, x: number, y: number): [number, number] => [f.o[0] + x * Math.cos(rad(f.a)) - y * Math.sin(rad(f.a)), f.o[1] + x * Math.sin(rad(f.a)) + y * Math.cos(rad(f.a))];
const toLocal = (f: P2, p: [number, number]): [number, number] => {
  const d = [p[0] - f.o[0], p[1] - f.o[1]];
  return [d[0]! * Math.cos(rad(f.a)) + d[1]! * Math.sin(rad(f.a)), -d[0]! * Math.sin(rad(f.a)) + d[1]! * Math.cos(rad(f.a))];
};
const OLD_HIP = old.rig.bones[0].y;
const oldRest = rest2(old.rig, OLD_HIP);
const lifted = (id: string): [number, number] => {
  const b = old.rig.bones.find((q: Json) => q.id === id);
  const p = toWorld(oldRest[id]!, b.mass.comX, b.mass.comY ?? 0);
  return [p[0], p[1] + (RIG.hip - (0.065 + RIG.shank + RIG.thigh))];
};

/* ------------------------------------------------------------------ materials: the skeleton of the structure list */
if (!asset.materials.some((m: Json) => m.id === "bone")) asset.materials.push({ id: "bone", title: { en: "Bone", de: "Knochen" }, emissivity: 0.9, solarAbsorptivity: 0.5, densityKgM3: 1900, source: "authored-estimate" });

/* ------------------------------------------------------------------ rig */
const keepCom = ["torso", "head"].map((id) => [id, lifted(id)] as const);
Object.assign(bone("pelvis"), { y: r5(RIG.hip) });
Object.assign(bone("torso"), { x: RIG.pelvisLen, y: RIG.lumboBack, angleDeg: r4(RIG.torsoRel), lengthM: r5(RIG.torsoLen), title: { en: "Torso with sweater", de: "Rumpf mit Pullover" } });
Object.assign(bone("neck"), { x: r5(RIG.torsoLen), y: 0, angleDeg: r4(RIG.neckRel), lengthM: r5(RIG.neckLen) });
bone("neck").mass.comX = r4(RIG.neckLen / 2);
Object.assign(bone("head"), { x: r5(RIG.neckLen), y: 0, angleDeg: r4(RIG.headRel), lengthM: RIG.headLen });
Object.assign(bone("belly"), { x: RIG.bellyAt, y: 0, angleDeg: r4(-90 - RIG.torsoRel), lengthM: RIG.bellyLen });
bone("belly").mass.comX = RIG.bellyCom;
const fresh = rest2(asset.rig, RIG.hip);
const shoulder = toLocal(fresh.torso!, RIG.shoulder);
for (const s of ["l", "r"]) {
  Object.assign(bone(`arm-${s}`), { x: r5(shoulder[0]), y: r5(shoulder[1]), angleDeg: r4(180 - RIG.torsoRel), z: s === "l" ? RIG.shoulderZ : -RIG.shoulderZ });
  Object.assign(bone(`thigh-${s}`), { z: s === "l" ? RIG.hipZ : -RIG.hipZ });
  Object.assign(bone(`foot-${s}`), { angleDeg: RIG.footRel, lengthM: r5(RIG.footLen), title: s === "l" ? { en: "Left foot in the ankle boot", de: "Linker Fuß im Stiefel" } : { en: "Right foot in the ankle boot", de: "Rechter Fuß im Stiefel" } });
  Object.assign(bone(`foot-${s}`).mass, { comX: 0.056, comY: -0.035 });
  Object.assign(bone(`toe-${s}`), { x: r5(RIG.footLen), y: r5(-RIG.ankle), angleDeg: 0, lengthM: RIG.toeLen });
  Object.assign(bone(`toe-${s}`).mass, { comX: r4(RIG.toeLen * 0.45), comY: 0.022 });
}
const fresh2 = rest2(asset.rig, RIG.hip);
for (const [id, p] of keepCom) {
  const l = toLocal(fresh2[id]!, p);
  bone(id).mass.comX = r4(l[0]);
  bone(id).mass.comY = r4(l[1]);
}

/* ------------------------------------------------------------------ anchors (positions on the modelled body, see the 1:50 model) */
for (const s of ["l", "r"]) {
  Object.assign(anchor(`heel-${s}`), { x: -0.036, y: r5(-RIG.ankle), footprintLengthM: 0.062, footprintWidthM: 0.058 });
  Object.assign(anchor(`ball-${s}`), { x: r5(RIG.footLen), y: r5(-RIG.ankle), footprintLengthM: 0.095, footprintWidthM: 0.086 });
  anchor(`palm-${s}`).x = 0.075;
}
export const HEAD_ANCHORS: Record<string, [number, number, number?]> = { mouth: [-0.0215, -0.098], nostrils: [-0.0005, -0.107], crown: [0.152, 0.012] };
for (const [id, [x, y]] of Object.entries(HEAD_ANCHORS)) Object.assign(anchor(id), { x, y });
Object.assign(anchor("belly-front"), { x: 0.286, y: -0.012 });
Object.assign(anchor("belly-cog"), { x: RIG.bellyCom, y: -0.01 });

/* ------------------------------------------------------------------ standing legs */
const chord = (k: number): number => Math.sqrt(RIG.thigh ** 2 + RIG.shank ** 2 + 2 * RIG.thigh * RIG.shank * Math.cos(rad(k)));
/** 🦵 Thigh, shank and foot offsets that put the ankle `dx` ahead of the hip with the knee bent `knee` degrees and the sole level, the pelvis turned `pelvisDeg` from upright. */
function leg(pelvisDeg: number, dx: number, knee: number): Json {
  const d = chord(knee);
  const h = Math.sqrt(d * d - dx * dx);
  const alpha = Math.acos(cl((RIG.thigh ** 2 + d * d - RIG.shank ** 2) / (2 * RIG.thigh * d), -1, 1));
  const thighW = deg(Math.atan2(-h, dx) + alpha);
  const pelvisW = 90 + pelvisDeg;
  const t = thighW - (pelvisW - 180);
  const shankW = thighW - knee;
  return { thigh: r2(t), shank: r2(-knee), foot: r2(-(shankW + 90)) };
}
/** 🦿 Both legs of a standing pose, the feet under the hip joints (0.17 m apart). */
function standing(pelvisDeg: number, dx: number, knee: number): Json {
  const l = leg(pelvisDeg, dx, knee);
  return { "thigh-l": { angleDeg: l.thigh }, "shank-l": { angleDeg: l.shank }, "foot-l": { angleDeg: l.foot }, "toe-l": { angleDeg: 0 }, "thigh-r": { angleDeg: l.thigh }, "shank-r": { angleDeg: l.shank }, "foot-r": { angleDeg: l.foot }, "toe-r": { angleDeg: 0 } };
}
type Stand = { pelvis: number; torso: number; neck: number; head: number; dx: number; knee: number };
export const STAND: Record<string, Stand> = {
  "hand-on-back": { pelvis: -3, torso: 9, neck: -2, head: -4, dx: -0.02, knee: 5 },
  "cradling-belly": { pelvis: -2, torso: 6, neck: -2, head: -14, dx: -0.02, knee: 5 },
  "stretching-back": { pelvis: -4, torso: 12, neck: 2, head: 0, dx: -0.03, knee: 6 },
};
for (const [id, s] of Object.entries(STAND)) {
  const p = pose(id);
  Object.assign(p.bones, { pelvis: { angleDeg: s.pelvis }, torso: { angleDeg: s.torso }, neck: { angleDeg: s.neck }, head: { angleDeg: s.head } }, standing(s.pelvis, s.dx, s.knee));
}
const wc = pose("waddle-contact");
Object.assign(wc.bones, { neck: { angleDeg: -2 }, head: { angleDeg: -3 } });

/* ------------------------------------------------------------------ arms: optimiser on the product's chain */
const light = (): Json => ({ ...asset, representations: Object.fromEntries(Object.entries(asset.representations).map(([k, v]: [string, Json]) => [k, { ...v, geometry: { elevation: [], front: [], plan: [] } }])) });
export function frames(a: Json, poseId: string): Record<string, Frame> {
  const s = evaluate(a as never, { kind: "pose", pose: poseId } as never) as Json;
  const v = (p: Json): V3 => [p.x, p.y, p.z ?? 0];
  const out: Record<string, Frame> = {};
  for (const b of a.rig.bones) out[b.id] = { o: v(s.bones[b.id].frame.o), u: v(s.bones[b.id].frame.u), v: v(s.bones[b.id].frame.v), w: v(s.bones[b.id].frame.w) };
  return out;
}
export const at = (f: Frame, p: readonly number[]): V3 => [0, 1, 2].map((k) => f.o[k]! + p[0]! * f.u[k]! + p[1]! * f.v[k]! + (p[2] ?? 0) * f.w[k]!) as V3;
const sub = (a: V3, b: V3): V3 => [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
const dotv = (a: readonly number[], b: readonly number[]): number => a[0]! * b[0]! + a[1]! * b[1]! + a[2]! * b[2]!;
const d2 = (a: V3, b: V3): number => dotv(sub(a, b), sub(a, b));
const unit = (a: V3): V3 => { const l = Math.hypot(...a); return [a[0] / l, a[1] / l, a[2] / l]; };

type Param = [string, string, number, number, number];
/** 🎯 Nelder-Mead over the named bone offsets of a pose, minimising `cost(frames)`. */
function solve(poseId: string, params: Param[], cost: (f: Record<string, Frame>) => number, report?: (f: Record<string, Frame>) => string): void {
  const p = pose(poseId);
  const lt = light();
  const lp = lt.poses.find((q: Json) => q.id === poseId);
  const set = (x: number[]): void => params.forEach(([b, key, , lo, hi], i) => { lp.bones[b] = { ...(lp.bones[b] ?? {}), [key]: cl(x[i]!, lo, hi) }; });
  const f = (x: number[]): number => { set(x); return cost(frames(lt, poseId)) + x.reduce((s, xi, i) => s + (xi < params[i]![3] || xi > params[i]![4] ? 1 : 0), 0); };
  const n = params.length;
  let simplex = [params.map((q) => q[2])];
  for (let i = 0; i < n; i++) simplex.push(simplex[0]!.map((x, j) => (j === i ? x + 15 : x)));
  let values = simplex.map(f);
  for (let it = 0; it < 1500; it++) {
    const order = values.map((v, i) => [v, i] as const).sort((a, b) => a[0] - b[0]).map((q) => q[1]);
    simplex = order.map((i) => simplex[i]!);
    values = order.map((i) => values[i]!);
    if (values[n]! - values[0]! < 1e-11 && it > 60) break;
    const c = simplex[0]!.map((_, j) => simplex.slice(0, n).reduce((s, x) => s + x[j]!, 0) / n);
    const xr = c.map((cj, j) => cj + (cj - simplex[n]![j]!));
    const fr = f(xr);
    if (fr < values[0]!) {
      const xe = c.map((cj, j) => cj + 2 * (cj - simplex[n]![j]!));
      const fe = f(xe);
      [simplex[n], values[n]] = fe < fr ? [xe, fe] : [xr, fr];
    } else if (fr < values[n - 1]!) [simplex[n], values[n]] = [xr, fr];
    else {
      const xc = c.map((cj, j) => cj + 0.5 * (simplex[n]![j]! - cj));
      const fc = f(xc);
      if (fc < values[n]!) [simplex[n], values[n]] = [xc, fc];
      else { simplex = simplex.map((x) => x.map((xj, j) => simplex[0]![j]! + 0.5 * (xj - simplex[0]![j]!))); values = simplex.map(f); }
    }
  }
  const best = f(simplex[0]!);
  if (report) console.log(`[DEBUG]   ${report(frames(lt, poseId))}`);
  for (const [b, key] of params) p.bones[b] = { ...(p.bones[b] ?? {}), [key]: r2(lp.bones[b][key]) };
  console.log(`[DEBUG] ${poseId}: cost ${best.toExponential(2)} ${params.map(([b, k]) => `${b}.${k}=${p.bones[b][k]}`).join(" ")}`);
}
/** 📍 A point of the trunk: `a` up the torso bone, `fwd` ahead of it, `lat` to the left. */
const Tp = (f: Record<string, Frame>, a: number, fwd: number, lat: number): V3 => at(f.torso!, [a, -fwd, lat]);
/** 🫃 A point of the abdomen: `a` ahead along the belly bone, `up` above it, `lat` to the left. */
const Bp = (f: Record<string, Frame>, a: number, up: number, lat: number): V3 => at(f.belly!, [a, up, lat]);
const arm = (side: "l" | "r", a: number, ab: number, tw: number, fa: number, ft: number, ha: number): Param[] => [
  [`arm-${side}`, "angleDeg", a, -60, 170], [`arm-${side}`, "abductionDeg", ab, -90, 90], [`arm-${side}`, "twistDeg", tw, -120, 120], [`forearm-${side}`, "angleDeg", fa, 0, 150], [`forearm-${side}`, "twistDeg", ft, -170, 170], [`hand-${side}`, "angleDeg", ha, -60, 60],
];
/** 🖐 Palm centre, palm normal (+w of the right hand, -w of the left: the side the thumb folds to) and finger direction of a hand. */
const palm = (f: Record<string, Frame>, side: "l" | "r"): { c: V3; n: V3; dir: V3 } => {
  const h = f[`hand-${side}`]!;
  const n: V3 = side === "r" ? h.w : [-h.w[0], -h.w[1], -h.w[2]];
  return { c: at(h, [0.075, 0, side === "r" ? 0.012 : -0.012]), n, dir: h.u };
};
export type ArmTarget = { palm: (f: Record<string, Frame>) => V3; normal?: (f: Record<string, Frame>) => V3; fingers?: (f: Record<string, Frame>) => V3; elbow?: (f: Record<string, Frame>) => V3; wElbow?: number; wFingers?: number };
/** 🎯 The cost of one hand against its target: palm centre, palm normal towards the surface, finger direction and, weakly, the elbow. */
const armCost = (side: "l" | "r", t: ArmTarget) => (f: Record<string, Frame>): number => {
  const p = palm(f, side);
  let c = d2(p.c, t.palm(f));
  if (t.normal) c += 0.004 * (1 - dotv(p.n, unit(t.normal(f))));
  if (t.fingers) c += (t.wFingers ?? 0.002) * (1 - dotv(p.dir, unit(t.fingers(f))));
  if (t.elbow) c += (t.wElbow ?? 0.05) * d2(at(f[`forearm-${side}`]!, [0, 0, 0]), t.elbow(f));
  return c;
};
/** 🫸 A point on the sweater back of the upright woman at the height `y`, `lat` to the left, as a point of the posed trunk. */
const BACK_PALM = (f: Record<string, Frame>, lat: number, y = 1.05): V3 => { const [a, fw] = torsoAF(sliceX(SWEATER, y, lat, -1) - 0.003, y); return Tp(f, a, fw, lat); };
/** 🫴 A point on the underside (-1) or front of the abdomen cover at `x` ahead, `lat` to the left, as a point of the posed abdomen. */
const UNDER = (f: Record<string, Frame>, x: number, lat: number, off = -0.004): V3 => Bp(f, x - BELLY_O[0], bellyY(x, lat, -1) + off - BELLY_O[1], lat);
const down = (f: Record<string, Frame>): V3 => { const u = f.torso!.u; return [-u[0], -u[1], -u[2]]; };
const intoBack = (f: Record<string, Frame>): V3 => { const v = f.torso!.v; return [-v[0], -v[1], -v[2]]; };
/** 🤲 The hand-placement targets of every pose (palms on the modelled cloth surface, see the 1:50 trunk and abdomen). */
export const TARGETS: Record<string, Partial<Record<"l" | "r", ArmTarget>>> = {
  "hand-on-back": {
    r: { palm: (f) => BACK_PALM(f, -0.055), normal: intoBack, fingers: (f) => { const d = down(f); return [d[0], d[1], d[2] + 0.35]; }, elbow: (f) => Tp(f, 0.2, -0.17, -0.25), wElbow: 0.05 },
    l: { palm: (f) => UNDER(f, 0.2, 0.035), normal: (f) => f.belly!.v, fingers: (f) => { const u = f.belly!.u; const v = f.belly!.v; const w = f.belly!.w; return [u[0] * 0.3 + v[0] * 0.15 - w[0], u[1] * 0.3 + v[1] * 0.15 - w[1], u[2] * 0.3 + v[2] * 0.15 - w[2]]; }, wFingers: 0.008, elbow: (f) => Tp(f, 0.1, 0.02, 0.225), wElbow: 0.05 },
  },
  "cradling-belly": {
    r: { palm: (f) => Bp(f, 0.268, 0.05, -0.065), normal: (f) => { const u = f.belly!.u; const v = f.belly!.v; return [-u[0] - 0.45 * v[0], -u[1] - 0.45 * v[1], -u[2] - 0.45 * v[2]]; }, fingers: (f) => { const w = f.belly!.w; const v = f.belly!.v; return [w[0] - 0.3 * v[0], w[1] - 0.3 * v[1], w[2] - 0.3 * v[2]]; }, elbow: (f) => Tp(f, 0.2, 0.06, -0.22), wElbow: 0.03 },
    l: { palm: (f) => Bp(f, 0.262, -0.07, 0.06), normal: (f) => { const u = f.belly!.u; const v = f.belly!.v; return [-u[0] + 0.6 * v[0], -u[1] + 0.6 * v[1], -u[2] + 0.6 * v[2]]; }, fingers: (f) => { const w = f.belly!.w; return [-w[0], -w[1], -w[2]]; }, elbow: (f) => Tp(f, 0.12, 0.06, 0.22), wElbow: 0.03 },
  },
  "stretching-back": {
    r: { palm: (f) => BACK_PALM(f, -0.06), normal: intoBack, fingers: (f) => { const d = down(f); return [d[0], d[1], d[2] + 0.5]; }, elbow: (f) => Tp(f, 0.19, -0.19, -0.24), wElbow: 0.05 },
    l: { palm: (f) => BACK_PALM(f, 0.06), normal: intoBack, fingers: (f) => { const d = down(f); return [d[0], d[1], d[2] - 0.5]; }, elbow: (f) => Tp(f, 0.19, -0.19, 0.24), wElbow: 0.05 },
  },
};
TARGETS["waddle-contact"] = TARGETS["hand-on-back"]!;
const START: Record<"l" | "r", [number, number, number, number, number, number]> = { r: [-45, -25, -60, 95, -60, -20], l: [5, 20, 45, 95, -80, -10] };

const OLD_ARMS: Record<string, Json> = {};
for (const p of old.poses) OLD_ARMS[p.id] = Object.fromEntries(["arm-l", "forearm-l", "hand-l", "arm-r", "forearm-r", "hand-r"].map((b) => [b, { angleDeg: 0, abductionDeg: 0, twistDeg: 0, ...(p.bones[b] ?? {}) }]));
if (process.argv.includes("--solve")) {
  const cache: Record<string, Json> = {};
  for (const [id, sides] of Object.entries(TARGETS)) {
    for (const side of ["r", "l"] as const) {
      const t = sides[side];
      if (!t) continue;
      const prev = pose(id).bones;
      const s0 = START[side];
      const startOf = (b: string, k: string, d: number): number => prev[b]?.[k] ?? d;
      for (let round = 0; round < 3; round++) solve(id, arm(side, startOf(`arm-${side}`, "angleDeg", s0[0]), startOf(`arm-${side}`, "abductionDeg", s0[1]), startOf(`arm-${side}`, "twistDeg", s0[2]), startOf(`forearm-${side}`, "angleDeg", s0[3]), startOf(`forearm-${side}`, "twistDeg", s0[4]), startOf(`hand-${side}`, "angleDeg", s0[5])), armCost(side, t), (f) => { const p = palm(f, side); return `${side} palm off ${(Math.sqrt(d2(p.c, t.palm(f))) * 1000).toFixed(1)} mm, normal ${t.normal ? dotv(p.n, unit(t.normal(f))).toFixed(3) : "-"}, fingers ${t.fingers ? dotv(p.dir, unit(t.fingers(f))).toFixed(3) : "-"}, elbow off ${t.elbow ? (Math.sqrt(d2(at(f[`forearm-${side}`]!, [0, 0, 0]), t.elbow(f))) * 1000).toFixed(0) : "-"} mm`; });
    }
    cache[id] = Object.fromEntries(["arm-l", "forearm-l", "hand-l", "arm-r", "forearm-r", "hand-r"].map((b) => [b, pose(id).bones[b]]));
  }
  writeFileSync(ARMS, JSON.stringify(cache, null, 1));
}
if (existsSync(ARMS)) {
  const cache = JSON.parse(readFileSync(ARMS, "utf8")) as Record<string, Json>;
  for (const [id, bones] of Object.entries(cache)) Object.assign(pose(id).bones, bones);
}

/* ------------------------------------------------------------------ clips */
const walk = clip("waddle-walk");
const ARM_BONES = ["arm-l", "forearm-l", "hand-l", "arm-r", "forearm-r", "hand-r"];
for (const k of walk.keys) {
  for (const b of ARM_BONES) {
    if (!k.bones?.[b]) continue;
    const o = OLD_ARMS["waddle-contact"]![b];
    const n = { angleDeg: 0, abductionDeg: 0, twistDeg: 0, ...pose("waddle-contact").bones[b] };
    const ok = old.clips.find((c: Json) => c.id === "waddle-walk").keys[walk.keys.indexOf(k)].bones[b];
    const j = bone(b).joint;
    k.bones[b] = { angleDeg: r4(cl(n.angleDeg + (ok.angleDeg - o.angleDeg), j.minDeg + 0.5, j.maxDeg - 0.5)) };
  }
  for (const b of ["neck", "head"]) if (k.bones?.[b]) k.bones[b] = { angleDeg: wc.bones[b].angleDeg + r4((old.clips.find((c: Json) => c.id === "waddle-walk").keys[walk.keys.indexOf(k)].bones[b].angleDeg) - old.poses.find((p: Json) => p.id === "waddle-contact").bones[b].angleDeg) };
}
walk.advanceM = 0;
walk.advanceM = r4(sampleClip(asset as never, walk as never, walk.durationS - 1e-9).root.x - sampleClip(asset as never, walk as never, 0).root.x);
walk.description = { en: `Level walking at ${(walk.advanceM / walk.durationS).toFixed(2)} m/s, 92 steps per minute with a shorter stride and the feet at hip width; feet planted without sliding, torso leaning back to balance the abdomen, the right hand on the lower back, the left under the abdomen.`, de: `Gehen in der Ebene mit ${(walk.advanceM / walk.durationS).toFixed(2).replace(".", ",")} m/s, 92 Schritten pro Minute, kürzerem Schritt und Füßen in Hüftbreite; Füße gleiten nicht, Oberkörper lehnt zurück, um den Bauch auszugleichen, die rechte Hand am unteren Rücken, die linke unter dem Bauch.` };
const rest = clip("rest-and-breathe");
const legsOf = (id: string): Json => Object.fromEntries(Object.entries(pose(id).bones).filter(([b]) => /^(thigh|shank|foot|toe)-/.test(b)).map(([b, v]) => [b, { ...(v as Json) }]));
for (const k of rest.keys) if (k.bones) k.bones = legsOf("hand-on-back");

/* ------------------------------------------------------------------ texts, provenance, dimensions */
pose("hand-on-back").description = {
  en: "Leaning back about 6 degrees with the lumbar curve increased and the pelvis tipped forward, knees almost straight; the right hand rests flat on the lower back, the left hand cradles the underside of the abdomen; head up.",
  de: "Rund 6 Grad nach hinten gelehnt mit verstärkter Lendenkrümmung und nach vorn gekipptem Becken, die Knie fast gestreckt; die rechte Hand liegt flach am unteren Rücken, die linke Hand stützt den Bauch von unten; Kopf aufrecht.",
};
pose("cradling-belly").description = { en: "Both hands flat on the front of the abdomen, the right one higher, head bowed towards it, slight lean back.", de: "Beide Hände flach auf der Vorderseite des Bauchs, die rechte höher, Kopf dem Bauch zugeneigt, leichte Rücklage." };
pose("stretching-back").description = { en: "Both palms on the lower back beside the lumbar spine with the elbows drawn back, torso leaning back 8 degrees, head level, deep breath.", de: "Beide Handflächen am unteren Rücken neben der Lendenwirbelsäule, die Ellbogen nach hinten gezogen, Oberkörper 8 Grad nach hinten, Kopf gerade, tiefer Atemzug." };
asset.structure.assumptions[0] = {
  en: "Segments are point masses at the de Leva centres of the non-gravid body; the 6 kg fetal unit (fetus, placenta, amniotic fluid, uterus) is a point mass on the abdomen bone 0.15 m in front of the lumbar spine, the centre of the uterus.",
  de: "Segmente sind Punktmassen in den de-Leva-Schwerpunkten des nicht schwangeren Körpers; die 6 kg schwere Frucht (Kind, Plazenta, Fruchtwasser, Gebärmutter) ist eine Punktmasse auf dem Bauchknochen 0,15 m vor der Lendenwirbelsäule, in der Mitte der Gebärmutter.",
};
const refs = asset.provenance.references as Json[];
if (!refs.some((r) => r.id === "gordon-2014")) refs.push({ id: "gordon-2014", citation: "Gordon, C. C. et al. (2014). 2012 Anthropometric Survey of U.S. Army Personnel (ANSUR II): Methods and Summary Statistics. NATICK/TR-15/007 (female head, hand, foot, chest, waist, hip and limb breadths, depths and girths)." });
if (!refs.some((r) => r.id === "okanishi-2012")) refs.push({ id: "okanishi-2012", citation: "Okanishi, N., Kito, N., Akiyama, M. and Yamamoto, M. (2012). Spinal curvature and characteristics of postural change in pregnant women. Acta Obstetricia et Gynecologica Scandinavica 91(7), 856-861 (third-trimester lumbar lordosis, thoracic kyphosis and backward trunk lean)." });
asset.provenance.notes = {
  en: "Rig scaled to 1.66 m with de Leva female fractions for the non-gravid body mass plus a separate abdomen bone (6 kg fetal unit, guided by Hytten and Chamberlain). Round 3 (2026-11-20): landmarks after Winter 2009 in ankle boots with a 25 mm heel (lumbosacral joint 30 mm behind the hip joints, C7 at 0.855 H, atlanto-occipital joint 0.145 m below the vertex, shoulder joints at 0.818 H, foot to the ground under the ball), the abdomen bone moved to the centre of the uterus, standing legs straightened to 5 degrees of knee flexion, the arms re-placed on the modelled body; body, thermal model and skeleton handcrafted per scale, 2D drawings redrawn as views of the 3D body. Metabolism, ventilation and skin blood flow are third-trimester adjustments guided by Butte 2004, Lof 2005 and LoMauro and Aliverti 2015 and tagged as authored estimates.",
  de: "Skelett auf 1,66 m skaliert mit de-Leva-Frauenanteilen für die Körpermasse ohne Schwangerschaft plus einem eigenen Bauchknochen (6 kg Frucht, nach Hytten und Chamberlain). Runde 3 (2026-11-20): Gelenkpunkte nach Winter 2009 in Stiefeletten mit 25 mm Absatz (Lenden-Kreuzbein-Gelenk 30 mm hinter den Hüftgelenken, C7 bei 0,855 H, Kopfgelenk 0,145 m unter dem Scheitel, Schultergelenke bei 0,818 H, Fuß bis zum Boden unter dem Ballen), der Bauchknochen in die Mitte der Gebärmutter verlegt, die Standbeine auf 5 Grad Kniebeugung gestreckt, die Arme auf den modellierten Körper gesetzt; Körper, Wärmemodell und Skelett je Maßstab von Hand modelliert, 2D-Zeichnungen als Ansichten des 3D-Körpers neu gezeichnet. Stoffwechsel, Atmung und Hautdurchblutung sind Anpassungen für das dritte Trimester nach Butte 2004, Lof 2005 sowie LoMauro und Aliverti 2015 und als Schätzwerte gekennzeichnet.",
};
const DIMS = fileURLToPath(new URL("./r3-pregnant-person-dimensions.json", import.meta.url));
if (existsSync(DIMS)) Object.assign(asset.dimensions, JSON.parse(readFileSync(DIMS, "utf8")));

const SHAPES = `${OUT}/shapes.json`;
if (existsSync(SHAPES)) {
  const g = JSON.parse(readFileSync(SHAPES, "utf8")) as Json;
  for (const [scale, views] of Object.entries(g)) Object.assign(asset.representations[scale].geometry, views);
}
const TEXTS = fileURLToPath(new URL("./r3-pregnant-person-texts.json", import.meta.url));
if (existsSync(TEXTS)) {
  const t = JSON.parse(readFileSync(TEXTS, "utf8")) as Json;
  for (const [scale, d] of Object.entries(t.representations ?? {})) Object.assign(asset.representations[scale].decisions, d);
  if (t.summary) asset.summary = t.summary;
}
if (process.argv.includes("--with-volumes") && existsSync(SOURCE)) asset.volumes = JSON.parse(readFileSync(SOURCE, "utf8")).volumes;
writeFileSync(CANDIDATE, `${dump(doc)}\n`);
console.log("candidate", CANDIDATE);

/** ✂️ The text span of a member of the asset object (four-space indent) in the catalog file. */
function memberSpan(text: string, key: string): [number, number] {
  const m = new RegExp(`\\n    "${key}": `).exec(text);
  if (!m) throw new Error(`member ${key} not found`);
  let i = m.index + m[0].length;
  const start = m.index + 1;
  let depth = 0;
  let quoted = false;
  for (; i < text.length; i++) {
    const ch = text[i]!;
    if (quoted) { if (ch === "\\") i++; else if (ch === '"') quoted = false; continue; }
    if (ch === '"') quoted = true;
    else if (ch === "{" || ch === "[") depth++;
    else if (ch === "}" || ch === "]") { depth--; if (depth === 0) return [start, i + 1]; }
    else if (depth === 0 && (ch === "," || ch === "\n")) return [start, i];
  }
  throw new Error("unbalanced");
}
if (process.argv.includes("--commit")) {
  let text = readFileSync(CATALOG, "utf8");
  const live = JSON.parse(text) as Json;
  for (const k of OWNED) {
    if (JSON.stringify(live.asset[k]) === JSON.stringify(asset[k])) continue;
    const [a, b] = memberSpan(text, k);
    const body = dump({ [k]: asset[k] }, 1).split("\n").slice(1, -1).join("\n").trimStart();
    text = text.slice(0, a) + "    " + body + text.slice(b);
  }
  JSON.parse(text);
  writeFileSync(CATALOG, text);
  const fresh3 = readFileSync(MANIFEST, "utf8");
  const re = /("id": "pregnant-person",\s*"category": "humans",\s*"title": \{[^{}]*\},\s*"heightM": )([0-9.]+)(,\s*"widthM": )([0-9.]+)/;
  if (!re.test(fresh3)) throw new Error("manifest entry not found");
  const next = fresh3.replace(re, `$1${asset.dimensions.heightM}$3${asset.dimensions.widthM}`);
  if (next !== fresh3) writeFileSync(MANIFEST, next);
  console.log("committed", CATALOG);
}
