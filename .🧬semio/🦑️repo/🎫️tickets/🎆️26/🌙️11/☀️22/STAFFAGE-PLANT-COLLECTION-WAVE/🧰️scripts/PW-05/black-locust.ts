/** 🌼️ Design of the black locust (Robinia pseudoacacia), a park or roadside tree of about 50 years: 20 m high, a leaning, deeply furrowed bole that forks at 4.7 m into a sinuous leader and a steep co-dominant limb, an open, irregular crown of fine pinnate leaves with sky between its clusters, and white flower racemes that hang below the leaves in the flowering pose.
 * Every number is chosen by hand against the references; `assemble.ts` builds the asset from it.
 * @see ./tree-kit.ts
 * @see ./common.ts */
import { REFERENCES as R, limb, lobe, windPoseTexts } from "./common.ts";
import type { Limb, Mass, Species, Text } from "./tree-kit.ts";

const name: Text = { en: "Black Locust", de: "Robinie" };
const L = (n: number): Text => (n === 1 ? { en: "Co-dominant fork limb", de: "Gleichrangiger Zwieselast" } : { en: `Limb ${n}`, de: `Ast ${n}` });
const side = (elA: number, r: number) => [{ seg: 0, at: 0.6, dAz: 48, el: elA + 10, len: 1.5, r: Math.max(0.024, 0.24 * r) }, { seg: 1, at: 0.45, dAz: -40, el: 26, len: 1.3, r: Math.max(0.02, 0.18 * r) }];
const B = (n: number, parent: string, at: number, az: number, rc: number, els: [number, number, number], size: [number, number, number], r0: number, spin = 0, raceme = true): Limb => limb(`l${n}`, L(n), parent, at, az, rc, els, size, r0, { spin, side: side(els[0], r0), raceme });

const limbs: Limb[] = [
  B(1, "trunk", 4.3, 195, 3.9, [60, 50, 40], [1.9, 1.7, 2.0], 0.2, 0, false),
  B(2, "trunk", 3.6, 62, 4.4, [30, 16, 24], [1.7, 1.35, 1.9], 0.14, 30),
  B(3, "trunk", 4.0, 298, 4.3, [26, 14, 22], [1.7, 1.35, 1.9], 0.13, -25),
  B(4, "ld1", 1.0, 122, 4.5, [32, 20, 26], [1.6, 1.35, 1.8], 0.12, 15),
  B(5, "ld1", 2.2, 12, 4.6, [28, 14, 24], [1.7, 1.35, 1.9], 0.12, -30),
  B(6, "ld1", 3.4, 248, 4.2, [36, 22, 28], [1.55, 1.35, 1.75], 0.11, 20),
  B(7, "ld2", 0.6, 158, 4.0, [40, 26, 30], [1.55, 1.35, 1.75], 0.1, -15),
  B(8, "ld2", 1.8, 338, 3.9, [38, 24, 30], [1.55, 1.35, 1.75], 0.1, 10),
  B(9, "ld2", 3.0, 84, 3.4, [44, 30, 34], [1.45, 1.25, 1.65], 0.085, -20),
  B(10, "ld3", 0.6, 218, 2.7, [48, 34, 38], [1.3, 1.2, 1.5], 0.07, 25, false),
  B(11, "ld3", 1.8, 28, 2.3, [50, 38, 40], [1.2, 1.1, 1.4], 0.06, 0, false),
];

const core = (id: string, bone: string, y: number, r: [number, number, number], x = 0, z = 0): Mass => ({ id, bone, c: [x, y, z], radii: r, az: 0, group: "shade" });
const { "wet-snow": _snow, ...wind } = windPoseTexts(name, 0.45, 0);

export const species: Species = {
  id: "black-locust",
  title: name,
  summary: { en: "A black locust of about 50 years, 20 m high: a leaning bole with deeply furrowed, interlacing bark forks at 4.7 m into a sinuous leader and a steep co-dominant limb, and the fine pinnate leaves form an open, irregular crown about 13 m wide with sky between its clusters; in the flowering pose white flower racemes hang below the leaves.", de: "Eine Robinie von etwa 50 Jahren, 20 m hoch: Ein geneigter Stamm mit tief gefurchter, netzartig verflochtener Borke gabelt sich in 4,7 m in einen gewundenen Leittrieb und einen steilen gleichrangigen Ast, und die feinen gefiederten Blätter bilden eine offene, unregelmäßige Krone von etwa 13 m Breite mit Himmel zwischen den Büscheln; in der Blütenpose hängen weiße Blütentrauben unter den Blättern." },
  tags: ["tree", "deciduous", "broadleaf", "open-crown", "flowering", "pioneer", "wind", "transpiration"],
  latin: "Robinia pseudoacacia",
  references: [R.woodHandbook, R.en199114, R.vogel, R.jones, R.monteith, R.silvics, R.dirr, R.roloff],
  notes: { en: "Rig and model are authored by hand for a black locust of about 50 years: 20 m high (Silvics of North America gives 12 to 30 m and stem diameters of 0.3 to 0.6 m; the bark is deeply furrowed into rough, interlacing ridges, the crown open and irregular, the trunk often leaning or forked), stem diameter 0.58 m at breast height, crown 13 m wide. Flowers are fragrant white racemes 10 to 20 cm long in late May and June, the pods 5 to 10 cm long. Wood values are those of green black locust (Wood Handbook, Table 5-3a: specific gravity 0.66, E 12.8 GPa, modulus of rupture 95 MPa; green weight about 930 kg/m3). The leaf area index, the conductances, the foliage and flower masses are authored estimates.", de: "Rig und Modell sind von Hand für eine Robinie von etwa 50 Jahren entworfen: 20 m hoch (Silvics of North America nennt 12 bis 30 m und Stammdurchmesser von 0,3 bis 0,6 m; die Borke ist tief in raue, netzartig verflochtene Rippen gefurcht, die Krone offen und unregelmäßig, der Stamm oft geneigt oder zwieselig), Brusthöhendurchmesser 0,58 m, Krone 13 m breit. Die Blüten sind duftende weiße Trauben von 10 bis 20 cm Länge Ende Mai und im Juni, die Hülsen 5 bis 10 cm lang. Die Holzwerte sind die von frischem Robinienholz (Wood Handbook, Tabelle 5-3a: Rohdichtezahl 0,66, E 12,8 GPa, Biegefestigkeit 95 MPa; Frischgewicht etwa 930 kg/m3). Blattflächenindex, Leitfähigkeiten, Laub- und Blütenmassen sind Autorenschätzungen." },
  distinction: { en: "A 20 m black locust (Robinia pseudoacacia) with a leaning, deeply furrowed bole forked at 4.7 m, an open irregular crown of fine pinnate leaves about 13 m wide, and white flower racemes hanging below the leaves in the flowering pose.", de: "Eine 20 m hohe Robinie (Robinia pseudoacacia) mit geneigtem, tief gefurchtem Stamm, in 4,7 m gegabelt, einer offenen, unregelmäßigen Krone aus feinen gefiederten Blättern von etwa 13 m Breite und weißen Blütentrauben, die in der Blütenpose unter den Blättern hängen." },
  materials: [
    { id: "locust-bark", title: { en: "Black locust bark, deeply furrowed into rough interlacing ridges", de: "Robinienborke, tief in raue, netzartig verflochtene Rippen gefurcht" }, emissivity: 0.95, solarAbsorptivity: 0.75, densityKgM3: 620, conductivityWmK: 0.12, source: "authored-estimate", reference: "wood-handbook-2021" },
    { id: "locust-bark-young", title: { en: "Grey-brown bark of the limbs, finely fissured", de: "Graubraune, fein rissige Rinde der Äste" }, emissivity: 0.95, solarAbsorptivity: 0.7, densityKgM3: 660, conductivityWmK: 0.13, source: "authored-estimate", reference: "wood-handbook-2021" },
    { id: "locust-leaf-sun", title: { en: "Black locust sun leaflets", de: "Robinien-Sonnenfiederblättchen" }, emissivity: 0.97, solarAbsorptivity: 0.48, source: "literature", reference: "jones-2014" },
    { id: "locust-leaf-shade", title: { en: "Black locust shade leaflets (effective)", de: "Robinien-Schattenfiederblättchen (wirksam)" }, emissivity: 0.97, solarAbsorptivity: 0.24, source: "authored-estimate", reference: "jones-2014" },
    { id: "locust-flower", title: { en: "White flower racemes and young pods", de: "Weiße Blütentrauben und junge Hülsen" }, emissivity: 0.95, solarAbsorptivity: 0.35, source: "authored-estimate", reference: "jones-2014" },
    { id: "locust-wood", title: { en: "Green black locust wood", de: "Frisches Robinienholz" }, emissivity: 0.9, solarAbsorptivity: 0.7, densityKgM3: 930, conductivityWmK: 0.17, elasticModulusPa: 12800000000, strengthPa: 95000000, source: "literature", reference: "wood-handbook-2021" },
  ],
  wood: { material: "locust-wood", density: 930, bark: "locust-bark", barkYoung: "locust-bark-young", leafSun: "locust-leaf-sun", leafShade: "locust-leaf-shade", flower: "locust-flower" },
  root: { len: 0.45, flareR: 0.62, topR: 0.33, roots: [{ az: 10, len: 1.0, r: 0.13 }, { az: 85, len: 0.85, r: 0.11 }, { az: 150, len: 1.1, r: 0.14 }, { az: 225, len: 0.9, r: 0.12 }, { az: 300, len: 1.05, r: 0.13 }] },
  stem: [
    { id: "trunk", title: { en: "Leaning bole", de: "Geneigter Stamm" }, len: 4.55, r0: 0.3, r1: 0.25, leanDeg: 4, limit: 3 },
    { id: "ld1", title: { en: "Leader above the fork", de: "Leittrieb über dem Zwiesel" }, len: 4.5, r0: 0.21, r1: 0.16, leanDeg: -2, limit: 3 },
    { id: "ld2", title: { en: "Leader in the middle crown", de: "Leittrieb in der mittleren Krone" }, len: 4.2, r0: 0.16, r1: 0.1, leanDeg: 3, limit: 4 },
    { id: "ld3", title: { en: "Leader in the upper crown", de: "Leittrieb in der oberen Krone" }, len: 3.8, r0: 0.1, r1: 0.055, leanDeg: -1.5, limit: 6 },
  ],
  top: { len: 1.8, r: 0.03, size: [1.3, 1.15, 1.3] },
  limbs,
  templates: {
    50: [lobe(0, 0, 0, 0.82), lobe(0.7, 0.25, 0.3, 0.5), lobe(0.25, 0.68, -0.4, 0.46), lobe(0.55, -0.25, -0.55, 0.46, 0.4, 0.46), lobe(-0.2, 0.55, 0.45, 0.42), lobe(0.85, -0.15, -0.05, 0.38)],
    100: [lobe(0, 0, 0, 0.84), lobe(0.7, 0.25, 0.3, 0.52), lobe(0.25, 0.68, -0.4, 0.48), lobe(0.55, -0.25, -0.55, 0.48, 0.42, 0.48), lobe(-0.2, 0.55, 0.45, 0.44), lobe(0.85, -0.15, -0.05, 0.4)],
    200: [lobe(0.0, 0.0, 0.0, 0.874, 0.874, 0.874), lobe(0.7, 0.25, 0.3, 0.541, 0.541, 0.541), lobe(0.25, 0.68, -0.4, 0.499, 0.499, 0.499), lobe(0.55, -0.25, -0.55, 0.499, 0.437, 0.499), lobe(-0.2, 0.55, 0.45, 0.458, 0.458, 0.458), lobe(0.85, -0.15, -0.05, 0.416, 0.416, 0.416)],
    500: [lobe(0.0, 0.0, 0.0, 0.909, 0.909, 0.909), lobe(0.7, 0.25, 0.3, 0.563, 0.563, 0.563), lobe(0.25, 0.68, -0.4, 0.519, 0.519, 0.519), lobe(0.55, -0.25, -0.55, 0.519, 0.454, 0.519), lobe(-0.2, 0.55, 0.45, 0.476, 0.476, 0.476), lobe(0.85, -0.15, -0.05, 0.433, 0.433, 0.433)],
  },
  coarse: { 50: 1, 100: 1, 200: 1, 500: 1 },
  shade: {
    50: [core("shade-1", "ld1", 7.4, [2.0, 1.6, 2.0], 0.3), core("shade-2", "ld1", 9.2, [2.2, 1.8, 2.2], 0.2), core("shade-3", "ld2", 12.4, [1.9, 1.8, 1.9], 0.3), core("shade-4", "ld3", 15.6, [1.3, 1.4, 1.3], 0.35), core("shade-5", "trunk", 8.6, [1.3, 1.6, 1.3], -1.4, -0.4)],
    100: [core("shade-1", "ld1", 7.4, [2.04, 1.63, 2.04], 0.3), core("shade-2", "ld1", 9.2, [2.24, 1.84, 2.24], 0.2), core("shade-3", "ld2", 12.4, [1.94, 1.84, 1.94], 0.3), core("shade-4", "ld3", 15.6, [1.33, 1.43, 1.33], 0.35), core("shade-5", "trunk", 8.6, [1.33, 1.63, 1.33], -1.4, -0.4)],
    200: [core("shade-1", "ld1", 7.4, [2.1, 1.68, 2.1], 0.3), core("shade-2", "ld1", 9.2, [2.31, 1.89, 2.31], 0.2), core("shade-3", "ld2", 12.4, [1.99, 1.89, 1.99], 0.3), core("shade-4", "ld3", 15.6, [1.37, 1.47, 1.37], 0.35)],
    500: [core("shade-1", "ld1", 7.4, [2.2, 1.76, 2.2], 0.3), core("shade-2", "ld1", 9.2, [2.42, 1.98, 2.42], 0.2), core("shade-3", "ld2", 12.4, [2.09, 1.98, 2.09], 0.3)],
  },
  crown500: [],

  ridges: { count: 5, depth: 0.04, width: 0.05, scales: [50, 100] },
  leaf: { sunGs: 0.0055, shadeGs: 0.0035, size: 0.03, kgPerM2: 0.16, lai: 3.2, sunShare: 0.6, porosity: 0.45, snowKgM2: 0 },
  heights: { crownBase: 4 },
  sway: { periodS: 3.8, windyLean: 2, stormLean: 5 },
  poseTexts: {
    ...wind,
    flowering: { title: { en: "In flower", de: "In Blüte" }, description: { en: "Late May on a calm day: the white flower racemes, 15 to 25 cm long, hang below the young leaves of every flowering limb system; their weight of about 1 kg per cluster bends the limbs slightly. Light air of 1.5 m/s at 10 m; the racemes are their own thermal region.", de: "Ende Mai an einem ruhigen Tag: Die weißen Blütentrauben von 15 bis 25 cm Länge hängen unter dem jungen Laub jedes blühenden Astsystems; ihr Gewicht von etwa 1 kg je Büschel biegt die Äste leicht. Leiser Zug von 1,5 m/s in 10 m Höhe; die Trauben sind eine eigene thermische Region." } },
  },
  racemes: { len: 0.24, r: 0.032, kgPerCluster: 0.9, areaPerCluster: 3 },
  scaleTexts: {
    50: { silhouette: { en: "flared root collar with five root spurs, the leaning bole with its interlacing bark ridges, the sinuous leader in three segments, the steep co-dominant fork limb and ten limbs with side branches and end shoots, the leaves of every limb system as a loose cluster of six masses with sky between them, and the flower racemes folded into eight of the clusters.", de: "Wurzelhals mit fünf Wurzelanläufen, der geneigte Stamm mit seinen verflochtenen Borkenrippen, der gewundene Leittrieb in drei Abschnitten, der steile gleichrangige Zwieselast und zehn Äste mit Seitenzweigen und Endtrieben, das Laub jedes Astsystems als ein lockeres Büschel aus sechs Massen mit Himmel dazwischen und die in acht der Büschel hochgeklappten Blütentrauben." }, body: { en: "the black locust as it grows: a root collar of 1.2 m with five root spurs carries a bole of 0.58 m at breast height that leans 4 degrees and forks at 4.7 m; the sinuous leader runs on to 20 m beside a steep co-dominant limb, ten more limbs leave it irregularly, every limb ends in a loose cluster of six foliage masses, and eight clusters carry five racemes each on their own bone.", de: "Die Robinie, wie sie wächst: Ein Wurzelhals von 1,2 m mit fünf Wurzelanläufen trägt einen Stamm von 0,58 m in Brusthöhe, der 4 Grad geneigt ist und sich in 4,7 m gabelt; der gewundene Leittrieb läuft neben einem steilen gleichrangigen Ast bis 20 m weiter, zehn weitere Äste verlassen ihn unregelmäßig, jeder Ast endet in einem lockeren Büschel aus sechs Laubmassen, und acht Büschel tragen je fünf Trauben an einem eigenen Knochen." } },
    100: { silhouette: { en: "root collar with three spurs, bole with its bark ridges and leader, the fork limb and ten limbs with their stronger side branches, each limb system as one lobed outline of four masses, and three racemes per flowering cluster.", de: "Wurzelhals mit drei Anläufen, Stamm mit Borkenrippen und Leittrieb, der Zwieselast und zehn Äste mit ihren stärkeren Seitenzweigen, jedes Astsystem als ein gelappter Umriss aus vier Massen und drei Trauben je blühendem Büschel." }, body: { en: "the same stem and eleven limbs; the thinnest side branches and two of the five root spurs are below what this scale models, each limb cluster is four larger masses and each flowering cluster three racemes.", de: "Derselbe Stamm und elf Äste; die dünnsten Seitenzweige und zwei der fünf Wurzelanläufe liegen unter dem, was dieser Maßstab modelliert, jedes Astbüschel besteht aus vier größeren Massen und jedes blühende Büschel aus drei Trauben." } },
    200: { silhouette: { en: "the leaning bole and leader, the eleven limbs where they show between the clusters, each limb system as one outline of two masses, and one bunch of racemes per flowering cluster.", de: "der geneigte Stamm und Leittrieb, die elf Äste, wo sie zwischen den Büscheln sichtbar werden, jedes Astsystem als ein Umriss aus zwei Massen und ein Traubenbündel je blühendem Büschel." }, body: { en: "stem, flare and the eleven limbs without side branches and end shoots; every limb cluster is one main mass with one shoulder around two shade cores, and the racemes of a cluster are one bunch.", de: "Stamm, Anlauf und die elf Äste ohne Seitenzweige und Endtriebe; jedes Astbüschel ist eine Hauptmasse mit einer Schulter um zwei Schattenkerne, und die Trauben eines Büschels sind ein Bündel." } },
    500: { silhouette: { en: "the leaning bole on its flare, the open crown as fifteen masses in three loose tiers and a top, merged into one outline per stem segment, and the raceme bunches.", de: "der geneigte Stamm auf seinem Anlauf, die offene Krone als fünfzehn Massen in drei lockeren Etagen und eine Spitze, je Stammabschnitt zu einem Umriss verschmolzen, und die Traubenbündel." }, body: { en: "stem and flare, the inner and outer limb parts with the fork limb, and the open crown as a ring of six masses, a ring of five, three upper masses and the top around one shade core; one bunch of racemes per flowering cluster.", de: "Stamm und Anlauf, die inneren und äußeren Astteile mit dem Zwieselast und die offene Krone als ein Ring aus sechs Massen, ein Ring aus fünf, drei obere Massen und die Spitze um einen Schattenkern; ein Traubenbündel je blühendem Büschel." } },
  },
};
