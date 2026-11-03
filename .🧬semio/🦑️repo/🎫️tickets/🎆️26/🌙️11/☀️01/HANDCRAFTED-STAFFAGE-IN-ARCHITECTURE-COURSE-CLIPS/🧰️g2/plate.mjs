import { ASHRAE, EN_308, SRC, VDI_3803, baseContact, bindOf, clip, contacts, dec, fixed, level, m, materialOf, mkAsset, pose, ref, rep, reg, space2 as space, tx, un } from "./g2.mjs";

const sp = space([
  ["frame", null, 0, 0, { title: tx("Casing frame with stand", "Gehäuserahmen mit Fuß"), lengthM: 0.6, joint: fixed, mass: m(5, 0, 0.3) }],
  ["plates", "frame", 0, 0.03, { title: tx("Aluminium plate pack", "Aluminium-Plattenpaket"), lengthM: 0.54, joint: fixed, mass: m(12, 0, 0.3) }],
  ["ports", "frame", 0, 0.3, { title: tx("Four duct collars", "Vier Kanalstutzen"), lengthM: 0.62, joint: fixed, mass: m(0.4, 0, 0.3) }],
  ["damper", "frame", -0.2, 0.62, { title: tx("Bypass damper flap", "Bypass-Klappe"), lengthM: 0.4, joint: { type: "revolute", minDeg: 0, maxDeg: 70 }, mass: m(0.6, 0, 0.627) }],
]);
const { rect, poly } = sp;
const HATCH = [{ id: "plates", title: tx("Plate pack in section", "Plattenpaket im Schnitt"), pattern: "lines", angleDeg: 45, spacingMm: 0.5, penMm: 0.1 }];
const COLLARS = (z) => [
  rect("collar-upper-left", "ports", "body", z, true, 0.4, -0.31, 0.4, 0.06, 0.15, { region: "ports", material: "galvanised-steel" }),
  rect("collar-lower-left", "ports", "body", z, true, 0.4, -0.31, 0.06, 0.06, 0.15, { region: "ports", material: "galvanised-steel" }),
  rect("collar-upper-right", "ports", "body", z, true, 0.4, 0.25, 0.4, 0.06, 0.15, { region: "ports", material: "galvanised-steel" }),
  rect("collar-lower-right", "ports", "body", z, true, 0.4, 0.25, 0.06, 0.06, 0.15, { region: "ports", material: "galvanised-steel" }),
];

const rep50 = { ...rep(50, [20, 75], { body: 0.18, detail: 0.13, texture: 0.1, glazing: null, ground: 0.35 }, [
  ...COLLARS(3.2),
  rect("frame", "frame", "body", 3.3, true, 0.35, -0.25, 0, 0.5, 0.6, { region: "frame", material: "galvanised-steel" }),
  rect("plate-pack", "plates", "body", 3.4, true, 0.7, -0.22, 0.03, 0.44, 0.54, { region: "plates", material: "aluminium-plate", hatch: "plates" }),
  poly("damper", "damper", "body", 3.5, true, 0.3, [[-0.2, 0.6], [0.2, 0.6], [0.2, 0.614], [-0.2, 0.614]], true, { region: "damper", material: "galvanised-steel" }),
  poly("damper-hinge", "damper", "detail", 3.6, false, 0.1, [[-0.2, 0.6], [-0.2, 0.62]], false, { region: "damper", material: "galvanised-steel" }),
], dec(
  ["A 0.5 m by 0.6 m cross-flow plate heat exchanger in section, 10 mm by 13 mm on paper: a casing frame around a hatched plate pack, a duct collar at each of the four corners and a bypass damper flap on top.", "Ein 0,5 m mal 0,6 m großer Kreuzstrom-Plattenwärmeübertrager im Schnitt, auf Papier 10 mm mal 13 mm: ein Gehäuserahmen um ein schraffiertes Plattenpaket, je ein Kanalstutzen an den vier Ecken und eine Bypass-Klappe oben."],
  ["Frame, plate pack with its hatch, four collars, damper flap and hinge.", "Rahmen, Plattenpaket mit Schraffur, vier Stutzen, Klappe und Scharnier."],
  ["Nothing is merged: the plates are shown by a hatch.", "Nichts ist zusammengeführt: die Platten zeigt eine Schraffur."],
  ["The corrugation of the plates, the drain pan and the inspection door.", "Die Prägung der Platten, die Kondensatwanne und die Revisionstür."],
  ["The 0.3 mm plates (4 mm pitch) are drawn as a 45 degree hatch of 25 mm real pitch; the damper flap is 14 mm thick.", "Die 0,3 mm dicken Platten (4 mm Teilung) sind als 45-Grad-Schraffur mit 25 mm realer Teilung gezeichnet; die Klappe ist 14 mm dick."],
  ["Frame, plates, ports and damper are four regions: the plate pack takes an intermediate temperature between the hot and the cold air stream, the collars warm at the hot side, the damper stays cool.", "Rahmen, Platten, Stutzen und Klappe sind vier Regionen: das Plattenpaket nimmt eine Temperatur zwischen heißem und kaltem Luftstrom an, die Stutzen erwärmen sich auf der heißen Seite, die Klappe bleibt kühl."],
  { body: 0.18, detail: 0.13, texture: 0.1, glazing: null, ground: 0.35 },
)), hatches: HATCH };

const rep100 = { ...rep(100, [75, 150], { body: 0.18, detail: null, texture: 0.1, glazing: null, ground: 0.25 }, [
  ...COLLARS(3.2),
  rect("frame", "frame", "body", 3.3, true, 0.35, -0.25, 0, 0.5, 0.6, { region: "frame", material: "galvanised-steel" }),
  rect("plate-pack", "plates", "body", 3.4, true, 0.7, -0.21, 0.03, 0.42, 0.54, { region: "plates", material: "aluminium-plate", hatch: "plates" }),
  rect("damper", "damper", "body", 3.5, true, 0.3, -0.2, 0.6, 0.4, 0.02, { region: "damper", material: "galvanised-steel" }),
], dec(
  ["A 5 mm by 6 mm exchanger: frame, hatched plate pack, four collars and a damper flap.", "Ein 5 mm mal 6 mm großer Übertrager: Rahmen, schraffiertes Plattenpaket, vier Stutzen und eine Klappe."],
  ["Frame, hatched plate pack, collars and damper.", "Rahmen, schraffiertes Plattenpaket, Stutzen und Klappe."],
  ["Nothing is merged.", "Nichts ist zusammengeführt."],
  ["The hinge line and the corrugation.", "Die Scharnierlinie und die Prägung."],
  ["The hatch pitch is 50 mm real; the damper is 20 mm thick.", "Die Schraffur hat 50 mm reale Teilung; die Klappe ist 20 mm dick."],
  ["Same four regions as at 1:50.", "Dieselben vier Regionen wie bei 1:50."],
  { body: 0.18, detail: null, texture: 0.1, glazing: null, ground: 0.25 },
)), hatches: HATCH };

const rep200 = rep(200, [150, 350], { body: 0.13, detail: null, texture: null, glazing: null, ground: 0.25 }, [
  ...COLLARS(3.2).map((s) => ({ ...s })),
  rect("frame", "frame", "body", 3.3, true, 0.35, -0.25, 0, 0.5, 0.6, { region: "frame", material: "galvanised-steel" }),
  rect("plate-pack", "plates", "body", 3.4, true, 0.8, -0.2, 0.05, 0.4, 0.5, { region: "plates", material: "aluminium-plate" }),
  rect("damper", "damper", "body", 3.5, true, 0.3, -0.2, 0.6, 0.4, 0.04, { region: "damper", material: "galvanised-steel" }),
], dec(
  ["A 2.5 mm by 3 mm exchanger of four masses.", "Ein 2,5 mm mal 3 mm großer Übertrager aus vier Massen."],
  ["Frame, plate block, collars and damper.", "Rahmen, Plattenblock, Stutzen und Klappe."],
  ["The plate pack is one plain block.", "Das Plattenpaket ist ein einfacher Block."],
  ["Hatch, hinge and corrugation.", "Schraffur, Scharnier und Prägung."],
  ["The damper is 40 mm thick (0.2 mm).", "Die Klappe ist 40 mm dick (0,2 mm)."],
  ["Same four regions as at 1:50.", "Dieselben vier Regionen wie bei 1:50."],
  { body: 0.13, detail: null, texture: null, glazing: null, ground: 0.25 },
));

const regions = [
  reg("frame", ["Casing frame", "Gehäuserahmen"], "surface", "passive", "galvanised-steel", 0.9, 0.6, "vertical-plate", 0.001, 0.00003, 3500),
  reg("plates", ["Plate pack", "Plattenpaket"], "heater", "operating", "aluminium-plate", 0.6, 0.54, "vertical-plate", 0.0003, 0.000002, 4000, { operating: { key: "exchange", heatW: 60 } }),
  reg("ports", ["Duct collars", "Kanalstutzen"], "surface", "operating", "galvanised-steel", 0.05, 0.15, "cylinder", 0.001, 0.00003, 1500, { operating: { key: "exchange", heatW: 5 } }),
  reg("damper", ["Bypass damper", "Bypass-Klappe"], "surface", "passive", "galvanised-steel", 0.15, 0.4, "horizontal-plate-up", 0.001, 0.00003, 2000),
];
const bind = bindOf({ frame: { exposedFraction: 0.9, contact: { fraction: 0.1, tempC: 21 } }, plates: { exposedFraction: 1 }, ports: { exposedFraction: 1 }, damper: { exposedFraction: 1 } });
const sup = ["base-contact"];
const poses = [
  pose("off", ["Off", "Aus"], ["Both air streams stand still: the exchanger is at room temperature.", "Beide Luftströme stehen still: der Übertrager hat Raumtemperatur."], {}, sup, "auto", bind({ exchange: 0 })),
  pose("partial", ["Partial bypass", "Teil-Bypass"], ["The damper is half open: part of the air bypasses the plates, which take half the heat.", "Die Klappe ist halb offen: ein Teil der Luft umgeht die Platten, die nur die halbe Wärme übertragen."], { damper: { angleDeg: 35 } }, sup, "auto", bind({ exchange: 0.5 })),
  pose("full", ["Full exchange", "Voller Austausch"], ["The damper is closed and all air passes the plates: the pack carries the full heat transfer.", "Die Klappe ist geschlossen und alle Luft strömt durch die Platten: das Paket überträgt die volle Wärme."], {}, sup, "auto", bind({ exchange: 1 })),
];
const clips = [
  clip("bypass", ["Bypass damper", "Bypass-Klappe"], ["The damper opens and closes again while the exchange follows.", "Die Klappe öffnet und schließt wieder, während der Austausch folgt."],
    4, 15,
    [{ t: 0, pose: "full", ease: "smooth" }, { t: 2, pose: "partial", ease: "smooth", bones: { damper: { angleDeg: 60 } } }, { t: 4, pose: "full", ease: "smooth" }],
    contacts(sup, 4), [["close", 0, 2, ["Opening", "Öffnen"]], ["open", 2, 4, ["Closing", "Schließen"]]], [],
    [["operating:exchange", "smooth", [[0, 1], [2, 0.5], [4, 1]]]]),
  clip("warm-up", ["Warm up", "Anlaufen"], ["When both fans start the plate pack takes its temperature within a few minutes.", "Wenn beide Ventilatoren starten, nimmt das Plattenpaket binnen weniger Minuten seine Temperatur an."],
    300, 1,
    [{ t: 0, pose: "off", ease: "step" }, { t: 5, pose: "full", ease: "step" }, { t: 300, pose: "full", ease: "step" }],
    contacts(sup, 300), [["off", 0, 5, ["Off", "Aus"]], ["run", 5, 300, ["Exchanging", "Austausch"]]], [[5, "operating", ["Fans start", "Ventilatoren starten"], "exchange"]], [["operating:exchange", "smooth", [[0, 0], [5, 0], [6, 1]]]]),
];

export const plate = () =>
  mkAsset({
    id: "plate-heat-exchanger",
    category: "appliances",
    title: ["Cross-Flow Plate Heat Exchanger", "Kreuzstrom-Plattenwärmeübertrager"],
    summary: [
      "A 0.5 m by 0.6 m aluminium cross-flow plate heat exchanger of an air handler, cut open to the hatched plate pack, with four duct collars and a bypass damper flap on top.",
      "Ein 0,5 m mal 0,6 m großer Kreuzstrom-Plattenwärmeübertrager aus Aluminium eines Lüftungsgeräts, aufgeschnitten bis zum schraffierten Plattenpaket, mit vier Kanalstutzen und einer Bypass-Klappe oben.",
    ],
    tags: ["plate-heat-exchanger", "heat-recovery", "cross-flow", "air-handling", "bypass", "dec", "appliance"],
    references: [EN_308, VDI_3803, ASHRAE],
    notes: [
      "A cross-flow plate heat exchanger of the 1000 m3/h class: 220 embossed aluminium plates of 0.3 mm at 2.5 mm pitch between 0.5 m by 0.6 m end plates in a galvanised frame, with a duct collar at each of the four corners (two for supply, two for exhaust) and a bypass damper flap of 0.4 m on a hinge at the top. The mass is 18 kg (frame 5, plates 12, collars 0.4, damper 0.6). The drawing is the section through the pack, so the plates appear as a diagonal hatch. At full exchange the pack takes a temperature between the hot supply and the cold exhaust stream (60 W, +7 K against room air, authored estimate); the damper reduces it. The origin is the centre of the base on the surface.",
      "Ein Kreuzstrom-Plattenwärmeübertrager der Klasse 1000 m3/h: 220 geprägte Aluminiumplatten von 0,3 mm im Abstand von 2,5 mm zwischen 0,5 m mal 0,6 m großen Endplatten in einem verzinkten Rahmen, mit je einem Kanalstutzen an den vier Ecken (zwei für Zuluft, zwei für Abluft) und einer 0,4 m langen Bypass-Klappe auf einem Scharnier oben. Die Masse beträgt 18 kg (Rahmen 5, Platten 12, Stutzen 0,4, Klappe 0,6). Die Zeichnung ist der Schnitt durch das Paket, daher erscheinen die Platten als diagonale Schraffur. Bei vollem Austausch nimmt das Paket eine Temperatur zwischen heißem Zuluft- und kaltem Abluftstrom an (60 W, +7 K gegen Raumluft, Schätzwert); die Klappe verringert sie. Der Ursprung ist die Mitte des Fußes auf der Unterlage.",
    ],
    dims: [0.634, 0.62, 0.5],
    materials: [
      materialOf("galvanised-steel", "Galvanised steel", "Verzinkter Stahl", 0.25, 0.5, 7800, 50, 480, "literature", "incropera-2018"),
      materialOf("aluminium-plate", "Embossed aluminium plates", "Geprägte Aluminiumplatten", 0.1, 0.3, 2700, 237, 900, "literature", "incropera-2018"),
    ],
    root: "frame",
    bones: sp.bones,
    anchors: [baseContact("frame", 0, 0, 0.5, 0.5)],
    reps: { 500: un(500, "the 0.3 mm plates and the 14 mm damper", "die 0,3 mm dicken Platten und die 14 mm dicke Klappe", 0.0003, 1.2, [200, 100]), 200: rep200, 100: rep100, 50: rep50 },
    poses,
    clips,
    operating: [{ id: "exchange", title: tx("Heat exchange", "Wärmeübertragung"), levels: [level(0, "No flow", "Kein Durchfluss"), level(0.5, "Half bypassed", "Halb im Bypass"), level(1, "Full exchange", "Voller Austausch")] }],
    regions,
    emitters: [],
    assumptions: [
      ["The air streams are not emitters of this asset; the plate pack temperature is the surface temperature of the casing section against still room air.", "Die Luftströme sind keine Emitter dieses Assets; die Plattenpakettemperatur ist die Oberflächentemperatur des Gehäuseschnitts gegen ruhende Raumluft."],
    ],
    structure: [["The 18 kg exchanger stands on its frame: a unilateral contact of 0.5 m by 0.5 m carries the weight.", "Der 18 kg schwere Übertrager steht auf seinem Rahmen: ein einseitiger Kontakt von 0,5 m mal 0,5 m trägt das Gewicht."]],
    defaultPose: "full",
    defaultClip: "bypass",
  });
