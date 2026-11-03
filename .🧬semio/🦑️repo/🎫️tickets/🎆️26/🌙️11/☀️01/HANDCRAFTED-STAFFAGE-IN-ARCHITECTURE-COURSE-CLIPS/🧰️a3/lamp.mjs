import { ALL_EXPORTS, INCROPERA, ISO10456, VDI_2078, authored, bone, clip, layer, level, path, pose, poly, rect, ref, tx, unavailable, wrap } from "./lib.mjs";

const SOURCE = "authored-estimate";

const ROSE = "M -0.05 -0.024 L 0.05 -0.024 L 0.05 -0.016 C 0.05 -0.006 0.042 0 0.032 0 L -0.032 0 C -0.042 0 -0.05 -0.006 -0.05 -0.016 Z";
const COLLAR = "M -0.01 0 L 0.01 0 L 0.0105 -0.006 L 0.0155 -0.0075 L 0.0155 -0.03 L -0.0155 -0.03 L -0.0155 -0.0075 L -0.0105 -0.006 Z";
const SHADE = "M -0.017 0 C -0.06 0 -0.11 -0.03 -0.13 -0.133 L -0.133 -0.133 L -0.133 -0.1385 L 0.133 -0.1385 L 0.133 -0.133 L 0.13 -0.133 C 0.11 -0.03 0.06 0 0.017 0 Z";
const SHADE_PLAIN = "M -0.017 0 C -0.06 0 -0.11 -0.03 -0.13 -0.133 L 0.13 -0.133 C 0.11 -0.03 0.06 0 0.017 0 Z";
const GLOBE = "M 0 0 C 0.014 0 0.03 0.014 0.03 0.032 C 0.03 0.046 0.023 0.054 0.0185 0.063 C 0.016 0.069 0.0143 0.073 0.0138 0.078 L -0.0138 0.078 C -0.0143 0.073 -0.016 0.069 -0.0185 0.063 C -0.023 0.054 -0.03 0.046 -0.03 0.032 C -0.03 0.014 -0.014 0 0 0 Z";
const CAP = "M -0.0135 0.078 L 0.0135 0.078 L 0.0135 0.105 L -0.0135 0.105 Z";

const rep50 = authored(
  50,
  [20, 75],
  { body: 0.18, detail: 0.13, texture: null, glazing: null, ground: 0.35 },
  [
    path("rose", "rose", "body", 3, true, 0.7, ROSE, { region: "rose", material: "rose-polymer" }),
    rect("cord", "cord", "body", 3, true, 0.15, -0.0033, -0.521, 0.0066, 0.521, { region: "cord", material: "textile-cord" }),
    path("collar", "holder", "body", 3.2, true, 0.45, COLLAR, { region: "lampholder", material: "brass-lacquered" }),
    path("shade", "shade", "body", 3.4, true, 0.4, SHADE, { region: "shade", material: "enamel-white" }),
    poly("shade-light", "shade", "detail", 3.5, false, 0.7, [[-0.095, -0.05], [-0.108, -0.09], [-0.114, -0.118]], false),
    path("bulb-cap", "bulb", "body", 3.1, true, 0.5, CAP, { region: "lampholder", material: "brass-lacquered" }),
    poly("cap-thread", "bulb", "detail", 3.15, false, 0.3, [[-0.0135, 0.0865], [0.0135, 0.0865], [0.0135, 0.0955], [-0.0135, 0.0955]], false),
    path("bulb-glass", "bulb", "body", 3.3, true, 0.9, GLOBE, { region: "bulb-glass", material: "bulb-glass" }),
    poly("filament", "bulb", "detail", 3.35, false, 0.2, [[-0.0055, 0.074], [-0.0055, 0.06], [-0.004, 0.044], [-0.002, 0.037], [0, 0.044], [0.002, 0.037], [0.004, 0.044], [0.0055, 0.06], [0.0055, 0.074]], false),
  ],
  {
    silhouette: ["A hanging sign 0.8 m by 0.27 m, 16 mm by 5.3 mm on paper: a ceiling rose, a thin cord, a brass collar, a domed enamel shade with a rolled rim and a pear-shaped bulb hanging out of it.", "Ein hängendes Zeichen von 0,8 m mal 0,27 m, auf Papier 16 mm mal 5,3 mm: eine Deckenrosette, ein dünnes Kabel, eine Messinghülse, ein gewölbter Emailleschirm mit Wulstrand und eine birnenförmige Glühlampe, die daraus herabhängt."],
    retained: ["Overall height 0.8 m, shade width 0.266 m, bulb diameter 0.06 m and length 0.105 m (E27, A60), the cord drop, the rolled rim, the brass cap with thread and the filament with its two support wires.", "Gesamthöhe 0,8 m, Schirmbreite 0,266 m, Lampendurchmesser 0,06 m und Länge 0,105 m (E27, A60), die Abhängung, der Wulstrand, die Messingkappe mit Gewinde und die Wendel mit ihren beiden Haltedrähten."],
    merged: ["Rose and canopy collar are one chamfered disc; the socket inside the shade is hidden behind it.", "Rosette und Baldachinkragen sind eine gefaste Scheibe; die Fassung im Schirm ist dahinter verdeckt."],
    omitted: ["The pull-switch, the cable knot, the bulb text and the socket inside the dome.", "Der Zugschalter, der Kabelknoten, die Lampenbeschriftung und die Fassung im Schirm."],
    thinElements: ["The 6.6 mm cord is a 0.13 mm line and relies on the 0.18 mm pen; the filament is a 0.13 mm detail line with a 1 mm coil.", "Das 6,6 mm dicke Kabel ist eine 0,13 mm breite Linie und stützt sich auf den 0,18-mm-Strich; die Wendel ist ein 0,13-mm-Detailzug mit einer 1-mm-Wicklung."],
    hatching: ["None; the highlight on the shade is one detail line.", "Keine; das Glanzlicht am Schirm ist ein Detailzug."],
    styles: ["Rose 0.7, cord 0.15, brass 0.45, shade 0.4 with a light highlight, bulb glass 0.9 very light; body pen 0.18 mm, detail 0.13 mm.", "Rosette 0,7, Kabel 0,15, Messing 0,45, Schirm 0,4 mit hellem Glanzlicht, Lampenglas 0,9 sehr hell; Körperstift 0,18 mm, Detail 0,13 mm."],
    overlays: ["Bulb glass, shade, cap with collar, cord and rose are separate regions: in the thermal view the glass and the shade glow, the cord and rose stay cool.", "Lampenglas, Schirm, Kappe mit Hülse, Kabel und Rosette sind getrennte Regionen: in der Thermalansicht glühen Glas und Schirm, Kabel und Rosette bleiben kühl."],
  },
);

const rep100 = authored(
  100,
  [75, 150],
  { body: 0.18, detail: null, texture: null, glazing: null, ground: 0.25 },
  [
    path("rose", "rose", "body", 3, true, 0.7, ROSE, { region: "rose", material: "rose-polymer" }),
    rect("cord", "cord", "body", 3, true, 0.15, -0.0045, -0.521, 0.009, 0.521, { region: "cord", material: "textile-cord" }),
    path("collar", "holder", "body", 3.2, true, 0.45, COLLAR, { region: "lampholder", material: "brass-lacquered" }),
    path("shade", "shade", "body", 3.4, true, 0.4, SHADE_PLAIN, { region: "shade", material: "enamel-white" }),
    path("bulb-cap", "bulb", "body", 3.1, true, 0.5, CAP, { region: "lampholder", material: "brass-lacquered" }),
    path("bulb-glass", "bulb", "body", 3.3, true, 0.9, GLOBE, { region: "bulb-glass", material: "bulb-glass" }),
  ],
  {
    silhouette: ["A hanging bell 8 mm by 2.7 mm on paper: a disc, a cord, a collar, a plain dome and a pear below its rim.", "Eine hängende Glocke von 8 mm mal 2,7 mm auf Papier: eine Scheibe, ein Kabel, eine Hülse, eine schlichte Kuppel und eine Birne unter ihrem Rand."],
    retained: ["Height 0.8 m, shade width 0.26 m, bulb diameter 0.06 m and length 0.105 m, the cord drop, the brass collar and cap.", "Höhe 0,8 m, Schirmbreite 0,26 m, Lampendurchmesser 0,06 m und Länge 0,105 m, die Abhängung, Messinghülse und -kappe."],
    merged: ["The rolled rim joins the dome; collar and cap read as brass accents above and below the shade.", "Der Wulstrand geht in der Kuppel auf; Hülse und Kappe lesen sich als Messingakzente über und unter dem Schirm."],
    omitted: ["Thread lines, filament and its supports, the highlight on the shade, the pull-switch.", "Gewindelinien, Wendel und Haltedrähte, das Glanzlicht am Schirm, der Zugschalter."],
    thinElements: ["The cord is widened from 6.6 to 9 mm (0.09 mm paper) so that fill and 0.18 mm pen reach the feature width.", "Das Kabel ist von 6,6 auf 9 mm verbreitert (0,09 mm Papier), damit Füllung und 0,18-mm-Strich die Mindestbreite erreichen."],
    hatching: ["None.", "Keine."],
    styles: ["Rose 0.7, cord 0.15, brass 0.45, shade 0.4, glass 0.9; one body pen of 0.18 mm.", "Rosette 0,7, Kabel 0,15, Messing 0,45, Schirm 0,4, Glas 0,9; ein Körperstift von 0,18 mm."],
    overlays: ["Each part keeps its region: the glass and shade carry the heat of the lamp.", "Jedes Teil behält seine Region: Glas und Schirm tragen die Wärme der Lampe."],
  },
);

const rep200 = authored(
  200,
  [150, 350],
  { body: 0.18, detail: null, texture: null, glazing: null, ground: 0.25 },
  [
    path("rose", "rose", "body", 3, true, 0.7, "M -0.05 -0.03 L 0.05 -0.03 L 0.05 0 L -0.05 0 Z", { region: "rose", material: "rose-polymer" }),
    rect("cord", "cord", "body", 3, true, 0.15, -0.007, -0.521, 0.014, 0.521, { region: "cord", material: "textile-cord" }),
    path("shade", "shade", "body", 3.4, true, 0.4, "M -0.025 0.03 L 0.025 0.03 L 0.025 0 C 0.06 0 0.11 -0.03 0.133 -0.1385 L -0.133 -0.1385 C -0.11 -0.03 -0.06 0 -0.025 0 Z", { regions: ["shade", "lampholder"], material: "enamel-white" }),
    path("bulb", "bulb", "body", 3.3, true, 0.85, "M 0 0 C 0.014 0 0.03 0.014 0.03 0.032 C 0.03 0.046 0.023 0.054 0.0185 0.063 L 0.0135 0.08 L -0.0135 0.08 L -0.0185 0.063 C -0.023 0.054 -0.03 0.046 -0.03 0.032 C -0.03 0.014 -0.014 0 0 0 Z", { region: "bulb-glass", material: "bulb-glass" }),
  ],
  {
    silhouette: ["A hanging sign 4 mm by 1.3 mm on paper: a bar, a cord, a trapezoid shade and a pear below it.", "Ein hängendes Zeichen von 4 mm mal 1,3 mm auf Papier: ein Balken, ein Kabel, ein trapezförmiger Schirm und darunter eine Birne."],
    retained: ["Height 0.8 m, shade width 0.266 m, bulb diameter 0.06 m, the cord drop and the bulb hanging below the rim.", "Höhe 0,8 m, Schirmbreite 0,266 m, Lampendurchmesser 0,06 m, die Abhängung und die unter dem Rand hängende Lampe."],
    merged: ["Collar and shade are one shape; the brass cap is hidden in it and its region joins the shade shape as a merged region.", "Hülse und Schirm sind eine Form; die Messingkappe ist darin verborgen und ihre Region wird der Schirmform zusammengeführt."],
    omitted: ["Rolled rim, thread, filament, highlight and the pull-switch.", "Wulstrand, Gewinde, Wendel, Glanzlicht und Zugschalter."],
    thinElements: ["The cord is widened from 6.6 to 14 mm (0.07 mm paper) and the rose from 24 to 30 mm so that fill and 0.18 mm pen reach the feature width.", "Das Kabel ist von 6,6 auf 14 mm (0,07 mm Papier) und die Rosette von 24 auf 30 mm verbreitert, damit Füllung und 0,18-mm-Strich die Mindestbreite erreichen."],
    hatching: ["None.", "Keine."],
    styles: ["Rose 0.7, cord 0.15, shade 0.4, glass 0.85; one body pen of 0.18 mm.", "Rosette 0,7, Kabel 0,15, Schirm 0,4, Glas 0,85; ein Körperstift von 0,18 mm."],
    overlays: ["The shade shape lists shade and lampholder as merged regions; the bulb is its own region.", "Die Schirmform führt Schirm und Fassung als zusammengeführte Regionen; die Lampe ist eine eigene Region."],
  },
);

const rep500 = unavailable(
  500,
  ["At 1:500 the 0.8 m lamp is 1.6 mm tall and its cord 0.013 mm wide, the bulb is a 0.12 mm dot, below every print profile.", "Bei 1:500 ist die 0,8 m lange Lampe 1,6 mm hoch, ihr Kabel 0,013 mm breit und die Glühlampe ein 0,12-mm-Punkt, unter jedem Druckprofil."],
  ["Cord diameter of 6.6 mm", "Kabeldurchmesser von 6,6 mm"],
  0.0066,
  [{ kind: "scale", denominator: 200 }, { kind: "scale", denominator: 100 }],
);

const single = (id, en, de, kind, material, thicknessM, res, cap) => layer(id, [en, de], kind, material, thicknessM, res, cap, SOURCE, "iso-10456-2007");

const regions = [
  {
    id: "bulb-glass",
    title: tx("Bulb glass envelope", "Glaskolben der Glühlampe"),
    kind: "lamp",
    model: "operating",
    material: "bulb-glass",
    areaM2: 0.012,
    characteristicLengthM: 0.06,
    geometry: "sphere",
    skyViewFactor: 0.5,
    layers: [single("envelope", "Soda-lime glass 0.8 mm", "Kalk-Natron-Glas 0,8 mm", "glass", "bulb-glass", 0.0008, 0.0008, 1700)],
    heatCapacityJM2K: 1700,
    operating: { key: "power", heatW: 36 },
    source: SOURCE,
    reference: "iec-60064",
  },
  {
    id: "lampholder",
    title: tx("Bulb cap, socket and collar", "Lampensockel, Fassung und Hülse"),
    kind: "lamp",
    model: "operating",
    material: "brass-lacquered",
    areaM2: 0.012,
    characteristicLengthM: 0.05,
    geometry: "cylinder",
    skyViewFactor: 0.4,
    layers: [single("brass-shell", "Lacquered brass 0.6 mm", "Lackiertes Messing 0,6 mm", "metal", "brass-lacquered", 0.0006, 6e-6, 4600)],
    heatCapacityJM2K: 4600,
    operating: { key: "power", heatW: 6 },
    source: SOURCE,
    reference: "iec-60064",
  },
  {
    id: "shade",
    title: tx("Enamelled shade", "Emaillierter Schirm"),
    kind: "lamp",
    model: "operating",
    material: "enamel-white",
    areaM2: 0.24,
    characteristicLengthM: 0.26,
    geometry: "sphere",
    skyViewFactor: 0.6,
    layers: [single("shade-steel", "Enamelled steel sheet 0.6 mm", "Emailliertes Stahlblech 0,6 mm", "metal", "enamel-white", 0.0006, 1.2e-5, 2800)],
    heatCapacityJM2K: 2800,
    operating: { key: "power", heatW: 18 },
    source: SOURCE,
    reference: "incropera-2018",
  },
  {
    id: "cord",
    title: tx("Textile-sheathed cord", "Textilummanteltes Kabel"),
    kind: "surface",
    model: "passive",
    material: "textile-cord",
    areaM2: 0.0108,
    characteristicLengthM: 0.0066,
    geometry: "cylinder",
    skyViewFactor: 0.5,
    layers: [single("sheath", "PVC cable with textile sheath 1.5 mm", "PVC-Leitung mit Textilmantel 1,5 mm", "polymer", "textile-cord", 0.0015, 0.0075, 2000)],
    heatCapacityJM2K: 2000,
    source: SOURCE,
    reference: "iso-10456-2007",
  },
  {
    id: "rose",
    title: tx("Ceiling rose", "Deckenrosette"),
    kind: "surface",
    model: "passive",
    material: "rose-polymer",
    areaM2: 0.012,
    characteristicLengthM: 0.1,
    geometry: "horizontal-plate-down",
    skyViewFactor: 0,
    layers: [single("rose-shell", "Polymer rose 3 mm", "Kunststoffrosette 3 mm", "polymer", "rose-polymer", 0.003, 0.015, 4200)],
    heatCapacityJM2K: 4200,
    source: SOURCE,
    reference: "iso-10456-2007",
  },
];

const bind = (power, plume, sunlit = 0) => ({
  regions: {
    "bulb-glass": { exposedFraction: 1, sunlitFraction: sunlit },
    lampholder: { exposedFraction: 0.6, sunlitFraction: 0 },
    shade: { exposedFraction: 1, sunlitFraction: sunlit },
    cord: { exposedFraction: 1, sunlitFraction: 0 },
    rose: { exposedFraction: 0.6, sunlitFraction: 0, contact: { fraction: 0.4, tempC: 21 } },
  },
  emitters: { "shade-plume": plume },
  operating: { power },
});

const poses = [
  pose("on", ["Lit", "Eingeschaltet"], ["The 60 W bulb burns: the glass and the shade are hot, the cap is warm, a plume rises through the shade.", "Die 60-W-Glühlampe brennt: Glas und Schirm sind heiß, die Kappe ist warm, durch den Schirm steigt eine Fahne auf."], {}, ["ceiling-hook"], "authored", bind(1, 1)),
  pose("dimmed", ["Dimmed to 40 %", "Auf 40 % gedimmt"], ["The incandescent bulb runs at 40 % power, 24 W: the glass is lukewarm.", "Die Glühlampe läuft mit 40 % Leistung, 24 W: das Glas ist handwarm."], {}, ["ceiling-hook"], "authored", bind(0.4, 0.5)),
  pose("off", ["Off", "Ausgeschaltet"], ["Switched off, everything at room temperature.", "Ausgeschaltet, alles hat Raumtemperatur."], {}, ["ceiling-hook"], "authored", bind(0, 0)),
];

const hook = (d) => [{ anchor: "ceiling-hook", fromS: 0, toS: d }];

const clips = [
  clip(
    "switch-on-warm-up",
    ["Switch on and warm up", "Einschalten und aufwärmen"],
    ["The cold lamp is switched on after five seconds: the glass reaches its working temperature within a minute and the shade warms over four minutes.", "Die kalte Lampe wird nach fünf Sekunden eingeschaltet: das Glas erreicht binnen einer Minute seine Betriebstemperatur und der Schirm wärmt sich in vier Minuten auf."],
    300,
    1,
    [{ t: 0, pose: "off", ease: "step" }, { t: 5, pose: "on", ease: "step" }, { t: 300, pose: "on", ease: "step" }],
    hook(300),
    [["cold", 0, 5, ["Off", "Aus"]], ["warm-up", 5, 300, ["Lit, warming up", "Brennt, wärmt auf"]]],
    [[5, "operating", ["Switched on", "Eingeschaltet"], "power"]],
    [["operating:power", "step", [[0, 0], [5, 1]]], ["emitter:shade-plume", "step", [[0, 0], [5, 1]]]],
  ),
  clip(
    "switch-off-cool-down",
    ["Switch off and cool down", "Ausschalten und abkühlen"],
    ["The hot lamp is switched off after five seconds: the glass cools within a minute, the shade and the cap follow within five minutes.", "Die heiße Lampe wird nach fünf Sekunden ausgeschaltet: das Glas kühlt binnen einer Minute, Schirm und Kappe folgen in fünf Minuten."],
    300,
    1,
    [{ t: 0, pose: "on", ease: "step" }, { t: 5, pose: "off", ease: "step" }, { t: 300, pose: "off", ease: "step" }],
    hook(300),
    [["lit", 0, 5, ["Lit", "Brennt"]], ["cool-down", 5, 300, ["Off, cooling", "Aus, kühlt ab"]]],
    [[5, "shutdown", ["Switched off", "Ausgeschaltet"], "power"]],
    [["operating:power", "step", [[0, 1], [5, 0]]], ["emitter:shade-plume", "step", [[0, 1], [5, 0]]]],
  ),
];

export const lamp = () =>
  wrap({
    id: "pendant-lamp",
    category: "appliances",
    title: tx("Pendant Lamp with Incandescent Bulb", "Pendelleuchte mit Glühlampe"),
    summary: tx(
      "A 0.8 m pendant luminaire with an enamelled dome shade 0.266 m wide and a 60 W incandescent bulb hanging below the rim: almost all of the 60 W ends as heat, the glass and the shade are the warm surfaces.",
      "Eine 0,8 m lange Pendelleuchte mit emailliertem Kuppelschirm von 0,266 m Breite und einer 60-W-Glühlampe, die unter dem Rand hängt: fast die gesamten 60 W enden als Wärme, Glas und Schirm sind die warmen Flächen.",
    ),
    tags: ["pendant-lamp", "luminaire", "incandescent", "internal-gain", "electrical", "heat-source"],
    provenance: {
      author: "Semio staffage content (A3)",
      method: "handcrafted",
      created: "2026-11-01",
      references: [
        ref("iec-60064", "IEC 60064: Tungsten filament lamps for domestic and similar general lighting purposes - Performance requirements (bulb envelope A60 with 60 mm diameter, E27 cap, rated power and luminous efficacy)."),
        ref("iec-60061-1", "IEC 60061-1: Lamp caps and holders together with gauges for the control of interchangeability and safety - Part 1: Lamp caps (E27 Edison screw cap, 27 mm)."),
        ref("reg-ec-244-2009", "Commission Regulation (EC) No 244/2009 of 18 March 2009 implementing Directive 2005/32/EC with regard to ecodesign requirements for non-directional household lamps (phase-out of incandescent lamps; context of the 60 W class)."),
        VDI_2078,
        INCROPERA,
        ISO10456,
      ],
      notes: tx(
        "Elevation of a pendant luminaire with a ceiling rose, a 0.52 m textile cord, a brass collar, a domed enamelled shade (0.266 m wide, 0.133 m high) and an A60 incandescent bulb with an E27 cap (0.060 m diameter, 0.105 m long, IEC 60064 and IEC 60061-1) hanging out of the open rim. The origin is the lowest point of the bulb; the rose lies 0.8 m above it, so a call site places the origin 0.8 m under the ceiling. Dimensions and mass 0.62 kg are rounded from this product class (authored estimate). Electrical power at level 1 is the rated 60 W, which ends entirely as heat in the room: an incandescent lamp turns about 95 % of its power into infrared radiation and heat and the visible light is absorbed by the room surfaces. The asset releases it in the surfaces of the lamp: 36 W at the glass (about 130 to 160 degrees Celsius), 18 W at the shade that the radiation and the rising hot air warm, 6 W at the cap and the collar; the split is an authored estimate.",
        "Ansicht einer Pendelleuchte mit Deckenrosette, 0,52 m langem Textilkabel, Messinghülse, gewölbtem emailliertem Schirm (0,266 m breit, 0,133 m hoch) und einer A60-Glühlampe mit E27-Sockel (0,060 m Durchmesser, 0,105 m Länge, IEC 60064 und IEC 60061-1), die aus dem offenen Rand herabhängt. Der Ursprung ist der tiefste Punkt der Lampe; die Rosette liegt 0,8 m darüber, sodass ein Aufrufer den Ursprung 0,8 m unter der Decke setzt. Maße und Masse von 0,62 kg sind aus dieser Produktklasse gerundet (Schätzwert). Die elektrische Leistung auf Stufe 1 ist die Nennleistung von 60 W, die vollständig als Wärme im Raum endet: eine Glühlampe wandelt etwa 95 % ihrer Leistung in Infrarotstrahlung und Wärme um, und das sichtbare Licht wird von den Raumflächen absorbiert. Das Asset gibt sie in den Flächen der Leuchte ab: 36 W am Glas (etwa 130 bis 160 Grad Celsius), 18 W am Schirm, den Strahlung und aufsteigende Heißluft erwärmen, 6 W an Sockel und Hülse; die Aufteilung ist ein Schätzwert.",
      ),
    },
    dimensions: { heightM: 0.8, widthM: 0.266, depthM: 0.266, massKg: 0.62 },
    materials: [
      { id: "bulb-glass", title: tx("Clear soda-lime glass", "Klares Kalk-Natron-Glas"), emissivity: 0.9, solarAbsorptivity: 0.1, densityKgM3: 2500, conductivityWmK: 1.0, specificHeatJKgK: 840, source: "literature", reference: "incropera-2018" },
      { id: "brass-lacquered", title: tx("Lacquered brass", "Lackiertes Messing"), emissivity: 0.3, solarAbsorptivity: 0.5, densityKgM3: 8500, conductivityWmK: 110, specificHeatJKgK: 380, source: "literature", reference: "incropera-2018" },
      { id: "enamel-white", title: tx("White enamel on steel", "Weißes Email auf Stahl"), emissivity: 0.9, solarAbsorptivity: 0.3, densityKgM3: 7850, conductivityWmK: 50, specificHeatJKgK: 450, source: SOURCE, reference: "incropera-2018" },
      { id: "textile-cord", title: tx("Textile-sheathed PVC cord", "Textilummanteltes PVC-Kabel"), emissivity: 0.9, solarAbsorptivity: 0.6, densityKgM3: 1300, conductivityWmK: 0.2, specificHeatJKgK: 1300, source: SOURCE, reference: "iso-10456-2007" },
      { id: "rose-polymer", title: tx("White thermoset rose", "Weiße Duroplastrosette"), emissivity: 0.9, solarAbsorptivity: 0.3, densityKgM3: 1400, conductivityWmK: 0.3, specificHeatJKgK: 1400, source: SOURCE, reference: "iso-10456-2007" },
    ],
    rig: {
      kind: "mechanical",
      root: "rose",
      bones: [
        bone("rose", null, ["Ceiling rose", "Deckenrosette"], 0, 0.8, 0.024, { type: "fixed" }, { kg: 0.06, comX: 0, comY: -0.012, source: SOURCE }),
        bone("cord", "rose", ["Cord", "Kabel"], 0, -0.024, 0.521, { type: "fixed" }, { kg: 0.07, comX: 0, comY: -0.26, source: SOURCE }),
        bone("holder", "cord", ["Brass collar", "Messinghülse"], 0, -0.521, 0.03, { type: "fixed" }, { kg: 0.07, comX: 0, comY: -0.015, source: SOURCE }),
        bone("shade", "holder", ["Enamelled shade", "Emaillierter Schirm"], 0, -0.03, 0.1385, { type: "fixed" }, { kg: 0.39, comX: 0, comY: -0.06, source: SOURCE }),
        bone("bulb", "shade", ["Incandescent bulb with cap", "Glühlampe mit Sockel"], 0, -0.225, 0.105, { type: "fixed" }, { kg: 0.03, comX: 0, comY: 0.05, source: SOURCE }),
      ],
    },
    anchors: [
      { id: "ceiling-hook", bone: "rose", x: 0, y: 0, kind: "support", title: tx("Ceiling hook behind the rose", "Deckenhaken hinter der Rosette"), support: "pin", normalDeg: 90, footprintLengthM: 0.02, footprintWidthM: 0.02 },
      { id: "shade-top", bone: "shade", x: 0, y: 0, kind: "emitter", title: tx("Top of the shade", "Scheitel des Schirms") },
    ],
    representations: { 500: rep500, 200: rep200, 100: rep100, 50: rep50 },
    symbols: [],
    poses,
    clips,
    thermal: {
      environment: { airTempC: 21, meanRadiantTempC: 21, relativeHumidity: 0.5, windMS: 0.1, windDirectionDeg: 0, solarWM2: 0, diffuseFraction: 0, pressurePa: 101325 },
      passengers: [],
      operating: [
        { id: "power", title: tx("Electrical power", "Elektrische Leistung"), levels: [level(0, "Off", "Aus"), level(0.4, "Dimmed (24 W)", "Gedimmt (24 W)"), level(1, "Lit (60 W)", "Eingeschaltet (60 W)")] },
      ],
      regions,
      emitters: [
        { id: "shade-plume", kind: "plume", title: tx("Hot air rising over the shade", "Heißluft über dem Schirm"), anchor: "shade-top", directionDeg: 90, diameterM: 0.3, region: "shade", source: SOURCE, reference: "incropera-2018" },
      ],
      assumptions: [
        tx(
          "The 60 W end entirely as heat in the room: the glass releases 36 W, the shade 18 W and the cap with the collar 6 W at level 1; the light output of about 3 W is absorbed by room surfaces and counted in these surfaces.",
          "Die 60 W enden vollständig als Wärme im Raum: das Glas gibt bei Stufe 1 36 W ab, der Schirm 18 W und Sockel mit Hülse 6 W; die Lichtleistung von etwa 3 W wird von den Raumflächen absorbiert und ist in diesen Flächen mitgezählt.",
        ),
        tx(
          "The regions exchange no heat with each other; the shade share stands for the radiation and the hot air from the bulb that the shade intercepts.",
          "Die Regionen tauschen keine Wärme miteinander aus; der Schirmanteil steht für die Strahlung und die Heißluft der Lampe, die der Schirm auffängt.",
        ),
      ],
    },
    structure: {
      gravityMS2: 9.80665,
      dynamics: "quasi-static",
      supports: ["ceiling-hook"],
      assumptions: [
        tx(
          "A rigid chain hanging from a pin at the ceiling hook; the 0.62 kg weight is carried in tension by the cord, no sway.",
          "Eine starre Kette, die an einem Festlager am Deckenhaken hängt; das Gewicht von 0,62 kg trägt das Kabel auf Zug, keine Pendelbewegung.",
        ),
      ],
      clinical: false,
    },
    presentation: { defaultPose: "on", defaultClip: "switch-on-warm-up", defaultScale: 50, viewpoints: ["elevation"] },
    exports: ALL_EXPORTS,
    completion: "authored",
  });
