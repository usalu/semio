import { ALL_EXPORTS, INCROPERA, ISO10456, authored, clip, layer, level, pose, ref, tx, unavailable, wrap } from "./lib.mjs";
import { materialOf, space, sum } from "./space.mjs";

const SRC = "authored-estimate";
const m = (kg, comX, comY) => ({ kg, comX, comY, distribution: "point", source: SRC });
const fixed = { type: "fixed" };

const sp = space([
  ["canopy", null, 0, 0, { title: tx("Canopy with motor and filter", "Haube mit Motor und Filter"), lengthM: 0.5, joint: fixed, mass: m(8.5, 0.22, 0.1) }],
  ["chimney", "canopy", 0, 0.2, { title: tx("Lower chimney tube", "Unteres Kaminrohr"), lengthM: 0.35, joint: fixed, mass: m(3.2, 0.15, 0.17) }],
  ["chimney-top", "chimney", 0, 0.55, { title: tx("Upper chimney tube", "Oberes Kaminrohr"), lengthM: 0.365, joint: fixed, mass: m(2.8, 0.14, 0.17) }],
]);
const { rect, poly } = sp;

const CANOPY = [[0, 0.02], [0.5, 0.02], [0.5, 0.06], [0.4, 0.2], [0, 0.2]];

const rep50 = authored(
  50,
  [20, 75],
  { body: 0.18, detail: 0.13, texture: null, glazing: null, ground: 0.35 },
  [
    poly("canopy", "canopy", "body", 3, true, 0.55, CANOPY, true, { region: "canopy", material: "stainless-steel" }),
    rect("filter", "canopy", "body", 3.1, true, 0.7, 0.02, 0, 0.46, 0.02, { region: "filter", material: "filter-mesh" }),
    rect("lamp-a", "canopy", "detail", 3.2, true, 0.95, 0.07, 0, 0.06, 0.012, { region: "lamps", material: "lamp-diffuser" }),
    rect("lamp-b", "canopy", "detail", 3.2, true, 0.95, 0.34, 0, 0.06, 0.012, { region: "lamps", material: "lamp-diffuser" }),
    poly("panel-edge", "canopy", "detail", 3.3, false, 0.35, [[0.485, 0.065], [0.405, 0.185]], false),
    rect("chimney-lower", "chimney", "body", 3.4, true, 0.6, 0, 0.2, 0.3, 0.35, { region: "chimney", material: "stainless-steel" }),
    rect("chimney-upper", "chimney-top", "body", 3.5, true, 0.65, 0, 0.55, 0.28, 0.35, { region: "chimney", material: "stainless-steel" }),
    rect("collar", "chimney-top", "body", 3.6, true, 0.45, 0.04, 0.9, 0.2, 0.015, { region: "chimney", material: "stainless-steel" }),
    poly("seam", "chimney-top", "detail", 3.55, false, 0.3, [[0, 0.55], [0.28, 0.55]], false),
  ],
  {
    silhouette: ["A 0.5 m by 0.915 m side profile, 10 mm by 18.3 mm on paper: a sloping canopy box on the wall with a grease filter and two lamp recesses under it and a two-part stainless chimney above, closed by a duct collar.", "Ein Seitenprofil von 0,5 m mal 0,915 m, auf Papier 10 mm mal 18,3 mm: ein geneigter Haubenkörper an der Wand mit Fettfilter und zwei Lampenmulden darunter und ein zweiteiliger Edelstahlkamin darüber, abgeschlossen von einem Kanalkragen."],
    retained: ["Canopy depth 0.5 m and height 0.2 m with the sloped front, the 20 mm filter strip, the two lamp recesses, the 0.30 m lower and 0.28 m upper chimney tube with the telescopic step, the duct collar and the edge of the control panel.", "Haubentiefe 0,5 m und Höhe 0,2 m mit der geneigten Front, der 20 mm hohe Filterstreifen, die zwei Lampenmulden, das untere Kaminrohr von 0,30 m und das obere von 0,28 m mit dem Teleskopabsatz, der Kanalkragen und die Kante der Bedienblende."],
    merged: ["The wall mounting plate is part of the canopy shape; the motor box inside is not drawn.", "Die Wandplatte gehört zur Haubenform; der Motorkasten im Inneren ist nicht gezeichnet."],
    omitted: ["Buttons, the display, brackets, the grease drain and the internal motor and fan wheel.", "Tasten, das Display, Halter, die Fettablaufrinne und Motor und Lüfterrad im Inneren."],
    thinElements: ["The seam and the panel edge are 0.13 mm detail lines; the collar is 15 mm (0.3 mm on paper).", "Fuge und Blendenkante sind 0,13-mm-Detailstriche; der Kragen ist 15 mm hoch (0,3 mm auf Papier)."],
    hatching: ["None; the filter is a light tone strip.", "Keine; der Filter ist ein heller Tonstreifen."],
    styles: ["Canopy 0.55, filter 0.7, lamps 0.95, chimney 0.6 to 0.65, collar 0.45; body pen 0.18 mm, detail 0.13 mm, ground 0.35 mm.", "Haube 0,55, Filter 0,7, Lampen 0,95, Kamin 0,6 bis 0,65, Kragen 0,45; Körperstift 0,18 mm, Detail 0,13 mm, Boden 0,35 mm."],
    overlays: ["Canopy, filter, lamps and chimney are separate regions: in the thermal view the canopy and the lamps warm, the chimney stays cool; the air leaves at the collar.", "Haube, Filter, Lampen und Kamin sind getrennte Regionen: in der Thermalansicht erwärmen sich Haube und Lampen, der Kamin bleibt kühl; die Luft tritt am Kragen aus."],
  },
);

const rep100 = authored(
  100,
  [75, 150],
  { body: 0.18, detail: null, texture: null, glazing: null, ground: 0.25 },
  [
    poly("canopy", "canopy", "body", 3, true, 0.55, [[0, 0.025], [0.5, 0.025], [0.5, 0.06], [0.4, 0.2], [0, 0.2]], true, { region: "canopy", material: "stainless-steel" }),
    rect("filter", "canopy", "body", 3.1, true, 0.75, 0.02, 0, 0.46, 0.025, { regions: ["filter", "lamps"], material: "filter-mesh" }),
    rect("chimney-lower", "chimney", "body", 3.4, true, 0.6, 0, 0.2, 0.3, 0.35, { region: "chimney", material: "stainless-steel" }),
    rect("chimney-upper", "chimney-top", "body", 3.5, true, 0.65, 0, 0.55, 0.27, 0.35, { region: "chimney", material: "stainless-steel" }),
    rect("collar", "chimney-top", "body", 3.6, true, 0.45, 0.03, 0.9, 0.21, 0.02, { region: "chimney", material: "stainless-steel" }),
  ],
  {
    silhouette: ["A 5 mm by 9.2 mm profile: a sloping box under a two-part pipe with a collar on top.", "Ein Profil von 5 mm mal 9,2 mm: ein geneigter Kasten unter einem zweiteiligen Rohr mit einem Kragen oben."],
    retained: ["Canopy depth 0.5 m with the sloped front, the filter strip, both chimney tubes and the collar.", "Haubentiefe 0,5 m mit der geneigten Front, der Filterstreifen, beide Kaminrohre und der Kragen."],
    merged: ["The two lamp recesses join the filter strip (regions filter and lamps merged).", "Die beiden Lampenmulden gehen im Filterstreifen auf (Regionen Filter und Lampen zusammengeführt)."],
    omitted: ["Seam line, panel edge and the lamp recesses as separate shapes.", "Fugenlinie, Blendenkante und die Lampenmulden als eigene Formen."],
    thinElements: ["The filter strip is thickened from 20 to 25 mm (0.25 mm on paper) and the collar from 15 to 20 mm.", "Der Filterstreifen ist von 20 auf 25 mm (0,25 mm auf Papier) und der Kragen von 15 auf 20 mm verdickt."],
    hatching: ["None.", "Keine."],
    styles: ["Canopy 0.55, filter 0.75, chimney 0.6 to 0.65, collar 0.45; one body pen of 0.18 mm.", "Haube 0,55, Filter 0,75, Kamin 0,6 bis 0,65, Kragen 0,45; ein Körperstift von 0,18 mm."],
    overlays: ["The strip lists filter and lamps as merged regions, so the lamp heat still shows in the thermal view.", "Der Streifen führt Filter und Lampen als zusammengeführte Regionen, sodass die Lampenwärme in der Thermalansicht sichtbar bleibt."],
  },
);

const rep200 = authored(
  200,
  [150, 350],
  { body: 0.18, detail: null, texture: null, glazing: null, ground: 0.25 },
  [
    poly("canopy", "canopy", "body", 3, true, 0.55, [[0, 0], [0.5, 0], [0.5, 0.06], [0.4, 0.2], [0, 0.2]], true, { regions: ["canopy", "filter", "lamps"], material: "stainless-steel" }),
    rect("chimney-lower", "chimney", "body", 3.4, true, 0.6, 0, 0.2, 0.3, 0.35, { region: "chimney", material: "stainless-steel" }),
    rect("chimney-upper", "chimney-top", "body", 3.5, true, 0.65, 0, 0.55, 0.3, 0.37, { region: "chimney", material: "stainless-steel" }),
  ],
  {
    silhouette: ["A 2.5 mm by 4.6 mm profile: a sloped wedge under a plain pipe.", "Ein Profil von 2,5 mm mal 4,6 mm: ein geneigter Keil unter einem glatten Rohr."],
    retained: ["Canopy depth 0.5 m with the sloped front and the chimney of 0.3 m width up to 0.92 m.", "Haubentiefe 0,5 m mit der geneigten Front und der Kamin von 0,3 m Breite bis 0,92 m."],
    merged: ["Filter and lamps join the canopy (regions canopy, filter and lamps merged); collar and telescopic step join the chimney.", "Filter und Lampen gehen in der Haube auf (Regionen Haube, Filter und Lampen zusammengeführt); Kragen und Teleskopabsatz gehen im Kamin auf."],
    omitted: ["Filter strip, lamp recesses, seam, panel edge and the collar.", "Filterstreifen, Lampenmulden, Fuge, Blendenkante und der Kragen."],
    thinElements: ["None below the feature width; the 0.3 m chimney is 1.5 mm.", "Nichts unterhalb der Mindestbreite; der 0,3 m breite Kamin ist 1,5 mm breit."],
    hatching: ["None.", "Keine."],
    styles: ["Canopy 0.55, chimney 0.6 to 0.65; one body pen of 0.18 mm.", "Haube 0,55, Kamin 0,6 bis 0,65; ein Körperstift von 0,18 mm."],
    overlays: ["The wedge lists canopy, filter and lamps as merged regions; the chimney stays separate.", "Der Keil führt Haube, Filter und Lampen als zusammengeführte Regionen; der Kamin bleibt getrennt."],
  },
);

const rep500 = unavailable(
  500,
  ["At 1:500 the 0.5 m by 0.915 m hood is 1 mm by 1.8 mm and its 20 mm filter strip 0.04 mm, below every print profile.", "Bei 1:500 ist die 0,5 m mal 0,915 m große Haube 1 mm mal 1,8 mm groß und ihr 20 mm hoher Filterstreifen 0,04 mm, unter jedem Druckprofil."],
  ["Filter strip of 20 mm", "Filterstreifen von 20 mm"],
  0.02,
  [{ kind: "scale", denominator: 200 }, { kind: "scale", denominator: 100 }],
);

const single = (id, en, de, kind, material, thicknessM, res, cap, reference = "iso-10456-2007") => layer(id, [en, de], kind, material, thicknessM, res, cap, SRC, reference);
const region = (id, title, kind, model, material, areaM2, characteristicLengthM, geometry, layers, heatCapacityJM2K, extra = {}, reference = "iso-10456-2007") => ({
  id,
  title: tx(...title),
  kind,
  model,
  material,
  areaM2,
  characteristicLengthM,
  geometry,
  skyViewFactor: 0.4,
  layers,
  heatCapacityJM2K,
  ...extra,
  source: SRC,
  reference,
});

const regions = [
  region("canopy", ["Canopy shell with motor", "Haubenkörper mit Motor"], "surface", "operating", "stainless-steel", 0.9, 0.5, "horizontal-plate-down", [single("shell", "Stainless steel sheet 0.8 mm", "Edelstahlblech 0,8 mm", "metal", "stainless-steel", 0.0008, 0.000053, 3160)], 3160, { operating: { key: "fan", heatW: 120 } }, "incropera-2018"),
  region("filter", ["Grease filter", "Fettfilter"], "surface", "passive", "filter-mesh", 0.2, 0.46, "horizontal-plate-down", [single("mesh", "Aluminium mesh pack 10 mm", "Aluminium-Gitterpaket 10 mm", "metal", "filter-mesh", 0.01, 0.0003, 900)], 900),
  region("lamps", ["Lamp modules", "Lampenmodule"], "lamp", "operating", "lamp-diffuser", 0.03, 0.06, "horizontal-plate-down", [single("diffuser", "Opal polycarbonate diffuser 2 mm", "Opaldiffusor aus Polycarbonat 2 mm", "polymer", "lamp-diffuser", 0.002, 0.01, 2400)], 2400, { operating: { key: "light", heatW: 30 } }, "incropera-2018"),
  region("chimney", ["Telescopic chimney", "Teleskopkamin"], "surface", "passive", "stainless-steel", 0.75, 0.7, "vertical-plate", [single("tube", "Stainless steel sheet 0.6 mm", "Edelstahlblech 0,6 mm", "metal", "stainless-steel", 0.0006, 0.00004, 2370)], 2370),
];

const bind = (fan, light) => ({
  regions: {
    canopy: { exposedFraction: 0.85, sunlitFraction: 0, contact: { fraction: 0.15, tempC: 21 } },
    filter: { exposedFraction: 1, sunlitFraction: 0 },
    lamps: { exposedFraction: 1, sunlitFraction: 0 },
    chimney: { exposedFraction: 0.85, sunlitFraction: 0, contact: { fraction: 0.15, tempC: 21 } },
  },
  emitters: { extract: fan },
  operating: { fan, light },
});

const wall = ["wall-hook", "wall-rest"];
const poses = [
  pose("off", ["Off", "Aus"], ["Fan and lamps are off, the hood is at room temperature.", "Lüfter und Lampen sind aus, die Haube hat Raumtemperatur."], {}, wall, "authored", bind(0, 0)),
  pose("low", ["Low speed", "Kleine Stufe"], ["The fan runs on the low stage at 40 percent, the lamps are on.", "Der Lüfter läuft auf der kleinen Stufe mit 40 Prozent, die Lampen sind an."], {}, wall, "authored", bind(0.4, 1)),
  pose("extracting", ["Extracting", "Absaugend"], ["The fan runs at full speed and extracts 500 cubic metres per hour through the filter and the chimney; the motor and the lamps release 150 W.", "Der Lüfter läuft mit voller Drehzahl und saugt 500 Kubikmeter je Stunde durch Filter und Kamin ab; Motor und Lampen geben 150 W ab."], {}, wall, "authored", bind(1, 1)),
];

const contacts = (d) => wall.map((anchor) => ({ anchor, fromS: 0, toS: d }));

const clips = [
  clip(
    "switch-on-warm-up",
    ["Switch on and warm up", "Einschalten und aufwärmen"],
    ["The hood is switched on at full speed after five seconds: the extract air starts at once and the canopy warms within ten minutes.", "Die Haube wird nach fünf Sekunden auf voller Stufe eingeschaltet: der Abluftstrom setzt sofort ein und die Haube erwärmt sich binnen zehn Minuten."],
    600,
    1,
    [{ t: 0, pose: "off", ease: "step" }, { t: 5, pose: "extracting", ease: "step" }, { t: 600, pose: "extracting", ease: "step" }],
    contacts(600),
    [["off", 0, 5, ["Off", "Aus"]], ["run", 5, 600, ["Extracting, warming up", "Saugt ab, wärmt auf"]]],
    [[5, "operating", ["Fan and lamps on", "Lüfter und Lampen an"], "fan"]],
    [["operating:fan", "step", [[0, 0], [5, 1]]], ["operating:light", "step", [[0, 0], [5, 1]]], ["emitter:extract", "step", [[0, 0], [5, 1]]]],
  ),
  clip(
    "step-down-and-off",
    ["Step down and switch off", "Herunterschalten und ausschalten"],
    ["The fan steps from full speed to the low stage after one minute and is switched off after two more minutes.", "Der Lüfter schaltet nach einer Minute von voller auf die kleine Stufe und nach zwei weiteren Minuten aus."],
    300,
    1,
    [{ t: 0, pose: "extracting", ease: "step" }, { t: 60, pose: "low", ease: "step" }, { t: 180, pose: "off", ease: "step" }, { t: 300, pose: "off", ease: "step" }],
    contacts(300),
    [["full", 0, 60, ["Full speed", "Volle Stufe"]], ["low", 60, 180, ["Low stage", "Kleine Stufe"]], ["cool-down", 180, 300, ["Off, cooling", "Aus, kühlt ab"]]],
    [[180, "shutdown", ["Fan and lamps off", "Lüfter und Lampen aus"], "fan"]],
    [["operating:fan", "step", [[0, 1], [60, 0.4], [180, 0]]], ["operating:light", "step", [[0, 1], [180, 0]]], ["emitter:extract", "step", [[0, 1], [60, 0.4], [180, 0]]]],
  ),
];

export const hood = () => {
  const massKg = sum(sp.bones.map((b) => b.mass.kg));
  return wrap({
    id: "kitchen-extractor-hood",
    category: "appliances",
    title: tx("Kitchen Chimney Extractor Hood", "Küchen-Kaminhaube"),
    summary: tx(
      "A wall-mounted chimney extractor hood 0.9 m wide in side elevation: a sloped stainless canopy with grease filter and two lamps under a telescopic chimney; at full speed it extracts 500 m3/h and its motor and lamps release 150 W.",
      "Eine an der Wand montierte Kaminhaube von 0,9 m Breite in der Seitenansicht: ein geneigter Edelstahlkörper mit Fettfilter und zwei Lampen unter einem Teleskopkamin; auf voller Stufe saugt sie 500 m3/h ab, Motor und Lampen geben 150 W ab.",
    ),
    tags: ["extractor-hood", "kitchen", "ventilation", "appliance", "wall-mounted", "internal-gain", "electrical", "extract-air"],
    provenance: {
      author: "Semio staffage content (A8)",
      method: "handcrafted",
      created: "2026-11-01",
      references: [
        ref("iec-60335-2-31", "IEC 60335-2-31: Household and similar electrical appliances - Safety - Part 2-31: Particular requirements for range hoods and other cooking fume extractors (installation height above the hob, power)."),
        ref("iec-61591", "IEC 61591 / EN 61591: Household range hoods and other cooking fume extractors - Methods for measuring performance (volume flow at the maximum speed, typical 300 to 600 cubic metres per hour)."),
        INCROPERA,
        ISO10456,
      ],
      notes: tx(
        "Side elevation of a wall-mounted chimney hood of the common 0.9 m width class: a canopy of 0.5 m depth and 0.2 m height with a sloped front, a 20 mm grease filter and two lamp modules under it, a lower tube of 0.30 m width and a telescopic upper tube of 0.28 m width up to 0.915 m total height. The wall lies at x = 0; the origin is the underside of the filter on the wall, and a call site places it about 0.65 m above the hob (IEC 60335-2-31 asks for at least 0.65 m over an electric hob). The width of 0.9 m is the depth in the elevation (depthM). Dimensions and the mass of 14.5 kg are rounded from this product class (authored estimate). At level 1 the extract airflow is 0.14 m3/s (500 m3/h) leaving at the duct collar; the fan motor and the lamps take 150 W (120 W motor, 30 W LED modules), which ends entirely as heat in the room: the asset releases it at the canopy shell and the lamp modules; the split is an authored estimate. The extracted cooking vapour itself is not modelled (temperature rise 0).",
        "Seitenansicht einer wandmontierten Kaminhaube der üblichen Breitenklasse von 0,9 m: ein Haubenkörper von 0,5 m Tiefe und 0,2 m Höhe mit geneigter Front, ein 20 mm hoher Fettfilter und zwei Lampenmodule darunter, ein unteres Rohr von 0,30 m Breite und ein teleskopierbares oberes Rohr von 0,28 m Breite bis zu einer Gesamthöhe von 0,915 m. Die Wand liegt bei x = 0; der Ursprung ist die Filterunterseite an der Wand, und ein Aufrufer setzt ihn etwa 0,65 m über das Kochfeld (IEC 60335-2-31 verlangt mindestens 0,65 m über einem Elektrokochfeld). Die Breite von 0,9 m ist die Tiefe in der Ansicht (depthM). Maße und die Masse von 14,5 kg sind aus dieser Produktklasse gerundet (Schätzwert). Auf Stufe 1 beträgt der Abluftstrom 0,14 m3/s (500 m3/h) und tritt am Kanalkragen aus; Lüftermotor und Lampen nehmen 150 W auf (120 W Motor, 30 W LED-Module), die vollständig als Wärme im Raum enden: das Asset gibt sie am Haubenkörper und an den Lampenmodulen ab; die Aufteilung ist ein Schätzwert. Der abgesaugte Kochdunst selbst ist nicht modelliert (Temperaturerhöhung 0).",
      ),
    },
    dimensions: { heightM: 0.915, widthM: 0.5, depthM: 0.9, massKg },
    materials: [
      materialOf("stainless-steel", "Brushed stainless steel", "Gebürsteter Edelstahl", 0.2, 0.5, 7900, 15, 500, "literature", "incropera-2018"),
      materialOf("filter-mesh", "Aluminium mesh grease filter", "Aluminium-Gitterfilter", 0.3, 0.5, 300, 20, 900, SRC, "incropera-2018"),
      materialOf("lamp-diffuser", "Opal polycarbonate diffuser", "Opaldiffusor aus Polycarbonat", 0.9, 0.2, 1200, 0.2, 1200, SRC, "incropera-2018"),
    ],
    rig: { kind: "mechanical", root: "canopy", bones: sp.bones.map((b) => ({ ...b, mass: { ...b.mass } })) },
    anchors: [
      { id: "wall-hook", bone: "canopy", x: 0, y: 0.19, kind: "support", title: tx("Upper hook on the wall plate", "Oberer Haken an der Wandplatte"), support: "pin", normalDeg: 0, footprintLengthM: 0.02, footprintWidthM: 0.3 },
      { id: "wall-rest", bone: "canopy", x: 0, y: 0.03, kind: "support", title: tx("Lower rest against the wall", "Untere Anlage an der Wand"), support: "roller", normalDeg: 0, footprintLengthM: 0.02, footprintWidthM: 0.3, frictionCoefficient: 0.4 },
      { id: "outlet", bone: "chimney-top", x: 0.14, y: 0.915, kind: "emitter", title: tx("Duct collar on top of the chimney", "Kanalkragen oben am Kamin") },
    ],
    representations: { 500: rep500, 200: rep200, 100: rep100, 50: rep50 },
    symbols: [],
    poses,
    clips,
    thermal: {
      environment: { airTempC: 21, meanRadiantTempC: 21, relativeHumidity: 0.5, windMS: 0.1, windDirectionDeg: 0, solarWM2: 0, diffuseFraction: 0, pressurePa: 101325 },
      passengers: [],
      operating: [
        { id: "fan", title: tx("Fan", "Lüfter"), levels: [level(0, "Off", "Aus"), level(0.4, "Low stage (40 percent)", "Kleine Stufe (40 Prozent)"), level(1, "Full speed (500 m3/h)", "Volle Stufe (500 m3/h)")] },
        { id: "light", title: tx("Lamps", "Lampen"), levels: [level(0, "Off", "Aus"), level(1, "On (30 W)", "An (30 W)")] },
      ],
      regions,
      emitters: [{ id: "extract", kind: "vent", title: tx("Extract air leaving at the duct collar", "Abluft am Kanalkragen"), anchor: "outlet", directionDeg: 90, diameterM: 0.15, operatingKey: "fan", volumeFlowM3S: 0.14, temperatureRiseK: 0, vapourKgPerKg: 0, source: SRC, reference: "iec-61591" }],
      assumptions: [
        tx(
          "The 150 W at full speed end entirely as heat in the room: 120 W at the canopy shell (motor, electronics) and 30 W at the lamp modules; the heat of the extracted cooking vapour belongs to the hob, not to the hood.",
          "Die 150 W auf voller Stufe enden vollständig als Wärme im Raum: 120 W am Haubenkörper (Motor, Elektronik) und 30 W an den Lampenmodulen; die Wärme des abgesaugten Kochdunstes gehört zum Kochfeld, nicht zur Haube.",
        ),
        tx(
          "The airflow scales with the fan level: 0.056 m3/s at the low stage, 0.14 m3/s at full speed.",
          "Der Luftstrom skaliert mit der Lüfterstufe: 0,056 m3/s auf der kleinen Stufe, 0,14 m3/s auf voller Stufe.",
        ),
      ],
    },
    structure: {
      gravityMS2: 9.80665,
      dynamics: "quasi-static",
      supports: wall,
      assumptions: [
        tx(
          "The 14.5 kg hood hangs on the upper hook (pin) and leans against the wall at the lower edge (roller): the hook carries the weight, the pair resists the overturning moment of the centre of mass 0.2 m from the wall.",
          "Die 14,5 kg schwere Haube hängt am oberen Haken (Festlager) und lehnt unten an der Wand (Rollenlager): der Haken trägt das Gewicht, das Paar nimmt das Kippmoment des Schwerpunkts 0,2 m vor der Wand auf.",
        ),
      ],
      clinical: false,
    },
    presentation: { defaultPose: "extracting", defaultClip: "switch-on-warm-up", defaultScale: 50, viewpoints: ["elevation"] },
    exports: ALL_EXPORTS,
    completion: "authored",
  });
};
