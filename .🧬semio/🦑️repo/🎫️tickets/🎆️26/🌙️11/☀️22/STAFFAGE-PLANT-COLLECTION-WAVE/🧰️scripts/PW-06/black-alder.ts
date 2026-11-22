/** 🌊️ Black alder (Alnus glutinosa): a riverbank tree of about 45 years, 20 m high, with one straight, dark, fissured stem running through to the top (excurrent), a flared base with surface roots at the waterline, thirteen short limbs in a spiral that are near horizontal below and ascend above, and a conical-ovoid crown of rounded, notched leaves. Every station, limb, lobe ring and outlet is stated here per scale in world metres (x ahead, y up, z to the left).
 * Usage: `repo tools exec bun -- black-alder.ts [--catalog]` (without `--catalog` the candidate goes to `🗑️generated/PW-06/black-alder/🔣️.json`).
 * @see ./plant-kit.ts
 * @see https://doi.org/10.2307/2257067 — McVean 1953, Biological Flora of the British Isles: Alnus glutinosa */
import { axisAt, barkMarks, binding, branch, build, clamp, diurnalClip, freeLobe, GUST, gustKeys, offsets, r4, ringLobes, stemTube, swayKeys, tubeArea, tx, verticalLoads, windLoads, write, along, type BoneDef, type Ctx, type Json, type Lobe, type RingDef, type Scale, type ScaleModel, type Species, type Tube, type V3, type View } from "./plant-kit.ts";

const BARK = "alder-bark", YOUNG = "alder-bark-young", SUN = "alder-leaf-sun", SHADE = "alder-leaf-shade";
const TONE = { root: 0.22, stem: 0.28, limb: 0.36 };
const STEMS = ["root-plate", "stem-1", "stem-2", "stem-3", "leader"] as const;

type StemRows = [number, number, number, number][];
const STEM: Record<Scale, Record<string, StemRows>> = {
  50: {
    "root-plate": [[0, 0.44, 0, 0], [0.1, 0.34, 0, 0], [0.25, 0.27, 0, 0], [0.42, 0.235, 0, 0], [0.56, 0.226, 0, 0]],
    "stem-1": [[0.45, 0.228, 0, 0], [1.3, 0.212, 0.01, 0.004], [2.4, 0.2, 0.018, 0.01], [3.6, 0.19, 0.02, 0.012], [4.6, 0.181, 0.014, 0.01], [5.6, 0.172, 0.006, 0.006]],
    "stem-2": [[5.45, 0.173, 0.007, 0.006], [6.8, 0.161, -0.004, 0], [8.2, 0.148, -0.012, -0.008], [9.6, 0.133, -0.01, -0.012], [11.1, 0.117, 0, -0.008]],
    "stem-3": [[10.95, 0.118, 0, -0.008], [12.3, 0.102, 0.008, -0.002], [13.8, 0.085, 0.012, 0.004], [15.2, 0.069, 0.006, 0.006], [16.1, 0.06, 0, 0.004]],
    leader: [[15.95, 0.061, 0, 0.004], [17.2, 0.05, -0.006, 0], [18.4, 0.038, -0.004, -0.004], [19.3, 0.028, 0, -0.004]],
  },
  100: {
    "root-plate": [[0, 0.43, 0, 0], [0.16, 0.3, 0, 0], [0.56, 0.226, 0, 0]],
    "stem-1": [[0.45, 0.228, 0, 0], [2.4, 0.2, 0.018, 0.01], [5.6, 0.172, 0.006, 0.006]],
    "stem-2": [[5.45, 0.173, 0.007, 0.006], [8.2, 0.148, -0.012, -0.008], [11.1, 0.117, 0, -0.008]],
    "stem-3": [[10.95, 0.118, 0, -0.008], [13.8, 0.085, 0.012, 0.004], [16.1, 0.06, 0, 0.004]],
    leader: [[15.95, 0.061, 0, 0.004], [19.3, 0.03, 0, -0.004]],
  },
  200: {
    "root-plate": [[0, 0.41, 0, 0], [0.56, 0.226, 0, 0]],
    "stem-1": [[0.45, 0.228, 0, 0], [5.6, 0.172, 0, 0]],
    "stem-2": [[5.45, 0.173, 0, 0], [11.1, 0.117, 0, 0]],
    "stem-3": [[10.95, 0.118, 0, 0], [16.1, 0.06, 0, 0]],
    leader: [[15.95, 0.061, 0, 0], [19.3, 0.03, 0, 0]],
  },
  500: {
    "root-plate": [[0, 0.4, 0, 0], [0.56, 0.23, 0, 0]],
    "stem-1": [[0.45, 0.23, 0, 0], [5.6, 0.175, 0, 0]],
    "stem-2": [[5.45, 0.175, 0, 0], [11.1, 0.12, 0, 0]],
    "stem-3": [[10.95, 0.12, 0, 0], [16.1, 0.07, 0, 0]],
    leader: [[15.95, 0.07, 0, 0], [19.3, 0.06, 0, 0]],
  },
};

/** 🌿️ The limbs in a spiral up the stem: [id, parent, y, azimuth, elevation at the stem and at the tip, length, r0, r1, bend]; low limbs near horizontal, high limbs ascending. */
type LimbRow = [id: string, parent: string, y: number, az: number, e0: number, e1: number, L: number, r0: number, r1: number, bend: number];
const LIMBS: LimbRow[] = [
  ["limb-1", "stem-1", 4.0, 20, 28, 10, 2.9, 0.07, 0.022, 6],
  ["limb-2", "stem-1", 4.6, 160, 30, 12, 3.1, 0.072, 0.022, -6],
  ["limb-3", "stem-1", 5.3, 280, 32, 14, 3.2, 0.072, 0.022, 6],
  ["limb-4", "stem-2", 6.2, 50, 34, 16, 3.3, 0.07, 0.021, -6],
  ["limb-5", "stem-2", 7.1, 190, 36, 18, 3.3, 0.068, 0.021, 6],
  ["limb-6", "stem-2", 8.0, 310, 38, 20, 3.3, 0.064, 0.02, -6],
  ["limb-7", "stem-2", 9.0, 85, 40, 24, 3.2, 0.06, 0.019, 6],
  ["limb-8", "stem-2", 10.0, 225, 42, 26, 3.0, 0.056, 0.018, -6],
  ["limb-9", "stem-3", 11.2, 345, 44, 30, 2.7, 0.05, 0.017, 6],
  ["limb-10", "stem-3", 12.5, 120, 46, 32, 2.4, 0.045, 0.016, -6],
  ["limb-11", "stem-3", 13.8, 255, 48, 36, 2.1, 0.04, 0.015, 6],
  ["limb-12", "leader", 16.2, 30, 52, 40, 1.6, 0.032, 0.013, -6],
  ["limb-13", "leader", 17.4, 170, 56, 46, 1.25, 0.026, 0.012, 6],
];
type SideRow = [limb: string, f: number, dAz: number, e: number, L: number, r0: number, r1: number];
const SIDES_50: SideRow[] = LIMBS.slice(0, 11).flatMap(([id], i): SideRow[] => [[id, 0.42, 38 - 6 * (i % 3), 26, 1.3 - 0.05 * i, 0.026, 0.01], [id, 0.7, -36 + 5 * (i % 2), 22, 1.0 - 0.04 * i, 0.02, 0.009]]);
const SIDES_100: SideRow[] = LIMBS.slice(0, 9).map(([id], i): SideRow => [id, 0.5, i % 2 ? -36 : 38, 24, 1.2 - 0.05 * i, 0.026, 0.012]);

function woody(scale: Scale): Tube[] {
  const stems = Object.entries(STEM[scale]).map(([id, r]) => stemTube(id, id, BARK, id === "root-plate" ? TONE.root : TONE.stem, id === "root-plate" ? "root" : id === "leader" ? "leader" : "stem", r));
  const byId = (id: string): Tube => stems.find((t) => t.id === id)!;
  const n = scale === 50 ? 7 : scale === 100 ? 5 : scale === 200 ? 3 : 2;
  const limbs = LIMBS.map(([id, parent, y, az, e0, e1, L, r0, r1, bend]) => branch(id, id, YOUNG, TONE.limb, "limb", axisAt(byId(parent), y), az, e0, e1, L, r0, r1, scale === 500 ? 0 : bend, n));
  const sides = (scale === 50 ? SIDES_50 : scale === 100 ? SIDES_100 : []).map(([limb, f, dAz, e, L, r0, r1], i) => branch(`${limb}-side-${i + 1}`, limb, YOUNG, TONE.limb, "branch", along(limbs.find((x) => x.id === limb)!, f), LIMBS.find((x) => x[0] === limb)![3] + dAz, e, e - 12, L, r0, r1, 0, scale === 50 ? 4 : 3));
  const roots = scale <= 100 ? [15, 85, 160, 230, 300].map((az, i) => branch(`root-${i + 1}`, "root-plate", BARK, TONE.root, "root", [0, 0.2, 0], az, -12, -12, 0.55, 0.12, 0.05, 0, scale === 50 ? 3 : 2)) : [];
  return [...stems, ...roots, ...limbs, ...sides];
}

/** 🔺️ The conical-ovoid crown: widest 4.0 m at 7.5 m, tapering like a cone to the top at 20 m, base at 3.5 m; 0.94 as deep across as ahead. */
const env = (az: number, y: number): number => {
  const t = y - 7.5;
  const R = t >= 0 ? 4.0 * Math.max(0, 1 - (t / 12.5) ** 1.6) ** (1 / 1.6) : 4.0 * Math.max(0, 1 - Math.abs(t / 4.0) ** 2.2) ** (1 / 2.2);
  return R / Math.hypot(Math.cos((az * Math.PI) / 180), Math.sin((az * Math.PI) / 180) / 0.94);
};

const WOOD_50 = woody(50);
const TIPS = LIMBS.map(([id, , , az]) => { const p = WOOD_50.find((x) => x.id === id)!.points.at(-1)!.p; return { id: `fol-${id.slice(5)}`, az, y: p[1], p }; });
const TOP_TIP = WOOD_50.find((t) => t.id === "leader")!.points.at(-1)!.p;
const angle = (a: number, b: number): number => Math.abs(((((a - b) % 360) + 540) % 360) - 180);
const boneAt = (az: number, y: number): string => (y >= 18.0 ? "fol-top" : TIPS.reduce((best, t) => ((angle(az, t.az) / 60) ** 2 + ((y - t.y) / 1.8) ** 2 < (angle(az, best.az) / 60) ** 2 + ((y - best.y) / 1.8) ** 2 ? t : best)).id);
const matOf = (bone: string): string => (bone === "fol-core" ? SHADE : SUN);

/** 🍃️ The main rings shared by every scale (height, radial and vertical semi-axes, droop); the scales differ in the number and tangential width of their lobes. The crown is open: no rings between the main rings at any scale, so stem and limbs show through. */
const MAIN: [id: string, y: number, a: number, b: number, tilt: number][] = [
  ["s1", 4.4, 0.95, 0.7, 14], ["s2", 5.6, 1.0, 0.78, 8], ["s3", 6.85, 1.02, 0.82, 4], ["s4", 8.1, 1.02, 0.82, 0], ["s5", 9.4, 0.98, 0.8, -2], ["s6", 10.7, 0.94, 0.78, -4], ["s7", 12.0, 0.9, 0.76, -6],
  ["s8", 13.3, 0.84, 0.72, -8], ["s9", 14.6, 0.78, 0.7, -10], ["s10", 15.9, 0.7, 0.66, -12], ["s11", 17.1, 0.62, 0.6, -14], ["s12", 18.2, 0.54, 0.55, -16],
];
const main = (n: number[], c: number[], grow = 1): RingDef[] => MAIN.map(([id, y, a, b, tilt], i) => ({ id, y, n: n[i]!, az0: 0, a, b: b * grow, c: c[i]!, tilt }));
const RINGS: Record<Scale, RingDef[]> = {
  50: main([12, 12, 16, 16, 16, 12, 12, 12, 12, 8, 8, 4], [1.0, 1.1, 0.95, 0.95, 0.92, 1.1, 1.0, 0.92, 0.82, 1.0, 0.82, 0.85]),
  100: main([8, 12, 12, 12, 12, 8, 8, 8, 8, 8, 4, 4], [1.4, 1.15, 1.2, 1.22, 1.18, 1.5, 1.38, 1.26, 1.12, 0.98, 1.1, 0.85], 1.04),
  200: main([8, 8, 8, 8, 8, 8, 8, 8, 4, 4, 4, 4], [1.5, 1.65, 1.75, 1.75, 1.72, 1.62, 1.5, 1.36, 1.7, 1.5, 1.28, 0.95], 1.1),
  500: [
    { id: "s1", y: 4.6, n: 6, az0: 0, a: 1.0, b: 0.95, c: 2.2, tilt: 12 },
    { id: "s2", y: 6.6, n: 8, az0: 0, a: 1.05, b: 1.15, c: 2.4, tilt: 4 },
    { id: "s3", y: 8.9, n: 8, az0: 0, a: 1.05, b: 1.2, c: 2.3 },
    { id: "s4", y: 11.3, n: 8, az0: 0, a: 0.95, b: 1.15, c: 2.0, tilt: -4 },
    { id: "s5", y: 13.8, n: 6, az0: 0, a: 0.85, b: 1.1, c: 1.75, tilt: -8 },
    { id: "s6", y: 16.3, n: 4, az0: 0, a: 0.7, b: 1.0, c: 1.5, tilt: -12 },
    { id: "s7", y: 17.85, n: 3, az0: 0, a: 0.55, b: 0.85, c: 1.0, tilt: -14 },
  ],
};
const TOP: Record<Scale, [V3, V3]> = { 50: [[0, 19.35, 0], [0.5, 0.65, 0.48]], 100: [[0, 19.3, 0], [0.56, 0.7, 0.54]], 200: [[0, 19.2, 0], [0.66, 0.8, 0.64]], 500: [[0, 18.95, 0], [0.85, 1.05, 0.82]] };
const CORE: Record<Scale, [V3, V3][]> = {
  50: [[[0.3, 6.0, 0.2], [1.6, 1.2, 1.5]], [[-0.3, 8.6, -0.2], [1.7, 1.3, 1.6]], [[0.2, 11.2, 0.1], [1.5, 1.2, 1.4]], [[-0.1, 13.8, -0.1], [1.2, 1.1, 1.15]], [[0, 16.2, 0], [0.8, 0.9, 0.8]]],
  100: [[[0.2, 6.6, 0.1], [1.7, 1.4, 1.6]], [[-0.2, 9.8, -0.1], [1.7, 1.5, 1.6]], [[0, 13.2, 0], [1.3, 1.3, 1.25]], [[0, 16.0, 0], [0.85, 1.0, 0.85]]],
  200: [[[0, 7.6, 0], [1.9, 2.1, 1.8]], [[0, 12.6, 0], [1.4, 1.9, 1.35]]],
  500: [[[0, 9.5, 0], [2.0, 4.0, 1.9]]],
};

function crown(scale: Scale): Lobe[] {
  const at500 = scale === 500;
  const sun = ringLobes(RINGS[scale], env, at500 ? () => "stem-2" : boneAt, matOf);
  const top = freeLobe("top", at500 ? "stem-2" : "fol-top", SUN, "sun", TOP[scale][0], TOP[scale][1]);
  const core = CORE[scale].map(([c, r], i) => freeLobe(`core-${i + 1}`, at500 ? "stem-2" : "fol-core", SHADE, "shade", c, r));
  return [...sun, top, ...core];
}

const OUTLETS: Record<Scale, ScaleModel["outlets"]> = {
  50: [["transpiration-sun", "tp-sun-1", "s3-1", 0, 25], ["transpiration-sun", "tp-sun-2", "s3-5", 90, 25], ["transpiration-sun", "tp-sun-3", "s3-9", 180, 25], ["transpiration-sun", "tp-sun-4", "s3-13", 270, 25], ["transpiration-sun", "tp-sun-5", "s7-3", 60, 45], ["transpiration-sun", "tp-sun-6", "s7-9", 240, 45], ["transpiration-sun", "tp-sun-7", "top", 0, 90],
    ["transpiration-shade", "tp-shade-1", "core-1", 0, 90], ["transpiration-shade", "tp-shade-2", "core-2", 0, 90], ["transpiration-shade", "tp-shade-3", "core-3", 0, 90], ["transpiration-shade", "tp-shade-4", "core-4", 0, 90], ["transpiration-shade", "tp-shade-5", "core-5", 0, 90]],
  100: [["transpiration-sun", null, "s3-1", 0, 25], ["transpiration-sun", null, "s3-7", 180, 25], ["transpiration-sun", null, "s7-3", 90, 45], ["transpiration-sun", null, "top", 0, 90], ["transpiration-shade", null, "core-1", 0, 90], ["transpiration-shade", null, "core-3", 0, 90]],
  200: [["transpiration-sun", null, "s3-1", 0, 25], ["transpiration-sun", null, "top", 0, 90], ["transpiration-shade", null, "core-1", 0, 90]],
  500: [["transpiration-sun", null, "top", 0, 90], ["transpiration-shade", null, "core-1", 0, 90]],
};

const DECISIONS: Record<Scale, Json[]> = {
  50: [
    tx("Body at 1:50: a broad root flare with five surface roots at the waterline, one straight dark stem in four parts that runs through to the top at 19.3 m, thirteen short limbs in a spiral (28 degrees above the horizontal at 4 m, 56 degrees at 17 m) with twenty-two side branches, and a conical-ovoid crown of rounded-leaf lobes on twelve rings from 4.4 m to 18.2 m, narrowing like a cone above 9 m, with a pointed top lobe at 20 m and five shade lobes along the stem (thermal solids only); there are no rings between the main rings, so the crown stays open and the stem shows through.", "Körper bei 1:50: ein breiter Wurzelanlauf mit fünf Oberflächenwurzeln an der Wasserlinie, ein gerader dunkler Stamm in vier Teilen, der bis zum Gipfel auf 19,3 m durchläuft, dreizehn kurze Äste in einer Spirale (28 Grad über der Waagerechten auf 4 m, 56 Grad auf 17 m) mit zweiundzwanzig Seitenzweigen und eine kegelig-eiförmige Krone aus Lappen rundlicher Blätter auf zwölf Ringen von 4,4 m bis 18,2 m, die sich über 9 m wie ein Kegel verjüngt, mit einem spitzen Gipfellappen auf 20 m und fünf Schattenlappen entlang des Stamms (nur als Thermikkörper); zwischen den Hauptringen gibt es keine Ringe, so bleibt die Krone offen und der Stamm scheint durch."),
    tx("Thermal at 1:50: stem bark is the root flare, roots, stem and leader; branch bark the limbs and side branches; sun leaves the outer lobes, shade leaves the five core lobes, the visible regions by reference to the body; the shade lobes (thermal solids only) are thermal solids of their own (a hidden region). The transpiration outlets are lens patches in the free faces of the lobes that carry the emitter anchors.", "Thermik bei 1:50: Stammrinde sind Wurzelanlauf, Wurzeln, Stamm und Leittrieb; Astrinde die Äste und Seitenzweige; Sonnenblätter die äußeren Lappen, Schattenblätter die fünf Kernlappen, die sichtbaren Regionen als Verweis auf den Körper; die Schattenlappen (nur als Thermikkörper) sind eigene Thermikkörper (eine verdeckte Region). Die Transpirationsauslässe sind Linsenflächen in den freien Seiten der Lappen, die die Emitteranker tragen."),
    tx("Structure at 1:50: the wood under the bark of the root flare, the four stem parts and the thirteen limbs along their real axes, joints at the rig joints, the round fixed root plate and the masses of the wood bones and of the fifteen foliage masses.", "Tragwerk bei 1:50: das Holz unter der Rinde von Wurzelanlauf, den vier Stammteilen und den dreizehn Ästen entlang ihrer wirklichen Achsen, Gelenke an den Rig-Gelenken, der runde eingespannte Wurzelteller und die Massen der Holzknochen und der fünfzehn Laubmassen."),
  ],
  100: [
    tx("Body at 1:100, authored anew: the root flare with five short roots, the stem on three stations per part, the thirteen limbs on five sections with nine side branches, and the crown redrawn with fewer, broader lobes on the same twelve rings, the top lobe and four shade lobes (thermal solids only).", "Körper bei 1:100, neu modelliert: der Wurzelanlauf mit fünf kurzen Wurzeln, der Stamm auf drei Stationen je Teil, die dreizehn Äste auf fünf Schnitten mit neun Seitenzweigen und die Krone mit weniger, breiteren Lappen auf denselben zwölf Ringen neu gezeichnet, der Gipfellappen und vier Schattenlappen (nur als Thermikkörper)."),
    tx("Thermal at 1:100: the same four regions by reference; six outlets.", "Thermik bei 1:100: dieselben vier Regionen als Verweis; sechs Auslässe."),
    tx("Structure at 1:100: the wood of the eighteen load-bearing bones with simplified taper, joints, the root plate and the masses.", "Tragwerk bei 1:100: das Holz der achtzehn tragenden Knochen mit vereinfachter Verjüngung, Gelenke, der Wurzelteller und die Massen."),
  ],
  200: [
    tx("Body at 1:200, authored anew: root flare and the four stem parts as plain tapering lofts, the thirteen limbs on three sections without side branches (tips raised to the printable 46 mm), and the crown as twelve rings of broad lobes, a top lobe and two shade cores (thermal only).", "Körper bei 1:200, neu modelliert: Wurzelanlauf und die vier Stammteile als einfache verjüngte Lofts, die dreizehn Äste auf drei Schnitten ohne Seitenzweige (Spitzen auf druckbare 46 mm angehoben) und die Krone als zwölf Ringe breiter Lappen, ein Gipfellappen und zwei Schattenkerne (nur Thermik)."),
    tx("Thermal at 1:200: bark and leaves by reference; three outlets.", "Thermik bei 1:200: Rinde und Laub als Verweis; drei Auslässe."),
    tx("Structure at 1:200: the wood of the eighteen load-bearing bones, joints where they are at least one smallest feature thick, the root plate and the masses.", "Tragwerk bei 1:200: das Holz der achtzehn tragenden Knochen, Gelenke, wo sie mindestens ein kleinstes Merkmal dick sind, der Wurzelteller und die Massen."),
  ],
  500: [
    tx("Body at 1:500: root flare and the lower two stem parts as plain lofts and the crown as forty-five masses bound to the middle stem as the drawing binds it (seven rings tapering to the top, the top and one long shade core) (thermal only); the limbs and the upper stem lie inside the crown and are not part of the body.", "Körper bei 1:500: Wurzelanlauf und die unteren zwei Stammteile als einfache Lofts und die Krone als fünfundvierzig Massen, an den mittleren Stamm gebunden wie in der Zeichnung (sieben zum Gipfel hin kleiner werdende Ringe, der Gipfel und ein langer Schattenkern) (nur Thermik); die Äste und der obere Stamm liegen in der Krone und gehören nicht zum Körper."),
    tx("Thermal at 1:500: stem bark and leaves by reference; the upper stem, the leader and the thirteen limbs are thermal solids of their own inside the crown; two outlets.", "Thermik bei 1:500: Stammrinde und Laub als Verweis; oberer Stamm, Leittrieb und die dreizehn Äste sind eigene Thermikkörper in der Krone; zwei Auslässe."),
    tx("Structure at 1:500: the eighteen load-bearing bones as members of their declared section, the joints of the stem, the root plate and the masses; this list is already minimal.", "Tragwerk bei 1:500: die achtzehn tragenden Knochen als Stäbe ihres angegebenen Querschnitts, die Gelenke des Stamms, der Wurzelteller und die Massen; diese Liste ist bereits minimal."),
  ],
};

const models = {} as Record<Scale, ScaleModel>;
for (const scale of [50, 100, 200, 500] as Scale[]) {
  const all = clamp(scale, woody(scale));
  const hidden = scale === 500 ? new Set(["stem-3", "leader", ...LIMBS.map((l) => l[0])]) : new Set<string>();
  models[scale] = { scale, tubes: all.filter((t) => !hidden.has(t.bone)), hiddenTubes: all.filter((t) => hidden.has(t.bone)), lobes: crown(scale), clusters: [], hiddenClusters: [], outlets: OUTLETS[scale], crownBone: scale === 500 ? "stem-2" : undefined, decisions: DECISIONS[scale] };
}

const tubeOf = (id: string): Tube => WOOD_50.find((t) => t.id === id)!;
const startOf = (id: string, y: number): V3 => axisAt(tubeOf(id), y);
const L = (id: string, title: [string, string], group: BoneDef["group"], parent: string | null, from: V3, to: V3, limit: number): BoneDef => ({ id, parent, title: tx(...title), group, from, to, limit });
const STEM_BONES: [id: string, title: [string, string], from: number, to: number, limit: number][] = [["stem-1", ["Lower stem", "Unterer Stamm"], 0.5, 5.5, 2], ["stem-2", ["Middle stem", "Mittlerer Stamm"], 5.5, 11.0, 3], ["stem-3", ["Upper stem", "Oberer Stamm"], 11.0, 16.0, 5], ["leader", ["Leader", "Leittrieb"], 16.0, 19.3, 8]];
const BONES: BoneDef[] = [
  L("root-plate", ["Root plate and flare", "Wurzelteller und Wurzelanlauf"], "root", null, [0, 0, 0], [0, 0.5, 0], 0),
  ...STEM_BONES.map(([id, title, a, b, limit], i) => L(id, title, id === "leader" ? "leader" : "stem", STEMS[i]!, startOf(id, a + (i === 0 ? 0 : 0)), id === "leader" ? TOP_TIP : startOf(id, b), limit)),
  ...LIMBS.map(([id, parent, y]) => L(id, [`Limb ${id.slice(5)} of the spiral`, `Ast ${id.slice(5)} der Spirale`], "limb", parent, startOf(parent, y), tubeOf(id).points.at(-1)!.p, 12)),
  ...TIPS.map((t) => L(t.id, [`Foliage of limb ${t.id.slice(4)}`, `Laub von Ast ${t.id.slice(4)}`], "foliage", `limb-${t.id.slice(4)}`, t.p, t.p, 30)),
  L("fol-top", ["Foliage of the crown top", "Laub des Kronengipfels"], "foliage", "leader", TOP_TIP, TOP_TIP, 30),
  L("fol-core", ["Shaded inner foliage", "Beschattetes Innenlaub"], "foliage", "stem-2", startOf("stem-2", 8.0), startOf("stem-2", 8.0), 20),
];

const REFS = [
  { id: "mcvean-1953", citation: "McVean, D. N. (1953). Alnus glutinosa (L.) Gaertn. Biological Flora of the British Isles. Journal of Ecology 41(2), 447-466. Habit (to 20-25 m, conical crown, excurrent stem), riparian sites, adventitious roots at the waterline, rounded leaves with a notched tip." },
  { id: "houston-durrant-2016", citation: "Houston Durrant, T., de Rigo, D. and Caudullo, G. (2016). Alnus glutinosa in Europe: distribution, habitat, usage and threats. In San-Miguel-Ayanz, J. et al. (eds.), European Atlas of Forest Tree Species. Publications Office of the European Union, Luxembourg, e01f3c0." },
  { id: "wood-handbook-2021", citation: "Forest Products Laboratory (2021). Wood Handbook: Wood as an Engineering Material. General Technical Report FPL-GTR-282, USDA Forest Service. Chapter 5 (mechanical properties of green wood, red alder as the nearest listed alder); used as a guide, the species values are authored estimates." },
  { id: "en-1991-1-4", citation: "EN 1991-1-4:2005 Eurocode 1, Actions on structures, Part 1-4: Wind actions. Air density 1.25 kg/m3, drag force F = 0.5 rho cd A v^2, power-law profile." },
  { id: "en-1991-1-3", citation: "EN 1991-1-3:2003 Eurocode 1, Actions on structures, Part 1-3: Snow loads. Used for the order of magnitude of wet-snow accretion on foliage." },
  { id: "vogel-1989", citation: "Vogel, S. (1989). Drag and reconfiguration of broad leaves in high winds. Journal of Experimental Botany 40(8), 941-948." },
  { id: "moore-maguire-2004", citation: "Moore, J. R. and Maguire, D. A. (2004). Natural sway frequencies and damping ratios of trees: concepts, review and synthesis of previous studies. Trees 18, 195-203. Used for the order of magnitude of the first sway frequency." },
  { id: "jones-2014", citation: "Jones, H. G. (2014). Plants and Microclimate, 3rd ed., Cambridge University Press. Leaf energy balance, leaf area index, stomatal and boundary-layer conductance." },
  { id: "monteith-unsworth-2013", citation: "Monteith, J. L. and Unsworth, M. H. (2013). Principles of Environmental Physics, 4th ed., Academic Press. Penman-Monteith leaf transpiration." },
  { id: "flood-drag", citation: "Hydrodynamic drag on the submerged stem base, F = 0.5 rho_w cd d h v^2 with rho_w 1000 kg/m3, cd 1.0 for a rough cylinder, d 0.7 m (flare and roots), h 0.8 m and v 1.2 m/s; an authored estimate of a summer spate, not a measured flood." },
];

const CALM = { "stem-bark": 0.3, "branch-bark": 0.22, "leaf-sun": 0.85, "leaf-shade": 0.12 };
const WINDY = { stem: 0.5, leader: 2.0, limb: 2.5, branch: 2.5, foliage: 6 } as const;
const GALE = { stem: 1.6, leader: 6, limb: 7, branch: 7, foliage: 16 } as const;
const flood = (ctx: Ctx): Json => ({ id: "flood-drag", kind: "imposed", title: tx("Drag of the flood water on the stem base", "Strömungsdruck des Hochwassers auf den Stammfuß"), bone: "root-plate", x: 0.4, y: 0, fxN: Math.round(0.5 * 1000 * 1.0 * 0.7 * 0.8 * 1.2 ** 2), fyN: 0, source: "authored-estimate", reference: "flood-drag", ...(ctx ? {} : {}) });
export const blackAlder: Species = {
  id: "black-alder",
  title: tx("Black Alder", "Schwarz-Erle"),
  summary: tx("A riverbank black alder of about 45 years, 20 m high: one straight, dark, fissured stem runs through to the top from a flared base with surface roots at the waterline, thirteen short limbs in a spiral carry a conical-ovoid, open crown of rounded, notched leaves; wind, a summer flood around the stem base and early wet snow on the late green foliage are seasonal poses and loads.", "Eine Schwarz-Erle am Flussufer von etwa 45 Jahren, 20 m hoch: Ein gerader, dunkler, rissiger Stamm läuft von einem ausgestellten Fuß mit Oberflächenwurzeln an der Wasserlinie bis zum Gipfel durch, dreizehn kurze Äste in einer Spirale tragen eine kegelig-eiförmige, offene Krone aus rundlichen, an der Spitze eingekerbten Blättern; Wind, ein Sommerhochwasser um den Stammfuß und früher Nassschnee auf dem lange grünen Laub sind jahreszeitliche Posen und Lasten."),
  tags: ["tree", "deciduous", "broadleaf", "native", "riparian", "conical", "excurrent", "wind", "transpiration"],
  provenance: { author: "Semio staffage plant collection worker PW-06", method: "handcrafted", created: "2026-11-22", references: REFS, notes: tx("Rig, body and drawings are authored by hand for an open-grown black alder of about 45 years on a riverbank: 20 m high (black alders reach 20 to 25 m, rarely 35 m), crown 8 m wide and 7.5 m deep, stem diameter about 0.42 m at breast height, the crown base at 3.5 m. Catkins and the woody cones (1 to 2 cm) are below the printable feature and are not modelled. The flood, snow and wood values are authored estimates guided by the cited sources.", "Rig, Körper und Zeichnungen sind von Hand für eine frei gewachsene Schwarz-Erle von etwa 45 Jahren an einem Flussufer entworfen: 20 m hoch (Schwarz-Erlen werden 20 bis 25 m, selten 35 m), Krone 8 m breit und 7,5 m tief, Stammdurchmesser etwa 0,42 m in Brusthöhe, der Kronenansatz auf 3,5 m. Kätzchen und die verholzten Zapfen (1 bis 2 cm) liegen unter dem druckbaren Merkmal und werden nicht modelliert. Hochwasser-, Schnee- und Holzwerte sind Autorenschätzungen, angelehnt an die genannten Quellen.") },
  materials: [
    { id: BARK, title: tx("Alder bark, dark grey-brown, fissured into square plates", "Erlenrinde, dunkel graubraun, in quadratische Platten gerissen"), emissivity: 0.95, solarAbsorptivity: 0.78, densityKgM3: 650, conductivityWmK: 0.12, source: "authored-estimate", reference: "wood-handbook-2021" },
    { id: YOUNG, title: tx("Smooth grey-brown bark of limbs and twigs, with lenticels", "Glatte graubraune Rinde der Äste und Zweige, mit Korkporen"), emissivity: 0.95, solarAbsorptivity: 0.7, densityKgM3: 680, conductivityWmK: 0.13, source: "authored-estimate", reference: "wood-handbook-2021" },
    { id: SUN, title: tx("Alder sun leaves (dark green, rounded, notched)", "Erlen-Sonnenblätter (dunkelgrün, rundlich, eingekerbt)"), emissivity: 0.97, solarAbsorptivity: 0.55, source: "literature", reference: "jones-2014" },
    { id: SHADE, title: tx("Alder shade leaves (effective)", "Erlen-Schattenblätter (wirksam)"), emissivity: 0.97, solarAbsorptivity: 0.25, source: "authored-estimate", reference: "jones-2014" },
    { id: "alder-wood", title: tx("Green alder wood", "Frisches Erlenholz"), emissivity: 0.9, solarAbsorptivity: 0.6, densityKgM3: 830, conductivityWmK: 0.16, elasticModulusPa: 7500000000, strengthPa: 45000000, source: "authored-estimate", reference: "wood-handbook-2021" },
  ],
  bones: BONES, models, woodDensity: 830, leafMassKgM2: 0.2, barkFraction: 0.08,
  regionOfTube: (t) => (t.group === "limb" || t.group === "branch" ? "branch-bark" : "stem-bark"),
  regionOfLobe: (l) => (l.group === "shade" ? "leaf-shade" : "leaf-sun"),
  regions: [
    { id: "stem-bark", title: tx("Dark fissured bark of flare, roots and stem", "Dunkle rissige Rinde von Fuß, Wurzeln und Stamm"), kind: "bark", model: "passive", material: BARK, areaM2: 0, characteristicLengthM: 0.36, geometry: "cylinder", skyViewFactor: 0.4, layers: [], heatCapacityJM2K: 75000, source: "authored-estimate", reference: "wood-handbook-2021" },
    { id: "branch-bark", title: tx("Bark of the spiral limbs and side branches", "Rinde der Spiraläste und Seitenzweige"), kind: "bark", model: "passive", material: YOUNG, areaM2: 0, characteristicLengthM: 0.08, geometry: "cylinder", skyViewFactor: 0.45, layers: [], heatCapacityJM2K: 30000, source: "authored-estimate", reference: "wood-handbook-2021" },
    { id: "leaf-sun", title: tx("Sun leaves of the outer crown", "Sonnenblätter der Außenkrone"), kind: "foliage", model: "leaf", material: SUN, areaM2: 105, areaBasis: "exchange", characteristicLengthM: 0.07, geometry: "vertical-plate", skyViewFactor: 0.7, layers: [], heatCapacityJM2K: 800, leaf: { stomatalConductanceMaxMS: 0.007, leafAreaIndex: 2.2, photosynthesis: "c3" }, source: "authored-estimate", reference: "jones-2014" },
    { id: "leaf-shade", title: tx("Shade leaves along the stem", "Schattenblätter entlang des Stamms"), kind: "foliage", model: "leaf", material: SHADE, hidden: tx("The shade leaves lie inside the crown, behind the sun leaves from every side, so they are drawn by no shape; their lobes are thermal solids of their own inside the body.", "Die Schattenblätter liegen im Inneren der Krone, von jeder Seite hinter den Sonnenblättern, und werden daher von keiner Form gezeichnet; ihre Lappen sind eigene Thermikkörper im Körper."), areaM2: 75, areaBasis: "exchange", characteristicLengthM: 0.08, geometry: "vertical-plate", skyViewFactor: 0.3, layers: [], heatCapacityJM2K: 650, leaf: { stomatalConductanceMaxMS: 0.0035, leafAreaIndex: 1.6, photosynthesis: "c3" }, source: "authored-estimate", reference: "jones-2014" },
  ],
  emitters: [
    { id: "transpiration-sun", kind: "transpiration", title: tx("Transpiration of the sun leaves", "Transpiration der Sonnenblätter"), anchor: "tp-sun-1", anchors: ["tp-sun-1", "tp-sun-2", "tp-sun-3", "tp-sun-4", "tp-sun-5", "tp-sun-6", "tp-sun-7"], directionDeg: 0, diameterM: 2.0, region: "leaf-sun", source: "authored-estimate", reference: "monteith-unsworth-2013" },
    { id: "transpiration-shade", kind: "transpiration", title: tx("Transpiration of the shade leaves", "Transpiration der Schattenblätter"), anchor: "tp-shade-1", anchors: ["tp-shade-1", "tp-shade-2", "tp-shade-3", "tp-shade-4", "tp-shade-5"], directionDeg: 0, diameterM: 1.5, region: "leaf-shade", source: "authored-estimate", reference: "monteith-unsworth-2013" },
  ],
  environment: { airTempC: 22, meanRadiantTempC: 24, relativeHumidity: 0.7, windMS: 1.5, windDirectionDeg: 0, solarWM2: 700, pressurePa: 101325, diffuseFraction: 0.15 },
  thermalAssumptions: [
    tx("Leaf regions use one-sided leaf area; the leaf area index is that area over the crown ground projection of about 47 m2. The maximum stomatal conductance is an authored estimate at the upper end of the broadleaf range given by Jones (2014), since alder on wet sites keeps its stomata open and transpires freely; no published species maximum is claimed. Photosynthesis type: C3 (the nitrogen-fixing root nodules do not change it). Aperture and night closure are the engine's business, not authored here.", "Blattregionen verwenden die einseitige Blattfläche; der Blattflächenindex ist diese Fläche bezogen auf die Kronenprojektion von etwa 47 m2. Die maximale Spaltöffnungsleitfähigkeit ist eine Autorenschätzung am oberen Ende der von Jones (2014) genannten Spanne für Laubgehölze, da die Erle auf nassen Standorten ihre Spaltöffnungen offen hält und reichlich transpiriert; ein veröffentlichter Artwert wird nicht behauptet. Photosynthesetyp: C3 (die stickstoffbindenden Wurzelknöllchen ändern daran nichts). Öffnung und nächtliches Schließen sind Sache der Engine und werden hier nicht vorgegeben."),
    tx("Bark is passive. Its areal heat capacity is the volumetric heat capacity of green wood (about 2.7 MJ per m3 and K) times an effective depth of 1 to 2.8 cm, an authored estimate guided by the Wood Handbook; the bark areas are the mantles of the modelled stem and limbs. In the flood pose the submerged share of the stem bark is bound as a contact with the river water. No surface temperature is authored.", "Rinde ist passiv. Ihre flächenbezogene Wärmekapazität ist die volumetrische Wärmekapazität von frischem Holz (etwa 2,7 MJ je m3 und K) mal einer wirksamen Tiefe von 1 bis 2,8 cm, eine Autorenschätzung angelehnt an das Wood Handbook; die Rindenflächen sind die Mäntel des modellierten Stamms und der Äste. In der Hochwasserpose ist der untergetauchte Anteil der Stammrinde als Kontakt mit dem Flusswasser gebunden. Es werden keine Oberflächentemperaturen vorgegeben."),
    tx("Transpiration emitters release the latent flux of their foliage region as vapour, spread over at least five anchors on the crown (the shade anchors on the shade lobes along the stem (thermal solids only)); the direction is the vertical measured in the frame of the first anchor's bone. Catkins, cones and leaf fall are not geometric: a pose cannot hide shapes, so seasonal poses change the thermal binding and the loads only.", "Transpirationsemitter geben den latenten Strom ihrer Laubregion als Dampf ab, verteilt auf mindestens fünf Anker an der Krone (die Schattenanker an den Schattenlappen entlang des Stamms (nur als Thermikkörper)); die Richtung ist die Senkrechte im Bezugssystem des Knochens des ersten Ankers. Kätzchen, Zapfen und Laubfall sind nicht geometrisch: Eine Pose kann keine Formen ausblenden, daher ändern jahreszeitliche Posen nur die thermische Bindung und die Lasten."),
  ],
  structureAssumptions: [
    tx("The tree is one rigid body per evaluated pose, fixed at the root plate; wind drag acts at the centre of each foliage mass and on the stem as a line load, gravity on every bone mass, snow as vertical loads on the foliage masses and the flood as a point load on the stem base. The base moment follows from statics at the posed geometry; root-soil failure (alder roots in saturated soil), scour and dynamic amplification are not modelled. Pose lean angles are authored estimates that grow with the drag force.", "Der Baum ist je ausgewerteter Pose ein starrer Körper, am Wurzelteller eingespannt; Winddruck wirkt im Schwerpunkt jeder Laubmasse und am Stamm als Linienlast, Schwerkraft auf jede Knochenmasse, Schnee als senkrechte Lasten auf die Laubmassen und das Hochwasser als Punktlast auf den Stammfuß. Das Fußmoment folgt aus der Statik der gezeigten Geometrie; Wurzel-Boden-Versagen (Erlenwurzeln in nassem Boden), Auskolkung und dynamische Überhöhung werden nicht modelliert. Die Neigungswinkel der Posen sind Autorenschätzungen, die mit der Widerstandskraft wachsen."),
    tx("Sections are solid circles of green wood under a bark of 8 % of the radius (authored-estimate properties guided by the Wood Handbook: 830 kg/m3, E 7.5 GPa, bending strength 45 MPa); the rig is planar, so limbs that run out of the side-elevation plane carry their lateral position as an offset and bend in that plane only.", "Querschnitte sind Vollkreise aus frischem Holz unter einer Rinde von 8 % des Radius (Autorenschätzungen angelehnt an das Wood Handbook: 830 kg/m3, E 7,5 GPa, Biegefestigkeit 45 MPa); das Rig ist eben, daher tragen Äste, die aus der Ebene der Seitenansicht laufen, ihre seitliche Lage als Versatz und biegen sich nur in dieser Ebene."),
  ],
  poses: (ctx) => [
    { id: "summer-calm", title: tx("Summer, calm", "Sommer, windstill"), description: tx("Light air of 1.5 m/s at 10 m in July on the riverbank: the straight stem and the spiral limbs stand at rest; drag per foliage mass is a few newtons (cd 0.45).", "Leiser Zug von 1,5 m/s in 10 m Höhe im Juli am Flussufer: Der gerade Stamm und die Spiraläste stehen in Ruhe; der Widerstand je Laubmasse beträgt wenige Newton (cd 0,45)."), bones: {}, ground: "authored", contacts: [], thermal: binding(blackAlder, CALM, { "transpiration-sun": 0.9, "transpiration-shade": 0.65 }), loads: windLoads(ctx, 1.5, 0.45) } as Json,
    { id: "fresh-breeze", title: tx("Fresh breeze", "Frische Brise"), description: tx("Beaufort 5, 8 m/s at 10 m, cd 0.35 on the front area of each foliage mass: the stem leans by 0.5 degrees per part, the slender leader by 2, the limbs by 2.5 and the leafy masses by 6 degrees downwind.", "Beaufort 5, 8 m/s in 10 m Höhe, cd 0,35 auf die Anströmfläche jeder Laubmasse: Der Stamm neigt sich um 0,5 Grad je Teil, der schlanke Leittrieb um 2, die Äste um 2,5 und die Laubmassen um 6 Grad mit dem Wind."), bones: offsets(ctx, WINDY), ground: "authored", contacts: [], thermal: binding(blackAlder, CALM, { "transpiration-sun": 1, "transpiration-shade": 0.72 }), loads: windLoads(ctx, 8, 0.35) } as Json,
    { id: "gale", title: tx("Gale with rain", "Sturm mit Regen"), description: tx("Beaufort 9, 22 m/s at 10 m with rain, cd 0.22 after the reconfiguration of the leaves (Vogel 1989): the tall crown leans strongly, the leader by 6 and the masses by 16 degrees; wet bark and leaves.", "Beaufort 9, 22 m/s in 10 m Höhe mit Regen, cd 0,22 nach der Rekonfiguration der Blätter (Vogel 1989): Die hohe Krone neigt sich stark, der Leittrieb um 6 und die Massen um 16 Grad; nasse Rinde und Blätter."), bones: offsets(ctx, GALE), ground: "authored", contacts: [], thermal: binding(blackAlder, { "stem-bark": 0.05, "branch-bark": 0.03, "leaf-sun": 0.12, "leaf-shade": 0.02 }, { "transpiration-sun": 0.4, "transpiration-shade": 0.28 }, 0.6), loads: windLoads(ctx, 22, 0.22) } as Json,
    { id: "summer-flood", title: tx("Summer flood around the stem base", "Sommerhochwasser um den Stammfuß"), description: tx("The river in spate after a thunderstorm: the stem base stands in 0.8 m of water flowing at 1.2 m/s at 14 degrees C; the submerged 7 % of the stem bark is in contact with the water and the drag of the flow (about 400 N) pushes on the flare; overcast, humid, light air.", "Der Fluss führt nach einem Gewitter Hochwasser: Der Stammfuß steht in 0,8 m Wasser, das mit 1,2 m/s bei 14 Grad C fließt; die untergetauchten 7 % der Stammrinde berühren das Wasser, und der Strömungsdruck (etwa 400 N) drückt auf den Wurzelanlauf; bedeckt, feucht, leiser Zug."), bones: {}, ground: "authored", contacts: [], thermal: binding(blackAlder, { "stem-bark": 0.08, "branch-bark": 0.06, "leaf-sun": 0.25, "leaf-shade": 0.04 }, { "transpiration-sun": 0.55, "transpiration-shade": 0.4 }, 0, { "stem-bark": { fraction: 0.07, tempC: 14 } }), loads: [...windLoads(ctx, 1.5, 0.45), flood(ctx)] } as Json,
    { id: "early-wet-snow", title: tx("Early wet snow on green leaves", "Früher Nassschnee auf grünem Laub"), description: tx("Alder keeps its leaves green into November: an early wet snowfall of about 8 kg/m2 on the plan area of each foliage mass (order of magnitude after EN 1991-1-3) bows the short limbs by 6 degrees and the masses by 11 degrees; wet, cold, overcast, the stomata nearly closed.", "Die Erle behält ihr Laub bis in den November grün: Ein früher Nassschneefall von etwa 8 kg/m2 auf der Grundrissfläche jeder Laubmasse (Größenordnung nach EN 1991-1-3) biegt die kurzen Äste um 6 Grad und die Massen um 11 Grad nach unten; nass, kalt, bedeckt, die Spaltöffnungen fast geschlossen."), bones: offsets(ctx, {}, { limb: 6, branch: 6, foliage: 11 }), ground: "authored", contacts: [], thermal: binding(blackAlder, { "stem-bark": 0.02, "branch-bark": 0.02, "leaf-sun": 0.05, "leaf-shade": 0.01 }, { "transpiration-sun": 0.08, "transpiration-shade": 0.05 }, 0.9), loads: verticalLoads(ctx, "snow", "snow", (id) => tx(`Wet snow on foliage mass ${id}`, `Nassschnee auf Laubmasse ${id}`), "en-1991-1-3", 8) } as Json,
  ],
  clips: (ctx) => {
    const windIds = windLoads(ctx, 8, 0.35).map((l) => l.id);
    return [
      { id: "wind-sway", title: tx("Wind sway", "Windschwingung"), description: tx("A deterministic 4.2 s sway cycle (first mode of a slender 20 m tree of about 0.24 Hz, an authored estimate within the range reviewed by Moore and Maguire 2004) about the fresh-breeze lean, plus and minus 35 % of it; the leader lags the stem by 5 %, the limbs by 8 % and the leafy masses by 12 % of the period.", "Ein deterministischer Schwingungszyklus von 4,2 s (erste Eigenform eines schlanken 20-m-Baums von etwa 0,24 Hz, eine Autorenschätzung im von Moore und Maguire 2004 zusammengestellten Bereich) um die Neigung der frischen Brise, plus und minus 35 % davon; der Leittrieb folgt dem Stamm mit 5 %, die Äste mit 8 % und die Laubmassen mit 12 % der Periode."), durationS: 4.2, fps: 30, loop: true, rootMotion: "none", keys: swayKeys(ctx, "fresh-breeze", WINDY, 0.35, { leader: 0.05, limb: 0.08, branch: 0.08, foliage: 0.12 }, 4.2), contacts: [], phases: [{ id: "sway", fromS: 0, toS: 4.2, title: tx("Sway cycle", "Schwingungszyklus") }], events: [{ t: 0, kind: "motion", title: tx("Cycle start at the mean lean", "Zyklusbeginn in mittlerer Neigung") }], tracks: [{ target: "emitter:transpiration-sun", ease: "smooth", keys: [{ t: 0, v: 1 }, { t: 2.1, v: 0.92 }, { t: 4.2, v: 1 }] }, { target: "emitter:transpiration-shade", ease: "smooth", keys: [{ t: 0, v: 0.72 }, { t: 2.1, v: 0.68 }, { t: 4.2, v: 0.72 }] }] },
      { id: "gust", title: tx("Gust", "Windböe"), description: tx("A single 8 s gust: the wind-load tracks rise from 5 % to 2.75 times the fresh-breeze load at 2.5 s (EN 1991-1-4 peak velocity pressure with a turbulence intensity of 0.25) and decay; the crown leans with the load to the power 0.8, at most 70 % of the gale lean, and springs back.", "Eine einzelne Böe über 8 s: Die Windlast-Spuren steigen bei 2,5 s von 5 % auf das 2,75-fache der Last der frischen Brise (Spitzengeschwindigkeitsdruck nach EN 1991-1-4 mit einer Turbulenzintensität von 0,25) und klingen ab; die Krone neigt sich mit der Last hoch 0,8, höchstens 70 % der Sturmneigung, und federt zurück."), durationS: 8, fps: 30, loop: false, rootMotion: "none", keys: gustKeys(ctx, "fresh-breeze", WINDY, GALE), contacts: [], phases: [{ id: "rise", fromS: 0, toS: 2.5, title: tx("Gust front", "Böenfront") }, { id: "peak", fromS: 2.5, toS: 3.5, title: tx("Peak", "Spitze") }, { id: "decay", fromS: 3.5, toS: 8, title: tx("Decay and recovery", "Abklingen und Erholung") }], events: [{ t: 2.5, kind: "load-on", title: tx("Peak gust load", "Böenspitze") }, { t: 5, kind: "load-off", title: tx("Load below the mean again", "Last wieder unter dem Mittelwert") }], tracks: windIds.map((id) => ({ target: `load:${id}`, ease: "smooth", keys: GUST.map(([t, v]) => ({ t, v })) })) },
      diurnalClip(blackAlder, "summer-calm", CALM, { "transpiration-sun": [[0, 0.04], [5, 0.06], [7, 0.32], [9, 0.72], [12, 1], [15, 0.92], [18, 0.38], [20, 0.1], [24, 0.04]], "transpiration-shade": [[0, 0.03], [5, 0.04], [7, 0.22], [9, 0.5], [12, 0.72], [15, 0.66], [18, 0.27], [20, 0.07], [24, 0.03]] }),
    ];
  },
  presentation: { defaultPose: "summer-calm", defaultClip: "wind-sway", defaultScale: 200 },
  drawing: {
    tones: TONE,
    arcScales: [100],
    glyph: (c, k) => {
      const a = (k * 53 + 30) * (Math.PI / 180);
      const pts: [number, number][] = [[0, -0.05], [0.025, -0.038], [0.038, -0.01], [0.036, 0.018], [0.02, 0.038], [0.006, 0.042], [0, 0.034], [-0.006, 0.042], [-0.02, 0.038], [-0.036, 0.018], [-0.038, -0.01], [-0.025, -0.038]];
      return { closed: true, rings: [pts.map(([x, y]) => ({ x: c.x + x * Math.cos(a) - y * Math.sin(a), y: c.y + x * Math.sin(a) + y * Math.cos(a) }))] };
    },
    bark: (view: View, t: Tube, scale: Scale) => barkMarks(view, t, scale === 50 ? 0.28 : 0.6, (n, r) => {
      const off = [-0.45, 0.05, 0.5, -0.2, 0.3][n % 5]! * r;
      return scale === 50 ? [[off, -0.12], [off, 0.1], [off + 0.22 * r, 0.1]] : [[off, -0.15], [off, 0.15]];
    }),
    decisions: {
      50: {
        silhouette: () => tx("At 1:50 (1 m is 20 mm) the three drawings are the true views of the 1:50 body: the broad flare with surface roots, the straight dark stem with fissures and plate cracks running up to the top, the thirteen spiral limbs and a conical-ovoid, open crown of fifteen foliage masses whose outlines follow their lobes, with rounded-leaf marks; the stem shows between the masses. Plan: the round crown and the stem as a dot.", "Bei 1:50 (1 m sind 20 mm) sind die drei Zeichnungen die wahren Ansichten des 1:50-Körpers: der breite Fuß mit Oberflächenwurzeln, der gerade dunkle Stamm mit Rissen und Plattenrissen bis zum Gipfel, die dreizehn Spiraläste und eine kegelig-eiförmige, offene Krone aus fünfzehn Laubmassen, deren Umrisse ihren Lappen folgen, mit Zeichen rundlicher Blätter; der Stamm scheint zwischen den Massen durch. Draufsicht: die runde Krone und der Stamm als Punkt."),
        retained: tx("Retained: every woody member of the rig with twenty-two side branches, every foliage mass, the bark plates of stem and limbs.", "Beibehalten: jedes Holzglied des Rigs mit zweiundzwanzig Seitenzweigen, jede Laubmasse, die Rindenplatten von Stamm und Ästen."),
        merged: tx("Single leaves are merged into lobed mass outlines; the bark plates into fissure strokes with a cross crack.", "Einzelne Blätter gehen in gelappten Massenumrissen auf; die Rindenplatten in Rissstrichen mit einem Querriss."),
        omitted: tx("Omitted: single leaves (4 to 10 cm), catkins and cones (1 to 2 cm), twigs thinner than 1 cm.", "Weggelassen: einzelne Blätter (4 bis 10 cm), Kätzchen und Zapfen (1 bis 2 cm), Zweige unter 1 cm."),
        thinElements: tx("Twig ends and the thin top of the leader are kept at 1.15 times the printable minimum (11.5 mm real); leaf marks and fissures are 0.13 mm texture strokes kept 0.35 mm inside every outline.", "Zweigenden und die dünne Spitze des Leittriebs bleiben beim 1,15-fachen des druckbaren Minimums (11,5 mm wirklich); Blattzeichen und Risse sind Texturstriche von 0,13 mm, mindestens 0,35 mm innerhalb jedes Umrisses."),
        hatching: tx("No hatching: the shaded inner foliage is a hidden region inside the crown and is not drawn; bark and leaves are drawn with strokes.", "Keine Schraffur: Das beschattete Innenlaub ist eine verdeckte Region im Inneren der Krone und wird nicht gezeichnet; Rinde und Blätter werden mit Strichen gezeichnet."),
        styles: tx("Light line: 0.25 mm outlines, 0.13 mm texture. Dark solid: tonal fills separate the very dark stem, the limbs, the foliage.", "Helle Linie: 0,25 mm Umrisse, 0,13 mm Textur. Dunkle Fläche: getönte Füllungen trennen den sehr dunklen Stamm, die Äste, das Laub."),
        overlays: tx("Each thermal region has its own shapes; leaf marks and fissures are strokes without a region and follow their bone.", "Jede thermische Region hat eigene Formen; Blattzeichen und Risse sind Striche ohne Region und folgen ihrem Knochen."),
      },
      100: {
        silhouette: () => tx("At 1:100 (1 m is 10 mm) the drawings are the true views of the 1:100 body: flare with roots, the straight stem, thirteen limbs with nine side branches and the conical crown as fifteen masses of broader lobes with leaf-cluster arcs.", "Bei 1:100 (1 m sind 10 mm) sind die Zeichnungen die wahren Ansichten des 1:100-Körpers: Fuß mit Wurzeln, der gerade Stamm, dreizehn Äste mit neun Seitenzweigen und die kegelige Krone aus fünfzehn Massen breiterer Lappen mit Blattbüschelbögen."),
        retained: tx("Retained: fifteen foliage masses, flare with roots, stem, all limbs, nine side branches, a few fissures on the stem.", "Beibehalten: fünfzehn Laubmassen, Fuß mit Wurzeln, Stamm, alle Äste, neun Seitenzweige, einige Risse am Stamm."),
        merged: tx("Leaves and small lobes are merged into broader lobed outlines.", "Blätter und kleine Lappen gehen in breiteren gelappten Umrissen auf."),
        omitted: tx("Omitted: leaf marks, thirteen weaker side branches, cross cracks, catkins and cones.", "Weggelassen: Blattzeichen, dreizehn schwächere Seitenzweige, Querrisse, Kätzchen und Zapfen."),
        thinElements: tx("Wood thinner than 23 mm is raised to it (the printable 0.2 mm plus 15 %); arcs and fissures use the 0.13 mm detail pen.", "Holz unter 23 mm wird darauf angehoben (die druckbaren 0,2 mm plus 15 %); Bögen und Risse nutzen den Detailstift von 0,13 mm."),
        hatching: tx("No hatching: the shaded inner foliage is hidden inside the crown; arcs and bark strokes carry the texture.", "Keine Schraffur: Das beschattete Innenlaub liegt verdeckt im Inneren der Krone; Bögen und Rindenstriche tragen die Textur."),
        styles: tx("Light line: 0.2 mm outlines, 0.13 mm detail. Dark solid: tonal masses, detail strokes stay visible on them.", "Helle Linie: 0,2 mm Umrisse, 0,13 mm Detail. Dunkle Fläche: getönte Massen, Detailstriche bleiben darauf sichtbar."),
        overlays: tx("Every region keeps its shapes.", "Jede Region behält ihre Formen."),
      },
      200: {
        silhouette: () => tx("At 1:200 (1 m is 5 mm) the drawings are the true views of the 1:200 body: the straight dark stem with its flare and the thirteen spiral limbs in a conical crown of fifteen scalloped foliage masses, 40 mm wide and 100 mm high.", "Bei 1:200 (1 m sind 5 mm) sind die Zeichnungen die wahren Ansichten des 1:200-Körpers: der gerade dunkle Stamm mit Fuß und die dreizehn Spiraläste in einer kegeligen Krone aus fünfzehn gewellten Laubmassen, 40 mm breit und 100 mm hoch."),
        retained: tx("Retained: flare, stem, the thirteen limbs at no less than 46 mm and the foliage masses.", "Beibehalten: Fuß, Stamm, die dreizehn Äste mit mindestens 46 mm und die Laubmassen."),
        merged: tx("Side branches, leaves and small lobes are merged into the large scalloped masses.", "Seitenzweige, Blätter und kleine Lappen gehen in den großen gewellten Massen auf."),
        omitted: tx("Omitted: fissures, leaf marks, arcs, roots and side branches.", "Weggelassen: Risse, Blattzeichen, Bögen, Wurzeln und Seitenzweige."),
        thinElements: tx("Limb tips and the top of the leader are raised to 46 mm (0.23 mm on paper); they run into the masses.", "Astspitzen und die Spitze des Leittriebs werden auf 46 mm (0,23 mm auf dem Papier) angehoben; sie laufen in die Massen."),
        hatching: tx("No hatching and no internal edge.", "Keine Schraffur und keine innere Kante."),
        styles: tx("Light line: outlines 0.18 mm. Dark solid: the foliage in two tones, the stem darkest.", "Helle Linie: Umrisse 0,18 mm. Dunkle Fläche: das Laub in zwei Tönen, der Stamm am dunkelsten."),
        overlays: tx("Every thermal region keeps a shape: stem, limbs, the foliage masses.", "Jede thermische Region behält eine Form: Stamm, Äste, die Laubmassen."),
      },
      500: {
        silhouette: () => tx("At 1:500 (1 m is 2 mm) the black alder is one conical-ovoid crown, 16 mm wide and 40 mm high, with a scalloped edge from 3.5 m up to the pointed top, over a straight dark stem with its flare; the drawings are the true views of the 1:500 body. Plan: one round scalloped crown; the stem is hidden.", "Bei 1:500 (1 m sind 2 mm) ist die Schwarz-Erle eine kegelig-eiförmige Krone von 16 mm Breite und 40 mm Höhe mit gewelltem Rand von 3,5 m bis zum spitzen Gipfel über einem geraden dunklen Stamm mit Fuß; die Zeichnungen sind die wahren Ansichten des 1:500-Körpers. Draufsicht: eine runde gewellte Krone; der Stamm ist verdeckt."),
        retained: tx("Retained: stem with flare and the crown outline; the stem runs on into the crown for a clean join.", "Beibehalten: Stamm mit Fuß und der Kronenumriss; der Stamm läuft für einen sauberen Anschluss in die Krone weiter."),
        merged: tx("Merged into the crown shape: the foliage, the limbs and the upper stem (as merged regions).", "In der Kronenform zusammengefasst: das Laub, die Äste und der obere Stamm (als zusammengefasste Regionen)."),
        omitted: tx("Omitted: single limbs, fissures; no internal edge.", "Weggelassen: einzelne Äste, Risse; keine innere Kante."),
        thinElements: tx("The stem keeps its real width (0.9 mm at the base); limbs are not thickened but merged into the filled crown.", "Der Stamm behält seine wirkliche Breite (0,9 mm am Fuß); Äste werden nicht verdickt, sondern in die gefüllte Krone aufgenommen."),
        hatching: tx("No hatching; the pen is 0.15 mm for outlines and 0.25 mm for the ground line.", "Keine Schraffur; der Stift ist 0,15 mm für Umrisse und 0,25 mm für die Bodenlinie."),
        styles: tx("Light line: closed outlines only. Dark solid: the crown reads as one tall mass over the dark stem.", "Helle Linie: nur geschlossene Umrisse. Dunkle Fläche: Die Krone liest sich als eine hohe Masse über dem dunklen Stamm."),
        overlays: tx("Overlays map onto: stem-bark (stem) and, merged in the crown shape, leaf-sun, leaf-shade, branch-bark and stem-bark of the upper stem; merged shapes show the area-weighted value of their regions, labelled as merged.", "Überlagerungen bilden ab auf: stem-bark (Stamm) und, in der Kronenform zusammengefasst, leaf-sun, leaf-shade, branch-bark und stem-bark des oberen Stamms; zusammengefasste Formen zeigen den flächengewichteten Wert ihrer Regionen, als zusammengefasst gekennzeichnet."),
      },
    },
  },
  symbol: { id: "black-alder-symbol", title: tx("Plan symbol with true crown extent", "Grundrisssymbol mit wahrer Kronenausdehnung"), description: tx("A conventional plan sign of the black alder for riverbank, site and planting plans: the true outline of the crown and the stem section. It is a separate symbol and is never used silently for a pictorial scale.", "Ein konventionelles Grundrisszeichen der Schwarz-Erle für Ufer-, Lage- und Pflanzpläne: der wahre Umriss der Krone und der Stammquerschnitt. Es ist ein eigenes Symbol und wird nie stillschweigend für einen Bildmaßstab verwendet.") },
  distinction: tx("A tall riverbank tree with one straight dark stem running through a conical, open crown of short spiral limbs; the only plant of the archive with a flood pose, in which the stem base stands in flowing water (contact with the water and a hydrodynamic drag load).", "Ein hoher Ufergehölzbaum mit einem geraden dunklen Stamm, der durch eine kegelige, offene Krone aus kurzen Spiralästen läuft; die einzige Pflanze des Archivs mit einer Hochwasserpose, in der der Stammfuß in fließendem Wasser steht (Kontakt mit dem Wasser und eine Strömungslast)."),
  anchorsTitle: (em, n) => (em === "transpiration-sun" ? tx(`Transpiration of the sun leaves, anchor ${n}`, `Transpiration der Sonnenblätter, Anker ${n}`) : tx(`Transpiration of the shade leaves, anchor ${n}`, `Transpiration der Schattenblätter, Anker ${n}`)),
};

const barkScale: Record<string, number> = { "stem-bark": 0.95, "branch-bark": 0.95 };
for (const r of blackAlder.regions) if (barkScale[r.id] !== undefined) r.areaM2 = r4(models[50].tubes.filter((t) => blackAlder.regionOfTube(t) === r.id).reduce((s, t) => s + tubeArea(t), 0) * barkScale[r.id]!);
export default blackAlder;
if (import.meta.main) {
  const result = build(blackAlder);
  const path = write(blackAlder, result, process.argv.includes("--catalog") ? "catalog" : "candidate");
  console.log(result.report.join("\n"));
  console.log(path);
}
