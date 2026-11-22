/** 🌰️ Horse chestnut (Aesculus hippocastanum), an open-grown park tree of about 90 years: short massive stem, broad domed crown of heavy ascending limbs whose lowest ones sweep down and turn up at their tips, large palmate leaves, white flower candles in May. Every number is chosen by hand here.
 * @see ./kit.ts
 * @see ../../📝️PW-02.md */
import { tx, type BranchSpec, type LimbSpec, type PoseSpec, type Scale, type Species, type Text } from "./kit.ts";
import { COMMON_REFERENCES, broadleafAssumptions, diurnalSunlit, leafRegion, barkRegion, structureAssumptions } from "./common.ts";

const TIERS: [string, number, number[], [number, number][], number, number, number, number, number, Text, Text][] = [
  ["t1", 3.55, [12, 102, 194, 283], [[0, 40], [0.3, 20], [0.62, -6], [0.85, 4], [1, 18]], 8.5, 0.205, 0.05, 0.48, 9, tx("Lowest scaffold limb", "Unterster Gerüstast"), tx("Lowest scaffold limb, sweeping outer part", "Unterster Gerüstast, ausschwingender Außenteil")],
  ["t2", 5.7, [57, 148, 238, 326], [[0, 46], [0.45, 28], [1, 10]], 8.2, 0.175, 0.045, 0.5, 9, tx("Second-tier limb", "Ast der zweiten Etage"), tx("Second-tier limb, outer part", "Ast der zweiten Etage, Außenteil")],
  ["t3", 8.6, [5, 78, 151, 221, 292], [[0, 54], [0.5, 38], [1, 20]], 7.4, 0.135, 0.04, 0.55, 10, tx("Third-tier limb", "Ast der dritten Etage"), tx("Third-tier limb, outer part", "Ast der dritten Etage, Außenteil")],
  ["t4", 12.4, [33, 124, 212, 301], [[0, 60], [1, 34]], 6.1, 0.095, 0.03, 0, 11, tx("Fourth-tier limb", "Ast der vierten Etage"), tx("", "")],
  ["t5", 16.0, [84, 203, 329], [[0, 64], [1, 44]], 4.4, 0.065, 0.025, 0, 12, tx("Upper crown limb", "Ast der oberen Krone"), tx("", "")],
];
const SIDE: Record<string, string> = { 0: "east", 1: "north", 2: "west", 3: "south" };
const sideOf = (az: number): Text => {
  const k = Math.round((((az % 360) + 360) % 360) / 90) % 4;
  return [tx("east", "Ost"), tx("north", "Nord"), tx("west", "West"), tx("south", "Süd")][k]!;
};
void SIDE;

const limbs: LimbSpec[] = [];
const branches: BranchSpec[] = [];
for (const [tier, y, azs, elev, L, r0, r1, split, limit, title, titleB] of TIERS) {
  azs.forEach((az, k) => {
    const id = `${tier}-${k + 1}`;
    const s = sideOf(az);
    limbs.push({ id, parent: "stem", at: y + 0.22 * k, az, elev, L: L * (1 + 0.04 * Math.sin(k * 2.1)), r0, r1, bend: 9 * Math.sin(k * 1.3 + y), ...(split ? { split } : {}), title: tx(`${title.en}, ${s.en}`, `${title.de}, ${s.de}`), ...(split ? { titleB: tx(`${titleB.en}, ${s.en}`, `${titleB.de}, ${s.de}`) } : {}), limit });
    const sc: Scale[][] = [[50, 100, 200], [50, 100], [50]];
    [[0.38, 34, 0.42], [0.62, -30, 0.36], [0.8, 26, 0.3]].forEach(([t, daz, fL], j) => {
      branches.push({ id: `${id}-${j + 1}`, limb: id, t: t!, az: az + daz!, elev: [[0, (elev[0]![1] + 12) * 0.8], [1, 22]], L: L * fL!, r0: r0 * 0.42, r1: 0.02, bend: daz! * 0.3, scales: sc[j]! });
    });
  });
}

/** 🌐️ Crown envelope: widest (9.2 m) at 8.2 m, full rounded skirt down to 3.0 m, domed top at 24.0 m, slightly wider to the south-east. */
const envelope = (y: number, az: number): number => {
  const [ym, base, top, R] = [8.2, 3.0, 24.0, 9.2];
  const t = y < ym ? (ym - y) / (ym - base) : (y - ym) / (top - ym);
  const p = y < ym ? 3.0 : 2.3;
  const f = Math.pow(Math.max(0, 1 - Math.pow(Math.min(1, t), p)), 1 / p);
  const a = (az * Math.PI) / 180;
  return R * f * (1 + 0.045 * Math.cos(a + 0.6) + 0.03 * Math.cos(2 * a - 0.7) + 0.02 * Math.cos(3 * a + 1.1));
};

const ring = (y: number, n: number, phase: number, a: number, b: number, extra: Partial<Species["scales"][50]["rings"][0]> = {}) => ({ y, n, phase, a, b, region: "sun" as const, jitter: 1, ...extra });
const core = (y: number, n: number, phase: number, a: number, b: number, reach: number) => ({ y, n, phase, a, b, region: "shade" as const, reach, ck: 1.1, jitter: 0.6 });

const poses: PoseSpec[] = [
  { id: "candle-bloom", title: tx("Candle bloom in May", "Kerzenblüte im Mai"), description: tx("Mid-May, calm and sunny: the white flower candles (upright panicles of 20 to 30 cm) stand on the tips of the outer shoots all over the crown; the fresh leaves are fully open. The panicles add little load and transpire little; leaves transpire at their spring rate.", "Mitte Mai, windstill und sonnig: Die weißen Blütenkerzen (aufrechte Rispen von 20 bis 30 cm) stehen über die ganze Krone auf den Spitzen der äußeren Triebe; das frische Laub ist ganz entfaltet. Die Rispen tragen wenig Last und verdunsten wenig; die Blätter transpirieren mit ihrer Frühjahrsrate."), v10: 1.2, cd: 0.45, lean: 0, droop: 0, candlesOut: true, fruitKgM2: 0.12, fruitTitle: tx("Weight of the flower candles", "Gewicht der Blütenkerzen"),
    regions: { "trunk-bark": { exposedFraction: 1, sunlitFraction: 0.3 }, "limb-bark": { exposedFraction: 1, sunlitFraction: 0.22 }, "leaf-sun": { exposedFraction: 1, sunlitFraction: 0.86 }, "leaf-shade": { exposedFraction: 1, sunlitFraction: 0.07 }, "flower-candles": { exposedFraction: 1, sunlitFraction: 0.8 } }, emitters: { sun: 0.82, shade: 0.56 } },
  { id: "summer-calm", title: tx("Summer calm", "Sommerliche Windstille"), description: tx("July noon, light air of 1.2 m/s: the flowers are over, the panicle stalks have withdrawn into the dense crown; full transpiration of the sun leaves.", "Juli, Mittag, leiser Zug von 1,2 m/s: Die Blüte ist vorbei, die Rispenstiele sind in der dichten Krone verschwunden; volle Transpiration der Sonnenblätter."), v10: 1.2, cd: 0.45, lean: 0, droop: 0, candlesOut: false,
    regions: { "trunk-bark": { exposedFraction: 1, sunlitFraction: 0.28 }, "limb-bark": { exposedFraction: 1, sunlitFraction: 0.2 }, "leaf-sun": { exposedFraction: 1, sunlitFraction: 0.9 }, "leaf-shade": { exposedFraction: 1, sunlitFraction: 0.06 }, "flower-candles": { exposedFraction: 0.05, sunlitFraction: 0.02 } }, emitters: { sun: 1, shade: 0.7 } },
  { id: "summer-breeze", title: tx("Summer breeze", "Sommerbrise"), description: tx("Moderate breeze of 5.5 m/s at 10 m (Beaufort 4): the heavy crown leans slightly downwind; drag F = 0.5 rho cd A v^2 with cd 0.35 on the porous frontal crown area, power-law wind profile.", "Mäßige Brise von 5,5 m/s in 10 m Höhe (Beaufort 4): Die schwere Krone neigt sich leicht in Windrichtung; Widerstand F = 0,5 rho cd A v^2 mit cd 0,35 auf der porösen Kronenstirnfläche, Potenzgesetz-Windprofil."), v10: 5.5, cd: 0.35, lean: 0.17, droop: 0, candlesOut: false,
    regions: { "trunk-bark": { exposedFraction: 1, sunlitFraction: 0.28 }, "limb-bark": { exposedFraction: 1, sunlitFraction: 0.22 }, "leaf-sun": { exposedFraction: 1, sunlitFraction: 0.88 }, "leaf-shade": { exposedFraction: 1, sunlitFraction: 0.09 }, "flower-candles": { exposedFraction: 0.05, sunlitFraction: 0.02 } }, emitters: { sun: 0.95, shade: 0.68 } },
  { id: "gale", title: tx("Summer gale", "Sommersturm"), description: tx("Gale of 22 m/s at 10 m (Beaufort 9) in full leaf, overcast: the large leaves fold and the crown reconfigures (cd 0.22 after Vogel 1989); limbs and stem lean downwind, transpiration is low under cloud.", "Sturm von 22 m/s in 10 m Höhe (Beaufort 9) im vollen Laub, bedeckt: Die großen Blätter falten sich und die Krone formt sich um (cd 0,22 nach Vogel 1989); Äste und Stamm neigen sich in Windrichtung, die Transpiration ist unter Wolken gering."), v10: 22, cd: 0.22, lean: 1.65, droop: 0, candlesOut: false,
    regions: { "trunk-bark": { exposedFraction: 1, sunlitFraction: 0.03, wettedness: 0.3 }, "limb-bark": { exposedFraction: 1, sunlitFraction: 0.03, wettedness: 0.3 }, "leaf-sun": { exposedFraction: 1, sunlitFraction: 0.08, wettedness: 0.2 }, "leaf-shade": { exposedFraction: 1, sunlitFraction: 0.01, wettedness: 0.1 }, "flower-candles": { exposedFraction: 0.05, sunlitFraction: 0 } }, emitters: { sun: 0.35, shade: 0.22 } },
  { id: "conker-fruit", title: tx("Conkers ripening", "Reifende Kastanien"), description: tx("Mid-September, calm: the spiny husks with their conkers hang in bunches of one to three at the shoot tips, about 1.2 kg of fruit per m2 of crown plan area (order of magnitude of a mast year); the outer limbs bend a little under the load.", "Mitte September, windstill: Die stacheligen Fruchtbecher mit den Kastanien hängen zu einer bis drei an den Triebspitzen, etwa 1,2 kg Früchte je m2 Kronengrundfläche (Größenordnung eines Mastjahres); die äußeren Äste biegen sich unter der Last ein wenig."), v10: 1.2, cd: 0.45, lean: 0, droop: 0.7, candlesOut: false, fruitKgM2: 1.2, fruitTitle: tx("Weight of the conkers in their husks", "Gewicht der Kastanien in ihren Fruchtbechern"),
    regions: { "trunk-bark": { exposedFraction: 1, sunlitFraction: 0.22 }, "limb-bark": { exposedFraction: 1, sunlitFraction: 0.18 }, "leaf-sun": { exposedFraction: 1, sunlitFraction: 0.78 }, "leaf-shade": { exposedFraction: 1, sunlitFraction: 0.05 }, "flower-candles": { exposedFraction: 0.05, sunlitFraction: 0.02 } }, emitters: { sun: 0.62, shade: 0.4 } },
  { id: "wet-snow", title: tx("Early wet snow", "Früher Nassschnee"), description: tx("An early snowfall in late October on the still leafy crown: 10 kg of wet snow per m2 of foliage plan area (order of magnitude from EN 1991-1-3) weighs the limbs down; calm, no sun, stomata nearly closed.", "Ein früher Schneefall Ende Oktober auf die noch belaubte Krone: 10 kg nasser Schnee je m2 Laubgrundfläche (Größenordnung nach EN 1991-1-3) drückt die Äste nieder; windstill, keine Sonne, Stomata fast geschlossen."), v10: 1.0, cd: 0.45, lean: 0, droop: 2.4, snowKgM2: 10, candlesOut: false,
    regions: { "trunk-bark": { exposedFraction: 1, sunlitFraction: 0.01, wettedness: 1 }, "limb-bark": { exposedFraction: 1, sunlitFraction: 0.01, wettedness: 1 }, "leaf-sun": { exposedFraction: 1, sunlitFraction: 0.03, wettedness: 1 }, "leaf-shade": { exposedFraction: 1, sunlitFraction: 0.002, wettedness: 1 }, "flower-candles": { exposedFraction: 0.05, sunlitFraction: 0 } }, emitters: { sun: 0.05, shade: 0.03 } },
];

/** ✋️ A palmate chestnut leaf mark: seven leaflets radiating from the petiole tip, drawn as one stroke out and back. */
function leafMark(c: { x: number; y: number }, size: number, deg: number): { x: number; y: number }[][] {
  const out: { x: number; y: number }[] = [c];
  const lens = [0.55, 0.8, 1, 1.05, 1, 0.8, 0.55];
  lens.forEach((l, i) => {
    const a = ((deg + 90 + (i - 3) * 30) * Math.PI) / 180;
    out.push({ x: c.x + size * l * Math.cos(a), y: c.y + size * l * Math.sin(a) }, c);
  });
  return [out];
}

/** 🪵️ Spiralling scaly plates of old chestnut bark on the clear stem (side and front elevation). */
function barkMark(k: number, view: string, stem: (y: number) => { x: number; z: number; r: number }): { x: number; y: number }[] | null {
  const y = 0.55 + (k % 7) * 0.4 + 0.1 * Math.floor(k / 7);
  if (y > 3.2) return null;
  const s = stem(y);
  const u = ((Math.floor(k / 7) % 3) - 1) * 0.42 + 0.12 * Math.sin(k * 1.7);
  const cx = view === "elevation" ? s.x : s.z;
  const x0 = cx + u * s.r;
  return [0, 1, 2].map((i) => ({ x: x0 + (i - 1) * 0.07, y: y + (i - 1) * 0.15 }));
}

export const species: Species = {
  id: "horse-chestnut",
  title: tx("Horse Chestnut", "Rosskastanie"),
  summary: tx("A mature open-grown horse chestnut (Aesculus hippocastanum) of about 24 m: a short massive stem, a broad domed crown of heavy ascending limbs whose lowest ones sweep down and turn up again at their tips, large palmate leaves and, in May, white flower candles all over the crown.", "Eine ausgewachsene, frei stehende Rosskastanie (Aesculus hippocastanum) von etwa 24 m: ein kurzer, mächtiger Stamm, eine breite, gewölbte Krone aus schweren, aufsteigenden Ästen, deren unterste nach unten ausschwingen und an den Spitzen wieder aufbiegen, große gefingerte Blätter und im Mai weiße Blütenkerzen über die ganze Krone."),
  distinction: tx("A 24 m horse chestnut (Aesculus hippocastanum) with a short stem of 0.95 m, a broad 16.5 m dome of sweeping lower limbs and palmate leaves, and white flower candles that stand on the crown in May and withdraw into it afterwards.", "Eine 24 m hohe Rosskastanie (Aesculus hippocastanum) mit kurzem Stamm von 0,95 m, einer breiten Kuppel von 16,5 m aus ausschwingenden unteren Ästen und gefingerten Blättern sowie weißen Blütenkerzen, die im Mai auf der Krone stehen und danach in ihr verschwinden."),
  tags: ["tree", "deciduous", "broadleaf", "park-tree", "dense-crown", "flowering", "fruit", "wind", "transpiration"],
  created: "2026-11-22",
  notes: tx("Rig and drawings are authored by hand for an open-grown park horse chestnut of about 90 years: stem 3.4 m to the first tier, a continuous leader to 22 m, twenty scaffold limbs in five tiers, the lowest sweeping down and up again as old chestnuts do. Crown size and habit follow Roloff (2010) and Pretzsch et al. (2015) as orders of magnitude; wood properties, masses and conductances are authored estimates.", "Rig und Zeichnungen sind von Hand für eine frei stehende Parkkastanie von etwa 90 Jahren entworfen: Stamm 3,4 m bis zur ersten Etage, eine durchgehende Stammverlängerung bis 22 m, zwanzig Gerüstäste in fünf Etagen, die untersten nach unten und wieder hinauf schwingend wie bei alten Kastanien. Kronengröße und Wuchs folgen Roloff (2010) und Pretzsch et al. (2015) als Größenordnungen; Holzeigenschaften, Massen und Leitfähigkeiten sind Autorenschätzungen."),
  references: [...COMMON_REFERENCES, { id: "pretzsch-2015", citation: "Pretzsch, H., Biber, P., Uhl, E. et al. (2015). Crown size and growing space requirement of common tree species in urban centres, parks, and forests. Urban Forestry and Urban Greening 14(3), 466-479." }],
  materials: [
    { id: "chestnut-bark", title: tx("Old horse chestnut bark, grey-brown in scaly plates", "Alte Rosskastanienrinde, graubraun in schuppigen Platten"), emissivity: 0.95, solarAbsorptivity: 0.74, densityKgM3: 620, conductivityWmK: 0.12, source: "authored-estimate", reference: "wood-handbook-2021" },
    { id: "chestnut-bark-young", title: tx("Smooth red-brown bark of limbs and shoots", "Glatte rotbraune Rinde der Äste und Triebe"), emissivity: 0.95, solarAbsorptivity: 0.7, densityKgM3: 640, conductivityWmK: 0.13, source: "authored-estimate", reference: "wood-handbook-2021" },
    { id: "chestnut-wood", title: tx("Green horse chestnut wood", "Frisches Rosskastanienholz"), emissivity: 0.9, solarAbsorptivity: 0.6, densityKgM3: 850, conductivityWmK: 0.14, elasticModulusPa: 6000000000, strengthPa: 35000000, source: "authored-estimate", reference: "wood-handbook-2021" },
    { id: "chestnut-leaf-sun", title: tx("Horse chestnut sun leaves", "Rosskastanien-Sonnenblätter"), emissivity: 0.97, solarAbsorptivity: 0.5, source: "literature", reference: "jones-2014" },
    { id: "chestnut-leaf-shade", title: tx("Horse chestnut shade leaves (effective)", "Rosskastanien-Schattenblätter (wirksam)"), emissivity: 0.97, solarAbsorptivity: 0.25, source: "authored-estimate", reference: "jones-2014" },
    { id: "chestnut-candle", title: tx("White flower panicles", "Weiße Blütenrispen"), emissivity: 0.95, solarAbsorptivity: 0.3, source: "authored-estimate", reference: "jones-2014" },
  ],
  barkMaterial: "chestnut-bark", youngBarkMaterial: "chestnut-bark-young", woodMaterial: "chestnut-wood", sunMaterial: "chestnut-leaf-sun", shadeMaterial: "chestnut-leaf-shade",
  density: 850, foliageKgPerM2: 0.24,
  stem: {
    stations: [[0, 0, 0, 0.86], [0.12, 0, 0, 0.71], [0.3, 0, 0, 0.6], [0.7, 0.01, 0, 0.52], [1.3, 0.02, -0.01, 0.475], [2.4, 0.03, -0.02, 0.455], [3.4, 0.05, -0.02, 0.43], [5.5, 0.08, 0, 0.37], [7.5, 0.06, 0.03, 0.315], [10, 0.02, 0.05, 0.25], [12, -0.03, 0.04, 0.205], [14.5, -0.06, 0.02, 0.15], [16.5, -0.05, 0, 0.115], [19.5, -0.02, -0.02, 0.07], [22, 0.0, -0.03, 0.035]],
    bones: [
      { id: "root-plate", y0: 0, y1: 0.45, title: tx("Root plate and flared collar", "Wurzelteller und ausgestellter Wurzelhals"), limit: 0, region: "trunk-bark" },
      { id: "trunk", y0: 0.45, y1: 3.4, title: tx("Clear stem", "Schaft"), limit: 3, region: "trunk-bark" },
      { id: "s1", y0: 3.4, y1: 7.5, title: tx("Stem in the lower crown", "Stamm in der unteren Krone"), limit: 3, region: "trunk-bark" },
      { id: "s2", y0: 7.5, y1: 12, title: tx("Stem in the middle crown", "Stamm in der mittleren Krone"), limit: 4, region: "limb-bark" },
      { id: "s3", y0: 12, y1: 16.5, title: tx("Stem in the upper crown", "Stamm in der oberen Krone"), limit: 5, region: "limb-bark" },
      { id: "leader", y0: 16.5, y1: 22, title: tx("Leader", "Wipfeltrieb"), limit: 7, region: "limb-bark" },
    ],
  },
  limbs, branches, envelope,
  scales: {
    50: {
      rings: [
        ring(3.9, 15, 0, 1.65, 1.45, { tilt: 14 }), ring(5.3, 19, 13, 1.62, 1.42, { tilt: 8 }), ring(6.75, 21, 26, 1.59, 1.39), ring(8.2, 21, 2, 1.56, 1.35), ring(9.65, 21, 15, 1.54, 1.32), ring(11.1, 21, 28, 1.51, 1.29), ring(12.55, 20, 4, 1.48, 1.26), ring(14.0, 19, 17, 1.45, 1.23), ring(15.45, 18, 30, 1.42, 1.19), ring(16.9, 16, 6, 1.39, 1.16), ring(18.35, 15, 19, 1.36, 1.13), ring(19.8, 13, 32, 1.34, 1.10), ring(21.15, 10, 8, 1.31, 1.06), ring(22.4, 7, 21, 1.28, 1.03), ring(22.85, 4, 34, 1.25, 1.00),
        core(5.8, 6, 0, 2.3, 2.0, 0.5), core(9.0, 7, 29, 2.3, 2.0, 0.55), core(12.4, 7, 58, 2.3, 2.0, 0.55), core(15.8, 6, 27, 2.3, 2.0, 0.55), core(19.2, 5, 56, 2.3, 2.0, 0.5), core(21.8, 3, 25, 2.3, 2.0, 0.4),
      ],
      top: { y: 23.0, a: 1.7, b: 1.0 }, tubeN: 10, rmin: 0.009, twigR: 0.014, leafMarks: 70, markSize: 0.16, barkMarks: 21, candles: 64, candleSize: [0.3, 0.055],
    },
    100: {
      rings: [
        ring(4.1, 13, 0, 2.00, 1.75, { tilt: 12 }), ring(5.9, 16, 13, 1.95, 1.70, { tilt: 5 }), ring(7.7, 17, 26, 1.91, 1.64), ring(9.5, 17, 2, 1.86, 1.59), ring(11.3, 17, 15, 1.82, 1.53), ring(13.1, 16, 28, 1.77, 1.48), ring(14.9, 15, 4, 1.73, 1.42), ring(16.7, 14, 17, 1.68, 1.37), ring(18.5, 12, 30, 1.64, 1.31), ring(20.3, 10, 6, 1.59, 1.26), ring(21.9, 7, 19, 1.55, 1.20), ring(22.6, 4, 32, 1.50, 1.15),
        core(6.2, 5, 0, 2.6, 2.2, 0.5), core(10.2, 6, 29, 2.6, 2.2, 0.55), core(14.4, 5, 58, 2.6, 2.2, 0.55), core(18.6, 4, 27, 2.6, 2.2, 0.5),
      ],
      top: { y: 22.9, a: 1.9, b: 1.1 }, tubeN: 7, rmin: 0.018, twigR: 0.02, leafMarks: 34, markSize: 0.24, barkMarks: 12, candles: 40, candleSize: [0.32, 0.06],
    },
    200: {
      rings: [
        ring(4.4, 11, 0, 2.50, 2.15, { tilt: 10, jitter: 0.7 }), ring(6.7, 13, 13, 2.41, 2.06, { jitter: 0.7 }), ring(9.0, 13, 26, 2.33, 1.96, { jitter: 0.7 }), ring(11.3, 13, 2, 2.24, 1.87, { jitter: 0.7 }), ring(13.6, 12, 15, 2.15, 1.77, { jitter: 0.7 }), ring(15.9, 11, 28, 2.06, 1.68, { jitter: 0.7 }), ring(18.2, 9, 4, 1.98, 1.59, { jitter: 0.7 }), ring(20.4, 7, 17, 1.89, 1.49, { jitter: 0.7 }), ring(22.2, 4, 30, 1.80, 1.40, { jitter: 0.7 }),
        core(7.2, 4, 0, 3.0, 2.5, 0.48), core(12.4, 5, 29, 3.0, 2.5, 0.5), core(17.6, 4, 58, 3.0, 2.5, 0.48),
      ],
      top: { y: 22.8, a: 2.1, b: 1.2 }, tubeN: 5, rmin: 0.04, twigR: 0.04, leafMarks: 0, markSize: 0, barkMarks: 0, candles: 24, candleSize: [0.36, 0.07],
    },
    500: {
      rings: [
        ring(4.8, 9, 0, 2.85, 2.40, { tilt: 8, jitter: 0.4 }), ring(7.4, 10, 17, 2.74, 2.29, { jitter: 0.4 }), ring(10.0, 10, 34, 2.64, 2.17, { jitter: 0.4 }), ring(12.6, 10, 11, 2.53, 2.06, { jitter: 0.4 }), ring(15.2, 9, 28, 2.42, 1.94, { jitter: 0.4 }), ring(17.8, 8, 5, 2.31, 1.83, { jitter: 0.4 }), ring(20.2, 6, 22, 2.21, 1.71, { jitter: 0.4 }), ring(22.0, 3, 39, 2.10, 1.60, { jitter: 0.4 }),
        core(9.0, 3, 0, 3.6, 3.0, 0.45), core(15.0, 3, 29, 3.6, 3.0, 0.45),
      ],
      top: { y: 22.5, a: 2.4, b: 1.5 }, tubeN: 3, rmin: 0.1, twigR: 0.1, leafMarks: 0, markSize: 0, barkMarks: 0, candles: 12, candleSize: [0.5, 0.12],
    },
  },
  leafArea: { sun: 540, shade: 660, projectionM2: 205 },
  regions: [
    barkRegion("trunk-bark", tx("Grey-brown scaly bark of stem and lower crown stem", "Graubraune Schuppenrinde von Schaft und unterem Kronenstamm"), "chestnut-bark", 0.45, 0.4, 75000),
    barkRegion("limb-bark", tx("Bark of upper stem, limbs, branches and twigs", "Rinde von oberem Stamm, Ästen, Zweigen und Trieben"), "chestnut-bark-young", 0.12, 0.5, 40000),
    leafRegion("leaf-sun", tx("Sun leaves of the outer crown", "Sonnenblätter der Außenkrone"), "chestnut-leaf-sun", 0.16, 0.7, 900, 0.0042),
    leafRegion("leaf-shade", tx("Shade leaves of the dense inner crown", "Schattenblätter der dichten Innenkrone"), "chestnut-leaf-shade", 0.2, 0.3, 700, 0.0026),
    { id: "flower-candles", title: tx("White flower candles (panicles)", "Weiße Blütenkerzen (Rispen)"), kind: "foliage", model: "passive", material: "chestnut-candle", areaM2: 55, areaBasis: "exchange", characteristicLengthM: 0.05, geometry: "cylinder", skyViewFactor: 0.7, layers: [], heatCapacityJM2K: 900, source: "authored-estimate", reference: "roloff-2010" },
  ],
  leafRegionTitles: { sun: tx("", ""), shade: tx("", "") },
  emitterDiameter: { sun: 1.2, shade: 0.9 },
  tones: { trunk: 0.3, limb: 0.36, sun: 0.58, shade: 0.44, candle: 0.92 },
  porosity: 0.72, cdStem: 1.0,
  thermal: {
    environment: { airTempC: 20, meanRadiantTempC: 23, relativeHumidity: 0.55, windMS: 1.2, windDirectionDeg: 0, solarWM2: 650, pressurePa: 101325, diffuseFraction: 0.18 },
    assumptions: [...broadleafAssumptions("2.6 + 3.2", "0.0042", "0.0026"), tx("The flower candles are a passive region with an exchange area of about 55 m2 (some 2000 panicles of 0.03 m2); the 64 panicles drawn at 1:50 each stand for about thirty real ones and carry their mass. Outside the flowering pose the panicles are withdrawn into the crown on prismatic bones (the candles of May become the fruit stalks of September), so their region is bound with an exposed fraction of 0.05.", "Die Blütenkerzen sind eine passive Region mit einer Austauschfläche von etwa 55 m2 (rund 2000 Rispen zu 0,03 m2); die 64 bei 1:50 gezeichneten Rispen stehen je für etwa dreißig echte und tragen deren Masse. Außerhalb der Blütenpose sind die Rispen auf Schiebeknochen in die Krone zurückgezogen (aus den Kerzen im Mai werden die Fruchtstiele im September), ihre Region ist daher mit einem freien Anteil von 0,05 gebunden.")],
  },
  structureAssumptions: structureAssumptions("850"),
  poses, defaultPose: "candle-bloom", breezePose: "summer-breeze", calmPose: "summer-calm",
  sway: { period: 4.6, amp: { trunk: 0.05, upper: 0.14, limb: 0.32, outer: 0.55 }, lag: 0.35, text: tx("A deterministic %P s sway cycle about the breeze lean (5.5 m/s at 10 m; estimated first mode of the 24 m stem with its heavy crown about 0.22 Hz). Every level lags the one below it; the outer limb parts swing about half a degree.", "Ein deterministischer Schwingungszyklus von %P s um die Neigung bei Brise (5,5 m/s in 10 m Höhe; geschätzte erste Eigenform des 24 m hohen Stamms mit seiner schweren Krone etwa 0,22 Hz). Jede Ebene hinkt der darunter nach; die äußeren Astteile schwingen etwa einen halben Grad.") },
  gust: { from: 5.5, peak: 15, text: tx("A single gust over 8 s from %A to %B m/s at 10 m: the wind-load tracks rise with the reconfigured drag law and the crown leans with a level-wise lag, outer limbs first, the stem last.", "Eine einzelne Böe über 8 s von %A auf %B m/s in 10 m Höhe: Die Windlast-Spuren steigen mit dem umgeformten Widerstandsgesetz, die Krone neigt sich ebenenweise verzögert, zuerst die äußeren Äste, zuletzt der Stamm.") },
  diurnal: {
    text: tx("A 24 h summer day compressed to 24 s (1 s = 1 h), calm: the sunlit share of every region and the transpiration of the sun and shade leaves follow the sun, with stomata nearly closed at night and a mid-afternoon depression; leaf and bark temperatures are not authored but follow from the thermal solver.", "Ein 24-h-Sommertag auf 24 s gerafft (1 s = 1 h) bei Windstille: Der besonnte Anteil jeder Region und die Transpiration der Sonnen- und Schattenblätter folgen der Sonne, nachts sind die Stomata fast geschlossen, am Nachmittag gibt es eine Mittagsdepression; Blatt- und Rindentemperaturen werden nicht vorgegeben, sondern folgen aus dem thermischen Löser."),
    sun: [[0, 0.03], [5, 0.05], [7, 0.35], [9, 0.85], [11.5, 1], [14, 0.72], [16, 0.76], [18, 0.3], [20, 0.07], [24, 0.03]],
    shade: [[0, 0.02], [5, 0.03], [7, 0.2], [9, 0.55], [12, 0.7], [14, 0.5], [16, 0.52], [18, 0.2], [20, 0.04], [24, 0.02]],
    sunlit: diurnalSunlit({ "trunk-bark": 0.28, "limb-bark": 0.2, "leaf-sun": 0.9, "leaf-shade": 0.06, "flower-candles": 0.02 }),
  },
  candles: { length: 0.26, radius: 0.055, lean: 14, retract: 0.62, massKg: 0.8, region: "flower-candles", material: "chestnut-candle", title: tx("Flower candles", "Blütenkerzen"), minY: 5.5 },
  extraClips: (ctx) => [{
    id: "candle-emergence", title: tx("Candles come out", "Die Kerzen treiben aus"),
    description: tx("Late April to mid-May compressed to 6 s, calm: the flower panicles push up from the shoot tips until they stand as white candles over the whole crown.", "Ende April bis Mitte Mai auf 6 s gerafft, windstill: Die Blütenrispen schieben sich aus den Triebspitzen, bis sie als weiße Kerzen über der ganzen Krone stehen."),
    durationS: 6, fps: 25, loop: false, rootMotion: "none",
    keys: [{ t: 0, pose: "summer-calm", ease: "smooth" }, { t: 6, pose: "candle-bloom", ease: "smooth" }], contacts: [],
    phases: [{ id: "budburst", fromS: 0, toS: 2.5, title: tx("Panicles in the bud", "Rispen in der Knospe") }, { id: "emergence", fromS: 2.5, toS: 6, title: tx("Candles rise", "Kerzen richten sich auf") }],
    events: [{ t: 6, kind: "motion", title: tx("Full bloom", "Vollblüte") }],
    tracks: [{ target: "region:flower-candles:exposedFraction", ease: "smooth", keys: [{ t: 0, v: 0.05 }, { t: 6, v: 1 }] }],
  }],
  leafMark, barkMark,
  texts: {
    scale: {
      50: {
        silhouette: tx("At 1:50 (1 m is 20 mm) the horse chestnut is drawn as the true views of its 1:50 body: the flared collar and the short massive stem, the five tiers of scaffold limbs with their side branches and twigs, the broad dome of some three hundred rounded leaf lobes whose scallops read as the heavy palmate foliage, and the white flower candles on the outer shoots.", "Bei 1:50 (1 m sind 20 mm) ist die Rosskastanie als wahre Ansichten ihres Körpers 1:50 gezeichnet: der ausgestellte Wurzelhals und der kurze, mächtige Stamm, die fünf Etagen von Gerüstästen mit Seitenzweigen und Trieben, die breite Kuppel aus rund dreihundert gerundeten Laubkörpern, deren Bögen sich als schweres gefingertes Laub lesen, und die weißen Blütenkerzen auf den äußeren Trieben."),
        merged: tx("The wood of one bone is one outline (limb, its side branches and the twigs it sends into its foliage); the leaf lobes of one carrying bone and leaf region are one scalloped outline with its sky openings; the panicles of one shoot group are one outline.", "Das Holz eines Knochens ist ein Umriss (Ast, seine Seitenzweige und die Triebe, die er in sein Laub schickt); die Laubkörper eines tragenden Knochens und einer Blattregion sind ein gebogter Umriss mit seinen Himmelslücken; die Rispen einer Triebgruppe sind ein Umriss."),
        omitted: tx("Omitted: the single leaflets and buds below the print minimum; leaf marks and bark plates where nearer foliage covers them.", "Weggelassen: einzelne Teilblättchen und Knospen unter dem Druckminimum; Blattzeichen und Rindenplatten, wo näheres Laub sie verdeckt."),
        thinElements: tx("Branches and twigs taper to 18 mm, still 0.24 mm at the coarse end 1:75 of this drawing; leaf and bark marks are 0.13 mm texture strokes inside their outlines.", "Zweige und Triebe verjüngen sich bis 18 mm, am groben Ende 1:75 dieser Zeichnung noch 0,24 mm; Blatt- und Rindenzeichen sind 0,13-mm-Texturstriche innerhalb ihrer Umrisse."),
        styles: tx("Light line: closed outlines with the seven-fingered leaf marks and the scaly bark strokes. Dark solid: the dense crown reads as a dark dome, the shade core darker, the stem and the white candles lighter.", "Helle Linie: geschlossene Umrisse mit den siebenfingrigen Blattzeichen und den schuppigen Rindenstrichen. Dunkle Fläche: Die dichte Krone liest sich als dunkle Kuppel, der Schattenkern dunkler, Stamm und weiße Kerzen heller."),
      },
      100: {
        silhouette: tx("At 1:100 (1 m is 10 mm) the true views of the 1:100 body: stem and the twenty scaffold limbs with their main side branches, the dome of larger lobes with fewer, deeper scallops, and fewer but larger flower candles.", "Bei 1:100 (1 m sind 10 mm) die wahren Ansichten des Körpers 1:100: der Stamm und die zwanzig Gerüstäste mit ihren Hauptseitenzweigen, die Kuppel aus größeren Laubkörpern mit weniger, tieferen Bögen und weniger, aber größere Blütenkerzen."),
        merged: tx("One outline per bone for the wood and per carrying bone and leaf region for the foliage; the outer side branches are merged into their limbs.", "Ein Umriss je Knochen für das Holz und je tragendem Knochen und Blattregion für das Laub; die äußeren Seitenzweige sind in ihre Äste aufgegangen."),
        omitted: tx("Omitted: the third side branch of every limb, the finest twigs and every leaf mark that a nearer lobe covers.", "Weggelassen: der dritte Seitenzweig jedes Astes, die feinsten Triebe und jedes Blattzeichen, das ein näherer Laubkörper verdeckt."),
        thinElements: tx("Wood is never thinner than 36 mm (0.24 mm at 1:150, the coarse end of this drawing); marks are 0.1 mm texture strokes.", "Holz ist nie dünner als 36 mm (0,24 mm bei 1:150, dem groben Ende dieser Zeichnung); Zeichen sind 0,1-mm-Texturstriche."),
        styles: tx("Light line: outlines and a few leaf and bark marks. Dark solid: crown dark, core darker, stem and candles lighter.", "Helle Linie: Umrisse und wenige Blatt- und Rindenzeichen. Dunkle Fläche: Krone dunkel, Kern dunkler, Stamm und Kerzen heller."),
      },
      200: {
        silhouette: tx("At 1:200 (1 m is 5 mm) the dome is composed of about eighty large lobes over the shaded core; stem, scaffold limbs and the first side branch of the lower limbs are drawn at a printable width; the candles become small white tufts.", "Bei 1:200 (1 m sind 5 mm) ist die Kuppel aus etwa achtzig großen Laubkörpern über dem beschatteten Kern komponiert; Stamm, Gerüstäste und der erste Seitenzweig der unteren Äste sind in druckbarer Breite gezeichnet; die Kerzen werden zu kleinen weißen Büscheln."),
        merged: tx("The wood of one bone is one outline; the lobes of one carrying bone and region are one outline; small sky openings are closed.", "Das Holz eines Knochens ist ein Umriss; die Laubkörper eines tragenden Knochens und einer Region sind ein Umriss; kleine Himmelslücken sind geschlossen."),
        omitted: tx("Omitted: twigs, the outer side branches, leaf and bark texture.", "Weggelassen: Triebe, die äußeren Seitenzweige, Blatt- und Rindentextur."),
        thinElements: tx("Limbs are drawn at least 80 mm wide, 0.23 mm at 1:350, the coarse end of this drawing.", "Äste sind mindestens 80 mm breit gezeichnet, 0,23 mm bei 1:350, dem groben Ende dieser Zeichnung."),
        styles: tx("Light line: closed outlines only. Dark solid: one dark crown with a darker core and a lighter stem.", "Helle Linie: nur geschlossene Umrisse. Dunkle Fläche: eine dunkle Krone mit dunklerem Kern und hellerem Stamm."),
      },
      500: {
        silhouette: tx("At 1:500 (1 m is 2 mm) the horse chestnut is one broad dome of some fifty massive lobes on its short stem; the wide skirt of the sweeping lower limbs keeps the characteristic low, round crown base; a dozen candle tufts mark the flowering.", "Bei 1:500 (1 m sind 2 mm) ist die Rosskastanie eine breite Kuppel aus etwa fünfzig massigen Laubkörpern auf ihrem kurzen Stamm; der weite Saum der ausschwingenden unteren Äste bewahrt den typischen tiefen, runden Kronenansatz; ein Dutzend Kerzenbüschel kennzeichnet die Blüte."),
        merged: tx("Foliage of one carrying bone and region is one outline without sky openings; the wood of one bone is one outline; the candles are typified as twelve larger tufts.", "Das Laub eines tragenden Knochens und einer Region ist ein Umriss ohne Himmelslücken; das Holz eines Knochens ist ein Umriss; die Kerzen sind zu zwölf größeren Büscheln typisiert."),
        omitted: tx("Omitted: every side branch and twig, all texture; inner limbs show only where the crown leaves them visible.", "Weggelassen: alle Seitenzweige und Triebe, jede Textur; innere Äste zeigen sich nur, wo die Krone sie sichtbar lässt."),
        thinElements: tx("Wood is drawn at least 200 mm wide, the smallest width that still prints at 1:1000, the coarse end of this drawing.", "Holz ist mindestens 200 mm breit gezeichnet, die kleinste Breite, die bei 1:1000, dem groben Ende dieser Zeichnung, noch druckt."),
        styles: tx("Light line: closed outlines only. Dark solid: the crown as one dark mass, the stem lighter.", "Helle Linie: nur geschlossene Umrisse. Dunkle Fläche: die Krone als eine dunkle Masse, der Stamm heller."),
      },
    },
    volume: {
      50: tx("Body at 1:50 (smallest feature 7.5 mm): the flared collar and the stem as one tapered loft per stem bone, twenty scaffold limbs in five tiers (the lowest sweeping down and up again), sixty side branches and a twig into every leaf lobe, all lofted through hand-stated stations that start inside their parent so that sway never opens a joint; the crown of rounded lobes in sixteen rings that follow the domed envelope, with five rings of large shaded core lobes round the stem; sixty-four flower panicles stand on the highest sun lobes.", "Körper bei 1:50 (kleinstes Merkmal 7,5 mm): der ausgestellte Wurzelhals und der Stamm als je ein verjüngter Loft je Stammknochen, zwanzig Gerüstäste in fünf Etagen (die untersten nach unten und wieder hinauf schwingend), sechzig Seitenzweige und ein Trieb in jeden Laubkörper, alle durch von Hand gesetzte Stationen gelofted, die im Mutterteil beginnen, sodass kein Gelenk beim Schwingen aufgeht; die Krone aus gerundeten Laubkörpern in sechzehn Ringen entlang der gewölbten Hülle, mit fünf Ringen großer beschatteter Kernkörper um den Stamm; vierundsechzig Blütenrispen stehen auf den höchsten Sonnenkörpern."),
      100: tx("Body at 1:100 (smallest feature 15 mm): the same stem and twenty limbs with forty side branches at fewer stations, wood never thinner than 36 mm, thirteen rings of larger lobes and four core rings, forty panicles.", "Körper bei 1:100 (kleinstes Merkmal 15 mm): derselbe Stamm und zwanzig Äste mit vierzig Seitenzweigen an weniger Stationen, Holz nie dünner als 36 mm, dreizehn Ringe größerer Laubkörper und vier Kernringe, vierzig Rispen."),
      200: tx("Body at 1:200 (smallest feature 30 mm): stem and limbs at five stations, wood at least 80 mm, twenty side branches of the lower tiers, nine rings of large lobes and three core rings, twenty-four panicles.", "Körper bei 1:200 (kleinstes Merkmal 30 mm): Stamm und Äste an fünf Stationen, Holz mindestens 80 mm, zwanzig Seitenzweige der unteren Etagen, neun Ringe großer Laubkörper und drei Kernringe, vierundzwanzig Rispen."),
      500: tx("Body at 1:500 (smallest feature 75 mm): stem and limbs as three-station lofts at least 200 mm thick, no side branches, six rings of massive lobes and two core rings: the dome of the tree; twelve panicles typified as larger tufts (envelope solids against the finer panicles of the same bones).", "Körper bei 1:500 (kleinstes Merkmal 75 mm): Stamm und Äste als Lofts mit drei Stationen, mindestens 200 mm dick, keine Seitenzweige, sechs Ringe massiger Laubkörper und zwei Kernringe: die Kuppel des Baums; zwölf Rispen typisiert als größere Büschel (Hüllkörper gegen die feineren Rispen derselben Knochen)."),
    },
    symbol: tx("A conventional plan sign for site and landscape plans: the true plan outline of the broad crown, the stem section at breast height and the directions of the four lowest scaffold limbs. It is a separate symbol and is never used silently for a pictorial scale.", "Ein konventionelles Grundrisszeichen für Lage- und Freiraumpläne: der wahre Grundrissumriss der breiten Krone, der Stammquerschnitt in Brusthöhe und die Richtungen der vier untersten Gerüstäste. Es ist ein eigenes Symbol und wird nie stillschweigend für einen Bildmaßstab verwendet."),
  },
  symbolLimbs: ["t1-1", "t1-2", "t1-3", "t1-4"],
  defaultScale: 200,
};
