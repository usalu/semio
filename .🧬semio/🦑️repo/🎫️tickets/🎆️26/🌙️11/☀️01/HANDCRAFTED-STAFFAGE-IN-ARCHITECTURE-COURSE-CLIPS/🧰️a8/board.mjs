import { ALL_EXPORTS, INCROPERA, ISO10456, authored, clip, layer, pose, ref, tx, unavailable, wrap } from "./lib.mjs";
import { materialOf, space, sum } from "./space.mjs";

const SRC = "authored-estimate";
const m = (kg, comX, comY) => ({ kg, comX, comY, distribution: "point", source: SRC });
const fixed = { type: "fixed" };

const sp = space([
  ["frame", null, 0, 0, { title: tx("Board frame with two back panels", "Tafelrahmen mit zwei Rückplatten"), lengthM: 4, joint: fixed, mass: m(45, 0, 0.66) }],
  ["tray", "frame", 0, 0, { title: tx("Chalk tray", "Kreideablage"), lengthM: 4, joint: fixed, mass: m(5, 0, 0.03) }],
  ["leaf", "frame", 0, 0.06, { title: tx("Sliding front leaf", "Schiebeflügel"), lengthM: 2, joint: { type: "prismatic", axisDeg: 0, minM: 0, maxM: 2 }, mass: m(22, -1, 0.6) }],
]);
const { rect } = sp;

const rep50 = authored(
  50,
  [20, 75],
  { body: 0.18, detail: 0.13, texture: null, glazing: null, ground: 0.35 },
  [
    rect("frame", "frame", "body", 3, true, 0.75, -2, 0.06, 4, 1.2, { region: "frame", material: "aluminium-frame" }),
    rect("face-left", "frame", "body", 3.1, true, 0.3, -1.965, 0.095, 1.95, 1.13, { region: "board-face", material: "enamel-board" }),
    rect("face-right", "frame", "body", 3.1, true, 0.3, 0.015, 0.095, 1.95, 1.13, { region: "board-face", material: "enamel-board" }),
    rect("tray", "tray", "body", 3.2, true, 0.65, -2, 0, 4, 0.06, { region: "tray", material: "aluminium-frame" }),
    rect("chalk-a", "tray", "detail", 3.3, true, 0.9, -1.3, 0.02, 0.09, 0.014, { region: "tray", material: "chalk" }),
    rect("chalk-b", "tray", "detail", 3.3, true, 0.85, -1.15, 0.02, 0.07, 0.014, { region: "tray", material: "chalk" }),
    rect("sponge", "tray", "detail", 3.3, true, 0.6, 0.9, 0.012, 0.14, 0.04, { region: "tray", material: "sponge" }),
    rect("leaf-frame", "leaf", "body", 4, true, 0.85, -2, 0.06, 2, 1.2, { region: "leaf", material: "aluminium-frame" }),
    rect("leaf-face", "leaf", "body", 4.1, true, 0.24, -1.965, 0.095, 1.93, 1.13, { region: "leaf", material: "enamel-board" }),
    rect("leaf-grip-a", "leaf", "detail", 4.2, true, 0.4, -1.6, 0.108, 0.12, 0.02, { region: "leaf", material: "aluminium-frame" }),
    rect("leaf-grip-b", "leaf", "detail", 4.2, true, 0.4, -0.4, 0.108, 0.12, 0.02, { region: "leaf", material: "aluminium-frame" }),
  ],
  {
    silhouette: ["A 4.0 m wide, 1.26 m high sign, 80 mm by 25 mm on paper: a dark panel in a light aluminium frame with a slim chalk tray below and a second, sliding panel in front of the left half.", "Ein 4,0 m breites und 1,26 m hohes Zeichen, auf Papier 80 mm mal 25 mm: eine dunkle Tafel in einem hellen Aluminiumrahmen mit einer schmalen Kreideablage darunter und eine zweite, davor verschiebbare Tafel vor der linken Hälfte."],
    retained: ["Overall 4.0 m by 1.2 m board plus the 0.06 m tray, the 35 mm frame, the 30 mm centre mullion, the sliding leaf of 2.0 m by 1.2 m with its own frame and two finger grips, two pieces of chalk and a sponge on the tray.", "Gesamt 4,0 m mal 1,2 m Tafel plus die 0,06 m hohe Ablage, der 35 mm breite Rahmen, der 30 mm breite Mittelsteg, der Schiebeflügel von 2,0 m mal 1,2 m mit eigenem Rahmen und zwei Griffmulden, zwei Kreidestücke und ein Schwamm auf der Ablage."],
    merged: ["The two back panels share one frame shape; the tray lip and the tray are one strip.", "Die beiden Rückplatten teilen sich eine Rahmenform; Ablagelippe und Ablage sind ein Streifen."],
    omitted: ["Wall brackets, the hanging rail, the cord guides, rulers and compasses, every writing on the surface.", "Wandhalter, Hängeschiene, Seilführungen, Lineale und Zirkel, jede Beschriftung auf der Fläche."],
    thinElements: ["The chalk pieces are 14 mm thick (0.28 mm on paper) and the finger grips 20 mm.", "Die Kreidestücke sind 14 mm dick (0,28 mm auf Papier) und die Griffmulden 20 mm."],
    hatching: ["None; the dark panel is a tone fill.", "Keine; die dunkle Tafel ist eine Tonfüllung."],
    styles: ["Panels 0.1 to 0.12, frames 0.5 to 0.55, tray 0.45, chalk 0.85 to 0.9, sponge 0.6; body pen 0.18 mm, detail 0.13 mm, ground 0.35 mm.", "Tafeln 0,1 bis 0,12, Rahmen 0,5 bis 0,55, Ablage 0,45, Kreide 0,85 bis 0,9, Schwamm 0,6; Körperstift 0,18 mm, Detail 0,13 mm, Boden 0,35 mm."],
    overlays: ["Back panels, frame, tray and sliding leaf are four regions: the thermal view shows how the leaf shades half of the back panels.", "Rückplatten, Rahmen, Ablage und Schiebeflügel sind vier Regionen: die Thermalansicht zeigt, wie der Flügel die Hälfte der Rückplatten abdeckt."],
  },
);

const rep100 = authored(
  100,
  [75, 150],
  { body: 0.18, detail: null, texture: null, glazing: null, ground: 0.25 },
  [
    rect("frame", "frame", "body", 3, true, 0.75, -2, 0.06, 4, 1.2, { region: "frame", material: "aluminium-frame" }),
    rect("face-left", "frame", "body", 3.1, true, 0.3, -1.96, 0.1, 1.94, 1.12, { region: "board-face", material: "enamel-board" }),
    rect("face-right", "frame", "body", 3.1, true, 0.3, 0.02, 0.1, 1.94, 1.12, { region: "board-face", material: "enamel-board" }),
    rect("tray", "tray", "body", 3.2, true, 0.65, -2, 0, 4, 0.06, { region: "tray", material: "aluminium-frame" }),
    rect("leaf-frame", "leaf", "body", 4, true, 0.85, -2, 0.06, 2, 1.2, { region: "leaf", material: "aluminium-frame" }),
    rect("leaf-face", "leaf", "body", 4.1, true, 0.24, -1.96, 0.1, 1.92, 1.12, { region: "leaf", material: "enamel-board" }),
  ],
  {
    silhouette: ["A 40 mm by 12.6 mm sign: a framed dark panel with a tray strip and a front leaf over its left half.", "Ein Zeichen von 40 mm mal 12,6 mm: eine gerahmte dunkle Tafel mit Ablagestreifen und ein Vorderflügel über der linken Hälfte."],
    retained: ["Board 4.0 m by 1.2 m, the tray 0.06 m, a 40 mm frame, the centre mullion and the leaf with its frame.", "Tafel 4,0 m mal 1,2 m, die Ablage von 0,06 m, ein 40 mm breiter Rahmen, der Mittelsteg und der Flügel mit seinem Rahmen."],
    merged: ["The leaf grips, the chalk and the sponge join the tray and the leaf frame.", "Griffmulden, Kreide und Schwamm gehen in Ablage und Flügelrahmen auf."],
    omitted: ["Chalk pieces, sponge, finger grips, the tray lip.", "Kreidestücke, Schwamm, Griffmulden, die Ablagelippe."],
    thinElements: ["The frame is widened from 35 to 40 mm (0.4 mm on paper) so that fill and 0.18 mm pen reach the feature width.", "Der Rahmen ist von 35 auf 40 mm (0,4 mm auf Papier) verbreitert, damit Füllung und 0,18-mm-Strich die Mindestbreite erreichen."],
    hatching: ["None.", "Keine."],
    styles: ["Panels 0.1 to 0.12, frames 0.5 to 0.55, tray 0.45; one body pen of 0.18 mm.", "Tafeln 0,1 bis 0,12, Rahmen 0,5 bis 0,55, Ablage 0,45; ein Körperstift von 0,18 mm."],
    overlays: ["Every part keeps its region, so the leaf and the back panels can differ in the thermal view.", "Jedes Teil behält seine Region, sodass sich Flügel und Rückplatten in der Thermalansicht unterscheiden können."],
  },
);

const rep200 = authored(
  200,
  [150, 350],
  { body: 0.18, detail: null, texture: null, glazing: null, ground: 0.25 },
  [
    rect("frame", "frame", "body", 3, true, 0.5, -2, 0.07, 4, 1.19, { regions: ["frame", "board-face"], material: "enamel-board" }),
    rect("tray", "tray", "body", 3.2, true, 0.65, -2, 0, 4, 0.07, { region: "tray", material: "aluminium-frame" }),
    rect("leaf", "leaf", "body", 4, true, 0.35, -2, 0.07, 2, 1.19, { region: "leaf", material: "enamel-board" }),
  ],
  {
    silhouette: ["A 20 mm by 6.3 mm sign: a dark rectangle on a thin tray strip with the sliding leaf as a second rectangle in front of the left half.", "Ein Zeichen von 20 mm mal 6,3 mm: ein dunkles Rechteck auf einem dünnen Ablagestreifen, davor der Schiebeflügel als zweites Rechteck vor der linken Hälfte."],
    retained: ["Board 4.0 m by 1.2 m, the tray strip and the sliding leaf of 2.0 m by 1.2 m.", "Tafel 4,0 m mal 1,2 m, der Ablagestreifen und der Schiebeflügel von 2,0 m mal 1,2 m."],
    merged: ["Frame and back panels are one rectangle (regions frame and board-face merged); the leaf frame and leaf face are one rectangle.", "Rahmen und Rückplatten sind ein Rechteck (Regionen Rahmen und Tafelfläche zusammengeführt); Flügelrahmen und Flügelfläche sind ein Rechteck."],
    omitted: ["The mullion, grips, chalk, sponge and tray lip.", "Der Mittelsteg, Griffmulden, Kreide, Schwamm und die Ablagelippe."],
    thinElements: ["The tray is thickened from 60 to 70 mm (0.35 mm on paper); the 35 mm frame is only 0.18 mm on paper and shows as the outline pen.", "Die Ablage ist von 60 auf 70 mm (0,35 mm auf Papier) verdickt; der 35 mm breite Rahmen ist auf Papier nur 0,18 mm breit und zeigt sich als Umrissstift."],
    hatching: ["None.", "Keine."],
    styles: ["Back board 0.3, leaf 0.15, tray 0.45; one body pen of 0.18 mm.", "Rückplatte 0,3, Flügel 0,15, Ablage 0,45; ein Körperstift von 0,18 mm."],
    overlays: ["The rectangle lists frame and board face as merged regions; the leaf and the tray stay separate.", "Das Rechteck führt Rahmen und Tafelfläche als zusammengeführte Regionen; Flügel und Ablage bleiben getrennt."],
  },
);

const rep500 = unavailable(
  500,
  ["At 1:500 the 1.2 m board is 2.4 mm high, its frame 0.07 mm and the tray 0.12 mm wide, below every print profile.", "Bei 1:500 ist die 1,2 m hohe Tafel 2,4 mm hoch, ihr Rahmen 0,07 mm und die Ablage 0,12 mm breit, unter jedem Druckprofil."],
  ["Frame width of 35 mm", "Rahmenbreite von 35 mm"],
  0.035,
  [{ kind: "scale", denominator: 200 }, { kind: "scale", denominator: 100 }],
);

const single = (id, en, de, kind, material, thicknessM, res, cap, reference = "iso-10456-2007") => layer(id, [en, de], kind, material, thicknessM, res, cap, SRC, reference);
const region = (id, title, kind, material, areaM2, characteristicLengthM, geometry, layers, heatCapacityJM2K) => ({
  id,
  title: tx(...title),
  kind,
  model: "passive",
  material,
  areaM2,
  characteristicLengthM,
  geometry,
  skyViewFactor: 0.5,
  layers,
  heatCapacityJM2K,
  source: SRC,
  reference: "iso-10456-2007",
});

const boardLayers = () => [single("enamel", "Ceramic-steel writing surface 0.6 mm", "Schreibfläche aus Keramikstahl 0,6 mm", "metal", "enamel-board", 0.0006, 0.000012, 2260), single("core", "MDF core 16 mm", "MDF-Kern 16 mm", "timber", "mdf-core", 0.016, 0.114, 20400)];

const regions = [
  region("board-face", ["Back panels with writing surface", "Rückplatten mit Schreibfläche"], "wall", "enamel-board", 4.5, 1.2, "vertical-plate", boardLayers(), 22660),
  region("frame", ["Aluminium frame and mullion", "Aluminiumrahmen und Mittelsteg"], "surface", "aluminium-frame", 0.8, 0.04, "cylinder", [single("profile", "Anodised aluminium profile 2 mm", "Eloxiertes Aluminiumprofil 2 mm", "metal", "aluminium-frame", 0.002, 0.000012, 5400)], 5400),
  region("leaf", ["Sliding front leaf", "Schiebeflügel"], "wall", "enamel-board", 2.4, 1.2, "vertical-plate", boardLayers(), 22660),
  region("tray", ["Chalk tray with chalk and sponge", "Kreideablage mit Kreide und Schwamm"], "surface", "aluminium-frame", 0.32, 0.08, "horizontal-plate-up", [single("tray-sheet", "Aluminium sheet 1.5 mm", "Aluminiumblech 1,5 mm", "metal", "aluminium-frame", 0.0015, 0.00001, 4050)], 4050),
];

const bind = (sun = 0) => ({
  regions: {
    "board-face": { exposedFraction: 0.5, sunlitFraction: sun },
    frame: { exposedFraction: 1, sunlitFraction: sun },
    leaf: { exposedFraction: 1, sunlitFraction: sun },
    tray: { exposedFraction: 1, sunlitFraction: 0 },
  },
  emitters: {},
});

const wall = ["wall-hook", "wall-rest"];
const poses = [
  pose("leaf-left", ["Leaf on the left", "Flügel links"], ["The sliding leaf covers the left back panel: two writing surfaces are free, the leaf on the left and the back panel on the right.", "Der Schiebeflügel deckt die linke Rückplatte ab: zwei Schreibflächen sind frei, der Flügel links und die Rückplatte rechts."], {}, wall, "authored", bind()),
  pose("leaf-middle", ["Leaf in the middle", "Flügel in der Mitte"], ["The leaf is pushed to the middle and covers half of each back panel.", "Der Flügel ist in die Mitte geschoben und deckt die Hälfte jeder Rückplatte ab."], { leaf: { dx: 1 } }, wall, "authored", bind()),
  pose("leaf-right", ["Leaf on the right", "Flügel rechts"], ["The leaf covers the right back panel and the left back panel is free.", "Der Flügel deckt die rechte Rückplatte ab und die linke Rückplatte ist frei."], { leaf: { dx: 2 } }, wall, "authored", bind()),
];

const contacts = (d) => wall.map((anchor) => ({ anchor, fromS: 0, toS: d }));

const clips = [
  clip(
    "slide-leaf",
    ["Slide the leaf", "Flügel verschieben"],
    ["The leaf slides from the left over the middle to the right in six seconds, rests for three seconds and returns.", "Der Flügel gleitet in sechs Sekunden von links über die Mitte nach rechts, ruht drei Sekunden und kehrt zurück."],
    18,
    15,
    [{ t: 0, pose: "leaf-left", ease: "smooth" }, { t: 6, pose: "leaf-right", ease: "smooth" }, { t: 9, pose: "leaf-right", ease: "smooth" }, { t: 15, pose: "leaf-left", ease: "smooth" }, { t: 18, pose: "leaf-left", ease: "smooth" }],
    contacts(18),
    [["slide-out", 0, 6, ["Sliding to the right", "Gleiten nach rechts"]], ["rest", 6, 9, ["At rest on the right", "Ruhe rechts"]], ["slide-back", 9, 15, ["Sliding back", "Zurückgleiten"]], ["rest-left", 15, 18, ["At rest on the left", "Ruhe links"]]],
    [[0, "motion", ["Leaf starts to slide", "Flügel beginnt zu gleiten"]], [6, "motion", ["Right end reached", "Rechtes Ende erreicht"]]],
    [],
  ),
  clip(
    "sun-on-board",
    ["Sun patch crosses the board", "Sonnenfleck auf der Tafel"],
    ["Over ten minutes the sun reaches 60 percent of the board: the writing surfaces warm up, the frame follows.", "Binnen zehn Minuten erreicht die Sonne 60 Prozent der Tafel: die Schreibflächen erwärmen sich, der Rahmen folgt."],
    600,
    1,
    [{ t: 0, pose: "leaf-left", ease: "step" }, { t: 600, pose: "leaf-left", ease: "step" }],
    contacts(600),
    [["shade", 0, 60, ["In the shade", "Im Schatten"]], ["sun", 60, 600, ["Sun on the board", "Sonne auf der Tafel"]]],
    [[60, "thermal", ["Sun reaches the board", "Sonne erreicht die Tafel"]]],
    [
      ["region:board-face:sunlitFraction", "linear", [[0, 0], [60, 0], [600, 0.6]]],
      ["region:leaf:sunlitFraction", "linear", [[0, 0], [60, 0], [600, 0.6]]],
    ],
  ),
];

export const board = () => {
  const massKg = sum(sp.bones.map((b) => b.mass.kg));
  return wrap({
    id: "chalkboard",
    category: "objects",
    title: tx("Wall-mounted Classroom Chalkboard", "Wandtafel für den Klassenraum"),
    summary: tx(
      "A wall-mounted classroom chalkboard 4.0 m wide and 1.2 m high in an aluminium frame with a chalk tray and a 2.0 m sliding front leaf, shown in front elevation; a passive surface that warms in the sun.",
      "Eine an der Wand montierte Klassenraumtafel von 4,0 m Breite und 1,2 m Höhe in einem Aluminiumrahmen mit Kreideablage und einem 2,0 m breiten Schiebeflügel, in Vorderansicht; eine passive Fläche, die sich in der Sonne erwärmt.",
    ),
    tags: ["chalkboard", "classroom", "teaching-board", "school", "wall-mounted", "object"],
    provenance: {
      author: "Semio staffage content (A8)",
      method: "handcrafted",
      created: "2026-11-01",
      references: [
        ref("en-14434-2010", "EN 14434:2010 Writing boards for educational institutions - Functional requirements, including safety requirements and test methods (board sizes, writing surface, tray, fixing)."),
        INCROPERA,
        ISO10456,
      ],
      notes: tx(
        "Front elevation of a classroom board of the common 4.0 m by 1.2 m class: two fixed back panels of 2.0 m by 1.2 m in a 35 mm anodised aluminium frame with a 30 mm centre mullion, a 0.06 m chalk tray along the bottom edge and a front leaf of 2.0 m by 1.2 m that slides on rails in front of the panels. The origin is the middle of the lower edge of the tray; a call site places it about 0.85 m above the floor so that the writing surface runs from 0.95 m to 2.15 m. Panels are ceramic steel of 0.6 mm on a 16 mm MDF core (about 9 kg per square metre); dimensions and masses (72 kg) are rounded from this product class (authored estimate). The board has no heat source: it exchanges heat with the room and takes the sun that falls on it.",
        "Vorderansicht einer Klassenraumtafel der üblichen Größe von 4,0 m mal 1,2 m: zwei feste Rückplatten von 2,0 m mal 1,2 m in einem 35 mm breiten eloxierten Aluminiumrahmen mit einem 30 mm breiten Mittelsteg, eine 0,06 m hohe Kreideablage am unteren Rand und ein Vorderflügel von 2,0 m mal 1,2 m, der auf Schienen vor den Platten gleitet. Der Ursprung ist die Mitte der Unterkante der Ablage; ein Aufrufer setzt ihn etwa 0,85 m über den Boden, sodass die Schreibfläche von 0,95 m bis 2,15 m reicht. Die Platten sind 0,6 mm Keramikstahl auf einem 16 mm starken MDF-Kern (etwa 9 kg je Quadratmeter); Maße und Massen (72 kg) sind aus dieser Produktklasse gerundet (Schätzwert). Die Tafel hat keine Wärmequelle: sie tauscht Wärme mit dem Raum aus und nimmt die Sonne auf, die auf sie fällt.",
      ),
    },
    dimensions: { heightM: 1.26, widthM: 4.0, depthM: 0.08, massKg },
    materials: [
      materialOf("enamel-board", "Dark ceramic-steel writing surface", "Dunkle Schreibfläche aus Keramikstahl", 0.9, 0.85, 7850, 50, 450, SRC, "incropera-2018"),
      materialOf("mdf-core", "MDF core", "MDF-Kern", 0.9, 0.6, 750, 0.14, 1700, "literature", "iso-10456-2007"),
      materialOf("aluminium-frame", "Anodised aluminium", "Eloxiertes Aluminium", 0.2, 0.4, 2700, 160, 900, "literature", "incropera-2018"),
      materialOf("chalk", "Chalk", "Kreide", 0.9, 0.2, 2300, 0.9, 900, SRC, "iso-10456-2007"),
      materialOf("sponge", "Cellulose sponge", "Zellulose-Schwamm", 0.9, 0.5, 60, 0.05, 1500, SRC, "iso-10456-2007"),
    ],
    rig: { kind: "mechanical", root: "frame", bones: sp.bones.map((b) => ({ ...b, mass: { ...b.mass } })) },
    anchors: [
      { id: "wall-hook", bone: "frame", x: 0, y: 1.2, kind: "support", title: tx("Upper hanging rail on the wall", "Obere Hängeschiene an der Wand"), support: "pin", normalDeg: 0, footprintLengthM: 0.02, footprintWidthM: 0.3 },
      { id: "wall-rest", bone: "frame", x: 0, y: 0.1, kind: "support", title: tx("Lower distance buffers against the wall", "Untere Abstandspuffer an der Wand"), support: "roller", normalDeg: 0, footprintLengthM: 0.02, footprintWidthM: 0.3, frictionCoefficient: 0.4 },
    ],
    representations: { 500: rep500, 200: rep200, 100: rep100, 50: rep50 },
    symbols: [],
    poses,
    clips,
    thermal: {
      environment: { airTempC: 21, meanRadiantTempC: 21, relativeHumidity: 0.5, windMS: 0.1, windDirectionDeg: 0, solarWM2: 300, diffuseFraction: 0.5, pressurePa: 101325 },
      passengers: [],
      operating: [],
      regions,
      emitters: [],
      assumptions: [
        tx(
          "The board is a passive wall panel: the ceramic-steel face and the MDF core carry the sun load, the frame and the tray follow as thin aluminium parts.",
          "Die Tafel ist eine passive Wandplatte: Keramikstahlfläche und MDF-Kern tragen die Sonnenlast, Rahmen und Ablage folgen als dünne Aluminiumteile.",
        ),
      ],
    },
    structure: {
      gravityMS2: 9.80665,
      dynamics: "quasi-static",
      supports: wall,
      assumptions: [
        tx(
          "The 72 kg board hangs on the upper rail (pin) and leans on buffers at the lower edge (roller): the rail carries the weight, the pair resists the overturning moment of the sliding leaf.",
          "Die 72 kg schwere Tafel hängt an der oberen Schiene (Festlager) und lehnt unten an Puffern (Rollenlager): die Schiene trägt das Gewicht, das Paar nimmt das Kippmoment des Schiebeflügels auf.",
        ),
      ],
      clinical: false,
    },
    presentation: { defaultPose: "leaf-left", defaultClip: "slide-leaf", defaultScale: 100, viewpoints: ["elevation"] },
    exports: ALL_EXPORTS,
    completion: "authored",
  });
};
