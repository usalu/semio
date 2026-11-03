import { ALL_EXPORTS, INCROPERA, authored, clip, level, pose, ref, tx, unavailable, wrap } from "./lib.mjs";
import { materialOf, space, sum } from "./space.mjs";
import { SRC, environment, fixed, m, region, single, supportAnchor } from "./kit.mjs";

const sp = space([
  ["body", null, 0, 0, { title: tx("Angle valve body with pipe stub", "Eckventilkörper mit Rohranschluss"), lengthM: 0.075, joint: fixed, mass: m(0.26, 0, -0.02) }],
  ["head", "body", 0, 0.04, { title: tx("Thermostatic head collar with sensor", "Thermostatkopf-Kragen mit Fühler"), lengthM: 0.03, joint: fixed, mass: m(0.08, 0, 0) }],
  ["knob", "head", 0, 0.04, { title: tx("Setting knob", "Einstellknopf"), lengthM: 0.02, joint: { type: "revolute", minDeg: -10, maxDeg: 300 }, mass: m(0.04, 0, 0) }],
]);
const { rect, ellipse, poly } = sp;

const bodyOutline = (stub, nut) => [
  [-stub, -0.075], [stub, -0.075], [stub, -0.052], [nut, -0.052], [nut, -0.042], [0.019, -0.042], [0.019, 0], [0.017, 0], [0.017, 0.012], [-0.017, 0.012],
  [-0.017, 0], [-0.019, 0], [-0.019, -0.042], [-nut, -0.042], [-nut, -0.052], [-stub, -0.052],
];

const rep50 = authored(
  50,
  [10, 75],
  { body: 0.18, detail: 0.13, texture: null, glazing: null, ground: 0.35 },
  [
    poly("body", "body", "body", 3, true, 0.6, bodyOutline(0.0085, 0.012), true, { region: "body", material: "brass" }),
    ellipse("collar", "head", "body", 3.1, true, 0.8, 0, 0.04, 0.03, 0.03, { region: "head", material: "abs-plastic" }),
    rect("index", "head", "detail", 3.15, true, 0.12, -0.0055, 0.0605, 0.011, 0.0085, { region: "head", material: "abs-plastic" }),
    ellipse("knob", "knob", "body", 3.2, true, 0.95, 0, 0.04, 0.02, 0.02, { region: "knob", material: "abs-plastic" }),
    rect("pointer", "knob", "detail", 3.3, true, 0.1, -0.0055, 0.0435, 0.011, 0.0135, { region: "knob", material: "abs-plastic" }),
  ],
  {
    silhouette: ["A 60 mm thermostatic radiator valve seen along its axis, 3 mm tall on paper: a round head with a turning knob and a pointer, above an angle valve body with a pipe stub below.", "Ein Thermostat-Heizkörperventil von 60 mm in Achsrichtung gesehen, auf Papier 3 mm hoch: ein runder Kopf mit drehendem Knopf und Zeiger über einem Eckventilkörper mit Rohranschluss darunter."],
    retained: ["Collar of 60 mm, knob of 40 mm with its 11 mm pointer, the index mark on the collar, the hexagon neck of the valve body and the pipe stub with its union nut.", "Kragen von 60 mm, Knopf von 40 mm mit dem 11 mm breiten Zeiger, die Indexmarke am Kragen, der Sechskanthals des Ventilkörpers und der Rohranschluss mit Überwurfmutter."],
    merged: ["Body, neck, union nut and pipe stub are one outline carrying the body region.", "Körper, Hals, Überwurfmutter und Rohranschluss sind eine Kontur, die die Körperregion trägt."],
    omitted: ["The scale numerals 1 to 5 on the collar, the lockshield thread and the second connection behind the body.", "Die Skalenziffern 1 bis 5 am Kragen, das Gewinde der Absperrkappe und der zweite Anschluss hinter dem Körper."],
    thinElements: ["Pointer and index mark are 11 mm wide (0.22 mm on paper) and filled; the pipe stub is 17 mm wide (0.34 mm).", "Zeiger und Indexmarke sind 11 mm breit (0,22 mm auf Papier) und gefüllt; der Rohranschluss ist 17 mm breit (0,34 mm)."],
    hatching: ["None.", "Keine."],
    styles: ["Body 0.6, collar 0.8, knob 0.95, pointer and index 0.1; body pen 0.18 mm, detail 0.13 mm, ground 0.35 mm.", "Körper 0,6, Kragen 0,8, Knopf 0,95, Zeiger und Index 0,1; Körperstift 0,18 mm, Detail 0,13 mm, Boden 0,35 mm."],
    overlays: ["Body, collar and knob are three regions: the body carries the supply water temperature all the time, the collar and the knob warm only while hot water flows through the open valve; the knob and its pointer turn with the knob bone.", "Körper, Kragen und Knopf sind drei Regionen: der Körper trägt dauernd die Vorlauftemperatur, Kragen und Knopf erwärmen sich nur, solange heißes Wasser durch das offene Ventil fließt; Knopf und Zeiger drehen sich mit dem Knopfknochen."],
  },
);

const rep100 = authored(
  100,
  [75, 150],
  { body: 0.18, detail: null, texture: null, glazing: null, ground: 0.25 },
  [
    poly("body", "body", "body", 3, true, 0.6, bodyOutline(0.012, 0.015), true, { region: "body", material: "brass" }),
    ellipse("head", "head", "body", 3.1, true, 0.8, 0, 0.04, 0.03, 0.03, { regions: ["head", "knob"], material: "abs-plastic" }),
  ],
  {
    silhouette: ["A 1.5 mm tall valve: a round head above a body with a pipe stub.", "Ein 1,5 mm hohes Ventil: ein runder Kopf über einem Körper mit Rohranschluss."],
    retained: ["Head disc, valve body, neck and pipe stub.", "Kopfscheibe, Ventilkörper, Hals und Rohranschluss."],
    merged: ["Collar and knob are one disc carrying both regions.", "Kragen und Knopf sind eine Scheibe, die beide Regionen trägt."],
    omitted: ["Knob, pointer and index mark: the turning of the knob is not drawn at this scale.", "Knopf, Zeiger und Indexmarke: das Drehen des Knopfs wird bei diesem Maßstab nicht gezeichnet."],
    thinElements: ["The pipe stub is widened to 24 mm (0.24 mm on paper) and the union nut to 30 mm.", "Der Rohranschluss ist auf 24 mm (0,24 mm auf Papier) und die Überwurfmutter auf 30 mm verbreitert."],
    hatching: ["None.", "Keine."],
    styles: ["Body 0.6, head 0.8; one body pen of 0.18 mm.", "Körper 0,6, Kopf 0,8; ein Körperstift von 0,18 mm."],
    overlays: ["The disc carries collar and knob regions; the body keeps its own.", "Die Scheibe trägt Kragen- und Knopfregion; der Körper behält seine eigene."],
  },
);

const unavail = (denominator) =>
  unavailable(
    denominator,
    [`At 1:${denominator} the 0.145 m valve is ${(145 / denominator).toFixed(2)} mm tall and its pipe stub ${(17 / denominator).toFixed(2)} mm wide, below every print profile; use the 1:100 drawing at this scale.`, `Bei 1:${denominator} ist das 0,145 m hohe Ventil ${(145 / denominator).toFixed(2).replace(".", ",")} mm hoch und sein Rohranschluss ${(17 / denominator).toFixed(2).replace(".", ",")} mm breit, unter jedem Druckprofil; bei diesem Maßstab dient die Zeichnung 1:100.`],
    ["Pipe stub of 17 mm width", "Rohranschluss von 17 mm Breite"],
    0.017,
    [{ kind: "scale", denominator: 100 }, { kind: "scale", denominator: 50 }],
  );

const regions = [
  region("body", ["Valve body and pipe stub", "Ventilkörper und Rohranschluss"], "surface", "operating", "brass", 0.0125, 0.12, "cylinder", [single("wall", "Brass wall 3 mm", "Messingwand 3 mm", "metal", "brass", 0.003, 0.00003, 2800)], 2800, { operating: { key: "supply", heatW: 3 } }),
  region("head", ["Thermostat head collar", "Thermostatkopf-Kragen"], "surface", "operating", "abs-plastic", 0.0075, 0.06, "cylinder", [single("shell", "ABS shell 2 mm", "ABS-Schale 2 mm", "polymer", "abs-plastic", 0.002, 0.012, 2000)], 2000, { operating: { key: "flow", heatW: 0.8 } }),
  region("knob", ["Setting knob", "Einstellknopf"], "surface", "operating", "abs-plastic", 0.0028, 0.04, "cylinder", [single("shell", "ABS knob 3 mm", "ABS-Knopf 3 mm", "polymer", "abs-plastic", 0.003, 0.018, 3000)], 3000, { operating: { key: "flow", heatW: 0.4 } }),
];

const bind = (supply, flow) => ({
  regions: {
    body: { exposedFraction: 0.9, sunlitFraction: 0, contact: { fraction: 0.1, tempC: 55 } },
    head: { exposedFraction: 1, sunlitFraction: 0 },
    knob: { exposedFraction: 1, sunlitFraction: 0 },
  },
  emitters: {},
  operating: { supply, flow },
});

const pipe = ["pipe-joint-upper", "pipe-joint-lower"];
const knob = (deg) => ({ knob: { angleDeg: deg } });
const poses = [
  pose("open", ["Fully open, setting 5", "Ganz offen, Stufe 5"], ["The knob stands at setting 5 (270 degrees): hot water flows through the valve and warms the head and the knob a little.", "Der Knopf steht auf Stufe 5 (270 Grad): heißes Wasser fließt durch das Ventil und erwärmt Kopf und Knopf ein wenig."], knob(270), pipe, "authored", bind(1, 1)),
  pose("part-open", ["Half open, setting 3", "Halb offen, Stufe 3"], ["The knob stands at setting 3 (162 degrees): about 60 percent of the water flows.", "Der Knopf steht auf Stufe 3 (162 Grad): etwa 60 Prozent des Wassers fließen."], knob(162), pipe, "authored", bind(1, 0.6)),
  pose("closed", ["Closed, setting star", "Geschlossen, Frostschutzstufe"], ["The knob stands at the star setting (0 degrees): no water flows, the head cools to the room temperature while the body stays at the supply temperature.", "Der Knopf steht auf der Frostschutzstufe (0 Grad): kein Wasser fließt, der Kopf kühlt auf Raumtemperatur ab, während der Körper auf Vorlauftemperatur bleibt."], knob(0), pipe, "authored", bind(1, 0)),
];

const contacts = (d) => pipe.map((anchor) => ({ anchor, fromS: 0, toS: d }));

const clips = [
  clip(
    "close-down",
    ["Closing the valve", "Ventil schließen"],
    ["The knob rests at setting 5 for one second, turns down to the star setting between 1 and 7 seconds and rests there; the flow through the valve falls with it and the head cools.", "Der Knopf ruht eine Sekunde auf Stufe 5, dreht zwischen 1 und 7 Sekunden auf die Frostschutzstufe und ruht dort; der Durchfluss fällt mit, und der Kopf kühlt ab."],
    8,
    15,
    [{ t: 0, pose: "open", ease: "smooth" }, { t: 1, pose: "open", ease: "smooth" }, { t: 7, pose: "closed", ease: "smooth" }, { t: 8, pose: "closed", ease: "smooth" }],
    contacts(8),
    [["open", 0, 1, ["Open", "Offen"]], ["closing", 1, 7, ["Closing", "Schließen"]], ["closed", 7, 8, ["Closed", "Geschlossen"]]],
    [[1, "motion", ["Knob starts to turn", "Knopf beginnt zu drehen"], "knob"]],
    [["operating:flow", "smooth", [[0, 1], [1, 1], [7, 0], [8, 0]]]],
  ),
  clip(
    "open-up",
    ["Opening the valve", "Ventil öffnen"],
    ["The valve is closed for one second, the knob turns up to setting 5 between 1 and 7 seconds and the flow and the head warmth rise with it.", "Das Ventil ist eine Sekunde geschlossen, der Knopf dreht zwischen 1 und 7 Sekunden auf Stufe 5, und Durchfluss und Kopfwärme steigen mit."],
    8,
    15,
    [{ t: 0, pose: "closed", ease: "smooth" }, { t: 1, pose: "closed", ease: "smooth" }, { t: 7, pose: "open", ease: "smooth" }, { t: 8, pose: "open", ease: "smooth" }],
    contacts(8),
    [["closed", 0, 1, ["Closed", "Geschlossen"]], ["opening", 1, 7, ["Opening", "Öffnen"]], ["open", 7, 8, ["Open", "Offen"]]],
    [[1, "motion", ["Knob starts to turn", "Knopf beginnt zu drehen"], "knob"]],
    [["operating:flow", "smooth", [[0, 0], [1, 0], [7, 1], [8, 1]]]],
  ),
];

export const trv = () => {
  const massKg = Math.round(sum(sp.bones.map((b) => b.mass.kg)) * 1000) / 1000;
  return wrap({
    id: "thermostatic-radiator-valve",
    category: "appliances",
    title: tx("Thermostatic Radiator Valve with Setting Knob", "Thermostat-Heizkörperventil mit Einstellknopf"),
    summary: tx(
      "A 60 mm thermostatic radiator valve seen along its axis: an angle valve body with a pipe stub, a thermostat head collar and a setting knob that turns from the star setting to setting 5, with the flow and the head warmth following the knob.",
      "Ein Thermostat-Heizkörperventil von 60 mm in Achsrichtung gesehen: ein Eckventilkörper mit Rohranschluss, ein Thermostatkopf-Kragen und ein Einstellknopf, der von der Frostschutzstufe auf Stufe 5 dreht, wobei Durchfluss und Kopfwärme dem Knopf folgen.",
    ),
    tags: ["valve", "thermostatic-valve", "radiator-valve", "hydronic", "hydraulic-balancing", "heating-control", "component", "hot-water", "dial"],
    provenance: {
      author: "Semio staffage content (G1)",
      method: "handcrafted",
      created: "2026-11-01",
      references: [
        ref("din-en-215-2004", "DIN EN 215:2004 Thermostatic radiator valves - Requirements and test methods (setting scale 0, star, 1 to 5; flow at the proportional band; head and valve body construction)."),
        ref("din-18380-2019", "DIN 18380:2019 VOB Part C: Heating systems and central water heating systems (hydraulic balancing with presettable radiator valves)."),
        INCROPERA,
      ],
      notes: tx(
        "A brass angle-pattern thermostatic radiator valve with a liquid-filled ABS head in the 30 mm connection standard, seen along the head axis as in a section across the wall behind the end of a radiator: the pipe comes up from below into the 17 mm stub, the hexagon neck carries the round head of 60 mm collar diameter and the setting knob of 40 mm. The origin is the centre of the pipe axis where it meets the valve body top. The knob turns from the star setting (0 degrees, frost protection) through settings 1 to 5 at 54 degrees each (270 degrees); positive angles turn counter-clockwise, which opens the valve. The body stays at the supply water temperature the whole time because the water stands in it even when the seat is closed (3 W of loss at 21 degrees Celsius room air), while the head and the knob warm only with the flow through the seat (0.8 W and 0.4 W at full flow, 60 percent at setting 3). All sizes, masses and loads are authored estimates for this product class.",
        "Ein Eckventil aus Messing als Thermostat-Heizkörperventil mit flüssigkeitsgefülltem ABS-Kopf im 30-mm-Anschlussstandard, in Richtung der Kopfachse gesehen wie in einem Schnitt quer zur Wand hinter dem Ende eines Heizkörpers: das Rohr kommt von unten in den 17 mm breiten Anschluss, der Sechskanthals trägt den runden Kopf mit 60 mm Kragendurchmesser und den Einstellknopf von 40 mm. Der Ursprung ist die Mitte der Rohrachse, wo sie auf die Oberkante des Ventilkörpers trifft. Der Knopf dreht von der Frostschutzstufe (0 Grad) über die Stufen 1 bis 5 zu je 54 Grad (270 Grad); positive Winkel drehen gegen den Uhrzeigersinn und öffnen das Ventil. Der Körper bleibt dauernd auf Vorlauftemperatur, weil das Wasser auch bei geschlossenem Sitz in ihm steht (3 W Verlust bei 21 Grad Celsius Raumluft), während Kopf und Knopf sich nur mit dem Durchfluss durch den Sitz erwärmen (0,8 W und 0,4 W bei vollem Durchfluss, 60 Prozent auf Stufe 3). Alle Maße, Massen und Lasten sind Schätzwerte für diese Produktklasse.",
      ),
    },
    dimensions: { heightM: 0.145, widthM: 0.06, depthM: 0.09, massKg },
    materials: [
      materialOf("brass", "Nickel-plated brass", "Vernickeltes Messing", 0.3, 0.5, 8500, 110, 380, SRC, "incropera-2018"),
      materialOf("abs-plastic", "White ABS plastic", "Weißer ABS-Kunststoff", 0.9, 0.3, 1050, 0.17, 1400, SRC, "incropera-2018"),
    ],
    rig: { kind: "mechanical", root: "body", bones: sp.bones.map((b) => ({ ...b, mass: { ...b.mass } })) },
    anchors: [
      supportAnchor("pipe-joint-upper", "body", 0, -0.048, ["Compression joint on the pipe, upper", "Klemmverschraubung am Rohr, oben"], "pin", { footprintLengthM: 0.017, footprintWidthM: 0.017 }),
      supportAnchor("pipe-joint-lower", "body", 0, -0.07, ["Compression joint on the pipe, lower", "Klemmverschraubung am Rohr, unten"], "roller", { footprintLengthM: 0.017, footprintWidthM: 0.017, frictionCoefficient: 0.4 }),
    ],
    representations: { 500: unavail(500), 200: unavail(200), 100: rep100, 50: rep50 },
    symbols: [],
    poses,
    clips,
    thermal: {
      environment: environment(),
      passengers: [],
      operating: [
        { id: "supply", title: tx("Supply water in the body", "Vorlaufwasser im Körper"), levels: [level(0, "Heating off, cold", "Heizung aus, kalt"), level(1, "Hot supply water", "Heißes Vorlaufwasser")] },
        { id: "flow", title: tx("Flow through the valve seat", "Durchfluss durch den Ventilsitz"), levels: [level(0, "Closed, star setting", "Geschlossen, Frostschutzstufe"), level(0.6, "Setting 3", "Stufe 3"), level(1, "Setting 5, fully open", "Stufe 5, ganz offen")] },
      ],
      regions,
      emitters: [],
      assumptions: [
        tx(
          "The 3 W of the body is the heat lost by the standing hot water to the room through the brass wall; it holds the body some 30 kelvin above the room at the supply temperature.",
          "Die 3 W des Körpers sind die Wärme, die das stehende heiße Wasser durch die Messingwand an den Raum verliert; sie hält den Körper bei Vorlauftemperatur rund 30 Kelvin über dem Raum.",
        ),
        tx(
          "The head and the knob receive their heat only by conduction from the valve seat while water flows, so they follow the flow level and not the supply level; the liquid-filled sensor is not modelled.",
          "Kopf und Knopf erhalten ihre Wärme nur durch Leitung vom Ventilsitz, solange Wasser fließt, folgen also dem Durchflussniveau und nicht dem Vorlauf; der flüssigkeitsgefüllte Fühler wird nicht nachgebildet.",
        ),
      ],
    },
    structure: {
      gravityMS2: 9.80665,
      dynamics: "quasi-static",
      supports: pipe,
      assumptions: [
        tx(
          "The 0.38 kg valve hangs on its pipe stub: the upper compression joint (pin) and the lower one (roller) carry the weight and the small overturning moment of the 90 mm deep head.",
          "Das 0,38 kg schwere Ventil hängt an seinem Rohranschluss: die obere Klemmverschraubung (Festlager) und die untere (Rollenlager) tragen das Gewicht und das kleine Kippmoment des 90 mm tiefen Kopfes.",
        ),
      ],
      clinical: false,
    },
    presentation: { defaultPose: "open", defaultClip: "close-down", defaultScale: 50, viewpoints: ["elevation"] },
    exports: ALL_EXPORTS,
    completion: "authored",
  });
};
