import { ALL_EXPORTS, INCROPERA, authored, clip, layer, level, pose, ref, tx, unavailable, wrap } from "./lib.mjs";
import { materialOf, space, sum } from "./space.mjs";
import { SRC, environment, fixed, m, region, single, supportAnchor } from "./kit.mjs";

const sp = space([
  ["housing", null, 0, 0, { title: tx("Housing with display and electronics", "Gehäuse mit Anzeige und Elektronik"), lengthM: 0.09, joint: fixed, mass: m(0.11, 0, 0) }],
  ["dial", "housing", 0, -0.014, { title: tx("Set-point dial", "Sollwert-Drehknopf"), lengthM: 0.021, joint: { type: "revolute", minDeg: -135, maxDeg: 135 }, mass: m(0.025, 0, 0) }],
]);
const { rect, ellipse } = sp;

const rep50 = authored(
  50,
  [10, 75],
  { body: 0.18, detail: 0.13, texture: null, glazing: 0.13, ground: 0.35 },
  [
    rect("housing", "housing", "body", 3, true, 0.9, -0.045, -0.045, 0.09, 0.09, { r: 0.008, region: "casing", material: "abs-plastic" }),
    rect("display", "housing", "glazing", 3.1, true, 0.12, -0.018, 0.016, 0.036, 0.021, { region: "display", material: "lcd-glass" }),
    ellipse("led", "housing", "detail", 3.1, true, 0.35, 0.0315, 0.0265, 0.0035, 0.0035, { region: "led", material: "led-lens" }),
    ellipse("dial", "dial", "body", 3.2, true, 0.62, 0, -0.014, 0.021, 0.021, { region: "dial", material: "abs-plastic" }),
    rect("pointer", "dial", "detail", 3.3, true, 0.95, -0.0055, -0.01, 0.011, 0.0145, { region: "dial", material: "abs-plastic" }),
  ],
  {
    silhouette: ["A 90 mm square wall thermostat seen face-on, 1.8 mm on paper: a rounded housing with a dark display window, a small lamp and a round set-point dial with a pointer.", "Ein quadratischer Raumthermostat von 90 mm in der Frontansicht, auf Papier 1,8 mm: ein Gehäuse mit gerundeten Ecken, ein dunkles Anzeigefenster, eine kleine Lampe und ein runder Sollwert-Drehknopf mit Zeiger."],
    retained: ["Housing of 0.09 m, display window of 36 by 21 mm, lamp of 7 mm, dial of 42 mm and the 11 mm pointer that turns with the dial bone.", "Gehäuse von 0,09 m, Anzeigefenster von 36 mal 21 mm, Lampe von 7 mm, Drehknopf von 42 mm und der 11 mm breite Zeiger, der sich mit dem Knopf dreht."],
    merged: ["Nothing is merged: housing, display, lamp and dial each carry their own region.", "Nichts wird zusammengefasst: Gehäuse, Anzeige, Lampe und Drehknopf tragen je eine eigene Region."],
    omitted: ["The scale ticks, the digits on the display, the screws and the wall-plate seam.", "Die Skalenstriche, die Ziffern der Anzeige, die Schrauben und die Fuge zur Wandplatte."],
    thinElements: ["The pointer is 11 mm wide (0.22 mm on paper) and filled, so that it stays above the feature width when the dial turns.", "Der Zeiger ist 11 mm breit (0,22 mm auf Papier) und gefüllt, damit er beim Drehen über der Mindestbreite bleibt."],
    hatching: ["None; the display is a dark tone.", "Keine; die Anzeige ist ein dunkler Ton."],
    styles: ["Housing 0.9, dial 0.62, pointer 0.95, display 0.12, lamp 0.35; body pen 0.18 mm, detail and glazing 0.13 mm, ground 0.35 mm.", "Gehäuse 0,9, Drehknopf 0,62, Zeiger 0,95, Anzeige 0,12, Lampe 0,35; Körperstift 0,18 mm, Detail und Verglasung 0,13 mm, Boden 0,35 mm."],
    overlays: ["Housing, display, lamp and dial are four regions: in the thermal view the housing warms with the electronics and the lamp glows when the relay is closed; the dial turns with its bone.", "Gehäuse, Anzeige, Lampe und Drehknopf sind vier Regionen: in der Thermalansicht erwärmt sich das Gehäuse mit der Elektronik, und die Lampe leuchtet bei geschlossenem Relais; der Knopf dreht sich mit seinem Knochen."],
  },
);

const rep100 = authored(
  100,
  [75, 150],
  { body: 0.18, detail: null, texture: null, glazing: null, ground: 0.25 },
  [
    rect("housing", "housing", "body", 3, true, 0.9, -0.045, -0.045, 0.09, 0.09, { r: 0.008, regions: ["casing", "display", "led"], material: "abs-plastic" }),
    ellipse("dial", "dial", "body", 3.2, true, 0.45, 0, -0.014, 0.024, 0.024, { region: "dial", material: "abs-plastic" }),
  ],
  {
    silhouette: ["A 0.9 mm square with a dark round dial.", "Ein Quadrat von 0,9 mm mit einem dunklen runden Drehknopf."],
    retained: ["Housing and the round dial that carries the set point.", "Gehäuse und der runde Drehknopf, der den Sollwert trägt."],
    merged: ["Display and lamp join the housing (it carries their regions); the pointer joins the dial.", "Anzeige und Lampe gehen im Gehäuse auf (es trägt ihre Regionen); der Zeiger geht im Drehknopf auf."],
    omitted: ["Display window, lamp and pointer: the turning of the dial is not drawn at this scale.", "Anzeigefenster, Lampe und Zeiger: das Drehen des Knopfs wird bei diesem Maßstab nicht gezeichnet."],
    thinElements: ["The dial is widened to 48 mm (0.48 mm on paper) so that it prints as a filled disc.", "Der Drehknopf ist auf 48 mm (0,48 mm auf Papier) verbreitert, damit er als gefüllte Scheibe druckt."],
    hatching: ["None.", "Keine."],
    styles: ["Housing 0.9, dial 0.45; one body pen of 0.18 mm.", "Gehäuse 0,9, Drehknopf 0,45; ein Körperstift von 0,18 mm."],
    overlays: ["The housing carries casing, display and lamp; the dial keeps its region.", "Das Gehäuse trägt Gehäuse, Anzeige und Lampe; der Drehknopf behält seine Region."],
  },
);

const unavail = (denominator) =>
  unavailable(
    denominator,
    [`At 1:${denominator} the 90 mm thermostat is ${(90 / denominator).toFixed(2)} mm wide and its dial smaller than every print profile allows; use the 1:100 drawing or a thermostat symbol at this scale.`, `Bei 1:${denominator} ist der 90 mm große Thermostat ${(90 / denominator).toFixed(2).replace(".", ",")} mm breit und sein Drehknopf kleiner, als es jedes Druckprofil zulässt; bei diesem Maßstab dient die Zeichnung 1:100 oder ein Thermostat-Symbol.`],
    ["Dial of 42 mm diameter", "Drehknopf von 42 mm Durchmesser"],
    0.042,
    [{ kind: "scale", denominator: 100 }, { kind: "scale", denominator: 50 }],
  );

const regions = [
  region("casing", ["Housing", "Gehäuse"], "surface", "operating", "abs-plastic", 0.012, 0.09, "vertical-plate", [single("shell", "ABS shell 2 mm", "ABS-Schale 2 mm", "polymer", "abs-plastic", 0.002, 0.012, 2000)], 2000, { operating: { key: "electronics", heatW: 0.7 } }),
  region("display", ["Display window", "Anzeigefenster"], "glazing", "passive", "lcd-glass", 0.00076, 0.036, "vertical-plate", [single("glass", "Glass 1 mm", "Glas 1 mm", "glass", "lcd-glass", 0.001, 0.001, 2000)], 2000),
  region("led", ["Heating-call lamp", "Heizanforderungs-Lampe"], "lamp", "operating", "led-lens", 0.0001, 0.007, "vertical-plate", [single("lens", "Epoxy lens 2 mm", "Epoxidlinse 2 mm", "polymer", "led-lens", 0.002, 0.01, 2000)], 2000, { operating: { key: "electronics", heatW: 0.005 } }),
  region("dial", ["Set-point dial", "Sollwert-Drehknopf"], "surface", "passive", "abs-plastic", 0.0033, 0.042, "vertical-plate", [single("knob", "ABS knob 3 mm", "ABS-Knopf 3 mm", "polymer", "abs-plastic", 0.003, 0.018, 3000)], 3000),
];

const bind = (electronics) => ({
  regions: {
    casing: { exposedFraction: 0.85, sunlitFraction: 0, contact: { fraction: 0.15, tempC: 21 } },
    display: { exposedFraction: 1, sunlitFraction: 0 },
    led: { exposedFraction: 1, sunlitFraction: 0 },
    dial: { exposedFraction: 1, sunlitFraction: 0 },
  },
  emitters: {},
  operating: { electronics },
});

const wall = ["wall-screw-top", "wall-screw-bottom"];
const dial = (celsius) => ({ dial: { angleDeg: Math.round((135 - ((celsius - 5) / 25) * 270) * 10) / 10 } });
const poses = [
  pose("idle", ["Set point reached", "Sollwert erreicht"], ["The dial stands at 20 degrees; the room is warm enough, the relay is open and the electronics idle at 0.14 W.", "Der Knopf steht auf 20 Grad; der Raum ist warm genug, das Relais ist offen und die Elektronik läuft im Leerlauf mit 0,14 W."], dial(20), wall, "authored", bind(0.2)),
  pose("calling", ["Calling for heat", "Wärmeanforderung"], ["The dial stands at 22 degrees; the room is colder than that, the relay is closed, the lamp glows and the thermostat takes 0.7 W.", "Der Knopf steht auf 22 Grad; der Raum ist kälter, das Relais ist geschlossen, die Lampe leuchtet und der Thermostat nimmt 0,7 W auf."], dial(22), wall, "authored", bind(1)),
  pose("set-high", ["Turned up to 24 degrees", "Auf 24 Grad gedreht"], ["The dial stands at 24 degrees and the relay stays closed until the room has caught up.", "Der Knopf steht auf 24 Grad und das Relais bleibt geschlossen, bis der Raum nachgezogen hat."], dial(24), wall, "authored", bind(1)),
];

const contacts = (d) => wall.map((anchor) => ({ anchor, fromS: 0, toS: d }));

const clips = [
  clip(
    "turn-up",
    ["Turning the set point up", "Sollwert hochdrehen"],
    ["The dial rests at 20 degrees, is turned to 24 degrees between 1.5 and 4 seconds, and the relay closes the moment the set point lies above the room temperature.", "Der Knopf steht auf 20 Grad, wird zwischen 1,5 und 4 Sekunden auf 24 Grad gedreht, und das Relais schließt, sobald der Sollwert über der Raumtemperatur liegt."],
    6,
    15,
    [
      { t: 0, pose: "idle", ease: "smooth" },
      { t: 1.5, pose: "idle", ease: "smooth" },
      { t: 4, pose: "set-high", ease: "smooth" },
      { t: 6, pose: "set-high", ease: "smooth" },
    ],
    contacts(6),
    [["rest", 0, 1.5, ["At rest", "In Ruhe"]], ["turn", 1.5, 4, ["Turning", "Drehen"]], ["call", 4, 6, ["Relay closed", "Relais geschlossen"]]],
    [[3.6, "operating", ["Relay closes", "Relais schließt"], "electronics"]],
    [["operating:electronics", "step", [[0, 0.2], [3.6, 1], [6, 1]]]],
  ),
  clip(
    "heating-cycle",
    ["Two-point control cycle", "Zweipunktregelung"],
    ["The relay is closed for 90 seconds while the room warms up, opens for 150 seconds and closes again for the last 60 seconds; the electronics warm and cool a little with it.", "Das Relais ist 90 Sekunden geschlossen, während sich der Raum erwärmt, öffnet 150 Sekunden lang und schließt in den letzten 60 Sekunden wieder; die Elektronik erwärmt und kühlt sich dabei leicht."],
    300,
    1,
    [{ t: 0, pose: "calling", ease: "step" }, { t: 90, pose: "idle", ease: "step" }, { t: 240, pose: "calling", ease: "step" }, { t: 300, pose: "calling", ease: "step" }],
    contacts(300),
    [["on", 0, 90, ["Heating call", "Wärmeanforderung"]], ["off", 90, 240, ["Set point reached", "Sollwert erreicht"]], ["on-again", 240, 300, ["Heating call again", "Erneute Anforderung"]]],
    [[90, "shutdown", ["Relay opens", "Relais öffnet"], "electronics"], [240, "operating", ["Relay closes again", "Relais schließt wieder"], "electronics"]],
    [["operating:electronics", "step", [[0, 1], [90, 0.2], [240, 1], [300, 1]]]],
  ),
];

export const roomThermostat = () => {
  const massKg = Math.round(sum(sp.bones.map((b) => b.mass.kg)) * 1000) / 1000;
  return wrap({
    id: "room-thermostat",
    category: "appliances",
    title: tx("Wall-Mounted Room Thermostat with Dial", "Raumthermostat mit Drehknopf für die Wand"),
    summary: tx(
      "A 90 mm square electronic room thermostat seen face-on: a housing with display window and heating-call lamp and a round set-point dial that turns; its electronics take 0.14 W at idle and 0.7 W while the relay is closed.",
      "Ein quadratischer elektronischer Raumthermostat von 90 mm in der Frontansicht: ein Gehäuse mit Anzeigefenster und Lampe für die Heizanforderung und ein runder Sollwert-Drehknopf, der sich dreht; die Elektronik nimmt 0,14 W im Leerlauf und 0,7 W bei geschlossenem Relais auf.",
    ),
    tags: ["thermostat", "room-thermostat", "heating-control", "wall-mounted", "appliance", "electronics", "internal-gain", "electrical", "dial"],
    provenance: {
      author: "Semio staffage content (G1)",
      method: "handcrafted",
      created: "2026-11-01",
      references: [
        ref("din-en-60730-1-2020", "DIN EN 60730-1:2020 Automatic electrical controls - Part 1: General requirements (room thermostats for heating systems, relay switching, power consumption of the control electronics)."),
        ref("en-14597-2012", "EN 14597:2012 Temperature control devices and temperature limiters for heat generating systems (set-point range and switching differential of room temperature controllers)."),
        INCROPERA,
      ],
      notes: tx(
        "A battery-free electronic room thermostat for flush wall mounting at 1.5 m height seen face-on: a rounded square housing of 0.09 m by 0.09 m and 27 mm depth with a 36 mm by 21 mm display window, a 7 mm heating-call lamp and a 42 mm set-point dial turning over 270 degrees for 5 to 30 degrees Celsius. It stands for the circle on the wall next to the radiator in the heating clips. The origin is the centre of the housing in the wall plane. The control electronics (power supply, microcontroller, relay) take about 0.14 W at idle and 0.7 W with the relay coil and lamp energised, which also warms the housing by a few kelvin; all values are authored estimates for this product class. The dial angle follows from the set point: 135 degrees at 5 degrees Celsius to -135 degrees at 30 degrees Celsius, so 20 degrees Celsius stands at -27 degrees.",
        "Ein batterieloser elektronischer Raumthermostat zur Unterputzmontage in 1,5 m Höhe, frontal gesehen: ein gerundetes quadratisches Gehäuse von 0,09 m mal 0,09 m und 27 mm Tiefe mit einem Anzeigefenster von 36 mm mal 21 mm, einer Lampe für die Heizanforderung von 7 mm und einem Sollwert-Drehknopf von 42 mm, der sich über 270 Grad für 5 bis 30 Grad Celsius dreht. Er steht für den Kreis an der Wand neben dem Heizkörper in den Heizungsclips. Der Ursprung ist die Mitte des Gehäuses in der Wandebene. Die Regelelektronik (Netzteil, Mikrocontroller, Relais) nimmt im Leerlauf etwa 0,14 W und mit erregter Relaisspule und Lampe 0,7 W auf, was das Gehäuse um einige Kelvin erwärmt; alle Werte sind Schätzwerte für diese Produktklasse. Der Knopfwinkel folgt aus dem Sollwert: 135 Grad bei 5 Grad Celsius bis -135 Grad bei 30 Grad Celsius, also steht 20 Grad Celsius bei -27 Grad.",
      ),
    },
    dimensions: { heightM: 0.09, widthM: 0.09, depthM: 0.027, massKg },
    materials: [
      materialOf("abs-plastic", "White ABS plastic", "Weißer ABS-Kunststoff", 0.9, 0.3, 1050, 0.17, 1400, SRC, "incropera-2018"),
      materialOf("lcd-glass", "Display glass", "Anzeigeglas", 0.9, 0.1, 2500, 1.0, 750, SRC, "incropera-2018"),
      materialOf("led-lens", "Epoxy lamp lens", "Epoxid-Lampenlinse", 0.9, 0.2, 1200, 0.2, 1100, SRC, "incropera-2018"),
    ],
    rig: { kind: "mechanical", root: "housing", bones: sp.bones.map((b) => ({ ...b, mass: { ...b.mass } })) },
    anchors: [
      supportAnchor("wall-screw-top", "housing", 0, 0.034, ["Upper screw in the wall box", "Obere Schraube in der Unterputzdose"], "pin", { footprintLengthM: 0.012, footprintWidthM: 0.012 }),
      supportAnchor("wall-screw-bottom", "housing", 0, -0.034, ["Lower screw in the wall box", "Untere Schraube in der Unterputzdose"], "roller", { footprintLengthM: 0.012, footprintWidthM: 0.012, frictionCoefficient: 0.4 }),
    ],
    representations: { 500: unavail(500), 200: unavail(200), 100: rep100, 50: rep50 },
    symbols: [],
    poses,
    clips,
    thermal: {
      environment: environment(),
      passengers: [],
      operating: [{ id: "electronics", title: tx("Control electronics", "Regelelektronik"), levels: [level(0, "Unpowered", "Ohne Spannung"), level(0.2, "Idle, relay open (0.14 W)", "Leerlauf, Relais offen (0,14 W)"), level(1, "Heating call, relay closed (0.7 W)", "Heizanforderung, Relais geschlossen (0,7 W)")] }],
      regions,
      emitters: [],
      assumptions: [
        tx(
          "All of the electronic load ends as heat in the housing: 0.7 W with the relay closed, which holds the surface a few kelvin above the room; the lamp adds 5 mW at its lens.",
          "Die gesamte elektrische Last endet als Wärme im Gehäuse: 0,7 W bei geschlossenem Relais, was die Oberfläche einige Kelvin über dem Raum hält; die Lampe fügt 5 mW an ihrer Linse hinzu.",
        ),
        tx(
          "The sensor sits in the housing, so its reading is biased by the self-heating; the model reports the surface temperatures only and does not model the control loop.",
          "Der Fühler sitzt im Gehäuse, seine Anzeige wird also von der Eigenerwärmung verfälscht; das Modell gibt nur die Oberflächentemperaturen an und bildet den Regelkreis nicht nach.",
        ),
      ],
    },
    structure: {
      gravityMS2: 9.80665,
      dynamics: "quasi-static",
      supports: wall,
      assumptions: [
        tx(
          "The 0.135 kg thermostat hangs on the upper screw (pin) and leans against the wall at the lower screw (roller); the pair carries the weight and the small overturning moment of the 27 mm deep housing.",
          "Der 0,135 kg schwere Thermostat hängt an der oberen Schraube (Festlager) und lehnt an der unteren Schraube an der Wand (Rollenlager); das Paar trägt das Gewicht und das kleine Kippmoment des 27 mm tiefen Gehäuses.",
        ),
      ],
      clinical: false,
    },
    presentation: { defaultPose: "calling", defaultClip: "turn-up", defaultScale: 50, viewpoints: ["elevation"] },
    exports: ALL_EXPORTS,
    completion: "authored",
  });
};
