/** 🌾️ Gleditsia triacanthos f. inermis, the thornless honey locust of street and square planting, about 40 years old: a short stem that divides at 3 m into four arching, vase-shaped main limbs with steep risers and a weak central leader, all carrying small flat sprays of fine pinnate leaves, so the broad, airy crown filters the light and shows sky between its clusters.
 *
 * Every number below is hand-chosen for this tree (sources in `provenance.references`); `tree-asset.ts` only expands them.
 * Run: `repo tools exec bun -- honey-locust.ts` from this folder.
 * @see ./tree-asset.ts
 * @see https://doi.org/10.2737/FPL-GTR-282 — Wood Handbook, honeylocust */
import { T, type Species, type V3 } from "./tree-lib.ts";
import { along, branches, clusters, limb, stem, type AlongRow, type BranchRow, type ClusterRow, type LimbRow } from "./tree-design.ts";
import { writeAll } from "./tree-asset.ts";

const ROOT_L = 0.45;
const stemBones = stem(
  [
    [3.0, 0.03, 0.02, 0.24],
    [6.5, -0.12, 0.08, 0.1],
    [10.0, -0.05, -0.06, 0.06],
    [14.1, 0.08, -0.02, 0.025],
  ],
  ROOT_L,
  T("Short bole", "Kurzer Schaft"),
  (i) => T(`Weak central leader, part ${i}`, `Schwache Mittelachse, Teil ${i}`),
);

const L = (id: string, on: string, t: number, az: number, r0: number, segs: LimbRow["segs"], en: string, dt: string): LimbRow => ({ id, on, t, az, r0, segs, title: T(en, dt) });
const mainRows: LimbRow[] = [
  L("main-1", "leader-1", 0.02, 28, 0.15, [[61, 3.3, 0.1], [45, 2.8, 0.06, 8], [24, 2.3, 0.03, 6]], "Main limb, ahead left", "Hauptast, vorn links"),
  L("main-2", "leader-1", 0.04, 120, 0.14, [[58, 3.2, 0.095], [43, 2.7, 0.055, -7], [22, 2.2, 0.028, -6]], "Main limb, behind left", "Hauptast, hinten links"),
  L("main-3", "leader-1", 0.03, 207, 0.15, [[60, 3.3, 0.1], [44, 2.85, 0.06, 6], [25, 2.25, 0.03, 8]], "Main limb, behind right", "Hauptast, hinten rechts"),
  L("main-4", "leader-1", 0.05, 296, 0.13, [[57, 3.1, 0.09], [42, 2.65, 0.052, -8], [23, 2.15, 0.027, -5]], "Main limb, ahead right", "Hauptast, vorn rechts"),
];
const mains = mainRows.map((row) => limb(row, stemBones));
const woodSoFar = [...stemBones, ...mains.flatMap((l) => l.wood)];
const riserRows: LimbRow[] = [
  L("riser-1", "main-1", 0.92, 52, 0.07, [[72, 2.5, 0.045], [62, 1.9, 0.022, 10]], "Riser on the main limb ahead left", "Steigast auf dem Hauptast vorn links"),
  L("riser-2", "main-2", 0.9, 145, 0.065, [[74, 2.4, 0.042], [63, 1.8, 0.021, -10]], "Riser on the main limb behind left", "Steigast auf dem Hauptast hinten links"),
  L("riser-3", "main-3", 0.92, 232, 0.07, [[73, 2.5, 0.045], [62, 1.85, 0.022, 9]], "Riser on the main limb behind right", "Steigast auf dem Hauptast hinten rechts"),
  L("riser-4", "main-4", 0.9, 322, 0.062, [[71, 2.3, 0.04], [61, 1.75, 0.02, -9]], "Riser on the main limb ahead right", "Steigast auf dem Hauptast vorn rechts"),
  L("upper-1", "leader-2", 0.3, 345, 0.045, [[44, 1.9, 0.03], [34, 1.3, 0.016, 8]], "Upper limb on the leader, ahead", "Oberer Ast an der Mittelachse, vorn"),
  L("upper-2", "leader-2", 0.72, 168, 0.04, [[47, 1.8, 0.026], [36, 1.2, 0.015, -8]], "Upper limb on the leader, behind", "Oberer Ast an der Mittelachse, hinten"),
];
const risers = riserRows.map((row) => limb(row, woodSoFar));
const all = [...mains, ...risers];
const wood = [...woodSoFar, ...risers.flatMap((l) => l.wood)];
const foliage = all.map((l) => l.foliage);
const azOf = (id: string): number => {
  if (id.startsWith("leader")) return 0;
  const w = wood.find((b) => b.id === id)!;
  return (Math.atan2(w.b[2] - w.a[2], w.b[0] - w.a[0]) * 180) / Math.PI;
};

const branchRows: BranchRow[] = [
  ["main-1-2", 0.5, 40, 35, 1.1, 0.032, 0.016], ["main-1-2", 0.75, -45, 28, 1.0, 0.028, 0.015], ["main-1-3", 0.45, 50, 20, 0.9, 0.02, 0.012, [50]],
  ["main-2-2", 0.5, -42, 34, 1.05, 0.03, 0.016], ["main-2-2", 0.78, 44, 27, 0.95, 0.027, 0.015], ["main-2-3", 0.5, -48, 18, 0.85, 0.02, 0.012, [50]],
  ["main-3-2", 0.48, 44, 36, 1.1, 0.032, 0.016], ["main-3-2", 0.76, -40, 26, 1.0, 0.028, 0.015], ["main-3-3", 0.5, 46, 20, 0.9, 0.02, 0.012, [50]],
  ["main-4-2", 0.52, -44, 33, 1.0, 0.03, 0.016], ["main-4-2", 0.8, 42, 26, 0.9, 0.027, 0.015], ["main-4-3", 0.45, -50, 19, 0.8, 0.019, 0.012, [50]],
  ["riser-1", 0.6, -55, 45, 0.9, 0.026, 0.015], ["riser-2", 0.6, 55, 45, 0.85, 0.025, 0.015], ["riser-3", 0.6, -55, 46, 0.9, 0.026, 0.015], ["riser-4", 0.6, 55, 44, 0.85, 0.024, 0.015],
  ["leader-2", 0.15, 255, 40, 0.9, 0.024, 0.015], ["leader-2", 0.55, 75, 42, 0.85, 0.022, 0.015],
];

const mainAlong: AlongRow[] = [
  [0.42, 0.85, 0.42, 0.35, "leaf-shade"],
  [0.6, 0.9, 0.38, -0.45, "leaf-shade"],
  [0.78, 0.95, 0.34, 0.42, "leaf-sun"],
  [1.0, 1.0, 0.3, 0, "leaf-sun"],
];
const riserAlong: AlongRow[] = [
  [0.55, 0.9, 0.25, 0.35, "leaf-shade"],
  [0.8, 0.95, 0.25, -0.35, "leaf-sun"],
  [1.0, 1.0, 0.28, 0, "leaf-sun"],
];
const upperAlong: AlongRow[] = [
  [0.6, 0.9, 0.25, 0.3, "leaf-sun"],
  [1.0, 1.0, 0.28, 0, "leaf-sun"],
];
const crow: ClusterRow[] = [
  ...mains.flatMap((l, i) => along(l, mainAlong, [0.92, 0.5, 0.88], `m-main-${i + 1}`, 10 * (i + 1))),
  ...risers.slice(0, 4).flatMap((l, i) => along(l, riserAlong, [0.85, 0.52, 0.82], `m-riser-${i + 1}`, 50 + 10 * i)),
  ...risers.slice(4).flatMap((l, i) => along(l, upperAlong, [0.8, 0.5, 0.78], `m-upper-${i + 1}`, 90 + 5 * i)),
  ["cl-core", "leader-2", [-0.08, 8.2, 0.02], [0.8, 0.55, 0.78], "leaf-shade", "m-top", 101],
  ["cl-leader-high", "leader-3", [-0.02, 11.6, -0.05], [0.85, 0.55, 0.8], "leaf-sun", "m-top", 102],
  ["cl-leader-top", "leader-3", [0.07, 14.45, -0.02], [0.75, 0.55, 0.72], "leaf-sun", "m-top", 103],
];

const honeyLocust: Species = {
  id: "honey-locust",
  title: T("Thornless Honey Locust", "Dornenlose Gleditschie"),
  summary: T(
    "A thornless honey locust (Gleditsia triacanthos f. inermis) as planted in streets and squares: a short stem dividing at 3 m into four arching, vase-shaped main limbs with steep risers and a weak central leader; small flat sprays of fine pinnate leaves make the broad crown airy and light-filtering.",
    "Eine dornenlose Gleditschie (Gleditsia triacanthos f. inermis), wie sie in Straßen und auf Plätzen gepflanzt wird: ein kurzer Stamm, der sich in 3 m Höhe in vier bogige, vasenförmige Hauptäste mit steilen Steigästen und eine schwache Mittelachse teilt; kleine flache Zweigbüschel aus feinem gefiedertem Laub machen die breite Krone luftig und lichtdurchlässig.",
  ),
  distinction: T(
    "A 15 m thornless honey locust (Gleditsia triacanthos f. inermis) whose short stem forks at 3 m into four arching main limbs: a broad vase-to-umbrella crown about 12 m wide of small flat sprays of fine pinnate leaves with sky between them, the lightest shade of the street trees.",
    "Eine 15 m hohe dornenlose Gleditschie (Gleditsia triacanthos f. inermis), deren kurzer Stamm sich in 3 m Höhe in vier bogige Hauptäste gabelt: eine breite Vasen- bis Schirmkrone von etwa 12 m Breite aus kleinen flachen Büscheln feinen Fiederlaubs mit Himmel dazwischen, der lichteste Schatten der Straßenbäume.",
  ),
  tags: ["tree", "deciduous", "broadleaf", "street-tree", "vase-crown", "light-filtering", "urban", "wind", "transpiration"],
  provenance: {
    author: "Semio staffage plant worker PW-04",
    method: "handcrafted",
    created: "2026-11-22",
    references: [
      { id: "dirr-2009", citation: "Dirr, M. A. (2009). Manual of Woody Landscape Plants, 6th ed. Stipes Publishing, Champaign IL. Gleditsia triacanthos var. inermis: thornless, broad open spreading crown casting light shade, 30 to 70 ft; pinnately and bipinnately compound leaves with leaflets about 1 in long, yellow in autumn." },
      { id: "galk-strassenbaumliste", citation: "Deutsche Gartenamtsleiterkonferenz (GALK), Arbeitskreis Stadtbäume: GALK-Straßenbaumliste (online, current edition). Gleditsia triacanthos 'Inermis' and 'Skyline' listed as street trees with a light, open crown." },
      { id: "fll-ztv-baumpflege-2017", citation: "FLL (2017). ZTV-Baumpflege: Zusätzliche Technische Vertragsbedingungen und Richtlinien für Baumpflege, 6th ed. Forschungsgesellschaft Landschaftsentwicklung Landschaftsbau, Bonn. Clearance profile above footways, used for the fork height of 3 m." },
      { id: "wood-handbook-2021", citation: "Forest Products Laboratory (2021). Wood Handbook: Wood as an Engineering Material. General Technical Report FPL-GTR-282, USDA Forest Service. Table 5-3a, honeylocust, green: specific gravity 0.60, modulus of elasticity 8.9 GPa, modulus of rupture 70 MPa." },
      { id: "en-1991-1-4", citation: "EN 1991-1-4:2005 Eurocode 1, Actions on structures, Part 1-4: Wind actions. Air density 1.25 kg/m3, drag force F = 0.5 rho cd A v^2, peak velocity pressure." },
      { id: "vogel-1989", citation: "Vogel, S. (1989). Drag and reconfiguration of broad leaves in high winds. Journal of Experimental Botany 40(8), 941-948." },
      { id: "moore-maguire-2004", citation: "Moore, J. R. and Maguire, D. A. (2004). Natural sway frequencies and damping ratios of trees: concepts, review and synthesis of previous studies. Trees 18, 195-203." },
      { id: "jones-2014", citation: "Jones, H. G. (2014). Plants and Microclimate, 3rd ed. Cambridge University Press. Leaf energy balance, leaf area index, stomatal and boundary-layer conductance." },
      { id: "monteith-unsworth-2013", citation: "Monteith, J. L. and Unsworth, M. H. (2013). Principles of Environmental Physics, 4th ed. Academic Press. Penman-Monteith leaf transpiration." },
    ],
    notes: T(
      "Rig, stations and crown clusters are chosen by hand for a thornless honey locust of about 40 years (Dirr 2009 habit, GALK street-tree list): stem forked at 3 m (FLL clearance), four arching main limbs with risers, a weak central leader to 13 m. Green wood density and strength follow the Wood Handbook values for honeylocust; masses, conductances and areas are authored estimates.",
      "Rig, Stationen und Kronenballen sind von Hand für eine dornenlose Gleditschie von etwa 40 Jahren gewählt (Wuchsform nach Dirr 2009, GALK-Straßenbaumliste): Stamm in 3 m gegabelt (FLL-Lichtraum), vier bogige Hauptäste mit Steigästen, eine schwache Mittelachse bis 13 m. Dichte und Festigkeit des frischen Holzes folgen den Werten des Wood Handbook für Honeylocust; Massen, Leitfähigkeiten und Flächen sind Autorenschätzungen.",
    ),
  },
  materials: [
    { id: "honey-locust-bark", title: T("Honey locust bark, grey-brown with long narrow plates", "Gleditschienrinde, graubraun mit langen schmalen Platten"), emissivity: 0.95, solarAbsorptivity: 0.74, densityKgM3: 650, conductivityWmK: 0.13, source: "authored-estimate", reference: "wood-handbook-2021" },
    { id: "honey-locust-bark-young", title: T("Smooth brown bark of the limbs", "Glatte braune Rinde der Äste"), emissivity: 0.95, solarAbsorptivity: 0.7, densityKgM3: 700, conductivityWmK: 0.14, source: "authored-estimate", reference: "wood-handbook-2021" },
    { id: "honey-locust-leaf-sun", title: T("Sun leaflets of the outer sprays", "Sonnenfiedern der äußeren Büschel"), emissivity: 0.97, solarAbsorptivity: 0.5, source: "literature", reference: "jones-2014" },
    { id: "honey-locust-leaf-shade", title: T("Shade leaflets (effective)", "Schattenfiedern (wirksam)"), emissivity: 0.97, solarAbsorptivity: 0.35, source: "authored-estimate", reference: "jones-2014" },
    { id: "honey-locust-wood", title: T("Green honey locust wood", "Frisches Gleditschienholz"), emissivity: 0.9, solarAbsorptivity: 0.6, densityKgM3: 980, conductivityWmK: 0.17, elasticModulusPa: 8900000000, strengthPa: 70000000, source: "literature", reference: "wood-handbook-2021" },
  ],
  regions: {
    stem: { id: "stem-bark", title: T("Plated bark of stem and central leader", "Plattige Rinde von Stamm und Mittelachse"), kind: "bark", model: "passive", material: "honey-locust-bark", areaM2: 12.6, characteristicLengthM: 0.3, geometry: "cylinder", skyViewFactor: 0.45, layers: [], heatCapacityJM2K: 65000, source: "authored-estimate", reference: "wood-handbook-2021" },
    limb: { id: "limb-bark", title: T("Bark of the main limbs, risers and side branches", "Rinde der Hauptäste, Steigäste und Seitenzweige"), kind: "bark", model: "passive", material: "honey-locust-bark-young", areaM2: 24.1, characteristicLengthM: 0.12, geometry: "cylinder", skyViewFactor: 0.6, layers: [], heatCapacityJM2K: 35000, source: "authored-estimate", reference: "wood-handbook-2021" },
    sun: { id: "leaf-sun", title: T("Sun leaflets of the outer sprays", "Sonnenfiedern der äußeren Büschel"), kind: "foliage", model: "leaf", material: "honey-locust-leaf-sun", areaM2: 150, areaBasis: "exchange", characteristicLengthM: 0.02, geometry: "horizontal-plate-up", skyViewFactor: 0.75, layers: [], heatCapacityJM2K: 800, leaf: { stomatalConductanceMaxMS: 0.005, leafAreaIndex: 1.4, photosynthesis: "c3" }, source: "authored-estimate", reference: "jones-2014" },
    shade: { id: "leaf-shade", title: T("Shade leaflets of the inner sprays", "Schattenfiedern der inneren Büschel"), kind: "foliage", model: "leaf", material: "honey-locust-leaf-shade", areaM2: 85, areaBasis: "exchange", characteristicLengthM: 0.022, geometry: "horizontal-plate-up", skyViewFactor: 0.45, layers: [], heatCapacityJM2K: 650, leaf: { stomatalConductanceMaxMS: 0.0032, leafAreaIndex: 0.8, photosynthesis: "c3" }, source: "authored-estimate", reference: "jones-2014" },
  },
  assumptions: [
    T("Leaf regions use one-sided leaf area (235 m2, a leaf area index of about 2.2 over a crown projection of about 108 m2, low for the light, filtering crown); leaflets of 1.5 to 3 cm give a small characteristic length and a strong coupling to the air. The maximum stomatal conductance is an authored estimate for a drought-tolerant legume tree, guided by Jones (2014). Photosynthesis type: C3.", "Die Blattregionen verwenden die einseitige Blattfläche (235 m2, ein Blattflächenindex von etwa 2,2 über einer Kronenprojektion von etwa 108 m2, niedrig für die lichte, filternde Krone); Fiederblättchen von 1,5 bis 3 cm geben eine kleine charakteristische Länge und eine starke Kopplung an die Luft. Die maximale stomatäre Leitfähigkeit ist eine Autorenschätzung für einen trockenheitsverträglichen Leguminosenbaum, angelehnt an Jones (2014). Photosynthesetyp: C3."),
    T("Bark of stem, leader, limbs and risers is passive; its areal heat capacity is the volumetric heat capacity of green wood (about 2.9 MJ per m3 and K for this dense wood) times an effective depth of 1 to 2.5 cm, an authored estimate. No surface temperature is authored.", "Die Rinde von Stamm, Mittelachse, Ästen und Steigästen ist passiv; ihre flächenbezogene Wärmekapazität ist die volumetrische Wärmekapazität frischen Holzes (etwa 2,9 MJ je m3 und K für dieses dichte Holz) mal einer wirksamen Tiefe von 1 bis 2,5 cm, eine Autorenschätzung. Es wird keine Oberflächentemperatur vorgegeben."),
    T("Transpiration emitters release the latent flux of their foliage region as vapour, spread over at least five anchors across the crown; sun and shade sprays are separate regions, and each pose authors their sunlit share. Leaf colour and the bare winter state with its long pods are not geometric: a pose cannot hide shapes, so the autumn pose changes the thermal binding and the loads only.", "Die Transpirationsemitter geben den latenten Fluss ihrer Laubregion als Wasserdampf ab, verteilt auf mindestens fünf Anker über die Krone; Sonnen- und Schattenbüschel sind getrennte Regionen, jede Pose gibt ihren besonnten Anteil an. Laubfarbe und der kahle Winterzustand mit den langen Hülsen sind nicht geometrisch: eine Pose kann keine Formen ausblenden, die Herbstpose ändert daher nur die thermische Bindung und die Lasten."),
  ],
  structureAssumptions: [
    T("The tree is one rigid body per evaluated pose, fixed at the root plate; wind drag acts at the centre of the foliage sprays of every bone and on stem and leader as a line load, gravity on every bone mass. The base moment follows from statics at the posed geometry; the fork of the four main limbs is a rigid joint (included bark and union failure are not modelled), nor are root-soil failure and dynamic amplification. Pose lean angles are authored estimates scaled with the drag.", "Der Baum ist je ausgewerteter Pose ein starrer Körper, am Wurzelteller eingespannt; Winddruck wirkt im Mittelpunkt der Laubbüschel jedes Knochens und an Stamm und Mittelachse als Linienlast, die Schwerkraft auf jede Knochenmasse. Das Fußmoment folgt aus der Statik der gezeigten Geometrie; die Gabel der vier Hauptäste ist ein starrer Anschluss (eingewachsene Rinde und Zwieselbruch werden nicht modelliert), ebenso wenig Wurzel-Boden-Versagen und dynamische Überhöhung. Die Neigungswinkel der Posen sind Autorenschätzungen proportional zum Widerstand."),
    T("Sections are solid circles of green honey locust wood with the Wood Handbook values (specific gravity 0.60 green, E 8.9 GPa, MOR 70 MPa). Limbs that grow sideways are bones of the sagittal rig placed by their lateral offset: their solids follow the true limb axis, their bone the projection of it onto the plane of the wind.", "Querschnitte sind Vollkreise aus frischem Gleditschienholz mit den Werten des Wood Handbook (Rohdichte frisch 0,60, E 8,9 GPa, Biegefestigkeit 70 MPa). Seitlich wachsende Äste sind Knochen des sagittalen Rigs, durch ihren seitlichen Versatz platziert: ihre Körper folgen der wahren Astachse, ihr Knochen deren Projektion in die Windebene."),
  ],
  environment: { airTempC: 26, meanRadiantTempC: 31, skyTempC: 14, groundTempC: 40, relativeHumidity: 0.45, windMS: 1.5, windDirectionDeg: 0, solarWM2: 750, diffuseFraction: 0.15, pressurePa: 101325 },
  densityKgM3: 980,
  leafAreaM2: { sun: 150, shade: 85 },
  leafFreshKgM2: 0.2,
  rootFlare: { profile: [[0, 0.55], [0.08, 0.45], [0.2, 0.34], [0.34, 0.285], [0.45, 0.27]], lengthM: ROOT_L },
  wood,
  foliage,
  branches: branches(branchRows, wood, azOf),
  clusters: clusters(crow),
  masses500: [
    ...[1, 2, 3, 4].map((k) => ({ id: `m-main-${k}`, host: `main-${k}`, members: crow.filter((c) => c[5] === `m-main-${k}`).map((c) => c[0]), shrink: [1, 0.94, 0.94] as V3 })),
    ...[1, 2, 3, 4].map((k) => ({ id: `m-riser-${k}`, host: `riser-${k}`, members: crow.filter((c) => c[5] === `m-riser-${k}`).map((c) => c[0]), shrink: [1, 0.94, 0.94] as V3 })),
    ...[1, 2].map((k) => ({ id: `m-upper-${k}`, host: `upper-${k}`, members: crow.filter((c) => c[5] === `m-upper-${k}`).map((c) => c[0]), shrink: [1, 0.95, 0.95] as V3 })),
    { id: "m-top", host: "leader-2", members: crow.filter((c) => c[5] === "m-top").map((c) => c[0]), shrink: [0.97, 0.95, 0.95] },
  ],
  lobes: { 100: { n: 6, core: 0.78, coreSide: 0.93, k: 0.56, s: 0.44, minUp: -0.45 }, 50: { n: 11, core: 0.8, coreSide: 0.94, k: 0.64, s: 0.36, minUp: -0.5 } },
  marks: { kind: "pinnate", size: 0.16, spacing: 0.32 },
  bark: { kind: "plate", lines: 5, seed: 13 },
  sway: { periodS: 3.6, breezeMS: 6, galeMS: 22, gustPeak: 2.75, lean: { 0: 0.06, 1: 0.18, 2: 0.4, 3: 0.65, 4: 1.3 } },
  cd: { breeze: 0.28, gale: 0.18, stem: 0.8 },
  poses: [
    { id: "calm", title: T("Calm summer day", "Windstiller Sommertag"), description: T("Light air of 1.5 m/s at 10 m over a sunny square: the airy crown stands at rest and lets dappled light through; transpiration at 80 % of the daytime maximum.", "Leiser Zug von 1,5 m/s in 10 m Höhe über einem sonnigen Platz: die luftige Krone steht still und lässt gesprenkeltes Licht durch; die Transpiration liegt bei 80 % des Tagesmaximums."), windMS: 1.5, leanFactor: 0, sun: { "stem-bark": 0.45, "limb-bark": 0.45, "leaf-sun": 0.9, "leaf-shade": 0.25 }, emitters: { sun: 0.8, shade: 0.56 } },
    { id: "windy", title: T("Fresh breeze", "Frische Brise"), description: T("Moderate to fresh breeze of 6 m/s at 10 m (Beaufort 4): the long main limbs and the flat sprays lean downwind; the fine pinnate leaves fold early, drag F = 0.5 rho cd A v^2 with cd 0.28 on the porous sprays.", "Mäßige bis frische Brise von 6 m/s in 10 m Höhe (Beaufort 4): die langen Hauptäste und die flachen Büschel neigen sich in Windrichtung; die feinen Fiederblätter falten sich früh, Widerstand F = 0,5 rho cd A v^2 mit cd 0,28 an den durchlässigen Büscheln."), windMS: 6, leanFactor: 1, sun: { "stem-bark": 0.45, "limb-bark": 0.45, "leaf-sun": 0.9, "leaf-shade": 0.25 }, emitters: { sun: 0.95, shade: 0.66 } },
    { id: "storm", title: T("Gale with rain", "Sturm mit Regen"), description: T("Beaufort 9, 22 m/s at 10 m with rain: the vase crown leans strongly downwind; the leaflets fold flat, so the drag coefficient falls to 0.18 (reconfiguration after Vogel 1989); wet bark and leaves, overcast sky.", "Beaufort 9, 22 m/s in 10 m Höhe mit Regen: die Vasenkrone neigt sich stark in Windrichtung; die Fiederblättchen legen sich flach, der Widerstandsbeiwert sinkt auf 0,18 (Rekonfiguration nach Vogel 1989); nasse Rinde und Blätter, bedeckter Himmel."), windMS: 22, leanFactor: 6.5, sun: { "stem-bark": 0.05, "limb-bark": 0.05, "leaf-sun": 0.12, "leaf-shade": 0.03 }, wet: 0.6, emitters: { sun: 0.45, shade: 0.3 } },
    { id: "autumn-gold", title: T("Yellow autumn", "Gelber Herbst"), description: T("A mild, sunny October afternoon: the leaflets have turned clear yellow and stomata are mostly closed (transpiration 15 %); the fine leaflets will fall without raking and leave the long twisted pods. The drawn foliage stays: shapes are not toggled by a pose.", "Ein milder, sonniger Oktobernachmittag: die Fiederblättchen sind klar gelb gefärbt, die Spaltöffnungen weitgehend geschlossen (Transpiration 15 %); das feine Laub fällt, ohne gerecht werden zu müssen, und lässt die langen gedrehten Hülsen zurück. Das gezeichnete Laub bleibt: eine Pose blendet keine Formen aus."), windMS: 3, leanFactor: 0.3, sun: { "stem-bark": 0.4, "limb-bark": 0.4, "leaf-sun": 0.65, "leaf-shade": 0.2 }, wet: 0.1, emitters: { sun: 0.15, shade: 0.1 } },
  ],
  texts: {
    crown: T("the broad vase of four arching main limbs with their risers and the weak central leader, carrying small flat sprays with sky between them.", "die breite Vase aus vier bogigen Hauptästen mit ihren Steigästen und der schwachen Mittelachse, die kleine flache Büschel mit Himmel dazwischen tragen."),
    stem: T("the short stem forked at 3 m with its root flare,", "der kurze, in 3 m gegabelte Stamm mit Anlauf,"),
    limbs: T("four arching main limbs, four risers, the weak leader with two upper limbs", "vier bogige Hauptäste, vier Steigäste, die schwache Mittelachse mit zwei oberen Ästen"),
    leaf: T("pinnate and bipinnate leaves with leaflets of 1.5 to 3 cm", "gefiederte und doppelt gefiederte Blätter mit Fiederblättchen von 1,5 bis 3 cm"),
    bark: T("short offset plate lines", "kurze versetzte Plattenstriche"),
    omitted500: T("Omitted: side branches, single sprays, the sky gaps under 0.25 m2 and every texture; the crown reads as one broad vase.", "Weggelassen: Seitenzweige, einzelne Büschel, Himmelslücken unter 0,25 m2 und jede Textur; die Krone liest sich als eine breite Vase."),
    omitted200: T("Omitted: side branches, lobes of the sprays, leaf and bark texture.", "Weggelassen: Seitenzweige, Lappen der Büschel, Blatt- und Rindentextur."),
  },
};

if (import.meta.main) console.log(JSON.stringify(writeAll(honeyLocust).dimensions));

export default honeyLocust;
