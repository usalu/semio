import { ASHRAE, SRC, baseContact, bindOf, clip, contacts, dec, fixed, level, m, materialOf, mkAsset, pose, ref, rep, reg, space2 as space, tx, u500 } from "./g2.mjs";

const sp = space([
  ["pot", null, 0, 0, { title: tx("Pot wall and base", "Topfwand und -boden"), lengthM: 0.13, joint: fixed, mass: m(0.9, 0, 0.03) }],
  ["handles", "pot", 0, 0.1, { title: tx("Side handles", "Seitengriffe"), lengthM: 0.3, joint: fixed, mass: m(0.14, 0, 0.1) }],
  ["water", "pot", 0, 0.005, { title: tx("Water fill", "Wasserfüllung"), lengthM: 0.09, joint: fixed, mass: m(3.0, 0, 0.05) }],
  ["surface", "water", 0, 0.095, { title: tx("Water surface, bobs when boiling", "Wasseroberfläche, wogt beim Kochen"), lengthM: 0.02, joint: { type: "prismatic", axisDeg: 90, minM: -0.004, maxM: 0.004 } }],
]);
const { rect, poly, path } = sp;

const rep50 = rep(50, [20, 75], { body: 0.18, detail: 0.13, texture: null, glazing: null, ground: 0.35 }, [
  rect("water", "water", "body", 3.2, true, 0.45, -0.106, 0.005, 0.212, 0.09, { region: "water", material: "water" }),
  poly("surface", "surface", "detail", 3.3, false, 0.9, [[-0.106, 0.095], [-0.05, 0.097], [0, 0.094], [0.05, 0.097], [0.106, 0.095]], false, { region: "water", material: "water" }),
  rect("wall-left", "pot", "body", 3.5, true, 0.4, -0.11, 0, 0.006, 0.13, { region: "wall", material: "stainless" }),
  rect("wall-right", "pot", "body", 3.5, true, 0.4, 0.104, 0, 0.006, 0.13, { region: "wall", material: "stainless" }),
  rect("base", "pot", "body", 3.5, true, 0.4, -0.11, 0, 0.22, 0.008, { region: "base", material: "stainless" }),
  rect("handle-left", "handles", "body", 3.6, true, 0.2, -0.15, 0.094, 0.042, 0.014, { region: "handle", material: "stainless" }),
  rect("handle-right", "handles", "body", 3.6, true, 0.2, 0.108, 0.094, 0.042, 0.014, { region: "handle", material: "stainless" }),
], dec(
  ["A 0.30 m wide stock pot cut open through its axis: two thin walls, a thick base, water to 90 mm and a handle at each side.", "Ein 0,30 m breiter Kochtopf, durch die Achse aufgeschnitten: zwei dünne Wände, ein dicker Boden, Wasser bis 90 mm und je ein Griff an den Seiten."],
  ["Walls, base, both handles, the water body and its surface line.", "Wände, Boden, beide Griffe, der Wasserkörper und seine Oberflächenlinie."],
  ["Nothing is merged.", "Nichts ist zusammengeführt."],
  ["The rim bead, the lid and the base disc seam.", "Der Randwulst, der Deckel und die Bodenfuge."],
  ["Walls are drawn 6 mm instead of 4 mm so that they print.", "Die Wände sind 6 mm statt 4 mm stark gezeichnet, damit sie drucken."],
  ["Wall, base, handle and water are four regions; the water body warms through, the surface line is the water region too.", "Wand, Boden, Griff und Wasser sind vier Regionen; der Wasserkörper erwärmt sich durch, die Oberflächenlinie gehört ebenfalls zur Wasserregion."],
  { body: 0.18, detail: 0.13, texture: null, glazing: null, ground: 0.35 },
));
const rep100 = rep(100, [75, 150], { body: 0.18, detail: null, texture: null, glazing: null, ground: 0.25 }, [
  rect("water", "water", "body", 3.2, true, 0.45, -0.104, 0.005, 0.208, 0.09, { region: "water", material: "water" }),
  poly("pot", "pot", "body", 3.5, true, 0.4, [[-0.11, 0], [0.11, 0], [0.11, 0.13], [0.102, 0.13], [0.102, 0.008], [-0.102, 0.008], [-0.102, 0.13], [-0.11, 0.13]], true, { regions: ["wall", "base"], material: "stainless" }),
  rect("handle-left", "handles", "body", 3.6, true, 0.2, -0.15, 0.092, 0.044, 0.016, { region: "handle", material: "stainless" }),
  rect("handle-right", "handles", "body", 3.6, true, 0.2, 0.106, 0.092, 0.044, 0.016, { region: "handle", material: "stainless" }),
], dec(
  ["A 3 mm wide cut pot: one U-shaped wall, the water block and two handle stubs.", "Ein 3 mm breiter aufgeschnittener Topf: eine U-förmige Wand, der Wasserblock und zwei Griffstummel."],
  ["The U-shaped wall, water block and handles.", "Die U-förmige Wand, der Wasserblock und die Griffe."],
  ["Wall and base are one U (regions wall and base).", "Wand und Boden sind ein U (Regionen Wand und Boden)."],
  ["Surface line and rim bead.", "Oberflächenlinie und Randwulst."],
  ["Walls are 8 mm (0.08 mm) in the U, drawn with the 0.18 mm pen.", "Die Wände sind 8 mm im U, mit dem 0,18-mm-Stift gezeichnet."],
  ["The U carries wall and base as merged regions; water and handles stay separate.", "Das U trägt Wand und Boden als zusammengeführte Regionen; Wasser und Griffe bleiben getrennt."],
  { body: 0.18, detail: null, texture: null, glazing: null, ground: 0.25 },
));
const rep200 = rep(200, [150, 350], { body: 0.13, detail: null, texture: null, glazing: null, ground: 0.25 }, [
  rect("water", "water", "body", 3.2, true, 0.45, -0.1, 0.01, 0.2, 0.085, { region: "water", material: "water" }),
  poly("pot", "pot", "body", 3.5, true, 0.4, [[-0.115, 0], [0.115, 0], [0.115, 0.13], [0.1, 0.13], [0.1, 0.012], [-0.1, 0.012], [-0.1, 0.13], [-0.115, 0.13]], true, { regions: ["wall", "base"], material: "stainless" }),
  rect("handles", "handles", "body", 3.6, true, 0.2, -0.15, 0.094, 0.3, 0.04, { region: "handle", material: "stainless" }),
], dec(
  ["A 1.5 mm wide pot with a bar across the rim for both handles.", "Ein 1,5 mm breiter Topf mit einem Balken über dem Rand für beide Griffe."],
  ["U-shaped wall, water block and a handle bar.", "U-förmige Wand, Wasserblock und ein Griffbalken."],
  ["Both handles are one bar through the wall.", "Beide Griffe sind ein Balken durch die Wand."],
  ["Surface line, rim bead and the water gap at the wall.", "Oberflächenlinie, Randwulst und die Wasserlücke an der Wand."],
  ["Walls thickened from 4 to 15 mm, handle bar from 14 to 40 mm.", "Wände von 4 auf 15 mm verdickt, Griffbalken von 14 auf 40 mm."],
  ["Wall and base merged, handles one region, water its own.", "Wand und Boden zusammengeführt, Griffe eine Region, Wasser eine eigene."],
  { body: 0.13, detail: null, texture: null, glazing: null, ground: 0.25 },
));

const regions = [
  reg("wall", ["Pot wall", "Topfwand"], "surface", "operating", "stainless", 0.09, 0.13, "cylinder", 0.0008, 0.00005, 4000, { operating: { key: "heat", heatW: 28 } }),
  reg("base", ["Pot base", "Topfboden"], "heater", "operating", "stainless", 0.038, 0.22, "horizontal-plate-down", 0.005, 0.0001, 6000, { operating: { key: "heat", heatW: 40 } }),
  reg("handle", ["Handles", "Griffe"], "surface", "passive", "stainless", 0.004, 0.04, "cylinder", 0.002, 0.0001, 2500),
  reg("water", ["Water", "Wasser"], "water", "operating", "water", 0.034, 0.2, "horizontal-plate-up", 0.01, 0.0017, 3000, { operating: { key: "heat", heatW: 28 } }, { layerKind: "polymer", source: "illustrative" }),
];
const bind = bindOf({
  wall: { exposedFraction: 0.95, contact: { fraction: 0.05, tempC: 21 } },
  base: { exposedFraction: 0.1, contact: { fraction: 0.9, tempC: 21 } },
  handle: { exposedFraction: 1 },
  water: { exposedFraction: 1 },
});
const sup = ["base-contact"];
const poses = [
  pose("cold", ["Cold", "Kalt"], ["Water of room temperature stands in the pot.", "Raumwarmes Wasser steht im Topf."], {}, sup, "auto", bind({ heat: 0 }, { steam: 0, "steam-plume": 0 })),
  pose("heating", ["Heating", "Aufheizen"], ["The pot stands on a flame: water and wall warm up, a little vapour rises.", "Der Topf steht auf einer Flamme: Wasser und Wand erwärmen sich, etwas Dampf steigt auf."], {}, sup, "auto", bind({ heat: 0.55 }, { steam: 0.2, "steam-plume": 0.3 })),
  pose("boiling", ["Boiling", "Kochend"], ["The water boils at 100 degrees: the surface bobs and a dense steam plume rises.", "Das Wasser kocht bei 100 Grad: die Oberfläche wogt und eine dichte Dampffahne steigt auf."], { surface: { dy: 0.002 } }, sup, "auto", bind({ heat: 1 }, { steam: 1, "steam-plume": 1 })),
];
const clips = [
  clip("heat-to-boil", ["Heat to boil", "Zum Kochen bringen"],
    ["The cold water warms for five minutes until it boils and the steam starts.", "Das kalte Wasser erwärmt sich fünf Minuten lang, bis es kocht und der Dampf einsetzt."],
    300, 1,
    [{ t: 0, pose: "cold", ease: "linear" }, { t: 120, pose: "heating", ease: "linear" }, { t: 240, pose: "boiling", ease: "step" }, { t: 300, pose: "boiling", ease: "step" }],
    contacts(sup, 300),
    [["warm", 0, 120, ["Warming", "Erwärmen"]], ["near", 120, 240, ["Nearly boiling", "Fast kochend"]], ["boil", 240, 300, ["Boiling", "Kochen"]]],
    [[240, "operating", ["Boiling starts", "Sieden beginnt"], "heat"]],
    [["operating:heat", "linear", [[0, 0], [120, 0.55], [240, 1], [300, 1]]], ["emitter:steam", "linear", [[0, 0], [120, 0.2], [240, 1]]], ["emitter:steam-plume", "linear", [[0, 0], [120, 0.3], [240, 1]]]]),
  clip("boil", ["Rolling boil", "Siedendes Wasser"],
    ["The surface bobs up and down while the steam keeps rising.", "Die Oberfläche wogt auf und ab, während der Dampf weiter aufsteigt."],
    1.5, 15,
    [{ t: 0, pose: "boiling", ease: "smooth" }, { t: 0.4, pose: "boiling", ease: "smooth", bones: { surface: { dy: 0.004 } } }, { t: 0.8, pose: "boiling", ease: "smooth", bones: { surface: { dy: -0.002 } } }, { t: 1.15, pose: "boiling", ease: "smooth", bones: { surface: { dy: 0.003 } } }, { t: 1.5, pose: "boiling", ease: "smooth" }],
    contacts(sup, 1.5),
    [["bob", 0, 1.5, ["Bobbing", "Wogen"]]],
    [],
    [["emitter:steam-plume", "smooth", [[0, 1], [0.4, 1], [0.8, 0.85], [1.15, 1], [1.5, 1]]]]),
];

export const pot = () =>
  mkAsset({
    id: "cooking-pot",
    category: "objects",
    title: ["Stock Pot with Boiling Water", "Kochtopf mit kochendem Wasser"],
    summary: [
      "A 0.22 m stainless stock pot cut open through its axis with 3 L of water that warms on a flame and boils: a steam plume of vapour rises from the bobbing surface.",
      "Ein 0,22 m großer Edelstahl-Kochtopf, durch die Achse aufgeschnitten, mit 3 l Wasser, das auf einer Flamme warm wird und kocht: aus der wogenden Oberfläche steigt eine Dampffahne auf.",
    ],
    tags: ["pot", "cooking", "water", "steam", "boiling", "latent-heat", "kitchen", "plume"],
    references: [ASHRAE, ref("cengel-2015", "Cengel, Y. A., Boles, M. A. (2015). Thermodynamics: An Engineering Approach, 8th ed., McGraw-Hill (heating water at 4.19 kJ/(kg K), latent heat of vaporisation 2257 kJ/kg at 100 degC).")],
    notes: [
      "A stainless stock pot of 0.22 m inner width and 0.13 m height with 0.8 mm walls, a 5 mm sandwich base and two side handles reaching to 0.30 m overall, filled with 3 L of water to 90 mm. It is cut open through its axis so that the water, the hot base and the wall read in the thermal view. Pot and handles are 1.04 kg, the water 3.0 kg; the origin is the centre of the base on the hob. Heating 1 kg of water by 80 K takes 335 kJ and evaporating it 2257 kJ (Cengel). The net heating of the water surface node (38 W) is an illustrative figure chosen so that the surface reaches the boiling point at steady state; the burner power itself is carried by the hob asset.",
      "Ein Edelstahl-Kochtopf von 0,22 m Innenweite und 0,13 m Höhe mit 0,8 mm Wänden, einem 5 mm starken Sandwichboden und zwei Seitengriffen bis 0,30 m Gesamtbreite, mit 3 l Wasser bis 90 mm gefüllt. Er ist durch die Achse aufgeschnitten, damit Wasser, heißer Boden und Wand in der Thermalansicht lesbar sind. Topf und Griffe wiegen 1,04 kg, das Wasser 3,0 kg; der Ursprung ist die Mitte des Bodens auf dem Kochfeld. Die Erwärmung von 1 kg Wasser um 80 K braucht 335 kJ, das Verdampfen 2257 kJ (Cengel). Die Nettoerwärmung des Wasseroberflächenknotens (38 W) ist ein Anschauungswert, der so gewählt ist, dass die Oberfläche im stationären Zustand den Siedepunkt erreicht; die Brennerleistung selbst trägt das Kochfeld-Asset.",
    ],
    dims: [0.13, 0.3, 0.22],
    materials: [
      materialOf("stainless", "Stainless steel", "Edelstahl", 0.3, 0.5, 7900, 16, 500, "literature", "incropera-2018"),
      materialOf("water", "Water", "Wasser", 0.96, 0.1, 1000, 0.6, 4190, "literature", "incropera-2018"),
    ],
    root: "pot",
    bones: sp.bones,
    anchors: [baseContact("pot", 0, 0, 0.2, 0.2), sp.anchor("steam-top", "surface", 0, 0.11, "emitter", ["Above the water surface", "Über der Wasseroberfläche"])],
    reps: { 500: u500("the 4 mm pot wall", "die 4 mm starke Topfwand", 0.004, 0.6), 200: rep200, 100: rep100, 50: rep50 },
    poses,
    clips,
    operating: [{ id: "heat", title: tx("Heat from the hob", "Wärme vom Kochfeld"), levels: [level(0, "Cold", "Kalt"), level(0.55, "Heating", "Aufheizen"), level(1, "Boiling", "Kochend")] }],
    regions,
    emitters: [
      { id: "steam", kind: "vent", title: tx("Steam leaving the water surface", "Dampf von der Wasseroberfläche"), anchor: "steam-top", directionDeg: 90, diameterM: 0.2, operatingKey: "heat", volumeFlowM3S: 0.004, temperatureRiseK: 75, vapourKgPerKg: 0.3, source: SRC, reference: "ashrae-2021" },
      { id: "steam-plume", kind: "plume", title: tx("Plume above the boiling water", "Fahne über dem kochenden Wasser"), anchor: "steam-top", directionDeg: 90, diameterM: 0.2, region: "water", source: SRC, reference: "ashrae-2021" },
    ],
    assumptions: [
      ["The burner heat is not part of this asset: the 38 W net heating of the water node is illustrative, the steam emitter (0.004 m3/s at +75 K with 0.3 kg/kg vapour) carries the latent share.", "Die Brennerwärme gehört nicht zu diesem Asset: die 38 W Nettoerwärmung des Wasserknotens sind ein Anschauungswert, der Dampfemitter (0,004 m3/s bei +75 K mit 0,3 kg/kg Dampf) trägt den latenten Anteil."],
    ],
    structure: [["The 4 kg filled pot stands on its base: a unilateral contact of 0.2 m diameter carries the weight.", "Der 4 kg schwere gefüllte Topf steht auf seinem Boden: ein einseitiger Kontakt von 0,2 m Durchmesser trägt das Gewicht."]],
    defaultPose: "boiling",
    defaultClip: "boil",
  });
