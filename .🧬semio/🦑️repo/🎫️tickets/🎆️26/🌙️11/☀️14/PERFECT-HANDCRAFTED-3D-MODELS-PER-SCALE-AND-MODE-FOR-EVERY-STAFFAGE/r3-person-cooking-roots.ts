/** 🦴 Root fixes of `humans/person-cooking` (round 3), stated as absolute values so the script can run any number of times on the original catalog document: rig landmarks after Winter 2009 and de Leva 1996 for a woman of 1.69 m in canvas sneakers, the kitchen unit moved to a working distance, the pot bone on its hob zone and the spoon bone at the grip, anchors on the modelled body, standing legs re-solved by two-link geometry, the right arm with the spoon placed by a small optimiser on the product's own chain (`--solve`, cached in `arms.json`) for the poses and for every key of the stirring loop, texts, dimensions and, when present, the redrawn 2D shapes and decisions. Writes the candidate document `🗑️generated/r3-person-cooking/🔣️.json`; `--commit` replaces the owned members in the catalog file (re-read right before, member by member, the rest of the file untouched).
 *
 * Usage: `bun r3-person-cooking-roots.ts [--solve] [--with-volumes] [--commit]`
 * @see ./r3-humans-person-cooking.md */
import { fileURLToPath } from "node:url";
import { existsSync, readFileSync, writeFileSync, mkdirSync } from "node:fs";
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { type Frame } from "./r3-lib.ts";
import { dump } from "./r3-person-cooking-fmt.ts";
import { CANDIDATE, OUT, SOURCE } from "./r3-person-cooking-io.ts";
import { POT, RIG, SPOON, UNIT } from "./r3-person-cooking-data.ts";

type Json = any;
type V3 = [number, number, number];
const CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/person-cooking/🔣️.json";
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
  return [p[0], p[1] + (RIG.hip - OLD_HIP)];
};

/* ------------------------------------------------------------------ materials: the skeleton of the structure list, the water in the pot, the steel of handles and feet */
const material = (m: Json): void => { if (!asset.materials.some((q: Json) => q.id === m.id)) asset.materials.push(m); };
material({ id: "bone", title: { en: "Bone", de: "Knochen" }, emissivity: 0.9, solarAbsorptivity: 0.5, densityKgM3: 1900, source: "authored-estimate" });
material({ id: "water", title: { en: "Water in the pot", de: "Wasser im Topf" }, emissivity: 0.96, solarAbsorptivity: 0.3, densityKgM3: 998, specificHeatJKgK: 4180, source: "authored-estimate" });

/* ------------------------------------------------------------------ rig */
const keepCom = ["torso", "head"].map((id) => [id, lifted(id)] as const);
Object.assign(bone("pelvis"), { y: r5(RIG.hip) });
Object.assign(bone("torso"), { x: RIG.pelvisLen, y: RIG.lumboBack, angleDeg: r4(RIG.torsoRel), lengthM: r5(RIG.torsoLen), title: { en: "Torso with hooded sweatshirt", de: "Rumpf mit Kapuzenpullover" } });
Object.assign(bone("neck"), { x: r5(RIG.torsoLen), y: 0, angleDeg: r4(RIG.neckRel), lengthM: r5(RIG.neckLen) });
bone("neck").mass.comX = r4(RIG.neckLen / 2);
Object.assign(bone("head"), { x: r5(RIG.neckLen), y: 0, angleDeg: r4(RIG.headRel), lengthM: RIG.headLen });
const fresh = rest2(asset.rig, RIG.hip);
const shoulder = toLocal(fresh.torso!, RIG.shoulder);
for (const s of ["l", "r"]) {
  Object.assign(bone(`arm-${s}`), { x: r5(shoulder[0]), y: r5(shoulder[1]), angleDeg: r4(180 - RIG.torsoRel), z: s === "l" ? RIG.shoulderZ : -RIG.shoulderZ });
  Object.assign(bone(`thigh-${s}`), { z: s === "l" ? RIG.hipZ : -RIG.hipZ });
  Object.assign(bone(`foot-${s}`), { angleDeg: RIG.footRel, lengthM: r5(RIG.footLen), title: s === "l" ? { en: "Left foot in the sneaker", de: "Linker Fuß im Sneaker" } : { en: "Right foot in the sneaker", de: "Rechter Fuß im Sneaker" } });
  Object.assign(bone(`foot-${s}`).mass, { comX: 0.05, comY: -0.035 });
  Object.assign(bone(`toe-${s}`), { x: r5(RIG.footLen), y: r5(-RIG.ankle), angleDeg: 0, lengthM: RIG.toeLen });
  Object.assign(bone(`toe-${s}`).mass, { comX: r4(RIG.toeLen * 0.45), comY: 0.022 });
}
const fresh2 = rest2(asset.rig, RIG.hip);
for (const [id, p] of keepCom) {
  const l = toLocal(fresh2[id]!, p);
  bone(id).mass.comX = r4(l[0]);
  bone(id).mass.comY = r4(l[1]);
}
Object.assign(bone("counter-lift"), { x: r5(-RIG.hip), y: -UNIT.ahead, title: { en: "Vertical float of the kitchen unit on the floor", de: "Senkrechte Führung des Küchenschranks am Boden" } });
Object.assign(bone("counter"), { lengthM: UNIT.depth, title: { en: "Hob base unit with worktop and hob", de: "Kochfeld-Unterschrank mit Arbeitsplatte und Kochfeld" } });
Object.assign(bone("counter").mass, { comX: 0.31, comY: 0.47 });
Object.assign(bone("pot"), { x: POT.x, y: POT.y, z: POT.z, angleDeg: 0, lengthM: POT.r, title: { en: "Pot of 22 cm with 2.5 litres of water", de: "Topf von 22 cm mit 2,5 Litern Wasser" } });
Object.assign(bone("pot").mass, { comX: 0, comY: 0.045 });
Object.assign(bone("spoon"), { x: SPOON.gripX, y: SPOON.gripY, z: SPOON.gripZ, angleDeg: SPOON.rest, lengthM: SPOON.len, joint: { type: "revolute", minDeg: -60, maxDeg: 150 }, title: { en: "Wooden cooking spoon", de: "Holzkochlöffel" } });
Object.assign(bone("spoon").mass, { comX: 0.075, comY: 0 });

/* ------------------------------------------------------------------ anchors (positions on the modelled body, see the 1:50 model) */
for (const s of ["l", "r"]) {
  Object.assign(anchor(`heel-${s}`), { x: -0.04, y: r5(-RIG.ankle), footprintLengthM: 0.07, footprintWidthM: 0.062 });
  Object.assign(anchor(`ball-${s}`), { x: r5(RIG.footLen), y: r5(-RIG.ankle), footprintLengthM: 0.1, footprintWidthM: 0.09 });
}
export const HEAD_ANCHORS: Record<string, [number, number]> = { mouth: [-0.0215, -0.1], nostrils: [-0.0005, -0.107], crown: [0.156, 0.012], ear: [0.03, -0.002] };
for (const [id, [x, y]] of Object.entries(HEAD_ANCHORS)) Object.assign(anchor(id), { x, y });
Object.assign(anchor("ear"), { z: -0.071 });
Object.assign(anchor("counter-contact"), { x: 0.335, y: 0, footprintLengthM: 0.53, footprintWidthM: 0.6, title: { en: "Kitchen unit on the floor", de: "Küchenschrank am Boden" } });
Object.assign(anchor("pot-mouth"), { x: 0, y: POT.h });
Object.assign(anchor("grip-r"), { x: SPOON.gripX, y: SPOON.gripY, z: SPOON.gripZ });
Object.assign(anchor("spoon-cog"), { x: 0.075, y: 0 });

/* ------------------------------------------------------------------ standing legs */
const chord = (k: number): number => Math.sqrt(RIG.thigh ** 2 + RIG.shank ** 2 + 2 * RIG.thigh * RIG.shank * Math.cos(rad(k)));
/** 🦵 Thigh, shank and foot offsets that put the ankle `dx` ahead of the hip with the knee bent `knee` degrees and the sole level. */
function leg(dx: number, knee: number): Json {
  const d = chord(knee);
  const h = Math.sqrt(d * d - dx * dx);
  const alpha = Math.acos(cl((RIG.thigh ** 2 + d * d - RIG.shank ** 2) / (2 * RIG.thigh * d), -1, 1));
  const thighW = deg(Math.atan2(-h, dx) + alpha);
  const t = thighW + 90;
  const shankW = thighW - knee;
  return { thigh: r2(t), shank: r2(-knee), foot: r2(-(shankW + 90)), drop: RIG.thigh + RIG.shank - h };
}
type Stand = { torso: number; neck: number; head: number; dx: number; knee: number; neckTwist?: number; headTwist?: number };
export const STAND: Record<string, Stand> = {
  stirring: { torso: -7, neck: -10, head: -20, dx: -0.025, knee: 4 },
  tasting: { torso: -2, neck: -4, head: -6, dx: -0.025, knee: 4 },
  "standing-by": { torso: 0, neck: -4, head: -12, dx: -0.025, knee: 4 },
};
const LEFT_ARM = { "arm-l": { angleDeg: 4, abductionDeg: 5 }, "forearm-l": { angleDeg: 16 }, "hand-l": { angleDeg: 4 } };
for (const [id, s] of Object.entries(STAND)) {
  const p = pose(id);
  const l = leg(s.dx, s.knee);
  const legs: Json = {};
  for (const side of ["l", "r"]) Object.assign(legs, { [`thigh-${side}`]: { angleDeg: l.thigh }, [`shank-${side}`]: { angleDeg: l.shank }, [`foot-${side}`]: { angleDeg: l.foot }, [`toe-${side}`]: { angleDeg: 0 } });
  p.bones = { ...legs, torso: { angleDeg: s.torso }, neck: { angleDeg: s.neck, ...(s.neckTwist ? { twistDeg: s.neckTwist } : {}) }, head: { angleDeg: s.head, ...(s.headTwist ? { twistDeg: s.headTwist } : {}) }, ...JSON.parse(JSON.stringify(LEFT_ARM)), "arm-r": { angleDeg: 2, abductionDeg: -6 }, "forearm-r": { angleDeg: 14 }, "hand-r": { angleDeg: 0 }, spoon: { angleDeg: 8 }, "counter-lift": { dx: r5(l.drop) } };
}

/* ------------------------------------------------------------------ the right arm with the spoon: optimiser on the product's chain */
const light = (): Json => ({ ...asset, representations: Object.fromEntries(Object.entries(asset.representations).map(([k, v]: [string, Json]) => [k, { ...v, geometry: { elevation: [], front: [], plan: [] } }])) });
export function frames(a: Json, poseId: string): Record<string, Frame> {
  const s = evaluate(a as never, { kind: "pose", pose: poseId } as never) as Json;
  const v = (p: Json): V3 => [p.x, p.y, p.z ?? 0];
  const out: Record<string, Frame> = {};
  for (const b of a.rig.bones) out[b.id] = { o: v(s.bones[b.id].frame.o), u: v(s.bones[b.id].frame.u), v: v(s.bones[b.id].frame.v), w: v(s.bones[b.id].frame.w) };
  return out;
}
export const at = (f: Frame, p: readonly number[]): V3 => [0, 1, 2].map((k) => f.o[k]! + p[0]! * f.u[k]! + p[1]! * f.v[k]! + (p[2] ?? 0) * f.w[k]!) as V3;
const add = (a: V3, b: V3): V3 => [a[0] + b[0], a[1] + b[1], a[2] + b[2]];
const sub = (a: V3, b: V3): V3 => [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
const mul = (a: V3, k: number): V3 => [a[0] * k, a[1] * k, a[2] * k];
const dotv = (a: readonly number[], b: readonly number[]): number => a[0]! * b[0]! + a[1]! * b[1]! + a[2]! * b[2]!;
const d2 = (a: V3, b: V3): number => dotv(sub(a, b), sub(a, b));
const unit = (a: V3): V3 => { const l = Math.hypot(...a); return [a[0] / l, a[1] / l, a[2] / l]; };

type Param = [string, string, number, number, number];
/** 🎯 Nelder-Mead over the named bone offsets of `bones` (a copy of the pose `base` with these offsets is evaluated), minimising `cost(frames)`; the result is written into `bones`. */
function solve(label: string, base: string, bones: Json, params: Param[], cost: (f: Record<string, Frame>) => number, report?: (f: Record<string, Frame>) => string): void {
  const lt = light();
  const lp = { ...JSON.parse(JSON.stringify(pose(base))), id: "r3-solve", bones: JSON.parse(JSON.stringify({ ...pose(base).bones, ...bones })) };
  lt.poses = [...lt.poses, lp];
  const set = (x: number[]): void => params.forEach(([b, key, , lo, hi], i) => { lp.bones[b] = { ...(lp.bones[b] ?? {}), [key]: cl(x[i]!, lo, hi) }; });
  const f = (x: number[]): number => { set(x); return cost(frames(lt, "r3-solve")) + x.reduce((s, xi, i) => s + (xi < params[i]![3] || xi > params[i]![4] ? 1 : 0), 0); };
  const n = params.length;
  let simplex = [params.map((q) => q[2])];
  for (let i = 0; i < n; i++) simplex.push(simplex[0]!.map((x, j) => (j === i ? x + 12 : x)));
  let values = simplex.map(f);
  for (let it = 0; it < 1800; it++) {
    const order = values.map((v, i) => [v, i] as const).sort((a, b) => a[0] - b[0]).map((q) => q[1]);
    simplex = order.map((i) => simplex[i]!);
    values = order.map((i) => values[i]!);
    if (values[n]! - values[0]! < 1e-12 && it > 60) break;
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
  if (report) console.log(`[DEBUG]   ${report(frames(lt, "r3-solve"))}`);
  for (const [b, key] of params) bones[b] = { ...(bones[b] ?? {}), [key]: r2(lp.bones[b][key]) };
  console.log(`[DEBUG] ${label}: cost ${best.toExponential(2)} ${params.map(([b, k]) => `${b}.${k}=${bones[b][k]}`).join(" ")}`);
}
const RIGHT = ["arm-r", "forearm-r", "hand-r", "spoon"];
const params = (b: Json): Param[] => [
  ["arm-r", "angleDeg", b["arm-r"]?.angleDeg ?? 35, -60, 170], ["arm-r", "abductionDeg", b["arm-r"]?.abductionDeg ?? -12, -90, 90], ["arm-r", "twistDeg", b["arm-r"]?.twistDeg ?? 0, -90, 90],
  ["forearm-r", "angleDeg", b["forearm-r"]?.angleDeg ?? 60, 0, 150], ["forearm-r", "twistDeg", b["forearm-r"]?.twistDeg ?? 0, -90, 90], ["spoon", "angleDeg", b.spoon?.angleDeg ?? 0, -45, 150],
];
type Target = { hand: number; wElbow?: number; tip: (f: Record<string, Frame>) => V3; grip: (f: Record<string, Frame>) => V3; elbow: V3; hollow?: V3; palm?: V3 };
const costOf = (t: Target) => (f: Record<string, Frame>): number => {
  const s = f.spoon!;
  let c = d2(at(s, [SPOON.bowl, 0, 0]), t.tip(f)) + d2(s.o, t.grip(f));
  c += (t.wElbow ?? 0.006) * d2(f["forearm-r"]!.o, add(f["arm-r"]!.o, t.elbow));
  if (t.hollow) c += 0.002 * (1 - dotv(s.v, unit(t.hollow)));
  if (t.palm) c += 0.002 * (1 - dotv(f["hand-r"]!.w, unit(t.palm)));
  return c;
};
const reportOf = (t: Target) => (f: Record<string, Frame>): string => `tip off ${(Math.sqrt(d2(at(f.spoon!, [SPOON.bowl, 0, 0]), t.tip(f))) * 1000).toFixed(1)} mm, grip off ${(Math.sqrt(d2(f.spoon!.o, t.grip(f))) * 1000).toFixed(1)} mm, elbow ${f["forearm-r"]!.o.map((x) => x.toFixed(3)).join(" ")}, palm ${f["hand-r"]!.w.map((x) => x.toFixed(2)).join(" ")}`;
/** 🥄 The stirring stroke: the bowl runs on a circle of 50 mm radius 35 mm above the pot base, the handle pivots about a point 0.30 m above the base, slightly towards the cook. */
export function stirTarget(phaseDeg: number, radius = 0.05): Target {
  const tip = (f: Record<string, Frame>): V3 => at(f.pot!, [radius * Math.cos(rad(phaseDeg)), 0.035, radius * Math.sin(rad(phaseDeg))]);
  const pivot = (f: Record<string, Frame>): V3 => at(f.pot!, [-0.05, 0.3, -0.02]);
  return { tip, grip: (f) => { const t = tip(f); return add(t, mul(unit(sub(pivot(f), t)), SPOON.bowl)); }, elbow: [0.1, -0.27, -0.08], palm: [0.1, -0.3, 1], hand: -20 };
}
/** 👄 Tasting: the bowl of the spoon just in front of the lips, hollow up, the handle running down, ahead and to the right into the hand. */
export const TASTE: Target = (() => {
  const mouth = (f: Record<string, Frame>): V3 => at(f.head!, [HEAD_ANCHORS.mouth![0], HEAD_ANCHORS.mouth![1], 0]);
  const dir = unit([-0.5, 0.42, 0.76]);
  const tip = (f: Record<string, Frame>): V3 => add(mouth(f), [0.03, -0.012, 0]);
  return { tip, grip: (f) => sub(tip(f), mul(dir, SPOON.bowl)), elbow: [0.12, -0.26, -0.1], wElbow: 0.03, hollow: [0.25, 1, 0.1], hand: 8 };
})();
const STIR_KEYS = 8;
if (process.argv.includes("--solve")) {
  const cache: Json = { poses: {}, keys: [] };
  const run = (label: string, base: string, start: Json, t: Target): Json => {
    const b: Json = { ...JSON.parse(JSON.stringify(start)), "hand-r": { angleDeg: t.hand } };
    for (let round = 0; round < 3; round++) solve(label, base, b, params(b), costOf(t), reportOf(t));
    return Object.fromEntries(RIGHT.map((id) => [id, b[id]]));
  };
  cache.poses.stirring = run("stirring", "stirring", {}, stirTarget(0, 0));
  cache.poses.tasting = run("tasting", "tasting", { "arm-r": { angleDeg: 35, abductionDeg: -20, twistDeg: 0 }, "forearm-r": { angleDeg: 125, twistDeg: 0 }, spoon: { angleDeg: 100 } }, TASTE);
  let prev = cache.poses.stirring;
  for (let k = 0; k < STIR_KEYS; k++) { prev = run(`stir key ${k}`, "stirring", prev, stirTarget((360 * k) / STIR_KEYS)); cache.keys.push(prev); }
  writeFileSync(ARMS, JSON.stringify(cache, null, 1));
}
if (existsSync(ARMS)) {
  const cache = JSON.parse(readFileSync(ARMS, "utf8")) as Json;
  for (const [id, bones] of Object.entries(cache.poses)) Object.assign(pose(id).bones, bones);
  const stir = clip("stirring");
  const sway = (k: number): number => 0.8 * Math.cos((2 * Math.PI * k) / STIR_KEYS);
  const keyAt = (k: number): Json => ({ t: r4((stir.durationS * k) / STIR_KEYS), pose: "stirring", ease: "smooth", bones: { torso: { angleDeg: r2(STAND.stirring!.torso + sway(k)) }, head: { angleDeg: r2(STAND.stirring!.head + 1.2 * sway(k)) }, ...JSON.parse(JSON.stringify(cache.keys[k % STIR_KEYS])) } });
  stir.keys = Array.from({ length: STIR_KEYS + 1 }, (_, k) => keyAt(k));
}

/* ------------------------------------------------------------------ texts, provenance, dimensions */
const TEXTS = fileURLToPath(new URL("./r3-person-cooking-texts.json", import.meta.url));
const texts: Json = existsSync(TEXTS) ? JSON.parse(readFileSync(TEXTS, "utf8")) : {};
if (texts.summary) asset.summary = texts.summary;
for (const [id, d] of Object.entries(texts.poses ?? {})) pose(id).description = d;
for (const [id, d] of Object.entries(texts.clips ?? {})) clip(id).description = d;
if (texts.assumptions) asset.structure.assumptions = texts.assumptions;
if (texts.notes) asset.provenance.notes = texts.notes;
const refs = asset.provenance.references as Json[];
for (const r of texts.references ?? []) if (!refs.some((q) => q.id === r.id)) refs.push(r);
for (const [id, t] of Object.entries(texts.regions ?? {})) asset.thermal.regions.find((r: Json) => r.id === id).title = t;
const DIMS = fileURLToPath(new URL("./r3-person-cooking-dimensions.json", import.meta.url));
if (existsSync(DIMS)) Object.assign(asset.dimensions, JSON.parse(readFileSync(DIMS, "utf8")));

const SHAPES = `${OUT}/shapes.json`;
if (existsSync(SHAPES)) {
  const g = JSON.parse(readFileSync(SHAPES, "utf8")) as Json;
  for (const [scale, views] of Object.entries(g)) Object.assign(asset.representations[scale].geometry, views);
}
for (const [scale, d] of Object.entries(texts.representations ?? {})) Object.assign(asset.representations[scale].decisions, d);
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
  const re = /("id": "person-cooking",\s*"category": "humans",\s*"title": \{[^{}]*\},\s*"heightM": )([0-9.]+)(,\s*"widthM": )([0-9.]+)/;
  if (!re.test(fresh3)) throw new Error("manifest entry not found");
  const next = fresh3.replace(re, `$1${asset.dimensions.heightM}$3${asset.dimensions.widthM}`);
  if (next !== fresh3) writeFileSync(MANIFEST, next);
  console.log("committed", CATALOG);
}
