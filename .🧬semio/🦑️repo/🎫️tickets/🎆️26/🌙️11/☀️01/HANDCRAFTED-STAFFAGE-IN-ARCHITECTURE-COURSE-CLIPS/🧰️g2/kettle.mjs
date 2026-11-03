import { ASHRAE, SRC, baseContact, bindOf, clip, contacts, dec, fixed, level, m, materialOf, mkAsset, pose, ref, rep, reg, space2 as space, tx, u500 } from "./g2.mjs";

const sp = space([
  ["base", null, 0, 0, { title: tx("Power base with heating plate", "Sockel mit Heizplatte"), lengthM: 0.18, joint: fixed, mass: m(0.55, 0, 0.015) }],
  ["body", "base", 0, 0.03, { title: tx("Jug body with spout", "Kannenkörper mit Ausguss"), lengthM: 0.19, joint: fixed, mass: m(0.45, 0, 0.12) }],
  ["handle", "body", 0.098, 0.12, { title: tx("Handle with switch", "Griff mit Schalter"), lengthM: 0.1, joint: fixed, mass: m(0.12, 0.098, 0.12) }],
  ["water", "body", 0, 0.04, { title: tx("Water fill", "Wasserfüllung"), lengthM: 0.13, joint: fixed, mass: m(1.7, 0, 0.1) }],
  ["lid", "body", 0, 0.205, { title: tx("Lid, rattles when boiling", "Deckel, klappert beim Kochen"), lengthM: 0.03, joint: { type: "prismatic", axisDeg: 90, minM: 0, maxM: 0.005 }, mass: m(0.1, 0, 0.215) }],
]);
const { rect, poly, ellipse } = sp;
const BODY = [[-0.075, 0.03], [0.075, 0.03], [0.07, 0.205], [-0.07, 0.205]];
const SPOUT = [[-0.07, 0.17], [-0.116, 0.222], [-0.07, 0.205]];
const HANDLE = [[0.07, 0.2], [0.12, 0.195], [0.125, 0.07], [0.073, 0.045], [0.073, 0.068], [0.108, 0.085], [0.105, 0.178], [0.07, 0.18]];

const rep50 = rep(50, [20, 75], { body: 0.18, detail: 0.13, texture: null, glazing: 0.13, ground: 0.35 }, [
  rect("water", "water", "body", 3.1, true, 0.45, -0.07, 0.04, 0.14, 0.125, { region: "window", material: "water" }),
  poly("spout", "body", "body", 3.3, true, 0.4, SPOUT, true, { region: "shell", material: "stainless" }),
  poly("body", "body", "body", 3.4, false, 0.4, BODY, true, { region: "shell", material: "stainless" }),
  rect("gauge", "body", "glazing", 3.5, true, 0.7, -0.014, 0.05, 0.028, 0.12, { region: "window", material: "water" }),
  poly("handle", "handle", "body", 3.5, true, 0.2, HANDLE, true, { region: "handle", material: "pp-plastic" }),
  rect("lid", "lid", "body", 3.6, true, 0.45, -0.066, 0.205, 0.132, 0.016, { region: "lid", material: "stainless" }),
  rect("lid-button", "lid", "body", 3.7, true, 0.2, -0.012, 0.221, 0.024, 0.016, { region: "lid", material: "pp-plastic" }),
  rect("base", "base", "body", 3.6, true, 0.5, -0.09, 0, 0.18, 0.03, { region: "plate", material: "pp-plastic" }),
], dec(
  ["A 0.24 m wide jug kettle on its power base, 12 mm by 12 mm on paper: a tapered body with a spout at the left, a C-shaped handle at the right, a domed lid with a button and a water gauge strip.", "Ein 0,24 m breiter Wasserkocher auf seinem Sockel, auf Papier 12 mm mal 12 mm: ein sich verjüngender Körper mit Ausguss links, einem C-förmigen Griff rechts, einem gewölbten Deckel mit Knopf und einem Wasserstandsstreifen."],
  ["Base, tapered body, spout, handle, lid with button, the gauge window and the water behind it.", "Sockel, sich verjüngender Körper, Ausguss, Griff, Deckel mit Knopf, das Sichtfenster und das Wasser dahinter."],
  ["Nothing is merged.", "Nichts ist zusammengeführt."],
  ["The switch, the limescale filter and the cord.", "Der Schalter, der Kalkfilter und das Kabel."],
  ["The gauge strip is 28 mm wide so that it prints; the handle neck is 35 mm.", "Der Wasserstandsstreifen ist 28 mm breit, damit er druckt; der Griffhals ist 35 mm."],
  ["Plate, shell, handle, lid and window are five regions: in the thermal view the base plate and the water window glow while the plastic handle stays cool.", "Platte, Hülle, Griff, Deckel und Fenster sind fünf Regionen: in der Thermalansicht glühen Heizplatte und Wasserfenster, während der Kunststoffgriff kühl bleibt."],
  { body: 0.18, detail: 0.13, texture: null, glazing: 0.13, ground: 0.35 },
));
const rep100 = rep(100, [75, 150], { body: 0.18, detail: null, texture: null, glazing: null, ground: 0.25 }, [
  rect("water", "water", "body", 3.1, true, 0.45, -0.065, 0.04, 0.13, 0.125, { region: "window", material: "water" }),
  poly("body", "body", "body", 3.4, true, 0.4, [[-0.075, 0.03], [0.075, 0.03], [0.07, 0.205], [-0.07, 0.205], [-0.07, 0.17], [-0.116, 0.222], [-0.07, 0.205]], true, { regions: ["shell", "window"], material: "stainless" }),
  poly("handle", "handle", "body", 3.5, true, 0.2, HANDLE, true, { region: "handle", material: "pp-plastic" }),
  rect("lid", "lid", "body", 3.6, true, 0.45, -0.066, 0.205, 0.132, 0.03, { region: "lid", material: "stainless" }),
  rect("base", "base", "body", 3.6, true, 0.5, -0.09, 0, 0.18, 0.03, { region: "plate", material: "pp-plastic" }),
], dec(
  ["A 2.4 mm wide jug: body with spout, handle, lid knob and base as five masses.", "Ein 2,4 mm breiter Krug: Körper mit Ausguss, Griff, Deckelknopf und Sockel als fünf Massen."],
  ["Body with spout in one outline, handle, lid with knob, base and the water block.", "Körper mit Ausguss in einer Kontur, Griff, Deckel mit Knopf, Sockel und der Wasserblock."],
  ["Spout and body are one outline, the gauge window is part of the water block (regions shell and window).", "Ausguss und Körper bilden eine Kontur, das Sichtfenster gehört zum Wasserblock (Regionen Hülle und Fenster)."],
  ["Gauge strip, lid button as its own shape, switch.", "Wasserstandsstreifen, Deckelknopf als eigene Form, Schalter."],
  ["Lid raised to 30 mm so that it keeps 0.3 mm on paper.", "Deckel auf 30 mm erhöht, damit er 0,3 mm auf Papier behält."],
  ["Body and window share the outline; handle, lid and plate stay own regions.", "Körper und Fenster teilen die Kontur; Griff, Deckel und Platte bleiben eigene Regionen."],
  { body: 0.18, detail: null, texture: null, glazing: null, ground: 0.25 },
));
const rep200 = rep(200, [150, 350], { body: 0.13, detail: null, texture: null, glazing: null, ground: 0.25 }, [
  poly("body", "body", "body", 3.4, true, 0.4, [[-0.078, 0.03], [0.078, 0.03], [0.072, 0.205], [-0.07, 0.205], [-0.07, 0.17], [-0.118, 0.224], [-0.07, 0.207]], true, { regions: ["shell", "window"], material: "stainless" }),
  poly("handle", "handle", "body", 3.5, true, 0.2, [[0.07, 0.2], [0.122, 0.195], [0.128, 0.065], [0.073, 0.04], [0.073, 0.075], [0.1, 0.09], [0.098, 0.17], [0.07, 0.175]], true, { region: "handle", material: "pp-plastic" }),
  rect("lid", "lid", "body", 3.6, true, 0.45, -0.07, 0.205, 0.14, 0.04, { region: "lid", material: "stainless" }),
  rect("base", "base", "body", 3.6, true, 0.5, -0.095, 0, 0.19, 0.04, { region: "plate", material: "pp-plastic" }),
], dec(
  ["A 1.2 mm wide jug of four shapes: body with spout, handle, lid and base.", "Ein 1,2 mm breiter Krug aus vier Formen: Körper mit Ausguss, Griff, Deckel und Sockel."],
  ["The four masses.", "Die vier Massen."],
  ["Body, spout, gauge window and water share one outline.", "Körper, Ausguss, Sichtfenster und Wasser teilen eine Kontur."],
  ["Water block, gauge, button and switch.", "Wasserblock, Sichtfenster, Knopf und Schalter."],
  ["Lid and base are 40 mm tall (0.2 mm), the handle neck 27 mm.", "Deckel und Sockel sind 40 mm hoch (0,2 mm), der Griffhals 27 mm."],
  ["Body carries shell and window; handle, lid and plate keep their own regions.", "Der Körper trägt Hülle und Fenster; Griff, Deckel und Platte behalten ihre Regionen."],
  { body: 0.13, detail: null, texture: null, glazing: null, ground: 0.25 },
));

const regions = [
  reg("plate", ["Base and heating plate", "Sockel und Heizplatte"], "heater", "operating", "pp-plastic", 0.05, 0.18, "cylinder", 0.003, 0.012, 3000, { operating: { key: "power", heatW: 20 } }, { layerKind: "polymer" }),
  reg("shell", ["Stainless jug shell", "Edelstahl-Kannenhülle"], "surface", "operating", "stainless", 0.11, 0.19, "cylinder", 0.0007, 0.00004, 3500, { operating: { key: "power", heatW: 45 } }),
  reg("window", ["Water gauge and water", "Wasserstand und Wasser"], "water", "operating", "water", 0.012, 0.12, "vertical-plate", 0.004, 0.0021, 3000, { operating: { key: "power", heatW: 10 } }, { layerKind: "polymer", source: "illustrative" }),
  reg("lid", ["Lid", "Deckel"], "surface", "operating", "stainless", 0.025, 0.14, "horizontal-plate-up", 0.0007, 0.00004, 2000, { operating: { key: "power", heatW: 14 } }),
  reg("handle", ["Handle", "Griff"], "surface", "passive", "pp-plastic", 0.012, 0.1, "vertical-plate", 0.003, 0.05, 1500, {}, { layerKind: "polymer" }),
];
const bind = bindOf({
  plate: { exposedFraction: 0.8, contact: { fraction: 0.2, tempC: 21 } },
  shell: { exposedFraction: 1 },
  window: { exposedFraction: 1 },
  lid: { exposedFraction: 1 },
  handle: { exposedFraction: 1 },
});
const sup = ["base-contact"];
const poses = [
  pose("off", ["Off", "Aus"], ["The kettle is cold and filled.", "Der Wasserkocher ist kalt und gefüllt."], {}, sup, "auto", bind({ power: 0 }, { steam: 0, "steam-plume": 0 })),
  pose("heating", ["Heating", "Heizt"], ["The 2 kW element warms the water; the shell warms through and a little vapour appears.", "Das 2-kW-Heizelement erwärmt das Wasser; die Hülle erwärmt sich durch und etwas Dampf tritt aus."], {}, sup, "auto", bind({ power: 0.6 }, { steam: 0.1, "steam-plume": 0.2 })),
  pose("boiling", ["Boiling", "Kocht"], ["The water boils: steam leaves the spout and the lid rattles.", "Das Wasser kocht: Dampf tritt am Ausguss aus und der Deckel klappert."], { lid: { dy: 0.003 } }, sup, "auto", bind({ power: 1 }, { steam: 1, "steam-plume": 1 })),
];
const clips = [
  clip("boil", ["Boil", "Aufkochen"], ["The kettle heats for three minutes, then boils with steam at the spout.", "Der Wasserkocher heizt drei Minuten lang und kocht dann mit Dampf am Ausguss."],
    180, 1,
    [{ t: 0, pose: "off", ease: "step" }, { t: 1, pose: "heating", ease: "linear" }, { t: 120, pose: "heating", ease: "linear" }, { t: 150, pose: "boiling", ease: "step" }, { t: 180, pose: "boiling", ease: "step" }],
    contacts(sup, 180),
    [["off", 0, 1, ["Switched off", "Aus"]], ["heat", 1, 150, ["Heating", "Heizen"]], ["boil", 150, 180, ["Boiling", "Kochen"]]],
    [[1, "operating", ["Switched on", "Eingeschaltet"], "power"]],
    [["operating:power", "linear", [[0, 0], [1, 0.6], [120, 0.6], [150, 1]]], ["emitter:steam", "linear", [[0, 0], [1, 0.1], [120, 0.1], [150, 1]]], ["emitter:steam-plume", "linear", [[0, 0], [1, 0.2], [120, 0.2], [150, 1]]]]),
  clip("rattle", ["Rattle", "Klappern"], ["The boiling lid lifts and falls in the steam.", "Der Deckel hebt und senkt sich im Dampf."],
    1.2, 15,
    [{ t: 0, pose: "boiling", ease: "smooth" }, { t: 0.3, pose: "boiling", ease: "smooth", bones: { lid: { dy: 0.005 } } }, { t: 0.6, pose: "boiling", ease: "smooth", bones: { lid: { dy: 0.001 } } }, { t: 0.9, pose: "boiling", ease: "smooth", bones: { lid: { dy: 0.004 } } }, { t: 1.2, pose: "boiling", ease: "smooth" }],
    contacts(sup, 1.2), [["rattle", 0, 1.2, ["Rattling", "Klappern"]]], [],
    [["emitter:steam-plume", "smooth", [[0, 1], [0.3, 0.9], [0.6, 1], [0.9, 0.9], [1.2, 1]]]]),
];

export const kettle = () =>
  mkAsset({
    id: "electric-kettle",
    category: "appliances",
    title: ["Electric Jug Kettle", "Elektrischer Wasserkocher"],
    summary: [
      "A 1.7 L stainless jug kettle of 2 kW on its power base: it heats the water, warms its shell and the base plate and boils with steam at the spout and a rattling lid.",
      "Ein 1,7-l-Edelstahl-Wasserkocher von 2 kW auf seinem Sockel: er erwärmt das Wasser, seine Hülle und die Heizplatte und kocht mit Dampf am Ausguss und klapperndem Deckel.",
    ],
    tags: ["kettle", "kitchen", "water", "boiling", "steam", "appliance", "internal-gain"],
    references: [ASHRAE, ref("cengel-2015", "Cengel, Y. A., Boles, M. A. (2015). Thermodynamics: An Engineering Approach, 8th ed., McGraw-Hill (specific heat 4.19 kJ/(kg K) and latent heat 2257 kJ/kg of water).")],
    notes: [
      "A 1.7 L jug kettle with a 2 kW concealed element in a 0.18 m power base, a stainless body of 0.15 m by 0.19 m tapering upwards, a spout at the left, a C-shaped polypropylene handle at the right, a domed lid with a button and a vertical water gauge strip. With the full 1.7 kg of water the kettle weighs 2.92 kg (base 0.55, body 0.45, handle 0.12, lid 0.1, water 1.7). Heating 1.7 kg by 80 K needs 570 kJ, which takes 4.8 minutes at 2 kW; the clip shows three minutes of heating at 60 percent and then boiling (compressed, authored estimate). The origin is the centre of the base on the surface.",
      "Ein 1,7-l-Wasserkocher mit einem verdeckten 2-kW-Heizelement in einem 0,18 m breiten Sockel, einem Edelstahlkörper von 0,15 m mal 0,19 m, der sich nach oben verjüngt, einem Ausguss links, einem C-förmigen Polypropylengriff rechts, einem gewölbten Deckel mit Knopf und einem senkrechten Wasserstandsstreifen. Mit den vollen 1,7 kg Wasser wiegt der Kocher 2,92 kg (Sockel 0,55, Körper 0,45, Griff 0,12, Deckel 0,1, Wasser 1,7). Das Erwärmen von 1,7 kg um 80 K braucht 570 kJ, bei 2 kW sind das 4,8 Minuten; der Clip zeigt drei Minuten Heizen mit 60 Prozent und dann das Kochen (gestaucht, Schätzwert). Der Ursprung ist die Mitte des Sockels auf der Unterlage.",
    ],
    dims: [0.24, 0.245, 0.18],
    materials: [
      materialOf("stainless", "Stainless steel", "Edelstahl", 0.3, 0.5, 7900, 16, 500, "literature", "incropera-2018"),
      materialOf("pp-plastic", "Polypropylene", "Polypropylen", 0.9, 0.6, 900, 0.22, 1900, SRC, "incropera-2018"),
      materialOf("water", "Water", "Wasser", 0.96, 0.1, 1000, 0.6, 4190, "literature", "incropera-2018"),
    ],
    root: "base",
    bones: sp.bones,
    anchors: [baseContact("base", 0, 0, 0.17, 0.17), sp.anchor("spout-tip", "body", -0.112, 0.222, "emitter", ["Spout opening", "Ausgussöffnung"])],
    reps: { 500: u500("the 15 mm lid and base rims", "die 15 mm hohen Deckel- und Sockelränder", 0.015, 0.5), 200: rep200, 100: rep100, 50: rep50 },
    poses,
    clips,
    operating: [{ id: "power", title: tx("Heating element", "Heizelement"), levels: [level(0, "Off", "Aus"), level(0.6, "Heating up", "Aufheizen"), level(1, "Boiling (2 kW)", "Kochend (2 kW)")] }],
    regions,
    emitters: [
      { id: "steam", kind: "vent", title: tx("Steam leaving the spout", "Dampf am Ausguss"), anchor: "spout-tip", directionDeg: 60, diameterM: 0.03, operatingKey: "power", volumeFlowM3S: 0.0015, temperatureRiseK: 75, vapourKgPerKg: 0.3, source: SRC, reference: "ashrae-2021" },
      { id: "steam-plume", kind: "plume", title: tx("Plume above the spout", "Fahne über dem Ausguss"), anchor: "spout-tip", directionDeg: 90, diameterM: 0.06, region: "shell", source: SRC, reference: "ashrae-2021" },
    ],
    assumptions: [
      ["Of the 2 kW only the losses to the shell, plate, lid and water window are modelled (189 W); the rest goes into the 1.7 kg of water, which is not a region, and leaves as latent heat with the steam emitter once it boils.", "Von den 2 kW sind nur die Verluste an Hülle, Platte, Deckel und Wasserfenster modelliert (189 W); der Rest geht in die 1,7 kg Wasser, die keine Region sind, und verlässt den Kocher beim Sieden als latente Wärme über den Dampfemitter."],
    ],
    structure: [["The 2.9 kg kettle stands on its base: a unilateral contact of 0.17 m diameter carries the weight.", "Der 2,9 kg schwere Wasserkocher steht auf seinem Sockel: ein einseitiger Kontakt von 0,17 m Durchmesser trägt das Gewicht."]],
    defaultPose: "boiling",
    defaultClip: "rattle",
  });
