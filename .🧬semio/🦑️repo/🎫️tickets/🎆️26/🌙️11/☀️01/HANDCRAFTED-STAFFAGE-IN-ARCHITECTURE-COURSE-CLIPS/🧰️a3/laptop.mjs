import { ALL_EXPORTS, HOSNI_1999, INCROPERA, ISO10456, VDI_2078, WILKINS_HOSNI, authored, bone, clip, ellipse, layer, level, path, pose, poly, rect, ref, tx, unavailable, wrap } from "./lib.mjs";

const SOURCE = "authored-estimate";
const REG = ["deck", "underside", "lid"];

const keys = () => {
  const pts = [];
  for (let i = 0; i < 8; i += 1) {
    const x0 = -0.09 + 0.0135 * i;
    pts.push([x0, 0.0125], [x0, 0.0137], [x0 + 0.011, 0.0137], [x0 + 0.011, 0.0125]);
  }
  return pts;
};

const WEDGE = "M -0.109 0.0066 L 0.109 0.0066 L 0.109 0.0092 L 0.078 0.0125 L -0.106 0.0125 C -0.108 0.0125 -0.109 0.0115 -0.109 0.0105 Z";

const rep50 = authored(
  50,
  [20, 75],
  { body: 0.18, detail: 0.13, texture: null, glazing: 0.18, ground: 0.35 },
  [
    rect("foot-front", "base", "body", 3, true, 0.15, 0.08, 0, 0.02, 0.002, { region: "underside", material: "rubber-foot" }),
    rect("foot-rear", "base", "body", 3, true, 0.15, -0.1, 0, 0.02, 0.002, { region: "underside", material: "rubber-foot" }),
    rect("base-lower", "base", "body", 3, true, 0.5, -0.109, 0.002, 0.218, 0.0046, { r: 0.0012, region: "underside", material: "alu-anodised" }),
    path("base-upper", "base", "body", 3.1, true, 0.62, WEDGE, { region: "deck", material: "alu-anodised" }),
    poly("keys", "base", "detail", 3.2, false, 0.3, keys(), false),
    rect("lid-shell", "lid", "body", 3.4, true, 0.55, 0, 0, 0.213, 0.0065, { r: 0.0022, region: "lid", material: "alu-anodised" }),
    rect("display", "lid", "glazing", 3.5, true, 0.1, 0.005, 0.0003, 0.203, 0.0023, { region: "lid", material: "display-glass" }),
    ellipse("hinge", "lid", "body", 3.6, true, 0.4, 0.002, 0.00325, 0.0036, 0.0036, { region: "lid", material: "alu-anodised" }),
  ],
  {
    silhouette: ["A 0.22 m by 0.22 m side view, 4.4 mm by 4.4 mm on paper: a wedge-fronted base slab on two rubber feet and a tall lid leaning back by 15 degrees, joined by a round hinge barrel.", "Eine Seitenansicht von 0,22 m mal 0,22 m, auf Papier 4,4 mm mal 4,4 mm: eine Basisplatte mit keilförmiger Vorderkante auf zwei Gummifüßen und ein hoher, um 15 Grad nach hinten geneigter Deckel, verbunden durch eine runde Scharniertrommel."],
    retained: ["Overall height 0.218 m with the lid open at 105 degrees, base depth 0.218 m, base thickness 12.5 mm, lid thickness 6.5 mm, both feet, the wedge front, the hinge barrel, the glass of the display and a row of key tops.", "Gesamthöhe 0,218 m bei 105 Grad geöffnetem Deckel, Basistiefe 0,218 m, Basisdicke 12,5 mm, Deckeldicke 6,5 mm, beide Füße, die Keilkante, die Scharniertrommel, das Displayglas und eine Reihe Tastenköpfe."],
    merged: ["The anodised shell of the underside and the one of the deck are two stacked slabs; the lid shell, bezel and back cover are one shape.", "Die eloxierte Unterschale und die Deckschale bilden zwei gestapelte Platten; Deckelschale, Rahmen und Rückwand sind eine Form."],
    omitted: ["Ports, vents, the touchpad recess, the camera and the speaker slots, which lie outside the side view.", "Anschlüsse, Lüftungsschlitze, die Touchpad-Mulde, die Kamera und die Lautsprecherschlitze, die außerhalb der Seitenansicht liegen."],
    thinElements: ["The 2.3 mm display glass is drawn as a 0.046 mm glass slab inside the lid; the 2 mm feet stay real and rely on the pen weight to reach the feature width.", "Das 2,3 mm dicke Displayglas ist als 0,046-mm-Glasplatte im Deckel gezeichnet; die 2 mm hohen Füße bleiben real und erreichen die Mindestbreite durch die Strichstärke."],
    hatching: ["None; the key tops are one detail polyline.", "Keine; die Tastenköpfe sind ein Detailzug."],
    styles: ["Slabs 0.5 to 0.62, lid 0.55, display 0.1 dark, feet 0.15; body pen 0.18 mm, detail 0.13 mm, glazing 0.18 mm.", "Platten 0,5 bis 0,62, Deckel 0,55, Display 0,1 dunkel, Füße 0,15; Körperstift 0,18 mm, Detail 0,13 mm, Glas 0,18 mm."],
    overlays: ["The keyboard deck, the underside and the lid are separate regions, so the thermal view paints the hot underside and deck apart from the cooler lid.", "Tastaturdeck, Unterseite und Deckel sind getrennte Regionen, sodass die Thermalansicht die heiße Unterseite und das Deck vom kühleren Deckel trennt."],
  },
);

const rep100 = authored(
  100,
  [75, 150],
  { body: 0.18, detail: 0.13, texture: null, glazing: null, ground: 0.25 },
  [
    rect("base-lower", "base", "body", 3, true, 0.5, -0.109, 0, 0.218, 0.0066, { r: 0.0012, region: "underside", material: "alu-anodised" }),
    path("base-upper", "base", "body", 3.1, true, 0.62, WEDGE, { region: "deck", material: "alu-anodised" }),
    rect("lid-shell", "lid", "body", 3.4, true, 0.5, 0, 0, 0.213, 0.0065, { r: 0.002, region: "lid", material: "alu-anodised" }),
    poly("screen-line", "lid", "detail", 3.5, false, 0.2, [[0.006, 0.0021], [0.207, 0.0021]], false),
  ],
  {
    silhouette: ["An L-shaped side sign 2.2 mm by 2.2 mm: a sloped-front base slab with the lid leaning back from its rear edge.", "Ein L-förmiges Seitenzeichen von 2,2 mm mal 2,2 mm: eine Basisplatte mit geneigter Vorderkante, von deren Hinterkante sich der Deckel nach hinten lehnt."],
    retained: ["Overall height 0.218 m, base depth 0.218 m, the wedge front, the base slab height and the lid thickness; the screen face as one line.", "Gesamthöhe 0,218 m, Basistiefe 0,218 m, die Keilkante, die Höhe der Basisplatte und die Deckeldicke; die Bildschirmseite als ein Strich."],
    merged: ["The feet join the underside slab, the hinge barrel joins the lid shell, glass and bezel join the lid.", "Die Füße gehen in der Unterplatte auf, die Scharniertrommel in der Deckelschale, Glas und Rahmen im Deckel."],
    omitted: ["Key tops, hinge barrel as a separate form, ports, vents, camera.", "Tastenköpfe, die Scharniertrommel als eigene Form, Anschlüsse, Lüftung, Kamera."],
    thinElements: ["The 12.5 mm base is 0.125 mm of paper; the 0.18 mm pen outline lifts the slab to the feature width, so the two base slabs read as one dark block with a lighter deck line.", "Die 12,5 mm dicke Basis sind 0,125 mm Papier; die 0,18-mm-Umrisslinie hebt die Platte auf die Mindestbreite, sodass die beiden Basisplatten als ein dunkler Block mit hellerer Decklinie lesen."],
    hatching: ["None.", "Keine."],
    styles: ["Underside 0.5, deck 0.62, lid 0.5; body pen 0.18 mm, the screen line 0.13 mm.", "Unterseite 0,5, Deck 0,62, Deckel 0,5; Körperstift 0,18 mm, der Bildschirmstrich 0,13 mm."],
    overlays: ["Deck, underside and lid keep their own regions, so the heat of the underside and the keyboard deck reads separately from the lid.", "Deck, Unterseite und Deckel behalten ihre eigenen Regionen, sodass die Wärme von Unterseite und Tastaturdeck getrennt vom Deckel lesbar bleibt."],
  },
);

const rep200 = authored(
  200,
  [150, 350],
  { body: 0.18, detail: null, texture: null, glazing: null, ground: 0.25 },
  [
    rect("base", "base", "body", 3, true, 0.55, -0.109, 0, 0.218, 0.0125, { r: 0.002, regions: ["deck", "underside"], material: "alu-anodised" }),
    rect("lid", "lid", "body", 3.4, true, 0.5, -0.003, -0.003, 0.219, 0.0095, { r: 0.002, region: "lid", material: "alu-anodised" }),
  ],
  {
    silhouette: ["Two bars, 1.1 mm by 1.1 mm on paper: a base bar and a lid bar leaning back from its rear end.", "Zwei Balken, auf Papier 1,1 mm mal 1,1 mm: ein Basisbalken und ein Deckelbalken, der sich von dessen hinterem Ende nach hinten lehnt."],
    retained: ["Overall height 0.218 m, depth 0.218 m, the lid angle and the hinge at the rear edge of the base.", "Gesamthöhe 0,218 m, Tiefe 0,218 m, der Deckelwinkel und das Scharnier an der Hinterkante der Basis."],
    merged: ["Deck and underside merge into one base bar; hinge, shell, bezel and glass merge into one lid bar.", "Deck und Unterseite gehen in einem Basisbalken auf; Scharnier, Schale, Rahmen und Glas in einem Deckelbalken."],
    omitted: ["Feet, wedge front, key tops, display line and every port.", "Füße, Keilkante, Tastenköpfe, Displaylinie und alle Anschlüsse."],
    thinElements: ["The 6.5 mm lid is widened to 9.5 mm (0.0475 mm paper) so that pen and fill reach the feature width; the base keeps its real 12.5 mm.", "Der 6,5 mm dicke Deckel ist auf 9,5 mm verbreitert (0,0475 mm Papier), damit Strich und Füllung die Mindestbreite erreichen; die Basis behält ihre realen 12,5 mm."],
    hatching: ["None.", "Keine."],
    styles: ["Base 0.55, lid 0.5; one body pen of 0.18 mm.", "Basis 0,55, Deckel 0,5; ein Körperstift von 0,18 mm."],
    overlays: ["The base bar lists deck and underside as merged regions; the lid bar is the lid region.", "Der Basisbalken führt Deck und Unterseite als zusammengeführte Regionen; der Deckelbalken ist die Deckelregion."],
  },
);

const rep500 = unavailable(
  500,
  ["At 1:500 the whole laptop is 0.4 mm wide and its lid is a 0.013 mm sliver, below every print profile; only a dot would remain.", "Bei 1:500 ist der ganze Laptop 0,4 mm breit und sein Deckel ein 0,013 mm dünner Span, unter jedem Druckprofil; es bliebe nur ein Punkt."],
  ["Lid thickness of 6.5 mm", "Deckeldicke von 6,5 mm"],
  0.0065,
  [{ kind: "scale", denominator: 200 }, { kind: "scale", denominator: 100 }],
);

const heat = (key) => ({ key, heatW: 0 });

const regions = [
  {
    id: "deck",
    title: tx("Keyboard deck and palm rests", "Tastaturdeck und Handballenauflage"),
    kind: "surface",
    model: "operating",
    material: "alu-anodised",
    areaM2: 0.068,
    characteristicLengthM: 0.22,
    geometry: "horizontal-plate-up",
    skyViewFactor: 0.5,
    layers: [layer("deck-shell", ["Anodised aluminium deck 1.2 mm", "Eloxiertes Aluminiumdeck 1,2 mm"], "metal", "alu-anodised", 0.0012, 7e-6, 4700, SOURCE, "incropera-2018")],
    heatCapacityJM2K: 4700,
    operating: { key: "power", heatW: 17 },
    source: SOURCE,
    reference: "wilkins-hosni-2011",
  },
  {
    id: "underside",
    title: tx("Underside with CPU hot spot", "Unterseite mit CPU-Wärmefleck"),
    kind: "surface",
    model: "operating",
    material: "alu-anodised",
    areaM2: 0.068,
    characteristicLengthM: 0.22,
    geometry: "horizontal-plate-down",
    skyViewFactor: 0.0,
    layers: [layer("bottom-shell", ["Anodised aluminium bottom plate 1.2 mm", "Eloxierte Aluminium-Bodenplatte 1,2 mm"], "metal", "alu-anodised", 0.0012, 7e-6, 4700, SOURCE, "incropera-2018")],
    heatCapacityJM2K: 4700,
    operating: { key: "power", heatW: 24 },
    source: SOURCE,
    reference: "wilkins-hosni-2011",
  },
  {
    id: "lid",
    title: tx("Lid with display panel", "Deckel mit Displaypanel"),
    kind: "surface",
    model: "operating",
    material: "alu-anodised",
    areaM2: 0.137,
    characteristicLengthM: 0.2,
    geometry: "vertical-plate",
    skyViewFactor: 0.5,
    layers: [layer("lid-shell", ["Anodised aluminium lid 1 mm with display stack", "Eloxierter Aluminiumdeckel 1 mm mit Displayaufbau"], "metal", "alu-anodised", 0.001, 6e-6, 2900, SOURCE, "incropera-2018")],
    heatCapacityJM2K: 2900,
    operating: { key: "power", heatW: 4 },
    source: SOURCE,
    reference: "wilkins-hosni-2011",
  },
];

const bind = (deck, underside, lid, power, plume, contact) => ({
  regions: {
    deck: { exposedFraction: deck, sunlitFraction: 0 },
    underside: { exposedFraction: underside, sunlitFraction: 0, contact: { fraction: contact, tempC: 21 } },
    lid: { exposedFraction: lid, sunlitFraction: 0 },
  },
  emitters: { "deck-plume": plume },
  operating: { power },
});

const OPEN = { lid: { angleDeg: 105 } };
const CLOSED = { lid: { angleDeg: 0 } };

const poses = [
  pose(
    "open",
    ["Open, working", "Offen, in Arbeit"],
    ["Lid open at 105 degrees, full office load of 45 W: the underside and the deck are warm, the lid is a little warm, a plume rises behind the keyboard.", "Deckel auf 105 Grad geöffnet, volle Büroauslastung von 45 W: Unterseite und Deck sind warm, der Deckel leicht warm, hinter der Tastatur steigt eine Fahne auf."],
    OPEN,
    ["feet-contact"],
    "auto",
    bind(1, 0.8, 1, 1, 1, 0.06),
  ),
  pose(
    "open-idle",
    ["Open, idle", "Offen, im Leerlauf"],
    ["Lid open, screen on and nothing computed: 11 W, a faint warmth.", "Deckel offen, Bildschirm an und keine Rechenlast: 11 W, eine schwache Wärme."],
    OPEN,
    ["feet-contact"],
    "auto",
    bind(1, 0.8, 1, 0.25, 0.4, 0.06),
  ),
  pose(
    "closed",
    ["Closed, asleep", "Geschlossen, im Ruhezustand"],
    ["Lid closed, the machine sleeps at about 1.4 W: the deck and the screen face are covered, the shell is at room temperature.", "Deckel geschlossen, die Maschine ruht bei etwa 1,4 W: Deck und Bildschirmseite sind verdeckt, die Schale hat Raumtemperatur."],
    CLOSED,
    ["feet-contact"],
    "auto",
    bind(0.1, 0.8, 0.5, 0.03, 0, 0.06),
  ),
];

const holdContacts = (d) => [{ anchor: "feet-contact", fromS: 0, toS: d }];

const clips = [
  clip(
    "open-lid",
    ["Open the lid", "Deckel öffnen"],
    ["The lid swings from closed to 105 degrees in two seconds and the machine wakes from sleep to idle.", "Der Deckel schwenkt in zwei Sekunden von geschlossen auf 105 Grad und die Maschine erwacht aus dem Ruhezustand in den Leerlauf."],
    2.4,
    15,
    [
      { t: 0, pose: "closed", ease: "smooth" },
      { t: 2.4, pose: "open-idle", ease: "smooth" },
    ],
    holdContacts(2.4),
    [
      ["closed", 0, 0.4, ["Closed", "Geschlossen"]],
      ["opening", 0.4, 2.4, ["Opening", "Öffnet"]],
    ],
    [
      [0.4, "motion", ["The lid starts to rise", "Der Deckel beginnt sich zu heben"], "lid"],
      [1.6, "operating", ["The machine wakes up", "Die Maschine erwacht"], "power"],
    ],
    [["operating:power", "smooth", [[0, 0.03], [1.6, 0.03], [2.4, 0.25]]], ["emitter:deck-plume", "smooth", [[0, 0], [2.4, 0.4]]]],
  ),
  clip(
    "work-and-sleep",
    ["Work, then close the lid", "Arbeiten, dann Deckel schließen"],
    ["Ten minutes of heat: the open machine idles, works at full load from the tenth second and goes to sleep with the lid closed after five minutes, so the shell cools down.", "Zehn Minuten Wärme: die offene Maschine läuft im Leerlauf, arbeitet ab der zehnten Sekunde unter Volllast und geht nach fünf Minuten mit geschlossenem Deckel in den Ruhezustand, sodass die Schale abkühlt."],
    600,
    1,
    [
      { t: 0, pose: "open-idle", ease: "step" },
      { t: 10, pose: "open", ease: "step" },
      { t: 300, pose: "closed", ease: "step" },
      { t: 600, pose: "closed", ease: "step" },
    ],
    holdContacts(600),
    [
      ["idle", 0, 10, ["Idle", "Leerlauf"]],
      ["working", 10, 300, ["Full load", "Volllast"]],
      ["asleep", 300, 600, ["Asleep, cooling", "Ruhezustand, kühlt ab"]],
    ],
    [
      [10, "operating", ["Full load starts", "Volllast beginnt"], "power"],
      [300, "shutdown", ["Lid closed, the machine sleeps", "Deckel zu, die Maschine ruht"], "power"],
    ],
    [["operating:power", "step", [[0, 0.25], [10, 1], [300, 0.03]]], ["emitter:deck-plume", "step", [[0, 0.4], [10, 1], [300, 0]]]],
  ),
];

export const laptop = () => {
  void REG;
  void heat;
  return wrap({
    id: "laptop",
    category: "appliances",
    title: tx("Laptop Computer", "Laptop"),
    summary: tx(
      "A 14-inch aluminium laptop, 0.218 m deep, 13 mm thick with the lid open at 105 degrees: it releases 45 W of electrical power as heat at full office load, mostly from the underside and the keyboard deck, 1.4 W asleep with the lid closed.",
      "Ein 14-Zoll-Laptop aus Aluminium, 0,218 m tief, 13 mm dick und mit auf 105 Grad geöffnetem Deckel: unter voller Büroauslastung gibt er 45 W elektrische Leistung als Wärme ab, hauptsächlich über Unterseite und Tastaturdeck, geschlossen im Ruhezustand 1,4 W.",
    ),
    tags: ["laptop", "computer", "office-equipment", "internal-gain", "electrical", "heat-source"],
    provenance: {
      author: "Semio staffage content (A3)",
      method: "handcrafted",
      created: "2026-11-01",
      references: [
        WILKINS_HOSNI,
        HOSNI_1999,
        VDI_2078,
        ref("energy-star-computers-v8", "ENERGY STAR Program Requirements for Computers, Version 8.0 (2020) (idle, sleep and off mode power classes of notebook computers)."),
        INCROPERA,
        ISO10456,
      ],
      notes: tx(
        "Side elevation of a 14-inch aluminium unibody laptop of the common thin-and-light class: 0.313 m wide, 0.218 m deep, 13 mm thick base and 6.5 mm lid, 1.30 kg with battery; all measures are rounded from that product class (authored estimate, no single product). Electrical power at level 1 is 45 W (peak of the 30 to 60 W class of Wilkins and Hosni 2011), idle 11 W and sleep 1.4 W (authored estimates for the same class). All electrical power ends as heat in the room: 17 W at the keyboard deck, 24 W at the underside above the processor and the battery, 4 W at the lid and the display backlight. The split between the deck and the underside is an authored estimate; the radiative share of the equipment follows Hosni et al. 1999.",
        "Seitenansicht eines 14-Zoll-Laptops aus Aluminium (Unibody) der gängigen flachen Leichtklasse: 0,313 m breit, 0,218 m tief, 13 mm dicke Basis und 6,5 mm Deckel, 1,30 kg mit Akku; alle Maße sind aus dieser Produktklasse gerundet (Schätzwert, kein einzelnes Produkt). Die elektrische Leistung auf Stufe 1 beträgt 45 W (Spitze der Klasse von 30 bis 60 W nach Wilkins und Hosni 2011), im Leerlauf 11 W und im Ruhezustand 1,4 W (Schätzwerte für dieselbe Klasse). Die gesamte elektrische Leistung endet als Wärme im Raum: 17 W am Tastaturdeck, 24 W an der Unterseite über Prozessor und Akku, 4 W am Deckel und an der Displayhintergrundbeleuchtung. Die Aufteilung auf Deck und Unterseite ist ein Schätzwert; der Strahlungsanteil der Geräte folgt Hosni et al. 1999.",
      ),
    },
    dimensions: { heightM: 0.218, widthM: 0.275, depthM: 0.313, massKg: 1.3 },
    materials: [
      { id: "alu-anodised", title: tx("Anodised aluminium", "Eloxiertes Aluminium"), emissivity: 0.77, solarAbsorptivity: 0.6, densityKgM3: 2700, conductivityWmK: 177, specificHeatJKgK: 875, source: "literature", reference: "incropera-2018" },
      { id: "display-glass", title: tx("Display cover glass", "Display-Deckglas"), emissivity: 0.92, solarAbsorptivity: 0.9, densityKgM3: 2500, conductivityWmK: 1.4, specificHeatJKgK: 840, source: "literature", reference: "incropera-2018" },
      { id: "rubber-foot", title: tx("Rubber foot", "Gummifuß"), emissivity: 0.9, solarAbsorptivity: 0.9, densityKgM3: 1100, conductivityWmK: 0.15, specificHeatJKgK: 1800, source: SOURCE, reference: "iso-10456-2007" },
    ],
    rig: {
      kind: "mechanical",
      root: "base",
      bones: [
        bone("base", null, ["Base with keyboard, battery and board", "Basis mit Tastatur, Akku und Platine"], 0, 0, 0.218, { type: "fixed" }, { kg: 0.82, comX: 0, comY: 0.0065, source: SOURCE }),
        bone("lid", "base", ["Lid with display", "Deckel mit Display"], -0.104, 0.0125, 0.213, { type: "revolute", minDeg: 0, maxDeg: 135 }, { kg: 0.48, comX: 0.106, comY: 0.003, source: SOURCE }),
      ],
    },
    anchors: [
      { id: "feet-contact", bone: "base", x: 0, y: 0, kind: "contact", title: tx("Rubber feet on the desk", "Gummifüße auf dem Tisch"), support: "contact", footprintLengthM: 0.19, footprintWidthM: 0.28, frictionCoefficient: 0.6 },
      { id: "deck-top", bone: "base", x: 0.01, y: 0.0125, kind: "emitter", title: tx("Warm air above the keyboard deck", "Warme Luft über dem Tastaturdeck") },
    ],
    representations: { 500: rep500, 200: rep200, 100: rep100, 50: rep50 },
    symbols: [],
    poses,
    clips,
    thermal: {
      environment: { airTempC: 21, meanRadiantTempC: 21, relativeHumidity: 0.5, windMS: 0.1, windDirectionDeg: 0, solarWM2: 0, diffuseFraction: 0, pressurePa: 101325 },
      passengers: [],
      operating: [
        {
          id: "power",
          title: tx("Electrical power", "Elektrische Leistung"),
          levels: [
            level(0, "Off", "Aus"),
            level(0.03, "Sleep (1.4 W)", "Ruhezustand (1,4 W)"),
            level(0.25, "Idle (11 W)", "Leerlauf (11 W)"),
            level(1, "Full office load (45 W)", "Volle Büroauslastung (45 W)"),
          ],
        },
      ],
      regions,
      emitters: [
        {
          id: "deck-plume",
          kind: "plume",
          title: tx("Free convection above the warm deck", "Freie Konvektion über dem warmen Deck"),
          anchor: "deck-top",
          directionDeg: 90,
          diameterM: 0.2,
          region: "deck",
          source: SOURCE,
          reference: "incropera-2018",
        },
      ],
      assumptions: [
        tx(
          "Every watt of electrical power ends as heat in the room: the three regions release 17 W, 24 W and 4 W at level 1 (45 W together, class of 30 to 60 W) and lose it by convection and radiation; the fan exhaust is not modelled separately.",
          "Jedes Watt elektrischer Leistung endet als Wärme im Raum: die drei Regionen geben bei Stufe 1 17 W, 24 W und 4 W ab (zusammen 45 W, Klasse von 30 bis 60 W) und verlieren sie durch Konvektion und Strahlung; der Lüfterauslass ist nicht gesondert modelliert.",
        ),
        tx(
          "The closed lid covers the deck and the screen face, so the closed pose binds a small exposed fraction to them; the underside rests on four rubber feet with 6 % conductive contact to the desk at room temperature.",
          "Der geschlossene Deckel verdeckt Deck und Bildschirmseite, daher bindet die geschlossene Pose einen kleinen freien Anteil an sie; die Unterseite steht auf vier Gummifüßen mit 6 % Wärmeleitkontakt zum Tisch bei Raumtemperatur.",
        ),
      ],
    },
    structure: {
      gravityMS2: 9.80665,
      dynamics: "quasi-static",
      supports: ["feet-contact"],
      assumptions: [
        tx(
          "A rigid base on four rubber feet with the lid as a rotated rigid plate; open at 105 degrees the lid overhangs the rear of the base, and the centre of pressure moves 5 cm back, still inside the feet.",
          "Eine starre Basis auf vier Gummifüßen mit dem Deckel als gedrehte starre Platte; bei 105 Grad geöffnet überragt der Deckel die Hinterkante der Basis, und der Druckmittelpunkt wandert 5 cm nach hinten, bleibt aber innerhalb der Füße.",
        ),
      ],
      clinical: false,
    },
    presentation: { defaultPose: "open", defaultClip: "open-lid", defaultScale: 50, viewpoints: ["elevation"] },
    exports: ALL_EXPORTS,
    completion: "authored",
  });
};
