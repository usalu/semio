/** 🌳️ European beech (Fagus sylvatica), an open-grown park beech of about 150 years: a smooth grey stem with an elephant-foot flare, steep ascending main limbs above long, almost level lower limbs whose sprays hang low, and a wide, dense dome. Every number is chosen by hand here.
 * @see ./kit.ts
 * @see ../../📝️PW-02.md */
import { tx, type BranchSpec, type LimbSpec, type PoseSpec, type Scale, type Species, type Text } from "./kit.ts";
import { COMMON_REFERENCES, barkRegion, broadleafAssumptions, diurnalSunlit, leafRegion, structureAssumptions } from "./common.ts";

const TIERS: [string, number, number[], [number, number][], number, number, number, number, number, Text, Text][] = [
  ["t1", 3.1, [14, 86, 158, 229, 301], [[0, 30], [0.45, 12], [0.8, 2], [1, 6]], 9.2, 0.2, 0.04, 0.5, 9, tx("Lowest limb", "Unterster Ast"), tx("Lowest limb, outer part with hanging sprays", "Unterster Ast, Außenteil mit hängenden Zweigfächern")],
  ["t2", 5.0, [50, 124, 196, 268, 338], [[0, 44], [0.5, 28], [1, 14]], 8.8, 0.19, 0.042, 0.5, 9, tx("Second-tier limb", "Ast der zweiten Etage"), tx("Second-tier limb, outer part", "Ast der zweiten Etage, Außenteil")],
  ["t3", 7.6, [2, 93, 183, 272], [[0, 66], [0.6, 55], [1, 36]], 10.2, 0.25, 0.05, 0.5, 8, tx("Steep main limb", "Steiler Hauptast"), tx("Steep main limb, upper part", "Steiler Hauptast, oberer Teil")],
  ["t4", 11.6, [40, 112, 182, 252, 322], [[0, 58], [1, 30]], 7.2, 0.15, 0.035, 0.55, 10, tx("Fourth-tier limb", "Ast der vierten Etage"), tx("Fourth-tier limb, outer part", "Ast der vierten Etage, Außenteil")],
  ["t5", 16.4, [70, 160, 250, 340], [[0, 60], [1, 38]], 5.6, 0.1, 0.03, 0, 11, tx("Upper crown limb", "Ast der oberen Krone"), tx("", "")],
  ["t6", 21.2, [22, 142, 262], [[0, 64], [1, 46]], 4.0, 0.07, 0.025, 0, 12, tx("Top crown limb", "Ast der Kronenspitze"), tx("", "")],
];
const sideOf = (az: number): Text => {
  const k = Math.round((((az % 360) + 360) % 360) / 90) % 4;
  return [tx("east", "Ost"), tx("north", "Nord"), tx("west", "West"), tx("south", "Süd")][k]!;
};

const limbs: LimbSpec[] = [];
const branches: BranchSpec[] = [];
for (const [tier, y, azs, elev, L, r0, r1, split, limit, title, titleB] of TIERS) {
  azs.forEach((az, k) => {
    const id = `${tier}-${k + 1}`;
    const s = sideOf(az);
    limbs.push({ id, parent: "stem", at: y + 0.2 * k, az, elev, L: L * (1 + 0.05 * Math.cos(k * 1.9)), r0, r1, bend: 7 * Math.sin(k * 1.7 + y), ...(split ? { split } : {}), title: tx(`${title.en}, ${s.en}`, `${title.de}, ${s.de}`), ...(split ? { titleB: tx(`${titleB.en}, ${s.en}`, `${titleB.de}, ${s.de}`) } : {}), limit });
    const sc: Scale[][] = [[50, 100, 200], [50, 100], [50]];
    [[0.32, 38, 0.4], [0.58, -34, 0.34], [0.78, 30, 0.28]].forEach(([t, daz, fL], j) => {
      branches.push({ id: `${id}-${j + 1}`, limb: id, t: t!, az: az + daz!, elev: [[0, Math.max(8, elev[0]![1] * 0.55)], [1, tier === "t1" ? -4 : 10]], L: L * fL!, r0: r0 * 0.4, r1: 0.018, bend: daz! * 0.25, scales: sc[j]! });
    });
  });
}

/** 🌐️ Crown envelope: a wide dense dome, widest (10.2 m) at 11 m, the lowest sprays hanging to 2.2 m, top at 29.6 m. */
const envelope = (y: number, az: number): number => {
  const [ym, base, top, R] = [11, 2.2, 29.6, 10.2];
  const t = y < ym ? (ym - y) / (ym - base) : (y - ym) / (top - ym);
  const p = y < ym ? 3.2 : 1.95;
  const f = Math.pow(Math.max(0, 1 - Math.pow(Math.min(1, t), p)), 1 / p);
  const a = (az * Math.PI) / 180;
  return R * f * (1 + 0.035 * Math.cos(a - 0.4) + 0.025 * Math.cos(2 * a + 1.2) + 0.015 * Math.cos(4 * a - 0.3));
};

const ring = (y: number, n: number, phase: number, a: number, b: number, extra: Partial<Species["scales"][50]["rings"][0]> = {}) => ({ y, n, phase, a, b, region: "sun" as const, jitter: 1, ck: 1.3, ...extra });
const core = (y: number, n: number, phase: number, a: number, b: number, reach: number) => ({ y, n, phase, a, b, region: "shade" as const, reach, ck: 1.1, jitter: 0.6 });

const calmRegions = { "trunk-bark": { exposedFraction: 1, sunlitFraction: 0.25 }, "limb-bark": { exposedFraction: 1, sunlitFraction: 0.12 }, "leaf-sun": { exposedFraction: 1, sunlitFraction: 0.9 }, "leaf-shade": { exposedFraction: 1, sunlitFraction: 0.04 } };
const poses: PoseSpec[] = [
  { id: "summer-calm", title: tx("Summer calm", "Sommerliche Windstille"), description: tx("July noon, light air of 1.2 m/s: the dense dome casts the deep shade beeches are known for; the sun leaves transpire at their full rate, the shade leaves of the inner crown little.", "Juli, Mittag, leiser Zug von 1,2 m/s: Die dichte Kuppel wirft den tiefen Schatten, für den Buchen bekannt sind; die Sonnenblätter transpirieren mit voller Rate, die Schattenblätter der Innenkrone wenig."), v10: 1.2, cd: 0.45, lean: 0, droop: 0, candlesOut: false, regions: calmRegions, emitters: { sun: 1, shade: 0.62 } },
  { id: "summer-breeze", title: tx("Summer breeze", "Sommerbrise"), description: tx("Moderate breeze of 5.5 m/s at 10 m (Beaufort 4): the long lower limbs rock and the dome leans slightly downwind; drag F = 0.5 rho cd A v^2 with cd 0.35 on the porous frontal crown area.", "Mäßige Brise von 5,5 m/s in 10 m Höhe (Beaufort 4): Die langen unteren Äste wiegen sich und die Kuppel neigt sich leicht in Windrichtung; Widerstand F = 0,5 rho cd A v^2 mit cd 0,35 auf der porösen Kronenstirnfläche."), v10: 5.5, cd: 0.35, lean: 0.16, droop: 0, candlesOut: false, regions: { ...calmRegions, "leaf-shade": { exposedFraction: 1, sunlitFraction: 0.07 } }, emitters: { sun: 0.95, shade: 0.6 } },
  { id: "gale", title: tx("Summer gale", "Sommersturm"), description: tx("Gale of 22 m/s at 10 m (Beaufort 9) in full leaf, overcast: the leaves fold into streamlined clusters (cd 0.22 after Vogel 1989); the broad crown of the open-grown beech gives a large base moment, the steep limbs lean downwind.", "Sturm von 22 m/s in 10 m Höhe (Beaufort 9) im vollen Laub, bedeckt: Die Blätter falten sich zu stromlinienförmigen Büscheln (cd 0,22 nach Vogel 1989); die breite Krone der frei stehenden Buche ergibt ein großes Fußmoment, die steilen Äste neigen sich in Windrichtung."), v10: 22, cd: 0.22, lean: 1.55, droop: 0, candlesOut: false,
    regions: { "trunk-bark": { exposedFraction: 1, sunlitFraction: 0.03, wettedness: 0.4 }, "limb-bark": { exposedFraction: 1, sunlitFraction: 0.02, wettedness: 0.4 }, "leaf-sun": { exposedFraction: 1, sunlitFraction: 0.08, wettedness: 0.2 }, "leaf-shade": { exposedFraction: 1, sunlitFraction: 0.01, wettedness: 0.1 } }, emitters: { sun: 0.35, shade: 0.2 } },
  { id: "wet-snow", title: tx("Early wet snow", "Früher Nassschnee"), description: tx("Early snow in late October on the still leafy beech (beeches keep their leaves long): 10 kg of wet snow per m2 of foliage plan area (order of magnitude from EN 1991-1-3) presses the long lower limbs down; calm, no sun, stomata nearly closed.", "Früher Schnee Ende Oktober auf die noch belaubte Buche (Buchen halten ihr Laub lange): 10 kg nasser Schnee je m2 Laubgrundfläche (Größenordnung nach EN 1991-1-3) drückt die langen unteren Äste nieder; windstill, keine Sonne, Stomata fast geschlossen."), v10: 1, cd: 0.45, lean: 0, droop: 2.6, snowKgM2: 10, candlesOut: false,
    regions: { "trunk-bark": { exposedFraction: 1, sunlitFraction: 0.01, wettedness: 1 }, "limb-bark": { exposedFraction: 1, sunlitFraction: 0.01, wettedness: 1 }, "leaf-sun": { exposedFraction: 1, sunlitFraction: 0.03, wettedness: 1 }, "leaf-shade": { exposedFraction: 1, sunlitFraction: 0.002, wettedness: 1 } }, emitters: { sun: 0.05, shade: 0.03 } },
  { id: "autumn-copper", title: tx("Autumn copper", "Herbstliches Kupferlaub"), description: tx("A clear day in early November: the leaves have turned copper-brown and their stomata are nearly closed, so the crown warms in the low sun with little transpiration; the colour itself is not geometric and shows only in the thermal binding.", "Ein klarer Tag Anfang November: Das Laub ist kupferbraun geworden und seine Stomata sind fast geschlossen, die Krone erwärmt sich daher in der tiefen Sonne mit wenig Transpiration; die Farbe selbst ist nicht geometrisch und zeigt sich nur in der thermischen Bindung."), v10: 2, cd: 0.45, lean: 0.03, droop: 0, candlesOut: false,
    regions: { "trunk-bark": { exposedFraction: 1, sunlitFraction: 0.35 }, "limb-bark": { exposedFraction: 1, sunlitFraction: 0.2 }, "leaf-sun": { exposedFraction: 1, sunlitFraction: 0.7 }, "leaf-shade": { exposedFraction: 1, sunlitFraction: 0.08 } }, emitters: { sun: 0.12, shade: 0.06 } },
  { id: "mast-year", title: tx("Mast year", "Mastjahr"), description: tx("Late September of a mast year: the spiny cupules with their beechnuts hang all over the outer crown, about 0.5 kg of fruit per m2 of crown plan area (order of magnitude of a full mast); the outer limbs bend a little under the load, calm.", "Ende September eines Mastjahres: Die stacheligen Fruchtbecher mit ihren Bucheckern hängen über die ganze Außenkrone, etwa 0,5 kg Früchte je m2 Kronengrundfläche (Größenordnung einer Vollmast); die äußeren Äste biegen sich unter der Last ein wenig, windstill."), v10: 1.2, cd: 0.45, lean: 0, droop: 0.5, candlesOut: false, fruitKgM2: 0.5, fruitTitle: tx("Weight of the beechnuts in their cupules", "Gewicht der Bucheckern in ihren Fruchtbechern"),
    regions: { "trunk-bark": { exposedFraction: 1, sunlitFraction: 0.22 }, "limb-bark": { exposedFraction: 1, sunlitFraction: 0.1 }, "leaf-sun": { exposedFraction: 1, sunlitFraction: 0.82 }, "leaf-shade": { exposedFraction: 1, sunlitFraction: 0.04 } }, emitters: { sun: 0.7, shade: 0.42 } },
];

/** 🍂️ A beech leaf mark: the oval blade with its midrib, as one stroke round and through the middle. */
function leafMark(c: { x: number; y: number }, size: number, deg: number): { x: number; y: number }[][] {
  const a = (deg * Math.PI) / 180;
  const pts: [number, number][] = [[-1, 0], [-0.55, 0.42], [0.1, 0.5], [0.7, 0.3], [1, 0], [0.7, -0.3], [0.1, -0.5], [-0.55, -0.42], [-0.92, -0.06], [1, 0]];
  return [pts.map(([x, y]) => ({ x: c.x + size * (x * Math.cos(a) - 0.62 * y * Math.sin(a)), y: c.y + size * (x * Math.sin(a) + 0.62 * y * Math.cos(a)) }))];
}

/** 🐘️ The smooth grey beech bark: a few horizontal elephant-skin creases above the flare and the curved branch-scar marks of the stem. */
function barkMark(k: number, view: string, stem: (y: number) => { x: number; z: number; r: number }): { x: number; y: number }[] | null {
  const rows = [0.38, 0.52, 0.66, 1.1, 1.7, 2.3, 2.75];
  if (k >= rows.length * 2) return null;
  const y = rows[k % rows.length]! + (k >= rows.length ? 0.07 : 0);
  const s = stem(y);
  const cx = view === "elevation" ? s.x : s.z;
  const side = k >= rows.length ? 1 : -1;
  const w = y < 0.8 ? 0.55 : 0.32;
  const x0 = cx + side * 0.3 * s.r;
  return [-1, -0.4, 0.3, 1].map((u) => ({ x: x0 + u * w * s.r * 0.6, y: y + (y < 0.8 ? 0.03 : 0.06) * (1 - u * u) }));
}

export const species: Species = {
  id: "european-beech",
  title: tx("European Beech", "Rotbuche"),
  summary: tx("A mature open-grown European beech (Fagus sylvatica) of about 30 m: a smooth grey stem with an elephant-foot flare, steep ascending main limbs above long, almost level lower limbs whose sprays hang low, and a wide, dense dome of small oval leaves that casts deep shade.", "Eine ausgewachsene, frei stehende Rotbuche (Fagus sylvatica) von etwa 30 m: ein glatter grauer Stamm mit elefantenfußartigem Anlauf, steil aufsteigende Hauptäste über langen, fast waagrechten unteren Ästen, deren Zweigfächer tief hängen, und eine weite, dichte Kuppel aus kleinen ovalen Blättern, die tiefen Schatten wirft."),
  distinction: tx("A 30 m open-grown European beech (Fagus sylvatica) with a smooth grey stem of 1.05 m on an elephant-foot flare, steep main limbs and a dense 20 m dome whose lowest sprays hang down to 2.2 m.", "Eine 30 m hohe, frei stehende Rotbuche (Fagus sylvatica) mit glattem grauem Stamm von 1,05 m auf elefantenfußartigem Anlauf, steilen Hauptästen und einer dichten Kuppel von 20 m, deren unterste Zweigfächer bis 2,2 m herabhängen."),
  tags: ["tree", "deciduous", "broadleaf", "park-tree", "dense-crown", "smooth-bark", "fruit", "wind", "transpiration"],
  created: "2026-11-22",
  notes: tx("Rig and drawings are authored by hand for an open-grown park beech of about 150 years: clear stem 3 m on a broad flare, a continuous leader to 27.5 m, twenty-six limbs in six tiers, the lowest long and almost level with hanging sprays, the middle ones steep as in open-grown beeches. Crown size and habit follow Roloff (2010) as orders of magnitude; wood properties, masses and conductances are authored estimates.", "Rig und Zeichnungen sind von Hand für eine frei stehende Parkbuche von etwa 150 Jahren entworfen: Schaft 3 m auf breitem Anlauf, eine durchgehende Stammverlängerung bis 27,5 m, sechsundzwanzig Äste in sechs Etagen, die untersten lang und fast waagrecht mit hängenden Zweigfächern, die mittleren steil wie bei frei stehenden Buchen. Kronengröße und Wuchs folgen Roloff (2010) als Größenordnungen; Holzeigenschaften, Massen und Leitfähigkeiten sind Autorenschätzungen."),
  references: [...COMMON_REFERENCES],
  materials: [
    { id: "beech-bark", title: tx("Smooth silver-grey beech bark", "Glatte silbergraue Buchenrinde"), emissivity: 0.95, solarAbsorptivity: 0.6, densityKgM3: 700, conductivityWmK: 0.15, source: "authored-estimate", reference: "wood-handbook-2021" },
    { id: "beech-bark-young", title: tx("Smooth grey bark of limbs and twigs", "Glatte graue Rinde der Äste und Zweige"), emissivity: 0.95, solarAbsorptivity: 0.63, densityKgM3: 700, conductivityWmK: 0.15, source: "authored-estimate", reference: "wood-handbook-2021" },
    { id: "beech-wood", title: tx("Green beech wood", "Frisches Buchenholz"), emissivity: 0.9, solarAbsorptivity: 0.55, densityKgM3: 1000, conductivityWmK: 0.17, elasticModulusPa: 9500000000, strengthPa: 50000000, source: "authored-estimate", reference: "wood-handbook-2021" },
    { id: "beech-leaf-sun", title: tx("Beech sun leaves", "Buchen-Sonnenblätter"), emissivity: 0.97, solarAbsorptivity: 0.5, source: "literature", reference: "jones-2014" },
    { id: "beech-leaf-shade", title: tx("Beech shade leaves (effective)", "Buchen-Schattenblätter (wirksam)"), emissivity: 0.97, solarAbsorptivity: 0.22, source: "authored-estimate", reference: "jones-2014" },
  ],
  barkMaterial: "beech-bark", youngBarkMaterial: "beech-bark-young", woodMaterial: "beech-wood", sunMaterial: "beech-leaf-sun", shadeMaterial: "beech-leaf-shade",
  density: 1000, foliageKgPerM2: 0.18,
  stem: {
    stations: [[0, 0, 0, 1.0], [0.1, 0, 0, 0.84], [0.25, 0, 0, 0.73], [0.5, 0.01, 0, 0.63], [0.8, 0.01, 0, 0.57], [1.3, 0.02, 0.01, 0.525], [2.2, 0.03, 0.02, 0.505], [3.0, 0.04, 0.02, 0.485], [5.0, 0.06, 0.01, 0.43], [7.5, 0.05, -0.02, 0.38], [10, 0.02, -0.04, 0.31], [12, -0.01, -0.04, 0.265], [15, -0.04, -0.02, 0.2], [17, -0.05, 0, 0.165], [20, -0.03, 0.02, 0.115], [22, -0.01, 0.03, 0.09], [25, 0.01, 0.02, 0.055], [27.5, 0.02, 0.01, 0.03]],
    bones: [
      { id: "root-plate", y0: 0, y1: 0.45, title: tx("Root plate and elephant-foot flare", "Wurzelteller und Elefantenfuß-Anlauf"), limit: 0, region: "trunk-bark" },
      { id: "trunk", y0: 0.45, y1: 3.0, title: tx("Clear stem", "Schaft"), limit: 3, region: "trunk-bark" },
      { id: "s1", y0: 3.0, y1: 7.5, title: tx("Stem in the lower crown", "Stamm in der unteren Krone"), limit: 3, region: "trunk-bark" },
      { id: "s2", y0: 7.5, y1: 12, title: tx("Stem in the middle crown", "Stamm in der mittleren Krone"), limit: 4, region: "trunk-bark" },
      { id: "s3", y0: 12, y1: 17, title: tx("Stem in the upper crown", "Stamm in der oberen Krone"), limit: 4, region: "limb-bark" },
      { id: "s4", y0: 17, y1: 22, title: tx("Stem below the top", "Stamm unter dem Wipfel"), limit: 5, region: "limb-bark" },
      { id: "leader", y0: 22, y1: 27.5, title: tx("Leader", "Wipfeltrieb"), limit: 7, region: "limb-bark" },
    ],
  },
  limbs, branches, envelope,
  scales: {
    50: {
      rings: [
        ring(3.05, 20, 6, 1.6, 0.85, { tilt: 16 }), ring(4.1, 22, 15, 1.6, 0.9, { tilt: 9 }), ring(5.2, 23, 4, 1.55, 0.92, { tilt: 4 }), ring(6.35, 24, 13, 1.55, 0.95), ring(7.5, 24, 2, 1.55, 0.95), ring(8.65, 24, 11, 1.55, 0.95), ring(9.8, 24, 20, 1.55, 0.95),
        ring(10.95, 24, 7, 1.55, 0.95), ring(12.1, 23, 16, 1.55, 0.95), ring(13.25, 22, 5, 1.5, 0.95), ring(14.4, 21, 14, 1.5, 0.95), ring(15.55, 20, 3, 1.5, 0.95), ring(16.7, 19, 12, 1.45, 0.95), ring(17.85, 17, 21, 1.45, 0.92),
        ring(19.0, 16, 8, 1.4, 0.92), ring(20.15, 14, 17, 1.35, 0.9), ring(21.3, 12, 2, 1.3, 0.9), ring(22.45, 11, 11, 1.25, 0.88), ring(23.6, 9, 20, 1.2, 0.86), ring(24.7, 7, 9, 1.15, 0.85), ring(25.8, 6, 18, 1.1, 0.85), ring(26.8, 4, 30, 1.0, 0.82), ring(27.7, 3, 10, 0.95, 0.8),
        core(5.4, 6, 30, 2.4, 1.6, 0.55), core(8.6, 7, 0, 2.6, 1.8, 0.56), core(12, 7, 26, 2.6, 1.8, 0.57), core(15.4, 6, 8, 2.4, 1.7, 0.56), core(18.8, 5, 36, 2.1, 1.6, 0.55), core(22.2, 4, 15, 1.8, 1.5, 0.52),
      ],
      top: { y: 28.75, a: 1.05, b: 0.85 }, tubeN: 10, rmin: 0.009, twigR: 0.013, leafMarks: 80, markSize: 0.09, barkMarks: 14,
    },
    100: {
      rings: [
        ring(3.2, 16, 10, 1.9, 1.0, { tilt: 14 }), ring(4.6, 18, 22, 1.9, 1.05, { tilt: 6 }), ring(6.0, 18, 4, 1.9, 1.1), ring(7.45, 19, 16, 1.9, 1.15), ring(8.9, 19, 8, 1.9, 1.15), ring(10.35, 19, 20, 1.9, 1.15),
        ring(11.8, 18, 12, 1.85, 1.15), ring(13.25, 17, 2, 1.85, 1.15), ring(14.7, 16, 14, 1.8, 1.15), ring(16.15, 15, 6, 1.75, 1.12), ring(17.6, 13, 18, 1.7, 1.1), ring(19.05, 12, 9, 1.65, 1.1),
        ring(20.5, 10, 21, 1.55, 1.05), ring(21.95, 9, 3, 1.5, 1.05), ring(23.4, 7, 15, 1.4, 1.0), ring(24.8, 5, 27, 1.3, 1.0), ring(26.1, 4, 8, 1.2, 0.95), ring(27.25, 3, 33, 1.1, 0.9),
        core(6.0, 5, 36, 2.8, 1.9, 0.55), core(10.5, 6, 10, 2.9, 2.0, 0.56), core(15.0, 5, 30, 2.7, 1.9, 0.56), core(19.5, 4, 0, 2.3, 1.8, 0.54),
      ],
      top: { y: 28.55, a: 1.15, b: 0.95 }, tubeN: 7, rmin: 0.018, twigR: 0.02, leafMarks: 36, markSize: 0.14, barkMarks: 8,
    },
    200: {
      rings: [
        ring(3.5, 12, 14, 2.4, 1.3, { tilt: 12, jitter: 0.6 }), ring(5.6, 13, 32, 2.4, 1.4, { jitter: 0.6 }), ring(7.7, 14, 6, 2.4, 1.45, { jitter: 0.6 }), ring(9.8, 14, 24, 2.4, 1.45, { jitter: 0.6 }), ring(11.9, 13, 10, 2.35, 1.45, { jitter: 0.6 }),
        ring(14.0, 12, 28, 2.3, 1.45, { jitter: 0.6 }), ring(16.1, 11, 14, 2.2, 1.4, { jitter: 0.6 }), ring(18.2, 9, 34, 2.1, 1.4, { jitter: 0.6 }), ring(20.3, 8, 18, 1.95, 1.35, { jitter: 0.6 }), ring(22.4, 6, 40, 1.8, 1.3, { jitter: 0.6 }), ring(24.4, 5, 20, 1.6, 1.2, { jitter: 0.6 }), ring(26.2, 3, 45, 1.4, 1.15, { jitter: 0.6 }),
        core(7.0, 4, 45, 3.3, 2.3, 0.52), core(12.5, 5, 0, 3.3, 2.4, 0.54), core(18.0, 4, 30, 2.9, 2.2, 0.52),
      ],
      top: { y: 28.1, a: 1.5, b: 1.3 }, tubeN: 5, rmin: 0.04, twigR: 0.04, leafMarks: 0, markSize: 0, barkMarks: 0,
    },
    500: {
      rings: [
        ring(4.0, 8, 20, 3.2, 1.8, { tilt: 10, jitter: 0.3 }), ring(7.1, 9, 45, 3.2, 2.0, { jitter: 0.3 }), ring(10.3, 9, 10, 3.15, 2.05, { jitter: 0.3 }), ring(13.5, 8, 35, 3.05, 2.05, { jitter: 0.3 }),
        ring(16.7, 7, 5, 2.85, 2.0, { jitter: 0.3 }), ring(19.9, 6, 40, 2.6, 1.9, { jitter: 0.3 }), ring(23.0, 4, 15, 2.25, 1.75, { jitter: 0.3 }), ring(25.4, 3, 50, 1.9, 1.6, { jitter: 0.3 }),
        core(9.0, 3, 60, 3.9, 2.9, 0.48), core(15.5, 3, 0, 3.6, 2.7, 0.48),
      ],
      top: { y: 27.6, a: 2.0, b: 1.9 }, tubeN: 3, rmin: 0.1, twigR: 0.1, leafMarks: 0, markSize: 0, barkMarks: 0,
    },
  },
  leafArea: { sun: 920, shade: 1180, projectionM2: 325 },
  regions: [
    barkRegion("trunk-bark", tx("Smooth silver-grey bark of stem and lower crown stem", "Glatte silbergraue Rinde von Schaft und unterem Kronenstamm"), "beech-bark", 0.5, 0.35, 70000),
    barkRegion("limb-bark", tx("Smooth grey bark of upper stem, limbs, branches and twigs", "Glatte graue Rinde von oberem Stamm, Ästen, Zweigen und Trieben"), "beech-bark-young", 0.12, 0.45, 40000),
    leafRegion("leaf-sun", tx("Sun leaves of the outer crown", "Sonnenblätter der Außenkrone"), "beech-leaf-sun", 0.07, 0.7, 750, 0.0045),
    leafRegion("leaf-shade", tx("Shade leaves of the dense inner crown", "Schattenblätter der dichten Innenkrone"), "beech-leaf-shade", 0.09, 0.25, 550, 0.0022),
  ],
  leafRegionTitles: { sun: tx("", ""), shade: tx("", "") },
  emitterDiameter: { sun: 1.3, shade: 1.0 },
  tones: { trunk: 0.42, limb: 0.44, sun: 0.56, shade: 0.4 },
  porosity: 0.78, cdStem: 1.0,
  thermal: {
    environment: { airTempC: 24, meanRadiantTempC: 28, relativeHumidity: 0.5, windMS: 1.2, windDirectionDeg: 0, solarWM2: 700, pressurePa: 101325, diffuseFraction: 0.15 },
    assumptions: [...broadleafAssumptions("2.83 + 3.63", "0.0045", "0.0022"), tx("Beech bark is thin (about 1 cm) and smooth: it is the lightest bark of the three park trees (solar absorptivity 0.6) and warms quickly in sun; bark scorch on suddenly exposed stems is a known consequence, not modelled.", "Buchenrinde ist dünn (etwa 1 cm) und glatt: Sie ist die hellste Rinde der drei Parkbäume (solare Absorption 0,6) und erwärmt sich in der Sonne schnell; Rindenbrand an plötzlich freigestellten Stämmen ist eine bekannte Folge, wird aber nicht modelliert.")],
  },
  structureAssumptions: structureAssumptions("1000"),
  poses, defaultPose: "summer-calm", breezePose: "summer-breeze", calmPose: "summer-calm",
  sway: { period: 5.2, amp: { trunk: 0.04, upper: 0.12, limb: 0.3, outer: 0.5 }, lag: 0.35, text: tx("A deterministic %P s sway cycle about the breeze lean (5.5 m/s at 10 m; estimated first mode of the 30 m stem with its wide crown about 0.19 Hz). Every level lags the one below it; the long lower limbs rock about half a degree.", "Ein deterministischer Schwingungszyklus von %P s um die Neigung bei Brise (5,5 m/s in 10 m Höhe; geschätzte erste Eigenform des 30 m hohen Stamms mit seiner weiten Krone etwa 0,19 Hz). Jede Ebene hinkt der darunter nach; die langen unteren Äste wiegen sich etwa einen halben Grad.") },
  gust: { from: 5.5, peak: 15, text: tx("A single gust over 8 s from %A to %B m/s at 10 m: the wind-load tracks rise with the reconfigured drag law and the crown leans with a level-wise lag, outer limbs first, the stem last.", "Eine einzelne Böe über 8 s von %A auf %B m/s in 10 m Höhe: Die Windlast-Spuren steigen mit dem umgeformten Widerstandsgesetz, die Krone neigt sich ebenenweise verzögert, zuerst die äußeren Äste, zuletzt der Stamm.") },
  diurnal: {
    text: tx("A 24 h summer day compressed to 24 s (1 s = 1 h), calm: the sunlit share of every region and the transpiration of the sun and shade leaves follow the sun, with stomata nearly closed at night and a mid-afternoon depression; leaf and bark temperatures are not authored but follow from the thermal solver.", "Ein 24-h-Sommertag auf 24 s gerafft (1 s = 1 h) bei Windstille: Der besonnte Anteil jeder Region und die Transpiration der Sonnen- und Schattenblätter folgen der Sonne, nachts sind die Stomata fast geschlossen, am Nachmittag gibt es eine Mittagsdepression; Blatt- und Rindentemperaturen werden nicht vorgegeben, sondern folgen aus dem thermischen Löser."),
    sun: [[0, 0.03], [5, 0.05], [7, 0.33], [9, 0.82], [11.5, 1], [14, 0.7], [16, 0.74], [18, 0.28], [20, 0.06], [24, 0.03]],
    shade: [[0, 0.02], [5, 0.03], [7, 0.18], [9, 0.5], [12, 0.62], [14, 0.45], [16, 0.47], [18, 0.18], [20, 0.04], [24, 0.02]],
    sunlit: diurnalSunlit({ "trunk-bark": 0.25, "limb-bark": 0.12, "leaf-sun": 0.9, "leaf-shade": 0.04 }),
  },
  leafMark, barkMark,
  texts: {
    scale: {
      50: {
        silhouette: tx("At 1:50 (1 m is 20 mm) the beech is drawn as the true views of its 1:50 body: the broad elephant-foot flare and the smooth stem, twenty-six limbs in six tiers (long level lower limbs, steep main limbs above) with their side branches and twigs, and the wide dome of some four hundred flat, overlapping leaf sprays that read as the dense layered foliage of a beech.", "Bei 1:50 (1 m sind 20 mm) ist die Buche als wahre Ansichten ihres Körpers 1:50 gezeichnet: der breite Elefantenfuß-Anlauf und der glatte Stamm, sechsundzwanzig Äste in sechs Etagen (lange, waagrechte untere Äste, darüber steile Hauptäste) mit Seitenzweigen und Trieben und die weite Kuppel aus rund vierhundert flachen, überlappenden Zweigfächern, die sich als dichtes, geschichtetes Buchenlaub lesen."),
        merged: tx("The wood of one bone is one outline (limb, side branches and the twigs it sends into its sprays); the sprays of one carrying bone and leaf region are one scalloped outline with its sky openings.", "Das Holz eines Knochens ist ein Umriss (Ast, Seitenzweige und die Triebe in seine Zweigfächer); die Zweigfächer eines tragenden Knochens und einer Blattregion sind ein gebogter Umriss mit seinen Himmelslücken."),
        omitted: tx("Omitted: single leaves and buds below the print minimum; leaf marks and bark creases where nearer foliage covers them.", "Weggelassen: einzelne Blätter und Knospen unter dem Druckminimum; Blattzeichen und Rindenfalten, wo näheres Laub sie verdeckt."),
        thinElements: tx("Branches and twigs taper to 18 mm, still 0.24 mm at 1:75, the coarse end of this drawing; leaf marks and the creases of the smooth bark are 0.13 mm texture strokes.", "Zweige und Triebe verjüngen sich bis 18 mm, bei 1:75, dem groben Ende dieser Zeichnung, noch 0,24 mm; Blattzeichen und die Falten der glatten Rinde sind 0,13-mm-Texturstriche."),
        styles: tx("Light line: closed outlines with oval leaf marks and the few elephant-skin creases of the smooth bark. Dark solid: the dense dome dark, the shade core darker, the grey stem light.", "Helle Linie: geschlossene Umrisse mit ovalen Blattzeichen und den wenigen Elefantenhaut-Falten der glatten Rinde. Dunkle Fläche: die dichte Kuppel dunkel, der Schattenkern dunkler, der graue Stamm hell."),
      },
      100: {
        silhouette: tx("At 1:100 (1 m is 10 mm) the true views of the 1:100 body: the flared stem and the twenty-six limbs with their main side branches, the dome of larger, flat sprays with fewer, wider scallops.", "Bei 1:100 (1 m sind 10 mm) die wahren Ansichten des Körpers 1:100: der ausgestellte Stamm und die sechsundzwanzig Äste mit ihren Hauptseitenzweigen, die Kuppel aus größeren, flachen Zweigfächern mit weniger, weiteren Bögen."),
        merged: tx("One outline per bone for the wood and per carrying bone and leaf region for the foliage; the outer side branches are merged into their limbs.", "Ein Umriss je Knochen für das Holz und je tragendem Knochen und Blattregion für das Laub; die äußeren Seitenzweige sind in ihre Äste aufgegangen."),
        omitted: tx("Omitted: the third side branch of every limb, the finest twigs and every mark that a nearer spray covers.", "Weggelassen: der dritte Seitenzweig jedes Astes, die feinsten Triebe und jedes Zeichen, das ein näherer Zweigfächer verdeckt."),
        thinElements: tx("Wood is never thinner than 36 mm (0.24 mm at 1:150, the coarse end of this drawing); marks are 0.1 mm texture strokes.", "Holz ist nie dünner als 36 mm (0,24 mm bei 1:150, dem groben Ende dieser Zeichnung); Zeichen sind 0,1-mm-Texturstriche."),
        styles: tx("Light line: outlines and a few leaf marks and creases. Dark solid: dome dark, core darker, stem light.", "Helle Linie: Umrisse und wenige Blattzeichen und Falten. Dunkle Fläche: Kuppel dunkel, Kern dunkler, Stamm hell."),
      },
      200: {
        silhouette: tx("At 1:200 (1 m is 5 mm) the dome is composed of about one hundred and thirty wide sprays over the shaded core; stem, limbs and the first side branch of every limb are drawn at a printable width, the low hanging skirt kept.", "Bei 1:200 (1 m sind 5 mm) ist die Kuppel aus etwa hundertdreißig weiten Zweigfächern über dem beschatteten Kern komponiert; Stamm, Äste und der erste Seitenzweig jedes Astes sind in druckbarer Breite gezeichnet, der tief hängende Saum bleibt."),
        merged: tx("The wood of one bone is one outline; the sprays of one carrying bone and region are one outline; small sky openings are closed.", "Das Holz eines Knochens ist ein Umriss; die Zweigfächer eines tragenden Knochens und einer Region sind ein Umriss; kleine Himmelslücken sind geschlossen."),
        omitted: tx("Omitted: twigs, the outer side branches, leaf and bark texture.", "Weggelassen: Triebe, die äußeren Seitenzweige, Blatt- und Rindentextur."),
        thinElements: tx("Limbs are drawn at least 80 mm wide, 0.23 mm at 1:350, the coarse end of this drawing.", "Äste sind mindestens 80 mm breit gezeichnet, 0,23 mm bei 1:350, dem groben Ende dieser Zeichnung."),
        styles: tx("Light line: closed outlines only. Dark solid: one dark dome with a darker core and a light stem.", "Helle Linie: nur geschlossene Umrisse. Dunkle Fläche: eine dunkle Kuppel mit dunklerem Kern und hellem Stamm."),
      },
      500: {
        silhouette: tx("At 1:500 (1 m is 2 mm) the beech is one wide, dense dome of some sixty broad sprays on its smooth stem; the low skirt of the hanging lower sprays and the steep upper dome keep the beech's broad bell shape.", "Bei 1:500 (1 m sind 2 mm) ist die Buche eine weite, dichte Kuppel aus etwa sechzig breiten Zweigfächern auf ihrem glatten Stamm; der tiefe Saum der hängenden unteren Fächer und die steile obere Kuppel bewahren die breite Glockenform der Buche."),
        merged: tx("Foliage of one carrying bone and region is one outline without sky openings; the wood of one bone is one outline.", "Das Laub eines tragenden Knochens und einer Region ist ein Umriss ohne Himmelslücken; das Holz eines Knochens ist ein Umriss."),
        omitted: tx("Omitted: every side branch and twig, all texture; inner limbs show only where the crown leaves them visible.", "Weggelassen: alle Seitenzweige und Triebe, jede Textur; innere Äste zeigen sich nur, wo die Krone sie sichtbar lässt."),
        thinElements: tx("Wood is drawn at least 200 mm wide, the smallest width that still prints at 1:1000, the coarse end of this drawing.", "Holz ist mindestens 200 mm breit gezeichnet, die kleinste Breite, die bei 1:1000, dem groben Ende dieser Zeichnung, noch druckt."),
        styles: tx("Light line: closed outlines only. Dark solid: the crown as one dark mass, the stem lighter.", "Helle Linie: nur geschlossene Umrisse. Dunkle Fläche: die Krone als eine dunkle Masse, der Stamm heller."),
      },
    },
    volume: {
      50: tx("Body at 1:50 (smallest feature 7.5 mm): the elephant-foot flare and the smooth stem as one tapered loft per stem bone, twenty-six limbs in six tiers (the lowest long and almost level, the middle ones steep), seventy-eight side branches and a twig into every spray, all lofted through hand-stated stations that start inside their parent so that sway never opens a joint; the crown of flat, wide sprays (1.6 by 0.9 m) in twenty-three rings that follow the domed envelope, with six rings of large shaded core masses round the stem.", "Körper bei 1:50 (kleinstes Merkmal 7,5 mm): der Elefantenfuß-Anlauf und der glatte Stamm als je ein verjüngter Loft je Stammknochen, sechsundzwanzig Äste in sechs Etagen (die untersten lang und fast waagrecht, die mittleren steil), achtundsiebzig Seitenzweige und ein Trieb in jeden Zweigfächer, alle durch von Hand gesetzte Stationen gelofted, die im Mutterteil beginnen, sodass kein Gelenk beim Schwingen aufgeht; die Krone aus flachen, weiten Zweigfächern (1,6 auf 0,9 m) in dreiundzwanzig Ringen entlang der gewölbten Hülle, mit sechs Ringen großer beschatteter Kernmassen um den Stamm."),
      100: tx("Body at 1:100 (smallest feature 15 mm): the same stem and twenty-six limbs with fifty-two side branches at fewer stations, wood never thinner than 36 mm, eighteen rings of larger sprays and four core rings.", "Körper bei 1:100 (kleinstes Merkmal 15 mm): derselbe Stamm und sechsundzwanzig Äste mit zweiundfünfzig Seitenzweigen an weniger Stationen, Holz nie dünner als 36 mm, achtzehn Ringe größerer Zweigfächer und vier Kernringe."),
      200: tx("Body at 1:200 (smallest feature 30 mm): stem and limbs at five stations, wood at least 80 mm, twenty-six side branches, twelve rings of wide sprays and three core rings.", "Körper bei 1:200 (kleinstes Merkmal 30 mm): Stamm und Äste an fünf Stationen, Holz mindestens 80 mm, sechsundzwanzig Seitenzweige, zwölf Ringe weiter Zweigfächer und drei Kernringe."),
      500: tx("Body at 1:500 (smallest feature 75 mm): stem and limbs as three-station lofts at least 200 mm thick, no side branches, seven rings of massive sprays and two core rings: the dome of the tree.", "Körper bei 1:500 (kleinstes Merkmal 75 mm): Stamm und Äste als Lofts mit drei Stationen, mindestens 200 mm dick, keine Seitenzweige, sieben Ringe massiger Zweigfächer und zwei Kernringe: die Kuppel des Baums."),
    },
    symbol: tx("A conventional plan sign for site and landscape plans: the true plan outline of the wide crown, the stem section at breast height and the directions of the five lowest limbs. It is a separate symbol and is never used silently for a pictorial scale.", "Ein konventionelles Grundrisszeichen für Lage- und Freiraumpläne: der wahre Grundrissumriss der weiten Krone, der Stammquerschnitt in Brusthöhe und die Richtungen der fünf untersten Äste. Es ist ein eigenes Symbol und wird nie stillschweigend für einen Bildmaßstab verwendet."),
  },
  symbolLimbs: ["t1-1", "t1-2", "t1-3", "t1-4", "t1-5"],
  defaultScale: 200,
};
