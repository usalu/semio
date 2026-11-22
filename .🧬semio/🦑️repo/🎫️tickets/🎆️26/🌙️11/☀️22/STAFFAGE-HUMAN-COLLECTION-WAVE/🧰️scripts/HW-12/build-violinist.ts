/** 🎻️ Builds `standing-violinist` from the archive's `standing-adult` (same 1.69 m, 59 kg body and rig): a street violinist playing a full-size violin, with handcrafted violin and bow at four scales, poses solved through `evaluate` and a looping bowing clip.
 *  bun build-violinist.ts
 * @see ./violin.ts — the instrument
 * @see ./hold.ts — the playing hold */
import { HEAD, holdReport, solveBowArm, solveLeftHand, toRest, violinFrame } from "./hold.ts";
import { addPrisms, fistHands, fitDimensions, revolveLimbs, at, evalOffsets, frameOf, len, loadDoc, massOf, mul, placeBone, rd, restState, saveDoc, SCALES, sub, T, unit, add, type Any } from "./lib.ts";
import { bowPrisms, violinPrisms } from "./violin.ts";
import { VIOLINIST_DECISIONS, violinistTexts } from "./texts-violinist.ts";
import { applyDecisions } from "./decisions.ts";

const doc = await loadDoc("standing-adult");
const a = doc.asset;
a.id = "standing-violinist";
const relaxed: Record<string, Any> = structuredClone(a.poses.find((p: Any) => p.id === "relaxed").bones);
const fingersOf = (p: string) => Object.fromEntries(Object.entries(a.poses.find((q: Any) => q.id === p).bones).filter(([b]) => /^(thumb|index|middle|ring|little)-/.test(b)));
const crossedFingers = fingersOf("arms-crossed");

a.materials.push(
  { id: "violin-wood", title: T("Varnished spruce and maple", "Lackiertes Fichten- und Ahornholz"), emissivity: 0.9, solarAbsorptivity: 0.7, specificHeatJKgK: 1700, densityKgM3: 500, source: "authored-estimate", reference: "ashrae-2021" },
  { id: "ebony", title: T("Ebony fittings", "Ebenholzgarnitur"), emissivity: 0.9, solarAbsorptivity: 0.9, specificHeatJKgK: 1600, densityKgM3: 1100, source: "authored-estimate" },
  { id: "pernambuco-horsehair", title: T("Pernambuco bow stick with horsehair", "Fernambukbogen mit Rosshaar"), emissivity: 0.9, solarAbsorptivity: 0.7, specificHeatJKgK: 1600, densityKgM3: 1000, source: "authored-estimate" },
);
a.rig.bones.push(
  { id: "violin", parent: "torso", title: T("Violin held between chin and collarbone", "Geige zwischen Kinn und Schlüsselbein gehalten"), x: 0, y: 0, z: 0, angleDeg: 0, lengthM: 0.593, joint: { type: "fixed" }, mass: { kg: 0.46, comX: 0.19, comY: 0, distribution: "point", source: "authored-estimate", reference: "violin-dimensions" } },
  { id: "bow", parent: "hand-r", title: T("Bow held at the frog", "Am Frosch gehaltener Bogen"), x: 0, y: 0, z: 0, angleDeg: 0, lengthM: 0.75, joint: { type: "revolute", minDeg: -30, maxDeg: 30 }, mass: { kg: 0.06, comX: 0.25, comY: 0, distribution: "point", source: "literature", reference: "bow-dimensions" } },
);

const holdBase = { ...relaxed, ...HEAD };
const held = evalOffsets(a, holdBase);
const Vw = violinFrame(held);
const placed = placeBone(a, "violin", toRest(a, held, "torso", Vw));
const restHand = frameOf(restState(a), "hand-r");
const ub = unit(add(mul(restHand.u, 0.42), mul(restHand.v, 0.91)));
const grip = at(restHand, 0.08, 0.004, 0.026);
const placedBow = placeBone(a, "bow", { o: sub(grip, mul(ub, 0.05)), u: ub, v: restHand.w });
console.log(`[DEBUG] violin placed ${placed.toExponential(2)}, bow placed ${placedBow.toExponential(2)}`);

const V = frameOf(evalOffsets(a, holdBase), "violin");
console.log("[DEBUG] violin o", V.o.map((x) => rd(x, 3)), "scroll", at(V, 0.59, 0.02, 0).map((x) => rd(x, 3)), "err", rd(len(sub(V.o, Vw.o)), 5));
const left = solveLeftHand(a, holdBase, V);
console.log("[DEBUG] left hand residual", left.residual.toExponential(2), JSON.stringify({ a: left.bones["arm-l"], f: left.bones["forearm-l"], h: left.bones["hand-l"] }));
const leftFingers = {
  "index-proximal-l": { angleDeg: 4, abductionDeg: 0 }, "index-middle-l": { angleDeg: 70 }, "index-distal-l": { angleDeg: 40 },
  "middle-proximal-l": { angleDeg: 6 }, "middle-middle-l": { angleDeg: 80 }, "middle-distal-l": { angleDeg: 35 },
  "ring-proximal-l": { angleDeg: 8 }, "ring-middle-l": { angleDeg: 85 }, "ring-distal-l": { angleDeg: 35 },
  "little-proximal-l": { angleDeg: 10 }, "little-middle-l": { angleDeg: 80 }, "little-distal-l": { angleDeg: 30 },
  "thumb-metacarpal-l": { angleDeg: 0, abductionDeg: -10, twistDeg: 60 }, "thumb-proximal-l": { angleDeg: 20 }, "thumb-distal-l": { angleDeg: 15 },
};
const rightFingers = Object.fromEntries(Object.entries(crossedFingers).filter(([b]) => b.endsWith("-r")));
const playBase = { ...left.bones, ...leftFingers, ...rightFingers };

type Stroke = { s: number; string: "g" | "d" | "a" | "e" };
const bowFor = (k: Stroke, start?: number[]) => {
  const r = solveBowArm(a, playBase, V, k.s, k.string, start);
  console.log(`[DEBUG] bow s=${k.s} ${k.string}: ${holdReport(a, r.bones, V, k.s, k.string)}`, JSON.stringify({ a: r.bones["arm-r"], f: r.bones["forearm-r"], h: r.bones["hand-r"] }));
  return r.bones;
};
const pick = (bones: Record<string, Any>) => ({ "arm-r": bones["arm-r"], "forearm-r": bones["forearm-r"], "hand-r": bones["hand-r"], bow: bones.bow });
const mid = bowFor({ s: 0.38, string: "a" });
const startOf = (b: Record<string, Any>) => [b["arm-r"].angleDeg, b["arm-r"].abductionDeg, b["arm-r"].twistDeg, b["forearm-r"].angleDeg, b["forearm-r"].twistDeg, b["hand-r"].angleDeg, b["hand-r"].abductionDeg, b.bow.angleDeg, b.bow.abductionDeg];
const frog = bowFor({ s: 0.1, string: "a" }, startOf(mid));
const tip = bowFor({ s: 0.58, string: "a" }, startOf(mid));
const frogE = bowFor({ s: 0.12, string: "e" }, startOf(frog));
const tipG = bowFor({ s: 0.56, string: "g" }, startOf(tip));
const strokes = [0.1, 0.22, 0.34, 0.46, 0.58].map((s) => pick(bowFor({ s, string: "a" }, startOf(mid))));

const rest = { ...left.bones, ...leftFingers, "arm-r": { angleDeg: 14, abductionDeg: -8, twistDeg: -20 }, "forearm-r": { angleDeg: 36 }, "hand-r": { angleDeg: 6 }, ...rightFingers, bow: { angleDeg: 0 } };

violinistTexts(a, { mid: pick(mid), frog: pick(frog), tip: pick(tip), frogE: pick(frogE), tipG: pick(tipG), strokes, playBase, rest });

const restAll = restState(a);
for (const scale of SCALES) addPrisms(a, scale, [...violinPrisms(scale), ...bowPrisms(scale)], restAll);
fistHands(a, ["200", "500"], 0.125);
revolveLimbs(a, new Set(["arm-l", "forearm-l", "arm-r", "forearm-r"]));
applyDecisions(a, VIOLINIST_DECISIONS);
a.dimensions.massKg = massOf(a);
fitDimensions(a);
console.log("[DEBUG] written", await saveDoc(doc));
