/** 🌵️ PW-11: the saguaro (Carnegiea gigantea) stated by hand: a 10 m fluted column of 18 ribs with five arms in the shape of the species (each rising from a narrow neck, bending up through a smooth elbow and standing upright), a corky brown base, woolly apices, the woody rib skeleton inside, CAM physiology (stomata open at night). Builds the whole `semio.staffage/v2` asset.
 * @see ../../📋️brief.md
 * @see ./kit.ts */
import { FEATURE, PENS, RANGES, RigBuilder, SCALES, T, add, barOutline, cross, loftOf, unit, dir, documentOf, drag, ellipsoidSolid, leanOffsets, mul, poseFrames, r4, r5, revolveSolid, revolvedArea, revolvedShape, ribbedColumn, ribbedPerimeterRatio, ribbedProfile, ribbedShape, table, upNormal, views, world, type Drawn, type Frame, type Json, type Scale, type V3, EXPORTS, sub } from "./kit.ts";

export const ID = "saguaro-cactus";

const REF = {
  steenbergh: { id: "steenbergh-lowe-1977", citation: "Steenbergh, W. F. and Lowe, C. H. (1977). Ecology of the Saguaro II: Reproduction, Germination, Establishment, Growth, and Survival of the Young Plant. National Park Service Scientific Monograph Series 8, Washington DC. Saguaro height growth, arm initiation at about 2-5 m, trunk diameters 30-75 cm, age." },
  niklas: { id: "niklas-buchman-1994", citation: "Niklas, K. J. and Buchman, S. L. (1994). The allometry of saguaro height. American Journal of Botany 81(9), 1161-1168. Stem diameter against height, mechanical safety of the columnar stem, the woody rib skeleton." },
  gibson: { id: "gibson-nobel-1986", citation: "Gibson, A. C. and Nobel, P. S. (1986). The Cactus Primer. Harvard University Press, Cambridge MA. Rib (pleat) anatomy, areoles and spines, apical pubescence, the cylinder of woody ribs, water storage parenchyma." },
  nobel88: { id: "nobel-1988", citation: "Nobel, P. S. (1988). Environmental Biology of Agaves and Cacti. Cambridge University Press. Crassulacean acid metabolism, nocturnal stomatal opening, stem water content, stem surface temperatures and thermal capacity." },
  nobel80: { id: "nobel-1980", citation: "Nobel, P. S. (1980). Morphology, surface temperatures, and northern limits of columnar cacti in the Sonoran Desert. Ecology 61(1), 1-7. Apical spines and pubescence of Carnegiea gigantea, stem diameter and apex temperature, freezing limits." },
  talley: { id: "talley-2001", citation: "Talley, S., Iaccarino, G., Mungal, G. and Mansour, N. N. (2001). An experimental and computational investigation of flow past cacti. Annual Research Briefs, Center for Turbulence Research, Stanford University, 51-63. Ribbed (saguaro-like) cylinders have lower drag than smooth cylinders at the same Reynolds number." },
  nps: { id: "nps-saguaro", citation: "U.S. National Park Service, Saguaro National Park: Saguaro cactus facts (nps.gov/sagu). Mature saguaros 12-18 m tall at the maximum, first arm at 50-70 years, a fully hydrated plant weighs about 1.4-2.2 t." },
  monteith: { id: "monteith-unsworth-2013", citation: "Monteith, J. L. and Unsworth, M. H. (2013). Principles of Environmental Physics, 4th ed., Academic Press. Penman-Monteith leaf transpiration, boundary layers of cylinders." },
  en: { id: "en-1991-1-4", citation: "EN 1991-1-4:2005 Eurocode 1, Actions on structures, Part 1-4: Wind actions. Dynamic pressure q = 0.5 rho v^2 with air density 1.25 kg/m3, force F = cf q A; mean wind profile." },
  en13: { id: "en-1991-1-3", citation: "EN 1991-1-3:2003 Eurocode 1, Actions on structures, Part 1-3: Snow loads. Density of settled and wet snow (about 250-400 kg/m3)." },
};

type Arm = { n: number; az: number; h: number; r: number; segs: [number, number][]; tip: number };

/** 📋️ The five arms as stated by hand, oldest (lowest) first: azimuth (counter-clockwise from ahead, seen from above), insertion height on the trunk axis, arm radius at the ribs' crests, the elevation and length of each of the six bones (neck, four elbow bones of 14-16 degrees each, upright), and the woolly apex. First arms at 3-5 m and arms thinner than the trunk follow Steenbergh and Lowe (1977); the elbow radius of about 0.6 m and the narrow neck follow photographs of mature plants in Saguaro National Park. */
const ARMS: Arm[] = [
  { n: 1, az: 25, h: 3.3, r: 0.19, segs: [[12, 0.32], [26, 0.16], [42, 0.16], [58, 0.16], [74, 0.16], [90, 3.05]], tip: 0.19 },
  { n: 2, az: 152, h: 3.85, r: 0.185, segs: [[10, 0.34], [24, 0.16], [40, 0.17], [56, 0.17], [72, 0.17], [90, 3.0]], tip: 0.185 },
  { n: 3, az: -78, h: 4.45, r: 0.18, segs: [[14, 0.3], [28, 0.15], [44, 0.15], [60, 0.15], [76, 0.15], [90, 2.35]], tip: 0.18 },
  { n: 4, az: 88, h: 5.4, r: 0.165, segs: [[16, 0.28], [30, 0.14], [46, 0.14], [62, 0.14], [78, 0.14], [90, 1.45]], tip: 0.165 },
  { n: 5, az: -140, h: 6.25, r: 0.145, segs: [[18, 0.26], [34, 0.13], [50, 0.13], [66, 0.13], [80, 0.12], [90, 0.62]], tip: 0.145 },
];
const SEG = ["a", "b", "c", "d", "e", "f"];
const armBone = (a: Arm, i: number) => `arm-${a.n}-${SEG[i]}`;

/** 📏️ Crest radius of the trunk by height: a slight waist at the ground, fullest at mid-height, tapering to the apex. */
const TRUNK_R: [number, number][] = [[0, 0.205], [0.25, 0.214], [0.8, 0.23], [2, 0.242], [4, 0.25], [6, 0.247], [7.5, 0.24], [8.6, 0.23], [9.3, 0.218]];
const APEX_Y = 10.0, DOME_Y = 9.3;
const trunkR = (y: number) => (y <= DOME_Y ? table(TRUNK_R, y) : 0.218 * Math.sqrt(Math.max(0, 1 - ((y - DOME_Y) / (APEX_Y - DOME_Y)) ** 2)));
const TRUNK = [{ id: "trunk-1", y0: 0, y1: 3 }, { id: "trunk-2", y0: 3, y1: 6 }, { id: "trunk-3", y0: 6, y1: 8.6 }, { id: "crown", y0: 8.6, y1: APEX_Y }];
const COLLAR = 1.15, OV = 0.05, EXT = 0.035;
const FLUTE_STEPS = 4;
/** 🖊️ The elevations in which the stroke of a rib crest at angle `phi` on a column of radius `r` is drawn: not where the crest is the outline itself (within half its pen of the edge of the view), there the outline of the body already draws it. */
const crestViews = (r: number, phi: number, scale: Scale): ("elevation" | "front")[] => (["elevation", "front"] as const).filter((view) => r * (1 - Math.abs(view === "elevation" ? Math.cos(phi) : Math.sin(phi))) >= ((PENS[scale] as Json).detail * scale) / 2000 + 0.002);
const RIBS = 18, ARM_RIBS = 14, RIB_DEPTH = 0.034, ARM_RIB_DEPTH = 0.026, COLLAR_RIB_DEPTH = 0.02;
const DENSITY = 960;

const B = new RigBuilder({ id: "root-plate", title: T("Root plate: taproot and the shallow lateral roots in the desert soil", "Wurzelteller: Pfahlwurzel und flache Seitenwurzeln im Wüstenboden"), lengthM: 0.5, joint: { type: "fixed" } });
const UP: V3 = [0, 1, 0], BACK: V3 = [-1, 0, 0];
const trunkTitles: Record<string, Json> = {
  "trunk-1": T("Trunk, lower third with the corky base", "Stamm, unteres Drittel mit verkorkter Basis"),
  "trunk-2": T("Trunk, middle third (arm insertions)", "Stamm, mittleres Drittel (Armansätze)"),
  "trunk-3": T("Trunk, upper third", "Stamm, oberes Drittel"),
  crown: T("Crown of the trunk with the woolly apex", "Stammkrone mit wolligem Scheitel"),
};
const TRUNK_LIMITS: Record<string, number> = { "trunk-1": 3, "trunk-2": 4, "trunk-3": 5, crown: 6 };
const section = (n: number, d: number, R: number, title: Json) => {
  const A = n * Math.PI * (d / 2) ** 2;
  return { areaM2: r5(A), secondMomentM4: Number((A * R * R / 2).toPrecision(4)), extremeFibreM: r4(R + d / 2), material: "saguaro-wood", title };
};
const TRUNK_SECTION = section(18, 0.03, 0.15, T("Cylinder of 18 woody ribs, d = 3 cm at r = 15 cm", "Zylinder aus 18 Holzrippen, d = 3 cm bei r = 15 cm"));
const ARM_RING = 0.5;
const armSection = (r: number) => section(14, 0.022, ARM_RING * r, T(`Cylinder of 14 woody ribs, d = 2.2 cm at r = ${Math.round(ARM_RING * r * 100)} cm`, `Zylinder aus 14 Holzrippen, d = 2,2 cm bei r = ${Math.round(ARM_RING * r * 100)} cm`));

/** ⚖️ Volume of a fluted column segment: crest circle area times the area share of the star, integrated over height. */
const STAR_SHARE = 0.93;
function columnMass(r: (u: number) => number, u0: number, u1: number): number {
  let v = 0;
  const n = 40;
  for (let i = 0; i < n; i++) { const u = u0 + ((u1 - u0) * (i + 0.5)) / n; v += Math.PI * r(u) ** 2 * STAR_SHARE * ((u1 - u0) / n); }
  return v * DENSITY;
}

for (const t of TRUNK) {
  const kg = columnMass(trunkR, t.y0, t.y1);
  B.add(t.id, t.id === "trunk-1" ? "root-plate" : TRUNK[TRUNK.indexOf(t) - 1]!.id, trunkTitles[t.id], [0, t.y0, 0], UP, BACK, t.y1 - t.y0, { type: "revolute", minDeg: -TRUNK_LIMITS[t.id]!, maxDeg: TRUNK_LIMITS[t.id]! }, {
    mass: { kg: r4(kg), comX: r4((t.y1 - t.y0) * (t.id === "crown" ? 0.38 : 0.5)), comY: 0, distribution: "uniform", source: "authored-estimate", reference: "nobel-1988" },
    section: TRUNK_SECTION,
  });
}

type ArmGeo = { a: Arm; frames: Frame[]; lengths: number[]; parent: string; r: (i: number, u: number) => number };
const ARM_GEO: ArmGeo[] = [];
for (const a of ARMS) {
  const parent = TRUNK.find((t) => a.h >= t.y0 && a.h < t.y1)!.id;
  let o: V3 = add([0, a.h, 0], mul(dir(a.az, 0), 0.12));
  const frames: Frame[] = [], lengths: number[] = [];
  a.segs.forEach(([el, L], i) => {
    const u = dir(a.az, el), v = upNormal(a.az, el);
    const neck = i === 0;
    const radiusAt = (uu: number) => (neck ? table([[0, 0.62], [0.12, 0.7], [0.19, 0.74], [0.27, 0.88], [L, 1]], uu) * a.r : a.r);
    let kg = 0;
    for (let k = 0; k < 20; k++) kg += Math.PI * radiusAt((L * (k + 0.5)) / 20) ** 2 * STAR_SHARE * (L / 20) * DENSITY;
    const title = i === 0 ? T(`Arm ${a.n}, narrow neck at the trunk`, `Arm ${a.n}, schmaler Hals am Stamm`) : i < 5 ? T(`Arm ${a.n}, elbow ${i} of 4`, `Arm ${a.n}, Ellbogen ${i} von 4`) : T(`Arm ${a.n}, upright with the woolly apex`, `Arm ${a.n}, aufrecht mit wolligem Scheitel`);
    const f = B.add(armBone(a, i), i === 0 ? parent : armBone(a, i - 1), title, o, u, v, L, { type: "revolute", minDeg: i === 0 ? -4 : -6, maxDeg: i === 0 ? 4 : 6 }, {
      mass: { kg: r4(kg), comX: r4(L * (i === 5 ? 0.46 : 0.5)), comY: 0, distribution: "uniform", source: "authored-estimate", reference: "nobel-1988" },
      section: armSection(a.r),
    });
    frames.push(f);
    lengths.push(L);
    o = add(o, mul(u, L));
  });
  ARM_GEO.push({ a, frames, lengths, parent, r: (i, u) => (i === 0 ? table([[0, 0.62], [0.12, 0.7], [0.19, 0.74], [0.27, 0.88], [a.segs[0]![1], 1]], u) * a.r : a.r) });
}
B.bones[0].mass = { kg: 180, comX: 0, comY: -0.35, distribution: "point", source: "authored-estimate", reference: "steenbergh-lowe-1977" };

const rigError = B.verify();
const RIG = B.rig();

/** 🌡️ Thermal regions and their areas from the 1:50 forms (crest circle times the perimeter share of the ribbed star; the smooth coarse scales carry the circle). */
const ribRatio = ribbedPerimeterRatio(RIBS, 0.24, RIB_DEPTH);
const armRibRatio = ribbedPerimeterRatio(ARM_RIBS, 0.18, ARM_RIB_DEPTH);
const collarRatio = ribbedPerimeterRatio(RIBS, 0.22, COLLAR_RIB_DEPTH);
const lateral = (r: (y: number) => number, y0: number, y1: number) => { let a = 0; for (let i = 0; i < 60; i++) { const y = y0 + ((y1 - y0) * (i + 0.5)) / 60; a += 2 * Math.PI * r(y) * ((y1 - y0) / 60); } return a; };
const domeProfile = (r0: number, h: number, n: number, u0 = 0): [number, number][] => Array.from({ length: n + 1 }, (_, i) => { const t = (i / n) * Math.PI / 2; return [u0 + h * Math.sin(t), r0 * Math.cos(t)] as [number, number]; });
const AREA = {
  collar: lateral(trunkR, 0, COLLAR) * (1 + collarRatio) / 2 + Math.PI * trunkR(0) ** 2 + Math.PI * trunkR(COLLAR) ** 2,
  trunk: lateral(trunkR, COLLAR, DOME_Y) * ribRatio,
  arms: ARM_GEO.reduce((s, g) => s + g.lengths.reduce((t, L, i) => t + (i === 5 ? 2 * Math.PI * g.a.r * (L - g.a.tip) : lateral((u) => g.r(i, u), 0, L)), 0), 0) * armRibRatio,
  apex: revolvedArea(domeProfile(0.218, APEX_Y - DOME_Y, 16)) + Math.PI * 0.218 ** 2 + ARM_GEO.reduce((s, g) => s + revolvedArea(domeProfile(g.a.r, g.a.tip, 16)) + Math.PI * g.a.r ** 2, 0),
};

const MATERIALS = [
  { id: "saguaro-stem", title: T("Saguaro chlorenchyma over water-storing parenchyma (about 90 % water), waxy green cuticle", "Saguaro-Chlorenchym über wasserspeicherndem Parenchym (etwa 90 % Wasser), wachsige grüne Kutikula"), emissivity: 0.97, solarAbsorptivity: 0.6, densityKgM3: DENSITY, specificHeatJKgK: 3900, conductivityWmK: 0.55, source: "authored-estimate", reference: "nobel-1988" },
  { id: "saguaro-wood", title: T("Woody ribs of the saguaro skeleton", "Holzrippen des Saguaro-Skeletts"), emissivity: 0.9, densityKgM3: 550, elasticModulusPa: 5000000000, strengthPa: 40000000, source: "authored-estimate", reference: "niklas-buchman-1994" },
  { id: "saguaro-bark", title: T("Corky brown bark of the aged saguaro base", "Verkorkte braune Rinde der gealterten Saguaro-Basis"), emissivity: 0.93, solarAbsorptivity: 0.75, densityKgM3: 700, conductivityWmK: 0.2, specificHeatJKgK: 2000, source: "authored-estimate", reference: "gibson-nobel-1986" },
  { id: "saguaro-areole-wool", title: T("Felted apex: areole wool and dense young spines", "Filziger Scheitel: Areolenwolle und dichte junge Dornen"), emissivity: 0.9, solarAbsorptivity: 0.45, densityKgM3: 900, conductivityWmK: 0.1, specificHeatJKgK: 3000, source: "authored-estimate", reference: "nobel-1980" },
];

const REGIONS = [
  { id: "trunk-chlorenchyma", title: T("Green ribbed trunk (photosynthetic CAM stem)", "Grüner gerippter Stamm (photosynthetischer CAM-Stamm)"), kind: "stem", model: "leaf", material: "saguaro-stem", areaM2: r4(AREA.trunk), areaBasis: "exchange", characteristicLengthM: 0.48, geometry: "cylinder", skyViewFactor: 0.5, layers: [], heatCapacityJM2K: 250000, leaf: { stomatalConductanceMaxMS: 0.0008, leafAreaIndex: 1, photosynthesis: "cam" }, source: "authored-estimate", reference: "nobel-1988" },
  { id: "arm-chlorenchyma", title: T("Green ribbed arms (photosynthetic CAM stems)", "Grüne gerippte Arme (photosynthetische CAM-Stämme)"), kind: "stem", model: "leaf", material: "saguaro-stem", areaM2: r4(AREA.arms), areaBasis: "exchange", characteristicLengthM: 0.36, geometry: "cylinder", skyViewFactor: 0.55, layers: [], heatCapacityJM2K: 200000, leaf: { stomatalConductanceMaxMS: 0.0008, leafAreaIndex: 1, photosynthesis: "cam" }, source: "authored-estimate", reference: "nobel-1988" },
  { id: "apex-areoles", title: T("Woolly apices of trunk and arms (areole felt and young spines)", "Wollige Scheitel von Stamm und Armen (Areolenfilz und junge Dornen)"), kind: "stem", model: "passive", material: "saguaro-areole-wool", areaM2: r4(AREA.apex), characteristicLengthM: 0.4, geometry: "horizontal-plate-up", skyViewFactor: 0.9, layers: [], heatCapacityJM2K: 150000, source: "authored-estimate", reference: "nobel-1980" },
  { id: "basal-bark", title: T("Corky brown base of the trunk", "Verkorkte braune Stammbasis"), kind: "bark", model: "passive", material: "saguaro-bark", areaM2: r4(AREA.collar), characteristicLengthM: 0.44, geometry: "cylinder", skyViewFactor: 0.45, layers: [], heatCapacityJM2K: 220000, source: "authored-estimate", reference: "gibson-nobel-1986" },
];

/** 📍️ Anchors: the fixed base, six transpiration outlets on the trunk (spread over height and azimuth), five on the arms (one on each upright). */
const TRUNK_OUTLETS: [string, number, number][] = [["trunk-1", 1.8, 20], ["trunk-1", 2.6, 200], ["trunk-2", 0.8, 110], ["trunk-2", 2.2, -60], ["trunk-3", 0.9, 160], ["trunk-3", 2.1, -10]];
const localOnTrunk = (bone: string, u: number, az: number, out: number): V3 => {
  const y = TRUNK.find((t) => t.id === bone)!.y0 + u;
  const r = trunkR(y) + out;
  const p = dir(az, 0);
  return [u, -p[0] * r, p[2] * r];
};
const ANCHORS = [
  { id: "base-support", bone: "root-plate", x: 0, y: 0, kind: "support", title: T("Root plate in the desert soil (fixed)", "Wurzelteller im Wüstenboden (eingespannt)"), support: "fixed", normalDeg: 90, footprintLengthM: 1.2, footprintWidthM: 1.2, frictionCoefficient: 0.6 },
  ...TRUNK_OUTLETS.map(([bone, u, az], i) => { const p = localOnTrunk(bone, u, az, 0.004); return { id: `tp-trunk-${i + 1}`, bone, x: r4(p[0]), y: r4(p[1]), z: r4(p[2]), kind: "emitter", title: T(`Transpiration outlet ${i + 1} on the trunk`, `Transpirationsauslass ${i + 1} am Stamm`) }; }),
  ...ARM_GEO.map((g, i) => { const u = g.lengths[5]! * 0.45; const p: V3 = [u, g.a.r + 0.004, 0]; return { id: `tp-arm-${i + 1}`, bone: armBone(g.a, 5), x: r4(p[0]), y: r4(p[1]), kind: "emitter", title: T(`Transpiration outlet on arm ${g.a.n}`, `Transpirationsauslass an Arm ${g.a.n}`) }; }),
];
const EMITTERS = [
  { id: "transpiration-trunk", kind: "transpiration", title: T("Transpiration of the trunk (at night, CAM; six outlets over the column)", "Transpiration des Stamms (nachts, CAM; sechs Auslässe über die Säule)"), anchor: "tp-trunk-1", anchors: TRUNK_OUTLETS.map((_, i) => `tp-trunk-${i + 1}`), directionDeg: 0, diameterM: 0.5, region: "trunk-chlorenchyma", source: "authored-estimate", reference: "monteith-unsworth-2013" },
  { id: "transpiration-arms", kind: "transpiration", title: T("Transpiration of the arms (at night, CAM; one outlet on each upright)", "Transpiration der Arme (nachts, CAM; ein Auslass an jedem aufrechten Arm)"), anchor: "tp-arm-1", anchors: ARM_GEO.map((_, i) => `tp-arm-${i + 1}`), directionDeg: 0, diameterM: 0.38, region: "arm-chlorenchyma", source: "authored-estimate", reference: "monteith-unsworth-2013" },
];

const ENV = { airTempC: 40, meanRadiantTempC: 50, skyTempC: 22, groundTempC: 62, relativeHumidity: 0.15, windMS: 2, windDirectionDeg: 0, solarWM2: 950, diffuseFraction: 0.1, pressurePa: 101325 };

// Poses -------------------------------------------------------------------------------------------------------------

const v10 = (y: number, v: number) => v * Math.pow(Math.max(y, 1) / 10, 0.22);
const CD = 0.7;
const trunkD = (bone: string) => { const t = TRUNK.find((x) => x.id === bone)!; return 2 * trunkR((t.y0 + t.y1) / 2); };
function windLoads(v: number, label: Json): Json[] {
  const loads: Json[] = [];
  for (const t of TRUNK) {
    const ym = (t.y0 + t.y1) / 2, w = r4(drag(CD, trunkD(t.id), v10(ym, v)));
    loads.push({ id: `wind-${t.id}`, kind: "wind", title: T(`Wind drag on the ${t.id === "crown" ? "crown" : `trunk (${t.id.slice(-1)})`}, ${label.en}`, `Winddruck auf ${t.id === "crown" ? "die Krone" : `den Stamm (${t.id.slice(-1)})`}, ${label.de}`), fxN: 0, fyN: 0, distributed: { bone: t.id, fromM: 0, toM: r4(t.y1 - t.y0), wxNPerM: w, wyNPerM: 0 }, source: "literature", reference: "en-1991-1-4" });
  }
  for (const g of ARM_GEO) {
    const f = g.frames[5]!, L = g.lengths[5]!;
    const ym = f.o[1] + L / 2;
    loads.push({ id: `wind-arm-${g.a.n}`, kind: "wind", title: T(`Wind drag on arm ${g.a.n}, ${label.en}`, `Winddruck auf Arm ${g.a.n}, ${label.de}`), fxN: 0, fyN: 0, distributed: { bone: armBone(g.a, 5), fromM: 0, toM: r4(L), wxNPerM: r4(drag(CD, 2 * g.a.r, v10(ym, v))), wyNPerM: 0 }, source: "literature", reference: "en-1991-1-4" });
    const fe = g.frames[2]!;
    loads.push({ id: `wind-elbow-${g.a.n}`, kind: "wind", title: T(`Wind drag on the elbow of arm ${g.a.n}, ${label.en}`, `Winddruck auf den Ellbogen von Arm ${g.a.n}, ${label.de}`), bone: armBone(g.a, 2), x: r4(g.lengths[2]! / 2), y: 0, fxN: r4(drag(CD, 2 * g.a.r * 0.75, v10(fe.o[1], v))), fyN: 0, source: "literature", reference: "en-1991-1-4" });
  }
  return loads;
}
const totalWind = (loads: Json[]) => loads.reduce((s, l) => s + (l.distributed ? l.distributed.wxNPerM * (l.distributed.toM - l.distributed.fromM) : l.fxN), 0);

/** 🌬️ Lean of every bone about the world axis z in degrees: stiffness from the woody rib cylinder (E I about 0.7 MN m2 trunk), the tip slope of a cantilever under the drag w L^3 / (6 E I) shared over the segments; arms lean a little more than the trunk carries them. */
function leanFor(scale: number): { id: string; deg: number }[] {
  const out: { id: string; deg: number }[] = [];
  const trunk: Record<string, number> = { "trunk-1": -0.45, "trunk-2": -0.5, "trunk-3": -0.42, crown: -0.23 };
  for (const t of TRUNK) out.push({ id: t.id, deg: r4(trunk[t.id]! * scale) });
  for (const g of ARM_GEO) g.a.segs.forEach((_, i) => out.push({ id: armBone(g.a, i), deg: r4((i === 0 ? -0.35 : i === 5 ? -0.4 : -0.12) * scale) }));
  return out;
}
const WINDY = leanOffsets(RIG, {}, leanFor(0.2));
const STORM = leanOffsets(RIG, {}, leanFor(1));

const regionsDay = { "trunk-chlorenchyma": { exposedFraction: 1, sunlitFraction: 0.5 }, "arm-chlorenchyma": { exposedFraction: 1, sunlitFraction: 0.5 }, "apex-areoles": { exposedFraction: 1, sunlitFraction: 0.9 }, "basal-bark": { exposedFraction: 1, sunlitFraction: 0.45 } };
const regionsNight = { "trunk-chlorenchyma": { exposedFraction: 1, sunlitFraction: 0 }, "arm-chlorenchyma": { exposedFraction: 1, sunlitFraction: 0 }, "apex-areoles": { exposedFraction: 1, sunlitFraction: 0 }, "basal-bark": { exposedFraction: 1, sunlitFraction: 0 } };
const closed = { "transpiration-trunk": 0.04, "transpiration-arms": 0.04 };
const open = { "transpiration-trunk": 1, "transpiration-arms": 1 };
const windyLoads = windLoads(12, T("12 m/s", "12 m/s")), stormLoads = windLoads(28, T("28 m/s", "28 m/s"));
const snowLoads = [
  { id: "snow-crown", kind: "snow", title: T("Wet snow cap on the trunk apex", "Nasse Schneekappe auf dem Stammscheitel"), bone: "crown", x: r4(APEX_Y - 8.6 - 0.15), y: 0, fxN: 0, fyN: r4(-Math.PI * 0.2 ** 2 * 0.06 * 300 * 9.81), source: "literature", reference: "en-1991-1-3" },
  ...ARM_GEO.map((g) => ({ id: `snow-arm-${g.a.n}`, kind: "snow", title: T(`Wet snow cap on the apex of arm ${g.a.n}`, `Nasse Schneekappe auf dem Scheitel von Arm ${g.a.n}`), bone: armBone(g.a, 5), x: r4(g.lengths[5]! - 0.12), y: 0, fxN: 0, fyN: r4(-Math.PI * (g.a.r * 0.9) ** 2 * 0.06 * 300 * 9.81), source: "literature", reference: "en-1991-1-3" })),
];

const POSES = [
  { id: "calm", title: T("Calm summer day", "Windstiller Sommertag"), description: T("A still afternoon in the Sonoran Desert (40 degC air, strong sun): the column stands upright, the ribs are half filled with stored water and the stomata are shut (CAM), so the plant transpires almost nothing during the day.", "Ein windstiller Nachmittag in der Sonora-Wüste (40 °C Luft, starke Sonne): Die Säule steht aufrecht, die Rippen sind halb mit gespeichertem Wasser gefüllt und die Spaltöffnungen sind geschlossen (CAM), die Pflanze transpiriert tagsüber fast nichts."), bones: {}, ground: "authored", contacts: [], thermal: { regions: regionsDay, emitters: closed }, loads: [] },
  { id: "night-stomata-open", title: T("Night, stomata open (CAM)", "Nacht, Spaltöffnungen offen (CAM)"), description: T("A warm desert night: the CAM stomata are open and take up carbon dioxide while the air is cool and the vapour deficit small, so nearly all of the day's water loss happens now.", "Eine warme Wüstennacht: Die CAM-Spaltöffnungen sind offen und nehmen Kohlendioxid auf, solange die Luft kühl und das Sättigungsdefizit klein ist; fast der ganze Wasserverlust des Tages geschieht jetzt."), bones: {}, ground: "authored", contacts: [], thermal: { regions: regionsNight, emitters: open }, loads: [] },
  { id: "windy", title: T("Windy (12 m/s)", "Windig (12 m/s)"), description: T(`Strong breeze of 12 m/s at 10 m (power-law profile, exponent 0.22), drag coefficient 0.7 of a ribbed column (lower than 1.2 of a smooth cylinder, Talley et al. 2001) on the trunk and the uprights: ${Math.round(totalWind(windyLoads))} N in total. The stiff column leans by about 0.3 degrees at the apex.`, `Starker Wind von 12 m/s in 10 m Höhe (Potenzprofil, Exponent 0,22), Widerstandsbeiwert 0,7 einer gerippten Säule (kleiner als 1,2 eines glatten Zylinders, Talley u. a. 2001) auf Stamm und aufrechten Armen: insgesamt ${Math.round(totalWind(windyLoads))} N. Die steife Säule neigt sich am Scheitel um etwa 0,3 Grad.`), bones: WINDY, ground: "authored", contacts: [], thermal: { regions: regionsDay, emitters: closed }, loads: windyLoads },
  { id: "storm", title: T("Monsoon downburst (28 m/s)", "Monsun-Fallböe (28 m/s)"), description: T(`A summer thunderstorm downburst of 28 m/s at 10 m: ${Math.round(totalWind(stormLoads))} N of drag; the column leans by about 1.6 degrees at the apex (cantilever on the woody rib cylinder, E I about 0.7 MN m2, estimated) and the arms a little more. Waterlogged saguaros are thrown in such storms by root failure, which is not modelled.`, `Eine Fallböe eines Sommergewitters mit 28 m/s in 10 m Höhe: ${Math.round(totalWind(stormLoads))} N Winddruck; die Säule neigt sich am Scheitel um etwa 1,6 Grad (Kragarm auf dem Zylinder der Holzrippen, E I etwa 0,7 MN m2, geschätzt), die Arme etwas mehr. Wassergesättigte Saguaros werden in solchen Stürmen durch Wurzelversagen geworfen; das ist nicht modelliert.`), bones: STORM, ground: "authored", contacts: [], thermal: { regions: regionsDay, emitters: closed }, loads: stormLoads },
  { id: "snow", title: T("Rare snow", "Seltener Schnee"), description: T("A rare winter snowfall in the upper Sonoran Desert: caps of wet snow (6 cm, 300 kg/m3) on the woolly apices, which the felt and the dense young spines keep a few degrees warmer than the ribs (Nobel 1980). The column does not bend; frost below about -7 degC for many hours would kill the apices.", "Ein seltener Winterschneefall in der oberen Sonora-Wüste: Kappen aus nassem Schnee (6 cm, 300 kg/m3) auf den wolligen Scheiteln, die der Filz und die dichten jungen Dornen einige Grad wärmer halten als die Rippen (Nobel 1980). Die Säule biegt sich nicht; Frost unter etwa -7 °C über viele Stunden würde die Scheitel töten."), bones: {}, ground: "authored", contacts: [], thermal: { regions: { "trunk-chlorenchyma": { exposedFraction: 1, sunlitFraction: 0.1 }, "arm-chlorenchyma": { exposedFraction: 1, sunlitFraction: 0.1 }, "apex-areoles": { exposedFraction: 0.2, sunlitFraction: 0 }, "basal-bark": { exposedFraction: 1, sunlitFraction: 0.05 } }, emitters: { "transpiration-trunk": 0.15, "transpiration-arms": 0.15 } }, loads: snowLoads },
];

// Clips -------------------------------------------------------------------------------------------------------------

const PERIOD = 1.9;
function swayClip(): Json {
  const n = 12, keys: Json[] = [];
  for (let i = 0; i <= n; i++) {
    const ph = (2 * Math.PI * (i % n)) / n;
    const f = 0.2 * (1 + 0.45 * Math.sin(ph));
    const arm = 0.2 * (1 + 0.6 * Math.sin(ph - 0.7));
    const leans = leanFor(1).map((l) => ({ id: l.id, deg: r4(l.deg * (l.id.startsWith("arm") ? arm : f)) }));
    keys.push({ t: r4((PERIOD * i) / n), pose: "windy", ease: "smooth", bones: leanOffsets(RIG, {}, leans) });
  }
  return { id: "wind-sway", title: T("Wind sway", "Windschwingung"), description: T("A deterministic 1.9 s cycle in the 12 m/s breeze (about 0.53 Hz, the first bending mode of a 10 m column on its woody rib cylinder: f = 3.52 / (2 pi L^2) sqrt(E I / m), E I about 0.7 MN m2 and 120 kg/m, estimated): the column nods about its lean and the arms follow a quarter period later.", "Ein deterministischer Zyklus von 1,9 s im Wind von 12 m/s (etwa 0,53 Hz, die erste Biegeeigenform einer 10 m hohen Säule auf ihrem Zylinder aus Holzrippen: f = 3,52 / (2 pi L^2) Wurzel(E I / m), E I etwa 0,7 MN m2 und 120 kg/m, geschätzt): Die Säule nickt um ihre Neigung, die Arme folgen eine Viertelperiode später."), durationS: PERIOD, fps: 30, loop: true, rootMotion: "none", keys, contacts: [], phases: [], events: [], tracks: [] };
}
function gustClip(): Json {
  const ts = [0, 0.8, 1.6, 2.4, 3.2, 4.1, 5.0, 6.0];
  const amp = [0, 0.35, 0.75, 1, 0.55, 0.78, 0.6, 0.5];
  const keys = ts.map((t, i) => ({ t, pose: i === 0 ? "calm" : "storm", ease: "smooth", bones: i === 0 ? {} : leanOffsets(RIG, {}, leanFor(amp[i]!)) }));
  keys[0]!.pose = "calm";
  const loadIds = stormLoads.map((l) => l.id);
  const lt = [{ t: 0, v: 0.05 }, { t: 1.6, v: 0.6 }, { t: 2.4, v: 1 }, { t: 3.2, v: 0.5 }, { t: 4.1, v: 0.78 }, { t: 6, v: 0.5 }];
  return { id: "gust", title: T("Gust of a downburst", "Böe einer Fallböe"), description: T("A 6 s gust of a summer downburst: the drag rises to the storm value at 2.4 s, the column leans to 1.6 degrees and rings at its 0.53 Hz mode with the arms a beat behind, then settles in the remaining wind.", "Eine Böe von 6 s einer sommerlichen Fallböe: Der Winddruck steigt bis 2,4 s auf den Sturmwert, die Säule neigt sich bis 1,6 Grad und schwingt in ihrer Eigenform von 0,53 Hz nach, die Arme einen Takt später, und beruhigt sich im verbleibenden Wind."), durationS: 6, fps: 30, loop: false, rootMotion: "none", keys, contacts: [], phases: [{ id: "gust", fromS: 0, toS: 3.2, title: T("Gust", "Böe") }, { id: "settle", fromS: 3.2, toS: 6, title: T("Settling", "Abklingen") }], events: [{ t: 2.4, kind: "motion", title: T("Peak lean", "Größte Neigung") }], tracks: loadIds.map((id) => ({ target: `load:${id}`, ease: "smooth", keys: lt })) };
}
const CAM_SCHEDULE: [number, number][] = [[0, 0.95], [3, 1], [5, 0.85], [7, 0.35], [9, 0.05], [12, 0.03], [15, 0.03], [17, 0.12], [19, 0.45], [21, 0.8], [24, 0.95]];
function diurnalClip(): Json {
  const sun = (k: number): Json[] => [{ t: 0, v: 0 }, { t: 5.5, v: 0 }, { t: 7, v: 0.2 * k }, { t: 9, v: 0.42 * k }, { t: 12, v: 0.5 * k }, { t: 15, v: 0.44 * k }, { t: 17.5, v: 0.22 * k }, { t: 19, v: 0 }, { t: 24, v: 0 }];
  return {
    id: "diurnal-transpiration", title: T("Day of CAM transpiration", "CAM-Transpirationstag"), description: T("A 24 h cycle compressed to 24 s (1 s = 1 h, 00:00 to 24:00) on a calm summer day: the CAM stomata open after sunset, transpire most in the cool night, close in the early morning (phase II) and stay shut through the hot day, opening a little in the late afternoon (phase IV, Nobel 1988); the sunlit share of the column follows the sun. Stem temperatures are not authored; the massive water-filled stem lags the air by hours in the thermal solver.", "Ein 24-h-Zyklus auf 24 s gerafft (1 s = 1 h, 00:00 bis 24:00) an einem windstillen Sommertag: Die CAM-Spaltöffnungen öffnen sich nach Sonnenuntergang, transpirieren am meisten in der kühlen Nacht, schließen sich am frühen Morgen (Phase II) und bleiben den heißen Tag über zu, mit einer kleinen Öffnung am späten Nachmittag (Phase IV, Nobel 1988); der besonnte Anteil der Säule folgt der Sonne. Stammtemperaturen werden nicht vorgegeben; der massive wassergefüllte Stamm hinkt der Luft im thermischen Löser um Stunden nach."),
    durationS: 24, fps: 10, loop: true, rootMotion: "none",
    keys: [{ t: 0, pose: "night-stomata-open", ease: "smooth" }, { t: 6, pose: "night-stomata-open", ease: "smooth" }, { t: 12, pose: "calm", ease: "smooth" }, { t: 18, pose: "calm", ease: "smooth" }, { t: 24, pose: "night-stomata-open", ease: "smooth" }],
    contacts: [], phases: [{ id: "night", fromS: 0, toS: 6, title: T("Night, stomata open", "Nacht, Spaltöffnungen offen") }, { id: "morning", fromS: 6, toS: 12, title: T("Morning, stomata closing", "Morgen, Spaltöffnungen schließen") }, { id: "afternoon", fromS: 12, toS: 18, title: T("Hot afternoon, stomata shut", "Heißer Nachmittag, Spaltöffnungen zu") }, { id: "evening", fromS: 18, toS: 24, title: T("Evening, stomata opening", "Abend, Spaltöffnungen öffnen") }],
    events: [{ t: 6, kind: "thermal", title: T("Sunrise", "Sonnenaufgang") }, { t: 12, kind: "thermal", title: T("Solar noon", "Sonnenhöchststand") }, { t: 18.5, kind: "thermal", title: T("Sunset", "Sonnenuntergang") }],
    tracks: [
      ...["transpiration-trunk", "transpiration-arms"].map((id) => ({ target: `emitter:${id}`, ease: "smooth", keys: CAM_SCHEDULE.map(([t, v]) => ({ t, v })) })),
      { target: "region:trunk-chlorenchyma:sunlitFraction", ease: "smooth", keys: sun(1) },
      { target: "region:arm-chlorenchyma:sunlitFraction", ease: "smooth", keys: sun(1) },
      { target: "region:apex-areoles:sunlitFraction", ease: "smooth", keys: sun(1.8) },
      { target: "region:basal-bark:sunlitFraction", ease: "smooth", keys: sun(0.9) },
    ],
  };
}

// Geometry per scale ------------------------------------------------------------------------------------------------

type Parts = { drawn: Drawn[]; body: Json[]; thermal: Json[]; structure: Json[] };
const TONE = { trunk: 0.42, arm: 0.46, collar: 0.3, apex: 0.62 };
const frameOf = (id: string): Frame => B.frames[id]!;

/** 🏛️ All solids and drawn shapes of one scale. */
function build(scale: Scale): Parts {
  const f = FEATURE[scale];
  const drawn: Drawn[] = [], body: Json[] = [], thermal: Json[] = [], structure: Json[] = [];
  const ribbed = scale <= 100;
  const points = scale === 50 ? 8 : 4;
  const step = scale === 50 ? 0.25 : scale === 100 ? 0.5 : scale === 200 ? 0.75 : 1.5;
  const at = (bone: string, u: number): V3 => world(frameOf(bone), [u, 0, 0]);
  const trunkProfile = (t: { y0: number; y1: number }, u0: number, u1: number, shrinkEnd: boolean): [number, number][] => {
    const n = Math.max(2, Math.ceil((u1 - u0) / step));
    return Array.from({ length: n + 1 }, (_, i) => { const u = u0 + ((u1 - u0) * i) / n; const r = trunkR(t.y0 + Math.max(0, Math.min(u, t.y1 - t.y0))); return [u, r * (shrinkEnd && u > t.y1 - t.y0 ? 0.985 : 1)] as [number, number]; });
  };
  for (const t of TRUNK) {
    const L = t.y1 - t.y0;
    const isCrown = t.id === "crown";
    const u0 = t.id === "trunk-1" ? 0 : -OV;
    const u1 = isCrown ? DOME_Y - t.y0 : L + OV;
    const parts: { id: string; a: number; b: number; region: string; tone: number; material: string; depth: number }[] = t.id === "trunk-1"
      ? [{ id: "collar", a: 0, b: COLLAR, region: "basal-bark", tone: TONE.collar, material: "saguaro-bark", depth: COLLAR_RIB_DEPTH }, { id: "trunk-1", a: COLLAR - 0.04, b: u1, region: "trunk-chlorenchyma", tone: TONE.trunk, material: "saguaro-stem", depth: RIB_DEPTH }]
      : [{ id: t.id, a: u0, b: u1, region: "trunk-chlorenchyma", tone: TONE.trunk, material: "saguaro-stem", depth: RIB_DEPTH }];
    for (const p of parts) {
      const prof = trunkProfile(t, p.a, p.b, !isCrown && p.b > L);
      const fit = p.id === "collar" ? 1.012 : 1;
      if (ribbed) {
        const r0 = prof[0]![1] * fit, r1 = prof[prof.length - 1]![1] * fit, rm = Math.max(r0, r1);
        body.push(ribbedColumn(p.id, t.id, RIBS, rm, p.depth * (scale === 50 ? 1 : 0.9), p.a, p.b, [r4(r0 / rm), r4(r1 / rm)], 0, points, { material: p.material, role: "body", tone: p.tone }));
      } else body.push(revolveSolid(p.id, t.id, prof.map(([u, r]) => [u, r * fit] as [number, number]), { material: p.material, role: "body", tone: p.tone }));
      thermal.push({ id: `th-${p.id}`, of: p.id, region: p.region });
      const depth = p.depth * (scale === 50 ? 1 : 0.9), widest = Math.max(prof[0]![1], prof[prof.length - 1]![1]) * fit;
      const across = ribbed ? Math.max(...ribbedProfile(RIBS, widest, depth, 0, points).map(([, w]) => Math.abs(w))) / widest : 1;
      const straight = ribbed ? [prof[0]!, prof[prof.length - 1]!] : prof;
      const round = (k: number) => revolvedShape(p.id, t.id, straight.map(([u, r]) => [r5(u), r5(r * fit * k)] as [number, number]), { tone: p.tone, region: p.region });
      drawn.push({ shape: round(1), at: at(t.id, (p.a + p.b) / 2), ...(ribbed ? { views: ["elevation"] } : {}) });
      if (ribbed) drawn.push({ shape: round(across), at: at(t.id, (p.a + p.b) / 2), views: ["front"] });
      if (ribbed) drawn.push({ shape: ribbedShape(p.id, t.id, RIBS, widest, depth, p.a, p.b, 0, points, { tone: p.tone, region: p.region }), at: at(t.id, (p.a + p.b) / 2), views: ["plan"] });
    }
    if (isCrown) {
      const dome = domeProfile(0.218, APEX_Y - DOME_Y, scale === 50 ? 10 : scale === 100 ? 7 : scale === 200 ? 5 : 3, DOME_Y - t.y0);
      body.push(revolveSolid("apex", "crown", dome, { material: "saguaro-areole-wool", role: "body", tone: TONE.apex }));
      thermal.push({ id: "th-apex", of: "apex", region: "apex-areoles" });
      drawn.push({ shape: revolvedShape("apex", "crown", dome, { tone: TONE.apex, region: "apex-areoles" }), at: at("crown", 1.2) });
    }
    if (scale <= 100) {
      const every = scale === 50 ? 1 : 2;
      for (let k = 0; k < RIBS; k += every) {
        const phi = (360 * k) / RIBS;
        const a = t.id === "trunk-1" ? COLLAR + 0.05 : 0.02, b = isCrown ? DOME_Y - t.y0 : L - 0.02;
        const ys = Array.from({ length: 5 }, (_, i) => a + ((b - a) * i) / 4);
        const pts = ys.map((u) => [u, trunkR(t.y0 + u) * Math.cos(phi * Math.PI / 180) * 0.999] as [number, number]);
        const z = (u: number) => trunkR(t.y0 + u) * Math.sin(phi * Math.PI / 180);
        const kz = (z(b) - z(a)) / (b - a), zs = z(a) - kz * a, ze = zs + kz * L;
        const shape = { id: `${t.id}-rib-${k + 1}`, bone: t.id, role: "detail", filled: false, tone: 0.2, kind: "poly", points: pts.map(([u, v]) => [r5(u), r5(v)]), closed: false, solid: { kind: "extruded", sweep: "w", z0: r5(zs - 0.001), z1: r5(zs + 0.001), zTip: [r5(ze - 0.001), r5(ze + 0.001)] } };
        drawn.push({ shape, at: world(frameOf(t.id), [(a + b) / 2, trunkR(t.y0 + (a + b) / 2) * Math.cos(phi * Math.PI / 180), z((a + b) / 2)]), views: crestViews(trunkR(t.y0 + b), phi * Math.PI / 180, scale) });
      }
    }
  }
  for (const g of ARM_GEO) {
    g.a.segs.forEach((_, i) => {
      const bone = armBone(g.a, i), L = g.lengths[i]!;
      const last = i === 5;
      const u0 = i === 0 ? 0 : -EXT, u1 = last ? L - g.a.tip : L + EXT;
      const n = i === 0 ? (scale <= 100 ? 6 : 3) : last ? Math.max(2, Math.ceil((u1 - u0) / step)) : 1;
      const prof: [number, number][] = Array.from({ length: n + 1 }, (_, k) => { const u = u0 + ((u1 - u0) * k) / n; return [u, g.r(i, Math.max(0, Math.min(u, L)))] as [number, number]; });
      if (ribbed && i > 0) body.push(ribbedColumn(bone, bone, ARM_RIBS, g.a.r, ARM_RIB_DEPTH * (scale === 50 ? 1 : 0.9), u0, u1, [1, 1], 0, points, { material: "saguaro-stem", role: "body", tone: TONE.arm }));
      else if (ribbed) {
        body.push(revolveSolid(`${bone}-neck`, bone, [0, 0.04, 0.08, 0.12, 0.16, 0.2].map((u) => [u, g.r(0, u)] as [number, number]), { material: "saguaro-stem", role: "body", tone: TONE.arm }));
        const rs = g.r(0, 0.17), re = g.a.r;
        body.push(ribbedColumn(bone, bone, ARM_RIBS, re, ARM_RIB_DEPTH * (scale === 50 ? 1 : 0.9), 0.17, u1, [r4(rs / re), 1], 0, points, { material: "saguaro-stem", role: "body", tone: TONE.arm }));
        thermal.push({ id: `th-${bone}-neck`, of: `${bone}-neck`, region: "arm-chlorenchyma" });
      } else body.push(revolveSolid(bone, bone, prof, { material: "saguaro-stem", role: "body", tone: TONE.arm }));
      thermal.push({ id: `th-${bone}`, of: bone, region: "arm-chlorenchyma" });
      const depth = ARM_RIB_DEPTH * (scale === 50 ? 1 : 0.9), look = { tone: TONE.arm, region: "arm-chlorenchyma" };
      if (ribbed && i === 0) {
        drawn.push({ shape: revolvedShape(`${bone}-neck`, bone, [0, 0.04, 0.08, 0.12, 0.16, 0.2].map((u) => [u, g.r(0, u)] as [number, number]), look), at: at(bone, 0.1) });
        const rs = g.r(0, 0.17) / g.a.r;
        for (let k = 0; k < FLUTE_STEPS; k++) {
          const [a, b] = [0.17 + ((u1 - 0.17) * k) / FLUTE_STEPS, 0.17 + ((u1 - 0.17) * (k + 1)) / FLUTE_STEPS];
          const s = rs + ((1 - rs) * (k + 0.5)) / FLUTE_STEPS;
          drawn.push({ shape: ribbedShape(`${bone}-flute-${k + 1}`, bone, ARM_RIBS, g.a.r * s, depth * s, a, b, 0, points, look), at: at(bone, (a + b) / 2) });
        }
      } else drawn.push({ shape: ribbed ? ribbedShape(bone, bone, ARM_RIBS, g.a.r, depth, u0, u1, 0, points, look) : revolvedShape(bone, bone, prof, look), at: at(bone, L / 2) });
      if (last) {
        const dome = domeProfile(g.a.r, g.a.tip, scale === 50 ? 8 : scale === 100 ? 6 : scale === 200 ? 4 : 3, L - g.a.tip);
        body.push(revolveSolid(`${bone}-apex`, bone, dome, { material: "saguaro-areole-wool", role: "body", tone: TONE.apex }));
        thermal.push({ id: `th-${bone}-apex`, of: `${bone}-apex`, region: "apex-areoles" });
        drawn.push({ shape: revolvedShape(`${bone}-apex`, bone, dome, { tone: TONE.apex, region: "apex-areoles" }), at: at(bone, L) });
        if (scale <= 100) {
          const every = scale === 50 ? 1 : 2;
          for (let k = 0; k < ARM_RIBS; k += every) {
            const phi = (360 * k) / ARM_RIBS * Math.PI / 180;
            const a = 0.04, b = L - g.a.tip;
            const fr = frameOf(bone);
            const pts = [0, 1, 2, 3].map((j) => [a + ((b - a) * j) / 3, g.a.r * Math.cos(phi) * 0.999] as [number, number]);
            const zw = fr.o[2] + g.a.r * Math.sin(phi);
            drawn.push({ shape: { id: `${bone}-rib-${k + 1}`, bone, role: "detail", filled: false, tone: 0.2, kind: "poly", points: pts.map(([u, v]) => [r5(u), r5(v)]), closed: false, solid: { kind: "extruded", sweep: "w", z0: r5(zw - 0.001), z1: r5(zw + 0.001) } }, at: world(fr, [(a + b) / 2, g.a.r * Math.cos(phi), g.a.r * Math.sin(phi)]), views: crestViews(g.a.r, phi, scale) });
          }
        }
      }
    });
  }
  const pv = Math.max(0.004, f / 2 + 0.0008);
  for (const a of ANCHORS.filter((x) => x.kind === "emitter")) {
    const em = EMITTERS.find((e) => e.anchors.includes(a.id))!;
    const p: V3 = [a.x, a.y, (a as Json).z ?? 0];
    const radial = Math.hypot(p[1], p[2]);
    const nrm: V3 = [0, p[1] / radial, p[2] / radial];
    const c = add(p, mul(nrm, pv - 0.003));
    const q = quatToNormal(nrm);
    thermal.push({ ...ellipsoidSolid(`th-${a.id}`, a.bone, c, [r5(Math.max(0.06, 2.2 * pv)), r5(pv), r5(Math.max(0.05, 2 * pv))], { material: "saguaro-stem", role: "body", tone: 0.8 }, q), region: em.region, tags: ["emitter", em.id] });
  }
  const tube = (id: string, bone: string, R: number, th: number, u0: number, u1: number, member: string, rods: number): Json => {
    const t = Math.max(th, f + 0.001);
    if (scale === 200) return { id, kind: "revolve", bone, material: "saguaro-wood", role: "body", tone: 0.5, axis: [1, 0, 0], origin: [0, 0, 0], profile: [[r5(u0), r5(R + t / 2)], [r5(u1), r5(R + t / 2)]], member, part: "member" };
    if (scale === 50) return { id, kind: "capsule", bone, material: "saguaro-wood", role: "body", tone: 0.5, p0: [r5(u0), r5(R), 0], p1: [r5(u1), r5(R), 0], r0: r5(th / 2), r1: r5(th / 2), repeat: { count: rods, rotationAxis: [1, 0, 0], rotationRad: r5((2 * Math.PI) / rods), origin: [0, 0, 0] }, slender: true, member, part: "member" };
    if (scale === 500) return { id, kind: "revolve", bone, material: "saguaro-wood", role: "body", tone: 0.5, axis: [1, 0, 0], origin: [0, 0, 0], profile: [[r5(u0), r5(Math.max(0.0385, 0.9 * R))], [r5(u1), r5(Math.max(0.0385, 0.9 * R))]], member, part: "member" };
    return { id, kind: "revolve", bone, material: "saguaro-wood", role: "body", tone: 0.5, axis: [1, 0, 0], origin: [0, 0, 0], profile: [[r5(u0), r5(R - t / 2)], [r5(u1), r5(R - t / 2)], [r5(u1), r5(R + t / 2)], [r5(u0), r5(R + t / 2)], [r5(u0), r5(R - t / 2)]], member, part: "member" };
  };
  if (scale === 500) {
    structure.push({ id: "m-trunk", kind: "revolve", bone: "trunk-1", material: "saguaro-wood", role: "body", tone: 0.5, axis: [1, 0, 0], origin: [0, 0, 0], profile: [[0.02, 0.135], [DOME_Y + 0.2, 0.135], [APEX_Y - 0.04, 0.03]], member: "trunk-1", members: ["trunk-2", "trunk-3", "crown"], part: "member" });
    for (const g of ARM_GEO) {
      const pts: V3[] = [...g.frames.map((fr) => fr.o), world(g.frames[5]!, [g.lengths[5]! - g.a.tip * 0.3, 0, 0])];
      const side = g.frames[0]!.w;
      const secs = pts.map((p, k) => {
        const tdir = unit(sub(pts[Math.min(k + 1, pts.length - 1)]!, pts[Math.max(k - 1, 0)]!));
        return { p, x: side, y: cross(tdir, side), a: Math.max(0.0385, 0.45 * g.a.r), b: Math.max(0.0385, 0.45 * g.a.r) };
      });
      structure.push({ ...loftOf(`m-arm-${g.a.n}`, armBone(g.a, 0), g.frames[0]!, secs, { material: "saguaro-wood", role: "body", tone: 0.5 }), member: armBone(g.a, 0), members: g.a.segs.slice(1).map((_, i) => armBone(g.a, i + 1)), part: "member" });
    }
  } else for (const t of TRUNK) {
    const L = t.y1 - t.y0;
    structure.push(tube(`m-${t.id}`, t.id, 0.15, 0.03, t.id === "trunk-1" ? 0.02 : -0.04, t.id === "crown" ? DOME_Y - t.y0 + 0.2 : L + 0.04, t.id, 18));
    if (scale === 50 && t.id !== "trunk-1") structure.push({ ...ellipsoidSolid(`j-${t.id}`, t.id, [0, 0, 0], [Math.max(0.04, f / 2 + 0.001), 0.17, 0.17], { material: "saguaro-wood", role: "body", tone: 0.5 }), member: t.id, part: "joint" });
  }
  if (scale !== 500) for (const g of ARM_GEO) {
    g.a.segs.forEach((_, i) => {
      const bone = armBone(g.a, i), L = g.lengths[i]!;
      structure.push(tube(`m-${bone}`, bone, ARM_RING * g.a.r, 0.022, i === 0 ? 0.02 : -0.03, i === 5 ? L - g.a.tip * 0.6 : L + 0.03, bone, 14));
      if (scale === 50) structure.push({ ...ellipsoidSolid(`j-${bone}`, bone, [0, 0, 0], [Math.max(0.03, f / 2 + 0.001), r5(ARM_RING * g.a.r + 0.015), r5(ARM_RING * g.a.r + 0.015)], { material: "saguaro-wood", role: "body", tone: 0.5 }), member: bone, part: "joint" });
    });
  }
  const plateT = Math.max(0.04, f + 0.001), padT = Math.max(0.02, f + 0.001);
  structure.push({ id: "m-root-plate", kind: "revolve", bone: "root-plate", material: "saguaro-wood", role: "body", tone: 0.5, axis: [0, 1, 0], origin: [0, 0, 0], profile: [[0, 0.17], [r5(plateT), 0.17]], member: "root-plate", part: "member" });
  structure.push({ id: "s-base-support", kind: "revolve", bone: "root-plate", material: "saguaro-wood", role: "ground", tone: 0.5, axis: [0, 1, 0], origin: [0, 0, 0], profile: [[0, 0.19], [r5(padT), 0.19]], member: "root-plate", part: "support", anchor: "base-support" });
  structure.push({ id: "k-root-plate", kind: "ellipsoid", bone: "root-plate", material: "saguaro-wood", role: "body", tone: 0.5, radii: [0.08, r5(Math.max(0.03, f / 2 + 0.001)), 0.08], transform: { translation: [0, r5(Math.max(0.035, f / 2 + 0.002)), 0] }, member: "root-plate", part: "mass" });
  return { drawn, body, thermal, structure };
}

/** 🧭️ The rotation that turns the local v axis onto the unit normal `n` (in the (v, w) plane) about u. */
function quatToNormal(n: V3): [number, number, number, number] {
  const a = Math.atan2(n[2], n[1]);
  return [r5(Math.sin(a / 2)), 0, 0, r5(Math.cos(a / 2))];
}

const PARTS = Object.fromEntries(SCALES.map((s) => [s, build(s)])) as Record<Scale, Parts>;

// Texts per scale -----------------------------------------------------------------------------------------------------

const REP_TEXT: Record<Scale, Json> = {
  50: {
    silhouette: T("At 1:50 (1 m is 20 mm) every view is the true orthographic view of the 1:50 model: the fluted column with its waist at the ground, five arms rising from narrow necks through smooth elbows of six bones each, woolly domes on the trunk and every upright; the 18 rib crests of the trunk and the 14 of each upright are drawn as detail strokes in the side and front elevations.", "Bei 1:50 (1 m sind 20 mm) ist jede Ansicht die wahre Parallelprojektion des 1:50-Modells: die gerippte Säule mit ihrer Taille am Boden, fünf Arme, die aus schmalen Hälsen durch weiche Ellbogen aus je sechs Knochen aufsteigen, wollige Kuppeln auf Stamm und jedem Arm; die 18 Rippengrate des Stamms und die 14 jedes aufrechten Arms sind in Seiten- und Vorderansicht als Detailstriche gezeichnet."),
    retained: T("Retained: every arm with neck, elbow and apex; the corky base below 1.15 m; every rib crest of the trunk and the uprights.", "Beibehalten: jeder Arm mit Hals, Ellbogen und Scheitel; die verkorkte Basis unter 1,15 m; jeder Rippengrat von Stamm und aufrechten Armen."),
    merged: T("Merged: the areoles (every 2-3 cm on each crest) and their spine clusters into the rib crest strokes.", "Zusammengefasst: die Areolen (alle 2-3 cm auf jedem Grat) und ihre Dornenbüschel in die Striche der Rippengrate."),
    omitted: T("Omitted: single spines (up to 7 cm long, 1 mm thick), flowers and fruit (seasonal, not part of this standing form), woodpecker holes.", "Weggelassen: einzelne Dornen (bis 7 cm lang, 1 mm dick), Blüten und Früchte (jahreszeitlich, nicht Teil dieser stehenden Form), Spechthöhlen."),
    thinElements: T("Rib crests are 0.18 mm detail strokes on the body fill; the rib valleys are not drawn separately.", "Rippengrate sind Detailstriche von 0,18 mm auf der Körperfläche; die Rippentäler sind nicht eigens gezeichnet."),
    hatching: T("None.", "Keine."), styles: T("Trunk and arms mid grey, the corky base dark, the woolly apices light.", "Stamm und Arme mittelgrau, die verkorkte Basis dunkel, die wolligen Scheitel hell."), overlays: T("Trunk, arms, base and apices carry their four regions; strokes stay neutral.", "Stamm, Arme, Basis und Scheitel tragen ihre vier Regionen; Striche bleiben neutral."),
  },
  100: {
    silhouette: T("At 1:100 (1 m is 10 mm) the column and its five arms are the true views of the 1:100 model, every second rib crest drawn on the trunk and the uprights.", "Bei 1:100 (1 m sind 10 mm) sind Säule und fünf Arme die wahren Ansichten des 1:100-Modells, jeder zweite Rippengrat auf Stamm und aufrechten Armen gezeichnet."),
    retained: T("Retained: necks, elbows, apices, the corky base.", "Beibehalten: Hälse, Ellbogen, Scheitel, die verkorkte Basis."), merged: T("Merged: the 18 crests into 9 strokes (the trunk is 5 mm wide on paper).", "Zusammengefasst: die 18 Grate in 9 Striche (der Stamm ist auf dem Papier 5 mm breit)."), omitted: T("Omitted: areoles and spines.", "Weggelassen: Areolen und Dornen."),
    thinElements: T("Crest strokes 0.13 mm.", "Gratstriche 0,13 mm."), hatching: T("None.", "Keine."), styles: T("As at 1:50.", "Wie bei 1:50."), overlays: T("Four regions as at 1:50.", "Vier Regionen wie bei 1:50."),
  },
  200: {
    silhouette: T("At 1:200 (1 m is 5 mm) the plant is a smooth column with five arms of the 1:200 model; the ribs are no longer drawn.", "Bei 1:200 (1 m sind 5 mm) ist die Pflanze eine glatte Säule mit fünf Armen des 1:200-Modells; die Rippen werden nicht mehr gezeichnet."),
    retained: T("Retained: the candelabra silhouette with necks and elbows, the corky base and the apices as separate tones.", "Beibehalten: der Kandelaber-Umriss mit Hälsen und Ellbogen, die verkorkte Basis und die Scheitel als eigene Töne."), merged: T("Merged: the 34 mm deep ribs into the smooth crest circle (barely one smallest feature).", "Zusammengefasst: die 34 mm tiefen Rippen in den glatten Gratkreis (kaum ein kleinstes Merkmal)."), omitted: T("Omitted: crest strokes.", "Weggelassen: Gratstriche."),
    thinElements: T("None below the print minimum.", "Keine unter dem Druckminimum."), hatching: T("None.", "Keine."), styles: T("As at 1:50.", "Wie bei 1:50."), overlays: T("Four regions as at 1:50.", "Vier Regionen wie bei 1:50."),
  },
  500: {
    silhouette: T("At 1:500 (1 m is 2 mm) the saguaro is the 20 mm candelabra of the 1:500 model: column, five arms and their domes as revolved outlines.", "Bei 1:500 (1 m sind 2 mm) ist der Saguaro der 20 mm hohe Kandelaber des 1:500-Modells: Säule, fünf Arme und ihre Kuppeln als rotierte Umrisse."),
    retained: T("Retained: the five arms at their true heights and azimuths, the elbows.", "Beibehalten: die fünf Arme in ihren wahren Höhen und Richtungen, die Ellbogen."), merged: T("Merged: ribs, neck constrictions and the profile of the trunk into a few stations.", "Zusammengefasst: Rippen, Halsverengungen und das Stammprofil in wenige Stationen."), omitted: T("Omitted: every detail below 75 mm.", "Weggelassen: jedes Detail unter 75 mm."),
    thinElements: T("The youngest arm (29 cm) stays at its true width, 0.58 mm on paper.", "Der jüngste Arm (29 cm) bleibt in seiner wahren Breite, 0,58 mm auf dem Papier."), hatching: T("None.", "Keine."), styles: T("As at 1:50.", "Wie bei 1:50."), overlays: T("Four regions as at 1:50.", "Vier Regionen wie bei 1:50."),
  },
};

const count = (s: Scale, l: "body" | "thermal" | "structure") => PARTS[s][l].length;
const VOL_TEXT: Record<Scale, () => Json[]> = {
  50: () => [
    T(`Body at 1:50 (${count(50, "body")} solids, smallest feature 7.5 mm): the trunk as four fluted columns of 18 ribs (34 mm deep, 8 profile points per rib) over the corky base with shallower ribs, the woolly apex a revolved dome; each arm a revolved neck and six fluted bones of 14 ribs that overlap 35 mm at every joint, so the elbow bends smoothly in steps of 14-16 degrees, and a woolly dome.`, `Körper bei 1:50 (${count(50, "body")} Körper, kleinstes Merkmal 7,5 mm): der Stamm als vier gerippte Säulen mit 18 Rippen (34 mm tief, 8 Profilpunkte je Rippe) über der verkorkten Basis mit flacheren Rippen, der wollige Scheitel eine rotierte Kuppel; jeder Arm ein rotierter Hals und sechs gerippte Knochen mit 14 Rippen, die sich an jedem Gelenk um 35 mm überlappen, so dass der Ellbogen in Schritten von 14-16 Grad weich biegt, und eine wollige Kuppel.`),
    T(`Thermal at 1:50 (${count(50, "thermal")} solids): every body solid by reference in its region (trunk, arms, apices, base) and eleven outlet patches on the stem surface.`, `Thermik bei 1:50 (${count(50, "thermal")} Körper): jeder Körper per Verweis in seiner Region (Stamm, Arme, Scheitel, Basis) und elf Auslassflächen auf der Stammoberfläche.`),
    T(`Structure at 1:50 (${count(50, "structure")} solids): the woody rib skeleton itself, 18 rods of 3 cm on a circle of 15 cm in every trunk bone and 14 rods of 2.2 cm on half the arm radius in every arm bone (each a slender capsule repeated about the bone axis), joints at every bone that tie the rods of neighbouring bones, the root plate, the support pad and the root mass.`, `Tragwerk bei 1:50 (${count(50, "structure")} Körper): das Skelett der Holzrippen selbst, 18 Stäbe von 3 cm auf einem Kreis von 15 cm in jedem Stammknochen und 14 Stäbe von 2,2 cm auf dem halben Armradius in jedem Armknochen (jeder eine schlanke Kapsel, um die Knochenachse wiederholt), Gelenke an jedem Knochen, die die Stäbe benachbarter Knochen verbinden, der Wurzelteller, das Auflager und die Wurzelmasse.`),
  ],
  100: () => [
    T(`Body at 1:100 (${count(100, "body")} solids, smallest feature 15 mm): as 1:50 with ribs of four profile points each (crest, flanks, valley) and 31 mm deep.`, `Körper bei 1:100 (${count(100, "body")} Körper, kleinstes Merkmal 15 mm): wie 1:50 mit Rippen aus je vier Profilpunkten (Grat, Flanken, Tal) und 31 mm Tiefe.`),
    T(`Thermal at 1:100 (${count(100, "thermal")} solids): by reference, eleven outlets.`, `Thermik bei 1:100 (${count(100, "thermal")} Körper): per Verweis, elf Auslässe.`),
    T(`Structure at 1:100 (${count(100, "structure")} solids): the rods of each bone merged into one tube member of the rib cylinder (30 and 22 mm wall), no joints (the tubes of neighbouring bones overlap), root plate, support, mass.`, `Tragwerk bei 1:100 (${count(100, "structure")} Körper): die Stäbe jedes Knochens zu einem Rohrstab des Rippenzylinders zusammengefasst (Wand 30 und 22 mm), keine Gelenke (die Rohre benachbarter Knochen überlappen), Wurzelteller, Auflager, Masse.`),
  ],
  200: () => [
    T(`Body at 1:200 (${count(200, "body")} solids, smallest feature 30 mm): smooth revolved column and arms (the 34 mm ribs merged into the crest circle), stations every 0.75 m, necks of three stations.`, `Körper bei 1:200 (${count(200, "body")} Körper, kleinstes Merkmal 30 mm): glatte rotierte Säule und Arme (die 34 mm tiefen Rippen in den Gratkreis aufgenommen), Stationen alle 0,75 m, Hälse aus drei Stationen.`),
    T(`Thermal at 1:200 (${count(200, "thermal")} solids): by reference, eleven outlets of one feature.`, `Thermik bei 1:200 (${count(200, "thermal")} Körper): per Verweis, elf Auslässe von einem Merkmal.`),
    T(`Structure at 1:200 (${count(200, "structure")} solids): the rib cylinder of each bone as one solid rod of its outer radius (a 22-30 mm wall is below 30 mm), no joints at this scale.`, `Tragwerk bei 1:200 (${count(200, "structure")} Körper): der Rippenzylinder jedes Knochens als ein Vollstab seines Außenradius (eine Wand von 22-30 mm liegt unter 30 mm), keine Gelenke in diesem Maßstab.`),
  ],
  500: () => [
    T(`Body at 1:500 (${count(500, "body")} solids, smallest feature 75 mm): the column with stations every 1.5 m, arms of plain cylinders, domes of three stations.`, `Körper bei 1:500 (${count(500, "body")} Körper, kleinstes Merkmal 75 mm): die Säule mit Stationen alle 1,5 m, Arme aus einfachen Zylindern, Kuppeln aus drei Stationen.`),
    T(`Thermal at 1:500 (${count(500, "thermal")} solids): by reference, eleven outlets of one feature.`, `Thermik bei 1:500 (${count(500, "thermal")} Körper): per Verweis, elf Auslässe von einem Merkmal.`),
    T(`Structure at 1:500 (${count(500, "structure")} solids): one rod for the whole trunk (four bones) and one curved rod per arm along its six bones, root plate, support, mass.`, `Tragwerk bei 1:500 (${count(500, "structure")} Körper): ein Stab für den ganzen Stamm (vier Knochen) und ein gebogener Stab je Arm entlang seiner sechs Knochen, Wurzelteller, Auflager, Masse.`),
  ],
};

// Symbol --------------------------------------------------------------------------------------------------------------

function symbol(): Json {
  const bars: Record<"elevation" | "front" | "plan", [number, number, number, number, number, number][]> = { elevation: [], front: [], plan: [] };
  const proj = { elevation: (p: V3) => [p[0], p[1]], front: (p: V3) => [p[2], p[1]], plan: (p: V3) => [p[0], p[2]] } as const;
  for (const view of ["elevation", "front"] as const) {
    bars[view].push([0, 0.3, 0, APEX_Y - 0.218, 0.23, 0.218]);
    for (const g of ARM_GEO) g.frames.forEach((fr, i) => { const a = proj[view](fr.o), b = proj[view](world(fr, [g.lengths[i]!, 0, 0])); bars[view].push([a[0]!, a[1]!, b[0]!, b[1]!, g.a.r, g.a.r]); });
    bars[view].push([0, 0, 0, 0.3, 0.21, 0.21]);
  }
  for (const g of ARM_GEO) g.frames.forEach((fr, i) => { const a = proj.plan(fr.o), b = proj.plan(world(fr, [g.lengths[i]!, 0, 0])); bars.plan.push([a[0]!, a[1]!, b[0]!, b[1]!, g.a.r, g.a.r]); });
  bars.plan.push([0, 0, 0.0001, 0, 0.25, 0.25]);
  const geo: Json = {};
  for (const view of ["elevation", "front", "plan"] as const) {
    const rings = barOutline(bars[view], 0.05);
    geo[view] = rings.map((ring, i) => ({ id: `extent-${i + 1}`, bone: "root-plate", role: "body", z: 2 + i * 0.01, filled: false, tone: 0.5, kind: "poly", points: view === "elevation" ? ring.map(([x, y]) => [x, y < 0.02 ? 0 : y]) : ring.map(([x, y]) => [x, view === "front" && y < 0.02 ? 0 : y]), closed: true }));
  }
  geo.plan.push({ id: "trunk", bone: "root-plate", role: "detail", z: 3, filled: false, tone: 0.3, kind: "ellipse", cx: 0, cy: 0, rx: 0.25, ry: 0.25 });
  return { id: "saguaro-symbol", title: T("Planting-plan sign with the true extent of trunk and arms", "Pflanzplanzeichen mit der wahren Ausdehnung von Stamm und Armen"), description: T("A conventional sign for planting plans: the outline of the trunk and its five arms in every view at their true size, with the trunk circle in plan; a separate symbol, never used silently for a pictorial scale.", "Ein konventionelles Zeichen für Pflanzpläne: der Umriss des Stamms und seiner fünf Arme in jeder Ansicht in wahrer Größe, im Grundriss mit dem Stammkreis; ein eigenes Symbol, nie stillschweigend für einen Bildmaßstab verwendet."), semantics: "true-extent", units: "model-m", denominators: [500, 200, 100], pens: { body: 0.18, detail: 0.13, texture: null, glazing: null, ground: 0.25 }, hatches: [], geometry: geo, dimensionalNote: T("Outline at the true extent of the default pose.", "Umriss in der wahren Ausdehnung der Standardpose.") };
}

// Asset ---------------------------------------------------------------------------------------------------------------

function bounds(): { x: [number, number]; y: [number, number]; z: [number, number] } {
  const b = { x: [Infinity, -Infinity] as [number, number], y: [0, -Infinity] as [number, number], z: [Infinity, -Infinity] as [number, number] };
  const push = (p: V3) => { b.x = [Math.min(b.x[0], p[0]), Math.max(b.x[1], p[0])]; b.y[1] = Math.max(b.y[1], p[1]); b.z = [Math.min(b.z[0], p[2]), Math.max(b.z[1], p[2])]; };
  for (let k = 0; k < 72; k++) {
    const a = (k * 5 * Math.PI) / 180;
    for (const y of [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 9.3, 9.6, 9.9]) push([trunkR(y) * Math.cos(a), y, trunkR(y) * Math.sin(a)]);
    for (const g of ARM_GEO) g.frames.forEach((fr, i) => { const L = g.lengths[i]!; for (const u of [0, L * 0.5, L]) push(world(fr, [u, g.r(i, u) * Math.cos(a), g.r(i, u) * Math.sin(a)])); });
  }
  push([0, APEX_Y, 0]);
  return b;
}

export function asset(): Json {
  const b = bounds();
  const mass = RIG.bones.reduce((s: number, x: Json) => s + (x.mass?.kg ?? 0), 0);
  const representations: Json = {}, volumes: Json = {};
  for (const s of SCALES) {
    representations[String(s)] = { status: "authored", denominator: s, validRange: RANGES[s], pens: PENS[s], hatches: [], geometry: views(PARTS[s].drawn), decisions: REP_TEXT[s] };
    volumes[String(s)] = { status: "authored", denominator: s, body: PARTS[s].body, thermal: PARTS[s].thermal, structure: PARTS[s].structure, decisions: VOL_TEXT[s]() };
  }
  return {
    id: ID, category: "vegetation",
    rig: RIG, anchors: ANCHORS,
    title: T("Saguaro Cactus", "Saguaro-Kaktus"),
    summary: T(`A mature saguaro (Carnegiea gigantea) of the Sonoran Desert, ${r4(b.y[1]).toFixed(1)} m tall, about 150 years old: a fluted green column of 18 ribs (0.49 m across at the crests) with a corky brown base and five arms that rise from narrow necks through smooth elbows to woolly apices; inside, a cylinder of woody ribs carries the water-filled stem of about ${Math.round(mass / 100) * 100} kg. A CAM plant: its stomata open at night.`, `Ein ausgewachsener Saguaro (Carnegiea gigantea) der Sonora-Wüste, ${r4(b.y[1]).toFixed(1).replace(".", ",")} m hoch, etwa 150 Jahre alt: eine gerippte grüne Säule mit 18 Rippen (0,49 m über den Graten) mit verkorkter brauner Basis und fünf Armen, die aus schmalen Hälsen durch weiche Ellbogen zu wolligen Scheiteln aufsteigen; innen trägt ein Zylinder aus Holzrippen den wassergefüllten Stamm von etwa ${Math.round(mass / 100) * 100} kg. Eine CAM-Pflanze: Ihre Spaltöffnungen öffnen sich nachts.`),
    tags: ["cactus", "succulent", "desert", "cam", "columnar", "sonoran", "transpiration"],
    provenance: {
      author: "Semio staffage plant collection wave, worker PW-11", method: "handcrafted", created: "2026-11-22", references: Object.values(REF),
      notes: T("Trunk profile, the five arms (azimuth, insertion height, the six bones of neck, elbow and upright), the rib counts (18 on the trunk, 14 on the arms) and depths are stated by hand after Steenbergh and Lowe (1977), Gibson and Nobel (1986) and Niklas and Buchman (1994); masses are the modelled volume times 960 kg/m3 (about 90 % water, Nobel 1988) and lie above the 1.4-2.2 t that the National Park Service gives for a typical mature plant, because this one has five long arms. Stiffness, thermal capacity, conductance and drag are authored estimates with the derivations given in the poses and assumptions; nothing is measured.", "Stammprofil, die fünf Arme (Richtung, Ansatzhöhe, die sechs Knochen von Hals, Ellbogen und aufrechtem Teil), die Rippenzahlen (18 am Stamm, 14 an den Armen) und Rippentiefen sind von Hand nach Steenbergh und Lowe (1977), Gibson und Nobel (1986) sowie Niklas und Buchman (1994) festgelegt; die Massen sind das modellierte Volumen mal 960 kg/m3 (etwa 90 % Wasser, Nobel 1988) und liegen über den 1,4-2,2 t, die der National Park Service für eine typische ausgewachsene Pflanze angibt, weil diese fünf lange Arme hat. Steifigkeit, Wärmekapazität, Leitfähigkeit und Widerstand sind Autorenschätzungen mit den in Posen und Annahmen angegebenen Herleitungen; nichts ist gemessen."),
    },
    dimensions: { heightM: r4(b.y[1]), widthM: r4(b.x[1] - b.x[0]), depthM: r4(b.z[1] - b.z[0]), massKg: Math.round(mass) },
    materials: MATERIALS,
    representations,
    symbols: [symbol()],
    poses: POSES, clips: [swayClip(), gustClip(), diurnalClip()],
    thermal: {
      environment: ENV, passengers: [], operating: [], regions: REGIONS, emitters: EMITTERS,
      assumptions: [
        T(`Stem regions use the ribbed outer surface of the 1:50 model as exchange area (trunk ${r4(AREA.trunk)} m2, arms ${r4(AREA.arms)} m2; the 18 ribs add ${Math.round((ribRatio - 1) * 100)} % to the crest circle). CAM: stomata open at night (Nobel 1988); the maximum conductance 0.0008 m/s (about 0.03 mol/(m2 s)) is an authored estimate.`, `Die Stammregionen verwenden die gerippte Außenfläche des 1:50-Modells als Austauschfläche (Stamm ${r4(AREA.trunk)} m2, Arme ${r4(AREA.arms)} m2; die 18 Rippen fügen dem Gratkreis ${Math.round((ribRatio - 1) * 100)} % hinzu). CAM: Spaltöffnungen öffnen nachts (Nobel 1988); die maximale Leitfähigkeit 0,0008 m/s (etwa 0,03 mol/(m2 s)) ist eine Autorenschätzung.`),
        T("Areal heat capacity 250 kJ/(m2 K) on the trunk and 200 kJ/(m2 K) on the arms: the water-filled tissue (about 4 MJ/(m3 K)) that a daily temperature wave reaches, about 6 cm deep (diffusivity 1.4e-7 m2/s); the massive stem therefore lags the air by hours (Nobel 1988).", "Flächenbezogene Wärmekapazität 250 kJ/(m2 K) am Stamm und 200 kJ/(m2 K) an den Armen: das wassergefüllte Gewebe (etwa 4 MJ/(m3 K)), das eine tägliche Temperaturwelle erreicht, etwa 6 cm tief (Temperaturleitfähigkeit 1,4e-7 m2/s); der massive Stamm hinkt der Luft daher um Stunden nach (Nobel 1988)."),
        T("Apices and the corky base are passive surfaces. The transpiration emitters release the latent flux of their stem region over six outlets on the trunk and five on the arm uprights, directed upwards.", "Scheitel und verkorkte Basis sind passive Oberflächen. Die Transpirationsemitter geben den latenten Strom ihrer Stammregion über sechs Auslässe am Stamm und fünf an den aufrechten Armen ab, nach oben gerichtet."),
      ],
    },
    structure: {
      gravityMS2: 9.80665, dynamics: "quasi-static", supports: ["base-support"],
      assumptions: [
        T("One rigid body per evaluated pose, fixed at the root plate; wind drag acts as line loads on the four trunk bones and the five uprights and as point loads on the elbows (cd 0.7 of a ribbed column, Talley et al. 2001; power-law profile after EN 1991-1-4), snow on the apices; gravity on every bone mass.", "Ein starrer Körper je ausgewerteter Pose, am Wurzelteller eingespannt; Winddruck wirkt als Linienlast auf die vier Stammknochen und die fünf aufrechten Arme und als Punktlast auf die Ellbogen (cd 0,7 einer gerippten Säule, Talley u. a. 2001; Potenzprofil nach EN 1991-1-4), Schnee auf die Scheitel; Schwerkraft auf jede Knochenmasse."),
        T("Sections are the cylinder of woody ribs (18 rods of 3 cm at r 15 cm in the trunk, 14 rods of 2.2 cm at half the arm radius (7-9.5 cm) in the arms; E 5 GPa, strength 40 MPa, authored estimates after Niklas and Buchman 1994 and Gibson and Nobel 1986); the turgid parenchyma adds weight but almost no stiffness. Root anchorage failure is not modelled.", "Querschnitte sind der Zylinder der Holzrippen (18 Stäbe von 3 cm bei r 15 cm im Stamm, 14 Stäbe von 2,2 cm beim halben Armradius (7-9,5 cm) in den Armen; E 5 GPa, Festigkeit 40 MPa, Autorenschätzungen nach Niklas und Buchman 1994 sowie Gibson und Nobel 1986); das pralle Parenchym bringt Gewicht, aber fast keine Steifigkeit. Versagen der Wurzelverankerung ist nicht modelliert."),
      ],
      clinical: false,
    },
    volumes,
    presentation: { defaultPose: "calm", defaultClip: "wind-sway", defaultScale: 200 },
    exports: EXPORTS,
    completion: "authored",
  };
}

export const DISTINCTION = T("The archive's only columnar cactus: a 10 m fluted saguaro of 18 ribs with five arms bending up through smooth elbows, a corky base and a woody rib skeleton inside; a CAM stem that transpires at night.", "Der einzige Säulenkaktus des Archivs: ein 10 m hoher gerippter Saguaro mit 18 Rippen und fünf Armen, die sich durch weiche Ellbogen aufrichten, mit verkorkter Basis und einem Skelett aus Holzrippen; ein CAM-Stamm, der nachts transpiriert.");
export const REPORT = { rigError, area: AREA, ribRatio, armRibRatio, collarRatio };
export { documentOf, poseFrames, sub };
