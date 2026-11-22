/** 🍐️ PW-11: the Indian fig opuntia (Opuntia ficus-indica) stated by hand: a short woody trunk, four lignified old stem segments and forty flat obovate cladodes (pads) in four generations, each pad on its own bone at the margin of the pad that bore it, the youngest bright and thin, ripe fruits (tunas) on the upper margins of the third-generation pads; CAM physiology (stomata open at night). Builds the whole `semio.staffage/v2` asset.
 * @see ../../📋️brief.md
 * @see ./kit.ts */
import { FEATURE, PENS, RANGES, RigBuilder, SCALES, T, add, barOutline, dir, toLocal, documentOf, dot, drag, ellipsoidSolid, leanOffsets, loftU, mul, r4, r5, revolveSolid, revolvedArea, revolvedShape, plateShape, plateSolid, sub, table, unit, views, world, EXPORTS, type Drawn, type Frame, type Json, type Scale, type V3 } from "./kit.ts";

export const ID = "prickly-pear";

const REF = {
  nobel88: { id: "nobel-1988", citation: "Nobel, P. S. (1988). Environmental Biology of Agaves and Cacti. Cambridge University Press. Crassulacean acid metabolism, nocturnal stomatal opening, cladode orientation and interception of radiation by Opuntia, cladode water content." },
  nobel02: { id: "nobel-2002", citation: "Nobel, P. S. (ed.) (2002). Cacti: Biology and Uses. University of California Press. Opuntia ficus-indica: shrub or small tree 3-5 m, cladodes 30-60 cm long and 20-40 cm wide, fruits (tunas) 5-10 cm on the margins of one-year-old cladodes." },
  fao: { id: "fao-2017", citation: "Inglese, P., Mondragon, C., Nefzaoui, A. and Saenz, C. (eds.) (2017). Crop Ecology, Cultivation and Uses of Cactus Pear. FAO and ICARDA, Rome. Cladode thickness 1.5-3 cm, fresh weight about 1 kg per mature cladode, 90-93 % water, fruit 100-200 g, cold damage below about -5 degC." },
  monteith: { id: "monteith-unsworth-2013", citation: "Monteith, J. L. and Unsworth, M. H. (2013). Principles of Environmental Physics, 4th ed., Academic Press. Penman-Monteith leaf transpiration, boundary layers of flat plates." },
  en: { id: "en-1991-1-4", citation: "EN 1991-1-4:2005 Eurocode 1, Actions on structures, Part 1-4: Wind actions. Dynamic pressure q = 0.5 rho v^2 with air density 1.25 kg/m3, force F = cf q A; mean wind profile." },
  en13: { id: "en-1991-1-3", citation: "EN 1991-1-3:2003 Eurocode 1, Actions on structures, Part 1-3: Snow loads. Density of wet snow (about 300-400 kg/m3)." },
  hoerner: { id: "hoerner-1965", citation: "Hoerner, S. F. (1965). Fluid-Dynamic Drag. Hoerner Fluid Dynamics, Bricktown NJ. Drag coefficient of a flat plate normal to the flow about 1.1-1.2." },
};

type Gen = 1 | 2 | 3 | 4;
type Pad = { id: string; parent: string; az: number; el: number; face: number; fruit?: boolean };
const SIZE: Record<Gen, { L: number; W: number; T: number }> = { 1: { L: 0.48, W: 0.31, T: 0.03 }, 2: { L: 0.42, W: 0.27, T: 0.026 }, 3: { L: 0.36, W: 0.23, T: 0.022 }, 4: { L: 0.24, W: 0.15, T: 0.016 } };

/** 📋️ The forty cladodes as stated by hand: parent, direction (azimuth counter-clockwise from ahead, elevation) and the azimuth the flat face looks towards; cladodes of one branch fan out round the plant, the outer ones lean out and down, the inner ones stand up, and most faces look east or west (here ahead or behind) as Nobel (1988) found for the terminal cladodes; fruit on eight third-generation pads. */
const PADS: Pad[] = [
  { id: "a1", parent: "stem-1", az: 20, el: 70, face: 110 }, { id: "a11", parent: "a1", az: 5, el: 52, face: 95 }, { id: "a111", parent: "a11", az: -5, el: 36, face: 85, fruit: true }, { id: "a112", parent: "a11", az: 32, el: 70, face: 120, fruit: true }, { id: "a1121", parent: "a112", az: 40, el: 80, face: 130 },
  { id: "a12", parent: "a1", az: 48, el: 72, face: 140 }, { id: "a121", parent: "a12", az: 62, el: 58, face: 150, fruit: true },
  { id: "a2", parent: "stem-1", az: -12, el: 44, face: 80 }, { id: "a21", parent: "a2", az: -22, el: 26, face: 70 }, { id: "a211", parent: "a21", az: -28, el: 12, face: 65 }, { id: "a22", parent: "a2", az: 2, el: 64, face: 92 },
  { id: "b1", parent: "stem-2", az: 130, el: 72, face: 40 }, { id: "b11", parent: "b1", az: 152, el: 54, face: 62 }, { id: "b111", parent: "b11", az: 166, el: 38, face: 76, fruit: true }, { id: "b112", parent: "b11", az: 140, el: 74, face: 50 }, { id: "b1121", parent: "b112", az: 128, el: 84, face: 38 },
  { id: "b12", parent: "b1", az: 108, el: 66, face: 18 }, { id: "b121", parent: "b12", az: 98, el: 54, face: 8, fruit: true },
  { id: "b2", parent: "stem-2", az: 162, el: 42, face: 72 }, { id: "b21", parent: "b2", az: 176, el: 24, face: 86 }, { id: "b22", parent: "b2", az: 150, el: 60, face: 60 },
  { id: "c1", parent: "stem-3", az: 244, el: 70, face: 154 }, { id: "c11", parent: "c1", az: 228, el: 54, face: 138 }, { id: "c111", parent: "c11", az: 218, el: 38, face: 128, fruit: true }, { id: "c12", parent: "c1", az: 266, el: 72, face: 176 }, { id: "c121", parent: "c12", az: 276, el: 60, face: 186, fruit: true }, { id: "c1211", parent: "c121", az: 282, el: 80, face: 192 },
  { id: "c2", parent: "stem-3", az: 272, el: 40, face: 182 }, { id: "c21", parent: "c2", az: 286, el: 22, face: 196 }, { id: "c22", parent: "c2", az: 258, el: 58, face: 168 },
  { id: "d1", parent: "stem-4", az: 0, el: 86, face: 90 }, { id: "d11", parent: "d1", az: 70, el: 76, face: 160 }, { id: "d111", parent: "d11", az: 80, el: 80, face: 170, fruit: true }, { id: "d1111", parent: "d111", az: 90, el: 85, face: 180 },
  { id: "d12", parent: "d1", az: 200, el: 78, face: 110 }, { id: "d121", parent: "d12", az: 210, el: 82, face: 120 },
  { id: "d13", parent: "d1", az: -60, el: 74, face: 30 }, { id: "d131", parent: "d13", az: -70, el: 80, face: 20 },
  { id: "e1", parent: "stem-1", az: 70, el: 58, face: 160 }, { id: "e11", parent: "e1", az: 85, el: 40, face: 175 },
];
const genOf = (p: Pad): Gen => (p.parent.startsWith("stem") ? 1 : (Math.min(4, genOf(PADS.find((q) => q.id === p.parent)!) + 1) as Gen));
const pb = (id: string) => `pad-${id}`;

/** 🍃️ Half width of the obovate cladode as a share of its greatest width along the pad from its narrow base: widest at two thirds, round at the top. */
const SHAPE: [number, number][] = [[0, 0.2], [0.06, 0.42], [0.15, 0.62], [0.3, 0.83], [0.45, 0.95], [0.62, 1], [0.76, 0.96], [0.86, 0.84], [0.93, 0.64], [0.975, 0.38], [1, 0]];
const hw = (W: number, s: number) => (W / 2) * table(SHAPE, s);
const thick = (T0: number, s: number) => T0 * table([[0, 1.25], [0.15, 1.1], [0.5, 1], [0.85, 0.9], [1, 0.6]], s);
const INSET = 0.035;

const TRUNK_H = 0.75;
/** 🧭️ Turn of the whole plant about its trunk so that it stands centred across the side view (its lateral extent symmetric about the trunk). */
const ROT = -44;
const STEMS = [{ id: "stem-1", az: 15, el: 50, L: 0.5 }, { id: "stem-2", az: 135, el: 52, L: 0.5 }, { id: "stem-3", az: 250, el: 48, L: 0.5 }, { id: "stem-4", az: 0, el: 88, L: 0.55 }];
const TRUNK_PROFILE: [number, number][] = [[0, 0.125], [0.1, 0.112], [0.4, 0.1], [0.7, 0.096], [0.8, 0.09]];
const stemR = (u: number, L: number) => table([[0, 0.085], [L * 0.5, 0.075], [L, 0.07]], u);
const PAD_DENSITY = 950, WOOD_DENSITY = 750;

const B = new RigBuilder({ id: "root-plate", title: T("Root plate: the shallow fibrous roots in the dry soil", "Wurzelteller: die flachen Faserwurzeln im trockenen Boden"), lengthM: 0.4, joint: { type: "fixed" } });
B.add("trunk", "root-plate", T("Woody trunk (lignified oldest segments)", "Holziger Stamm (verholzte älteste Glieder)"), [0, 0, 0], [0, 1, 0], [-1, 0, 0], TRUNK_H, { type: "revolute", minDeg: -3, maxDeg: 3 }, {
  mass: { kg: r4(revolvedVolume(TRUNK_PROFILE) * WOOD_DENSITY), comX: 0.33, comY: 0, distribution: "uniform", source: "authored-estimate", reference: "fao-2017" },
  section: { areaM2: r5(Math.PI * 0.1 ** 2), secondMomentM4: Number((Math.PI * 0.1 ** 4 / 4).toPrecision(4)), extremeFibreM: 0.1, material: "opuntia-wood", title: T("Solid woody section, d = 20 cm", "Holziger Vollquerschnitt, d = 20 cm") },
});
function revolvedVolume(p: readonly [number, number][]): number { let v = 0; for (let i = 1; i < p.length; i++) v += Math.PI * (p[i]![0] - p[i - 1]![0]) * (p[i]![1] ** 2 + p[i]![1] * p[i - 1]![1] + p[i - 1]![1] ** 2) / 3; return v; }

const stemFrames: Record<string, { f: Frame; L: number }> = {};
for (const s of STEMS) {
  const o: V3 = add([0, TRUNK_H - 0.06, 0], mul(dir(s.az + ROT, 0), s.id === "stem-4" ? 0 : 0.03));
  const u = dir(s.az + ROT, s.el);
  const f = B.add(s.id, "trunk", s.id === "stem-4" ? T("Lignified old segment, central", "Verholztes altes Glied, mittig") : T(`Lignified old segment towards ${s.az} degrees`, `Verholztes altes Glied in Richtung ${s.az} Grad`), o, u, [0, 1, 0], s.L, { type: "revolute", minDeg: -6, maxDeg: 6 }, {
    mass: { kg: r4(revolvedVolume([[0, stemR(0, s.L)], [s.L / 2, stemR(s.L / 2, s.L)], [s.L, stemR(s.L, s.L)]]) * WOOD_DENSITY), comX: r4(s.L / 2), comY: 0, distribution: "uniform", source: "authored-estimate", reference: "fao-2017" },
    section: { areaM2: r5(Math.PI * 0.075 ** 2), secondMomentM4: Number((Math.PI * 0.075 ** 4 / 4).toPrecision(4)), extremeFibreM: 0.075, material: "opuntia-wood", title: T("Lignified terete segment, d = 15 cm", "Verholztes rundes Glied, d = 15 cm") },
  });
  stemFrames[s.id] = { f, L: s.L };
}

type PadGeo = { p: Pad; g: Gen; f: Frame; L: number; W: number; T: number };
const GEO: Record<string, PadGeo> = {};
const tipOf = (id: string): { f: Frame; L: number; pad?: PadGeo } => (id.startsWith("stem") ? stemFrames[id]! : { f: GEO[id]!.f, L: GEO[id]!.L, pad: GEO[id] });
for (const p of PADS) {
  const g = genOf(p), { L, W, T: T0 } = SIZE[g];
  const d = dir(p.az + ROT, p.el);
  const face = add(dir(p.face + ROT, 0), [0, 0.15 + 0.6 * (1 - p.el / 90), 0]);
  const v = unit(sub(face, mul(d, dot(face, d))));
  const par = tipOf(p.parent);
  let attach: V3;
  if (par.pad === undefined) attach = world(par.f, [par.L, 0, 0]);
  else {
    let best = -Infinity;
    attach = par.f.o;
    for (let k = 0; k <= 80; k++) {
      const s = 0.4 + (0.6 * k) / 80;
      for (const side of [1, -1]) {
        const q = world(par.f, [s * par.L, 0, side * hw(par.pad.W, s) * 0.92]);
        const c = world(par.f, [par.L * 0.55, 0, 0]);
        const score = dot(unit(sub(q, c)), d);
        if (score > best) { best = score; attach = q; }
      }
    }
  }
  const o = sub(attach, mul(d, INSET));
  const kg = (() => { let m = 0; for (let k = 0; k < 30; k++) { const s = (k + 0.5) / 30; m += 2 * hw(W, s) * thick(T0, s) * (Math.PI / 4) * (L / 30); } return m * PAD_DENSITY; })();
  const gname = { 1: T("first generation, mature", "erste Generation, ausgewachsen"), 2: T("second generation, mature", "zweite Generation, ausgewachsen"), 3: T("third generation, one year old", "dritte Generation, einjährig"), 4: T("young, this season", "jung, diese Saison") }[g];
  const f = B.add(pb(p.id), p.parent.startsWith("stem") ? p.parent : pb(p.parent), T(`Cladode ${p.id.toUpperCase()} (${gname.en}${p.fruit ? ", bearing fruit" : ""})`, `Flachspross ${p.id.toUpperCase()} (${gname.de}${p.fruit ? ", mit Früchten" : ""})`), o, d, v, L, { type: "revolute", minDeg: -25, maxDeg: 25 }, {
    mass: { kg: r4(kg), comX: r4(L * 0.55), comY: 0, distribution: "uniform", source: "authored-estimate", reference: "fao-2017" },
    section: { areaM2: r5(W * 0.35 * T0 * Math.PI / 4), secondMomentM4: Number((W * 0.35 * T0 ** 3 / 12).toPrecision(4)), extremeFibreM: r5(T0 / 2), material: "opuntia-cladode", title: T(`Cladode base, ${Math.round(W * 35)} by ${Math.round(T0 * 1000)} mm`, `Spross-Basis, ${Math.round(W * 35)} mal ${Math.round(T0 * 1000)} mm`) },
  });
  GEO[p.id] = { p, g, f, L, W, T: T0 };
}

/** 🍐️ The fruits (tunas): three on the upper margin of each fruiting pad, egg-shaped, 7.5 cm long and 5 cm across, pointing outwards and upwards. */
type Fruit = { id: string; pad: string; f: Frame };
const FRUITS: Fruit[] = [];
const FRUIT_L = 0.075, FRUIT_R = 0.026;
const FRUIT_PROFILE: [number, number][] = [[0, 0.008], [0.01, 0.016], [0.025, 0.024], [0.042, 0.026], [0.058, 0.023], [0.069, 0.016], [0.075, 0.006]];
for (const g of Object.values(GEO).filter((x) => x.p.fruit)) {
  [[0.8, 1], [0.97, 0.25], [0.8, -1]].forEach(([s, side], i) => {
    const sm = s!, m = side!;
    const edge: V3 = [sm * g.L, 0, m * hw(g.W, sm) * (Math.abs(m) === 1 ? 0.9 : 1)];
    const p = world(g.f, edge);
    const out = unit(add(mul(g.f.w, m), mul(g.f.u, Math.abs(m) === 1 ? 0.8 : 2.5)));
    const id = `fruit-${g.p.id}-${i + 1}`;
    const f = B.add(id, pb(g.p.id), T(`Fruit ${i + 1} on cladode ${g.p.id.toUpperCase()}`, `Frucht ${i + 1} an Flachspross ${g.p.id.toUpperCase()}`), sub(p, mul(out, 0.012)), out, g.f.v, FRUIT_L, { type: "fixed" }, {
      mass: { kg: r4(revolvedVolume(FRUIT_PROFILE) * 1050), comX: 0.04, comY: 0, distribution: "uniform", source: "authored-estimate", reference: "fao-2017" },
    });
    FRUITS.push({ id, pad: g.p.id, f });
  });
}
B.bones[0].mass = { kg: 12, comX: 0, comY: -0.15, distribution: "point", source: "authored-estimate", reference: "fao-2017" };
const rigError = B.verify();
const RIG = B.rig();

const YOUNG = (g: PadGeo) => g.g >= 3;
const regionOf = (g: PadGeo) => (YOUNG(g) ? "young-cladodes" : "mature-cladodes");

const padArea = (g: PadGeo) => { let a = 0; for (let k = 0; k < 40; k++) { const s = (k + 0.5) / 40; a += 2 * 2 * hw(g.W, s) * (g.L / 40); } return a; };
const AREA = {
  young: Object.values(GEO).filter(YOUNG).reduce((s, g) => s + padArea(g), 0),
  mature: Object.values(GEO).filter((g) => !YOUNG(g)).reduce((s, g) => s + padArea(g), 0),
  wood: 0.87 * (revolvedArea(TRUNK_PROFILE) + STEMS.reduce((s, x) => s + 2 * Math.PI * 0.077 * x.L, 0)),
  fruit: FRUITS.length * (revolvedArea(FRUIT_PROFILE) + Math.PI * 0.008 ** 2),
};

const MATERIALS = [
  { id: "opuntia-cladode", title: T("Succulent cladode: waxy green cuticle over water-storing tissue (about 90 % water)", "Sukkulenter Flachspross: wachsige grüne Kutikula über wasserspeicherndem Gewebe (etwa 90 % Wasser)"), emissivity: 0.97, solarAbsorptivity: 0.6, densityKgM3: PAD_DENSITY, specificHeatJKgK: 3900, conductivityWmK: 0.5, elasticModulusPa: 30000000, strengthPa: 1000000, source: "authored-estimate", reference: "fao-2017" },
  { id: "opuntia-wood", title: T("Lignified trunk and old segments with grey bark", "Verholzter Stamm und alte Glieder mit grauer Rinde"), emissivity: 0.93, solarAbsorptivity: 0.7, densityKgM3: WOOD_DENSITY, conductivityWmK: 0.18, specificHeatJKgK: 2200, elasticModulusPa: 3000000000, strengthPa: 25000000, source: "authored-estimate", reference: "nobel-2002" },
  { id: "opuntia-fruit", title: T("Ripe cactus pear (tuna): orange-red rind with glochids over juicy pulp", "Reife Kaktusfeige: orangerote Schale mit Glochiden über saftigem Fruchtfleisch"), emissivity: 0.95, solarAbsorptivity: 0.7, densityKgM3: 1050, specificHeatJKgK: 3700, conductivityWmK: 0.5, source: "authored-estimate", reference: "fao-2017" },
];
const REGIONS = [
  { id: "young-cladodes", title: T("Young and one-year-old cladodes (bright green, CAM)", "Junge und einjährige Flachsprosse (hellgrün, CAM)"), kind: "stem", model: "leaf", material: "opuntia-cladode", areaM2: r4(AREA.young), areaBasis: "exchange", characteristicLengthM: 0.25, geometry: "vertical-plate", skyViewFactor: 0.6, layers: [], heatCapacityJM2K: 45000, leaf: { stomatalConductanceMaxMS: 0.0015, leafAreaIndex: 1.2, photosynthesis: "cam" }, source: "authored-estimate", reference: "nobel-1988" },
  { id: "mature-cladodes", title: T("Mature cladodes of the first two generations (CAM)", "Ausgewachsene Flachsprosse der ersten zwei Generationen (CAM)"), kind: "stem", model: "leaf", material: "opuntia-cladode", areaM2: r4(AREA.mature), areaBasis: "exchange", characteristicLengthM: 0.36, geometry: "vertical-plate", skyViewFactor: 0.45, layers: [], heatCapacityJM2K: 55000, leaf: { stomatalConductanceMaxMS: 0.0012, leafAreaIndex: 1.6, photosynthesis: "cam" }, source: "authored-estimate", reference: "nobel-1988" },
  { id: "woody-trunk", title: T("Woody trunk and lignified old segments", "Holziger Stamm und verholzte alte Glieder"), kind: "bark", model: "passive", material: "opuntia-wood", areaM2: r4(AREA.wood), characteristicLengthM: 0.18, geometry: "cylinder", skyViewFactor: 0.3, layers: [], heatCapacityJM2K: 60000, source: "authored-estimate", reference: "nobel-2002" },
  { id: "fruit", title: T("Ripe fruits (tunas)", "Reife Früchte (Kaktusfeigen)"), kind: "surface", model: "passive", material: "opuntia-fruit", areaM2: r4(AREA.fruit), characteristicLengthM: 0.05, geometry: "sphere", skyViewFactor: 0.6, layers: [], heatCapacityJM2K: 30000, source: "authored-estimate", reference: "fao-2017" },
];
const YOUNG_OUT = ["a111", "a1121", "b111", "b1121", "c121", "d111", "d1111"];
const MATURE_OUT = ["a1", "a2", "b1", "b2", "c1", "d1", "d12"];
const outletOf = (id: string, n: number, group: string) => { const g = GEO[id]!; return { id: `tp-${group}-${n}`, bone: pb(id), x: r4(g.L * 0.55), y: r4(g.T / 2 + 0.003), kind: "emitter", title: T(`Transpiration of cladode ${id.toUpperCase()}`, `Transpiration von Flachspross ${id.toUpperCase()}`) }; };
const ANCHORS = [
  { id: "base-support", bone: "root-plate", x: 0, y: 0, kind: "support", title: T("Root plate in the soil (fixed)", "Wurzelteller im Boden (eingespannt)"), support: "fixed", normalDeg: 90, footprintLengthM: 0.8, footprintWidthM: 0.8, frictionCoefficient: 0.6 },
  ...YOUNG_OUT.map((id, i) => outletOf(id, i + 1, "young")), ...MATURE_OUT.map((id, i) => outletOf(id, i + 1, "mature")),
];
const EMITTERS = [
  { id: "transpiration-young", kind: "transpiration", title: T("Transpiration of the young cladodes (at night, CAM; seven outlets)", "Transpiration der jungen Flachsprosse (nachts, CAM; sieben Auslässe)"), anchor: "tp-young-1", anchors: YOUNG_OUT.map((_, i) => `tp-young-${i + 1}`), directionDeg: 90, diameterM: 0.25, region: "young-cladodes", source: "authored-estimate", reference: "monteith-unsworth-2013" },
  { id: "transpiration-mature", kind: "transpiration", title: T("Transpiration of the mature cladodes (at night, CAM; seven outlets)", "Transpiration der ausgewachsenen Flachsprosse (nachts, CAM; sieben Auslässe)"), anchor: "tp-mature-1", anchors: MATURE_OUT.map((_, i) => `tp-mature-${i + 1}`), directionDeg: 90, diameterM: 0.3, region: "mature-cladodes", source: "authored-estimate", reference: "monteith-unsworth-2013" },
];
const ENV = { airTempC: 33, meanRadiantTempC: 40, skyTempC: 20, groundTempC: 52, relativeHumidity: 0.3, windMS: 1.5, windDirectionDeg: 0, solarWM2: 880, diffuseFraction: 0.12, pressurePa: 101325 };

const v10 = (y: number, v: number) => v * Math.pow(Math.max(y, 1) / 10, 0.22);
function windLoads(v: number, label: Json): Json[] {
  const loads: Json[] = [];
  for (const g of Object.values(GEO)) {
    const A = padArea(g) / 2 * Math.abs(dot(g.f.v, [1, 0, 0])) + g.L * g.T * 0.3;
    const c = world(g.f, [g.L * 0.55, 0, 0]);
    loads.push({ id: `wind-${g.p.id}`, kind: "wind", title: T(`Wind drag on cladode ${g.p.id.toUpperCase()}, ${label.en}`, `Winddruck auf Flachspross ${g.p.id.toUpperCase()}, ${label.de}`), bone: pb(g.p.id), x: r4(g.L * 0.55), y: 0, fxN: r4(drag(1.1, A, v10(c[1], v))), fyN: 0, source: "literature", reference: "en-1991-1-4" });
  }
  loads.push({ id: "wind-trunk", kind: "wind", title: T(`Wind drag on the trunk, ${label.en}`, `Winddruck auf den Stamm, ${label.de}`), fxN: 0, fyN: 0, distributed: { bone: "trunk", fromM: 0, toM: TRUNK_H, wxNPerM: r4(drag(1.0, 0.2, v10(0.4, v))), wyNPerM: 0 }, source: "literature", reference: "en-1991-1-4" });
  return loads;
}
const total = (l: Json[]) => l.reduce((s, x) => s + (x.distributed ? x.distributed.wxNPerM * x.distributed.toM : x.fxN), 0);

/** 🌬️ Lean about world z per bone: the narrow pad joints yield, the woody trunk and segments hardly. */
const leanFor = (k: number) => [{ id: "trunk", deg: -0.3 * k }, ...STEMS.map((s) => ({ id: s.id, deg: -0.8 * k })), ...PADS.map((p) => ({ id: pb(p.id), deg: r4(-(genOf(p) <= 2 ? 2.2 : 3.2) * k) }))];
const WINDY = leanOffsets(RIG, {}, leanFor(0.25));
const STORM = leanOffsets(RIG, {}, leanFor(1));

const rDay = { "young-cladodes": { exposedFraction: 1, sunlitFraction: 0.55 }, "mature-cladodes": { exposedFraction: 1, sunlitFraction: 0.4 }, "woody-trunk": { exposedFraction: 1, sunlitFraction: 0.15 }, fruit: { exposedFraction: 1, sunlitFraction: 0.6 } };
const rNight = { "young-cladodes": { exposedFraction: 1, sunlitFraction: 0 }, "mature-cladodes": { exposedFraction: 1, sunlitFraction: 0 }, "woody-trunk": { exposedFraction: 1, sunlitFraction: 0 }, fruit: { exposedFraction: 1, sunlitFraction: 0 } };
const shut = { "transpiration-young": 0.05, "transpiration-mature": 0.05 }, open = { "transpiration-young": 1, "transpiration-mature": 1 };
const windyLoads = windLoads(10, T("10 m/s", "10 m/s")), stormLoads = windLoads(25, T("25 m/s", "25 m/s"));
const snowLoads = Object.values(GEO).filter((g) => g.g >= 3).map((g) => ({ id: `snow-${g.p.id}`, kind: "snow", title: T(`Wet snow on the upper margin of cladode ${g.p.id.toUpperCase()}`, `Nasser Schnee auf dem oberen Rand von Flachspross ${g.p.id.toUpperCase()}`), bone: pb(g.p.id), x: r4(g.L * 0.85), y: 0, fxN: 0, fyN: r4(-(g.W * 0.6 * 0.05 * 0.06 + Math.abs(g.f.v[1]) * padArea(g) / 2 * 0.03) * 350 * 9.81), source: "literature", reference: "en-1991-1-3" }));

const POSES = [
  { id: "fruiting", title: T("Late summer with ripe fruit", "Spätsommer mit reifen Früchten"), description: T(`A calm, hot late-summer afternoon: ${FRUITS.length} ripe tunas stand on the upper margins of the one-year-old cladodes, the stomata are shut (CAM) and the flat pads, most facing east and west, catch the low morning and evening sun rather than the noon sun (Nobel 1988).`, `Ein windstiller, heißer Spätsommernachmittag: ${FRUITS.length} reife Kaktusfeigen stehen auf den oberen Rändern der einjährigen Flachsprosse, die Spaltöffnungen sind geschlossen (CAM) und die flachen Sprosse, meist nach Osten und Westen gerichtet, fangen eher die tiefe Morgen- und Abendsonne als die Mittagssonne (Nobel 1988).`), bones: {}, ground: "authored", contacts: [], thermal: { regions: rDay, emitters: shut }, loads: [] },
  { id: "night-stomata-open", title: T("Night, stomata open (CAM)", "Nacht, Spaltöffnungen offen (CAM)"), description: T("A warm night: the stomata of every cladode are open and take up carbon dioxide while the vapour deficit is small; nearly all water is lost now.", "Eine warme Nacht: Die Spaltöffnungen aller Flachsprosse sind offen und nehmen Kohlendioxid auf, solange das Sättigungsdefizit klein ist; fast alles Wasser geht jetzt verloren."), bones: {}, ground: "authored", contacts: [], thermal: { regions: rNight, emitters: open }, loads: [] },
  { id: "windy", title: T("Windy (10 m/s)", "Windig (10 m/s)"), description: T(`A fresh wind of 10 m/s at 10 m (power-law profile, exponent 0.22): drag coefficient 1.1 of a flat plate on the projected area of every cladode (Hoerner 1965), ${Math.round(total(windyLoads))} N in total; each narrow pad joint yields by about half a degree to a degree downwind.`, `Ein frischer Wind von 10 m/s in 10 m Höhe (Potenzprofil, Exponent 0,22): Widerstandsbeiwert 1,1 einer ebenen Platte auf der projizierten Fläche jedes Flachsprosses (Hoerner 1965), insgesamt ${Math.round(total(windyLoads))} N; jedes schmale Sprossgelenk gibt um einen halben bis einen Grad nach Lee nach.`), bones: WINDY, ground: "authored", contacts: [], thermal: { regions: rDay, emitters: shut }, loads: windyLoads },
  { id: "storm", title: T("Storm (25 m/s)", "Sturm (25 m/s)"), description: T(`A storm of 25 m/s: ${Math.round(total(stormLoads))} N of drag; the pad joints yield by 2 to 3 degrees each, so the young tips swing out by up to 12 degrees. Cladodes break off at these joints in real storms and root as cuttings, which is not modelled.`, `Ein Sturm mit 25 m/s: ${Math.round(total(stormLoads))} N Winddruck; die Sprossgelenke geben je um 2 bis 3 Grad nach, die jungen Spitzen schwingen bis 12 Grad aus. In echten Stürmen brechen Flachsprosse an diesen Gelenken ab und wurzeln als Stecklinge; das ist nicht modelliert.`), bones: STORM, ground: "authored", contacts: [], thermal: { regions: rDay, emitters: shut }, loads: stormLoads },
  { id: "snow", title: T("Wet snow", "Nasser Schnee"), description: T("A rare Mediterranean snowfall: wet snow (350 kg/m3) lies a few centimetres deep on the upper margins and inclined faces of the young cladodes; the plant tolerates it, frost below about -5 degC would damage the young pads (FAO 2017).", "Ein seltener Schneefall am Mittelmeer: Nasser Schnee (350 kg/m3) liegt einige Zentimeter hoch auf den oberen Rändern und geneigten Flächen der jungen Flachsprosse; die Pflanze verträgt ihn, Frost unter etwa -5 °C würde die jungen Sprosse schädigen (FAO 2017)."), bones: {}, ground: "authored", contacts: [], thermal: { regions: { "young-cladodes": { exposedFraction: 0.8, sunlitFraction: 0.1 }, "mature-cladodes": { exposedFraction: 0.9, sunlitFraction: 0.05 }, "woody-trunk": { exposedFraction: 1, sunlitFraction: 0.05 }, fruit: { exposedFraction: 0.8, sunlitFraction: 0.05 } }, emitters: { "transpiration-young": 0.1, "transpiration-mature": 0.1 } }, loads: snowLoads },
];

const PERIOD = 0.8;
function swayClip(): Json {
  const n = 12, keys: Json[] = [];
  for (let i = 0; i <= n; i++) {
    const ph = (2 * Math.PI * (i % n)) / n;
    const leans = leanFor(1).map((l) => { const young = l.id.startsWith("pad-") && genOf(PADS.find((p) => pb(p.id) === l.id)!) >= 3; return { id: l.id, deg: r4(l.deg * 0.25 * (1 + (young ? 0.8 : 0.5) * Math.sin(ph - (young ? 0.9 : 0)))) }; });
    keys.push({ t: r4((PERIOD * i) / n), pose: "windy", ease: "smooth", bones: leanOffsets(RIG, {}, leans) });
  }
  return { id: "wind-sway", title: T("Wind sway", "Windschwingung"), description: T("A deterministic 0.8 s cycle in the 10 m/s wind (about 1.25 Hz, a 3-5 kg chain of cladodes on narrow joints, estimated): the mature pads nod about their lean and the young tips follow a little later with a larger swing.", "Ein deterministischer Zyklus von 0,8 s im Wind von 10 m/s (etwa 1,25 Hz, eine Kette von 3-5 kg Flachsprossen an schmalen Gelenken, geschätzt): Die ausgewachsenen Sprosse nicken um ihre Neigung, die jungen Spitzen folgen etwas später mit größerem Ausschlag."), durationS: PERIOD, fps: 30, loop: true, rootMotion: "none", keys, contacts: [], phases: [], events: [], tracks: [] };
}
function gustClip(): Json {
  const ts = [0, 0.7, 1.4, 2.0, 2.6, 3.3, 4.1, 5.0], amp = [0, 0.4, 0.8, 1, 0.6, 0.82, 0.62, 0.55];
  const keys = ts.map((t, i) => ({ t, pose: i === 0 ? "fruiting" : "storm", ease: "smooth", bones: i === 0 ? {} : leanOffsets(RIG, {}, leanFor(amp[i]!)) }));
  const lt = [{ t: 0, v: 0.05 }, { t: 1.4, v: 0.65 }, { t: 2, v: 1 }, { t: 2.6, v: 0.55 }, { t: 3.3, v: 0.8 }, { t: 5, v: 0.55 }];
  return { id: "gust", title: T("Gust", "Böe"), description: T("A 5 s gust: drag rises to the storm value at 2 s, the cladode chains lean out and ring at about 1.25 Hz, then settle in the remaining wind.", "Eine Böe von 5 s: Der Winddruck steigt bis 2 s auf den Sturmwert, die Sprossketten neigen sich und schwingen mit etwa 1,25 Hz nach, dann beruhigen sie sich im verbleibenden Wind."), durationS: 5, fps: 30, loop: false, rootMotion: "none", keys, contacts: [], phases: [{ id: "gust", fromS: 0, toS: 2.6, title: T("Gust", "Böe") }, { id: "settle", fromS: 2.6, toS: 5, title: T("Settling", "Abklingen") }], events: [{ t: 2, kind: "motion", title: T("Peak lean", "Größte Neigung") }], tracks: stormLoads.map((l) => ({ target: `load:${l.id}`, ease: "smooth", keys: lt })) };
}
const CAM: [number, number][] = [[0, 0.95], [3, 1], [5, 0.85], [7, 0.4], [9, 0.06], [12, 0.04], [15, 0.04], [17, 0.15], [19, 0.5], [21, 0.85], [24, 0.95]];
function diurnalClip(): Json {
  const sun = (k: number) => [{ t: 0, v: 0 }, { t: 5.5, v: 0 }, { t: 7, v: 0.45 * k }, { t: 9, v: 0.55 * k }, { t: 12, v: 0.4 * k }, { t: 15, v: 0.55 * k }, { t: 17.5, v: 0.45 * k }, { t: 19, v: 0 }, { t: 24, v: 0 }];
  return {
    id: "diurnal-transpiration", title: T("Day of CAM transpiration", "CAM-Transpirationstag"), description: T("A 24 h cycle compressed to 24 s (1 s = 1 h): the CAM stomata open after sunset and transpire through the night, close in the early morning and stay shut in the heat of the day. Because most cladodes face east and west, their sunlit share peaks in the morning and the late afternoon and dips at noon (Nobel 1988).", "Ein 24-h-Zyklus auf 24 s gerafft (1 s = 1 h): Die CAM-Spaltöffnungen öffnen sich nach Sonnenuntergang und transpirieren durch die Nacht, schließen sich am frühen Morgen und bleiben in der Tageshitze zu. Weil die meisten Flachsprosse nach Osten und Westen schauen, ist ihr besonnter Anteil morgens und am späten Nachmittag am größten und mittags kleiner (Nobel 1988)."),
    durationS: 24, fps: 10, loop: true, rootMotion: "none",
    keys: [{ t: 0, pose: "night-stomata-open", ease: "smooth" }, { t: 6, pose: "night-stomata-open", ease: "smooth" }, { t: 12, pose: "fruiting", ease: "smooth" }, { t: 18, pose: "fruiting", ease: "smooth" }, { t: 24, pose: "night-stomata-open", ease: "smooth" }],
    contacts: [], phases: [{ id: "night", fromS: 0, toS: 6, title: T("Night, stomata open", "Nacht, Spaltöffnungen offen") }, { id: "morning", fromS: 6, toS: 12, title: T("Morning sun on the faces", "Morgensonne auf den Flächen") }, { id: "afternoon", fromS: 12, toS: 18, title: T("Afternoon, stomata shut", "Nachmittag, Spaltöffnungen zu") }, { id: "evening", fromS: 18, toS: 24, title: T("Evening, stomata opening", "Abend, Spaltöffnungen öffnen") }],
    events: [{ t: 6, kind: "thermal", title: T("Sunrise", "Sonnenaufgang") }, { t: 12, kind: "thermal", title: T("Solar noon", "Sonnenhöchststand") }, { t: 18.5, kind: "thermal", title: T("Sunset", "Sonnenuntergang") }],
    tracks: [
      ...["transpiration-young", "transpiration-mature"].map((id) => ({ target: `emitter:${id}`, ease: "smooth", keys: CAM.map(([t, v]) => ({ t, v })) })),
      { target: "region:young-cladodes:sunlitFraction", ease: "smooth", keys: sun(1) }, { target: "region:mature-cladodes:sunlitFraction", ease: "smooth", keys: sun(0.75) },
      { target: "region:woody-trunk:sunlitFraction", ease: "smooth", keys: sun(0.3) }, { target: "region:fruit:sunlitFraction", ease: "smooth", keys: sun(1.1) },
    ],
  };
}

type Parts = { drawn: Drawn[]; body: Json[]; thermal: Json[]; structure: Json[] };
const toneOf = (g: PadGeo) => ({ 1: 0.36, 2: 0.42, 3: 0.5, 4: 0.6 })[g.g];
/** 🫓️ Thickness of a pad at a scale: true at 1:50 and 1:100, at least the smallest feature plus a margin above (the drawn plate is the same thickness). */
const padT = (g: PadGeo, scale: Scale) => (scale <= 100 ? g.T : Math.max(g.T, FEATURE[scale] + (scale === 200 ? 0.004 : 0.005)));

/** 🧵️ The point on the fibrous core of the parent (pad axis or stem tip) next to where a cladode springs, so its core member starts in the core that carries it. */
function coreOf(g: PadGeo): V3 {
  if (g.p.parent.startsWith("stem")) return g.f.o;
  const par = GEO[g.p.parent]!;
  const a = add(g.f.o, mul(g.f.u, INSET));
  const s = Math.max(0.06, Math.min(0.88, dot(sub(a, par.f.o), par.f.u) / par.L));
  return add(par.f.o, mul(par.f.u, s * par.L));
}

function build(scale: Scale): Parts {
  const f = FEATURE[scale];
  const drawn: Drawn[] = [], body: Json[] = [], thermal: Json[] = [], structure: Json[] = [];
  const at = (bone: string, u: number) => world(B.frames[bone]!, [u, 0, 0]);
  const trunkProfile: [number, number][] = scale <= 100 ? TRUNK_PROFILE : [[0, 0.122], [0.4, 0.1], [0.8, 0.09]];
  body.push(revolveSolid("trunk", "trunk", trunkProfile, { material: "opuntia-wood", role: "body", tone: 0.28 }));
  thermal.push({ id: "th-trunk", of: "trunk", region: "woody-trunk" });
  drawn.push({ shape: revolvedShape("trunk", "trunk", trunkProfile, { tone: 0.28, region: "woody-trunk" }), at: at("trunk", 0.4) });
  for (const s of STEMS) {
    const prof: [number, number][] = scale <= 100 ? [[-0.03, stemR(0, s.L)], [s.L * 0.5, stemR(s.L / 2, s.L)], [s.L + 0.04, stemR(s.L, s.L)]] : [[-0.03, stemR(0, s.L)], [s.L + 0.04, stemR(s.L, s.L)]];
    body.push(revolveSolid(s.id, s.id, prof, { material: "opuntia-wood", role: "body", tone: 0.3 }));
    thermal.push({ id: `th-${s.id}`, of: s.id, region: "woody-trunk" });
    drawn.push({ shape: revolvedShape(s.id, s.id, prof, { tone: 0.3, region: "woody-trunk" }), at: at(s.id, s.L / 2) });
  }
  for (const g of Object.values(GEO)) {
    const bone = pb(g.p.id), Tt = padT(g, scale), k = Tt / g.T;
    const ringN = scale === 50 ? 24 : scale === 100 ? 16 : scale === 200 ? 12 : 10;
    const ss = Array.from({ length: ringN + 1 }, (_, i) => 0.5 - 0.5 * Math.cos((Math.PI * i) / ringN));
    const ring = [...ss.map((s) => [s * g.L, hw(g.W, s)] as [number, number]), ...ss.slice(1, -1).reverse().map((s) => [s * g.L, -hw(g.W, s)] as [number, number])];
    const half = (thick(g.T, 0.5) * k) / 2;
    body.push(plateSolid(bone, bone, ring, [-half, half], { material: "opuntia-cladode", role: "body", tone: toneOf(g) }));
    thermal.push({ id: `th-${bone}`, of: bone, region: regionOf(g) });
    drawn.push({ shape: plateShape(bone, bone, ring, [-half, half], undefined, { tone: toneOf(g), region: regionOf(g) }), at: at(bone, g.L * 0.5) });
    const nm = { 50: 6, 100: 5, 200: 4, 500: 3 }[scale];
    const ms = Array.from({ length: nm }, (_, i) => { const s = 0.06 + (0.82 * i) / (nm - 1); return { at: [s * g.L, 0, 0] as V3, rx: Math.max(thick(g.T, s) * k * 0.36, f / 2 + 0.0006), ry: Math.max(hw(g.W, s) * 0.3, f / 2 + 0.0006) }; });
    const core = toLocal(g.f, coreOf(g));
    const r0 = Math.max(0.01, f / 2 + 0.0006);
    ms.unshift({ at: core, rx: r0, ry: r0 });
    structure.push({ ...loftU(`m-${bone}`, bone, ms, { material: "opuntia-cladode", role: "body", tone: 0.5 }), member: bone, part: "member" });
    if (scale === 50) structure.push({ ...ellipsoidSolid(`j-${bone}`, bone, [INSET, 0, 0], [0.03, Math.max(g.T * 0.45, 0.006), 0.04], { material: "opuntia-cladode", role: "body", tone: 0.5 }), member: bone, part: "joint" });
  }
  {
    const prof: [number, number][] = scale === 50 ? FRUIT_PROFILE : scale === 100 ? [[0, 0.01], [0.02, 0.023], [0.045, 0.026], [0.065, 0.019], [0.075, 0.006]] : scale === 200 ? [[0, 0.012], [0.035, 0.026], [0.075, 0.016]] : [[0, 0.03], [0.04, 0.039], [0.08, 0.03]];
    for (const fr of FRUITS) {
      body.push(revolveSolid(fr.id, fr.id, prof, { material: "opuntia-fruit", role: "body", tone: 0.22 }));
      thermal.push({ id: `th-${fr.id}`, of: fr.id, region: "fruit" });
      drawn.push({ shape: revolvedShape(fr.id, fr.id, prof, { tone: 0.22, region: "fruit" }), at: at(fr.id, 0.04), rank: 1 });
    }
  }
  const pv = Math.max(0.004, f / 2 + 0.0008);
  for (const a of ANCHORS.filter((x) => x.kind === "emitter")) {
    const em = EMITTERS.find((e) => e.anchors.includes(a.id))!;
    const g = GEO[a.bone.slice(4)]!;
    const half = (thick(g.T, 0.55) * padT(g, scale) / g.T) / 2;
    thermal.push({ ...ellipsoidSolid(`th-${a.id}`, a.bone, [a.x, half + pv - 0.002, 0], [Math.max(0.05, 2.2 * pv), pv, Math.max(0.05, 2 * pv)], { material: "opuntia-cladode", role: "body", tone: 0.8 }), region: em.region, tags: ["emitter", em.id] });
  }
  structure.push({ id: "m-trunk", kind: "revolve", bone: "trunk", material: "opuntia-wood", role: "body", tone: 0.5, axis: [1, 0, 0], origin: [0, 0, 0], profile: [[0.02, 0.07], [TRUNK_H, 0.06]], member: "trunk", part: "member" });
  for (const s of STEMS) structure.push({ id: `m-${s.id}`, kind: "revolve", bone: s.id, material: "opuntia-wood", role: "body", tone: 0.5, axis: [1, 0, 0], origin: [0, 0, 0], profile: [[-0.02, 0.045], [s.L + 0.03, Math.max(0.04, f / 2 + 0.001)]], member: s.id, part: "member" });
  const plateT = Math.max(0.04, f + 0.001), padTh = Math.max(0.02, f + 0.001);
  structure.push({ id: "m-root-plate", kind: "revolve", bone: "root-plate", material: "opuntia-wood", role: "body", tone: 0.5, axis: [0, 1, 0], origin: [0, 0, 0], profile: [[0, 0.1], [r5(plateT), 0.1]], member: "root-plate", part: "member" });
  structure.push({ id: "s-base-support", kind: "revolve", bone: "root-plate", material: "opuntia-wood", role: "ground", tone: 0.5, axis: [0, 1, 0], origin: [0, 0, 0], profile: [[0, 0.115], [r5(padTh), 0.115]], member: "root-plate", part: "support", anchor: "base-support" });
  structure.push({ id: "k-root-plate", kind: "ellipsoid", bone: "root-plate", material: "opuntia-wood", role: "body", tone: 0.5, radii: [0.06, r5(Math.max(0.03, f / 2 + 0.001)), 0.06], transform: { translation: [0, r5(Math.max(0.035, f / 2 + 0.002)), 0] }, member: "root-plate", part: "mass" });
  return { drawn, body, thermal, structure };
}
const PARTS = Object.fromEntries(SCALES.map((s) => [s, build(s)])) as Record<Scale, Parts>;
const count = (s: Scale, l: "body" | "thermal" | "structure") => PARTS[s][l].length;

const REP_TEXT: Record<Scale, Json> = {
  50: { silhouette: T("At 1:50 (1 m is 20 mm) every view is the true view of the 1:50 model: forty obovate cladodes, each drawn as its true outline in its own plane (face-on as a pad, edge-on as a narrow band), on four lignified segments over the woody trunk, with the ripe fruits on the upper margins.", "Bei 1:50 (1 m sind 20 mm) ist jede Ansicht die wahre Ansicht des 1:50-Modells: vierzig verkehrt eiförmige Flachsprosse, jeder als wahrer Umriss in seiner eigenen Ebene gezeichnet (von vorn als Fläche, von der Kante als schmales Band), auf vier verholzten Gliedern über dem holzigen Stamm, mit den reifen Früchten auf den oberen Rändern."), retained: T("Retained: every cladode with its narrow base and round top, every fruit.", "Beibehalten: jeder Flachspross mit schmaler Basis und rundem Kopf, jede Frucht."), merged: T("Merged: the slightly wavy margin into a smooth outline of 24 stations.", "Zusammengefasst: der leicht wellige Rand in einen glatten Umriss aus 24 Stationen."), omitted: T("Omitted: the areoles with their glochid tufts (4-5 mm; on pads seen obliquely a mark of them falls below the 0.2 mm print minimum), the few spines of this nearly spineless cultivar, flowers (spring, not shown with ripe fruit).", "Weggelassen: die Areolen mit ihren Glochidenbüscheln (4-5 mm; auf schräg gesehenen Sprossen fällt ihre Marke unter das Druckminimum von 0,2 mm), die wenigen Dornen dieser fast dornenlosen Sorte, Blüten (Frühling, nicht zusammen mit reifen Früchten)."), thinElements: T("Edge-on cladodes are drawn at their true 16-30 mm thickness (0.3-0.6 mm on paper).", "Flachsprosse von der Kante sind in ihrer wahren Dicke von 16-30 mm gezeichnet (0,3-0,6 mm auf dem Papier)."), hatching: T("None.", "Keine."), styles: T("Pads lighter with youth, the woody parts dark, fruits darkest.", "Sprosse mit der Jugend heller, die holzigen Teile dunkel, Früchte am dunkelsten."), overlays: T("Young and mature cladodes, wood and fruit carry their regions.", "Junge und ausgewachsene Flachsprosse, Holz und Früchte tragen ihre Regionen.") },
  100: { silhouette: T("At 1:100 (1 m is 10 mm) the forty cladodes, the woody frame and the fruits of the 1:100 model, without areole marks.", "Bei 1:100 (1 m sind 10 mm) die vierzig Flachsprosse, das holzige Gerüst und die Früchte des 1:100-Modells, ohne Areolenmarken."), retained: T("Retained: every cladode and fruit.", "Beibehalten: jeder Flachspross und jede Frucht."), merged: T("Merged: the outline into 16 stations per side.", "Zusammengefasst: der Umriss in 16 Stationen je Seite."), omitted: T("Omitted: areoles.", "Weggelassen: Areolen."), thinElements: T("Edge-on cladodes at their true thickness.", "Flachsprosse von der Kante in wahrer Dicke."), hatching: T("None.", "Keine."), styles: T("As at 1:50.", "Wie bei 1:50."), overlays: T("Four regions as at 1:50.", "Vier Regionen wie bei 1:50.") },
  200: { silhouette: T("At 1:200 (1 m is 5 mm) the cladodes of the 1:200 model with 12-point outlines; edge-on pads are drawn 34 mm thick, the print minimum.", "Bei 1:200 (1 m sind 5 mm) die Flachsprosse des 1:200-Modells mit Umrissen aus 12 Punkten; Sprosse von der Kante sind 34 mm dick gezeichnet, das Druckminimum."), retained: T("Retained: every cladode and fruit.", "Beibehalten: jeder Flachspross und jede Frucht."), merged: T("Merged: the thickness of the pads into the print minimum.", "Zusammengefasst: die Dicke der Sprosse ins Druckminimum."), omitted: T("Omitted: areoles.", "Weggelassen: Areolen."), thinElements: T("Pad edges at 34 mm.", "Sprosskanten mit 34 mm."), hatching: T("None.", "Keine."), styles: T("As at 1:50.", "Wie bei 1:50."), overlays: T("Four regions as at 1:50.", "Vier Regionen wie bei 1:50.") },
  500: { silhouette: T("At 1:500 (1 m is 2 mm) the clump of the 1:500 model: forty pads of 10-point outlines at 80 mm thickness, every fruit at 78 mm; the fan of flat pads still reads.", "Bei 1:500 (1 m sind 2 mm) der Busch des 1:500-Modells: vierzig Sprosse mit Umrissen aus 10 Punkten und 80 mm Dicke, jede Frucht mit 78 mm; der Fächer flacher Sprosse bleibt lesbar."), retained: T("Retained: the pads and the woody frame.", "Beibehalten: die Sprosse und das holzige Gerüst."), merged: T("Merged: the egg shape of each fruit into a three-station body of 78 mm.", "Zusammengefasst: die Eiform jeder Frucht in einen Körper aus drei Stationen von 78 mm."), omitted: T("Omitted: every detail below 75 mm.", "Weggelassen: jedes Detail unter 75 mm."), thinElements: T("Pads and fruits at the print minimum (80 and 78 mm).", "Sprosse mit dem Druckminimum von 80 mm."), hatching: T("None.", "Keine."), styles: T("As at 1:50.", "Wie bei 1:50."), overlays: T("Four regions as at 1:50.", "Vier Regionen wie bei 1:50.") },
};
const VOL_TEXT: Record<Scale, () => Json[]> = {
  50: () => [T(`Body at 1:50 (${count(50, "body")} solids): the woody trunk and four lignified segments as revolved stems; every cladode a flat plate of its obovate planform (narrow base, widest at 62 %, round top, 48 outline points) and of its mid thickness of 16-38 mm, the same prism its three drawings show, so the slots between neighbouring pads are alike in body and drawing; ${FRUITS.length} egg-shaped fruits.`, `Körper bei 1:50 (${count(50, "body")} Körper): der holzige Stamm und vier verholzte Glieder als rotierte Stämme; jeder Flachspross eine ebene Platte mit seinem verkehrt eiförmigen Grundriss (schmale Basis, breiteste Stelle bei 62 %, runder Kopf, 48 Umrisspunkte) und seiner mittleren Dicke von 16-38 mm, dasselbe Prisma, das seine drei Zeichnungen zeigen, so dass die Schlitze zwischen benachbarten Sprossen in Körper und Zeichnung gleich sind; ${FRUITS.length} eiförmige Früchte.`), T(`Thermal at 1:50 (${count(50, "thermal")} solids): by reference, fourteen outlets on the faces.`, `Thermik bei 1:50 (${count(50, "thermal")} Körper): per Verweis, vierzehn Auslässe auf den Flächen.`), T(`Structure at 1:50 (${count(50, "structure")} solids): a fibrous core member in every cladode, joints at the narrow pad bases, the woody members of trunk and segments, root plate, support, mass.`, `Tragwerk bei 1:50 (${count(50, "structure")} Körper): ein faseriger Kernstab in jedem Flachspross, Gelenke an den schmalen Sprossbasen, die Holzstäbe von Stamm und Gliedern, Wurzelteller, Auflager, Masse.`)],
  100: () => [T(`Body at 1:100 (${count(100, "body")} solids): as 1:50 with 32 outline points per cladode and simpler fruits.`, `Körper bei 1:100 (${count(100, "body")} Körper): wie 1:50 mit 32 Umrisspunkten je Flachspross und einfacheren Früchten.`), T(`Thermal at 1:100 (${count(100, "thermal")} solids): by reference, fourteen outlets.`, `Thermik bei 1:100 (${count(100, "thermal")} Körper): per Verweis, vierzehn Auslässe.`), T(`Structure at 1:100 (${count(100, "structure")} solids): core members, no joints.`, `Tragwerk bei 1:100 (${count(100, "structure")} Körper): Kernstäbe, keine Gelenke.`)],
  200: () => [T(`Body at 1:200 (${count(200, "body")} solids): 24 outline points per cladode, the plate thickness raised to 34 mm (print minimum), three-station fruits.`, `Körper bei 1:200 (${count(200, "body")} Körper): 24 Umrisspunkte je Flachspross, die Plattendicke auf 34 mm angehoben (Druckminimum), Früchte aus drei Stationen.`), T(`Thermal at 1:200 (${count(200, "thermal")} solids): by reference, fourteen outlets.`, `Thermik bei 1:200 (${count(200, "thermal")} Körper): per Verweis, vierzehn Auslässe.`), T(`Structure at 1:200 (${count(200, "structure")} solids): core members of the feature, no joints.`, `Tragwerk bei 1:200 (${count(200, "structure")} Körper): Kernstäbe vom Merkmalsmaß, keine Gelenke.`)],
  500: () => [T(`Body at 1:500 (${count(500, "body")} solids): 20 outline points per cladode, plates of 80 mm, fruits of 78 mm.`, `Körper bei 1:500 (${count(500, "body")} Körper): 20 Umrisspunkte je Flachspross, Platten von 80 mm, Früchte von 78 mm.`), T(`Thermal at 1:500 (${count(500, "thermal")} solids): by reference, fourteen outlets.`, `Thermik bei 1:500 (${count(500, "thermal")} Körper): per Verweis, vierzehn Auslässe.`), T(`Structure at 1:500 (${count(500, "structure")} solids): core members of 76 mm inside the pads, no joints.`, `Tragwerk bei 1:500 (${count(500, "structure")} Körper): Kernstäbe von 76 mm in den Sprossen, keine Gelenke.`)],
};

function samples(): V3[] {
  const pts: V3[] = [];
  for (const g of Object.values(GEO)) for (let k = 0; k <= 40; k++) { const s = k / 40; for (const sd of [-1, 1]) for (const vv of [-1, 1]) pts.push(world(g.f, [s * g.L, vv * thick(g.T, s) / 2, sd * hw(g.W, s)])); }
  for (const fr of FRUITS) for (let a = 0; a < 12; a++) { const c = Math.cos(a * Math.PI / 6), s = Math.sin(a * Math.PI / 6); for (const [u, r] of FRUIT_PROFILE) pts.push(world(fr.f, [u, r * c, r * s])); pts.push(world(fr.f, [FRUIT_L, 0, 0])); }
  for (let a = 0; a < 12; a++) pts.push([0.125 * Math.cos(a * Math.PI / 6), 0, 0.125 * Math.sin(a * Math.PI / 6)]);
  return pts;
}
function symbol(): Json {
  const pts = samples();
  const proj = { elevation: (p: V3) => [p[0], p[1]], front: (p: V3) => [p[2], p[1]], plan: (p: V3) => [p[0], p[2]] } as const;
  const geo: Json = {};
  for (const view of ["elevation", "front", "plan"] as const) {
    const bars: [number, number, number, number, number, number][] = [];
    for (const g of Object.values(GEO)) { const a = proj[view](world(g.f, [g.L * 0.2, 0, 0])), b = proj[view](world(g.f, [g.L * 0.8, 0, 0])); bars.push([a[0]!, a[1]!, b[0]!, b[1]!, g.W * 0.45, g.W * 0.45]); }
    const tr = view === "plan" ? [0, 0, 0.001, 0, 0.13, 0.13] : [0, 0.13, 0, 1.2, 0.13, 0.13];
    bars.push(tr as [number, number, number, number, number, number]);
    const rings = barOutline(bars, 0.06);
    const ext = pts.map((p) => proj[view](p));
    const [x0, x1, y0, y1] = [Math.min(...ext.map((p) => p[0]!)), Math.max(...ext.map((p) => p[0]!)), Math.min(...ext.map((p) => p[1]!)), Math.max(...ext.map((p) => p[1]!))];
    const fit = (r: [number, number][]) => { const [a0, a1, b0, b1] = [Math.min(...r.map((p) => p[0])), Math.max(...r.map((p) => p[0])), Math.min(...r.map((p) => p[1])), Math.max(...r.map((p) => p[1]))]; return r.map(([x, y]) => [r4(x0 + ((x - a0) * (x1 - x0)) / (a1 - a0)), r4(view === "plan" ? y0 + ((y - b0) * (y1 - y0)) / (b1 - b0) : Math.max(0, y0 + ((y - b0) * (y1 - y0)) / (b1 - b0)))]); };
    const main = rings.sort((a, b) => b.length - a.length)[0]!;
    geo[view] = [{ id: "extent", bone: "root-plate", role: "body", z: 2, filled: false, tone: 0.5, kind: "poly", points: fit(main), closed: true }];
  }
  geo.plan.push({ id: "trunk", bone: "root-plate", role: "detail", z: 3, filled: false, tone: 0.3, kind: "ellipse", cx: 0, cy: 0, rx: 0.12, ry: 0.12 });
  return { id: "prickly-pear-symbol", title: T("Planting-plan sign with the true extent of the clump", "Pflanzplanzeichen mit der wahren Ausdehnung des Buschs"), description: T("A conventional sign for planting plans: the lobed outline of the cladode clump at its true extent in every view, with the trunk circle in plan; a separate symbol, never used silently for a pictorial scale.", "Ein konventionelles Zeichen für Pflanzpläne: der gelappte Umriss des Sprossbuschs in wahrer Ausdehnung in jeder Ansicht, im Grundriss mit dem Stammkreis; ein eigenes Symbol, nie stillschweigend für einen Bildmaßstab verwendet."), semantics: "true-extent", units: "model-m", denominators: [500, 200, 100], pens: { body: 0.18, detail: 0.13, texture: null, glazing: null, ground: 0.25 }, hatches: [], geometry: geo, dimensionalNote: T("Outline at the true extent of the default pose.", "Umriss in der wahren Ausdehnung der Standardpose.") };
}

export function asset(): Json {
  const pts = samples();
  const xs = pts.map((p) => p[0]), ys = pts.map((p) => p[1]), zs = pts.map((p) => p[2]);
  const mass = RIG.bones.reduce((s: number, x: Json) => s + (x.mass?.kg ?? 0), 0);
  const representations: Json = {}, volumes: Json = {};
  for (const s of SCALES) {
    representations[String(s)] = { status: "authored", denominator: s, validRange: RANGES[s], pens: PENS[s], hatches: [], geometry: views(PARTS[s].drawn), decisions: REP_TEXT[s] };
    volumes[String(s)] = { status: "authored", denominator: s, body: PARTS[s].body, thermal: PARTS[s].thermal, structure: PARTS[s].structure, decisions: VOL_TEXT[s]() };
  }
  const H = Math.max(...ys);
  return {
    id: ID, category: "vegetation", rig: RIG, anchors: ANCHORS,
    title: T("Prickly Pear", "Feigenkaktus"),
    summary: T(`An Indian fig opuntia (Opuntia ficus-indica) of ${H.toFixed(1)} m: a short woody trunk and four lignified old segments carry forty flat, obovate cladodes (pads) of 24-48 cm in four generations, each growing from the margin of the pad that bore it, most facing east and west; ${FRUITS.length} ripe orange-red fruits stand on the upper margins of the one-year-old pads. A CAM plant: its stomata open at night.`, `Ein Feigenkaktus (Opuntia ficus-indica) von ${H.toFixed(1).replace(".", ",")} m: ein kurzer holziger Stamm und vier verholzte alte Glieder tragen vierzig flache, verkehrt eiförmige Flachsprosse (Kladodien) von 24-48 cm in vier Generationen, jeder aus dem Rand des Sprosses gewachsen, der ihn trug, die meisten nach Osten und Westen gerichtet; ${FRUITS.length} reife orangerote Früchte stehen auf den oberen Rändern der einjährigen Sprosse. Eine CAM-Pflanze: Ihre Spaltöffnungen öffnen sich nachts.`),
    tags: ["cactus", "succulent", "desert", "cam", "opuntia", "fruit", "mediterranean", "transpiration"],
    provenance: { author: "Semio staffage plant collection wave, worker PW-11", method: "handcrafted", created: "2026-11-22", references: Object.values(REF), notes: T("Every cladode is stated by hand (parent, direction, facing) and placed by the script at the margin point of its parent that faces its direction; sizes per generation (48 x 31 x 3 cm down to 24 x 15 x 1.6 cm) and the obovate planform follow Nobel (2002) and FAO (2017); masses are the modelled volume times 950 kg/m3 for pads and 750 kg/m3 for the lignified wood. Stiffness, conductance and heat capacity are authored estimates.", "Jeder Flachspross ist von Hand festgelegt (Mutterspross, Richtung, Ausrichtung) und vom Skript an den Randpunkt seines Muttersprosses gesetzt, der in seine Richtung zeigt; die Größen je Generation (48 x 31 x 3 cm bis 24 x 15 x 1,6 cm) und der verkehrt eiförmige Grundriss folgen Nobel (2002) und FAO (2017); die Massen sind das modellierte Volumen mal 950 kg/m3 für Sprosse und 750 kg/m3 für das verholzte Holz. Steifigkeit, Leitfähigkeit und Wärmekapazität sind Autorenschätzungen.") },
    dimensions: { heightM: r4(H), widthM: r4(Math.max(...xs) - Math.min(...xs)), depthM: r4(Math.max(...zs) - Math.min(...zs)), massKg: Math.round(mass) },
    materials: MATERIALS, representations, symbols: [symbol()], poses: POSES, clips: [swayClip(), gustClip(), diurnalClip()],
    thermal: { environment: ENV, passengers: [], operating: [], regions: REGIONS, emitters: EMITTERS, assumptions: [
      T(`Cladode regions use both faces of the forty pads as exchange area (young ${r4(AREA.young)} m2, mature ${r4(AREA.mature)} m2). CAM: stomata open at night (Nobel 1988); maximum conductances of 0.0015 and 0.0012 m/s are authored estimates.`, `Die Sprossregionen verwenden beide Flächen der vierzig Sprosse als Austauschfläche (jung ${r4(AREA.young)} m2, ausgewachsen ${r4(AREA.mature)} m2). CAM: Die Spaltöffnungen öffnen nachts (Nobel 1988); die maximalen Leitfähigkeiten 0,0015 und 0,0012 m/s sind Autorenschätzungen.`),
      T("Areal heat capacity 45-55 kJ/(m2 K): half the water-rich pad (12-15 mm, about 3.7 MJ/(m3 K)) per face. Trunk and fruit are passive; transpiration leaves through seven outlets on young and seven on mature pads, normal to the face.", "Flächenbezogene Wärmekapazität 45-55 kJ/(m2 K): der halbe wasserreiche Spross (12-15 mm, etwa 3,7 MJ/(m3 K)) je Fläche. Stamm und Früchte sind passiv; die Transpiration tritt durch sieben Auslässe an jungen und sieben an ausgewachsenen Sprossen aus, normal zur Fläche."),
    ] },
    structure: { gravityMS2: 9.80665, dynamics: "quasi-static", supports: ["base-support"], assumptions: [
      T("One rigid body per evaluated pose, fixed at the root plate; wind drag acts at the centre of each cladode (cd 1.1 of a flat plate on its projected area, Hoerner 1965; power-law profile after EN 1991-1-4) and on the trunk as a line load; snow on the upper margins; gravity on every bone mass including the fruit.", "Ein starrer Körper je ausgewerteter Pose, am Wurzelteller eingespannt; Winddruck wirkt im Mittelpunkt jedes Flachsprosses (cd 1,1 einer ebenen Platte auf ihrer projizierten Fläche, Hoerner 1965; Potenzprofil nach EN 1991-1-4) und als Linienlast auf den Stamm; Schnee auf den oberen Rändern; Schwerkraft auf jede Knochenmasse einschließlich der Früchte."),
      T("Sections: the narrow base of each cladode as a rectangle of 35 % of its width by its thickness (E 30 MPa of turgid tissue, authored estimate); trunk and segments as solid woody circles (E 3 GPa). Breaking of cladodes at their joints is not predicted.", "Querschnitte: die schmale Basis jedes Flachsprosses als Rechteck von 35 % seiner Breite mal seiner Dicke (E 30 MPa prallen Gewebes, Autorenschätzung); Stamm und Glieder als holzige Vollkreise (E 3 GPa). Das Abbrechen von Sprossen an ihren Gelenken wird nicht vorhergesagt."),
    ], clinical: false },
    volumes, presentation: { defaultPose: "fruiting", defaultClip: "wind-sway", defaultScale: 100 }, exports: EXPORTS, completion: "authored",
  };
}
export const DISTINCTION = T("The archive's only cladode cactus: a 3 m clump of forty flat obovate pads in four generations on a short woody trunk, each pad on its own bone at the margin of its parent, with ripe fruits on the young pads; CAM, transpiring at night.", "Der einzige Flachsprosskaktus des Archivs: ein 3 m hoher Busch aus vierzig flachen, verkehrt eiförmigen Sprossen in vier Generationen auf kurzem holzigem Stamm, jeder Spross an eigenem Knochen am Rand seines Muttersprosses, mit reifen Früchten an den jungen Sprossen; CAM, transpiriert nachts.");
const balance = (() => { const pts = samples(); let best = { deg: 0, off: Infinity }; for (let deg = -60; deg <= 60; deg++) { const c = Math.cos(deg * Math.PI / 180), sn = Math.sin(deg * Math.PI / 180); const zs = pts.map((p) => -sn * p[0] + c * p[2]); const off = Math.abs(Math.max(...zs) + Math.min(...zs)); if (off < best.off) best = { deg, off }; } return best; })();
export const REPORT = { rigError, area: AREA, pads: PADS.length, fruits: FRUITS.length, balance };
