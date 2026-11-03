import { ASHRAE, EN_14511, SRC, baseContact, bindOf, clip, contacts, dec, fixed, level, m, materialOf, mkAsset, pose, ref, rep, reg, space2 as space, tx, un } from "./g2.mjs";

const sp = space([
  ["frame", null, 0, 0, { title: tx("Headers and end plates", "Sammler und Seitenbleche"), lengthM: 0.52, joint: fixed, mass: m(1.3, 0, 0.26) }],
  ["fins", "frame", 0, 0.02, { title: tx("Aluminium fin pack", "Aluminium-Lamellenpaket"), lengthM: 0.48, joint: fixed, mass: m(2.6, 0, 0.26) }],
  ["tubes", "fins", 0, 0.02, { title: tx("Copper tube rows", "Kupferrohrreihen"), lengthM: 0.48, joint: fixed, mass: m(1.1, 0, 0.26) }],
]);
const { rect, ellipse } = sp;
const COL1 = Array.from({ length: 12 }, (_, i) => [-0.0225, 0.04 + i * 0.04]);
const COL2 = Array.from({ length: 11 }, (_, i) => [0.0225, 0.06 + i * 0.04]);
const tubes = (r, region) => [...COL1, ...COL2].map(([x, y], i) => ellipse(`tube-${i + 1}`, "tubes", "body", 3.6, true, 0.5, x, y, r, r, { region, material: "copper" }));
const FINS = [{ id: "fins", title: tx("Fin pack in section", "Lamellenpaket im Schnitt"), pattern: "lines", angleDeg: 0, spacingMm: 0.5, penMm: 0.1 }];

const rep50 = { ...rep(50, [20, 75], { body: 0.18, detail: 0.13, texture: 0.1, glazing: null, ground: 0.35 }, [
  rect("fin-pack", "fins", "body", 3.3, true, 0.7, -0.05, 0.02, 0.1, 0.48, { region: "fins", material: "aluminium-fin", hatch: "fins" }),
  ...tubes(0.009, "tubes"),
  rect("header-bottom", "frame", "body", 3.5, true, 0.4, -0.075, 0, 0.15, 0.02, { region: "headers", material: "copper", r: 0.01 }),
  rect("header-top", "frame", "body", 3.5, true, 0.4, -0.075, 0.5, 0.15, 0.02, { region: "headers", material: "copper", r: 0.01 }),
  rect("inlet-stub", "frame", "body", 3.4, true, 0.4, 0.075, 0.504, 0.055, 0.012, { region: "headers", material: "copper" }),
  rect("outlet-stub", "frame", "body", 3.4, true, 0.4, 0.075, 0.004, 0.055, 0.012, { region: "headers", material: "copper" }),
], dec(
  ["A 0.52 m tall finned coil cut across its tubes, 10 mm by 10 mm on paper: a fin pack with 23 staggered tube circles between a bottom and a top header, a connection stub at each header.", "Ein 0,52 m hoher Lamellenregister im Schnitt quer zu den Rohren, auf Papier 10 mm mal 10 mm: ein Lamellenpaket mit 23 versetzten Rohrkreisen zwischen einem unteren und einem oberen Sammler und je einem Anschlussstutzen am Sammler."],
  ["The fin pack as hatched block, every one of the 23 tubes in two staggered columns, both headers and both stubs.", "Das Lamellenpaket als schraffierter Block, alle 23 Rohre in zwei versetzten Spalten, beide Sammler und beide Stutzen."],
  ["Nothing is merged: the fins are shown by a hatch.", "Nichts ist zusammengeführt: die Lamellen zeigt eine Schraffur."],
  ["The end plates, the return bends and the fin collars.", "Die Seitenbleche, die Rohrbögen und die Lamellenkragen."],
  ["The 0.25 mm real fins (about 3 mm pitch) are drawn as a 0.5 mm hatch, 25 mm real pitch, so that they print; the tubes are 18 mm across.", "Die realen 0,25 mm dicken Lamellen (rund 3 mm Teilung) sind als 0,5-mm-Schraffur mit 25 mm realer Teilung gezeichnet, damit sie drucken; die Rohre sind 18 mm weit."],
  ["Fins, tubes and headers are three regions: in the thermal view the whole pack warms (hot coil) or cools (cold coil), the tubes first.", "Lamellen, Rohre und Sammler sind drei Regionen: in der Thermalansicht erwärmt sich das ganze Paket (heißer Register) oder kühlt ab (kalter Register), die Rohre zuerst."],
  { body: 0.18, detail: 0.13, texture: 0.1, glazing: null, ground: 0.35 },
)), hatches: FINS };

const rep100 = { ...rep(100, [75, 150], { body: 0.18, detail: null, texture: 0.1, glazing: null, ground: 0.25 }, [
  rect("fin-pack", "fins", "body", 3.3, true, 0.7, -0.05, 0.02, 0.1, 0.48, { region: "fins", material: "aluminium-fin", hatch: "fins" }),
  ...tubes(0.011, "tubes").filter((_, i) => i % 1 === 0),
  rect("header-bottom", "frame", "body", 3.5, true, 0.4, -0.075, 0, 0.15, 0.025, { region: "headers", material: "copper" }),
  rect("header-top", "frame", "body", 3.5, true, 0.4, -0.075, 0.495, 0.15, 0.025, { region: "headers", material: "copper" }),
  rect("inlet-stub", "frame", "body", 3.4, true, 0.4, 0.075, 0.5, 0.055, 0.02, { region: "headers", material: "copper" }),
  rect("outlet-stub", "frame", "body", 3.4, true, 0.4, 0.075, 0, 0.055, 0.02, { region: "headers", material: "copper" }),
], dec(
  ["A 5 mm tall coil: a hatched fin block with 23 tube dots, two headers and two stubs.", "Ein 5 mm hoher Register: ein schraffierter Lamellenblock mit 23 Rohrpunkten, zwei Sammlern und zwei Stutzen."],
  ["Fin block, all tubes, headers and stubs.", "Lamellenblock, alle Rohre, Sammler und Stutzen."],
  ["Nothing is merged.", "Nichts ist zusammengeführt."],
  ["Return bends and end plates.", "Rohrbögen und Seitenbleche."],
  ["Tubes drawn 22 mm across, headers 25 mm; the hatch is 50 mm real pitch.", "Rohre 22 mm weit gezeichnet, Sammler 25 mm; die Schraffur hat 50 mm reale Teilung."],
  ["Same three regions as at 1:50.", "Dieselben drei Regionen wie bei 1:50."],
  { body: 0.18, detail: null, texture: 0.1, glazing: null, ground: 0.25 },
)), hatches: FINS };

const rep200 = rep(200, [150, 350], { body: 0.13, detail: null, texture: null, glazing: null, ground: 0.25 }, [
  rect("fin-pack", "fins", "body", 3.3, true, 0.6, -0.05, 0.03, 0.1, 0.46, { regions: ["fins", "tubes"], material: "aluminium-fin" }),
  rect("header-bottom", "frame", "body", 3.5, true, 0.4, -0.075, 0, 0.15, 0.04, { region: "headers", material: "copper" }),
  rect("header-top", "frame", "body", 3.5, true, 0.4, -0.075, 0.48, 0.15, 0.04, { region: "headers", material: "copper" }),
  rect("inlet-stub", "frame", "body", 3.4, true, 0.4, 0.075, 0.49, 0.06, 0.03, { region: "headers", material: "copper" }),
  rect("outlet-stub", "frame", "body", 3.4, true, 0.4, 0.075, 0, 0.06, 0.03, { region: "headers", material: "copper" }),
], dec(
  ["A 2.6 mm tall coil: one solid block between two headers with two stubs.", "Ein 2,6 mm hoher Register: ein einziger Block zwischen zwei Sammlern mit zwei Stutzen."],
  ["Fin block, headers and stubs.", "Lamellenblock, Sammler und Stutzen."],
  ["Fins and the 23 tubes are one block (regions fins and tubes).", "Lamellen und die 23 Rohre sind ein Block (Regionen Lamellen und Rohre)."],
  ["All tube circles, hatch and return bends.", "Alle Rohrkreise, Schraffur und Rohrbögen."],
  ["Headers thickened from 20 to 40 mm, stubs from 12 to 30 mm.", "Sammler von 20 auf 40 mm verdickt, Stutzen von 12 auf 30 mm."],
  ["The block lists fins and tubes as merged regions.", "Der Block führt Lamellen und Rohre als zusammengeführte Regionen."],
  { body: 0.13, detail: null, texture: null, glazing: null, ground: 0.25 },
));

const regions = [
  reg("fins", ["Aluminium fins", "Aluminium-Lamellen"], "surface", "operating", "aluminium-fin", 2.0, 0.1, "vertical-plate", 0.0002, 0.000002, 2500, { operating: { key: "duty", heatW: 500 } }),
  reg("tubes", ["Copper tubes", "Kupferrohre"], "heater", "operating", "copper", 0.08, 0.018, "cylinder", 0.0007, 0.00001, 3500, { operating: { key: "duty", heatW: 24 } }),
  reg("headers", ["Headers and stubs", "Sammler und Stutzen"], "surface", "operating", "copper", 0.012, 0.15, "cylinder", 0.001, 0.00001, 1500, { operating: { key: "duty", heatW: 3 } }),
];
const bind = bindOf({ fins: { exposedFraction: 1 }, tubes: { exposedFraction: 1 }, headers: { exposedFraction: 1 } });
const sup = ["base-contact"];
const poses = [
  pose("cold", ["Cold coil", "Kalter Register"], ["Evaporating refrigerant or chilled water at about -5 degC cools tubes and fins: the coil is an evaporator or cooling coil.", "Verdampfendes Kältemittel oder Kaltwasser von etwa -5 degC kühlt Rohre und Lamellen: der Register ist ein Verdampfer oder Kühler."], {}, sup, "auto", bind({ duty: -1 })),
  pose("idle", ["Idle", "Ruhend"], ["No flow: the coil is at room temperature.", "Kein Durchfluss: der Register hat Raumtemperatur."], {}, sup, "auto", bind({ duty: 0 })),
  pose("hot", ["Hot coil", "Heißer Register"], ["Hot gas or hot water at about 55 degC warms tubes and fins: the coil is a condenser or heating coil.", "Heißgas oder Heißwasser von etwa 55 degC erwärmt Rohre und Lamellen: der Register ist ein Verflüssiger oder Erhitzer."], {}, sup, "auto", bind({ duty: 1 })),
];
const clips = [
  clip("reverse", ["Reversal", "Umkehr"], ["The coil starts cold, passes through idle and ends hot, as the roles of a reversible heat pump swap.", "Der Register startet kalt, geht über den Ruhezustand und endet heiß, wenn eine reversible Wärmepumpe die Rollen tauscht."],
    60, 2,
    [{ t: 0, pose: "cold", ease: "linear" }, { t: 20, pose: "idle", ease: "linear" }, { t: 40, pose: "hot", ease: "linear" }, { t: 60, pose: "hot", ease: "linear" }],
    contacts(sup, 60), [["cold", 0, 20, ["Cold", "Kalt"]], ["swap", 20, 40, ["Swapping", "Wechsel"]], ["hot", 40, 60, ["Hot", "Heiß"]]], [], []),
  clip("warm-up", ["Warm up", "Aufwärmen"], ["The idle coil heats within a minute when hot water starts flowing.", "Der ruhende Register heizt sich innerhalb einer Minute auf, wenn heißes Wasser zu fließen beginnt."],
    60, 2,
    [{ t: 0, pose: "idle", ease: "step" }, { t: 5, pose: "hot", ease: "step" }, { t: 60, pose: "hot", ease: "step" }],
    contacts(sup, 60), [["idle", 0, 5, ["Idle", "Ruhend"]], ["hot", 5, 60, ["Heating", "Heizen"]]], [[5, "operating", ["Flow starts", "Durchfluss beginnt"], "duty"]], []),
];

export const coil = () =>
  mkAsset({
    id: "finned-heat-exchanger",
    category: "machinery",
    title: ["Finned Tube Coil", "Lamellenrohr-Register"],
    summary: [
      "A 0.52 m finned copper-aluminium coil cut across its tubes: cold as evaporator or cooling coil, hot as condenser or heating coil, with headers and stubs.",
      "Ein 0,52 m hoher Kupfer-Aluminium-Lamellenregister im Schnitt quer zu den Rohren: kalt als Verdampfer oder Kühler, heiß als Verflüssiger oder Erhitzer, mit Sammlern und Stutzen.",
    ],
    tags: ["coil", "heat-exchanger", "evaporator", "condenser", "heating-coil", "cooling-coil", "fins", "machinery"],
    references: [EN_14511, ASHRAE, ref("vdi-3803-2010-coil", "VDI 3803 Part 1:2010 Air-conditioning - Central air-handling units (heating and cooling coils, fin pitch 2 to 4 mm).")],
    notes: [
      "A finned tube coil of a small heat pump or air handler: 23 copper tubes of 9.5 mm in two staggered columns (45 mm pitch rows, 40 mm tube pitch) through 0.2 mm aluminium fins at 3 mm pitch, 0.1 m deep and 0.48 m high, between a bottom and a top header of 0.15 m, each with a connection stub. The drawing is the section across the tubes, so the fins appear as a hatch and the tubes as circles. Mass 5.0 kg (fins 2.6, tubes 1.1, headers and end plates 1.3). The duty key runs from -1 (cold: evaporating at about -5 degC or chilled water) over 0 to +1 (hot: condensing at about 55 degC or hot water); the heatW values (fins 500 W, tubes 24 W, headers 3 W) are authored estimates of the heat the surfaces exchange with 21 degC room air, so the fins reach about -9 and +51 degC. The same asset stands for the evaporator and the condenser of the heat pump and for the heating coil of an air handler. The origin is the centre of the bottom header on the base line.",
      "Ein Lamellenregister einer kleinen Wärmepumpe oder eines Lüftungsgeräts: 23 Kupferrohre von 9,5 mm in zwei versetzten Spalten (Reihenabstand 45 mm, Rohrteilung 40 mm) durch 0,2 mm dicke Aluminiumlamellen im Abstand von 3 mm, 0,1 m tief und 0,48 m hoch, zwischen einem unteren und einem oberen Sammler von 0,15 m mit je einem Anschlussstutzen. Die Zeichnung ist der Schnitt quer zu den Rohren, daher erscheinen die Lamellen als Schraffur und die Rohre als Kreise. Masse 5,0 kg (Lamellen 2,6, Rohre 1,1, Sammler und Seitenbleche 1,3). Der Schlüssel duty läuft von -1 (kalt: Verdampfen bei etwa -5 degC oder Kaltwasser) über 0 bis +1 (heiß: Kondensieren bei etwa 55 degC oder Heißwasser); die heatW-Werte (Lamellen 500 W, Rohre 24 W, Sammler 3 W) sind Schätzwerte der Wärme, die die Oberflächen mit 21 degC warmer Raumluft austauschen, sodass die Lamellen etwa -9 und +51 degC erreichen. Dasselbe Asset steht für Verdampfer und Verflüssiger der Wärmepumpe und für den Erhitzer eines Lüftungsgeräts. Der Ursprung ist die Mitte des unteren Sammlers auf der Grundlinie.",
    ],
    dims: [0.52, 0.205, 0.5],
    materials: [
      materialOf("aluminium-fin", "Aluminium fins", "Aluminium-Lamellen", 0.1, 0.3, 2700, 237, 900, "literature", "incropera-2018"),
      materialOf("copper", "Copper tube", "Kupferrohr", 0.05, 0.3, 8900, 390, 385, "literature", "incropera-2018"),
    ],
    root: "frame",
    bones: sp.bones,
    anchors: [baseContact("frame", 0, 0, 0.15, 0.4)],
    reps: { 500: un(500, "the 0.2 mm fins and the 18 mm tubes", "die 0,2 mm dicken Lamellen und die 18 mm dicken Rohre", 0.0002, 1, [200, 100]), 200: rep200, 100: rep100, 50: rep50 },
    poses,
    clips,
    operating: [{ id: "duty", title: tx("Coil duty", "Registerleistung"), levels: [level(-1, "Cold (evaporator or cooling coil)", "Kalt (Verdampfer oder Kühler)"), level(0, "Idle", "Ruhend"), level(1, "Hot (condenser or heating coil)", "Heiß (Verflüssiger oder Erhitzer)")] }],
    regions,
    emitters: [],
    assumptions: [
      ["The air flowing through the coil is not part of this asset; the heatW values give only the surface temperatures against still room air, the heat of the coil itself (kilowatts) is carried by the cycle drawing.", "Die durch den Register strömende Luft gehört nicht zu diesem Asset; die heatW-Werte liefern nur die Oberflächentemperaturen gegen ruhende Raumluft, die Wärme des Registers selbst (Kilowatt) trägt die Kreislaufzeichnung."],
    ],
    structure: [["The 5 kg coil stands on its bottom header: a unilateral contact of 0.15 m by 0.4 m carries the weight.", "Der 5 kg schwere Register steht auf seinem unteren Sammler: ein einseitiger Kontakt von 0,15 m mal 0,4 m trägt das Gewicht."]],
    defaultPose: "hot",
    defaultClip: "reverse",
  });
