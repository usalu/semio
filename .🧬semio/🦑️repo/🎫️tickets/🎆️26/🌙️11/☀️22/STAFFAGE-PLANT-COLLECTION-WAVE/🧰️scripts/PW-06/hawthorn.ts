/** 🌸️ Common hawthorn (Crataegus monogyna): an open-grown hedgerow and park tree of about 50 years, 6 m high, with a short, leaning, fluted stem of 1.2 m that splits into six crooked ascending limbs and an upright centre limb under a dense, low, rounded dome of small lobed leaves reaching down to 1.2 m. Every station, limb, lobe ring and outlet is stated here per scale in world metres (x ahead, y up, z to the left).
 * Usage: `repo tools exec bun -- hawthorn.ts [--catalog]` (without `--catalog` the candidate goes to `🗑️generated/PW-06/hawthorn/🔣️.json`).
 * @see ./plant-kit.ts
 * @see https://doi.org/10.1111/1365-2745.13554 — Thomas et al. 2021, Biological Flora of Britain and Ireland: Crataegus monogyna */
import { along, axisAt, barkMarks, binding, branch, build, clamp, diurnalClip, freeLobe, GUST, gustKeys, offsets, r4, ringLobes, stemTube, swayKeys, tubeArea, tx, verticalLoads, windLoads, write, type BoneDef, type Ctx, type Json, type Lobe, type RingDef, type Scale, type ScaleModel, type Species, type Tube, type V3, type View } from "./plant-kit.ts";

const BARK = "hawthorn-bark", YOUNG = "hawthorn-bark-young", SUN = "hawthorn-leaf-sun", SHADE = "hawthorn-leaf-shade";
const TONE = { root: 0.28, stem: 0.4, limb: 0.42 };

type StemRows = [number, number, number, number][];
const STEM: Record<Scale, Record<string, StemRows>> = {
  50: { "root-plate": [[0, 0.3, 0, 0], [0.07, 0.235, 0.005, 0], [0.18, 0.19, 0.012, 0.004], [0.3, 0.168, 0.02, 0.008], [0.4, 0.162, 0.028, 0.01]], trunk: [[0.25, 0.168, 0.018, 0.008], [0.5, 0.16, 0.04, 0.014], [0.75, 0.156, 0.066, 0.012], [1.0, 0.158, 0.092, 0.004], [1.2, 0.168, 0.11, -0.004], [1.38, 0.175, 0.118, -0.008]] },
  100: { "root-plate": [[0, 0.29, 0, 0], [0.12, 0.2, 0.01, 0], [0.4, 0.162, 0.028, 0.01]], trunk: [[0.25, 0.168, 0.018, 0.008], [0.75, 0.156, 0.066, 0.012], [1.38, 0.172, 0.118, -0.008]] },
  200: { "root-plate": [[0, 0.27, 0, 0], [0.4, 0.165, 0.028, 0.01]], trunk: [[0.25, 0.168, 0.018, 0.008], [1.38, 0.17, 0.118, -0.008]] },
  500: { "root-plate": [[0, 0.25, 0, 0], [0.4, 0.17, 0.028, 0]], trunk: [[0.25, 0.17, 0.018, 0], [1.38, 0.17, 0.118, 0]] },
};

/** 🌿️ The limbs from the head of the stem: [id, y on the stem, azimuth, elevation at the stem and at the tip, length, r0, r1, bend in azimuth]; hawthorn limbs are crooked, so they bend strongly. */
type LimbRow = [id: string, y: number, az: number, e0: number, e1: number, L: number, r0: number, r1: number, bend: number];
const LIMBS: LimbRow[] = [
  ["limb-1", 1.2, 10, 50, 26, 3.2, 0.085, 0.024, 16],
  ["limb-2", 1.25, 72, 54, 30, 3.0, 0.078, 0.022, -18],
  ["limb-3", 1.15, 135, 46, 22, 3.1, 0.082, 0.023, 14],
  ["limb-4", 1.3, 195, 52, 28, 3.2, 0.084, 0.024, -16],
  ["limb-5", 1.2, 255, 48, 26, 3.0, 0.08, 0.023, 18],
  ["limb-6", 1.32, 318, 56, 32, 2.9, 0.076, 0.022, -14],
  ["limb-7", 1.36, 100, 80, 66, 3.7, 0.07, 0.02, 10],
];
/** 🌿️ Side branches: [limb, fraction along, azimuth turn, elevation, length, r0, r1]. */
type SideRow = [limb: string, f: number, dAz: number, e: number, L: number, r0: number, r1: number];
const SIDES_50: SideRow[] = LIMBS.flatMap(([id], i): SideRow[] => [[id, 0.38, 42 - 8 * (i % 3), 34, 1.1, 0.03, 0.01], [id, 0.66, -40 + 6 * (i % 2), 22, 0.9, 0.024, 0.009]]);
const SIDES_100: SideRow[] = LIMBS.filter((_, i) => i < 6).map(([id], i): SideRow => [id, 0.45, i % 2 ? -38 : 40, 30, 1.0, 0.03, 0.012]);

function woody(scale: Scale): Tube[] {
  const stems = Object.entries(STEM[scale]).map(([id, r]) => stemTube(id, id, BARK, id === "root-plate" ? TONE.root : TONE.stem, id === "root-plate" ? "root" : "stem", r));
  const trunk = stems.find((t) => t.id === "trunk")!;
  const n = scale === 50 ? 7 : scale === 100 ? 5 : scale === 200 ? 3 : 2;
  const limbs = LIMBS.map(([id, y, az, e0, e1, L, r0, r1, bend]) => branch(id, id, YOUNG, TONE.limb, "limb", axisAt(trunk, y), az, e0, e1, L, r0, r1, scale === 500 ? 0 : bend, n));
  const sides = (scale === 50 ? SIDES_50 : scale === 100 ? SIDES_100 : []).map(([limb, f, dAz, e, L, r0, r1], i) => branch(`${limb}-side-${i + 1}`, limb, YOUNG, TONE.limb, "branch", along(limbs.find((x) => x.id === limb)!, f), LIMBS.find((x) => x[0] === limb)![2] + dAz, e, e - 14, L, r0, r1, 0, scale === 50 ? 4 : 3));
  const roots = scale <= 100 ? [0, 72, 144, 216, 288].map((az, i) => branch(`root-${i + 1}`, "root-plate", BARK, TONE.root, "root", [0, 0.12, 0], az + 20, -8, -8, 0.34, 0.09, 0.04, 0, scale === 50 ? 3 : 2)) : [];
  return [...stems, ...roots, ...limbs, ...sides];
}

/** ⛰️ The dense dome: widest 3.2 m at 3.2 m, top 6 m, the skirt sweeping down to 1.15 m; 0.94 as deep across as ahead. */
const env = (az: number, y: number): number => {
  const t = y - 3.2;
  const R = t >= 0 ? 3.2 * Math.max(0, 1 - (t / 2.8) ** 2.2) ** (1 / 2.2) : 3.2 * Math.max(0, 1 - Math.abs(t / 2.05) ** 2.6) ** (1 / 2.6);
  return R / Math.hypot(Math.cos((az * Math.PI) / 180), Math.sin((az * Math.PI) / 180) / 0.94);
};

const WOOD_50 = woody(50);
const TIPS = LIMBS.map(([id, , az]) => ({ id: `fol-${id.slice(5)}`, az, y: WOOD_50.find((x) => x.id === id)!.points.at(-1)!.p[1], p: WOOD_50.find((x) => x.id === id)!.points.at(-1)!.p }));
const angle = (a: number, b: number): number => Math.abs(((((a - b) % 360) + 540) % 360) - 180);
const boneAt = (az: number, y: number): string => (y >= 4.9 ? "fol-7" : TIPS.filter((t) => t.id !== "fol-7").reduce((best, t) => ((angle(az, t.az) / 45) ** 2 + ((y - t.y) / 2) ** 2 < (angle(az, best.az) / 45) ** 2 + ((y - best.y) / 2) ** 2 ? t : best)).id);
const matOf = (bone: string): string => (bone === "fol-core" ? SHADE : SUN);

/** 🍃️ The main rings shared by every scale (height, radial and vertical semi-axes, droop); the scales differ in the number and tangential width of their lobes. The small lobes give the fine, dense texture of a hawthorn crown. */
const MAIN: [id: string, y: number, a: number, b: number, tilt: number][] = [["s1", 1.6, 0.5, 0.42, 22], ["s2", 2.3, 0.56, 0.48, 12], ["s3", 3.0, 0.58, 0.5, 4], ["s4", 3.7, 0.58, 0.5, 0], ["s5", 4.4, 0.55, 0.47, -6], ["s6", 5.05, 0.5, 0.42, -12], ["s7", 5.55, 0.46, 0.38, -16]];
const main = (n: number[], c: number[], grow = 1): RingDef[] => MAIN.map(([id, y, a, b, tilt], i) => ({ id, y, n: n[i]!, az0: 0, a, b: b * grow, c: c[i]!, tilt }));
const RINGS: Record<Scale, RingDef[]> = {
  50: [
    ...main([16, 20, 20, 20, 16, 12, 8], [0.58, 0.62, 0.66, 0.64, 0.6, 0.55, 0.5]),
    { id: "b1", y: 1.95, n: 12, az0: 15, a: 0.44, b: 0.38, c: 0.5, k: 0.94 },
    { id: "b2", y: 2.65, n: 14, az0: 12.9, a: 0.46, b: 0.4, c: 0.52, k: 0.94 },
    { id: "b3", y: 3.35, n: 14, az0: 12.9, a: 0.46, b: 0.4, c: 0.52, k: 0.94 },
    { id: "b4", y: 4.05, n: 12, az0: 15, a: 0.44, b: 0.38, c: 0.5, k: 0.94 },
    { id: "b5", y: 4.75, n: 10, az0: 18, a: 0.42, b: 0.36, c: 0.48, k: 0.94 },
  ],
  100: [
    ...main([12, 16, 16, 16, 12, 8, 8], [0.72, 0.74, 0.78, 0.76, 0.76, 0.68, 0.58]),
    { id: "b2", y: 2.65, n: 8, az0: 22.5, a: 0.52, b: 0.44, c: 0.62, k: 0.93 },
    { id: "b4", y: 4.05, n: 8, az0: 22.5, a: 0.5, b: 0.42, c: 0.6, k: 0.93 },
  ],
  200: [...main([8, 8, 12, 8, 8, 4, 4], [1.0, 1.28, 1.0, 1.3, 1.12, 0.98, 0.7], 1.14).filter((r) => r.id !== "s7"), { id: "s67", y: 5.3, n: 4, az0: 45, a: 0.48, b: 0.48, c: 0.8, tilt: -14 }],
  500: [
    { id: "s1", y: 1.8, n: 6, az0: 0, a: 0.62, b: 0.62, c: 1.4, tilt: 18 },
    { id: "s2", y: 3.1, n: 8, az0: 0, a: 0.66, b: 0.85, c: 1.6, tilt: 4 },
    { id: "s3", y: 4.4, n: 8, az0: 0, a: 0.62, b: 0.72, c: 1.4, tilt: -6 },
    { id: "s4", y: 5.3, n: 4, az0: 0, a: 0.55, b: 0.5, c: 1.1, tilt: -14 },
  ],
};
const TOP: Record<Scale, [V3, V3]> = { 50: [[0.05, 5.62, 0], [0.62, 0.38, 0.58]], 100: [[0.05, 5.58, 0], [0.7, 0.42, 0.66]], 200: [[0.05, 5.52, 0], [0.8, 0.48, 0.76]], 500: [[0.05, 5.45, 0], [1.0, 0.55, 0.95]] };
const CORE: Record<Scale, [V3, V3][]> = {
  50: [[[0.3, 2.4, 0.2], [1.6, 0.8, 1.5]], [[-0.3, 3.3, -0.2], [1.8, 0.85, 1.7]], [[0.2, 4.2, -0.2], [1.5, 0.8, 1.4]], [[0, 4.95, 0.1], [1.0, 0.6, 1.0]]],
  100: [[[0.2, 2.6, 0.1], [1.7, 0.95, 1.6]], [[-0.2, 3.7, -0.1], [1.7, 0.95, 1.6]], [[0.1, 4.8, 0], [1.1, 0.7, 1.1]]],
  200: [[[0.1, 2.9, 0], [1.8, 1.1, 1.7]], [[0, 4.3, 0], [1.5, 0.95, 1.4]]],
  500: [[[0.05, 3.5, 0], [2.0, 1.5, 1.9]]],
};

function crown(scale: Scale): Lobe[] {
  const at500 = scale === 500;
  const sun = ringLobes(RINGS[scale], env, at500 ? () => "trunk" : boneAt, matOf);
  const top = freeLobe("top", at500 ? "trunk" : "fol-7", SUN, "sun", TOP[scale][0], TOP[scale][1]);
  const core = CORE[scale].map(([c, r], i) => freeLobe(`core-${i + 1}`, at500 ? "trunk" : "fol-core", SHADE, "shade", c, r));
  return [...sun, top, ...core];
}

const OUTLETS: Record<Scale, ScaleModel["outlets"]> = {
  50: [["transpiration-sun", "tp-sun-1", "s3-1", 0, 25], ["transpiration-sun", "tp-sun-2", "s3-6", 90, 25], ["transpiration-sun", "tp-sun-3", "s3-11", 180, 25], ["transpiration-sun", "tp-sun-4", "s3-16", 270, 25], ["transpiration-sun", "tp-sun-5", "s5-3", 70, 50], ["transpiration-sun", "tp-sun-6", "s5-11", 250, 50], ["transpiration-sun", "tp-sun-7", "top", 0, 90],
    ["transpiration-shade", "tp-shade-1", "core-1", 0, 90], ["transpiration-shade", "tp-shade-2", "core-2", 0, 90], ["transpiration-shade", "tp-shade-3", "core-3", 0, 90], ["transpiration-shade", "tp-shade-4", "core-4", 0, 90], ["transpiration-shade", "tp-shade-5", "core-2", 180, 20]],
  100: [["transpiration-sun", null, "s3-1", 0, 25], ["transpiration-sun", null, "s3-9", 180, 25], ["transpiration-sun", null, "s5-4", 90, 50], ["transpiration-sun", null, "top", 0, 90], ["transpiration-shade", null, "core-1", 0, 90], ["transpiration-shade", null, "core-2", 0, 90]],
  200: [["transpiration-sun", null, "s3-1", 0, 25], ["transpiration-sun", null, "top", 0, 90], ["transpiration-shade", null, "core-1", 0, 90]],
  500: [["transpiration-sun", null, "top", 0, 90], ["transpiration-shade", null, "core-1", 0, 90]],
};

const DECISIONS: Record<Scale, Json[]> = {
  50: [
    tx("Body at 1:50: root flare with five surface roots, a short leaning stem of 1.2 m that swells under the head, six crooked limbs leaving the head at 46 to 56 degrees and bending in azimuth, an upright centre limb, fourteen side branches, and the dense low dome of small-leaved lobes: seven rings from the skirt at 1.6 m to 5.55 m with five rings of smaller lobes between them, a flat top lobe at 6 m and four broad shade lobes inside (thermal solids only).", "Körper bei 1:50: Wurzelanlauf mit fünf Oberflächenwurzeln, ein kurzer, geneigter Stamm von 1,2 m, der sich unter dem Kopf verdickt, sechs knorrige Äste, die den Kopf unter 46 bis 56 Grad verlassen und sich im Grundriss krümmen, ein aufrechter Mittelast, vierzehn Seitenzweige und die dichte, tiefe Kuppel aus kleinblättrigen Lappen: sieben Ringe vom Saum auf 1,6 m bis 5,55 m mit fünf Ringen kleinerer Lappen dazwischen, ein flacher Gipfellappen auf 6 m und vier breite Schattenlappen innen (nur als Thermikkörper)."),
    tx("Thermal at 1:50: stem bark is the root flare, roots and stem; branch bark the limbs and side branches; sun leaves the outer lobes, shade leaves the four core lobes, the visible regions by reference to the body; the shade lobes (thermal solids only) are thermal solids of their own (a hidden region). The transpiration outlets are lens patches in the free faces of the lobes that carry the emitter anchors.", "Thermik bei 1:50: Stammrinde sind Wurzelanlauf, Wurzeln und Stamm; Astrinde die Äste und Seitenzweige; Sonnenblätter die äußeren Lappen, Schattenblätter die vier Kernlappen, die sichtbaren Regionen als Verweis auf den Körper; die Schattenlappen (nur als Thermikkörper) sind eigene Thermikkörper (eine verdeckte Region). Die Transpirationsauslässe sind Linsenflächen in den freien Seiten der Lappen, die die Emitteranker tragen."),
    tx("Structure at 1:50: the wood under the bark of the root flare, the stem and the seven limbs along their real axes, joints at the rig joints, the round fixed root plate and the masses of the wood bones and of the eight foliage masses.", "Tragwerk bei 1:50: das Holz unter der Rinde von Wurzelanlauf, Stamm und den sieben Ästen entlang ihrer wirklichen Achsen, Gelenke an den Rig-Gelenken, der runde eingespannte Wurzelteller und die Massen der Holzknochen und der acht Laubmassen."),
  ],
  100: [
    tx("Body at 1:100, authored anew: a flared root with five short roots, the stem on three stations, the seven limbs on five sections with six side branches, and the dome redrawn with fewer, broader lobes (seven main rings, two rings between, the top lobe and three shade lobes (thermal solids only)).", "Körper bei 1:100, neu modelliert: ein ausgestellter Wurzelanlauf mit fünf kurzen Wurzeln, der Stamm auf drei Stationen, die sieben Äste auf fünf Schnitten mit sechs Seitenzweigen und die Kuppel mit weniger, breiteren Lappen neu gezeichnet (sieben Hauptringe, zwei Zwischenringe, der Gipfellappen und drei Schattenlappen (nur als Thermikkörper))."),
    tx("Thermal at 1:100: the same four regions by reference; six outlets.", "Thermik bei 1:100: dieselben vier Regionen als Verweis; sechs Auslässe."),
    tx("Structure at 1:100: the wood of the nine load-bearing bones with simplified taper, joints, the root plate and the masses.", "Tragwerk bei 1:100: das Holz der neun tragenden Knochen mit vereinfachter Verjüngung, Gelenke, der Wurzelteller und die Massen."),
  ],
  200: [
    tx("Body at 1:200, authored anew: root flare and stem as two plain lofts, the seven limbs on three sections without side branches (tips raised to the printable 46 mm), and the dome as seven rings of large lobes (the two upper rings merged into one), a top lobe and two shade cores (thermal only).", "Körper bei 1:200, neu modelliert: Wurzelanlauf und Stamm als zwei einfache Lofts, die sieben Äste auf drei Schnitten ohne Seitenzweige (Spitzen auf druckbare 46 mm angehoben) und die Kuppel als sieben Ringe großer Lappen (die zwei oberen Ringe zu einem zusammengefasst), ein Gipfellappen und zwei Schattenkerne (nur Thermik)."),
    tx("Thermal at 1:200: bark and leaves by reference; three outlets.", "Thermik bei 1:200: Rinde und Laub als Verweis; drei Auslässe."),
    tx("Structure at 1:200: the wood of the nine load-bearing bones, joints where they are at least one smallest feature thick, the root plate and the masses.", "Tragwerk bei 1:200: das Holz der neun tragenden Knochen, Gelenke, wo sie mindestens ein kleinstes Merkmal dick sind, der Wurzelteller und die Massen."),
  ],
  500: [
    tx("Body at 1:500: root flare and stem as two plain lofts and the dome as twenty-eight masses bound to the stem as the drawing binds it (four rings, the top and the shade core) (thermal only); the limbs lie inside the dome and are not part of the body.", "Körper bei 1:500: Wurzelanlauf und Stamm als zwei einfache Lofts und die Kuppel als achtundzwanzig Massen, an den Stamm gebunden wie in der Zeichnung (vier Ringe, der Gipfel und der Schattenkern) (nur Thermik); die Äste liegen in der Kuppel und gehören nicht zum Körper."),
    tx("Thermal at 1:500: stem bark and leaves by reference; the seven limbs are thermal solids of their own inside the dome; two outlets.", "Thermik bei 1:500: Stammrinde und Laub als Verweis; die sieben Äste sind eigene Thermikkörper in der Kuppel; zwei Auslässe."),
    tx("Structure at 1:500: the nine load-bearing bones as members of their declared section, the joint of the stem, the root plate and the masses; this list is already minimal.", "Tragwerk bei 1:500: die neun tragenden Knochen als Stäbe ihres angegebenen Querschnitts, das Gelenk des Stamms, der Wurzelteller und die Massen; diese Liste ist bereits minimal."),
  ],
};

const models = {} as Record<Scale, ScaleModel>;
for (const scale of [50, 100, 200, 500] as Scale[]) {
  const all = clamp(scale, woody(scale));
  const hidden = scale === 500 ? new Set(LIMBS.map((l) => l[0])) : new Set<string>();
  models[scale] = { scale, tubes: all.filter((t) => !hidden.has(t.bone)), hiddenTubes: all.filter((t) => hidden.has(t.bone)), lobes: crown(scale), clusters: [], hiddenClusters: [], outlets: OUTLETS[scale], crownBone: scale === 500 ? "trunk" : undefined, decisions: DECISIONS[scale] };
}

const trunk50 = WOOD_50.find((t) => t.id === "trunk")!;
const L = (id: string, title: [string, string], group: BoneDef["group"], parent: string | null, from: V3, to: V3, limit: number): BoneDef => ({ id, parent, title: tx(...title), group, from, to, limit });
const BONES: BoneDef[] = [
  L("root-plate", ["Root plate and flare", "Wurzelteller und Wurzelanlauf"], "root", null, [0, 0, 0], [0.028, 0.32, 0.01], 0),
  L("trunk", ["Short fluted stem", "Kurzer, spannrückiger Stamm"], "stem", "root-plate", [0.028, 0.32, 0.01], axisAt(trunk50, 1.3), 3),
  ...LIMBS.map(([id, y]) => L(id, id === "limb-7" ? ["Upright centre limb", "Aufrechter Mittelast"] : [`Crooked limb ${id.slice(5)}`, `Knorriger Ast ${id.slice(5)}`], "limb", "trunk", axisAt(trunk50, y), WOOD_50.find((t) => t.id === id)!.points.at(-1)!.p, 12)),
  ...TIPS.map((t) => L(t.id, t.id === "fol-7" ? ["Foliage of the dome top", "Laub des Kuppelgipfels"] : [`Foliage of limb ${t.id.slice(4)}`, `Laub von Ast ${t.id.slice(4)}`], "foliage", `limb-${t.id.slice(4)}`, t.p, t.p, 30)),
  L("fol-core", ["Shaded inner foliage", "Beschattetes Innenlaub"], "foliage", "trunk", axisAt(trunk50, 1.3), axisAt(trunk50, 1.3), 25),
];

const REFS = [
  { id: "thomas-2021", citation: "Thomas, P. A., Leski, T., La Porta, N., Dering, M. and Iszkuło, G. (2021). Biological Flora of Britain and Ireland: Crataegus monogyna. Journal of Ecology 109(1), 572-596. Habit (shrub or small tree to 10-15 m, dense rounded crown), lobed leaves of 2-4 cm, flowering in May, haws of about 1 cm, very hard heavy wood." },
  { id: "wood-handbook-2021", citation: "Forest Products Laboratory (2021). Wood Handbook: Wood as an Engineering Material. General Technical Report FPL-GTR-282, USDA Forest Service. Chapter 5 (mechanical properties of green wood); used as a guide, the species values are authored estimates." },
  { id: "en-1991-1-4", citation: "EN 1991-1-4:2005 Eurocode 1, Actions on structures, Part 1-4: Wind actions. Air density 1.25 kg/m3, drag force F = 0.5 rho cd A v^2, power-law profile." },
  { id: "en-1991-1-3", citation: "EN 1991-1-3:2003 Eurocode 1, Actions on structures, Part 1-3: Snow loads. Used for the order of magnitude of wet-snow accretion on a dense twiggy crown." },
  { id: "vogel-1989", citation: "Vogel, S. (1989). Drag and reconfiguration of broad leaves in high winds. Journal of Experimental Botany 40(8), 941-948." },
  { id: "moore-maguire-2004", citation: "Moore, J. R. and Maguire, D. A. (2004). Natural sway frequencies and damping ratios of trees: concepts, review and synthesis of previous studies. Trees 18, 195-203. Used for the order of magnitude of the first sway frequency." },
  { id: "jones-2014", citation: "Jones, H. G. (2014). Plants and Microclimate, 3rd ed., Cambridge University Press. Leaf energy balance, leaf area index, stomatal and boundary-layer conductance." },
  { id: "monteith-unsworth-2013", citation: "Monteith, J. L. and Unsworth, M. H. (2013). Principles of Environmental Physics, 4th ed., Academic Press. Penman-Monteith leaf transpiration." },
  { id: "hawthorn-crop", citation: "Authored estimate of the blossom and haw mass of a mature open-grown hawthorn (some 5 kg of blossom in May, 10 to 20 kg of haws of about 1 g in autumn), guided by Thomas et al. (2021); not a cited measurement." },
];

const CALM = { "stem-bark": 0.3, "branch-bark": 0.15, "leaf-sun": 0.85, "leaf-shade": 0.08 };
const WINDY = { stem: 0.35, limb: 1.6, branch: 1.6, foliage: 5 } as const;
const GALE = { stem: 1.1, limb: 4.5, branch: 4.5, foliage: 13 } as const;
const outer = (ctx: Ctx): string[] => ctx.foliage.filter((id) => id !== "fol-core");
const crop = (ctx: Ctx, what: "blossom" | "haws", kg: number): Json[] => verticalLoads(ctx, "carried", what, (id) => (what === "blossom" ? tx(`Blossom on foliage mass ${id}`, `Blüten an Laubmasse ${id}`) : tx(`Haws on foliage mass ${id}`, `Mehlfäßchen an Laubmasse ${id}`)), "hawthorn-crop", null, kg, outer(ctx));
export const hawthorn: Species = {
  id: "hawthorn",
  title: tx("Common Hawthorn", "Eingriffeliger Weißdorn"),
  summary: tx("An open-grown common hawthorn of about 50 years, 6 m high: a short, leaning, fluted stem of 1.2 m divides into six crooked ascending limbs and an upright centre limb under a dense, rounded dome of small lobed leaves that sweeps down to 1.2 m; May blossom, autumn haws, wind and wet snow are seasonal poses and loads.", "Ein frei gewachsener Eingriffeliger Weißdorn von etwa 50 Jahren, 6 m hoch: Ein kurzer, geneigter, spannrückiger Stamm von 1,2 m teilt sich in sechs knorrige, aufsteigende Äste und einen aufrechten Mittelast unter einer dichten, runden Kuppel aus kleinen gelappten Blättern, die bis auf 1,2 m herabreicht; Maiblüte, herbstliche Früchte, Wind und Nassschnee sind jahreszeitliche Posen und Lasten."),
  tags: ["tree", "shrub", "deciduous", "broadleaf", "native", "hedgerow", "thorny", "dense-crown", "wind", "transpiration"],
  provenance: { author: "Semio staffage plant collection worker PW-06", method: "handcrafted", created: "2026-11-22", references: REFS, notes: tx("Rig, body and drawings are authored by hand for an open-grown hawthorn of about 50 years in a hedgerow, field corner or park: 6 m high (hawthorns reach 5 to 10 m, rarely 15 m), crown 6.4 m wide and 6 m deep, stem diameter about 0.31 m at 1 m. Thorns (1 to 2.5 cm), single leaves (2 to 4 cm), flowers and haws (about 1 cm) are far below the printable feature and are not modelled; blossom and haws are loads of their seasonal poses. Wood and crop values are authored estimates guided by the cited sources.", "Rig, Körper und Zeichnungen sind von Hand für einen frei gewachsenen Weißdorn von etwa 50 Jahren in einer Hecke, an einem Feldrand oder in einem Park entworfen: 6 m hoch (Weißdorne werden 5 bis 10 m, selten 15 m), Krone 6,4 m breit und 6 m tief, Stammdurchmesser etwa 0,31 m in 1 m Höhe. Dornen (1 bis 2,5 cm), einzelne Blätter (2 bis 4 cm), Blüten und Früchte (etwa 1 cm) liegen weit unter dem druckbaren Merkmal und werden nicht modelliert; Blüten und Früchte sind Lasten ihrer jahreszeitlichen Posen. Holz- und Behangswerte sind Autorenschätzungen, angelehnt an die genannten Quellen.") },
  materials: [
    { id: BARK, title: tx("Hawthorn bark, grey-brown, cracked into small plates", "Weißdornrinde, graubraun, in kleine Schuppen gerissen"), emissivity: 0.95, solarAbsorptivity: 0.7, densityKgM3: 750, conductivityWmK: 0.13, source: "authored-estimate", reference: "wood-handbook-2021" },
    { id: YOUNG, title: tx("Smooth grey bark of limbs and thorny twigs", "Glatte graue Rinde der Äste und dornigen Zweige"), emissivity: 0.95, solarAbsorptivity: 0.66, densityKgM3: 780, conductivityWmK: 0.14, source: "authored-estimate", reference: "wood-handbook-2021" },
    { id: SUN, title: tx("Hawthorn sun leaves (small, glossy, deeply lobed)", "Weißdorn-Sonnenblätter (klein, glänzend, tief gelappt)"), emissivity: 0.97, solarAbsorptivity: 0.52, source: "literature", reference: "jones-2014" },
    { id: SHADE, title: tx("Hawthorn shade leaves (effective)", "Weißdorn-Schattenblätter (wirksam)"), emissivity: 0.97, solarAbsorptivity: 0.25, source: "authored-estimate", reference: "jones-2014" },
    { id: "hawthorn-wood", title: tx("Green hawthorn wood (hard and heavy)", "Frisches Weißdornholz (hart und schwer)"), emissivity: 0.9, solarAbsorptivity: 0.6, densityKgM3: 1050, conductivityWmK: 0.19, elasticModulusPa: 9000000000, strengthPa: 60000000, source: "authored-estimate", reference: "wood-handbook-2021" },
  ],
  bones: BONES, models, woodDensity: 1050, leafMassKgM2: 0.22, barkFraction: 0.07,
  regionOfTube: (t) => (t.group === "limb" || t.group === "branch" ? "branch-bark" : "stem-bark"),
  regionOfLobe: (l) => (l.group === "shade" ? "leaf-shade" : "leaf-sun"),
  regions: [
    { id: "stem-bark", title: tx("Grey-brown bark of root flare and stem", "Graubraune Rinde von Wurzelanlauf und Stamm"), kind: "bark", model: "passive", material: BARK, areaM2: 0, characteristicLengthM: 0.32, geometry: "cylinder", skyViewFactor: 0.25, layers: [], heatCapacityJM2K: 70000, source: "authored-estimate", reference: "wood-handbook-2021" },
    { id: "branch-bark", title: tx("Bark of the crooked limbs and side branches", "Rinde der knorrigen Äste und Seitenzweige"), kind: "bark", model: "passive", material: YOUNG, areaM2: 0, characteristicLengthM: 0.09, geometry: "cylinder", skyViewFactor: 0.3, layers: [], heatCapacityJM2K: 35000, source: "authored-estimate", reference: "wood-handbook-2021" },
    { id: "leaf-sun", title: tx("Sun leaves of the outer dome", "Sonnenblätter der äußeren Kuppel"), kind: "foliage", model: "leaf", material: SUN, areaM2: 72, areaBasis: "exchange", characteristicLengthM: 0.025, geometry: "vertical-plate", skyViewFactor: 0.7, layers: [], heatCapacityJM2K: 750, leaf: { stomatalConductanceMaxMS: 0.0045, leafAreaIndex: 2.4, photosynthesis: "c3" }, source: "authored-estimate", reference: "jones-2014" },
    { id: "leaf-shade", title: tx("Shade leaves of the dense interior", "Schattenblätter des dichten Inneren"), kind: "foliage", model: "leaf", material: SHADE, hidden: tx("The shade leaves lie inside the crown, behind the sun leaves from every side, so they are drawn by no shape; their lobes are thermal solids of their own inside the body.", "Die Schattenblätter liegen im Inneren der Krone, von jeder Seite hinter den Sonnenblättern, und werden daher von keiner Form gezeichnet; ihre Lappen sind eigene Thermikkörper im Körper."), areaM2: 58, areaBasis: "exchange", characteristicLengthM: 0.03, geometry: "vertical-plate", skyViewFactor: 0.25, layers: [], heatCapacityJM2K: 600, leaf: { stomatalConductanceMaxMS: 0.0025, leafAreaIndex: 1.9, photosynthesis: "c3" }, source: "authored-estimate", reference: "jones-2014" },
  ],
  emitters: [
    { id: "transpiration-sun", kind: "transpiration", title: tx("Transpiration of the sun leaves", "Transpiration der Sonnenblätter"), anchor: "tp-sun-1", anchors: ["tp-sun-1", "tp-sun-2", "tp-sun-3", "tp-sun-4", "tp-sun-5", "tp-sun-6", "tp-sun-7"], directionDeg: 0, diameterM: 1.4, region: "leaf-sun", source: "authored-estimate", reference: "monteith-unsworth-2013" },
    { id: "transpiration-shade", kind: "transpiration", title: tx("Transpiration of the shade leaves", "Transpiration der Schattenblätter"), anchor: "tp-shade-1", anchors: ["tp-shade-1", "tp-shade-2", "tp-shade-3", "tp-shade-4", "tp-shade-5"], directionDeg: 0, diameterM: 1.2, region: "leaf-shade", source: "authored-estimate", reference: "monteith-unsworth-2013" },
  ],
  environment: { airTempC: 22, meanRadiantTempC: 25, relativeHumidity: 0.6, windMS: 1.5, windDirectionDeg: 0, solarWM2: 700, pressurePa: 101325, diffuseFraction: 0.15 },
  thermalAssumptions: [
    tx("Leaf regions use one-sided leaf area; the leaf area index is that area over the crown ground projection of about 30 m2 (a dense crown, 4.3 in total). The maximum stomatal conductance is an authored estimate for a hedgerow broadleaf, guided by Jones (2014) (typical range 0.003 to 0.008 m/s); no published species maximum is claimed. Photosynthesis type: C3. Aperture and night closure are the engine's business, not authored here.", "Blattregionen verwenden die einseitige Blattfläche; der Blattflächenindex ist diese Fläche bezogen auf die Kronenprojektion von etwa 30 m2 (eine dichte Krone, insgesamt 4,3). Die maximale Spaltöffnungsleitfähigkeit ist eine Autorenschätzung für ein Heckengehölz, angelehnt an Jones (2014) (übliche Spanne 0,003 bis 0,008 m/s); ein veröffentlichter Artwert wird nicht behauptet. Photosynthesetyp: C3. Öffnung und nächtliches Schließen sind Sache der Engine und werden hier nicht vorgegeben."),
    tx("Bark is passive. Its areal heat capacity is the volumetric heat capacity of green wood (about 2.9 MJ per m3 and K for this heavy wood) times an effective depth of 1.2 to 2.4 cm, an authored estimate guided by the Wood Handbook; the bark areas are the mantles of the modelled stem and limbs. No surface temperature is authored.", "Rinde ist passiv. Ihre flächenbezogene Wärmekapazität ist die volumetrische Wärmekapazität von frischem Holz (etwa 2,9 MJ je m3 und K bei diesem schweren Holz) mal einer wirksamen Tiefe von 1,2 bis 2,4 cm, eine Autorenschätzung angelehnt an das Wood Handbook; die Rindenflächen sind die Mäntel des modellierten Stamms und der Äste. Es werden keine Oberflächentemperaturen vorgegeben."),
    tx("Transpiration emitters release the latent flux of their foliage region as vapour, spread over at least five anchors on the dome (the shade anchors on the shade lobes inside (thermal solids only)); the direction is the vertical measured in the frame of the first anchor's bone. Blossom, haws, leaf colour and leaf fall are not geometric: a pose cannot hide shapes, so seasonal poses change the thermal binding and the loads only.", "Transpirationsemitter geben den latenten Strom ihrer Laubregion als Dampf ab, verteilt auf mindestens fünf Anker an der Kuppel (die Schattenanker an den inneren Schattenlappen (nur als Thermikkörper)); die Richtung ist die Senkrechte im Bezugssystem des Knochens des ersten Ankers. Blüten, Früchte, Laubfärbung und Laubfall sind nicht geometrisch: Eine Pose kann keine Formen ausblenden, daher ändern jahreszeitliche Posen nur die thermische Bindung und die Lasten."),
  ],
  structureAssumptions: [
    tx("The tree is one rigid body per evaluated pose, fixed at the root plate; wind drag acts at the centre of each foliage mass and on the stem as a line load, gravity on every bone mass, blossom, haws and snow as vertical loads on the foliage masses. The base moment follows from statics at the posed geometry; root-soil failure and dynamic amplification are not modelled. Pose lean angles are authored estimates that grow with the drag force.", "Der Baum ist je ausgewerteter Pose ein starrer Körper, am Wurzelteller eingespannt; Winddruck wirkt im Schwerpunkt jeder Laubmasse und am Stamm als Linienlast, Schwerkraft auf jede Knochenmasse, Blüten, Früchte und Schnee als senkrechte Lasten auf die Laubmassen. Das Fußmoment folgt aus der Statik der gezeigten Geometrie; Wurzel-Boden-Versagen und dynamische Überhöhung werden nicht modelliert. Die Neigungswinkel der Posen sind Autorenschätzungen, die mit der Widerstandskraft wachsen."),
    tx("Sections are solid circles of green wood under a bark of 7 % of the radius (authored-estimate properties guided by the Wood Handbook for a hard, heavy wood: 1050 kg/m3, E 9 GPa, bending strength 60 MPa); the fluting of the stem is not represented. The rig is planar, so limbs that run out of the side-elevation plane carry their lateral position as an offset and bend in that plane only.", "Querschnitte sind Vollkreise aus frischem Holz unter einer Rinde von 7 % des Radius (Autorenschätzungen angelehnt an das Wood Handbook für ein hartes, schweres Holz: 1050 kg/m3, E 9 GPa, Biegefestigkeit 60 MPa); die Spannrückigkeit des Stamms wird nicht abgebildet. Das Rig ist eben, daher tragen Äste, die aus der Ebene der Seitenansicht laufen, ihre seitliche Lage als Versatz und biegen sich nur in dieser Ebene."),
  ],
  poses: (ctx) => [
    { id: "summer-calm", title: tx("Summer, calm", "Sommer, windstill"), description: tx("Light air of 1.5 m/s at 10 m in July: the dense dome stands at rest; drag per foliage mass is a few newtons (cd 0.45), the green haws weigh about 4 kg.", "Leiser Zug von 1,5 m/s in 10 m Höhe im Juli: Die dichte Kuppel steht in Ruhe; der Widerstand je Laubmasse beträgt wenige Newton (cd 0,45), die grünen Früchte wiegen etwa 4 kg."), bones: {}, ground: "authored", contacts: [], thermal: binding(hawthorn, CALM, { "transpiration-sun": 0.85, "transpiration-shade": 0.55 }), loads: [...windLoads(ctx, 1.5, 0.45), ...crop(ctx, "haws", 4)] } as Json,
    { id: "may-blossom", title: tx("May blossom", "Maiblüte"), description: tx("A sunny May day in full flower: about 5 kg of white blossom lie on the outer dome (not drawn, below the printable feature), the young leaves transpire at 80 %, light air of 1.5 m/s.", "Ein sonniger Maitag in voller Blüte: Etwa 5 kg weiße Blüten liegen auf der äußeren Kuppel (nicht gezeichnet, unter dem druckbaren Merkmal), die jungen Blätter transpirieren zu 80 %, leiser Zug von 1,5 m/s."), bones: {}, ground: "authored", contacts: [], thermal: binding(hawthorn, { "stem-bark": 0.32, "branch-bark": 0.18, "leaf-sun": 0.9, "leaf-shade": 0.1 }, { "transpiration-sun": 0.8, "transpiration-shade": 0.5 }), loads: [...windLoads(ctx, 1.5, 0.45), ...crop(ctx, "blossom", 5)] } as Json,
    { id: "fresh-breeze", title: tx("Fresh breeze", "Frische Brise"), description: tx("Beaufort 5, 8 m/s at 10 m, cd 0.35 on the front area of each foliage mass; the short stem barely moves (0.35 degrees), the limbs lean by 1.6 and the leafy masses by 5 degrees downwind.", "Beaufort 5, 8 m/s in 10 m Höhe, cd 0,35 auf die Anströmfläche jeder Laubmasse; der kurze Stamm bewegt sich kaum (0,35 Grad), die Äste neigen sich um 1,6 und die Laubmassen um 5 Grad mit dem Wind."), bones: offsets(ctx, WINDY), ground: "authored", contacts: [], thermal: binding(hawthorn, CALM, { "transpiration-sun": 1, "transpiration-shade": 0.65 }), loads: [...windLoads(ctx, 8, 0.35), ...crop(ctx, "haws", 4)] } as Json,
    { id: "gale", title: tx("Gale with rain", "Sturm mit Regen"), description: tx("Beaufort 9, 22 m/s at 10 m with rain, cd 0.22 after the reconfiguration of the leaves (Vogel 1989): the limbs lean by 4.5 and the masses by 13 degrees; wet bark and leaves.", "Beaufort 9, 22 m/s in 10 m Höhe mit Regen, cd 0,22 nach der Rekonfiguration der Blätter (Vogel 1989): Die Äste neigen sich um 4,5 und die Massen um 13 Grad; nasse Rinde und Blätter."), bones: offsets(ctx, GALE), ground: "authored", contacts: [], thermal: binding(hawthorn, { "stem-bark": 0.04, "branch-bark": 0.02, "leaf-sun": 0.12, "leaf-shade": 0.01 }, { "transpiration-sun": 0.4, "transpiration-shade": 0.25 }, 0.6), loads: [...windLoads(ctx, 22, 0.22), ...crop(ctx, "haws", 4)] } as Json,
    { id: "autumn-haws", title: tx("Autumn, laden with haws", "Herbst, voller Früchte"), description: tx("A calm October day with about 15 kg of dark red haws on the outer dome (not drawn: single haws of 1 cm): the limbs bow down by 2 degrees, the masses by 4; the ageing leaves transpire at half strength.", "Ein windstiller Oktobertag mit etwa 15 kg dunkelroter Früchte auf der äußeren Kuppel (nicht gezeichnet: einzelne Früchte von 1 cm): Die Äste neigen sich um 2 Grad nach unten, die Massen um 4; die alternden Blätter transpirieren mit halber Stärke."), bones: offsets(ctx, {}, { limb: 2, branch: 2, foliage: 4 }), ground: "authored", contacts: [], thermal: binding(hawthorn, { "stem-bark": 0.22, "branch-bark": 0.1, "leaf-sun": 0.65, "leaf-shade": 0.06 }, { "transpiration-sun": 0.5, "transpiration-shade": 0.3 }), loads: [...windLoads(ctx, 1.5, 0.45), ...crop(ctx, "haws", 15)] } as Json,
    { id: "wet-snow", title: tx("Wet snow on the dense dome", "Nassschnee auf der dichten Kuppel"), description: tx("An early wet snowfall on the still leafy, dense dome: about 10 kg/m2 on the plan area of each foliage mass (order of magnitude after EN 1991-1-3) bows the limbs by 5 degrees and the masses by 10 degrees; wet, cold, overcast, the stomata nearly closed.", "Ein früher Nassschneefall auf die noch belaubte, dichte Kuppel: etwa 10 kg/m2 auf der Grundrissfläche jeder Laubmasse (Größenordnung nach EN 1991-1-3) biegen die Äste um 5 Grad und die Massen um 10 Grad nach unten; nass, kalt, bedeckt, die Spaltöffnungen fast geschlossen."), bones: offsets(ctx, {}, { limb: 5, branch: 5, foliage: 10 }), ground: "authored", contacts: [], thermal: binding(hawthorn, { "stem-bark": 0.02, "branch-bark": 0.01, "leaf-sun": 0.05, "leaf-shade": 0.01 }, { "transpiration-sun": 0.08, "transpiration-shade": 0.05 }, 0.9), loads: [...verticalLoads(ctx, "snow", "snow", (id) => tx(`Wet snow on foliage mass ${id}`, `Nassschnee auf Laubmasse ${id}`), "en-1991-1-3", 10), ...crop(ctx, "haws", 8)] } as Json,
  ],
  clips: (ctx) => {
    const windIds = windLoads(ctx, 8, 0.35).map((l) => l.id);
    return [
      { id: "wind-sway", title: tx("Wind sway", "Windschwingung"), description: tx("A deterministic 1.7 s sway cycle (first mode of a short, stiff 6 m tree of about 0.6 Hz, an authored estimate within the range reviewed by Moore and Maguire 2004) about the fresh-breeze lean, plus and minus 30 % of it; the limbs lag the stem by 6 % and the dense masses by 10 % of the period.", "Ein deterministischer Schwingungszyklus von 1,7 s (erste Eigenform eines kurzen, steifen 6-m-Baums von etwa 0,6 Hz, eine Autorenschätzung im von Moore und Maguire 2004 zusammengestellten Bereich) um die Neigung der frischen Brise, plus und minus 30 % davon; die Äste folgen dem Stamm mit 6 %, die dichten Massen mit 10 % der Periode."), durationS: 1.7, fps: 30, loop: true, rootMotion: "none", keys: swayKeys(ctx, "fresh-breeze", WINDY, 0.3, { limb: 0.06, branch: 0.06, foliage: 0.1 }, 1.7), contacts: [], phases: [{ id: "sway", fromS: 0, toS: 1.7, title: tx("Sway cycle", "Schwingungszyklus") }], events: [{ t: 0, kind: "motion", title: tx("Cycle start at the mean lean", "Zyklusbeginn in mittlerer Neigung") }], tracks: [{ target: "emitter:transpiration-sun", ease: "smooth", keys: [{ t: 0, v: 1 }, { t: 0.85, v: 0.93 }, { t: 1.7, v: 1 }] }, { target: "emitter:transpiration-shade", ease: "smooth", keys: [{ t: 0, v: 0.65 }, { t: 0.85, v: 0.62 }, { t: 1.7, v: 0.65 }] }] },
      { id: "gust", title: tx("Gust", "Windböe"), description: tx("A single 8 s gust: the wind-load tracks rise from 5 % to 2.75 times the fresh-breeze load at 2.5 s (EN 1991-1-4 peak velocity pressure with a turbulence intensity of 0.25) and decay; the dome leans with the load to the power 0.8, at most 70 % of the gale lean, and springs back.", "Eine einzelne Böe über 8 s: Die Windlast-Spuren steigen bei 2,5 s von 5 % auf das 2,75-fache der Last der frischen Brise (Spitzengeschwindigkeitsdruck nach EN 1991-1-4 mit einer Turbulenzintensität von 0,25) und klingen ab; die Kuppel neigt sich mit der Last hoch 0,8, höchstens 70 % der Sturmneigung, und federt zurück."), durationS: 8, fps: 30, loop: false, rootMotion: "none", keys: gustKeys(ctx, "fresh-breeze", WINDY, GALE), contacts: [], phases: [{ id: "rise", fromS: 0, toS: 2.5, title: tx("Gust front", "Böenfront") }, { id: "peak", fromS: 2.5, toS: 3.5, title: tx("Peak", "Spitze") }, { id: "decay", fromS: 3.5, toS: 8, title: tx("Decay and recovery", "Abklingen und Erholung") }], events: [{ t: 2.5, kind: "load-on", title: tx("Peak gust load", "Böenspitze") }, { t: 5, kind: "load-off", title: tx("Load below the mean again", "Last wieder unter dem Mittelwert") }], tracks: windIds.map((id) => ({ target: `load:${id}`, ease: "smooth", keys: GUST.map(([t, v]) => ({ t, v })) })) },
      diurnalClip(hawthorn, "summer-calm", CALM, { "transpiration-sun": [[0, 0.03], [5, 0.05], [7, 0.3], [9, 0.7], [12, 1], [15, 0.9], [18, 0.35], [20, 0.08], [24, 0.03]], "transpiration-shade": [[0, 0.02], [5, 0.03], [7, 0.18], [9, 0.42], [12, 0.6], [15, 0.54], [18, 0.21], [20, 0.05], [24, 0.02]] }),
    ];
  },
  presentation: { defaultPose: "summer-calm", defaultClip: "wind-sway", defaultScale: 100 },
  drawing: {
    tones: TONE,
    arcScales: [100],
    glyph: (c, k) => {
      const a = (k * 61 + 10) * (Math.PI / 180);
      const pts: [number, number][] = [[0, -0.03], [0.012, -0.012], [0.026, -0.016], [0.02, 0.002], [0.03, 0.014], [0.012, 0.016], [0, 0.034], [-0.012, 0.016], [-0.03, 0.014], [-0.02, 0.002], [-0.026, -0.016], [-0.012, -0.012]];
      return { closed: true, rings: [pts.map(([x, y]) => ({ x: c.x + (x * Math.cos(a) - y * Math.sin(a)) * 1.4, y: c.y + (x * Math.sin(a) + y * Math.cos(a)) * 1.4 }))] };
    },
    bark: (view: View, t: Tube, scale: Scale) => barkMarks(view, t, scale === 50 ? 0.22 : 0.45, (n, r) => {
      const off = [-0.42, 0.1, 0.48, -0.15][n % 4]! * r;
      return scale === 50 ? [[off, -0.08], [off + 0.03 * r, -0.03], [off - 0.02 * r, 0.03], [off + 0.01 * r, 0.08]] : [[off, -0.1], [off, 0.1]];
    }),
    decisions: {
      50: {
        silhouette: () => tx("At 1:50 (1 m is 20 mm) the three drawings are the true views of the 1:50 body: root flare with surface roots, the short leaning stem with its vertical fissures, six crooked limbs and the upright centre limb running into a dense, low, rounded dome drawn as eight foliage masses whose outlines follow their many small lobes, with lobed leaf marks; the skirt reaches down to 1.2 m. Plan: the scalloped dome and the stem as a dot.", "Bei 1:50 (1 m sind 20 mm) sind die drei Zeichnungen die wahren Ansichten des 1:50-Körpers: Wurzelanlauf mit Oberflächenwurzeln, der kurze, geneigte Stamm mit senkrechten Rissen, sechs knorrige Äste und der aufrechte Mittelast, die in eine dichte, tiefe, runde Kuppel aus acht Laubmassen laufen, deren Umrisse ihren vielen kleinen Lappen folgen, mit gelappten Blattzeichen; der Saum reicht bis auf 1,2 m herab. Draufsicht: die gewellte Kuppel und der Stamm als Punkt."),
        retained: tx("Retained: every woody member of the rig with fourteen side branches, every foliage mass, the fissures of the stem.", "Beibehalten: jedes Holzglied des Rigs mit vierzehn Seitenzweigen, jede Laubmasse, die Risse des Stamms."),
        merged: tx("Single leaves and the dense twig mass are merged into lobed mass outlines; the bark plates into short wavy fissure strokes.", "Einzelne Blätter und die dichte Zweigmasse gehen in gelappten Massenumrissen auf; die Rindenschuppen in kurzen welligen Rissstrichen."),
        omitted: tx("Omitted: thorns (1 to 2.5 cm), single leaves (2 to 4 cm), blossom and haws (about 1 cm), twigs thinner than 1 cm and the fluting of the stem.", "Weggelassen: Dornen (1 bis 2,5 cm), einzelne Blätter (2 bis 4 cm), Blüten und Früchte (etwa 1 cm), Zweige unter 1 cm und die Spannrückigkeit des Stamms."),
        thinElements: tx("Twig ends are kept at 1.15 times the printable minimum (11.5 mm real); leaf marks and fissures are 0.13 mm texture strokes kept 0.35 mm inside every outline.", "Zweigenden bleiben beim 1,15-fachen des druckbaren Minimums (11,5 mm wirklich); Blattzeichen und Risse sind Texturstriche von 0,13 mm, mindestens 0,35 mm innerhalb jedes Umrisses."),
        hatching: tx("No hatching: the shaded inner foliage is a hidden region inside the crown and is not drawn; bark and leaves are drawn with strokes.", "Keine Schraffur: Das beschattete Innenlaub ist eine verdeckte Region im Inneren der Krone und wird nicht gezeichnet; Rinde und Blätter werden mit Strichen gezeichnet."),
        styles: tx("Light line: 0.25 mm outlines, 0.13 mm texture. Dark solid: tonal fills separate the dark stem, the limbs, the foliage.", "Helle Linie: 0,25 mm Umrisse, 0,13 mm Textur. Dunkle Fläche: getönte Füllungen trennen den dunklen Stamm, die Äste, das Laub."),
        overlays: tx("Each thermal region has its own shapes; leaf marks and fissures are strokes without a region and follow their bone.", "Jede thermische Region hat eigene Formen; Blattzeichen und Risse sind Striche ohne Region und folgen ihrem Knochen."),
      },
      100: {
        silhouette: () => tx("At 1:100 (1 m is 10 mm) the drawings are the true views of the 1:100 body: flared root, stem, the seven limbs with six side branches and the dense dome as eight masses of broader lobes with leaf-cluster arcs.", "Bei 1:100 (1 m sind 10 mm) sind die Zeichnungen die wahren Ansichten des 1:100-Körpers: ausgestellter Wurzelanlauf, Stamm, die sieben Äste mit sechs Seitenzweigen und die dichte Kuppel aus acht Massen breiterer Lappen mit Blattbüschelbögen."),
        retained: tx("Retained: eight foliage masses, root flare with roots, stem, all limbs, six side branches, a few fissures on the stem.", "Beibehalten: acht Laubmassen, Wurzelanlauf mit Wurzeln, Stamm, alle Äste, sechs Seitenzweige, einige Risse am Stamm."),
        merged: tx("Leaves and small lobes are merged into broader lobed outlines.", "Blätter und kleine Lappen gehen in breiteren gelappten Umrissen auf."),
        omitted: tx("Omitted: leaf marks, eight weaker side branches, thorns, blossom and haws.", "Weggelassen: Blattzeichen, acht schwächere Seitenzweige, Dornen, Blüten und Früchte."),
        thinElements: tx("Wood thinner than 23 mm is raised to it (the printable 0.2 mm plus 15 %); arcs and fissures use the 0.13 mm detail pen.", "Holz unter 23 mm wird darauf angehoben (die druckbaren 0,2 mm plus 15 %); Bögen und Risse nutzen den Detailstift von 0,13 mm."),
        hatching: tx("No hatching: the shaded inner foliage is hidden inside the crown; arcs and bark strokes carry the texture.", "Keine Schraffur: Das beschattete Innenlaub liegt verdeckt im Inneren der Krone; Bögen und Rindenstriche tragen die Textur."),
        styles: tx("Light line: 0.2 mm outlines, 0.13 mm detail. Dark solid: tonal masses, detail strokes stay visible on them.", "Helle Linie: 0,2 mm Umrisse, 0,13 mm Detail. Dunkle Fläche: getönte Massen, Detailstriche bleiben darauf sichtbar."),
        overlays: tx("Every region keeps its shapes.", "Jede Region behält ihre Formen."),
      },
      200: {
        silhouette: () => tx("At 1:200 (1 m is 5 mm) the drawings are the true views of the 1:200 body: the short stem with its flare and the seven limbs under a dense, low dome of eight scalloped foliage masses.", "Bei 1:200 (1 m sind 5 mm) sind die Zeichnungen die wahren Ansichten des 1:200-Körpers: der kurze Stamm mit Wurzelanlauf und die sieben Äste unter einer dichten, tiefen Kuppel aus acht gewellten Laubmassen."),
        retained: tx("Retained: root flare, stem, the seven limbs at no less than 46 mm and the foliage masses.", "Beibehalten: Wurzelanlauf, Stamm, die sieben Äste mit mindestens 46 mm und die Laubmassen."),
        merged: tx("Side branches, leaves and small lobes are merged into the large scalloped masses.", "Seitenzweige, Blätter und kleine Lappen gehen in den großen gewellten Massen auf."),
        omitted: tx("Omitted: fissures, leaf marks, arcs, roots and side branches.", "Weggelassen: Risse, Blattzeichen, Bögen, Wurzeln und Seitenzweige."),
        thinElements: tx("Limb tips are raised to 46 mm (0.23 mm on paper); they run into the masses.", "Astspitzen werden auf 46 mm (0,23 mm auf dem Papier) angehoben; sie laufen in die Massen."),
        hatching: tx("No hatching and no internal edge.", "Keine Schraffur und keine innere Kante."),
        styles: tx("Light line: outlines 0.18 mm. Dark solid: the foliage in two tones, wood darker.", "Helle Linie: Umrisse 0,18 mm. Dunkle Fläche: das Laub in zwei Tönen, Holz dunkler."),
        overlays: tx("Every thermal region keeps a shape: stem, limbs, the foliage masses.", "Jede thermische Region behält eine Form: Stamm, Äste, die Laubmassen."),
      },
      500: {
        silhouette: () => tx("At 1:500 (1 m is 2 mm) the hawthorn is one dense rounded dome, 13 mm wide and 12 mm high, reaching down to 1.2 m over a short dark stem with its flare; the drawings are the true views of the 1:500 body. Plan: one scalloped dome outline; the stem is hidden.", "Bei 1:500 (1 m sind 2 mm) ist der Weißdorn eine dichte runde Kuppel von 13 mm Breite und 12 mm Höhe, die bis auf 1,2 m über einem kurzen dunklen Stamm mit Wurzelanlauf herabreicht; die Zeichnungen sind die wahren Ansichten des 1:500-Körpers. Draufsicht: ein gewellter Kuppelumriss; der Stamm ist verdeckt."),
        retained: tx("Retained: stem with root flare and the dome outline; the stem runs on into the dome for a clean join.", "Beibehalten: Stamm mit Wurzelanlauf und der Kuppelumriss; der Stamm läuft für einen sauberen Anschluss in die Kuppel weiter."),
        merged: tx("Merged into the dome shape: the foliage and the limbs (as merged regions).", "In der Kuppelform zusammengefasst: das Laub und die Äste (als zusammengefasste Regionen)."),
        omitted: tx("Omitted: single limbs, fissures; no internal edge.", "Weggelassen: einzelne Äste, Risse; keine innere Kante."),
        thinElements: tx("The stem keeps its real width (0.7 mm); limbs are not thickened but merged into the filled dome.", "Der Stamm behält seine wirkliche Breite (0,7 mm); Äste werden nicht verdickt, sondern in die gefüllte Kuppel aufgenommen."),
        hatching: tx("No hatching; the pen is 0.15 mm for outlines and 0.25 mm for the ground line.", "Keine Schraffur; der Stift ist 0,15 mm für Umrisse und 0,25 mm für die Bodenlinie."),
        styles: tx("Light line: closed outlines only. Dark solid: the dome reads as one dark mass over the stem.", "Helle Linie: nur geschlossene Umrisse. Dunkle Fläche: Die Kuppel liest sich als eine dunkle Masse über dem Stamm."),
        overlays: tx("Overlays map onto: stem-bark (stem) and, merged in the dome shape, leaf-sun, leaf-shade and branch-bark; merged shapes show the area-weighted value of their regions, labelled as merged.", "Überlagerungen bilden ab auf: stem-bark (Stamm) und, in der Kuppelform zusammengefasst, leaf-sun, leaf-shade und branch-bark; zusammengefasste Formen zeigen den flächengewichteten Wert ihrer Regionen, als zusammengefasst gekennzeichnet."),
      },
    },
  },
  symbol: { id: "hawthorn-symbol", title: tx("Plan symbol with true crown extent", "Grundrisssymbol mit wahrer Kronenausdehnung"), description: tx("A conventional plan sign of the hawthorn for site, hedgerow and planting plans: the true outline of the dome and the stem section. It is a separate symbol and is never used silently for a pictorial scale.", "Ein konventionelles Grundrisszeichen des Weißdorns für Lage-, Hecken- und Pflanzpläne: der wahre Umriss der Kuppel und der Stammquerschnitt. Es ist ein eigenes Symbol und wird nie stillschweigend für einen Bildmaßstab verwendet.") },
  distinction: tx("A dense small tree on a short leaning stem: six crooked limbs and an upright centre limb from one head under a low dome of small lobed leaves that reaches down to 1.2 m, with blossom and haws as seasonal loads of the heaviest wood of the collection.", "Ein dichter kleiner Baum auf kurzem, geneigtem Stamm: sechs knorrige Äste und ein aufrechter Mittelast aus einem Kopf unter einer tiefen Kuppel aus kleinen gelappten Blättern, die bis auf 1,2 m herabreicht, mit Blüten und Früchten als jahreszeitlichen Lasten des schwersten Holzes der Sammlung."),
  anchorsTitle: (em, n) => (em === "transpiration-sun" ? tx(`Transpiration of the sun leaves, anchor ${n}`, `Transpiration der Sonnenblätter, Anker ${n}`) : tx(`Transpiration of the shade leaves, anchor ${n}`, `Transpiration der Schattenblätter, Anker ${n}`)),
};

const barkScale: Record<string, number> = { "stem-bark": 0.95, "branch-bark": 0.95 };
for (const r of hawthorn.regions) if (barkScale[r.id] !== undefined) r.areaM2 = r4(models[50].tubes.filter((t) => hawthorn.regionOfTube(t) === r.id).reduce((s, t) => s + tubeArea(t), 0) * barkScale[r.id]!);
export default hawthorn;
if (import.meta.main) {
  const result = build(hawthorn);
  const path = write(hawthorn, result, process.argv.includes("--catalog") ? "catalog" : "candidate");
  console.log(result.report.join("\n"));
  console.log(path);
}
