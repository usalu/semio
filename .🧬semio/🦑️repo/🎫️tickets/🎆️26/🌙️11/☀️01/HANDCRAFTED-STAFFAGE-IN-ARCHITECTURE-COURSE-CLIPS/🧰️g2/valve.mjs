import { ASHRAE, EN_14511, SRC, baseContact, bindOf, clip, contacts, dec, fixed, level, m, materialOf, mkAsset, pose, ref, rep, reg, space2 as space, tx, un } from "./g2.mjs";

const sp = space([
  ["body", null, 0, 0, { title: tx("Brass valve body", "Messing-Ventilkörper"), lengthM: 0.064, joint: fixed, mass: m(0.62, 0, 0.025) }],
  ["head", "body", 0, 0.05, { title: tx("Diaphragm head", "Membrankopf"), lengthM: 0.03, joint: fixed, mass: m(0.14, 0, 0.065) }],
  ["stem", "head", 0, 0.078, { title: tx("Superheat adjusting stem", "Überhitzungs-Einstellspindel"), lengthM: 0.012, joint: { type: "prismatic", axisDeg: 90, minM: -0.008, maxM: 0 }, mass: m(0.02, 0, 0.084) }],
  ["bulb", "body", 0.108, 0.03, { title: tx("Sensing bulb on the suction line", "Fühler an der Saugleitung"), lengthM: 0.035, joint: fixed, mass: m(0.05, 0.108, 0.032) }],
]);
const { rect, poly, path } = sp;
const HEAD = "M -0.025 0.05 C -0.025 0.068 -0.014 0.078 0 0.078 C 0.014 0.078 0.025 0.068 0.025 0.05 Z";
const CAP = [[0.018, 0.072], [0.05, 0.088], [0.09, 0.082], [0.108, 0.05]];

const rep50 = rep(50, [20, 75], { body: 0.18, detail: 0.13, texture: null, glazing: null, ground: 0.35 }, [
  poly("capillary", "head", "detail", 3.2, false, 0.1, CAP, false, { region: "capillary", material: "copper" }),
  rect("bulb", "bulb", "body", 3.3, true, 0.5, 0.098, 0.015, 0.02, 0.035, { region: "bulb", material: "copper" }),
  rect("stem", "stem", "body", 3.3, true, 0.3, -0.01, 0.078, 0.02, 0.012, { region: "head", material: "brass" }),
  path("head", "head", "body", 3.4, true, 0.55, HEAD, { region: "head", material: "brass" }),
  rect("inlet", "body", "body", 3.3, true, 0.4, -0.078, 0.02, 0.046, 0.018, { region: "inlet", material: "copper" }),
  rect("outlet", "body", "body", 3.3, true, 0.4, 0.032, 0.01, 0.046, 0.018, { region: "outlet", material: "copper" }),
  rect("body", "body", "body", 3.5, true, 0.5, -0.032, 0, 0.064, 0.05, { region: "body", material: "brass" }),
], dec(
  ["A thermostatic expansion valve 9 mm tall and 19 mm wide on paper: a brass body with a diaphragm dome and adjusting stem, a liquid inlet at the left, an outlet at the right and a capillary tube to the sensing bulb.", "Ein thermostatisches Expansionsventil, auf Papier 9 mm hoch und 19 mm breit: ein Messingkörper mit Membrandom und Einstellspindel, ein Flüssigkeitseingang links, ein Ausgang rechts und ein Kapillarrohr zum Fühler."],
  ["Body, head dome, stem, both connections, capillary and bulb.", "Körper, Membrandom, Spindel, beide Anschlüsse, Kapillare und Fühler."],
  ["Nothing is merged.", "Nichts ist zusammengeführt."],
  ["The Schrader port, the equaliser line and the flare nuts.", "Der Schraderanschluss, die Druckausgleichsleitung und die Bördelmuttern."],
  ["The capillary (1.6 mm real) is a 0.13 mm line; both connections are 18 mm thick.", "Die Kapillare (real 1,6 mm) ist eine 0,13-mm-Linie; beide Anschlüsse sind 18 mm dick."],
  ["Body, head, inlet, outlet, bulb and capillary are six regions: the inlet is warm liquid, body, outlet, head and bulb are cold.", "Körper, Kopf, Eingang, Ausgang, Fühler und Kapillare sind sechs Regionen: der Eingang führt warme Flüssigkeit, Körper, Ausgang, Kopf und Fühler sind kalt."],
  { body: 0.18, detail: 0.13, texture: null, glazing: null, ground: 0.35 },
));
const rep100 = rep(100, [75, 150], { body: 0.18, detail: null, texture: null, glazing: null, ground: 0.25 }, [
  rect("bulb", "bulb", "body", 3.3, true, 0.5, 0.096, 0.015, 0.024, 0.035, { regions: ["bulb", "capillary"], material: "copper" }),
  path("head", "head", "body", 3.4, true, 0.55, HEAD, { regions: ["head"], material: "brass" }),
  rect("inlet", "body", "body", 3.3, true, 0.4, -0.078, 0.018, 0.046, 0.022, { region: "inlet", material: "copper" }),
  rect("outlet", "body", "body", 3.3, true, 0.4, 0.032, 0.008, 0.046, 0.022, { region: "outlet", material: "copper" }),
  rect("body", "body", "body", 3.5, true, 0.5, -0.032, 0, 0.064, 0.05, { region: "body", material: "brass" }),
], dec(
  ["A 1.9 mm wide valve: body with dome, two connection stubs and a bulb block to the right.", "Ein 1,9 mm breites Ventil: Körper mit Dom, zwei Anschlussstutzen und ein Fühlerblock rechts."],
  ["Body, dome, inlet, outlet and bulb.", "Körper, Dom, Eingang, Ausgang und Fühler."],
  ["The capillary joins the bulb block (regions bulb and capillary).", "Die Kapillare geht im Fühlerblock auf (Regionen Fühler und Kapillare)."],
  ["Stem and capillary line.", "Spindel und Kapillarlinie."],
  ["The stubs are 22 mm thick, the bulb 24 mm wide.", "Die Stutzen sind 22 mm dick, der Fühler 24 mm breit."],
  ["The bulb block carries bulb and capillary as merged regions.", "Der Fühlerblock trägt Fühler und Kapillare als zusammengeführte Regionen."],
  { body: 0.18, detail: null, texture: null, glazing: null, ground: 0.25 },
));

const regions = [
  reg("body", ["Valve body", "Ventilkörper"], "surface", "operating", "brass", 0.012, 0.06, "cylinder", 0.004, 0.00004, 3000, { operating: { key: "flow", heatW: -2.4 } }),
  reg("head", ["Diaphragm head", "Membrankopf"], "surface", "operating", "brass", 0.004, 0.05, "cylinder", 0.002, 0.00004, 1500, { operating: { key: "flow", heatW: -0.8 } }),
  reg("inlet", ["Liquid inlet", "Flüssigkeitseingang"], "surface", "operating", "copper", 0.004, 0.046, "cylinder", 0.001, 0.00001, 800, { operating: { key: "flow", heatW: 0.6 } }),
  reg("outlet", ["Cold outlet", "Kalter Ausgang"], "surface", "operating", "copper", 0.004, 0.046, "cylinder", 0.001, 0.00001, 800, { operating: { key: "flow", heatW: -0.9 } }),
  reg("bulb", ["Sensing bulb", "Fühler"], "surface", "operating", "copper", 0.002, 0.035, "cylinder", 0.001, 0.00001, 800, { operating: { key: "flow", heatW: -0.4 } }),
  reg("capillary", ["Capillary tube", "Kapillarrohr"], "surface", "passive", "copper", 0.0004, 0.1, "cylinder", 0.0007, 0.00001, 400),
];
const bind = bindOf({
  body: { exposedFraction: 1 },
  head: { exposedFraction: 1 },
  inlet: { exposedFraction: 1 },
  outlet: { exposedFraction: 1 },
  bulb: { exposedFraction: 1 },
  capillary: { exposedFraction: 1 },
});
const sup = ["base-contact"];
const poses = [
  pose("closed", ["Closed", "Geschlossen"], ["The valve is shut: no refrigerant flows and everything is at room temperature.", "Das Ventil ist geschlossen: kein Kältemittel fließt und alles hat Raumtemperatur."], {}, sup, "auto", bind({ flow: 0 })),
  pose("half-open", ["Half open", "Halb offen"], ["Half the rated flow passes: the outlet and the body cool.", "Der halbe Nennstrom fließt: Ausgang und Körper kühlen ab."], { stem: { dy: -0.004 } }, sup, "auto", bind({ flow: 0.5 })),
  pose("open", ["Open", "Offen"], ["The rated flow expands from liquid to a cold two-phase mixture: the outlet is the coldest part.", "Der Nennstrom entspannt von Flüssigkeit zu einem kalten Zweiphasengemisch: der Ausgang ist der kälteste Teil."], { stem: { dy: -0.008 } }, sup, "auto", bind({ flow: 1 })),
];
const clips = [
  clip("modulate", ["Modulate", "Regeln"], ["The stem moves in and out as the superheat changes and the cold outlet follows.", "Die Spindel bewegt sich mit der Überhitzung ein und aus, und der kalte Ausgang folgt."],
    3, 15,
    [{ t: 0, pose: "half-open", ease: "smooth" }, { t: 1.5, pose: "open", ease: "smooth" }, { t: 3, pose: "half-open", ease: "smooth" }],
    contacts(sup, 3), [["close", 0, 1.5, ["Opening", "Öffnen"]], ["open", 1.5, 3, ["Closing", "Schließen"]]], [],
    [["operating:flow", "smooth", [[0, 0.5], [1.5, 1], [3, 0.5]]]]),
  clip("open-up", ["Open up", "Aufgehen"], ["The valve opens within a minute and the outlet and body cool down.", "Das Ventil öffnet innerhalb einer Minute, Ausgang und Körper kühlen ab."],
    60, 2,
    [{ t: 0, pose: "closed", ease: "linear" }, { t: 10, pose: "open", ease: "step" }, { t: 60, pose: "open", ease: "step" }],
    contacts(sup, 60), [["shut", 0, 10, ["Opening", "Öffnen"]], ["run", 10, 60, ["Open", "Offen"]]],
    [[10, "operating", ["Fully open", "Ganz offen"], "flow"]],
    [["operating:flow", "linear", [[0, 0], [10, 1], [60, 1]]]]),
];

export const valve = () =>
  mkAsset({
    id: "expansion-valve",
    category: "machinery",
    title: ["Thermostatic Expansion Valve", "Thermostatisches Expansionsventil"],
    summary: [
      "A brass thermostatic expansion valve of a small heat pump with a sensing bulb: warm liquid enters at the left and leaves as a very cold two-phase mixture, so body, outlet and bulb cool below room temperature.",
      "Ein thermostatisches Expansionsventil aus Messing einer kleinen Wärmepumpe mit Fühler: warme Flüssigkeit tritt links ein und verlässt es als sehr kaltes Zweiphasengemisch, sodass Körper, Ausgang und Fühler unter Raumtemperatur abkühlen.",
    ],
    tags: ["expansion-valve", "txv", "heat-pump", "refrigeration", "throttling", "cold", "machinery"],
    references: [EN_14511, ASHRAE],
    notes: [
      "A thermostatic expansion valve of the 3 kW class: a forged brass body of 64 mm by 50 mm with a liquid inlet connection (10 mm) at the left and an outlet connection at the right, a diaphragm head of 50 mm across above it with a superheat adjusting stem, and a 1.6 mm capillary to a 35 mm sensing bulb clamped on the suction line. The mass is 0.81 kg. The inlet carries warm liquid of about 35 degC, so it is slightly warmer than the room; the throttled outlet and the body are near the evaporating temperature of -5 degC. The heatW values (authored estimates) are the net heat the surfaces take from the room at that temperature difference. The origin is the centre of the body on the base line.",
      "Ein thermostatisches Expansionsventil der 3-kW-Klasse: ein geschmiedeter Messingkörper von 64 mm mal 50 mm mit einem Flüssigkeitsanschluss (10 mm) links und einem Ausgangsanschluss rechts, darüber ein Membrankopf von 50 mm Durchmesser mit einer Überhitzungs-Einstellspindel und eine 1,6 mm dicke Kapillare zu einem 35 mm langen Fühler an der Saugleitung. Die Masse beträgt 0,81 kg. Der Eingang führt warme Flüssigkeit von rund 35 degC und ist daher etwas wärmer als der Raum; der gedrosselte Ausgang und der Körper liegen nahe der Verdampfungstemperatur von -5 degC. Die heatW-Werte (Schätzwerte) sind die Nettowärme, die die Oberflächen bei diesem Temperaturunterschied dem Raum entziehen. Der Ursprung ist die Mitte des Körpers auf der Grundlinie.",
    ],
    dims: [0.09, 0.19, 0.06],
    materials: [
      materialOf("brass", "Brass", "Messing", 0.3, 0.5, 8500, 110, 380, "literature", "incropera-2018"),
      materialOf("copper", "Copper", "Kupfer", 0.05, 0.3, 8900, 390, 385, "literature", "incropera-2018"),
    ],
    root: "body",
    bones: sp.bones,
    anchors: [baseContact("body", 0, 0, 0.05, 0.05)],
    reps: { 500: un(500, "the 18 mm connection stubs", "die 18 mm dicken Anschlussstutzen", 0.018, 0.4, [100, 50]), 200: un(200, "the 18 mm connection stubs", "die 18 mm dicken Anschlussstutzen", 0.018, 1, [100, 50]), 100: rep100, 50: rep50 },
    poses,
    clips,
    operating: [{ id: "flow", title: tx("Refrigerant flow", "Kältemittelstrom"), levels: [level(0, "Closed", "Geschlossen"), level(0.5, "Half open", "Halb offen"), level(1, "Open (rated)", "Offen (Nenn)")] }],
    regions,
    emitters: [],
    assumptions: [
      ["The valve is mounted on its body without a supporting structure here: the 0.8 kg is carried through the contact below, as a stand-in for the pipe clamps.", "Das Ventil steht hier ohne Tragwerk auf seinem Körper: die 0,8 kg werden über den Kontakt unten abgetragen, stellvertretend für die Rohrschellen."],
    ],
    structure: [["The 0.8 kg valve rests on its body: a unilateral contact of 0.05 m by 0.05 m carries the weight.", "Das 0,8 kg schwere Ventil ruht auf seinem Körper: ein einseitiger Kontakt von 0,05 m mal 0,05 m trägt das Gewicht."]],
    defaultPose: "open",
    defaultClip: "modulate",
  });
