/** 🪭️ Ginkgo biloba, a mature male street tree of about 50 years: straight stem limbed up to 3 m, one continuous leader to about 14 m and nine irregularly spaced, ascending scaffold limbs that carry clumpy foliage on spur shoots, so the young conical crown has turned irregular with sky gaps.
 *
 * Every number below is hand-chosen for this tree (sources in `provenance.references`); `tree-asset.ts` only expands them.
 * Run: `repo tools exec bun -- ginkgo.ts` from this folder.
 * @see ./tree-asset.ts
 * @see https://doi.org/10.2737/FPL-GTR-282 — Wood Handbook */
import { T, type Species, type V3 } from "./tree-lib.ts";
import { along, branches, clusters, limb, stem, type AlongRow, type BranchRow, type ClusterRow, type LimbRow } from "./tree-design.ts";
import { writeAll } from "./tree-asset.ts";

const ROOT_L = 0.45;
const stemBones = stem(
  [
    [3.2, 0.02, -0.01, 0.205],
    [6.0, 0.1, -0.06, 0.16],
    [9.0, 0.06, 0.07, 0.115],
    [12.0, -0.08, 0.05, 0.07],
    [14.1, -0.04, -0.02, 0.035],
  ],
  ROOT_L,
  T("Bole", "Schaft"),
  (i) => T(`Leader, part ${i}`, `Stammverlängerung, Teil ${i}`),
);

const L = (id: string, on: string, t: number, az: number, r0: number, segs: LimbRow["segs"], en: string, dt: string): LimbRow => ({ id, on, t, az, r0, segs, title: T(en, dt) });
const limbRows: LimbRow[] = [
  L("limb-1", "leader-1", 0.03, 18, 0.085, [[27, 2.39, 0.06], [36, 1.77, 0.03, 12]], "Lowest scaffold limb, ahead", "Unterster Gerüstast, vorn"),
  L("limb-2", "leader-1", 0.2, 152, 0.08, [[30, 2.28, 0.055], [40, 1.6, 0.028, -10]], "Low scaffold limb, behind left", "Unterer Gerüstast, hinten links"),
  L("limb-3", "leader-1", 0.48, 258, 0.075, [[33, 2.17, 0.052], [42, 1.54, 0.027, 14]], "Low scaffold limb, right", "Unterer Gerüstast, rechts"),
  L("limb-4", "leader-1", 0.78, 76, 0.07, [[37, 2.11, 0.048], [46, 1.42, 0.025, -12]], "Scaffold limb, left", "Gerüstast, links"),
  L("limb-5", "leader-2", 0.15, 205, 0.06, [[40, 1.84, 0.042], [50, 1.13, 0.022, 10]], "Middle scaffold limb, behind right", "Mittlerer Gerüstast, hinten rechts"),
  L("limb-6", "leader-2", 0.45, 318, 0.055, [[42, 1.67, 0.038], [52, 1.03, 0.02, -8]], "Middle scaffold limb, ahead right", "Mittlerer Gerüstast, vorn rechts"),
  L("limb-7", "leader-2", 0.78, 112, 0.05, [[46, 1.46, 0.033], [55, 0.86, 0.018, 9]], "Upper scaffold limb, left behind", "Oberer Gerüstast, links hinten"),
  L("limb-8", "leader-3", 0.22, 22, 0.04, [[50, 1.25, 0.025], [58, 0.6, 0.016, -6]], "Upper scaffold limb, ahead", "Oberer Gerüstast, vorn"),
  L("limb-9", "leader-3", 0.62, 236, 0.035, [[54, 1.05, 0.022], [60, 0.5, 0.015, 7]], "Top scaffold limb, behind right", "Oberster Gerüstast, hinten rechts"),
];
const limbs = limbRows.map((row) => limb(row, stemBones));
const wood = [...stemBones, ...limbs.flatMap((l) => l.wood)];
const foliage = limbs.map((l) => l.foliage);
const azOf = (id: string): number => {
  if (id.startsWith("leader")) return 0;
  const w = wood.find((b) => b.id === id)!;
  return (Math.atan2(w.b[2] - w.a[2], w.b[0] - w.a[0]) * 180) / Math.PI;
};

const branchRows: BranchRow[] = [
  ["limb-1", 0.55, 38, 30, 0.95, 0.03, 0.016], ["limb-1", 0.8, -42, 44, 0.85, 0.026, 0.015], ["limb-1-2", 0.5, 40, 50, 0.7, 0.02, 0.012, [50]],
  ["limb-2", 0.5, -40, 38, 0.9, 0.028, 0.016], ["limb-2", 0.82, 36, 48, 0.8, 0.025, 0.015], ["limb-2-2", 0.45, -38, 52, 0.65, 0.019, 0.012, [50]],
  ["limb-3", 0.55, 42, 40, 0.85, 0.027, 0.015], ["limb-3", 0.85, -36, 50, 0.75, 0.024, 0.015],
  ["limb-4", 0.5, -40, 45, 0.8, 0.026, 0.015], ["limb-4", 0.82, 38, 52, 0.7, 0.023, 0.015],
  ["limb-5", 0.55, 40, 48, 0.75, 0.024, 0.015], ["limb-6", 0.55, -40, 50, 0.7, 0.022, 0.015],
  ["limb-7", 0.6, 38, 55, 0.6, 0.02, 0.015], ["leader-2", 0.3, 300, 35, 0.8, 0.026, 0.015], ["leader-3", 0.45, 135, 40, 0.75, 0.022, 0.015],
];

const rows: AlongRow[] = [
  [0.4, 0.8, 0.34, 0.28, "leaf-shade"],
  [0.6, 0.86, 0.3, -0.42, "leaf-shade"],
  [0.8, 0.92, 0.3, 0.38, "leaf-sun"],
  [1.0, 1.0, 0.28, 0, "leaf-sun"],
];
const size: V3[] = [[0.88, 0.66, 0.85], [0.86, 0.65, 0.84], [0.84, 0.64, 0.84], [0.82, 0.63, 0.8], [0.78, 0.6, 0.76], [0.74, 0.58, 0.72], [0.7, 0.57, 0.68], [0.66, 0.56, 0.64], [0.62, 0.55, 0.6]];
const clusterRows: ClusterRow[] = [
  ...limbs.flatMap((l, i) => along(l, i < 7 ? rows : rows.slice(1), size[i]!, `m-limb-${i + 1}`, 10 * (i + 1))),
  ["cl-core-1", "leader-1", [0.12, 5.0, 0.18], [0.78, 0.7, 0.74], "leaf-shade", "m-core", 101],
  ["cl-core-2", "leader-2", [-0.1, 6.7, -0.15], [0.8, 0.72, 0.76], "leaf-shade", "m-core", 102],
  ["cl-core-3", "leader-2", [0.14, 8.3, 0.1], [0.76, 0.72, 0.72], "leaf-shade", "m-core", 103],
  ["cl-core-4", "leader-3", [-0.06, 9.9, 0.12], [0.72, 0.72, 0.7], "leaf-shade", "m-core", 104],
  ["cl-leader-1", "leader-3", [0.1, 11.3, -0.08], [0.7, 0.78, 0.66], "leaf-sun", "m-top", 121],
  ["cl-leader-2", "leader-4", [-0.08, 12.7, 0.06], [0.62, 0.8, 0.6], "leaf-sun", "m-top", 122],
  ["cl-leader-top", "leader-4", [-0.04, 14.05, -0.02], [0.48, 0.85, 0.48], "leaf-sun", "m-top", 123],
];

const ginkgo: Species = {
  id: "ginkgo",
  title: T("Ginkgo, Street Tree", "Ginkgo, Straßenbaum"),
  summary: T(
    "A mature male ginkgo (Ginkgo biloba) planted as a street tree: a straight grey stem limbed up to 3 m, one continuous leader and nine irregularly spaced, ascending scaffold limbs whose clumpy fan-leaf foliage on spur shoots gives the young cone an irregular, gappy outline.",
    "Ein ausgewachsener männlicher Ginkgo (Ginkgo biloba) als Straßenbaum: ein gerader grauer Stamm, bis 3 m aufgeastet, eine durchgehende Stammverlängerung und neun unregelmäßig verteilte, aufsteigende Gerüstäste, deren büschelige Fächerblätter an Kurztrieben dem jungen Kegel einen unregelmäßigen, lückigen Umriss geben.",
  ),
  distinction: T(
    "A 15 m male street ginkgo (Ginkgo biloba) with one continuous leader, irregular ascending tiers of scaffold limbs and clumpy fan-leaf foliage on spur shoots: an open, irregularly conical crown about 8 m wide, golden yellow in autumn.",
    "Ein 15 m hoher männlicher Straßen-Ginkgo (Ginkgo biloba) mit durchgehender Stammverlängerung, unregelmäßigen aufsteigenden Etagen von Gerüstästen und büscheligem Fächerlaub an Kurztrieben: eine offene, unregelmäßig kegelförmige Krone von etwa 8 m Breite, im Herbst goldgelb.",
  ),
  tags: ["tree", "deciduous", "gymnosperm", "street-tree", "conical-crown", "urban", "wind", "transpiration"],
  provenance: {
    author: "Semio staffage plant worker PW-04",
    method: "handcrafted",
    created: "2026-11-22",
    references: [
      { id: "dirr-2009", citation: "Dirr, M. A. (2009). Manual of Woody Landscape Plants, 6th ed. Stipes Publishing, Champaign IL. Ginkgo biloba: habit pyramidal when young, irregular and wide-spreading with age, 50 to 80 ft; fan-shaped leaves 2 to 3 in wide, clear yellow in autumn; male clones for street use." },
      { id: "galk-strassenbaumliste", citation: "Deutsche Gartenamtsleiterkonferenz (GALK), Arbeitskreis Stadtbäume: GALK-Straßenbaumliste (online, current edition). Ginkgo biloba (male selections) listed as suitable street tree, conical to irregular crown." },
      { id: "fll-ztv-baumpflege-2017", citation: "FLL (2017). ZTV-Baumpflege: Zusätzliche Technische Vertragsbedingungen und Richtlinien für Baumpflege, 6th ed. Forschungsgesellschaft Landschaftsentwicklung Landschaftsbau, Bonn. Clearance profile above footways and roads, used for the limbed-up stem of 3 m." },
      { id: "wagenfuehr-2007", citation: "Wagenführ, R. (2007). Holzatlas, 6th ed. Fachbuchverlag Leipzig im Carl Hanser Verlag. Ginkgo wood: light, soft, density about 0.45 to 0.55 g/cm3 air-dry; used as a guide for the green density." },
      { id: "wood-handbook-2021", citation: "Forest Products Laboratory (2021). Wood Handbook: Wood as an Engineering Material. General Technical Report FPL-GTR-282, USDA Forest Service. Chapter 5 (mechanical properties of green wood); used as a guide, ginkgo values are authored estimates." },
      { id: "en-1991-1-4", citation: "EN 1991-1-4:2005 Eurocode 1, Actions on structures, Part 1-4: Wind actions. Air density 1.25 kg/m3, drag force F = 0.5 rho cd A v^2, peak velocity pressure." },
      { id: "vogel-1989", citation: "Vogel, S. (1989). Drag and reconfiguration of broad leaves in high winds. Journal of Experimental Botany 40(8), 941-948." },
      { id: "moore-maguire-2004", citation: "Moore, J. R. and Maguire, D. A. (2004). Natural sway frequencies and damping ratios of trees: concepts, review and synthesis of previous studies. Trees 18, 195-203." },
      { id: "jones-2014", citation: "Jones, H. G. (2014). Plants and Microclimate, 3rd ed. Cambridge University Press. Leaf energy balance, leaf area index, stomatal and boundary-layer conductance." },
      { id: "monteith-unsworth-2013", citation: "Monteith, J. L. and Unsworth, M. H. (2013). Principles of Environmental Physics, 4th ed. Academic Press. Penman-Monteith leaf transpiration." },
      { id: "pretzsch-2015", citation: "Pretzsch, H., Biber, P., Uhl, E. et al. (2015). Crown size and growing space requirement of common tree species in urban centres, parks, and forests. Urban Forestry and Urban Greening 14(3), 466-479." },
    ],
    notes: T(
      "Rig, stations and crown clusters are chosen by hand for a male street ginkgo of about 50 years (Dirr 2009 habit, GALK street-tree list): stem limbed up to 3 m (FLL clearance), leader to 14 m, nine ascending scaffold limbs on irregular tiers. Wood density, masses, conductances and areas are authored estimates.",
      "Rig, Stationen und Kronenballen sind von Hand für einen männlichen Straßen-Ginkgo von etwa 50 Jahren gewählt (Wuchsform nach Dirr 2009, GALK-Straßenbaumliste): Stamm bis 3 m aufgeastet (FLL-Lichtraum), Stammverlängerung bis 14 m, neun aufsteigende Gerüstäste in unregelmäßigen Etagen. Holzdichte, Massen, Leitfähigkeiten und Flächen sind Autorenschätzungen.",
    ),
  },
  materials: [
    { id: "ginkgo-bark", title: T("Ginkgo bark, grey-brown, deeply furrowed", "Ginkgorinde, graubraun, tief gefurcht"), emissivity: 0.95, solarAbsorptivity: 0.72, densityKgM3: 600, conductivityWmK: 0.12, source: "authored-estimate", reference: "wood-handbook-2021" },
    { id: "ginkgo-bark-young", title: T("Smoother grey bark of the limbs", "Glattere graue Rinde der Äste"), emissivity: 0.95, solarAbsorptivity: 0.66, densityKgM3: 650, conductivityWmK: 0.13, source: "authored-estimate", reference: "wood-handbook-2021" },
    { id: "ginkgo-leaf-sun", title: T("Ginkgo sun leaves (fan-shaped)", "Ginkgo-Sonnenblätter (fächerförmig)"), emissivity: 0.97, solarAbsorptivity: 0.5, source: "literature", reference: "jones-2014" },
    { id: "ginkgo-leaf-shade", title: T("Ginkgo shade leaves (effective)", "Ginkgo-Schattenblätter (wirksam)"), emissivity: 0.97, solarAbsorptivity: 0.28, source: "authored-estimate", reference: "jones-2014" },
    { id: "ginkgo-wood", title: T("Green ginkgo wood", "Frisches Ginkgoholz"), emissivity: 0.9, solarAbsorptivity: 0.6, densityKgM3: 850, conductivityWmK: 0.14, elasticModulusPa: 6000000000, strengthPa: 35000000, source: "authored-estimate", reference: "wagenfuehr-2007" },
  ],
  regions: {
    stem: { id: "stem-bark", title: T("Furrowed grey bark of stem and leader", "Gefurchte graue Rinde von Stamm und Stammverlängerung"), kind: "bark", model: "passive", material: "ginkgo-bark", areaM2: 13.6, characteristicLengthM: 0.3, geometry: "cylinder", skyViewFactor: 0.4, layers: [], heatCapacityJM2K: 60000, source: "authored-estimate", reference: "wood-handbook-2021" },
    limb: { id: "limb-bark", title: T("Bark of the scaffold limbs and side branches", "Rinde der Gerüstäste und Seitenzweige"), kind: "bark", model: "passive", material: "ginkgo-bark-young", areaM2: 9.4, characteristicLengthM: 0.08, geometry: "cylinder", skyViewFactor: 0.5, layers: [], heatCapacityJM2K: 30000, source: "authored-estimate", reference: "wood-handbook-2021" },
    sun: { id: "leaf-sun", title: T("Sun leaves of the outer clusters", "Sonnenblätter der äußeren Ballen"), kind: "foliage", model: "leaf", material: "ginkgo-leaf-sun", areaM2: 68, areaBasis: "exchange", characteristicLengthM: 0.06, geometry: "vertical-plate", skyViewFactor: 0.7, layers: [], heatCapacityJM2K: 950, leaf: { stomatalConductanceMaxMS: 0.003, leafAreaIndex: 1.45, photosynthesis: "c3" }, source: "authored-estimate", reference: "jones-2014" },
    shade: { id: "leaf-shade", title: T("Shade leaves of the inner clusters", "Schattenblätter der inneren Ballen"), kind: "foliage", model: "leaf", material: "ginkgo-leaf-shade", areaM2: 74, areaBasis: "exchange", characteristicLengthM: 0.065, geometry: "vertical-plate", skyViewFactor: 0.3, layers: [], heatCapacityJM2K: 750, leaf: { stomatalConductanceMaxMS: 0.0018, leafAreaIndex: 1.6, photosynthesis: "c3" }, source: "authored-estimate", reference: "jones-2014" },
  },
  assumptions: [
    T("Leaf regions use one-sided leaf area (142 m2, a leaf area index of about 3 over a crown projection of about 47 m2); the maximum stomatal conductance is an authored estimate for ginkgo, which keeps a low conductance among temperate trees, guided by Jones (2014). Photosynthesis type: C3. Aperture and night closure are the engine's business, not authored here.", "Die Blattregionen verwenden die einseitige Blattfläche (142 m2, ein Blattflächenindex von etwa 3 über einer Kronenprojektion von etwa 47 m2); die maximale stomatäre Leitfähigkeit ist eine Autorenschätzung für den Ginkgo, der unter den Bäumen gemäßigter Zonen eine geringe Leitfähigkeit hat, angelehnt an Jones (2014). Photosynthesetyp: C3. Öffnungsgrad und Nachtschluss regelt die Engine, sie werden hier nicht vorgegeben."),
    T("Bark of stem, leader and limbs is passive; its areal heat capacity is the volumetric heat capacity of green wood (about 2.7 MJ per m3 and K) times an effective depth of 1 to 2.5 cm, an authored estimate. No surface temperature is authored.", "Die Rinde von Stamm, Stammverlängerung und Ästen ist passiv; ihre flächenbezogene Wärmekapazität ist die volumetrische Wärmekapazität frischen Holzes (etwa 2,7 MJ je m3 und K) mal einer wirksamen Tiefe von 1 bis 2,5 cm, eine Autorenschätzung. Es wird keine Oberflächentemperatur vorgegeben."),
    T("Transpiration emitters release the latent flux of their foliage region as vapour, spread over at least five anchors across the crown; sun and shade foliage are separate regions, and each pose authors their sunlit share. Leaf colour and the bare winter state are not geometric: a pose cannot hide shapes, so the autumn pose changes the thermal binding and the loads only.", "Die Transpirationsemitter geben den latenten Fluss ihrer Laubregion als Wasserdampf ab, verteilt auf mindestens fünf Anker über die Krone; Sonnen- und Schattenlaub sind getrennte Regionen, jede Pose gibt ihren besonnten Anteil an. Laubfarbe und der kahle Winterzustand sind nicht geometrisch: eine Pose kann keine Formen ausblenden, die Herbstpose ändert daher nur die thermische Bindung und die Lasten."),
  ],
  structureAssumptions: [
    T("The tree is one rigid body per evaluated pose, fixed at the root plate; wind drag acts at the centre of the foliage clusters of every bone and on stem and leader as a line load, gravity on every bone mass. The base moment follows from statics at the posed geometry; root-soil failure and dynamic amplification are not modelled. Pose lean angles are authored estimates scaled with the drag, not a computed deflection.", "Der Baum ist je ausgewerteter Pose ein starrer Körper, am Wurzelteller eingespannt; Winddruck wirkt im Mittelpunkt der Laubballen jedes Knochens und an Stamm und Stammverlängerung als Linienlast, die Schwerkraft auf jede Knochenmasse. Das Fußmoment folgt aus der Statik der gezeigten Geometrie; Wurzel-Boden-Versagen und dynamische Überhöhung werden nicht modelliert. Die Neigungswinkel der Posen sind Autorenschätzungen proportional zum Widerstand, keine berechnete Durchbiegung."),
    T("Sections are solid circles of green ginkgo wood (authored-estimate properties guided by Wagenführ and the Wood Handbook). Limbs that grow sideways are bones of the sagittal rig placed by their lateral offset: their solids follow the true limb axis, their bone the projection of it onto the plane of the wind.", "Querschnitte sind Vollkreise aus frischem Ginkgoholz (Autorenschätzungen angelehnt an Wagenführ und das Wood Handbook). Seitlich wachsende Äste sind Knochen des sagittalen Rigs, durch ihren seitlichen Versatz platziert: ihre Körper folgen der wahren Astachse, ihr Knochen deren Projektion in die Windebene."),
  ],
  environment: { airTempC: 26, meanRadiantTempC: 31, skyTempC: 14, groundTempC: 40, relativeHumidity: 0.45, windMS: 1.5, windDirectionDeg: 0, solarWM2: 750, diffuseFraction: 0.15, pressurePa: 101325 },
  densityKgM3: 850,
  leafAreaM2: { sun: 68, shade: 74 },
  leafFreshKgM2: 0.27,
  rootFlare: { profile: [[0, 0.46], [0.08, 0.38], [0.2, 0.29], [0.34, 0.245], [0.45, 0.225]], lengthM: ROOT_L },
  wood,
  foliage,
  branches: branches(branchRows, wood, azOf),
  clusters: clusters(clusterRows),
  masses500: [
    ...limbRows.map((row, i) => ({ id: `m-limb-${i + 1}`, host: row.id, members: clusterRows.filter((c) => c[5] === `m-limb-${i + 1}`).map((c) => c[0]), shrink: [1, 0.94, 0.94] as V3 })),
    { id: "m-core", host: "leader-2", members: clusterRows.filter((c) => c[5] === "m-core").map((c) => c[0]), shrink: [1, 0.94, 0.94] },
    { id: "m-top", host: "leader-4", members: clusterRows.filter((c) => c[5] === "m-top").map((c) => c[0]), shrink: [1, 0.95, 0.95] },
  ],
  lobes: { 100: { n: 6, core: 0.8, coreSide: 0.93, k: 0.55, s: 0.45, minUp: -0.55 }, 50: { n: 10, core: 0.82, coreSide: 0.94, k: 0.63, s: 0.37, minUp: -0.6 } },
  marks: { kind: "fan", size: 0.14, spacing: 0.34 },
  bark: { kind: "furrow", lines: 4, seed: 7 },
  sway: { periodS: 3.4, breezeMS: 6, galeMS: 22, gustPeak: 2.75, lean: { 0: 0.05, 1: 0.15, 2: 0.45, 3: 0.7, 4: 1.4 } },
  cd: { breeze: 0.32, gale: 0.2, stem: 0.8 },
  poses: [
    { id: "calm", title: T("Calm summer day", "Windstiller Sommertag"), description: T("Light air of 1.5 m/s at 10 m over a sunny street: the crown stands at rest, the outer clusters lit, the inner clusters shaded, transpiration at 80 % of the daytime maximum.", "Leiser Zug von 1,5 m/s in 10 m Höhe über einer sonnigen Straße: die Krone steht still, die äußeren Ballen sind besonnt, die inneren beschattet, die Transpiration liegt bei 80 % des Tagesmaximums."), windMS: 1.5, leanFactor: 0, sun: { "stem-bark": 0.35, "limb-bark": 0.3, "leaf-sun": 0.85, "leaf-shade": 0.12 }, emitters: { sun: 0.8, shade: 0.56 } },
    { id: "windy", title: T("Fresh breeze", "Frische Brise"), description: T("Moderate to fresh breeze of 6 m/s at 10 m (Beaufort 4): limbs and clusters lean downwind; drag F = 0.5 rho cd A v^2 with cd 0.32 on the porous clusters.", "Mäßige bis frische Brise von 6 m/s in 10 m Höhe (Beaufort 4): Äste und Ballen neigen sich in Windrichtung; Widerstand F = 0,5 rho cd A v^2 mit cd 0,32 an den durchlässigen Ballen."), windMS: 6, leanFactor: 1, sun: { "stem-bark": 0.35, "limb-bark": 0.3, "leaf-sun": 0.85, "leaf-shade": 0.12 }, emitters: { sun: 0.95, shade: 0.66 } },
    { id: "storm", title: T("Gale with rain", "Sturm mit Regen"), description: T("Beaufort 9, 22 m/s at 10 m with rain: the crown leans strongly downwind; the fan leaves fold and stack, so the drag coefficient falls to 0.2 (reconfiguration after Vogel 1989); wet bark and leaves, overcast sky.", "Beaufort 9, 22 m/s in 10 m Höhe mit Regen: die Krone neigt sich stark in Windrichtung; die Fächerblätter falten und stapeln sich, der Widerstandsbeiwert sinkt auf 0,2 (Rekonfiguration nach Vogel 1989); nasse Rinde und Blätter, bedeckter Himmel."), windMS: 22, leanFactor: 6.5, sun: { "stem-bark": 0.04, "limb-bark": 0.03, "leaf-sun": 0.1, "leaf-shade": 0.01 }, wet: 0.6, emitters: { sun: 0.45, shade: 0.3 } },
    { id: "autumn-gold", title: T("Golden autumn", "Goldener Herbst"), description: T("A still, sunny late-October day: the fan leaves have turned butter yellow all at once and stomata are mostly closed (transpiration 15 %); ginkgo drops its leaves within days after the first frost. The drawn foliage stays: shapes are not toggled by a pose.", "Ein stiller, sonniger Tag Ende Oktober: die Fächerblätter sind auf einmal buttergelb gefärbt, die Spaltöffnungen weitgehend geschlossen (Transpiration 15 %); der Ginkgo wirft sein Laub binnen Tagen nach dem ersten Frost ab. Das gezeichnete Laub bleibt: eine Pose blendet keine Formen aus."), windMS: 3, leanFactor: 0.3, sun: { "stem-bark": 0.3, "limb-bark": 0.25, "leaf-sun": 0.6, "leaf-shade": 0.08 }, wet: 0.15, emitters: { sun: 0.15, shade: 0.1 } },
  ],
  texts: {
    crown: T("the irregular cone of nine ascending limb systems around the leader, clumpy fan-leaf clusters with sky gaps between the tiers.", "der unregelmäßige Kegel aus neun aufsteigenden Astsystemen um die Stammverlängerung, büschelige Fächerlaubballen mit Himmelslücken zwischen den Etagen."),
    stem: T("the straight stem limbed up to 3 m with its root flare,", "der gerade, bis 3 m aufgeastete Stamm mit Anlauf,"),
    limbs: T("the leader and nine ascending scaffold limbs", "die Stammverlängerung und neun aufsteigende Gerüstäste"),
    leaf: T("fan-shaped leaves of 5 to 8 cm on spur shoots", "fächerförmige Blätter von 5 bis 8 cm an Kurztrieben"),
    bark: T("long wavy furrows", "lange wellige Furchen"),
    omitted500: T("Omitted: the side branches, single clusters and every texture; the crown reads as one irregular cone.", "Weggelassen: Seitenzweige, einzelne Ballen und jede Textur; die Krone liest sich als ein unregelmäßiger Kegel."),
    omitted200: T("Omitted: side branches, lobes of the clusters, leaf and bark texture.", "Weggelassen: Seitenzweige, Lappen der Ballen, Blatt- und Rindentextur."),
  },
};

if (import.meta.main) console.log(JSON.stringify(writeAll(ginkgo), null, 1).slice(0, 4000));

export default ginkgo;
