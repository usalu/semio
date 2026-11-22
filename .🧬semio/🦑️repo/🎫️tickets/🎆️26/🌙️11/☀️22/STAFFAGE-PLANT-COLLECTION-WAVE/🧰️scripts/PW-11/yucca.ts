/** 🌴️ PW-11: the spineless yucca (Yucca gigantea, syn. Y. elephantipes) stated by hand: a swollen elephant-foot base, a short trunk that forks into four stems of different height, each crowned by a rosette of twenty stiff strap-shaped leaves in the 137.5 degree spiral (old leaves reflexed, young leaves erect). Leaves are thin sheets on their own bones; C3 physiology (stomata open by day). Builds the whole `semio.staffage/v2` asset; the leaf blades reuse the planform-and-plate method of the agave generators.
 * @see ../../📋️brief.md
 * @see ./kit.ts */
import { FEATURE, PENS, RANGES, RigBuilder, SCALES, T, add, barOutline, dir, documentOf, dot, drag, ellipsoidSolid, leanOffsets, loftU, mul, r4, r5, revolveSolid, revolvedArea, revolvedShape, plateShape, plateSolid, sub, table, upNormal, views, world, EXPORTS, type Drawn, type Frame, type Json, type Scale, type V3 } from "./kit.ts";

export const ID = "yucca";

const REF = {
  irish: { id: "irish-2000", citation: "Irish, M. and Irish, G. (2000). Agaves, Yuccas, and Related Plants: A Gardener's Guide. Timber Press. Yucca gigantea (Y. elephantipes): tree-like, swollen base, branched stems with terminal rosettes of leaves 50-100 cm long and 5-7 cm wide, soft (not spine-tipped) leaf points." },
  heyduk: { id: "heyduk-2016", citation: "Heyduk, K., Burrell, N., Lalani, F. and Leebens-Mack, J. (2016). Gas exchange and leaf anatomy of a C3-CAM hybrid, Yucca gloriosa (Asparagaceae). Journal of Experimental Botany 67(5), 1369-1379. Yucca species range from C3 to CAM photosynthesis." },
  jones: { id: "jones-2014", citation: "Jones, H. G. (2014). Plants and Microclimate, 3rd ed., Cambridge University Press. Leaf energy balance, stomatal conductance of sclerophyllous leaves." },
  monteith: { id: "monteith-unsworth-2013", citation: "Monteith, J. L. and Unsworth, M. H. (2013). Principles of Environmental Physics, 4th ed., Academic Press. Penman-Monteith leaf transpiration." },
  vogel: { id: "vogel-1989", citation: "Vogel, S. (1989). Drag and reconfiguration of broad leaves in high winds. Journal of Experimental Botany 40(8), 941-948. Leaves and leaf clusters reconfigure and lower their drag coefficient in strong wind." },
  en: { id: "en-1991-1-4", citation: "EN 1991-1-4:2005 Eurocode 1, Actions on structures, Part 1-4: Wind actions. Dynamic pressure q = 0.5 rho v^2 with air density 1.25 kg/m3, force F = cf q A; mean wind profile." },
  en13: { id: "en-1991-1-3", citation: "EN 1991-1-3:2003 Eurocode 1, Actions on structures, Part 1-3: Snow loads. Density of wet snow (about 300-400 kg/m3)." },
};

/** 🌿️ The four stems from the fork at 1.15 m: azimuth (counter-clockwise from ahead), elevation, length and radius at both ends; the central leader is the tallest, the side stems lean out and stand lower, as in garden specimens of about 15 years. */
const FORK = 1.15;
const STEMS = [
  { id: "stem-a", az: 35, el: 70, L: 0.95, r0: 0.085, r1: 0.07 },
  { id: "stem-b", az: 160, el: 74, L: 0.72, r0: 0.08, r1: 0.068 },
  { id: "stem-c", az: 275, el: 66, L: 0.52, r0: 0.075, r1: 0.065 },
  { id: "stem-d", az: 0, el: 88, L: 1.08, r0: 0.09, r1: 0.075 },
];
const BASE_PROFILE: [number, number][] = [[0, 0.27], [0.06, 0.25], [0.15, 0.21], [0.28, 0.16], [0.4, 0.135], [0.5, 0.125]];
const TRUNK_PROFILE: [number, number][] = [[-0.03, 0.125], [0.3, 0.115], [0.62, 0.105], [0.68, 0.11]];
const HEAD = 0.12;
const LEAVES = 20, GOLDEN = 137.5;

/** 🍃️ One leaf of a rosette by its spiral index k (0 oldest): elevation from -32 degrees (reflexed old leaf) to 84 degrees (young erect leaf), length 0.5-0.74 m, width 6-7 cm; insertion along the head from its base to its tip. */
const leafOf = (k: number) => {
  const t = k / (LEAVES - 1);
  return { el: -32 + 116 * Math.pow(t, 0.85), L: table([[0, 0.66], [0.35, 0.74], [0.75, 0.68], [1, 0.5]], t), W: table([[0, 0.068], [0.6, 0.066], [1, 0.058]], t), h: HEAD * table([[0, 0.05], [1, 0.95]], t) };
};
/** 📏️ Half width of the strap-shaped leaf along it: clasping base, slightly narrowed above it, parallel-sided, then tapering to a soft point. */
const SHAPE: [number, number][] = [[0, 0.46], [0.06, 0.38], [0.18, 0.46], [0.45, 0.5], [0.7, 0.47], [0.85, 0.36], [0.95, 0.17], [1, 0.02]];
const hw = (W: number, s: number) => W * table(SHAPE, s);
const LEAF_T = 0.0028;
const WOOD = 650;

const B = new RigBuilder({ id: "root-plate", title: T("Root plate in the planting bed", "Wurzelteller im Pflanzbeet"), lengthM: 0.4, joint: { type: "fixed" } });
function volOf(p: readonly [number, number][]): number { let v = 0; for (let i = 1; i < p.length; i++) v += Math.PI * (p[i]![0] - p[i - 1]![0]) * (p[i]![1] ** 2 + p[i]![1] * p[i - 1]![1] + p[i - 1]![1] ** 2) / 3; return v; }
const sec = (r: number, title: Json) => ({ areaM2: r5(Math.PI * r * r), secondMomentM4: Number((Math.PI * r ** 4 / 4).toPrecision(4)), extremeFibreM: r5(r), material: "yucca-stem", title });
B.add("foot", "root-plate", T("Swollen base (elephant foot), water store", "Verdickte Basis (Elefantenfuß), Wasserspeicher"), [0, 0, 0], [0, 1, 0], [-1, 0, 0], 0.5, { type: "fixed" }, { mass: { kg: r4(volOf(BASE_PROFILE) * WOOD), comX: 0.17, comY: 0, distribution: "uniform", source: "authored-estimate", reference: "irish-2000" }, section: sec(0.125, T("Solid stem, d = 25 cm at the neck of the foot", "Vollstamm, d = 25 cm am Hals des Fußes")) });
B.add("trunk", "foot", T("Trunk up to the fork", "Stamm bis zur Gabelung"), [0, 0.5, 0], [0, 1, 0], [-1, 0, 0], FORK - 0.5, { type: "revolute", minDeg: -3, maxDeg: 3 }, { mass: { kg: r4(volOf(TRUNK_PROFILE) * WOOD), comX: 0.32, comY: 0, distribution: "uniform", source: "authored-estimate", reference: "irish-2000" }, section: sec(0.105, T("Solid stem, d = 21 cm", "Vollstamm, d = 21 cm")) });

type Rosette = { stem: typeof STEMS[number]; head: Frame; leaves: { id: string; k: number; f: Frame; L: number; W: number; el: number; az: number }[] };
const ROSETTES: Rosette[] = [];
for (const s of STEMS) {
  const u = dir(s.az, s.el);
  const o: V3 = add([0, FORK - 0.04, 0], mul(dir(s.az, 0), s.id === "stem-d" ? 0 : 0.03));
  B.add(s.id, "trunk", T(`Stem ${s.id.slice(-1).toUpperCase()} (${Math.round(s.L * 100)} cm)`, `Stamm ${s.id.slice(-1).toUpperCase()} (${Math.round(s.L * 100)} cm)`), o, u, upNormal(s.az, s.el), s.L, { type: "revolute", minDeg: -6, maxDeg: 6 }, { mass: { kg: r4(Math.PI * ((s.r0 + s.r1) / 2) ** 2 * s.L * WOOD), comX: r4(s.L / 2), comY: 0, distribution: "uniform", source: "authored-estimate", reference: "irish-2000" }, section: sec(s.r1, T(`Solid stem, d = ${Math.round(s.r1 * 200)} cm`, `Vollstamm, d = ${Math.round(s.r1 * 200)} cm`)) });
  const top = add(o, mul(u, s.L));
  const hid = `head-${s.id.slice(-1)}`;
  const head = B.add(hid, s.id, T(`Rosette head ${s.id.slice(-1).toUpperCase()}: leaf insertions and the growing point`, `Rosettenkopf ${s.id.slice(-1).toUpperCase()}: Blattansätze und Vegetationspunkt`), top, [0, 1, 0], [-Math.cos(s.az * Math.PI / 180), 0, -Math.sin(s.az * Math.PI / 180)], HEAD, { type: "fixed" }, { mass: { kg: r4(Math.PI * 0.07 ** 2 * HEAD * WOOD), comX: 0.06, comY: 0, distribution: "uniform", source: "authored-estimate", reference: "irish-2000" } });
  const r: Rosette = { stem: s, head, leaves: [] };
  const phase = { "stem-a": 12, "stem-b": 75, "stem-c": 140, "stem-d": 200 }[s.id]!;
  for (let k = 0; k < LEAVES; k++) {
    const lf = leafOf(k), az = phase + k * GOLDEN;
    const d = dir(az, lf.el), v = upNormal(az, lf.el);
    const origin = add(add(top, [0, lf.h, 0]), mul(dir(az, 0), 0.045));
    const id = `leaf-${s.id.slice(-1)}-${String(k + 1).padStart(2, "0")}`;
    const f = B.add(id, hid, T(`Leaf ${k + 1} of rosette ${s.id.slice(-1).toUpperCase()} (${lf.el < 0 ? "old, reflexed" : lf.el < 45 ? "spreading" : "young, erect"})`, `Blatt ${k + 1} der Rosette ${s.id.slice(-1).toUpperCase()} (${lf.el < 0 ? "alt, zurückgebogen" : lf.el < 45 ? "abstehend" : "jung, aufrecht"})`), origin, d, v, lf.L, { type: "revolute", minDeg: -30, maxDeg: 30 }, { mass: { kg: r4(lf.L * lf.W * 0.85 * 0.0022 * 1050 + 0.01), comX: r4(lf.L * 0.4), comY: 0, distribution: "uniform", source: "authored-estimate", reference: "irish-2000" } });
    r.leaves.push({ id, k, f, L: lf.L, W: lf.W, el: lf.el, az });
  }
  ROSETTES.push(r);
}
B.bones[0].mass = { kg: 15, comX: 0, comY: -0.2, distribution: "point", source: "authored-estimate", reference: "irish-2000" };
const rigError = B.verify();
const RIG = B.rig();
const upper = (l: { el: number }) => l.el >= 25;
const ALL = ROSETTES.flatMap((r) => r.leaves);

const leafArea = (l: { L: number; W: number }) => { let a = 0; for (let k = 0; k < 40; k++) a += 2 * 2 * hw(l.W, (k + 0.5) / 40) * (l.L / 40); return a; };
const stemProfile = (s: typeof STEMS[number]): [number, number][] => [[-0.03, s.r0], [s.L * 0.5, (s.r0 + s.r1) / 2], [s.L + 0.02, s.r1]];
const headProfile: [number, number][] = [[-0.02, 0.078], [0.05, 0.075], [HEAD, 0.06], [HEAD + 0.04, 0.03]];
const AREA = {
  upper: ALL.filter(upper).reduce((s, l) => s + leafArea(l), 0),
  lower: ALL.filter((l) => !upper(l)).reduce((s, l) => s + leafArea(l), 0),
  bark: 0.9 * (revolvedArea(BASE_PROFILE) + revolvedArea(TRUNK_PROFILE) + STEMS.reduce((s, x) => s + revolvedArea(stemProfile(x)) + revolvedArea(headProfile), 0)),
};

const MATERIALS = [
  { id: "yucca-leaf", title: T("Stiff, leathery yucca leaf with a thick cuticle", "Steifes, ledriges Palmlilienblatt mit dicker Kutikula"), emissivity: 0.96, solarAbsorptivity: 0.55, densityKgM3: 1050, specificHeatJKgK: 3500, conductivityWmK: 0.4, elasticModulusPa: 400000000, strengthPa: 15000000, source: "authored-estimate", reference: "jones-2014" },
  { id: "yucca-stem", title: T("Fibrous, water-storing yucca stem with ringed grey bark", "Faseriger, wasserspeichernder Palmlilienstamm mit geringelter grauer Rinde"), emissivity: 0.92, solarAbsorptivity: 0.65, densityKgM3: WOOD, conductivityWmK: 0.25, specificHeatJKgK: 2800, elasticModulusPa: 1500000000, strengthPa: 15000000, source: "authored-estimate", reference: "irish-2000" },
];
const REGIONS = [
  { id: "leaves-upper", title: T("Young and spreading leaves of the rosettes", "Junge und abstehende Blätter der Rosetten"), kind: "foliage", model: "leaf", material: "yucca-leaf", areaM2: r4(AREA.upper), areaBasis: "exchange", characteristicLengthM: 0.065, geometry: "vertical-plate", skyViewFactor: 0.6, layers: [], heatCapacityJM2K: 2500, leaf: { stomatalConductanceMaxMS: 0.004, leafAreaIndex: 2.5, photosynthesis: "c3" }, source: "authored-estimate", reference: "jones-2014" },
  { id: "leaves-lower", title: T("Old reflexed leaves under the rosettes", "Alte zurückgebogene Blätter unter den Rosetten"), kind: "foliage", model: "leaf", material: "yucca-leaf", areaM2: r4(AREA.lower), areaBasis: "exchange", characteristicLengthM: 0.065, geometry: "horizontal-plate-up", skyViewFactor: 0.35, layers: [], heatCapacityJM2K: 2500, leaf: { stomatalConductanceMaxMS: 0.0025, leafAreaIndex: 1.5, photosynthesis: "c3" }, source: "authored-estimate", reference: "jones-2014" },
  { id: "stem-bark", title: T("Elephant foot, trunk and stems", "Elefantenfuß, Stamm und Stämme"), kind: "bark", model: "passive", material: "yucca-stem", areaM2: r4(AREA.bark), characteristicLengthM: 0.18, geometry: "cylinder", skyViewFactor: 0.4, layers: [], heatCapacityJM2K: 90000, source: "authored-estimate", reference: "irish-2000" },
];
const outlets = (pred: (l: typeof ALL[number]) => boolean, n: number) => { const ls = ALL.filter(pred); return Array.from({ length: n }, (_, i) => ls[Math.floor((i * ls.length) / n + ls.length / (2 * n))]!); };
const UP_OUT = outlets(upper, 8), LOW_OUT = outlets((l) => !upper(l), 6);
const ANCHORS = [
  { id: "base-support", bone: "root-plate", x: 0, y: 0, kind: "support", title: T("Root plate in the bed (fixed)", "Wurzelteller im Beet (eingespannt)"), support: "fixed", normalDeg: 90, footprintLengthM: 0.8, footprintWidthM: 0.8, frictionCoefficient: 0.6 },
  ...UP_OUT.map((l, i) => ({ id: `tp-upper-${i + 1}`, bone: l.id, x: r4(l.L * 0.5), y: r4(LEAF_T / 2 + 0.002), kind: "emitter", title: T(`Transpiration of ${l.id}`, `Transpiration von ${l.id}`) })),
  ...LOW_OUT.map((l, i) => ({ id: `tp-lower-${i + 1}`, bone: l.id, x: r4(l.L * 0.5), y: r4(LEAF_T / 2 + 0.002), kind: "emitter", title: T(`Transpiration of ${l.id}`, `Transpiration von ${l.id}`) })),
];
const EMITTERS = [
  { id: "transpiration-upper", kind: "transpiration", title: T("Transpiration of the young and spreading leaves (by day, C3; eight outlets over the four rosettes)", "Transpiration der jungen und abstehenden Blätter (tagsüber, C3; acht Auslässe über die vier Rosetten)"), anchor: "tp-upper-1", anchors: UP_OUT.map((_, i) => `tp-upper-${i + 1}`), directionDeg: 90, diameterM: 0.5, region: "leaves-upper", source: "authored-estimate", reference: "monteith-unsworth-2013" },
  { id: "transpiration-lower", kind: "transpiration", title: T("Transpiration of the old reflexed leaves (six outlets)", "Transpiration der alten zurückgebogenen Blätter (sechs Auslässe)"), anchor: "tp-lower-1", anchors: LOW_OUT.map((_, i) => `tp-lower-${i + 1}`), directionDeg: 90, diameterM: 0.4, region: "leaves-lower", source: "authored-estimate", reference: "monteith-unsworth-2013" },
];
const ENV = { airTempC: 31, meanRadiantTempC: 36, skyTempC: 19, groundTempC: 45, relativeHumidity: 0.35, windMS: 1.5, windDirectionDeg: 0, solarWM2: 850, diffuseFraction: 0.12, pressurePa: 101325 };

const v10 = (y: number, v: number) => v * Math.pow(Math.max(y, 1) / 10, 0.22);
function windLoads(v: number, cd: number, label: Json): Json[] {
  const loads: Json[] = ROSETTES.map((r) => {
    const A = r.leaves.reduce((s, l) => s + (leafArea(l) / 2) * Math.abs(dot(l.f.v, [1, 0, 0])) * 0.6, 0);
    return { id: `wind-rosette-${r.stem.id.slice(-1)}`, kind: "wind", title: T(`Wind drag on rosette ${r.stem.id.slice(-1).toUpperCase()}, ${label.en}`, `Winddruck auf Rosette ${r.stem.id.slice(-1).toUpperCase()}, ${label.de}`), bone: `head-${r.stem.id.slice(-1)}`, x: 0.1, y: 0, fxN: r4(drag(cd, A, v10(r.head.o[1], v))), fyN: 0, source: "literature", reference: "en-1991-1-4" };
  });
  loads.push({ id: "wind-foot-trunk", kind: "wind", title: T(`Wind drag on foot and trunk, ${label.en}`, `Winddruck auf Fuß und Stamm, ${label.de}`), fxN: 0, fyN: 0, distributed: { bone: "trunk", fromM: 0, toM: r4(FORK - 0.5), wxNPerM: r4(drag(1.0, 0.22, v10(0.8, v))), wyNPerM: 0 }, source: "literature", reference: "en-1991-1-4" } as Json);
  return loads;
}
const total = (l: Json[]) => l.reduce((s, x) => s + (x.distributed ? x.distributed.wxNPerM * x.distributed.toM : x.fxN), 0);
const leanFor = (k: number, leaves = 1) => [{ id: "trunk", deg: -0.4 * k }, ...STEMS.map((s) => ({ id: s.id, deg: -1.6 * k })), ...ALL.map((l) => ({ id: l.id, deg: r4(-(upper(l) ? 9 : 5) * k * leaves) }))];
const WINDY = leanOffsets(RIG, {}, leanFor(0.22));
const STORM = leanOffsets(RIG, {}, leanFor(1));
const SNOW = Object.fromEntries(ALL.filter((l) => l.el > -10 && l.el < 60).map((l) => [l.id, { angleDeg: r4(-12 * Math.cos((l.el * Math.PI) / 180)) }]));

const rDay = { "leaves-upper": { exposedFraction: 1, sunlitFraction: 0.5 }, "leaves-lower": { exposedFraction: 1, sunlitFraction: 0.3 }, "stem-bark": { exposedFraction: 1, sunlitFraction: 0.25 } };
const rNight = { "leaves-upper": { exposedFraction: 1, sunlitFraction: 0 }, "leaves-lower": { exposedFraction: 1, sunlitFraction: 0 }, "stem-bark": { exposedFraction: 1, sunlitFraction: 0 } };
const day = { "transpiration-upper": 1, "transpiration-lower": 0.7 }, night = { "transpiration-upper": 0.05, "transpiration-lower": 0.04 };
const windyLoads = windLoads(12, 0.8, T("12 m/s", "12 m/s")), stormLoads = windLoads(26, 0.45, T("26 m/s", "26 m/s"));
const snowLoads = ROSETTES.map((r) => ({ id: `snow-rosette-${r.stem.id.slice(-1)}`, kind: "snow", title: T(`Wet snow held in rosette ${r.stem.id.slice(-1).toUpperCase()}`, `Nasser Schnee in Rosette ${r.stem.id.slice(-1).toUpperCase()}`), bone: `head-${r.stem.id.slice(-1)}`, x: 0.1, y: 0, fxN: 0, fyN: r4(-Math.PI * 0.45 ** 2 * 0.05 * 350 * 9.81 * 0.5), source: "literature", reference: "en-1991-1-3" }));

const POSES = [
  { id: "calm", title: T("Calm sunny day", "Windstiller Sonnentag"), description: T("A still, warm day: the rosettes stand open, the young leaves erect and the old leaves reflexed; the stomata are open by day (C3) and the leaves transpire at full rate.", "Ein windstiller, warmer Tag: Die Rosetten stehen offen, die jungen Blätter aufrecht und die alten zurückgebogen; die Spaltöffnungen sind tagsüber offen (C3) und die Blätter transpirieren mit voller Rate."), bones: {}, ground: "authored", contacts: [], thermal: { regions: rDay, emitters: day }, loads: [] },
  { id: "night", title: T("Night, stomata closed", "Nacht, Spaltöffnungen geschlossen"), description: T("A clear night: the C3 stomata of the yucca are closed in the dark (unlike the CAM cacti beside it), transpiration falls to the cuticular rate.", "Eine klare Nacht: Die C3-Spaltöffnungen der Palmlilie sind im Dunkeln geschlossen (anders als bei den CAM-Kakteen daneben), die Transpiration fällt auf die kutikuläre Rate."), bones: {}, ground: "authored", contacts: [], thermal: { regions: rNight, emitters: night }, loads: [] },
  { id: "windy", title: T("Windy (12 m/s)", "Windig (12 m/s)"), description: T(`A strong breeze of 12 m/s at 10 m: drag coefficient 0.8 on 60 % of the projected leaf area of every rosette (the leaves shelter each other), ${Math.round(total(windyLoads))} N in total; the leaves bend 1-2 degrees downwind, the stems lean slightly.`, `Starker Wind von 12 m/s in 10 m Höhe: Widerstandsbeiwert 0,8 auf 60 % der projizierten Blattfläche jeder Rosette (die Blätter schützen sich gegenseitig), insgesamt ${Math.round(total(windyLoads))} N; die Blätter biegen sich 1-2 Grad nach Lee, die Stämme neigen sich leicht.`), bones: WINDY, ground: "authored", contacts: [], thermal: { regions: rDay, emitters: day }, loads: windyLoads },
  { id: "storm", title: T("Storm (26 m/s)", "Sturm (26 m/s)"), description: T(`A storm of 26 m/s: the leaves streamline and the drag coefficient falls to 0.45 (reconfiguration, Vogel 1989), ${Math.round(total(stormLoads))} N in total; young leaves bend 9 degrees, the stems 1.6 degrees.`, `Ein Sturm mit 26 m/s: Die Blätter legen sich an, der Widerstandsbeiwert sinkt auf 0,45 (Rekonfiguration, Vogel 1989), insgesamt ${Math.round(total(stormLoads))} N; junge Blätter biegen sich um 9 Grad, die Stämme um 1,6 Grad.`), bones: STORM, ground: "authored", contacts: [], thermal: { regions: rDay, emitters: day }, loads: stormLoads },
  { id: "snow", title: T("Wet snow in the rosettes", "Nasser Schnee in den Rosetten"), description: T("A rare wet snowfall (5 cm, 350 kg/m3) caught in the four rosettes: the spreading leaves are pressed down by up to 12 degrees; the plant tolerates short frosts to about -5 degC.", "Ein seltener nasser Schneefall (5 cm, 350 kg/m3) in den vier Rosetten: Die abstehenden Blätter werden um bis zu 12 Grad niedergedrückt; die Pflanze verträgt kurze Fröste bis etwa -5 °C."), bones: SNOW, ground: "authored", contacts: [], thermal: { regions: { "leaves-upper": { exposedFraction: 0.7, sunlitFraction: 0.05 }, "leaves-lower": { exposedFraction: 0.9, sunlitFraction: 0.02 }, "stem-bark": { exposedFraction: 1, sunlitFraction: 0.05 } }, emitters: { "transpiration-upper": 0.1, "transpiration-lower": 0.08 } }, loads: snowLoads },
];

const PERIOD = 1.2;
function swayClip(): Json {
  const n = 12, keys: Json[] = [];
  for (let i = 0; i <= n; i++) {
    const ph = (2 * Math.PI * (i % n)) / n;
    const leans = leanFor(0.22).map((l) => ({ id: l.id, deg: r4(l.deg * (1 + (l.id.startsWith("leaf") ? 0.7 : 0.4) * Math.sin(ph - (l.id.startsWith("leaf") ? 1 : 0)))) }));
    keys.push({ t: r4((PERIOD * i) / n), pose: "windy", ease: "smooth", bones: leanOffsets(RIG, {}, leans) });
  }
  return { id: "wind-sway", title: T("Wind sway", "Windschwingung"), description: T("A deterministic 1.2 s cycle in the 12 m/s breeze (about 0.8 Hz, the stems with their heavy rosettes, estimated): the stems nod about their lean and the leaves flutter a quarter period behind.", "Ein deterministischer Zyklus von 1,2 s im Wind von 12 m/s (etwa 0,8 Hz, die Stämme mit ihren schweren Rosetten, geschätzt): Die Stämme nicken um ihre Neigung, die Blätter flattern eine Viertelperiode später."), durationS: PERIOD, fps: 30, loop: true, rootMotion: "none", keys, contacts: [], phases: [], events: [], tracks: [] };
}
function gustClip(): Json {
  const ts = [0, 0.8, 1.5, 2.2, 2.9, 3.7, 4.5, 5.5], amp = [0, 0.35, 0.75, 1, 0.6, 0.8, 0.6, 0.5];
  const keys = ts.map((t, i) => ({ t, pose: i === 0 ? "calm" : "storm", ease: "smooth", bones: i === 0 ? {} : leanOffsets(RIG, {}, leanFor(amp[i]!)) }));
  const lt = [{ t: 0, v: 0.05 }, { t: 1.5, v: 0.6 }, { t: 2.2, v: 1 }, { t: 2.9, v: 0.55 }, { t: 3.7, v: 0.8 }, { t: 5.5, v: 0.5 }];
  return { id: "gust", title: T("Gust", "Böe"), description: T("A 5.5 s gust: drag rises to the storm value at 2.2 s, the rosettes lean out and the leaves stream, then the plant settles in the remaining wind.", "Eine Böe von 5,5 s: Der Winddruck steigt bis 2,2 s auf den Sturmwert, die Rosetten neigen sich und die Blätter strömen mit, dann beruhigt sich die Pflanze im verbleibenden Wind."), durationS: 5.5, fps: 30, loop: false, rootMotion: "none", keys, contacts: [], phases: [{ id: "gust", fromS: 0, toS: 2.9, title: T("Gust", "Böe") }, { id: "settle", fromS: 2.9, toS: 5.5, title: T("Settling", "Abklingen") }], events: [{ t: 2.2, kind: "motion", title: T("Peak lean", "Größte Neigung") }], tracks: stormLoads.map((l) => ({ target: `load:${l.id}`, ease: "smooth", keys: lt })) };
}
function diurnalClip(): Json {
  const C3: [number, number][] = [[0, 0.05], [5, 0.05], [7, 0.4], [9, 0.85], [11.5, 1], [14, 0.75], [16, 0.8], [18, 0.35], [20, 0.07], [24, 0.05]];
  const sun = (k: number) => [{ t: 0, v: 0 }, { t: 5.5, v: 0 }, { t: 7, v: 0.2 * k }, { t: 9, v: 0.4 * k }, { t: 12, v: 0.5 * k }, { t: 15, v: 0.42 * k }, { t: 17.5, v: 0.2 * k }, { t: 19, v: 0 }, { t: 24, v: 0 }];
  return {
    id: "diurnal-transpiration", title: T("Day of transpiration", "Transpirationstag"), description: T("A 24 h cycle compressed to 24 s (1 s = 1 h): the C3 stomata open in the morning, transpire most before noon with a small midday depression, and close at dusk, the opposite rhythm to the CAM cacti; leaf temperatures follow from the thermal solver.", "Ein 24-h-Zyklus auf 24 s gerafft (1 s = 1 h): Die C3-Spaltöffnungen öffnen sich am Morgen, transpirieren vor Mittag am stärksten mit einer kleinen Mittagsdepression und schließen sich in der Dämmerung, der umgekehrte Rhythmus der CAM-Kakteen; die Blatttemperaturen folgen aus dem thermischen Löser."),
    durationS: 24, fps: 10, loop: true, rootMotion: "none",
    keys: [{ t: 0, pose: "night", ease: "smooth" }, { t: 6, pose: "night", ease: "smooth" }, { t: 12, pose: "calm", ease: "smooth" }, { t: 18, pose: "calm", ease: "smooth" }, { t: 24, pose: "night", ease: "smooth" }],
    contacts: [], phases: [{ id: "night", fromS: 0, toS: 6, title: T("Night", "Nacht") }, { id: "morning", fromS: 6, toS: 12, title: T("Morning to noon", "Morgen bis Mittag") }, { id: "afternoon", fromS: 12, toS: 18, title: T("Afternoon", "Nachmittag") }, { id: "evening", fromS: 18, toS: 24, title: T("Evening and night", "Abend und Nacht") }],
    events: [{ t: 6, kind: "thermal", title: T("Sunrise", "Sonnenaufgang") }, { t: 12, kind: "thermal", title: T("Solar noon", "Sonnenhöchststand") }, { t: 18.5, kind: "thermal", title: T("Sunset", "Sonnenuntergang") }],
    tracks: [
      { target: "emitter:transpiration-upper", ease: "smooth", keys: C3.map(([t, v]) => ({ t, v })) }, { target: "emitter:transpiration-lower", ease: "smooth", keys: C3.map(([t, v]) => ({ t, v: r4(v * 0.7) })) },
      { target: "region:leaves-upper:sunlitFraction", ease: "smooth", keys: sun(1) }, { target: "region:leaves-lower:sunlitFraction", ease: "smooth", keys: sun(0.6) }, { target: "region:stem-bark:sunlitFraction", ease: "smooth", keys: sun(0.5) },
    ],
  };
}

type Parts = { drawn: Drawn[]; body: Json[]; thermal: Json[]; structure: Json[] };
const toneOf = (l: { el: number }) => r4(0.38 + 0.22 * Math.max(0, Math.min(1, (l.el + 32) / 116)));

function build(scale: Scale): Parts {
  const f = FEATURE[scale];
  const drawn: Drawn[] = [], body: Json[] = [], thermal: Json[] = [], structure: Json[] = [];
  const at = (bone: string, u: number) => world(B.frames[bone]!, [u, 0, 0]);
  const stemPart = (id: string, bone: string, prof: [number, number][], u: number) => {
    body.push(revolveSolid(id, bone, prof, { material: "yucca-stem", role: "body", tone: 0.3 }));
    thermal.push({ id: `th-${id}`, of: id, region: "stem-bark" });
    drawn.push({ shape: revolvedShape(id, bone, prof, { tone: 0.3, region: "stem-bark" }), at: at(bone, u) });
  };
  stemPart("foot", "foot", scale <= 100 ? BASE_PROFILE : [[0, 0.27], [0.15, 0.21], [0.5, 0.125]], 0.25);
  stemPart("trunk", "trunk", scale <= 100 ? TRUNK_PROFILE : [[-0.03, 0.125], [0.68, 0.108]], 0.3);
  for (const r of ROSETTES) {
    const s = r.stem, hid = `head-${s.id.slice(-1)}`;
    stemPart(s.id, s.id, scale <= 100 ? stemProfile(s) : [[-0.03, s.r0], [s.L + 0.02, s.r1]], s.L / 2);
    stemPart(hid, hid, scale <= 100 ? headProfile : [[-0.02, 0.078], [HEAD + 0.04, 0.04]], HEAD / 2);
    for (const l of r.leaves) {
      const coarse = scale === 500;
      const W = coarse ? Math.max(l.W, 0.095) : scale === 200 ? Math.max(l.W, 0.072) : l.W, th = coarse ? 0.085 : LEAF_T;
      const ringN = { 50: 16, 100: 10, 200: 7, 500: 5 }[scale];
      const ss = Array.from({ length: ringN + 1 }, (_, i) => 0.5 - 0.5 * Math.cos((Math.PI * i) / ringN));
      const ring = [...ss.map((t) => [t * l.L, coarse ? Math.max(hw(W, t), t < 0.97 ? th / 2 : 0.005) : hw(W, t)] as [number, number]), ...ss.slice(1, -1).reverse().map((t) => [t * l.L, -(coarse ? Math.max(hw(W, t), t < 0.97 ? th / 2 : 0.005) : hw(W, t))] as [number, number])];
      body.push({ ...plateSolid(l.id, l.id, ring, [-th / 2, th / 2], { material: "yucca-leaf", role: "body", tone: toneOf(l) }), ...(coarse ? {} : { sheet: true }) });
      const region = upper(l) ? "leaves-upper" : "leaves-lower";
      thermal.push({ id: `th-${l.id}`, of: l.id, region });
      drawn.push({ shape: plateShape(l.id, l.id, ring, [-th / 2, th / 2], undefined, { tone: toneOf(l), region }), at: at(l.id, l.L * 0.5) });
      if (scale === 50) {
        const mid = [0.06, 0.3, 0.55, 0.8, 0.93].map((t) => [t * l.L, 0] as [number, number]);
        drawn.push({ shape: plateShape(`${l.id}-midrib`, l.id, mid, [th / 2, th / 2 + 0.0005], undefined, { role: "detail", tone: 0.2 }, false), at: add(at(l.id, l.L * 0.5), mul(l.f.v, 0.005)), views: ["elevation", "front", "plan"] });
      }
      const mr = Math.max(f / 2 + 0.0006, 0.004);
      const st = { 50: [0.03, 0.3, 0.55, 0.8, 0.93], 100: [0.03, 0.4, 0.75, 0.92], 200: [0.03, 0.5, 0.9], 500: [0.03, 0.85] }[scale];
      const ms = st.map((t) => ({ at: [t * l.L, 0, 0] as V3, rx: coarse ? Math.min(mr, th / 2 - 0.001) : (th / 2) * 0.9, ry: coarse ? mr : Math.max(hw(l.W, t) * 0.9, 0.004) }));
      structure.push({ ...loftU(`m-${l.id}`, l.id, [{ at: toLocalHead(l), rx: coarse ? 0.038 : (th / 2) * 0.9, ry: coarse ? 0.038 : 0.03 }, ...ms], { material: "yucca-leaf", role: "body", tone: 0.5 }), ...(coarse ? {} : { sheet: true }), member: l.id, part: "member" });
    }
  }
  const pv = Math.max(0.004, f / 2 + 0.0008);
  for (const a of ANCHORS.filter((x) => x.kind === "emitter")) {
    const em = EMITTERS.find((e) => e.anchors.includes(a.id))!;
    const th = scale === 500 ? 0.085 : LEAF_T;
    thermal.push({ ...ellipsoidSolid(`th-${a.id}`, a.bone, [a.x, th / 2 + pv - 0.002, 0], [Math.max(0.05, 2.2 * pv), pv, Math.max(f / 2 + 0.001, Math.min(0.03, 2 * pv))], { material: "yucca-leaf", role: "body", tone: 0.8 }), region: em.region, tags: ["emitter", em.id] });
  }
  const rod = (id: string, bone: string, prof: [number, number][], member: string) => structure.push({ id, kind: "revolve", bone, material: "yucca-stem", role: "body", tone: 0.5, axis: [1, 0, 0], origin: [0, 0, 0], profile: prof.map(([u, r]) => [r5(u), r5(Math.max(r, f / 2 + 0.001))]), member, part: "member" });
  rod("m-foot", "foot", [[0.02, 0.15], [0.5, 0.08]], "foot");
  rod("m-trunk", "trunk", [[-0.03, 0.075], [FORK - 0.5, 0.065]], "trunk");
  for (const s of STEMS) { rod(`m-${s.id}`, s.id, [[-0.02, s.r0 * 0.6], [s.L + 0.02, s.r1 * 0.6]], s.id); rod(`m-head-${s.id.slice(-1)}`, `head-${s.id.slice(-1)}`, [[-0.02, 0.045], [HEAD, 0.04]], `head-${s.id.slice(-1)}`); }
  const plateT = Math.max(0.04, f + 0.001), padT = Math.max(0.02, f + 0.001);
  structure.push({ id: "m-root-plate", kind: "revolve", bone: "root-plate", material: "yucca-stem", role: "body", tone: 0.5, axis: [0, 1, 0], origin: [0, 0, 0], profile: [[0, 0.2], [r5(plateT), 0.2]], member: "root-plate", part: "member" });
  structure.push({ id: "s-base-support", kind: "revolve", bone: "root-plate", material: "yucca-stem", role: "ground", tone: 0.5, axis: [0, 1, 0], origin: [0, 0, 0], profile: [[0, 0.22], [r5(padT), 0.22]], member: "root-plate", part: "support", anchor: "base-support" });
  structure.push({ id: "k-root-plate", kind: "ellipsoid", bone: "root-plate", material: "yucca-stem", role: "body", tone: 0.5, radii: [0.1, r5(Math.max(0.03, f / 2 + 0.001)), 0.1], transform: { translation: [0, r5(Math.max(0.035, f / 2 + 0.002)), 0] }, member: "root-plate", part: "mass" });
  return { drawn, body, thermal, structure };
}
/** 🧵️ The point on the axis of the rosette head next to a leaf's insertion, in the leaf's frame: where its member starts, inside the head. */
function toLocalHead(l: typeof ALL[number]): V3 {
  const head = B.frames[`head-${l.id.split("-")[1]}`]!;
  const s = Math.max(0, Math.min(HEAD, dot(sub(l.f.o, head.o), head.u)));
  const p = add(head.o, mul(head.u, s));
  const d = sub(p, l.f.o);
  return [r5(dot(d, l.f.u)), r5(dot(d, l.f.v)), r5(dot(d, l.f.w))];
}
const PARTS = Object.fromEntries(SCALES.map((s) => [s, build(s)])) as Record<Scale, Parts>;
const count = (s: Scale, l: "body" | "thermal" | "structure") => PARTS[s][l].length;

const REP_TEXT: Record<Scale, Json> = {
  50: { silhouette: T("At 1:50 (1 m is 20 mm) every view is the true view of the 1:50 model: the swollen elephant foot, the trunk and four stems, and eighty strap-shaped leaves in four rosettes, each leaf its true planform in its own plane (face-on as a strap, edge-on as a line) with its midrib.", "Bei 1:50 (1 m sind 20 mm) ist jede Ansicht die wahre Ansicht des 1:50-Modells: der verdickte Elefantenfuß, Stamm und vier Stämme und achtzig riemenförmige Blätter in vier Rosetten, jedes Blatt sein wahrer Grundriss in seiner eigenen Ebene (von vorn als Riemen, von der Kante als Linie) mit Mittelrippe."), retained: T("Retained: every leaf with clasping base, parallel sides and soft point; the rosette heads; the swollen base.", "Beibehalten: jedes Blatt mit umfassender Basis, parallelen Rändern und weicher Spitze; die Rosettenköpfe; die verdickte Basis."), merged: T("Merged: the fine marginal serration into the outline.", "Zusammengefasst: die feine Randzähnung in den Umriss."), omitted: T("Omitted: the ringed leaf scars of the stems, dead leaves, the summer flower panicle (seasonal).", "Weggelassen: die geringelten Blattnarben der Stämme, abgestorbene Blätter, die sommerliche Blütenrispe (jahreszeitlich)."), thinElements: T("Edge-on leaves are 2.8 mm sheets drawn with the body pen; midribs 0.18 mm detail strokes.", "Blätter von der Kante sind 2,8 mm dünne Flächen, mit dem Körperstift gezeichnet; Mittelrippen Detailstriche von 0,18 mm."), hatching: T("None.", "Keine."), styles: T("Leaves lighter towards the young erect ones, stems dark.", "Blätter zu den jungen aufrechten hin heller, Stämme dunkel."), overlays: T("Upper and lower leaves and the stems carry their regions.", "Obere und untere Blätter und die Stämme tragen ihre Regionen.") },
  100: { silhouette: T("At 1:100 (1 m is 10 mm) the eighty leaves and the stems of the 1:100 model, outlines of ten stations, no midribs.", "Bei 1:100 (1 m sind 10 mm) die achtzig Blätter und die Stämme des 1:100-Modells, Umrisse aus zehn Stationen, ohne Mittelrippen."), retained: T("Retained: every leaf.", "Beibehalten: jedes Blatt."), merged: T("Merged: the outline into ten stations.", "Zusammengefasst: der Umriss in zehn Stationen."), omitted: T("Omitted: midribs.", "Weggelassen: Mittelrippen."), thinElements: T("Leaves as 2.8 mm sheets.", "Blätter als 2,8 mm dünne Flächen."), hatching: T("None.", "Keine."), styles: T("As at 1:50.", "Wie bei 1:50."), overlays: T("Three regions as at 1:50.", "Drei Regionen wie bei 1:50.") },
  200: { silhouette: T("At 1:200 (1 m is 5 mm) the rosettes of the 1:200 model: every leaf a sheet of seven stations, the foot and stems as plain revolved profiles.", "Bei 1:200 (1 m sind 5 mm) die Rosetten des 1:200-Modells: jedes Blatt eine Fläche aus sieben Stationen, Fuß und Stämme als einfache rotierte Profile."), retained: T("Retained: the eighty leaves as the sunburst of each rosette.", "Beibehalten: die achtzig Blätter als Strahlenkranz jeder Rosette."), merged: T("Merged: the base swelling into three stations.", "Zusammengefasst: die Fußverdickung in drei Stationen."), omitted: T("Omitted: midribs.", "Weggelassen: Mittelrippen."), thinElements: T("Leaves as 2.8 mm sheets, the narrowest widened from 58 to 72 mm so that a leaf seen obliquely still prints at 1:350.", "Blätter als 2,8 mm dünne Flächen, die schmalsten von 58 auf 72 mm verbreitert, damit ein schräg gesehenes Blatt bei 1:350 noch druckt."), hatching: T("None.", "Keine."), styles: T("As at 1:50, with a 0.2 mm body pen so that the thin leaf ends that other leaves overlap still print at 1:350.", "Wie bei 1:50, mit einem Körperstift von 0,2 mm, damit die dünnen Blattenden, die andere Blätter überdecken, bei 1:350 noch drucken."), overlays: T("Three regions as at 1:50.", "Drei Regionen wie bei 1:50.") },
  500: { silhouette: T("At 1:500 (1 m is 2 mm) the yucca of the 1:500 model: four starbursts of eighty leaves drawn as blades of 95 by 85 mm (so the most foreshortened young leaf still prints at 1:350) on the forked stem.", "Bei 1:500 (1 m sind 2 mm) die Palmlilie des 1:500-Modells: vier Strahlenkränze aus achtzig Blättern, als Klingen von 95 mal 85 mm gezeichnet (damit auch das am stärksten verkürzte junge Blatt bei 1:350 druckt), auf dem gegabelten Stamm."), retained: T("Retained: every leaf direction, so each rosette stays a starburst.", "Beibehalten: jede Blattrichtung, so bleibt jede Rosette ein Strahlenkranz."), merged: T("Merged: width and thickness of each leaf into a 95 by 85 mm blade.", "Zusammengefasst: Breite und Dicke jedes Blatts in eine Klinge von 95 mal 85 mm."), omitted: T("Omitted: every detail below 75 mm.", "Weggelassen: jedes Detail unter 75 mm."), thinElements: T("Leaves at the print minimum.", "Blätter mit dem Druckminimum."), hatching: T("None.", "Keine."), styles: T("As at 1:50, body pen 0.2 mm.", "Wie bei 1:50, Körperstift 0,2 mm."), overlays: T("Three regions as at 1:50.", "Drei Regionen wie bei 1:50.") },
};
const VOL_TEXT: Record<Scale, () => Json[]> = {
  50: () => [T(`Body at 1:50 (${count(50, "body")} solids): the elephant foot (0.54 m across at the ground), trunk, four stems and rosette heads as revolved stems; every leaf a declared thin sheet (2.8 mm, 57-68 mm wide), a flat plate of the strap planform with 32 outline points (clasping base, slight narrowing, parallel sides, soft point), the same prism its three drawings show.`, `Körper bei 1:50 (${count(50, "body")} Körper): Elefantenfuß (0,54 m am Boden), Stamm, vier Stämme und Rosettenköpfe als rotierte Stämme; jedes Blatt eine erklärte dünne Fläche (2,8 mm, 57-68 mm breit), eine ebene Platte mit dem Riemengrundriss aus 32 Umrisspunkten (umfassende Basis, leichte Verengung, parallele Ränder, weiche Spitze), dasselbe Prisma, das seine drei Zeichnungen zeigen.`), T(`Thermal at 1:50 (${count(50, "thermal")} solids): by reference, fourteen outlets on the leaf faces.`, `Thermik bei 1:50 (${count(50, "thermal")} Körper): per Verweis, vierzehn Auslässe auf den Blattflächen.`), T(`Structure at 1:50 (${count(50, "structure")} solids): a midrib member in every leaf from the axis of its rosette head, solid members of foot, trunk, stems and heads, root plate, support, mass.`, `Tragwerk bei 1:50 (${count(50, "structure")} Körper): ein Mittelrippenstab in jedem Blatt von der Achse seines Rosettenkopfs aus, Vollstäbe von Fuß, Stamm, Stämmen und Köpfen, Wurzelteller, Auflager, Masse.`)],
  100: () => [T(`Body at 1:100 (${count(100, "body")} solids): as 1:50 with 20 outline points per leaf.`, `Körper bei 1:100 (${count(100, "body")} Körper): wie 1:50 mit 20 Umrisspunkten je Blatt.`), T(`Thermal at 1:100 (${count(100, "thermal")} solids): by reference, fourteen outlets.`, `Thermik bei 1:100 (${count(100, "thermal")} Körper): per Verweis, vierzehn Auslässe.`), T(`Structure at 1:100 (${count(100, "structure")} solids): as 1:50.`, `Tragwerk bei 1:100 (${count(100, "structure")} Körper): wie 1:50.`)],
  200: () => [T(`Body at 1:200 (${count(200, "body")} solids): 14 outline points per leaf (at least 72 mm wide), the foot and stems with two or three stations.`, `Körper bei 1:200 (${count(200, "body")} Körper): 14 Umrisspunkte je Blatt (mindestens 72 mm breit), Fuß und Stämme mit zwei oder drei Stationen.`), T(`Thermal at 1:200 (${count(200, "thermal")} solids): by reference, fourteen outlets.`, `Thermik bei 1:200 (${count(200, "thermal")} Körper): per Verweis, vierzehn Auslässe.`), T(`Structure at 1:200 (${count(200, "structure")} solids): members of at least 31 mm.`, `Tragwerk bei 1:200 (${count(200, "structure")} Körper): Stäbe von mindestens 31 mm.`)],
  500: () => [T(`Body at 1:500 (${count(500, "body")} solids): every leaf a plate of 95 by 85 mm with 10 outline points (no longer a sheet), the foot and stems plain.`, `Körper bei 1:500 (${count(500, "body")} Körper): jedes Blatt eine Platte von 95 mal 85 mm mit 10 Umrisspunkten (keine dünne Fläche mehr), Fuß und Stämme einfach.`), T(`Thermal at 1:500 (${count(500, "thermal")} solids): by reference, fourteen outlets.`, `Thermik bei 1:500 (${count(500, "thermal")} Körper): per Verweis, vierzehn Auslässe.`), T(`Structure at 1:500 (${count(500, "structure")} solids): leaf members of 76 mm inside the blades, three sections each.`, `Tragwerk bei 1:500 (${count(500, "structure")} Körper): Blattstäbe von 76 mm in den Klingen, je drei Schnitte.`)],
};

function samples(): V3[] {
  const pts: V3[] = [];
  for (const l of ALL) for (let k = 0; k <= 30; k++) { const t = k / 30; for (const sd of [-1, 1]) pts.push(world(l.f, [t * l.L, 0, sd * hw(l.W, t)])); }
  for (let a = 0; a < 24; a++) pts.push([0.27 * Math.cos(a * Math.PI / 12), 0, 0.27 * Math.sin(a * Math.PI / 12)]);
  return pts;
}
function symbol(): Json {
  const proj = { elevation: (p: V3) => [p[0], p[1]], front: (p: V3) => [p[2], p[1]], plan: (p: V3) => [p[0], p[2]] } as const;
  const geo: Json = {};
  for (const view of ["elevation", "front", "plan"] as const) {
    const bars: [number, number, number, number, number, number][] = ALL.map((l) => { const a = proj[view](l.f.o), b = proj[view](world(l.f, [l.L, 0, 0])); return [a[0]!, a[1]!, b[0]!, b[1]!, 0.028, 0.006]; });
    if (view !== "plan") { bars.push([0, 0.27, 0, FORK, 0.27, 0.12]); for (const r of ROSETTES) { const a = proj[view](B.frames[r.stem.id]!.o), b = proj[view](r.head.o); bars.push([a[0]!, a[1]!, b[0]!, b[1]!, 0.08, 0.08]); } }
    else bars.push([0, 0, 0.001, 0, 0.27, 0.27]);
    const ring = barOutline(bars, 0.05).sort((a, b) => b.length - a.length)[0]!;
    geo[view] = [{ id: "extent", bone: "root-plate", role: "body", z: 2, filled: false, tone: 0.5, kind: "poly", points: ring.map(([x, y]) => [x, view === "plan" ? y : Math.max(0, y)]), closed: true }];
  }
  geo.plan.push({ id: "foot", bone: "root-plate", role: "detail", z: 3, filled: false, tone: 0.3, kind: "ellipse", cx: 0, cy: 0, rx: 0.27, ry: 0.27 });
  return { id: "yucca-symbol", title: T("Planting-plan sign with the true extent of the four rosettes", "Pflanzplanzeichen mit der wahren Ausdehnung der vier Rosetten"), description: T("A conventional sign for planting plans: the starburst outline of the four rosettes on the forked stem at their true extent in every view, with the elephant foot in plan; a separate symbol, never used silently for a pictorial scale.", "Ein konventionelles Zeichen für Pflanzpläne: der Strahlenumriss der vier Rosetten auf dem gegabelten Stamm in wahrer Ausdehnung in jeder Ansicht, im Grundriss mit dem Elefantenfuß; ein eigenes Symbol, nie stillschweigend für einen Bildmaßstab verwendet."), semantics: "true-extent", units: "model-m", denominators: [500, 200, 100], pens: { body: 0.18, detail: 0.13, texture: null, glazing: null, ground: 0.25 }, hatches: [], geometry: geo, dimensionalNote: T("Outline at the true extent of the default pose.", "Umriss in der wahren Ausdehnung der Standardpose.") };
}

export function asset(): Json {
  const pts = samples();
  const xs = pts.map((p) => p[0]), ys = pts.map((p) => p[1]), zs = pts.map((p) => p[2]);
  const mass = RIG.bones.reduce((s: number, x: Json) => s + (x.mass?.kg ?? 0), 0);
  const representations: Json = {}, volumes: Json = {};
  for (const s of SCALES) {
    representations[String(s)] = { status: "authored", denominator: s, validRange: RANGES[s], pens: s >= 200 ? { ...PENS[s], body: 0.2 } : PENS[s], hatches: [], geometry: views(PARTS[s].drawn), decisions: REP_TEXT[s] };
    volumes[String(s)] = { status: "authored", denominator: s, body: PARTS[s].body, thermal: PARTS[s].thermal, structure: PARTS[s].structure, decisions: VOL_TEXT[s]() };
  }
  const H = Math.max(...ys);
  return {
    id: ID, category: "vegetation", rig: RIG, anchors: ANCHORS,
    title: T("Yucca", "Palmlilie"),
    summary: T(`A spineless yucca (Yucca gigantea, syn. Y. elephantipes) of ${H.toFixed(1)} m: a swollen elephant-foot base 0.54 m across stores water, the trunk forks at 1.15 m into four stems of different height, and each carries a rosette of twenty stiff strap-shaped leaves (50-74 cm by 6-7 cm) in the 137.5 degree spiral, old leaves reflexed and young leaves erect. Unlike the cacti it is a C3 plant: its stomata open by day.`, `Eine Riesen-Palmlilie (Yucca gigantea, syn. Y. elephantipes) von ${H.toFixed(1).replace(".", ",")} m: ein verdickter Elefantenfuß von 0,54 m speichert Wasser, der Stamm gabelt sich bei 1,15 m in vier verschieden hohe Stämme, und jeder trägt eine Rosette aus zwanzig steifen, riemenförmigen Blättern (50-74 cm mal 6-7 cm) in der 137,5-Grad-Spirale, alte Blätter zurückgebogen, junge aufrecht. Anders als die Kakteen ist sie eine C3-Pflanze: Ihre Spaltöffnungen öffnen sich tagsüber.`),
    tags: ["yucca", "succulent", "desert", "c3", "rosette", "elephant-foot", "transpiration"],
    provenance: { author: "Semio staffage plant collection wave, worker PW-11", method: "handcrafted", created: "2026-11-22", references: Object.values(REF), notes: T("Stems (azimuth, elevation, length, radii), the elephant-foot profile and the leaf law (elevation, length, width by spiral index; strap planform) are stated by hand after Irish and Irish (2000); the leaves follow the 137.5 degree spiral of the agave generators. Y. gigantea is treated as a C3 plant (the genus ranges from C3 to CAM, Heyduk et al. 2016); conductance, heat capacity and stiffness are authored estimates. The flower panicle is not modelled.", "Stämme (Richtung, Neigung, Länge, Radien), das Profil des Elefantenfußes und das Blattgesetz (Neigung, Länge, Breite nach Spiralindex; Riemengrundriss) sind von Hand nach Irish und Irish (2000) festgelegt; die Blätter folgen der 137,5-Grad-Spirale der Agaven-Generatoren. Y. gigantea wird als C3-Pflanze behandelt (die Gattung reicht von C3 bis CAM, Heyduk u. a. 2016); Leitfähigkeit, Wärmekapazität und Steifigkeit sind Autorenschätzungen. Die Blütenrispe ist nicht modelliert.") },
    dimensions: { heightM: r4(H), widthM: r4(Math.max(...xs) - Math.min(...xs)), depthM: r4(Math.max(...zs) - Math.min(...zs)), massKg: Math.round(mass) },
    materials: MATERIALS, representations, symbols: [symbol()], poses: POSES, clips: [swayClip(), gustClip(), diurnalClip()],
    thermal: { environment: ENV, passengers: [], operating: [], regions: REGIONS, emitters: EMITTERS, assumptions: [
      T(`Leaf regions use both faces of the eighty leaves as exchange area (upper ${r4(AREA.upper)} m2, lower ${r4(AREA.lower)} m2); C3: stomata open by day; maximum conductance 4 mm/s for the young and 2.5 mm/s for the old leaves (authored estimates for a sclerophyllous leaf, Jones 2014).`, `Die Blattregionen verwenden beide Flächen der achtzig Blätter als Austauschfläche (oben ${r4(AREA.upper)} m2, unten ${r4(AREA.lower)} m2); C3: Spaltöffnungen tagsüber offen; maximale Leitfähigkeit 4 mm/s für junge und 2,5 mm/s für alte Blätter (Autorenschätzungen für ein hartlaubiges Blatt, Jones 2014).`),
      T("The stem surface is passive; its area is the revolved outer surface less about 10 % hidden in the forks and heads. Transpiration leaves through eight outlets on upper and six on lower leaves, normal to the face.", "Die Stammoberfläche ist passiv; ihre Fläche ist die rotierte Außenfläche abzüglich etwa 10 %, die in Gabeln und Köpfen verborgen sind. Die Transpiration tritt durch acht Auslässe an oberen und sechs an unteren Blättern aus, normal zur Fläche."),
    ] },
    structure: { gravityMS2: 9.80665, dynamics: "quasi-static", supports: ["base-support"], assumptions: [
      T("One rigid body per evaluated pose, fixed at the root plate; wind drag acts at each rosette head (cd 0.8 in a breeze, 0.45 in a storm after reconfiguration, Vogel 1989; power-law profile after EN 1991-1-4) and on foot and trunk as a line load; snow in the rosettes; gravity on every bone mass.", "Ein starrer Körper je ausgewerteter Pose, am Wurzelteller eingespannt; Winddruck wirkt an jedem Rosettenkopf (cd 0,8 bei Brise, 0,45 im Sturm nach Rekonfiguration, Vogel 1989; Potenzprofil nach EN 1991-1-4) und als Linienlast auf Fuß und Stamm; Schnee in den Rosetten; Schwerkraft auf jede Knochenmasse."),
      T("Sections: solid fibrous stems (E 1.5 GPa, authored estimate); leaves are carried by their midribs and are not dimensioned.", "Querschnitte: volle faserige Stämme (E 1,5 GPa, Autorenschätzung); Blätter werden von ihren Mittelrippen getragen und nicht bemessen."),
    ], clinical: false },
    volumes, presentation: { defaultPose: "calm", defaultClip: "wind-sway", defaultScale: 100 }, exports: EXPORTS, completion: "authored",
  };
}
export const DISTINCTION = T("A tree-like spineless yucca of 3 m: an elephant-foot base, a trunk forked into four stems, and four rosettes of twenty thin strap leaves each on their own bones; a C3 plant that transpires by day, the counterpart of the CAM cacti.", "Eine baumartige Riesen-Palmlilie von 3 m: ein Elefantenfuß, ein in vier Stämme gegabelter Stamm und vier Rosetten aus je zwanzig dünnen Riemenblättern an eigenen Knochen; eine C3-Pflanze, die tagsüber transpiriert, das Gegenstück zu den CAM-Kakteen.");
export const REPORT = { rigError, area: AREA, leaves: ALL.length };
