/** 🛏️ Root fixes of `humans/sleeping-person` (round 3), stated as final values and solved again on every run (idempotent): the rig of a 1.70 m woman lying supine on a soft mattress (C7, atlanto-occipital joint and vertex at their anthropometric distances, hip joints 0.174 m apart, shoulder joints 0.316 m apart, thigh, shank and foot lengths of the stature, joints at their heights above the mattress), a 40 x 80 cm pillow across the bed and a 135 x 200 cm duvet whose foot end is folded back over the shins, the supine pose solved in three dimensions with the product's own chain (left arm alongside the body under the duvet, right arm on the duvet with the hand on the belly, feet falling outward), the side poses lifted so the rolled trunk rests on the mattress, the anchors on the new surfaces, the passive region of the duvet beyond the body and the texts that named the old construction.
 * Usage: `bun r3-sleeping-person-roots.ts [--print]` (as module: `rooted()` gives the catalog document with these fixes).
 * @see ./r3-sleeping-person-ref.ts */
import { readFileSync } from "node:fs";

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
  const pose = supine(rig, report);
  const poses = asset.poses as Json[];
  const supinePose = poses.find((p) => p.id === "supine")!;
  supinePose.bones = { pelvis: { angleDeg: 0 }, torso: { angleDeg: 0 }, neck: { angleDeg: 0 }, head: { angleDeg: 0 }, ...pose };
  supinePose.description = {
    en: "Flat on the back, the head on the pillow with the neck slightly flexed, the legs stretched and turned out under the duvet with the feet out of its folded foot end; the left arm lies alongside the body under the duvet, the right arm on the duvet with the hand on the belly.",
    de: "Flach auf dem Rücken, der Kopf auf dem Kissen und der Hals leicht gebeugt, die Beine gestreckt und nach außen gedreht unter der Decke, die Füße schauen aus dem zurückgeschlagenen Fußende; der linke Arm liegt neben dem Körper unter der Decke, der rechte Arm auf der Decke mit der Hand auf dem Bauch.",
  };
  for (const [id, thigh, foot, lift] of [["side", 33, 16, 0.05], ["curled", 59, 12, 0.065]] as const) {
    const p = poses.find((q) => q.id === id)!;
    p.bones["hip-lift"] = { dy: lift };
    for (const side of ["l", "r"]) {
      p.bones[`thigh-${side}`].angleDeg = thigh;
      p.bones[`foot-${side}`].angleDeg = foot;
      p.bones[`arm-${side}`] = { ...p.bones[`arm-${side}`], abductionDeg: side === "l" ? 6 : -6 };
    }
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
