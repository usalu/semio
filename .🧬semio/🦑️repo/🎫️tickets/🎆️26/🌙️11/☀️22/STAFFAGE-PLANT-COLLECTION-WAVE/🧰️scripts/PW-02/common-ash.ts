/** 🍃️ Common ash (Fraxinus excelsior), an open-grown park ash of about 80 years: a tall pale grey stem with diamond-fissured bark that forks at 10 m into three steep leaders, limbs that dip and turn up at their tips, and an open, airy crown of loose pinnate-leaf clusters with sky between them. Every number is chosen by hand here.
 * @see ./kit.ts
 * @see ../../📝️PW-02.md */
import { tx, type BranchSpec, type LimbSpec, type PoseSpec, type Scale, type Species, type Text } from "./kit.ts";
import { COMMON_REFERENCES, barkRegion, broadleafAssumptions, diurnalSunlit, leafRegion, structureAssumptions } from "./common.ts";

const sideOf = (az: number): Text => {
  const k = Math.round((((az % 360) + 360) % 360) / 90) % 4;
  return [tx("east", "Ost"), tx("north", "Nord"), tx("west", "West"), tx("south", "Süd")][k]!;
};
const named = (base: Text, az: number): Text => {
  const s = sideOf(az);
  return tx(`${base.en}, ${s.en}`, `${base.de}, ${s.de}`);
};

const LEADERS: [string, number, number, [number, number][], number, number][] = [
  ["lead-a", 9.55, 22, [[0, 80], [0.5, 74], [1, 66]], 12.2, 0.235],
  ["lead-b", 9.75, 148, [[0, 75], [0.5, 66], [1, 58]], 11.0, 0.215],
  ["lead-c", 9.9, 262, [[0, 72], [0.5, 64], [1, 55]], 10.2, 0.2],
];
const LOW: [string, number, number[], [number, number][], number, number][] = [
  ["t1", 5.9, [64, 186, 308], [[0, 40], [0.45, 22], [0.75, 24], [1, 44]], 7.0, 0.15],
  ["t2", 8.0, [118, 244, 2], [[0, 46], [0.5, 30], [0.8, 34], [1, 50]], 6.4, 0.13],
];

const limbs: LimbSpec[] = [];
const branches: BranchSpec[] = [];
for (const [id, y, az, elev, L, r0] of LEADERS) limbs.push({ id, parent: "s1", at: y, az, elev, L, r0, r1: 0.04, split: 0.5, bend: 6, title: named(tx("Leader of the stem fork", "Leittrieb der Stammgabel"), az), titleB: named(tx("Leader of the stem fork, upper part", "Leittrieb der Stammgabel, oberer Teil"), az), limit: 5 });
for (const [tier, y, azs, elev, L, r0] of LOW)
  azs.forEach((az, k) => limbs.push({ id: `${tier}-${k + 1}`, parent: "stem", at: y + 0.25 * k, az, elev, L: L * (1 + 0.05 * Math.sin(k * 2.3)), r0, r1: 0.035, split: 0.5, bend: 10 * Math.sin(k * 1.9 + y), title: named(tx(tier === "t1" ? "Lowest limb" : "Second-tier limb", tier === "t1" ? "Unterster Ast" : "Ast der zweiten Etage"), az), titleB: named(tx(tier === "t1" ? "Lowest limb, upturned outer part" : "Second-tier limb, upturned outer part", tier === "t1" ? "Unterster Ast, aufgebogener Außenteil" : "Ast der zweiten Etage, aufgebogener Außenteil"), az), limit: 10 }));
for (const [lead, , az0] of LEADERS) {
  [[0.32, 62, [[0, 46], [0.5, 30], [1, 44]], 5.4, 0.105], [0.6, -66, [[0, 50], [0.5, 36], [1, 48]], 4.6, 0.09], [0.86, 44, [[0, 56], [1, 46]], 3.0, 0.06]].forEach(([t, daz, elev, L, r0], j) => {
    const az = (az0 as number) + (daz as number);
    limbs.push({ id: `${lead}-${j + 1}`, parent: lead, at: t as number, az, elev: elev as [number, number][], L: L as number, r0: r0 as number, r1: 0.03, bend: 8, title: named(tx(`Limb ${j + 1} of the leader ${lead.slice(-1)}`, `Ast ${j + 1} des Leittriebs ${lead.slice(-1)}`), az), limit: 11 });
  });
}
const sc: Scale[][] = [[50, 100, 200], [50, 100], [50]];
for (const l of limbs) {
  const lead = l.id.startsWith("lead-") && !/-\d$/.test(l.id);
  const rows: [number, number, number][] = lead ? [[0.45, 70, 0.3], [0.7, -60, 0.26], [0.9, 50, 0.2]] : [[0.4, 40, 0.38], [0.65, -36, 0.32], [0.85, 30, 0.26]];
  rows.forEach(([t, daz, fL], j) => branches.push({ id: `${l.id}-${j + 1}`, limb: l.id, t, az: l.az + daz, elev: [[0, 34], [0.6, 18], [1, 36]], L: l.L * fL, r0: l.r0 * 0.38, r1: 0.016, bend: daz * 0.3, scales: sc[j]! }));
}

/** 🌐️ Crown envelope: an open, irregular ovoid, widest (7.4 m) at 14.5 m, crown base 5.6 m, top 25.6 m, fuller to the south-west. */
const envelope = (y: number, az: number): number => {
  const [ym, base, top, R] = [14.5, 5.6, 25.6, 7.4];
  const t = y < ym ? (ym - y) / (ym - base) : (y - ym) / (top - ym);
  const p = y < ym ? 2.3 : 1.9;
  const f = Math.pow(Math.max(0, 1 - Math.pow(Math.min(1, t), p)), 1 / p);
  const a = (az * Math.PI) / 180;
  return R * f * (1 + 0.07 * Math.cos(a - 3.6) + 0.05 * Math.cos(2 * a + 0.5) + 0.04 * Math.cos(3 * a - 1.2));
};

const ring = (y: number, n: number, phase: number, a: number, b: number, extra: Partial<Species["scales"][50]["rings"][0]> = {}) => ({ y, n, phase, a, b, region: "sun" as const, jitter: 1.4, ck: 0.9, ...extra });
const core = (y: number, n: number, phase: number, a: number, b: number, reach: number) => ({ y, n, phase, a, b, region: "shade" as const, reach, ck: 0.8, jitter: 0.8 });

const calm = { "trunk-bark": { exposedFraction: 1, sunlitFraction: 0.4 }, "limb-bark": { exposedFraction: 1, sunlitFraction: 0.3 }, "leaf-sun": { exposedFraction: 1, sunlitFraction: 0.85 }, "leaf-shade": { exposedFraction: 1, sunlitFraction: 0.16 } };
const poses: PoseSpec[] = [
  { id: "summer-calm", title: tx("Summer calm", "Sommerliche Windstille"), description: tx("July noon, light air of 1.2 m/s: the open crown lets sun through to the stem and the inner leaves (light shade typical of ash); full transpiration.", "Juli, Mittag, leiser Zug von 1,2 m/s: Die offene Krone lässt Sonne bis an den Stamm und die inneren Blätter (der lichte Schatten der Esche); volle Transpiration."), v10: 1.2, cd: 0.45, lean: 0, droop: 0, candlesOut: false, regions: calm, emitters: { sun: 1, shade: 0.78 } },
  { id: "summer-breeze", title: tx("Summer breeze", "Sommerbrise"), description: tx("Moderate breeze of 5.5 m/s at 10 m (Beaufort 4): the light pinnate leaves flutter and the leaders lean slightly downwind; drag F = 0.5 rho cd A v^2 with cd 0.35 on the open, porous crown.", "Mäßige Brise von 5,5 m/s in 10 m Höhe (Beaufort 4): Die leichten Fiederblätter flattern und die Leittriebe neigen sich leicht in Windrichtung; Widerstand F = 0,5 rho cd A v^2 mit cd 0,35 auf der offenen, porösen Krone."), v10: 5.5, cd: 0.35, lean: 0.2, droop: 0, candlesOut: false, regions: { ...calm, "leaf-shade": { exposedFraction: 1, sunlitFraction: 0.2 } }, emitters: { sun: 0.95, shade: 0.75 } },
  { id: "gale", title: tx("Summer gale", "Sommersturm"), description: tx("Gale of 22 m/s at 10 m (Beaufort 9), overcast: the pinnate leaves fold their leaflets together (cd 0.22 after Vogel 1989); the three leaders of the fork lean downwind with their open crown.", "Sturm von 22 m/s in 10 m Höhe (Beaufort 9), bedeckt: Die Fiederblätter legen ihre Blättchen zusammen (cd 0,22 nach Vogel 1989); die drei Leittriebe der Gabel neigen sich mit ihrer offenen Krone in Windrichtung."), v10: 22, cd: 0.22, lean: 1.8, droop: 0, candlesOut: false,
    regions: { "trunk-bark": { exposedFraction: 1, sunlitFraction: 0.04, wettedness: 0.4 }, "limb-bark": { exposedFraction: 1, sunlitFraction: 0.03, wettedness: 0.4 }, "leaf-sun": { exposedFraction: 1, sunlitFraction: 0.09, wettedness: 0.25 }, "leaf-shade": { exposedFraction: 1, sunlitFraction: 0.02, wettedness: 0.15 } }, emitters: { sun: 0.35, shade: 0.25 } },
  { id: "spring-flush", title: tx("Late spring flush", "Später Austrieb"), description: tx("Late May: the ash, one of the last trees to leaf out, has just unfolded its pinnate leaves; the young leaves transpire little and the crown is still light, so stem and limbs stand in sun; calm.", "Ende Mai: Die Esche, einer der letzten Bäume im Austrieb, hat eben ihre Fiederblätter entfaltet; die jungen Blätter transpirieren wenig und die Krone ist noch licht, Stamm und Äste stehen daher in der Sonne; windstill."), v10: 1.5, cd: 0.45, lean: 0, droop: 0, candlesOut: false,
    regions: { "trunk-bark": { exposedFraction: 1, sunlitFraction: 0.6 }, "limb-bark": { exposedFraction: 1, sunlitFraction: 0.5 }, "leaf-sun": { exposedFraction: 1, sunlitFraction: 0.9 }, "leaf-shade": { exposedFraction: 1, sunlitFraction: 0.35 } }, emitters: { sun: 0.4, shade: 0.3 } },
  { id: "seed-keys", title: tx("Bunches of keys", "Fruchtbüschel"), description: tx("October: dense bunches of winged seeds (keys) hang from the outer shoots, about 0.35 kg of fruit per m2 of crown plan area on a heavily fruiting female ash; the upturned limb tips bend a little under the load, calm.", "Oktober: Dichte Büschel geflügelter Nüsschen hängen an den äußeren Trieben, etwa 0,35 kg Früchte je m2 Kronengrundfläche an einer stark fruchtenden weiblichen Esche; die aufgebogenen Astspitzen biegen sich unter der Last ein wenig, windstill."), v10: 1.2, cd: 0.45, lean: 0, droop: 0.5, candlesOut: false, fruitKgM2: 0.35, fruitTitle: tx("Weight of the bunches of keys", "Gewicht der Fruchtbüschel"),
    regions: { "trunk-bark": { exposedFraction: 1, sunlitFraction: 0.32 }, "limb-bark": { exposedFraction: 1, sunlitFraction: 0.25 }, "leaf-sun": { exposedFraction: 1, sunlitFraction: 0.75 }, "leaf-shade": { exposedFraction: 1, sunlitFraction: 0.14 } }, emitters: { sun: 0.55, shade: 0.42 } },
  { id: "wet-snow", title: tx("Early wet snow", "Früher Nassschnee"), description: tx("An early snowfall on the still green crown in October: 8 kg of wet snow per m2 of foliage plan area (order of magnitude from EN 1991-1-3; less sticks to the open crown) bends the limbs down; calm, no sun.", "Ein früher Schneefall im Oktober auf die noch grüne Krone: 8 kg nasser Schnee je m2 Laubgrundfläche (Größenordnung nach EN 1991-1-3; auf der offenen Krone haftet weniger) biegt die Äste nieder; windstill, keine Sonne."), v10: 1, cd: 0.45, lean: 0, droop: 2.2, snowKgM2: 8, candlesOut: false,
    regions: { "trunk-bark": { exposedFraction: 1, sunlitFraction: 0.01, wettedness: 1 }, "limb-bark": { exposedFraction: 1, sunlitFraction: 0.01, wettedness: 1 }, "leaf-sun": { exposedFraction: 1, sunlitFraction: 0.03, wettedness: 1 }, "leaf-shade": { exposedFraction: 1, sunlitFraction: 0.005, wettedness: 1 } }, emitters: { sun: 0.05, shade: 0.04 } },
];

/** 🪶️ A pinnate ash leaf mark: rachis with four pairs of leaflet ticks and the terminal leaflet, one stroke. */
function leafMark(c: { x: number; y: number }, size: number, deg: number): { x: number; y: number }[][] {
  const a = (deg * Math.PI) / 180;
  const at = (u: number, v: number) => ({ x: c.x + size * (u * Math.cos(a) - v * Math.sin(a)), y: c.y + size * (u * Math.sin(a) + v * Math.cos(a)) });
  const out = [at(-1, 0)];
  [-0.6, -0.2, 0.2, 0.6].forEach((u) => out.push(at(u, 0), at(u + 0.22, 0.32), at(u, 0), at(u + 0.22, -0.32), at(u, 0)));
  out.push(at(1.05, 0));
  return [out];
}

/** 💎️ The diamond network of fissures on the pale grey stem of an old ash: alternating diagonal strokes. */
function barkMark(k: number, view: string, stem: (y: number) => { x: number; z: number; r: number }): { x: number; y: number }[] | null {
  const row = Math.floor(k / 3), col = k % 3;
  const y = 0.6 + row * 0.55;
  if (y > 5.2) return null;
  const s = stem(y);
  const cx = view === "elevation" ? s.x : s.z;
  const u = (col - 1) * 0.5 + (row % 2) * 0.22;
  const sgn = (row + col) % 2 === 0 ? 1 : -1;
  return [0, 1, 2].map((i) => ({ x: cx + (u + sgn * (i - 1) * 0.12) * s.r, y: y + (i - 1) * 0.2 }));
}

export const species: Species = {
  id: "common-ash",
  title: tx("Common Ash", "Gemeine Esche"),
  summary: tx("A mature open-grown common ash (Fraxinus excelsior) of about 25 m: a tall pale grey stem with diamond-fissured bark that forks at 10 m into three steep leaders, limbs that dip and turn up at their tips, and an open, airy crown of loose pinnate-leaf clusters with sky between them.", "Eine ausgewachsene, frei stehende Gemeine Esche (Fraxinus excelsior) von etwa 25 m: ein hoher, hellgrauer Stamm mit rautenförmig gefurchter Rinde, der sich in 10 m in drei steile Leittriebe gabelt, Äste, die durchhängen und an den Spitzen aufbiegen, und eine offene, luftige Krone aus lockeren Fiederblattbüscheln mit Himmel dazwischen."),
  distinction: tx("A 25.6 m common ash (Fraxinus excelsior) with a tall pale grey stem of 0.8 m forking at 10 m into three leaders, upturned limb tips and an open 15 m crown of pinnate-leaf clusters through which the sky shows.", "Eine 25,6 m hohe Gemeine Esche (Fraxinus excelsior) mit hohem, hellgrauem Stamm von 0,8 m, der sich in 10 m in drei Leittriebe gabelt, aufgebogenen Astspitzen und einer offenen Krone von 15 m aus Fiederblattbüscheln, durch die der Himmel scheint."),
  tags: ["tree", "deciduous", "broadleaf", "park-tree", "open-crown", "pinnate-leaves", "fruit", "wind", "transpiration"],
  created: "2026-11-22",
  notes: tx("Rig and drawings are authored by hand for an open-grown park ash of about 80 years: a clear stem of 5.5 m, the stem fork at 10 m with three leaders to 24 m, six lower limbs in two tiers and nine limbs on the leaders, all dipping and turning up at their tips as ash limbs do. Crown size and habit follow Roloff (2010) as orders of magnitude; wood properties, masses and conductances are authored estimates.", "Rig und Zeichnungen sind von Hand für eine frei stehende Parkesche von etwa 80 Jahren entworfen: ein Schaft von 5,5 m, die Stammgabel in 10 m mit drei Leittrieben bis 24 m, sechs untere Äste in zwei Etagen und neun Äste an den Leittrieben, alle durchhängend und an den Spitzen aufgebogen, wie es Eschenäste tun. Kronengröße und Wuchs folgen Roloff (2010) als Größenordnungen; Holzeigenschaften, Massen und Leitfähigkeiten sind Autorenschätzungen."),
  references: [...COMMON_REFERENCES],
  materials: [
    { id: "ash-bark", title: tx("Pale grey ash bark with a diamond network of fissures", "Hellgraue Eschenrinde mit rautenförmigem Furchennetz"), emissivity: 0.95, solarAbsorptivity: 0.66, densityKgM3: 650, conductivityWmK: 0.13, source: "authored-estimate", reference: "wood-handbook-2021" },
    { id: "ash-bark-young", title: tx("Smooth greenish-grey bark of limbs and stout shoots", "Glatte grünlichgraue Rinde der Äste und kräftigen Triebe"), emissivity: 0.95, solarAbsorptivity: 0.65, densityKgM3: 650, conductivityWmK: 0.13, source: "authored-estimate", reference: "wood-handbook-2021" },
    { id: "ash-wood", title: tx("Green ash wood", "Frisches Eschenholz"), emissivity: 0.9, solarAbsorptivity: 0.55, densityKgM3: 800, conductivityWmK: 0.16, elasticModulusPa: 9000000000, strengthPa: 60000000, source: "authored-estimate", reference: "wood-handbook-2021" },
    { id: "ash-leaf-sun", title: tx("Ash sun leaflets", "Eschen-Sonnenblättchen"), emissivity: 0.97, solarAbsorptivity: 0.5, source: "literature", reference: "jones-2014" },
    { id: "ash-leaf-shade", title: tx("Ash shade leaflets (effective)", "Eschen-Schattenblättchen (wirksam)"), emissivity: 0.97, solarAbsorptivity: 0.32, source: "authored-estimate", reference: "jones-2014" },
  ],
  barkMaterial: "ash-bark", youngBarkMaterial: "ash-bark-young", woodMaterial: "ash-wood", sunMaterial: "ash-leaf-sun", shadeMaterial: "ash-leaf-shade",
  density: 800, foliageKgPerM2: 0.2,
  stem: {
    stations: [[0, 0, 0, 0.7], [0.12, 0, 0, 0.56], [0.35, 0, 0, 0.47], [0.8, 0.01, 0, 0.42], [1.3, 0.02, 0, 0.4], [3, 0.04, 0.01, 0.38], [5.5, 0.05, 0.03, 0.35], [8, 0.04, 0.05, 0.32], [10, 0.03, 0.06, 0.3]],
    bones: [
      { id: "root-plate", y0: 0, y1: 0.45, title: tx("Root plate and collar", "Wurzelteller und Wurzelhals"), limit: 0, region: "trunk-bark" },
      { id: "trunk", y0: 0.45, y1: 5.5, title: tx("Clear stem", "Schaft"), limit: 3, region: "trunk-bark" },
      { id: "s1", y0: 5.5, y1: 10, title: tx("Stem up to the fork", "Stamm bis zur Gabel"), limit: 3, region: "trunk-bark" },
    ],
  },
  limbs, branches, envelope,
  scales: {
    50: {
      rings: [
        ring(6.4, 11, 8, 1.25, 0.8, { tilt: 8 }), ring(7.9, 13, 30, 1.25, 0.82), ring(9.4, 14, 14, 1.25, 0.85), ring(10.9, 15, 2, 1.25, 0.85), ring(12.4, 15, 22, 1.25, 0.85), ring(13.9, 15, 9, 1.25, 0.85),
        ring(15.4, 15, 27, 1.2, 0.85), ring(16.9, 14, 15, 1.2, 0.85), ring(18.4, 13, 3, 1.15, 0.82), ring(19.9, 11, 21, 1.1, 0.8), ring(21.4, 9, 9, 1.05, 0.8), ring(22.8, 7, 27, 1.0, 0.78), ring(24.0, 4, 14, 0.95, 0.75),
        core(9.0, 4, 40, 1.6, 1.0, 0.5), core(13.0, 5, 10, 1.7, 1.05, 0.52), core(17.0, 4, 50, 1.5, 1.0, 0.5),
      ],
      top: { y: 24.9, a: 0.95, b: 0.7 }, tubeN: 10, rmin: 0.009, twigR: 0.013, leafMarks: 64, markSize: 0.15, barkMarks: 27,
    },
    100: {
      rings: [
        ring(6.6, 9, 12, 1.5, 0.95, { tilt: 6, ck: 0.95 }), ring(8.5, 10, 32, 1.5, 1.0, { ck: 0.95 }), ring(10.4, 11, 6, 1.5, 1.0, { ck: 0.95 }), ring(12.3, 11, 26, 1.5, 1.0, { ck: 0.95 }), ring(14.2, 11, 12, 1.5, 1.0, { ck: 0.95 }),
        ring(16.1, 11, 30, 1.45, 1.0, { ck: 0.95 }), ring(18.0, 10, 16, 1.4, 1.0, { ck: 0.95 }), ring(19.9, 8, 4, 1.3, 0.95, { ck: 0.95 }), ring(21.7, 6, 24, 1.2, 0.92, { ck: 0.95 }), ring(23.3, 4, 10, 1.1, 0.88, { ck: 0.95 }),
        core(9.8, 4, 20, 1.9, 1.2, 0.5), core(14.8, 4, 60, 1.9, 1.2, 0.5),
      ],
      top: { y: 24.6, a: 1.05, b: 0.85 }, tubeN: 7, rmin: 0.018, twigR: 0.02, leafMarks: 30, markSize: 0.22, barkMarks: 15,
    },
    200: {
      rings: [
        ring(7.0, 7, 18, 1.9, 1.3, { tilt: 5, ck: 1.05, jitter: 0.8 }), ring(9.5, 8, 40, 1.9, 1.35, { ck: 1.05, jitter: 0.8 }), ring(12.0, 9, 10, 1.9, 1.4, { ck: 1.05, jitter: 0.8 }), ring(14.5, 9, 32, 1.85, 1.4, { ck: 1.05, jitter: 0.8 }),
        ring(17.0, 8, 4, 1.8, 1.35, { ck: 1.05, jitter: 0.8 }), ring(19.5, 7, 26, 1.65, 1.3, { ck: 1.05, jitter: 0.8 }), ring(21.9, 5, 12, 1.45, 1.2, { ck: 1.05, jitter: 0.8 }),
        core(11.0, 3, 30, 2.3, 1.6, 0.48), core(16.0, 3, 80, 2.2, 1.5, 0.48),
      ],
      top: { y: 24.0, a: 1.3, b: 1.2 }, tubeN: 5, rmin: 0.04, twigR: 0.04, leafMarks: 0, markSize: 0, barkMarks: 0,
    },
    500: {
      rings: [
        ring(7.6, 6, 25, 2.4, 1.75, { tilt: 4, ck: 1.2, jitter: 0.4 }), ring(10.8, 7, 50, 2.4, 1.85, { ck: 1.2, jitter: 0.4 }), ring(14.0, 7, 15, 2.35, 1.9, { ck: 1.2, jitter: 0.4 }),
        ring(17.2, 6, 45, 2.2, 1.8, { ck: 1.2, jitter: 0.4 }), ring(20.3, 5, 10, 1.95, 1.65, { ck: 1.2, jitter: 0.4 }),
        core(12.5, 3, 60, 2.8, 2.0, 0.45),
      ],
      top: { y: 23.4, a: 1.75, b: 1.75 }, tubeN: 3, rmin: 0.1, twigR: 0.1, leafMarks: 0, markSize: 0, barkMarks: 0,
    },
  },
  leafArea: { sun: 330, shade: 170, projectionM2: 168 },
  regions: [
    barkRegion("trunk-bark", tx("Pale grey diamond-fissured bark of the stem", "Hellgraue, rautenförmig gefurchte Rinde des Stamms"), "ash-bark", 0.4, 0.45, 70000),
    barkRegion("limb-bark", tx("Bark of the leaders, limbs, branches and shoots", "Rinde der Leittriebe, Äste, Zweige und Triebe"), "ash-bark-young", 0.12, 0.55, 40000),
    leafRegion("leaf-sun", tx("Sun leaflets of the outer crown", "Sonnenblättchen der Außenkrone"), "ash-leaf-sun", 0.05, 0.75, 650, 0.0045),
    leafRegion("leaf-shade", tx("Shade leaflets of the light inner crown", "Schattenblättchen der lichten Innenkrone"), "ash-leaf-shade", 0.06, 0.4, 550, 0.003),
  ],
  leafRegionTitles: { sun: tx("", ""), shade: tx("", "") },
  emitterDiameter: { sun: 1.0, shade: 0.8 },
  tones: { trunk: 0.4, limb: 0.42, sun: 0.62, shade: 0.5 },
  porosity: 0.55, cdStem: 1.0,
  thermal: {
    environment: { airTempC: 24, meanRadiantTempC: 28, relativeHumidity: 0.5, windMS: 1.2, windDirectionDeg: 0, solarWM2: 700, pressurePa: 101325, diffuseFraction: 0.15 },
    assumptions: [...broadleafAssumptions("1.96 + 1.01", "0.0045", "0.003"), tx("The open ash crown lets a large share of the light through: the shade leaves receive more sun than in a beech or horse chestnut (sunlit fractions of 0.16 against 0.04 to 0.07), and the leaf area index is about half theirs.", "Die offene Eschenkrone lässt einen großen Teil des Lichts durch: Die Schattenblätter erhalten mehr Sonne als bei Buche oder Rosskastanie (besonnte Anteile von 0,16 gegenüber 0,04 bis 0,07), und der Blattflächenindex ist etwa halb so groß.")],
  },
  structureAssumptions: structureAssumptions("800"),
  poses, defaultPose: "summer-calm", breezePose: "summer-breeze", calmPose: "summer-calm",
  sway: { period: 4.2, amp: { trunk: 0.05, upper: 0.16, limb: 0.38, outer: 0.65 }, lag: 0.38, text: tx("A deterministic %P s sway cycle about the breeze lean (5.5 m/s at 10 m; estimated first mode of the 25 m forked stem with its light crown about 0.24 Hz). Every level lags the one below it; the upturned limb tips swing about two thirds of a degree.", "Ein deterministischer Schwingungszyklus von %P s um die Neigung bei Brise (5,5 m/s in 10 m Höhe; geschätzte erste Eigenform des 25 m hohen gegabelten Stamms mit seiner leichten Krone etwa 0,24 Hz). Jede Ebene hinkt der darunter nach; die aufgebogenen Astspitzen schwingen etwa zwei Drittel Grad.") },
  gust: { from: 5.5, peak: 15, text: tx("A single gust over 8 s from %A to %B m/s at 10 m: the wind-load tracks rise with the reconfigured drag law and the crown leans with a level-wise lag, outer limbs first, the stem last.", "Eine einzelne Böe über 8 s von %A auf %B m/s in 10 m Höhe: Die Windlast-Spuren steigen mit dem umgeformten Widerstandsgesetz, die Krone neigt sich ebenenweise verzögert, zuerst die äußeren Äste, zuletzt der Stamm.") },
  diurnal: {
    text: tx("A 24 h summer day compressed to 24 s (1 s = 1 h), calm: the sunlit share of every region and the transpiration of the sun and shade leaflets follow the sun, with stomata nearly closed at night and a mid-afternoon depression; leaf and bark temperatures are not authored but follow from the thermal solver.", "Ein 24-h-Sommertag auf 24 s gerafft (1 s = 1 h) bei Windstille: Der besonnte Anteil jeder Region und die Transpiration der Sonnen- und Schattenblättchen folgen der Sonne, nachts sind die Stomata fast geschlossen, am Nachmittag gibt es eine Mittagsdepression; Blatt- und Rindentemperaturen werden nicht vorgegeben, sondern folgen aus dem thermischen Löser."),
    sun: [[0, 0.03], [5, 0.05], [7, 0.36], [9, 0.86], [11.5, 1], [14, 0.74], [16, 0.78], [18, 0.32], [20, 0.07], [24, 0.03]],
    shade: [[0, 0.02], [5, 0.04], [7, 0.26], [9, 0.66], [12, 0.78], [14, 0.6], [16, 0.62], [18, 0.24], [20, 0.05], [24, 0.02]],
    sunlit: diurnalSunlit({ "trunk-bark": 0.4, "limb-bark": 0.3, "leaf-sun": 0.85, "leaf-shade": 0.16 }),
  },
  leafMark, barkMark,
  texts: {
    scale: {
      50: {
        silhouette: tx("At 1:50 (1 m is 20 mm) the ash is drawn as the true views of its 1:50 body: the tall pale stem with its fork into three leaders, the limbs that dip and turn up at their tips with their side branches and stout shoots, and the open crown of some one hundred and seventy loose leaf clusters with the sky showing between them.", "Bei 1:50 (1 m sind 20 mm) ist die Esche als wahre Ansichten ihres Körpers 1:50 gezeichnet: der hohe helle Stamm mit seiner Gabel in drei Leittriebe, die durchhängenden und an den Spitzen aufgebogenen Äste mit Seitenzweigen und kräftigen Trieben und die offene Krone aus rund hundertsiebzig lockeren Laubbüscheln, zwischen denen der Himmel durchscheint."),
        merged: tx("The wood of one bone is one outline (limb, side branches and the shoots into its clusters); the clusters of one carrying bone and leaf region are one outline with its sky openings, which stay open where they print.", "Das Holz eines Knochens ist ein Umriss (Ast, Seitenzweige und die Triebe in seine Büschel); die Büschel eines tragenden Knochens und einer Blattregion sind ein Umriss mit seinen Himmelslücken, die offen bleiben, wo sie drucken."),
        omitted: tx("Omitted: single leaflets, the black buds and the keys below the print minimum; leaf marks and bark fissures where nearer foliage covers them.", "Weggelassen: einzelne Blättchen, die schwarzen Knospen und die Nüsschen unter dem Druckminimum; Blattzeichen und Rindenfurchen, wo näheres Laub sie verdeckt."),
        thinElements: tx("Branches and shoots taper to 18 mm, still 0.24 mm at 1:75, the coarse end of this drawing; the pinnate leaf marks and the diamond fissures are 0.13 mm texture strokes.", "Zweige und Triebe verjüngen sich bis 18 mm, bei 1:75, dem groben Ende dieser Zeichnung, noch 0,24 mm; die gefiederten Blattzeichen und die Rautenfurchen sind 0,13-mm-Texturstriche."),
        styles: tx("Light line: closed outlines with pinnate leaf marks and the diamond network of the bark. Dark solid: light, broken crown masses with sky between, a lighter core, the pale stem light.", "Helle Linie: geschlossene Umrisse mit gefiederten Blattzeichen und dem Rautennetz der Rinde. Dunkle Fläche: lichte, durchbrochene Kronenmassen mit Himmel dazwischen, ein hellerer Kern, der helle Stamm hell."),
      },
      100: {
        silhouette: tx("At 1:100 (1 m is 10 mm) the true views of the 1:100 body: the forked stem, the leaders and limbs with their main side branches, and an open crown of larger clusters with fewer, wider sky openings.", "Bei 1:100 (1 m sind 10 mm) die wahren Ansichten des Körpers 1:100: der gegabelte Stamm, die Leittriebe und Äste mit ihren Hauptseitenzweigen und eine offene Krone aus größeren Büscheln mit weniger, weiteren Himmelslücken."),
        merged: tx("One outline per bone for the wood and per carrying bone and leaf region for the foliage; the outer side branches are merged into their limbs.", "Ein Umriss je Knochen für das Holz und je tragendem Knochen und Blattregion für das Laub; die äußeren Seitenzweige sind in ihre Äste aufgegangen."),
        omitted: tx("Omitted: the third side branch of every limb, the finest shoots and every mark that a nearer cluster covers.", "Weggelassen: der dritte Seitenzweig jedes Astes, die feinsten Triebe und jedes Zeichen, das ein näheres Büschel verdeckt."),
        thinElements: tx("Wood is never thinner than 36 mm (0.24 mm at 1:150, the coarse end of this drawing); marks are 0.1 mm texture strokes.", "Holz ist nie dünner als 36 mm (0,24 mm bei 1:150, dem groben Ende dieser Zeichnung); Zeichen sind 0,1-mm-Texturstriche."),
        styles: tx("Light line: outlines and a few leaf and bark marks. Dark solid: open crown masses, lighter core, light stem.", "Helle Linie: Umrisse und wenige Blatt- und Rindenzeichen. Dunkle Fläche: offene Kronenmassen, hellerer Kern, heller Stamm."),
      },
      200: {
        silhouette: tx("At 1:200 (1 m is 5 mm) the crown is composed of about sixty clusters over a light core, still showing the largest sky openings; the forked stem, leaders, limbs and the first side branch of every limb are drawn at a printable width.", "Bei 1:200 (1 m sind 5 mm) ist die Krone aus etwa sechzig Büscheln über einem lichten Kern komponiert und zeigt noch die größten Himmelslücken; der gegabelte Stamm, Leittriebe, Äste und der erste Seitenzweig jedes Astes sind in druckbarer Breite gezeichnet."),
        merged: tx("The wood of one bone is one outline; the clusters of one carrying bone and region are one outline; small sky openings are closed.", "Das Holz eines Knochens ist ein Umriss; die Büschel eines tragenden Knochens und einer Region sind ein Umriss; kleine Himmelslücken sind geschlossen."),
        omitted: tx("Omitted: shoots, the outer side branches, leaf and bark texture.", "Weggelassen: Triebe, die äußeren Seitenzweige, Blatt- und Rindentextur."),
        thinElements: tx("Limbs are drawn at least 80 mm wide, 0.23 mm at 1:350, the coarse end of this drawing.", "Äste sind mindestens 80 mm breit gezeichnet, 0,23 mm bei 1:350, dem groben Ende dieser Zeichnung."),
        styles: tx("Light line: closed outlines only. Dark solid: open crown masses with a lighter core and a light stem.", "Helle Linie: nur geschlossene Umrisse. Dunkle Fläche: offene Kronenmassen mit hellerem Kern und hellem Stamm."),
      },
      500: {
        silhouette: tx("At 1:500 (1 m is 2 mm) the ash is a tall, loosely lobed ovoid crown of some thirty-five masses on its tall clear stem; the deep notches of its margin keep the open character of the crown where the openings themselves no longer print.", "Bei 1:500 (1 m sind 2 mm) ist die Esche eine hohe, locker gelappte eiförmige Krone aus etwa fünfunddreißig Massen auf ihrem hohen, astfreien Stamm; die tiefen Einschnitte ihres Randes bewahren den offenen Charakter der Krone, wo die Lücken selbst nicht mehr drucken."),
        merged: tx("Foliage of one carrying bone and region is one outline without sky openings; the wood of one bone is one outline.", "Das Laub eines tragenden Knochens und einer Region ist ein Umriss ohne Himmelslücken; das Holz eines Knochens ist ein Umriss."),
        omitted: tx("Omitted: every side branch and shoot, all texture; inner limbs show only where the crown leaves them visible.", "Weggelassen: alle Seitenzweige und Triebe, jede Textur; innere Äste zeigen sich nur, wo die Krone sie sichtbar lässt."),
        thinElements: tx("Wood is drawn at least 200 mm wide, the smallest width that still prints at 1:1000, the coarse end of this drawing.", "Holz ist mindestens 200 mm breit gezeichnet, die kleinste Breite, die bei 1:1000, dem groben Ende dieser Zeichnung, noch druckt."),
        styles: tx("Light line: closed outlines only. Dark solid: the crown as a light, lobed mass, the stem lighter.", "Helle Linie: nur geschlossene Umrisse. Dunkle Fläche: die Krone als lichte, gelappte Masse, der Stamm heller."),
      },
    },
    volume: {
      50: tx("Body at 1:50 (smallest feature 7.5 mm): the collar and the tall stem as one tapered loft per stem bone up to the fork, three leaders, six lower limbs and nine limbs on the leaders, all dipping and turning up at their tips, forty-five side branches and a shoot into every leaf cluster, lofted through hand-stated stations that start inside their parent so that sway never opens a joint; the open crown of loose clusters (1.25 by 0.85 m) in thirteen rings with gaps between them, and three rings of light core clusters round the leaders.", "Körper bei 1:50 (kleinstes Merkmal 7,5 mm): der Wurzelhals und der hohe Stamm als je ein verjüngter Loft je Stammknochen bis zur Gabel, drei Leittriebe, sechs untere Äste und neun Äste an den Leittrieben, alle durchhängend und an den Spitzen aufgebogen, fünfundvierzig Seitenzweige und ein Trieb in jedes Laubbüschel, durch von Hand gesetzte Stationen gelofted, die im Mutterteil beginnen, sodass kein Gelenk beim Schwingen aufgeht; die offene Krone aus lockeren Büscheln (1,25 auf 0,85 m) in dreizehn Ringen mit Lücken dazwischen und drei Ringe lichter Kernbüschel um die Leittriebe."),
      100: tx("Body at 1:100 (smallest feature 15 mm): the same stem, leaders and fifteen limbs with thirty side branches at fewer stations, wood never thinner than 36 mm, ten rings of larger clusters and two core rings.", "Körper bei 1:100 (kleinstes Merkmal 15 mm): derselbe Stamm, Leittriebe und fünfzehn Äste mit dreißig Seitenzweigen an weniger Stationen, Holz nie dünner als 36 mm, zehn Ringe größerer Büschel und zwei Kernringe."),
      200: tx("Body at 1:200 (smallest feature 30 mm): stem, leaders and limbs at five stations, wood at least 80 mm, fifteen side branches, seven rings of large clusters and two core rings.", "Körper bei 1:200 (kleinstes Merkmal 30 mm): Stamm, Leittriebe und Äste an fünf Stationen, Holz mindestens 80 mm, fünfzehn Seitenzweige, sieben Ringe großer Büschel und zwei Kernringe."),
      500: tx("Body at 1:500 (smallest feature 75 mm): stem, leaders and limbs as three-station lofts at least 200 mm thick, no side branches, five rings of crown masses and one core ring: the loosely lobed ovoid of the tree.", "Körper bei 1:500 (kleinstes Merkmal 75 mm): Stamm, Leittriebe und Äste als Lofts mit drei Stationen, mindestens 200 mm dick, keine Seitenzweige, fünf Ringe von Kronenmassen und ein Kernring: das locker gelappte Ei des Baums."),
    },
    symbol: tx("A conventional plan sign for site and landscape plans: the true plan outline of the open crown, the stem section at breast height and the directions of the three leaders of the fork. It is a separate symbol and is never used silently for a pictorial scale.", "Ein konventionelles Grundrisszeichen für Lage- und Freiraumpläne: der wahre Grundrissumriss der offenen Krone, der Stammquerschnitt in Brusthöhe und die Richtungen der drei Leittriebe der Gabel. Es ist ein eigenes Symbol und wird nie stillschweigend für einen Bildmaßstab verwendet."),
  },
  symbolLimbs: ["lead-a", "lead-b", "lead-c"],
  defaultScale: 200,
};
