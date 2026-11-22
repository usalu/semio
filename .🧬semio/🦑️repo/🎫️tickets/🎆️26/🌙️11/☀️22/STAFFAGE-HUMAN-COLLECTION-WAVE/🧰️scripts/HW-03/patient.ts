/** 🏥️ Generator of `patient-with-iv-pole`, derived from `walking-commuter` (rig, body, walk cycle with planted feet, knee-length coat cut): the overcoat becomes a knee-length patient gown over bare legs, the shoes felt slippers, scarf and bag go; the right hand pushes a five-castor IV stand with saline bag and infusion pump that rolls beside the patient, kept upright and on the floor by a revolute and a sliding joint solved for every pose and clip key; a slow walk and a rest at the pole.
 *  bun patient.ts [--out file.json]
 * @see ./lib.ts — the derivation helpers
 * @see ../../📋️brief.md — the brief */
import { parseArgs } from "node:util";
import { type J, S, bindRegions, dropBones, dropRegion, dropSolids, dropShapes, dubois, finishDimensions, load, pruneMaterials, references, regionOf, renameRegion, replaceMaterial, retime, save, shapeOf, shapes, solidOf, solids, t } from "./lib.ts";
import { posed, retrace, toRest, traced, twinDepth } from "./trace.ts";
import { PATIENT_TEXT } from "./patient-text.ts";

const ev = await import(`${S}/🔨️modules/🧮️evaluation/🟦️.ts`);
const { values } = parseArgs({ options: { out: { type: "string" } } });
const doc = await load("walking-commuter");
const a: J = doc.asset;
a.id = "patient-with-iv-pole";

//#region 🧹️ bag, scarf, coat trimmings
dropBones(a, ["bag"]);
dropRegion(a, "bag-leather");
for (const view of ["elevation", "front", "plan"]) Object.assign(shapeOf(a, "500", view, "scarf"), { id: "collar", region: "coat", material: "wool-coat" });
Object.assign(solidOf(a, "500", "body", "scarf"), { id: "collar", material: "wool-coat" });
const th500 = a.volumes["500"].thermal;
th500.splice(th500.findIndex((x: J) => x.id === "th-scarf"), 1, { id: "th-collar", of: "collar", region: "coat" });
for (const s of ["100", "50"]) {
  Object.assign(shapeOf(a, s, "front", "scarf-front"), { id: "neck-opening", region: "face-skin", material: "skin", tone: 0.78 });
  Object.assign(solidOf(a, s, "body", "scarf-front"), { id: "neck-opening", material: "skin", tone: 0.78 });
  for (const th of a.volumes[s].thermal) if (th.of === "scarf-front") Object.assign(th, { id: "th-neck-opening", of: "neck-opening", region: "face-skin" }), delete th.layer;
}
dropRegion(a, "scarf");
dropSolids(a, (x) => x.material === "wool-scarf");
const trim = (id: string): boolean => /^coat-(button|pocket)/.test(id) || /^(th-)?coat-(button|pocket)/.test(id);
dropShapes(a, (x) => trim(x.id));
dropSolids(a, (x) => trim(x.id) || (x.of !== undefined && trim(x.of)));
dropSolids(a, (x, _s, list) => list === "thermal" && ["tee", "shirt", "jacket", "briefs", "socks"].includes(x.layer));
for (const { solid } of solids(a)) {
  if (solid.layer === "overcoat") solid.layer = "gown";
  else if (solid.layer === "shoe") solid.layer = "slipper";
  else delete solid.layer;
}
a.symbols = a.symbols.map((s: J) => ({ ...s, id: "slow-walker-marker", title: t("Walking patient marker", "Gehende Patientin oder gehender Patient als Marke"), description: t("Position dot with a direction arrow for ward plans at 1:500 and smaller.", "Positionspunkt mit Richtungspfeil für Stationspläne ab 1:500.") }));
//#endregion

//#region 🧵️ gown, bare legs, slippers, grey hair
replaceMaterial(a, "wool-coat", { id: "gown-cotton", title: t("Cotton patient gown", "Baumwoll-Patientenhemd"), emissivity: 0.9, solarAbsorptivity: 0.35, specificHeatJKgK: 1300, source: "authored-estimate", reference: "ashrae-2021" });
replaceMaterial(a, "wool-trouser", { id: "leg-skin", title: t("Skin of the legs", "Haut der Beine"), emissivity: 0.98, solarAbsorptivity: 0.6, source: "literature", reference: "ashrae-2021" });
replaceMaterial(a, "leather-shoe", { id: "felt-slipper", title: t("Felt slipper with rubber sole", "Filzpantoffel mit Gummisohle"), emissivity: 0.9, solarAbsorptivity: 0.7, specificHeatJKgK: 1400, source: "authored-estimate", reference: "ashrae-2021" });
replaceMaterial(a, "cotton-sock", { id: "ankle-skin", title: t("Skin of the ankles", "Haut der Knöchel"), emissivity: 0.98, solarAbsorptivity: 0.6, source: "literature", reference: "ashrae-2021" });
const retone = (m: string, f: (x: number) => number): void => {
  for (const { shape } of shapes(a)) if (shape.material === m) (shape.tone = +f(shape.tone).toFixed(3)), delete shape.hatch;
  for (const { solid } of solids(a)) if (solid.material === m && solid.tone !== undefined) solid.tone = +f(solid.tone).toFixed(3);
};
retone("gown-cotton", (x) => Math.min(0.86, x + 0.45));
retone("leg-skin", () => 0.74);
retone("ankle-skin", () => 0.74);
retone("felt-slipper", (x) => Math.min(0.62, x + 0.25));
retone("hair", () => 0.8);
for (const s of ["100", "50"]) a.representations[s].hatches = a.representations[s].hatches.filter((h: J) => h.id !== "wool");
renameRegion(a, "coat", "gown");
renameRegion(a, "trousers", "legs");
renameRegion(a, "shoes", "slippers");
//#endregion

//#region ⚖️ body and thermal regions
const BODY_KG = 70, CLOTHES_KG = 0.8, STAND_KG = 6.3, BAG_KG = 1.05, PUMP_KG = 1.9, STATURE = 1.78;
const before = a.rig.bones.reduce((s: number, b: J) => s + (b.mass?.kg ?? 0), 0);
for (const b of a.rig.bones) if (b.mass) b.mass.kg = +((b.mass.kg * (BODY_KG + CLOTHES_KG)) / before).toFixed(4);
const skin = +dubois(BODY_KG, STATURE).toFixed(3);
Object.assign(a.thermal.body, { massKg: BODY_KG, skinAreaM2: skin, basalMetabolismW: 66, coreSetPointC: 36.9, skinSetPointC: 34.0, maxSkinBloodFlowLM2H: 70 });
a.thermal.environment = { airTempC: 23, meanRadiantTempC: 23, skyTempC: 23, groundTempC: 22, relativeHumidity: 0.4, windMS: 0.1, windDirectionDeg: 180, solarWM2: 0, diffuseFraction: 1, pressurePa: 101325 };
a.thermal.referenceEnvironment = "office-neutral";
a.thermal.baseMet = 1.1;
const R = (clo: number): number => +(clo * 0.155 * 1.1).toFixed(5);
const RV = (r: number): number => +(r / (0.0165 * 0.38)).toFixed(1);
const face = regionOf(a, "face-skin");
Object.assign(face.body, { metabolicShare: 0.07, skinFraction: 0.071 });
face.areaM2 = 0.146;
face.title = t("Face, neck and neckline skin", "Haut von Gesicht, Hals und Ausschnitt");
const gown = regionOf(a, "gown");
Object.assign(gown, { title: t("Patient gown on torso, arms and thighs", "Patientenhemd an Rumpf, Armen und Oberschenkeln"), material: "gown-cotton", heatCapacityJM2K: 13000 + 260 });
gown.layers = [{ id: "gown", title: t("Knee-length cotton patient gown, tied at the back", "Knielanges Baumwoll-Patientenhemd, hinten gebunden"), kind: "clothing", material: "gown-cotton", thicknessM: 0.0012, resistanceM2KW: R(0.3), vapourResistanceM2PaW: RV(R(0.3)), heatCapacityJM2K: 260, source: "authored-estimate", reference: "iso-9920-2007" }];
const legs = regionOf(a, "legs");
Object.assign(legs, { title: t("Bare legs below the gown", "Nackte Beine unter dem Hemd"), kind: "skin", material: "leg-skin", layers: [], heatCapacityJM2K: 13000, skyViewFactor: 0.1 });
const sl = regionOf(a, "slippers");
Object.assign(sl, { title: t("Felt slippers on bare feet", "Filzpantoffeln an nackten Füßen"), material: "felt-slipper", heatCapacityJM2K: 13000 + 700 });
sl.layers = [{ id: "slipper", title: t("Felt slipper, rubber sole", "Filzpantoffel, Gummisohle"), kind: "clothing", material: "felt-slipper", thicknessM: 0.006, resistanceM2KW: R(0.05), vapourResistanceM2PaW: RV(R(0.05)), heatCapacityJM2K: 700, source: "authored-estimate", reference: "iso-9920-2007" }];
regionOf(a, "hair").title = t("Short grey hair", "Kurzes graues Haar");
a.materials.push(
  { id: "chromed-steel", title: t("Chromed steel tube", "Verchromtes Stahlrohr"), emissivity: 0.1, solarAbsorptivity: 0.4, densityKgM3: 7850, conductivityWmK: 45, specificHeatJKgK: 480, source: "literature", reference: "incropera-2007" },
  { id: "castor-polymer", title: t("Polyamide castor and base cover", "Polyamid-Lenkrolle und Fußabdeckung"), emissivity: 0.9, solarAbsorptivity: 0.8, densityKgM3: 1140, source: "authored-estimate", reference: "incropera-2007" },
  { id: "saline", title: t("Saline in a PVC infusion bag", "Kochsalzlösung im PVC-Infusionsbeutel"), emissivity: 0.95, solarAbsorptivity: 0.1, densityKgM3: 1005, conductivityWmK: 0.6, specificHeatJKgK: 4180, source: "literature", reference: "incropera-2007" },
  { id: "pump-housing", title: t("ABS pump housing", "ABS-Pumpengehäuse"), emissivity: 0.9, solarAbsorptivity: 0.4, densityKgM3: 1050, source: "authored-estimate", reference: "incropera-2007" },
);
a.thermal.operating = [{ id: "infusion-pump", title: t("Volumetric infusion pump", "Volumetrische Infusionspumpe"), levels: [{ v: 0, title: t("Off", "Aus") }, { v: 1, title: t("Infusing", "Infundiert") }] }];
a.thermal.regions.push(
  { id: "iv-stand", title: t("IV stand with saline bag", "Infusionsständer mit Kochsalzbeutel"), kind: "surface", model: "passive", material: "chromed-steel", areaM2: 0.42, characteristicLengthM: 0.025, geometry: "cylinder", skyViewFactor: 0.4, layers: [], heatCapacityJM2K: 9000, ratedPowerW: 0, ratedPowerBasis: "passive", source: "authored-estimate", reference: "incropera-2007" },
  { id: "infusion-pump", title: t("Infusion pump on the pole", "Infusionspumpe am Ständer"), kind: "surface", model: "operating", material: "pump-housing", areaM2: 0.12, characteristicLengthM: 0.18, geometry: "vertical-plate", skyViewFactor: 0.5, layers: [], heatCapacityJM2K: 16000, operating: { key: "infusion-pump", heatW: 4 }, ratedPowerW: 15, ratedPowerBasis: "electrical-input", source: "authored-estimate", reference: "iec-60601-2-24-2012" },
);
//#endregion

//#region 🦯️ IV stand rig: a revolute grip that keeps the stand upright and a sliding pole that keeps its castors on the floor
const hr = a.rig.bones.findIndex((b: J) => b.id === "hand-r");
const HANDLE = 0.2;
a.rig.bones.splice(hr + 1, 0,
  { id: "iv-grip", parent: "hand-r", title: t("Push handle of the IV stand", "Schiebegriff des Infusionsständers"), x: 0.07, y: 0, angleDeg: 180, lengthM: 0.05, joint: { type: "revolute", minDeg: -90, maxDeg: 90 } },
  { id: "iv-pole", parent: "iv-grip", title: t("IV pole (slides in its handle clamp)", "Infusionsstange (gleitet in der Griffklemme)"), x: 0, y: 0, z: -HANDLE, angleDeg: 0, lengthM: 1.0, joint: { type: "prismatic", axisDeg: 0, minM: -0.35, maxM: 0.35 } },
);
const BASE = -1.0, LEG_R = 0.25;
const legDirs = [0, 1, 2, 3, 4].map((k) => (Math.PI / 180) * (90 + 72 * k));
const part = (s: string, id: string, kind: string, geo: J, material: string, tone: number, extra: J = {}): J => ({ id, kind, bone: id === "iv-handle" || id === "iv-clamp" ? "iv-grip" : "iv-pole", material, role: "body", tone, ...geo, ...extra });
const ivSolids = (s: string): J[] => {
  const legR = s === "200" ? 0.02 : 0.015;
  const out: J[] = [part(s, "iv-pole-tube", "capsule", { p0: [BASE + 0.05, 0, 0], p1: [1.01, 0, 0], r0: 0.0125, r1: 0.0125 }, "chromed-steel", 0.72, { slender: true })];
  if (s === "500") out.push(part(s, "iv-base", "ellipsoid", { radii: [0.05, LEG_R + 0.013, 0.238], transform: { translation: [BASE + 0.05, 0, 0.024] } }, "castor-polymer", 0.3));
  else {
    out.push(part(s, "iv-hub", "capsule", { p0: [BASE + 0.035, 0, 0], p1: [BASE + 0.095, 0, 0], r0: 0.03, r1: 0.03 }, "castor-polymer", 0.3));
    legDirs.forEach((f, k) => {
      const tip = [BASE + 0.045, +(LEG_R * Math.cos(f)).toFixed(4), +(LEG_R * Math.sin(f)).toFixed(4)];
      out.push(part(s, `iv-leg-${k + 1}`, "capsule", { p0: [BASE + 0.065, +(0.025 * Math.cos(f)).toFixed(4), +(0.025 * Math.sin(f)).toFixed(4)], p1: tip, r0: legR, r1: legR }, "chromed-steel", 0.6));
      out.push(part(s, `iv-castor-${k + 1}`, "ellipsoid", { radii: [0.025, 0.025, 0.012 + (s === "200" ? 0.004 : 0)], transform: { translation: [BASE + 0.025, tip[1], tip[2]] } }, "castor-polymer", 0.25));
    });
  }
  if (s !== "500") out.push(part(s, "iv-crossbar", "capsule", { p0: [1.0, 0, -0.13], p1: [1.0, 0, 0.13], r0: 0.006, r1: 0.006 }, "chromed-steel", 0.72, { slender: true }));
  if (s !== "500") out.push(part(s, "iv-bag", "box", { size: [0.2, s === "200" ? 0.04 : 0.035, 0.12], cornerRadius: 0.01, transform: { translation: [0.895, 0, 0.11] } }, "saline", 0.92));
  out.push(part(s, "iv-pump", "box", { size: [0.18, 0.1, 0.14], cornerRadius: 0.01, transform: { translation: [0.24, -0.062, 0] } }, "pump-housing", 0.82));
  const hR = s === "500" ? 0.0375 : s === "200" ? 0.02 : 0.011;
  out.push(part(s, "iv-handle", "capsule", { p0: [0, 0, 0.02], p1: [0, 0, -HANDLE], r0: hR, r1: hR }, "chromed-steel", 0.6));
  if (s === "50" || s === "100") out.push(part(s, "iv-clamp", "capsule", { p0: [-0.03, 0, -HANDLE], p1: [0.03, 0, -HANDLE], r0: 0.022, r1: 0.022 }, "chromed-steel", 0.5));
  return out;
};
for (const s of ["500", "200", "100", "50"]) {
  const v = a.volumes[s];
  for (const solid of ivSolids(s)) {
    v.body.push(solid);
    v.thermal.push({ id: `th-${solid.id}`, of: solid.id, region: solid.id === "iv-pump" ? "infusion-pump" : "iv-stand" });
  }
  if (s === "200") solidOf(a, s, "body", "hand-r").transform.translation[0] = 0.048;
  v.structure.push({ id: "member-iv-pole", kind: "capsule", bone: "iv-pole", material: "chromed-steel", role: "body", tone: 0.5, slender: true, p0: [BASE + 0.0125, 0, 0], p1: [1.0, 0, 0], r0: 0.0125, r1: 0.0125, member: "iv-pole", part: "member" });
}
//#endregion

//#region 🧍️ poses: right hand on the handle, slight stoop; IV joints solved per pose and key
const HOLD = { "arm-r": { angleDeg: 9 }, "forearm-r": { angleDeg: 62 }, "hand-r": { angleDeg: -25 } };
const STOOP = { lumbar: -1.5, torso: -2, neck: 4, head: -4 };
const HEAD = { neck: 4, head: -4 };
a.poses = a.poses.filter((p: J) => p.id !== "check-watch");
for (const p of a.poses) {
  Object.assign(p.bones, structuredClone(HOLD));
  for (const [b, d] of Object.entries(p.id === "standing" ? HEAD : STOOP)) p.bones[b] = { angleDeg: +((p.bones[b]?.angleDeg ?? 0) + d).toFixed(3) };
  p.loads = [{ id: "push-handle", kind: "equipment", title: t("Hand on the IV stand handle", "Hand am Griff des Infusionsständers"), bone: "iv-grip", x: 0, y: 0, fxN: 0, fyN: -6, source: "authored-estimate" }];
}
const standing = a.poses.find((p: J) => p.id === "standing");
Object.assign(standing, { id: "standing-with-pole", title: t("Standing at the IV stand", "Stehend am Infusionsständer"), description: t("Standing slightly stooped beside the IV stand, the right hand on its push handle, the left arm hanging.", "Leicht gebeugt neben dem Infusionsständer stehend, die rechte Hand am Schiebegriff, der linke Arm hängt.") });
standing.bones["arm-l"] = { angleDeg: 3 };
standing.bones["forearm-l"] = { angleDeg: 12 };
const titles: Record<string, J> = {
  "contact-left": [t("Left heel strike with the stand", "Linker Fersenkontakt mit Ständer"), t("Slow walk: the left heel touches down while the right hand pushes the stand ahead.", "Langsames Gehen: Die linke Ferse setzt auf, während die rechte Hand den Ständer voranschiebt.")],
  "passing-right": [t("Right mid-stance with the stand", "Rechte Standbeinmitte mit Ständer"), t("Slow walk: single support on the right leg, the stand rolling beside it.", "Langsames Gehen: Einbeinstand rechts, der Ständer rollt daneben.")],
};
for (const [id, [title, description]] of Object.entries(titles)) Object.assign(a.poses.find((p: J) => p.id === id), { title, description });
const walk = a.clips.find((c: J) => c.id === "walk-cycle");
const idle = a.clips.find((c: J) => c.id === "idle-breathing");
a.clips = [walk, idle];
walk.id = "slow-walk";
retime(walk, 1.7 / 1.2);
walk.durationS = 1.7;
for (const k of walk.keys) if (k.bones) {
  Object.assign(k.bones, structuredClone(HOLD));
  for (const [b, d] of Object.entries(STOOP)) if (k.bones[b]) k.bones[b] = { angleDeg: +(k.bones[b].angleDeg + d).toFixed(3) };
}
for (const k of idle.keys) {
  k.pose = "standing-with-pole";
  if (k.bones) for (const [b, d] of Object.entries(HEAD)) if (k.bones[b]) k.bones[b] = { angleDeg: +(k.bones[b].angleDeg + d).toFixed(3) };
}
a.presentation = { defaultPose: "standing-with-pole", defaultClip: "slow-walk", defaultScale: 100 };
const solveIv = (query: J, bones: J): J => {
  const get = (): J => ev.evaluate(a, query).bones;
  for (let it = 0; it < 4; it++) {
    const f = get();
    const u = f["iv-grip"].frame.u;
    const err = (Math.atan2(u.x, u.y) * 180) / Math.PI;
    bones["iv-grip"] = { angleDeg: +((bones["iv-grip"]?.angleDeg ?? 0) + err).toFixed(4) };
    const g = get();
    const base = g["iv-pole"].frame.o.y + BASE * g["iv-pole"].frame.u.y;
    bones["iv-pole"] = { dx: +((bones["iv-pole"]?.dx ?? 0) - base).toFixed(5) };
  }
  return bones;
};
for (const p of a.poses) solveIv({ kind: "pose", pose: p.id }, p.bones);
for (const c of [walk, idle]) for (const k of c.keys) {
  k.bones ??= {};
  const pose = a.poses.find((p: J) => p.id === k.pose);
  k.bones["iv-grip"] ??= structuredClone(pose.bones["iv-grip"]);
  k.bones["iv-pole"] ??= structuredClone(pose.bones["iv-pole"]);
  if (k.t < c.durationS) solveIv({ kind: "clip", clip: c.id, timeS: k.t }, k.bones);
}
for (const c of [walk, idle]) {
  const first = c.keys[0], last = c.keys[c.keys.length - 1];
  if (last.t === c.durationS) (last.bones["iv-grip"] = structuredClone(first.bones["iv-grip"])), (last.bones["iv-pole"] = structuredClone(first.bones["iv-pole"]));
}
//#endregion

//#region ✏️ the stand drawn from its volume at every scale; thin parts as strokes
const geo = (s: string): J => a.representations[s].geometry;
const zTop = (list: J[]): number => Math.max(0, ...list.map((x) => x.z)) + 0.2;
const draw = (s: string, id: string, ids: string[], region: string, material: string, tone: number, bone = "iv-pole"): void => {
  const g = geo(s);
  const props = { id, role: "body", filled: true, tone, region, material };
  const el = traced(a, s, "elevation", ids, bone, { ...props, z: zTop(g.elevation) });
  if (!el) throw new Error(`${s} ${id}`);
  g.elevation.push(el);
  for (const view of ["front", "plan"] as const) {
    const sh = traced(a, s, view, ids, bone, { ...props, z: zTop(g[view]) }, twinDepth(a, el, view));
    if (sh) g[view].push(sh);
  }
};
const strokeLine = (s: string, id: string, from: number[], to: number[], region: string | string[], pen: number): void => {
  const g = geo(s);
  const reg = Array.isArray(region) ? { regions: region } : { region };
  g.elevation.push({ id, bone: "iv-pole", role: "body", z: zTop(g.elevation), filled: false, tone: 0.7, ...reg, material: "chromed-steel", kind: "poly", closed: false, points: [[from[0], from[1]], [to[0], to[1]]], penMm: pen });
};
for (const s of ["500", "200", "100", "50"]) {
  const thinPole = s === "500" || s === "200";
  if (thinPole) {
    strokeLine(s, "iv-pole-tube", [BASE + 0.08, 0], [1.01, 0], "iv-stand", 0.13);
    const { state } = posed(a, s);
    const f = state.bones["iv-pole"].frame;
    const at = (u: number): [number, number, number] => [f.o.x + u * f.u.x, f.o.y + u * f.u.y, f.o.z + u * f.u.z];
    const el = geo(s).elevation.find((x: J) => x.id === "iv-pole-tube");
    const front = toRest(a, state, "iv-pole", "front", [BASE + 0.08, 1.01].map((u) => [at(u)[2], at(u)[1]] as [number, number]), twinDepth(a, el, "front"));
    geo(s).front.push({ ...el, z: zTop(geo(s).front), points: front });
    const top = toRest(a, state, "iv-pole", "plan", [[at(1)[0], at(1)[2]]], twinDepth(a, el, "plan"))![0]!;
    const r = s === "500" ? 0.05 : 0.02;
    geo(s).plan.push({ id: "iv-pole-tube", bone: "iv-pole", role: "body", z: zTop(geo(s).plan), filled: true, tone: 0.7, region: "iv-stand", material: "chromed-steel", kind: "poly", closed: true, points: Array.from({ length: 12 }, (_, i) => [+(top[0]! + r * Math.cos((i * Math.PI) / 6)).toFixed(4), +(top[1]! + r * Math.sin((i * Math.PI) / 6)).toFixed(4)]) });
  } else draw(s, "iv-pole-tube", ["iv-pole-tube"], "iv-stand", "chromed-steel", 0.72);
  draw(s, "iv-base", s === "500" ? ["iv-base"] : ["iv-hub", ...[1, 2, 3, 4, 5].flatMap((k) => [`iv-leg-${k}`, `iv-castor-${k}`])], "iv-stand", "castor-polymer", 0.3);
  if (s !== "500") draw(s, "iv-bag", ["iv-bag"], "iv-stand", "saline", 0.92);
  if (s === "50") draw(s, "iv-crossbar", ["iv-crossbar"], "iv-stand", "chromed-steel", 0.72);
  else if (s !== "500") {
    strokeLine(s, "iv-crossbar", [1.0, -0.006], [1.0, 0.006], "iv-stand", 0.1);
    const { state } = posed(a, s);
    const f = state.bones["iv-pole"].frame;
    const at = (w: number): number[] => [f.o.x + 1.0 * f.u.x + w * f.w.x, f.o.y + 1.0 * f.u.y + w * f.w.y, f.o.z + 1.0 * f.u.z + w * f.w.z];
    const el = geo(s).elevation.find((x: J) => x.id === "iv-crossbar");
    for (const view of ["front", "plan"] as const) {
      const pts = [-0.13, 0.13].map((w) => (view === "front" ? [at(w)[2]!, at(w)[1]!] : [at(w)[0]!, at(w)[2]!]) as [number, number]);
      geo(s)[view].push({ ...el, z: zTop(geo(s)[view]), points: toRest(a, state, "iv-pole", view, pts, twinDepth(a, el, view)) });
    }
  }
  draw(s, "iv-pump", ["iv-pump"], "infusion-pump", "pump-housing", 0.82);
  draw(s, "iv-handle", s === "50" || s === "100" ? ["iv-handle", "iv-clamp"] : ["iv-handle"], "iv-stand", "chromed-steel", 0.6, "iv-grip");
}
//#endregion

//#region 🫁️ breathing, activity, bindings
const rest = a.thermal.respiration.patterns.find((p: J) => p.id === "rest-standing");
a.thermal.respiration.patterns = [
  { ...rest, rateBpm: 15, title: t("Quiet standing", "Ruhiges Stehen") },
  { id: "walking", title: t("Slow walk at 2.2 km/h", "Langsames Gehen mit 2,2 km/h"), rateBpm: 17, tidalVolumeL: 0.8, inhaleFraction: 0.42, pauseAfterInhaleFraction: 0.03, exhaleFraction: 0.5, pauseAfterExhaleFraction: 0.05, route: "nose", source: "authored-estimate", reference: "icrp-66-1994" },
];
for (const p of a.poses) {
  const walking = p.thermal.breathing === "walking";
  p.thermal.activityMet = walking ? 2.0 : 1.2;
  p.thermal.operating = { "infusion-pump": 1 };
  p.thermal.regions = {};
}
for (const tr of walk.tracks) if (tr.target === "activityMet") for (const k of tr.keys) k.v = 2.0;
bindRegions(a, (p, r) => ({ exposedFraction: r === "gown" ? 0.9 : r === "legs" ? 0.8 : 1, sunlitFraction: 0 }));
walk.title = t("Slow walk with the IV stand", "Langsames Gehen mit dem Infusionsständer");
walk.description = t("A slow heel-to-toe walk of 1.7 s per cycle (0.71 m/s, 71 steps per minute) with planted feet; the right hand pushes the stand, which stays upright and rolls on the floor beside the right foot, the left arm swings a little.", "Langsamer Fersen-Zehen-Gang mit 1,7 s je Zyklus (0,71 m/s, 71 Schritte je Minute) mit aufgesetzten Füßen; die rechte Hand schiebt den Ständer, der aufrecht bleibt und neben dem rechten Fuß über den Boden rollt, der linke Arm schwingt leicht mit.");
idle.title = t("Resting at the IV stand", "Pause am Infusionsständer");
idle.description = t("Standing at the stand with the quiet breathing pattern (15 per minute); the chest rises by the derived tidal motion, the shoulders follow by 1.5 degrees.", "Stehen am Ständer mit ruhigem Atemmuster (15 je Minute); die Brust hebt sich mit der abgeleiteten Atembewegung, die Schultern folgen um 1,5 Grad.");
idle.phases = [{ id: "rest", fromS: 0, toS: idle.durationS, title: t("Resting in the corridor", "Pause im Flur") }];
//#endregion

//#region ✒️ traced outlines for the default pose, ankle gaps, texts
retrace(a, new Set(["arm-r", "forearm-r", "hand-r", "neck", "head"]), (shape, s) => [...(s === "500" && shape.id === "forearm-r" ? ["hand-r"] : []), ...a.volumes[s].body.filter((x: J) => x.id.startsWith(`${shape.id}-`) && !x.id.startsWith("coat-")).map((x: J) => x.id)]);
retrace(a, new Set(["neck"]), () => [], ["elevation"]);
for (const side of ["l", "r"]) {
  const g = geo("50");
  const shank = g.elevation.find((x: J) => x.id === `shank-${side}`);
  const props = { id: `ankle-${side}`, role: "body", filled: true, tone: 0.74, region: "legs", material: "ankle-skin" };
  const el = traced(a, "50", "elevation", [`sock-${side}`], `shank-${side}`, { ...props, z: shank.z - 0.01 });
  g.elevation.push(el);
  for (const view of ["front", "plan"] as const) {
    const twin = g[view].find((x: J) => x.id === `shank-${side}`);
    const sh = traced(a, "50", view, [`sock-${side}`], `shank-${side}`, { ...props, z: twin.z - 0.01 }, twinDepth(a, el, view));
    if (sh) g[view].push(sh);
  }
}
Object.assign(a, PATIENT_TEXT.identity);
a.structure.dynamics = "quasi-static";
a.structure.assumptions = PATIENT_TEXT.structure;
a.thermal.assumptions = PATIENT_TEXT.thermal(skin);
for (const s of ["500", "200", "100", "50"]) {
  a.representations[s].decisions = PATIENT_TEXT.representation[s];
  a.volumes[s].decisions = PATIENT_TEXT.volume[s];
}
a.provenance = { ...a.provenance, author: "Semio staffage human worker HW-03", created: "2026-11-22", notes: PATIENT_TEXT.notes };
references(a, PATIENT_TEXT.references);
pruneMaterials(a);
//#endregion

await finishDimensions(a);
a.dimensions.massKg = +(a.dimensions.massKg + STAND_KG + BAG_KG + PUMP_KG).toFixed(2);
if (values.out) await Bun.write(values.out, `${JSON.stringify(doc, null, 2)}\n`);
else console.log(save(doc));
