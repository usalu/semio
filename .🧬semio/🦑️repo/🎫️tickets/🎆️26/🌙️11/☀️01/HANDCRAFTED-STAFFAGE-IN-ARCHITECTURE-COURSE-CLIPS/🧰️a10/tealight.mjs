import { ALL_EXPORTS, INCROPERA, authored, clip, layer, level, pose, ref, tx, unavailable, wrap } from "./lib.mjs";
import { materialOf, space, sum } from "./space.mjs";

const SRC = "authored-estimate";
const m = (kg, comX, comY) => ({ kg, comX, comY, distribution: "point", source: SRC });
const fixed = { type: "fixed" };

const sp = space([
  ["cup", null, 0, 0, { title: tx("Aluminium cup", "Aluminiumschälchen"), lengthM: 0.016, joint: fixed, mass: m(0.001, 0, 0.007) }],
  ["wax", "cup", 0, 0.0014, { title: tx("Wax fill", "Wachsfüllung"), lengthM: 0.013, joint: fixed, mass: m(0.0128, 0, 0.0065) }],
  ["wick", "wax", 0, 0.0145, { title: tx("Wick with sustainer tab", "Docht mit Dochtfuß"), lengthM: 0.005, joint: fixed, mass: m(0.0002, 0, 0.0025) }],
  ["flame", "wick", 0, 0.0185, { title: tx("Flame lift (hides the flame in the cup)", "Flammenhub (verbirgt die Flamme im Schälchen)"), lengthM: 0.0125, joint: { type: "prismatic", axisDeg: 90, minM: -0.0165, maxM: 0.003 } }],
  ["flame-body", "flame", 0, 0.0185, { title: tx("Flame body, leans in a draught", "Flammenkörper, neigt sich im Luftzug"), lengthM: 0.0125, joint: { type: "revolute", minDeg: -25, maxDeg: 25 } }],
  ["flame-tip", "flame-body", 0, 0.0265, { title: tx("Flame tongue, stretches at the flicker peak", "Flammenzunge, streckt sich im Flackermaximum"), lengthM: 0.014, joint: { type: "prismatic", axisDeg: 90, minM: -0.011, maxM: 0.006 } }],
]);
const { poly, path, ellipse } = sp;

const CUP = [[-0.0155, 0], [0.0155, 0], [0.019, 0.016], [-0.019, 0.016]];
const BODY = "M 0 0.0185 C 0.0058 0.0215 0.0058 0.0275 0.0016 0.031 C 0.0008 0.0318 0.0003 0.0322 0 0.0325 C -0.0003 0.0322 -0.0008 0.0318 -0.0016 0.031 C -0.0058 0.0275 -0.0058 0.0215 0 0.0185 Z";
const TONGUE = "M 0 0.0265 C 0.0016 0.0285 0.0019 0.0335 0.0007 0.0385 C 0.0004 0.0398 0.0002 0.0403 0 0.0405 C -0.0002 0.0403 -0.0004 0.0398 -0.0007 0.0385 C -0.0019 0.0335 -0.0016 0.0285 0 0.0265 Z";

const rep50 = authored(
  50,
  [20, 75],
  { body: 0.18, detail: 0.13, texture: null, glazing: null, ground: 0.35 },
  [
    path("tongue", "flame-tip", "body", 3.2, true, 0.97, TONGUE, { region: "flame", material: "flame" }),
    path("flame", "flame-body", "body", 3.3, true, 0.9, BODY, { region: "flame", material: "flame" }),
    poly("wick", "wick", "detail", 3.4, false, 0.1, [[0, 0.0145], [0, 0.0198]], false, { region: "wick", material: "wick-cotton" }),
    poly("cup", "cup", "body", 3.5, true, 0.5, CUP, true, { region: "cup", material: "aluminium-cup" }),
    poly("wax-surface", "wax", "detail", 3.6, false, 0.85, [[-0.0172, 0.0152], [0, 0.0158], [0.0172, 0.0152]], false, { region: "wax", material: "paraffin" }),
  ],
  {
    silhouette: ["A 38 mm by 40 mm candle, 0.8 mm by 0.8 mm on paper: a flat aluminium cup with a flame standing on it; the flame is drawn 11 mm wide so that it prints.", "Eine Kerze von 38 mm mal 40 mm, auf Papier 0,8 mm mal 0,8 mm: ein flaches Aluminiumschälchen mit einer Flamme darauf; die Flamme ist 11 mm breit gezeichnet, damit sie druckt."],
    retained: ["The 16 mm cup tapering from 31 to 38 mm, the wick standing 4 mm above the cup, the flame with its tongue, and the wax surface as a curved line at the cup rim.", "Das 16 mm hohe Schälchen, das sich von 31 auf 38 mm weitet, der 4 mm über das Schälchen ragende Docht, die Flamme mit ihrer Zunge und die Wachsoberfläche als gebogene Linie am Schälchenrand."],
    merged: ["The wax lies inside the cup: its surface shape carries the wax region, the cup wall carries the aluminium.", "Das Wachs liegt im Schälchen: seine Oberflächenform trägt die Wachsregion, die Schälchenwand das Aluminium."],
    omitted: ["Cup ribs, the sustainer tab, the blue flame base and the melt pool.", "Rippen des Schälchens, der Dochtfuß, der blaue Flammenfuß und das Schmelzbad."],
    thinElements: ["The flame is thickened from 8 to 11 mm (0.22 mm); the wick is a 0.13 mm line; the flame tongue is a sharp tip.", "Die Flamme ist von 8 auf 11 mm (0,22 mm) verdickt; der Docht ist eine 0,13-mm-Linie; die Flammenzunge läuft spitz zu."],
    hatching: ["None.", "Keine."],
    styles: ["Cup 0.5, flame 0.9 and tongue 0.97, wax line 0.85, wick 0.1; body pen 0.18 mm, detail 0.13 mm, ground 0.35 mm.", "Schälchen 0,5, Flamme 0,9 und Zunge 0,97, Wachslinie 0,85, Docht 0,1; Körperstift 0,18 mm, Detail 0,13 mm, Boden 0,35 mm."],
    overlays: ["Cup, wax line, wick and flame are four regions; in the thermal view only the flame is hot and the plume rises from its tip.", "Schälchen, Wachslinie, Docht und Flamme sind vier Regionen; in der Thermalansicht ist nur die Flamme heiß, und die Fahne steigt von ihrer Spitze auf."],
  },
);

const rep100 = authored(
  100,
  [75, 150],
  { body: 0.18, detail: null, texture: null, glazing: null, ground: 0.25 },
  [
    path("tongue", "flame-tip", "body", 3.2, true, 0.97, TONGUE, { region: "flame", material: "flame" }),
    path("flame", "flame-body", "body", 3.3, true, 0.9, "M 0 0.0185 C 0.0072 0.0212 0.0072 0.0275 0.002 0.031 C 0.001 0.0318 0.0004 0.0322 0 0.0325 C -0.0004 0.0322 -0.001 0.0318 -0.002 0.031 C -0.0072 0.0275 -0.0072 0.0212 0 0.0185 Z", { region: "flame", material: "flame" }),
    poly("cup", "cup", "body", 3.5, true, 0.5, [[-0.0155, 0], [0.0155, 0], [0.0195, 0.016], [-0.0195, 0.016]], true, { regions: ["cup", "wax", "wick"], material: "aluminium-cup" }),
  ],
  {
    silhouette: ["A 0.4 mm by 0.4 mm candle: a cup wedge with a teardrop flame on top.", "Eine Kerze von 0,4 mm mal 0,4 mm: ein Schälchenkeil mit einer tropfenförmigen Flamme darauf."],
    retained: ["The cup wedge of 16 mm height and 39 mm rim, the flame with its tongue and its lean.", "Der Schälchenkeil von 16 mm Höhe und 39 mm Rand, die Flamme mit ihrer Zunge und ihrer Neigung."],
    merged: ["Wax and wick join the cup (regions cup, wax and wick merged).", "Wachs und Docht gehen im Schälchen auf (Regionen Schälchen, Wachs und Docht zusammengeführt)."],
    omitted: ["Wax line, wick line, cup rim and ribs.", "Wachslinie, Dochtlinie, Schälchenrand und Rippen."],
    thinElements: ["The flame is thickened from 8 to 14 mm (0.14 mm wide with its pen 0.32 mm); the tongue stays a sharp tip.", "Die Flamme ist von 8 auf 14 mm verdickt (0,14 mm breit, mit Stift 0,32 mm); die Zunge bleibt eine scharfe Spitze."],
    hatching: ["None.", "Keine."],
    styles: ["Cup 0.5, flame 0.9, tongue 0.97; one body pen of 0.18 mm.", "Schälchen 0,5, Flamme 0,9, Zunge 0,97; ein Körperstift von 0,18 mm."],
    overlays: ["The cup wedge lists cup, wax and wick as merged regions; the flame stays its own region.", "Der Schälchenkeil führt Schälchen, Wachs und Docht als zusammengeführte Regionen; die Flamme bleibt eine eigene Region."],
  },
);

const rep200 = authored(
  200,
  [150, 350],
  { body: 0.13, detail: null, texture: null, glazing: null, ground: 0.25 },
  [
    path("flame", "flame-body", "body", 3.3, true, 0.9, "M 0 0.0185 C 0.0105 0.021 0.0105 0.0285 0.0034 0.034 C 0.0017 0.0368 0.0006 0.039 0 0.0405 C -0.0006 0.039 -0.0017 0.0368 -0.0034 0.034 C -0.0105 0.0285 -0.0105 0.021 0 0.0185 Z", { region: "flame", material: "flame" }),
    poly("cup", "cup", "body", 3.5, true, 0.5, [[-0.0165, 0], [0.0165, 0], [0.0215, 0.016], [-0.0215, 0.016]], true, { regions: ["cup", "wax", "wick"], material: "aluminium-cup" }),
  ],
  {
    silhouette: ["A 0.2 mm dot with a flame: a cup wedge and a drop.", "Ein Punkt von 0,2 mm mit einer Flamme: ein Schälchenkeil und ein Tropfen."],
    retained: ["Cup wedge and flame as the two masses.", "Schälchenkeil und Flamme als die zwei Massen."],
    merged: ["Wax and wick join the cup; the tongue is part of the flame drop, which is drawn at its full 22 mm height (the stretch of the tongue is not shown at this scale).", "Wachs und Docht gehen im Schälchen auf; die Zunge ist Teil des Flammentropfens, der in voller Höhe von 22 mm gezeichnet ist (das Strecken der Zunge zeigt dieser Maßstab nicht)."],
    omitted: ["Every detail line and the separate flame tongue.", "Jede Detaillinie und die eigene Flammenzunge."],
    thinElements: ["The cup is thickened from 31 to 33 mm at the foot and from 38 to 43 mm at the rim, the flame from 8 to 21 mm, to reach the feature width of 0.2 mm on paper.", "Das Schälchen ist am Fuß von 31 auf 33 mm und am Rand von 38 auf 43 mm verdickt, die Flamme von 8 auf 21 mm, um die Mindestbreite von 0,2 mm auf Papier zu erreichen."],
    hatching: ["None.", "Keine."],
    styles: ["Cup 0.5, flame 0.9; one body pen of 0.13 mm.", "Schälchen 0,5, Flamme 0,9; ein Körperstift von 0,13 mm."],
    overlays: ["Cup wedge and flame carry the same regions as at 1:100.", "Schälchenkeil und Flamme tragen dieselben Regionen wie bei 1:100."],
  },
);

const rep500 = unavailable(
  500,
  ["At 1:500 the 38 mm tealight is 0.08 mm wide and its flame 0.02 mm, far below every print profile.", "Bei 1:500 ist das 38 mm große Teelicht 0,08 mm breit und seine Flamme 0,02 mm, weit unter jedem Druckprofil."],
  ["Flame of 8 mm width", "Flamme von 8 mm Breite"],
  0.008,
  [{ kind: "scale", denominator: 200 }, { kind: "scale", denominator: 100 }],
);

const single = (id, en, de, kind, material, thicknessM, res, cap, reference = "incropera-2018") => layer(id, [en, de], kind, material, thicknessM, res, cap, SRC, reference);
const region = (id, title, kind, model, material, areaM2, characteristicLengthM, geometry, layers, heatCapacityJM2K, extra = {}, reference = "incropera-2018") => ({
  id,
  title: tx(...title),
  kind,
  model,
  material,
  areaM2,
  characteristicLengthM,
  geometry,
  skyViewFactor: 0.5,
  layers,
  heatCapacityJM2K,
  ...extra,
  source: SRC,
  reference,
});

const regions = [
  region("cup", ["Aluminium cup", "Aluminiumschälchen"], "surface", "passive", "aluminium-cup", 0.003, 0.016, "cylinder", [single("foil", "Aluminium foil 0.1 mm", "Aluminiumfolie 0,1 mm", "metal", "aluminium-cup", 0.0001, 0.0000004, 243)], 243),
  region("wax", ["Wax fill", "Wachsfüllung"], "surface", "passive", "paraffin", 0.001, 0.036, "horizontal-plate-up", [single("wax", "Paraffin fill 14 mm", "Paraffinfüllung 14 mm", "polymer", "paraffin", 0.014, 0.058, 30000)], 30000),
  region("wick", ["Wick", "Docht"], "surface", "passive", "wick-cotton", 0.00003, 0.005, "cylinder", [single("braid", "Braided cotton wick 2 mm", "Geflochtener Baumwolldocht 2 mm", "polymer", "wick-cotton", 0.002, 0.02, 1300)], 1300),
  region("flame", ["Flame", "Flamme"], "heater", "operating", "flame", 0.0006, 0.022, "cylinder", [single("envelope", "Combustion zone 1 mm", "Verbrennungszone 1 mm", "polymer", "flame", 0.001, 0.04, 150)], 150, { operating: { key: "burn", heatW: 40 } }),
];

const bind = (burn, plume) => ({
  regions: {
    cup: { exposedFraction: 0.8, sunlitFraction: 0, contact: { fraction: 0.2, tempC: 21 } },
    wax: { exposedFraction: 1, sunlitFraction: 0 },
    wick: { exposedFraction: 1, sunlitFraction: 0 },
    flame: { exposedFraction: 1, sunlitFraction: 0 },
  },
  emitters: { "flame-plume": plume },
  operating: { burn },
});

const sup = ["base-contact"];
const poses = [
  pose("unlit", ["Unlit", "Unbrennend"], ["The wick is cold, the flame is gone and the wax is solid.", "Der Docht ist kalt, die Flamme fehlt und das Wachs ist fest."], { flame: { dy: -0.0165 }, "flame-tip": { dy: -0.011 } }, sup, "auto", bind(0, 0)),
  pose("lit", ["Lit", "Brennend"], ["The flame stands still and upright and releases 35 W.", "Die Flamme steht ruhig und aufrecht und gibt 35 W ab."], {}, sup, "auto", bind(0.875, 0.875)),
  pose("flicker-peak", ["Flicker peak", "Flackermaximum"], ["A gust leans the flame by 10 degrees, lifts it by 2 mm and stretches its tongue by 4 mm; the release peaks at 40 W.", "Ein Luftzug neigt die Flamme um 10 Grad, hebt sie um 2 mm und streckt ihre Zunge um 4 mm; die Abgabe erreicht 40 W."], { flame: { dy: 0.002 }, "flame-body": { angleDeg: -10 }, "flame-tip": { dy: 0.004 } }, sup, "auto", bind(1, 1)),
];

const contacts = (d) => sup.map((anchor) => ({ anchor, fromS: 0, toS: d }));

const clips = [
  clip(
    "flicker",
    ["Flicker", "Flackern"],
    ["The lit flame leans left and right, stretches its tongue and settles again every 2 seconds; the heat release breathes between 35 and 40 W.", "Die brennende Flamme neigt sich nach links und rechts, streckt ihre Zunge und beruhigt sich alle 2 Sekunden wieder; die Wärmeabgabe schwankt zwischen 35 und 40 W."],
    2,
    15,
    [
      { t: 0, pose: "lit", ease: "smooth" },
      { t: 0.5, pose: "flicker-peak", ease: "smooth" },
      { t: 1, pose: "lit", ease: "smooth", bones: { "flame-body": { angleDeg: 9 } } },
      { t: 1.5, pose: "flicker-peak", ease: "smooth", bones: { "flame-body": { angleDeg: 4 }, "flame-tip": { dy: 0.002 } } },
      { t: 2, pose: "lit", ease: "smooth" },
    ],
    contacts(2),
    [["rest", 0, 0.4, ["Upright", "Aufrecht"]], ["gust", 0.4, 1.4, ["Gusts", "Böen"]], ["settle", 1.4, 2, ["Settling", "Beruhigen"]]],
    [[0.5, "thermal", ["Heat release peak", "Wärmeabgabe-Spitze"], "burn"]],
    [["operating:burn", "smooth", [[0, 0.875], [0.5, 1], [1, 0.875], [1.5, 1], [2, 0.875]]], ["emitter:flame-plume", "smooth", [[0, 0.875], [0.5, 1], [1, 0.875], [1.5, 1], [2, 0.875]]]],
  ),
  clip(
    "light-and-burn",
    ["Light and burn", "Anzünden und abbrennen"],
    ["The wick is lit after two seconds: the flame grows out of the cup within one second and burns on; the cup warms slowly.", "Der Docht wird nach zwei Sekunden angezündet: die Flamme wächst binnen einer Sekunde aus dem Schälchen und brennt weiter; das Schälchen erwärmt sich langsam."],
    300,
    1,
    [{ t: 0, pose: "unlit", ease: "step" }, { t: 2, pose: "unlit", ease: "smooth" }, { t: 3, pose: "lit", ease: "step" }, { t: 300, pose: "lit", ease: "step" }],
    contacts(300),
    [["cold", 0, 2, ["Unlit", "Unbrennend"]], ["grow", 2, 3, ["Flame grows", "Flamme wächst"]], ["burn", 3, 300, ["Burning, cup warming", "Brennt, Schälchen erwärmt sich"]]],
    [[2, "ignition", ["Wick lit", "Docht angezündet"], "burn"]],
    [["operating:burn", "smooth", [[0, 0], [2, 0], [3, 0.875]]], ["emitter:flame-plume", "smooth", [[0, 0], [2, 0], [3, 0.875]]]],
  ),
];

export const tealight = () => {
  const massKg = Math.round(sum(sp.bones.filter((b) => b.mass).map((b) => b.mass.kg)) * 1e6) / 1e6;
  return wrap({
    id: "tealight",
    category: "objects",
    title: tx("Tealight Candle", "Teelicht"),
    summary: tx(
      "A tealight candle of 38 mm diameter and 16 mm height in a thin aluminium cup with a cotton wick and a flame of 22 mm; burning it releases about 35 W as a small plume.",
      "Ein Teelicht von 38 mm Durchmesser und 16 mm Höhe in einem dünnen Aluminiumschälchen mit Baumwolldocht und einer 22 mm hohen Flamme; brennend gibt es etwa 35 W als kleine Fahne ab.",
    ),
    tags: ["candle", "tealight", "flame", "combustion", "internal-gain", "plume", "decor", "open-flame"],
    provenance: {
      author: "Semio staffage content (A10)",
      method: "handcrafted",
      created: "2026-11-01",
      references: [
        ref("din-en-15426-2007", "DIN EN 15426:2007 Candles - Specification for sooting behaviour (candle class and typical dimensions)."),
        ref("drysdale-2011", "Drysdale, D. (2011). An Introduction to Fire Dynamics, 3rd ed., Wiley (burning rate of a candle, flame height and heat release of small diffusion flames)."),
        ref("quintiere-2006", "Quintiere, J. G. (2006). Fundamentals of Fire Phenomena, Wiley (candle flame as a laminar diffusion flame, 40 to 50 percent of the combustion heat is radiated or conducted)."),
        INCROPERA,
      ],
      notes: tx(
        "A standard tealight: a cup of 0.1 mm aluminium foil, 38 mm across the rim and 16 mm high, filled with 12.8 g of paraffin up to 14 mm and a braided cotton wick standing about 5 mm above the wax. The flame is 22 mm high and 8 mm wide at the widest point. A tealight burns about 4 hours, which is 3.2 g of wax per hour or 0.9 mg per second; at the heat of combustion of paraffin of 42 to 46 MJ/kg this is 38 to 41 W, of which roughly 85 percent leave the flame as convective plume and radiation; the pose lit takes 35 W and the flicker peak 40 W (authored estimate from this range). The unlit pose hides the flame inside the cup by lowering the flame bones, so one asset shows all states. The origin is the centre of the base on the surface.",
        "Ein handelsübliches Teelicht: ein Schälchen aus 0,1 mm Aluminiumfolie, 38 mm am Rand weit und 16 mm hoch, mit 12,8 g Paraffin bis 14 mm Höhe gefüllt und einem geflochtenen Baumwolldocht, der etwa 5 mm über das Wachs ragt. Die Flamme ist 22 mm hoch und an der breitesten Stelle 8 mm breit. Ein Teelicht brennt etwa 4 Stunden, das sind 3,2 g Wachs je Stunde oder 0,9 mg je Sekunde; bei einem Heizwert von Paraffin von 42 bis 46 MJ/kg sind das 38 bis 41 W, von denen etwa 85 Prozent als konvektive Fahne und Strahlung die Flamme verlassen; die Pose brennend nimmt 35 W und das Flackermaximum 40 W (Schätzwert aus diesem Bereich). Die Pose unbrennend verbirgt die Flamme im Schälchen, indem sie die Flammenknochen absenkt, sodass ein Asset alle Zustände zeigt. Der Ursprung ist die Mitte des Bodens auf der Unterlage.",
      ),
    },
    dimensions: { heightM: 0.0405, widthM: 0.038, depthM: 0.038, massKg },
    materials: [
      materialOf("aluminium-cup", "Aluminium foil", "Aluminiumfolie", 0.1, 0.3, 2700, 237, 900, "literature", "incropera-2018"),
      materialOf("paraffin", "Paraffin wax", "Paraffinwachs", 0.9, 0.3, 900, 0.24, 2500, SRC, "incropera-2018"),
      materialOf("wick-cotton", "Braided cotton wick", "Geflochtener Baumwolldocht", 0.9, 0.5, 600, 0.06, 1300, SRC),
      materialOf("flame", "Candle flame", "Kerzenflamme", 0.2, 0.05, 0.3, 0.07, 1100, SRC, "quintiere-2006"),
    ],
    rig: { kind: "mechanical", root: "cup", bones: sp.bones.map((b) => ({ ...b, ...(b.mass ? { mass: { ...b.mass } } : {}) })) },
    anchors: [
      { id: "base-contact", bone: "cup", x: 0, y: 0, kind: "contact", title: tx("Base of the cup on the surface", "Boden des Schälchens auf der Unterlage"), support: "contact", footprintLengthM: 0.031, footprintWidthM: 0.031, frictionCoefficient: 0.4 },
      { id: "flame-top", bone: "flame-tip", x: 0, y: 0.0405, kind: "emitter", title: tx("Tip of the flame", "Spitze der Flamme") },
    ],
    representations: { 500: rep500, 200: rep200, 100: rep100, 50: rep50 },
    symbols: [],
    poses,
    clips,
    thermal: {
      environment: { airTempC: 21, meanRadiantTempC: 21, relativeHumidity: 0.5, windMS: 0.1, windDirectionDeg: 0, solarWM2: 0, diffuseFraction: 0, pressurePa: 101325 },
      passengers: [],
      operating: [{ id: "burn", title: tx("Combustion", "Verbrennung"), levels: [level(0, "Unlit", "Unbrennend"), level(0.875, "Steady flame (35 W)", "Ruhige Flamme (35 W)"), level(1, "Flicker peak (40 W)", "Flackermaximum (40 W)")] }],
      regions,
      emitters: [{ id: "flame-plume", kind: "plume", title: tx("Convective plume above the flame", "Konvektive Fahne über der Flamme"), anchor: "flame-top", directionDeg: 90, diameterM: 0.02, region: "flame", source: SRC, reference: "quintiere-2006" }],
      assumptions: [
        tx(
          "The 35 to 40 W of combustion are released into the flame region and leave as a buoyant plume and by radiation; soot, carbon dioxide and the 0.9 mg/s of water vapour are neglected.",
          "Die 35 bis 40 W der Verbrennung werden in die Flammenregion freigesetzt und verlassen sie als aufsteigende Fahne und durch Strahlung; Ruß, Kohlendioxid und die 0,9 mg/s Wasserdampf bleiben unberücksichtigt.",
        ),
        tx(
          "The wax and the cup stay passive: the melt pool and the cup warming are carried by the conduction contact of the cup with the surface only.",
          "Wachs und Schälchen bleiben passiv: Schmelzbad und Erwärmung des Schälchens werden nur über den Leitungskontakt des Schälchens mit der Unterlage getragen.",
        ),
      ],
    },
    structure: {
      gravityMS2: 9.80665,
      dynamics: "quasi-static",
      supports: sup,
      assumptions: [
        tx(
          "The 14 g candle stands on its base: a unilateral contact of 31 mm by 31 mm carries the weight.",
          "Die 14 g schwere Kerze steht auf ihrem Boden: ein einseitiger Kontakt von 31 mm mal 31 mm trägt das Gewicht.",
        ),
      ],
      clinical: false,
    },
    presentation: { defaultPose: "lit", defaultClip: "flicker", defaultScale: 50, viewpoints: ["elevation"] },
    exports: ALL_EXPORTS,
    completion: "authored",
  });
};
