import { ALL_EXPORTS, INCROPERA, ISO10456, authored, clip, layer, pose, ref, tx, unavailable, wrap } from "./lib.mjs";
import { materialOf, space, sum } from "./space.mjs";

const SRC = "authored-estimate";
const N = 5;
const PITCH = 0.3;
const NEST = 0.08;
const ids = Array.from({ length: N }, (_, i) => i + 1);
const y0 = (k) => PITCH * (k - 1);
const m = (kg, comX, comY) => ({ kg, comX, comY, distribution: "point", source: SRC });

const sp = space(
  ids.map((k) => [
    `crate-${k}`,
    k === 1 ? null : `crate-${k - 1}`,
    0,
    y0(k),
    {
      title: tx(`Crate ${k}`, `Kiste ${k}`),
      lengthM: 0.3,
      joint: k === 1 ? { type: "fixed" } : { type: "prismatic", axisDeg: 90, minM: -(PITCH - NEST), maxM: 0 },
      mass: m(1.9, 0, 0.15),
    },
  ]),
);
const { rect, poly } = sp;

const tone = (k) => (k % 2 ? 0.45 : 0.55);
const z = (k) => 5 - 0.1 * k;
const reg = (k) => ({ region: `crate-${k}`, material: "hdpe-crate" });

const rep50 = authored(
  50,
  [20, 75],
  { body: 0.18, detail: 0.13, texture: null, glazing: null, ground: 0.35 },
  ids.flatMap((k) => [
    rect(`c${k}-foot`, `crate-${k}`, "body", z(k), true, tone(k) - 0.1, -0.27, y0(k), 0.54, 0.02, reg(k)),
    poly(`c${k}-body`, `crate-${k}`, "body", z(k) + 0.01, true, tone(k), [[-0.28, y0(k) + 0.02], [0.28, y0(k) + 0.02], [0.3, y0(k) + 0.28], [-0.3, y0(k) + 0.28]], true, reg(k)),
    rect(`c${k}-rim`, `crate-${k}`, "body", z(k) + 0.02, true, tone(k) + 0.1, -0.31, y0(k) + 0.28, 0.62, 0.02, reg(k)),
    ...[-0.22, -0.05, 0.12].map((x, i) => rect(`c${k}-slot-${i + 1}`, `crate-${k}`, "detail", z(k) + 0.03, false, 0.3, x, y0(k) + 0.12, 0.1, 0.04)),
  ]),
  {
    silhouette: ["A stack of five crates, 12.4 mm wide and 30 mm high on paper: each a slightly tapered box with a projecting rim and a foot ring, stacked rim to foot, or nested so that only a strip of each crate shows above the one in front.", "Ein Stapel aus fünf Kisten, auf Papier 12,4 mm breit und 30 mm hoch: jede ein leicht konischer Kasten mit vorstehendem Rand und Fußring, Rand auf Fuß gestapelt oder ineinandergestellt, sodass von jeder Kiste nur ein Streifen über der davorstehenden zu sehen ist."],
    retained: ["Crate 0.60 m by 0.30 m (long side, ISO 3394 module 600 by 400), the taper from 0.56 to 0.60 m, the 20 mm rim, the 20 mm foot ring and three vent slots of 0.10 m by 0.04 m; nesting steps of 0.08 m.", "Kiste 0,60 m mal 0,30 m (Längsseite, Modul 600 mal 400 nach ISO 3394), die Konizität von 0,56 auf 0,60 m, der 20 mm hohe Rand, der 20 mm hohe Fußring und drei Lüftungsschlitze von 0,10 m mal 0,04 m; Ineinanderstell-Stufen von 0,08 m."],
    merged: ["The reinforcing ribs and the hand-hole in the short ends are not seen from the long side; the corner posts join the walls.", "Die Verstärkungsrippen und das Griffloch in den Stirnseiten sieht man von der Längsseite nicht; die Eckpfosten gehen in den Wänden auf."],
    omitted: ["Moulded text, the pivot handles of folding types, floor ribs and the grid of the base.", "Geformte Beschriftung, die Klappgriffe faltbarer Typen, Bodenrippen und das Raster des Bodens."],
    thinElements: ["The slots are 0.13 mm detail outlines of 0.8 mm by 2 mm; the rim of 20 mm is 0.4 mm on paper.", "Die Schlitze sind 0,13-mm-Detailumrisse von 0,8 mm mal 2 mm; der 20 mm hohe Rand ist auf Papier 0,4 mm hoch."],
    hatching: ["None.", "Keine."],
    styles: ["Crates alternate between tone 0.45 and 0.55 with a lighter rim and darker foot so neighbouring crates separate; body pen 0.18 mm, detail 0.13 mm, ground 0.35 mm.", "Die Kisten wechseln zwischen Ton 0,45 und 0,55 mit hellerem Rand und dunklerem Fuß, damit sich benachbarte Kisten trennen; Körperstift 0,18 mm, Detail 0,13 mm, Boden 0,35 mm."],
    overlays: ["Each crate is its own region: nested crates show a smaller exposed fraction in the thermal view.", "Jede Kiste ist eine eigene Region: ineinandergestellte Kisten zeigen in der Thermalansicht einen kleineren frei liegenden Anteil."],
  },
);

const rep100 = authored(
  100,
  [75, 150],
  { body: 0.18, detail: null, texture: null, glazing: null, ground: 0.25 },
  ids.flatMap((k) => [
    poly(`c${k}-body`, `crate-${k}`, "body", z(k) + 0.01, true, tone(k), [[-0.28, y0(k)], [0.28, y0(k)], [0.3, y0(k) + 0.275], [-0.3, y0(k) + 0.275]], true, reg(k)),
    rect(`c${k}-rim`, `crate-${k}`, "body", z(k) + 0.02, true, tone(k) + 0.1, -0.31, y0(k) + 0.275, 0.62, 0.025, reg(k)),
  ]),
  {
    silhouette: ["A stack of five crates, 6.2 mm wide and 15 mm high: tapered boxes with a rim strip on top of each.", "Ein Stapel aus fünf Kisten, 6,2 mm breit und 15 mm hoch: konische Kästen mit einem Randstreifen auf jeder."],
    retained: ["Crate 0.60 m by 0.30 m, the taper, the rim of 25 mm and the nesting step of 0.08 m.", "Kiste 0,60 m mal 0,30 m, die Konizität, der Rand von 25 mm und die Ineinanderstell-Stufe von 0,08 m."],
    merged: ["The foot ring joins the body.", "Der Fußring geht im Körper auf."],
    omitted: ["Vent slots and the foot ring as an own shape.", "Lüftungsschlitze und der Fußring als eigene Form."],
    thinElements: ["The rim is thickened from 20 to 25 mm (0.25 mm on paper) so that fill and pen reach the feature width.", "Der Rand ist von 20 auf 25 mm (0,25 mm auf Papier) verdickt, damit Füllung und Strich die Mindestbreite erreichen."],
    hatching: ["None.", "Keine."],
    styles: ["Alternating tones 0.45 and 0.55, lighter rim; one body pen of 0.18 mm.", "Wechselnde Töne 0,45 und 0,55, hellerer Rand; ein Körperstift von 0,18 mm."],
    overlays: ["Each crate keeps its region.", "Jede Kiste behält ihre Region."],
  },
);

const rep200 = authored(
  200,
  [150, 350],
  { body: 0.18, detail: null, texture: null, glazing: null, ground: 0.25 },
  ids.map((k) => poly(`c${k}`, `crate-${k}`, "body", z(k) + 0.01, true, tone(k), [[-0.28, y0(k)], [0.28, y0(k)], [0.31, y0(k) + 0.3], [-0.31, y0(k) + 0.3]], true, reg(k))),
  {
    silhouette: ["A stack of five trapezoids, 3.1 mm wide and 7.5 mm high, each crate one tapered box; nested crates show as short steps.", "Ein Stapel aus fünf Trapezen, 3,1 mm breit und 7,5 mm hoch, jede Kiste ein konischer Kasten; ineinandergestellte Kisten zeigen sich als kurze Stufen."],
    retained: ["Crate 0.60 m by 0.30 m with the taper, the stack pitch of 0.30 m and the nesting step of 0.08 m (0.4 mm).", "Kiste 0,60 m mal 0,30 m mit der Konizität, der Stapelhöhe von 0,30 m und der Ineinanderstell-Stufe von 0,08 m (0,4 mm)."],
    merged: ["Rim, foot ring and body are one trapezoid per crate.", "Rand, Fußring und Körper sind ein Trapez je Kiste."],
    omitted: ["Rim strip, foot ring and vent slots.", "Randstreifen, Fußring und Lüftungsschlitze."],
    thinElements: ["None below the feature width; the nesting step is 0.4 mm and stays separate from the next outline.", "Nichts unterhalb der Mindestbreite; die Ineinanderstell-Stufe ist 0,4 mm hoch und bleibt vom nächsten Umriss getrennt."],
    hatching: ["None.", "Keine."],
    styles: ["Alternating tones 0.45 and 0.55; one body pen of 0.18 mm.", "Wechselnde Töne 0,45 und 0,55; ein Körperstift von 0,18 mm."],
    overlays: ["Each crate keeps its region.", "Jede Kiste behält ihre Region."],
  },
);

const rep500 = unavailable(
  500,
  ["At 1:500 a crate is 1.2 mm by 0.6 mm and a nesting step of 0.08 m is 0.16 mm, below every print profile.", "Bei 1:500 ist eine Kiste 1,2 mm mal 0,6 mm groß und eine Ineinanderstell-Stufe von 0,08 m 0,16 mm, unter jedem Druckprofil."],
  ["Nesting step of 80 mm", "Ineinanderstell-Stufe von 80 mm"],
  0.08,
  [{ kind: "scale", denominator: 200 }, { kind: "scale", denominator: 100 }],
);

const single = (id, en, de, kind, material, thicknessM, res, cap) => layer(id, [en, de], kind, material, thicknessM, res, cap, SRC, "iso-10456-2007");
const regions = ids.map((k) => ({
  id: `crate-${k}`,
  title: tx(`Crate ${k}`, `Kiste ${k}`),
  kind: "surface",
  model: "passive",
  material: "hdpe-crate",
  areaM2: 0.6,
  characteristicLengthM: 0.3,
  geometry: "vertical-plate",
  skyViewFactor: 0.4,
  layers: [single("wall", "HDPE wall 3 mm", "PE-HD-Wand 3 mm", "polymer", "hdpe-crate", 0.003, 0.0075, 6555)],
  heatCapacityJM2K: 6555,
  source: SRC,
  reference: "iso-10456-2007",
}));

const bind = (nested) => ({
  regions: Object.fromEntries(ids.map((k) => [`crate-${k}`, { exposedFraction: nested.includes(k) ? 0.3 : 1, sunlitFraction: 0, ...(k === 1 ? { contact: { fraction: 0.05, tempC: 20 } } : {}) }])),
  emitters: {},
});

const loadsFor = (n) =>
  ids.slice(0, n).map((k) => ({ id: `goods-${k}`, kind: "carried", title: tx(`Goods in crate ${k} (10 kg)`, `Ware in Kiste ${k} (10 kg)`), bone: `crate-${k}`, x: 0, y: 0.1, fxN: 0, fyN: -98.0665, source: SRC }));

const base = ["base-contact"];
const withLoads = (p, n) => ({ ...p, loads: loadsFor(n) });
const down = (from) => Object.fromEntries(ids.filter((k) => k >= from).map((k) => [`crate-${k}`, { dy: -(PITCH - NEST) }]));
const poses = [
  withLoads(pose("high", ["High stack", "Hoher Stapel"], ["All five crates are stacked rim to foot, 1.5 m high, each carrying 10 kg of goods.", "Alle fünf Kisten sind Rand auf Fuß gestapelt, 1,5 m hoch, jede trägt 10 kg Ware."], {}, base, "auto", bind([])), 5),
  withLoads(pose("half", ["Half-high stack", "Halbhoher Stapel"], ["Three crates are stacked with goods and the two top crates are nested empty into the third: 1.06 m high.", "Drei Kisten sind beladen gestapelt und die zwei oberen Kisten sind leer in die dritte gestellt: 1,06 m hoch."], down(4), base, "auto", bind([4, 5])), 3),
  withLoads(pose("low", ["Low stack", "Niedriger Stapel"], ["All five empty crates are nested into each other: 0.62 m high.", "Alle fünf leeren Kisten sind ineinandergestellt: 0,62 m hoch."], down(2), base, "auto", bind([2, 3, 4, 5])), 0),
];
const contacts = (d) => [{ anchor: "base-contact", fromS: 0, toS: d }];
const clips = [
  clip(
    "unload-stack",
    ["Unload the stack", "Stapel abbauen"],
    ["The goods are taken out and the crates are nested: the stack drops from 1.5 m over 1.06 m to 0.62 m in twelve seconds.", "Die Ware wird entnommen und die Kisten werden ineinandergestellt: der Stapel sinkt in zwölf Sekunden von 1,5 m über 1,06 m auf 0,62 m."],
    14,
    15,
    [{ t: 0, pose: "high", ease: "smooth" }, { t: 6, pose: "half", ease: "smooth" }, { t: 12, pose: "low", ease: "smooth" }, { t: 14, pose: "low", ease: "smooth" }],
    contacts(14),
    [["high", 0, 6, ["Unloading the top", "Oben entladen"]], ["nest", 6, 12, ["Nesting the rest", "Rest ineinanderstellen"]], ["rest", 12, 14, ["Low stack", "Niedriger Stapel"]]],
    [[0, "motion", ["Unloading starts", "Entladen beginnt"]], [12, "motion", ["Low stack reached", "Niedriger Stapel erreicht"]]],
    [],
  ),
  clip(
    "load-stack",
    ["Load the stack", "Stapel aufbauen"],
    ["The empty nested crates are pulled apart, loaded and stacked: the stack rises from 0.62 m to 1.5 m in twelve seconds.", "Die leeren ineinandergestellten Kisten werden auseinandergezogen, beladen und gestapelt: der Stapel wächst in zwölf Sekunden von 0,62 m auf 1,5 m."],
    14,
    15,
    [{ t: 0, pose: "low", ease: "smooth" }, { t: 6, pose: "half", ease: "smooth" }, { t: 12, pose: "high", ease: "smooth" }, { t: 14, pose: "high", ease: "smooth" }],
    contacts(14),
    [["pull", 0, 6, ["Pulling apart", "Auseinanderziehen"]], ["stack", 6, 12, ["Stacking", "Stapeln"]], ["rest", 12, 14, ["High stack", "Hoher Stapel"]]],
    [[0, "motion", ["Loading starts", "Beladen beginnt"]], [12, "motion", ["High stack reached", "Hoher Stapel erreicht"]]],
    [],
  ),
];

export const crates = () => {
  const massKg = sum(sp.bones.map((b) => b.mass.kg));
  return wrap({
    id: "storage-crate-stack",
    category: "objects",
    title: tx("Stack of Plastic Storage Crates", "Stapel aus Kunststoff-Lagerkisten"),
    summary: tx(
      "Five stackable and nestable 600 by 400 by 300 mm plastic crates in side elevation: stacked loaded to 1.5 m, or nested empty to 0.62 m, with a half-high pose in between.",
      "Fünf stapel- und ineinanderstellbare Kunststoffkisten von 600 mal 400 mal 300 mm in der Seitenansicht: beladen auf 1,5 m gestapelt oder leer auf 0,62 m ineinandergestellt, mit einer halbhohen Stellung dazwischen.",
    ),
    tags: ["storage-crate", "crate-stack", "plastic-crate", "warehouse", "object", "storage"],
    provenance: {
      author: "Semio staffage content (A8)",
      method: "handcrafted",
      created: "2026-11-01",
      references: [
        ref("iso-3394", "ISO 3394: Packaging - Complete, filled transport packages and unit loads - Dimensions of rigid rectangular packages (modular footprint 600 mm by 400 mm)."),
        INCROPERA,
        ISO10456,
      ],
      notes: tx(
        "Side elevation of five stackable and nestable crates of the 600 mm by 400 mm module (ISO 3394), 0.30 m high, with sidewalls tapering from 0.56 m at the foot to 0.60 m at the rim, a 20 mm rim and a 20 mm foot ring; the crates are 1.9 kg each (HDPE, authored estimate) and 9.5 kg in all. Stacked, each crate sits with its foot on the rim of the one below (pitch 0.30 m); nested, a crate sinks into the one below and only 0.08 m of its height shows above it (authored estimate; real nesting depths range from about 0.05 to 0.2 m with the crate type). The origin is the middle of the lowest crate's base on the floor. The crates have no heat source and exchange heat with the room; a nested crate exposes only 30 percent of its surface. Loaded poses carry 10 kg per crate as carried loads.",
        "Seitenansicht von fünf stapel- und ineinanderstellbaren Kisten des Moduls 600 mm mal 400 mm (ISO 3394), 0,30 m hoch, mit Seitenwänden, die sich vom Fuß mit 0,56 m auf 0,60 m am Rand weiten, einem 20 mm hohen Rand und einem 20 mm hohen Fußring; die Kisten wiegen je 1,9 kg (PE-HD, Schätzwert) und zusammen 9,5 kg. Gestapelt steht jede Kiste mit ihrem Fuß auf dem Rand der darunter (Raster 0,30 m); ineinandergestellt sinkt eine Kiste in die darunter, und nur 0,08 m ihrer Höhe ragen darüber hinaus (Schätzwert; echte Einstelltiefen liegen je nach Kistentyp bei etwa 0,05 bis 0,2 m). Der Ursprung ist die Mitte der Unterseite der untersten Kiste auf dem Boden. Die Kisten haben keine Wärmequelle und tauschen Wärme mit dem Raum aus; eine ineinandergestellte Kiste legt nur 30 Prozent ihrer Fläche frei. Beladene Stellungen tragen 10 kg je Kiste als mitgeführte Lasten.",
      ),
    },
    dimensions: { heightM: 1.5, widthM: 0.62, depthM: 0.4, massKg },
    materials: [materialOf("hdpe-crate", "HDPE crate moulding", "PE-HD-Spritzguss der Kiste", 0.9, 0.5, 950, 0.5, 2300, SRC, "iso-10456-2007")],
    rig: { kind: "mechanical", root: "crate-1", bones: sp.bones.map((b) => ({ ...b, mass: { ...b.mass } })) },
    anchors: [{ id: "base-contact", bone: "crate-1", x: 0, y: 0, kind: "contact", title: tx("Foot ring on the floor", "Fußring auf dem Boden"), support: "contact", normalDeg: 90, footprintLengthM: 0.54, footprintWidthM: 0.34, frictionCoefficient: 0.4 }],
    representations: { 500: rep500, 200: rep200, 100: rep100, 50: rep50 },
    symbols: [],
    poses,
    clips,
    thermal: {
      environment: { airTempC: 20, meanRadiantTempC: 20, relativeHumidity: 0.5, windMS: 0.1, windDirectionDeg: 0, solarWM2: 0, diffuseFraction: 0, pressurePa: 101325 },
      passengers: [],
      operating: [],
      regions,
      emitters: [],
      assumptions: [
        tx(
          "The crates are passive HDPE shells; the contents are not modelled thermally, only as loads.",
          "Die Kisten sind passive PE-HD-Schalen; der Inhalt ist thermisch nicht modelliert, nur als Last.",
        ),
      ],
    },
    structure: {
      gravityMS2: 9.80665,
      dynamics: "quasi-static",
      supports: base,
      assumptions: [
        tx(
          "A rigid column of crates standing on the foot ring of the lowest crate; every crate transfers its weight and its 10 kg of goods through the rim to the one below.",
          "Eine starre Kistensäule auf dem Fußring der untersten Kiste; jede Kiste gibt ihr Gewicht und ihre 10 kg Ware über den Rand an die darunter weiter.",
        ),
      ],
      clinical: false,
    },
    presentation: { defaultPose: "high", defaultClip: "unload-stack", defaultScale: 50, viewpoints: ["elevation"] },
    exports: ALL_EXPORTS,
    completion: "authored",
  });
};
