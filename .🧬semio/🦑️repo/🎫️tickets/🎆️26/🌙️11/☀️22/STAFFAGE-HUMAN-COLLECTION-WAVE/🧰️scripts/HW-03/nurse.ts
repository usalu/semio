/** 🩺️ Generator of `nurse-walking`, derived from `walking-commuter` (rig, body, planted-foot walk cycle): the knee-length overcoat is cut to a hip-length scrub tunic (lofts trimmed at the hem, outlines re-traced from the trimmed volume), sleeves end above the elbow over bare forearms ("bare below the elbows"), the trousers become scrub trousers and the shoes clogs; scarf, bag, lapels, buttons and the coat's back vent go; both arms swing and the cycle is shortened to 1.0 s for a brisk ward walk.
 *  bun nurse.ts [--out file.json], then python3 ../FIN-A/fit_dims.py nurse-walking --write (the 1:500 figure is 38 mm taller than the finer scales: the dimensions are fitted to the volume and drawing guards of every scale)
 * @see ./lib.ts — the derivation helpers
 * @see ../../📋️brief.md — the brief */
import { parseArgs } from "node:util";
import { type J, bindRegions, dropBones, dropRegion, dropShapes, dropSolids, dubois, finishDimensions, load, mergeRegion, pruneMaterials, references, regionOf, renameRegion, replaceMaterial, retime, save, shapeOf, shapes, solidOf, solids, t, thinBone, trimLoft } from "./lib.ts";
import { retrace, traced, twinDepth } from "./trace.ts";
import { NURSE_TEXT } from "./nurse-text.ts";

const { values } = parseArgs({ options: { out: { type: "string" } } });
const doc = await load("walking-commuter");
const a: J = doc.asset;
a.id = "nurse-walking";

//#region 🧹️ bag, scarf, lapels, buttons, vent, front edge, glance pose
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
const gone = (id: string): boolean => /^(th-)?(coat-button|lapel|coat-back-vent|coat-side-seam|coat-front-edge|shirt-cuff)/.test(id);
dropShapes(a, (x) => gone(x.id));
dropSolids(a, (x) => gone(x.id) || (x.of !== undefined && gone(x.of)));
dropSolids(a, (x, _s, list) => list === "thermal" && ["shirt", "jacket"].includes(x.layer));
dropSolids(a, (x, _s, list) => list === "thermal" && /^forearm-/.test(x.bone ?? "") && x.layer !== undefined && x.of === undefined);
a.poses = a.poses.filter((p: J) => p.id !== "check-watch");
a.clips = a.clips.filter((c: J) => c.id !== "glance-at-watch");
//#endregion

//#region ✂️ the overcoat cut to a scrub tunic at the hem; bare forearms
const HEM = 0.765;
const pelvisY = 0.959, thighY = 0.959;
for (const { solid, list } of solids(a)) {
  if (solid.material !== "wool-coat" || solid.kind !== "loft") continue;
  const ok = solid.bone === "pelvis" ? trimLoft(solid, HEM - pelvisY, (u) => u >= HEM - pelvisY) : /^thigh-/.test(solid.bone) ? trimLoft(solid, thighY - HEM, (u) => u <= thighY - HEM) : true;
  if (!ok) list.splice(list.indexOf(solid), 1);
}
for (const side of ["l", "r"]) {
  thinBone(a, `forearm-${side}`, 0.72, (id) => !id.startsWith("sk-") && !id.startsWith("joint-") && id !== `ulna-${side}` && id !== `radius-${side}`);
  for (const { shape } of shapes(a)) if (shape.bone === `forearm-${side}` && shape.region === "coat") Object.assign(shape, { region: "hand-skin", material: "skin", tone: 0.76 });
  for (const { solid } of solids(a)) {
    if (solid.bone === `forearm-${side}` && solid.material === "wool-coat") Object.assign(solid, { material: "skin", tone: 0.76 });
    if (solid.of === `forearm-${side}`) (solid.region = "hand-skin"), delete solid.layer;
  }
}
//#endregion

//#region 🧵️ scrubs, clogs
replaceMaterial(a, "wool-coat", { id: "scrub-twill", title: t("Polyester-cotton scrub twill", "Polyester-Baumwoll-Köper der Bereichskleidung"), emissivity: 0.9, solarAbsorptivity: 0.7, specificHeatJKgK: 1300, source: "authored-estimate", reference: "ashrae-2021" });
replaceMaterial(a, "wool-trouser", { id: "scrub-trouser", title: t("Polyester-cotton scrub trousers", "Bereichshose aus Polyester-Baumwolle"), emissivity: 0.9, solarAbsorptivity: 0.7, specificHeatJKgK: 1300, source: "authored-estimate", reference: "ashrae-2021" });
replaceMaterial(a, "leather-shoe", { id: "clog-eva", title: t("Closed EVA clog", "Geschlossener EVA-Clog"), emissivity: 0.9, solarAbsorptivity: 0.3, specificHeatJKgK: 1600, source: "authored-estimate", reference: "ashrae-2021" });
replaceMaterial(a, "cotton-sock", { id: "compression-sock", title: t("Knee-high compression sock", "Kniestrumpf mit Kompression"), emissivity: 0.93, solarAbsorptivity: 0.6, specificHeatJKgK: 1300, source: "authored-estimate", reference: "ashrae-2021" });
const retone = (m: string, f: (x: number) => number): void => {
  for (const { shape } of shapes(a)) if (shape.material === m) (shape.tone = +f(shape.tone).toFixed(3)), delete shape.hatch;
  for (const { solid } of solids(a)) if (solid.material === m && solid.tone !== undefined) solid.tone = +f(solid.tone).toFixed(3);
};
retone("scrub-twill", (x) => Math.min(0.62, x + 0.22));
retone("scrub-trouser", (x) => Math.min(0.6, x + 0.16));
retone("clog-eva", () => 0.86);
for (const s of ["100", "50"]) a.representations[s].hatches = a.representations[s].hatches.filter((h: J) => h.id !== "wool");
for (const { shape } of shapes(a)) if (shape.hatch === "wool") delete shape.hatch;
renameRegion(a, "coat", "scrub-top");
renameRegion(a, "trousers", "scrub-trousers");
renameRegion(a, "shoes", "clogs");
for (const { solid } of solids(a)) {
  if (solid.layer === "overcoat") solid.layer = "scrub-top";
  else if (solid.layer === "trousers") solid.layer = "scrub-trousers";
  else if (solid.layer === "shoe") solid.layer = "clog";
}
//#endregion

//#region ⚖️ body and thermal regions
const BODY_KG = 76, CLOTHES_KG = 1.5, STATURE = 1.78;
const before = a.rig.bones.reduce((s: number, b: J) => s + (b.mass?.kg ?? 0), 0);
for (const b of a.rig.bones) if (b.mass) b.mass.kg = +((b.mass.kg * (BODY_KG + CLOTHES_KG)) / before).toFixed(4);
const skin = +dubois(BODY_KG, STATURE).toFixed(3);
Object.assign(a.thermal.body, { massKg: BODY_KG, skinAreaM2: skin, basalMetabolismW: 82 });
a.thermal.environment = { airTempC: 22, meanRadiantTempC: 22, skyTempC: 22, groundTempC: 22, relativeHumidity: 0.4, windMS: 0.1, windDirectionDeg: 180, solarWM2: 0, diffuseFraction: 1, pressurePa: 101325 };
a.thermal.referenceEnvironment = "office-neutral";
a.thermal.baseMet = 1.4;
const R = (clo: number): number => +(clo * 0.155 * 1.1).toFixed(5);
const RV = (r: number): number => +(r / (0.0165 * 0.38)).toFixed(1);
const face = regionOf(a, "face-skin");
Object.assign(face.body, { metabolicShare: 0.07, skinFraction: 0.071 });
face.title = t("Face, neck and V-neck skin", "Haut von Gesicht, Hals und V-Ausschnitt");
const top = regionOf(a, "scrub-top");
Object.assign(top, { title: t("Scrub tunic over an undershirt on torso and upper arms", "Kasack über einem Unterhemd an Rumpf und Oberarmen"), material: "scrub-twill" });
top.layers = [
  { ...top.layers[0], title: t("Cotton undershirt, short sleeve", "Baumwollunterhemd, kurzarm"), resistanceM2KW: R(0.08), vapourResistanceM2PaW: RV(R(0.08)) },
  { id: "scrub-top", title: t("Short-sleeved V-neck scrub tunic", "Kurzärmeliger Kasack mit V-Ausschnitt"), kind: "clothing", material: "scrub-twill", thicknessM: 0.006, resistanceM2KW: R(0.17), vapourResistanceM2PaW: RV(R(0.17)), heatCapacityJM2K: 300, source: "authored-estimate", reference: "iso-9920-2007" },
];
top.heatCapacityJM2K = 13000 + 180 + 300;
top.body.skinFraction = 0.42;
top.body.metabolicShare = +(top.body.metabolicShare - 0.03).toFixed(4);
const hands = regionOf(a, "hand-skin");
Object.assign(hands, { title: t("Bare forearms and hands", "Nackte Unterarme und Hände"), areaM2: 0.2 });
hands.body.skinFraction = +(hands.body.skinFraction + 0.07).toFixed(4);
hands.body.metabolicShare = +(hands.body.metabolicShare + 0.03).toFixed(4);
const tr = regionOf(a, "scrub-trousers");
tr.title = t("Scrub trousers over the legs", "Bereichshose über den Beinen");
tr.layers[1] = { ...tr.layers[1], id: "scrub-trousers", title: t("Scrub trousers with drawstring", "Bereichshose mit Kordelzug"), material: "scrub-trouser", resistanceM2KW: R(0.15), vapourResistanceM2PaW: RV(R(0.15)), heatCapacityJM2K: 300 };
const cl = regionOf(a, "clogs");
cl.title = t("Compression socks and clogs", "Kompressionsstrümpfe und Clogs");
cl.layers = [{ ...cl.layers[0], title: t("Knee-high compression socks", "Kniestrümpfe mit Kompression"), material: "compression-sock" }, { ...cl.layers[1], id: "clog", title: t("Closed EVA clog", "Geschlossener EVA-Clog"), material: "clog-eva", resistanceM2KW: R(0.04), vapourResistanceM2PaW: RV(R(0.04)) }];
void mergeRegion;
//#endregion

//#region 🚶️ brisk walk: 1.0 s cycle, both arms swing in counter-phase
const walk = a.clips.find((c: J) => c.id === "walk-cycle");
walk.id = "brisk-walk";
retime(walk, 1.0 / 1.2);
const arm = (b: J): J => ({ arm: b["arm-l"], fore: b["forearm-l"], hand: b["hand-l"] });
const keys = walk.keys.filter((k: J) => k.bones);
const at = (time: number): J => {
  const tt = ((time % walk.durationS) + walk.durationS) % walk.durationS;
  return keys.reduce((best: J, k: J) => (Math.abs(k.t - tt) < Math.abs(best.t - tt) ? k : best));
};
const swing = keys.map((k: J) => arm(at(k.t + walk.durationS / 2).bones));
keys.forEach((k: J, i: number) => Object.assign(k.bones, { "arm-r": structuredClone(swing[i].arm), "forearm-r": structuredClone(swing[i].fore), "hand-r": structuredClone(swing[i].hand) }));
const opposite: Record<string, string> = { "contact-left": "contact-right", "loading-left": "loading-right", "passing-left": "passing-right", "contact-right": "contact-left", "loading-right": "loading-left", "passing-right": "passing-left" };
const left = Object.fromEntries(a.poses.map((p: J) => [p.id, arm(p.bones)]));
for (const p of a.poses) {
  const o = opposite[p.id];
  const src = o ? left[o] : { arm: { angleDeg: -3 }, fore: { angleDeg: 10 }, hand: { angleDeg: 4 } };
  Object.assign(p.bones, { "arm-r": structuredClone(src.arm), "forearm-r": structuredClone(src.fore), "hand-r": structuredClone(src.hand) });
  p.loads = [];
}
const standing = a.poses.find((p: J) => p.id === "standing");
Object.assign(standing, { title: t("Standing at the nurses' station", "Stehend am Stationstresen"), description: t("Standing upright between two errands, both arms relaxed.", "Aufrecht stehend zwischen zwei Wegen, beide Arme entspannt.") });
standing.bones["arm-l"] = { angleDeg: 3 };
standing.bones["forearm-l"] = { angleDeg: 10 };
for (const k of walk.keys) if (k.bones) for (const b of ["neck", "head"]) if (k.bones[b]) k.bones[b] = { angleDeg: +(k.bones[b].angleDeg - 2).toFixed(3) };
a.presentation = { defaultPose: "contact-left", defaultClip: "brisk-walk", defaultScale: 100 };
for (const [id, title, description] of [
  ["contact-left", t("Brisk stride, left heel strike", "Zügiger Schritt, linker Fersenkontakt"), t("Brisk ward walk: the left heel strikes while the right arm swings forward and the left arm back.", "Zügiger Gang über die Station: Die linke Ferse setzt auf, der rechte Arm schwingt vor, der linke zurück.")],
  ["passing-right", t("Brisk stride, right mid-stance", "Zügiger Schritt, rechte Standbeinmitte"), t("Brisk ward walk: single support on the right leg, the left leg swinging through.", "Zügiger Gang über die Station: Einbeinstand rechts, das linke Bein schwingt durch.")],
] as [string, J, J][]) Object.assign(a.poses.find((p: J) => p.id === id), { title, description });
//#endregion

//#region 🫁️ breathing, activity, bindings
const pat = a.thermal.respiration.patterns;
Object.assign(pat.find((p: J) => p.id === "walking"), { title: t("Brisk walking at 4.3 km/h", "Zügiges Gehen mit 4,3 km/h"), rateBpm: 22, tidalVolumeL: 1.3 });
for (const p of a.poses) {
  p.thermal.activityMet = p.thermal.breathing === "walking" ? 3.5 : 1.4;
  p.thermal.regions = {};
}
for (const tk of walk.tracks) if (tk.target === "activityMet") for (const k of tk.keys) k.v = 3.5;
bindRegions(a, (p, r) => ({ exposedFraction: r === "scrub-top" ? 0.92 : 1, sunlitFraction: 0 }));
walk.title = t("Brisk ward walk", "Zügiger Gang über die Station");
walk.description = t("A brisk heel-to-toe walk of 1.0 s per cycle (120 steps per minute, 1.2 m/s) with planted feet and double support; both arms swing in counter-phase to the legs.", "Ein zügiger Fersen-Zehen-Gang mit 1,0 s je Zyklus (120 Schritte je Minute, 1,2 m/s) mit aufgesetzten Füßen und Doppelstandphase; beide Arme schwingen gegengleich zu den Beinen.");
const idle = a.clips.find((c: J) => c.id === "idle-breathing");
Object.assign(idle, { title: t("Catching breath at the station", "Durchatmen am Stationstresen"), description: t("Standing at rest with the quiet breathing pattern (13 per minute); the chest rises by the derived tidal motion, the shoulders follow by 1.5 degrees.", "Ruhiges Stehen mit dem Ruheatemmuster (13 je Minute); die Brust hebt sich mit der abgeleiteten Atembewegung, die Schultern folgen um 1,5 Grad.") });
idle.phases = [{ id: "rest", fromS: 0, toS: idle.durationS, title: t("Standing between errands", "Stehen zwischen zwei Wegen") }];
a.structure.assumptions = NURSE_TEXT.structure;
//#endregion

//#region ✒️ outlines re-traced from the volume in the default pose; ankle gaps
const geo = (s: string): J => a.representations[s].geometry;
const coatParts = (s: string): string[] => a.volumes[s].body.filter((x: J) => x.material === "scrub-twill" && !/sleeve|collar|pocket/.test(x.id)).map((x: J) => x.id);
retrace(a, new Set(["torso"]), (shape, s) => (shape.id === "coat" ? coatParts(s) : []), ["elevation", "front", "plan"]);
retrace(a, new Set(["arm-l", "forearm-l", "hand-l", "arm-r", "forearm-r", "hand-r", "thigh-l", "shank-l", "foot-l", "toe-l", "thigh-r", "shank-r", "foot-r", "toe-r", "pelvis"]), (shape, s) => [...(s === "500" && shape.id === "forearm-r" ? ["hand-r"] : []), ...a.volumes[s].body.filter((x: J) => x.id.startsWith(`${shape.id}-`)).map((x: J) => x.id)]);
retrace(a, new Set(["forearm-l", "forearm-r"]), () => [], ["elevation"]);
for (const side of ["l", "r"]) {
  const g = geo("50");
  const shank = g.elevation.find((x: J) => x.id === `shank-${side}`);
  const props = { id: `sock-${side}`, role: "body", filled: true, tone: 0.3, region: "clogs", material: "compression-sock" };
  const el = traced(a, "50", "elevation", [`sock-${side}`], `shank-${side}`, { ...props, z: shank.z - 0.01 });
  g.elevation.push(el);
  for (const view of ["front", "plan"] as const) {
    const twin = g[view].find((x: J) => x.id === `shank-${side}`);
    const sh = traced(a, "50", view, [`sock-${side}`], `shank-${side}`, { ...props, z: twin.z - 0.01 }, twinDepth(a, el, view));
    if (sh) g[view].push(sh);
  }
}
//#endregion

//#region 📝️ texts
Object.assign(a, NURSE_TEXT.identity);
a.structure.dynamics = "inertial";
a.thermal.assumptions = NURSE_TEXT.thermal(skin);
for (const s of ["500", "200", "100", "50"]) {
  a.representations[s].decisions = NURSE_TEXT.representation[s];
  a.volumes[s].decisions = NURSE_TEXT.volume[s];
}
a.symbols = a.symbols.map((s: J) => ({ ...s, id: "brisk-walker-marker", title: t("Walking nurse marker", "Gehende Pflegekraft als Marke") }));
a.provenance = { ...a.provenance, author: "Semio staffage human worker HW-03", created: "2026-11-22", notes: NURSE_TEXT.notes };
references(a, NURSE_TEXT.references);
pruneMaterials(a);
//#endregion

await finishDimensions(a);
if (values.out) await Bun.write(values.out, `${JSON.stringify(doc, null, 2)}\n`);
else console.log(save(doc));
