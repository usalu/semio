/** 🌡️ The physical scenario of `sunbather-on-towel`: identity texts, low-clo swimwear and bare skin with a high solar absorptivity, a towel on hot sand, the summer beach reference environment with strong sun, contact anchors found on the lying body, the supine, prone and propped poses, the breathing and fanning loops and the provenance.
 *
 * @see ./sunbather.ts
 */
import { clone, framesOf, r5, reach, t, type Json } from "./lib.ts";
import { worldSolids } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";

const SKIN_NODE = 12000;

const MATERIALS: Json[] = [
  { id: "skin", title: t("Skin, medium tone", "Haut, mittlerer Hautton"), emissivity: 0.98, solarAbsorptivity: 0.67, source: "literature", reference: "ashrae-2021" },
  { id: "hair", title: t("Hair, dark brown", "Haar, dunkelbraun"), emissivity: 0.98, solarAbsorptivity: 0.9, source: "authored-estimate" },
  { id: "lycra-swimsuit", title: t("Polyamide-elastane swimsuit, navy", "Badeanzug aus Polyamid-Elasthan, marineblau"), emissivity: 0.93, solarAbsorptivity: 0.88, specificHeatJKgK: 1600, source: "authored-estimate", reference: "ashrae-2021" },
  { id: "cotton-terry", title: t("Cotton terry beach towel, striped", "Strandtuch aus Baumwollfrottee, gestreift"), emissivity: 0.93, solarAbsorptivity: 0.55, densityKgM3: 160, specificHeatJKgK: 1300, source: "authored-estimate", reference: "ashrae-2021" },
  { id: "bone", title: t("Bone", "Knochen"), emissivity: 0.9, solarAbsorptivity: 0.5, densityKgM3: 1900, source: "authored-estimate" },
  { id: "costal-cartilage", title: t("Costal cartilage", "Rippenknorpel"), emissivity: 0.9, solarAbsorptivity: 0.5, densityKgM3: 1100, source: "authored-estimate" },
];

const REFERENCES: Json[] = [
  { id: "ainsworth-2011", citation: "Ainsworth, B. E. et al. (2011). 2011 Compendium of Physical Activities: a second update of codes and MET values. Medicine and Science in Sports and Exercise 43(8), 1575-1581; code 07011 (lying quietly, 1.3 MET), 07010 (lying quietly, watching television, 1.0 MET)." },
  { id: "iso-7730-2005", citation: "ISO 7730:2005 Ergonomics of the thermal environment, analytical determination of thermal comfort; Annex B clothing insulation (panties and swimwear about 0.03 to 0.05 clo) and Annex A metabolic rates (reclining 0.8 met)." },
  { id: "blum-1945", citation: "Blum, H. F. (1945). The solar heat load: its relationship to total heat load and its relative importance in the design of clothing. Journal of Clinical Investigation 24(5), 712-721; mean absorptivity of human skin for sunlight 0.6 to 0.8 depending on pigmentation." },
  { id: "de-dear-2013", citation: "de Dear, R. J. et al. (2013). Progress in thermal comfort research over the last twenty years. Indoor Air 23(6), 442-461; projected area factors of supine and prone postures under direct solar radiation." },
  { id: "din-33402-2-2020", citation: "DIN 33402-2:2020-12 Ergonomie, Körpermaße des Menschen, Teil 2: Werte; body heights of German adults 18-65 years (women P50 1.66 m)." },
];

/** 🌍️ Regions of the lying body; each `of` thermal solid takes the region of the body solid it shows. */
export const regionOf = (bone: string, material: string): string => (material === "hair" ? "hair" : material === "cotton-terry" ? "towel" : material === "lycra-swimsuit" ? "swimsuit" : /^(head|neck)$/.test(bone) ? "face-skin" : /^(hand|thumb|index|middle|ring|little)/.test(bone) ? "hand-skin" : /^(arm|forearm)-/.test(bone) ? "arm-skin" : /^(foot|toe)-/.test(bone) ? "feet-skin" : /^(thigh|shank|kneecap)/.test(bone) ? "leg-skin" : "swimsuit");

/** 📍️ A contact anchor at the lowest point of a bone's body solids in a pose, written in the bone's frame. */
function lowAnchor(a: Json, offsets: Record<string, Json>, bone: string, id: string, title: Json, footprint: [number, number], scale = "50"): Json {
  const F = framesOf(a, offsets);
  const posed: Record<string, { frame: Json }> = {};
  const vec = (v: number[]) => ({ x: v[0]!, y: v[1]!, z: v[2]! });
  for (const [k, f] of Object.entries(F) as [string, Json][]) posed[k] = { frame: { o: vec(f.frame.o), u: vec(f.frame.u), v: vec(f.frame.v), w: vec(f.frame.w) } };
  let best: number[] | null = null;
  for (const w of worldSolids(a, posed as never, Number(scale), "body")) {
    const s = a.volumes[scale].body.find((b: Json) => b.id === w.id);
    if (!s || s.bone !== bone) continue;
    for (let i = 0; i < w.positions.length; i += 3) if (!best || w.positions[i + 1]! < best[1]!) best = [w.positions[i]!, w.positions[i + 1]!, w.positions[i + 2]!];
  }
  if (!best) throw new Error(`[DEBUG] no body solid on ${bone} for anchor ${id}`);
  const f = F[bone].frame;
  const d = [0, 1, 2].map((k) => best![k]! - f.o[k]!);
  const loc = [f.u, f.v, f.w].map((ax: number[]) => d[0]! * ax[0]! + d[1]! * ax[1]! + d[2]! * ax[2]!);
  return { id, bone, x: r5(loc[0]!), y: r5(loc[1]!), ...(Math.abs(loc[2]!) > 1e-4 ? { z: r5(loc[2]!) } : {}), kind: "contact", title, support: "contact", normalDeg: 90, footprintLengthM: footprint[0], footprintWidthM: footprint[1], frictionCoefficient: 0.6 };
}

/** 📉️ The lowest point of the body (towel excluded) in a pose. */
function minY(a: Json, offsets: Record<string, Json>): number {
  const F = framesOf(a, offsets);
  const vec = (v: number[]) => ({ x: v[0]!, y: v[1]!, z: v[2]! });
  const posed: Record<string, { frame: Json }> = {};
  for (const [k, f] of Object.entries(F) as [string, Json][]) posed[k] = { frame: { o: vec(f.frame.o), u: vec(f.frame.u), v: vec(f.frame.v), w: vec(f.frame.w) } };
  let lo = Infinity;
  for (const w of worldSolids(a, posed as never, 50, "body")) if (w.bone !== "towel") for (let i = 1; i < w.positions.length; i += 3) lo = Math.min(lo, w.positions[i]!);
  return lo;
}

/** 🧘️ Lifts the body so its lowest point rests on the towel top (0.01 m) in a pose. */
function settle(a: Json, offsets: Record<string, Json>): Record<string, Json> {
  const out = clone(offsets);
  const dy = out["body-lift"]?.dy ?? 0;
  out["body-lift"] = { dy: r5(dy + 0.01 - minY(a, out)) };
  return out;
}

/** 🙇️ The prone pose: the body rolled onto the front about the long axis of the pelvis, the forehead resting on the stacked forearms and hands, solved by a small IK for each arm within the joint limits. */
function pronePose(a: Json): Record<string, Json> {
  let off: Record<string, Json> = { pelvis: { angleDeg: 0, twistDeg: 180 }, neck: { angleDeg: 12 }, head: { angleDeg: 8 }, "foot-l": { angleDeg: -55 }, "foot-r": { angleDeg: -55 } };
  for (const side of ["l", "r"] as const) {
    const w = side === "l" ? 1 : -1;
    const res = reach(a, off, [
      { bone: `arm-${side}`, field: "angleDeg", min: 60, max: 170, start: 140 },
      { bone: `arm-${side}`, field: "abductionDeg", min: -45, max: 45, start: 0 },
      { bone: `arm-${side}`, field: "twistDeg", min: -90, max: 90, start: 0 },
      { bone: `forearm-${side}`, field: "angleDeg", min: 40, max: 150, start: 110 },
      { bone: `hand-${side}`, field: "angleDeg", min: -50, max: 50, start: 0 },
    ], { bone: `hand-${side}`, at: [0.06, 0, 0] }, { bone: "head", at: [0.08, -0.13, 0.05 * w] });
    off = res.offsets;
    if (res.error > 0.03) console.log(`[DEBUG] prone ${side} residual ${res.error.toFixed(3)} m`);
  }
  return settle(a, off);
}

export function sunbatherPhysics(a: Json, base: Json): void {
  a.title = t("Sunbather on Towel", "Sonnenbadende Person auf Strandtuch");
  a.summary = t(
    "A woman of 1.69 m in a navy one-piece swimsuit lying on a striped 1.80 × 0.90 m beach towel in the midday sun: on her back with the arms beside her, on her front with the head turned to the side, and propped on the elbows to look out to sea. Bare skin and the dark swimsuit absorb most of the sunlight, the towel lies on sand at 45 °C, and a loop shows her fanning her face with the hand.",
    "Eine Frau von 1,69 m im marineblauen Badeanzug liegt in der Mittagssonne auf einem gestreiften Strandtuch von 1,80 × 0,90 m: auf dem Rücken mit den Armen neben dem Körper, auf dem Bauch mit zur Seite gedrehtem Kopf und auf die Ellbogen gestützt mit Blick aufs Meer. Die nackte Haut und der dunkle Badeanzug nehmen den Großteil des Sonnenlichts auf, das Tuch liegt auf 45 °C heißem Sand, und eine Schleife zeigt, wie sie sich mit der Hand Luft zufächelt.",
  );
  a.tags = ["human", "adult", "summer", "beach", "sunbathing", "lying", "swimwear", "towel", "solar-load"];
  a.provenance.author = "Semio staffage human collection worker HW-16";
  a.provenance.method = "handcrafted";
  a.provenance.created = "2026-11-22";
  a.provenance.references = [...a.provenance.references.filter((r: Json) => !REFERENCES.some((n) => n.id === r.id)), ...REFERENCES];
  a.provenance.notes = t(
    "Rig and body are derived from the standing adult (1.69 m, de Leva female segment masses), re-rooted on a towel bone with a prismatic body lift so the rest pose lies on the back; the sweatshirt, jeans and sneaker lofts are pared down to the body (bare arms and legs, bare feet) and the trunk is a one-piece swimsuit. Stature 1.69 m lies above the DIN 33402-2 female median (1.66 m), 56.4 kg without clothing (BMI 19.7). Every view is drawn as the projection of the solids of its scale.",
    "Rig und Körper stammen von der stehenden erwachsenen Person (1,69 m, weibliche Segmentmassen nach de Leva), neu auf einen Tuchknochen mit prismatischem Körperhub gesetzt, sodass die Ruhelage auf dem Rücken liegt; die Lofts von Sweatshirt, Jeans und Turnschuhen sind auf den Körper zurückgeschnitten (nackte Arme und Beine, bloße Füße) und der Rumpf trägt einen Badeanzug. Die Körpergröße von 1,69 m liegt über dem Frauenmedian von DIN 33402-2 (1,66 m), 56,4 kg ohne Kleidung (BMI 19,7). Jede Ansicht ist als Projektion der Körper ihres Maßstabs gezeichnet.",
  );
  a.materials = MATERIALS;
  for (const b of a.rig.bones) {
    if (b.id === "torso") b.title = t("Torso in the swimsuit", "Rumpf im Badeanzug");
    if (b.mass && ["torso", "lumbar", "pelvis"].includes(b.id)) b.mass.kg = r5(b.mass.kg * 0.975);
    if (b.mass && /^(foot|shank|thigh|arm|forearm)-/.test(b.id)) b.mass.kg = r5(b.mass.kg * (b.id.startsWith("foot") ? 0.86 : 0.985));
  }

  const body = (sc: string) => new Map(a.volumes[sc].body.map((s: Json) => [s.id, s]));
  for (const sc of ["50", "100", "200", "500"]) {
    const bm = body(sc);
    for (const s of a.volumes[sc].thermal) {
      if ("of" in s) {
        const src = bm.get(s.of) as Json;
        if (src) s.region = regionOf(src.bone, src.material);
        delete s.layer;
      } else if (s.id === "th-body-plume") Object.assign(s, { bone: "torso", material: "lycra-swimsuit", region: "swimsuit", radii: [0.12, 0.04, 0.11], transform: { translation: [0.19, -0.09, 0] } });
      else if (s.region && !["face-skin"].includes(s.region)) s.region = "swimsuit";
    }
    a.volumes[sc].thermal = a.volumes[sc].thermal.filter((s: Json) => !("of" in s) || bm.has(s.of));
  }

  const env = { airTempC: 31, meanRadiantTempC: 38, skyTempC: 18, groundTempC: 45, relativeHumidity: 0.5, windMS: 2.5, windDirectionDeg: 90, solarWM2: 850, diffuseFraction: 0.15, pressurePa: 101325 };
  a.thermal.environment = env;
  a.thermal.baseMet = 1.0;
  a.thermal.body = { ...a.thermal.body, massKg: 56.4, skinAreaM2: 1.646, basalMetabolismW: 63.5, source: "authored-estimate", reference: "mifflin-1990" };
  a.thermal.respiration = { ...a.thermal.respiration, patterns: [
    { id: "lying-rest", title: t("Lying at rest", "Ruhiges Liegen"), rateBpm: 11, tidalVolumeL: 0.42, inhaleFraction: 0.38, pauseAfterInhaleFraction: 0.04, exhaleFraction: 0.46, pauseAfterExhaleFraction: 0.12, route: "nose", source: "authored-estimate", reference: "icrp-66-1994" },
    { id: "fanning", title: t("Fanning the face in the heat", "Sich in der Hitze Luft zufächeln"), rateBpm: 15, tidalVolumeL: 0.48, inhaleFraction: 0.4, pauseAfterInhaleFraction: 0.03, exhaleFraction: 0.47, pauseAfterExhaleFraction: 0.1, route: "nose-and-mouth", source: "authored-estimate", reference: "icrp-66-1994" },
  ], motion: [{ bone: "torso", angleDeg: 0.4, dy: 0 }] };
  const region = (id: string, en: string, de: string, material: string, segment: string | null, share: number, skin: number, cl: number, geometry: string, sky: number, layers: Json[] = []) => ({ id, title: t(en, de), kind: segment ? (layers.length ? "clothing" : material === "hair" ? "hair" : "skin") : "surface", model: segment ? "body" : "passive", material, areaM2: 0.1, characteristicLengthM: cl, geometry, skyViewFactor: sky, layers, heatCapacityJM2K: segment ? SKIN_NODE + layers.reduce((s: number, l: Json) => s + l.heatCapacityJM2K, 0) : 2100, ...(segment ? { body: { segment, metabolicShare: share, skinFraction: skin } } : {}), source: "authored-estimate", reference: segment ? "iso-7730-2005" : "ashrae-2021" });
  const suit = { id: "swimsuit", title: t("One-piece swimsuit, wet-dry", "Badeanzug"), kind: "clothing", material: "lycra-swimsuit", thicknessM: 0.0007, resistanceM2KW: 0.0078, vapourResistanceM2PaW: 1.2, heatCapacityJM2K: 110, source: "literature", reference: "iso-7730-2005" };
  const hair = { id: "hair", title: t("Long hair spread on the towel", "Langes Haar auf dem Tuch"), kind: "hair", material: "hair", thicknessM: 0.015, resistanceM2KW: 0.05, vapourResistanceM2PaW: 8, heatCapacityJM2K: 260, source: "authored-estimate" };
  a.thermal.regions = [
    region("face-skin", "Face and neck", "Gesicht und Hals", "skin", "head", 0.07, 0.07, 0.18, "sphere", 0.9),
    region("hair", "Hair", "Haar", "hair", "head", 0.03, 0.02, 0.18, "sphere", 0.5, [hair]),
    region("arm-skin", "Bare arms", "Nackte Arme", "skin", "arm", 0.09, 0.13, 0.08, "cylinder", 0.7),
    region("hand-skin", "Hands", "Hände", "skin", "hand", 0.04, 0.05, 0.05, "horizontal-plate-up", 0.8),
    region("swimsuit", "Trunk in the swimsuit", "Rumpf im Badeanzug", "lycra-swimsuit", "trunk", 0.47, 0.37, 0.3, "horizontal-plate-up", 0.8, [suit]),
    region("leg-skin", "Bare legs", "Nackte Beine", "skin", "leg", 0.27, 0.3, 0.12, "cylinder", 0.75),
    region("feet-skin", "Bare feet", "Nackte Füße", "skin", "foot", 0.03, 0.06, 0.08, "cylinder", 0.8),
    region("towel", "Beach towel beside the body", "Strandtuch neben dem Körper", "cotton-terry", null, 0, 0, 0.9, "horizontal-plate-up", 0.95),
  ];
  a.thermal.emitters = a.thermal.emitters.map((e: Json) => (e.kind === "plume" ? { ...e, anchor: "chest-plume", directionDeg: -90, region: "swimsuit", diameterM: 0.3 } : e));
  a.thermal.assumptions = [
    t("Skin area 1.646 m2 after DuBois and DuBois (1916) for 56.4 kg and 1.69 m; skin fractions 0.07 face and neck, 0.02 scalp, 0.13 arms, 0.05 hands, 0.37 trunk under the swimsuit, 0.30 legs, 0.06 feet (rule-of-nines proportions, authored).", "Hautfläche 1,646 m2 nach DuBois und DuBois (1916) für 56,4 kg und 1,69 m; Hautanteile 0,07 Gesicht und Hals, 0,02 Kopfhaut, 0,13 Arme, 0,05 Hände, 0,37 Rumpf unter dem Badeanzug, 0,30 Beine, 0,06 Füße (Neunerregel-Proportionen, geschätzt)."),
    t("Clothing 0.05 clo only on the trunk (one-piece swimsuit, ISO 7730 Annex B); bare skin absorbs 0.67 of the sunlight (medium skin tone, Blum 1945: 0.6 to 0.8), the navy swimsuit 0.88, the hair 0.9.", "Kleidung 0,05 clo nur am Rumpf (Badeanzug, ISO 7730 Anhang B); nackte Haut nimmt 0,67 des Sonnenlichts auf (mittlerer Hautton, Blum 1945: 0,6 bis 0,8), der marineblaue Badeanzug 0,88, das Haar 0,9."),
    t("Lying at rest 1.0 met, propped on the elbows 1.2 met, fanning the face 1.4 met (Ainsworth 2011, codes 07010 and 07011; ISO 7730 Annex A reclining 0.8 met). In the supine pose about half of the body surface faces the sun; the side lying on the towel conducts to the cotton terry over sand at 45 °C, which reaches about 40 °C under the body.", "Ruhiges Liegen 1,0 met, auf die Ellbogen gestützt 1,2 met, Luft zufächeln 1,4 met (Ainsworth 2011, Codes 07010 und 07011; ISO 7730 Anhang A Liegen 0,8 met). In der Rückenlage ist etwa die Hälfte der Körperoberfläche der Sonne zugewandt; die aufliegende Seite leitet Wärme an das Baumwollfrottee über 45 °C heißem Sand, das unter dem Körper etwa 40 °C erreicht."),
    t("Region heat capacity = skin node 12 kJ/(m2K) plus the areal heat capacity of the swimsuit or hair; the towel is a passive cotton layer of 2.1 kJ/(m2K) on the sand, its rated power 0 W.", "Wärmekapazität der Region = Hautknoten 12 kJ/(m2K) plus die flächenbezogene Wärmekapazität von Badeanzug oder Haar; das Tuch ist eine passive Baumwollschicht von 2,1 kJ/(m2K) auf dem Sand, ihre Nennleistung 0 W."),
  ];
  a.thermal.regions.find((r: Json) => r.id === "towel").ratedPowerW = 0;
  a.thermal.regions.find((r: Json) => r.id === "towel").ratedPowerBasis = "passive";
  posesAndClips(a, base);
}

const REG = ["face-skin", "hair", "arm-skin", "hand-skin", "swimsuit", "leg-skin", "feet-skin", "towel"] as const;
const bind = (sun: Record<string, number>, contact: Record<string, number>, exposure: Record<string, number> = {}) =>
  Object.fromEntries(REG.map((r) => [r, { exposedFraction: exposure[r] ?? 1 - (contact[r] ?? 0), sunlitFraction: sun[r] ?? 0, ...(contact[r] ? { contact: { fraction: contact[r], tempC: r === "towel" ? 45 : 39 } } : {}) }]));

function posesAndClips(a: Json, base: Json): void {
  const supine: Record<string, Json> = { neck: { angleDeg: 6 }, head: { angleDeg: 14 }, "foot-l": { angleDeg: -38 }, "foot-r": { angleDeg: -38 }, "toe-l": { angleDeg: 0 }, "toe-r": { angleDeg: 0 }, "arm-l": { angleDeg: 0, abductionDeg: 12 }, "arm-r": { angleDeg: 0, abductionDeg: -12 }, "forearm-l": { angleDeg: 6 }, "forearm-r": { angleDeg: 6 }, "thigh-l": { angleDeg: 0 }, "thigh-r": { angleDeg: 0 } };
  const prone = pronePose(a);
  const propped0: Record<string, Json> = { ...clone(supine), lumbar: { angleDeg: -22 }, torso: { angleDeg: -14 }, neck: { angleDeg: -18 }, head: { angleDeg: -6 }, "arm-l": { angleDeg: -58 }, "arm-r": { angleDeg: -58 }, "forearm-l": { angleDeg: 112 }, "forearm-r": { angleDeg: 112 }, "hand-l": { angleDeg: -30 }, "hand-r": { angleDeg: -30 } };
  const propped = settle(a, propped0);
  Object.assign(supine, settle(a, supine));
  const anchors = a.anchors.filter((x: Json) => !["heel-l", "ball-l", "heel-r", "ball-r"].includes(x.id));
  const low = (pose: Record<string, Json>, bone: string, id: string, en: string, de: string, fp: [number, number]) => lowAnchor(a, pose, bone, id, t(en, de), fp);
  const contactsSupine = [low(supine, "head", "occiput", "Back of the head on the towel", "Hinterkopf auf dem Tuch", [0.08, 0.08]), low(supine, "torso", "shoulder-blades", "Shoulder blades on the towel", "Schulterblätter auf dem Tuch", [0.16, 0.3]), low(supine, "pelvis", "sacrum", "Buttocks on the towel", "Gesäß auf dem Tuch", [0.16, 0.28]), low(supine, "shank-l", "calf-l", "Left calf on the towel", "Linke Wade auf dem Tuch", [0.14, 0.08]), low(supine, "shank-r", "calf-r", "Right calf on the towel", "Rechte Wade auf dem Tuch", [0.14, 0.08]), low(supine, "foot-l", "heel-l", "Left heel on the towel", "Linke Ferse auf dem Tuch", [0.06, 0.05]), low(supine, "foot-r", "heel-r", "Right heel on the towel", "Rechte Ferse auf dem Tuch", [0.06, 0.05])];
  const contactsProne = [low(prone, "torso", "chest", "Chest on the towel", "Brust auf dem Tuch", [0.18, 0.3]), low(prone, "pelvis", "hips", "Front of the hips on the towel", "Hüftvorderseite auf dem Tuch", [0.14, 0.28]), low(prone, "thigh-l", "thigh-front-l", "Left thigh on the towel", "Linker Oberschenkel auf dem Tuch", [0.2, 0.1]), low(prone, "thigh-r", "thigh-front-r", "Right thigh on the towel", "Rechter Oberschenkel auf dem Tuch", [0.2, 0.1]), low(prone, "foot-l", "instep-l", "Left instep on the towel", "Linker Spann auf dem Tuch", [0.08, 0.06]), low(prone, "foot-r", "instep-r", "Right instep on the towel", "Rechter Spann auf dem Tuch", [0.08, 0.06]), low(prone, "head", "cheek", "Cheek on the towel", "Wange auf dem Tuch", [0.08, 0.06])];
  const contactsPropped = [low(propped, "pelvis", "sacrum-propped", "Buttocks on the towel, propped up", "Gesäß auf dem Tuch, aufgestützt", [0.16, 0.28]), low(propped, "forearm-l", "elbow-l", "Left forearm on the towel", "Linker Unterarm auf dem Tuch", [0.16, 0.06]), low(propped, "forearm-r", "elbow-r", "Right forearm on the towel", "Rechter Unterarm auf dem Tuch", [0.16, 0.06]), low(propped, "shank-l", "calf-l-propped", "Left calf on the towel, propped up", "Linke Wade auf dem Tuch, aufgestützt", [0.14, 0.08]), low(propped, "shank-r", "calf-r-propped", "Right calf on the towel, propped up", "Rechte Wade auf dem Tuch, aufgestützt", [0.14, 0.08]), low(propped, "foot-l", "heel-l-propped", "Left heel on the towel, propped up", "Linke Ferse auf dem Tuch, aufgestützt", [0.06, 0.05]), low(propped, "foot-r", "heel-r-propped", "Right heel on the towel, propped up", "Rechte Ferse auf dem Tuch, aufgestützt", [0.06, 0.05])];
  const corners = [[0.02, 0.43, "head-l"], [0.02, -0.43, "head-r"], [1.78, 0.43, "foot-l"], [1.78, -0.43, "foot-r"]].map(([x, z, n]) => ({ id: `towel-${n}`, bone: "towel", x, y: 0, z, kind: "support", title: t(`Towel corner on the sand, ${String(n).replace("-l", " left").replace("-r", " right")} end`, `Tuchecke auf dem Sand, ${String(n).startsWith("head") ? "Kopfende" : "Fußende"} ${String(n).endsWith("-l") ? "links" : "rechts"}`), support: "contact", normalDeg: 90, frictionCoefficient: 0.7, footprintLengthM: 0.04, footprintWidthM: 0.04 }));
  const plume = { id: "chest-plume", bone: "torso", x: 0.19, y: -0.12, kind: "emitter", title: t("Above the chest", "Über der Brust") };
  a.anchors = [...anchors.filter((x: Json) => x.id !== "crown"), ...contactsSupine, ...contactsProne, ...contactsPropped, ...corners, plume, anchors.find((x: Json) => x.id === "crown")].filter(Boolean);
  a.structure.supports = corners.map((c) => c.id);
  a.structure.assumptions = [t("The towel lies flat on firm sand and carries the body through unilateral contacts at the back of the head, shoulder blades, buttocks, calves and heels (supine), chest, hips, thighs, insteps and cheek (prone) or forearms, buttocks, calves and heels (propped); segments are point masses at the de Leva centres of the female table.", "Das Tuch liegt flach auf festem Sand und trägt den Körper über einseitige Kontakte an Hinterkopf, Schulterblättern, Gesäß, Waden und Fersen (Rückenlage), Brust, Hüften, Oberschenkeln, Spann und Wange (Bauchlage) oder Unterarmen, Gesäß, Waden und Fersen (aufgestützt); die Segmente sind Punktmassen in den Schwerpunkten nach de Leva (Frauentabelle).")];
  a.structure.dynamics = "quasi-static";
  for (const sc of ["50", "100", "200", "500"]) {
    const st = a.volumes[sc].structure;
    a.volumes[sc].structure = st.filter((s: Json) => !(s.part === "support" && /^(heel|ball)-/.test(s.anchor)));
    a.volumes[sc].structure.push({ id: "member-towel", kind: "box", bone: "towel", material: "cotton-terry", role: "body", tone: 0.6, size: [1.7, 0.006, 0.12], transform: { translation: [0.9, 0.003, 0] }, member: "towel", part: "member", slender: true });
    const feat = { "50": 0.0075, "100": 0.015, "200": 0.03, "500": 0.075 }[sc]!;
    for (const c of corners) a.volumes[sc].structure.push({ id: `support-${c.id}`, kind: "box", bone: "towel", material: "cotton-terry", role: "detail", tone: 0.52, size: [Math.max(0.04, feat), Math.max(0.01, feat), Math.max(0.04, feat)], transform: { translation: [c.x, Math.max(0.01, feat) / 2, c.z] }, member: "towel", part: "support", anchor: c.id });
    if (sc !== "50")
      for (const side of ["l", "r"]) {
        const fingerBones = a.rig.bones.filter((b: Json) => new RegExp(`^(thumb|index|middle|ring|little)-.*-${side}$`).test(b.id)).map((b: Json) => b.id);
        const F = framesOf(a, {});
        const h = F[`hand-${side}`].frame;
        const pts: number[][] = [];
        for (const id of fingerBones) {
          const f = F[id].frame;
          const bone = a.rig.bones.find((b: Json) => b.id === id);
          for (const s0 of [0, bone.lengthM]) pts.push([0, 1, 2].map((k) => f.o[k] + s0 * f.u[k] - h.o[k]));
        }
        const loc = pts.map((d) => [h.u, h.v, h.w].map((ax: number[]) => d[0]! * ax[0]! + d[1]! * ax[1]! + d[2]! * ax[2]!));
        const lo = [0, 1, 2].map((k) => Math.min(...loc.map((p) => p[k]!)));
        const hi = [0, 1, 2].map((k) => Math.max(...loc.map((p) => p[k]!)));
        const size = [0, 1, 2].map((k) => r5(Math.max(hi[k]! - lo[k]! + 0.01, feat)));
        a.volumes[sc].structure.push({ id: `finger-bones-${side}`, kind: "box", bone: `hand-${side}`, material: "bone", role: "body", tone: 0.62, size, cornerRadius: r5(Math.min(...size) / 4), transform: { translation: [0, 1, 2].map((k) => r5((lo[k]! + hi[k]!) / 2)) }, member: `hand-${side}`, members: fingerBones, part: "member" });
        if (sc !== "100") {
          const ft = F[`foot-${side}`].frame;
          const toe = F[`toe-${side}`].frame;
          const tl = a.rig.bones.find((b: Json) => b.id === `toe-${side}`).lengthM;
          const ends = [0, tl].map((s0) => [0, 1, 2].map((k) => toe.o[k] + s0 * toe.u[k] - ft.o[k])).map((d) => [ft.u, ft.v, ft.w].map((ax: number[]) => d[0]! * ax[0]! + d[1]! * ax[1]! + d[2]! * ax[2]!));
          a.volumes[sc].structure.push({ id: `toe-bones-${side}`, kind: "capsule", bone: `foot-${side}`, material: "bone", role: "body", tone: 0.62, p0: ends[0]!.map(r5), p1: ends[1]!.map(r5), r0: r5(feat / 2), r1: r5(feat / 2), member: `foot-${side}`, members: [`toe-${side}`], part: "member" });
        }
      }
  }

  const sunSupine = { "face-skin": 0.85, hair: 0.35, "arm-skin": 0.5, "hand-skin": 0.5, swimsuit: 0.5, "leg-skin": 0.5, "feet-skin": 0.6, towel: 0.75 };
  const sunProne = { "face-skin": 0.4, hair: 0.6, "arm-skin": 0.5, "hand-skin": 0.5, swimsuit: 0.5, "leg-skin": 0.5, "feet-skin": 0.35, towel: 0.75 };
  const sunPropped = { "face-skin": 0.9, hair: 0.45, "arm-skin": 0.45, "hand-skin": 0.3, swimsuit: 0.6, "leg-skin": 0.5, "feet-skin": 0.6, towel: 0.72 };
  const em = { "body-plume": 0.7 };
  const ids = (xs: Json[]) => xs.map((x) => x.id);
  a.poses = [
    { id: "supine", title: t("Lying on the back", "Rückenlage"), description: t("Flat on the back on the towel, the head resting on its back with the chin slightly raised, the arms beside the body with the palms down and the legs stretched with the feet relaxed outwards.", "Flach auf dem Rücken auf dem Tuch, der Kopf liegt mit leicht angehobenem Kinn auf dem Hinterkopf, die Arme neben dem Körper mit den Handflächen nach unten, die Beine gestreckt und die Füße entspannt nach außen gekippt."), bones: supine, ground: "authored", contacts: ids(contactsSupine), thermal: { activityMet: 1.0, breathing: "lying-rest", regions: bind(sunSupine, { "face-skin": 0.05, hair: 0.45, "arm-skin": 0.2, "hand-skin": 0.35, swimsuit: 0.3, "leg-skin": 0.25, "feet-skin": 0.1, towel: 0.98 }), emitters: em }, loads: [] },
    { id: "prone", title: t("Lying on the front", "Bauchlage"), description: t("On the stomach with the head turned to the left and resting on the cheek, the arms alongside the body and the insteps flat on the towel, tanning the back.", "Auf dem Bauch, der Kopf nach links gedreht und auf der Wange liegend, die Arme neben dem Körper und der Spann flach auf dem Tuch, um den Rücken zu bräunen."), bones: prone, ground: "authored", contacts: ids(contactsProne), thermal: { activityMet: 1.0, breathing: "lying-rest", regions: bind(sunProne, { "face-skin": 0.3, hair: 0.05, "arm-skin": 0.2, "hand-skin": 0.35, swimsuit: 0.32, "leg-skin": 0.3, "feet-skin": 0.25, towel: 0.98 }), emitters: em }, loads: [] },
    { id: "propped-on-elbows", title: t("Propped on the elbows", "Auf die Ellbogen gestützt"), description: t("On the back with the upper body raised on both forearms, the chin lifted to look out to sea, the legs stretched on the towel.", "Auf dem Rücken mit auf beide Unterarme gestütztem Oberkörper, das Kinn gehoben mit Blick aufs Meer, die Beine auf dem Tuch ausgestreckt."), bones: propped, ground: "authored", contacts: ids(contactsPropped), thermal: { activityMet: 1.2, breathing: "lying-rest", regions: bind(sunPropped, { "face-skin": 0, hair: 0, "arm-skin": 0.25, "hand-skin": 0.3, swimsuit: 0.18, "leg-skin": 0.25, "feet-skin": 0.1, towel: 0.97 }), emitters: em }, loads: [] },
  ];
  const fan = base.clips.find((c: Json) => c.id === "fanning");
  const fanKeys = fan.keys.map((k: Json) => {
    const b = clone(k.bones ?? {});
    for (const key of ["torso", "lumbar", "neck", "head"]) delete b[key];
    return { t: k.t, pose: "supine", ease: k.ease, bones: { ...b, "forearm-r": { angleDeg: (b["forearm-r"]?.angleDeg ?? 110) + 10 } } };
  });
  const span = (d: number, xs: Json[]) => xs.map((x) => ({ anchor: x.id, fromS: 0, toS: d }));
  a.clips = [
    { id: "idle-breathing", title: t("Dozing in the sun", "Dösen in der Sonne"), description: t("Lying still on the back; the chest and belly rise and fall slowly and the head rolls a little to the side.", "Ruhig auf dem Rücken liegend; Brust und Bauch heben und senken sich langsam, der Kopf rollt ein wenig zur Seite."), durationS: 5.4, fps: 30, loop: true, rootMotion: "none", keys: [{ t: 0, pose: "supine", ease: "smooth" }, { t: 2.7, pose: "supine", ease: "smooth", bones: { neck: { angleDeg: 8 }, head: { angleDeg: 12 }, torso: { angleDeg: 0.5 } } }, { t: 5.4, pose: "supine", ease: "smooth" }], contacts: span(5.4, contactsSupine), phases: [{ id: "dozing", fromS: 0, toS: 5.4, title: t("Dozing", "Dösen") }], events: [], tracks: [{ target: "breathing", ease: "step", keys: [{ t: 0, v: 0 }, { t: 5.4, v: 0 }] }, { target: "activityMet", ease: "step", keys: [{ t: 0, v: 1.0 }, { t: 5.4, v: 1.0 }] }] },
    { id: "fanning-face", title: t("Fanning the face", "Luft zufächeln"), description: t("Lying on the back in the heat and fanning the face with the right hand, the wrist flicking twice a second.", "In der Hitze auf dem Rücken liegend und mit der rechten Hand dem Gesicht Luft zufächeln, das Handgelenk schlägt zweimal pro Sekunde."), durationS: fan.durationS, fps: 30, loop: true, rootMotion: "none", keys: fanKeys, contacts: span(fan.durationS, contactsSupine), phases: [{ id: "fanning", fromS: 0, toS: fan.durationS, title: t("Fanning", "Fächeln") }], events: [], tracks: [{ target: "breathing", ease: "step", keys: [{ t: 0, v: 1 }, { t: fan.durationS, v: 1 }] }, { target: "activityMet", ease: "smooth", keys: [{ t: 0, v: 1.4 }, { t: fan.durationS, v: 1.4 }] }] },
  ];
  a.presentation = { defaultPose: "supine", defaultClip: "idle-breathing", defaultScale: 100 };
}
