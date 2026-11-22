/** 🍂️ Design of the northern red oak (Quercus rubra), an open-grown park tree of about 70 years: 22 m high, a short stout bole that loses itself in big spreading limbs from 3 m on, and a broad, round crown about 19 m wide of large bristle-lobed leaves.
 * Every number is chosen by hand against the references; `assemble.ts` builds the asset from it.
 * @see ./tree-kit.ts
 * @see ./common.ts */
import { REFERENCES as R, limb, lobe, windPoseTexts } from "./common.ts";
import type { Limb, Mass, Species, Text } from "./tree-kit.ts";

const name: Text = { en: "Red Oak", de: "Roteiche" };
const L = (n: number): Text => ({ en: `Limb ${n}`, de: `Ast ${n}` });
const side = (elA: number, r: number) => [{ seg: 0, at: 0.55, dAz: 42, el: elA + 8, len: 2.1, r: Math.max(0.03, 0.26 * r) }, { seg: 0, at: 0.78, dAz: -38, el: elA + 14, len: 1.8, r: Math.max(0.028, 0.22 * r) }, { seg: 1, at: 0.5, dAz: 32, el: 24, len: 1.6, r: Math.max(0.025, 0.17 * r) }];
const O = (n: number, parent: string, at: number, az: number, rc: number, els: [number, number, number], size: [number, number, number], r0: number, spin = 0): Limb => limb(`l${n}`, L(n), parent, at, az, rc, els, size, r0, { spin, side: side(els[0], r0) });

const limbs: Limb[] = [
  O(1, "trunk", 2.7, 15, 6.5, [14, 8, 24], [2.6, 2.0, 2.8], 0.26, 0),
  O(2, "trunk", 2.85, 140, 6.7, [15, 7, 22], [2.7, 2.0, 2.9], 0.27, 30),
  O(3, "ld1", 0.3, 255, 6.6, [17, 8, 22], [2.6, 2.1, 2.9], 0.26, -25),
  O(4, "ld1", 1.2, 72, 6.4, [22, 11, 24], [2.6, 2.2, 2.8], 0.24, 20),
  O(5, "ld1", 2.0, 200, 6.5, [24, 11, 24], [2.6, 2.2, 2.8], 0.24, -30),
  O(6, "ld1", 2.9, 322, 6.3, [28, 15, 26], [2.5, 2.2, 2.8], 0.22, 15),
  O(7, "ld1", 3.8, 108, 5.6, [34, 22, 28], [2.4, 2.2, 2.6], 0.2, -10),
  O(8, "ld2", 0.3, 232, 5.4, [38, 26, 30], [2.4, 2.2, 2.6], 0.19, 25),
  O(9, "ld2", 1.4, 0, 5.0, [42, 30, 32], [2.3, 2.2, 2.5], 0.17, -20),
  O(10, "ld2", 2.6, 165, 4.6, [46, 34, 34], [2.2, 2.1, 2.4], 0.16, 10),
  O(11, "ld2", 3.8, 290, 4.2, [50, 36, 36], [2.1, 2.0, 2.3], 0.14, -15),
  O(12, "ld3", 0.8, 52, 3.4, [54, 40, 40], [1.9, 1.9, 2.1], 0.12, 35),
  O(13, "ld3", 2.2, 198, 2.8, [56, 44, 42], [1.7, 1.8, 1.9], 0.1, -20),
  O(14, "ld3", 3.4, 330, 2.2, [58, 46, 44], [1.5, 1.6, 1.7], 0.08, 0),
];

const core = (id: string, bone: string, y: number, r: [number, number, number], x = 0, z = 0): Mass => ({ id, bone, c: [x, y, z], radii: r, az: 0, group: "shade" });

export const species: Species = {
  id: "red-oak",
  title: name,
  summary: { en: "An open-grown northern red oak of about 70 years, 22 m high: a short, stout bole with the flat-ridged, fissured bark of the species divides from 3 m on into fourteen big, spreading limbs, and the large, bristle-lobed leaves form a broad, round crown about 19 m wide.", de: "Eine frei gewachsene Roteiche von etwa 70 Jahren, 22 m hoch: Ein kurzer, kräftiger Stamm mit der flachrippigen, gefurchten Borke der Art teilt sich ab 3 m in vierzehn große, ausladende Äste, und die großen, grannig gelappten Blätter bilden eine breite, runde Krone von etwa 19 m Breite." },
  tags: ["tree", "deciduous", "broadleaf", "broad-crown", "park-tree", "wind", "transpiration"],
  latin: "Quercus rubra",
  references: [R.woodHandbook, R.en199114, R.en199113, R.vogel, R.jones, R.monteith, R.silvics, R.dirr, R.roloff],
  notes: { en: "Rig and model are authored by hand for a park red oak of about 70 years grown in the open: 22 m high (Silvics of North America gives 20 to 30 m and stem diameters of 0.6 to 0.9 m; open-grown trees have a short bole and a broad, round crown), stem diameter 0.86 m at breast height, crown 19 m wide. Wood values are those of green northern red oak (Wood Handbook, Table 5-3a: specific gravity 0.56, E 9.3 GPa, modulus of rupture 57 MPa; green weight about 1000 kg/m3). The leaf area index, the conductances and the foliage masses are authored estimates.", de: "Rig und Modell sind von Hand für eine frei gewachsene Park-Roteiche von etwa 70 Jahren entworfen: 22 m hoch (Silvics of North America nennt 20 bis 30 m und Stammdurchmesser von 0,6 bis 0,9 m; frei gewachsene Bäume haben einen kurzen Stamm und eine breite, runde Krone), Brusthöhendurchmesser 0,86 m, Krone 19 m breit. Die Holzwerte sind die von frischem Roteichenholz (Wood Handbook, Tabelle 5-3a: Rohdichtezahl 0,56, E 9,3 GPa, Biegefestigkeit 57 MPa; Frischgewicht etwa 1000 kg/m3). Blattflächenindex, Leitfähigkeiten und Laubmassen sind Autorenschätzungen." },
  distinction: { en: "A 22 m northern red oak (Quercus rubra) with a short, stout bole of flat-ridged bark that divides into big spreading limbs from 3 m on, and a broad, round crown of large bristle-lobed leaves about 19 m wide.", de: "Eine 22 m hohe Roteiche (Quercus rubra) mit kurzem, kräftigem Stamm aus flachrippiger Borke, der sich ab 3 m in große, ausladende Äste teilt, und einer breiten, runden Krone aus großen, grannig gelappten Blättern von etwa 19 m Breite." },
  materials: [
    { id: "red-oak-bark", title: { en: "Red oak bark, dark grey with shallow fissures between flat, shiny ridges", de: "Roteichenborke, dunkelgrau mit flachen Furchen zwischen flachen, glänzenden Rippen" }, emissivity: 0.95, solarAbsorptivity: 0.78, densityKgM3: 650, conductivityWmK: 0.12, source: "authored-estimate", reference: "wood-handbook-2021" },
    { id: "red-oak-bark-young", title: { en: "Smooth grey bark of the limbs", de: "Glatte graue Rinde der Äste" }, emissivity: 0.95, solarAbsorptivity: 0.68, densityKgM3: 680, conductivityWmK: 0.13, source: "authored-estimate", reference: "wood-handbook-2021" },
    { id: "red-oak-leaf-sun", title: { en: "Red oak sun leaves", de: "Roteichen-Sonnenblätter" }, emissivity: 0.97, solarAbsorptivity: 0.5, source: "literature", reference: "jones-2014" },
    { id: "red-oak-leaf-shade", title: { en: "Red oak shade leaves (effective, shaded by the outer crown)", de: "Roteichen-Schattenblätter (wirksam, durch die Außenkrone beschattet)" }, emissivity: 0.97, solarAbsorptivity: 0.22, source: "authored-estimate", reference: "jones-2014" },
    { id: "red-oak-wood", title: { en: "Green northern red oak wood", de: "Frisches Roteichenholz" }, emissivity: 0.9, solarAbsorptivity: 0.7, densityKgM3: 1000, conductivityWmK: 0.17, elasticModulusPa: 9300000000, strengthPa: 57000000, source: "literature", reference: "wood-handbook-2021" },
  ],
  wood: { material: "red-oak-wood", density: 1000, bark: "red-oak-bark", barkYoung: "red-oak-bark-young", leafSun: "red-oak-leaf-sun", leafShade: "red-oak-leaf-shade" },
  root: { len: 0.5, flareR: 0.95, topR: 0.46, roots: [{ az: 25, len: 1.4, r: 0.2 }, { az: 95, len: 1.2, r: 0.18 }, { az: 160, len: 1.5, r: 0.21 }, { az: 220, len: 1.1, r: 0.17 }, { az: 285, len: 1.45, r: 0.2 }, { az: 340, len: 1.2, r: 0.17 }] },
  stem: [
    { id: "trunk", title: { en: "Short bole", de: "Kurzer Stamm" }, len: 2.9, r0: 0.44, r1: 0.39, leanDeg: 0.5, limit: 3 },
    { id: "ld1", title: { en: "Stem among the main limbs", de: "Stamm zwischen den Hauptästen" }, len: 5, r0: 0.36, r1: 0.27, leanDeg: -0.6, limit: 3 },
    { id: "ld2", title: { en: "Stem in the middle crown", de: "Stamm in der mittleren Krone" }, len: 4.6, r0: 0.26, r1: 0.17, leanDeg: 0.8, limit: 4 },
    { id: "ld3", title: { en: "Leader in the upper crown", de: "Leittrieb in der oberen Krone" }, len: 4, r0: 0.17, r1: 0.08, leanDeg: -0.5, limit: 6 },
  ],
  top: { len: 2.5, r: 0.035, size: [1.8, 1.8, 1.8] },
  limbs,
  templates: {
    50: [lobe(0, 0, 0, 1), lobe(0.56, 0.28, 0.1, 0.56, 0.58, 0.55), lobe(0.04, 0.64, -0.38, 0.52), lobe(0.12, 0.6, 0.42, 0.5), lobe(0.5, -0.3, 0.48, 0.5, 0.44, 0.5), lobe(0.42, -0.34, -0.5, 0.5, 0.44, 0.5), lobe(-0.36, 0.5, 0, 0.5, 0.48, 0.55), lobe(0.74, 0.02, -0.12, 0.42, 0.42, 0.44)],
    100: [lobe(0, 0, 0, 1.02), lobe(0.56, 0.28, 0.1, 0.58, 0.6, 0.57), lobe(0.04, 0.64, -0.38, 0.54), lobe(0.12, 0.6, 0.42, 0.52), lobe(0.5, -0.3, 0.48, 0.52, 0.46, 0.52), lobe(0.42, -0.34, -0.5, 0.52, 0.46, 0.52), lobe(0.74, 0.02, -0.12, 0.44, 0.44, 0.46)],
    200: [lobe(0.0, 0.0, 0.0, 1.061, 1.061, 1.061), lobe(0.56, 0.28, 0.1, 0.603, 0.624, 0.593), lobe(0.04, 0.64, -0.38, 0.562, 0.562, 0.562), lobe(0.12, 0.6, 0.42, 0.541, 0.541, 0.541), lobe(0.5, -0.3, 0.48, 0.541, 0.478, 0.541), lobe(0.42, -0.34, -0.5, 0.541, 0.478, 0.541), lobe(0.74, 0.02, -0.12, 0.458, 0.458, 0.478)],
    500: [lobe(0.0, 0.0, 0.0, 1.103, 1.103, 1.103), lobe(0.56, 0.28, 0.1, 0.627, 0.649, 0.617), lobe(0.04, 0.64, -0.38, 0.584, 0.584, 0.584), lobe(0.12, 0.6, 0.42, 0.563, 0.563, 0.563), lobe(0.5, -0.3, 0.48, 0.563, 0.497, 0.563), lobe(0.42, -0.34, -0.5, 0.563, 0.497, 0.563), lobe(0.74, 0.02, -0.12, 0.476, 0.476, 0.497)],
  },
  coarse: { 50: 1, 100: 1, 200: 1, 500: 1 },
  shade: {
    50: [core("shade-1", "ld1", 6.6, [3.6, 2.2, 3.6]), core("shade-2", "ld1", 9.2, [3.9, 2.4, 3.9]), core("shade-3", "ld2", 12.4, [3.5, 2.5, 3.5]), core("shade-4", "ld3", 15.4, [2.6, 2.2, 2.6]), core("shade-5", "ld3", 17.9, [1.5, 1.5, 1.5])],
    100: [core("shade-1", "ld1", 6.6, [3.67, 2.24, 3.67]), core("shade-2", "ld1", 9.2, [3.98, 2.45, 3.98]), core("shade-3", "ld2", 12.4, [3.57, 2.55, 3.57]), core("shade-4", "ld3", 15.4, [2.65, 2.24, 2.65]), core("shade-5", "ld3", 17.9, [1.53, 1.53, 1.53])],
    200: [core("shade-1", "ld1", 6.6, [3.78, 2.31, 3.78]), core("shade-2", "ld1", 9.2, [4.09, 2.52, 4.09]), core("shade-3", "ld2", 12.4, [3.68, 2.62, 3.68]), core("shade-4", "ld3", 15.4, [2.73, 2.31, 2.73])],
    500: [core("shade-1", "ld1", 6.6, [3.96, 2.42, 3.96]), core("shade-2", "ld1", 9.2, [4.29, 2.64, 4.29]), core("shade-3", "ld2", 12.4, [3.85, 2.75, 3.85])],
  },
  crown500: [],

  ridges: { count: 5, depth: 0.03, width: 0.05, scales: [50] },
  leaf: { sunGs: 0.0035, shadeGs: 0.0022, size: 0.08, kgPerM2: 0.18, lai: 4.5, sunShare: 0.55, porosity: 0.55, snowKgM2: 10 },
  heights: { crownBase: 3 },
  sway: { periodS: 4.5, windyLean: 1.6, stormLean: 4.2 },
  poseTexts: windPoseTexts(name, 0.55, 10),
  scaleTexts: {
    50: { silhouette: { en: "flared root collar with six root spurs, the short bole with its bark ridges, the leader in three segments, fourteen spreading limbs with their side branches and end shoots, and the leaves of every limb system as one lobed cluster of eight masses around the shaded inner crown.", de: "Wurzelhals mit sechs Wurzelanläufen, der kurze Stamm mit seinen Borkenrippen, der Leittrieb in drei Abschnitten, vierzehn ausladende Äste mit Seitenzweigen und Endtrieben und das Laub jedes Astsystems als ein gelapptes Büschel aus acht Massen um die beschattete Innenkrone." }, body: { en: "the red oak as it grows: a root collar of 1.9 m with six root spurs carries a short bole of 0.86 m at breast height that divides from 3 m on; fourteen big limbs leave it, the lowest sweeping out almost level to 9 m, the upper ones rising ever steeper, and every limb ends in a cluster of eight foliage masses; five shade cores fill the inner crown along the stem.", de: "Die Roteiche, wie sie wächst: Ein Wurzelhals von 1,9 m mit sechs Wurzelanläufen trägt einen kurzen Stamm von 0,86 m in Brusthöhe, der sich ab 3 m teilt; vierzehn große Äste verlassen ihn, die untersten schwingen fast waagrecht bis 9 m aus, die oberen steigen immer steiler, und jeder Ast endet in einem Büschel aus acht Laubmassen; fünf Schattenkerne füllen die Innenkrone entlang des Stamms." } },
    100: { silhouette: { en: "root collar with three spurs, bole and leader, fourteen limbs with their stronger side branches, and each limb system as one lobed outline of four masses.", de: "Wurzelhals mit drei Anläufen, Stamm und Leittrieb, vierzehn Äste mit ihren stärkeren Seitenzweigen und jedes Astsystem als ein gelappter Umriss aus vier Massen." }, body: { en: "the same stem and fourteen limbs; the thinnest side branches and three of the six root spurs are below what this scale models, each limb cluster is four larger masses and three shade cores fill the inner crown.", de: "Derselbe Stamm und vierzehn Äste; die dünnsten Seitenzweige und drei der sechs Wurzelanläufe liegen unter dem, was dieser Maßstab modelliert, jedes Astbüschel besteht aus vier größeren Massen und drei Schattenkerne füllen die Innenkrone." } },
    200: { silhouette: { en: "bole and leader, the fourteen limbs where they show below and between the clusters, and each limb system as one outline of two masses.", de: "Stamm und Leittrieb, die vierzehn Äste, wo sie unter und zwischen den Büscheln sichtbar werden, und jedes Astsystem als ein Umriss aus zwei Massen." }, body: { en: "stem, flare and the fourteen limbs without side branches and end shoots; every limb cluster is one main mass with one shoulder, around two shade cores.", de: "Stamm, Wurzelanlauf und die vierzehn Äste ohne Seitenzweige und Endtriebe; jedes Astbüschel ist eine Hauptmasse mit einer Schulter um zwei Schattenkerne." } },
    500: { silhouette: { en: "the short bole on its flare and the broad crown as twenty large masses in four tiers and a rounded top, merged into one outline per stem segment.", de: "der kurze Stamm auf seinem Anlauf und die breite Krone als zwanzig große Massen in vier Etagen und eine gerundete Spitze, je Stammabschnitt zu einem Umriss verschmolzen." }, body: { en: "stem and flare, the inner and outer limb parts, and the broad round crown as two rings of six masses, a ring of five, three head masses and the top around one shade core.", de: "Stamm und Anlauf, die inneren und äußeren Astteile und die breite runde Krone als zwei Ringe aus sechs Massen, ein Ring aus fünf, drei Kopfmassen und die Spitze um einen Schattenkern." } },
  },
};
