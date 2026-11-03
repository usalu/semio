import { ALL_EXPORTS, INCROPERA, authored, clip, layer, level, pose, ref, tx, unavailable, wrap } from "./lib.mjs";
import { materialOf, space, sum } from "./space.mjs";

const SRC = "authored-estimate";
const m = (kg, comX, comY) => ({ kg, comX, comY, distribution: "point", source: SRC });
const fixed = { type: "fixed" };
const r5 = (n) => Math.round(n * 1e5) / 1e5;

const P = [0.09, 0.62];
const sp = space([
  ["rail", null, 0, 0, { title: tx("Wall brackets and riser rail", "Wandhalter und Brausestange"), lengthM: 0.81, joint: fixed, mass: m(1.2, 0.05, 0.4) }],
  ["elbow", "rail", 0.03, 0.04, { title: tx("Wall elbow with shut-off valve and hose", "Wandanschlussbogen mit Absperrventil und Schlauch"), lengthM: 0.06, joint: fixed, mass: m(0.55, 0.02, -0.1) }],
  ["lever", "elbow", 0.03, 0.04, { title: tx("Valve lever", "Ventilhebel"), lengthM: 0.05, joint: { type: "revolute", minDeg: -90, maxDeg: 90 }, mass: m(0.02, 0.025, 0) }],
  ["holder", "rail", P[0], P[1], { title: tx("Sliding holder", "Gleithalter"), lengthM: 0.06, joint: { type: "prismatic", axisDeg: 90, minM: -0.45, maxM: 0 }, mass: m(0.12, 0, 0) }],
  ["shower", "holder", P[0], P[1], { title: tx("Hand shower with handle", "Handbrause mit Griff"), lengthM: 0.2, joint: { type: "revolute", minDeg: -40, maxDeg: 70 }, mass: m(0.38, 0.03, 0.08) }],
]);
const { rect, poly, path } = sp;

const tail = (o) => ({ ...(o.region ? { region: o.region } : {}), ...(o.regions ? { regions: o.regions } : {}), ...(o.material ? { material: o.material } : {}), ...(o.penMm ? { penMm: o.penMm } : {}) });
const cap = (id, bone, role, z, filled, tone, a, b, r0, r1, o = {}) => {
  const [x0, y0] = sp.local(bone, a[0], a[1]);
  const [x1, y1] = sp.local(bone, b[0], b[1]);
  return { id, bone, role, z, filled, tone, kind: "capsule", x0, y0, x1, y1, r0, r1, ...tail(o) };
};

const U = [Math.sin((25 * Math.PI) / 180), Math.cos((25 * Math.PI) / 180)];
const T = [Math.sin((33 * Math.PI) / 180), Math.cos((33 * Math.PI) / 180)];
const at = (p, v, s) => [r5(p[0] + v[0] * s), r5(p[1] + v[1] * s)];
const HB = at(P, U, -0.09);
const HN = at(P, U, 0.09);
const HC = at(HN, U, 0.05);
const H1 = at(HC, T, -0.055);
const H2 = at(HC, T, 0.055);
const NOZ = [r5(HC[0] + 0.839 * 0.0), 0];
const FACE = [r5(HC[0] + 0.839 * 0.0135), r5(HC[1] - 0.545 * 0.0135)];

const HOSE = `M 0.04 0.036 C 0.1 0.03 0.25 -0.1 0.23 -0.22 C 0.21 -0.34 0.1 -0.32 0.12 -0.18 C 0.14 -0.02 0.17 0.3 ${HB[0]} ${r5(HB[1] + 0.02)}`;

const rep50 = authored(
  50,
  [20, 75],
  { body: 0.18, detail: 0.13, texture: null, glazing: null, ground: 0.35 },
  [
    path("hose", "elbow", "body", 3, false, 0.55, HOSE, { region: "hose", material: "epdm-hose", penMm: 0.35 }),
    rect("bracket-low", "rail", "body", 3.1, true, 0.6, 0, 0.08, 0.05, 0.035, { region: "rail", material: "chrome" }),
    rect("bracket-top", "rail", "body", 3.1, true, 0.6, 0, 0.775, 0.05, 0.035, { region: "rail", material: "chrome" }),
    rect("tube", "rail", "body", 3.2, true, 0.8, 0.049, 0.1, 0.022, 0.69, { r: 0.008, region: "rail", material: "chrome" }),
    poly("elbow", "elbow", "body", 3.3, true, 0.7, [[0, 0], [0.06, 0], [0.06, 0.08], [0, 0.08]], true, { region: "valve", material: "chrome" }),
    cap("lever", "lever", "detail", 3.4, true, 0.85, [0.03, 0.04], [0.075, 0.04], 0.005, 0.005, { region: "valve", material: "chrome" }),
    rect("holder", "holder", "body", 3.3, true, 0.7, 0.062, 0.59, 0.056, 0.06, { r: 0.008, region: "rail", material: "chrome" }),
    cap("handle", "shower", "body", 3.5, true, 0.82, HB, HN, 0.0165, 0.0165, { region: "handle", material: "abs-chrome" }),
    cap("head", "shower", "body", 3.6, true, 0.9, H1, H2, 0.0135, 0.0135, { region: "head", material: "abs-chrome" }),
    poly("face", "shower", "detail", 3.7, false, 0.4, [at(H1, [0.839, -0.545], 0.0135), at(H2, [0.839, -0.545], 0.0135)], false, { region: "head", material: "rubber-nozzle" }),
  ],
  {
    silhouette: ["A 1.1 m by 0.22 m wall set, 22 mm by 4.4 mm on paper: a thin riser rail between two wall brackets with a sliding holder carrying a hand shower (a slim handle and a flat round head), a wall elbow with a small lever, and a hose loop sagging from the elbow to the handle.", "Eine Wandgarnitur von 1,1 m mal 0,22 m, auf Papier 22 mm mal 4,4 mm: eine dünne Brausestange zwischen zwei Wandhaltern mit einem Gleithalter, der eine Handbrause trägt (ein schlanker Griff und ein flacher runder Kopf), ein Wandanschlussbogen mit kleinem Hebel und ein Schlauchbogen, der vom Anschluss zum Griff durchhängt."],
    retained: ["Rail of 22 mm with both brackets, holder, handle of 33 mm, head of 0.11 m seen edge-on with its nozzle face line, elbow with lever, hose of 1.2 m as a loop.", "Stange von 22 mm mit beiden Haltern, Gleithalter, Griff von 33 mm, Kopf von 0,11 m in der Kante gesehen mit seiner Düsenfläche als Linie, Anschlussbogen mit Hebel, Schlauch von 1,2 m als Bogen."],
    merged: ["The holder clamp and the ball joint are one rectangle; the rubber nozzles are the face line.", "Halterklemme und Kugelgelenk sind ein Rechteck; die Gummidüsen sind die Linie der Düsenfläche."],
    omitted: ["The mode selector, the hose nuts, the soap dish and the water itself, which only the emitters show.", "Der Strahlartwähler, die Schlauchmuttern, die Seifenschale und das Wasser selbst, das nur die Emitter zeigen."],
    thinElements: ["The hose is drawn as a 0.35 mm line (a 12 mm hose is 0.24 mm); the lever is 10 mm (0.2 mm); the face line is a 0.13 mm detail line.", "Der Schlauch wird als 0,35-mm-Linie gezeichnet (ein 12-mm-Schlauch ist 0,24 mm); der Hebel ist 10 mm (0,2 mm) breit; die Düsenlinie ist eine 0,13-mm-Detaillinie."],
    hatching: ["None.", "Keine."],
    styles: ["Brackets 0.6, tube 0.8, elbow 0.7, holder 0.7, handle 0.82, head 0.9, hose 0.55; body pen 0.18 mm, detail 0.13 mm, ground 0.35 mm.", "Halter 0,6, Rohr 0,8, Anschluss 0,7, Gleithalter 0,7, Griff 0,82, Kopf 0,9, Schlauch 0,55; Körperstift 0,18 mm, Detail 0,13 mm, Boden 0,35 mm."],
    overlays: ["Rail, valve, hose, handle and head are five regions: in the thermal view the running water warms the head, the handle and the hose; the spray region is air-borne and has no shape (see its hidden note).", "Stange, Ventil, Schlauch, Griff und Kopf sind fünf Regionen: in der Thermalansicht erwärmt das laufende Wasser Kopf, Griff und Schlauch; die Sprühregion ist luftgetragen und hat keine Form (siehe ihren Hinweis)."],
  },
);

const rep100 = authored(
  100,
  [75, 150],
  { body: 0.18, detail: null, texture: null, glazing: null, ground: 0.25 },
  [
    path("hose", "elbow", "body", 3, false, 0.55, HOSE, { region: "hose", material: "epdm-hose", penMm: 0.3 }),
    rect("bracket-low", "rail", "body", 3.1, true, 0.6, 0, 0.075, 0.055, 0.045, { regions: ["rail"], material: "chrome" }),
    rect("bracket-top", "rail", "body", 3.1, true, 0.6, 0, 0.765, 0.055, 0.045, { region: "rail", material: "chrome" }),
    rect("tube", "rail", "body", 3.2, true, 0.8, 0.045, 0.1, 0.03, 0.69, { r: 0.01, region: "rail", material: "chrome" }),
    poly("elbow", "elbow", "body", 3.3, true, 0.7, [[0, 0], [0.065, 0], [0.065, 0.085], [0, 0.085]], true, { regions: ["valve"], material: "chrome" }),
    rect("holder", "holder", "body", 3.3, true, 0.7, 0.06, 0.585, 0.06, 0.07, { r: 0.008, region: "rail", material: "chrome" }),
    cap("handle", "shower", "body", 3.5, true, 0.82, HB, HN, 0.02, 0.02, { region: "handle", material: "abs-chrome" }),
    cap("head", "shower", "body", 3.6, true, 0.9, H1, H2, 0.017, 0.017, { region: "head", material: "abs-chrome" }),
  ],
  {
    silhouette: ["A 11 mm by 2.2 mm set: a thin rail with a holder carrying a slim handle with a flat head, a wall elbow and a hose loop.", "Eine Garnitur von 11 mm mal 2,2 mm: eine dünne Stange mit Halter, der einen schlanken Griff mit flachem Kopf trägt, ein Wandanschluss und ein Schlauchbogen."],
    retained: ["Rail with both brackets, holder, handle, head, elbow and hose loop.", "Stange mit beiden Haltern, Gleithalter, Griff, Kopf, Anschluss und Schlauchbogen."],
    merged: ["The valve lever joins the elbow block; the nozzle line joins the head.", "Der Ventilhebel geht im Anschlussblock auf; die Düsenlinie im Kopf."],
    omitted: ["Lever, nozzle line, mode selector.", "Hebel, Düsenlinie, Strahlartwähler."],
    thinElements: ["The tube is thickened from 22 to 30 mm (0.3 mm), the handle from 33 to 40 mm and the head from 27 to 34 mm; the hose is a 0.3 mm line.", "Das Rohr ist von 22 auf 30 mm (0,3 mm), der Griff von 33 auf 40 mm und der Kopf von 27 auf 34 mm verdickt; der Schlauch ist eine 0,3-mm-Linie."],
    hatching: ["None.", "Keine."],
    styles: ["Brackets 0.6, tube 0.8, elbow 0.7, holder 0.7, handle 0.82, head 0.9, hose 0.55; one body pen of 0.18 mm.", "Halter 0,6, Rohr 0,8, Anschluss 0,7, Gleithalter 0,7, Griff 0,82, Kopf 0,9, Schlauch 0,55; ein Körperstift von 0,18 mm."],
    overlays: ["The elbow block lists the valve as merged; the other regions keep their shape.", "Der Anschlussblock führt das Ventil als zusammengeführt; die anderen Regionen behalten ihre Form."],
  },
);

const rep200 = authored(
  200,
  [150, 350],
  { body: 0.13, detail: null, texture: null, glazing: null, ground: 0.25 },
  [
    path("hose", "elbow", "body", 3, false, 0.55, HOSE, { region: "hose", material: "epdm-hose", penMm: 0.25 }),
    rect("tube", "rail", "body", 3.2, true, 0.8, 0.04, 0.0, 0.04, 0.81, { regions: ["rail", "valve"], material: "chrome" }),
    cap("handle", "shower", "body", 3.5, true, 0.82, HB, HN, 0.026, 0.026, { region: "handle", material: "abs-chrome" }),
    cap("head", "shower", "body", 3.6, true, 0.9, H1, H2, 0.024, 0.024, { region: "head", material: "abs-chrome" }),
  ],
  {
    silhouette: ["A 5.5 mm by 1.1 mm set: a pole with a short handle and a head at the top and a hose line.", "Eine Garnitur von 5,5 mm mal 1,1 mm: eine Stange mit kurzem Griff und einem Kopf oben und eine Schlauchlinie."],
    retained: ["Rail pole, handle, head and hose loop.", "Stange, Griff, Kopf und Schlauchbogen."],
    merged: ["Brackets, holder, elbow and lever join the pole (regions rail and valve merged).", "Halter, Gleithalter, Anschluss und Hebel gehen in der Stange auf (Regionen Stange und Ventil zusammengeführt)."],
    omitted: ["All fittings and the nozzle line.", "Alle Beschläge und die Düsenlinie."],
    thinElements: ["The pole is drawn 40 mm wide (0.2 mm) instead of 22 mm, handle 52 mm and head 48 mm; the hose is a 0.25 mm line.", "Die Stange ist 40 mm (0,2 mm) statt 22 mm breit gezeichnet, der Griff 52 mm und der Kopf 48 mm; der Schlauch ist eine 0,25-mm-Linie."],
    hatching: ["None.", "Keine."],
    styles: ["Pole 0.8, handle 0.82, head 0.9, hose 0.55; one body pen of 0.13 mm.", "Stange 0,8, Griff 0,82, Kopf 0,9, Schlauch 0,55; ein Körperstift von 0,13 mm."],
    overlays: ["The pole lists rail and valve as merged regions.", "Die Stange führt Stange und Ventil als zusammengeführte Regionen."],
  },
);

const rep500 = unavailable(
  500,
  ["At 1:500 the rail is 0.04 mm wide and the head 0.2 mm long, below every print profile.", "Bei 1:500 ist die Stange 0,04 mm breit und der Kopf 0,2 mm lang, unter jedem Druckprofil."],
  ["Rail of 22 mm and head of 27 mm thickness", "Stange von 22 mm und Kopf von 27 mm Dicke"],
  0.022,
  [{ kind: "scale", denominator: 200 }, { kind: "scale", denominator: 100 }],
);

const single = (id, en, de, kind, material, thicknessM, res, cap_, reference = "incropera-2018") => layer(id, [en, de], kind, material, thicknessM, res, cap_, SRC, reference);
const region = (id, title, kind, model, material, areaM2, characteristicLengthM, geometry, layers, heatCapacityJM2K, extra = {}, reference = "incropera-2018") => ({
  id,
  title: tx(...title),
  kind,
  model,
  material,
  areaM2,
  characteristicLengthM,
  geometry,
  skyViewFactor: 0.3,
  layers,
  heatCapacityJM2K,
  ...extra,
  source: SRC,
  reference,
});

const regions = [
  region("rail", ["Rail, brackets and holder", "Stange, Halter und Gleithalter"], "surface", "passive", "chrome", 0.06, 0.7, "vertical-plate", [single("tube", "Chrome plated brass tube 1.2 mm", "Verchromtes Messingrohr 1,2 mm", "metal", "chrome", 0.0012, 0.00001, 3400)], 3400),
  region("valve", ["Wall elbow with valve", "Wandanschlussbogen mit Ventil"], "surface", "passive", "chrome", 0.012, 0.06, "cylinder", [single("body", "Brass body 3 mm", "Messingkörper 3 mm", "metal", "chrome", 0.003, 0.00003, 8000)], 8000),
  region("hose", ["Shower hose", "Brauseschlauch"], "surface", "passive", "epdm-hose", 0.047, 0.012, "cylinder", [single("rubber", "EPDM hose wall 2 mm over a steel braid", "EPDM-Schlauchwand 2 mm über Stahlgeflecht", "polymer", "epdm-hose", 0.002, 0.012, 2400)], 2400),
  region("handle", ["Handle", "Griff"], "surface", "passive", "abs-chrome", 0.012, 0.18, "cylinder", [single("shell", "Chrome plated ABS 2 mm", "Verchromtes ABS 2 mm", "polymer", "abs-chrome", 0.002, 0.012, 2500)], 2500),
  region("head", ["Spray head", "Brausekopf"], "surface", "passive", "abs-chrome", 0.0095, 0.11, "vertical-plate", [single("shell", "Chrome plated ABS 2 mm with silicone nozzles", "Verchromtes ABS 2 mm mit Silikondüsen", "polymer", "abs-chrome", 0.002, 0.012, 2500)], 2500),
  {
    ...region("spray", ["Warm spray", "Warmer Strahl"], "water", "operating", "water", 0.5, 0.11, "cylinder", [single("drops", "Water drops of 1 mm in flight", "Wassertropfen von 1 mm im Flug", "polymer", "water", 0.001, 0.0002, 4180)], 4180, { operating: { key: "water", heatW: 300 } }),
    hidden: tx("The spray is air-borne water without a fixed outline: it is not drawn in any elevation but carried by the emitters of the transport view and by the warmed head.", "Der Strahl ist luftgetragenes Wasser ohne feste Kontur: er wird in keiner Ansicht gezeichnet, sondern von den Emittern der Transportansicht und vom erwärmten Kopf getragen."),
  },
];

const warm = (fraction) => ({ fraction, tempC: 38 });
const bind = (water, contactScale, wet, lever) => ({
  regions: {
    rail: { exposedFraction: 1, sunlitFraction: 0, contact: { fraction: 0.05, tempC: 21 } },
    valve: { exposedFraction: 1, sunlitFraction: 0, ...(water ? { contact: warm(0.4 * contactScale) } : {}) },
    hose: { exposedFraction: 1, sunlitFraction: 0, ...(water ? { contact: warm(0.3 * contactScale) } : {}) },
    handle: { exposedFraction: 1, sunlitFraction: 0, ...(water ? { contact: warm(0.4 * contactScale) } : {}) },
    head: { exposedFraction: 1, sunlitFraction: 0, ...(water ? { contact: warm(0.7 * contactScale), wettedness: wet } : {}) },
    spray: { exposedFraction: 1, sunlitFraction: 0, wettedness: water ? 1 : 0 },
  },
  emitters: { "spray-mist": water, "steam-plume": water },
  operating: { water },
});

const wall = ["wall-low", "wall-top"];
const poses = [
  pose("off", ["Off", "Aus"], ["The valve lever points down, the hand shower hangs in its holder and the hose is dry and cold.", "Der Ventilhebel zeigt nach unten, die Handbrause hängt im Halter und der Schlauch ist trocken und kalt."], { lever: { angleDeg: -80 } }, wall, "authored", bind(0, 0, 0)),
  pose("running", ["Running at 12 L/min", "Läuft mit 12 L/min"], ["The lever stands up and 12 litres per minute of 38 degC water leave the head as a warm spray; the shower is tilted 8 degrees downward.", "Der Hebel steht oben und 12 Liter je Minute Wasser von 38 degC verlassen den Kopf als warmer Strahl; die Brause ist um 8 Grad nach unten geneigt."], { lever: { angleDeg: 60 }, shower: { angleDeg: -8 } }, wall, "authored", bind(1, 1, 1)),
  pose("eco", ["Eco at 9 L/min", "Sparbetrieb mit 9 L/min"], ["The lever stands halfway and the water-saving mode gives 9 litres per minute at 38 degC with an aerated spray; the shower is tilted 4 degrees downward.", "Der Hebel steht auf halber Höhe und der Sparmodus liefert 9 Liter je Minute bei 38 degC mit belüftetem Strahl; die Brause ist um 4 Grad nach unten geneigt."], { lever: { angleDeg: 5 }, shower: { angleDeg: -4 } }, wall, "authored", bind(0.75, 1, 0.8)),
];

const contacts = (d) => wall.map((anchor) => ({ anchor, fromS: 0, toS: d }));

const clips = [
  clip(
    "turn-on-and-run",
    ["Turn on and run", "Aufdrehen und laufen lassen"],
    ["The lever is turned up after five seconds: the spray starts at once and the head, handle and hose warm up during the next five minutes.", "Der Hebel wird nach fünf Sekunden aufgedreht: der Strahl setzt sofort ein, und Kopf, Griff und Schlauch erwärmen sich in den nächsten fünf Minuten."],
    300,
    1,
    [{ t: 0, pose: "off", ease: "step" }, { t: 5, pose: "running", ease: "step" }, { t: 300, pose: "running", ease: "step" }],
    contacts(300),
    [["off", 0, 5, ["Off", "Aus"]], ["run", 5, 300, ["Running, warming up", "Läuft, erwärmt sich"]]],
    [[5, "operating", ["Valve opened", "Ventil geöffnet"], "water"]],
    [["operating:water", "step", [[0, 0], [5, 1]]], ["emitter:spray-mist", "step", [[0, 0], [5, 1]]], ["emitter:steam-plume", "step", [[0, 0], [5, 1]]]],
  ),
  clip(
    "eco-and-off",
    ["Eco mode and off", "Sparmodus und Aus"],
    ["The shower runs at full flow for one minute, goes to the eco mode for three and is turned off.", "Die Dusche läuft eine Minute mit voller Menge, geht für drei Minuten in den Sparmodus und wird abgedreht."],
    420,
    1,
    [{ t: 0, pose: "running", ease: "step" }, { t: 60, pose: "eco", ease: "step" }, { t: 240, pose: "off", ease: "step" }, { t: 420, pose: "off", ease: "step" }],
    contacts(420),
    [["full", 0, 60, ["Full flow", "Volle Menge"]], ["eco", 60, 240, ["Eco mode", "Sparmodus"]], ["cool-down", 240, 420, ["Off, cooling", "Aus, kühlt ab"]]],
    [[240, "shutdown", ["Valve closed", "Ventil geschlossen"], "water"]],
    [["operating:water", "step", [[0, 1], [60, 0.75], [240, 0]]], ["emitter:spray-mist", "step", [[0, 1], [60, 0.75], [240, 0]]], ["emitter:steam-plume", "step", [[0, 1], [60, 0.75], [240, 0]]]],
  ),
];

export const shower = () => {
  const massKg = sum(sp.bones.map((b) => b.mass.kg));
  return wrap({
    id: "shower-head",
    category: "appliances",
    title: tx("Wall-Mounted Hand Shower on Riser Rail", "Handbrause an Brausestange"),
    summary: tx(
      "A wall-mounted hand shower of 0.11 m head diameter on a 0.7 m riser rail with a hose loop to the wall elbow; running it delivers 12 L/min (eco 9 L/min) of 38 degC water as a warm spray and a vapour plume.",
      "Eine wandmontierte Handbrause mit 0,11 m Kopfdurchmesser an einer 0,7 m langen Brausestange mit Schlauchbogen zum Wandanschluss; im Betrieb liefert sie 12 L/min (Sparmodus 9 L/min) Wasser von 38 degC als warmen Strahl und Dampffahne.",
    ),
    tags: ["shower", "hand-shower", "riser-rail", "bathroom", "hot-water", "spray", "vapour", "moisture", "internal-gain", "appliance"],
    provenance: {
      author: "Semio staffage content (A10)",
      method: "handcrafted",
      created: "2026-11-01",
      references: [
        ref("en-1112-2008", "EN 1112:2008 Sanitary tapware - Shower outlets for sanitary tapware for water supply systems of type 1 and type 2 (flow classes, flow rates of 8 to 12 litres per minute at 3 bar)."),
        ref("din-1988-300-2012", "DIN 1988-300:2012 Codes of practice for drinking water installations - Part 300: Pipe sizing (design flow of a shower of 0.15 litres per second at about 38 degC)."),
        ref("ashrae-2021", "ASHRAE (2021). Handbook - Fundamentals, Chapter 18 Nonresidential cooling and heating load calculations and Chapter 10 Indoor environmental modelling (latent and sensible gains of a shower, moisture production of a 10 minute shower)."),
        INCROPERA,
      ],
      notes: tx(
        "A hand shower of the common 110 mm head diameter in side elevation on a 0.7 m riser rail (wall brackets 0.1 m from the ends, rail centre 60 mm from the wall) with a sliding holder, a wall elbow with a shut-off valve and a 1.2 m hose hanging as a loop. The wall lies at x = 0; the origin is the lower edge of the wall elbow on the wall. The handle is 0.18 m long, the head a flat disc 0.11 m across and 27 mm thick seen edge-on, tilted so that the spray goes 33 degrees below the horizontal; the poses tilt it by 8 and 4 degrees. Dimensions and the mass of 2.3 kg are rounded from this product class (authored estimate). The valve lever stands for the flow: down = off, halfway = eco 9 L/min (an aerated, water-saving spray), up = 12 L/min. The water temperature is 38 degC (DIN 1988-300). The spray is air-borne water without outline and carries 300 W of sensible heat to the room air at level 1 (12 L/min x 4.18 kJ/kgK x 1.5 K of cooling in flight); the latent part is the vapour of the spray-mist emitter: 0.036 kg/s of air with 0.007 kg/kg of excess vapour = 0.25 g/s = 600 W latent, about 0.15 kg of vapour in a 10 minute shower (ASHRAE).",
        "Eine Handbrause mit üblichem Kopfdurchmesser von 110 mm in der Seitenansicht an einer 0,7 m langen Brausestange (Wandhalter 0,1 m von den Enden, Stangenmitte 60 mm von der Wand) mit Gleithalter, einem Wandanschlussbogen mit Absperrventil und einem 1,2 m langen Schlauch, der als Bogen hängt. Die Wand liegt bei x = 0; der Ursprung ist die Unterkante des Wandanschlussbogens an der Wand. Der Griff ist 0,18 m lang, der Kopf eine flache Scheibe von 0,11 m Durchmesser und 27 mm Dicke, in der Kante gesehen und so geneigt, dass der Strahl 33 Grad unter die Waagerechte geht; die Posen neigen ihn um 8 und 4 Grad. Maße und die Masse von 2,3 kg sind aus dieser Produktklasse gerundet (Schätzwert). Der Ventilhebel steht für die Menge: unten = aus, halbhoch = Sparmodus 9 L/min (ein belüfteter, wassersparender Strahl), oben = 12 L/min. Die Wassertemperatur beträgt 38 degC (DIN 1988-300). Der Strahl ist luftgetragenes Wasser ohne Kontur und gibt auf Stufe 1 300 W fühlbare Wärme an die Raumluft ab (12 L/min x 4,18 kJ/kgK x 1,5 K Abkühlung im Flug); der latente Anteil ist der Dampf des Emitters spray-mist: 0,036 kg/s Luft mit 0,007 kg/kg Dampfüberschuss = 0,25 g/s = 600 W latent, etwa 0,15 kg Dampf in einer Dusche von 10 Minuten (ASHRAE).",
      ),
    },
    dimensions: { heightM: 1.12, widthM: 0.23, depthM: 0.2, massKg },
    materials: [
      materialOf("chrome", "Chrome plated brass", "Verchromtes Messing", 0.1, 0.4, 8500, 110, 380, SRC, "incropera-2018"),
      materialOf("abs-chrome", "Chrome plated ABS", "Verchromtes ABS", 0.2, 0.4, 1050, 0.17, 1400, SRC, "incropera-2018"),
      materialOf("epdm-hose", "EPDM hose with steel braid", "EPDM-Schlauch mit Stahlgeflecht", 0.9, 0.5, 1100, 0.25, 2000, SRC, "incropera-2018"),
      materialOf("rubber-nozzle", "Silicone nozzles", "Silikondüsen", 0.9, 0.5, 1100, 0.2, 1500, SRC),
      materialOf("water", "Water at 38 degC", "Wasser von 38 degC", 0.96, 0.05, 993, 0.63, 4180, "literature", "incropera-2018"),
    ],
    rig: { kind: "mechanical", root: "rail", bones: sp.bones.map((b) => ({ ...b, mass: { ...b.mass } })) },
    anchors: [
      { id: "wall-low", bone: "rail", x: 0, y: 0.095, kind: "support", title: tx("Lower bracket on the wall", "Unterer Halter an der Wand"), support: "pin", normalDeg: 0, footprintLengthM: 0.035, footprintWidthM: 0.05 },
      { id: "wall-top", bone: "rail", x: 0, y: 0.79, kind: "support", title: tx("Upper bracket on the wall", "Oberer Halter an der Wand"), support: "roller", normalDeg: 0, footprintLengthM: 0.035, footprintWidthM: 0.05, frictionCoefficient: 0.4 },
      { id: "nozzle", bone: "shower", x: sp.local("shower", FACE[0], FACE[1])[0], y: sp.local("shower", FACE[0], FACE[1])[1], kind: "emitter", title: tx("Centre of the nozzle face", "Mitte der Düsenfläche") },
      { id: "steam-top", bone: "shower", x: sp.local("shower", HC[0] + 0.04, HC[1] - 0.1)[0], y: sp.local("shower", HC[0] + 0.04, HC[1] - 0.1)[1], kind: "emitter", title: tx("Rising vapour in front of the head", "Aufsteigender Dampf vor dem Kopf") },
    ],
    representations: { 500: rep500, 200: rep200, 100: rep100, 50: rep50 },
    symbols: [],
    poses,
    clips,
    thermal: {
      environment: { airTempC: 24, meanRadiantTempC: 24, relativeHumidity: 0.6, windMS: 0.05, windDirectionDeg: 0, solarWM2: 0, diffuseFraction: 0, pressurePa: 101325 },
      passengers: [],
      operating: [{ id: "water", title: tx("Hot water", "Warmwasser"), levels: [level(0, "Off", "Aus"), level(0.75, "Eco (9 L/min at 38 degC)", "Sparmodus (9 L/min bei 38 degC)"), level(1, "Running (12 L/min at 38 degC)", "Läuft (12 L/min bei 38 degC)")] }],
      regions,
      emitters: [
        { id: "spray-mist", kind: "vent", title: tx("Warm moist air carried by the spray", "Warme feuchte Luft im Strahl"), anchor: "nozzle", directionDeg: -33, diameterM: 0.11, operatingKey: "water", volumeFlowM3S: 0.03, temperatureRiseK: 3, vapourKgPerKg: 0.007, source: SRC, reference: "ashrae-2021" },
        { id: "steam-plume", kind: "plume", title: tx("Vapour plume rising from the warm spray", "Dampffahne über dem warmen Strahl"), anchor: "steam-top", directionDeg: 90, diameterM: 0.3, region: "spray", source: SRC, reference: "ashrae-2021" },
      ],
      assumptions: [
        tx(
          "The 12 L/min of 38 degC water cool by about 1.5 K in flight and give 300 W of sensible heat to the room air; most of the heat leaves with the drain water and is not modelled. The 0.25 g/s of evaporating water add 600 W of latent heat, carried by the vapour excess of the spray-mist emitter.",
          "Die 12 L/min Wasser von 38 degC kühlen im Flug um etwa 1,5 K ab und geben 300 W fühlbare Wärme an die Raumluft ab; der größte Teil der Wärme geht mit dem Abwasser und wird nicht modelliert. Die 0,25 g/s verdunstendes Wasser fügen 600 W latente Wärme hinzu, getragen vom Dampfüberschuss des Emitters spray-mist.",
        ),
        tx(
          "The running water warms the head, handle, hose and valve through a conductive contact to 38 degC with the fractions of the pose; the eco mode runs at 75 percent of the flow (9 of 12 L/min) and with a 20 percent lower contact of the head.",
          "Das laufende Wasser erwärmt Kopf, Griff, Schlauch und Ventil über einen Leitungskontakt auf 38 degC mit den Anteilen der Pose; der Sparmodus läuft mit 75 Prozent der Menge (9 von 12 L/min) und mit einem um 20 Prozent geringeren Kontakt am Kopf.",
        ),
      ],
    },
    structure: {
      gravityMS2: 9.80665,
      dynamics: "quasi-static",
      supports: wall,
      assumptions: [
        tx(
          "The 2.3 kg set hangs on the lower bracket (pin) and is held at the upper bracket (roller): the pair resists the overturning moment of the hand shower, 0.15 m from the wall.",
          "Die 2,3 kg schwere Garnitur hängt am unteren Halter (Festlager) und wird am oberen Halter gehalten (Rollenlager): das Paar nimmt das Kippmoment der Handbrause 0,15 m vor der Wand auf.",
        ),
      ],
      clinical: false,
    },
    presentation: { defaultPose: "running", defaultClip: "turn-on-and-run", defaultScale: 50, viewpoints: ["elevation"] },
    exports: ALL_EXPORTS,
    completion: "authored",
  });
};
