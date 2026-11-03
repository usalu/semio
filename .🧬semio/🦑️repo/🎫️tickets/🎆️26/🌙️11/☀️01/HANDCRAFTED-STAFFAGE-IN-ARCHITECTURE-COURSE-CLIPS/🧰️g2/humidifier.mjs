import { ASHRAE, SRC, VDI_3803, baseContact, bindOf, clip, contacts, dec, fixed, level, m, materialOf, mkAsset, pose, ref, rep, reg, space2 as space, tx, un } from "./g2.mjs";

const sp = space([
  ["casing", null, 0, 0, { title: tx("Casing with sump", "Gehäuse mit Wanne"), lengthM: 0.6, joint: fixed, mass: m(4, 0, 0.3) }],
  ["pad", "casing", 0, 0.08, { title: tx("Cellulose evaporation pad", "Cellulose-Verdunstungsmatte"), lengthM: 0.4, joint: fixed, mass: m(3, 0, 0.29) }],
  ["trough", "casing", 0, 0.48, { title: tx("Water distribution trough", "Wasserverteilrinne"), lengthM: 0.4, joint: fixed, mass: m(0.8, 0, 0.51) }],
  ["water", "casing", 0, 0.02, { title: tx("Sump water", "Wannenwasser"), lengthM: 0.44, joint: fixed, mass: m(6, 0, 0.05) }],
  ["drip", "trough", 0, 0.5, { title: tx("Falling drops", "Fallende Tropfen"), lengthM: 0.02, joint: { type: "prismatic", axisDeg: 90, minM: -0.38, maxM: 0 } }],
]);
const { rect, ellipse } = sp;
const HATCH = [{ id: "flutes", title: tx("Cellulose flutes", "Cellulosewellen"), pattern: "lines", angleDeg: 90, spacingMm: 0.5, penMm: 0.1 }];
const drops = (r) => [-0.1, 0, 0.1].map((x, i) => ellipse(`drop-${i + 1}`, "drip", "body", 3.9, true, 0.85, x, 0.5, r, r * 1.3, { region: "pad", material: "water" }));

const rep50 = { ...rep(50, [20, 75], { body: 0.18, detail: 0.13, texture: 0.1, glazing: null, ground: 0.35 }, [
  rect("water", "water", "body", 3.2, true, 0.45, -0.22, 0.02, 0.44, 0.05, { region: "sump", material: "water" }),
  rect("pad", "pad", "body", 3.3, true, 0.7, -0.2, 0.08, 0.4, 0.4, { region: "pad", material: "cellulose", hatch: "flutes" }),
  ...drops(0.01),
  rect("trough", "trough", "body", 3.5, true, 0.4, -0.21, 0.48, 0.42, 0.05, { region: "trough", material: "pp-plastic" }),
  rect("casing", "casing", "body", 3.1, false, 0.3, -0.25, 0, 0.5, 0.6, { region: "casing", material: "galvanised-steel" }),
  rect("sump-tray", "casing", "body", 3.4, true, 0.3, -0.24, 0, 0.48, 0.02, { region: "casing", material: "galvanised-steel" }),
], dec(
  ["A 0.5 m by 0.6 m evaporative humidifier cut open, 10 mm by 12 mm on paper: a casing outline around a fluted cellulose pad, a distribution trough above it with three falling drops and a water sump below.", "Ein aufgeschnittener Verdunstungsbefeuchter von 0,5 m mal 0,6 m, auf Papier 10 mm mal 12 mm: eine Gehäusekontur um eine gewellte Cellulosematte, darüber eine Verteilrinne mit drei fallenden Tropfen und unten eine Wasserwanne."],
  ["Casing outline, sump tray and water, pad with its flute hatch, trough and three drops.", "Gehäusekontur, Wannenblech und Wasser, Matte mit Wellenschraffur, Rinne und drei Tropfen."],
  ["Nothing is merged.", "Nichts ist zusammengeführt."],
  ["The pump and its hose, the drift eliminator and the float valve.", "Die Pumpe mit Schlauch, der Tropfenabscheider und das Schwimmerventil."],
  ["The 5 mm flutes of the pad are a 0.5 mm hatch (25 mm real pitch); the drops are 20 mm across.", "Die 5-mm-Wellen der Matte sind eine 0,5-mm-Schraffur (25 mm reale Teilung); die Tropfen sind 20 mm groß."],
  ["Casing, pad, trough and sump are four regions: the wet pad cools well below the air, the sump water follows, the drops move with the drip bone.", "Gehäuse, Matte, Rinne und Wanne sind vier Regionen: die nasse Matte kühlt weit unter die Lufttemperatur, das Wannenwasser folgt, die Tropfen bewegen sich mit dem Tropfenknochen."],
  { body: 0.18, detail: 0.13, texture: 0.1, glazing: null, ground: 0.35 },
)), hatches: HATCH };

const rep100 = { ...rep(100, [75, 150], { body: 0.18, detail: null, texture: 0.1, glazing: null, ground: 0.25 }, [
  rect("water", "water", "body", 3.2, true, 0.45, -0.22, 0.02, 0.44, 0.05, { region: "sump", material: "water" }),
  rect("pad", "pad", "body", 3.3, true, 0.7, -0.2, 0.08, 0.4, 0.4, { region: "pad", material: "cellulose", hatch: "flutes" }),
  ...drops(0.014),
  rect("trough", "trough", "body", 3.5, true, 0.4, -0.21, 0.48, 0.42, 0.05, { region: "trough", material: "pp-plastic" }),
  rect("casing", "casing", "body", 3.1, false, 0.3, -0.25, 0, 0.5, 0.6, { region: "casing", material: "galvanised-steel" }),
], dec(
  ["A 5 mm by 6 mm humidifier: casing outline, hatched pad, trough, water and three drops.", "Ein 5 mm mal 6 mm großer Befeuchter: Gehäusekontur, schraffierte Matte, Rinne, Wasser und drei Tropfen."],
  ["Casing, pad with hatch, trough, sump water and drops.", "Gehäuse, Matte mit Schraffur, Rinne, Wannenwasser und Tropfen."],
  ["The sump tray joins the casing outline.", "Das Wannenblech geht in der Gehäusekontur auf."],
  ["Pump, hose and drift eliminator.", "Pumpe, Schlauch und Tropfenabscheider."],
  ["The hatch pitch is 50 mm real, the drops 28 mm.", "Die Schraffur hat 50 mm reale Teilung, die Tropfen 28 mm."],
  ["Same four regions as at 1:50.", "Dieselben vier Regionen wie bei 1:50."],
  { body: 0.18, detail: null, texture: 0.1, glazing: null, ground: 0.25 },
)), hatches: HATCH };

const rep200 = rep(200, [150, 350], { body: 0.13, detail: null, texture: null, glazing: null, ground: 0.25 }, [
  rect("water", "water", "body", 3.2, true, 0.45, -0.22, 0.04, 0.44, 0.05, { region: "sump", material: "water" }),
  rect("pad", "pad", "body", 3.3, true, 0.7, -0.2, 0.1, 0.4, 0.37, { regions: ["pad"], material: "cellulose" }),
  rect("trough", "trough", "body", 3.5, true, 0.4, -0.21, 0.48, 0.42, 0.06, { region: "trough", material: "pp-plastic" }),
  rect("casing", "casing", "body", 3.1, false, 0.3, -0.25, 0, 0.5, 0.6, { region: "casing", material: "galvanised-steel" }),
], dec(
  ["A 2.5 mm by 3 mm humidifier: casing outline, pad, trough and water.", "Ein 2,5 mm mal 3 mm großer Befeuchter: Gehäusekontur, Matte, Rinne und Wasser."],
  ["Casing outline, pad block, trough and sump water.", "Gehäusekontur, Mattenblock, Rinne und Wannenwasser."],
  ["The drops belong to the pad block (they are not drawn).", "Die Tropfen gehören zum Mattenblock (sie werden nicht gezeichnet)."],
  ["Hatch, drops and pump.", "Schraffur, Tropfen und Pumpe."],
  ["Trough and sump are 50 to 60 mm thick.", "Rinne und Wanne sind 50 bis 60 mm dick."],
  ["Same four regions as at 1:50.", "Dieselben vier Regionen wie bei 1:50."],
  { body: 0.13, detail: null, texture: null, glazing: null, ground: 0.25 },
));

const regions = [
  reg("casing", ["Casing", "Gehäuse"], "surface", "passive", "galvanised-steel", 0.9, 0.6, "vertical-plate", 0.001, 0.00003, 3500),
  reg("pad", ["Wet cellulose pad", "Nasse Cellulosematte"], "water", "operating", "cellulose", 1.2, 0.4, "vertical-plate", 0.1, 0.5, 4000, { operating: { key: "water", heatW: -90 } }, { layerKind: "polymer", source: "illustrative" }),
  reg("trough", ["Distribution trough", "Verteilrinne"], "surface", "passive", "pp-plastic", 0.1, 0.42, "horizontal-plate-up", 0.003, 0.03, 1500, {}, { layerKind: "polymer" }),
  reg("sump", ["Sump water", "Wannenwasser"], "water", "operating", "water", 0.2, 0.44, "horizontal-plate-up", 0.05, 0.08, 6000, { operating: { key: "water", heatW: -12 } }, { layerKind: "polymer", source: "illustrative" }),
];
const bind = bindOf({ casing: { exposedFraction: 0.9, contact: { fraction: 0.1, tempC: 21 } }, pad: { exposedFraction: 1 }, trough: { exposedFraction: 1 }, sump: { exposedFraction: 1 } });
const sup = ["base-contact"];
const poses = [
  pose("dry", ["Dry", "Trocken"], ["No water runs: the pad is dry and at room temperature, the drops wait in the trough.", "Es läuft kein Wasser: die Matte ist trocken und hat Raumtemperatur, die Tropfen warten in der Rinne."], {}, sup, "auto", bind({ water: 0 }, { "outlet-air": 0 })),
  pose("wetting", ["Wetting", "Befeuchten"], ["Water flows over the pad and evaporation starts to cool it.", "Wasser läuft über die Matte und die Verdunstung beginnt sie zu kühlen."], { drip: { dy: -0.12 } }, sup, "auto", bind({ water: 0.5 }, { "outlet-air": 0.5 })),
  pose("running", ["Running", "Läuft"], ["The wet pad evaporates water into the passing air: the pad is the coldest part and the air leaves cooler and moister.", "Die nasse Matte verdunstet Wasser in die vorbeiströmende Luft: die Matte ist der kälteste Teil, und die Luft verlässt sie kühler und feuchter."], { drip: { dy: -0.24 } }, sup, "auto", bind({ water: 1 }, { "outlet-air": 1 })),
];
const clips = [
  clip("drip", ["Dripping", "Tropfen"], ["Three drops fall from the trough along the pad.", "Drei Tropfen fallen von der Rinne an der Matte entlang."],
    1.2, 15,
    [{ t: 0, pose: "running", ease: "linear", bones: { drip: { dy: 0 } } }, { t: 1.2, pose: "running", ease: "linear", bones: { drip: { dy: -0.36 } } }],
    contacts(sup, 1.2), [["fall", 0, 1.2, ["Falling", "Fallen"]]], [], []),
  clip("start-wetting", ["Start wetting", "Befeuchten starten"], ["The pump starts and the pad cools within a few minutes.", "Die Pumpe startet und die Matte kühlt sich binnen weniger Minuten ab."],
    300, 1,
    [{ t: 0, pose: "dry", ease: "step" }, { t: 5, pose: "running", ease: "step" }, { t: 300, pose: "running", ease: "step" }],
    contacts(sup, 300), [["dry", 0, 5, ["Dry", "Trocken"]], ["wet", 5, 300, ["Wet and cooling", "Nass und kühlend"]]], [[5, "operating", ["Pump started", "Pumpe gestartet"], "water"]],
    [["operating:water", "smooth", [[0, 0], [5, 0], [6, 1]]], ["emitter:outlet-air", "smooth", [[0, 0], [5, 0], [6, 1]]]]),
];

export const humidifier = () =>
  mkAsset({
    id: "evaporative-humidifier",
    category: "appliances",
    title: ["Evaporative Cooling Humidifier", "Verdunstungsbefeuchter"],
    summary: [
      "A 0.5 m by 0.6 m evaporative humidifier of an air handler, cut open: water drips from a trough over a fluted cellulose pad into a sump, and the evaporating water cools the pad and the passing air.",
      "Ein aufgeschnittener Verdunstungsbefeuchter von 0,5 m mal 0,6 m eines Lüftungsgeräts: Wasser tropft aus einer Rinne über eine gewellte Cellulosematte in eine Wanne, und das verdunstende Wasser kühlt die Matte und die vorbeiströmende Luft.",
    ],
    tags: ["humidifier", "evaporative-cooling", "dec", "pad", "latent-heat", "air-handling", "appliance"],
    references: [VDI_3803, ASHRAE],
    notes: [
      "A pad-type evaporative humidifier of the 1000 m3/h class: a galvanised casing of 0.5 m by 0.6 m, a 0.1 m deep flute cellulose pad of 0.4 m by 0.4 m, a polypropylene distribution trough on top with 0.5 L/min, a stainless sump tray of 6 L under the pad and a recirculation pump (not drawn). Mass 14 kg with the full sump (casing 4, pad 3, trough 0.8, water 6) and a drift-free pad of 3 kg. Adiabatic saturation cools the 32 degC supply air to about 20 degC at 90 percent efficiency; the pad surface stays at the wet-bulb temperature of about 19 degC (heatW -90 W against still room air is an illustrative figure, not a rating). The origin is the centre of the base on the surface.",
      "Ein Matten-Verdunstungsbefeuchter der Klasse 1000 m3/h: ein verzinktes Gehäuse von 0,5 m mal 0,6 m, eine 0,1 m tiefe Cellulose-Wellenmatte von 0,4 m mal 0,4 m, oben eine Polypropylen-Verteilrinne mit 0,5 l/min, unter der Matte eine Edelstahlwanne von 6 l und eine Umwälzpumpe (nicht gezeichnet). Die Masse beträgt 14 kg mit gefüllter Wanne (Gehäuse 4, Matte 3, Rinne 0,8, Wasser 6) und einer tropfenfreien Matte von 3 kg. Die adiabate Sättigung kühlt die 32 degC warme Zuluft bei 90 Prozent Wirkungsgrad auf etwa 20 degC ab; die Mattenoberfläche bleibt auf der Kühlgrenztemperatur von etwa 19 degC (heatW -90 W gegen ruhende Raumluft ist ein Anschauungswert, keine Nennleistung). Der Ursprung ist die Mitte des Fußes auf der Unterlage.",
    ],
    dims: [0.6, 0.5, 0.4],
    materials: [
      materialOf("galvanised-steel", "Galvanised steel", "Verzinkter Stahl", 0.25, 0.5, 7800, 50, 480, "literature", "incropera-2018"),
      materialOf("cellulose", "Fluted cellulose pad", "Cellulose-Wellenmatte", 0.9, 0.6, 150, 0.08, 1500, SRC, "incropera-2018"),
      materialOf("pp-plastic", "Polypropylene", "Polypropylen", 0.9, 0.6, 900, 0.22, 1900, SRC, "incropera-2018"),
      materialOf("water", "Water", "Wasser", 0.96, 0.1, 1000, 0.6, 4190, "literature", "incropera-2018"),
    ],
    root: "casing",
    bones: sp.bones,
    anchors: [baseContact("casing", 0, 0, 0.5, 0.4), sp.anchor("outlet", "casing", 0.25, 0.3, "emitter", ["Air leaving the pad", "Luftaustritt hinter der Matte"])],
    reps: { 500: un(500, "the 0.5 mm drops and the 3 mm trough rim", "die Tropfen und der 3 mm dicke Rinnenrand", 0.003, 1, [200, 100]), 200: rep200, 100: rep100, 50: rep50 },
    poses,
    clips,
    operating: [{ id: "water", title: tx("Pump and evaporation", "Pumpe und Verdunstung"), levels: [level(0, "Dry", "Trocken"), level(0.5, "Wetting", "Befeuchten"), level(1, "Running", "Läuft")] }],
    regions,
    emitters: [{ id: "outlet-air", kind: "vent", title: tx("Cooled moist air leaving the pad", "Gekühlte feuchte Luft hinter der Matte"), anchor: "outlet", directionDeg: 0, diameterM: 0.3, operatingKey: "water", volumeFlowM3S: 0.28, temperatureRiseK: -10, vapourKgPerKg: 0.004, source: SRC, reference: "vdi-3803-2010" }],
    assumptions: [
      ["The pad cooling of -90 W is an illustrative surface heat flux; the air emitter carries the real sensible cooling and the latent gain (0.28 m3/s, -10 K, +4 g/kg).", "Die Mattenkühlung von -90 W ist ein Anschauungswert für den Oberflächenwärmestrom; der Luftemitter trägt die reale fühlbare Abkühlung und den latenten Gewinn (0,28 m3/s, -10 K, +4 g/kg)."],
    ],
    structure: [["The 14 kg humidifier stands on its sump tray: a unilateral contact of 0.5 m by 0.4 m carries the weight.", "Der 14 kg schwere Befeuchter steht auf seiner Wanne: ein einseitiger Kontakt von 0,5 m mal 0,4 m trägt das Gewicht."]],
    defaultPose: "running",
    defaultClip: "drip",
  });
