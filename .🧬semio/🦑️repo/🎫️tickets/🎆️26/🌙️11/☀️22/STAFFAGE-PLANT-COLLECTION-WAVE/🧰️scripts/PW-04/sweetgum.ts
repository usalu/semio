/** ⭐️ Liquidambar styraciflua, the sweetgum of avenues and car parks, about 45 years old: a strong straight central leader to the top and twelve limbs set in a spiral, long and spreading below, short and ascending above, so the dense crown of star-shaped leaves is a tall pyramid with a rounded tip.
 *
 * Every number below is hand-chosen for this tree (sources in `provenance.references`); `tree-asset.ts` only expands them.
 * Run: `repo tools exec bun -- sweetgum.ts` from this folder.
 * @see ./tree-asset.ts
 * @see https://doi.org/10.2737/FPL-GTR-282 — Wood Handbook, sweetgum */
import { T, type Species, type V3 } from "./tree-lib.ts";
import { along, branches, clusters, limb, stem, type AlongRow, type BranchRow, type ClusterRow, type LimbRow } from "./tree-design.ts";
import { writeAll } from "./tree-asset.ts";

const ROOT_L = 0.5;
const stemBones = stem(
  [
    [3.6, 0.02, 0.01, 0.235],
    [7.0, 0.07, -0.04, 0.19],
    [10.5, 0.02, 0.06, 0.145],
    [14.0, -0.06, 0.03, 0.095],
    [16.8, -0.03, -0.02, 0.045],
  ],
  ROOT_L,
  T("Bole", "Schaft"),
  (i) => T(`Central leader, part ${i}`, `Stammverlängerung, Teil ${i}`),
);

const tiers: [on: string, t: number, az: number, len: number, el1: number, el2: number, r0: number][] = [
  ["leader-1", 0.09, 10, 4.75, 20, 31, 0.11],
  ["leader-1", 0.29, 147, 4.65, 22, 32, 0.105],
  ["leader-1", 0.53, 285, 4.45, 23, 33, 0.1],
  ["leader-1", 0.79, 62, 4.2, 25, 35, 0.095],
  ["leader-2", 0.09, 200, 3.85, 30, 40, 0.085],
  ["leader-2", 0.34, 337, 3.5, 32, 42, 0.08],
  ["leader-2", 0.63, 115, 3.15, 35, 45, 0.072],
  ["leader-2", 0.89, 252, 2.8, 38, 48, 0.065],
  ["leader-3", 0.14, 30, 2.3, 44, 54, 0.055],
  ["leader-3", 0.43, 167, 1.9, 47, 57, 0.045],
  ["leader-3", 0.71, 305, 1.5, 50, 60, 0.038],
  ["leader-4", 0.21, 82, 1.1, 55, 63, 0.03],
];
const side = (az: number): Text => {
  const a = ((az % 360) + 360) % 360;
  return a < 45 || a >= 315 ? T("ahead", "vorn") : a < 135 ? T("left", "links") : a < 225 ? T("behind", "hinten") : T("right", "rechts");
};
type Text = { en: string; de: string };
const limbRows: LimbRow[] = tiers.map(([on, t, az, L, e1, e2, r0], i) => ({ id: `limb-${i + 1}`, on, t, az, r0, segs: [[e1, L * 0.58, r0 * 0.6], [e2, L * 0.42, r0 * 0.3, i % 2 === 0 ? 8 : -8]], title: T(`Limb ${i + 1} of the spiral, ${side(az).en}`, `Ast ${i + 1} der Spirale, ${side(az).de}`) }));
const limbs = limbRows.map((row) => limb(row, stemBones));
const wood = [...stemBones, ...limbs.flatMap((l) => l.wood)];
const foliage = limbs.map((l) => l.foliage);
const azOf = (id: string): number => {
  if (id.startsWith("leader")) return 0;
  const w = wood.find((b) => b.id === id)!;
  return (Math.atan2(w.b[2] - w.a[2], w.b[0] - w.a[0]) * 180) / Math.PI;
};

const branchRows: BranchRow[] = limbRows.slice(0, 9).flatMap((row, i): BranchRow[] => [
  [row.id, 0.55, i % 2 === 0 ? 42 : -42, 34, 1.0 - i * 0.05, 0.03, 0.016],
  [row.id, 0.85, i % 2 === 0 ? -40 : 40, 44, 0.9 - i * 0.05, 0.026, 0.015],
  ...(i < 4 ? ([[`${row.id}-2`, 0.5, i % 2 === 0 ? 38 : -38, 50, 0.7, 0.02, 0.012, [50]]] as BranchRow[]) : []),
]);

const rows: AlongRow[] = [
  [0.35, 0.82, 0.36, 0.3, "leaf-shade"],
  [0.55, 0.88, 0.32, -0.42, "leaf-shade"],
  [0.76, 0.93, 0.3, 0.36, "leaf-sun"],
  [1.0, 1.0, 0.28, 0, "leaf-sun"],
];
const size = (i: number): V3 => [0.98 - i * 0.03, 0.74 - i * 0.012, 0.95 - i * 0.03];
const clusterRows: ClusterRow[] = [
  ...limbs.flatMap((l, i) => along(l, i < 9 ? rows : rows.slice(1), size(i), `m-limb-${i + 1}`, 10 * (i + 1))),
  ["cl-core-1", "leader-1", [0.1, 5.0, -0.12], [0.85, 0.8, 0.82], "leaf-shade", "m-core", 201],
  ["cl-core-2", "leader-2", [-0.12, 6.9, 0.1], [0.85, 0.8, 0.82], "leaf-shade", "m-core", 202],
  ["cl-core-3", "leader-2", [0.1, 8.8, 0.12], [0.82, 0.8, 0.8], "leaf-shade", "m-core", 203],
  ["cl-core-4", "leader-3", [-0.08, 10.7, -0.1], [0.8, 0.8, 0.78], "leaf-shade", "m-core", 204],
  ["cl-core-5", "leader-3", [0.06, 12.5, 0.08], [0.76, 0.8, 0.74], "leaf-shade", "m-top", 205],
  ["cl-leader-1", "leader-4", [-0.08, 14.1, 0.04], [0.74, 0.85, 0.72], "leaf-sun", "m-top", 221],
  ["cl-leader-2", "leader-4", [-0.05, 15.5, -0.04], [0.62, 0.85, 0.6], "leaf-sun", "m-top", 222],
  ["cl-leader-top", "leader-4", [-0.03, 16.95, -0.02], [0.48, 1.05, 0.48], "leaf-sun", "m-top", 223],
];

const sweetgum: Species = {
  id: "sweetgum",
  title: T("Sweetgum", "Amberbaum"),
  summary: T(
    "A sweetgum (Liquidambar styraciflua) as planted along avenues: a strong straight leader to the top and twelve limbs set in a spiral, long and spreading below, short and ascending above, so the dense crown of star-shaped, five- to seven-lobed leaves forms a tall pyramid with a rounded tip that turns yellow, orange, red and purple in autumn.",
    "Ein Amberbaum (Liquidambar styraciflua), wie er an Alleen gepflanzt wird: eine kräftige gerade Stammverlängerung bis zum Wipfel und zwölf spiralig gestellte Äste, unten lang und ausladend, oben kurz und aufsteigend, sodass die dichte Krone aus sternförmigen, fünf- bis siebenlappigen Blättern eine hohe Pyramide mit gerundeter Spitze bildet, die sich im Herbst gelb, orange, rot und purpurn färbt.",
  ),
  distinction: T(
    "An 18 m sweetgum (Liquidambar styraciflua) with a dominant central leader and twelve spiral limbs: a dense, tall pyramidal crown about 9.5 m wide of star-shaped leaves, the most colourful autumn tree of the avenues.",
    "Ein 18 m hoher Amberbaum (Liquidambar styraciflua) mit durchgehender Stammverlängerung und zwölf spiralig gestellten Ästen: eine dichte, hohe Pyramidenkrone von etwa 9,5 m Breite aus sternförmigen Blättern, der farbigste Herbstbaum der Alleen.",
  ),
  tags: ["tree", "deciduous", "broadleaf", "street-tree", "pyramidal-crown", "autumn-colour", "urban", "wind", "transpiration"],
  provenance: {
    author: "Semio staffage plant worker PW-04",
    method: "handcrafted",
    created: "2026-11-22",
    references: [
      { id: "dirr-2009", citation: "Dirr, M. A. (2009). Manual of Woody Landscape Plants, 6th ed. Stipes Publishing, Champaign IL. Liquidambar styraciflua: pyramidal in youth, oval to rounded with age, 60 to 75 ft; star-shaped five- to seven-lobed leaves 4 to 7 in wide; yellow, orange, red and purple autumn colour; corky wings on the twigs." },
      { id: "galk-strassenbaumliste", citation: "Deutsche Gartenamtsleiterkonferenz (GALK), Arbeitskreis Stadtbäume: GALK-Straßenbaumliste (online, current edition). Liquidambar styraciflua listed as street tree with a conical to broadly pyramidal crown." },
      { id: "fll-ztv-baumpflege-2017", citation: "FLL (2017). ZTV-Baumpflege: Zusätzliche Technische Vertragsbedingungen und Richtlinien für Baumpflege, 6th ed. Forschungsgesellschaft Landschaftsentwicklung Landschaftsbau, Bonn. Clearance profile above footways, used for the crown base above 3.5 m." },
      { id: "wood-handbook-2021", citation: "Forest Products Laboratory (2021). Wood Handbook: Wood as an Engineering Material. General Technical Report FPL-GTR-282, USDA Forest Service. Table 5-3a, sweetgum, green: specific gravity 0.46, modulus of elasticity 8.3 GPa, modulus of rupture 49 MPa." },
      { id: "en-1991-1-4", citation: "EN 1991-1-4:2005 Eurocode 1, Actions on structures, Part 1-4: Wind actions. Air density 1.25 kg/m3, drag force F = 0.5 rho cd A v^2, peak velocity pressure." },
      { id: "en-1991-1-3", citation: "EN 1991-1-3:2003 Eurocode 1, Actions on structures, Part 1-3: Snow loads. Used for the order of magnitude of wet-snow accretion on foliage." },
      { id: "vogel-1989", citation: "Vogel, S. (1989). Drag and reconfiguration of broad leaves in high winds. Journal of Experimental Botany 40(8), 941-948." },
      { id: "moore-maguire-2004", citation: "Moore, J. R. and Maguire, D. A. (2004). Natural sway frequencies and damping ratios of trees: concepts, review and synthesis of previous studies. Trees 18, 195-203." },
      { id: "jones-2014", citation: "Jones, H. G. (2014). Plants and Microclimate, 3rd ed. Cambridge University Press. Leaf energy balance, leaf area index, stomatal and boundary-layer conductance." },
      { id: "monteith-unsworth-2013", citation: "Monteith, J. L. and Unsworth, M. H. (2013). Principles of Environmental Physics, 4th ed. Academic Press. Penman-Monteith leaf transpiration." },
    ],
    notes: T(
      "Rig, stations and crown clusters are chosen by hand for a sweetgum of about 45 years (Dirr 2009 habit, GALK street-tree list): crown base at 3.6 m (FLL clearance), central leader to 16.8 m, twelve limbs in a spiral of about 137 degrees. Modulus of elasticity and strength follow the Wood Handbook for sweetgum; green density, masses, conductances and areas are authored estimates.",
      "Rig, Stationen und Kronenballen sind von Hand für einen Amberbaum von etwa 45 Jahren gewählt (Wuchsform nach Dirr 2009, GALK-Straßenbaumliste): Kronenansatz in 3,6 m (FLL-Lichtraum), Stammverlängerung bis 16,8 m, zwölf Äste in einer Spirale von etwa 137 Grad. Elastizitätsmodul und Festigkeit folgen dem Wood Handbook für Sweetgum; Frischdichte, Massen, Leitfähigkeiten und Flächen sind Autorenschätzungen.",
    ),
  },
  materials: [
    { id: "sweetgum-bark", title: T("Sweetgum bark, grey-brown with deep furrows and narrow ridges", "Amberbaumrinde, graubraun mit tiefen Furchen und schmalen Rippen"), emissivity: 0.95, solarAbsorptivity: 0.74, densityKgM3: 600, conductivityWmK: 0.12, source: "authored-estimate", reference: "wood-handbook-2021" },
    { id: "sweetgum-bark-young", title: T("Grey bark of the limbs with corky wings on the twigs", "Graue Rinde der Äste mit Korkleisten an den Zweigen"), emissivity: 0.95, solarAbsorptivity: 0.68, densityKgM3: 550, conductivityWmK: 0.11, source: "authored-estimate", reference: "wood-handbook-2021" },
    { id: "sweetgum-leaf-sun", title: T("Star-shaped sun leaves", "Sternförmige Sonnenblätter"), emissivity: 0.97, solarAbsorptivity: 0.5, source: "literature", reference: "jones-2014" },
    { id: "sweetgum-leaf-shade", title: T("Shade leaves of the dense crown (effective)", "Schattenblätter der dichten Krone (wirksam)"), emissivity: 0.97, solarAbsorptivity: 0.25, source: "authored-estimate", reference: "jones-2014" },
    { id: "sweetgum-wood", title: T("Green sweetgum wood", "Frisches Amberbaumholz"), emissivity: 0.9, solarAbsorptivity: 0.6, densityKgM3: 880, conductivityWmK: 0.15, elasticModulusPa: 8300000000, strengthPa: 49000000, source: "literature", reference: "wood-handbook-2021" },
  ],
  regions: {
    stem: { id: "stem-bark", title: T("Ridged bark of stem and leader", "Gerippte Rinde von Stamm und Stammverlängerung"), kind: "bark", model: "passive", material: "sweetgum-bark", areaM2: 20, characteristicLengthM: 0.35, geometry: "cylinder", skyViewFactor: 0.35, layers: [], heatCapacityJM2K: 65000, source: "authored-estimate", reference: "wood-handbook-2021" },
    limb: { id: "limb-bark", title: T("Bark of the spiral limbs and side branches", "Rinde der Spiraläste und Seitenzweige"), kind: "bark", model: "passive", material: "sweetgum-bark-young", areaM2: 15.3, characteristicLengthM: 0.1, geometry: "cylinder", skyViewFactor: 0.4, layers: [], heatCapacityJM2K: 30000, source: "authored-estimate", reference: "wood-handbook-2021" },
    sun: { id: "leaf-sun", title: T("Star-shaped sun leaves of the outer clusters", "Sternförmige Sonnenblätter der äußeren Ballen"), kind: "foliage", model: "leaf", material: "sweetgum-leaf-sun", areaM2: 120, areaBasis: "exchange", characteristicLengthM: 0.12, geometry: "vertical-plate", skyViewFactor: 0.65, layers: [], heatCapacityJM2K: 900, leaf: { stomatalConductanceMaxMS: 0.006, leafAreaIndex: 1.8, photosynthesis: "c3" }, source: "authored-estimate", reference: "jones-2014" },
    shade: { id: "leaf-shade", title: T("Shade leaves of the dense inner crown", "Schattenblätter der dichten Innenkrone"), kind: "foliage", model: "leaf", material: "sweetgum-leaf-shade", areaM2: 145, areaBasis: "exchange", characteristicLengthM: 0.13, geometry: "vertical-plate", skyViewFactor: 0.25, layers: [], heatCapacityJM2K: 700, leaf: { stomatalConductanceMaxMS: 0.0035, leafAreaIndex: 2.2, photosynthesis: "c3" }, source: "authored-estimate", reference: "jones-2014" },
  },
  assumptions: [
    T("Leaf regions use one-sided leaf area (265 m2, a leaf area index of about 4 over a crown projection of about 66 m2, the densest of the three street trees); star leaves of 10 to 18 cm give a large characteristic length. The maximum stomatal conductance is an authored estimate for a fast-growing broadleaf of moist sites, guided by Jones (2014). Photosynthesis type: C3.", "Die Blattregionen verwenden die einseitige Blattfläche (265 m2, ein Blattflächenindex von etwa 4 über einer Kronenprojektion von etwa 66 m2, der dichteste der drei Straßenbäume); Sternblätter von 10 bis 18 cm geben eine große charakteristische Länge. Die maximale stomatäre Leitfähigkeit ist eine Autorenschätzung für einen raschwüchsigen Laubbaum feuchter Standorte, angelehnt an Jones (2014). Photosynthesetyp: C3."),
    T("Bark of stem, leader and limbs is passive; its areal heat capacity is the volumetric heat capacity of green wood (about 2.7 MJ per m3 and K) times an effective depth of 1 to 2.5 cm, an authored estimate. No surface temperature is authored.", "Die Rinde von Stamm, Stammverlängerung und Ästen ist passiv; ihre flächenbezogene Wärmekapazität ist die volumetrische Wärmekapazität frischen Holzes (etwa 2,7 MJ je m3 und K) mal einer wirksamen Tiefe von 1 bis 2,5 cm, eine Autorenschätzung. Es wird keine Oberflächentemperatur vorgegeben."),
    T("Transpiration emitters release the latent flux of their foliage region as vapour, spread over at least five anchors across the crown; sun and shade foliage are separate regions, and each pose authors their sunlit share. Leaf colour, the spiky seed balls and the bare winter state are not geometric: a pose cannot hide or add shapes, so the autumn and snow poses change the thermal binding and the loads only.", "Die Transpirationsemitter geben den latenten Fluss ihrer Laubregion als Wasserdampf ab, verteilt auf mindestens fünf Anker über die Krone; Sonnen- und Schattenlaub sind getrennte Regionen, jede Pose gibt ihren besonnten Anteil an. Laubfarbe, die stacheligen Fruchtkugeln und der kahle Winterzustand sind nicht geometrisch: eine Pose kann keine Formen aus- oder einblenden, Herbst- und Schneepose ändern daher nur die thermische Bindung und die Lasten."),
  ],
  structureAssumptions: [
    T("The tree is one rigid body per evaluated pose, fixed at the root plate; wind drag acts at the centre of the foliage clusters of every bone and on stem and leader as a line load, wet snow on the plan area of the clusters, gravity on every bone mass. The base moment follows from statics at the posed geometry; root-soil failure and dynamic amplification are not modelled. Pose lean angles are authored estimates scaled with the loads, not a computed deflection.", "Der Baum ist je ausgewerteter Pose ein starrer Körper, am Wurzelteller eingespannt; Winddruck wirkt im Mittelpunkt der Laubballen jedes Knochens und an Stamm und Stammverlängerung als Linienlast, Nassschnee auf die Grundrissfläche der Ballen, die Schwerkraft auf jede Knochenmasse. Das Fußmoment folgt aus der Statik der gezeigten Geometrie; Wurzel-Boden-Versagen und dynamische Überhöhung werden nicht modelliert. Die Neigungswinkel der Posen sind Autorenschätzungen proportional zu den Lasten, keine berechnete Durchbiegung."),
    T("Sections are solid circles of green sweetgum wood with the Wood Handbook stiffness and strength (E 8.3 GPa, MOR 49 MPa). Limbs that grow sideways are bones of the sagittal rig placed by their lateral offset: their solids follow the true limb axis, their bone the projection of it onto the plane of the wind.", "Querschnitte sind Vollkreise aus frischem Amberbaumholz mit Steifigkeit und Festigkeit nach dem Wood Handbook (E 8,3 GPa, Biegefestigkeit 49 MPa). Seitlich wachsende Äste sind Knochen des sagittalen Rigs, durch ihren seitlichen Versatz platziert: ihre Körper folgen der wahren Astachse, ihr Knochen deren Projektion in die Windebene."),
  ],
  environment: { airTempC: 26, meanRadiantTempC: 31, skyTempC: 14, groundTempC: 40, relativeHumidity: 0.45, windMS: 1.5, windDirectionDeg: 0, solarWM2: 750, diffuseFraction: 0.15, pressurePa: 101325 },
  densityKgM3: 880,
  leafAreaM2: { sun: 120, shade: 145 },
  leafFreshKgM2: 0.24,
  rootFlare: { profile: [[0, 0.56], [0.09, 0.45], [0.22, 0.33], [0.38, 0.27], [0.5, 0.25]], lengthM: ROOT_L },
  wood,
  foliage,
  branches: branches(branchRows, wood, azOf),
  clusters: clusters(clusterRows),
  masses500: [
    ...limbRows.map((row, i) => ({ id: `m-limb-${i + 1}`, host: row.id, members: clusterRows.filter((c) => c[5] === `m-limb-${i + 1}`).map((c) => c[0]), shrink: [1, 0.94, 0.94] as V3 })),
    { id: "m-core", host: "leader-2", members: clusterRows.filter((c) => c[5] === "m-core").map((c) => c[0]), shrink: [1, 0.94, 0.94] },
    { id: "m-top", host: "leader-4", members: clusterRows.filter((c) => c[5] === "m-top").map((c) => c[0]), shrink: [1, 0.95, 0.95] },
  ],
  lobes: { 100: { n: 7, core: 0.8, coreSide: 0.93, k: 0.56, s: 0.44, minUp: -0.55 }, 50: { n: 12, core: 0.82, coreSide: 0.94, k: 0.64, s: 0.36, minUp: -0.6 } },
  marks: { kind: "star", size: 0.15, spacing: 0.34 },
  bark: { kind: "ridge", lines: 5, seed: 19 },
  sway: { periodS: 4.2, breezeMS: 6, galeMS: 22, gustPeak: 2.75, lean: { 0: 0.05, 1: 0.14, 2: 0.38, 3: 0.6, 4: 1.25 } },
  cd: { breeze: 0.35, gale: 0.22, stem: 0.8 },
  poses: [
    { id: "calm", title: T("Calm summer day", "Windstiller Sommertag"), description: T("Light air of 1.5 m/s at 10 m over a sunny avenue: the dense pyramid stands at rest, outer leaves lit, the inner crown in deep shade, transpiration at 80 % of the daytime maximum.", "Leiser Zug von 1,5 m/s in 10 m Höhe über einer sonnigen Allee: die dichte Pyramide steht still, die äußeren Blätter sind besonnt, die Innenkrone liegt im tiefen Schatten, die Transpiration bei 80 % des Tagesmaximums."), windMS: 1.5, leanFactor: 0, sun: { "stem-bark": 0.25, "limb-bark": 0.22, "leaf-sun": 0.88, "leaf-shade": 0.07 }, emitters: { sun: 0.8, shade: 0.56 } },
    { id: "windy", title: T("Fresh breeze", "Frische Brise"), description: T("Moderate to fresh breeze of 6 m/s at 10 m (Beaufort 4): the long lower limbs and the clusters lean downwind; drag F = 0.5 rho cd A v^2 with cd 0.35 on the dense clusters.", "Mäßige bis frische Brise von 6 m/s in 10 m Höhe (Beaufort 4): die langen unteren Äste und die Ballen neigen sich in Windrichtung; Widerstand F = 0,5 rho cd A v^2 mit cd 0,35 an den dichten Ballen."), windMS: 6, leanFactor: 1, sun: { "stem-bark": 0.25, "limb-bark": 0.22, "leaf-sun": 0.88, "leaf-shade": 0.07 }, emitters: { sun: 0.95, shade: 0.66 } },
    { id: "storm", title: T("Gale with rain", "Sturm mit Regen"), description: T("Beaufort 9, 22 m/s at 10 m with rain: the tall crown leans strongly downwind; the star leaves curl into cones, so the drag coefficient falls to 0.22 (reconfiguration after Vogel 1989); wet bark and leaves, overcast sky.", "Beaufort 9, 22 m/s in 10 m Höhe mit Regen: die hohe Krone neigt sich stark in Windrichtung; die Sternblätter rollen sich zu Tüten, der Widerstandsbeiwert sinkt auf 0,22 (Rekonfiguration nach Vogel 1989); nasse Rinde und Blätter, bedeckter Himmel."), windMS: 22, leanFactor: 6.5, sun: { "stem-bark": 0.03, "limb-bark": 0.03, "leaf-sun": 0.1, "leaf-shade": 0.01 }, wet: 0.6, emitters: { sun: 0.45, shade: 0.3 } },
    { id: "autumn-colour", title: T("Autumn colour", "Herbstfärbung"), description: T("A clear, cool November day: the star leaves glow yellow, orange, scarlet and deep purple on one crown, stomata are mostly closed (transpiration 15 %) and the spiky seed balls hang on long stalks; sweetgum keeps its leaves late into the autumn. The drawn foliage stays: shapes are not toggled by a pose.", "Ein klarer, kühler Novembertag: die Sternblätter leuchten gelb, orange, scharlachrot und tief purpurn in einer Krone, die Spaltöffnungen sind weitgehend geschlossen (Transpiration 15 %), an langen Stielen hängen die stacheligen Fruchtkugeln; der Amberbaum hält sein Laub lange in den Herbst hinein. Das gezeichnete Laub bleibt: eine Pose blendet keine Formen aus."), windMS: 3, leanFactor: 0.3, sun: { "stem-bark": 0.2, "limb-bark": 0.18, "leaf-sun": 0.55, "leaf-shade": 0.05 }, wet: 0.15, emitters: { sun: 0.15, shade: 0.1 } },
    { id: "wet-snow", title: T("Early wet snow", "Früher Nassschnee"), description: T("An early wet snowfall of 10 kg per m2 of foliage plan area (order of magnitude from EN 1991-1-3) on the still leafy crown: the long lower limbs bend down, calm air, no sun, stomata closed. Late leaf fall makes sweetgum prone to such early snow and ice loads.", "Ein früher Nassschneefall von 10 kg je m2 Laubgrundfläche (Größenordnung nach EN 1991-1-3) auf die noch belaubte Krone: die langen unteren Äste biegen sich nach unten, windstill, keine Sonne, Spaltöffnungen geschlossen. Der späte Laubfall macht den Amberbaum anfällig für solche frühen Schnee- und Eislasten."), windMS: 1, leanFactor: 0, snowKgM2: 10, sun: { "stem-bark": 0.01, "limb-bark": 0.01, "leaf-sun": 0.03, "leaf-shade": 0.003 }, wet: 1, emitters: { sun: 0.05, shade: 0.035 } },
  ],
  texts: {
    crown: T("the tall pyramid of twelve spiral limb systems around the dominant leader, dense clusters of star leaves with few gaps.", "die hohe Pyramide aus zwölf spiralig gestellten Astsystemen um die durchgehende Stammverlängerung, dichte Ballen aus Sternblättern mit wenigen Lücken."),
    stem: T("the straight stem with its crown base at 3.6 m and its root flare,", "der gerade Stamm mit Kronenansatz in 3,6 m und Anlauf,"),
    limbs: T("the central leader and twelve spiral limbs", "die Stammverlängerung und zwölf spiralig gestellte Äste"),
    leaf: T("star-shaped five- to seven-lobed leaves of 10 to 18 cm", "sternförmige fünf- bis siebenlappige Blätter von 10 bis 18 cm"),
    bark: T("long ridge lines between deep furrows", "lange Rippenstriche zwischen tiefen Furchen"),
    omitted500: T("Omitted: side branches, single clusters and every texture; the crown reads as one tall pyramid.", "Weggelassen: Seitenzweige, einzelne Ballen und jede Textur; die Krone liest sich als eine hohe Pyramide."),
    omitted200: T("Omitted: side branches, lobes of the clusters, leaf and bark texture.", "Weggelassen: Seitenzweige, Lappen der Ballen, Blatt- und Rindentextur."),
  },
};

if (import.meta.main) console.log(JSON.stringify(writeAll(sweetgum).dimensions));

export default sweetgum;
