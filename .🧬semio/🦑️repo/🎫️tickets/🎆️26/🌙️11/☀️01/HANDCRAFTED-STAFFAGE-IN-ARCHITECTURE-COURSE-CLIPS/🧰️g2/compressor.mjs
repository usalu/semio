import { EN_14511, SRC, ASHRAE, baseContact, bindOf, clip, contacts, dec, fixed, level, m, materialOf, mkAsset, pose, ref, rep, reg, space2 as space, tx, u500 } from "./g2.mjs";

const sp = space([
  ["shell", null, 0, 0, { title: tx("Hermetic shell with feet", "Hermetisches Gehäuse mit Füßen"), lengthM: 0.34, joint: fixed, mass: m(9.4, 0, 0.17) }],
  ["dome", "shell", 0, 0.34, { title: tx("Discharge dome with stub", "Druckdom mit Stutzen"), lengthM: 0.09, joint: fixed, mass: m(2.6, 0, 0.37) }],
  ["terminal", "shell", 0.1, 0.25, { title: tx("Terminal box", "Anschlusskasten"), lengthM: 0.06, joint: fixed, mass: m(0.3, 0.12, 0.28) }],
  ["rotor", "shell", 0, 0.18, { title: tx("Orbiting scroll seen in the sight glass", "Orbitierende Spirale im Schauglas"), lengthM: 0.036, joint: { type: "revolute" }, mass: m(0.2, 0, 0.18) }],
]);
const { rect, ellipse, poly, path } = sp;
const r5 = (n) => Math.round(n * 1e5) / 1e5;
const spiral = (cx, cy, turns, r0, r1) => {
  const pts = [];
  const n = 28;
  for (let i = 0; i <= n; i += 1) {
    const u = i / n;
    const a = u * turns * 2 * Math.PI;
    const r = r0 + (r1 - r0) * u;
    pts.push([r5(cx + r * Math.cos(a)), r5(cy + r * Math.sin(a))]);
  }
  return pts;
};
const DOME = "M -0.1 0.34 C -0.1 0.382 -0.05 0.4 0 0.4 C 0.05 0.4 0.1 0.382 0.1 0.34 Z";

const rep50 = rep(50, [20, 75], { body: 0.18, detail: 0.13, texture: null, glazing: 0.13, ground: 0.35 }, [
  rect("discharge-stub", "dome", "body", 3.2, true, 0.7, -0.014, 0.398, 0.028, 0.032, { region: "discharge", material: "copper" }),
  path("dome", "dome", "body", 3.3, true, 0.7, DOME, { region: "dome", material: "steel-shell" }),
  rect("shell", "shell", "body", 3.4, true, 0.5, -0.1, 0.03, 0.2, 0.31, { region: "shell", material: "steel-shell" }),
  rect("suction-stub", "shell", "body", 3.2, true, 0.2, -0.15, 0.2, 0.052, 0.026, { region: "suction", material: "copper" }),
  rect("terminal", "terminal", "body", 3.5, true, 0.3, 0.1, 0.25, 0.04, 0.06, { region: "terminal", material: "pa-plastic" }),
  ellipse("sight-ring", "shell", "body", 3.6, true, 0.3, 0, 0.18, 0.05, 0.05, { region: "shell", material: "steel-shell" }),
  ellipse("scroll-disc", "rotor", "glazing", 3.7, true, 0.8, 0, 0.18, 0.04, 0.04, { region: "scroll", material: "cast-aluminium" }),
  poly("spiral", "rotor", "detail", 3.8, false, 0.1, spiral(0, 0.18, 1.3, 0.006, 0.034), false, { region: "scroll", material: "cast-aluminium" }),
  ellipse("scroll-hub", "rotor", "detail", 3.9, true, 0.2, 0, 0.18, 0.006, 0.006, { region: "scroll", material: "cast-aluminium" }),
  rect("feet", "shell", "body", 3.5, true, 0.4, -0.14, 0, 0.28, 0.03, { region: "feet", material: "steel-shell" }),
], dec(
  ["A 0.43 m tall hermetic scroll compressor, 8.6 mm tall on paper: a cylindrical shell on a foot plate with a domed top, a discharge stub on top, a suction stub at the left, a terminal box at the right and a round sight glass with the orbiting scroll.", "Ein 0,43 m hoher hermetischer Scrollverdichter, auf Papier 8,6 mm hoch: ein zylindrisches Gehäuse auf einer Fußplatte mit gewölbtem Dom, einem Druckstutzen oben, einem Saugstutzen links, einem Anschlusskasten rechts und einem runden Schauglas mit der orbitierenden Spirale."],
  ["Foot plate, shell, dome, both pipe stubs, terminal box, sight ring, scroll disc with its spiral and hub.", "Fußplatte, Gehäuse, Dom, beide Rohrstutzen, Anschlusskasten, Schauglasring, Spiralscheibe mit Spirale und Nabe."],
  ["Nothing is merged.", "Nichts ist zusammengeführt."],
  ["The oil sight glass, the welding seam, the rubber feet and the cable gland.", "Das Ölschauglas, die Schweißnaht, die Gummifüße und die Kabelverschraubung."],
  ["The sight glass is 80 mm wide (real window 70 mm); stubs are 26 to 28 mm thick.", "Das Schauglas ist 80 mm breit (Fenster real 70 mm); die Stutzen sind 26 bis 28 mm dick."],
  ["Shell, dome, two stubs, terminal, feet and scroll are seven regions: the dome and discharge stub run hot, the suction stub cold; the spiral turns with the rotor bone.", "Gehäuse, Dom, zwei Stutzen, Anschlusskasten, Füße und Spirale sind sieben Regionen: Dom und Druckstutzen laufen heiß, der Saugstutzen kalt; die Spirale dreht sich mit dem Rotorknochen."],
  { body: 0.18, detail: 0.13, texture: null, glazing: 0.13, ground: 0.35 },
));
const rep100 = rep(100, [75, 150], { body: 0.18, detail: 0.13, texture: null, glazing: null, ground: 0.25 }, [
  rect("discharge-stub", "dome", "body", 3.2, true, 0.7, -0.016, 0.398, 0.032, 0.034, { region: "discharge", material: "copper" }),
  path("dome", "dome", "body", 3.3, true, 0.7, DOME, { region: "dome", material: "steel-shell" }),
  rect("shell", "shell", "body", 3.4, true, 0.5, -0.1, 0.03, 0.2, 0.31, { regions: ["shell", "feet"], material: "steel-shell" }),
  rect("suction-stub", "shell", "body", 3.2, true, 0.2, -0.15, 0.19, 0.052, 0.034, { region: "suction", material: "copper" }),
  rect("terminal", "terminal", "body", 3.5, true, 0.3, 0.1, 0.25, 0.044, 0.06, { region: "terminal", material: "pa-plastic" }),
  ellipse("scroll-disc", "rotor", "body", 3.7, true, 0.8, 0, 0.18, 0.05, 0.05, { region: "scroll", material: "cast-aluminium" }),
  poly("spiral", "rotor", "detail", 3.8, false, 0.1, spiral(0, 0.18, 1.1, 0.01, 0.04), false, { region: "scroll", material: "cast-aluminium" }),
  rect("foot-plate", "shell", "body", 3.5, true, 0.4, -0.14, 0, 0.28, 0.03, { region: "feet", material: "steel-shell" }),
], dec(
  ["A 4.3 mm tall compressor: shell with dome on a foot plate, three stubs and a round sight disc with a spiral.", "Ein 4,3 mm hoher Verdichter: Gehäuse mit Dom auf einer Fußplatte, drei Stutzen und eine runde Schauscheibe mit Spirale."],
  ["Foot plate, shell, dome, discharge, suction, terminal and the scroll disc with spiral.", "Fußplatte, Gehäuse, Dom, Druck-, Saugstutzen, Anschlusskasten und die Spiralscheibe mit Spirale."],
  ["The shell shape carries shell and feet; the sight ring is dropped so that the disc sits directly on the shell.", "Die Gehäuseform trägt Gehäuse und Füße; der Schauglasring entfällt, die Scheibe sitzt direkt auf dem Gehäuse."],
  ["Sight ring, hub and welding seam.", "Schauglasring, Nabe und Schweißnaht."],
  ["Spiral drawn with 1.1 turns so that the arms stay 0.3 mm apart.", "Die Spirale ist mit 1,1 Windungen gezeichnet, damit ihre Arme 0,3 mm Abstand behalten."],
  ["The disc is the scroll region and still turns with the rotor bone.", "Die Scheibe ist die Spiralregion und dreht sich weiter mit dem Rotorknochen."],
  { body: 0.18, detail: 0.13, texture: null, glazing: null, ground: 0.25 },
));
const rep200 = rep(200, [150, 350], { body: 0.13, detail: null, texture: null, glazing: null, ground: 0.25 }, [
  rect("discharge-stub", "dome", "body", 3.2, true, 0.7, -0.02, 0.396, 0.04, 0.034, { region: "discharge", material: "copper" }),
  path("dome", "dome", "body", 3.3, true, 0.7, DOME, { region: "dome", material: "steel-shell" }),
  rect("shell", "shell", "body", 3.4, true, 0.5, -0.1, 0.04, 0.2, 0.3, { regions: ["shell", "feet", "scroll"], material: "steel-shell" }),
  rect("suction-stub", "shell", "body", 3.2, true, 0.2, -0.16, 0.185, 0.062, 0.04, { region: "suction", material: "copper" }),
  rect("terminal", "terminal", "body", 3.5, true, 0.3, 0.1, 0.25, 0.05, 0.06, { region: "terminal", material: "pa-plastic" }),
  rect("foot-plate", "shell", "body", 3.5, true, 0.4, -0.14, 0, 0.28, 0.04, { region: "feet", material: "steel-shell" }),
], dec(
  ["A 2.2 mm tall compressor of six shapes.", "Ein 2,2 mm hoher Verdichter aus sechs Formen."],
  ["Shell, dome, two stubs, terminal and foot plate.", "Gehäuse, Dom, zwei Stutzen, Anschlusskasten und Fußplatte."],
  ["The sight glass is merged into the shell shape (regions shell, feet, scroll).", "Das Schauglas geht in der Gehäuseform auf (Regionen Gehäuse, Füße, Spirale)."],
  ["Sight glass with scroll, any stub detail.", "Schauglas mit Spirale, jedes Stutzendetail."],
  ["Stubs are drawn 40 mm thick, the foot plate 40 mm.", "Die Stutzen sind 40 mm dick gezeichnet, die Fußplatte 40 mm."],
  ["The shell shape lists shell, feet and scroll as merged regions.", "Die Gehäuseform führt Gehäuse, Füße und Spirale als zusammengeführte Regionen."],
  { body: 0.13, detail: null, texture: null, glazing: null, ground: 0.25 },
));

const regions = [
  reg("shell", ["Motor shell", "Motorgehäuse"], "engine", "operating", "steel-shell", 0.1, 0.31, "cylinder", 0.003, 0.00006, 9000, { operating: { key: "run", heatW: 40 } }),
  reg("dome", ["Discharge dome", "Druckdom"], "exhaust", "operating", "steel-shell", 0.05, 0.2, "horizontal-plate-up", 0.003, 0.00006, 7000, { operating: { key: "run", heatW: 40 } }),
  reg("discharge", ["Discharge stub", "Druckstutzen"], "exhaust", "operating", "copper", 0.004, 0.03, "cylinder", 0.001, 0.00001, 800, { operating: { key: "run", heatW: 3 } }),
  reg("suction", ["Suction stub", "Saugstutzen"], "surface", "operating", "copper", 0.005, 0.05, "cylinder", 0.001, 0.00001, 800, { operating: { key: "run", heatW: -0.8 } }),
  reg("terminal", ["Terminal box", "Anschlusskasten"], "surface", "passive", "pa-plastic", 0.008, 0.06, "vertical-plate", 0.003, 0.02, 1500, {}, { layerKind: "polymer" }),
  reg("feet", ["Foot plate", "Fußplatte"], "surface", "passive", "steel-shell", 0.02, 0.28, "horizontal-plate-up", 0.004, 0.00008, 5000, {}),
  reg("scroll", ["Scroll in the sight glass", "Spirale im Schauglas"], "engine", "operating", "cast-aluminium", 0.006, 0.07, "vertical-plate", 0.01, 0.00005, 4000, { operating: { key: "run", heatW: 3 } }),
];
const bind = bindOf({
  shell: { exposedFraction: 0.9, contact: { fraction: 0.1, tempC: 21 } },
  dome: { exposedFraction: 1 },
  discharge: { exposedFraction: 1 },
  suction: { exposedFraction: 1 },
  terminal: { exposedFraction: 1 },
  feet: { exposedFraction: 0.6, contact: { fraction: 0.4, tempC: 21 } },
  scroll: { exposedFraction: 1 },
});
const sup = ["base-contact"];
const poses = [
  pose("off", ["Off", "Aus"], ["The compressor stands still and everything is at room temperature.", "Der Verdichter steht still und alles hat Raumtemperatur."], {}, sup, "auto", bind({ run: 0 }, { "dome-plume": 0 })),
  pose("part-load", ["Part load", "Teillast"], ["The motor turns at half speed: dome and shell warm, the suction line cools.", "Der Motor dreht mit halber Drehzahl: Dom und Gehäuse erwärmen sich, die Saugleitung kühlt ab."], { rotor: { angleDeg: 40 } }, sup, "auto", bind({ run: 0.5 }, { "dome-plume": 0.5 })),
  pose("full-load", ["Full load", "Volllast"], ["The motor runs at full speed: the discharge side runs hot, the suction side cold.", "Der Motor läuft mit voller Drehzahl: die Druckseite läuft heiß, die Saugseite kalt."], { rotor: { angleDeg: 80 } }, sup, "auto", bind({ run: 1 }, { "dome-plume": 1 })),
];
const clips = [
  clip("spin", ["Orbiting scroll", "Orbitierende Spirale"], ["The scroll in the sight glass turns three times in three seconds (the real shaft turns 50 times per second, shown slowed).", "Die Spirale im Schauglas dreht sich in drei Sekunden dreimal (die reale Welle dreht 50 Mal je Sekunde, verlangsamt gezeigt)."],
    3, 15,
    [{ t: 0, pose: "full-load", ease: "linear", bones: { rotor: { angleDeg: 0 } } }, { t: 3, pose: "full-load", ease: "linear", bones: { rotor: { angleDeg: 1080 } } }],
    contacts(sup, 3), [["run", 0, 3, ["Running", "Läuft"]]], [], []),
  clip("start-up", ["Start up", "Anlaufen"], ["The compressor starts and runs for five minutes: the dome heats, the suction stub cools.", "Der Verdichter startet und läuft fünf Minuten: der Dom erwärmt sich, der Saugstutzen kühlt ab."],
    300, 1,
    [{ t: 0, pose: "off", ease: "step" }, { t: 5, pose: "off", ease: "smooth" }, { t: 10, pose: "full-load", ease: "linear" }, { t: 300, pose: "full-load", ease: "linear", bones: { rotor: { angleDeg: 3600 } } }],
    contacts(sup, 300), [["off", 0, 5, ["Off", "Aus"]], ["ramp", 5, 10, ["Ramp up", "Hochlauf"]], ["run", 10, 300, ["Running", "Läuft"]]],
    [[5, "ignition", ["Motor started", "Motor gestartet"], "run"]],
    [["operating:run", "smooth", [[0, 0], [5, 0], [10, 1]]], ["emitter:dome-plume", "smooth", [[0, 0], [5, 0], [10, 1]]]]),
];

export const compressor = () =>
  mkAsset({
    id: "scroll-compressor",
    category: "machinery",
    title: ["Hermetic Scroll Compressor", "Hermetischer Scrollverdichter"],
    summary: [
      "A 0.43 m hermetic scroll compressor of a small heat pump with a round sight glass onto the orbiting scroll: the discharge dome runs hot, the suction stub stays cold and the motor shell warms.",
      "Ein 0,43 m hoher hermetischer Scrollverdichter einer kleinen Wärmepumpe mit rundem Schauglas auf die orbitierende Spirale: der Druckdom läuft heiß, der Saugstutzen bleibt kalt und das Motorgehäuse erwärmt sich.",
    ],
    tags: ["compressor", "scroll", "heat-pump", "refrigeration", "hot-gas", "machinery"],
    references: [EN_14511, ASHRAE, ref("copeland-2015", "Emerson Climate Technologies (2015). Copeland Scroll compressors for refrigeration and air conditioning, application engineering bulletin (shell temperatures: discharge dome 80 to 110 degC, suction line cold).")],
    notes: [
      "A hermetic scroll compressor of the 4 to 6 kW class of an air-to-water heat pump: a welded steel shell 0.2 m across and 0.31 m high with a domed top and a discharge stub, a suction stub at the left, a terminal box at the right and a foot plate on rubber grommets. The round sight glass of the asset is an authored device showing the orbiting scroll, so that the cycle clips can show the compression; real hermetic scroll shells have no sight window. The mass 12.5 kg comes from shell and feet 9.4, dome 2.6, terminal 0.3 and scroll 0.2 kg. The dome and the shell release 40 W each into the room at full speed (surfaces of 80 and 60 degC, authored estimates after Copeland 2015); the suction side is 5 degC.",
      "Ein hermetischer Scrollverdichter der Klasse 4 bis 6 kW einer Luft-Wasser-Wärmepumpe: ein geschweißtes Stahlgehäuse von 0,2 m Durchmesser und 0,31 m Höhe mit gewölbtem Dom und einem Druckstutzen, einem Saugstutzen links, einem Anschlusskasten rechts und einer Fußplatte auf Gummitüllen. Das runde Schauglas des Assets ist ein erfundenes Bauteil, das die orbitierende Spirale zeigt, damit die Kreislauf-Clips die Verdichtung zeigen können; echte hermetische Scrollgehäuse haben kein Schauglas. Die Masse von 12,5 kg ergibt sich aus Gehäuse und Füßen 9,4, Dom 2,6, Anschlusskasten 0,3 und Spirale 0,2 kg. Dom und Gehäuse geben bei voller Drehzahl je 40 W an den Raum ab (Oberflächen von 80 und 60 degC, Schätzwerte nach Copeland 2015); die Saugseite hat 5 degC.",
    ],
    dims: [0.43, 0.3, 0.28],
    materials: [
      materialOf("steel-shell", "Painted steel", "Lackierter Stahl", 0.9, 0.8, 7800, 50, 480, "literature", "incropera-2018"),
      materialOf("copper", "Copper pipe", "Kupferrohr", 0.05, 0.3, 8900, 390, 385, "literature", "incropera-2018"),
      materialOf("pa-plastic", "Glass-filled polyamide", "Glasfaserverstärktes Polyamid", 0.9, 0.7, 1400, 0.3, 1600, SRC),
      materialOf("cast-aluminium", "Cast aluminium", "Aluminiumguss", 0.2, 0.4, 2700, 160, 900, "literature", "incropera-2018"),
    ],
    root: "shell",
    bones: sp.bones,
    anchors: [baseContact("shell", 0, 0, 0.26, 0.26), sp.anchor("dome-top", "dome", 0, 0.43, "emitter", ["Top of the discharge stub", "Oberkante des Druckstutzens"])],
    reps: { 500: u500("the 28 mm discharge stub", "der 28 mm dicke Druckstutzen", 0.028, 0.6), 200: rep200, 100: rep100, 50: rep50 },
    poses,
    clips,
    operating: [{ id: "run", title: tx("Compressor speed", "Verdichterdrehzahl"), levels: [level(0, "Off", "Aus"), level(0.5, "Part load", "Teillast"), level(1, "Full load", "Volllast")] }],
    regions,
    emitters: [{ id: "dome-plume", kind: "plume", title: tx("Plume above the hot dome", "Fahne über dem heißen Dom"), anchor: "dome-top", directionDeg: 90, diameterM: 0.12, region: "dome", source: SRC, reference: "incropera-2018" }],
    assumptions: [
      ["The 80 W released to the room are the convective and radiative losses of dome and shell; the refrigerant power of the cycle is carried by the pipes, not by this asset.", "Die 80 W an den Raum sind die konvektiven und Strahlungsverluste von Dom und Gehäuse; die Kältemittelleistung des Kreislaufs tragen die Rohre, nicht dieses Asset."],
    ],
    structure: [["The 12.5 kg compressor stands on its foot plate: a unilateral contact of 0.26 m by 0.26 m carries the weight.", "Der 12,5 kg schwere Verdichter steht auf seiner Fußplatte: ein einseitiger Kontakt von 0,26 m mal 0,26 m trägt das Gewicht."]],
    defaultPose: "full-load",
    defaultClip: "spin",
  });
