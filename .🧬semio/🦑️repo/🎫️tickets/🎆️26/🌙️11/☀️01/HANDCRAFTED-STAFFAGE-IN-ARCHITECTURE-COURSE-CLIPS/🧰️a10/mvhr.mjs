import { ALL_EXPORTS, INCROPERA, authored, clip, layer, level, pose, ref, tx, unavailable, wrap } from "./lib.mjs";
import { materialOf, space, sum } from "./space.mjs";

const SRC = "authored-estimate";
const m = (kg, comX, comY) => ({ kg, comX, comY, distribution: "point", source: SRC });
const fixed = { type: "fixed" };

const CY = 0.3;
const sp = space([
  ["casing", null, 0, 0, { title: tx("Casing with heat exchanger", "Gehäuse mit Wärmeübertrager"), lengthM: 0.6, joint: fixed, mass: m(21, 0, 0.3) }],
  ["spigots", "casing", 0, 0.6, { title: tx("Four duct spigots", "Vier Kanalstutzen"), lengthM: 0.065, joint: fixed, mass: m(1.6, 0, 0.03) }],
  ["impeller", "casing", 0, CY, { title: tx("Supply impeller behind the window", "Zuluftlaufrad hinter dem Fenster"), lengthM: 0.1, joint: { type: "revolute" }, mass: m(1.4, 0, 0) }],
]);
const { path, ellipse, rect, poly } = sp;

const r5 = (n) => Math.round(n * 1e5) / 1e5;
const bladePath = (k, n, rootR, tipR, rootW, tipW, sweepDeg) => {
  const steps = 8;
  const edge = (sign) => {
    const pts = [];
    for (let i = 0; i <= steps; i += 1) {
      const u = i / steps;
      const r = rootR + (tipR - rootR) * u;
      const w = rootW + (tipW - rootW) * u;
      const a = ((360 / n) * k + sweepDeg * u) * (Math.PI / 180) + sign * Math.atan2(w / 2, r);
      pts.push([r5(r * Math.cos(a)), r5(CY + r * Math.sin(a))]);
    }
    return pts;
  };
  const pts = [...edge(1), ...edge(-1).reverse()].map(([x, y]) => [x, y]);
  return `M ${pts.map(([x, y]) => `${x} ${y}`).join(" L ")} Z`;
};
const blades = (n, rootR, tipR, rootW, tipW, sweep, tone) =>
  Array.from({ length: n }, (_, k) => path(`blade-${k + 1}`, "impeller", "body", 3.5, true, tone, bladePath(k, n, rootR, tipR, rootW, tipW, sweep), { region: "impeller", material: "pp-impeller" }));

const X = [-0.225, -0.075, 0.075, 0.225];
const collars = (w, h, beadW, beadH, withBead) =>
  X.flatMap((x, i) => [
    rect(`spigot-${i + 1}`, "spigots", "body", 3.1, true, 0.66, x - w / 2, 0.6, w, h, { region: "ducts", material: "galvanised-steel" }),
    ...(withBead ? [rect(`bead-${i + 1}`, "spigots", "body", 3.2, true, 0.5, x - beadW / 2, 0.6 + h - beadH, beadW, beadH, { region: "ducts", material: "galvanised-steel" })] : []),
  ]);

const rep50 = authored(
  50,
  [20, 75],
  { body: 0.18, detail: 0.13, texture: null, glazing: 0.13, ground: 0.35 },
  [
    rect("casing", "casing", "body", 3, true, 0.82, -0.3, 0, 0.6, 0.6, { r: 0.025, region: "casing", material: "painted-steel" }),
    ...collars(0.098, 0.062, 0.114, 0.014, true),
    ellipse("window-ring", "casing", "body", 3.4, true, 0.6, 0, CY, 0.128, 0.128, { region: "casing", material: "painted-steel" }),
    ellipse("window", "casing", "glazing", 3.45, true, 0.14, 0, CY, 0.112, 0.112, { region: "window", material: "polycarbonate" }),
    ...blades(9, 0.04, 0.1, 0.026, 0.03, -42, 0.8),
    ellipse("hub", "impeller", "body", 3.6, true, 0.5, 0, CY, 0.04, 0.04, { region: "impeller", material: "pp-impeller" }),
    ellipse("led", "casing", "detail", 3.5, true, 0.95, 0.215, 0.1, 0.014, 0.014, { region: "casing", material: "painted-steel" }),
    rect("handle", "casing", "detail", 3.5, true, 0.45, -0.08, 0.03, 0.16, 0.024, { r: 0.012, region: "casing", material: "painted-steel" }),
    poly("cover-seam", "casing", "detail", 3.3, false, 0.4, [[-0.275, 0.07], [-0.275, 0.565], [0.275, 0.565], [0.275, 0.07], [-0.275, 0.07]], false),
  ],
  {
    silhouette: ["A 0.6 m by 0.665 m wall unit, 12 mm by 13.3 mm on paper: a light box with a round inspection window showing a nine-bladed impeller and four round duct spigots with a bead each on its top edge.", "Ein Wandgerät von 0,6 m mal 0,665 m, auf Papier 12 mm mal 13,3 mm: ein heller Kasten mit rundem Schauglas, das ein neunflügeliges Laufrad zeigt, und vier runde Kanalstutzen mit je einer Sicke am oberen Rand."],
    retained: ["Casing of 0.6 m with its rounded corners, window ring and window of 0.224 m, nine backward-curved blades with the hub, four spigots of 98 mm with their beads, the cover seam, the status light and the filter-door handle.", "Gehäuse von 0,6 m mit gerundeten Ecken, Fensterring und Fenster von 0,224 m, neun rückwärts gekrümmte Schaufeln mit Nabe, vier Stutzen von 98 mm mit ihren Sicken, die Deckelfuge, die Statusleuchte und der Griff der Filtertür."],
    merged: ["The heat exchanger, the extract fan and the bypass flap behind the cover are part of the casing region; only the supply impeller shows through the window.", "Wärmeübertrager, Abluftventilator und Bypassklappe hinter dem Deckel gehören zur Gehäuseregion; nur das Zuluftlaufrad zeigt sich durch das Fenster."],
    omitted: ["Condensate drain, power cable, screws, the label and the duct labels.", "Kondensatablauf, Netzkabel, Schrauben, das Typenschild und die Stutzenbeschriftung."],
    thinElements: ["The beads are 14 mm (0.28 mm); the blades are 26 to 30 mm wide (0.5 to 0.6 mm); the seam, the light and the handle are 0.13 mm detail pen.", "Die Sicken sind 14 mm (0,28 mm) hoch; die Schaufeln sind 26 bis 30 mm (0,5 bis 0,6 mm) breit; Fuge, Leuchte und Griff sind im 0,13-mm-Detailstift gezeichnet."],
    hatching: ["None; the window is a dark tone.", "Keine; das Fenster ist ein dunkler Ton."],
    styles: ["Casing 0.82, window ring 0.6, window 0.14, blades 0.8, spigots 0.66, beads 0.5; body pen 0.18 mm, detail and glazing 0.13 mm, ground 0.35 mm.", "Gehäuse 0,82, Fensterring 0,6, Fenster 0,14, Schaufeln 0,8, Stutzen 0,66, Sicken 0,5; Körperstift 0,18 mm, Detail und Verglasung 0,13 mm, Boden 0,35 mm."],
    overlays: ["Casing, window, impeller and ducts are four regions: in the thermal view the casing and the impeller warm, the spigots follow the air they carry.", "Gehäuse, Fenster, Laufrad und Kanäle sind vier Regionen: in der Thermalansicht erwärmen sich Gehäuse und Laufrad, die Stutzen folgen der Luft, die sie tragen."],
  },
);

const rep100 = authored(
  100,
  [75, 150],
  { body: 0.18, detail: null, texture: null, glazing: null, ground: 0.25 },
  [
    rect("casing", "casing", "body", 3, true, 0.82, -0.3, 0, 0.6, 0.6, { r: 0.025, region: "casing", material: "painted-steel" }),
    ...collars(0.098, 0.062, 0.114, 0.016, true),
    ellipse("window", "casing", "body", 3.4, true, 0.16, 0, CY, 0.118, 0.118, { regions: ["casing", "window"], material: "polycarbonate" }),
    ...blades(7, 0.045, 0.108, 0.034, 0.04, -40, 0.8),
    ellipse("hub", "impeller", "body", 3.6, true, 0.5, 0, CY, 0.048, 0.048, { region: "impeller", material: "pp-impeller" }),
  ],
  {
    silhouette: ["A 6 mm by 6.7 mm box with a round window holding a seven-bladed pinwheel and four spigots on top.", "Ein Kasten von 6 mm mal 6,7 mm mit rundem Fenster, das ein siebenflügeliges Windrad enthält, und vier Stutzen oben."],
    retained: ["Casing with rounded corners, window, seven curved blades and the hub, four spigots with beads.", "Gehäuse mit gerundeten Ecken, Fenster, sieben gekrümmte Schaufeln und die Nabe, vier Stutzen mit Sicken."],
    merged: ["Window ring and window are one disc (regions casing and window merged); nine blades become seven.", "Fensterring und Fenster sind eine Scheibe (Regionen Gehäuse und Fenster zusammengeführt); aus neun Schaufeln werden sieben."],
    omitted: ["Seam, status light and handle.", "Fuge, Statusleuchte und Griff."],
    thinElements: ["The blades are widened to 34 to 40 mm (0.34 to 0.4 mm) and the beads to 16 mm (0.16 mm).", "Die Schaufeln sind auf 34 bis 40 mm (0,34 bis 0,4 mm) und die Sicken auf 16 mm (0,16 mm) verbreitert."],
    hatching: ["None.", "Keine."],
    styles: ["Casing 0.82, window 0.16, blades 0.8, hub 0.5, spigots 0.66, beads 0.5; one body pen of 0.18 mm.", "Gehäuse 0,82, Fenster 0,16, Schaufeln 0,8, Nabe 0,5, Stutzen 0,66, Sicken 0,5; ein Körperstift von 0,18 mm."],
    overlays: ["The window disc lists casing and window as merged regions; impeller and ducts stay their own.", "Die Fensterscheibe führt Gehäuse und Fenster als zusammengeführte Regionen; Laufrad und Kanäle bleiben eigene."],
  },
);

const rep200 = authored(
  200,
  [150, 350],
  { body: 0.13, detail: null, texture: null, glazing: null, ground: 0.25 },
  [
    rect("casing", "casing", "body", 3, true, 0.82, -0.3, 0, 0.6, 0.6, { r: 0.03, region: "casing", material: "painted-steel" }),
    ...X.map((x, i) => rect(`spigot-${i + 1}`, "spigots", "body", 3.1, true, 0.62, x - 0.045, 0.6, 0.09, 0.065, { region: "ducts", material: "galvanised-steel" })),
    ellipse("window", "casing", "body", 3.4, true, 0.16, 0, CY, 0.125, 0.125, { regions: ["casing", "window"], material: "polycarbonate" }),
    ...blades(4, 0.05, 0.115, 0.08, 0.09, -35, 0.8),
    ellipse("hub", "impeller", "body", 3.6, true, 0.5, 0, CY, 0.056, 0.056, { region: "impeller", material: "pp-impeller" }),
  ],
  {
    silhouette: ["A 3 mm by 3.3 mm box with a dark disc holding a four-bladed pinwheel and four stubs on top.", "Ein Kasten von 3 mm mal 3,3 mm mit dunkler Scheibe, die ein vierflügeliges Windrad enthält, und vier Stummeln oben."],
    retained: ["Box, window disc, four wide curved blades with the hub, four spigots.", "Kasten, Fensterscheibe, vier breite gekrümmte Schaufeln mit Nabe, vier Stutzen."],
    merged: ["Window ring and window are one disc; the beads join the spigots, which are drawn 90 mm wide; nine blades become four.", "Fensterring und Fenster sind eine Scheibe; die Sicken gehen in den Stutzen auf, die 90 mm breit gezeichnet sind; aus neun Schaufeln werden vier."],
    omitted: ["Seam, light, handle and beads.", "Fuge, Leuchte, Griff und Sicken."],
    thinElements: ["The blades are 80 to 90 mm wide (0.4 to 0.45 mm).", "Die Schaufeln sind 80 bis 90 mm (0,4 bis 0,45 mm) breit."],
    hatching: ["None.", "Keine."],
    styles: ["Casing 0.82, window 0.16, blades 0.8, hub 0.5, spigots 0.62; one body pen of 0.13 mm.", "Gehäuse 0,82, Fenster 0,16, Schaufeln 0,8, Nabe 0,5, Stutzen 0,62; ein Körperstift von 0,13 mm."],
    overlays: ["As at 1:100.", "Wie bei 1:100."],
  },
);

const rep500 = unavailable(
  500,
  ["At 1:500 the 0.6 m unit is 1.2 mm wide and its spigot gaps 0.04 mm, below every print profile.", "Bei 1:500 ist das 0,6 m große Gerät 1,2 mm breit und die Lücken zwischen den Stutzen 0,04 mm, unter jedem Druckprofil."],
  ["Gap of 42 mm between the spigots", "Lücke von 42 mm zwischen den Stutzen"],
  0.042,
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
  skyViewFactor: 0.3,
  layers,
  heatCapacityJM2K,
  ...extra,
  source: SRC,
  reference,
});

const regions = [
  region("casing", ["Casing shell", "Gehäuseschale"], "surface", "operating", "painted-steel", 0.9, 0.6, "vertical-plate", [single("sheet", "Painted steel sheet 1 mm with 20 mm EPP lining", "Lackiertes Stahlblech 1 mm mit 20 mm EPP-Auskleidung", "insulation", "painted-steel", 0.02, 0.5, 3000)], 3000, { operating: { key: "fan", heatW: 45 } }),
  region("window", ["Inspection window", "Schauglas"], "glazing", "passive", "polycarbonate", 0.04, 0.224, "vertical-plate", [single("pane", "Polycarbonate pane 3 mm", "Polycarbonatscheibe 3 mm", "polymer", "polycarbonate", 0.003, 0.015, 3600)], 3600),
  region("impeller", ["Supply impeller and motor", "Zuluftlaufrad und Motor"], "engine", "operating", "pp-impeller", 0.06, 0.2, "vertical-plate", [single("blades", "Polypropylene blades 2 mm", "Polypropylenschaufeln 2 mm", "polymer", "pp-impeller", 0.002, 0.01, 2900)], 2900, { operating: { key: "fan", heatW: 30 } }),
  region("ducts", ["Duct spigots", "Kanalstutzen"], "surface", "passive", "galvanised-steel", 0.08, 0.098, "cylinder", [single("tube", "Galvanised steel tube 0.7 mm", "Verzinktes Stahlrohr 0,7 mm", "metal", "galvanised-steel", 0.0007, 0.00002, 3500)], 3500),
];

const bind = (fan) => ({
  regions: {
    casing: { exposedFraction: 0.85, sunlitFraction: 0, contact: { fraction: 0.15, tempC: 21 } },
    window: { exposedFraction: 1, sunlitFraction: 0 },
    impeller: { exposedFraction: 1, sunlitFraction: 0 },
    ducts: { exposedFraction: 1, sunlitFraction: 0 },
  },
  emitters: { "supply-air": fan, "extract-air": fan },
  operating: { fan },
});

const wall = ["wall-hook", "wall-rest"];
const poses = [
  pose("off", ["Off", "Aus"], ["Fans are off; the unit is at room temperature and no air moves.", "Die Ventilatoren sind aus; das Gerät hat Raumtemperatur und keine Luft bewegt sich."], {}, wall, "authored", bind(0)),
  pose("normal", ["Normal ventilation", "Grundlüftung"], ["Both fans run at the normal stage: 110 m3/h in each direction with 38 W; 85 percent of the heat of the extract air goes to the supply air.", "Beide Ventilatoren laufen auf der Grundstufe: 110 m3/h in jede Richtung mit 38 W; 85 Prozent der Wärme der Abluft gehen in die Zuluft."], { impeller: { angleDeg: 20 } }, wall, "authored", bind(0.5)),
  pose("boost", ["Boost", "Intensivlüftung"], ["Both fans run at full speed: 220 m3/h in each direction with 75 W.", "Beide Ventilatoren laufen mit voller Drehzahl: 220 m3/h in jede Richtung mit 75 W."], { impeller: { angleDeg: 40 } }, wall, "authored", bind(1)),
];

const contacts = (d) => wall.map((anchor) => ({ anchor, fromS: 0, toS: d }));

const clips = [
  clip(
    "boost-spin",
    ["Spin up to boost", "Hochlaufen auf Intensiv"],
    ["The impeller starts from rest, runs at the normal stage after one second and at boost after four; the turning rate is slowed to 120 and 240 degrees per second.", "Das Laufrad läuft aus dem Stillstand an, läuft nach einer Sekunde auf der Grundstufe und nach vier auf Intensiv; die Drehrate ist auf 120 und 240 Grad je Sekunde verlangsamt."],
    6,
    15,
    [
      { t: 0, pose: "off", ease: "in" },
      { t: 1, pose: "normal", ease: "linear", bones: { impeller: { angleDeg: 60 } } },
      { t: 3, pose: "normal", ease: "in", bones: { impeller: { angleDeg: 300 } } },
      { t: 4, pose: "boost", ease: "linear", bones: { impeller: { angleDeg: 540 } } },
      { t: 6, pose: "boost", ease: "linear", bones: { impeller: { angleDeg: 1020 } } },
    ],
    contacts(6),
    [["start", 0, 1, ["Starting", "Anlaufen"]], ["normal", 1, 3, ["Normal", "Grundlüftung"]], ["boost", 3, 6, ["Boost", "Intensiv"]]],
    [[0, "operating", ["Fans switched on", "Ventilatoren eingeschaltet"], "fan"]],
    [["operating:fan", "linear", [[0, 0], [1, 0.5], [3, 0.5], [4, 1], [6, 1]]], ["emitter:supply-air", "linear", [[0, 0], [1, 0.5], [3, 0.5], [4, 1], [6, 1]]], ["emitter:extract-air", "linear", [[0, 0], [1, 0.5], [3, 0.5], [4, 1], [6, 1]]]],
  ),
  clip(
    "day-cycle",
    ["Normal, boost and off", "Grundlüftung, Intensiv und Aus"],
    ["The unit runs at the normal stage, goes to boost for ten minutes (a shower) and returns to normal before it is switched off.", "Das Gerät läuft auf der Grundstufe, geht für zehn Minuten auf Intensiv (eine Dusche) und kehrt auf die Grundstufe zurück, bevor es ausgeschaltet wird."],
    3600,
    1,
    [{ t: 0, pose: "normal", ease: "step" }, { t: 600, pose: "boost", ease: "step" }, { t: 1200, pose: "normal", ease: "step" }, { t: 2400, pose: "off", ease: "step" }, { t: 3600, pose: "off", ease: "step" }],
    contacts(3600),
    [["normal", 0, 600, ["Normal", "Grundlüftung"]], ["boost", 600, 1200, ["Boost", "Intensiv"]], ["normal-2", 1200, 2400, ["Normal", "Grundlüftung"]], ["off", 2400, 3600, ["Off, cooling", "Aus, kühlt ab"]]],
    [[2400, "shutdown", ["Fans switched off", "Ventilatoren ausgeschaltet"], "fan"]],
    [["operating:fan", "step", [[0, 0.5], [600, 1], [1200, 0.5], [2400, 0]]], ["emitter:supply-air", "step", [[0, 0.5], [600, 1], [1200, 0.5], [2400, 0]]], ["emitter:extract-air", "step", [[0, 0.5], [600, 1], [1200, 0.5], [2400, 0]]]],
  ),
];

export const mvhr = () => {
  const massKg = sum(sp.bones.map((b) => b.mass.kg));
  return wrap({
    id: "mvhr-unit",
    category: "appliances",
    title: tx("Wall-Mounted Heat-Recovery Ventilation Unit", "Wandmontiertes Lüftungsgerät mit Wärmerückgewinnung"),
    summary: tx(
      "A wall-mounted mechanical ventilation unit with heat recovery of 0.6 m by 0.6 m: a box with an inspection window onto the turning supply impeller and four duct spigots on top; it moves 110 to 220 m3/h and returns 85 percent of the heat of the extract air for 38 to 75 W.",
      "Ein wandmontiertes Lüftungsgerät mit Wärmerückgewinnung von 0,6 m mal 0,6 m: ein Kasten mit Schauglas auf das drehende Zuluftlaufrad und vier Kanalstutzen oben; es bewegt 110 bis 220 m3/h und gibt 85 Prozent der Abluftwärme zurück bei 38 bis 75 W.",
    ),
    tags: ["mvhr", "heat-recovery", "ventilation", "mechanical-ventilation", "appliance", "wall-mounted", "supply-air", "extract-air", "internal-gain", "electrical", "rotating"],
    provenance: {
      author: "Semio staffage content (A10)",
      method: "handcrafted",
      created: "2026-11-01",
      references: [
        ref("din-en-13141-7-2021", "DIN EN 13141-7:2021 Ventilation for buildings - Performance testing of components/products for residential ventilation - Part 7: Mechanical supply and exhaust ventilation units with heat recovery (temperature ratio, electric power, specific fan power)."),
        ref("en-308-1997", "EN 308:1997 Heat exchangers - Test procedures for establishing performance of air to air and flue gases heat recovery devices (temperature efficiency, test point of 0 degC outdoor air and 21 degC extract air)."),
        ref("phi-2020", "Passive House Institute (2020). Criteria for certified ventilation units with heat recovery: effective heat recovery of at least 75 percent, electric efficiency of at most 0.45 Wh/m3."),
        INCROPERA,
      ],
      notes: tx(
        "A compact wall unit of the 250 m3/h class in front elevation: a casing of 0.6 m width and 0.6 m height with a round inspection window of 0.224 m showing the supply impeller (nine backward-curved blades), a status light and a filter-door handle, and four DN100 spigots of 98 mm on top (from the left: exhaust, extract, supply, outdoor air), each with a rolled bead. The unit is 0.3 m deep (depthM) and weighs 24 kg; the overall height with the spigots is 0.665 m. The origin is the lower edge of the casing on the wall, x = 0 on the centreline. Dimensions and masses are rounded from this product class (authored estimate). At level 1 each stream is 0.061 m3/s (220 m3/h) and the two fans take 75 W (45 W at the casing, 30 W at the supply impeller); at level 0.5 it is 110 m3/h and 38 W, linearised from a fan law with an SFP of 0.34 to 0.38 Wh/m3. The heat recovery is 85 percent (temperature ratio of the heat exchanger, EN 308 test point 0 degC outdoors, 21 degC extract): the supply air leaves at 0.85 x 21 = 17.9 degC plus 0.5 K of fan heat, 2.6 K below the room, and the exhaust air at 3.2 degC, 17.9 K below the room.",
        "Ein kompaktes Wandgerät der Klasse 250 m3/h in der Vorderansicht: ein Gehäuse von 0,6 m Breite und 0,6 m Höhe mit rundem Schauglas von 0,224 m, das das Zuluftlaufrad (neun rückwärts gekrümmte Schaufeln) zeigt, eine Statusleuchte und ein Griff der Filtertür, und vier DN100-Stutzen von 98 mm oben (von links: Fortluft, Abluft, Zuluft, Außenluft) mit je einer Sicke. Das Gerät ist 0,3 m tief (depthM) und wiegt 24 kg; die Gesamthöhe mit den Stutzen beträgt 0,665 m. Der Ursprung ist die Unterkante des Gehäuses an der Wand, x = 0 auf der Mittellinie. Maße und Massen sind aus dieser Produktklasse gerundet (Schätzwert). Auf Stufe 1 beträgt jeder Strom 0,061 m3/s (220 m3/h) und die beiden Ventilatoren nehmen 75 W auf (45 W am Gehäuse, 30 W am Zuluftlaufrad); bei Stufe 0,5 sind es 110 m3/h und 38 W, linearisiert nach einem Ventilatorgesetz mit einer SFP von 0,34 bis 0,38 Wh/m3. Die Wärmerückgewinnung beträgt 85 Prozent (Temperaturverhältnis des Wärmeübertragers, Prüfpunkt nach EN 308 mit 0 degC Außenluft und 21 degC Abluft): die Zuluft verlässt das Gerät mit 0,85 x 21 = 17,9 degC plus 0,5 K Ventilatorwärme, 2,6 K unter der Raumluft, und die Fortluft mit 3,2 degC, 17,9 K unter der Raumluft.",
      ),
    },
    dimensions: { heightM: 0.665, widthM: 0.6, depthM: 0.3, massKg },
    materials: [
      materialOf("painted-steel", "Powder-coated steel sheet", "Pulverbeschichtetes Stahlblech", 0.9, 0.35, 7800, 50, 490, SRC, "incropera-2018"),
      materialOf("galvanised-steel", "Galvanised steel", "Verzinkter Stahl", 0.25, 0.5, 7800, 50, 490, SRC, "incropera-2018"),
      materialOf("polycarbonate", "Polycarbonate", "Polycarbonat", 0.9, 0.1, 1200, 0.2, 1200, SRC, "incropera-2018"),
      materialOf("pp-impeller", "Polypropylene impeller", "Polypropylen-Laufrad", 0.9, 0.5, 900, 0.22, 1900, SRC, "incropera-2018"),
    ],
    rig: { kind: "mechanical", root: "casing", bones: sp.bones.map((b) => ({ ...b, mass: { ...b.mass } })) },
    anchors: [
      { id: "wall-hook", bone: "casing", x: 0, y: 0.57, kind: "support", title: tx("Upper hook on the wall plate", "Oberer Haken an der Wandplatte"), support: "pin", normalDeg: 0, footprintLengthM: 0.03, footprintWidthM: 0.3 },
      { id: "wall-rest", bone: "casing", x: 0, y: 0.03, kind: "support", title: tx("Lower rest against the wall", "Untere Anlage an der Wand"), support: "roller", normalDeg: 0, footprintLengthM: 0.03, footprintWidthM: 0.3, frictionCoefficient: 0.4 },
      { id: "supply-spigot", bone: "spigots", x: 0.075, y: 0.065, kind: "emitter", title: tx("Supply air spigot", "Zuluftstutzen") },
      { id: "exhaust-spigot", bone: "spigots", x: -0.225, y: 0.065, kind: "emitter", title: tx("Exhaust air spigot", "Fortluftstutzen") },
    ],
    representations: { 500: rep500, 200: rep200, 100: rep100, 50: rep50 },
    symbols: [],
    poses,
    clips,
    thermal: {
      environment: { airTempC: 21, meanRadiantTempC: 21, relativeHumidity: 0.5, windMS: 0.1, windDirectionDeg: 0, solarWM2: 0, diffuseFraction: 0, pressurePa: 101325 },
      passengers: [],
      operating: [{ id: "fan", title: tx("Fans", "Ventilatoren"), levels: [level(0, "Off", "Aus"), level(0.5, "Normal (110 m3/h, 38 W)", "Grundlüftung (110 m3/h, 38 W)"), level(1, "Boost (220 m3/h, 75 W)", "Intensiv (220 m3/h, 75 W)")] }],
      regions,
      emitters: [
        { id: "supply-air", kind: "vent", title: tx("Supply air leaving through the supply spigot", "Zuluft am Zuluftstutzen"), anchor: "supply-spigot", directionDeg: 90, diameterM: 0.098, operatingKey: "fan", volumeFlowM3S: 0.061, temperatureRiseK: -2.6, vapourKgPerKg: 0, source: SRC, reference: "en-308-1997" },
        { id: "extract-air", kind: "vent", title: tx("Exhaust air after the heat exchanger leaving to outside", "Fortluft nach dem Wärmeübertrager nach außen"), anchor: "exhaust-spigot", directionDeg: 90, diameterM: 0.098, operatingKey: "fan", volumeFlowM3S: 0.061, temperatureRiseK: -17.9, vapourKgPerKg: -0.0029, source: SRC, reference: "en-308-1997" },
      ],
      assumptions: [
        tx(
          "Heat recovery of 85 percent at the test point of EN 308 (0 degC outdoors, 21 degC extract air): supply air at 17.9 degC plus 0.5 K fan heat, exhaust air at 3.2 degC; the flows and the temperature differences scale with the fan level, the differences to the room air stay as authored at the reference room of 21 degC.",
          "Wärmerückgewinnung von 85 Prozent am Prüfpunkt der EN 308 (0 degC außen, 21 degC Abluft): Zuluft mit 17,9 degC plus 0,5 K Ventilatorwärme, Fortluft mit 3,2 degC; die Ströme und die Temperaturdifferenzen skalieren mit der Ventilatorstufe, die Differenzen zur Raumluft bleiben wie angesetzt für den Bezugsraum von 21 degC.",
        ),
        tx(
          "The 75 W of the two fans end entirely as heat: 45 W at the casing (extract fan, electronics) and 30 W at the supply impeller; the exhaust air is saturated at 3.2 degC (vapour 0.0029 kg/kg below the room air of 21 degC and 50 percent); condensate is drained and its heat is neglected.",
          "Die 75 W der beiden Ventilatoren enden vollständig als Wärme: 45 W am Gehäuse (Abluftventilator, Elektronik) und 30 W am Zuluftlaufrad; die Fortluft ist bei 3,2 degC gesättigt (Dampf 0,0029 kg/kg unter der Raumluft von 21 degC und 50 Prozent); Kondensat wird abgeführt und seine Wärme vernachlässigt.",
        ),
      ],
    },
    structure: {
      gravityMS2: 9.80665,
      dynamics: "quasi-static",
      supports: wall,
      assumptions: [
        tx(
          "The 24 kg unit hangs on the upper hook (pin) and leans against the wall at the lower edge (roller): the hook carries the weight, the pair resists the overturning moment of the centre of mass 0.12 m from the wall.",
          "Das 24 kg schwere Gerät hängt am oberen Haken (Festlager) und lehnt unten an der Wand (Rollenlager): der Haken trägt das Gewicht, das Paar nimmt das Kippmoment des Schwerpunkts 0,12 m vor der Wand auf.",
        ),
      ],
      clinical: false,
    },
    presentation: { defaultPose: "normal", defaultClip: "boost-spin", defaultScale: 50, viewpoints: ["elevation"] },
    exports: ALL_EXPORTS,
    completion: "authored",
  });
};
