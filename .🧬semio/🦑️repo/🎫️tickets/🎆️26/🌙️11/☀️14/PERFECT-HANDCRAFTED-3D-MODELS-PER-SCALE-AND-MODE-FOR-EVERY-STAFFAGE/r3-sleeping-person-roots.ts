/** 🛏️ Root fixes of `humans/sleeping-person` (round 3), stated as final values and solved again on every run (idempotent): the rig of a 1.70 m woman lying supine on a soft mattress (C7, atlanto-occipital joint and vertex at their anthropometric distances, hip joints 0.174 m apart, shoulder joints 0.316 m apart, thigh, shank and foot lengths of the stature, joints at their heights above the mattress), a 40 x 80 cm pillow across the bed and a 135 x 200 cm duvet whose foot end is folded back over the shins, the supine pose solved in three dimensions with the product's own chain (left arm alongside the body under the duvet, right arm on the duvet with the hand on the belly, feet falling outward), the side poses lifted so the rolled trunk rests on the mattress, the anchors on the new surfaces, the passive region of the duvet beyond the body and the texts that named the old construction.
 * Usage: `bun r3-sleeping-person-roots.ts [--print]` (as module: `rooted()` gives the catalog document with these fixes).
 * @see ./r3-sleeping-person-ref.ts */
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

import { solveChain } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts";
import { ANCHORS, BED, WOMAN, type V3 } from "./r3-sleeping-person-ref.ts";

export type Json = Record<string, any>;
export const ASSET = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/sleeping-person/🔣️.json";
export type Frame = { o: V3; u: V3; v: V3; w: V3 };
type Offsets = Record<string, Record<string, number>>;

const r4 = (x: number): number => Math.round(x * 1e4) / 1e4;
const r2 = (x: number): number => Math.round(x * 100) / 100;
const sub = (a: V3, b: V3): V3 => [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
const dot = (a: V3, b: V3): number => a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
export const at = (f: Frame, p: V3): V3 => [0, 1, 2].map((k) => f.o[k]! + p[0] * f.u[k]! + p[1] * f.v[k]! + p[2] * f.w[k]!) as unknown as V3;
/** 📍️ A world point in the frame of `f` as `[u, v, w]`. */
export const local = (f: Frame, p: V3): V3 => {
  const d = sub(p, f.o);
  return [dot(d, f.u), dot(d, f.v), dot(d, f.w)];
};

export function framesOf(rig: Json, offsets: Offsets): Record<string, Frame> {
  return Object.fromEntries(Object.entries(solveChain(rig as never, offsets as never, { x: 0, y: 0, angleDeg: 0 })).map(([id, b]) => [id, (b as Json).frame as Frame]));
}

/* ---------------------------------------------------------------- rig */

const BONES: Record<string, Json> = {
  "hip-lift": { y: WOMAN.hipY - BED.top },
  pelvis: { lengthM: WOMAN.pelvis },
  torso: { x: WOMAN.pelvis, lengthM: WOMAN.torso, "mass.comX": r4(WOMAN.torso * 0.4555) },
  neck: { x: WOMAN.torso, angleDeg: -WOMAN.neckRiseDeg, lengthM: WOMAN.neck, "mass.comX": r4(WOMAN.neck / 2) },
  head: { x: WOMAN.neck, angleDeg: WOMAN.neckRiseDeg - WOMAN.headRiseDeg, lengthM: WOMAN.head, "mass.comX": 0.035, "mass.comY": -0.012 },
  "arm-l": { x: WOMAN.shoulderOnTorso, y: WOMAN.shoulderBelowLine, z: WOMAN.shoulderZ },
  "arm-r": { x: WOMAN.shoulderOnTorso, y: WOMAN.shoulderBelowLine, z: -WOMAN.shoulderZ },
  "thigh-l": { angleDeg: WOMAN.thighDeg, lengthM: WOMAN.thigh, z: WOMAN.hipZ, "mass.comX": r4(WOMAN.thigh * 0.3612) },
  "thigh-r": { angleDeg: WOMAN.thighDeg, lengthM: WOMAN.thigh, z: -WOMAN.hipZ, "mass.comX": r4(WOMAN.thigh * 0.3612) },
  "shank-l": { x: WOMAN.thigh, angleDeg: WOMAN.kneeDeg, lengthM: WOMAN.shank, "mass.comX": r4(WOMAN.shank * 0.4416) },
  "shank-r": { x: WOMAN.thigh, angleDeg: WOMAN.kneeDeg, lengthM: WOMAN.shank, "mass.comX": r4(WOMAN.shank * 0.4416) },
  "foot-l": { joint: { type: "revolute", minDeg: -34, maxDeg: 56 }, x: WOMAN.shank, angleDeg: WOMAN.footDeg, lengthM: WOMAN.foot, "mass.comX": r4(WOMAN.foot * 0.426), "mass.comY": -0.012 },
  "foot-r": { joint: { type: "revolute", minDeg: -34, maxDeg: 56 }, x: WOMAN.shank, angleDeg: WOMAN.footDeg, lengthM: WOMAN.foot, "mass.comX": r4(WOMAN.foot * 0.426), "mass.comY": -0.012 },
  "toe-l": { x: WOMAN.foot, angleDeg: 0, lengthM: WOMAN.toe, "mass.comX": 0.025, "mass.comY": -0.006 },
  "toe-r": { x: WOMAN.foot, angleDeg: 0, lengthM: WOMAN.toe, "mass.comX": 0.025, "mass.comY": -0.006 },
  pillow: { x: BED.pillow.x0, lengthM: BED.pillow.length, "mass.comX": r4(BED.pillow.length / 2), "mass.comY": 0.035 },
  duvet: { x: BED.duvet.x0, lengthM: BED.duvet.x1 - BED.duvet.x0, "mass.comX": 0.62, "mass.comY": 0.07 },
};
const TITLES: Record<string, Json> = {
  torso: { en: "Torso in pyjama top", de: "Rumpf im Schlafanzugoberteil" },
  pillow: { en: "Pillow 40 x 80 cm across the head end", de: "Kopfkissen 40 x 80 cm quer am Kopfende" },
  duvet: { en: "Duvet 135 x 200 cm, foot end folded back", de: "Bettdecke 135 x 200 cm, Fußende zurückgeschlagen" },
};

/* ---------------------------------------------------------------- pose solving */

type Goal = { bone: string; point: V3; target: V3; weight?: number } | { bone: string; axis: "u" | "v" | "w"; sign: number; target: V3; weight?: number };
type Param = { bone: string; key: "angleDeg" | "abductionDeg" | "twistDeg"; start: number };

/** 🎯️ Nelder-Mead over the named offsets until the goals (points of bones and directions of bone axes) are met; returns the offsets and the largest point error in metres. */
function solve(rig: Json, base: Offsets, params: readonly Param[], goals: readonly Goal[]): { offsets: Offsets; errorM: number } {
  const apply = (x: readonly number[]): Offsets => {
    const o: Offsets = JSON.parse(JSON.stringify(base));
    params.forEach((p, i) => ((o[p.bone] ??= {})[p.key] = x[i]!));
    return o;
  };
  const cost = (x: readonly number[]): number => {
    const f = framesOf(rig, apply(x));
    let c = 0;
    for (const g of goals) {
      const fr = f[g.bone]!;
      const got = "point" in g ? at(fr, g.point) : (fr[g.axis].map((q) => q * g.sign) as unknown as V3);
      const d = sub(got, g.target);
      c += (g.weight ?? 1) * dot(d, d) * ("point" in g ? 1 : 0.01);
    }
    return c;
  };
  let best = { x: params.map((p) => p.start), c: Infinity };
  for (const scale of [20, 8, 3]) {
    const n = params.length;
    let simplex = [best.x, ...params.map((_, i) => best.x.map((v, j) => (i === j ? v + scale : v)))].map((x) => ({ x, c: cost(x) }));
    for (let it = 0; it < 4000; it++) {
      simplex.sort((a, b) => a.c - b.c);
      const centroid = simplex.slice(0, n).reduce((s, p) => s.map((v, j) => v + p.x[j]! / n), new Array(n).fill(0) as number[]);
      const worst = simplex[n]!;
      const towards = (t: number): number[] => centroid.map((v, j) => v + t * (worst.x[j]! - v));
      const r = { x: towards(-1), c: 0 };
      r.c = cost(r.x);
      if (r.c < simplex[0]!.c) {
        const e = { x: towards(-2), c: 0 };
        e.c = cost(e.x);
        simplex[n] = e.c < r.c ? e : r;
      } else if (r.c < simplex[n - 1]!.c) simplex[n] = r;
      else {
        const k = { x: towards(0.5), c: 0 };
        k.c = cost(k.x);
        if (k.c < worst.c) simplex[n] = k;
        else simplex = simplex.map((p, i) => (i === 0 ? p : { x: p.x.map((v, j) => simplex[0]!.x[j]! + 0.5 * (v - simplex[0]!.x[j]!)), c: 0 })).map((p) => ({ ...p, c: p.c || cost(p.x) }));
      }
    }
    simplex.sort((a, b) => a.c - b.c);
    if (simplex[0]!.c < best.c) best = simplex[0]!;
  }
  const f = framesOf(rig, apply(best.x));
  const errorM = Math.max(0, ...goals.filter((g): g is Goal & { point: V3 } => "point" in g).map((g) => Math.sqrt(dot(sub(at(f[g.bone]!, g.point), g.target), sub(at(f[g.bone]!, g.point), g.target)))));
  return { offsets: apply(best.x.map(r2)), errorM };
}

/** 🛌️ The supine pose solved in three dimensions: legs turned out with the feet apart, the left arm alongside the body on the mattress (palm to the thigh), the right arm on the duvet with the forearm across the belly and the palm on the duvet. */
function supine(rig: Json, report: string[]): Offsets {
  const A = WOMAN.arms;
  let o: Offsets = {};
  for (const [side, sign] of [["l", 1], ["r", -1]] as const) {
    const legs = solve(rig, o, [{ bone: `foot-${side}`, key: "twistDeg", start: -15 * sign }], [{ bone: `toe-${side}`, point: [WOMAN.toe, 0, 0], target: [WOMAN.toeTip[0], WOMAN.toeTip[1], sign * WOMAN.toeTip[2]] }]);
    o = legs.offsets;
    report.push(`foot ${side}: ${JSON.stringify(o[`foot-${side}`])} error ${(legs.errorM * 1000).toFixed(1)} mm`);
  }
  const left = [-150, -100, -50, 0, 50, 100, 150].map((twist) => solve(rig, o, [
    { bone: "arm-l", key: "angleDeg", start: 0 },
    { bone: "arm-l", key: "abductionDeg", start: 8 },
    { bone: "arm-l", key: "twistDeg", start: twist },
    { bone: "forearm-l", key: "angleDeg", start: 8 },
    { bone: "forearm-l", key: "twistDeg", start: 0 },
    { bone: "hand-l", key: "angleDeg", start: 0 },
    { bone: "hand-l", key: "abductionDeg", start: 0 },
  ], [
    { bone: "forearm-l", point: [0, 0, 0], target: A.lElbow },
    { bone: "hand-l", point: [0, 0, 0], target: A.lWrist },
    { bone: "hand-l", point: [WOMAN.hand, 0, 0], target: A.lTip },
    { bone: "hand-l", axis: "v", sign: -1, target: A.lPalm },
  ])).filter((s) => s.offsets["forearm-l"]!.angleDeg! >= 0).sort((a, b) => a.errorM - b.errorM)[0]!;
  o = left.offsets;
  report.push(`arm l: ${JSON.stringify([o["arm-l"], o["forearm-l"], o["hand-l"]])} error ${(left.errorM * 1000).toFixed(1)} mm`);
  const right = solve(rig, o, [
    { bone: "arm-r", key: "angleDeg", start: 10 },
    { bone: "arm-r", key: "abductionDeg", start: -10 },
    { bone: "arm-r", key: "twistDeg", start: 0 },
    { bone: "forearm-r", key: "angleDeg", start: 60 },
    { bone: "forearm-r", key: "twistDeg", start: 0 },
    { bone: "hand-r", key: "angleDeg", start: 0 },
    { bone: "hand-r", key: "abductionDeg", start: 0 },
  ], [
    { bone: "forearm-r", point: [0, 0, 0], target: A.rElbow },
    { bone: "hand-r", point: [0, 0, 0], target: A.rWrist },
    { bone: "hand-r", point: [WOMAN.hand, 0, 0], target: A.rTip },
    { bone: "hand-r", axis: "v", sign: -1, target: A.rPalm },
  ]);
  o = right.offsets;
  report.push(`arm r: ${JSON.stringify([o["arm-r"], o["forearm-r"], o["hand-r"]])} error ${(right.errorM * 1000).toFixed(1)} mm`);
  return o;
}

/** 🛌️ The two poses on the left side, stated as joint angles of trunk and legs and as world targets of the arms: the pelvis rolled by 82 to 84 degrees with the hip joints stacked and lifted so the lower hip presses 1 cm into the mattress, the trunk bent sideways by 6 degrees so the lower shoulder lies on the mattress, the head level on the pillow, the thighs drawn forward on the mattress with the upper knee on the lower one, the upper arm along the flank with the hand on the hip over the duvet, the lower arm forward on the mattress with the hand on the pillow in front of the face. */
const LATERAL: Record<"side" | "curled", { base: Offsets; r: readonly [V3, V3, V3]; l: readonly [V3, V3, V3]; description: Json }> = {
  side: {
    base: {
      "hip-lift": { dy: 0.074 },
      pelvis: { angleDeg: 0, twistDeg: 82 },
      torso: { angleDeg: 0, twistDeg: 4, abductionDeg: -6 },
      neck: { angleDeg: 4, abductionDeg: 10 },
      head: { angleDeg: -6, abductionDeg: 4 },
      "thigh-l": { angleDeg: 35, abductionDeg: 6 },
      "shank-l": { angleDeg: -50, abductionDeg: -4 },
      "foot-l": { angleDeg: 16 },
      "toe-l": { angleDeg: 0 },
      "thigh-r": { angleDeg: 35, abductionDeg: 15 },
      "shank-r": { angleDeg: -50, abductionDeg: -4 },
      "foot-r": { angleDeg: 16 },
      "toe-r": { angleDeg: 0 },
      "arm-r": { angleDeg: 5.29, abductionDeg: -10.71, twistDeg: -45.09 },
      "forearm-r": { angleDeg: 8.57, twistDeg: 128.9 },
      "hand-r": { angleDeg: -11.92, abductionDeg: 8.71 },
      "arm-l": { angleDeg: 77.88, abductionDeg: 3.78, twistDeg: 15.83 },
      "forearm-l": { angleDeg: 93.87, twistDeg: 90.44 },
      "hand-l": { angleDeg: -22.51, abductionDeg: 27.45 },
    },
    r: [[0.885, 0.852, 0], [1.125, 0.862, 0.05], [1.29, 0.8, 0.11]],
    l: [[0.6, 0.505, 0.3], [0.37, 0.6, 0.33], [0.205, 0.6, 0.27]],
    description: {
      en: "On the left side: hips and shoulders stacked, the lower shoulder and hip pressed into the mattress, the head level on the pillow, the thighs drawn forward by 35 degrees with the upper knee and ankle on the lower ones and the feet out of the duvet's foot end; the upper arm lies along the flank with the hand on the hip over the duvet, the lower arm forward on the mattress with the hand on the pillow in front of the face.",
      de: "Auf der linken Seite: Hüften und Schultern übereinander, untere Schulter und Hüfte in die Matratze gedrückt, der Kopf waagrecht auf dem Kissen, die Oberschenkel um 35 Grad nach vorn gezogen mit dem oberen Knie und Knöchel auf den unteren, die Füße schauen aus dem Fußende der Decke; der obere Arm liegt entlang der Flanke mit der Hand auf der Hüfte über der Decke, der untere Arm nach vorn auf der Matratze mit der Hand auf dem Kissen vor dem Gesicht.",
    },
  },
  curled: {
    base: {
      "hip-lift": { dy: 0.074 },
      pelvis: { angleDeg: 0, twistDeg: 84 },
      torso: { angleDeg: -10, twistDeg: 3, abductionDeg: -6 },
      neck: { angleDeg: 2, abductionDeg: 10 },
      head: { angleDeg: -4, abductionDeg: 4 },
      "thigh-l": { angleDeg: 70, abductionDeg: 6 },
      "shank-l": { angleDeg: -105, abductionDeg: 1.5 },
      "foot-l": { angleDeg: 12 },
      "toe-l": { angleDeg: 0 },
      "thigh-r": { angleDeg: 70, abductionDeg: 15 },
      "shank-r": { angleDeg: -105, abductionDeg: 9.5 },
      "foot-r": { angleDeg: 12 },
      "toe-r": { angleDeg: 0 },
      "arm-r": { angleDeg: 7.41, abductionDeg: -9, twistDeg: 178.18 },
      "forearm-r": { angleDeg: -7.15, twistDeg: -98.17 },
      "hand-r": { angleDeg: -16.86, abductionDeg: 18.72 },
      "arm-l": { angleDeg: 87.3, abductionDeg: 5.69, twistDeg: 17.13 },
      "forearm-l": { angleDeg: 106.31, twistDeg: 93.82 },
      "hand-l": { angleDeg: -24.99, abductionDeg: 12.77 },
    },
    r: [[0.88, 0.855, 0.03], [1.125, 0.862, 0.05], [1.29, 0.8, 0.11]],
    l: [[0.6, 0.505, 0.39], [0.38, 0.6, 0.35], [0.215, 0.6, 0.3]],
    description: {
      en: "Curled up on the left side: the trunk bent forward by 10 degrees, the thighs drawn up by 70 degrees and the knees bent by 105 degrees with the upper leg on the lower one under the duvet; the upper arm lies along the flank with the hand on the hip over the duvet, the lower arm forward on the mattress with the hand on the pillow in front of the face.",
      de: "Eingerollt auf der linken Seite: der Rumpf um 10 Grad nach vorn gebeugt, die Oberschenkel um 70 Grad angezogen und die Knie um 105 Grad gebeugt, das obere Bein auf dem unteren unter der Decke; der obere Arm liegt entlang der Flanke mit der Hand auf der Hüfte über der Decke, der untere Arm nach vorn auf der Matratze mit der Hand auf dem Kissen vor dem Gesicht.",
    },
  },
};

/** 🛌️ A pose on the left side as stated; the arm angles were solved once to the world targets (elbow, wrist, fingertip; palms down) with `solve` and are stated as their result, the report gives what is left to the targets. */
function lateral(rig: Json, id: "side" | "curled", report: string[]): Offsets {
  const o: Offsets = JSON.parse(JSON.stringify(LATERAL[id].base));
  const f = framesOf(rig, o);
  for (const side of ["r", "l"] as const) {
    const [elbow, wrist, tip] = LATERAL[id][side];
    const got = [f[`forearm-${side}`]!.o, f[`hand-${side}`]!.o, at(f[`hand-${side}`]!, [WOMAN.hand, 0, 0])];
    const error = Math.max(...[elbow, wrist, tip].map((t, i) => Math.hypot(...sub(got[i]!, t))));
    report.push(`${id} arm ${side}: ${(error * 1000).toFixed(1)} mm from its targets`);
  }
  for (const b of ["head", "shank-l", "shank-r", "foot-l", "foot-r"]) report.push(`${id} ${b}: joint ${f[b]!.o.map((x) => x.toFixed(3)).join(", ")}`);
  return o;
}

/* ---------------------------------------------------------------- document */

/** 📄️ The catalog document with every root fix of this round applied. */
export function rooted(report: string[] = []): Json {
  const document = JSON.parse(readFileSync(ASSET, "utf8")) as Json;
  const asset = document.asset as Json;
  for (const bone of asset.rig.bones as Json[]) {
    const fix = BONES[bone.id];
    if (fix !== undefined) for (const [k, v] of Object.entries(fix)) {
      if (k.startsWith("mass.")) bone.mass[k.slice(5)] = v;
      else bone[k] = v;
    }
    if (TITLES[bone.id] !== undefined) bone.title = TITLES[bone.id];
  }
  const rig = asset.rig;
  const solved = join(import.meta.dir, "r3-sleeping-person-supine.json");
  const pose: Offsets = existsSync(solved) ? JSON.parse(readFileSync(solved, "utf8")) : supine(rig, report);
  if (!existsSync(solved)) writeFileSync(solved, JSON.stringify(pose, null, 1));
  const poses = asset.poses as Json[];
  const supinePose = poses.find((p) => p.id === "supine")!;
  supinePose.bones = { pelvis: { angleDeg: 0 }, torso: { angleDeg: 0 }, neck: { angleDeg: 0 }, head: { angleDeg: 0 }, ...pose };
  supinePose.description = {
    en: "Flat on the back, the head on the pillow with the neck slightly flexed, the legs stretched and turned out under the duvet with the feet out of its folded foot end; the left arm lies alongside the body under the duvet, the right arm on the duvet with the hand on the belly.",
    de: "Flach auf dem Rücken, der Kopf auf dem Kissen und der Hals leicht gebeugt, die Beine gestreckt und nach außen gedreht unter der Decke, die Füße schauen aus dem zurückgeschlagenen Fußende; der linke Arm liegt neben dem Körper unter der Decke, der rechte Arm auf der Decke mit der Hand auf dem Bauch.",
  };
  for (const id of ["side", "curled"] as const) {
    const p = poses.find((q) => q.id === id)!;
    p.bones = lateral(rig, id, report);
    p.description = LATERAL[id].description;
  }
  const frames = framesOf(rig, supinePose.bones);
  for (const a of asset.anchors as Json[]) {
    const spec = ANCHORS[a.id];
    if (spec === undefined) continue;
    const q = local(frames[spec.bone]!, spec.world);
    a.bone = spec.bone;
    a.x = r4(q[0]);
    a.y = r4(q[1]);
    report.push(`anchor ${a.id}: ${spec.bone} (${a.x}, ${a.y}), lateral ${(q[2] * 1000).toFixed(0)} mm from the bone`);
  }
  if (!(asset.materials as Json[]).some((m) => m.id === "bone")) asset.materials.push({ id: "bone", title: { en: "Bone", de: "Knochen" }, emissivity: 0.9, solarAbsorptivity: 0.5, densityKgM3: 1900, source: "authored-estimate" });
  const thermal = asset.thermal as Json;
  const regions = thermal.regions as Json[];
  const byId = (id: string): Json => regions.find((r) => r.id === id)!;
  byId("hand-skin").title = { en: "Hands", de: "Hände" };
  byId("feet-skin").title = { en: "Bare feet and ankles", de: "Nackte Füße und Knöchel" };
  byId("top").title = { en: "Duvet over the pyjama top, trunk and left arm", de: "Bettdecke über Schlafanzugoberteil, Rumpf und linkem Arm" };
  byId("hair").layers[0].title = { en: "Loose scalp hair", de: "Offenes Kopfhaar" };
  byId("pillow").areaM2 = BED.pillow.areaM2;
  if (!regions.some((r) => r.id === "duvet-edge")) regions.splice(regions.findIndex((r) => r.id === "pillow"), 0, {});
  const edge = regions.findIndex((r) => r.id === "duvet-edge" || Object.keys(r).length === 0);
  regions[edge] = {
    id: "duvet-edge",
    title: { en: "Duvet beyond the body: flat on the mattress, side drapes and folded foot end", de: "Bettdecke neben dem Körper: flach auf der Matratze, seitlicher Behang und zurückgeschlagenes Fußende" },
    kind: "surface",
    model: "passive",
    material: "duvet-cover",
    areaM2: BED.duvet.edgeAreaM2,
    characteristicLengthM: 0.3,
    geometry: "horizontal-plate-up",
    skyViewFactor: 0.5,
    layers: [],
    heatCapacityJM2K: 1200,
    source: "authored-estimate",
    reference: "lin-deng-2008",
  };
  for (const p of poses) p.thermal.regions["duvet-edge"] = { exposedFraction: 0.75, sunlitFraction: 0, contact: { fraction: 0.25, tempC: 24 } };
  asset.thermal.assumptions[2] = {
    en: "Light sleepwear (long-sleeved pyjama top 0.12 clo, shorts 0.06 clo) under a 135 x 200 cm duvet of about 1.6 clo including the enclosed air layer (bedding insulation 1.4 to 3 clo; Lin and Deng 2008, authored estimate). The regions top and bottom are the duvet where it lies on the body; its flat parts on the mattress, the side drapes and the folded foot end are the passive region duvet-edge. The mattress top is the contact surface at 29 degrees C.",
    de: "Leichte Schlafkleidung (langärmeliges Oberteil 0,12 clo, Shorts 0,06 clo) unter einer Bettdecke 135 x 200 cm von etwa 1,6 clo einschließlich der eingeschlossenen Luftschicht (Bettzeugdämmung 1,4 bis 3 clo; Lin und Deng 2008, Schätzwert). Die Bereiche top und bottom sind die Decke, wo sie auf dem Körper liegt; ihre flachen Teile auf der Matratze, der seitliche Behang und das zurückgeschlagene Fußende bilden den passiven Bereich duvet-edge. Die Matratzenoberseite ist die Kontaktfläche mit 29 Grad C.",
  };
  asset.summary = {
    en: "A woman asleep on her back under a 135 x 200 cm duvet, the head on a 40 x 80 cm pillow, the right hand on the duvet over the belly and the bare feet out of the folded-back foot end: lying on the back, on the side and curled up, a looping breathing clip and a turn-over; at 0.7 met and made to lie on the catalog single-bed.",
    de: "Eine Frau schläft auf dem Rücken unter einer Bettdecke 135 x 200 cm, der Kopf auf einem Kissen 40 x 80 cm, die rechte Hand auf der Decke über dem Bauch und die nackten Füße schauen aus dem zurückgeschlagenen Fußende: in Rücken-, Seiten- und eingerollter Lage, mit einem Atemclip als Schleife und einem Umdrehen; bei 0,7 met und gemacht, um auf dem Katalog-Einzelbett zu liegen.",
  };
  return document;
}

if (import.meta.main) {
  const report: string[] = [];
  rooted(report);
  console.log(report.join("\n"));
}
