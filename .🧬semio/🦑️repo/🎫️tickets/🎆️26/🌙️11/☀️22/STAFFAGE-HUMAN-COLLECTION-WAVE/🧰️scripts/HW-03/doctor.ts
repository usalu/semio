/** 🩺️ Generator of `doctor-with-clipboard`, derived from `walking-commuter` (rig, body, knee-length coat cut, hands with fingers): the overcoat becomes a white lab coat over shirt and pullover, scarf and bag go, a stethoscope lies round the neck and a clipboard sits in the right hand with a pen in the left; reading, writing and talking poses, three clips.
 *  bun doctor.ts [--out file.json]
 * @see ./lib.ts — the derivation helpers
 * @see ../../📋️brief.md — the brief */
import { parseArgs } from "node:util";
import { type P2, type P3, ellipse, loftSurface, rect, stroke } from "./geo.ts";
import { framesOf, solvePose, worldOf, dist, dot } from "./ik.ts";
import { type J, dropSolids, repairPrisms, prismLimbs, finishDimensions, bindRegions, dropBones, dropRegion, dubois, load, pruneMaterials, references, regionOf, renameRegion, replaceMaterial, save, shapeOf, shapes, solidOf, solids, t } from "./lib.ts";
import { DOCTOR_TEXT } from "./doctor-text.ts";
import { retrace, traced, twinDepth } from "./trace.ts";

const { values } = parseArgs({ options: { out: { type: "string" } } });
const doc = await load("walking-commuter");
const a: J = doc.asset;
a.id = "doctor-with-clipboard";

//#region 🧹️ what the commuter carries and the doctor does not
dropBones(a, ["bag"]);
dropRegion(a, "bag-leather");
for (const view of ["elevation", "front", "plan"]) Object.assign(shapeOf(a, "500", view, "scarf"), { id: "collar", region: "coat", material: "wool-coat", tone: 0.88 });
Object.assign(solidOf(a, "500", "body", "scarf"), { id: "collar", material: "wool-coat", tone: 0.88 });
const th500 = a.volumes["500"].thermal;
th500.splice(th500.findIndex((x: J) => x.id === "th-scarf"), 1, { id: "th-collar", of: "collar", region: "coat" });
for (const s of ["100", "50"]) {
  Object.assign(shapeOf(a, s, "front", "scarf-front"), { id: "shirt-front", region: "coat", material: "cotton-shirt", tone: 0.97 });
  Object.assign(solidOf(a, s, "body", "scarf-front"), { id: "shirt-front", material: "cotton-shirt", tone: 0.97 });
  for (const th of a.volumes[s].thermal) if (th.of === "scarf-front") Object.assign(th, { id: "th-shirt-front", of: "shirt-front", region: "coat" }), delete th.layer;
}
dropRegion(a, "scarf");
dropSolids(a, (x) => x.material === "wool-scarf");
a.poses = a.poses.filter((p: J) => p.id === "standing");
a.clips = [];
a.symbols = [];
//#endregion

//#region 🧵️ lab coat, shirt, pullover, chinos
replaceMaterial(a, "wool-coat", { id: "lab-coat", title: t("Cotton-polyester lab coat twill", "Laborkittel aus Baumwoll-Polyester-Köper"), emissivity: 0.9, solarAbsorptivity: 0.3, specificHeatJKgK: 1300, source: "authored-estimate", reference: "ashrae-2021" });
replaceMaterial(a, "wool-suit", { id: "merino-knit", title: t("Fine merino knit", "Feiner Merinostrick"), emissivity: 0.92, solarAbsorptivity: 0.7, specificHeatJKgK: 1360, source: "authored-estimate", reference: "ashrae-2021" });
replaceMaterial(a, "wool-trouser", { id: "cotton-chino", title: t("Cotton twill chinos", "Chino aus Baumwollköper"), emissivity: 0.92, solarAbsorptivity: 0.75, specificHeatJKgK: 1300, source: "authored-estimate", reference: "ashrae-2021" });
for (const { shape } of shapes(a)) if (shape.material === "lab-coat") (shape.tone = +Math.min(0.95, shape.tone + 0.6).toFixed(3)), delete shape.hatch;
for (const { solid } of solids(a)) if (solid.material === "lab-coat" && solid.tone !== undefined) solid.tone = +Math.min(0.95, solid.tone + 0.6).toFixed(3);
for (const s of ["100", "50"]) a.representations[s].hatches = a.representations[s].hatches.filter((h: J) => h.id !== "wool");
renameRegion(a, "coat", "lab-coat");
for (const { solid } of solids(a)) if (solid.layer === "jacket") solid.layer = "pullover";
for (const { solid } of solids(a)) if (solid.layer === "overcoat") solid.layer = "lab-coat";
a.materials.push(
  { id: "pvc-tubing", title: t("PVC stethoscope tubing", "PVC-Stethoskopschlauch"), emissivity: 0.92, solarAbsorptivity: 0.85, densityKgM3: 1300, source: "authored-estimate", reference: "ashrae-2021" },
  { id: "stainless-steel", title: t("Brushed stainless steel", "Gebürsteter Edelstahl"), emissivity: 0.3, solarAbsorptivity: 0.5, densityKgM3: 7900, conductivityWmK: 16, specificHeatJKgK: 500, source: "literature", reference: "incropera-2007" },
  { id: "hardboard", title: t("Lacquered hardboard", "Lackierte Hartfaserplatte"), emissivity: 0.9, solarAbsorptivity: 0.7, densityKgM3: 900, conductivityWmK: 0.15, specificHeatJKgK: 1700, source: "authored-estimate", reference: "incropera-2007" },
  { id: "paper", title: t("Office paper", "Büropapier"), emissivity: 0.93, solarAbsorptivity: 0.25, densityKgM3: 800, specificHeatJKgK: 1340, source: "literature", reference: "incropera-2007" },
  { id: "pen-plastic", title: t("Ballpoint pen barrel (polystyrene)", "Kugelschreiberschaft (Polystyrol)"), emissivity: 0.9, solarAbsorptivity: 0.6, densityKgM3: 1050, source: "authored-estimate", reference: "incropera-2007" },
);
//#endregion

//#region ⚖️ mass, physiology, thermal regions
const BODY_KG = 75, CLOTHES_KG = 2.6, STETH_KG = 0.17, CLIP_KG = 0.42;
const before = a.rig.bones.reduce((s: number, b: J) => s + (b.mass?.kg ?? 0), 0);
for (const b of a.rig.bones) if (b.mass) b.mass.kg = +((b.mass.kg * (BODY_KG + CLOTHES_KG + STETH_KG)) / before).toFixed(4);
const STATURE = 1.78;
const skin = +dubois(BODY_KG, STATURE).toFixed(3);
Object.assign(a.thermal.body, { massKg: BODY_KG, skinAreaM2: skin, basalMetabolismW: 79, coreSetPointC: 36.8, skinSetPointC: 34.1 });
a.thermal.environment = { airTempC: 22, meanRadiantTempC: 22, skyTempC: 22, groundTempC: 22, relativeHumidity: 0.4, windMS: 0.1, windDirectionDeg: 180, solarWM2: 0, diffuseFraction: 1, pressurePa: 101325 };
a.thermal.referenceEnvironment = "office-neutral";
a.thermal.baseMet = 1.3;
const face = regionOf(a, "face-skin");
Object.assign(face.body, { metabolicShare: 0.07, skinFraction: 0.071 });
face.title = t("Face and neck skin", "Haut von Gesicht und Hals");
const R = (clo: number): number => +(clo * 0.155 * 1.15).toFixed(5);
const RV = (r: number): number => +(r / (0.0165 * 0.38)).toFixed(1);
const coat = regionOf(a, "lab-coat");
coat.title = t("Lab coat over pullover, shirt and undershirt on torso and arms", "Laborkittel über Pullover, Hemd und Unterhemd an Rumpf und Armen");
coat.material = "lab-coat";
coat.layers = [
  { id: "tee", title: t("Cotton undershirt, short sleeve", "Baumwollunterhemd, kurzarm"), kind: "clothing", material: "cotton-underwear", thicknessM: 0.0008, resistanceM2KW: R(0.08), vapourResistanceM2PaW: RV(R(0.08)), heatCapacityJM2K: 180, source: "authored-estimate", reference: "iso-9920-2007" },
  { id: "shirt", title: t("Cotton shirt, long sleeve", "Baumwollhemd, langarm"), kind: "clothing", material: "cotton-shirt", thicknessM: 0.0012, resistanceM2KW: R(0.25), vapourResistanceM2PaW: RV(R(0.25)), heatCapacityJM2K: 160, source: "authored-estimate", reference: "iso-9920-2007" },
  { id: "pullover", title: t("Thin merino V-neck pullover", "Dünner Merino-Pullover mit V-Ausschnitt"), kind: "clothing", material: "merino-knit", thicknessM: 0.003, resistanceM2KW: R(0.25), vapourResistanceM2PaW: RV(R(0.25)), heatCapacityJM2K: 450, source: "authored-estimate", reference: "iso-9920-2007" },
  { id: "lab-coat", title: t("Knee-length lab coat", "Knielanger Laborkittel"), kind: "clothing", material: "lab-coat", thicknessM: 0.006, resistanceM2KW: R(0.3), vapourResistanceM2PaW: RV(R(0.3)), heatCapacityJM2K: 520, source: "authored-estimate", reference: "iso-9920-2007" },
];
coat.heatCapacityJM2K = 13000 + 180 + 160 + 450 + 520;
const legs = regionOf(a, "trousers");
legs.title = t("Chinos over the legs", "Chinohose über den Beinen");
legs.layers[1] = { ...legs.layers[1], title: t("Cotton chinos, straight", "Baumwoll-Chino, gerade"), material: "cotton-chino", resistanceM2KW: R(0.24), vapourResistanceM2PaW: RV(R(0.24)), heatCapacityJM2K: 380 };
legs.heatCapacityJM2K = 13000 + 100 + 380;
a.thermal.regions.push(
  { id: "stethoscope", title: t("Stethoscope round the neck", "Stethoskop um den Hals"), kind: "surface", model: "passive", material: "pvc-tubing", areaM2: 0.025, characteristicLengthM: 0.01, geometry: "cylinder", skyViewFactor: 0.5, layers: [], heatCapacityJM2K: 2600, ratedPowerW: 0, ratedPowerBasis: "passive", source: "authored-estimate", reference: "incropera-2007" },
  { id: "clipboard", title: t("Clipboard with paper and pen", "Klemmbrett mit Papier und Stift"), kind: "surface", model: "passive", material: "hardboard", areaM2: 0.16, characteristicLengthM: 0.23, geometry: "horizontal-plate-up", skyViewFactor: 0.6, layers: [], heatCapacityJM2K: 5100, ratedPowerW: 0, ratedPowerBasis: "passive", source: "authored-estimate", reference: "incropera-2007" },
);
//#endregion


//#region 📋️ clipboard bone, board, paper, clip; pen in the left hand
const hr = a.rig.bones.findIndex((b: J) => b.id === "hand-r");
const BW0 = 0.05, BW1 = 0.3, BU0 = -0.085, BU1 = 0.235;
a.rig.bones.splice(hr + 1, 0, { id: "clipboard", parent: "hand-r", title: t("Clipboard (grip frame)", "Klemmbrett (Griffrahmen)"), x: 0.065, y: 0, angleDeg: 0, lengthM: 0.07, joint: { type: "fixed" }, mass: { kg: CLIP_KG, comX: 0.075, comY: 0.001, distribution: "point", source: "authored-estimate", reference: "iso-216-2007" } });
const restZ = -0.221;
const z0 = +(restZ + BW0).toFixed(4), z1 = +(restZ + BW1).toFixed(4);
const pz0 = +(z0 + 0.01).toFixed(4), pz1 = +(z1 - 0.01).toFixed(4);
const geo = (s: string): J => a.representations[s].geometry;
const maxZ = (list: J[], pred: (s: J) => boolean): number => Math.max(0, ...list.filter(pred).map((s) => s.z));
const SHEET = (thick: number): J => ({ kind: "poly", closed: true, points: rect(BU0, -0.0015, BU1, -0.0015 + thick) });
for (const s of ["500", "200", "100", "50"]) {
  const g = geo(s);
  const zr = maxZ(g.elevation, (x) => x.bone === "hand-r" || x.bone === "forearm-r") + 0.05;
  const fine = s === "50";
  const boardT = s === "50" ? 0.003 : 0.004;
  const board = { id: "clipboard-board", bone: "clipboard", role: "body", filled: true, tone: fine ? 0.32 : 0.55, region: "clipboard", material: "hardboard", ...SHEET(boardT) };
  g.elevation.push({ ...board, z: zr });
  if (fine) g.elevation.push({ id: "clipboard-paper", bone: "clipboard", role: "detail", z: zr + 0.01, filled: true, tone: 0.97, region: "clipboard", material: "paper", kind: "poly", closed: true, points: rect(BU0 + 0.012, 0.0015, BU1 - 0.012, 0.0025) }, { id: "clipboard-clip", bone: "clipboard", role: "body", z: zr + 0.02, filled: true, tone: 0.62, region: "clipboard", material: "stainless-steel", kind: "poly", closed: true, points: rect(BU1 - 0.06, 0.0015, BU1 - 0.004, 0.0115) });
  for (const view of ["front", "plan"]) {
    const zv = maxZ(g[view], (x) => x.bone === "hand-r" || x.bone === "forearm-r") + 0.05;
    g[view].push({ ...board, z: zv, solid: { kind: "extruded", sweep: "w", z0, z1 } });
    if (fine || s === "100") g[view].push({ id: "clipboard-paper", bone: "clipboard", role: "detail", z: zv + 0.01, filled: true, tone: 0.97, region: "clipboard", material: "paper", kind: "poly", closed: true, points: rect(BU0 + 0.012, 0.0005, BU1 - 0.012, 0.0025), solid: { kind: "extruded", sweep: "w", z0: pz0, z1: pz1 } });
    if (fine) g[view].push({ id: "clipboard-clip", bone: "clipboard", role: "body", z: zv + 0.02, filled: true, tone: 0.62, region: "clipboard", material: "stainless-steel", kind: "poly", closed: true, points: rect(BU1 - 0.06, 0.0015, BU1 - 0.004, 0.0115), solid: { kind: "extruded", sweep: "w", z0: +(restZ + 0.125).toFixed(4), z1: +(restZ + 0.225).toFixed(4) } });
  }
  const v = a.volumes[s];
  v.body.push({ id: "clipboard-board", kind: "box", bone: "clipboard", material: "hardboard", role: "body", tone: fine ? 0.32 : 0.55, sheet: true, size: [BU1 - BU0, boardT, BW1 - BW0], transform: { translation: [(BU0 + BU1) / 2, -0.0015 + boardT / 2, (BW0 + BW1) / 2] } });
  v.thermal.push({ id: "th-clipboard-board", of: "clipboard-board", region: "clipboard" });
  if (fine) {
    v.body.push(
      { id: "clipboard-paper", kind: "box", bone: "clipboard", material: "paper", role: "detail", tone: 0.97, sheet: true, size: [BU1 - BU0 - 0.024, 0.001, BW1 - BW0 - 0.02], transform: { translation: [(BU0 + BU1) / 2, 0.002, (BW0 + BW1) / 2] } },
      { id: "clipboard-clip", kind: "box", bone: "clipboard", material: "stainless-steel", role: "body", tone: 0.62, size: [0.056, 0.01, 0.1], cornerRadius: 0.002, transform: { translation: [BU1 - 0.032, 0.0065, 0.175] } },
    );
    v.thermal.push({ id: "th-clipboard-paper", of: "clipboard-paper", region: "clipboard" }, { id: "th-clipboard-clip", of: "clipboard-clip", region: "clipboard" });
  }
}
const PEN = { u: 0.08, v: 0.006, w0: -0.012, w1: -0.16, r: 0.0045 };
{
  const g = geo("50");
  const zl = maxZ(g.elevation, (x) => x.bone === "hand-l") + 0.02;
  const prof = ellipse(PEN.u, PEN.v, PEN.r, PEN.r, 12);
  g.elevation.push({ id: "pen", bone: "hand-l", role: "detail", z: zl, filled: true, tone: 0.2, region: "clipboard", material: "pen-plastic", kind: "poly", closed: true, points: prof });
  for (const view of ["front", "plan"]) g[view].push({ id: "pen", bone: "hand-l", role: "detail", z: maxZ(g[view], (x) => x.bone === "hand-l") + 0.02, filled: true, tone: 0.2, region: "clipboard", material: "pen-plastic", kind: "poly", closed: true, points: prof, solid: { kind: "extruded", sweep: "w", z0: +(0.221 + PEN.w1).toFixed(4), z1: +(0.221 + PEN.w0).toFixed(4) } });
  a.volumes["50"].body.push({ id: "pen", kind: "capsule", bone: "hand-l", material: "pen-plastic", role: "detail", tone: 0.2, p0: [PEN.u, PEN.v, PEN.w0], p1: [PEN.u, PEN.v, PEN.w1], r0: PEN.r, r1: PEN.r });
  a.volumes["50"].thermal.push({ id: "th-pen", of: "pen", region: "clipboard" });
}
//#endregion

//#region 🩺️ stethoscope round the neck, laid on the coat of every scale
const TUBE_R = 0.005;
for (const s of ["500", "200", "100", "50"]) {
  const coatLoft = solidOf(a, s, "body", "coat");
  const front = (u: number, w: number): P3 => [u, +((loftSurface(coatLoft, u, w, -1) ?? -0.06) - TUBE_R + 0.002).toFixed(4), w];
  const right: P3[] = [[0.12, -0.085], [0.17, -0.088], [0.22, -0.088], [0.265, -0.084], [0.295, -0.078]].map(([u, w]) => front(u!, w!));
  const left: P3[] = [[0.14, 0.086], [0.18, 0.088], [0.22, 0.088], [0.265, 0.084], [0.295, 0.078]].map(([u, w]) => front(u!, w!));
  const top: P3[] = [[0.319, -0.004, -0.076], [0.326, 0.04, -0.045], [0.329, 0.058, 0], [0.326, 0.04, 0.045], [0.319, -0.004, 0.076]];
  const path = [...right, ...top, ...left.reverse()];
  const v = a.volumes[s];
  v.body.push({ id: "stethoscope-tube", kind: "loft", bone: "torso", material: "pvc-tubing", role: "detail", tone: 0.18, slender: true, sections: path.map((p) => ({ at: p, rx: TUBE_R, ry: TUBE_R, exponent: 2 })), capStart: true, capEnd: true });
  v.thermal.push({ id: "th-stethoscope-tube", of: "stethoscope-tube", region: "stethoscope" });
  const end = left[0]!;
  if (s === "50") {
    v.body.push({ id: "stethoscope-chest-piece", kind: "ellipsoid", bone: "torso", material: "stainless-steel", role: "detail", tone: 0.62, radii: [0.0225, 0.006, 0.0225], transform: { translation: [end[0] - 0.02, +(end[1] - 0.003).toFixed(4), end[2]] } });
    v.thermal.push({ id: "th-stethoscope-chest-piece", of: "stethoscope-chest-piece", region: "stethoscope" });
  }
  const g = geo(s);
  const toFront = (p: P3): P2 => [p[2], +(1.239 + p[0]).toFixed(4)];
  const toPlan = (p: P3): P2 => [-p[1], p[2]];
  const toElev = (p: P3): P2 => [p[0], p[1]];
  const zOf = (view: string): number => maxZ(g[view], (x) => x.bone === "torso" || x.bone === "lumbar" || x.bone === "neck") + 0.03;
  if (s === "50" || s === "100") {
    const width = 0.01;
    const draw = (view: string, id: string, pts: P2[]): void => {
      if (s === "50") g[view].push({ id, bone: "torso", role: "detail", z: zOf(view), filled: true, tone: 0.18, region: "stethoscope", material: "pvc-tubing", kind: "poly", closed: true, points: stroke(pts, width) });
      else g[view].push({ id, bone: "torso", role: "detail", z: zOf(view), filled: false, tone: 0.18, region: "stethoscope", material: "pvc-tubing", kind: "poly", closed: false, points: pts, penMm: 0.1 });
    };
    draw("elevation", "stethoscope-tube", [...right, top[0]!].map(toElev));
    draw("front", "stethoscope-tube-r", right.map(toFront));
    draw("front", "stethoscope-tube-l", [...left].reverse().map(toFront));
    draw("plan", "stethoscope-tube", [right[right.length - 1]!, ...top, left[0]!].map(toPlan));
    if (s === "50") {
      g.front.push({ id: "stethoscope-chest-piece", bone: "torso", role: "detail", z: zOf("front") + 0.01, filled: true, tone: 0.62, region: "stethoscope", material: "stainless-steel", kind: "poly", closed: true, points: ellipse(end[2], 1.239 + end[0] - 0.02, 0.0225, 0.0225, 16) });
    }
  } else for (const view of ["elevation", "front", "plan"]) {
    const sil = g[view].find((x: J) => x.id === "coat");
    sil.regions = ["lab-coat", "stethoscope"];
    delete sil.region;
  }
}
//#endregion

//#region 🧍️ poses solved on the rig: reading, writing, talking
const standing = a.poses[0];
const head = { neck: { angleDeg: -9 }, head: { angleDeg: -13 } };
const boardCentre: P3 = [0.075, 0, 0.175];
const solveRight = (base: J, target: P3, deg: number): J => {
  const dir: P3 = [Math.cos((deg * Math.PI) / 180), Math.sin((deg * Math.PI) / 180), 0];
  return solvePose(a, base, ["arm-r.angleDeg", "forearm-r.angleDeg", "hand-r.angleDeg"], (f) => dist(worldOf(f, "clipboard", boardCentre), target) ** 2 + 0.05 * (1 - dot(f["clipboard"]!.frame.u, dir)));
};
const solveLeft = (base: J, lift: number): J => {
  const f0 = framesOf(a, base);
  const goal = worldOf(f0, "clipboard", [0.1, 0.0025 + lift, 0.221 + PEN.w1 - restZ]);
  return solvePose(a, base, ["arm-l.angleDeg", "forearm-l.angleDeg", "hand-l.angleDeg"], (f) => dist(worldOf(f, "hand-l", [PEN.u, PEN.v, PEN.w1]), goal) ** 2);
};
const bonesBase = { ...structuredClone(standing.bones), ...head, torso: { angleDeg: -1.2 }, lumbar: { angleDeg: -0.6 }, "arm-l": { angleDeg: 20 }, "forearm-l": { angleDeg: 80 }, "hand-l": { angleDeg: 0 } };
delete bonesBase.bag;
const reading = solveLeft(solveRight(bonesBase, [0.35, 1.32, -0.046], 36), 0.035);
const writing = solveLeft(structuredClone(reading), 0.0);
const talkingBase = { ...structuredClone(reading), neck: { angleDeg: 2 }, head: { angleDeg: 2 } };
const talking = solveLeft(solveRight(talkingBase, [0.33, 1.16, -0.046], 12), 0.03);
const round = (o: J): J => Object.fromEntries(Object.entries(o).map(([k, v]) => [k, Object.fromEntries(Object.entries(v as J).map(([f, x]) => [f, +(x as number).toFixed(3)]))]));
const binding = (exposed: Record<string, number>): J => Object.fromEntries(a.thermal.regions.map((r: J) => [r.id, { exposedFraction: exposed[r.id] ?? 1, sunlitFraction: 0 }]));
const thermalOf = (met: number, breathing: string): J => ({ activityMet: met, breathing, regions: binding({ "lab-coat": 0.9, "hand-skin": 0.92, clipboard: 0.85, stethoscope: 0.8 }), emitters: { "body-plume": 1 } });
const pose = (id: string, title: J, description: J, bones: J, met: number, breathing: string): J => ({ id, title, description, bones: round(bones), ground: "auto", contacts: ["heel-l", "ball-l", "heel-r", "ball-r"], thermal: thermalOf(met, breathing), loads: [] });
a.poses = [
  pose("reading-clipboard", t("Reading the clipboard", "Liest auf dem Klemmbrett"), t("Head bowed over the chart on the clipboard held in the right hand at chest height, tilted 36 degrees towards the eyes; the left hand rests with the pen beside the board.", "Kopf über das Krankenblatt auf dem Klemmbrett geneigt, das die rechte Hand in Brusthöhe hält, 36 Grad zu den Augen gekippt; die linke Hand ruht mit dem Stift neben dem Brett."), reading, 1.3, "rest-standing"),
  pose("writing-note", t("Writing a note", "Schreibt eine Notiz"), t("The left hand sets the pen on the left margin of the sheet while the right hand steadies the board; head bowed.", "Die linke Hand setzt den Stift auf den linken Rand des Blatts, während die rechte Hand das Brett hält; Kopf geneigt."), writing, 1.5, "rest-standing"),
  pose("talking-to-patient", t("Talking to a patient", "Spricht mit einer Patientin oder einem Patienten"), t("Head up and facing ahead, the clipboard lowered to waist height and almost level, the pen resting on it; breathing through the mouth while speaking.", "Kopf gehoben und nach vorn gerichtet, das Klemmbrett auf Taillenhöhe gesenkt und fast waagerecht, der Stift liegt darauf; Atmung beim Sprechen durch den Mund."), talking, 1.4, "talking"),
];
for (const p of a.poses) for (const [k, v] of Object.entries(p.bones) as [string, J][]) {
  const j = a.rig.bones.find((b: J) => b.id === k)?.joint;
  if (!j || j.type === "fixed") (delete p.bones[k]);
  else if (v.angleDeg !== undefined && (v.angleDeg < (j.minDeg ?? -180) || v.angleDeg > (j.maxDeg ?? 180))) throw new Error(`${p.id} ${k} ${v.angleDeg} outside ${j.minDeg}..${j.maxDeg}`);
}
a.presentation = { defaultPose: "reading-clipboard", defaultClip: "read-and-note", defaultScale: 100 };
//#endregion

//#region 🫁️ respiration and clips
const rest = a.thermal.respiration.patterns.find((p: J) => p.id === "rest-standing");
a.thermal.respiration.patterns = [
  { ...rest, title: t("Quiet standing, reading", "Ruhiges Stehen beim Lesen") },
  { id: "talking", title: t("Speaking with a patient", "Sprechen mit einer Patientin oder einem Patienten"), rateBpm: 15, tidalVolumeL: 0.6, inhaleFraction: 0.18, pauseAfterInhaleFraction: 0.02, exhaleFraction: 0.75, pauseAfterExhaleFraction: 0.05, route: "mouth", source: "authored-estimate", reference: "gupta-2010" },
];
const feet = (d: number): J[] => ["heel-l", "ball-l", "heel-r", "ball-r"].map((anchor) => ({ anchor, fromS: 0, toS: d }));
const P = (id: string): J => a.poses.find((p: J) => p.id === id).bones;
const scribble = (dz: number, dh: number): J => ({ "hand-l": { angleDeg: +(P("writing-note")["hand-l"].angleDeg + dh).toFixed(3) }, "forearm-l": { angleDeg: +(P("writing-note")["forearm-l"].angleDeg + dz).toFixed(3) } });
a.clips = [
  {
    id: "read-and-note", title: t("Reading and noting", "Lesen und Notieren"), description: t("Reads the chart, sets the pen on the sheet, writes three short strokes and reads on; a loop of 8 s at quiet breathing.", "Liest das Krankenblatt, setzt den Stift auf, schreibt drei kurze Striche und liest weiter; eine Schleife von 8 s bei ruhiger Atmung."),
    durationS: 8, fps: 30, loop: true, rootMotion: "none",
    keys: [
      { t: 0, pose: "reading-clipboard", ease: "smooth" },
      { t: 2.2, pose: "reading-clipboard", ease: "smooth", bones: { head: { angleDeg: +(P("reading-clipboard").head.angleDeg - 3).toFixed(3) } } },
      { t: 3.2, pose: "writing-note", ease: "smooth" },
      { t: 3.8, pose: "writing-note", ease: "smooth", bones: scribble(-2, 3) },
      { t: 4.4, pose: "writing-note", ease: "smooth", bones: scribble(1.5, -2) },
      { t: 5.0, pose: "writing-note", ease: "smooth", bones: scribble(-1.5, 3) },
      { t: 5.6, pose: "writing-note", ease: "smooth" },
      { t: 6.6, pose: "reading-clipboard", ease: "smooth" },
      { t: 8, pose: "reading-clipboard", ease: "smooth" },
    ],
    contacts: feet(8),
    phases: [
      { id: "reading", fromS: 0, toS: 3.2, title: t("Reading the chart", "Lesen im Krankenblatt") },
      { id: "writing", fromS: 3.2, toS: 5.6, title: t("Writing a note", "Schreiben einer Notiz") },
      { id: "reading-on", fromS: 5.6, toS: 8, title: t("Reading on", "Weiterlesen") },
    ],
    events: [
      { t: 3.2, kind: "motion", title: t("Pen on the sheet", "Stift auf dem Blatt") },
      { t: 5.6, kind: "motion", title: t("Pen lifted", "Stift abgehoben") },
    ],
    tracks: [
      { target: "activityMet", ease: "smooth", keys: [{ t: 0, v: 1.3 }, { t: 3.2, v: 1.5 }, { t: 5.6, v: 1.5 }, { t: 6.6, v: 1.3 }, { t: 8, v: 1.3 }] },
      { target: "breathing", ease: "step", keys: [{ t: 0, v: 0 }, { t: 8, v: 0 }] },
    ],
  },
  {
    id: "idle-breathing", title: t("Reading at rest", "Ruhiges Lesen"), description: t("Holds the reading pose with the quiet breathing pattern (13 per minute): the chest rises by the derived tidal motion, the shoulders follow by 1.5 degrees.", "Hält die Lesehaltung mit dem ruhigen Atemmuster (13 je Minute): Die Brust hebt sich mit der abgeleiteten Atembewegung, die Schultern folgen um 1,5 Grad."),
    durationS: 4, fps: 30, loop: true, rootMotion: "none",
    keys: [
      { t: 0, pose: "reading-clipboard", ease: "smooth" },
      { t: 2, pose: "reading-clipboard", ease: "smooth", bones: { torso: { angleDeg: +(P("reading-clipboard").torso.angleDeg + 0.6).toFixed(3) }, lumbar: { angleDeg: +(P("reading-clipboard").lumbar.angleDeg + 0.4).toFixed(3) }, "clavicle-l": { angleDeg: 1.5 }, "clavicle-r": { angleDeg: 1.5 } } },
      { t: 4, pose: "reading-clipboard", ease: "smooth" },
    ],
    contacts: feet(4),
    phases: [{ id: "quiet-reading", fromS: 0, toS: 4, title: t("Quiet reading at the bedside", "Ruhiges Lesen am Krankenbett") }],
    events: [],
    tracks: [{ target: "breathing", ease: "step", keys: [{ t: 0, v: 0 }, { t: 4, v: 0 }] }],
  },
  {
    id: "look-up-and-talk", title: t("Looking up to talk", "Aufblicken und sprechen"), description: t("Lowers the clipboard from reading to waist height, lifts the head towards the patient and starts to speak: the breathing switches to the speaking pattern at 1.6 s.", "Senkt das Klemmbrett aus der Lesehaltung auf Taillenhöhe, hebt den Kopf zur Patientin oder zum Patienten und beginnt zu sprechen: Bei 1,6 s wechselt die Atmung ins Sprechmuster."),
    durationS: 5, fps: 30, loop: false, rootMotion: "none",
    keys: [
      { t: 0, pose: "reading-clipboard", ease: "smooth" },
      { t: 1.6, pose: "talking-to-patient", ease: "smooth" },
      { t: 3.4, pose: "talking-to-patient", ease: "smooth", bones: { head: { angleDeg: 4 } } },
      { t: 5, pose: "talking-to-patient", ease: "smooth" },
    ],
    contacts: feet(5),
    phases: [
      { id: "lowering", fromS: 0, toS: 1.6, title: t("Lowering the clipboard", "Klemmbrett senken") },
      { id: "talking", fromS: 1.6, toS: 5, title: t("Talking", "Sprechen") },
    ],
    events: [{ t: 1.6, kind: "motion", title: t("Eye contact", "Blickkontakt") }],
    tracks: [
      { target: "activityMet", ease: "smooth", keys: [{ t: 0, v: 1.3 }, { t: 1.6, v: 1.4 }, { t: 5, v: 1.4 }] },
      { target: "breathing", ease: "step", keys: [{ t: 0, v: 0 }, { t: 1.6, v: 1 }, { t: 5, v: 1 }] },
    ],
  },
];
bindRegions(a, () => ({ exposedFraction: 1, sunlitFraction: 0 }));
a.structure.dynamics = "quasi-static";
a.structure.supports = [];
//#endregion

//#region 📝️ identity, texts, symbols, provenance
Object.assign(a, DOCTOR_TEXT.identity);
a.structure.assumptions = DOCTOR_TEXT.structure;
a.thermal.assumptions = DOCTOR_TEXT.thermal(skin);
for (const s of ["500", "200", "100", "50"]) {
  a.representations[s].decisions = DOCTOR_TEXT.representation[s];
  a.volumes[s].decisions = DOCTOR_TEXT.volume[s];
}
a.symbols = DOCTOR_TEXT.symbols;
a.provenance = { ...a.provenance, author: "Semio staffage human worker HW-03", created: "2026-11-22", notes: DOCTOR_TEXT.notes };
references(a, DOCTOR_TEXT.references);
pruneMaterials(a);
for (const { solid } of solids(a)) if (solid.layer === undefined) delete solid.layer;
//#endregion

const limbs = retrace(a, new Set(["arm-l", "forearm-l", "hand-l", "arm-r", "forearm-r", "hand-r", "neck", "head"]), (shape, s) => [...(s === "500" && shape.id === "forearm-r" ? ["hand-r"] : []), ...a.volumes[s].body.filter((x: J) => x.id.startsWith(`${shape.id}-`)).map((x: J) => x.id)]);
console.error("[DEBUG] traced", limbs.length, retrace(a, new Set(["neck"]), () => [], ["elevation"]).length);
for (const side of ["l", "r"]) {
  const g = geo("50");
  const shank = g.elevation.find((x: J) => x.id === `shank-${side}`);
  const props = { id: `sock-${side}`, role: "body", filled: true, tone: 0.3, region: "shoes", material: "cotton-sock" };
  const el = traced(a, "50", "elevation", [`sock-${side}`], `shank-${side}`, { ...props, z: shank.z - 0.01 });
  if (!el) throw new Error("sock");
  g.elevation.push(el);
  for (const view of ["front", "plan"] as const) {
    const twin = g[view].find((x: J) => x.id === `shank-${side}`);
    const sh = traced(a, "50", view, [`sock-${side}`], `shank-${side}`, { ...props, z: twin.z - 0.01 }, twinDepth(a, el, view));
    if (sh) g[view].push(sh);
  }
}
console.error("[DEBUG] hulls", await repairPrisms(a));
await finishDimensions(a);
if (values.out) await Bun.write(values.out, `${JSON.stringify(doc, null, 2)}\n`);
else console.log(save(doc));
