import { ALL_EXPORTS, INCROPERA, ISO10456, authored, clip, level, pose, ref, tx, unavailable, wrap } from "./lib.mjs";
import { materialOf, space, sum } from "./space.mjs";
import { SRC, circlePts, exp, regionOf, single } from "./geo.mjs";

const m = (kg, comX, comY, reference) => ({ kg, comX, comY, distribution: "point", source: SRC, ...(reference ? { reference } : {}) });
const fixed = { type: "fixed" };
const YC = 0.0575;

const sp = space([
  ["housing", null, 0, YC, { title: tx("Pump casing with unions and pipe stubs", "Pumpengehäuse mit Verschraubungen und Rohrstutzen"), lengthM: 0.115, joint: fixed, mass: m(1.5, 0, 0) }],
  ["motor", "housing", 0, YC, { title: tx("Wet-rotor motor head", "Nassläufer-Motorkopf"), lengthM: 0.09, joint: fixed, mass: m(0.9, 0, 0) }],
  ["terminal", "motor", 0, 0.1075, { title: tx("Terminal box", "Klemmenkasten"), lengthM: 0.0425, joint: fixed, mass: m(0.25, 0, 0.02) }],
  ["rotor", "motor", 0, YC, { title: tx("Rotor indicator behind the window", "Rotoranzeige hinter dem Fenster"), lengthM: 0.025, joint: { type: "revolute" }, mass: m(0.12, 0, 0) }],
]);
const { poly, rect } = sp;

const disc = (id, bone, role, z, tone, r, opts, cy = YC, n = 48) => poly(id, bone, role, z, true, tone, circlePts(0, cy, r, n), true, opts);
const ring = (id, bone, role, z, r, n = 40) => poly(id, bone, role, z, false, 0.5, circlePts(0, YC, r, n), true);
const pointer = (id, bone, z, w, len, opts) => poly(id, bone, "detail", z, true, 0.95, [[-w / 2, YC], [w / 2, YC], [0, YC + len]], true, opts);

const rep50 = authored(
  50,
  [20, 75],
  { body: 0.18, detail: 0.13, texture: null, glazing: 0.1, ground: 0.35 },
  [
    rect("stub-l", "housing", "body", 3.02, true, 0.7, -0.09, 0.0435, 0.015, 0.028, { region: "casing", material: "copper" }),
    rect("stub-r", "housing", "body", 3.02, true, 0.7, 0.075, 0.0435, 0.015, 0.028, { region: "casing", material: "copper" }),
    rect("union-l", "housing", "body", 3.1, true, 0.5, -0.0775, 0.0365, 0.0235, 0.042, { region: "casing", material: "brass" }),
    rect("union-r", "housing", "body", 3.1, true, 0.5, 0.054, 0.0365, 0.0235, 0.042, { region: "casing", material: "brass" }),
    disc("casing", "housing", "body", 3, 0.42, 0.0575, { region: "casing", material: "cast-iron" }),
    poly("nut-edge-l", "housing", "detail", 3.15, false, 0.3, [[-0.0645, 0.0395], [-0.0645, 0.0755]], false),
    poly("nut-edge-r", "housing", "detail", 3.15, false, 0.3, [[0.0645, 0.0395], [0.0645, 0.0755]], false),
    disc("motor-head", "motor", "body", 3.3, 0.62, 0.0455, { region: "motor", material: "aluminium" }),
    ring("motor-ring", "motor", "detail", 3.31, 0.0405),
    disc("window", "motor", "body", 3.4, 0.18, 0.0265, { region: "rotor-window", material: "window-glass" }),
    ring("bezel", "motor", "detail", 3.41, 0.0305, 36),
    rect("terminal-box", "terminal", "body", 3.5, true, 0.78, -0.03, 0.1075, 0.06, 0.0425, { region: "terminal-box", material: "polyamide", r: 0.004 }),
    poly("lid-seam", "terminal", "detail", 3.51, false, 0.3, [[-0.03, 0.1325], [0.03, 0.1325]], false),
    rect("gland", "terminal", "detail", 3.45, true, 0.4, -0.0365, 0.1115, 0.0065, 0.0145),
    disc("led", "terminal", "detail", 3.52, 0.95, 0.0045, {}, 0.1405, 16),
    pointer("pointer", "rotor", 3.6, 0.014, 0.026, { material: "window-glass" }),
    poly("tail", "rotor", "detail", 3.6, true, 0.7, [[-0.005, YC], [0.005, YC], [0, YC - 0.012]], true, { material: "window-glass" }),
    disc("hub", "rotor", "detail", 3.7, 0.35, 0.0055, { material: "window-glass" }, YC, 16),
  ],
  {
    silhouette: ["A 0.18 m by 0.15 m front view along the motor shaft, 3.6 mm by 3 mm on paper: the round volute casing with a union and a pipe stub on each side, the smaller motor head in front of it with the round rotor window, and the terminal box on top.", "Eine Vorderansicht von 0,18 m mal 0,15 m längs der Motorwelle, auf Papier 3,6 mm mal 3 mm: das runde Spiralgehäuse mit einer Verschraubung und einem Rohrstutzen an jeder Seite, davor der kleinere Motorkopf mit dem runden Rotorfenster und oben der Klemmenkasten."],
    retained: ["Casing disc of 115 mm, both brass unions with their hex edge, the copper stubs of 28 mm, the motor head of 91 mm with its inner ring, the rotor window of 53 mm with bezel, the pointer with its tail and hub, the terminal box with lid seam, cable gland and indicator.", "Gehäusescheibe von 115 mm, beide Messingverschraubungen mit ihrer Sechskantkante, die Kupferstutzen von 28 mm, der Motorkopf von 91 mm mit seinem Innenring, das Rotorfenster von 53 mm mit Blende, der Zeiger mit Gegenstück und Nabe, der Klemmenkasten mit Deckelfuge, Kabelverschraubung und Anzeige."],
    merged: ["Stubs and unions are separate shapes of the same region casing; the volute behind the casing disc is not drawn.", "Stutzen und Verschraubungen sind getrennte Formen derselben Region Gehäuse; die Spirale hinter der Gehäusescheibe ist nicht gezeichnet."],
    omitted: ["Flange screws, the vent plug, the name plate and the rotor can inside the water.", "Flanschschrauben, die Entlüftungsschraube, das Typenschild und der Spaltrohrtopf im Wasser."],
    thinElements: ["The pointer is 14 mm wide (0.28 mm on paper), the hub 11 mm (0.22 mm); the nut edges and the lid seam are 0.13 mm detail strokes.", "Der Zeiger ist 14 mm breit (0,28 mm auf Papier), die Nabe 11 mm (0,22 mm); Mutterkanten und Deckelfuge sind 0,13-mm-Detailstriche."],
    hatching: ["None; the casing is a mid tone, the motor head lighter, the window dark.", "Keine; das Gehäuse ist ein mittlerer Ton, der Motorkopf heller, das Fenster dunkel."],
    styles: ["Casing 0.42, unions 0.5, stubs 0.7, motor head 0.62, window 0.18, terminal box 0.78, pointer 0.95; body pen 0.18 mm, detail 0.13 mm.", "Gehäuse 0,42, Verschraubungen 0,5, Stutzen 0,7, Motorkopf 0,62, Fenster 0,18, Klemmenkasten 0,78, Zeiger 0,95; Körperstift 0,18 mm, Detail 0,13 mm."],
    overlays: ["Casing, motor, terminal box and window are separate regions: in the thermal view the motor head warms most, the casing follows the water, the window stays cool.", "Gehäuse, Motor, Klemmenkasten und Fenster sind getrennte Regionen: in der Thermalansicht erwärmt sich der Motorkopf am stärksten, das Gehäuse folgt dem Wasser, das Fenster bleibt kühl."],
  },
);

const rep100 = authored(
  100,
  [75, 150],
  { body: 0.18, detail: null, texture: null, glazing: null, ground: 0.25 },
  [
    rect("stub-l", "housing", "body", 3.02, true, 0.6, -0.09, 0.0405, 0.035, 0.034, { region: "casing", material: "brass" }),
    rect("stub-r", "housing", "body", 3.02, true, 0.6, 0.055, 0.0405, 0.035, 0.034, { region: "casing", material: "brass" }),
    disc("casing", "housing", "body", 3, 0.42, 0.0575, { regions: ["casing", "motor"], material: "cast-iron" }, YC, 36),
    disc("window", "motor", "body", 3.4, 0.18, 0.026, { region: "rotor-window", material: "window-glass" }, YC, 28),
    rect("terminal-box", "terminal", "body", 3.5, true, 0.78, -0.03, 0.1075, 0.06, 0.0425, { region: "terminal-box", material: "polyamide", r: 0.004 }),
  ],
  {
    silhouette: ["A 1.8 mm by 1.5 mm front view: a round casing with a dark window, a box on top and a thick stub on each side.", "Eine Vorderansicht von 1,8 mm mal 1,5 mm: ein rundes Gehäuse mit dunklem Fenster, oben ein Kasten und an jeder Seite ein dicker Stutzen."],
    retained: ["Casing disc of 115 mm, the dark rotor window of 52 mm, the terminal box of 60 mm by 43 mm and the two pipe stubs with their unions.", "Gehäusescheibe von 115 mm, das dunkle Rotorfenster von 52 mm, der Klemmenkasten von 60 mm mal 43 mm und die beiden Rohrstutzen mit ihren Verschraubungen."],
    merged: ["Motor head and casing share one disc (regions casing and motor merged); union and stub become one block of 34 mm height per side.", "Motorkopf und Gehäuse teilen eine Scheibe (Regionen Gehäuse und Motor zusammengeführt); Verschraubung und Stutzen werden zu einem Block von 34 mm Höhe je Seite."],
    omitted: ["Motor ring, bezel, nut edges, lid seam, gland, indicator, pointer, tail and hub: the rotor indicator is too fine (0.14 mm) and is not drawn.", "Motorring, Blende, Mutterkanten, Deckelfuge, Verschraubung, Anzeige, Zeiger, Gegenstück und Nabe: die Rotoranzeige ist zu fein (0,14 mm) und nicht gezeichnet."],
    thinElements: ["The stubs are thickened from 28 to 34 mm (0.34 mm on paper) and carry the unions.", "Die Stutzen sind von 28 auf 34 mm (0,34 mm auf Papier) verdickt und tragen die Verschraubungen."],
    hatching: ["None.", "Keine."],
    styles: ["Casing 0.42, window 0.18, stubs 0.6, terminal box 0.78; one body pen of 0.18 mm.", "Gehäuse 0,42, Fenster 0,18, Stutzen 0,6, Klemmenkasten 0,78; ein Körperstift von 0,18 mm."],
    overlays: ["The disc lists casing and motor as merged regions, so the motor heat still shows; the rotor does not turn visibly at this scale.", "Die Scheibe führt Gehäuse und Motor als zusammengeführte Regionen, sodass die Motorwärme sichtbar bleibt; der Rotor dreht sich in diesem Maßstab nicht sichtbar."],
  },
);

const rep200 = authored(
  200,
  [150, 350],
  { body: 0.13, detail: null, texture: null, glazing: null, ground: 0.2 },
  [
    rect("pipe", "housing", "body", 3.02, true, 0.6, -0.09, 0.0385, 0.18, 0.039, { region: "casing", material: "brass" }),
    disc("casing", "housing", "body", 3, 0.42, 0.0575, { regions: ["casing", "motor", "rotor-window"], material: "cast-iron" }, YC, 28),
    rect("terminal-box", "terminal", "body", 3.5, true, 0.78, -0.03, 0.1075, 0.06, 0.0425, { region: "terminal-box", material: "polyamide" }),
  ],
  {
    silhouette: ["A 0.9 mm by 0.75 mm mark: a disc with a box on top and a pipe through it.", "Ein Zeichen von 0,9 mm mal 0,75 mm: eine Scheibe mit einem Kasten oben und einem Rohr hindurch."],
    retained: ["The 115 mm casing disc, the 60 mm terminal box and the pipe of 180 mm length through the casing.", "Die Gehäusescheibe von 115 mm, der Klemmenkasten von 60 mm und das Rohr von 180 mm Länge durch das Gehäuse."],
    merged: ["Casing, motor head and window form one disc (three regions merged); unions and stubs form one pipe bar.", "Gehäuse, Motorkopf und Fenster bilden eine Scheibe (drei Regionen zusammengeführt); Verschraubungen und Stutzen bilden einen Rohrbalken."],
    omitted: ["Window, rotor indicator, motor ring, bezel, gland, indicator and all edges.", "Fenster, Rotoranzeige, Motorring, Blende, Verschraubung, Anzeige und alle Kanten."],
    thinElements: ["The pipe is thickened from 28 to 39 mm (0.2 mm on paper) so that it stays a closed fill.", "Das Rohr ist von 28 auf 39 mm (0,2 mm auf Papier) verdickt, damit es eine geschlossene Fläche bleibt."],
    hatching: ["None.", "Keine."],
    styles: ["Casing 0.42, pipe 0.6, terminal box 0.78; one body pen of 0.13 mm.", "Gehäuse 0,42, Rohr 0,6, Klemmenkasten 0,78; ein Körperstift von 0,13 mm."],
    overlays: ["The disc lists casing, motor and window as merged regions; the terminal box stays separate.", "Die Scheibe führt Gehäuse, Motor und Fenster als zusammengeführte Regionen; der Klemmenkasten bleibt getrennt."],
  },
);

const rep500 = unavailable(
  500,
  ["At 1:500 the 0.18 m by 0.15 m pump is 0.36 mm by 0.3 mm, a dot below every print profile; its 60 mm terminal box would be 0.12 mm.", "Bei 1:500 ist die 0,18 m mal 0,15 m große Pumpe 0,36 mm mal 0,3 mm groß, ein Punkt unter jedem Druckprofil; ihr 60 mm breiter Klemmenkasten wäre 0,12 mm."],
  ["Terminal box of 60 mm", "Klemmenkasten von 60 mm"],
  0.06,
  [{ kind: "scale", denominator: 200 }, { kind: "scale", denominator: 100 }],
);

const regions = [
  regionOf("casing", ["Pump casing, unions and stubs", "Pumpengehäuse, Verschraubungen und Stutzen"], "surface", "operating", "cast-iron", 0.035, 0.115, "vertical-plate", [single("shell", "Painted cast iron 5 mm", "Lackiertes Gusseisen 5 mm", "metal", "cast-iron", 0.005, 0.0001, 16600, "incropera-2018")], 16600, { operating: { key: "power", heatW: 10 } }, "incropera-2018"),
  regionOf("motor", ["Motor head", "Motorkopf"], "surface", "operating", "aluminium", 0.03, 0.09, "vertical-plate", [single("head", "Painted die-cast aluminium 3 mm", "Lackierter Aluminiumdruckguss 3 mm", "metal", "aluminium", 0.003, 0.000013, 7300, "incropera-2018")], 7300, { operating: { key: "power", heatW: 18 } }, "incropera-2018"),
  regionOf("terminal-box", ["Terminal box with electronics", "Klemmenkasten mit Elektronik"], "surface", "operating", "polyamide", 0.012, 0.06, "vertical-plate", [single("box", "Polyamide shell 2.5 mm", "Polyamidschale 2,5 mm", "polymer", "polyamide", 0.0025, 0.012, 3500, "incropera-2018")], 3500, { operating: { key: "power", heatW: 2 } }, "incropera-2018"),
  regionOf("rotor-window", ["Rotor window with indicator", "Rotorfenster mit Anzeige"], "glazing", "passive", "window-glass", 0.0022, 0.053, "vertical-plate", [single("pane", "Polycarbonate pane 2 mm", "Polycarbonatscheibe 2 mm", "polymer", "window-glass", 0.002, 0.01, 2400, "incropera-2018")], 2400, {}, "incropera-2018"),
];

const bind = (power, plume) => ({
  regions: {
    casing: exp(0.9, { fraction: 0.1, tempC: 21 }),
    motor: exp(1),
    "terminal-box": exp(0.8, { fraction: 0.2, tempC: 21 }),
    "rotor-window": exp(1),
  },
  emitters: { "motor-plume": plume },
  operating: { power },
});

const pipe = ["pipe-left", "pipe-right"];
const poses = [
  pose("off", ["Off", "Aus"], ["The pump is switched off: the rotor stands still and the housing is at room temperature.", "Die Pumpe ist ausgeschaltet: der Rotor steht still und das Gehäuse hat Raumtemperatur."], { rotor: { angleDeg: 0 } }, pipe, "authored", bind(0, 0)),
  pose("low", ["Low speed", "Kleine Stufe"], ["The pump runs on the low stage at 5 W: the rotor turns slowly, the housing warms slightly.", "Die Pumpe läuft auf der kleinen Stufe mit 5 W: der Rotor dreht langsam, das Gehäuse erwärmt sich leicht."], { rotor: { angleDeg: 0 } }, pipe, "authored", bind(0.111, 0.3)),
  pose("high", ["High speed", "Große Stufe"], ["The pump runs at the full stage at 45 W: the rotor turns fast, the motor head and the casing release 30 W.", "Die Pumpe läuft auf der großen Stufe mit 45 W: der Rotor dreht schnell, Motorkopf und Gehäuse geben 30 W ab."], { rotor: { angleDeg: 0 } }, pipe, "authored", bind(1, 1)),
];

const contacts = (d) => pipe.map((anchor) => ({ anchor, fromS: 0, toS: d }));
const rot = (a) => ({ rotor: { angleDeg: a } });

const clips = [
  {
    ...clip(
      "run-high",
      ["Running at the full stage", "Läuft auf großer Stufe"],
      ["The rotor indicator turns once in two seconds. The picture is a slow-motion view: the real rotor turns at several thousand revolutions per minute.", "Die Rotoranzeige dreht sich in zwei Sekunden einmal. Das Bild zeigt eine Zeitlupe: der echte Rotor dreht mit mehreren tausend Umdrehungen je Minute."],
      2,
      15,
      [{ t: 0, pose: "high", ease: "linear", bones: rot(0) }, { t: 2, pose: "high", ease: "linear", bones: rot(360) }],
      contacts(2),
      [["run", 0, 2, ["Turning", "Dreht"]]],
      [],
      [],
    ),
    loop: true,
  },
  {
    ...clip(
      "run-low",
      ["Running at the low stage", "Läuft auf kleiner Stufe"],
      ["The rotor indicator turns once in four seconds, half as fast as at the full stage, in slow motion.", "Die Rotoranzeige dreht sich in vier Sekunden einmal, halb so schnell wie auf großer Stufe, in Zeitlupe."],
      4,
      15,
      [{ t: 0, pose: "low", ease: "linear", bones: rot(0) }, { t: 4, pose: "low", ease: "linear", bones: rot(360) }],
      contacts(4),
      [["run", 0, 4, ["Turning slowly", "Dreht langsam"]]],
      [],
      [],
    ),
    loop: true,
  },
  clip(
    "start-and-throttle",
    ["Start, speed up and stop", "Anlaufen, hochschalten und stoppen"],
    ["From standstill the pump starts on the low stage after three seconds, steps to the full stage after eight seconds and is switched off after fourteen seconds; the rotor speeds up and runs down.", "Aus dem Stillstand läuft die Pumpe nach drei Sekunden auf kleiner Stufe an, schaltet nach acht Sekunden auf große Stufe und wird nach vierzehn Sekunden ausgeschaltet; der Rotor beschleunigt und läuft aus."],
    16,
    15,
    [
      { t: 0, pose: "off", ease: "linear", bones: rot(0) },
      { t: 2, pose: "off", ease: "linear", bones: rot(0) },
      { t: 3, pose: "low", ease: "linear", bones: rot(45) },
      { t: 7, pose: "low", ease: "linear", bones: rot(405) },
      { t: 8, pose: "high", ease: "linear", bones: rot(540) },
      { t: 12, pose: "high", ease: "linear", bones: rot(1260) },
      { t: 14, pose: "off", ease: "out", bones: rot(1440) },
      { t: 16, pose: "off", ease: "linear", bones: rot(1440) },
    ],
    contacts(16),
    [["standstill", 0, 2, ["Standstill", "Stillstand"]], ["start", 2, 3, ["Starting", "Anlaufen"]], ["low", 3, 8, ["Low stage", "Kleine Stufe"]], ["high", 8, 14, ["Full stage", "Große Stufe"]], ["run-down", 14, 16, ["Run-down", "Auslaufen"]]],
    [[3, "operating", ["Switched to the low stage", "Auf kleine Stufe geschaltet"], "power"], [8, "operating", ["Switched to the full stage", "Auf große Stufe geschaltet"], "power"], [14, "shutdown", ["Switched off", "Ausgeschaltet"], "power"]],
    [
      ["operating:power", "linear", [[0, 0], [2, 0], [3, 0.111], [7, 0.111], [8, 1], [12, 1], [14, 0], [16, 0]]],
      ["emitter:motor-plume", "linear", [[0, 0], [2, 0], [3, 0.3], [7, 0.3], [8, 1], [12, 1], [14, 0], [16, 0]]],
    ],
  ),
];

export const pump = () => {
  const massKg = sum(sp.bones.map((b) => b.mass.kg));
  return wrap({
    id: "circulation-pump",
    category: "appliances",
    title: tx("Heating Circulation Pump", "Heizungs-Umwälzpumpe"),
    summary: tx(
      "A wet-rotor heating circulator on a pipe section, seen along the motor shaft: round volute casing with unions, motor head with a rotor window and a rotating indicator, terminal box on top; 5 W on the low stage and 45 W at full speed, of which about 30 W warm the room.",
      "Eine Nassläufer-Heizungsumwälzpumpe in einem Rohrstück, gesehen längs der Motorwelle: rundes Spiralgehäuse mit Verschraubungen, Motorkopf mit Rotorfenster und drehender Anzeige, oben der Klemmenkasten; 5 W auf kleiner Stufe und 45 W bei voller Drehzahl, davon erwärmen etwa 30 W den Raum.",
    ),
    tags: ["circulation-pump", "circulator", "wet-rotor", "heating", "hydronic", "pump", "internal-gain", "electrical", "pipe"],
    provenance: {
      author: "Semio staffage content (A9)",
      method: "handcrafted",
      created: "2026-11-01",
      references: [
        ref("en-16297-1-2012", "EN 16297-1:2012 Pumps - Rotodynamic glandless circulator pumps - Part 1: General requirements and procedure for testing and calculation of energy efficiency index (EEI) (installation length 180 mm, power of a high-efficiency circulator)."),
        ref("eu-622-2012", "Commission Regulation (EU) No 622/2012 amending Regulation (EC) No 641/2009 with regard to ecodesign requirements for glandless standalone circulators and glandless circulators integrated in products (energy efficiency index of at most 0.20 from 2015)."),
        INCROPERA,
        ISO10456,
      ],
      notes: tx(
        "Front view along the motor shaft of a DN 25 circulator of the 25-60/180 class: installation length 180 mm between the pipe stubs (EN 16297-1), round volute casing of 115 mm, motor head of 91 mm, a terminal box of 60 mm by 43 mm on top and a round rotor window of 53 mm showing an indicator pointer; height 0.15 m, depth along the shaft 0.13 m (casing, motor head and window) and a mass of about 2.8 kg are rounded from this product class (authored estimate). The origin is the lowest point of the casing on its centre line; the pipe axis lies 57.5 mm above it and the asset hangs on the pipe at both stubs. A high-efficiency permanent-magnet circulator takes about 45 W at full speed and 3 to 5 W at the lowest stage (EEI 0.20 class); the electrical power is the level scale (level 1 = 45 W, low = 5 W). About two thirds of it leave as heat through motor head, casing and electronics into the room (motor head 18 W, casing 10 W, terminal box 2 W at level 1: authored estimate), the rest warms the water. The rotor indicator is an authored simplification: the real rotor is enclosed in the water behind the can and runs at several thousand revolutions per minute, so the clips show it as a pointer in slow motion.",
        "Vorderansicht längs der Motorwelle einer DN-25-Umwälzpumpe der Klasse 25-60/180: Einbaulänge 180 mm zwischen den Rohrstutzen (EN 16297-1), rundes Spiralgehäuse von 115 mm, Motorkopf von 91 mm, oben ein Klemmenkasten von 60 mm mal 43 mm und ein rundes Rotorfenster von 53 mm mit einem Anzeigezeiger; Höhe 0,15 m, Tiefe längs der Welle 0,13 m (Gehäuse, Motorkopf und Fenster) und eine Masse von etwa 2,8 kg sind aus dieser Produktklasse gerundet (Schätzwert). Der Ursprung ist der tiefste Punkt des Gehäuses auf seiner Mittellinie; die Rohrachse liegt 57,5 mm darüber und das Asset hängt an beiden Stutzen im Rohr. Eine hocheffiziente Permanentmagnet-Umwälzpumpe nimmt bei voller Drehzahl etwa 45 W und auf der niedrigsten Stufe 3 bis 5 W auf (Klasse EEI 0,20); die elektrische Leistung ist die Stufenskala (Stufe 1 = 45 W, klein = 5 W). Etwa zwei Drittel davon verlassen Motorkopf, Gehäuse und Elektronik als Wärme in den Raum (Motorkopf 18 W, Gehäuse 10 W, Klemmenkasten 2 W auf Stufe 1: Schätzwert), der Rest erwärmt das Wasser. Die Rotoranzeige ist eine gestalterische Vereinfachung: der echte Rotor läuft abgeschlossen im Wasser hinter dem Spaltrohr mit mehreren tausend Umdrehungen je Minute, daher zeigen die Clips ihn als Zeiger in Zeitlupe.",
      ),
    },
    dimensions: { heightM: 0.15, widthM: 0.18, depthM: 0.13, massKg },
    materials: [
      materialOf("cast-iron", "Painted grey cast iron", "Lackiertes Grauguss", 0.9, 0.6, 7200, 50, 460, "literature", "incropera-2018"),
      materialOf("aluminium", "Painted die-cast aluminium", "Lackierter Aluminiumdruckguss", 0.9, 0.5, 2700, 150, 900, "literature", "incropera-2018"),
      materialOf("polyamide", "Glass-filled polyamide", "Glasfaserverstärktes Polyamid", 0.9, 0.5, 1400, 0.3, 1700, SRC, "incropera-2018"),
      materialOf("window-glass", "Polycarbonate window pane", "Polycarbonat-Fensterscheibe", 0.9, 0.2, 1200, 0.2, 1200, SRC, "incropera-2018"),
      materialOf("copper", "Copper pipe", "Kupferrohr", 0.6, 0.6, 8900, 390, 385, "literature", "incropera-2018"),
      materialOf("brass", "Brass union nut", "Messing-Überwurfmutter", 0.5, 0.5, 8500, 120, 380, "literature", "incropera-2018"),
    ],
    rig: { kind: "mechanical", root: "housing", bones: sp.bones.map((b) => ({ ...b, mass: { ...b.mass } })) },
    anchors: [
      { id: "pipe-left", bone: "housing", x: -0.085, y: 0, kind: "support", title: tx("Left pipe stub (pin)", "Linker Rohrstutzen (Festlager)"), support: "pin", normalDeg: 90, footprintLengthM: 0.02, footprintWidthM: 0.028 },
      { id: "pipe-right", bone: "housing", x: 0.085, y: 0, kind: "support", title: tx("Right pipe stub (roller)", "Rechter Rohrstutzen (Rollenlager)"), support: "roller", normalDeg: 90, footprintLengthM: 0.02, footprintWidthM: 0.028 },
      { id: "motor-top", bone: "terminal", x: 0, y: 0.0425, kind: "emitter", title: tx("Top of the terminal box", "Oberseite des Klemmenkastens") },
    ],
    representations: { 500: rep500, 200: rep200, 100: rep100, 50: rep50 },
    symbols: [],
    poses,
    clips,
    thermal: {
      environment: { airTempC: 21, meanRadiantTempC: 21, relativeHumidity: 0.5, windMS: 0.1, windDirectionDeg: 0, solarWM2: 0, diffuseFraction: 0, pressurePa: 101325 },
      passengers: [],
      operating: [{ id: "power", title: tx("Electrical power", "Elektrische Leistung"), levels: [level(0, "Off", "Aus"), level(0.111, "Low stage (5 W)", "Kleine Stufe (5 W)"), level(1, "Full speed (45 W)", "Volle Drehzahl (45 W)")] }],
      regions,
      emitters: [{ id: "motor-plume", kind: "plume", title: tx("Warm air rising from the motor head", "Warme Luft über dem Motorkopf"), anchor: "motor-top", directionDeg: 90, diameterM: 0.08, region: "motor", source: SRC, reference: "incropera-2018" }],
      assumptions: [
        tx(
          "Of the 45 W at full speed about 30 W leave through the surfaces into the room (motor head 18 W, casing 10 W, electronics 2 W); the remaining 15 W warm the pumped water and appear at the radiators.",
          "Von den 45 W bei voller Drehzahl verlassen etwa 30 W die Oberflächen in den Raum (Motorkopf 18 W, Gehäuse 10 W, Elektronik 2 W); die übrigen 15 W erwärmen das Fördermedium und erscheinen an den Heizkörpern.",
        ),
        tx(
          "The heat of the heating water itself (pipe temperature) is not modelled; the pump is shown as a heat source of its own electrical losses.",
          "Die Wärme des Heizwassers selbst (Rohrtemperatur) ist nicht modelliert; die Pumpe wird als Wärmequelle ihrer eigenen elektrischen Verluste gezeigt.",
        ),
      ],
    },
    structure: {
      gravityMS2: 9.80665,
      dynamics: "quasi-static",
      supports: pipe,
      assumptions: [
        tx(
          "The 2.8 kg pump hangs in the pipe on both union stubs: a pin on the left and a roller on the right carry its weight; the pipe itself is rigid.",
          "Die 2,8 kg schwere Pumpe hängt an beiden Verschraubungsstutzen im Rohr: ein Festlager links und ein Rollenlager rechts tragen ihr Gewicht; das Rohr selbst ist starr.",
        ),
      ],
      clinical: false,
    },
    presentation: { defaultPose: "high", defaultClip: "run-high", defaultScale: 50, viewpoints: ["elevation"] },
    exports: ALL_EXPORTS,
    completion: "authored",
  });
};
