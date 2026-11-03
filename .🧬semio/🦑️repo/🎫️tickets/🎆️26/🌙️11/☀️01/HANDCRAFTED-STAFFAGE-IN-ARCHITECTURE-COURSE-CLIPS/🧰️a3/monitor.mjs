import { ALL_EXPORTS, HOSNI_1999, INCROPERA, ISO10456, WILKINS_HOSNI, authored, bone, clip, ellipse, layer, level, path, pose, poly, rect, ref, tx, unavailable, wrap } from "./lib.mjs";

const SOURCE = "authored-estimate";

const BASE_PLATE = "M -0.096 0.002 L 0.114 0.002 L 0.114 0.0075 C 0.114 0.0105 0.108 0.0122 0.098 0.0122 L -0.082 0.0122 C -0.092 0.0122 -0.096 0.0105 -0.096 0.0075 Z";
const NECK = [[-0.018, 0], [0.018, 0], [0.013, 0.2], [0.016, 0.243], [-0.016, 0.243], [-0.013, 0.2]];
const HOUSING = "M -0.125 -0.0305 C -0.125 -0.015 -0.105 -0.004 -0.085 -0.004 L 0.07 -0.004 C 0.09 -0.004 0.11 -0.015 0.11 -0.0305 Z";

const vents = () => {
  const pts = [];
  for (let i = 0; i < 6; i += 1) {
    const x = 0.012 + 0.014 * i;
    pts.push([x, -0.0075], [x, -0.0125], [x + 0.0001, -0.0125], [x + 0.0001, -0.0075]);
  }
  return pts;
};

const rep50 = authored(
  50,
  [20, 75],
  { body: 0.18, detail: 0.13, texture: null, glazing: 0.13, ground: 0.35 },
  [
    rect("foot-front", "base", "body", 3, true, 0.15, 0.085, 0, 0.022, 0.002, { region: "stand", material: "rubber-foot" }),
    rect("foot-rear", "base", "body", 3, true, 0.15, -0.088, 0, 0.022, 0.002, { region: "stand", material: "rubber-foot" }),
    path("base-plate", "base", "body", 3.1, true, 0.5, BASE_PLATE, { region: "stand", material: "stand-plastic" }),
    poly("neck", "neck", "body", 3.2, true, 0.45, NECK, true, { region: "stand", material: "stand-plastic" }),
    ellipse("hinge", "neck", "body", 3.3, true, 0.35, 0, 0.243, 0.012, 0.012, { region: "stand", material: "stand-plastic" }),
    path("housing", "panel", "body", 3.4, true, 0.55, HOUSING, { region: "housing", material: "housing-plastic" }),
    poly("vents", "panel", "detail", 3.5, false, 0.3, vents(), false),
    rect("panel-slab", "panel", "body", 3.6, true, 0.6, -0.17, -0.04, 0.323, 0.0095, { r: 0.0015, region: "panel-front", material: "bezel-plastic" }),
    rect("screen-glass", "panel", "glazing", 3.7, true, 0.1, -0.158, -0.04, 0.299, 0.0016, { region: "panel-front", material: "display-glass" }),
  ],
  {
    silhouette: ["A 0.41 m side view, 8.2 mm tall on paper: a flat base plate, a slim neck, and a panel tilted back by 5 degrees with a tapered rear housing and a dark front glass.", "Eine Seitenansicht von 0,41 m, auf Papier 8,2 mm hoch: eine flache Fußplatte, ein schlanker Hals und ein um 5 Grad nach hinten geneigtes Panel mit verjüngtem Rückgehäuse und dunklem Frontglas."],
    retained: ["Overall height 0.411 m, base depth 0.21 m, the 5 degree tilt, panel height 0.323 m, the 9.5 mm panel edge, the tapered housing, the neck width and the tilt hinge, both rubber feet.", "Gesamthöhe 0,411 m, Fußtiefe 0,21 m, die Neigung von 5 Grad, Panelhöhe 0,323 m, der 9,5 mm dicke Panelrand, das verjüngte Gehäuse, die Halsbreite und das Neigegelenk, beide Gummifüße."],
    merged: ["Bezel and panel back are one slab; the stand neck and its cable duct are one profile.", "Rahmen und Panelrückseite sind eine Platte; Standfuß-Hals und Kabelführung sind ein Profil."],
    omitted: ["Ports on the rear, the on-screen-menu buttons, the power cable and the VESA mount, which lie outside the silhouette.", "Anschlüsse hinten, die Menütasten, das Netzkabel und die VESA-Aufnahme, die außerhalb der Silhouette liegen."],
    thinElements: ["The 1.6 mm cover glass is a 0.032 mm glazing stroke on the front slab; vent slots are one detail polyline of 5 mm slits.", "Das 1,6 mm dicke Deckglas ist ein 0,032-mm-Glasstrich auf der Frontplatte; Lüftungsschlitze sind ein Detailzug aus 5-mm-Schlitzen."],
    hatching: ["None.", "Keine."],
    styles: ["Base 0.5, neck 0.45, housing 0.55, panel slab 0.6, glass 0.1 dark; body pen 0.18 mm, detail and glazing 0.13 mm.", "Fuß 0,5, Hals 0,45, Gehäuse 0,55, Panelplatte 0,6, Glas 0,1 dunkel; Körperstift 0,18 mm, Detail und Glas 0,13 mm."],
    overlays: ["Panel front, rear housing and stand are three regions: the thermal view shows the warm rear housing and the cooler front and stand.", "Panelfront, Rückgehäuse und Standfuß sind drei Regionen: die Thermalansicht zeigt das warme Rückgehäuse sowie die kühlere Front und den kühleren Fuß."],
  },
);

const rep100 = authored(
  100,
  [75, 150],
  { body: 0.18, detail: 0.13, texture: null, glazing: 0.13, ground: 0.25 },
  [
    path("base-plate", "base", "body", 3.1, true, 0.5, BASE_PLATE.replace(/0\.002/g, "0"), { region: "stand", material: "stand-plastic" }),
    poly("neck", "neck", "body", 3.2, true, 0.45, [[-0.019, 0], [0.019, 0], [0.014, 0.2], [0.017, 0.243], [-0.017, 0.243], [-0.014, 0.2]], true, { region: "stand", material: "stand-plastic" }),
    path("housing", "panel", "body", 3.4, true, 0.55, HOUSING, { region: "housing", material: "housing-plastic" }),
    rect("panel-slab", "panel", "body", 3.6, true, 0.6, -0.17, -0.04, 0.323, 0.0105, { r: 0.0015, region: "panel-front", material: "bezel-plastic" }),
    rect("screen-glass", "panel", "glazing", 3.7, true, 0.1, -0.158, -0.04, 0.299, 0.0018, { region: "panel-front", material: "display-glass" }),
  ],
  {
    silhouette: ["A tall flag sign 4.1 mm high on paper: a base plate, a neck column and a slightly back-leaning panel with a wedge-shaped housing.", "Ein hohes Fahnenzeichen von 4,1 mm Höhe auf Papier: eine Fußplatte, eine Halssäule und ein leicht nach hinten geneigtes Panel mit keilförmigem Gehäuse."],
    retained: ["Height 0.411 m, base depth 0.21 m, the 5 degree tilt, the panel height and the tapered housing.", "Höhe 0,411 m, Fußtiefe 0,21 m, die Neigung von 5 Grad, die Panelhöhe und das verjüngte Gehäuse."],
    merged: ["Feet join the base plate, the tilt hinge joins the neck.", "Die Füße gehen in der Fußplatte auf, das Neigegelenk im Hals."],
    omitted: ["Feet, tilt hinge, vent slots and all connectors.", "Füße, Neigegelenk, Lüftungsschlitze und alle Anschlüsse."],
    thinElements: ["The 9.5 mm panel edge is widened to 10.5 mm, the neck by 2 mm, so that fill and 0.18 mm pen reach the feature width; the cover glass is a 0.036 mm glazing stroke.", "Der 9,5 mm dicke Panelrand ist auf 10,5 mm verbreitert, der Hals um 2 mm, damit Füllung und 0,18-mm-Strich die Mindestbreite erreichen; das Deckglas ist ein 0,036-mm-Glasstrich."],
    hatching: ["None.", "Keine."],
    styles: ["Base 0.5, neck 0.45, housing 0.55, panel 0.6, glass 0.1; body pen 0.18 mm, glazing 0.13 mm.", "Fuß 0,5, Hals 0,45, Gehäuse 0,55, Panel 0,6, Glas 0,1; Körperstift 0,18 mm, Glas 0,13 mm."],
    overlays: ["The three regions keep their own shapes, so the rear housing reads warmer than the panel face and the stand.", "Die drei Regionen behalten ihre eigenen Formen, sodass das Rückgehäuse wärmer als Panelfront und Standfuß lesbar ist."],
  },
);

const rep200 = authored(
  200,
  [150, 350],
  { body: 0.18, detail: null, texture: null, glazing: null, ground: 0.25 },
  [
    path("stand", "base", "body", 3, true, 0.5, "M -0.096 0 L 0.114 0 L 0.114 0.012 L 0.0 0.012 L 0.0 0.26 L -0.06 0.26 L -0.06 0.012 L -0.096 0.012 Z", { region: "stand", material: "stand-plastic" }),
    path("panel", "panel", "body", 3.4, true, 0.58, "M -0.17 -0.04 L 0.153 -0.04 L 0.153 -0.0275 L 0.11 -0.0275 L 0.11 -0.004 L -0.125 -0.004 L -0.125 -0.0275 L -0.17 -0.0275 Z", { regions: ["panel-front", "housing"], material: "housing-plastic" }),
  ],
  {
    silhouette: ["A flag 2.05 mm wide and 2.05 mm high on paper: a stand bar with foot and a back-leaning panel bar with a thicker lower body.", "Eine Fahne von 2,05 mm Breite und 2,05 mm Höhe auf Papier: ein Standbalken mit Fuß und ein nach hinten geneigter Panelbalken mit dickerem unterem Körper."],
    retained: ["Height 0.411 m, base depth 0.21 m, the 5 degree lean and the housing bulge.", "Höhe 0,411 m, Fußtiefe 0,21 m, die Neigung von 5 Grad und die Gehäusewölbung."],
    merged: ["Base plate, neck and hinge form one stand shape; panel face and housing form one panel shape.", "Fußplatte, Hals und Gelenk bilden eine Standform; Panelfront und Gehäuse bilden eine Panelform."],
    omitted: ["Feet, glass line, vents, tilt hinge and connectors.", "Füße, Glaslinie, Lüftung, Neigegelenk und Anschlüsse."],
    thinElements: ["The neck is widened to 0.06 m (0.3 mm) and the panel edge to 12.5 mm so that both reach the feature width.", "Der Hals ist auf 0,06 m (0,3 mm) und der Panelrand auf 12,5 mm verbreitert, damit beide die Mindestbreite erreichen."],
    hatching: ["None.", "Keine."],
    styles: ["Stand 0.5, panel 0.58; one body pen of 0.18 mm.", "Fuß 0,5, Panel 0,58; ein Körperstift von 0,18 mm."],
    overlays: ["The panel shape lists panel front and housing as merged regions; the stand is its own region.", "Die Panelform führt Panelfront und Gehäuse als zusammengeführte Regionen; der Fuß ist eine eigene Region."],
  },
);

const rep500 = unavailable(
  500,
  ["At 1:500 the monitor is 0.4 mm wide and 0.8 mm tall with a 0.015 mm thick panel edge, below every print profile.", "Bei 1:500 ist der Monitor 0,4 mm breit und 0,8 mm hoch mit einem 0,015 mm dicken Panelrand, unter jedem Druckprofil."],
  ["Panel edge thickness of 9.5 mm", "Dicke des Panelrands von 9,5 mm"],
  0.0095,
  [{ kind: "scale", denominator: 200 }, { kind: "scale", denominator: 100 }],
);

const metal = (id, en, de, mat, t, r, c) => layer(id, [en, de], "polymer", mat, t, r, c, SOURCE, "iso-10456-2007");

const regions = [
  {
    id: "panel-front",
    title: tx("Panel front with cover glass", "Panelfront mit Deckglas"),
    kind: "surface",
    model: "operating",
    material: "bezel-plastic",
    areaM2: 0.175,
    characteristicLengthM: 0.32,
    geometry: "vertical-plate",
    skyViewFactor: 0.5,
    layers: [metal("front-stack", "LCD stack with cover glass 4 mm", "LCD-Aufbau mit Deckglas 4 mm", "display-glass", 0.004, 0.0037, 4800)],
    heatCapacityJM2K: 4800,
    operating: { key: "power", heatW: 9 },
    source: SOURCE,
    reference: "wilkins-hosni-2011",
  },
  {
    id: "housing",
    title: tx("Rear housing with power supply and driver boards", "Rückgehäuse mit Netzteil und Treiberplatinen"),
    kind: "surface",
    model: "operating",
    material: "housing-plastic",
    areaM2: 0.22,
    characteristicLengthM: 0.3,
    geometry: "vertical-plate",
    skyViewFactor: 0.5,
    layers: [metal("housing-shell", "ABS housing 2 mm", "ABS-Gehäuse 2 mm", "housing-plastic", 0.002, 0.01, 4800)],
    heatCapacityJM2K: 4800,
    operating: { key: "power", heatW: 16 },
    source: SOURCE,
    reference: "wilkins-hosni-2011",
  },
  {
    id: "stand",
    title: tx("Stand with neck and base plate", "Standfuß mit Hals und Fußplatte"),
    kind: "surface",
    model: "passive",
    material: "stand-plastic",
    areaM2: 0.125,
    characteristicLengthM: 0.2,
    geometry: "vertical-plate",
    skyViewFactor: 0.5,
    layers: [metal("stand-body", "ABS over a steel core, 3 mm", "ABS über Stahlkern, 3 mm", "stand-plastic", 0.003, 0.015, 4800)],
    heatCapacityJM2K: 4800,
    source: SOURCE,
    reference: "iso-10456-2007",
  },
];

const bind = (power, plume) => ({
  regions: {
    "panel-front": { exposedFraction: 1, sunlitFraction: 0 },
    housing: { exposedFraction: 1, sunlitFraction: 0 },
    stand: { exposedFraction: 0.9, sunlitFraction: 0, contact: { fraction: 0.05, tempC: 21 } },
  },
  emitters: { "rear-plume": plume },
  operating: { power },
});

const poses = [
  pose("on", ["On, showing a picture", "An, zeigt ein Bild"], ["Backlight on, 25 W: the rear housing with the power supply is warmest, the panel front is a few kelvin above the room, a plume rises behind the panel.", "Hintergrundbeleuchtung an, 25 W: das Rückgehäuse mit dem Netzteil ist am wärmsten, die Panelfront liegt einige Kelvin über der Raumtemperatur, hinter dem Panel steigt eine Fahne auf."], {}, ["base-contact"], "auto", bind(1, 1)),
  pose("standby", ["In standby", "Im Bereitschaftsbetrieb"], ["Backlight off, only the controller listens: 0.25 W, no measurable warmth.", "Hintergrundbeleuchtung aus, nur die Steuerung lauscht: 0,25 W, keine messbare Wärme."], {}, ["base-contact"], "auto", bind(0.01, 0)),
  pose("off", ["Switched off", "Ausgeschaltet"], ["Switched off at the mains switch, room temperature.", "Am Netzschalter ausgeschaltet, Raumtemperatur."], {}, ["base-contact"], "auto", bind(0, 0)),
];

const hold = (d) => [{ anchor: "base-contact", fromS: 0, toS: d }];

const clips = [
  clip(
    "power-on-warm-up",
    ["Power on and warm up", "Einschalten und aufwärmen"],
    ["Cold start: the monitor is switched on after five seconds and its housing and panel warm up over ten minutes to their working temperature.", "Kaltstart: der Monitor wird nach fünf Sekunden eingeschaltet, und Gehäuse und Panel wärmen sich über zehn Minuten auf ihre Arbeitstemperatur auf."],
    600,
    1,
    [{ t: 0, pose: "off", ease: "step" }, { t: 5, pose: "on", ease: "step" }, { t: 600, pose: "on", ease: "step" }],
    hold(600),
    [["cold", 0, 5, ["Off", "Aus"]], ["warm-up", 5, 600, ["On, warming up", "An, wärmt auf"]]],
    [[5, "operating", ["Switched on", "Eingeschaltet"], "power"]],
    [["operating:power", "step", [[0, 0], [5, 1]]], ["emitter:rear-plume", "step", [[0, 0], [5, 1]]]],
  ),
  clip(
    "standby-cool-down",
    ["Standby and cool down", "Bereitschaft und abkühlen"],
    ["The warm monitor goes to standby after five seconds; the housing and the panel cool down to room temperature over ten minutes.", "Der warme Monitor geht nach fünf Sekunden in den Bereitschaftsbetrieb; Gehäuse und Panel kühlen in zehn Minuten auf Raumtemperatur ab."],
    600,
    1,
    [{ t: 0, pose: "on", ease: "step" }, { t: 5, pose: "standby", ease: "step" }, { t: 600, pose: "standby", ease: "step" }],
    hold(600),
    [["working", 0, 5, ["On", "An"]], ["cool-down", 5, 600, ["Standby, cooling", "Bereitschaft, kühlt ab"]]],
    [[5, "shutdown", ["Switched to standby", "In den Bereitschaftsbetrieb geschaltet"], "power"]],
    [["operating:power", "step", [[0, 1], [5, 0.01]]], ["emitter:rear-plume", "step", [[0, 1], [5, 0]]]],
  ),
];

export const monitor = () =>
  wrap({
    id: "desktop-monitor",
    category: "appliances",
    title: tx("Desktop Monitor with Stand", "Desktop-Monitor mit Standfuß"),
    summary: tx(
      "A 24-inch LED desktop monitor on a stand, 0.41 m high and 0.21 m deep, panel tilted back by 5 degrees: it releases 25 W as heat when on, most of it through the rear housing, 0.25 W in standby.",
      "Ein 24-Zoll-LED-Desktop-Monitor auf Standfuß, 0,41 m hoch und 0,21 m tief, Panel um 5 Grad nach hinten geneigt: eingeschaltet gibt er 25 W als Wärme ab, den größten Teil über das Rückgehäuse, im Bereitschaftsbetrieb 0,25 W.",
    ),
    tags: ["monitor", "display", "office-equipment", "internal-gain", "electrical", "heat-source"],
    provenance: {
      author: "Semio staffage content (A3)",
      method: "handcrafted",
      created: "2026-11-01",
      references: [
        WILKINS_HOSNI,
        HOSNI_1999,
        ref("energy-star-displays-v8", "ENERGY STAR Program Requirements for Displays, Version 8.0 (2019) (on, sleep and off mode power of computer displays)."),
        INCROPERA,
        ISO10456,
      ],
      notes: tx(
        "Side elevation of a 24-inch (0.61 m diagonal) 16:9 IPS LED monitor with a height-fixed stand: panel 0.54 m wide and 0.323 m high, thin edge 9.5 mm, rear housing up to 40 mm, base plate 0.21 m deep, mass 3.6 kg; the sizes are rounded from this product class (authored estimate, no single product). The 25 W at level 1 are an authored estimate within the 15 to 35 W class of the measured LCD displays of Wilkins and Hosni 2011, standby 0.25 W follows the 0.5 W ceiling of the ENERGY STAR displays specification. All electrical power ends as heat: 9 W at the panel front (LED backlight and panel), 16 W at the rear housing (power supply, driver boards, backlight driver).",
        "Seitenansicht eines 24-Zoll-16:9-IPS-LED-Monitors (0,61 m Diagonale) mit höhenfestem Standfuß: Panel 0,54 m breit und 0,323 m hoch, dünner Rand 9,5 mm, Rückgehäuse bis 40 mm, Fußplatte 0,21 m tief, Masse 3,6 kg; die Maße sind aus dieser Produktklasse gerundet (Schätzwert, kein einzelnes Produkt). Die 25 W auf Stufe 1 sind ein Schätzwert innerhalb der Klasse von 15 bis 35 W gemessener LCD-Displays von Wilkins und Hosni 2011, der Bereitschaftsbetrieb mit 0,25 W folgt der Obergrenze von 0,5 W der ENERGY-STAR-Anforderungen für Displays. Alle elektrische Leistung endet als Wärme: 9 W an der Panelfront (LED-Hintergrundbeleuchtung und Panel), 16 W am Rückgehäuse (Netzteil, Treiberplatinen, Treiber der Hintergrundbeleuchtung).",
      ),
    },
    dimensions: { heightM: 0.411, widthM: 0.211, depthM: 0.54, massKg: 3.6 },
    materials: [
      { id: "bezel-plastic", title: tx("Black ABS bezel", "Schwarzer ABS-Rahmen"), emissivity: 0.9, solarAbsorptivity: 0.9, densityKgM3: 1050, conductivityWmK: 0.17, specificHeatJKgK: 1400, source: SOURCE, reference: "iso-10456-2007" },
      { id: "housing-plastic", title: tx("Grey ABS rear housing", "Grauweißes ABS-Rückgehäuse"), emissivity: 0.9, solarAbsorptivity: 0.7, densityKgM3: 1050, conductivityWmK: 0.17, specificHeatJKgK: 1400, source: SOURCE, reference: "iso-10456-2007" },
      { id: "stand-plastic", title: tx("ABS stand over steel core", "ABS-Standfuß über Stahlkern"), emissivity: 0.9, solarAbsorptivity: 0.7, densityKgM3: 1900, conductivityWmK: 0.4, specificHeatJKgK: 1000, source: SOURCE, reference: "iso-10456-2007" },
      { id: "display-glass", title: tx("Display cover glass", "Display-Deckglas"), emissivity: 0.92, solarAbsorptivity: 0.9, densityKgM3: 2500, conductivityWmK: 1.4, specificHeatJKgK: 840, source: "literature", reference: "incropera-2018" },
      { id: "rubber-foot", title: tx("Rubber foot", "Gummifuß"), emissivity: 0.9, solarAbsorptivity: 0.9, densityKgM3: 1100, conductivityWmK: 0.15, specificHeatJKgK: 1800, source: SOURCE, reference: "iso-10456-2007" },
    ],
    rig: {
      kind: "mechanical",
      root: "base",
      bones: [
        bone("base", null, ["Base plate", "Fußplatte"], 0, 0, 0.21, { type: "fixed" }, { kg: 0.85, comX: 0.01, comY: 0.006, source: SOURCE }),
        bone("neck", "base", ["Neck column", "Halssäule"], -0.06, 0.012, 0.243, { type: "fixed" }, { kg: 0.35, comX: 0, comY: 0.12, source: SOURCE }),
        { ...bone("panel", "neck", ["Display panel with housing", "Displaypanel mit Gehäuse"], 0, 0.243, 0.323, { type: "revolute", minDeg: -5, maxDeg: 20 }, { kg: 2.4, comX: -0.01, comY: -0.025, source: SOURCE }), angleDeg: 95 },
      ],
    },
    anchors: [
      { id: "base-contact", bone: "base", x: 0.01, y: 0, kind: "contact", title: tx("Base plate on the desk", "Fußplatte auf dem Tisch"), support: "contact", footprintLengthM: 0.19, footprintWidthM: 0.2, frictionCoefficient: 0.6 },
      { id: "housing-top", bone: "panel", x: 0.06, y: -0.004, kind: "emitter", title: tx("Warm air above the rear housing", "Warme Luft über dem Rückgehäuse") },
    ],
    representations: { 500: rep500, 200: rep200, 100: rep100, 50: rep50 },
    symbols: [],
    poses,
    clips,
    thermal: {
      environment: { airTempC: 21, meanRadiantTempC: 21, relativeHumidity: 0.5, windMS: 0.1, windDirectionDeg: 0, solarWM2: 0, diffuseFraction: 0, pressurePa: 101325 },
      passengers: [],
      operating: [
        { id: "power", title: tx("Electrical power", "Elektrische Leistung"), levels: [level(0, "Off", "Aus"), level(0.01, "Standby (0.25 W)", "Bereitschaft (0,25 W)"), level(1, "On (25 W)", "An (25 W)")] },
      ],
      regions,
      emitters: [
        { id: "rear-plume", kind: "plume", title: tx("Free convection behind the warm housing", "Freie Konvektion hinter dem warmen Gehäuse"), anchor: "housing-top", directionDeg: 90, diameterM: 0.3, region: "housing", source: SOURCE, reference: "incropera-2018" },
      ],
      assumptions: [
        tx(
          "Every watt ends as heat in the room: the panel front releases 9 W and the rear housing 16 W at level 1 (25 W together); the stand is passive and stays at room temperature.",
          "Jedes Watt endet als Wärme im Raum: die Panelfront gibt bei Stufe 1 9 W und das Rückgehäuse 16 W ab (zusammen 25 W); der Standfuß ist passiv und bleibt auf Raumtemperatur.",
        ),
      ],
    },
    structure: {
      gravityMS2: 9.80665,
      dynamics: "quasi-static",
      supports: ["base-contact"],
      assumptions: [
        tx(
          "A rigid body on four rubber feet; the 2.4 kg panel leans back by 5 degrees over the neck and its centre of mass lies 0.05 m behind the base centre, inside the footprint.",
          "Ein starrer Körper auf vier Gummifüßen; das 2,4 kg schwere Panel neigt sich um 5 Grad über dem Hals nach hinten und sein Schwerpunkt liegt 0,05 m hinter der Fußmitte, innerhalb der Standfläche.",
        ),
      ],
      clinical: false,
    },
    presentation: { defaultPose: "on", defaultClip: "power-on-warm-up", defaultScale: 50, viewpoints: ["elevation"] },
    exports: ALL_EXPORTS,
    completion: "authored",
  });
