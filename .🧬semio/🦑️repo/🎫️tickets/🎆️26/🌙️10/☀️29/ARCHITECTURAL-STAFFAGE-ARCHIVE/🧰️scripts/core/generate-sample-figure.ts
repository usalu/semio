import { readFileSync, writeFileSync } from "node:fs";
import { evaluate } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";

const S = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const v1 = JSON.parse(readFileSync(`${S}/🧫️fixtures/🚶️reference-walker/🔣️.json`, "utf8"));
const t = (en: string, de: string) => ({ en, de });
const regionOf: Record<string, string> = { head: "head", neck: "head", hair: "head", torso: "torso", "torso-weave": "torso", pelvis: "torso", "arm-l": "arms", "arm-r": "arms", "thigh-l": "legs", "shank-l": "legs", "thigh-r": "legs", "shank-r": "legs", "foot-l": "feet", "foot-r": "feet" };
const materialOf: Record<string, string> = { head: "skin", neck: "skin", hair: "hair", torso: "cotton-clothing", "torso-weave": "cotton-clothing", pelvis: "cotton-clothing", "arm-l": "cotton-clothing", "arm-r": "cotton-clothing", "thigh-l": "cotton-clothing", "shank-l": "cotton-clothing", "thigh-r": "cotton-clothing", "shank-r": "cotton-clothing", "foot-l": "leather-shoe", "foot-r": "leather-shoe" };

const shape = (id: string, extra: object = {}) => {
  const { lod, ...rest } = v1.geometry.elevation.find((s: any) => s.id === id);
  return { ...rest, region: regionOf[id], material: materialOf[id], ...extra };
};
const boot = (id: string) => ({ id, bone: id, role: "body", z: 1, filled: true, tone: 0.3, kind: "poly", closed: true, points: [[0, -0.06], [0, 0.06], [0.41, 0.05], [0.405, 0.21], [0.51, 0.21], [0.51, -0.06], [0.41, -0.05]], regions: ["legs", "feet"], material: "cotton-clothing" });
const decisions = (scale: string, extra: Record<string, [string, string]> = {}) => {
  const base: Record<string, [string, string]> = {
    silhouette: [`Test figure at 1:${scale}: shapes follow the bones.`, `Testfigur bei 1:${scale}: Die Formen folgen den Knochen.`],
    retained: ["Head, trunk, limbs.", "Kopf, Rumpf, Gliedmaßen."],
    merged: ["Nothing merged beyond the listed regions.", "Nichts über die genannten Regionen hinaus zusammengefasst."],
    omitted: ["No facial detail.", "Keine Gesichtsdetails."],
    thinElements: ["No thin elements.", "Keine dünnen Elemente."],
    hatching: ["None.", "Keine."],
    styles: ["Line and solid read the same masses.", "Linie und Vollton zeigen dieselben Massen."],
    overlays: ["Overlays map onto the regions.", "Überlagerungen bilden auf die Regionen ab."],
    ...extra,
  };
  return Object.fromEntries(Object.entries(base).map(([k, [en, de]]) => [k, t(en, de)]));
};
const cloth = { id: "cloth", title: t("Cloth", "Stoff"), pattern: "lines", angleDeg: 45, spacingMm: 0.6, penMm: 0.13 };
const rep500 = {
  status: "unavailable", denominator: 500,
  reason: t("At 1:500 the figure is 3.6 mm tall: the legs of a striding person come closer than the 0.15 mm minimum gap (75 mm real) and print as one mass, and arms thinner than 0.2 mm (100 mm real) do not print.", "Bei 1:500 ist die Figur 3,6 mm hoch: Die Beine einer schreitenden Person kommen näher als der Mindestabstand von 0,15 mm (75 mm real) und drucken als eine Masse, und Arme unter 0,2 mm (100 mm real) drucken nicht."),
  limitingFeature: t("Space between the legs and arm width", "Zwischenraum der Beine und Armbreite"), limitingSizeM: 0.075,
  alternatives: [{ kind: "scale", denominator: 200 }, { kind: "symbol", symbol: "position-dot" }],
};
const FACTOR: Record<number, number> = { 200: 1.08, 100: 1, 50: 0.97 };
const round5 = (v: number) => Math.round(v * 1e5) / 1e5;
const redrawn = (s: any, denominator: number) => { const f = FACTOR[denominator]!; return s.kind === "capsule" ? { ...s, r0: round5(s.r0 * f), r1: round5(s.r1 * f) } : s.kind === "ellipse" ? { ...s, rx: round5(s.rx * f), ry: round5(s.ry * f) } : s; };
const full = (denominator: number, range: [number, number], pens: object, weave: boolean) => ({
  status: "authored", denominator, validRange: { minDenominator: range[0], maxDenominator: range[1] }, pens,
  hatches: denominator <= 100 ? [{ ...cloth, spacingMm: denominator === 50 ? 0.8 : 0.6 }] : [],
  geometry: { elevation: [...v1.geometry.elevation.filter((s: any) => s.id !== "torso-weave" && s.id !== "hair" && !(denominator === 200 && s.id === "neck")).map((s: any) => redrawn(shape(s.id, denominator <= 100 && s.id === "torso" ? { hatch: "cloth" } : {}), denominator))] },
  decisions: decisions(String(denominator), denominator <= 100 ? { hatching: ["Cloth hatch on the trunk.", "Stoffschraffur auf dem Rumpf."] } : {}),
});
const bindings = (activity: number, breathing: string) => ({
  activityMet: activity, externalWorkW: 0, breathing,
  regions: Object.fromEntries(["head", "torso", "arms", "legs", "feet"].map((id) => [id, { exposedFraction: id === "head" ? 1 : 0.9, sunlitFraction: 0 }])),
  emitters: { "body-plume": 1 },
});
const asset: any = {
  id: "sample-figure", category: "humans", title: t("Sample figure", "Beispielfigur"), summary: t("A small complete v2 test figure with four representations, one symbol, layered thermal regions, respiration and a walking clip.", "Eine kleine vollständige v2-Testfigur mit vier Darstellungen, einem Symbol, geschichteten thermischen Regionen, Atmung und einem Gehzyklus."),
  tags: ["test", "human"],
  provenance: { ...v1.provenance, author: "Semio staffage core engine", notes: t("Generated test fixture; poses and gait are authored estimates, not a verified person.", "Erzeugte Test-Fixture; Posen und Gang sind Schätzungen, keine verifizierte Person.") },
  dimensions: { heightM: 0, widthM: 0, depthM: 0.46, massKg: 75.1 },
  materials: [...v1.materials, { id: "hair", title: t("Hair", "Haar"), emissivity: 0.95, solarAbsorptivity: 0.9, source: "authored-estimate" }],
  rig: v1.rig,
  anchors: [...v1.anchors.filter((a: any) => a.id !== "mouth"), { id: "mouth", bone: "head", x: 0.07, y: -0.09, kind: "emitter", title: t("Mouth", "Mund") }, { id: "nose", bone: "head", x: 0.1, y: -0.1, kind: "emitter", title: t("Nostrils", "Nasenlöcher") }, { id: "crown", bone: "head", x: 0.1, y: 0.095, kind: "emitter", title: t("Crown", "Scheitel") }],
  representations: {
    "500": rep500,
    "200": full(200, [150, 350], { body: 0.18, detail: 0.13, texture: null, glazing: null, ground: 0.35 }, false),
    "100": full(100, [75, 150], { body: 0.35, detail: 0.18, texture: 0.13, glazing: 0.13, ground: 0.5 }, false),
    "50": full(50, [20, 75], { body: 0.5, detail: 0.25, texture: 0.13, glazing: 0.18, ground: 0.7 }, true),
  },
  symbols: [{
    id: "position-dot", title: t("Position dot", "Standpunkt"), description: t("A filled dot marking where a person stands.", "Ein gefüllter Punkt, der den Standpunkt einer Person markiert."), semantics: "true-position", units: "paper-mm", denominators: [500, 200],
    pens: { body: 0.18, detail: null, texture: null, glazing: null, ground: 0.25 }, hatches: [],
    geometry: { elevation: [{ kind: "ellipse", id: "dot", bone: "pelvis", role: "body", z: 1, filled: true, tone: 0.1, cx: 0, cy: 0, rx: 0.6, ry: 0.6 }] },
    dimensionalNote: t("The position is true, the dot size is a fixed 1.2 mm on paper and carries no dimension.", "Die Lage ist wahr, die Punktgröße ist fest 1,2 mm auf Papier und trägt kein Maß."),
  }],
  poses: v1.poses.map((p: any) => ({ ...p, thermal: bindings(p.thermal.activityMet, p.id === "stand" ? "rest" : "walk") })),
  clips: v1.clips.map((c: any) => ({
    ...c,
    events: c.events.filter((e: any) => e.kind !== "inhale" && e.kind !== "exhale"),
    tracks: c.id === "walk" ? [{ target: "region:torso:sunlitFraction", ease: "smooth", keys: [{ t: 0, v: 0 }, { t: 0.5, v: 1 }, { t: 1, v: 0 }] }]
      : c.id === "idle-breathing" ? [{ target: "breathing", ease: "step", keys: [{ t: 0, v: 0 }, { t: 2, v: 1 }] }] : c.tracks.filter((x: any) => !x.target.startsWith("emitter:breath")),
  })),
  thermal: {
    passengers: [],
    environment: { airTempC: 20, meanRadiantTempC: 20, skyTempC: 10, groundTempC: 20, relativeHumidity: 0.5, windMS: 0.2, windDirectionDeg: 180, solarWM2: 0, diffuseFraction: 0.2, pressurePa: 101325 },
    baseMet: 1.2,
    body: { species: "human", massKg: 75.1, skinAreaM2: 1.88, basalMetabolismW: 80, metWM2: 58.15, coreSetPointC: 36.8, skinSetPointC: 33.7, sweating: true, panting: false, maxSkinBloodFlowLM2H: 90, minSkinBloodFlowLM2H: 6.3, source: "literature", reference: "ashrae-2021" },
    respiration: {
      noseEmitter: "nose-breath", mouthEmitter: "mouth-breath", exhaledTempC: 34, exhaledRelativeHumidity: 0.9,
      patterns: [
        { id: "rest", title: t("Rest", "Ruhe"), rateBpm: 14, tidalVolumeL: 0.5, inhaleFraction: 0.38, pauseAfterInhaleFraction: 0.04, exhaleFraction: 0.45, pauseAfterExhaleFraction: 0.13, route: "nose", source: "authored-estimate" },
        { id: "walk", title: t("Walking", "Gehen"), rateBpm: 20, tidalVolumeL: 1.1, inhaleFraction: 0.4, pauseAfterInhaleFraction: 0, exhaleFraction: 0.5, pauseAfterExhaleFraction: 0.1, route: "nose-and-mouth", source: "authored-estimate" },
      ],
      motion: [{ bone: "torso", angleDeg: 0.8, dy: 0.003 }], source: "authored-estimate",
    },
    passengers: [],
    operating: [],
    regions: [
      ["head", "Head and neck", "Kopf und Hals", "skin", "sphere", 0.12, 0.2, 0.6, "skin", "head", 0.2, 0.07, []],
      ["torso", "Trunk", "Rumpf", "clothing", "cylinder", 0.75, 0.3, 0.5, "cotton-clothing", "trunk", 0.5, 0.37, ["clothing"]],
      ["arms", "Arms", "Arme", "clothing", "cylinder", 0.3, 0.07, 0.4, "cotton-clothing", "arm", 0.08, 0.17, ["clothing"]],
      ["legs", "Legs", "Beine", "clothing", "cylinder", 0.65, 0.1, 0.3, "cotton-clothing", "leg", 0.18, 0.33, ["clothing"]],
      ["feet", "Feet", "Füße", "clothing", "horizontal-plate-up", 0.1, 0.25, 0.1, "leather-shoe", "foot", 0.04, 0.06, []],
    ].map(([id, en, de, kind, geometry, area, length, sky, material, segment, share, skin, layers]: any) => ({
      id, title: t(en, de), kind: id === "feet" ? "clothing" : kind, model: "body", material, areaM2: area, characteristicLengthM: length, geometry, skyViewFactor: sky,
      layers: layers.length === 0 ? [] : [{ id: "garment", title: t("Garment with enclosed air", "Kleidung mit eingeschlossener Luft"), kind: "clothing", material, thicknessM: 0.003, resistanceM2KW: 0.155, vapourResistanceM2PaW: 24.7, heatCapacityJM2K: 4000, source: "literature", reference: "iso-9920-2007" }],
      heatCapacityJM2K: 15000, body: { segment, metabolicShare: share, skinFraction: skin }, source: "authored-estimate",
    })),
    emitters: [
      { id: "nose-breath", kind: "breath", title: t("Nostrils", "Nasenlöcher"), anchor: "nose", directionDeg: -150, diameterM: 0.0095, source: "authored-estimate" },
      { id: "mouth-breath", kind: "breath", title: t("Mouth", "Mund"), anchor: "mouth", directionDeg: -100, diameterM: 0.0124, source: "authored-estimate" },
      { id: "body-plume", kind: "plume", title: t("Body plume", "Körperfahne"), anchor: "crown", directionDeg: 0, diameterM: 0.3, region: "torso", source: "authored-estimate" },
    ],
    assumptions: [t("All values are authored estimates for tests.", "Alle Werte sind Schätzungen für Tests.")],
  },
  structure: v1.structure, presentation: { defaultPose: "stand", defaultClip: "walk", defaultScale: 100, viewpoints: ["elevation", "plan"] }, exports: v1.exports, completion: "authored",
};
const probe = evaluate(asset, { kind: "pose", pose: "stand" }, { kind: "scale", scale: { denominator: 100 } });
asset.dimensions.heightM = Math.round((probe.bounds.maxY - probe.bounds.minY) * 1000) / 1000;
asset.dimensions.widthM = Math.round((probe.bounds.maxX - probe.bounds.minX) * 1000) / 1000;
writeFileSync(process.argv[2]!, JSON.stringify({ $schema: "../../🧬️schema/🔣️.json#/$defs/AssetDocument", version: "semio.staffage/v2", asset }, null, 2) + "\n");
console.log(asset.dimensions);
