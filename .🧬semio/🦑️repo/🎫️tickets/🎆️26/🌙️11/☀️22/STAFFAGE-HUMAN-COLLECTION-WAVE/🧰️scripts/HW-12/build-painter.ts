/** 🎨️ Builds `painter-at-easel` from the archive's `elderly-with-cane` (same 74-year-old body, rig, tweed jacket, flat cap and spectacles, cane removed): an elderly plein-air painter at a beech A-frame easel with a 50 × 60 cm canvas, a flat brush in the right hand and a palette on the left thumb, poses solved through `evaluate`, a looping brush-stroke clip.
 *  bun build-painter.ts [--resolve]
 * @see ./easel.ts — easel, canvas, brush and palette
 * @see ./texts-painter.ts — texts, thermal and structural model */
import { existsSync } from "node:fs";
import { toRest } from "./hold.ts";
import { addPrisms, at, cross, dot, dropBones, evalOffsets, fistHands, fitDimensions, frameOf, len, loadDoc, massOf, mul, placeBone, rd, restState, revolveLimbs, saveDoc, SCALES, solidifyFlat, solvePose, sub, T, TICKET, unit, add, type Any, type Frame, type V3 } from "./lib.ts";
import { BARS, barFrame, barPrism, brushPrisms, canvasFrame, canvasPrisms, CANVAS, lengthOf, palettePrisms } from "./easel.ts";
import { painterTexts, PAINTER_DECISIONS, propStructure } from "./texts-painter.ts";
import { applyDecisions } from "./decisions.ts";

const CACHE = `${TICKET}/🧰️scripts/HW-12/painter-poses.json`;
const resolve = process.argv.includes("--resolve") || !existsSync(CACHE);
const doc = await loadDoc("elderly-with-cane");
const a = doc.asset;
a.id = "painter-at-easel";
dropBones(a, new Set(["cane"]));
a.thermal.regions = a.thermal.regions.filter((r: Any) => r.id !== "cane-wood");
a.structure.supports = a.structure.supports.filter((s: string) => a.anchors.some((x: Any) => x.id === s));
const base: Record<string, Any> = structuredClone(a.poses.find((p: Any) => p.id === "stooped-standing").bones);
delete base.cane;
const legs = Object.fromEntries(Object.entries(base).filter(([b]) => /thigh|shank|foot|toe/.test(b)));

a.materials.push(
  { id: "oiled-beech", title: T("Oiled beech", "Geöltes Buchenholz"), emissivity: 0.9, solarAbsorptivity: 0.55, specificHeatJKgK: 1600, densityKgM3: 720, source: "authored-estimate" },
  { id: "primed-canvas", title: T("Primed linen canvas with oil paint", "Grundierte Leinwand mit Ölfarbe"), emissivity: 0.92, solarAbsorptivity: 0.45, specificHeatJKgK: 1400, source: "authored-estimate" },
  { id: "lacquered-brush", title: T("Lacquered brush handle, nickel ferrule, hog bristle", "Lackierter Pinselstiel, Zwinge aus Nickel, Schweineborste"), emissivity: 0.85, solarAbsorptivity: 0.6, specificHeatJKgK: 1500, source: "authored-estimate" },
  { id: "palette-plywood", title: T("Varnished plywood palette with oil paint", "Lackierte Sperrholzpalette mit Ölfarbe"), emissivity: 0.9, solarAbsorptivity: 0.55, specificHeatJKgK: 1600, densityKgM3: 600, source: "authored-estimate" },
);
const D = 0.6;
a.rig.bones.push(
  { id: "easel-lift", parent: "pelvis", title: T("Easel on the ground", "Staffelei am Boden"), x: 0, y: 0, angleDeg: -90, lengthM: 0.05, joint: { type: "prismatic", axisDeg: 0, minM: -0.15, maxM: 0.15 } },
  { id: "easel", parent: "easel-lift", title: T("Beech A-frame easel", "A-Staffelei aus Buche"), x: 0, y: 0, angleDeg: 0, lengthM: 0.05, joint: { type: "fixed" }, mass: { kg: 4.6, comX: 0.3, comY: 0.85, distribution: "point", source: "authored-estimate" } },
  ...BARS.map((bar) => ({ id: bar.id, parent: "easel", title: barTitle(bar.id), x: 0, y: 0, angleDeg: 0, lengthM: rd(lengthOf(bar), 4), joint: { type: "fixed" } })),
  { id: "canvas", parent: "easel", title: T("Stretched canvas 50 × 60 cm", "Keilrahmen mit Leinwand 50 × 60 cm"), x: 0, y: 0, angleDeg: 0, lengthM: CANVAS.height, joint: { type: "fixed" }, mass: { kg: 0.65, comX: 0.3, comY: -0.01, distribution: "point", source: "authored-estimate" } },
  { id: "brush", parent: "hand-r", title: T("Flat bristle brush", "Flachpinsel aus Borste"), x: 0, y: 0, angleDeg: 0, lengthM: 0.302, joint: { type: "revolute", minDeg: -35, maxDeg: 35 }, mass: { kg: 0.018, comX: 0.13, comY: 0, distribution: "point", source: "authored-estimate" } },
  { id: "palette", parent: "hand-l", title: T("Palette on the thumb", "Palette am Daumen"), x: 0, y: 0, angleDeg: 0, lengthM: 0.4, joint: { type: "revolute", minDeg: -20, maxDeg: 20 }, mass: { kg: 0.38, comX: 0.16, comY: 0, distribution: "point", source: "authored-estimate" } },
);
function barTitle(id: string) {
  return ({ "easel-leg-l": T("Left front leg", "Linkes Vorderbein"), "easel-leg-r": T("Right front leg", "Rechtes Vorderbein"), "easel-mast": T("Centre mast", "Mittelmast"), "easel-leg-back": T("Back leg", "Hinterbein"), "easel-brace": T("Lower brace", "Untere Strebe"), "easel-tray": T("Canvas tray", "Bildablage") } as Record<string, Any>)[id];
}

const stand = evalOffsets(a, legs);
const groundAt = (x: number): Frame => ({ o: [x, 0, 0], u: [1, 0, 0], v: [0, 1, 0], w: [0, 0, 1] });
console.log("[DEBUG] lift", placeBone(a, "easel-lift", toRest(a, stand, "pelvis", groundAt(D))).toExponential(2));
const restAll0 = restState(a);
const E = frameOf(restAll0, "easel");
const inEasel = (p: V3): V3 => add(E.o, add(mul(E.u, p[0]), add(mul(E.v, p[1]), mul(E.w, p[2]))));
const dirEasel = (p: V3): V3 => add(mul(E.u, p[0]), add(mul(E.v, p[1]), mul(E.w, p[2])));
for (const bar of BARS) { const f = barFrame(bar); placeBone(a, bar.id, { o: inEasel(f.o), u: dirEasel(f.u), v: dirEasel(f.v) }); }
{ const f = canvasFrame(); placeBone(a, "canvas", { o: inEasel(f.o), u: dirEasel(f.u), v: dirEasel(f.v) }); }
const rest = restState(a);
const Hr = frameOf(rest, "hand-r");
const bu = unit(add(mul(Hr.u, 0.93), add(mul(Hr.v, 0.3), mul(Hr.w, 0.2))));
const bv = unit(cross(cross(bu, Hr.v), bu));
console.log("[DEBUG] brush", placeBone(a, "brush", { o: sub(at(Hr, 0.07, 0.012, 0.018), mul(bu, 0.12)), u: bu, v: bv }).toExponential(2));
const Hl = frameOf(restState(a), "hand-l");
const pu = unit(add(mul(Hl.u, 0.87), mul(Hl.v, -0.5)));
const pv = unit(cross(cross(pu, mul(Hl.w, -1)), pu));
console.log("[DEBUG] palette", placeBone(a, "palette", { o: at(Hl, 0.045, 0.035, -0.024), u: pu, v: mul(pv, -1) }).toExponential(2));

const soft = (x: number, lo: number, hi: number): number => (x < lo ? (lo - x) ** 2 : x > hi ? (x - hi) ** 2 : 0);
const C = () => frameOf(evalOffsets(a, legs), "canvas");
const tipOf = (s: Any) => at(frameOf(s, "brush"), 0.302, 0, 0);
const armUnknowns = (side: "l" | "r") => [[`arm-${side}`, "angleDeg"], [`arm-${side}`, "abductionDeg"], [`arm-${side}`, "twistDeg"], [`forearm-${side}`, "angleDeg"], [`forearm-${side}`, "twistDeg"], [`hand-${side}`, "angleDeg"], [`hand-${side}`, "abductionDeg"]] as const;
const limits = (x: number[]) => soft(x[0]!, -55, 160) + soft(x[3]!, 2, 145) + soft(x[5]!, -30, 18) + soft(x[6]!, -60, 60);

/** 🎨️ Left hand holding the palette in front of the left hip, the board level. */
function solvePalette(baseBones: Record<string, Any>, start?: number[]) {
  const S0 = evalOffsets(a, baseBones);
  const pel = frameOf(S0, "pelvis");
  const target = add(pel.o, [0.27, 0.16, 0.2]);
  return solvePose(a, baseBones, armUnknowns("l"), start ?? [35, 10, 0, 85, 80, 0, 0], (s, x) => {
    const P = frameOf(s, "palette");
    return 300 * len(sub(P.o, target)) ** 2 + 1.5 * (1 - dot(P.v, [0, 1, 0])) + 0.3 * (1 - dot(P.u, unit([0.6, 0, 0.8]))) + 1e-3 * limits(x) + 1e-7 * x.reduce((q, b) => q + b * b, 0);
  });
}

/** 🖌️ Right hand: the bristle tip at a point and the brush pointing along a direction. */
function solveBrush(baseBones: Record<string, Any>, tip: V3, dir: V3, start?: number[]) {
  const unknowns = [...armUnknowns("r"), ["brush", "angleDeg"], ["brush", "abductionDeg"]] as const;
  return solvePose(a, baseBones, unknowns, start ?? [40, -10, -20, 70, -40, 0, 0, 0, 0], (s, x) => {
    const B = frameOf(s, "brush");
    return 400 * len(sub(tipOf(s), tip)) ** 2 + 1.2 * (1 - dot(B.u, dir)) + 1e-3 * (limits(x) + soft(x[7]!, -30, 30) + soft(x[8]!, -30, 30)) + 1e-7 * x.reduce((q, b) => q + b * b, 0) + 2e-5 * (x[7]! ** 2 + x[8]! ** 2);
  });
}

let solved: Record<string, Any>;
if (resolve) {
  const pal = solvePalette({ ...base });
  const holdBase = { ...base, ...pal.bones };
  const Cf = C();
  const onCanvas = (h: number, w: number, off = 0): V3 => at(Cf, h, off + 0.004, w);
  const into = (k: number): V3 => unit(add(mul(Cf.v, -1), [0, -0.25 * k, 0]));
  const strokePts: V3[] = [onCanvas(0.34, -0.1), onCanvas(0.38, -0.02), onCanvas(0.42, 0.06), onCanvas(0.4, 0.0, 0.03), onCanvas(0.36, -0.08, 0.03)];
  const strokes: Record<string, Any>[] = [];
  let start: number[] | undefined;
  for (const [i, p] of strokePts.entries()) {
    const r = solveBrush(holdBase, p, into(i < 3 ? 1 : 0.6), start);
    start = ["arm-r", "arm-r", "arm-r", "forearm-r", "forearm-r", "hand-r", "hand-r", "brush", "brush"].map((b, k) => r.bones[b][["angleDeg", "abductionDeg", "twistDeg", "angleDeg", "twistDeg", "angleDeg", "abductionDeg", "angleDeg", "abductionDeg"][k]!]);
    console.log(`[DEBUG] stroke ${i} tip ${(len(sub(tipOf(evalOffsets(a, r.bones)), p)) * 1000).toFixed(1)} mm`);
    strokes.push(Object.fromEntries(["arm-r", "forearm-r", "hand-r", "brush"].map((b) => [b, r.bones[b]])));
  }
  const P = frameOf(evalOffsets(a, holdBase), "palette");
  const mix = solveBrush(holdBase, at(P, 0.22, 0.012, 0.09), unit(add(mul(P.v, -1), mul(P.u, 0.4))), start);
  console.log(`[DEBUG] mix tip ${(len(sub(tipOf(evalOffsets(a, mix.bones)), at(P, 0.22, 0.012, 0.09))) * 1000).toFixed(1)} mm`);
  solved = { pal: pal.bones, strokes, mix: Object.fromEntries(["arm-r", "forearm-r", "hand-r", "brush"].map((b) => [b, mix.bones[b]])) };
  await Bun.write(CACHE, JSON.stringify(solved, null, 1));
} else solved = await Bun.file(CACHE).json();

const pal = Object.fromEntries(["arm-l", "forearm-l", "hand-l"].map((b) => [b, solved.pal[b]]));
painterTexts(a, { base, pal, strokes: solved.strokes, mix: solved.mix });

const restFinal = restState(a);
for (const scale of SCALES) addPrisms(a, scale, [...BARS.map((bar) => barPrism(bar, scale)).filter((p): p is NonNullable<typeof p> => p !== undefined), ...canvasPrisms(scale), ...brushPrisms(scale), ...palettePrisms(scale)], restFinal);
const lift = evalOffsets(a, a.poses[0].bones, a.poses[0].id);
const footY = Math.min(...["heel-l", "ball-l", "heel-r", "ball-r"].map((id) => lift.anchors[id].at.y));
const easelY = lift.anchors["easel-foot-l"].at.y;
console.log("[DEBUG] feet", rd(footY, 4), "easel feet", rd(easelY, 4));
fistHands(a, ["200", "500"], 0.11);
revolveLimbs(a, new Set(["arm-l", "forearm-l", "arm-r", "forearm-r"]));
solidifyFlat(a, new Set(a.rig.bones.filter((b: Any) => /^(hand|thumb|index|middle|ring|little)-/.test(b.id)).map((b: Any) => b.id)));
const stateDefault = evalOffsets(a, a.poses[0].bones, a.poses[0].id);
for (const scale of SCALES) a.volumes[scale].structure.push(...propStructure(a, scale, stateDefault));
applyDecisions(a, PAINTER_DECISIONS);
a.rig.bones.find((b: Any) => b.id === "hand-r").title = T("Right hand on the brush", "Rechte Hand am Pinsel");
a.anchors = a.anchors.filter((x: Any) => x.id !== "cane-grip");
for (const view of ["elevation", "front", "plan"]) for (const s of a.representations["500"].geometry[view]) if (s.id === "palette" || s.id === "brush") s.penMm = 0.2; else if (s.id === "hand-l") s.penMm = 0.16;
a.materials = a.materials.filter((m: Any) => { const { materials, ...rest } = a; void materials; return JSON.stringify(rest).includes(`"${m.id}"`); });
a.dimensions.massKg = massOf(a);
fitDimensions(a);
console.log("[DEBUG] written", await saveDoc(doc));
