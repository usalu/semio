import { ALL_EXPORTS, INCROPERA, authored, clip, layer, level, pose, ref, tx, unavailable, wrap } from "./lib.mjs";
import { materialOf, space, sum } from "./space.mjs";

const SRC = "authored-estimate";
const m = (kg, comX, comY) => ({ kg, comX, comY, distribution: "point", source: SRC });
const fixed = { type: "fixed" };

const sp = space([
  ["plate", null, 0, 0, { title: tx("Wall plate", "Wandplatte"), lengthM: 0.22, joint: fixed, mass: m(0.5, 0, 0) }],
  ["ring", "plate", 0, 0, { title: tx("Housing ring with motor spider", "Gehäusering mit Motorstern"), lengthM: 0.2, joint: fixed, mass: m(0.28, 0, 0) }],
  ["rotor", "ring", 0, 0, { title: tx("Rotor with blades", "Laufrad mit Flügeln"), lengthM: 0.09, joint: { type: "revolute" }, mass: m(0.12, 0, 0) }],
]);
const { path, ellipse, rect } = sp;

const r5 = (n) => Math.round(n * 1e5) / 1e5;
const bladePath = (k, n, rootR, tipR, rootW, tipW, sweepDeg) => {
  const steps = 8;
  const edge = (sign) => {
    const pts = [];
    for (let i = 0; i <= steps; i += 1) {
      const u = i / steps;
      const r = rootR + (tipR - rootR) * u;
      const w = rootW + (tipW - rootW) * u ** 0.7;
      const a = ((360 / n) * k + sweepDeg * u) * (Math.PI / 180) + sign * Math.atan2(w / 2, r);
      pts.push([r5(r * Math.cos(a)), r5(r * Math.sin(a))]);
    }
    return pts;
  };
  const lead = edge(1);
  const trail = edge(-1).reverse();
  const pts = [...lead, ...trail];
  return `M ${pts.map(([x, y]) => `${x} ${y}`).join(" L ")} Z`;
};
const blades = (idPrefix, n, rootR, tipR, rootW, tipW, sweep, tone, z = 3.3) =>
  Array.from({ length: n }, (_, k) => path(`${idPrefix}-${k + 1}`, "rotor", "body", z, true, tone, bladePath(k, n, rootR, tipR, rootW, tipW, sweep), { region: "blades", material: "abs-plastic" }));

const SCREWS = [[-0.092, -0.092], [0.092, -0.092], [-0.092, 0.092], [0.092, 0.092]];
const screws = SCREWS.map(([x, y], i) => ellipse(`screw-${i + 1}`, "plate", "detail", 3.2, true, 0.45, x, y, 0.008, 0.008, { region: "casing", material: "abs-plastic" }));

const rep50 = authored(
  50,
  [20, 75],
  { body: 0.18, detail: 0.13, texture: null, glazing: null, ground: 0.35 },
  [
    rect("plate", "plate", "body", 3, true, 0.88, -0.11, -0.11, 0.22, 0.22, { r: 0.014, region: "casing", material: "abs-plastic" }),
    ...screws,
    ellipse("ring", "ring", "body", 3.05, true, 0.7, 0, 0, 0.1, 0.1, { region: "casing", material: "abs-plastic" }),
    ellipse("opening", "ring", "body", 3.1, true, 0.12, 0, 0, 0.088, 0.088, {}),
    ...blades("blade", 5, 0.03, 0.091, 0.026, 0.06, 28, 0.82),
    ellipse("hub", "rotor", "body", 3.4, true, 0.55, 0, 0, 0.034, 0.034, { region: "hub", material: "motor-housing" }),
    ellipse("hub-cap", "rotor", "detail", 3.5, true, 0.9, 0.004, 0.004, 0.012, 0.012, { region: "hub", material: "motor-housing" }),
  ],
  {
    silhouette: ["A 0.22 m square wall fan, 4.4 mm on paper: a rounded plate with four screws, a ring with a dark opening and five swept blades around a hub.", "Ein quadratischer Wandventilator von 0,22 m, auf Papier 4,4 mm: eine Platte mit gerundeten Ecken und vier Schrauben, ein Ring mit dunkler Öffnung und fünf gepfeilte Flügel um eine Nabe."],
    retained: ["Plate of 0.22 m, housing ring of 0.2 m, opening of 0.176 m, five blades with their sweep, the 68 mm hub with its off-centre cap, four screws of 16 mm.", "Platte von 0,22 m, Gehäusering von 0,2 m, Öffnung von 0,176 m, fünf Flügel mit ihrer Pfeilung, die 68 mm große Nabe mit der außermittigen Kappe, vier Schrauben von 16 mm."],
    merged: ["The motor spider and the back-draught flap behind the blades are part of the dark opening.", "Der Motorstern und die Rückschlagklappe hinter den Flügeln gehen in der dunklen Öffnung auf."],
    omitted: ["The cable gland, the label and the grille bars.", "Die Kabeldurchführung, das Typenschild und die Gitterstäbe."],
    thinElements: ["The blades are 26 mm wide at the root (0.52 mm) and 60 mm at the tip; the cap is a 0.13 mm line pen.", "Die Flügel sind an der Wurzel 26 mm (0,52 mm) und an der Spitze 60 mm breit; die Kappe wird mit dem 0,13-mm-Detailstift gezeichnet."],
    hatching: ["None; the opening is a dark tone.", "Keine; die Öffnung ist ein dunkler Ton."],
    styles: ["Plate 0.88, ring 0.7, opening 0.12, blades 0.82, hub 0.55; body pen 0.18 mm, detail 0.13 mm, ground 0.35 mm.", "Platte 0,88, Ring 0,7, Öffnung 0,12, Flügel 0,82, Nabe 0,55; Körperstift 0,18 mm, Detail 0,13 mm, Boden 0,35 mm."],
    overlays: ["Casing, blades and hub are three regions: in the thermal view the hub (motor) warms and the blades stay cool; the blades turn with the rotor bone.", "Gehäuse, Flügel und Nabe sind drei Regionen: in der Thermalansicht erwärmt sich die Nabe (Motor), die Flügel bleiben kühl; die Flügel drehen sich mit dem Rotorknochen."],
  },
);

const rep100 = authored(
  100,
  [75, 150],
  { body: 0.18, detail: null, texture: null, glazing: null, ground: 0.25 },
  [
    rect("plate", "plate", "body", 3, true, 0.88, -0.11, -0.11, 0.22, 0.22, { r: 0.014, regions: ["casing"], material: "abs-plastic" }),
    ellipse("ring", "ring", "body", 3.05, true, 0.7, 0, 0, 0.1, 0.1, { region: "casing", material: "abs-plastic" }),
    ellipse("opening", "ring", "body", 3.1, true, 0.12, 0, 0, 0.086, 0.086, {}),
    ...blades("blade", 5, 0.03, 0.094, 0.034, 0.066, 28, 0.82),
    ellipse("hub", "rotor", "body", 3.4, true, 0.55, 0, 0, 0.036, 0.036, { region: "hub", material: "motor-housing" }),
  ],
  {
    silhouette: ["A 2.2 mm square with a ring and a five-bladed pinwheel.", "Ein Quadrat von 2,2 mm mit einem Ring und einem fünfflügeligen Windrad."],
    retained: ["Plate, ring, opening, five swept blades and the hub.", "Platte, Ring, Öffnung, fünf gepfeilte Flügel und die Nabe."],
    merged: ["Screws and cap are dropped; the blades touch the ring so that they join it.", "Schrauben und Kappe entfallen; die Flügel berühren den Ring und gehen in ihm auf."],
    omitted: ["Four screws, the hub cap, the spider and the flap.", "Vier Schrauben, die Nabenkappe, der Stern und die Klappe."],
    thinElements: ["The blades are widened to 34 mm at the root (0.34 mm) and 66 mm at the tip so that they print as filled shapes.", "Die Flügel sind an der Wurzel auf 34 mm (0,34 mm) und an der Spitze auf 66 mm verbreitert, damit sie als Flächen drucken."],
    hatching: ["None.", "Keine."],
    styles: ["Plate 0.88, ring 0.7, opening 0.12, blades 0.82, hub 0.55; one body pen of 0.18 mm.", "Platte 0,88, Ring 0,7, Öffnung 0,12, Flügel 0,82, Nabe 0,55; ein Körperstift von 0,18 mm."],
    overlays: ["Casing, blades and hub stay three regions, the blades turn with the rotor.", "Gehäuse, Flügel und Nabe bleiben drei Regionen, die Flügel drehen sich mit dem Rotor."],
  },
);

const rep200 = authored(
  200,
  [150, 350],
  { body: 0.13, detail: null, texture: null, glazing: null, ground: 0.25 },
  [
    rect("plate", "plate", "body", 3, true, 0.88, -0.11, -0.11, 0.22, 0.22, { r: 0.016, region: "casing", material: "abs-plastic" }),
    ellipse("opening", "ring", "body", 3.1, true, 0.12, 0, 0, 0.096, 0.096, { region: "casing", material: "abs-plastic" }),
    ...blades("blade", 3, 0.03, 0.098, 0.06, 0.09, 30, 0.82),
    ellipse("hub", "rotor", "body", 3.4, true, 0.55, 0, 0, 0.044, 0.044, { region: "hub", material: "motor-housing" }),
  ],
  {
    silhouette: ["A 1.1 mm square with a dark disc and a three-bladed pinwheel.", "Ein Quadrat von 1,1 mm mit einer dunklen Scheibe und einem dreiflügeligen Windrad."],
    retained: ["Plate, dark opening, three broad swept blades and the hub.", "Platte, dunkle Öffnung, drei breite gepfeilte Flügel und die Nabe."],
    merged: ["The housing ring joins the opening (it carries the casing region); five blades become three.", "Der Gehäusering geht in der Öffnung auf (sie trägt die Gehäuseregion); aus fünf Flügeln werden drei."],
    omitted: ["Screws, hub cap, ring outline.", "Schrauben, Nabenkappe, Ringkontur."],
    thinElements: ["The three blades are 60 to 90 mm wide (0.3 to 0.45 mm) and the hub 88 mm, so that they stay above the feature width and read as a pinwheel when they turn.", "Die drei Flügel sind 60 bis 90 mm (0,3 bis 0,45 mm) und die Nabe 88 mm breit, damit sie über der Mindestbreite bleiben und sich beim Drehen als Windrad lesen."],
    hatching: ["None.", "Keine."],
    styles: ["Plate 0.88, opening 0.12, blades 0.82, hub 0.55; one body pen of 0.13 mm.", "Platte 0,88, Öffnung 0,12, Flügel 0,82, Nabe 0,55; ein Körperstift von 0,13 mm."],
    overlays: ["The opening carries the casing region; blades and hub keep their regions.", "Die Öffnung trägt die Gehäuseregion; Flügel und Nabe behalten ihre Regionen."],
  },
);

const rep500 = unavailable(
  500,
  ["At 1:500 the 0.22 m fan is 0.44 mm wide and its blades 0.1 mm, below every print profile; use the airflow symbol of the clip at this scale.", "Bei 1:500 ist der 0,22 m große Ventilator 0,44 mm breit und seine Flügel 0,1 mm, unter jedem Druckprofil; bei diesem Maßstab dient das Luftstromsymbol des Clips."],
  ["Blades of 26 mm width", "Flügel von 26 mm Breite"],
  0.026,
  [{ kind: "scale", denominator: 200 }, { kind: "scale", denominator: 100 }],
);

const single = (id, en, de, kind, material, thicknessM, res, cap, reference = "incropera-2018") => layer(id, [en, de], kind, material, thicknessM, res, cap, SRC, reference);
const region = (id, title, kind, model, material, areaM2, characteristicLengthM, geometry, layers, heatCapacityJM2K, extra = {}, reference = "incropera-2018") => ({
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
  region("casing", ["Plate and housing ring", "Platte und Gehäusering"], "surface", "operating", "abs-plastic", 0.07, 0.22, "vertical-plate", [single("shell", "ABS shell 2.5 mm", "ABS-Schale 2,5 mm", "polymer", "abs-plastic", 0.0025, 0.012, 2500)], 2500, { operating: { key: "fan", heatW: 6 } }),
  region("blades", ["Blades", "Flügel"], "surface", "passive", "abs-plastic", 0.015, 0.06, "vertical-plate", [single("blade", "ABS blade 1.5 mm", "ABS-Flügel 1,5 mm", "polymer", "abs-plastic", 0.0015, 0.007, 1500)], 1500),
  region("hub", ["Motor hub", "Motornabe"], "engine", "operating", "motor-housing", 0.009, 0.068, "vertical-plate", [single("can", "Steel motor can 0.8 mm", "Stahlbecher des Motors 0,8 mm", "metal", "motor-housing", 0.0008, 0.00002, 6300)], 6300, { operating: { key: "fan", heatW: 22 } }),
];

const bind = (fan) => ({
  regions: {
    casing: { exposedFraction: 0.8, sunlitFraction: 0, contact: { fraction: 0.2, tempC: 21 } },
    blades: { exposedFraction: 1, sunlitFraction: 0 },
    hub: { exposedFraction: 1, sunlitFraction: 0 },
  },
  emitters: { extract: fan },
  operating: { fan },
});

const wall = ["wall-screw-top", "wall-screw-bottom"];
const poses = [
  pose("off", ["Off", "Aus"], ["The rotor stands still and the room air is at rest.", "Das Laufrad steht still und die Raumluft ruht."], {}, wall, "authored", bind(0)),
  pose("low", ["Low speed", "Kleine Stufe"], ["The rotor runs at half speed and extracts 90 cubic metres per hour; the motor takes 14 W.", "Das Laufrad läuft mit halber Drehzahl und saugt 90 Kubikmeter je Stunde ab; der Motor nimmt 14 W auf."], { rotor: { angleDeg: 18 } }, wall, "authored", bind(0.5)),
  pose("high", ["High speed", "Große Stufe"], ["The rotor runs at full speed and extracts 180 cubic metres per hour; the motor takes 28 W.", "Das Laufrad läuft mit voller Drehzahl und saugt 180 Kubikmeter je Stunde ab; der Motor nimmt 28 W auf."], { rotor: { angleDeg: 36 } }, wall, "authored", bind(1)),
];

const contacts = (d) => wall.map((anchor) => ({ anchor, fromS: 0, toS: d }));

const clips = [
  clip(
    "spin",
    ["Spin up and speed up", "Hochlaufen und beschleunigen"],
    ["The blades start from rest, reach the low stage after one second and the high stage after four; the shown turning rate is slowed to 180 and 360 degrees per second so that the blades stay readable.", "Die Flügel laufen aus dem Stillstand an, erreichen die kleine Stufe nach einer Sekunde und die große nach vier; die gezeigte Drehrate ist auf 180 und 360 Grad je Sekunde verlangsamt, damit die Flügel lesbar bleiben."],
    6,
    15,
    [
      { t: 0, pose: "off", ease: "in" },
      { t: 1, pose: "low", ease: "linear", bones: { rotor: { angleDeg: 90 } } },
      { t: 3, pose: "low", ease: "in", bones: { rotor: { angleDeg: 450 } } },
      { t: 4, pose: "high", ease: "linear", bones: { rotor: { angleDeg: 810 } } },
      { t: 6, pose: "high", ease: "linear", bones: { rotor: { angleDeg: 1530 } } },
    ],
    contacts(6),
    [["start", 0, 1, ["Starting", "Anlaufen"]], ["low", 1, 3, ["Low stage", "Kleine Stufe"]], ["high", 3, 6, ["High stage", "Große Stufe"]]],
    [[0, "operating", ["Fan switched on", "Ventilator eingeschaltet"], "fan"]],
    [["operating:fan", "linear", [[0, 0], [1, 0.5], [3, 0.5], [4, 1], [6, 1]]], ["emitter:extract", "linear", [[0, 0], [1, 0.5], [3, 0.5], [4, 1], [6, 1]]]],
  ),
  clip(
    "speed-steps",
    ["Speed steps and switch off", "Stufen und Ausschalten"],
    ["The fan is switched on at the low stage, goes to the high stage after one minute and is switched off after two more; the motor warms and cools.", "Der Ventilator wird auf der kleinen Stufe eingeschaltet, geht nach einer Minute auf die große Stufe und wird nach zwei weiteren ausgeschaltet; der Motor erwärmt sich und kühlt ab."],
    360,
    1,
    [{ t: 0, pose: "off", ease: "step" }, { t: 5, pose: "low", ease: "step" }, { t: 65, pose: "high", ease: "step" }, { t: 185, pose: "off", ease: "step" }, { t: 360, pose: "off", ease: "step" }],
    contacts(360),
    [["off", 0, 5, ["Off", "Aus"]], ["low", 5, 65, ["Low stage", "Kleine Stufe"]], ["high", 65, 185, ["High stage", "Große Stufe"]], ["cool-down", 185, 360, ["Off, cooling", "Aus, kühlt ab"]]],
    [[185, "shutdown", ["Fan switched off", "Ventilator ausgeschaltet"], "fan"]],
    [["operating:fan", "step", [[0, 0], [5, 0.5], [65, 1], [185, 0]]], ["emitter:extract", "step", [[0, 0], [5, 0.5], [65, 1], [185, 0]]]],
  ),
];

export const ductFan = () => {
  const massKg = sum(sp.bones.map((b) => b.mass.kg));
  return wrap({
    id: "duct-fan",
    category: "appliances",
    title: tx("Wall-Mounted Axial Extract Fan", "Axial-Wandventilator zur Abluft"),
    summary: tx(
      "A 0.22 m square wall-mounted axial extract fan seen from the room: a rounded plate with a ring, five swept blades on a motor hub that turn, extracting 180 m3/h at 28 W.",
      "Ein quadratischer Axial-Wandventilator von 0,22 m zur Abluft, vom Raum aus gesehen: eine Platte mit Ring und fünf gepfeilten Flügeln auf einer Motornabe, die sich drehen; er saugt 180 m3/h bei 28 W ab.",
    ),
    tags: ["fan", "axial-fan", "extract-air", "ventilation", "wall-mounted", "bathroom", "appliance", "internal-gain", "electrical", "rotating"],
    provenance: {
      author: "Semio staffage content (A10)",
      method: "handcrafted",
      created: "2026-11-01",
      references: [
        ref("iso-5801-2017", "ISO 5801:2017 Fans - Performance testing using standardized airways (volume flow, pressure and power of a fan; typical small axial wall fans of 100 to 150 mm nominal size reach 80 to 200 cubic metres per hour)."),
        ref("din-18017-3-2020", "DIN 18017-3:2020 Ventilation of bathrooms and toilets without outside windows with fans (extract flow 40 m3/h continuous, 120 m3/h demand-controlled)."),
        INCROPERA,
      ],
      notes: tx(
        "A bathroom-class axial wall fan of the 200 mm nominal size seen face-on from the room: a plate of 0.22 m by 0.22 m with rounded corners, a round housing of 0.2 m and an opening of 0.176 m with five swept ABS blades on a 68 mm motor hub. It stands for the circle-with-blades symbol of the ventilation clips. The origin is the centre of the hub in the wall plane; the fan is 0.09 m deep (depthM). Dimensions and the mass of 0.9 kg are rounded from this product class (authored estimate). At level 1 the fan extracts 0.05 m3/s (180 m3/h) with a motor power of 28 W (6 W electronics and housing, 22 W motor), at level 0.5 half the flow and 14 W, which also follows a typical axial fan: power grows faster than the flow, so the linear model is a slight overestimate of the motor heat at the low stage. The rotor turns around the axis normal to the drawing; the clip shows the real 1400 rpm slowed to 180 and 360 degrees per second.",
        "Ein Axial-Wandventilator der Nenngröße 200 mm für Bäder, frontal vom Raum aus gesehen: eine Platte von 0,22 m mal 0,22 m mit gerundeten Ecken, ein rundes Gehäuse von 0,2 m und eine Öffnung von 0,176 m mit fünf gepfeilten ABS-Flügeln auf einer 68 mm großen Motornabe. Er steht für das Symbol aus Kreis mit Flügeln der Lüftungsclips. Der Ursprung ist die Mitte der Nabe in der Wandebene; der Ventilator ist 0,09 m tief (depthM). Maße und die Masse von 0,9 kg sind aus dieser Produktklasse gerundet (Schätzwert). Auf Stufe 1 saugt er 0,05 m3/s (180 m3/h) mit einer Motorleistung von 28 W ab (6 W Elektronik und Gehäuse, 22 W Motor), bei Stufe 0,5 den halben Strom und 14 W; das folgt auch einem üblichen Axialventilator: die Leistung wächst stärker als der Volumenstrom, das lineare Modell überschätzt die Motorwärme auf der kleinen Stufe also leicht. Das Laufrad dreht sich um die Achse senkrecht zur Zeichnung; der Clip zeigt die echten 1400 U/min verlangsamt auf 180 und 360 Grad je Sekunde.",
      ),
    },
    dimensions: { heightM: 0.22, widthM: 0.22, depthM: 0.09, massKg },
    materials: [
      materialOf("abs-plastic", "White ABS plastic", "Weißer ABS-Kunststoff", 0.9, 0.3, 1050, 0.17, 1400, SRC, "incropera-2018"),
      materialOf("motor-housing", "Steel motor can", "Stahlbecher des Motors", 0.3, 0.5, 7800, 50, 490, SRC, "incropera-2018"),
    ],
    rig: { kind: "mechanical", root: "plate", bones: sp.bones.map((b) => ({ ...b, mass: { ...b.mass } })) },
    anchors: [
      { id: "wall-screw-top", bone: "plate", x: 0, y: 0.092, kind: "support", title: tx("Upper screw in the wall", "Obere Schraube in der Wand"), support: "pin", normalDeg: 0, footprintLengthM: 0.016, footprintWidthM: 0.016 },
      { id: "wall-screw-bottom", bone: "plate", x: 0, y: -0.092, kind: "support", title: tx("Lower screw in the wall", "Untere Schraube in der Wand"), support: "roller", normalDeg: 0, footprintLengthM: 0.016, footprintWidthM: 0.016, frictionCoefficient: 0.4 },
      { id: "outlet", bone: "ring", x: 0, y: 0, kind: "emitter", title: tx("Centre of the fan opening", "Mitte der Ventilatoröffnung") },
    ],
    representations: { 500: rep500, 200: rep200, 100: rep100, 50: rep50 },
    symbols: [],
    poses,
    clips,
    thermal: {
      environment: { airTempC: 21, meanRadiantTempC: 21, relativeHumidity: 0.5, windMS: 0.1, windDirectionDeg: 0, solarWM2: 0, diffuseFraction: 0, pressurePa: 101325 },
      passengers: [],
      operating: [{ id: "fan", title: tx("Fan", "Ventilator"), levels: [level(0, "Off", "Aus"), level(0.5, "Low stage (90 m3/h, 14 W)", "Kleine Stufe (90 m3/h, 14 W)"), level(1, "High stage (180 m3/h, 28 W)", "Große Stufe (180 m3/h, 28 W)")] }],
      regions,
      emitters: [{ id: "extract", kind: "fan", title: tx("Extract air through the fan opening", "Abluft durch die Ventilatoröffnung"), anchor: "outlet", directionDeg: 0, diameterM: 0.176, operatingKey: "fan", volumeFlowM3S: 0.05, temperatureRiseK: 0.4, vapourKgPerKg: 0, source: SRC, reference: "iso-5801-2017" }],
      assumptions: [
        tx(
          "The 28 W at full speed end entirely as heat: 22 W at the motor hub and 6 W at the plate and ring; the airstream carries a fraction of it as a temperature rise of 0.4 K (28 W / (0.05 m3/s x 1.2 kg/m3 x 1005 J/kgK) = 0.46 K, rounded).",
          "Die 28 W auf voller Stufe enden vollständig als Wärme: 22 W an der Motornabe und 6 W an Platte und Ring; der Luftstrom trägt einen Teil davon als Temperaturerhöhung von 0,4 K (28 W / (0,05 m3/s x 1,2 kg/m3 x 1005 J/kgK) = 0,46 K, gerundet).",
        ),
        tx(
          "The real discharge leaves normal to the drawing plane (through the wall); the emitter carries a nominal in-plane direction of 0 degrees so that the transport layers show a stream, and a mirrored cast flips it.",
          "Die echte Ausblasrichtung steht senkrecht zur Zeichenebene (durch die Wand); der Emitter trägt eine nominelle Richtung von 0 Grad in der Ebene, damit die Transportebenen einen Strom zeigen, und ein gespiegelter Cast kehrt sie um.",
        ),
      ],
    },
    structure: {
      gravityMS2: 9.80665,
      dynamics: "quasi-static",
      supports: wall,
      assumptions: [
        tx(
          "The 0.9 kg fan hangs on the upper screw (pin) and leans against the wall at the lower screw (roller): the pair carries the weight and the small overturning moment of the 90 mm deep housing.",
          "Der 0,9 kg schwere Ventilator hängt an der oberen Schraube (Festlager) und lehnt an der unteren Schraube an der Wand (Rollenlager): das Paar trägt das Gewicht und das kleine Kippmoment des 90 mm tiefen Gehäuses.",
        ),
      ],
      clinical: false,
    },
    presentation: { defaultPose: "high", defaultClip: "spin", defaultScale: 50, viewpoints: ["elevation"] },
    exports: ALL_EXPORTS,
    completion: "authored",
  });
};
