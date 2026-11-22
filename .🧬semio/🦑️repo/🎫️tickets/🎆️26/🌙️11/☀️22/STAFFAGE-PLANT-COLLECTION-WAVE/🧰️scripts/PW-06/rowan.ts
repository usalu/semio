/** 🍒️ Rowan, mountain ash (Sorbus aucuparia): an open-grown tree of about 40 years, 10 m high, with a clear silver-grey stem of 2.3 m, a continuous leader, nine steeply ascending limbs in an ovoid crown of pinnate leaves and orange-red berry corymbs hanging at the crown surface. Every station, limb, lobe ring, berry cluster and outlet is stated here per scale in world metres (x ahead, y up, z to the left).
 * Usage: `repo tools exec bun -- rowan.ts [--catalog]` (without `--catalog` the candidate goes to `🗑️generated/PW-06/rowan/🔣️.json`).
 * @see ./plant-kit.ts
 * @see https://doi.org/10.1046/j.1365-2745.2000.00507.x — Raspé et al. 2000, Biological Flora of the British Isles: Sorbus aucuparia */
import { add, along, axisAt, barkMarks, binding, branch, build, clamp, clusterOn, diurnalClip, freeLobe, GUST, gustKeys, offsets, r4, ringLobes, stemTube, swayKeys, tubeArea, tx, verticalLoads, windLoads, write, type BoneDef, type Ctx, type Json, type Lobe, type RingDef, type Scale, type ScaleModel, type Species, type Tube, type V3, type View } from "./plant-kit.ts";

const BARK = "rowan-bark", YOUNG = "rowan-bark-young", SUN = "rowan-leaf-sun", SHADE = "rowan-leaf-shade", BERRY = "rowan-berry";
const TONE = { root: 0.3, stem: 0.62, limb: 0.55 };

type StemRows = [number, number, number, number][];
const STEM_50: Record<string, StemRows> = {
  "root-plate": [[0, 0.27, 0, 0], [0.08, 0.21, 0, 0], [0.2, 0.172, 0, 0], [0.32, 0.153, 0, 0], [0.45, 0.146, 0, 0]],
  trunk: [[0.35, 0.148, 0, 0], [0.8, 0.14, 0.012, 0.005], [1.3, 0.136, 0.022, 0.01], [1.8, 0.132, 0.018, 0.012], [2.2, 0.128, 0.006, 0.008], [2.55, 0.124, -0.006, 0.004]],
  leader: [[2.45, 0.12, -0.004, 0.005], [3.0, 0.113, -0.012, 0], [3.8, 0.104, -0.02, -0.008], [4.6, 0.095, -0.012, -0.012], [5.4, 0.085, 0.004, -0.008], [6.05, 0.077, 0.014, 0]],
  "leader-top": [[5.95, 0.078, 0.012, 0], [6.8, 0.066, 0.02, 0.006], [7.6, 0.054, 0.012, 0.01], [8.4, 0.043, -0.004, 0.006], [9.1, 0.032, -0.012, 0]],
};
const STEM_100: Record<string, StemRows> = {
  "root-plate": [[0, 0.26, 0, 0], [0.14, 0.18, 0, 0], [0.45, 0.146, 0, 0]],
  trunk: [[0.35, 0.148, 0, 0], [1.3, 0.136, 0.02, 0.01], [2.55, 0.124, -0.006, 0.004]],
  leader: [[2.45, 0.12, -0.004, 0.005], [3.8, 0.104, -0.02, -0.008], [5.0, 0.09, -0.004, -0.01], [6.05, 0.077, 0.014, 0]],
  "leader-top": [[5.95, 0.078, 0.012, 0], [7.6, 0.054, 0.012, 0.01], [9.1, 0.032, -0.012, 0]],
};
const STEM_200: Record<string, StemRows> = {
  "root-plate": [[0, 0.25, 0, 0], [0.45, 0.146, 0, 0]],
  trunk: [[0.35, 0.148, 0, 0], [2.55, 0.124, 0, 0]],
  leader: [[2.45, 0.12, 0, 0], [4.2, 0.1, -0.015, -0.01], [6.05, 0.077, 0.014, 0]],
  "leader-top": [[5.95, 0.078, 0.012, 0], [9.1, 0.034, -0.012, 0]],
};
const STEM_500: Record<string, StemRows> = {
  "root-plate": [[0, 0.24, 0, 0], [0.45, 0.15, 0, 0]],
  trunk: [[0.35, 0.15, 0, 0], [2.55, 0.125, 0, 0]],
  leader: [[2.45, 0.12, 0, 0], [6.05, 0.08, 0, 0]],
  "leader-top": [[5.95, 0.08, 0, 0], [9.1, 0.06, 0, 0]],
};

type LimbRow = [id: string, parent: string, y: number, az: number, e0: number, e1: number, L: number, r0: number, r1: number, bend: number];
const LIMBS: LimbRow[] = [
  ["limb-1", "trunk", 2.35, 18, 52, 34, 3.3, 0.07, 0.022, 8],
  ["limb-2", "trunk", 2.5, 162, 50, 32, 3.1, 0.066, 0.021, -10],
  ["limb-3", "leader", 3.25, 262, 54, 36, 3.0, 0.06, 0.02, 6],
  ["limb-4", "leader", 3.85, 78, 52, 36, 2.9, 0.057, 0.02, -8],
  ["limb-5", "leader", 4.6, 208, 56, 42, 2.7, 0.05, 0.018, 10],
  ["limb-6", "leader", 5.3, 325, 58, 44, 2.5, 0.046, 0.017, -6],
  ["limb-7", "leader-top", 6.5, 125, 62, 48, 2.1, 0.038, 0.015, 8],
  ["limb-8", "leader-top", 7.2, 28, 64, 50, 1.8, 0.033, 0.013, -8],
  ["limb-9", "leader-top", 7.8, 240, 66, 54, 1.5, 0.028, 0.012, 6],
];
/** 🌿️ Side branches of the limbs: [limb, fraction along, azimuth turn, elevation, length, r0, r1]. */
type SideRow = [limb: string, f: number, dAz: number, e: number, L: number, r0: number, r1: number];
const SIDES_50: SideRow[] = [
  ["limb-1", 0.42, 38, 30, 1.3, 0.026, 0.01], ["limb-1", 0.7, -34, 26, 1.0, 0.02, 0.009],
  ["limb-2", 0.45, -36, 28, 1.2, 0.025, 0.01], ["limb-2", 0.72, 32, 24, 0.95, 0.019, 0.008],
  ["limb-3", 0.45, 40, 32, 1.1, 0.023, 0.009], ["limb-3", 0.72, -36, 28, 0.9, 0.018, 0.008],
  ["limb-4", 0.45, -38, 30, 1.1, 0.022, 0.009], ["limb-4", 0.72, 34, 28, 0.85, 0.017, 0.008],
  ["limb-5", 0.5, 36, 36, 0.95, 0.02, 0.008], ["limb-6", 0.5, -34, 38, 0.9, 0.018, 0.008],
  ["limb-7", 0.55, 36, 44, 0.75, 0.016, 0.007], ["limb-8", 0.55, -36, 46, 0.65, 0.014, 0.007],
];
const SIDES_100: SideRow[] = [
  ["limb-1", 0.5, 36, 28, 1.2, 0.026, 0.012], ["limb-2", 0.5, -34, 26, 1.1, 0.025, 0.012],
  ["limb-3", 0.5, 38, 30, 1.0, 0.023, 0.012], ["limb-4", 0.5, -36, 30, 1.0, 0.022, 0.012],
  ["limb-5", 0.55, 34, 36, 0.85, 0.02, 0.012], ["limb-6", 0.55, -32, 38, 0.8, 0.018, 0.012],
];

const MAT_OF_GROUP: Record<string, string> = { root: BARK, stem: BARK, leader: BARK, limb: YOUNG, branch: YOUNG };
function woody(scale: Scale): Tube[] {
  const rows = scale === 50 ? STEM_50 : scale === 100 ? STEM_100 : scale === 200 ? STEM_200 : STEM_500;
  const stems: Tube[] = Object.entries(rows).map(([id, r]) => stemTube(id, id, BARK, id === "root-plate" ? TONE.root : TONE.stem, id === "root-plate" ? "root" : id === "trunk" ? "stem" : "leader", r));
  const byId = (id: string): Tube => stems.find((t) => t.id === id)!;
  const n = scale === 50 ? 7 : scale === 100 ? 5 : scale === 200 ? 3 : 2;
  const limbs = LIMBS.map(([id, parent, y, az, e0, e1, L, r0, r1, bend]) => branch(id, id, YOUNG, TONE.limb, "limb", axisAt(byId(parent), y), az, e0, e1, L, r0, r1, scale === 500 ? 0 : bend, n));
  const sides = (scale === 50 ? SIDES_50 : scale === 100 ? SIDES_100 : []).map(([limb, f, dAz, e, L, r0, r1], i) => {
    const row = LIMBS.find((x) => x[0] === limb)!;
    const t = limbs.find((x) => x.id === limb)!;
    return branch(`${limb}-side-${i + 1}`, limb, YOUNG, TONE.limb, "branch", along(t, f), row[3] + dAz, e, e - 12, L, r0, r1, 0, scale === 50 ? 4 : 3);
  });
  const roots = scale <= 100 ? [30, 120, 210, 300].map((az, i) => branch(`root-${i + 1}`, "root-plate", BARK, TONE.root, "root", [0, 0.13, 0], az, -8, -8, 0.36, 0.085, 0.04, 0, scale === 50 ? 3 : 2)) : [];
  return [...stems, ...roots, ...limbs, ...sides];
}

/** 🥚️ The ovoid crown envelope: widest 3.1 m at 5.4 m, top 10 m, base 2.7 m; 0.94 as deep across as ahead. */
const env = (az: number, y: number): number => {
  const t = y - 5.4;
  const R = t >= 0 ? 3.1 * Math.sqrt(Math.max(0, 1 - (t / 4.6) ** 2)) : 3.1 * Math.max(0, 1 - Math.abs(t / 2.7) ** 2.4) ** (1 / 2.4);
  return R / Math.hypot(Math.cos((az * Math.PI) / 180), Math.sin((az * Math.PI) / 180) / 0.94);
};

const tipsOf = (wood: Tube[]): { id: string; az: number; y: number; p: V3 }[] => LIMBS.map(([id, , , az]) => {
  const t = wood.find((x) => x.id === id)!;
  const p = t.points[t.points.length - 1]!.p;
  return { id: `fol-${id.slice(5)}`, az, y: p[1], p };
});
const WOOD_50 = woody(50);
const TIPS = tipsOf(WOOD_50);
const TOP_TIP = WOOD_50.find((t) => t.id === "leader-top")!.points.at(-1)!.p;
const angle = (a: number, b: number): number => Math.abs(((((a - b) % 360) + 540) % 360) - 180);
const boneAt = (az: number, y: number): string => (y >= 9.0 ? "fol-top" : TIPS.reduce((best, t) => ((angle(az, t.az) / 50) ** 2 + ((y - t.y) / 1.6) ** 2 < (angle(az, best.az) / 50) ** 2 + ((y - best.y) / 1.6) ** 2 ? t : best)).id);
const matOf = (bone: string): string => (bone === "fol-core" ? SHADE : SUN);

/** 🍃️ The main rings shared by every scale (height, radial and vertical semi-axes, droop), so the silhouette lobes at the four cardinal azimuths agree between the scales; each scale states how many lobes a ring has and how wide (tangentially) they are. */
const MAIN: [id: string, y: number, a: number, b: number, tilt: number][] = [["s1", 3.35, 0.7, 0.5, 18], ["s2", 4.3, 0.78, 0.65, 8], ["s3", 5.35, 0.82, 0.72, 4], ["s4", 6.45, 0.8, 0.7, 0], ["s5", 7.5, 0.74, 0.66, -4], ["s6", 8.5, 0.68, 0.6, -8], ["s7", 9.3, 0.6, 0.55, -10]];
const main = (n: number[], c: number[], grow = 1): RingDef[] => MAIN.map(([id, y, a, b, tilt], i) => ({ id, y, n: n[i]!, az0: 0, a, b: b * grow, c: c[i]!, tilt }));
const RINGS: Record<Scale, RingDef[]> = {
  50: [
    ...main([12, 12, 12, 12, 12, 8, 4], [0.75, 0.85, 0.9, 0.88, 0.8, 0.75, 0.7]),
    { id: "b1", y: 3.85, n: 8, az0: 22.5, a: 0.6, b: 0.5, c: 0.7, k: 0.93 },
    { id: "b2", y: 4.85, n: 9, az0: 20, a: 0.62, b: 0.52, c: 0.72, k: 0.93 },
    { id: "b3", y: 5.9, n: 9, az0: 10, a: 0.62, b: 0.52, c: 0.72, k: 0.93 },
    { id: "b4", y: 7.0, n: 8, az0: 22.5, a: 0.6, b: 0.5, c: 0.7, k: 0.93 },
    { id: "b5", y: 8.0, n: 6, az0: 30, a: 0.56, b: 0.48, c: 0.66, k: 0.93 },
  ],
  100: [
    ...main([8, 8, 12, 8, 8, 4, 4], [1.0, 1.2, 0.95, 1.2, 1.1, 1.0, 0.78], 1.05),
    { id: "b2", y: 4.85, n: 6, az0: 30, a: 0.7, b: 0.58, c: 0.85, k: 0.92 },
    { id: "b4", y: 7.0, n: 6, az0: 30, a: 0.66, b: 0.55, c: 0.8, k: 0.92 },
  ],
  200: [
    ...main([8, 8, 8, 8, 8, 4, 4], [1.05, 1.35, 1.42, 1.38, 1.2, 1.05, 0.8], 1.12).filter((r) => r.id !== "s7"),
    { id: "s67", y: 9.0, n: 4, az0: 45, a: 0.66, b: 0.7, c: 0.95, tilt: -8 },
  ],
  500: [
    { id: "s1", y: 3.55, n: 6, az0: 0, a: 0.8, b: 0.88, c: 1.55, tilt: 12 },
    { id: "s2", y: 5.2, n: 8, az0: 0, a: 0.9, b: 1.05, c: 1.65, tilt: 4 },
    { id: "s3", y: 7.0, n: 8, az0: 0, a: 0.85, b: 0.95, c: 1.45 },
    { id: "s4", y: 8.6, n: 4, az0: 0, a: 0.75, b: 0.8, c: 1.3, tilt: -8 },
  ],
};
const TOP: Record<Scale, [V3, V3]> = { 50: [[0, 9.5, 0], [0.62, 0.5, 0.6]], 100: [[0, 9.45, 0], [0.7, 0.55, 0.68]], 200: [[0, 9.35, 0], [0.85, 0.65, 0.8]], 500: [[0, 9.2, 0], [1.0, 0.8, 1.0]] };
const CORE: Record<Scale, [V3, V3][]> = {
  50: [[[0.4, 4.6, 0.3], [1.2, 0.9, 1.1]], [[-0.5, 5.6, -0.3], [1.1, 0.9, 1.1]], [[0.2, 6.7, -0.4], [1.0, 0.85, 1.0]], [[-0.2, 7.7, 0.35], [0.85, 0.8, 0.85]]],
  100: [[[0.3, 4.8, 0.2], [1.3, 1.0, 1.2]], [[-0.3, 6.2, -0.3], [1.2, 0.95, 1.15]], [[0, 7.5, 0.2], [0.95, 0.85, 0.95]]],
  200: [[[0.2, 5.0, 0.1], [1.4, 1.1, 1.3]], [[-0.2, 6.9, -0.1], [1.2, 1.0, 1.15]]],
  500: [[[0, 6.0, 0], [1.6, 1.6, 1.5]]],
};

function crown(scale: Scale): Lobe[] {
  const at500 = scale === 500;
  const sun = ringLobes(RINGS[scale], env, at500 ? () => "leader" : boneAt, matOf);
  const top = freeLobe("top", at500 ? "leader" : "fol-top", SUN, "sun", TOP[scale][0], TOP[scale][1]);
  const core = CORE[scale].map(([c, r], i) => freeLobe(`core-${i + 1}`, at500 ? "leader" : "fol-core", SHADE, "shade", c, r));
  return [...sun, top, ...core];
}

/** 🍒️ Berry corymbs: [ring, every k-th lobe from j, azimuth turn, elevation] per scale; a corymb of 50 to 150 berries is 12 to 15 cm across. */
const CLUSTERS: Record<Scale, { rows: [ring: string, step: number, from: number, dAz: number, el: number][]; radii: V3 }> = {
  50: { rows: [["s1", 2, 0, 12, -30], ["s2", 2, 1, -10, -25], ["b2", 3, 0, 8, -20], ["s3", 3, 1, 14, -28], ["s3", 3, 2, -16, 6], ["s4", 2, 0, -12, -24], ["s4", 3, 1, 14, 8], ["s5", 2, 1, 10, -18]], radii: [0.07, 0.045, 0.07] },
  100: { rows: [["s1", 2, 0, 12, -28], ["s2", 2, 1, -10, -24], ["s3", 3, 1, 14, -26], ["s3", 3, 2, -16, 6], ["s4", 2, 0, -12, -22], ["s5", 3, 1, 10, -16]], radii: [0.075, 0.05, 0.075] },
  200: { rows: [["s1", 2, 0, 12, -26], ["s2", 2, 1, -12, -22], ["s3", 2, 0, 14, -20]], radii: [0.085, 0.056, 0.085] },
  500: { rows: [], radii: [0.1, 0.07, 0.1] },
};
function clusters(scale: Scale, lobes: Lobe[]) {
  const out = [];
  let n = 0;
  for (const [ring, step, from, dAz, el] of CLUSTERS[scale].rows) {
    const mine = lobes.filter((l) => l.id.startsWith(`${ring}-`));
    for (let i = from; i < mine.length; i += step) {
      const l = mine[i]!;
      const az = (Math.atan2(l.c[2], l.c[0]) * 180) / Math.PI;
      out.push(clusterOn(`berries-${++n}`, l, az + dAz, el, CLUSTERS[scale].radii, BERRY));
    }
  }
  return out;
}

const OUTLETS: Record<Scale, ScaleModel["outlets"]> = {
  50: [["transpiration-sun", "tp-sun-1", "s3-1", 0, 30], ["transpiration-sun", "tp-sun-2", "s3-4", 90, 30], ["transpiration-sun", "tp-sun-3", "s3-7", 180, 30], ["transpiration-sun", "tp-sun-4", "s3-10", 270, 30], ["transpiration-sun", "tp-sun-5", "s5-2", 60, 50], ["transpiration-sun", "tp-sun-6", "s5-6", 220, 50], ["transpiration-sun", "tp-sun-7", "top", 0, 90],
    ["transpiration-shade", "tp-shade-1", "core-1", 0, 90], ["transpiration-shade", "tp-shade-2", "core-2", 0, 90], ["transpiration-shade", "tp-shade-3", "core-3", 0, 90], ["transpiration-shade", "tp-shade-4", "core-4", 0, 90], ["transpiration-shade", "tp-shade-5", "core-1", 180, 20]],
  100: [["transpiration-sun", null, "s3-1", 0, 30], ["transpiration-sun", null, "s3-7", 180, 30], ["transpiration-sun", null, "s5-3", 100, 50], ["transpiration-sun", null, "top", 0, 90], ["transpiration-shade", null, "core-1", 0, 90], ["transpiration-shade", null, "core-2", 0, 90]],
  200: [["transpiration-sun", null, "s2-1", 0, 30], ["transpiration-sun", null, "top", 0, 90], ["transpiration-shade", null, "core-1", 0, 90]],
  500: [["transpiration-sun", null, "top", 0, 90], ["transpiration-shade", null, "core-1", 0, 90]],
};

const DECISIONS: Record<Scale, Json[]> = {
  50: [
    tx("Body at 1:50: root flare with four surface roots, a clear silver-grey stem of 2.3 m with a slight sway, the continuous leader in two parts, nine steeply ascending limbs (swept lofts, 52 to 66 degrees at the stem, opening to 34 to 54 degrees) with twelve side branches, and an ovoid crown of pinnate-leaf lobes: seven rings from the skirt at 3.35 m to 9.3 m with five rings of smaller lobes between them, a top lobe at 10 m and four shade lobes around the leader (thermal solids only); thirty-nine berry corymbs of 14 cm hang on the outer faces of the lower and middle crown.", "Körper bei 1:50: Wurzelanlauf mit vier Oberflächenwurzeln, ein astfreier silbergrauer Stamm von 2,3 m mit leichtem Schwung, der durchgehende Leittrieb in zwei Teilen, neun steil aufsteigende Äste (geführte Lofts, 52 bis 66 Grad am Stamm, sich auf 34 bis 54 Grad öffnend) mit zwölf Seitenzweigen und eine eiförmige Krone aus Lappen gefiederter Blätter: sieben Ringe vom Saum auf 3,35 m bis 9,3 m mit fünf Ringen kleinerer Lappen dazwischen, ein Gipfellappen auf 10 m und vier Schattenlappen um den Leittrieb (nur als Thermikkörper); neununddreißig Beerendolden von 14 cm hängen an den Außenseiten der unteren und mittleren Krone."),
    tx("Thermal at 1:50: stem bark is the root flare, roots, stem and leader; branch bark the limbs and side branches; sun leaves the outer lobes, shade leaves the four core lobes, the berries the fruit region, the visible regions by reference to the body; the shade lobes (thermal solids only) are thermal solids of their own (a hidden region). The transpiration outlets are lens patches in the free faces of the lobes that carry the emitter anchors (on the tops of the shade lobes (thermal solids only)).", "Thermik bei 1:50: Stammrinde sind Wurzelanlauf, Wurzeln, Stamm und Leittrieb; Astrinde die Äste und Seitenzweige; Sonnenblätter die äußeren Lappen, Schattenblätter die vier Kernlappen, die Beeren die Fruchtregion, die sichtbaren Regionen als Verweis auf den Körper; die Schattenlappen (nur als Thermikkörper) sind eigene Thermikkörper (eine verdeckte Region). Die Transpirationsauslässe sind Linsenflächen in den freien Seiten der Lappen, die die Emitteranker tragen (bei den Schattenlappen (nur als Thermikkörper) oben)."),
    tx("Structure at 1:50: the wood under the bark of every load-bearing bone along its real axis (root flare, stem, leader, upper leader, nine limbs), joints at the rig joints, the round fixed root plate at the ground and the masses of the wood bones and of the ten foliage masses at their centres of mass; no leaves, no bark.", "Tragwerk bei 1:50: das Holz unter der Rinde jedes tragenden Knochens entlang seiner wirklichen Achse (Wurzelanlauf, Stamm, Leittrieb, oberer Leittrieb, neun Äste), Gelenke an den Rig-Gelenken, der runde eingespannte Wurzelteller am Boden und die Massen der Holzknochen und der zehn Laubmassen in ihren Schwerpunkten; kein Laub, keine Rinde."),
  ],
  100: [
    tx("Body at 1:100, authored anew: one flared root loft without surface roots, stem and leader on three or four stations, the nine limbs on five sections with six side branches, and the crown redrawn with fewer, broader lobes (the same seven main rings with fewer, wider lobes, two rings between, the top lobe and three shade lobes (thermal solids only)) with twenty-three berry corymbs.", "Körper bei 1:100, neu modelliert: ein ausgestellter Wurzelloft ohne Oberflächenwurzeln, Stamm und Leittrieb auf drei oder vier Stationen, die neun Äste auf fünf Schnitten mit sechs Seitenzweigen und die Krone mit weniger, breiteren Lappen neu gezeichnet (dieselben sieben Hauptringe mit weniger, breiteren Lappen, zwei Zwischenringe, der Gipfellappen und drei Schattenlappen (nur als Thermikkörper)) mit dreiundzwanzig Beerendolden."),
    tx("Thermal at 1:100: the same five regions by reference to this scale's body; six transpiration outlets as lens patches in the free faces of the lobes.", "Thermik bei 1:100: dieselben fünf Regionen als Verweis auf den Körper dieses Maßstabs; sechs Transpirationsauslässe als Linsenflächen in den freien Seiten der Lappen."),
    tx("Structure at 1:100: the wood of the thirteen load-bearing bones with simplified taper, joints at the rig joints, the round root plate and the masses.", "Tragwerk bei 1:100: das Holz der dreizehn tragenden Knochen mit vereinfachter Verjüngung, Gelenke an den Rig-Gelenken, der runde Wurzelteller und die Massen."),
  ],
  200: [
    tx("Body at 1:200, authored anew: root flare, stem and leader as plain tapering lofts, the nine limbs on three sections without side branches (their tips raised to the printable 46 mm), and the crown as seven rings of large masses (the two upper rings merged into one), a top mass and two shade cores (thermal only), with twelve berry corymbs on the lower rings.", "Körper bei 1:200, neu modelliert: Wurzelanlauf, Stamm und Leittrieb als einfache verjüngte Lofts, die neun Äste auf drei Schnitten ohne Seitenzweige (ihre Spitzen auf druckbare 46 mm angehoben) und die Krone als sieben Ringe großer Massen (die zwei oberen Ringe zu einem zusammengefasst), eine Gipfelmasse und zwei Schattenkerne (nur Thermik), mit zwölf Beerendolden an den unteren Ringen."),
    tx("Thermal at 1:200: bark, leaves and berries by reference to the body; three outlets.", "Thermik bei 1:200: Rinde, Laub und Beeren als Verweis auf den Körper; drei Auslässe."),
    tx("Structure at 1:200: the wood of the thirteen load-bearing bones, joints where they are at least one smallest feature thick, the round root plate and the masses.", "Tragwerk bei 1:200: das Holz der dreizehn tragenden Knochen, Gelenke dort, wo sie mindestens ein kleinstes Merkmal dick sind, der runde Wurzelteller und die Massen."),
  ],
  500: [
    tx("Body at 1:500: root flare, stem and leader as three plain lofts and the crown as twenty-eight masses bound to the leader as the drawing binds it (a low ring of six, two rings of eight, four upper masses, the top and the shade core) (thermal only); the limbs, the upper leader and the berries lie inside the crown and are not part of the body.", "Körper bei 1:500: Wurzelanlauf, Stamm und Leittrieb als drei einfache Lofts und die Krone als achtundzwanzig Massen, an den Leittrieb gebunden wie in der Zeichnung (ein tiefer Ring aus sechs, zwei Ringe aus acht, vier obere Massen, der Gipfel und der Schattenkern) (nur Thermik); Äste, oberer Leittrieb und Beeren liegen in der Krone und gehören nicht zum Körper."),
    tx("Thermal at 1:500: stem bark and leaves by reference; the upper leader, the nine limbs and four berry corymbs are thermal solids of their own inside the crown; two outlets.", "Thermik bei 1:500: Stammrinde und Laub als Verweis; oberer Leittrieb, die neun Äste und vier Beerendolden sind eigene Thermikkörper in der Krone; zwei Auslässe."),
    tx("Structure at 1:500: the thirteen load-bearing bones as members of their declared section, the joints of the stem, the round root plate and the masses of the wood bones; this list is already minimal.", "Tragwerk bei 1:500: die dreizehn tragenden Knochen als Stäbe ihres angegebenen Querschnitts, die Gelenke des Stamms, der runde Wurzelteller und die Massen der Holzknochen; diese Liste ist bereits minimal."),
  ],
};

const models = {} as Record<Scale, ScaleModel>;
for (const scale of [50, 100, 200, 500] as Scale[]) {
  const all = clamp(scale, woody(scale));
  const hiddenIds = scale === 500 ? new Set(["leader-top", ...LIMBS.map((l) => l[0])]) : new Set<string>();
  const lobes = crown(scale);
  const cl = clusters(scale, lobes);
  const hiddenClusters = scale === 500 ? [[0, 1.2, 1.4], [120, -1.3, 1.1], [240, 0.2, -1.5], [60, 1.0, -1.0]].map(([, x, z], i) => ({ ...freeLobe(`berries-in-${i + 1}`, "leader", BERRY, "fruit", [x!, 4.6 + i * 0.4, z!] as V3, CLUSTERS[500].radii), clusterOf: "" })) : [];
  models[scale] = { scale, tubes: all.filter((t) => !hiddenIds.has(t.bone)), hiddenTubes: all.filter((t) => hiddenIds.has(t.bone)), lobes, clusters: cl, hiddenClusters, outlets: OUTLETS[scale], crownBone: scale === 500 ? "leader" : undefined, decisions: DECISIONS[scale] };
}

const tubeOf = (id: string): Tube => WOOD_50.find((t) => t.id === id)!;
const end = (id: string): V3 => tubeOf(id).points.at(-1)!.p;
const startOf = (id: string, y: number): V3 => axisAt(tubeOf(id), y);
const L = (id: string, title: [string, string], group: BoneDef["group"], parent: string | null, from: V3, to: V3, limit: number, material?: string): BoneDef => ({ id, parent, title: tx(...title), group, from, to, limit, ...(material ? { material } : {}) });
const BONES: BoneDef[] = [
  L("root-plate", ["Root plate and flare", "Wurzelteller und Wurzelanlauf"], "root", null, [0, 0, 0], [0, 0.4, 0], 0),
  L("trunk", ["Clear stem", "Astfreier Stamm"], "stem", "root-plate", [0, 0.4, 0], startOf("trunk", 2.5), 3),
  L("leader", ["Leader", "Leittrieb"], "leader", "trunk", startOf("trunk", 2.5), startOf("leader", 6.0), 5),
  L("leader-top", ["Upper leader", "Oberer Leittrieb"], "leader", "leader", startOf("leader", 6.0), end("leader-top"), 8),
  ...LIMBS.map(([id, parent, y]) => L(id, [`Ascending limb ${id.slice(5)}`, `Aufsteigender Ast ${id.slice(5)}`], "limb", parent, startOf(parent, y), end(id), 12)),
  ...TIPS.map((t) => L(t.id, [`Foliage of limb ${t.id.slice(4)}`, `Laub von Ast ${t.id.slice(4)}`], "foliage", `limb-${t.id.slice(4)}`, t.p, t.p, 30)),
  L("fol-top", ["Foliage of the crown top", "Laub des Kronengipfels"], "foliage", "leader-top", TOP_TIP, TOP_TIP, 30),
  L("fol-core", ["Shaded inner foliage", "Beschattetes Innenlaub"], "foliage", "leader", startOf("leader", 4.9), startOf("leader", 4.9), 30),
];

const REFS = [
  { id: "raspe-2000", citation: "Raspé, O., Findlay, C. and Jacquemart, A.-L. (2000). Sorbus aucuparia L. Biological Flora of the British Isles 214. Journal of Ecology 88(5), 910-930. Habit, size (to 15 m, usually 5-10 m), pinnate leaves of 11-19 leaflets, corymbs and berries of 6-9 mm." },
  { id: "raty-2016", citation: "Räty, M., Caudullo, G. and de Rigo, D. (2016). Sorbus aucuparia in Europe: distribution, habitat, usage and threats. In San-Miguel-Ayanz, J. et al. (eds.), European Atlas of Forest Tree Species. Publications Office of the European Union, Luxembourg, e01f0b9." },
  { id: "wood-handbook-2021", citation: "Forest Products Laboratory (2021). Wood Handbook: Wood as an Engineering Material. General Technical Report FPL-GTR-282, USDA Forest Service. Chapter 5 (mechanical properties of green wood); used as a guide, the species values are authored estimates." },
  { id: "en-1991-1-4", citation: "EN 1991-1-4:2005 Eurocode 1, Actions on structures, Part 1-4: Wind actions. Air density 1.25 kg/m3, drag force F = 0.5 rho cd A v^2, power-law profile." },
  { id: "en-1991-1-3", citation: "EN 1991-1-3:2003 Eurocode 1, Actions on structures, Part 1-3: Snow loads. Used for the order of magnitude of wet-snow accretion on foliage." },
  { id: "vogel-1989", citation: "Vogel, S. (1989). Drag and reconfiguration of broad leaves in high winds. Journal of Experimental Botany 40(8), 941-948." },
  { id: "moore-maguire-2004", citation: "Moore, J. R. and Maguire, D. A. (2004). Natural sway frequencies and damping ratios of trees: concepts, review and synthesis of previous studies. Trees 18, 195-203. Used for the order of magnitude of the first sway frequency." },
  { id: "jones-2014", citation: "Jones, H. G. (2014). Plants and Microclimate, 3rd ed., Cambridge University Press. Leaf energy balance, leaf area index, stomatal and boundary-layer conductance." },
  { id: "monteith-unsworth-2013", citation: "Monteith, J. L. and Unsworth, M. H. (2013). Principles of Environmental Physics, 4th ed., Academic Press. Penman-Monteith leaf transpiration." },
  { id: "rowan-crop", citation: "Authored estimate of the berry crop of a mature open-grown rowan (about 10 to 25 kg of berries of 0.3 to 0.4 g in a mast year), guided by Raspé et al. (2000); not a cited measurement." },
];

const cropBones = [...new Set(models[50].clusters.map((c) => c.bone))];
const berryArea = r4(((15 / 0.00035) * 4 * Math.PI * 0.0045 ** 2));
const CALM = { "stem-bark": 0.35, "branch-bark": 0.25, "leaf-sun": 0.85, "leaf-shade": 0.12, fruit: 0.5 };
const WINDY = { stem: 0.6, leader: 1.2, limb: 2.2, branch: 2.2, foliage: 6 } as const;
const GALE = { stem: 1.8, leader: 3.6, limb: 6, branch: 6, foliage: 16 } as const;
const berries = (ctx: Ctx, kg: number): Json[] => verticalLoads(ctx, "carried", "berries", (id) => tx(`Berry corymbs on foliage mass ${id}`, `Beerendolden an Laubmasse ${id}`), "rowan-crop", null, kg, cropBones);
export const rowan: Species = {
  id: "rowan",
  title: tx("Rowan, Mountain Ash", "Eberesche, Vogelbeere"),
  summary: tx("An open-grown rowan of about 40 years, 10 m high: a clear silver-grey stem of 2.3 m with horizontal lenticels, a continuous leader with nine steeply ascending limbs and an ovoid, airy crown of pinnate leaves on which orange-red berry corymbs hang from late summer; berry load, wind and early wet snow are seasonal poses and loads.", "Eine frei gewachsene Eberesche von etwa 40 Jahren, 10 m hoch: ein astfreier, silbergrauer Stamm von 2,3 m mit quer liegenden Korkporen, ein durchgehender Leittrieb mit neun steil aufsteigenden Ästen und eine eiförmige, lockere Krone aus gefiederten Blättern, an der ab dem Spätsommer orangerote Beerendolden hängen; Beerenlast, Wind und früher Nassschnee sind jahreszeitliche Posen und Lasten."),
  tags: ["tree", "deciduous", "broadleaf", "native", "pinnate", "fruit-load", "berries", "wind", "transpiration"],
  provenance: { author: "Semio staffage plant collection worker PW-06", method: "handcrafted", created: "2026-11-22", references: REFS, notes: tx("Rig, body and drawings are authored by hand for an open-grown rowan of about 40 years in a park or street verge: 10 m high (rowans reach 5 to 15 m), crown 6.2 m wide and 5.8 m deep, stem diameter about 0.27 m at breast height. Pinnate leaves of 15 to 20 cm (11 to 15 leaflets) form the foliage lobes; the corymbs of 50 to 150 orange-red berries of 8 mm are drawn as 14 cm clusters, a sample of the crop, not a count. Berry crop, snow and wood properties are authored estimates guided by the cited sources.", "Rig, Körper und Zeichnungen sind von Hand für eine frei gewachsene Eberesche von etwa 40 Jahren in einem Park oder Straßenrand entworfen: 10 m hoch (Ebereschen werden 5 bis 15 m), Krone 6,2 m breit und 5,8 m tief, Stammdurchmesser etwa 0,27 m in Brusthöhe. Gefiederte Blätter von 15 bis 20 cm (11 bis 15 Fiederblättchen) bilden die Laublappen; die Dolden aus 50 bis 150 orangeroten Beeren von 8 mm sind als Büschel von 14 cm gezeichnet, eine Auswahl des Behangs, keine Zählung. Beerenbehang, Schnee und Holzkennwerte sind Autorenschätzungen, angelehnt an die genannten Quellen.") },
  materials: [
    { id: BARK, title: tx("Rowan bark, smooth silver-grey with horizontal lenticels", "Ebereschenrinde, glatt silbergrau mit quer liegenden Korkporen"), emissivity: 0.95, solarAbsorptivity: 0.55, densityKgM3: 700, conductivityWmK: 0.12, source: "authored-estimate", reference: "wood-handbook-2021" },
    { id: YOUNG, title: tx("Smooth grey-brown bark of limbs and twigs", "Glatte graubraune Rinde der Äste und Zweige"), emissivity: 0.95, solarAbsorptivity: 0.62, densityKgM3: 720, conductivityWmK: 0.13, source: "authored-estimate", reference: "wood-handbook-2021" },
    { id: SUN, title: tx("Rowan sun leaves (pinnate)", "Ebereschen-Sonnenblätter (gefiedert)"), emissivity: 0.97, solarAbsorptivity: 0.5, source: "literature", reference: "jones-2014" },
    { id: SHADE, title: tx("Rowan shade leaves (effective)", "Ebereschen-Schattenblätter (wirksam)"), emissivity: 0.97, solarAbsorptivity: 0.25, source: "authored-estimate", reference: "jones-2014" },
    { id: BERRY, title: tx("Rowan berries, orange-red glossy pomes", "Vogelbeeren, glänzend orangerote Apfelfrüchte"), emissivity: 0.95, solarAbsorptivity: 0.65, densityKgM3: 1000, conductivityWmK: 0.45, specificHeatJKgK: 3600, source: "authored-estimate", reference: "raspe-2000" },
    { id: "rowan-wood", title: tx("Green rowan wood", "Frisches Ebereschenholz"), emissivity: 0.9, solarAbsorptivity: 0.6, densityKgM3: 950, conductivityWmK: 0.17, elasticModulusPa: 8000000000, strengthPa: 50000000, source: "authored-estimate", reference: "wood-handbook-2021" },
  ],
  bones: BONES, models, woodDensity: 950, leafMassKgM2: 0.2, barkFraction: 0.06,
  regionOfTube: (t) => (t.group === "limb" || t.group === "branch" ? "branch-bark" : "stem-bark"),
  regionOfLobe: (l) => (l.group === "shade" ? "leaf-shade" : "leaf-sun"),
  fruitRegion: "fruit",
  regions: [
    { id: "stem-bark", title: tx("Silver-grey bark of root flare, stem and leader", "Silbergraue Rinde von Wurzelanlauf, Stamm und Leittrieb"), kind: "bark", model: "passive", material: BARK, areaM2: 0, characteristicLengthM: 0.26, geometry: "cylinder", skyViewFactor: 0.45, layers: [], heatCapacityJM2K: 60000, source: "authored-estimate", reference: "wood-handbook-2021" },
    { id: "branch-bark", title: tx("Bark of the ascending limbs and side branches", "Rinde der aufsteigenden Äste und Seitenzweige"), kind: "bark", model: "passive", material: YOUNG, areaM2: 0, characteristicLengthM: 0.08, geometry: "cylinder", skyViewFactor: 0.5, layers: [], heatCapacityJM2K: 30000, source: "authored-estimate", reference: "wood-handbook-2021" },
    { id: "leaf-sun", title: tx("Sun leaves of the outer crown", "Sonnenblätter der Außenkrone"), kind: "foliage", model: "leaf", material: SUN, areaM2: 52, areaBasis: "exchange", characteristicLengthM: 0.02, geometry: "vertical-plate", skyViewFactor: 0.7, layers: [], heatCapacityJM2K: 700, leaf: { stomatalConductanceMaxMS: 0.005, leafAreaIndex: 1.84, photosynthesis: "c3" }, source: "authored-estimate", reference: "jones-2014" },
    { id: "leaf-shade", title: tx("Shade leaves of the inner crown", "Schattenblätter der Innenkrone"), kind: "foliage", model: "leaf", material: SHADE, hidden: tx("The shade leaves lie inside the crown, behind the sun leaves from every side, so they are drawn by no shape; their lobes are thermal solids of their own inside the body.", "Die Schattenblätter liegen im Inneren der Krone, von jeder Seite hinter den Sonnenblättern, und werden daher von keiner Form gezeichnet; ihre Lappen sind eigene Thermikkörper im Körper."), areaM2: 34, areaBasis: "exchange", characteristicLengthM: 0.025, geometry: "vertical-plate", skyViewFactor: 0.3, layers: [], heatCapacityJM2K: 550, leaf: { stomatalConductanceMaxMS: 0.0028, leafAreaIndex: 1.2, photosynthesis: "c3" }, source: "authored-estimate", reference: "jones-2014" },
    { id: "fruit", title: tx("Skin of the berries", "Schale der Beeren"), kind: "surface", model: "passive", material: BERRY, areaM2: berryArea, areaBasis: "exchange", characteristicLengthM: 0.008, geometry: "sphere", skyViewFactor: 0.5, layers: [], heatCapacityJM2K: 5400, source: "authored-estimate", reference: "raspe-2000" },
  ],
  emitters: [
    { id: "transpiration-sun", kind: "transpiration", title: tx("Transpiration of the sun leaves", "Transpiration der Sonnenblätter"), anchor: "tp-sun-1", anchors: ["tp-sun-1", "tp-sun-2", "tp-sun-3", "tp-sun-4", "tp-sun-5", "tp-sun-6", "tp-sun-7"], directionDeg: 0, diameterM: 1.6, region: "leaf-sun", source: "authored-estimate", reference: "monteith-unsworth-2013" },
    { id: "transpiration-shade", kind: "transpiration", title: tx("Transpiration of the shade leaves", "Transpiration der Schattenblätter"), anchor: "tp-shade-1", anchors: ["tp-shade-1", "tp-shade-2", "tp-shade-3", "tp-shade-4", "tp-shade-5"], directionDeg: 0, diameterM: 1.2, region: "leaf-shade", source: "authored-estimate", reference: "monteith-unsworth-2013" },
  ],
  environment: { airTempC: 22, meanRadiantTempC: 25, relativeHumidity: 0.55, windMS: 1.5, windDirectionDeg: 0, solarWM2: 700, pressurePa: 101325, diffuseFraction: 0.15 },
  thermalAssumptions: [
    tx("Leaf regions use one-sided leaf area; the leaf area index is that area over the crown ground projection of about 28 m2. The maximum stomatal conductance is an authored estimate for a mesophytic broadleaf of open sites, guided by Jones (2014) (typical range 0.003 to 0.008 m/s); no published species maximum is claimed. The characteristic length is the width of a leaflet (2 cm), not the pinnate leaf. Photosynthesis type: C3. Aperture and night closure are the engine's business, not authored here.", "Blattregionen verwenden die einseitige Blattfläche; der Blattflächenindex ist diese Fläche bezogen auf die Kronenprojektion von etwa 28 m2. Die maximale Spaltöffnungsleitfähigkeit ist eine Autorenschätzung für ein mesophytisches Laubgehölz offener Standorte, angelehnt an Jones (2014) (übliche Spanne 0,003 bis 0,008 m/s); ein veröffentlichter Artwert wird nicht behauptet. Die charakteristische Länge ist die Breite eines Fiederblättchens (2 cm), nicht das gefiederte Blatt. Photosynthesetyp: C3. Öffnung und nächtliches Schließen sind Sache der Engine und werden hier nicht vorgegeben."),
    tx("Bark is passive. Its areal heat capacity is the volumetric heat capacity of green wood (about 2.7 MJ per m3 and K) times an effective depth of 1 to 2.5 cm, an authored estimate guided by the Wood Handbook; the bark areas are the mantles of the modelled stem and limbs. No surface temperature is authored.", "Rinde ist passiv. Ihre flächenbezogene Wärmekapazität ist die volumetrische Wärmekapazität von frischem Holz (etwa 2,7 MJ je m3 und K) mal einer wirksamen Tiefe von 1 bis 2,5 cm, eine Autorenschätzung angelehnt an das Wood Handbook; die Rindenflächen sind die Mäntel des modellierten Stamms und der Äste. Es werden keine Oberflächentemperaturen vorgegeben."),
    tx("Transpiration emitters release the latent flux of their foliage region as vapour, spread over at least five anchors on the crown surface (the shade anchors on the shade lobes inside (thermal solids only)); the direction is the vertical measured in the frame of the first anchor's bone, since the release is diffuse. Each pose authors the sunlit share of every region.", "Transpirationsemitter geben den latenten Strom ihrer Laubregion als Dampf ab, verteilt auf mindestens fünf Anker an der Kronenoberfläche (die Schattenanker an den inneren Schattenlappen (nur als Thermikkörper)); die Richtung ist die Senkrechte im Bezugssystem des Knochens des ersten Ankers, da die Abgabe diffus ist. Jede Pose gibt den besonnten Anteil jeder Region vor."),
    tx(`The berry skin is a passive surface of a crop of about 15 kg (some 43 000 berries of 9 mm, ${berryArea} m2, exchange area); its areal heat capacity is that of a water-rich sphere (1000 kg/m3, 3600 J/(kg K), radius/3), an authored estimate. Leaf colour, leaf fall and the berry-free state are not geometric: a pose cannot hide shapes, so seasonal poses change the thermal binding and the loads only.`, `Die Beerenschale ist eine passive Fläche eines Behangs von etwa 15 kg (rund 43 000 Beeren von 9 mm, ${String(berryArea).replace(".", ",")} m2, Austauschfläche); ihre flächenbezogene Wärmekapazität ist die einer wasserreichen Kugel (1000 kg/m3, 3600 J/(kg K), Radius/3), eine Autorenschätzung. Laubfärbung, Laubfall und der beerenlose Zustand sind nicht geometrisch: Eine Pose kann keine Formen ausblenden, daher ändern jahreszeitliche Posen nur die thermische Bindung und die Lasten.`),
  ],
  structureAssumptions: [
    tx("The tree is one rigid body per evaluated pose, fixed at the root plate; wind drag acts at the centre of each foliage mass and on the stem as a line load, gravity on every bone mass, berries and snow as vertical loads on the foliage masses. The base moment follows from statics at the posed geometry; root-soil failure and dynamic amplification are not modelled. Pose lean angles are authored estimates that grow with the drag force, not a computed deflection.", "Der Baum ist je ausgewerteter Pose ein starrer Körper, am Wurzelteller eingespannt; Winddruck wirkt im Schwerpunkt jeder Laubmasse und am Stamm als Linienlast, Schwerkraft auf jede Knochenmasse, Beeren und Schnee als senkrechte Lasten auf die Laubmassen. Das Fußmoment folgt aus der Statik der gezeigten Geometrie; Wurzel-Boden-Versagen und dynamische Überhöhung werden nicht modelliert. Die Neigungswinkel der Posen sind Autorenschätzungen, die mit der Widerstandskraft wachsen, keine berechnete Durchbiegung."),
    tx("Sections are solid circles of green wood under a bark of 6 % of the radius (authored-estimate properties guided by the Wood Handbook: 950 kg/m3, E 8 GPa, bending strength 50 MPa); the rig is planar, so limbs that run out of the side-elevation plane carry their lateral position as an offset and bend in that plane only.", "Querschnitte sind Vollkreise aus frischem Holz unter einer Rinde von 6 % des Radius (Autorenschätzungen angelehnt an das Wood Handbook: 950 kg/m3, E 8 GPa, Biegefestigkeit 50 MPa); das Rig ist eben, daher tragen Äste, die aus der Ebene der Seitenansicht laufen, ihre seitliche Lage als Versatz und biegen sich nur in dieser Ebene."),
  ],
  poses: (ctx) => [
    { id: "late-summer-calm", title: tx("Late summer, calm", "Spätsommer, windstill"), description: tx("Light air of 1.5 m/s at 10 m in late August: about 15 kg of ripening berries hang in the outer crown, the limbs stand at rest; drag per foliage mass is a few newtons (cd 0.45).", "Leiser Zug von 1,5 m/s in 10 m Höhe Ende August: etwa 15 kg reifende Beeren hängen in der Außenkrone, die Äste stehen in Ruhe; der Widerstand je Laubmasse beträgt wenige Newton (cd 0,45)."), bones: {}, ground: "authored", contacts: [], thermal: binding(rowan, CALM, { "transpiration-sun": 0.85, "transpiration-shade": 0.6 }), loads: [...windLoads(ctx, 1.5, 0.45), ...berries(ctx, 15)] } as Json,
    { id: "fresh-breeze", title: tx("Fresh breeze", "Frische Brise"), description: tx("Beaufort 5, 8 m/s at 10 m, cd 0.35 for the pinnate foliage on the front area of each foliage mass: the stem leans by 0.6 degrees, the leader by 1.2, the limbs by 2.2 and the leafy masses by 6 degrees downwind; 15 kg of berries.", "Beaufort 5, 8 m/s in 10 m Höhe, cd 0,35 für das gefiederte Laub auf die Anströmfläche jeder Laubmasse: Der Stamm neigt sich um 0,6 Grad, der Leittrieb um 1,2, die Äste um 2,2 und die Laubmassen um 6 Grad mit dem Wind; 15 kg Beeren."), bones: offsets(ctx, WINDY), ground: "authored", contacts: [], thermal: binding(rowan, CALM, { "transpiration-sun": 1, "transpiration-shade": 0.7 }), loads: [...windLoads(ctx, 8, 0.35), ...berries(ctx, 15)] } as Json,
    { id: "gale", title: tx("Gale with rain", "Sturm mit Regen"), description: tx("Beaufort 9, 22 m/s at 10 m with rain, cd 0.22 after the reconfiguration of the leaves (Vogel 1989): strong lean of limbs and foliage, wet bark and leaves; 8 kg of berries are left after the stripping of the outer corymbs.", "Beaufort 9, 22 m/s in 10 m Höhe mit Regen, cd 0,22 nach der Rekonfiguration der Blätter (Vogel 1989): starke Neigung von Ästen und Laub, nasse Rinde und Blätter; 8 kg Beeren bleiben, nachdem die äußeren Dolden abgerissen sind."), bones: offsets(ctx, GALE), ground: "authored", contacts: [], thermal: binding(rowan, { "stem-bark": 0.05, "branch-bark": 0.04, "leaf-sun": 0.12, "leaf-shade": 0.02, fruit: 0.08 }, { "transpiration-sun": 0.4, "transpiration-shade": 0.28 }, 0.6), loads: [...windLoads(ctx, 22, 0.22), ...berries(ctx, 8)] } as Json,
    { id: "berry-laden", title: tx("Full berry crop, September", "Voller Beerenbehang, September"), description: tx("A calm September day in a mast year with about 25 kg of ripe berries: the limbs bow down by 3 degrees and the berry-laden foliage masses by 5 degrees; the ageing leaves transpire at 60 %.", "Ein windstiller Septembertag in einem Mastjahr mit etwa 25 kg reifen Beeren: Die Äste neigen sich um 3 Grad nach unten, die beerenbehangenen Laubmassen um 5 Grad; die alternden Blätter transpirieren zu 60 %."), bones: offsets(ctx, {}, { limb: 3, branch: 3, foliage: 5 }), ground: "authored", contacts: [], thermal: binding(rowan, { "stem-bark": 0.3, "branch-bark": 0.22, "leaf-sun": 0.78, "leaf-shade": 0.1, fruit: 0.55 }, { "transpiration-sun": 0.6, "transpiration-shade": 0.42 }), loads: [...windLoads(ctx, 1.5, 0.45), ...berries(ctx, 25)] } as Json,
    { id: "early-wet-snow", title: tx("Early wet snow in October", "Früher Nassschnee im Oktober"), description: tx("An early snowfall on the still leafy crown with 18 kg of berries: about 8 kg/m2 of wet snow on the plan area of each foliage mass (order of magnitude after EN 1991-1-3) bows the limbs by 7 degrees and the masses by 12 degrees; wet, cold, overcast, the stomata nearly closed.", "Ein früher Schneefall auf die noch belaubte Krone mit 18 kg Beeren: etwa 8 kg/m2 Nassschnee auf der Grundrissfläche jeder Laubmasse (Größenordnung nach EN 1991-1-3) biegen die Äste um 7 Grad und die Massen um 12 Grad nach unten; nass, kalt, bedeckt, die Spaltöffnungen fast geschlossen."), bones: offsets(ctx, {}, { limb: 7, branch: 7, foliage: 12 }), ground: "authored", contacts: [], thermal: binding(rowan, { "stem-bark": 0.02, "branch-bark": 0.02, "leaf-sun": 0.05, "leaf-shade": 0.01, fruit: 0.03 }, { "transpiration-sun": 0.08, "transpiration-shade": 0.05 }, 0.9), loads: [...verticalLoads(ctx, "snow", "snow", (id) => tx(`Wet snow on foliage mass ${id}`, `Nassschnee auf Laubmasse ${id}`), "en-1991-1-3", 8), ...berries(ctx, 18)] } as Json,
  ],
  clips: (ctx) => {
    const windIds = windLoads(ctx, 8, 0.35).map((l) => l.id);
    return [
      { id: "wind-sway", title: tx("Wind sway", "Windschwingung"), description: tx("A deterministic 2.6 s sway cycle (first mode of a 10 m broadleaf of about 0.38 Hz, an authored estimate within the range reviewed by Moore and Maguire 2004) about the fresh-breeze lean, plus and minus 35 % of it; the leader lags the stem by 3 %, the limbs by 7 % and the leafy masses by 12 % of the period.", "Ein deterministischer Schwingungszyklus von 2,6 s (erste Eigenform eines 10 m hohen Laubbaums von etwa 0,38 Hz, eine Autorenschätzung im von Moore und Maguire 2004 zusammengestellten Bereich) um die Neigung der frischen Brise, plus und minus 35 % davon; der Leittrieb folgt dem Stamm mit 3 %, die Äste mit 7 % und die Laubmassen mit 12 % der Periode."), durationS: 2.6, fps: 30, loop: true, rootMotion: "none", keys: swayKeys(ctx, "fresh-breeze", WINDY, 0.35, { leader: 0.03, limb: 0.07, branch: 0.07, foliage: 0.12 }, 2.6), contacts: [], phases: [{ id: "sway", fromS: 0, toS: 2.6, title: tx("Sway cycle", "Schwingungszyklus") }], events: [{ t: 0, kind: "motion", title: tx("Cycle start at the mean lean", "Zyklusbeginn in mittlerer Neigung") }], tracks: [{ target: "emitter:transpiration-sun", ease: "smooth", keys: [{ t: 0, v: 1 }, { t: 1.3, v: 0.92 }, { t: 2.6, v: 1 }] }, { target: "emitter:transpiration-shade", ease: "smooth", keys: [{ t: 0, v: 0.7 }, { t: 1.3, v: 0.66 }, { t: 2.6, v: 0.7 }] }] },
      { id: "gust", title: tx("Gust", "Windböe"), description: tx("A single 8 s gust: the wind-load tracks rise from 5 % to 2.75 times the fresh-breeze load at 2.5 s (EN 1991-1-4 peak velocity pressure with a turbulence intensity of 0.25) and decay; the crown leans with the load to the power 0.8, at most 70 % of the gale lean, and springs back.", "Eine einzelne Böe über 8 s: Die Windlast-Spuren steigen bei 2,5 s von 5 % auf das 2,75-fache der Last der frischen Brise (Spitzengeschwindigkeitsdruck nach EN 1991-1-4 mit einer Turbulenzintensität von 0,25) und klingen ab; die Krone neigt sich mit der Last hoch 0,8, höchstens 70 % der Sturmneigung, und federt zurück."), durationS: 8, fps: 30, loop: false, rootMotion: "none", keys: gustKeys(ctx, "fresh-breeze", WINDY, GALE), contacts: [], phases: [{ id: "rise", fromS: 0, toS: 2.5, title: tx("Gust front", "Böenfront") }, { id: "peak", fromS: 2.5, toS: 3.5, title: tx("Peak", "Spitze") }, { id: "decay", fromS: 3.5, toS: 8, title: tx("Decay and recovery", "Abklingen und Erholung") }], events: [{ t: 2.5, kind: "load-on", title: tx("Peak gust load", "Böenspitze") }, { t: 5, kind: "load-off", title: tx("Load below the mean again", "Last wieder unter dem Mittelwert") }], tracks: windIds.map((id) => ({ target: `load:${id}`, ease: "smooth", keys: GUST.map(([t, v]) => ({ t, v })) })) },
      diurnalClip(rowan, "late-summer-calm", CALM, { "transpiration-sun": [[0, 0.03], [5, 0.05], [7, 0.3], [9, 0.7], [12, 1], [15, 0.9], [18, 0.35], [20, 0.08], [24, 0.03]], "transpiration-shade": [[0, 0.02], [5, 0.04], [7, 0.2], [9, 0.5], [12, 0.7], [15, 0.63], [18, 0.25], [20, 0.06], [24, 0.02]] }),
    ];
  },
  presentation: { defaultPose: "late-summer-calm", defaultClip: "wind-sway", defaultScale: 100 },
  drawing: {
    tones: TONE,
    arcScales: [100],
    glyph: (c, k) => {
      const a = (k * 47 + 20) * (Math.PI / 180);
      const [u, v] = [{ x: Math.cos(a), y: Math.sin(a) }, { x: -Math.sin(a), y: Math.cos(a) }];
      const p = (s: number, t: number) => ({ x: c.x + u.x * s + v.x * t, y: c.y + u.y * s + v.y * t });
      const rachis = [p(-0.09, 0), p(0.09, 0)];
      const ticks = [-0.06, -0.02, 0.02, 0.06].flatMap((s) => [[p(s, 0), p(s + 0.02, 0.03)], [p(s, 0), p(s + 0.02, -0.03)]]);
      return { closed: false, rings: [rachis, ...ticks] };
    },
    bark: (view: View, t: Tube, scale: Scale) => barkMarks(view, t, scale === 50 ? 0.15 : 0.35, (n, r) => { const off = [-0.35, 0.2, -0.05, 0.4][n % 4]! * r, h = (scale === 50 ? 0.22 : 0.3) * r; return [[off - h, 0], [off + h, 0]]; }),
    decisions: {
      50: {
        silhouette: (c: Json) => tx(`At 1:50 (1 m is 20 mm) the three drawings are the true views of the 1:50 body: root flare with surface roots, the clear silver-grey stem with horizontal lenticels, the leader and nine steeply ascending limbs running into an ovoid crown drawn as ten foliage masses whose outlines follow their lobes, with pinnate leaf marks and every berry corymb visible from the viewpoint (${c.elevationFruit} in the side elevation, ${c.frontFruit} in the front elevation, ${c.planFruit} in the plan). Plan: the lobed crown with corymbs and the stem as a dot.`, `Bei 1:50 (1 m sind 20 mm) sind die drei Zeichnungen die wahren Ansichten des 1:50-Körpers: Wurzelanlauf mit Oberflächenwurzeln, der astfreie silbergraue Stamm mit quer liegenden Korkporen, Leittrieb und neun steil aufsteigende Äste, die in eine eiförmige Krone aus zehn Laubmassen laufen, deren Umrisse ihren Lappen folgen, mit Fiederblattzeichen und jeder aus der Blickrichtung sichtbaren Beerendolde (${c.elevationFruit} in der Seitenansicht, ${c.frontFruit} in der Vorderansicht, ${c.planFruit} in der Draufsicht). Draufsicht: die gelappte Krone mit Dolden und der Stamm als Punkt.`),
        retained: tx("Retained: every woody member of the rig with its side branches, every foliage mass, the visible berry corymbs at their real size (2.8 mm).", "Beibehalten: jedes Holzglied des Rigs mit seinen Seitenzweigen, jede Laubmasse, die sichtbaren Beerendolden in wahrer Größe (2,8 mm)."),
        merged: tx("Single pinnate leaves are merged into lobed mass outlines; the berries of one corymb into one cluster; lenticels into short transverse strokes.", "Einzelne Fiederblätter gehen in gelappten Massenumrissen auf; die Beeren einer Dolde in einem Büschel; Korkporen in kurzen Querstrichen."),
        omitted: tx("Omitted: the thousands of single leaves and leaflets (3 to 6 cm), buds, twigs thinner than 1 cm and the corymbs hidden behind foliage.", "Weggelassen: die Tausende einzelner Blätter und Fiederblättchen (3 bis 6 cm), Knospen, Zweige unter 1 cm und die hinter Laub verdeckten Dolden."),
        thinElements: tx("Twig ends are kept at 1.15 times the printable minimum (11.5 mm real); leaf marks and lenticels are 0.13 mm texture strokes kept 0.35 mm inside every outline.", "Zweigenden bleiben beim 1,15-fachen des druckbaren Minimums (11,5 mm wirklich); Blattzeichen und Korkporen sind Texturstriche von 0,13 mm, mindestens 0,35 mm innerhalb jedes Umrisses."),
        hatching: tx("No hatching: the shaded inner foliage is a hidden region inside the crown and is not drawn; bark and leaves are drawn with strokes.", "Keine Schraffur: Das beschattete Innenlaub ist eine verdeckte Region im Inneren der Krone und wird nicht gezeichnet; Rinde und Blätter werden mit Strichen gezeichnet."),
        styles: tx("Light line: 0.25 mm outlines, 0.18 mm detail (berries), 0.13 mm texture. Dark solid: tonal fills separate the light stem, the limbs, the foliage and the dark berries.", "Helle Linie: 0,25 mm Umrisse, 0,18 mm Detail (Beeren), 0,13 mm Textur. Dunkle Fläche: getönte Füllungen trennen den hellen Stamm, die Äste, das Laub und die dunklen Beeren."),
        overlays: tx("Each thermal region has its own shapes (the corymbs carry the fruit region); leaf marks and lenticels are strokes without a region and follow their bone.", "Jede thermische Region hat eigene Formen (die Dolden tragen die Fruchtregion); Blattzeichen und Korkporen sind Striche ohne Region und folgen ihrem Knochen."),
      },
      100: {
        silhouette: (c: Json) => tx(`At 1:100 (1 m is 10 mm) the drawings are the true views of the 1:100 body: flared root, stem, leader and the nine limbs with six side branches, the crown as ten masses of broader lobes with leaf-cluster arcs, and the visible corymbs (${c.elevationFruit} in the side elevation, ${c.frontFruit} in the front elevation, ${c.planFruit} in the plan).`, `Bei 1:100 (1 m sind 10 mm) sind die Zeichnungen die wahren Ansichten des 1:100-Körpers: ausgestellter Wurzelanlauf, Stamm, Leittrieb und die neun Äste mit sechs Seitenzweigen, die Krone aus zehn Massen breiterer Lappen mit Blattbüschelbögen und die sichtbaren Dolden (${c.elevationFruit} in der Seitenansicht, ${c.frontFruit} in der Vorderansicht, ${c.planFruit} in der Draufsicht).`),
        retained: tx("Retained: ten foliage masses, root flare, stem, leader, all limbs, six side branches, the visible corymbs as filled dots and a few lenticel strokes on the stem.", "Beibehalten: zehn Laubmassen, Wurzelanlauf, Stamm, Leittrieb, alle Äste, sechs Seitenzweige, die sichtbaren Dolden als gefüllte Punkte und einige Korkporenstriche am Stamm."),
        merged: tx("Leaves and small lobes are merged into broader lobed mass outlines; surface roots into the flare.", "Blätter und kleine Lappen gehen in breiteren gelappten Massenumrissen auf; Oberflächenwurzeln im Wurzelanlauf."),
        omitted: tx("Omitted: leaf marks, the six weaker side branches, the surface roots and most corymbs (the body carries a sample of twenty-three).", "Weggelassen: Blattzeichen, die sechs schwächeren Seitenzweige, die Oberflächenwurzeln und die meisten Dolden (der Körper trägt eine Auswahl von dreiundzwanzig)."),
        thinElements: tx("Wood thinner than 23 mm is raised to it (the printable 0.2 mm plus 15 %); arcs and lenticels use the 0.13 mm detail pen.", "Holz unter 23 mm wird darauf angehoben (die druckbaren 0,2 mm plus 15 %); Bögen und Korkporen nutzen den Detailstift von 0,13 mm."),
        hatching: tx("No hatching: the shaded inner foliage is hidden inside the crown; arcs and bark strokes carry the texture.", "Keine Schraffur: Das beschattete Innenlaub liegt verdeckt im Inneren der Krone; Bögen und Rindenstriche tragen die Textur."),
        styles: tx("Light line: 0.2 mm outlines, 0.13 mm detail, 0.1 mm texture. Dark solid: tonal masses, detail strokes stay visible on them.", "Helle Linie: 0,2 mm Umrisse, 0,13 mm Detail, 0,1 mm Textur. Dunkle Fläche: getönte Massen, Detailstriche bleiben darauf sichtbar."),
        overlays: tx("Every region keeps its shapes; the corymb dots carry the fruit region.", "Jede Region behält ihre Formen; die Doldenpunkte tragen die Fruchtregion."),
      },
      200: {
        silhouette: (c: Json) => tx(`At 1:200 (1 m is 5 mm) the drawings are the true views of the 1:200 body: stem with root flare, leader and nine ascending limbs under an ovoid crown of ten foliage masses with scalloped outlines; the visible corymbs are single dots (${c.elevationFruit} in the side elevation, ${c.frontFruit} in the front elevation, ${c.planFruit} in the plan).`, `Bei 1:200 (1 m sind 5 mm) sind die Zeichnungen die wahren Ansichten des 1:200-Körpers: Stamm mit Wurzelanlauf, Leittrieb und neun aufsteigende Äste unter einer eiförmigen Krone aus zehn Laubmassen mit gewellten Umrissen; die sichtbaren Dolden sind einzelne Punkte (${c.elevationFruit} in der Seitenansicht, ${c.frontFruit} in der Vorderansicht, ${c.planFruit} in der Draufsicht).`),
        retained: tx("Retained: root flare, stem, leader, the nine limbs at no less than 46 mm, the foliage masses and twelve sample corymbs.", "Beibehalten: Wurzelanlauf, Stamm, Leittrieb, die neun Äste mit mindestens 46 mm, die Laubmassen und zwölf Beispieldolden."),
        merged: tx("Side branches, leaves and small lobes are merged into the large scalloped masses.", "Seitenzweige, Blätter und kleine Lappen gehen in den großen gewellten Massen auf."),
        omitted: tx("Omitted: lenticels, leaf marks, arcs, side branches and all other corymbs.", "Weggelassen: Korkporen, Blattzeichen, Bögen, Seitenzweige und alle übrigen Dolden."),
        thinElements: tx("Limb tips are raised to 46 mm (0.23 mm on paper) instead of tapering below the print minimum; they run into the masses.", "Astspitzen werden auf 46 mm (0,23 mm auf dem Papier) angehoben, statt unter das Druckminimum zu verjüngen; sie laufen in die Massen."),
        hatching: tx("No hatching and no internal edge.", "Keine Schraffur und keine innere Kante."),
        styles: tx("Light line: outlines 0.18 mm. Dark solid: the foliage in two tones, wood lighter, berries dark.", "Helle Linie: Umrisse 0,18 mm. Dunkle Fläche: das Laub in zwei Tönen, Holz heller, Beeren dunkel."),
        overlays: tx("Every thermal region keeps a shape: stem, limbs, the foliage masses, the corymb dots.", "Jede thermische Region behält eine Form: Stamm, Äste, die Laubmassen, die Doldenpunkte."),
      },
      500: {
        silhouette: () => tx("At 1:500 (1 m is 2 mm) the rowan is one ovoid crown, 12 mm wide and 15 mm high, with a scalloped underside at 2.8 m, above a slim light stem with its root flare; the drawings are the true views of the 1:500 body. Front: stem and crown. Plan: one scalloped crown outline; the stem is hidden.", "Bei 1:500 (1 m sind 2 mm) ist die Eberesche eine eiförmige Krone von 12 mm Breite und 15 mm Höhe mit gewellter Unterseite auf 2,8 m über einem schlanken hellen Stamm mit Wurzelanlauf; die Zeichnungen sind die wahren Ansichten des 1:500-Körpers. Vorderansicht: Stamm und Krone. Draufsicht: ein gewellter Kronenumriss; der Stamm ist verdeckt."),
        retained: tx("Retained: stem with root flare and the crown outline; the leader runs on into the crown for a clean join.", "Beibehalten: Stamm mit Wurzelanlauf und der Kronenumriss; der Leittrieb läuft für einen sauberen Anschluss in die Krone weiter."),
        merged: tx("Merged into the crown shape: the foliage, the limbs and the berries (as merged regions).", "In der Kronenform zusammengefasst: das Laub, die Äste und die Beeren (als zusammengefasste Regionen)."),
        omitted: tx("Omitted: single limbs, corymbs (14 cm is 0.28 mm), lenticels; no internal edge.", "Weggelassen: einzelne Äste, Dolden (14 cm sind 0,28 mm), Korkporen; keine innere Kante."),
        thinElements: tx("The stem keeps its real width (0.5 mm); limbs are not thickened but merged into the filled crown.", "Der Stamm behält seine wirkliche Breite (0,5 mm); Äste werden nicht verdickt, sondern in die gefüllte Krone aufgenommen."),
        hatching: tx("No hatching; the pen is 0.15 mm for outlines and 0.25 mm for the ground line.", "Keine Schraffur; der Stift ist 0,15 mm für Umrisse und 0,25 mm für die Bodenlinie."),
        styles: tx("Light line: closed outlines only. Dark solid: the crown reads as one mass, the stem lighter.", "Helle Linie: nur geschlossene Umrisse. Dunkle Fläche: Die Krone liest sich als eine Masse, der Stamm heller."),
        overlays: tx("Overlays map onto: stem-bark (stem) and, merged in the crown shape, leaf-sun, leaf-shade, branch-bark and fruit; merged shapes show the area-weighted value of their regions, labelled as merged.", "Überlagerungen bilden ab auf: stem-bark (Stamm) und, in der Kronenform zusammengefasst, leaf-sun, leaf-shade, branch-bark und fruit; zusammengefasste Formen zeigen den flächengewichteten Wert ihrer Regionen, als zusammengefasst gekennzeichnet."),
      },
    },
  },
  symbol: { id: "rowan-symbol", title: tx("Plan symbol with true crown extent", "Grundrisssymbol mit wahrer Kronenausdehnung"), description: tx("A conventional plan sign of the rowan for site and planting plans: the true outline of the crown and the stem section at breast height. It is a separate symbol and is never used silently for a pictorial scale.", "Ein konventionelles Grundrisszeichen der Eberesche für Lage- und Pflanzpläne: der wahre Umriss der Krone und der Stammquerschnitt in Brusthöhe. Es ist ein eigenes Symbol und wird nie stillschweigend für einen Bildmaßstab verwendet.") },
  distinction: tx("A small native tree with a continuous silver-grey leader, steeply ascending limbs and an ovoid crown of pinnate leaves, the only plant of the archive that carries hanging berry corymbs as drawn clusters and a berry crop as a seasonal load.", "Ein kleiner heimischer Baum mit durchgehendem silbergrauem Leittrieb, steil aufsteigenden Ästen und einer eiförmigen Krone aus gefiederten Blättern, die einzige Pflanze des Archivs, die hängende Beerendolden als gezeichnete Büschel und einen Beerenbehang als jahreszeitliche Last trägt."),
  anchorsTitle: (em, n) => (em === "transpiration-sun" ? tx(`Transpiration of the sun leaves, anchor ${n}`, `Transpiration der Sonnenblätter, Anker ${n}`) : tx(`Transpiration of the shade leaves, anchor ${n}`, `Transpiration der Schattenblätter, Anker ${n}`)),
};

const barkScale: Record<string, number> = { "stem-bark": 0.97, "branch-bark": 0.95 };
for (const r of rowan.regions) if (barkScale[r.id] !== undefined) r.areaM2 = r4(models[50].tubes.filter((t) => rowan.regionOfTube(t) === r.id).reduce((s, t) => s + tubeArea(t), 0) * barkScale[r.id]!);
export default rowan;
if (import.meta.main) {
  const result = build(rowan);
  const path = write(rowan, result, process.argv.includes("--catalog") ? "catalog" : "candidate");
  console.log(result.report.join("\n"));
  console.log(path);
}
