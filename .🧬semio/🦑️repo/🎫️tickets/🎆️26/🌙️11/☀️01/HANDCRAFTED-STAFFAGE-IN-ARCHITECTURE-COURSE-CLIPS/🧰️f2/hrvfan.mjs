import { ALL_EXPORTS, INCROPERA, authored, clip, layer, level, pose, ref, tx, unavailable, wrap } from "../🧰️a10/lib.mjs";
import { materialOf, space, sum } from "../🧰️a10/space.mjs";

const SRC = "authored-estimate";
const m = (kg, comX, comY) => ({ kg, comX, comY, distribution: "point", source: SRC });
const fixed = { type: "fixed" };

const sp = space([
  ["plate", null, 0, 0, { title: tx("Inner cover plate", "Innenblende"), lengthM: 0.26, joint: fixed, mass: m(0.45, 0, 0) }],
  ["ring", "plate", 0, 0, { title: tx("Wall sleeve with fan housing", "Wandhülse mit Ventilatorgehäuse"), lengthM: 0.2, joint: fixed, mass: m(0.5, 0, 0) }],
  ["core", "ring", 0, 0, { title: tx("Ceramic regenerator core", "Keramik-Regeneratorkern"), lengthM: 0.16, joint: fixed, mass: m(1.6, 0, 0) }],
  ["rotor", "ring", 0, 0, { title: tx("Reversible rotor with blades", "Umkehrbares Laufrad mit Flügeln"), lengthM: 0.08, joint: { type: "revolute" }, mass: m(0.1, 0, 0) }],
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
  const pts = [...edge(1), ...edge(-1).reverse()];
  return `M ${pts.map(([x, y]) => `${x} ${y}`).join(" L ")} Z`;
};
const blades = (n, rootR, tipR, rootW, tipW, sweep, tone) =>
  Array.from({ length: n }, (_, k) => path(`blade-${k + 1}`, "rotor", "body", 3.3, true, tone, bladePath(k, n, rootR, tipR, rootW, tipW, sweep), { region: "blades", material: "abs-plastic" }));

const cells = (a, limit) => {
  const dx = Math.sqrt(3) * a;
  const parts = [];
  for (let row = -6; row <= 6; row += 1) {
    for (let col = -6; col <= 6; col += 1) {
      const cx = col * dx + (row % 2 ? dx / 2 : 0);
      const cy = row * 1.5 * a;
      if (Math.hypot(cx, cy) + a > limit) continue;
      const pts = Array.from({ length: 6 }, (_, i) => {
        const t = (Math.PI / 180) * (60 * i + 30);
        return `${r5(cx + a * Math.cos(t))} ${r5(cy + a * Math.sin(t))}`;
      });
      parts.push(`M ${pts.join(" L ")} Z`);
    }
  }
  return parts.join(" ");
};

const SCREWS = [[-0.108, -0.108], [0.108, -0.108], [-0.108, 0.108], [0.108, 0.108]];
const screws = SCREWS.map(([x, y], i) => ellipse(`screw-${i + 1}`, "plate", "detail", 3.02, true, 0.45, x, y, 0.007, 0.007, { region: "casing", material: "abs-plastic" }));

const rep50 = authored(
  50,
  [20, 75],
  { body: 0.18, detail: 0.13, texture: null, glazing: null, ground: 0.35 },
  [
    rect("plate", "plate", "body", 3, true, 0.88, -0.13, -0.13, 0.26, 0.26, { r: 0.02, region: "casing", material: "abs-plastic" }),
    ...screws,
    ellipse("ring", "ring", "body", 3.05, true, 0.7, 0, 0, 0.098, 0.098, { region: "casing", material: "abs-plastic" }),
    ellipse("opening", "ring", "body", 3.1, true, 0.12, 0, 0, 0.088, 0.088, {}),
    ellipse("core-disc", "core", "body", 3.2, true, 0.62, 0, 0, 0.08, 0.08, { region: "core", material: "ceramic-honeycomb" }),
    path("core-cells", "core", "detail", 3.25, false, 0.3, cells(0.0125, 0.08), { region: "core", material: "ceramic-honeycomb" }),
    ...blades(5, 0.026, 0.078, 0.022, 0.05, 28, 0.82),
    ellipse("hub", "rotor", "body", 3.4, true, 0.55, 0, 0, 0.028, 0.028, { region: "hub", material: "motor-housing" }),
    ellipse("hub-cap", "rotor", "detail", 3.5, true, 0.9, 0.003, 0.003, 0.01, 0.01, { region: "hub", material: "motor-housing" }),
  ],
  {
    silhouette: ["A 0.26 m square inner cover, 5.2 mm on paper, with the inner cover taken off: a round sleeve in which a honeycomb disc of ceramic shows between five swept fan blades.", "Eine quadratische Innenblende von 0,26 m, auf Papier 5,2 mm, bei abgenommener Blende: eine runde Hülse, in der zwischen fünf gepfeilten Flügeln eine Keramik-Wabenscheibe sichtbar ist."],
    retained: ["Plate of 0.26 m with its rounded corners, sleeve ring of 0.196 m, dark opening of 0.176 m, ceramic disc of 0.16 m with 37 hexagonal cells of 25 mm, five blades and the 56 mm hub with cap, four screws.", "Platte von 0,26 m mit gerundeten Ecken, Hülsenring von 0,196 m, dunkle Öffnung von 0,176 m, Keramikscheibe von 0,16 m mit 37 sechseckigen Zellen von 25 mm, fünf Flügel und die 56 mm große Nabe mit Kappe, vier Schrauben."],
    merged: ["The 0.1 m depth of the core is seen end-on: only its face with the cell pattern is drawn; the real cells are 2 to 3 mm and are shown as 25 mm hexagons.", "Die 0,1 m Tiefe des Kerns wird von vorn gesehen: nur seine Stirnfläche mit dem Zellmuster ist gezeichnet; die echten Zellen sind 2 bis 3 mm groß und werden als Sechsecke von 25 mm gezeigt."],
    omitted: ["The weather hood outside, the foam seal around the sleeve and the cable.", "Die Wetterschutzhaube außen, die Schaumdichtung um die Hülse und das Kabel."],
    thinElements: ["The cell walls are drawn with the 0.13 mm detail pen; the blades are 22 mm wide at the root (0.44 mm) and 50 mm at the tip.", "Die Zellwände werden mit dem 0,13-mm-Detailstift gezeichnet; die Flügel sind an der Wurzel 22 mm (0,44 mm) und an der Spitze 50 mm breit."],
    hatching: ["None; the honeycomb is a cell outline pattern on a light terracotta tone.", "Keine; die Wabe ist ein Zellmuster aus Konturen auf einem hellen Terrakotta-Ton."],
    styles: ["Plate 0.88, ring 0.7, opening 0.12, ceramic 0.62, blades 0.82, hub 0.55; body pen 0.18 mm, detail 0.13 mm, ground 0.35 mm.", "Platte 0,88, Ring 0,7, Öffnung 0,12, Keramik 0,62, Flügel 0,82, Nabe 0,55; Körperstift 0,18 mm, Detail 0,13 mm, Boden 0,35 mm."],
    overlays: ["Casing, core, blades and hub are four regions: in the thermal view the core charges and discharges with the air direction while the blades turn forward and back with the rotor bone.", "Gehäuse, Kern, Flügel und Nabe sind vier Regionen: in der Thermalansicht lädt und entlädt sich der Kern mit der Luftrichtung, während sich die Flügel mit dem Rotorknochen vor und zurück drehen."],
  },
);

const rep100 = authored(
  100,
  [75, 150],
  { body: 0.18, detail: 0.13, texture: null, glazing: null, ground: 0.25 },
  [
    rect("plate", "plate", "body", 3, true, 0.88, -0.13, -0.13, 0.26, 0.26, { r: 0.02, region: "casing", material: "abs-plastic" }),
    ellipse("ring", "ring", "body", 3.05, true, 0.7, 0, 0, 0.098, 0.098, { region: "casing", material: "abs-plastic" }),
    ellipse("opening", "ring", "body", 3.1, true, 0.12, 0, 0, 0.086, 0.086, {}),
    ellipse("core-disc", "core", "body", 3.2, true, 0.62, 0, 0, 0.08, 0.08, { region: "core", material: "ceramic-honeycomb" }),
    path("core-cells", "core", "detail", 3.25, false, 0.3, cells(0.022, 0.08), { region: "core", material: "ceramic-honeycomb" }),
    ...blades(5, 0.026, 0.078, 0.03, 0.058, 28, 0.82),
    ellipse("hub", "rotor", "body", 3.4, true, 0.55, 0, 0, 0.03, 0.03, { region: "hub", material: "motor-housing" }),
  ],
  {
    silhouette: ["A 2.6 mm square with a round sleeve, a light disc with a coarse cell pattern and a five-bladed pinwheel.", "Ein Quadrat von 2,6 mm mit einer runden Hülse, einer hellen Scheibe mit grobem Zellmuster und einem fünfflügeligen Windrad."],
    retained: ["Plate, ring, opening, ceramic disc with 19 cells of 44 mm, five blades and the hub.", "Platte, Ring, Öffnung, Keramikscheibe mit 19 Zellen von 44 mm, fünf Flügel und die Nabe."],
    merged: ["The cells are merged to 19 coarse hexagons; screws and hub cap are dropped.", "Die Zellen sind zu 19 groben Sechsecken zusammengefasst; Schrauben und Nabenkappe entfallen."],
    omitted: ["Four screws, the hub cap and the fine cell structure.", "Vier Schrauben, die Nabenkappe und die feine Zellstruktur."],
    thinElements: ["The blades are widened to 30 mm at the root (0.3 mm) and 58 mm at the tip; the cell outlines stay 0.13 mm.", "Die Flügel sind an der Wurzel auf 30 mm (0,3 mm) und an der Spitze auf 58 mm verbreitert; die Zellkonturen bleiben 0,13 mm."],
    hatching: ["None.", "Keine."],
    styles: ["Plate 0.88, ring 0.7, opening 0.12, ceramic 0.62, blades 0.82, hub 0.55; body pen 0.18 mm, detail 0.13 mm.", "Platte 0,88, Ring 0,7, Öffnung 0,12, Keramik 0,62, Flügel 0,82, Nabe 0,55; Körperstift 0,18 mm, Detail 0,13 mm."],
    overlays: ["Casing, core, blades and hub stay four regions.", "Gehäuse, Kern, Flügel und Nabe bleiben vier Regionen."],
  },
);

const rep200 = authored(
  200,
  [150, 350],
  { body: 0.13, detail: null, texture: null, glazing: null, ground: 0.25 },
  [
    rect("plate", "plate", "body", 3, true, 0.88, -0.13, -0.13, 0.26, 0.26, { r: 0.02, region: "casing", material: "abs-plastic" }),
    ellipse("opening", "ring", "body", 3.1, true, 0.12, 0, 0, 0.094, 0.094, { region: "casing", material: "abs-plastic" }),
    ellipse("core-disc", "core", "body", 3.2, true, 0.62, 0, 0, 0.082, 0.082, { region: "core", material: "ceramic-honeycomb" }),
    ...blades(3, 0.03, 0.082, 0.06, 0.09, 30, 0.82),
    ellipse("hub", "rotor", "body", 3.4, true, 0.55, 0, 0, 0.04, 0.04, { region: "hub", material: "motor-housing" }),
  ],
  {
    silhouette: ["A 1.3 mm square with a dark ring, a light ceramic disc and a three-bladed pinwheel.", "Ein Quadrat von 1,3 mm mit einem dunklen Ring, einer hellen Keramikscheibe und einem dreiflügeligen Windrad."],
    retained: ["Plate, dark opening ring, ceramic disc, three broad blades and the hub.", "Platte, dunkler Öffnungsring, Keramikscheibe, drei breite Flügel und die Nabe."],
    merged: ["The housing ring joins the opening; the cell pattern is merged into the tone of the disc; five blades become three.", "Der Gehäusering geht in der Öffnung auf; das Zellmuster geht im Ton der Scheibe auf; aus fünf Flügeln werden drei."],
    omitted: ["Screws, hub cap, ring outline and cells.", "Schrauben, Nabenkappe, Ringkontur und Zellen."],
    thinElements: ["The blades are 60 to 90 mm wide (0.3 to 0.45 mm), the ceramic disc 164 mm (0.82 mm) so that the light disc reads between the blades.", "Die Flügel sind 60 bis 90 mm (0,3 bis 0,45 mm) breit, die Keramikscheibe 164 mm (0,82 mm), damit die helle Scheibe zwischen den Flügeln lesbar bleibt."],
    hatching: ["None.", "Keine."],
    styles: ["Plate 0.88, opening 0.12, ceramic 0.62, blades 0.82, hub 0.55; one body pen of 0.13 mm.", "Platte 0,88, Öffnung 0,12, Keramik 0,62, Flügel 0,82, Nabe 0,55; ein Körperstift von 0,13 mm."],
    overlays: ["The opening carries the casing region; core, blades and hub keep theirs.", "Die Öffnung trägt die Gehäuseregion; Kern, Flügel und Nabe behalten ihre."],
  },
);

const rep500 = unavailable(
  500,
  ["At 1:500 the 0.26 m unit is 0.52 mm wide and its blades 0.1 mm, below every print profile; use the airflow symbol of the clip at this scale.", "Bei 1:500 ist das 0,26 m große Gerät 0,52 mm breit und seine Flügel 0,1 mm, unter jedem Druckprofil; bei diesem Maßstab dient das Luftstromsymbol des Clips."],
  ["Blades of 22 mm width", "Flügel von 22 mm Breite"],
  0.022,
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
  region("casing", ["Cover plate and wall sleeve", "Blende und Wandhülse"], "surface", "operating", "abs-plastic", 0.09, 0.26, "vertical-plate", [single("shell", "ABS shell 2.5 mm", "ABS-Schale 2,5 mm", "polymer", "abs-plastic", 0.0025, 0.012, 2500)], 2500, { operating: { key: "fan", heatW: 1 } }),
  region("core", ["Ceramic regenerator core", "Keramik-Regeneratorkern"], "surface", "passive", "ceramic-honeycomb", 0.9, 0.003, "cylinder", [single("wall", "Ceramic cell wall 0.6 mm (half wall seen by one air side)", "Keramik-Zellwand 0,6 mm (Halbwand einer Luftseite)", "masonry", "ceramic-honeycomb", 0.0003, 0.0003, 620)], 620, {}, "iso-10456-2007"),
  region("blades", ["Blades", "Flügel"], "surface", "passive", "abs-plastic", 0.012, 0.05, "vertical-plate", [single("blade", "ABS blade 1.5 mm", "ABS-Flügel 1,5 mm", "polymer", "abs-plastic", 0.0015, 0.007, 1500)], 1500),
  region("hub", ["Motor hub", "Motornabe"], "engine", "operating", "motor-housing", 0.006, 0.056, "vertical-plate", [single("can", "Steel motor can 0.8 mm", "Stahlbecher des Motors 0,8 mm", "metal", "motor-housing", 0.0008, 0.00002, 6300)], 6300, { operating: { key: "fan", heatW: 3 } }),
];

const bind = (fan, extract, supply, air, fraction) => ({
  regions: {
    casing: { exposedFraction: 0.85, sunlitFraction: 0, contact: { fraction: 0.15, tempC: 21 } },
    core: { exposedFraction: 1 - fraction, sunlitFraction: 0, ...(fraction > 0 ? { contact: { fraction, tempC: air } } : {}) },
    blades: { exposedFraction: 1, sunlitFraction: 0 },
    hub: { exposedFraction: 1, sunlitFraction: 0 },
  },
  emitters: { "exhaust-out": extract, "supply-in": supply },
  operating: { fan, extract, supply },
});

const wall = ["wall-screw-top", "wall-screw-bottom"];
const poses = [
  pose("off", ["Off", "Aus"], ["The rotor stands still, no air moves and the core is at room temperature.", "Das Laufrad steht still, keine Luft bewegt sich und der Kern hat Raumtemperatur."], {}, wall, "authored", bind(0, 0, 0, 21, 0)),
  pose("extract", ["Extract phase", "Abluftphase"], ["The fan blows room air of 21 degC outwards through the core: the ceramic takes up its heat and the air leaves the house at about 0 degC.", "Der Ventilator bläst Raumluft von 21 degC durch den Kern nach außen: die Keramik nimmt ihre Wärme auf und die Luft verlässt das Haus mit etwa 0 degC."], { rotor: { angleDeg: 0 } }, wall, "authored", bind(1, 1, 0, 21, 1)),
  pose("supply", ["Supply phase", "Zuluftphase"], ["The fan reverses and draws outdoor air of -5 degC inwards through the warm core: the ceramic gives its heat back and the air enters the room at about 16 degC.", "Der Ventilator kehrt um und saugt Außenluft von -5 degC durch den warmen Kern nach innen: die Keramik gibt ihre Wärme zurück und die Luft tritt mit etwa 16 degC in den Raum."], { rotor: { angleDeg: 720 } }, wall, "authored", bind(1, 0, 1, -5, 1)),
];

const contacts = (d) => wall.map((anchor) => ({ anchor, fromS: 0, toS: d }));

const PHASE = 70;
const PHASES = 6;
const cycleKeys = [];
for (let i = 0; i < PHASES; i += 1) {
  const extract = i % 2 === 0;
  cycleKeys.push({ t: i * PHASE, pose: extract ? "extract" : "supply", ease: "smooth", bones: { rotor: { angleDeg: extract ? 0 : 720 } } });
  cycleKeys.push({ t: i * PHASE + PHASE - 3, pose: extract ? "extract" : "supply", ease: "linear", bones: { rotor: { angleDeg: extract ? 720 : 0 } } });
}
cycleKeys.push({ t: PHASES * PHASE, pose: "supply", ease: "linear", bones: { rotor: { angleDeg: 0 } } });
const sw = Array.from({ length: PHASES }, (_, i) => [i * PHASE, i % 2 === 0 ? 1 : 0, i % 2 === 0 ? 0 : 1]);
const phases = Array.from({ length: PHASES }, (_, i) => [i % 2 === 0 ? `extract-${i / 2 + 1}` : `supply-${(i - 1) / 2 + 1}`, i * PHASE, (i + 1) * PHASE, i % 2 === 0 ? ["Extract", "Abluft"] : ["Supply", "Zuluft"]]);

const clips = [
  clip(
    "pendulum-cycle",
    ["Pendulum cycle with reversing spin", "Pendelbetrieb mit umkehrender Drehrichtung"],
    ["The fan pushes room air outwards for 70 seconds while the core charges, stops, turns the other way and draws outdoor air inwards for 70 seconds while the core discharges; three double cycles. The shown turning is slowed to two revolutions per phase.", "Der Ventilator bläst 70 Sekunden lang Raumluft nach außen, während sich der Kern lädt, steht kurz, dreht in die Gegenrichtung und saugt 70 Sekunden lang Außenluft nach innen, während sich der Kern entlädt; drei Doppelzyklen. Die gezeigte Drehung ist auf zwei Umdrehungen je Phase verlangsamt."],
    PHASES * PHASE,
    15,
    cycleKeys,
    contacts(PHASES * PHASE),
    phases,
    [[0, "operating", ["Fan runs, extract phase", "Ventilator läuft, Abluftphase"], "fan"], ...Array.from({ length: PHASES - 1 }, (_, i) => [(i + 1) * PHASE, "operating", [i % 2 === 0 ? "Reversal to supply" : "Reversal to extract", i % 2 === 0 ? "Umkehr auf Zuluft" : "Umkehr auf Abluft"], i % 2 === 0 ? "supply" : "extract"])],
    [
      ["operating:fan", "linear", [[0, 1], [PHASES * PHASE, 1]]],
      ["operating:extract", "step", sw.flatMap(([t, e]) => [[t, e]])],
      ["operating:supply", "step", sw.flatMap(([t, , s]) => [[t, s]])],
      ["emitter:exhaust-out", "step", sw.flatMap(([t, e]) => [[t, e]])],
      ["emitter:supply-in", "step", sw.flatMap(([t, , s]) => [[t, s]])],
    ],
  ),
  clip(
    "reverse-spin",
    ["Reversal of the spin", "Umkehr der Drehrichtung"],
    ["The rotor runs two revolutions in the extract direction, comes to rest and returns the same way in the supply direction; the turning rate is slowed to 180 degrees per second.", "Das Laufrad macht zwei Umdrehungen in Abluftrichtung, kommt zur Ruhe und läuft in Zuluftrichtung zurück; die Drehrate ist auf 180 Grad je Sekunde verlangsamt."],
    8,
    15,
    [
      { t: 0, pose: "extract", ease: "smooth", bones: { rotor: { angleDeg: 0 } } },
      { t: 4, pose: "extract", ease: "smooth", bones: { rotor: { angleDeg: 720 } } },
      { t: 8, pose: "supply", ease: "smooth", bones: { rotor: { angleDeg: 0 } } },
    ],
    contacts(8),
    [["extract", 0, 4, ["Extract direction", "Abluftrichtung"]], ["supply", 4, 8, ["Supply direction", "Zuluftrichtung"]]],
    [[4, "operating", ["Reversal", "Umkehr"], "supply"]],
    [["operating:fan", "linear", [[0, 1], [8, 1]]], ["operating:extract", "step", [[0, 1], [4, 0]]], ["operating:supply", "step", [[0, 0], [4, 1]]], ["emitter:exhaust-out", "step", [[0, 1], [4, 0]]], ["emitter:supply-in", "step", [[0, 0], [4, 1]]]],
  ),
];

export const hrvFan = () => {
  const massKg = sum(sp.bones.map((b) => b.mass.kg));
  return wrap({
    id: "decentralised-hrv-fan",
    category: "appliances",
    title: tx("Decentralised Reversing Fan with Ceramic Heat Store", "Dezentraler Pendellüfter mit Keramik-Wärmespeicher"),
    summary: tx(
      "A 0.26 m square decentralised push-pull ventilation unit with its inner cover off: a reversible fan turns in front of a ceramic honeycomb core that stores the heat of the extract air for 70 seconds and gives it back to the supply air for the next 70.",
      "Ein quadratisches dezentrales Pendellüftungsgerät von 0,26 m bei abgenommener Innenblende: ein umkehrbarer Ventilator dreht sich vor einem Keramik-Wabenkern, der die Wärme der Abluft 70 Sekunden speichert und in den nächsten 70 Sekunden an die Zuluft zurückgibt.",
    ),
    tags: ["fan", "reversing-fan", "push-pull", "pendulum-ventilation", "decentralised", "heat-recovery", "regenerator", "ceramic", "ventilation", "wall-mounted", "appliance", "electrical", "rotating"],
    provenance: {
      author: "Semio staffage content (F2)",
      method: "handcrafted",
      created: "2026-11-01",
      references: [
        ref("din-en-13141-8-2014", "DIN EN 13141-8:2014 Ventilation for buildings - Performance testing of components for residential ventilation - Unducted mechanical supply and exhaust ventilation units (including heat recovery); regenerative push-pull units reverse every 50 to 90 seconds."),
        ref("passive-house-institute-2021", "Passive House Institute (2021). Criteria for certified ventilation units: heat recovery efficiencies of 75 to 90 percent for decentralised units with ceramic regenerators and specific electric power below 0.45 Wh per cubic metre."),
        INCROPERA,
        ref("iso-10456-2007", "ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values and procedures for determining declared and design thermal values."),
      ],
      notes: tx(
        "A decentralised push-pull unit of the common DN160 wall sleeve class seen from the room with the inner cover and the filter taken off: a cover plate of 0.26 m by 0.26 m with rounded corners, a sleeve ring of 0.196 m, a ceramic honeycomb core of 0.16 m diameter and 0.1 m depth with hexagonal cells, and in front of it a five-bladed reversible axial fan on a 56 mm motor hub. The origin is the centre of the hub in the wall plane; the unit is 0.24 m deep (depthM, the sleeve through the wall is not drawn). Authored estimate for dimensions and the mass of 2.7 kg. The fan blows room air outwards for 70 seconds (the warm air heats the ceramic, the air leaves at about 0 degC) and then reverses and draws outdoor air inwards for 70 seconds (the ceramic warms it to about 16 degC): the heat recovery efficiency is about 80 percent, at 35 m3/h per unit and 4 W of electric power (core, motor and electronics) in the working stage. The clip shows the real 1500 rpm slowed to two revolutions per 70 s phase, with a short standstill at each reversal.",
        "Ein dezentrales Pendellüftungsgerät der üblichen Klasse für DN160-Wandhülsen, vom Raum aus gesehen bei abgenommener Innenblende und ohne Filter: eine Blende von 0,26 m mal 0,26 m mit gerundeten Ecken, ein Hülsenring von 0,196 m, ein Keramik-Wabenkern von 0,16 m Durchmesser und 0,1 m Tiefe mit sechseckigen Zellen und davor ein fünfflügeliger umkehrbarer Axialventilator auf einer 56 mm großen Motornabe. Der Ursprung ist die Mitte der Nabe in der Wandebene; das Gerät ist 0,24 m tief (depthM, die Hülse durch die Wand ist nicht gezeichnet). Maße und die Masse von 2,7 kg sind Schätzwerte. Der Ventilator bläst 70 Sekunden lang Raumluft nach außen (die warme Luft erwärmt die Keramik, die Luft verlässt das Haus mit etwa 0 degC) und kehrt dann um und saugt 70 Sekunden lang Außenluft nach innen (die Keramik erwärmt sie auf etwa 16 degC): der Wärmebereitstellungsgrad beträgt etwa 80 Prozent, bei 35 m3/h je Gerät und 4 W elektrischer Leistung (Kern, Motor und Elektronik) in der Arbeitsstufe. Der Clip zeigt die echten 1500 U/min verlangsamt auf zwei Umdrehungen je 70-s-Phase mit kurzem Stillstand bei jeder Umkehr.",
      ),
    },
    dimensions: { heightM: 0.26, widthM: 0.26, depthM: 0.24, massKg },
    materials: [
      materialOf("abs-plastic", "White ABS plastic", "Weißer ABS-Kunststoff", 0.9, 0.3, 1050, 0.17, 1400, SRC, "incropera-2018"),
      materialOf("motor-housing", "Steel motor can", "Stahlbecher des Motors", 0.3, 0.5, 7800, 50, 490, SRC, "incropera-2018"),
      materialOf("ceramic-honeycomb", "Ceramic honeycomb", "Keramik-Wabe", 0.9, 0.6, 2300, 1.2, 900, SRC, "iso-10456-2007"),
    ],
    rig: { kind: "mechanical", root: "plate", bones: sp.bones.map((b) => ({ ...b, mass: { ...b.mass } })) },
    anchors: [
      { id: "wall-screw-top", bone: "plate", x: 0, y: 0.108, kind: "support", title: tx("Upper screw in the wall", "Obere Schraube in der Wand"), support: "pin", normalDeg: 0, footprintLengthM: 0.014, footprintWidthM: 0.014 },
      { id: "wall-screw-bottom", bone: "plate", x: 0, y: -0.108, kind: "support", title: tx("Lower screw in the wall", "Untere Schraube in der Wand"), support: "roller", normalDeg: 0, footprintLengthM: 0.014, footprintWidthM: 0.014, frictionCoefficient: 0.4 },
      { id: "outlet", bone: "ring", x: 0, y: 0, kind: "emitter", title: tx("Centre of the sleeve opening", "Mitte der Hülsenöffnung") },
    ],
    representations: { 500: rep500, 200: rep200, 100: rep100, 50: rep50 },
    symbols: [],
    poses,
    clips,
    thermal: {
      environment: { airTempC: 21, meanRadiantTempC: 21, relativeHumidity: 0.5, windMS: 0.1, windDirectionDeg: 0, solarWM2: 0, diffuseFraction: 0, pressurePa: 101325 },
      passengers: [],
      operating: [
        { id: "fan", title: tx("Fan", "Ventilator"), levels: [level(0, "Off", "Aus"), level(1, "Running (35 m3/h, 4 W)", "Läuft (35 m3/h, 4 W)")] },
        { id: "extract", title: tx("Extract phase", "Abluftphase"), levels: [level(0, "Not extracting", "Keine Abluft"), level(1, "Extracting through the core", "Abluft durch den Kern")] },
        { id: "supply", title: tx("Supply phase", "Zuluftphase"), levels: [level(0, "Not supplying", "Keine Zuluft"), level(1, "Supplying through the core", "Zuluft durch den Kern")] },
      ],
      regions,
      emitters: [
        { id: "exhaust-out", kind: "vent", title: tx("Cooled extract air leaving to outside", "Abgekühlte Abluft nach außen"), anchor: "outlet", directionDeg: 180, diameterM: 0.16, operatingKey: "extract", volumeFlowM3S: 0.0097, temperatureRiseK: -21, vapourKgPerKg: 0, source: SRC, reference: "din-en-13141-8-2014" },
        { id: "supply-in", kind: "vent", title: tx("Preheated outdoor air entering the room", "Vorgewärmte Außenluft in den Raum"), anchor: "outlet", directionDeg: 0, diameterM: 0.16, operatingKey: "supply", volumeFlowM3S: 0.0097, temperatureRiseK: -5, vapourKgPerKg: 0, source: SRC, reference: "din-en-13141-8-2014" },
      ],
      assumptions: [
        tx(
          "The core is a regenerator: during the extract phase the 21 degC room air heats the ceramic and leaves at about 0 degC, in the supply phase the -5 degC outdoor air takes the heat back and reaches the room at about 16 degC (5 K below the 21 degC reference, 80 percent heat recovery of the 26 K difference); the core region sees the air stream as a contact at the temperature of the phase, with the half cell wall of 0.3 mm and about 0.9 m2 of cell surface giving a time constant of the order of the 70 s phase.",
          "Der Kern ist ein Regenerator: in der Abluftphase erwärmt die 21 degC warme Raumluft die Keramik und verlässt sie mit etwa 0 degC, in der Zuluftphase holt die -5 degC kalte Außenluft die Wärme zurück und erreicht den Raum mit etwa 16 degC (5 K unter der Bezugstemperatur von 21 degC, 80 Prozent Wärmerückgewinnung der 26 K Differenz); die Kernregion sieht den Luftstrom als Kontakt mit der Temperatur der Phase, mit der Halbwand von 0,3 mm und etwa 0,9 m2 Zelloberfläche ergibt sich eine Zeitkonstante in der Größenordnung der 70-s-Phase.",
        ),
        tx(
          "The 4 W of electric power end as heat: 1 W at the plate and sleeve and 3 W at the hub at full level; the emitter directions are nominal in-plane directions (the real flow is normal to the drawing plane).",
          "Die 4 W elektrische Leistung enden als Wärme: 1 W an Blende und Hülse und 3 W an der Nabe auf voller Stufe; die Emitterrichtungen sind nominelle Richtungen in der Ebene (die echte Strömung steht senkrecht zur Zeichenebene).",
        ),
      ],
    },
    structure: {
      gravityMS2: 9.80665,
      dynamics: "quasi-static",
      supports: wall,
      assumptions: [
        tx(
          "The 2.7 kg unit hangs on the upper screw (pin) and leans against the wall at the lower screw (roller); most of its mass sits in the sleeve inside the wall.",
          "Das 2,7 kg schwere Gerät hängt an der oberen Schraube (Festlager) und lehnt an der unteren Schraube an der Wand (Rollenlager); der größte Teil seiner Masse sitzt in der Hülse in der Wand.",
        ),
      ],
      clinical: false,
    },
    presentation: { defaultPose: "extract", defaultClip: "pendulum-cycle", defaultScale: 50, viewpoints: ["elevation"] },
    exports: ALL_EXPORTS,
    completion: "authored",
  });
};
