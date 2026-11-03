import { ALL_EXPORTS, INCROPERA, ISO10456, authored, clip, layer, level, pose, ref, tx, unavailable, wrap } from "./lib.mjs";
import { materialOf, space, sum } from "./space.mjs";

const SRC = "authored-estimate";
const STEEL = "powder-coated-steel";

const m = (kg, comX, comY) => ({ kg, comX, comY, distribution: "point", source: SRC });
const fixed = { type: "fixed" };

const sp = space([
  ["base", null, 0, 0, { title: tx("Carriage base with castors", "Fahrgestell mit Lenkrollen"), lengthM: 0.8, joint: fixed, mass: m(120, 0, 0.08) }],
  ["column", "base", 0, 0.16, { title: tx("Telescopic column", "Teleskopsäule"), lengthM: 0.62, joint: fixed, mass: m(42, 0, 0.3) }],
  ["top", "column", 0, 0.78, { title: tx("Table top with pad", "Tischplatte mit Polster"), lengthM: 2, joint: { type: "revolute", minDeg: -25, maxDeg: 25 }, mass: m(48, 0, 0.07) }],
  ["stand-base", "base", 0.62, 0, { title: tx("Light stand base with castors", "Leuchtenfuß mit Lenkrollen"), lengthM: 0.56, joint: fixed, mass: m(18, 0, 0.06) }],
  ["stand-column", "stand-base", 0.62, 0.1, { title: tx("Light stand column", "Leuchtensäule"), lengthM: 1.86, joint: fixed, mass: m(6, 0, 0.9) }],
  ["stand-arm", "stand-column", 0.62, 1.92, { title: tx("Spring arm", "Federarm"), lengthM: 0.5, joint: fixed, mass: m(5, -0.25, 0) }],
  ["light-head", "stand-arm", 0.12, 1.66, { title: tx("Surgical light head", "Operationsleuchtenkopf"), lengthM: 0.5, joint: fixed, mass: m(9, 0, 0.06) }],
]);
const { rect, ellipse, poly, path } = sp;

const HULL = [[-0.4, 0.07], [0.4, 0.07], [0.4, 0.12], [0.3, 0.16], [-0.3, 0.16], [-0.4, 0.12]];
const ST_HULL = [[0.34, 0.06], [0.9, 0.06], [0.86, 0.105], [0.38, 0.105]];
const HOUSING = "M -0.13 1.675 L 0.37 1.675 L 0.37 1.70 C 0.37 1.76 0.24 1.78 0.12 1.78 C 0 1.78 -0.13 1.76 -0.13 1.70 Z";
const HOUSING_PLAIN = "M -0.13 1.66 L 0.37 1.66 L 0.37 1.70 C 0.37 1.76 0.24 1.78 0.12 1.78 C 0 1.78 -0.13 1.76 -0.13 1.70 Z";

const rep50 = authored(
  50,
  [20, 75],
  { body: 0.18, detail: 0.13, texture: null, glazing: null, ground: 0.35 },
  [
    poly("st-base", "stand-base", "body", 2, true, 0.45, ST_HULL, true, { region: "stand", material: STEEL }),
    ellipse("st-castor-a", "stand-base", "body", 2.1, true, 0.2, 0.4, 0.03, 0.03, 0.03, { region: "stand", material: "rubber-castor" }),
    ellipse("st-castor-b", "stand-base", "body", 2.1, true, 0.2, 0.84, 0.03, 0.03, 0.03, { region: "stand", material: "rubber-castor" }),
    rect("st-column", "stand-column", "body", 2.2, true, 0.5, 0.59, 0.105, 0.06, 1.775, { region: "stand", material: STEEL }),
    rect("st-collar", "stand-column", "body", 2.3, true, 0.4, 0.575, 1.88, 0.09, 0.08, { region: "stand", material: STEEL }),
    rect("st-arm", "stand-arm", "body", 2.4, true, 0.5, 0.12, 1.9, 0.5, 0.04, { region: "stand", material: STEEL }),
    ellipse("st-knob", "stand-arm", "body", 2.45, true, 0.35, 0.12, 1.92, 0.03, 0.03, { region: "stand", material: STEEL }),
    rect("st-rod", "stand-arm", "body", 2.5, true, 0.5, 0.108, 1.78, 0.024, 0.12, { region: "stand", material: STEEL }),
    path("head-housing", "light-head", "body", 2.6, true, 0.35, HOUSING, { region: "light-housing", material: "light-aluminium" }),
    rect("head-lens", "light-head", "body", 2.65, true, 0.9, -0.12, 1.66, 0.48, 0.015, { region: "light-lens", material: "light-lens-glass" }),
    rect("head-handle", "light-head", "body", 2.55, true, 0.4, 0.1, 1.6, 0.04, 0.06, { region: "light-housing", material: "light-aluminium" }),
    poly("head-groove", "light-head", "detail", 2.7, false, 0.3, [[-0.12, 1.715], [0.36, 1.715]], false),
    poly("tb-plate", "base", "body", 3, true, 0.5, HULL, true, { region: "table-base", material: STEEL }),
    ellipse("tb-castor-a", "base", "body", 3.1, true, 0.2, -0.32, 0.04, 0.04, 0.04, { region: "table-base", material: "rubber-castor" }),
    ellipse("tb-castor-b", "base", "body", 3.1, true, 0.2, 0.32, 0.04, 0.04, 0.04, { region: "table-base", material: "rubber-castor" }),
    poly("col-lower", "column", "body", 3.2, true, 0.6, [[-0.15, 0.16], [0.15, 0.16], [0.13, 0.4], [-0.13, 0.4]], true, { region: "table-column", material: "stainless-steel" }),
    rect("col-upper", "column", "body", 3.3, true, 0.7, -0.105, 0.4, 0.21, 0.34, { region: "table-column", material: "stainless-steel" }),
    rect("col-head", "column", "body", 3.4, true, 0.5, -0.16, 0.74, 0.32, 0.04, { region: "table-frame", material: "stainless-steel" }),
    ...[0.46, 0.52, 0.58, 0.64, 0.7].map((y, i) => poly(`pleat-${i + 1}`, "column", "detail", 3.35, false, 0.3, [[-0.105, y], [0.105, y]], false)),
    rect("top-carrier", "top", "body", 3.5, true, 0.55, -0.98, 0.78, 1.96, 0.04, { region: "table-frame", material: "stainless-steel" }),
    rect("top-pad", "top", "body", 3.6, true, 0.3, -1, 0.82, 2, 0.08, { r: 0.03, region: "table-pad", material: "pad-foam" }),
    ...[-0.62, -0.05, 0.2].map((x, i) => poly(`pad-joint-${i + 1}`, "top", "detail", 3.7, false, 0.25, [[x, 0.82], [x, 0.9]], false)),
  ],
  {
    silhouette: ["A 2.0 m long, 1.96 m high sign, 40 mm by 39 mm on paper: a flat pad on a stainless carrier and a telescopic column over a low carriage, and a mobile light stand behind it whose spring arm carries a domed lamp head over the middle of the table.", "Ein 2,0 m langes und 1,96 m hohes Zeichen, auf Papier 40 mm mal 39 mm: ein flaches Polster auf einem Edelstahlträger und eine Teleskopsäule über einem niedrigen Fahrgestell, dahinter ein fahrbarer Leuchtenständer, dessen Federarm einen gewölbten Leuchtenkopf über die Tischmitte trägt."],
    retained: ["Overall length 2.0 m, top surface 0.90 m, pad thickness 0.08 m, carriage 0.8 m with two castors, bellows column with five pleats, the four pad sections, the lamp head 0.5 m wide with lens and sterile handle, the stand column and the spring arm.", "Gesamtlänge 2,0 m, Oberfläche 0,90 m, Polsterstärke 0,08 m, Fahrgestell 0,8 m mit zwei Lenkrollen, Balgsäule mit fünf Falten, die vier Polsterabschnitte, der Leuchtenkopf von 0,5 m Breite mit Linse und Sterilgriff, Leuchtensäule und Federarm."],
    merged: ["The accessory rail and the carrier are one stainless strip; the hull of the stand base and its five-star foot read as one plate.", "Zubehörschiene und Träger sind ein Edelstahlstreifen; Gehäuse und Sternfuß des Leuchtenständers lesen sich als eine Platte."],
    omitted: ["Armrests, leg-section hinges, the brake pedals, cables, the control handset and the counterweight of the spring arm.", "Armstützen, Gelenke der Beinplatte, Bremspedale, Kabel, die Handbedienung und das Gegengewicht des Federarms."],
    thinElements: ["The rod to the lamp head is 24 mm (0.48 mm on paper) and the pleat lines are 0.13 mm detail strokes.", "Die Stange zum Leuchtenkopf ist 24 mm breit (0,48 mm auf Papier) und die Faltenlinien sind 0,13-mm-Detailstriche."],
    hatching: ["None; the lens is a light fill instead.", "Keine; stattdessen ist die Linse hell gefüllt."],
    styles: ["Pad 0.3, stainless parts 0.5 to 0.7, powder-coated stand 0.45 to 0.5, castors 0.2, lamp housing 0.35, lens 0.9; body pen 0.18 mm, detail 0.13 mm, ground 0.35 mm.", "Polster 0,3, Edelstahlteile 0,5 bis 0,7, pulverbeschichteter Ständer 0,45 bis 0,5, Rollen 0,2, Leuchtengehäuse 0,35, Linse 0,9; Körperstift 0,18 mm, Detail 0,13 mm, Boden 0,35 mm."],
    overlays: ["Pad, frame, column, carriage, stand, lamp housing and lens are separate regions: in the thermal view only the lamp head glows, the table stays at room temperature.", "Polster, Rahmen, Säule, Fahrgestell, Ständer, Leuchtengehäuse und Linse sind getrennte Regionen: in der Thermalansicht glüht nur der Leuchtenkopf, der Tisch bleibt auf Raumtemperatur."],
  },
);

const rep100 = authored(
  100,
  [75, 150],
  { body: 0.18, detail: null, texture: null, glazing: null, ground: 0.25 },
  [
    poly("st-base", "stand-base", "body", 2, true, 0.45, ST_HULL, true, { region: "stand", material: STEEL }),
    ellipse("st-castor-a", "stand-base", "body", 2.1, true, 0.2, 0.4, 0.03, 0.03, 0.03, { region: "stand", material: "rubber-castor" }),
    ellipse("st-castor-b", "stand-base", "body", 2.1, true, 0.2, 0.84, 0.03, 0.03, 0.03, { region: "stand", material: "rubber-castor" }),
    rect("st-column", "stand-column", "body", 2.2, true, 0.5, 0.585, 0.105, 0.07, 1.775, { region: "stand", material: STEEL }),
    rect("st-arm", "stand-arm", "body", 2.4, true, 0.5, 0.11, 1.9, 0.52, 0.045, { region: "stand", material: STEEL }),
    rect("st-rod", "stand-arm", "body", 2.5, true, 0.5, 0.105, 1.78, 0.03, 0.12, { region: "stand", material: STEEL }),
    path("head-housing", "light-head", "body", 2.6, true, 0.35, HOUSING, { region: "light-housing", material: "light-aluminium" }),
    rect("head-lens", "light-head", "body", 2.65, true, 0.9, -0.12, 1.655, 0.48, 0.02, { region: "light-lens", material: "light-lens-glass" }),
    rect("head-handle", "light-head", "body", 2.55, true, 0.4, 0.1, 1.6, 0.04, 0.06, { region: "light-housing", material: "light-aluminium" }),
    poly("tb-plate", "base", "body", 3, true, 0.5, HULL, true, { region: "table-base", material: STEEL }),
    ellipse("tb-castor-a", "base", "body", 3.1, true, 0.2, -0.32, 0.04, 0.04, 0.04, { region: "table-base", material: "rubber-castor" }),
    ellipse("tb-castor-b", "base", "body", 3.1, true, 0.2, 0.32, 0.04, 0.04, 0.04, { region: "table-base", material: "rubber-castor" }),
    poly("col-lower", "column", "body", 3.2, true, 0.6, [[-0.15, 0.16], [0.15, 0.16], [0.13, 0.4], [-0.13, 0.4]], true, { region: "table-column", material: "stainless-steel" }),
    rect("col-upper", "column", "body", 3.3, true, 0.7, -0.105, 0.4, 0.21, 0.34, { region: "table-column", material: "stainless-steel" }),
    rect("col-head", "column", "body", 3.4, true, 0.5, -0.16, 0.74, 0.32, 0.04, { region: "table-frame", material: "stainless-steel" }),
    rect("top-carrier", "top", "body", 3.5, true, 0.55, -0.98, 0.78, 1.96, 0.04, { region: "table-frame", material: "stainless-steel" }),
    rect("top-pad", "top", "body", 3.6, true, 0.3, -1, 0.82, 2, 0.08, { r: 0.03, region: "table-pad", material: "pad-foam" }),
  ],
  {
    silhouette: ["A 20 mm by 19.6 mm sign: a slab on a column over a low carriage with a stand and a round lamp head hanging in front of it.", "Ein Zeichen von 20 mm mal 19,6 mm: eine Platte auf einer Säule über einem niedrigen Fahrgestell, dazu ein Ständer mit einem runden Leuchtenkopf, der davor hängt."],
    retained: ["Length 2.0 m, top 0.90 m, pad and carrier as two strips, carriage with two castors, column in two parts, lamp head 0.5 m wide with lens and handle, stand column and arm.", "Länge 2,0 m, Oberfläche 0,90 m, Polster und Träger als zwei Streifen, Fahrgestell mit zwei Rollen, Säule in zwei Teilen, Leuchtenkopf von 0,5 m Breite mit Linse und Griff, Ständersäule und Arm."],
    merged: ["The column collar and the bellows form two trapezoids; the stand collar joins the column.", "Säulenmanschette und Balg bilden zwei Trapeze; der Kragen des Ständers geht in die Säule auf."],
    omitted: ["Pleat lines, pad joints, the lamp groove, the arm knob, the collar and every accessory.", "Faltenlinien, Polsterfugen, die Leuchtenrille, der Armknauf, der Kragen und jedes Zubehör."],
    thinElements: ["The rod is widened from 24 to 30 mm and the stand column from 60 to 70 mm so that fill and 0.18 mm pen reach the feature width.", "Die Stange ist von 24 auf 30 mm und die Ständersäule von 60 auf 70 mm verbreitert, damit Füllung und 0,18-mm-Strich die Mindestbreite erreichen."],
    hatching: ["None.", "Keine."],
    styles: ["Pad 0.3, stainless 0.5 to 0.7, stand 0.45 to 0.5, castors 0.2, housing 0.35, lens 0.9; one body pen of 0.18 mm.", "Polster 0,3, Edelstahl 0,5 bis 0,7, Ständer 0,45 bis 0,5, Rollen 0,2, Gehäuse 0,35, Linse 0,9; ein Körperstift von 0,18 mm."],
    overlays: ["Regions stay separate, so the lamp head carries the heat of the light and the table stays cool.", "Die Regionen bleiben getrennt, sodass der Leuchtenkopf die Wärme der Leuchte trägt und der Tisch kühl bleibt."],
  },
);

const rep200 = authored(
  200,
  [150, 350],
  { body: 0.18, detail: null, texture: null, glazing: null, ground: 0.25 },
  [
    poly("st-base", "stand-base", "body", 2, true, 0.45, [[0.34, 0], [0.9, 0], [0.86, 0.105], [0.38, 0.105]], true, { region: "stand", material: STEEL }),
    rect("st-column", "stand-column", "body", 2.2, true, 0.5, 0.58, 0.105, 0.08, 1.795, { region: "stand", material: STEEL }),
    rect("st-arm", "stand-arm", "body", 2.4, true, 0.5, 0.1, 1.89, 0.54, 0.06, { region: "stand", material: STEEL }),
    rect("st-rod", "stand-arm", "body", 2.5, true, 0.5, 0.1, 1.78, 0.04, 0.12, { region: "stand", material: STEEL }),
    path("head", "light-head", "body", 2.6, true, 0.4, HOUSING_PLAIN, { regions: ["light-housing", "light-lens"], material: "light-aluminium" }),
    poly("tb-plate", "base", "body", 3, true, 0.5, [[-0.4, 0], [0.4, 0], [0.4, 0.12], [0.3, 0.16], [-0.3, 0.16], [-0.4, 0.12]], true, { region: "table-base", material: STEEL }),
    poly("col", "column", "body", 3.2, true, 0.65, [[-0.16, 0.16], [0.16, 0.16], [0.11, 0.78], [-0.11, 0.78]], true, { region: "table-column", material: "stainless-steel" }),
    rect("top", "top", "body", 3.6, true, 0.35, -1, 0.78, 2, 0.12, { regions: ["table-pad", "table-frame"], material: "pad-foam" }),
  ],
  {
    silhouette: ["A 10 mm by 9.8 mm sign: a slab on a column over a plate, with a stand and a lamp dome next to it.", "Ein Zeichen von 10 mm mal 9,8 mm: eine Platte auf einer Säule über einer Grundplatte, daneben ein Ständer mit einer Leuchtenkuppel."],
    retained: ["Length 2.0 m, top surface 0.90 m, column, carriage plate, stand with arm and the 0.5 m lamp dome with its hanging rod.", "Länge 2,0 m, Oberfläche 0,90 m, Säule, Fahrgestellplatte, Ständer mit Arm und die Leuchtenkuppel von 0,5 m mit ihrer Hängestange."],
    merged: ["Pad and carrier are one 0.12 m slab (regions pad and frame merged); castors join the carriage plate; lens and housing are one dome (regions housing and lens merged); collar, knob and handle join their parts.", "Polster und Träger sind eine 0,12 m dicke Platte (Regionen Polster und Rahmen zusammengeführt); Rollen gehen in der Fahrgestellplatte auf; Linse und Gehäuse sind eine Kuppel (Regionen Gehäuse und Linse zusammengeführt); Kragen, Knauf und Griff gehen in ihren Teilen auf."],
    omitted: ["Castors, pleats, pad joints, the lamp groove, the handle and the arm knob.", "Rollen, Falten, Polsterfugen, die Leuchtenrille, der Griff und der Armknauf."],
    thinElements: ["The stand column is widened from 60 to 80 mm, the arm from 40 to 60 mm and the rod from 24 to 40 mm (0.2 mm on paper).", "Die Ständersäule ist von 60 auf 80 mm, der Arm von 40 auf 60 mm und die Stange von 24 auf 40 mm (0,2 mm auf Papier) verbreitert."],
    hatching: ["None.", "Keine."],
    styles: ["Slab 0.35, column 0.65, carriage 0.5, stand 0.5, lamp dome 0.4; one body pen of 0.18 mm.", "Platte 0,35, Säule 0,65, Fahrgestell 0,5, Ständer 0,5, Leuchtenkuppel 0,4; ein Körperstift von 0,18 mm."],
    overlays: ["The slab shape lists pad and frame as merged regions and the dome lists housing and lens, so the thermal view still marks the lamp as the warm part.", "Die Plattenform führt Polster und Rahmen als zusammengeführte Regionen und die Kuppel Gehäuse und Linse, sodass die Thermalansicht die Leuchte weiter als warmen Teil markiert."],
  },
);

const rep500 = unavailable(
  500,
  ["At 1:500 the 2.0 m table is 4 mm long, its stand column 0.12 mm wide and the pad 0.16 mm thick, below every print profile.", "Bei 1:500 ist der 2,0 m lange Tisch 4 mm lang, seine Ständersäule 0,12 mm breit und das Polster 0,16 mm dick, unter jedem Druckprofil."],
  ["Stand column of 60 mm", "Ständersäule von 60 mm"],
  0.06,
  [{ kind: "scale", denominator: 200 }, { kind: "scale", denominator: 100 }],
);

const single = (id, en, de, kind, material, thicknessM, res, cap, reference = "iso-10456-2007", source = SRC) => layer(id, [en, de], kind, material, thicknessM, res, cap, source, reference);

const region = (id, title, kind, model, material, areaM2, characteristicLengthM, geometry, skyViewFactor, layers, heatCapacityJM2K, extra = {}, reference = "iso-10456-2007") => ({
  id,
  title: tx(...title),
  kind,
  model,
  material,
  areaM2,
  characteristicLengthM,
  geometry,
  skyViewFactor,
  layers,
  heatCapacityJM2K,
  ...extra,
  source: SRC,
  reference,
});

const regions = [
  region("table-pad", ["Table pad (vinyl over foam)", "Tischpolster (Vinyl über Schaum)"], "surface", "passive", "pad-foam", 0.9, 0.45, "horizontal-plate-up", 0.5, [single("vinyl", "PU-coated vinyl 1 mm", "PU-beschichtetes Vinyl 1 mm", "polymer", "pad-foam", 0.001, 0.00625, 1500), single("foam", "Polyurethane foam 70 mm", "Polyurethanschaum 70 mm", "insulation", "pad-foam", 0.07, 1.75, 3150)], 3150),
  region("table-frame", ["Carrier, rails and column head", "Träger, Schienen und Säulenkopf"], "surface", "passive", "stainless-steel", 0.6, 0.3, "horizontal-plate-down", 0.2, [single("carrier", "Stainless steel 2 mm", "Edelstahl 2 mm", "metal", "stainless-steel", 0.002, 0.000133, 7900)], 7900),
  region("table-column", ["Telescopic column cover", "Hülle der Teleskopsäule"], "surface", "passive", "stainless-steel", 0.45, 0.6, "cylinder", 0.3, [single("cover", "Stainless steel and polymer bellows 2 mm", "Edelstahl und Kunststoffbalg 2 mm", "metal", "stainless-steel", 0.002, 0.000133, 7900)], 7900),
  region("table-base", ["Carriage and castors", "Fahrgestell und Rollen"], "surface", "passive", STEEL, 0.9, 0.8, "horizontal-plate-up", 0.3, [single("shell", "Powder-coated steel 3 mm", "Pulverbeschichteter Stahl 3 mm", "metal", STEEL, 0.003, 0.00006, 11700)], 11700),
  region("stand", ["Light stand, arm and base", "Leuchtenständer, Arm und Fuß"], "surface", "passive", STEEL, 0.75, 1.9, "cylinder", 0.5, [single("tube", "Powder-coated steel tube 2.5 mm", "Pulverbeschichtetes Stahlrohr 2,5 mm", "metal", STEEL, 0.0025, 0.00005, 9750)], 9750),
  region("light-housing", ["Lamp head housing", "Gehäuse des Leuchtenkopfes"], "lamp", "operating", "light-aluminium", 0.45, 0.5, "sphere", 0.5, [single("housing", "Die-cast aluminium 3 mm", "Aluminiumdruckguss 3 mm", "metal", "light-aluminium", 0.003, 0.000014, 7290)], 7290, { operating: { key: "light", heatW: 90 } }, "incropera-2018"),
  region("light-lens", ["Lamp head lens", "Linse des Leuchtenkopfes"], "lamp", "operating", "light-lens-glass", 0.19, 0.48, "horizontal-plate-down", 0.2, [single("lens", "Polycarbonate lens 4 mm", "Polycarbonatlinse 4 mm", "polymer", "light-lens-glass", 0.004, 0.02, 4800)], 4800, { operating: { key: "light", heatW: 30 } }, "incropera-2018"),
];

const bind = (light, plume) => ({
  regions: {
    "table-pad": { exposedFraction: 0.85, sunlitFraction: 0 },
    "table-frame": { exposedFraction: 1, sunlitFraction: 0 },
    "table-column": { exposedFraction: 1, sunlitFraction: 0 },
    "table-base": { exposedFraction: 0.6, sunlitFraction: 0, contact: { fraction: 0.2, tempC: 21 } },
    stand: { exposedFraction: 1, sunlitFraction: 0 },
    "light-housing": { exposedFraction: 1, sunlitFraction: 0 },
    "light-lens": { exposedFraction: 1, sunlitFraction: 0 },
  },
  emitters: { "light-plume": plume },
  operating: { light },
});

const castors = ["castors-table", "castors-stand"];
const poses = [
  pose("light-off", ["Light off", "Leuchte aus"], ["The table is flat at 0.90 m and the surgical light is switched off: everything is at room temperature.", "Der Tisch steht eben auf 0,90 m und die Operationsleuchte ist ausgeschaltet: alles hat Raumtemperatur."], {}, castors, "auto", bind(0, 0)),
  pose("light-on", ["Light on", "Leuchte an"], ["The table is flat and the surgical light shines at full power: the lamp head releases 120 W and a plume rises over it.", "Der Tisch steht eben und die Operationsleuchte leuchtet mit voller Leistung: der Leuchtenkopf gibt 120 W ab und eine Fahne steigt darüber auf."], {}, castors, "auto", bind(1, 1)),
  pose("head-down", ["Head down 12 degrees", "Kopftieflage 12 Grad"], ["The top is tilted by 12 degrees about its pivot so the head end is lower than the foot end, the light stays on.", "Die Platte ist um 12 Grad um ihren Drehpunkt gekippt, sodass das Kopfende tiefer liegt als das Fußende; die Leuchte bleibt an."], { top: { angleDeg: 12 } }, castors, "auto", bind(1, 1)),
];

const contactsFor = (d) => castors.map((anchor) => ({ anchor, fromS: 0, toS: d }));

const clips = [
  clip(
    "switch-on-warm-up",
    ["Switch on and warm up", "Einschalten und aufwärmen"],
    ["The surgical light is switched on after five seconds: the lens warms within a minute and the housing within ten minutes while a plume builds up above the head.", "Die Operationsleuchte wird nach fünf Sekunden eingeschaltet: die Linse wärmt sich binnen einer Minute und das Gehäuse binnen zehn Minuten auf, während über dem Kopf eine Fahne entsteht."],
    600,
    1,
    [{ t: 0, pose: "light-off", ease: "step" }, { t: 5, pose: "light-on", ease: "step" }, { t: 600, pose: "light-on", ease: "step" }],
    contactsFor(600),
    [["cold", 0, 5, ["Off", "Aus"]], ["warm-up", 5, 600, ["Lit, warming up", "Leuchtet, wärmt auf"]]],
    [[5, "operating", ["Light switched on", "Leuchte eingeschaltet"], "light"]],
    [["operating:light", "step", [[0, 0], [5, 1]]], ["emitter:light-plume", "step", [[0, 0], [5, 1]]]],
  ),
  clip(
    "tilt-table",
    ["Tilt the table head down", "Tisch in Kopftieflage kippen"],
    ["The top tilts from flat to 12 degrees head down in six seconds, stays for four seconds and returns, the light stays on.", "Die Platte kippt in sechs Sekunden von eben auf 12 Grad Kopftieflage, bleibt vier Sekunden stehen und kehrt zurück; die Leuchte bleibt an."],
    20,
    15,
    [{ t: 0, pose: "light-on", ease: "smooth" }, { t: 6, pose: "head-down", ease: "smooth" }, { t: 10, pose: "head-down", ease: "smooth" }, { t: 16, pose: "light-on", ease: "smooth" }, { t: 20, pose: "light-on", ease: "smooth" }],
    contactsFor(20),
    [["tilt", 0, 6, ["Tilting", "Kippen"]], ["hold", 6, 10, ["Held", "Halten"]], ["return", 10, 16, ["Returning", "Zurückkehren"]], ["rest", 16, 20, ["Flat", "Eben"]]],
    [[0, "motion", ["Tilt starts", "Kippen beginnt"]], [6, "motion", ["Head-down position reached", "Kopftieflage erreicht"]]],
    [["operating:light", "step", [[0, 1], [20, 1]]], ["emitter:light-plume", "step", [[0, 1], [20, 1]]]],
  ),
];


export const table = () => {
  const massKg = sum(sp.bones.map((b) => b.mass.kg));
  return wrap({
    id: "operating-table",
    category: "furniture",
    title: tx("Operating Table with Surgical Light", "Operationstisch mit Operationsleuchte"),
    summary: tx(
      "A mobile operating table 2.0 m long and 0.5 m wide with a 0.90 m high padded top on a telescopic column and a mobile stand whose spring arm carries a 120 W surgical light over the table; the top tilts to head down, the lamp head is the only warm surface.",
      "Ein fahrbarer Operationstisch von 2,0 m Länge und 0,5 m Breite mit einer 0,90 m hohen gepolsterten Platte auf einer Teleskopsäule und ein fahrbarer Ständer, dessen Federarm eine 120-W-Operationsleuchte über den Tisch trägt; die Platte kippt in Kopftieflage, der Leuchtenkopf ist die einzige warme Fläche.",
    ),
    tags: ["operating-table", "operating-theatre", "surgical-light", "hospital", "furniture", "internal-gain", "electrical"],
    provenance: {
      author: "Semio staffage content (A8)",
      method: "handcrafted",
      created: "2026-11-01",
      references: [
        ref("iec-60601-2-46", "IEC 60601-2-46: Medical electrical equipment - Part 2-46: Particular requirements for the basic safety and essential performance of operating tables (table top size class, height range, tilt)."),
        ref("iec-60601-2-41", "IEC 60601-2-41: Medical electrical equipment - Part 2-41: Particular requirements for the basic safety and essential performance of surgical luminaires and luminaires for diagnosis (head size, working distance, power)."),
        ref("ashrae-170-2021", "ANSI/ASHRAE/ASHE Standard 170-2021: Ventilation of Health Care Facilities (operating rooms: air temperature 20 to 24 degrees Celsius, equipment heat gains in surgical suites)."),
        INCROPERA,
        ISO10456,
      ],
      notes: tx(
        "Side elevation of a mobile operating table and a mobile surgical light stand, the table top surface at 0.90 m (adjustable between 0.8 and 1.0 m on the real column, drawn at 0.90 m), 2.0 m long and 0.5 m wide (IEC 60601-2-46 class), a pad of 0.08 m on a 0.04 m stainless carrier, a bellows column and a 0.8 m carriage on two visible castors. The stand stands 0.62 m beside the table centre with a 1.96 m high column and a 0.5 m spring arm that carries a lamp head of 0.5 m diameter with its lens 1.66 m above the floor (0.76 m working distance, IEC 60601-2-41). Dimensions and masses (table 210 kg, stand 38 kg) are rounded from this product class (authored estimate). The origin is on the floor under the table column; the table top tilts about the pivot on the column head by up to 25 degrees. The light takes 120 W at level 1, which ends entirely as heat in the room (an LED or halogen head converts almost all of its electrical power into heat and absorbed light); the asset releases it at the head: 90 W at the housing and 30 W at the lens, a split that is an authored estimate.",
        "Seitenansicht eines fahrbaren Operationstisches und eines fahrbaren Operationsleuchtenständers, Oberfläche der Platte auf 0,90 m (an der echten Säule zwischen 0,8 und 1,0 m verstellbar, gezeichnet bei 0,90 m), 2,0 m lang und 0,5 m breit (Klasse nach IEC 60601-2-46), ein Polster von 0,08 m auf einem 0,04 m starken Edelstahlträger, eine Balgsäule und ein 0,8 m langes Fahrgestell mit zwei sichtbaren Rollen. Der Ständer steht 0,62 m neben der Tischmitte mit einer 1,96 m hohen Säule und einem 0,5 m langen Federarm, der einen Leuchtenkopf von 0,5 m Durchmesser trägt, dessen Linse 1,66 m über dem Boden liegt (0,76 m Arbeitsabstand, IEC 60601-2-41). Maße und Massen (Tisch 210 kg, Ständer 38 kg) sind aus dieser Produktklasse gerundet (Schätzwert). Der Ursprung liegt auf dem Boden unter der Tischsäule; die Tischplatte kippt um den Drehpunkt auf dem Säulenkopf bis 25 Grad. Die Leuchte nimmt auf Stufe 1 120 W auf, die vollständig als Wärme im Raum enden (ein LED- oder Halogenkopf wandelt fast die gesamte elektrische Leistung in Wärme und absorbiertes Licht um); das Asset gibt sie am Kopf ab: 90 W am Gehäuse und 30 W an der Linse, eine Aufteilung als Schätzwert.",
      ),
    },
    dimensions: { heightM: 1.96, widthM: 2.0, depthM: 0.6, massKg },
    materials: [
      materialOf("pad-foam", "PU-coated vinyl over polyurethane foam", "PU-beschichtetes Vinyl über Polyurethanschaum", 0.9, 0.5, 40, 0.04, 1500, SRC, "iso-10456-2007"),
      materialOf("stainless-steel", "Brushed stainless steel", "Gebürsteter Edelstahl", 0.2, 0.5, 7900, 15, 500, "literature", "incropera-2018"),
      materialOf(STEEL, "Powder-coated steel", "Pulverbeschichteter Stahl", 0.9, 0.5, 7850, 50, 450, SRC, "incropera-2018"),
      materialOf("rubber-castor", "Conductive rubber castor tyre", "Leitfähiger Gummireifen der Rolle", 0.9, 0.9, 1200, 0.25, 1800, SRC, "iso-10456-2007"),
      materialOf("light-aluminium", "Painted die-cast aluminium", "Lackierter Aluminiumdruckguss", 0.85, 0.4, 2700, 160, 900, SRC, "incropera-2018"),
      materialOf("light-lens-glass", "Clear polycarbonate lens", "Klare Polycarbonatlinse", 0.9, 0.1, 1200, 0.2, 1200, SRC, "incropera-2018"),
    ],
    rig: {
      kind: "mechanical",
      root: "base",
      bones: sp.bones.map((b) => ({ ...b, mass: { ...b.mass } })),
    },
    anchors: [
      { id: "castors-table", bone: "base", x: 0, y: 0, kind: "contact", title: tx("Table castors on the floor", "Tischrollen auf dem Boden"), support: "contact", normalDeg: 90, footprintLengthM: 0.64, footprintWidthM: 0.5, frictionCoefficient: 0.5 },
      { id: "castors-stand", bone: "stand-base", x: 0, y: 0, kind: "contact", title: tx("Light stand castors on the floor", "Ständerrollen auf dem Boden"), support: "contact", normalDeg: 90, footprintLengthM: 0.5, footprintWidthM: 0.5, frictionCoefficient: 0.5 },
      { id: "light-top", bone: "light-head", x: 0, y: 0.115, kind: "emitter", title: tx("Top of the lamp head", "Scheitel des Leuchtenkopfes") },
    ],
    representations: { 500: rep500, 200: rep200, 100: rep100, 50: rep50 },
    symbols: [],
    poses,
    clips,
    thermal: {
      environment: { airTempC: 21, meanRadiantTempC: 21, relativeHumidity: 0.5, windMS: 0.1, windDirectionDeg: 0, solarWM2: 0, diffuseFraction: 0, pressurePa: 101325 },
      passengers: [],
      operating: [{ id: "light", title: tx("Surgical light", "Operationsleuchte"), levels: [level(0, "Off", "Aus"), level(0.5, "Half power (60 W)", "Halbe Leistung (60 W)"), level(1, "Full power (120 W)", "Volle Leistung (120 W)")] }],
      regions,
      emitters: [{ id: "light-plume", kind: "plume", title: tx("Hot air rising over the lamp head", "Heißluft über dem Leuchtenkopf"), anchor: "light-top", directionDeg: 90, diameterM: 0.4, region: "light-housing", source: SRC, reference: "incropera-2018" }],
      assumptions: [
        tx(
          "The 120 W of the light end entirely as heat in the room: 90 W at the housing and 30 W at the lens at level 1; the light output is absorbed by the drapes and the patient and counted in the surfaces of the room.",
          "Die 120 W der Leuchte enden vollständig als Wärme im Raum: 90 W am Gehäuse und 30 W an der Linse auf Stufe 1; die Lichtleistung wird von den Abdeckungen und dem Patienten absorbiert und in den Flächen des Raumes mitgezählt.",
        ),
        tx(
          "The table carries no patient and no electrical load; the heat of a patient is the business of the person asset placed on it.",
          "Der Tisch trägt keinen Patienten und keine elektrische Last; die Wärme eines Patienten gehört zum Personen-Asset, das darauf gelegt wird.",
        ),
      ],
    },
    structure: {
      gravityMS2: 9.80665,
      dynamics: "quasi-static",
      supports: castors,
      assumptions: [
        tx(
          "A rigid assembly standing on the table castors and the stand castors; the locked castors carry the weight, the tilted top shifts the centre of mass inside the carriage footprint.",
          "Eine starre Baugruppe auf den Tischrollen und den Ständerrollen; die arretierten Rollen tragen das Gewicht, die gekippte Platte verschiebt den Schwerpunkt innerhalb der Aufstandsfläche des Fahrgestells.",
        ),
      ],
      clinical: false,
    },
    presentation: { defaultPose: "light-on", defaultClip: "switch-on-warm-up", defaultScale: 50, viewpoints: ["elevation"] },
    exports: ALL_EXPORTS,
    completion: "authored",
  });
};

