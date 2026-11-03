import { ALL_EXPORTS, INCROPERA, authored, clip, level, pose, ref, tx, unavailable, wrap } from "../🧰️a10/lib.mjs";
import { materialOf, space, sum } from "../🧰️a10/space.mjs";
import { SRC, fixed, m, region, single, supportAnchors, wallContacts } from "./common.mjs";

const sp = space([
  ["body", null, 0, 0, { title: tx("Meter housing", "Zählergehäuse"), lengthM: 0.27, joint: fixed, mass: m(1.95, 0, 0) }],
  ["pointer", "body", 0, 0.018, { title: tx("Test dial pointer", "Zeiger des Prüfrades"), lengthM: 0.025, joint: { type: "revolute" }, mass: m(0.002, 0, 0) }],
]);
const { rect, ellipse, poly } = sp;

const DIAL = [0, 0.018];
const tick = (k, r0, r1) => {
  const a = (k * Math.PI) / 2;
  return [[DIAL[0] + r0 * Math.cos(a), DIAL[1] + r0 * Math.sin(a)], [DIAL[0] + r1 * Math.cos(a), DIAL[1] + r1 * Math.sin(a)]];
};
const ticks = [0, 1, 2, 3].map((k) => poly(`tick-${k + 1}`, "body", "detail", 3.45, false, 0.2, tick(k, 0.021, 0.027), false, { region: "index", material: "abs-plastic" }));
const digits = Array.from({ length: 8 }, (_, i) => rect(`digit-${i + 1}`, "body", "detail", 3.3, true, i < 5 ? 0.92 : 0.5, -0.0715 + i * 0.0182, 0.0615, 0.0155, 0.022, { region: "index", material: "abs-plastic" }));
const stubs = [-0.055, 0.055].flatMap((x, i) => [
  rect(`stub-${i + 1}`, "body", "body", 2.9, true, 0.5, x - 0.014, 0.11, 0.028, 0.044, { region: "pipes", material: "steel-pipe" }),
  rect(`union-${i + 1}`, "body", "body", 3.0, true, 0.35, x - 0.02, 0.118, 0.04, 0.018, { r: 0.002, region: "pipes", material: "steel-pipe" }),
  rect(`face-${i + 1}`, "body", "detail", 3.05, true, 0.4, x - 0.017, 0.154, 0.034, 0.006, { region: "pipes", material: "steel-pipe" }),
]);

const rep50 = authored(
  50,
  [20, 75],
  { body: 0.18, detail: 0.13, texture: null, glazing: null, ground: 0.35 },
  [
    ...stubs,
    rect("lower-shell", "body", "body", 3, true, 0.55, -0.12, -0.11, 0.24, 0.11, { r: 0.012, region: "shell", material: "die-cast-aluminium" }),
    rect("upper-shell", "body", "body", 3.01, true, 0.6, -0.12, 0, 0.24, 0.11, { r: 0.012, region: "shell", material: "die-cast-aluminium" }),
    rect("seam", "body", "detail", 3.1, true, 0.3, -0.12, -0.006, 0.24, 0.012, { region: "shell", material: "die-cast-aluminium" }),
    rect("index", "body", "body", 3.2, true, 0.8, -0.095, 0.0, 0.19, 0.098, { r: 0.006, region: "index", material: "abs-plastic" }),
    rect("counter", "body", "body", 3.25, true, 0.1, -0.0755, 0.058, 0.151, 0.029, { r: 0.002, region: "index", material: "abs-plastic" }),
    ...digits,
    ellipse("dial", "body", "body", 3.3, true, 0.95, ...DIAL, 0.028, 0.028, { region: "index", material: "abs-plastic" }),
    ...ticks,
    poly("needle", "pointer", "body", 3.5, true, 0.1, [[-0.006, 0.012], [0.006, 0.012], [0.0025, 0.044], [-0.0025, 0.044]], true, { region: "index", material: "abs-plastic" }),
    ellipse("pivot", "pointer", "detail", 3.55, true, 0.1, ...DIAL, 0.006, 0.006, { region: "index", material: "abs-plastic" }),
    rect("plate", "body", "body", 3.2, true, 0.9, -0.07, -0.085, 0.14, 0.045, { r: 0.003, region: "shell", material: "die-cast-aluminium" }),
    poly("plate-line-1", "body", "detail", 3.3, false, 0.25, [[-0.055, -0.1], [0.055, -0.1]], false, { region: "shell", material: "die-cast-aluminium" }),
    poly("plate-line-2", "body", "detail", 3.3, false, 0.25, [[-0.055, -0.115], [0.04, -0.115]], false, { region: "shell", material: "die-cast-aluminium" }),
  ],
  {
    silhouette: ["A 0.24 m by 0.27 m bellows gas meter, 4.8 mm by 5.4 mm on paper: a two-shell aluminium housing with a bead seam, two union-nut pipe stubs on top, a pale index head with an eight-digit counter and a round test dial with a pointer, and a data plate below.", "Ein Balgengaszähler von 0,24 m mal 0,27 m, auf Papier 4,8 mm mal 5,4 mm: ein zweischaliges Aluminiumgehäuse mit Bördelnaht, zwei Rohrstutzen mit Überwurfmuttern oben, ein heller Zählwerkskopf mit achtstelligem Zählwerk und einem runden Prüfrad mit Zeiger, darunter ein Datenschild."],
    retained: ["Both shells, the seam bead, the 28 mm stubs with unions and sealing faces at 110 mm spacing, index head, eight digits (the last three dimmer for the decimals), the 56 mm test dial with four ticks and its pointer, data plate with two text lines.", "Beide Schalen, der Nahtwulst, die 28 mm breiten Stutzen mit Überwurfmuttern und Dichtflächen im Abstand von 110 mm, der Zählwerkskopf, acht Ziffern (die letzten drei matter für die Dezimalen), das 56 mm große Prüfrad mit vier Strichen und seinem Zeiger, das Datenschild mit zwei Textzeilen."],
    merged: ["Housing, seam and plate belong to one die-cast aluminium region; the glass of the index head is part of the index region.", "Gehäuse, Naht und Schild gehören zu einer Alu-Druckguss-Region; das Glas des Zählwerkskopfes gehört zur Zählwerksregion."],
    omitted: ["The mounting bracket, the pulse transmitter, the seal wires and the printed scale numerals.", "Der Montagewinkel, der Impulsgeber, die Plombierdrähte und die gedruckten Skalenziffern."],
    thinElements: ["The pointer is a tapered shape 12 mm wide at the pivot (0.24 mm); the ticks are 0.13 mm lines of 6 mm; digits are 15.5 mm wide (0.31 mm).", "Der Zeiger ist eine sich verjüngende Form, am Drehpunkt 12 mm (0,24 mm) breit; die Striche sind 0,13-mm-Linien von 6 mm; die Ziffern sind 15,5 mm breit (0,31 mm)."],
    hatching: ["None.", "Keine."],
    styles: ["Lower shell 0.55, upper shell 0.6, index 0.8, counter 0.1 with digits 0.92 and 0.5, dial 0.95, stubs 0.5, unions 0.35; body pen 0.18 mm, detail 0.13 mm, ground 0.35 mm.", "Untere Schale 0,55, obere Schale 0,6, Zählwerkskopf 0,8, Zählwerk 0,1 mit Ziffern 0,92 und 0,5, Prüfrad 0,95, Stutzen 0,5, Überwurfmuttern 0,35; Körperstift 0,18 mm, Detail 0,13 mm, Boden 0,35 mm."],
    overlays: ["Shell, index and pipes are three regions; in the thermal view the pipes and the shell sit at room temperature and the pointer turns with the pointer bone.", "Schale, Zählwerkskopf und Rohre sind drei Regionen; in der Thermalansicht liegen Rohre und Schale auf Raumtemperatur, und der Zeiger dreht sich mit dem Zeigerknochen."],
  },
);

const rep100 = authored(
  100,
  [75, 150],
  { body: 0.18, detail: null, texture: null, glazing: null, ground: 0.25 },
  [
    ...stubs.filter((s) => !s.id.startsWith("face")),
    rect("housing", "body", "body", 3, true, 0.58, -0.12, -0.11, 0.24, 0.22, { r: 0.012, region: "shell", material: "die-cast-aluminium" }),
    rect("index", "body", "body", 3.2, true, 0.8, -0.095, 0.0, 0.19, 0.098, { r: 0.006, region: "index", material: "abs-plastic" }),
    rect("counter", "body", "body", 3.25, true, 0.1, -0.0755, 0.058, 0.151, 0.03, { region: "index", material: "abs-plastic" }),
    ellipse("dial", "body", "body", 3.3, true, 0.95, ...DIAL, 0.028, 0.028, { region: "index", material: "abs-plastic" }),
    poly("needle", "pointer", "body", 3.5, true, 0.1, [[-0.006, 0.012], [0.006, 0.012], [0.0025, 0.044], [-0.0025, 0.044]], true, { region: "index", material: "abs-plastic" }),
    rect("plate", "body", "body", 3.2, true, 0.9, -0.07, -0.085, 0.14, 0.045, { region: "shell", material: "die-cast-aluminium" }),
  ],
  {
    silhouette: ["A 2.4 mm by 2.7 mm gas meter: a grey box with two stubs, a pale index head with a dark counter strip and a round dial with a pointer, and a pale plate.", "Ein Gaszähler von 2,4 mm mal 2,7 mm: ein grauer Kasten mit zwei Stutzen, ein heller Zählwerkskopf mit dunklem Zählwerksstreifen und einem runden Prüfrad mit Zeiger, und ein helles Schild."],
    retained: ["Housing, two stubs with unions, index head, counter strip, dial with pointer, data plate.", "Gehäuse, zwei Stutzen mit Überwurfmuttern, Zählwerkskopf, Zählwerksstreifen, Prüfrad mit Zeiger, Datenschild."],
    merged: ["Both shells and the seam are one housing; the digits join the dark strip; the dial ticks and pivot are dropped.", "Beide Schalen und die Naht sind ein Gehäuse; die Ziffern gehen im dunklen Streifen auf; die Striche und der Drehpunkt des Prüfrades entfallen."],
    omitted: ["Seam bead, digits, ticks, pivot, sealing faces and plate text.", "Nahtwulst, Ziffern, Striche, Drehpunkt, Dichtflächen und Schildtext."],
    thinElements: ["The pointer is a tapered shape 12 mm wide (0.24 mm); the stubs are 28 mm (0.28 mm) wide.", "Der Zeiger ist eine sich verjüngende Form von 12 mm (0,24 mm) Breite; die Stutzen sind 28 mm (0,28 mm) breit."],
    hatching: ["None.", "Keine."],
    styles: ["Housing 0.58, index 0.8, counter 0.1, dial 0.95, plate 0.9; one body pen of 0.18 mm.", "Gehäuse 0,58, Zählwerkskopf 0,8, Zählwerk 0,1, Prüfrad 0,95, Schild 0,9; ein Körperstift von 0,18 mm."],
    overlays: ["Shell, index and pipes stay three regions, the pointer turns with its bone.", "Schale, Zählwerkskopf und Rohre bleiben drei Regionen, der Zeiger dreht sich mit seinem Knochen."],
  },
);

const rep200 = authored(
  200,
  [150, 350],
  { body: 0.13, detail: null, texture: null, glazing: null, ground: 0.25 },
  [
    rect("stub-1", "body", "body", 2.9, true, 0.45, -0.075, 0.1, 0.04, 0.06, { region: "pipes", material: "steel-pipe" }),
    rect("stub-2", "body", "body", 2.9, true, 0.45, 0.035, 0.1, 0.04, 0.06, { region: "pipes", material: "steel-pipe" }),
    rect("housing", "body", "body", 3, true, 0.58, -0.12, -0.11, 0.24, 0.22, { r: 0.012, region: "shell", material: "die-cast-aluminium" }),
    rect("index", "body", "body", 3.2, true, 0.9, -0.095, -0.02, 0.19, 0.118, { region: "index", material: "abs-plastic" }),
    rect("counter", "body", "body", 3.25, true, 0.1, -0.075, 0.054, 0.15, 0.04, { region: "index", material: "abs-plastic" }),
    ellipse("dial", "body", "body", 3.3, true, 0.97, 0, 0.018, 0.03, 0.03, { region: "index", material: "abs-plastic" }),
    poly("needle", "pointer", "body", 3.5, true, 0.1, [[-0.007, 0.012], [0.007, 0.012], [0.003, 0.046], [-0.003, 0.046]], true, { region: "index", material: "abs-plastic" }),
  ],
  {
    silhouette: ["A 1.2 mm by 1.35 mm gas meter: a grey box with two stubs, a pale head with a dark counter bar and a round dial with a pointer.", "Ein Gaszähler von 1,2 mm mal 1,35 mm: ein grauer Kasten mit zwei Stutzen, ein heller Kopf mit dunklem Zählwerksbalken und einem runden Prüfrad mit Zeiger."],
    retained: ["Housing, stubs, index head, counter bar, dial and pointer.", "Gehäuse, Stutzen, Zählwerkskopf, Zählwerksbalken, Prüfrad und Zeiger."],
    merged: ["The unions join the stubs, the plate joins the housing; the index head is enlarged downwards by 20 mm to take the dial.", "Die Überwurfmuttern gehen in den Stutzen auf, das Schild geht im Gehäuse auf; der Zählwerkskopf ist um 20 mm nach unten vergrößert, damit er das Prüfrad aufnimmt."],
    omitted: ["Seam, digits, ticks, pivot and plate.", "Naht, Ziffern, Striche, Drehpunkt und Schild."],
    thinElements: ["The stubs are widened to 40 mm (0.2 mm); the counter bar is 40 mm high (0.2 mm); the pointer is a tapered shape 14 mm wide (0.07 mm at the tip, a hairline).", "Die Stutzen sind auf 40 mm (0,2 mm) verbreitert; der Zählwerksbalken ist 40 mm hoch (0,2 mm); der Zeiger ist eine sich verjüngende Form von 14 mm Breite (an der Spitze eine Haarlinie)."],
    hatching: ["None.", "Keine."],
    styles: ["Housing 0.58, head 0.9, counter 0.1, dial 0.97, stubs 0.45; one body pen of 0.13 mm.", "Gehäuse 0,58, Kopf 0,9, Zählwerk 0,1, Prüfrad 0,97, Stutzen 0,45; ein Körperstift von 0,13 mm."],
    overlays: ["Shell, index and pipes stay the three regions.", "Schale, Zählwerkskopf und Rohre bleiben die drei Regionen."],
  },
);

const rep500 = unavailable(
  500,
  ["At 1:500 the 0.24 m meter is 0.48 mm wide and its pointer 0.05 mm, below every print profile; use the dial symbol of the clip or a larger scale.", "Bei 1:500 ist der 0,24 m große Zähler 0,48 mm breit und sein Zeiger 0,05 mm, unter jedem Druckprofil; bei diesem Maßstab dient das Zifferblattsymbol des Clips oder ein größerer Maßstab."],
  ["Test dial pointer of 6 mm", "Zeiger des Prüfrades von 6 mm"],
  0.006,
  [{ kind: "scale", denominator: 200 }, { kind: "scale", denominator: 100 }],
);

const regions = [
  region("shell", ["Aluminium housing and data plate", "Aluminiumgehäuse und Datenschild"], "surface", "passive", "die-cast-aluminium", 0.26, 0.27, "vertical-plate", [single("shell", "Die-cast aluminium 2 mm", "Aluminium-Druckguss 2 mm", "metal", "die-cast-aluminium", 0.002, 0.000009, 2400)], 2400),
  region("index", ["Index head and dial", "Zählwerkskopf und Prüfrad"], "surface", "passive", "abs-plastic", 0.02, 0.19, "vertical-plate", [single("head", "ABS head with 2 mm glass-clear cover", "ABS-Kopf mit 2 mm klarer Abdeckung", "polymer", "abs-plastic", 0.002, 0.012, 2500)], 2500),
  region("pipes", ["Pipe stubs and unions", "Rohrstutzen und Überwurfmuttern"], "surface", "operating", "steel-pipe", 0.014, 0.028, "cylinder", [single("tube", "Steel tube 2.6 mm with gas inside", "Stahlrohr 2,6 mm mit Gas innen", "metal", "steel-pipe", 0.0026, 0.00005, 3600)], 3600, { operating: { key: "flow", heatW: 0.2 } }),
];

const bind = (flow) => ({
  regions: {
    shell: { exposedFraction: 0.8, sunlitFraction: 0, contact: { fraction: 0.2, tempC: 21 } },
    index: { exposedFraction: 1, sunlitFraction: 0 },
    pipes: { exposedFraction: 1, sunlitFraction: 0 },
  },
  emitters: {},
  operating: { flow },
});

const wall = ["wall-bracket-top", "wall-bracket-bottom"];
const poses = [
  pose("off", ["No gas flow", "Kein Gasdurchfluss"], ["No appliance burns gas: the pointer stands still.", "Kein Gerät verbrennt Gas: der Zeiger steht still."], {}, wall, "authored", bind(0)),
  pose("low-flow", ["Low flow", "Kleiner Durchfluss"], ["A hob burns at 1.8 cubic metres per hour: the pointer has turned a little way.", "Ein Kochfeld verbrennt 1,8 Kubikmeter je Stunde: der Zeiger hat sich ein Stück gedreht."], { pointer: { angleDeg: -60 } }, wall, "authored", bind(0.3)),
  pose("rated-flow", ["Rated flow", "Nenndurchfluss"], ["A boiler burns gas at the rated 6 cubic metres per hour of a G4 meter.", "Ein Heizkessel verbrennt Gas mit dem Nenndurchfluss von 6 Kubikmetern je Stunde eines G4-Zählers."], { pointer: { angleDeg: -120 } }, wall, "authored", bind(1)),
];

const clips = [
  clip(
    "run",
    ["Gas flows through", "Gas strömt durch"],
    ["The pointer of the test dial turns clockwise at a steady 180 degrees per second; a real G4 meter at rated flow needs two minutes per turn (0.2 cubic metres per turn), so the clip is shown 60 times faster. A scene can seek into the clip to follow its own gas flow.", "Der Zeiger des Prüfrades dreht sich im Uhrzeigersinn gleichmäßig mit 180 Grad je Sekunde; ein echter G4-Zähler braucht bei Nenndurchfluss zwei Minuten je Umdrehung (0,2 Kubikmeter je Umdrehung), der Clip läuft also 60-fach schneller. Eine Szene kann in den Clip springen, um ihrem eigenen Gasstrom zu folgen."],
    4,
    15,
    [
      { t: 0, pose: "rated-flow", ease: "linear", bones: { pointer: { angleDeg: 0 } } },
      { t: 4, pose: "rated-flow", ease: "linear", bones: { pointer: { angleDeg: -720 } } },
    ],
    wallContacts(wall, 4),
    [["turning", 0, 4, ["Pointer turning", "Zeiger dreht"]]],
    [[0, "operating", ["Gas appliances burning", "Gasgeräte brennen"], "flow"]],
    [["operating:flow", "linear", [[0, 1], [4, 1]]]],
  ),
  clip(
    "day-profile",
    ["Hob, boiler and night", "Kochfeld, Kessel und Nacht"],
    ["A hob runs on a low flow for a minute, the boiler runs at rated flow for two more, then all appliances switch off.", "Ein Kochfeld läuft eine Minute mit kleinem Durchfluss, der Kessel läuft zwei weitere mit Nenndurchfluss, dann schalten alle Geräte ab."],
    360,
    1,
    [{ t: 0, pose: "off", ease: "step" }, { t: 5, pose: "low-flow", ease: "step" }, { t: 65, pose: "rated-flow", ease: "step" }, { t: 185, pose: "off", ease: "step" }, { t: 360, pose: "off", ease: "step" }],
    wallContacts(wall, 360),
    [["off", 0, 5, ["No flow", "Kein Durchfluss"]], ["low", 5, 65, ["Hob", "Kochfeld"]], ["rated", 65, 185, ["Boiler", "Kessel"]], ["night", 185, 360, ["All off", "Alles aus"]]],
    [[185, "shutdown", ["Appliances switched off", "Geräte ausgeschaltet"], "flow"]],
    [["operating:flow", "step", [[0, 0], [5, 0.3], [65, 1], [185, 0]]]],
  ),
];

export const gasMeter = () => {
  const massKg = sum(sp.bones.map((b) => b.mass.kg));
  return wrap({
    id: "gas-meter",
    category: "appliances",
    title: tx("Bellows Gas Meter with Counter and Test Dial", "Balgengaszähler mit Zählwerk und Prüfrad"),
    summary: tx(
      "A 0.24 m by 0.27 m G4 bellows gas meter seen from the front: a two-shell aluminium housing, two union-nut pipe stubs on top, an eight-digit counter and a test dial whose pointer turns with the gas flow.",
      "Ein Balgengaszähler G4 von 0,24 m mal 0,27 m in der Frontansicht: ein zweischaliges Aluminiumgehäuse, zwei Rohrstutzen mit Überwurfmuttern oben, ein achtstelliges Zählwerk und ein Prüfrad, dessen Zeiger sich mit dem Gasdurchfluss dreht.",
    ),
    tags: ["meter", "gas-meter", "bellows-meter", "g4", "final-energy", "billing", "wall-mounted", "appliance", "gas", "rotating"],
    provenance: {
      author: "Semio staffage content (G4)",
      method: "handcrafted",
      created: "2026-11-01",
      references: [
        ref("din-en-1359-2017", "DIN EN 1359:2017 Gas meters - Diaphragm gas meters (G4 size: maximum flow 6 cubic metres per hour, 110 mm connection spacing, cyclic volume of about 2 litres, index with roller counter and test element)."),
        INCROPERA,
      ],
      notes: tx(
        "A domestic G4 diaphragm (bellows) meter seen from the front: a housing of 0.24 m width in two die-cast aluminium shells with a bead seam, two pipe stubs at a spacing of 110 mm with union nuts on top (overall height 0.27 m), an index head with an eight-digit roller counter (five whole cubic metres and three decimals) and a test dial with a pointer, and a data plate. The depth of 0.17 m and the mass of 1.95 kg are rounded from this product class (authored estimate). The origin is the centre of the housing front. The pointer is the pointer bone; one turn stands for 0.2 cubic metres, so at the rated 6 cubic metres per hour it needs 120 seconds, which the clip shows 60 times faster. Gas flowing through the pipes releases no heat of its own; the operating level only gives the pipe stubs 0.2 W of friction and ambient exchange.",
        "Ein häuslicher Balgengaszähler G4 in der Frontansicht: ein 0,24 m breites Gehäuse aus zwei Aluminium-Druckgussschalen mit Bördelnaht, zwei Rohrstutzen im Abstand von 110 mm mit Überwurfmuttern oben (Gesamthöhe 0,27 m), ein Zählwerkskopf mit achtstelligem Rollenzählwerk (fünf ganze Kubikmeter und drei Dezimalen) und einem Prüfrad mit Zeiger, und ein Datenschild. Die Tiefe von 0,17 m und die Masse von 1,95 kg sind aus dieser Produktklasse gerundet (Schätzwert). Der Ursprung ist die Mitte der Gehäusefront. Der Zeiger ist der Zeigerknochen; eine Umdrehung steht für 0,2 Kubikmeter, bei den Nenn-6 Kubikmetern je Stunde braucht er also 120 Sekunden, was der Clip 60-fach schneller zeigt. Das durchströmende Gas setzt keine eigene Wärme frei; die Betriebsstufe gibt den Stutzen nur 0,2 W aus Reibung und Umgebungsaustausch.",
      ),
    },
    dimensions: { heightM: 0.27, widthM: 0.24, depthM: 0.17, massKg },
    materials: [
      materialOf("die-cast-aluminium", "Die-cast aluminium, painted", "Aluminium-Druckguss, lackiert", 0.9, 0.4, 2700, 150, 900, SRC, "incropera-2018"),
      materialOf("abs-plastic", "White ABS plastic", "Weißer ABS-Kunststoff", 0.9, 0.3, 1050, 0.17, 1400, SRC, "incropera-2018"),
      materialOf("steel-pipe", "Steel pipe", "Stahlrohr", 0.3, 0.5, 7800, 50, 490, SRC, "incropera-2018"),
    ],
    rig: { kind: "mechanical", root: "body", bones: sp.bones.map((b) => ({ ...b, mass: { ...b.mass } })) },
    anchors: [
      ...supportAnchors("body", { id: "wall-bracket-top", x: 0, y: 0.1 }, { id: "wall-bracket-bottom", x: 0, y: -0.1 }, [["Upper bracket on the wall", "Oberer Halter an der Wand"], ["Lower bracket on the wall", "Unterer Halter an der Wand"]]),
    ],
    representations: { 500: rep500, 200: rep200, 100: rep100, 50: rep50 },
    symbols: [],
    poses,
    clips,
    thermal: {
      environment: { airTempC: 21, meanRadiantTempC: 21, relativeHumidity: 0.5, windMS: 0.1, windDirectionDeg: 0, solarWM2: 0, diffuseFraction: 0, pressurePa: 101325 },
      passengers: [],
      operating: [{ id: "flow", title: tx("Gas flow", "Gasdurchfluss"), levels: [level(0, "No flow", "Kein Durchfluss"), level(0.3, "Hob (1.8 m3/h)", "Kochfeld (1,8 m3/h)"), level(1, "Rated flow (6 m3/h)", "Nenndurchfluss (6 m3/h)")] }],
      regions,
      emitters: [],
      assumptions: [
        tx(
          "The gas is at room temperature and its flow releases no heat; the 0.2 W at level 1 stands for the friction of the gas in the stubs and the warm pipe from the boiler room, and keeps the operating key bound to a region.",
          "Das Gas hat Raumtemperatur und sein Strom setzt keine Wärme frei; die 0,2 W auf Stufe 1 stehen für die Reibung des Gases in den Stutzen und das warme Rohr aus dem Heizraum und halten den Betriebsschlüssel an eine Region gebunden.",
        ),
      ],
    },
    structure: {
      gravityMS2: 9.80665,
      dynamics: "quasi-static",
      supports: wall,
      assumptions: [
        tx(
          "The 1.95 kg meter hangs on the upper bracket (pin) and leans against the wall at the lower bracket (roller); the pipe connections carry no load.",
          "Der 1,95 kg schwere Zähler hängt am oberen Halter (Festlager) und lehnt am unteren Halter an der Wand (Rollenlager); die Rohranschlüsse tragen keine Last.",
        ),
      ],
      clinical: false,
    },
    presentation: { defaultPose: "rated-flow", defaultClip: "run", defaultScale: 50, viewpoints: ["elevation"] },
    exports: ALL_EXPORTS,
    completion: "authored",
  });
};
