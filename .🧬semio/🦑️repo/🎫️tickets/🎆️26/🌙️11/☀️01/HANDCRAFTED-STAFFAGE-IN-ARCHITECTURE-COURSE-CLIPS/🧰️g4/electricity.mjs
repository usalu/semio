import { ALL_EXPORTS, INCROPERA, authored, clip, level, pose, ref, tx, unavailable, wrap } from "../🧰️a10/lib.mjs";
import { materialOf, space, sum } from "../🧰️a10/space.mjs";
import { SRC, fixed, m, region, single, supportAnchors, wallContacts } from "./common.mjs";

const sp = space([
  ["body", null, 0, 0, { title: tx("Meter housing", "Zählergehäuse"), lengthM: 0.285, joint: fixed, mass: m(0.85, 0, 0) }],
  ["disc", "body", 0, 0.01, { title: tx("Aluminium rotor disc with mark", "Aluminium-Läuferscheibe mit Markierung"), lengthM: 0.042, joint: { type: "revolute" }, mass: m(0.015, 0, 0) }],
]);
const { rect, ellipse, poly } = sp;

const DISC = [0, 0.01];
const digits = (tone5, tone1) =>
  Array.from({ length: 6 }, (_, i) => rect(`digit-${i + 1}`, "body", "detail", 3.3, true, i < 5 ? tone5 : tone1, -0.0425 + i * 0.0145, 0.082, 0.011, 0.018, { region: "window", material: "polycarbonate" }));
const plateLines = [-0.055, -0.065, -0.075].map((y, i) => poly(`plate-line-${i + 1}`, "body", "detail", 3.2, false, 0.25, [[-0.052, y], [0.052, y]], false, { region: "plate", material: "abs-plastic" }));

const rep50 = authored(
  50,
  [20, 75],
  { body: 0.18, detail: 0.13, texture: null, glazing: null, ground: 0.35 },
  [
    rect("body", "body", "body", 3, true, 0.88, -0.085, -0.1425, 0.17, 0.285, { r: 0.012, region: "casing", material: "abs-plastic" }),
    ellipse("eye", "body", "detail", 3.1, true, 0.15, 0, 0.123, 0.008, 0.008, { region: "casing", material: "abs-plastic" }),
    rect("counter", "body", "body", 3.2, true, 0.1, -0.045, 0.078, 0.09, 0.026, { r: 0.003, region: "window", material: "polycarbonate" }),
    ...digits(0.92, 0.55),
    ellipse("rim", "body", "body", 3.1, true, 0.25, ...DISC, 0.05, 0.05, { region: "window", material: "polycarbonate" }),
    ellipse("disc", "disc", "body", 3.4, true, 0.86, ...DISC, 0.042, 0.042, { region: "disc", material: "aluminium-disc" }),
    ellipse("hub", "disc", "detail", 3.5, true, 0.3, ...DISC, 0.006, 0.006, { region: "disc", material: "aluminium-disc" }),
    rect("mark", "disc", "detail", 3.5, true, 0.1, 0.02, 0.005, 0.02, 0.01, { region: "disc", material: "aluminium-disc" }),
    rect("plate", "body", "body", 3.2, true, 0.78, -0.065, -0.085, 0.13, 0.04, { r: 0.003, region: "plate", material: "abs-plastic" }),
    ...plateLines,
    rect("terminals", "body", "body", 3.2, true, 0.55, -0.072, -0.1425, 0.144, 0.0425, { r: 0.004, region: "casing", material: "abs-plastic" }),
    ellipse("screw-left", "body", "detail", 3.3, true, 0.2, -0.058, -0.121, 0.005, 0.005, { region: "casing", material: "abs-plastic" }),
    ellipse("screw-right", "body", "detail", 3.3, true, 0.2, 0.058, -0.121, 0.005, 0.005, { region: "casing", material: "abs-plastic" }),
  ],
  {
    silhouette: ["A 0.17 m by 0.285 m electricity meter, 3.4 mm by 5.7 mm on paper: a white housing with a six-digit counter window on top, a round disc window with a turning aluminium disc below it, a rating plate and a dark terminal cover at the foot.", "Ein Stromzähler von 0,17 m mal 0,285 m, auf Papier 3,4 mm mal 5,7 mm: ein weißes Gehäuse mit einem sechsstelligen Zählwerksfenster oben, darunter ein rundes Scheibenfenster mit einer sich drehenden Aluminiumscheibe, ein Leistungsschild und eine dunkle Klemmenabdeckung am Fuß."],
    retained: ["Housing, hanging eye, counter window with its six digits (the last one dimmer for the tenths), the 100 mm window rim, the 84 mm disc with hub and the dark radial mark that shows the turning, rating plate with three text lines, terminal cover with two sealing screws.", "Gehäuse, Aufhängeauge, Zählwerksfenster mit seinen sechs Ziffern (die letzte matter für die Zehntel), der 100 mm große Fensterrand, die 84 mm große Scheibe mit Nabe und der dunklen radialen Markierung, die das Drehen zeigt, Leistungsschild mit drei Textzeilen, Klemmenabdeckung mit zwei Plombierschrauben."],
    merged: ["The meter cover and the housing are one white form; the glass of both windows is one polycarbonate region.", "Zählerhaube und Gehäuse sind eine weiße Form; das Glas beider Fenster ist eine Polycarbonat-Region."],
    omitted: ["The calibration screw, the optical pulse diode, the sealing wire, the printed scale and the brake magnet.", "Die Justierschraube, die optische Impulsdiode, der Plombierdraht, die aufgedruckte Skala und der Bremsmagnet."],
    thinElements: ["The disc mark is 20 mm by 10 mm (0.4 mm by 0.2 mm); digits are 11 mm wide (0.22 mm); the text lines are 0.13 mm detail pens.", "Die Scheibenmarkierung ist 20 mm mal 10 mm (0,4 mm mal 0,2 mm); die Ziffern sind 11 mm breit (0,22 mm); die Textzeilen sind Linien mit dem 0,13-mm-Detailstift."],
    hatching: ["None; windows are dark tones.", "Keine; die Fenster sind dunkle Töne."],
    styles: ["Housing 0.88, rim 0.25, disc 0.86, mark 0.1, counter 0.1 with digits 0.92 and 0.55, plate 0.78, terminal cover 0.55; body pen 0.18 mm, detail 0.13 mm, ground 0.35 mm.", "Gehäuse 0,88, Rand 0,25, Scheibe 0,86, Markierung 0,1, Zählwerk 0,1 mit Ziffern 0,92 und 0,55, Schild 0,78, Klemmenabdeckung 0,55; Körperstift 0,18 mm, Detail 0,13 mm, Boden 0,35 mm."],
    overlays: ["Casing, window, disc and rating plate are four regions: in the thermal view the plate (measuring system behind it) warms slightly with the load, and the disc turns with the rotor bone.", "Gehäuse, Fenster, Scheibe und Leistungsschild sind vier Regionen: in der Thermalansicht erwärmt sich das Schild (das Messwerk dahinter) mit der Last leicht, und die Scheibe dreht sich mit dem Rotorknochen."],
  },
);

const rep100 = authored(
  100,
  [75, 150],
  { body: 0.18, detail: null, texture: null, glazing: null, ground: 0.25 },
  [
    rect("body", "body", "body", 3, true, 0.88, -0.085, -0.1425, 0.17, 0.285, { r: 0.012, region: "casing", material: "abs-plastic" }),
    rect("counter", "body", "body", 3.2, true, 0.1, -0.045, 0.076, 0.09, 0.03, { r: 0.003, region: "window", material: "polycarbonate" }),
    ellipse("rim", "body", "body", 3.1, true, 0.25, ...DISC, 0.05, 0.05, { region: "window", material: "polycarbonate" }),
    ellipse("disc", "disc", "body", 3.4, true, 0.86, ...DISC, 0.042, 0.042, { region: "disc", material: "aluminium-disc" }),
    rect("mark", "disc", "body", 3.5, true, 0.1, 0.012, 0.0, 0.03, 0.02, { region: "disc", material: "aluminium-disc" }),
    rect("plate", "body", "body", 3.2, true, 0.78, -0.065, -0.085, 0.13, 0.04, { r: 0.003, region: "plate", material: "abs-plastic" }),
    rect("terminals", "body", "body", 3.2, true, 0.55, -0.072, -0.1425, 0.144, 0.0425, { r: 0.004, region: "casing", material: "abs-plastic" }),
  ],
  {
    silhouette: ["A 1.7 mm by 2.85 mm meter: a white housing with a dark counter strip, a round window with a light disc and a dark radial mark, a plate and a terminal cover.", "Ein Zähler von 1,7 mm mal 2,85 mm: ein weißes Gehäuse mit einem dunklen Zählwerksstreifen, ein rundes Fenster mit heller Scheibe und dunkler radialer Markierung, ein Schild und eine Klemmenabdeckung."],
    retained: ["Housing, counter window, disc window with disc and mark, rating plate, terminal cover.", "Gehäuse, Zählwerksfenster, Scheibenfenster mit Scheibe und Markierung, Leistungsschild, Klemmenabdeckung."],
    merged: ["The six digits join the dark counter strip; the hub joins the disc.", "Die sechs Ziffern gehen im dunklen Zählwerksstreifen auf; die Nabe geht in der Scheibe auf."],
    omitted: ["Hanging eye, digits, hub, text lines and sealing screws.", "Aufhängeauge, Ziffern, Nabe, Textzeilen und Plombierschrauben."],
    thinElements: ["The disc mark is widened to 30 mm by 20 mm (0.3 mm by 0.2 mm) so that it prints as a filled shape and shows the turn.", "Die Scheibenmarkierung ist auf 30 mm mal 20 mm (0,3 mm mal 0,2 mm) verbreitert, damit sie als Fläche druckt und das Drehen zeigt."],
    hatching: ["None.", "Keine."],
    styles: ["Same tones as 1:50 without digits; one body pen of 0.18 mm.", "Gleiche Töne wie bei 1:50 ohne Ziffern; ein Körperstift von 0,18 mm."],
    overlays: ["Casing, window, disc and plate stay four regions, the disc turns with the rotor.", "Gehäuse, Fenster, Scheibe und Schild bleiben vier Regionen, die Scheibe dreht sich mit dem Rotor."],
  },
);

const rep200 = authored(
  200,
  [150, 350],
  { body: 0.13, detail: null, texture: null, glazing: null, ground: 0.25 },
  [
    rect("body", "body", "body", 3, true, 0.88, -0.085, -0.1425, 0.17, 0.285, { r: 0.012, region: "casing", material: "abs-plastic" }),
    rect("counter", "body", "body", 3.2, true, 0.1, -0.045, 0.07, 0.09, 0.04, { region: "window", material: "polycarbonate" }),
    ellipse("rim", "body", "body", 3.1, true, 0.25, ...DISC, 0.052, 0.052, { region: "window", material: "polycarbonate" }),
    ellipse("disc", "disc", "body", 3.4, true, 0.86, ...DISC, 0.042, 0.042, { region: "disc", material: "aluminium-disc" }),
    rect("mark", "disc", "body", 3.5, true, 0.1, 0.008, -0.01, 0.036, 0.04, { region: "disc", material: "aluminium-disc" }),
    rect("terminals", "body", "body", 3.2, true, 0.55, -0.072, -0.1425, 0.144, 0.0425, { regions: ["casing", "plate"], material: "abs-plastic" }),
  ],
  {
    silhouette: ["A 0.85 mm by 1.4 mm meter: a white rectangle with a dark counter bar, a round window with a disc, and a grey foot.", "Ein Zähler von 0,85 mm mal 1,4 mm: ein weißes Rechteck mit einem dunklen Zählwerksbalken, ein rundes Fenster mit einer Scheibe und ein grauer Fuß."],
    retained: ["Housing, counter bar, disc window with disc and a broad mark, terminal cover.", "Gehäuse, Zählwerksbalken, Scheibenfenster mit Scheibe und einer breiten Markierung, Klemmenabdeckung."],
    merged: ["The rating plate merges into the terminal cover (it carries the plate region).", "Das Leistungsschild geht in der Klemmenabdeckung auf (sie trägt die Schildregion)."],
    omitted: ["Digits, hub, text lines, screws and eye.", "Ziffern, Nabe, Textzeilen, Schrauben und Auge."],
    thinElements: ["The counter bar is 40 mm high (0.2 mm); the mark is a 36 mm wide wedge of the disc so that it survives printing.", "Der Zählwerksbalken ist 40 mm hoch (0,2 mm); die Markierung ist ein 36 mm breiter Keil der Scheibe, damit sie den Druck übersteht."],
    hatching: ["None.", "Keine."],
    styles: ["Housing 0.88, rim 0.25, disc 0.86, mark 0.1, terminal cover 0.55; one body pen of 0.13 mm.", "Gehäuse 0,88, Rand 0,25, Scheibe 0,86, Markierung 0,1, Klemmenabdeckung 0,55; ein Körperstift von 0,13 mm."],
    overlays: ["The terminal cover carries the casing and plate regions; the disc still turns.", "Die Klemmenabdeckung trägt Gehäuse- und Schildregion; die Scheibe dreht sich weiter."],
  },
);

const rep500 = unavailable(
  500,
  ["At 1:500 the 0.17 m meter is 0.34 mm wide and its disc 0.17 mm, below every print profile; use the counter symbol of the clip or a larger scale.", "Bei 1:500 ist der 0,17 m große Zähler 0,34 mm breit und seine Scheibe 0,17 mm, unter jedem Druckprofil; bei diesem Maßstab dient das Zählersymbol des Clips oder ein größerer Maßstab."],
  ["Disc mark of 20 mm", "Scheibenmarkierung von 20 mm"],
  0.02,
  [{ kind: "scale", denominator: 200 }, { kind: "scale", denominator: 100 }],
);

const regions = [
  region("casing", ["Housing and terminal cover", "Gehäuse und Klemmenabdeckung"], "surface", "passive", "abs-plastic", 0.17, 0.285, "vertical-plate", [single("shell", "ABS shell 2 mm", "ABS-Schale 2 mm", "polymer", "abs-plastic", 0.002, 0.012, 2500)], 2500),
  region("window", ["Counter and disc windows", "Zählwerks- und Scheibenfenster"], "glazing", "passive", "polycarbonate", 0.011, 0.1, "vertical-plate", [single("pane", "Polycarbonate pane 2 mm", "Polycarbonatscheibe 2 mm", "polymer", "polycarbonate", 0.002, 0.01, 2400)], 2400),
  region("disc", ["Aluminium disc", "Aluminiumscheibe"], "surface", "passive", "aluminium-disc", 0.0055, 0.084, "vertical-plate", [single("sheet", "Aluminium sheet 1 mm", "Aluminiumblech 1 mm", "metal", "aluminium-disc", 0.001, 0.000005, 2700)], 2700),
  region("plate", ["Rating plate over the measuring system", "Leistungsschild über dem Messwerk"], "surface", "operating", "abs-plastic", 0.0052, 0.13, "vertical-plate", [single("plate", "ABS plate 2 mm over coils and magnet", "ABS-Schild 2 mm über Spulen und Magnet", "polymer", "abs-plastic", 0.002, 0.012, 2500)], 2500, { operating: { key: "load", heatW: 2.4 } }),
];

const bind = (load) => ({
  regions: {
    casing: { exposedFraction: 0.8, sunlitFraction: 0, contact: { fraction: 0.2, tempC: 21 } },
    window: { exposedFraction: 1, sunlitFraction: 0 },
    disc: { exposedFraction: 1, sunlitFraction: 0 },
    plate: { exposedFraction: 1, sunlitFraction: 0 },
  },
  emitters: {},
  operating: { load },
});

const wall = ["wall-screw-top", "wall-screw-bottom"];
const poses = [
  pose("idle", ["No load", "Ohne Last"], ["No consumer draws current: the disc stands still and the coils stay cold.", "Kein Verbraucher zieht Strom: die Scheibe steht still und die Spulen bleiben kalt."], {}, wall, "authored", bind(0)),
  pose("half-load", ["Half load", "Halbe Last"], ["The disc turns at half the speed of the full load; the current coils warm the plate.", "Die Scheibe dreht mit halber Drehzahl der vollen Last; die Stromspulen erwärmen das Schild."], { disc: { angleDeg: 18 } }, wall, "authored", bind(0.5)),
  pose("full-load", ["Full load", "Volle Last"], ["The disc turns at full speed and the plate takes the full 2.4 W of the measuring system.", "Die Scheibe dreht mit voller Drehzahl und das Schild nimmt die vollen 2,4 W des Messwerks auf."], { disc: { angleDeg: 36 } }, wall, "authored", bind(1)),
];

const clips = [
  clip(
    "run",
    ["Load steps up", "Last steigt an"],
    ["The disc rests, starts at half load after one second and runs at full load after four; the shown turning rate is slowed to 90 and 180 degrees per second so that the mark stays readable (a real meter turns about 60 degrees per second at 1 kW).", "Die Scheibe ruht, läuft nach einer Sekunde mit halber Last an und nach vier Sekunden mit voller Last; die gezeigte Drehrate ist auf 90 und 180 Grad je Sekunde verlangsamt, damit die Markierung lesbar bleibt (ein echter Zähler dreht bei 1 kW etwa 60 Grad je Sekunde)."],
    6,
    15,
    [
      { t: 0, pose: "idle", ease: "in" },
      { t: 1, pose: "half-load", ease: "linear", bones: { disc: { angleDeg: 0 } } },
      { t: 4, pose: "half-load", ease: "in", bones: { disc: { angleDeg: 270 } } },
      { t: 6, pose: "full-load", ease: "linear", bones: { disc: { angleDeg: 630 } } },
    ],
    wallContacts(wall, 6),
    [["rest", 0, 1, ["At rest", "In Ruhe"]], ["half", 1, 4, ["Half load", "Halbe Last"]], ["full", 4, 6, ["Full load", "Volle Last"]]],
    [[0, "operating", ["Consumers switched on", "Verbraucher eingeschaltet"], "load"]],
    [["operating:load", "linear", [[0, 0], [1, 0.5], [4, 0.5], [4.5, 1], [6, 1]]]],
  ),
  clip(
    "daily-load",
    ["Evening peak and night", "Abendspitze und Nacht"],
    ["The meter carries a base load, a full evening peak after one minute, and falls back to idle after three; the measuring system warms and cools.", "Der Zähler trägt eine Grundlast, nach einer Minute die volle Abendspitze und fällt nach drei Minuten in den Leerlauf zurück; das Messwerk erwärmt sich und kühlt ab."],
    360,
    1,
    [{ t: 0, pose: "idle", ease: "step" }, { t: 5, pose: "half-load", ease: "step" }, { t: 65, pose: "full-load", ease: "step" }, { t: 185, pose: "idle", ease: "step" }, { t: 360, pose: "idle", ease: "step" }],
    wallContacts(wall, 360),
    [["idle", 0, 5, ["No load", "Ohne Last"]], ["base", 5, 65, ["Base load", "Grundlast"]], ["peak", 65, 185, ["Evening peak", "Abendspitze"]], ["night", 185, 360, ["Night, cooling", "Nacht, kühlt ab"]]],
    [[185, "shutdown", ["Consumers switched off", "Verbraucher ausgeschaltet"], "load"]],
    [["operating:load", "step", [[0, 0], [5, 0.5], [65, 1], [185, 0]]]],
  ),
];

export const electricityMeter = () => {
  const massKg = sum(sp.bones.map((b) => b.mass.kg));
  return wrap({
    id: "electricity-meter",
    category: "appliances",
    title: tx("Electromechanical Electricity Meter with Rotor Disc", "Elektromechanischer Stromzähler mit Läuferscheibe"),
    summary: tx(
      "A 0.17 m by 0.285 m electricity meter of the Ferraris type seen from the room: a white housing with a six-digit counter, a window onto the aluminium rotor disc that turns with the load, a rating plate and a terminal cover.",
      "Ein Stromzähler der Bauart Ferraris von 0,17 m mal 0,285 m, vom Raum aus gesehen: ein weißes Gehäuse mit sechsstelligem Zählwerk, ein Fenster auf die Läuferscheibe aus Aluminium, die sich mit der Last dreht, ein Leistungsschild und eine Klemmenabdeckung.",
    ),
    tags: ["meter", "electricity-meter", "ferraris", "final-energy", "billing", "wall-mounted", "appliance", "electrical", "rotating"],
    provenance: {
      author: "Semio staffage content (G4)",
      method: "handcrafted",
      created: "2026-11-01",
      references: [
        ref("iec-62053-11-2003", "IEC 62053-11:2003 (DIN EN 62053-11) Electricity metering equipment (a.c.) - Particular requirements - Electromechanical meters for active energy (classes 0,5, 1 and 2): housing, display and the limits of the own consumption of the voltage and current circuits."),
        INCROPERA,
      ],
      notes: tx(
        "A single-phase electromechanical (Ferraris) meter of class 2 seen face-on, 0.17 m by 0.285 m and 0.13 m deep with a mass of 0.87 kg (rounded from this product class, authored estimate). From the top: a hanging eye, a six-digit roller counter window (five whole kilowatt hours and the dimmer tenth), a round window onto the 84 mm aluminium disc with its dark mark, the rating plate and the sealed terminal cover. The origin is the centre of the housing front. The disc is the rotor bone; the clip shows the real turning rate of about 60 degrees per second at 1 kW slowed to 90 and 180 degrees per second. The own consumption of a meter of this class is below 2 W in the voltage circuit and about 1 W in the current circuit at full load; the model releases 2.4 W at level 1 into the plate over the measuring system.",
        "Ein einphasiger elektromechanischer Zähler (Ferraris) der Klasse 2, frontal gesehen, 0,17 m mal 0,285 m und 0,13 m tief mit einer Masse von 0,87 kg (aus dieser Produktklasse gerundet, Schätzwert). Von oben: ein Aufhängeauge, ein Rollenzählwerk mit sechs Ziffern (fünf ganze Kilowattstunden und die mattere Zehntelstelle), ein rundes Fenster auf die 84 mm große Aluminiumscheibe mit ihrer dunklen Markierung, das Leistungsschild und die plombierte Klemmenabdeckung. Der Ursprung ist die Mitte der Gehäusefront. Die Scheibe ist der Rotorknochen; der Clip zeigt die echte Drehrate von etwa 60 Grad je Sekunde bei 1 kW verlangsamt auf 90 und 180 Grad je Sekunde. Der Eigenverbrauch eines Zählers dieser Klasse liegt unter 2 W im Spannungspfad und bei etwa 1 W im Strompfad bei voller Last; das Modell setzt auf Stufe 1 2,4 W in das Schild über dem Messwerk frei.",
      ),
    },
    dimensions: { heightM: 0.285, widthM: 0.17, depthM: 0.13, massKg },
    materials: [
      materialOf("abs-plastic", "White ABS plastic", "Weißer ABS-Kunststoff", 0.9, 0.3, 1050, 0.17, 1400, SRC, "incropera-2018"),
      materialOf("polycarbonate", "Clear polycarbonate", "Klares Polycarbonat", 0.9, 0.1, 1200, 0.2, 1200, SRC, "incropera-2018"),
      materialOf("aluminium-disc", "Aluminium sheet", "Aluminiumblech", 0.1, 0.4, 2700, 237, 900, SRC, "incropera-2018"),
    ],
    rig: { kind: "mechanical", root: "body", bones: sp.bones.map((b) => ({ ...b, mass: { ...b.mass } })) },
    anchors: [
      ...supportAnchors("body", { id: "wall-screw-top", x: 0, y: 0.123 }, { id: "wall-screw-bottom", x: 0, y: -0.121 }, [["Upper screw in the hanging eye", "Obere Schraube im Aufhängeauge"], ["Lower screw under the terminal cover", "Untere Schraube unter der Klemmenabdeckung"]]),
    ],
    representations: { 500: rep500, 200: rep200, 100: rep100, 50: rep50 },
    symbols: [],
    poses,
    clips,
    thermal: {
      environment: { airTempC: 21, meanRadiantTempC: 21, relativeHumidity: 0.5, windMS: 0.1, windDirectionDeg: 0, solarWM2: 0, diffuseFraction: 0, pressurePa: 101325 },
      passengers: [],
      operating: [{ id: "load", title: tx("Electrical load", "Elektrische Last"), levels: [level(0, "No load", "Ohne Last"), level(0.5, "Half load (disc at half speed)", "Halbe Last (Scheibe mit halber Drehzahl)"), level(1, "Full load (2.4 W own heat)", "Volle Last (2,4 W Eigenwärme)")] }],
      regions,
      emitters: [],
      assumptions: [
        tx(
          "The own consumption is lumped in the plate over the measuring system and scales with the load level; the real voltage coil also draws about 1.4 W at no load, which the linear model leaves out.",
          "Der Eigenverbrauch ist im Schild über dem Messwerk zusammengefasst und skaliert mit der Laststufe; die echte Spannungsspule nimmt auch ohne Last etwa 1,4 W auf, was das lineare Modell auslässt.",
        ),
      ],
    },
    structure: {
      gravityMS2: 9.80665,
      dynamics: "quasi-static",
      supports: wall,
      assumptions: [
        tx(
          "The 0.87 kg meter hangs on the upper screw in the hanging eye (pin) and leans against the board at the lower screw (roller).",
          "Der 0,87 kg schwere Zähler hängt an der oberen Schraube im Aufhängeauge (Festlager) und lehnt an der unteren Schraube an der Tafel (Rollenlager).",
        ),
      ],
      clinical: false,
    },
    presentation: { defaultPose: "full-load", defaultClip: "run", defaultScale: 50, viewpoints: ["elevation"] },
    exports: ALL_EXPORTS,
    completion: "authored",
  });
};
