/** 🌷️ Design of the tulip tree (Liriodendron tulipifera), an open-grown park specimen of about 80 years: 30 m high, a straight columnar bole clear to 7 m that runs as one leader to the top, seventeen limbs in a spiral of the golden angle rising steeper towards the top, and a dense columnar-oval crown about 14 m wide.
 * Every number is chosen by hand against the references; `assemble.ts` builds the asset from it.
 * @see ./tree-kit.ts
 * @see ./common.ts */
import { REFERENCES as R, limb, lobe, windPoseTexts } from "./common.ts";
import type { Limb, Mass, Species, Text } from "./tree-kit.ts";

const name: Text = { en: "Tulip Tree", de: "Tulpenbaum" };
const L = (n: number): Text => ({ en: `Limb ${n}`, de: `Ast ${n}` });
const side = (elA: number) => [{ seg: 0, at: 0.62, dAz: 38, el: elA + 12, len: 1.5, r: 0.045 }, { seg: 0, at: 0.4, dAz: -42, el: elA + 18, len: 1.2, r: 0.038 }, { seg: 1, at: 0.55, dAz: -30, el: 30, len: 1.1, r: 0.03 }];
const T = (n: number, parent: string, at: number, az: number, rc: number, els: [number, number, number], size: [number, number, number], r0: number, spin = 0): Limb => limb(`l${n}`, L(n), parent, at, az, rc, els, size, r0, { spin, side: side(els[0]) });

const limbs: Limb[] = [
  T(1, "ld1", 0.5, 20, 4.4, [22, 4, 18], [1.9, 1.5, 2.1], 0.17, 0),
  T(2, "ld1", 1.7, 157, 4.8, [26, 8, 20], [2.0, 1.6, 2.2], 0.18, 25),
  T(3, "ld1", 2.9, 295, 5.0, [28, 10, 22], [2.0, 1.7, 2.2], 0.18, -20),
  T(4, "ld1", 4.1, 72, 5.1, [32, 14, 24], [2.0, 1.7, 2.2], 0.17, 40),
  T(5, "ld1", 5.3, 210, 5.0, [34, 16, 26], [2.0, 1.7, 2.2], 0.16, -35),
  T(6, "ld2", 0.4, 347, 4.9, [36, 18, 28], [1.9, 1.7, 2.1], 0.15, 15),
  T(7, "ld2", 1.6, 125, 4.8, [38, 20, 30], [1.9, 1.7, 2.1], 0.145, -10),
  T(8, "ld2", 2.8, 262, 4.6, [40, 22, 32], [1.8, 1.7, 2.0], 0.14, 30),
  T(9, "ld2", 4.0, 40, 4.4, [42, 24, 34], [1.8, 1.6, 2.0], 0.13, -25),
  T(10, "ld2", 5.2, 177, 4.1, [44, 26, 36], [1.7, 1.6, 1.9], 0.12, 10),
  T(11, "ld3", 0.5, 315, 3.8, [46, 30, 38], [1.6, 1.5, 1.8], 0.11, -15),
  T(12, "ld3", 1.8, 92, 3.5, [48, 32, 40], [1.5, 1.5, 1.7], 0.1, 20),
  T(13, "ld3", 3.1, 230, 3.1, [50, 34, 42], [1.4, 1.4, 1.6], 0.09, -30),
  T(14, "ld3", 4.4, 7, 2.7, [52, 36, 44], [1.3, 1.3, 1.5], 0.08, 35),
  T(15, "ld4", 0.4, 145, 2.2, [54, 40, 46], [1.2, 1.2, 1.4], 0.07, 0),
  T(16, "ld4", 1.4, 282, 1.8, [56, 42, 48], [1.1, 1.1, 1.3], 0.06, 15),
  T(17, "ld4", 2.4, 60, 1.4, [58, 44, 50], [1.0, 1.0, 1.2], 0.05, -15),
];

const core = (id: string, bone: string, y: number, r: [number, number, number], x = 0, z = 0): Mass => ({ id, bone, c: [x, y, z], radii: r, az: 0, group: "shade" });

export const species: Species = {
  id: "tulip-tree",
  title: name,
  summary: { en: "An open-grown tulip tree of about 80 years, 30 m high: a straight, columnar bole clear to 7 m runs as one leader to the top, seventeen limbs leave it in a spiral and rise ever steeper, and the large four-lobed leaves form a dense, columnar-oval crown about 14 m wide.", de: "Ein frei gewachsener Tulpenbaum von etwa 80 Jahren, 30 m hoch: Ein gerader, säulenförmiger Stamm, astfrei bis 7 m, läuft als ein Leittrieb bis zur Spitze, siebzehn Äste verlassen ihn in einer Spirale und steigen nach oben immer steiler an, und die großen vierlappigen Blätter bilden eine dichte, säulenförmig-ovale Krone von etwa 14 m Breite." },
  tags: ["tree", "deciduous", "broadleaf", "columnar-oval-crown", "park-tree", "wind", "transpiration"],
  latin: "Liriodendron tulipifera",
  references: [R.woodHandbook, R.en199114, R.en199113, R.vogel, R.jones, R.monteith, R.silvics, R.dirr, R.roloff],
  notes: { en: "Rig and model are authored by hand for a park tulip tree of about 80 years grown in the open: 30 m high (Silvics of North America gives 24 to 37 m and stem diameters of 0.6 to 1.5 m on good sites), stem diameter 0.96 m at breast height, clear bole 7 m, crown 14 m wide. Wood values are those of green yellow-poplar (Wood Handbook, Table 5-3a: specific gravity 0.40, E 8.4 GPa, modulus of rupture 41 MPa; green weight about 610 kg/m3). The leaf area index, the conductances and the foliage masses are authored estimates.", de: "Rig und Modell sind von Hand für einen frei gewachsenen Park-Tulpenbaum von etwa 80 Jahren entworfen: 30 m hoch (Silvics of North America nennt 24 bis 37 m und Stammdurchmesser von 0,6 bis 1,5 m auf guten Standorten), Brusthöhendurchmesser 0,96 m, astfreier Stamm 7 m, Krone 14 m breit. Die Holzwerte sind die von frischem Tulpenbaumholz (Wood Handbook, Tabelle 5-3a: Rohdichtezahl 0,40, E 8,4 GPa, Biegefestigkeit 41 MPa; Frischgewicht etwa 610 kg/m3). Blattflächenindex, Leitfähigkeiten und Laubmassen sind Autorenschätzungen." },
  distinction: { en: "A 30 m tulip tree (Liriodendron tulipifera) with a straight columnar bole clear to 7 m, a single leader to the top and a dense columnar-oval crown of large four-lobed leaves, about 14 m wide.", de: "Ein 30 m hoher Tulpenbaum (Liriodendron tulipifera) mit geradem, säulenförmigem Stamm, astfrei bis 7 m, einem Leittrieb bis zur Spitze und einer dichten, säulenförmig-ovalen Krone aus großen vierlappigen Blättern von etwa 14 m Breite." },
  materials: [
    { id: "tulip-bark", title: { en: "Tulip tree bark, grey with interlacing shallow furrows", de: "Tulpenbaumrinde, grau mit flechtwerkartigen flachen Furchen" }, emissivity: 0.95, solarAbsorptivity: 0.72, densityKgM3: 600, conductivityWmK: 0.12, source: "authored-estimate", reference: "wood-handbook-2021" },
    { id: "tulip-bark-young", title: { en: "Smooth greyish bark of the limbs", de: "Glatte gräuliche Rinde der Äste" }, emissivity: 0.95, solarAbsorptivity: 0.65, densityKgM3: 620, conductivityWmK: 0.13, source: "authored-estimate", reference: "wood-handbook-2021" },
    { id: "tulip-leaf-sun", title: { en: "Tulip tree sun leaves", de: "Tulpenbaum-Sonnenblätter" }, emissivity: 0.97, solarAbsorptivity: 0.5, source: "literature", reference: "jones-2014" },
    { id: "tulip-leaf-shade", title: { en: "Tulip tree shade leaves (effective, shaded by the outer crown)", de: "Tulpenbaum-Schattenblätter (wirksam, durch die Außenkrone beschattet)" }, emissivity: 0.97, solarAbsorptivity: 0.22, source: "authored-estimate", reference: "jones-2014" },
    { id: "tulip-wood", title: { en: "Green yellow-poplar wood", de: "Frisches Tulpenbaumholz" }, emissivity: 0.9, solarAbsorptivity: 0.65, densityKgM3: 610, conductivityWmK: 0.14, elasticModulusPa: 8400000000, strengthPa: 41000000, source: "literature", reference: "wood-handbook-2021" },
  ],
  wood: { material: "tulip-wood", density: 610, bark: "tulip-bark", barkYoung: "tulip-bark-young", leafSun: "tulip-leaf-sun", leafShade: "tulip-leaf-shade" },
  root: { len: 0.6, flareR: 0.85, topR: 0.5, roots: [{ az: 15, len: 1.3, r: 0.19 }, { az: 78, len: 1.1, r: 0.16 }, { az: 140, len: 1.4, r: 0.2 }, { az: 205, len: 1.2, r: 0.17 }, { az: 258, len: 1.35, r: 0.19 }, { az: 322, len: 1.05, r: 0.15 }] },
  stem: [
    { id: "trunk", title: { en: "Clear bole", de: "Astfreier Stamm" }, len: 6.4, r0: 0.49, r1: 0.41, leanDeg: 0.3, limit: 3 },
    { id: "ld1", title: { en: "Stem through the lower crown", de: "Stamm durch die untere Krone" }, len: 6, r0: 0.41, r1: 0.33, leanDeg: 0.1, limit: 3 },
    { id: "ld2", title: { en: "Stem through the middle crown", de: "Stamm durch die mittlere Krone" }, len: 6, r0: 0.33, r1: 0.24, leanDeg: -0.2, limit: 4 },
    { id: "ld3", title: { en: "Leader in the upper crown", de: "Leittrieb in der oberen Krone" }, len: 5.5, r0: 0.24, r1: 0.14, leanDeg: 0, limit: 5 },
    { id: "ld4", title: { en: "Leader below the top", de: "Leittrieb unter der Spitze" }, len: 3.5, r0: 0.14, r1: 0.07, leanDeg: 0.4, limit: 6 },
  ],
  top: { len: 2.1, r: 0.035, size: [1.1, 0.95, 1.1] },
  limbs,
  templates: {
    50: [lobe(0, 0, 0, 1), lobe(0.55, 0.35, 0.12, 0.56, 0.6, 0.55), lobe(0.08, 0.72, -0.34, 0.5), lobe(0.15, 0.62, 0.46, 0.5), lobe(0.5, -0.36, 0.5, 0.5, 0.45, 0.5), lobe(0.45, -0.4, -0.52, 0.5, 0.45, 0.5), lobe(-0.32, 0.56, 0.02, 0.5, 0.5, 0.56)],
    100: [lobe(0, 0, 0, 1.02), lobe(0.55, 0.35, 0.12, 0.58, 0.62, 0.57), lobe(0.08, 0.72, -0.34, 0.52), lobe(0.15, 0.62, 0.46, 0.52), lobe(0.5, -0.36, 0.5, 0.52, 0.47, 0.52), lobe(0.45, -0.4, -0.52, 0.52, 0.47, 0.52)],
    200: [lobe(0.0, 0.0, 0.0, 1.061, 1.061, 1.061), lobe(0.55, 0.35, 0.12, 0.603, 0.645, 0.593), lobe(0.08, 0.72, -0.34, 0.541, 0.541, 0.541), lobe(0.15, 0.62, 0.46, 0.541, 0.541, 0.541), lobe(0.5, -0.36, 0.5, 0.541, 0.489, 0.541), lobe(0.45, -0.4, -0.52, 0.541, 0.489, 0.541)],
    500: [lobe(0.0, 0.0, 0.0, 1.103, 1.103, 1.103), lobe(0.55, 0.35, 0.12, 0.627, 0.671, 0.617), lobe(0.08, 0.72, -0.34, 0.563, 0.563, 0.563), lobe(0.15, 0.62, 0.46, 0.563, 0.563, 0.563), lobe(0.5, -0.36, 0.5, 0.563, 0.509, 0.563), lobe(0.45, -0.4, -0.52, 0.563, 0.509, 0.563)],
  },
  coarse: { 50: 1, 100: 1, 200: 1, 500: 1 },
  shade: {
    50: [core("shade-1", "ld1", 10.6, [2.99, 3.24, 2.99]), core("shade-2", "ld2", 15.6, [3.45, 3.78, 3.45]), core("shade-3", "ld2", 19.8, [2.88, 3.24, 2.88]), core("shade-4", "ld3", 23.8, [2.07, 2.7, 2.07]), core("shade-5", "ld3", 26.6, [1.26, 1.76, 1.26])],
    100: [core("shade-1", "ld1", 10.6, [3.05, 3.3, 3.05]), core("shade-2", "ld2", 15.6, [3.52, 3.86, 3.52]), core("shade-3", "ld2", 19.8, [2.94, 3.3, 2.94]), core("shade-4", "ld3", 23.8, [2.11, 2.75, 2.11]), core("shade-5", "ld3", 26.6, [1.29, 1.8, 1.29])],
    200: [core("shade-1", "ld1", 10.6, [3.14, 3.4, 3.14]), core("shade-2", "ld2", 15.6, [3.62, 3.97, 3.62]), core("shade-3", "ld2", 19.8, [3.02, 3.4, 3.02]), core("shade-4", "ld3", 23.8, [2.17, 2.84, 2.17])],
    500: [core("shade-1", "ld1", 10.6, [3.29, 3.56, 3.29]), core("shade-2", "ld2", 15.6, [3.8, 4.16, 3.8]), core("shade-3", "ld2", 19.8, [3.17, 3.56, 3.17])],
  },
  crown500: [],

  leaf: { sunGs: 0.0045, shadeGs: 0.0028, size: 0.1, kgPerM2: 0.2, lai: 5.5, sunShare: 0.5, porosity: 0.6, snowKgM2: 8 },
  heights: { crownBase: 7 },
  sway: { periodS: 5, windyLean: 2, stormLean: 5 },
  poseTexts: windPoseTexts(name, 0.6, 8),
  scaleTexts: {
    50: { silhouette: { en: "flared root collar with six root spurs, the straight bole, the leader in four segments, seventeen limbs with their side branches and end shoots, and the leaves of every limb system as one lobed cluster of seven masses around the shaded inner crown.", de: "Wurzelhals mit sechs Wurzelanläufen, der gerade Stamm, der Leittrieb in vier Abschnitten, siebzehn Äste mit Seitenzweigen und Endtrieben und das Laub jedes Astsystems als ein gelapptes Büschel aus sieben Massen um die beschattete Innenkrone." }, body: { en: "the tulip tree as it grows: a root collar of 1.7 m with six root spurs carries a bole of 0.96 m at breast height, clear to 7 m, that runs as one leader to the top at 30 m; seventeen limbs leave it in a spiral of the golden angle, the lowest almost level, the upper ones rising ever steeper, and every limb ends in a cluster of seven foliage masses; five shade cores fill the inner crown along the stem.", de: "Der Tulpenbaum, wie er wächst: Ein Wurzelhals von 1,7 m mit sechs Wurzelanläufen trägt einen Stamm von 0,96 m in Brusthöhe, astfrei bis 7 m, der als ein Leittrieb bis zur Spitze in 30 m durchläuft; siebzehn Äste verlassen ihn in einer Spirale des goldenen Winkels, die untersten fast waagrecht, die oberen immer steiler, und jeder Ast endet in einem Büschel aus sieben Laubmassen; fünf Schattenkerne füllen die Innenkrone entlang des Stamms." } },
    100: { silhouette: { en: "root collar with three spurs, bole and leader, seventeen limbs with their stronger side branches, and each limb system as one lobed outline of four masses.", de: "Wurzelhals mit drei Anläufen, Stamm und Leittrieb, siebzehn Äste mit ihren stärkeren Seitenzweigen und jedes Astsystem als ein gelappter Umriss aus vier Massen." }, body: { en: "the same stem and seventeen limbs; the thinnest side branches and three of the six root spurs are below what this scale models, each limb cluster is four larger masses and three shade cores fill the inner crown.", de: "Derselbe Stamm und siebzehn Äste; die dünnsten Seitenzweige und drei der sechs Wurzelanläufe liegen unter dem, was dieser Maßstab modelliert, jedes Astbüschel besteht aus vier größeren Massen und drei Schattenkerne füllen die Innenkrone." } },
    200: { silhouette: { en: "bole and leader, the seventeen limbs where they show below and between the clusters, and each limb system as one outline of two masses.", de: "Stamm und Leittrieb, die siebzehn Äste, wo sie unter und zwischen den Büscheln sichtbar werden, und jedes Astsystem als ein Umriss aus zwei Massen." }, body: { en: "stem, flare and the seventeen limbs without side branches and end shoots; every limb cluster is one main mass with one shoulder, around two shade cores.", de: "Stamm, Wurzelanlauf und die siebzehn Äste ohne Seitenzweige und Endtriebe; jedes Astbüschel ist eine Hauptmasse mit einer Schulter um zwei Schattenkerne." } },
    500: { silhouette: { en: "the clear bole on its flare and the crown as twelve large masses in four tiers and a rounded top, merged into one outline per stem segment.", de: "der astfreie Stamm auf seinem Anlauf und die Krone als zwölf große Massen in vier Etagen und eine gerundete Spitze, je Stammabschnitt zu einem Umriss verschmolzen." }, body: { en: "stem and flare, the inner and outer limb parts, and the columnar-oval crown as three rings of three masses, two head masses and the top around one long shade core.", de: "Stamm und Anlauf, die inneren und äußeren Astteile und die säulenförmig-ovale Krone als drei Ringe aus drei Massen, zwei Kopfmassen und die Spitze um einen langen Schattenkern." } },
  },
};
