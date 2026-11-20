/** 🍃️ Round-3 model of the ivy on trellis: every solid of every scale stated by hand in world metres (x along the trellis, y up, z through it; the elevation looks from -z), the 2D drawings redrawn from that body (ruling E1) and the root fixes; writes the volumes source and the candidate document `🗑️generated/r3-ivy-on-trellis/🔣️.json`.
 * @see ./round3-brief.md
 * @see ./r3-ivy-on-trellis-kit.ts */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { entryOf, frames, GENERATED, sourcePath, toLocal, type Frame, type Json, type P2, type V3 } from "./volumes-lib.ts";
import { add, cross3, mul, outline, pathOf, pointsOf, project, r4, r5, solidOf, toDrawing, tube, unit, type Box, type Ell, type Ext, type Loft, type Look, type Prim, type View } from "./r3-ivy-on-trellis-kit.ts";
import { DECISIONS_2D, DECISIONS_3D, T } from "./r3-ivy-on-trellis-texts.ts";

const SPEC = "vegetation/ivy-on-trellis";
const { entry, asset: catalog } = entryOf(SPEC);
const asset: Json = structuredClone(catalog);
const F: Record<string, Frame> = frames(asset);
const K = [1, 2, 3, 4] as const;
const VIEWS: View[] = ["elevation", "front", "plan"];

const TIMBER: Look = { material: "timber-c24", tone: 0.55 };
const LATH: Look = { material: "timber-c24", tone: 0.6, flags: { slender: true } };
const STEEL: Look = { material: "steel-s235", tone: 0.75 };
const STEEL_FINE: Look = { material: "steel-s235", tone: 0.7, role: "detail" };
const PLATE_GROUND: Look = { material: "steel-s235", tone: 0.75, role: "ground" };
const STEM: Look = { material: "ivy-stem", tone: 0.4, flags: { slender: true } };
const STEM_50: Look = { material: "ivy-stem", tone: 0.4 };
const LATH_50: Look = { material: "timber-c24", tone: 0.6 };
const SHOOT: Look = { material: "ivy-stem", tone: 0.42, role: "detail", flags: { slender: true } };
const CORE_HI: Look = { material: "ivy-leaf", tone: 0.24 };
const CORE_LO: Look = { material: "ivy-leaf", tone: 0.2 };
const LEAF_HI: Look = { material: "ivy-leaf", tone: 0.36, role: "texture", flags: { texture: true } };
const LEAF_LO: Look = { material: "ivy-leaf", tone: 0.3, role: "texture", flags: { texture: true } };

/** 🪵️ Trellis: posts 90 x 90 mm at x = ±0.9 m, 2.1 m high; rails 70 x 70 mm between the posts at 0.35 to 0.42 m and 1.93 to 2.0 m; laths 20 x 10 mm, eight vertical (front layer) at 0.2 m and seven horizontal (back layer) at 0.189 m. */
const POST_X = 0.9;
const POST_H = 0.045;
const RAIL = { lo: 0.385, hi: 1.965, half: 0.035, x: 0.855 };
const LATH_V_X = [-0.7, -0.5, -0.3, -0.1, 0.1, 0.3, 0.5, 0.7];
const LATH_H_Y = [1, 2, 3, 4, 5, 6, 7].map((i) => r4(0.42 + (1.51 * i) / 8));
const PLATE_T = 0.008;
const SLEEVE = { outer: 0.049, inner: 0.045, top: 0.28 };

const side = (s: "l" | "r"): number => (s === "l" ? -1 : 1);
const box = (id: string, bone: string, look: Look, c: V3, size: V3, corner?: number): Box => ({ k: "box", id, bone, look, c, size, corner });
const square = (h: number): [number, number][] => [[-h, -h], [h, -h], [h, h], [-h, h]];
const hexagon = (r: number): [number, number][] => [0, 1, 2, 3, 4, 5].map((i) => [r * Math.cos((Math.PI * i) / 3), r * Math.sin((Math.PI * i) / 3)]);
const upright = (id: string, bone: string, look: Look, x: number, z: number, profile: [number, number][], from: number, to: number, holes?: [number, number][][]): Ext => ({ k: "ext", id, bone, look, profile, holes, o: [x, 0, z], ex: [0, 0, 1], ey: [1, 0, 0], from, to });
const through = (id: string, bone: string, look: Look, x: number, y: number, profile: [number, number][], from: number, to: number): Ext => ({ k: "ext", id, bone, look, profile, o: [x, y, 0], ex: [1, 0, 0], ey: [0, 1, 0], from, to });

/** 🪵️ A post at 1:50: planed square with rounded arrises on the base plate, the head chamfered 15 mm on four sides. */
const post50 = (s: "l" | "r"): Loft => ({ k: "loft", id: `post-${s}`, bone: `post-${s}`, look: TIMBER, secs: [[PLATE_T, POST_H], [2.085, POST_H], [2.1, 0.03]].map(([y, a]) => ({ c: [side(s) * POST_X, y!, 0] as V3, a: a!, b: a!, e: 14, ax: [1, 0, 0] as V3, n: [0, 1, 0] as V3 })) });
const postBox = (s: "l" | "r", from = 0): Box => box(`post-${s}`, `post-${s}`, TIMBER, [side(s) * POST_X, (2.1 + from) / 2, 0], [0.09, 2.1 - from, 0.09]);
const plate = (s: "l" | "r", look: Look): Box => box(`shoe-${s}-plate`, `post-${s}`, look, [side(s) * POST_X, PLATE_T / 2, 0], [0.1, PLATE_T, 0.18]);
const sleeve = (s: "l" | "r", from: number): Ext => upright(`shoe-${s}-sleeve`, `post-${s}`, STEEL, side(s) * POST_X, 0, square(SLEEVE.outer), from, SLEEVE.top, [square(SLEEVE.inner)]);
const anchors = (s: "l" | "r"): Ext[] => ([[-0.03, -0.068], [0.03, -0.068], [-0.03, 0.068], [0.03, 0.068]] as const).map(([dx, dz], i) => upright(`shoe-${s}-anchor-${i + 1}`, `post-${s}`, STEEL_FINE, side(s) * POST_X + dx, dz, hexagon(0.0095), PLATE_T, PLATE_T + 0.009));
const bolts = (s: "l" | "r"): Ext[] => [0.09, 0.2].flatMap((y, i) => [through(`shoe-${s}-bolt-${i + 1}-head`, `post-${s}`, STEEL_FINE, side(s) * POST_X, y, hexagon(0.0095), -SLEEVE.outer - 0.008, -SLEEVE.outer), through(`shoe-${s}-bolt-${i + 1}-nut`, `post-${s}`, STEEL_FINE, side(s) * POST_X, y, hexagon(0.0095), SLEEVE.outer, SLEEVE.outer + 0.0095)]);
const rail = (which: "top" | "bottom", look: Look = TIMBER, corner = 0.004): Box => box(`rail-${which}`, "panel", look, [0, which === "top" ? RAIL.hi : RAIL.lo, 0], [2 * RAIL.x, 0.07, 0.07], corner || undefined);
const lathV = (i: number): Box => box(`lath-v-${i}`, "panel", LATH, [LATH_V_X[i]!, 1.175, -0.005], [0.02, 1.53, 0.01]);
const lathH = (i: number): Box => box(`lath-h-${i}`, "panel", LATH, [0, LATH_H_Y[i]!, 0.005], [2 * RAIL.x, 0.02, 0.01]);
const lathSolids = (look: Look = LATH): Json[] => [
  { ...solidOf({ ...lathV(0), id: "lath-v", look }, F), repeat: { count: 8, translation: [0.2, 0, 0] } },
  { ...solidOf({ ...lathH(0), id: "lath-h", look }, F), repeat: { count: 7, translation: [0, r5(1.51 / 8), 0] } },
];
/** 🧯️ The thermal surface of a sleeve: the closed steel shell (its inner faces lie against the post and exchange no heat with the air). */
const sleeveShell = (s: "l" | "r", from: number): Json => ({ ...solidOf(box(`th-shoe-${s}-sleeve`, `post-${s}`, STEEL, [side(s) * POST_X, (SLEEVE.top + from) / 2, 0], [2 * SLEEVE.outer, SLEEVE.top - from, 2 * SLEEVE.outer]), F), region: "post-shoes" });

/** 🌱️ Main stems: x of the root collar, length (the bone), and the sideways drift at the stations of `STEM_Y` (hand-stated); odd stems pass the first horizontal lath in front, even ones behind, then alternate (woven through the lattice). */
const XS: Record<number, number> = { 1: -0.62, 2: -0.2, 3: 0.22, 4: 0.64 };
const LS: Record<number, number> = { 1: 2.0, 2: 2.05, 3: 2.05, 4: 1.95 };
const FOL_LO_Y: Record<number, number> = { 1: 0.92, 2: 0.82, 3: 0.8, 4: 0.86 };
const STEM_Y = [0, 0.05, 0.2, 0.34, 0.44, 0.52, ...LATH_H_Y, 1.9];
const STEM_DX: Record<number, number[]> = {
  1: [0, 0.004, 0.012, 0.002, -0.01, -0.02, -0.03, -0.012, 0.02, 0.034, 0.012, -0.018, -0.026, -0.008],
  2: [0, -0.005, -0.015, -0.004, 0.008, 0.018, 0.028, 0.03, 0.005, -0.022, -0.03, -0.006, 0.02, 0.012],
  3: [0, 0.003, 0.01, 0.016, 0.018, 0.006, -0.008, -0.03, -0.024, 0.004, 0.028, 0.022, -0.004, -0.014],
  4: [0, -0.003, -0.008, -0.016, -0.02, -0.008, 0.006, 0.026, 0.02, -0.012, -0.032, -0.014, 0.012, 0.006],
};
const stemR = (y: number): number => 0.01 - 0.0065 * Math.min(1, y / 2.05);
function stemStations(k: number): [number, number, number, number][] {
  const out: [number, number, number, number][] = STEM_Y.map((y, i) => {
    const r = stemR(y);
    const weave = i - 6;
    const z = i === 0 ? -0.075 : i === 1 ? -0.074 : i === 2 ? -0.066 : i === 3 ? -0.049 : i === 4 ? -0.047 : i === 5 ? -0.024 : i === STEM_Y.length - 1 ? -0.035 - r - 0.0015 : (weave + k) % 2 === 1 ? -r - 0.0008 : 0.01 + r + 0.0008;
    return [XS[k]! + STEM_DX[k]![i]!, y, z, i === 0 ? 0.0125 : r];
  });
  const top = LS[k]!;
  if (top <= 2.0) out.push([XS[k]!, top, -0.035 - stemR(top) - 0.0015, stemR(top)]);
  else out.push([XS[k]! + 0.004, 2.0, -0.036, stemR(2)], [XS[k]! + 0.002, 2.03, -0.02, stemR(2.03)], [XS[k]!, top, -0.002, stemR(top)]);
  return out;
}
const stemAt = (k: number, y: number): V3 => {
  const st = stemStations(k);
  for (let i = 1; i < st.length; i++) if (y <= st[i]![1]) {
    const [a, b] = [st[i - 1]!, st[i]!];
    const f = (y - a[1]) / (b[1] - a[1]);
    return [a[0] + (b[0] - a[0]) * f, y, a[2] + (b[2] - a[2]) * f];
  }
  const e = st[st.length - 1]!;
  return [e[0], e[1], e[2]];
};
const stem50 = (k: number, id = `stem-${k}`, look: Look = STEM_50): Loft => tube(id, `stem-${k}`, look, stemStations(k));

/** 🌿️ Woody laterals: two per leaf-mat bone, leaving the main stem at the bone joint to the left and to the right, running out and up along the lattice (hand-stated: reach across, rise); each runs on the face on which its stem passes the nearest horizontal lath. */
type Lateral = { id: string; bone: string; k: number; pts: [number, number, number, number][] };
const LATERALS: [number, "lo" | "hi", number, number, number][] = [
  [1, "lo", -1, 0.25, 0.2], [1, "lo", 1, 0.17, 0.24], [1, "hi", -1, 0.27, 0.3], [1, "hi", 1, 0.18, 0.36],
  [2, "lo", -1, 0.18, 0.26], [2, "lo", 1, 0.17, 0.22], [2, "hi", -1, 0.17, 0.38], [2, "hi", 1, 0.18, 0.3],
  [3, "lo", -1, 0.17, 0.22], [3, "lo", 1, 0.18, 0.28], [3, "hi", -1, 0.18, 0.32], [3, "hi", 1, 0.17, 0.4],
  [4, "lo", -1, 0.17, 0.25], [4, "lo", 1, 0.24, 0.21], [4, "hi", -1, 0.18, 0.37], [4, "hi", 1, 0.26, 0.31],
];
const weaveSide = (k: number, y: number): number => {
  const i = LATH_H_Y.reduce((best, h, j) => (Math.abs(h - y) < Math.abs(LATH_H_Y[best]! - y) ? j : best), 0);
  return (i + k) % 2 === 1 ? -1 : 1;
};
function lateral([k, band, s, reach, rise]: (typeof LATERALS)[number], stations = 5): Lateral {
  const y0 = band === "lo" ? FOL_LO_Y[k]! : 1.3;
  const face = weaveSide(k, y0);
  const o = stemAt(k, y0);
  const pts: [number, number, number, number][] = [];
  for (let i = 0; i < stations; i++) {
    const t = i / (stations - 1);
    pts.push([o[0] + s * reach * (1 - (1 - t) ** 1.6), y0 + rise * t ** 1.25 + (i === 0 ? 0 : 0.004), o[2] + (face * 0.018 - o[2]) * Math.min(1, t * 2.2), 0.0048 - 0.0023 * t]);
  }
  return { id: `lateral-${k}-${band}-${s < 0 ? "l" : "r"}`, bone: `fol-${k}-${band}`, k, pts };
}
const lateralTube = (l: Lateral, look: Look = SHOOT, id = l.id): Loft => tube(id, l.bone, look, l.pts);

/** 🍃️ Leaf-mat masses: the inner leaf layers of each mat as cushions (superelliptic lofts on horizontal sections) and, for the small crown sprays, flattened lobes; the outer leaves lie on them. A cushion is stated by its axis x, half width, foot and head height, largest half depth, sideways lean over its height and depth offset; its sections follow the hand-stated profile of its band (share of the height, share of the half width, share of the half depth). */
type Lobe = [string, string, number, number, number, number, number, number, number];
type Cushion = { id: string; bone: string; e: number; cz: number; rows: [number, number, number, number][] };
type Mass = Lobe | Cushion;
const HI_50: [number, number, number][] = [[0, 0.4, 0.25], [0.06, 0.78, 0.6], [0.2, 0.97, 0.88], [0.55, 1, 1], [0.8, 0.97, 0.86], [0.92, 0.86, 0.62], [0.975, 0.62, 0.36], [1, 0.26, 0.12]];
const LO_50: [number, number, number][] = [[0, 0.14, 0.09], [0.025, 0.42, 0.28], [0.08, 0.7, 0.5], [0.24, 0.95, 0.8], [0.6, 1, 1], [0.9, 0.98, 0.9], [1, 0.95, 0.8]];
const HI_100: [number, number, number][] = [[0, 0.7, 0.4], [0.15, 0.97, 0.85], [0.55, 1, 1], [0.9, 0.96, 0.7], [0.975, 0.85, 0.38], [1, 0.6, 0.14]];
const LO_100: [number, number, number][] = [[0, 0.3, 0.12], [0.05, 0.62, 0.34], [0.2, 0.95, 0.75], [0.65, 1, 1], [1, 0.97, 0.8]];
const HI_200: [number, number, number][] = [[0, 0.8, 0.5], [0.3, 1, 1], [0.92, 0.97, 0.75], [1, 0.8, 0.3]];
const LO_200: [number, number, number][] = [[0, 0.5, 0.25], [0.12, 0.9, 0.7], [0.4, 1, 1], [1, 0.97, 0.9]];
const cushion = (id: string, bone: string, profile: [number, number, number][], cx: number, a: number, y0: number, y1: number, b: number, lean: number, cz: number, e = 3): Cushion => ({ id, bone, e, cz, rows: profile.map(([t, fa, fb]) => [r4(y0 + (y1 - y0) * t), r4(cx + lean * (t - 0.5)), r4(a * fa), r4(b * fb)]) });
const MASSES_50: Mass[] = [
  cushion("mat-1-hi-a", "fol-1-hi", HI_50, -0.765, 0.16, 1.24, 2.05, 0.116, 0.02, 0), cushion("mat-1-hi-b", "fol-1-hi", HI_50, -0.545, 0.16, 1.28, 2.035, 0.1, -0.02, -0.004),
  cushion("mat-2-hi-a", "fol-2-hi", HI_50, -0.305, 0.14, 1.27, 2.045, 0.105, -0.015, 0.004), cushion("mat-2-hi-b", "fol-2-hi", HI_50, -0.1, 0.135, 1.23, 2.055, 0.112, 0.02, -0.003),
  cushion("mat-3-hi-a", "fol-3-hi", HI_50, 0.115, 0.135, 1.25, 2.05, 0.108, 0.018, -0.004), cushion("mat-3-hi-b", "fol-3-hi", HI_50, 0.33, 0.14, 1.29, 2.04, 0.098, -0.016, 0.004),
  cushion("mat-4-hi-a", "fol-4-hi", HI_50, 0.545, 0.15, 1.27, 2.04, 0.102, -0.015, 0.003), cushion("mat-4-hi-b", "fol-4-hi", HI_50, 0.765, 0.16, 1.24, 2.05, 0.1235, 0.012, 0),
  cushion("mat-1-lo-a", "fol-1-lo", LO_50, -0.762, 0.16, 0.84, 1.39, 0.1, -0.02, 0), cushion("mat-1-lo-b", "fol-1-lo", LO_50, -0.54, 0.16, 0.77, 1.37, 0.09, 0.015, 0.003),
  cushion("mat-2-lo-a", "fol-2-lo", LO_50, -0.31, 0.14, 0.76, 1.38, 0.092, 0.015, -0.003), cushion("mat-2-lo-b", "fol-2-lo", LO_50, -0.095, 0.135, 0.73, 1.4, 0.1, -0.02, 0.002),
  cushion("mat-3-lo-a", "fol-3-lo", LO_50, 0.11, 0.135, 0.71, 1.39, 0.096, -0.012, 0.003), cushion("mat-3-lo-b", "fol-3-lo", LO_50, 0.33, 0.14, 0.77, 1.37, 0.088, 0.02, -0.002),
  cushion("mat-4-lo-a", "fol-4-lo", LO_50, 0.56, 0.15, 0.74, 1.38, 0.09, 0.018, 0), cushion("mat-4-lo-b", "fol-4-lo", LO_50, 0.762, 0.16, 0.85, 1.4, 0.105, -0.015, 0.003),
  ["crown-l-a", "crown-l", -0.69, 2.046, 0.012, 0.085, 0.034, 0.05, 10], ["crown-l-b", "crown-l", -0.52, 2.044, -0.016, 0.065, 0.03, 0.042, -14],
  ["crown-r-a", "crown-r", 0.71, 2.044, 0.01, 0.08, 0.034, 0.048, -9], ["crown-r-b", "crown-r", 0.545, 2.042, 0.014, 0.06, 0.03, 0.042, 13],
];
const MASSES_100: Mass[] = [
  cushion("mat-1-hi", "fol-1-hi", HI_100, -0.655, 0.295, 1.25, 2.05, 0.118, 0.02, 0, 4), cushion("mat-2-hi", "fol-2-hi", HI_100, -0.2, 0.275, 1.27, 2.055, 0.11, -0.01, 0, 4),
  cushion("mat-3-hi", "fol-3-hi", HI_100, 0.22, 0.275, 1.24, 2.045, 0.106, 0.01, 0, 4), cushion("mat-4-hi", "fol-4-hi", HI_100, 0.655, 0.295, 1.26, 2.05, 0.1245, -0.02, 0, 4),
  cushion("mat-1-lo", "fol-1-lo", LO_100, -0.655, 0.295, 0.82, 1.4, 0.1, 0, 0, 4), cushion("mat-2-lo", "fol-2-lo", LO_100, -0.2, 0.275, 0.75, 1.4, 0.098, 0, 0, 4),
  cushion("mat-3-lo", "fol-3-lo", LO_100, 0.22, 0.275, 0.73, 1.4, 0.095, 0, 0, 4), cushion("mat-4-lo", "fol-4-lo", LO_100, 0.655, 0.295, 0.8, 1.4, 0.1, 0, 0, 4),
  ["crown-l", "crown-l", -0.63, 2.046, 0, 0.13, 0.036, 0.05, 6], ["crown-r", "crown-r", 0.65, 2.044, 0, 0.125, 0.036, 0.05, -6],
];
const MASSES_200: Mass[] = [
  cushion("mat-1-hi", "fol-1-hi", HI_200, -0.65, 0.29, 1.25, 2.04, 0.094, 0, 0, 5), cushion("mat-2-hi", "fol-2-hi", HI_200, -0.2, 0.27, 1.25, 2.04, 0.09, 0, 0, 5),
  cushion("mat-3-hi", "fol-3-hi", HI_200, 0.22, 0.27, 1.25, 2.04, 0.088, 0, 0, 5), cushion("mat-4-hi", "fol-4-hi", HI_200, 0.65, 0.29, 1.25, 2.04, 0.101, 0, 0, 5),
  cushion("mat-1-lo", "fol-1-lo", LO_200, -0.65, 0.29, 0.84, 1.4, 0.082, 0, 0, 5), cushion("mat-2-lo", "fol-2-lo", LO_200, -0.2, 0.27, 0.79, 1.4, 0.08, 0, 0, 5),
  cushion("mat-3-lo", "fol-3-lo", LO_200, 0.22, 0.27, 0.78, 1.4, 0.078, 0, 0, 5), cushion("mat-4-lo", "fol-4-lo", LO_200, 0.65, 0.29, 0.83, 1.4, 0.082, 0, 0, 5),
];
const isLobe = (m: Mass): m is Lobe => Array.isArray(m);
const boneOf = (m: Mass): string => (isLobe(m) ? m[1] : m.bone);
const lobe = (m: Mass): Prim => {
  const look = boneOf(m).endsWith("lo") ? CORE_LO : CORE_HI;
  if (isLobe(m)) return { k: "ell", id: m[0], bone: m[1], look, c: [m[2], m[3], m[4]], r: [m[5], m[6], m[7]], turn: m[8] };
  return { k: "loft", id: m.id, bone: m.bone, look, secs: m.rows.map(([y, cx, a, b]) => ({ c: [cx, y, m.cz] as V3, a, b, e: m.e, ax: [1, 0, 0] as V3, n: [0, 1, 0] as V3 })) };
};

/** 📏️ Depth of the leaf-mat surface on a face (-1 front, +1 back) at a point of the elevation; 0 where no mass lies. */
function depth(masses: Mass[], face: number, x: number, y: number): number {
  let d = 0;
  for (const m of masses) {
    if (isLobe(m)) {
      const [, , cx, cy, cz, rx, ry, rz, turn] = m;
      const t = (turn * Math.PI) / 180;
      const [dx, dy] = [x - cx, y - cy];
      const [u, v] = [dx * Math.cos(t) + dy * Math.sin(t), -dx * Math.sin(t) + dy * Math.cos(t)];
      const q = (u / rx) ** 2 + (v / ry) ** 2;
      if (q < 1) d = Math.max(d, face * cz + rz * Math.sqrt(1 - q));
      continue;
    }
    for (let i = 1; i < m.rows.length; i++) {
      const [p, q] = [m.rows[i - 1]!, m.rows[i]!];
      if (y < p[0] || y > q[0]) continue;
      const f = (y - p[0]) / (q[0] - p[0]);
      const [cx, a, b] = [p[1] + (q[1] - p[1]) * f, p[2] + (q[2] - p[2]) * f, p[3] + (q[3] - p[3]) * f];
      const u = Math.abs(x - cx) / a;
      if (u < 1) d = Math.max(d, face * m.cz + b * (1 - u ** m.e) ** (1 / m.e));
    }
  }
  return d;
}

/** 🍀️ The blade of a juvenile ivy leaf (Hedera helix), unit width: cordate base at the origin, two basal and two lateral lobes, the terminal lobe pointing to -y. */
const BLADE_5: [number, number][] = [[0, 0], [0.12, 0.08], [0.33, 0.1], [0.47, -0.02], [0.39, -0.17], [0.5, -0.37], [0.29, -0.47], [0, -0.9], [-0.29, -0.47], [-0.5, -0.37], [-0.39, -0.17], [-0.47, -0.02], [-0.33, 0.1], [-0.12, 0.08]];
const BLADE_3: [number, number][] = [[0, 0], [0.34, 0.08], [0.5, -0.24], [0.27, -0.44], [0, -0.9], [-0.27, -0.44], [-0.5, -0.24], [-0.34, 0.08]];
const SIZES = [0.072, 0.06, 0.083, 0.066, 0.078, 0.058, 0.088, 0.069, 0.075, 0.063, 0.081];
const YAWS = [22, 35, 12, 41, 28, 17, 33, 8, 38, 25, 15, 30, 19];
const TILTS = [24, 31, 18, 27, 35, 21, 29, 16, 33, 23];
const ROLLS = [-8, 5, 12, -4, 9, -12, 3, -6, 10, 0, 7, -9];
type Leaf = { prim: Ext; face: number; ring: V3[] };
const rad = (d: number): number => (d * Math.PI) / 180;

/** 🍂️ One leaf: base point on the elevation, face, size, yaw of the tip off the plumb line, outward tilt of the tip, roll; it lies on the mat surface like a roof tile (base tucked in, tip out) and stays inside the declared depth and width. */
function leaf(id: string, bone: string, look: Look, lobes: Mass[], blade: [number, number][], face: number, x: number, y: number, s: number, yawDeg: number, tiltDeg: number, rollDeg: number, floor = 0.014): Leaf | null {
  const n0: V3 = [0, 0, face];
  const up0: V3 = [Math.sin(rad(yawDeg)), Math.cos(rad(yawDeg)), 0];
  const tipAt = (f: number): [number, number] => [x - up0[0] * 0.9 * s * f, y - up0[1] * 0.9 * s * f];
  let sinT = Math.sin(rad(tiltDeg));
  const at = (f: number): number => depth(lobes, face, ...tipAt(f));
  let base = Math.max(floor, at(0) - 0.003, at(0.5) - 0.45 * s * sinT, at(1) - 0.9 * s * sinT) + 0.002;
  const inPost = Math.abs(Math.abs(x) - POST_X) < POST_H + 0.035;
  const inRail = Math.abs(y - RAIL.hi) < 0.07 || Math.abs(y - RAIL.lo) < 0.07;
  if (inPost) base = Math.max(base, POST_H + 0.004);
  else if (inRail) base = Math.max(base, RAIL.half + 0.004);
  base = Math.min(base, 0.1195);
  sinT = Math.max(0.03, Math.min(sinT, (0.1242 - base) / (0.9 * s)));
  const cosT = Math.sqrt(1 - sinT * sinT);
  const upT: V3 = add(mul(up0, cosT), mul(n0, -sinT));
  const nT: V3 = add(mul(n0, cosT), mul(up0, sinT));
  const ex0 = cross3(upT, nT);
  const limit = Math.min(Math.abs(rollDeg), (Math.asin(Math.min(1, Math.max(0, (0.1242 - base - 0.2 * s * sinT) / (0.5 * s)))) * 180) / Math.PI);
  const roll = rad(Math.sign(rollDeg) * limit);
  const ex = unit(add(mul(ex0, Math.cos(roll)), mul(nT, Math.sin(roll))));
  const place = (o: V3): V3[] => blade.map(([a, b]): V3 => add(o, add(mul(ex, a * s), mul(upT, b * s))));
  const first = place([x, y, face * base]);
  const over = [Math.max(0, ...first.map((p) => p[0] - 0.957)) - Math.max(0, ...first.map((p) => -0.957 - p[0])), Math.max(0, ...first.map((p) => p[1] - 2.102))];
  if (Math.abs(over[0]!) > 0.05 || over[1]! > 0.04) return null;
  const o: V3 = [x - over[0]!, y - over[1]!, face * base];
  const ring = place(o);
  if (ring.some((p) => Math.abs(p[0]) > 0.958 || p[1] > 2.103 || Math.abs(p[2]) > 0.1247)) return null;
  return { prim: { k: "ext", id, bone, look, profile: blade.map(([a, b]) => [a * s, b * s]), o, ex, ey: upT, from: -0.0006, to: 0.0006 }, face, ring };
}

/** 🌱️ Leaf rows: the green shoots (3 mm, below the smallest feature, not modelled) leave each main stem to both sides on both faces and run out and up; the leaves sit on them alternately (distichous), one every `step`. */
const ROW_ANGLES = [40, 46, 35, 49, 38, 44, 33, 47, 42, 36, 45];
const REACH: Record<number, [number, number]> = { 1: [0.32, 0.25], 2: [0.25, 0.25], 3: [0.25, 0.25], 4: [0.25, 0.31] };
function rows(lobes: Mass[], blade: [number, number][], rowStep: number, step: number, grow: number, crownTop: number): Leaf[] {
  const out: Leaf[] = [];
  for (const k of K) for (const face of [-1, 1]) for (const s of [-1, 1]) {
    const start = 0.5 + (s < 0 ? 0 : rowStep / 2) + (face < 0 ? 0 : rowStep / 4) + 0.01 * k;
    for (let j = 0, y0 = start; y0 < 1.99; j++, y0 += rowStep) {
      const a = rad(ROW_ANGLES[(j + 2 * k + (face < 0 ? 0 : 5)) % ROW_ANGLES.length]!);
      const len = REACH[k]![s < 0 ? 0 : 1] / Math.cos(a);
      const o = stemAt(k, y0);
      const h = crownTop - y0;
      for (let i = 0, t = 0.03; t <= len + 0.012; i++, t += step) {
        const n = i + 3 * j + 5 * k + (face < 0 ? 0 : 7) + (s < 0 ? 0 : 4);
        const px = o[0] + s * t * Math.cos(a);
        const py = y0 + h * (1 - Math.exp((-t * Math.sin(a)) / h));
        const off = i % 2 === 0 ? 1 : -1;
        const x = px - off * 0.014 * Math.sin(a) * s;
        const y = py + off * 0.014 * Math.cos(a);
        const size = SIZES[n % SIZES.length]! * grow;
        if (depth(lobes, face, x, y - 0.4 * size) <= 0 && depth(lobes, face, x, y + 0.05) <= 0) continue;
        const band = y < 1.3 ? "lo" : "hi";
        const l = leaf(`leaf-${k}${face < 0 ? "f" : "b"}${s < 0 ? "l" : "r"}-${j}-${i}`, `fol-${k}-${band}`, band === "lo" ? LEAF_LO : LEAF_HI, lobes, blade, face, x, y, size, off * s * YAWS[n % YAWS.length]!, TILTS[n % TILTS.length]!, ROLLS[n % ROLLS.length]!);
        if (l) out.push(l);
      }
    }
  }
  return out;
}

/** 🪴️ Trailing shoots at the ragged lower edge of the mats (start, tip, face) and the overhanging shoots on the two crown bones (stations x y z). */
const TRAILERS: [number, number, number, number, number, number][] = [
  [1, -0.8, 0.9, -0.83, 0.64, -1], [1, -0.5, 0.8, -0.47, 0.6, 1], [2, -0.34, 0.79, -0.37, 0.57, -1], [2, -0.06, 0.83, -0.04, 0.63, 1],
  [3, 0.08, 0.8, 0.05, 0.6, 1], [3, 0.36, 0.77, 0.39, 0.55, -1], [4, 0.5, 0.82, 0.52, 0.62, -1], [4, 0.83, 0.88, 0.86, 0.61, 1],
];
const trailer = ([k, x0, y0, x1, y1, face]: (typeof TRAILERS)[number], i: number): Loft => tube(`trailer-${k}-${i % 2 === 0 ? "a" : "b"}`, `fol-${k}-lo`, SHOOT, [[x0, y0, face * 0.03, 0.003], [x0 + (x1 - x0) * 0.2 - 0.012 * face, y0 + (y1 - y0) * 0.4, face * 0.024, 0.0028], [x0 + (x1 - x0) * 0.7 + 0.008 * face, y0 + (y1 - y0) * 0.75, face * 0.021, 0.0025], [x1, y1, face * 0.02, 0.0022]]);
function trailerLeaves(lobes: Mass[], blade: [number, number][], every: number, grow: number): Leaf[] {
  const out: Leaf[] = [];
  TRAILERS.forEach(([k, x0, y0, x1, y1, face], i) => {
    const n = Math.round((y0 - y1) / every);
    for (let j = 0; j <= n; j++) {
      const f = j / n;
      const m = j + 3 * i;
      const off = j % 2 === 0 ? 1 : -1;
      const l = leaf(`leaf-trail-${k}${i % 2 === 0 ? "a" : "b"}-${j}`, `fol-${k}-lo`, LEAF_LO, lobes, blade, face, x0 + (x1 - x0) * f + off * 0.016, y0 + (y1 - y0) * f, SIZES[(m + 2) % SIZES.length]! * grow * (1 - 0.25 * f), off * YAWS[m % YAWS.length]!, TILTS[m % TILTS.length]! * 0.7, ROLLS[m % ROLLS.length]!, 0.022);
      if (l) out.push(l);
    }
  });
  return out;
}
const CROWN_SHOOTS: Record<string, [number, number, number][]> = {
  "crown-l-shoot-a": [[-0.62, 1.95, -0.041], [-0.64, 2.02, -0.036], [-0.68, 2.068, -0.004], [-0.74, 2.062, 0.03], [-0.775, 2.03, 0.047]],
  "crown-l-shoot-b": [[-0.62, 1.95, -0.041], [-0.59, 2.03, -0.034], [-0.54, 2.066, -0.012], [-0.49, 2.052, -0.036], [-0.462, 2.012, -0.052]],
  "crown-r-shoot-a": [[0.64, 1.9, -0.041], [0.655, 2.0, -0.04], [0.69, 2.064, -0.006], [0.74, 2.058, 0.028], [0.772, 2.024, 0.046]],
  "crown-r-shoot-b": [[0.64, 1.9, -0.041], [0.61, 2.01, -0.04], [0.57, 2.062, -0.008], [0.52, 2.05, 0.022], [0.492, 2.012, 0.045]],
};
const crownShoot = (id: string, look: Look = SHOOT, name = id): Loft => tube(name, id.slice(0, 7), look, CROWN_SHOOTS[id]!.map(([x, y, z], i) => [x, y, z, 0.0035 - 0.0003 * i]));
function crownLeaves(lobes: Mass[], blade: [number, number][], grow: number, per: number): Leaf[] {
  const out: Leaf[] = [];
  Object.entries(CROWN_SHOOTS).forEach(([id, pts], si) => {
    for (let j = 0; j < per; j++) {
      const f = 0.28 + (0.72 * j) / (per - 1);
      const g = f * (pts.length - 1);
      const [a, b] = [pts[Math.min(pts.length - 2, Math.floor(g))]!, pts[Math.min(pts.length - 1, Math.floor(g) + 1)]!];
      const h = g - Math.min(pts.length - 2, Math.floor(g));
      const p = [a[0] + (b[0] - a[0]) * h, a[1] + (b[1] - a[1]) * h, a[2] + (b[2] - a[2]) * h];
      const m = j + 4 * si;
      const off = j % 2 === 0 ? 1 : -1;
      const face = p[2]! + off * 0.01 < 0 ? -1 : 1;
      const l = leaf(`leaf-${id.slice(0, 7)}-${id.slice(-1)}-${j}`, id.slice(0, 7), LEAF_HI, lobes, blade, face, p[0]! + off * 0.02, p[1]! + 0.012, SIZES[(m + 5) % SIZES.length]! * grow * 0.85, off * YAWS[(m + 3) % YAWS.length]! * 1.4, TILTS[m % TILTS.length]!, ROLLS[(m + 2) % ROLLS.length]!, 0.02);
      if (l) out.push(l);
    }
  });
  return out;
}
/** 🌿️ Leaf sprays of the coarse scales: where single leaves are below the smallest feature, groups of leaves are one leaf-shaped flake each (the blade of the ivy leaf about its centre, unit width), a tapered slab one feature thick lying on the mat, set in staggered rows with hand-stated offsets, turns and sizes on both faces. */
const SPRAY: [number, number][] = [[0, 0.36], [0.14, 0.45], [0.34, 0.47], [0.49, 0.33], [0.4, 0.17], [0.52, -0.04], [0.3, -0.13], [0, -0.55], [-0.3, -0.13], [-0.52, -0.04], [-0.4, 0.17], [-0.49, 0.33], [-0.34, 0.47], [-0.14, 0.45]];
const SPRAY_SHIFT = [0.012, -0.02, 0.026, -0.008, 0.018, -0.028, 0.004, -0.014, 0.022];
const SPRAY_GROW = [1, 0.86, 1.14, 0.92, 1.06, 0.82, 1.1];
type Spray = { solid: Json; prim: Ext };
function sprays(masses: Mass[], boneAt: (x: number, y: number) => string, dx: number, dy: number, size: number, thick: number, y0: number): Spray[] {
  const out: Spray[] = [];
  for (const face of [-1, 1]) {
    let row = 0;
    for (let y = y0 + (face < 0 ? 0 : dy * 0.4); y < 2.08; y += dy, row++) {
      let i = 0;
      for (let x = -0.9 + (row % 2 === 0 ? 0 : dx / 2) + (face < 0 ? 0 : dx * 0.3); x <= 0.91; x += dx, i++) {
        const n = i + 3 * row + (face < 0 ? 0 : 4);
        const s = size * SPRAY_GROW[n % SPRAY_GROW.length]!;
        const px = Math.max(-0.955 + 0.6 * s, Math.min(0.955 - 0.6 * s, x + SPRAY_SHIFT[n % SPRAY_SHIFT.length]! * (dx / 0.15)));
        const py = Math.min(2.1 - 0.6 * s, y + SPRAY_SHIFT[(n + 4) % SPRAY_SHIFT.length]! * (dy / 0.13));
        const d = depth(masses, face, px, py);
        if (d < 0.02) continue;
        const base = Math.min(d - 0.006, 0.1245 - thick);
        const turn = rad((n % 2 === 0 ? 1 : -1) * YAWS[n % YAWS.length]! * 1.5);
        const ex: V3 = [face * Math.cos(turn), -face * Math.sin(turn), 0];
        const ey: V3 = [Math.sin(turn), Math.cos(turn), 0];
        const bone = boneAt(px, py);
        const prim: Ext = { k: "ext", id: `spray-${face < 0 ? "f" : "b"}-${row}-${i}`, bone, look: bone.endsWith("lo") ? CORE_LO : CORE_HI, profile: SPRAY.map(([a, b]) => [a * s, b * s]), o: [px, py, face * base], ex, ey, from: 0, to: thick };
        out.push({ prim, solid: { ...solidOf(prim, F), taperTo: [1, 0.6] } });
      }
    }
  }
  return out;
}
const folAt = (x: number, y: number): string => `fol-${x < -0.41 ? 1 : x < 0.01 ? 2 : x < 0.43 ? 3 : 4}-${y < 1.3 ? "lo" : "hi"}`;

/** 🧢️ The leaves that lie over the top of the mats: one every `step` along the ridge, alternately tipping to the front and to the back, their tips drooping over the edge. */
function ridgeLeaves(lobes: Mass[], blade: [number, number][], step: number, grow: number): Leaf[] {
  const out: Leaf[] = [];
  for (let i = 0, x = -0.925; x <= 0.926; i++, x += step) {
    let top = 2.1;
    while (top > 1.9 && Math.max(depth(lobes, -1, x, top), depth(lobes, 1, x, top)) < 0.03) top -= 0.004;
    if (top <= 1.9) continue;
    const face = i % 2 === 0 ? -1 : 1;
    const s = SIZES[(i * 3) % SIZES.length]! * grow;
    const yaw = rad(face * YAWS[i % YAWS.length]! * 0.8);
    const droop = rad(8 + TILTS[i % TILTS.length]! * 0.5);
    const ey: V3 = unit([Math.sin(yaw) * Math.cos(droop), Math.sin(droop), -face * Math.cos(yaw) * Math.cos(droop)]);
    const ex = unit(cross3(ey, [0, 1, 0]));
    const k = x < -0.41 ? 1 : x < 0.01 ? 2 : x < 0.43 ? 3 : 4;
    const o: V3 = [x, Math.min(top + 0.006, 2.1 - 0.12 * s), face * 0.012];
    const ring = blade.map(([a, b]): V3 => add(o, add(mul(ex, a * s), mul(ey, b * s))));
    if (ring.some((q) => Math.abs(q[0]) > 0.958 || q[1] > 2.103 || Math.abs(q[2]) > 0.1247)) continue;
    out.push({ prim: { k: "ext", id: `leaf-ridge-${i}`, bone: `fol-${k}-hi`, look: LEAF_HI, profile: blade.map(([a, b]) => [a * s, b * s]), o, ex, ey, from: -0.0006, to: 0.0006 }, face: 0, ring });
  }
  return out;
}
/** 🌾️ The few leaves on the bare lower stems (height, face side). */
function stemLeaves(lobes: Mass[], blade: [number, number][]): Leaf[] {
  const out: Leaf[] = [];
  for (const k of K) [0.5, 0.6, 0.68, 0.74].forEach((y, j) => {
    const o = stemAt(k, y + 0.01 * k);
    const off = (j + k) % 2 === 0 ? 1 : -1;
    const l = leaf(`leaf-stem-${k}-${j}`, `stem-${k}`, LEAF_LO, lobes, blade, -1, o[0] + off * 0.02, o[1], SIZES[(j + 2 * k) % SIZES.length]! * 0.85, off * YAWS[(j + k) % YAWS.length]!, TILTS[(j + 3 * k) % TILTS.length]! * 0.6, ROLLS[(j + k) % ROLLS.length]!, Math.max(0.03, -o[2] + 0.012));
    if (l) out.push(l);
  });
  return out;
}

const local = (bone: string, p: V3): V3 => toLocal(F[bone]!, p).map(r5) as V3;
const S = (p: Prim, extra: Json = {}): Json => ({ ...solidOf(p, F), ...extra });
const th = (id: string, region: string): Json => ({ id: `th-${id}`, of: id, region });
const st = (id: string, member: string, part: string, extra: Json = {}): Json => ({ id: `${part}-${id}`, of: id, member, part, ...extra });
const ball = (id: string, bone: string, look: Look, c: V3, r: number | V3): Ell => ({ k: "ell", id, bone, look, c, r: typeof r === "number" ? [r, r, r] : r });
const JOINT: Look = { material: "steel-s235", tone: 0.6, role: "detail" };
const TIE: Look = { material: "ivy-stem", tone: 0.5, role: "detail" };
const MASS: Look = { material: "timber-c24", tone: 0.3, role: "detail" };
const WORLD_ANCHOR = (id: string): V3 => {
  const a = asset.anchors.find((x: Json) => x.id === id);
  const f = F[a.bone]!;
  return [f.o[0] + f.u[0] * a.x + f.v[0] * a.y, f.o[1] + f.u[1] * a.x + f.v[1] * a.y, 0];
};

/** 💧️ Transpiration outlets (D2): one lens per emitter anchor, lying on the front face of the mat where the anchor points (on the crown masses: on their top). */
function outlets(lobes: Mass[], scale: number, which: "all" | "first"): Json[] {
  const thick = Math.max(0.004, (0.15 * scale) / 2000);
  const out: Json[] = [];
  for (const em of asset.thermal.emitters) {
    const ids: string[] = which === "all" ? em.anchors : [em.anchor];
    for (const id of ids) {
      const a = asset.anchors.find((x: Json) => x.id === id);
      const p = WORLD_ANCHOR(id);
      const crown = String(a.bone).startsWith("crown");
      const d = depth(lobes, -1, p[0], p[1]);
      const c: V3 = crown ? [p[0], p[1], 0] : [p[0], p[1], -(Math.min(d + 0.001, 0.1245) - thick)];
      out.push(S(ball(`th-${em.id}-${id.replace(/^tp-(upper|lower)-/, "")}`, a.bone, { material: "ivy-leaf", tone: 0.5, role: "detail" }, c, crown ? [0.045, thick, 0.03] : [0.075, 0.075, thick]), { region: em.region, tags: [em.id] }));
    }
  }
  return out;
}
/** 🎯️ Crown outlets sit on the top of the crown mass: the anchor is moved there by `roots()`, this gives the point. */
const crownTop = (lobes: Mass[], bone: string): V3 => {
  const l = lobes.find((x): x is Lobe => isLobe(x) && x[1] === bone)!;
  return [l[2], l[3] + l[6] - 0.002, l[4]];
};

type Scale = { volume: Json; fills: { id: string; bone: string; role: string; z: number; tone: number; region?: string; regions?: string[]; prims: Prim[]; views?: View[] }[]; strokes: { id: string; bone: string; role: string; z: number; tone: number; region?: string; penMm?: number; line: V3[]; views?: View[] }[]; textures: { id: string; bone: string; z: number; tone: number; region?: string; rings: V3[][]; views: View[] }[] };
const leafSolids = (leaves: Leaf[]): Json[] => leaves.map((l) => S(l.prim));
const leafRefs = (leaves: Leaf[]): Json[] => leaves.map((l) => th(l.prim.id, l.prim.bone.endsWith("lo") || l.prim.bone.startsWith("stem") ? "leaves-lower" : "leaves-upper"));
const matRegion = (bone: string): string => (bone.endsWith("lo") ? "leaves-lower" : "leaves-upper");
function leafTextures(leaves: Leaf[], bones: string[], every: number): Scale["textures"] {
  const out: Scale["textures"] = [];
  for (const bone of bones) {
    const mine = leaves.filter((l) => l.prim.bone === bone);
    const front = mine.filter((l, i) => l.face < 0 && i % every === 0);
    if (front.length) out.push({ id: `leaves-${bone}`, bone, z: 7, tone: 0.15, region: bone.startsWith("stem") ? undefined : matRegion(bone), rings: front.map((l) => l.ring), views: ["elevation"] });
    const edge = mine.filter((l, i) => i % every === 0 && Math.max(...l.ring.map((p) => p[0])) > 0.86);
    if (edge.length) out.push({ id: `leaves-${bone}-end`, bone, z: 7.1, tone: 0.15, region: matRegion(bone), rings: edge.map((l) => l.ring), views: ["front"] });
    const top = mine.filter((l, i) => l.face === 0 || i % every === 0 && Math.max(...l.ring.map((p) => p[1])) > 1.99 && bone.includes("hi") || (bone.startsWith("crown") && i % every === 0));
    if (top.length) out.push({ id: `leaves-${bone}-top`, bone, z: 7.2, tone: 0.15, region: matRegion(bone), rings: top.map((l) => l.ring), views: ["plan"] });
  }
  return out;
}
const FOL_BONES = K.flatMap((k) => [`fol-${k}-lo`, `fol-${k}-hi`]);
const stemLine = (k: number, ys: number[]): V3[] => ys.map((y) => stemAt(k, y));
const trellisFills = (posts: Prim[], shoes: Prim[][], rails: Prim[], fine: Prim[] = []): Scale["fills"] => [
  { id: "post-l", bone: "post-l", role: "body", z: 2, tone: 0.55, region: "trellis-timber", prims: [posts[0]!] },
  { id: "post-r", bone: "post-r", role: "body", z: 2.01, tone: 0.55, region: "trellis-timber", prims: [posts[1]!] },
  ...(rails.length ? [{ id: "rail-top", bone: "panel", role: "body", z: 3, tone: 0.55, region: "trellis-timber", prims: [rails[0]!] }, { id: "rail-bottom", bone: "panel", role: "body", z: 3.01, tone: 0.55, region: "trellis-timber", prims: [rails[1]!] }] : []),
  ...(shoes.length ? [{ id: "shoe-l", bone: "post-l", role: "body", z: 4, tone: 0.75, region: "post-shoes", prims: shoes[0]! }, { id: "shoe-r", bone: "post-r", role: "body", z: 4.01, tone: 0.75, region: "post-shoes", prims: shoes[1]! }] : []),
  ...fine.map((p, i) => ({ id: p.id, bone: p.bone, role: "detail", z: 4.2 + i / 1000, tone: 0.5, region: "post-shoes", prims: [p], views: (p.id.includes("anchor") ? ["plan", "front", "elevation"] : ["elevation", "plan"]) as View[] })),
];
const matFills = (lobes: Mass[], extra: Prim[] = []): Scale["fills"] => [...new Set(lobes.map(boneOf))].map((bone, i) => ({ id: bone.startsWith("crown") ? `${bone}-mass` : `mat-${bone.slice(4)}`, bone, role: "body", z: (bone.endsWith("lo") ? 6 : bone.startsWith("crown") ? 6.2 : 6.1) + i / 1000, tone: bone.endsWith("lo") ? 0.2 : 0.24, region: matRegion(bone), prims: [...lobes.filter((l) => boneOf(l) === bone).map(lobe), ...extra.filter((e) => e.bone === bone)] }));
const supports = (ids: [string, string]): Json[] => [st(ids[0], "post-l", "support", { anchor: "base-support-l" }), st(ids[1], "post-r", "support", { anchor: "base-support-r" })].map((s, i) => ({ ...s, id: `support-base-support-${i === 0 ? "l" : "r"}` }));
const masses = (r: number): Json[] => [
  S(ball("mass-post-l", "post-l", MASS, [-POST_X, 1.05, 0], r), { member: "post-l", part: "mass" }),
  S(ball("mass-post-r", "post-r", MASS, [POST_X, 1.05, 0], r), { member: "post-r", part: "mass" }),
  S(ball("mass-panel", "panel", MASS, [0, 1.175, 0], r * 1.25), { member: "panel", part: "mass" }),
];

const lathFills = (): Scale["fills"] => [
  ...LATH_V_X.map((_, i) => ({ id: `lath-v-${i}`, bone: "panel", role: "body", z: 2.5 + i / 1000, tone: 0.6, region: "trellis-timber", prims: [lathV(i)] as Prim[], views: ["elevation"] as View[] })),
  ...LATH_H_Y.map((_, i) => ({ id: `lath-h-${i}`, bone: "panel", role: "body", z: 2.4 + i / 1000, tone: 0.6, region: "trellis-timber", prims: [lathH(i)] as Prim[], views: ["elevation"] as View[] })),
];

function scale50(): Scale {
  const posts = [post50("l"), post50("r")];
  const shoes = (["l", "r"] as const).map((s) => [plate(s, STEEL), sleeve(s, PLATE_T)]);
  const fine = (["l", "r"] as const).flatMap((s) => [...anchors(s), ...bolts(s)]);
  const rails = [rail("top"), rail("bottom")];
  const stems = K.map((k) => stem50(k));
  const lats = LATERALS.map((l) => lateral(l));
  const trail = TRAILERS.map(trailer);
  const crowns = Object.keys(CROWN_SHOOTS).map((id) => crownShoot(id));
  const cores = MASSES_50.map(lobe);
  const leaves = [...rows(MASSES_50, BLADE_5, 0.09, 0.04, 1, 2.05), ...trailerLeaves(MASSES_50, BLADE_5, 0.05, 1), ...crownLeaves(MASSES_50, BLADE_5, 1, 8), ...ridgeLeaves(MASSES_50, BLADE_5, 0.045, 1), ...stemLeaves(MASSES_50, BLADE_5)];
  const body = [...posts.map((p) => S(p)), ...shoes.flat().map((p) => S(p)), ...fine.map((p) => S(p)), ...rails.map((p) => S(p)), ...lathSolids(LATH_50), ...stems.map((p) => S(p)), ...trail.map((p) => S(p)), ...crowns.map((p) => S(p)), ...cores.map((p) => S(p)), ...leafSolids(leaves)];
  const thermal = [
    ...cores.map((p) => th(p.id, matRegion(p.bone))), ...leafRefs(leaves),
    ...K.map((k) => S(stem50(k, `th-stem-${k}`), { region: "stems" })), ...lats.map((l) => S(lateralTube(l, SHOOT, `th-${l.id}`), { region: "stems" })),
    ...["post-l", "post-r", "rail-top", "rail-bottom", "lath-v", "lath-h"].map((id) => th(id, "trellis-timber")),
    th("shoe-l-plate", "post-shoes"), th("shoe-r-plate", "post-shoes"), sleeveShell("l", PLATE_T), sleeveShell("r", PLATE_T), ...fine.map((p) => th(p.id, "post-shoes")),
    ...outlets(MASSES_50, 50, "all"),
  ];
  const structure = [
    st("post-l", "post-l", "member"), st("post-r", "post-r", "member"), st("shoe-l-sleeve", "post-l", "joint"), st("shoe-r-sleeve", "post-r", "joint"), ...supports(["shoe-l-plate", "shoe-r-plate"]),
    st("rail-top", "panel", "member"), st("rail-bottom", "panel", "member"), st("lath-v", "panel", "member"), st("lath-h", "panel", "member"),
    ...[[-1, RAIL.hi, "top-l"], [1, RAIL.hi, "top-r"], [-1, RAIL.lo, "bottom-l"], [1, RAIL.lo, "bottom-r"]].map(([s, y, n]) => S(box(`joint-rail-${n}`, "panel", JOINT, [(s as number) * (RAIL.x - 0.012), y as number, 0], [0.024, 0.05, 0.05], 0.004), { member: "panel", part: "joint" })),
    ...K.flatMap((k) => [st(`stem-${k}`, `stem-${k}`, "member"), S(ball(`joint-stem-${k}-collar`, `stem-${k}`, TIE, add(stemAt(k, 0.012), [0, 0, 0]), [0.0145, 0.01, 0.0145]), { member: `stem-${k}`, part: "joint" }), ...[1, 4].map((i) => S(ball(`joint-stem-${k}-tie-${i}`, `stem-${k}`, TIE, [stemAt(k, LATH_H_Y[i]!)[0], LATH_H_Y[i]!, 0.005], [0.0105, 0.012, 0.02]), { member: `stem-${k}`, part: "joint" }))]),
    ...lats.map((l) => S(lateralTube(l, SHOOT, `member-${l.id}`), { member: l.bone, part: "member" })),
    ...FOL_BONES.map((b) => { const k = Number(b[4]); const y = b.endsWith("lo") ? FOL_LO_Y[k]! : 1.3; return S(ball(`joint-${b}`, b, TIE, stemAt(k, y), 0.0085), { member: b, part: "joint" }); }),
    ...crowns.map((c) => st(c.id, c.bone, "member")),
    ...(["crown-l", "crown-r"] as const).map((b) => S(ball(`joint-${b}`, b, TIE, CROWN_SHOOTS[`${b}-shoot-a`]![0]!, 0.009), { member: b, part: "joint" })),
    ...masses(0.036),
  ];
  return {
    volume: { status: "authored", denominator: 50, body, thermal, structure, decisions: DECISIONS_3D[50] },
    fills: [
      ...trellisFills(posts, shoes, rails, fine),
      ...lathFills(),
      ...stems.map((p, i) => ({ id: p.id, bone: p.bone, role: "body", z: 5 + i / 1000, tone: 0.4, region: "stems", prims: [p] as Prim[], views: ["elevation", "front"] as View[] })),
      ...matFills(MASSES_50),
    ],
    strokes: [
      ...trail.map((p, i) => ({ id: p.id, bone: p.bone, role: "detail", z: 5.5 + i / 1000, tone: 0.4, region: "stems", line: p.secs.map((s) => s.c) })),
      ...crowns.map((p, i) => ({ id: p.id, bone: p.bone, role: "detail", z: 5.6 + i / 1000, tone: 0.4, region: "stems", line: p.secs.map((s) => s.c) })),
    ],
    textures: leafTextures(leaves, [...FOL_BONES, "crown-l", "crown-r", ...K.map((k) => `stem-${k}`)], 3),
  };
}

function scale100(): Scale {
  const posts = [postBox("l"), postBox("r")];
  const shoes = (["l", "r"] as const).map((s) => [plate(s, PLATE_GROUND), sleeve(s, PLATE_T)]);
  const rails = [rail("top", TIMBER, 0), rail("bottom", TIMBER, 0)];
  const ys = [0, 0.2, 0.44, 0.62, 1.08, 1.46, 1.88, 1.95, 2.05];
  const stems = K.map((k) => tube(`stem-${k}`, `stem-${k}`, STEM, stemLine(k, climb(k, ys)).map((p, i): [number, number, number, number] => [p[0], p[1], i < 3 ? p[2] : slimZ(p[1]), stemR(p[1])])));
  const lats = LATERALS.filter((l) => l[2] === (l[0] % 2 === 0 ? 1 : -1)).map((l) => lateral(l, 3));
  const crowns = ["crown-l-shoot-a", "crown-r-shoot-a"].map((id) => crownShoot(id, SHOOT, id.slice(0, 13)));
  const cores = MASSES_100.map(lobe);
  const leaves = [...rows(MASSES_100, BLADE_3, 0.13, 0.06, 1.25, 2.045), ...trailerLeaves(MASSES_100, BLADE_3, 0.07, 1.25), ...crownLeaves(MASSES_100, BLADE_3, 1.2, 3).filter((l) => l.prim.id.includes("-a-")), ...ridgeLeaves(MASSES_100, BLADE_3, 0.07, 1.25)];
  const body = [...posts.map((p) => S(p)), ...shoes.flat().map((p) => S(p)), ...rails.map((p) => S(p)), ...lathSolids(), ...stems.map((p) => S(p)), ...cores.map((p) => S(p)), ...leafSolids(leaves)];
  const thermal = [
    ...cores.map((p) => th(p.id, matRegion(p.bone))), ...leafRefs(leaves),
    ...stems.map((p) => S({ ...p, id: `th-${p.id}` }, { region: "stems" })), ...LATERALS.map((l) => lateral(l, 3)).map((l) => S(lateralTube(l, SHOOT, `th-${l.id}`), { region: "stems" })),
    ...["post-l", "post-r", "rail-top", "rail-bottom", "lath-v", "lath-h"].map((id) => th(id, "trellis-timber")),
    th("shoe-l-plate", "post-shoes"), th("shoe-r-plate", "post-shoes"), sleeveShell("l", PLATE_T), sleeveShell("r", PLATE_T),
    ...outlets(MASSES_100, 100, "all"),
  ];
  const structure = [
    st("post-l", "post-l", "member"), st("post-r", "post-r", "member"), st("shoe-l-sleeve", "post-l", "joint"), st("shoe-r-sleeve", "post-r", "joint"), ...supports(["shoe-l-plate", "shoe-r-plate"]),
    st("rail-top", "panel", "member"), st("rail-bottom", "panel", "member"), st("lath-v", "panel", "member"), st("lath-h", "panel", "member"),
    ...[[-1, RAIL.hi, "top-l"], [1, RAIL.hi, "top-r"], [-1, RAIL.lo, "bottom-l"], [1, RAIL.lo, "bottom-r"]].map(([s, y, n]) => S(box(`joint-rail-${n}`, "panel", JOINT, [(s as number) * (RAIL.x - 0.012), y as number, 0], [0.024, 0.05, 0.05]), { member: "panel", part: "joint" })),
    ...K.map((k) => st(`stem-${k}`, `stem-${k}`, "member")),
    ...lats.map((l) => S(lateralTube(l), { member: l.bone, part: "member" })),
    ...FOL_BONES.map((b) => { const k = Number(b[4]); const y = b.endsWith("lo") ? FOL_LO_Y[k]! : 1.3; const p = stemAt(k, y); return S(ball(`joint-${b}`, b, TIE, [p[0], y, -0.012], 0.014), { member: b, part: "joint" }); }),
    ...crowns.map((c) => S(c, { member: c.bone, part: "member" })),
    ...masses(0.036),
  ];
  return {
    volume: { status: "authored", denominator: 100, body, thermal, structure, decisions: DECISIONS_3D[100] },
    fills: [...trellisFills(posts, shoes, rails), ...lathFills(), ...matFills(MASSES_100)],
    strokes: [
      ...stems.map((p, i) => ({ id: p.id, bone: p.bone, role: "detail", z: 5 + i / 1000, tone: 0.4, region: "stems", line: p.secs.map((s) => s.c), views: ["elevation", "front"] as View[] })),
    ],
    textures: leafTextures(leaves, [...FOL_BONES, "crown-l", "crown-r"], 2),
  };
}

const slimZ = (y: number): number => (y < 0.3 ? -0.07 : y < 0.6 ? -0.047 : y < 1.9 ? -0.012 : y <= 2.0 ? -0.045 : -0.02);
const climb = (k: number, ys: number[]): number[] => [...new Set(ys.map((y) => Math.min(y, LS[k]!)))];
const slim = (k: number, id: string, ys: number[]): Loft => tube(id, `stem-${k}`, STEM, climb(k, ys).map((y): [number, number, number, number] => [XS[k]!, y, slimZ(y), stemR(y)]));
const twig = (b: string, id: string, r: number): Loft => {
  const k = Number(b[4]);
  const y = b.endsWith("lo") ? FOL_LO_Y[k]! : 1.3;
  const s = k % 2 === 0 ? 1 : -1;
  return tube(id, b, SHOOT, [[XS[k]!, y, -0.012, r], [XS[k]! + s * 0.17, y + (b.endsWith("lo") ? 0.22 : 0.3), 0.012, r * 0.7]]);
};
const crownTwig = (b: "crown-l" | "crown-r", id: string): Loft => tube(id, b, SHOOT, CROWN_SHOOTS[`${b}-shoot-a`]!.slice(0, 3).map(([x, y, z]): [number, number, number, number] => [x, Math.min(y, 2.05), z, 0.003]));

function scale200(): Scale {
  const posts = [postBox("l"), postBox("r")];
  const shoes = (["l", "r"] as const).map((s) => [{ ...sleeve(s, 0), id: `shoe-${s}` } as Prim]);
  const rails = [rail("top", TIMBER, 0), rail("bottom", TIMBER, 0)];
  const cores = MASSES_200.map(lobe);
  const climbers = K.map((k) => slim(k, `stem-${k}`, [0, 0.3, 0.5, 0.62, 1.2, 1.88, 1.95, 2.05]));
  const flakes = sprays(MASSES_200, folAt, 0.15, 0.125, 0.21, 0.031, 0.86);
  const body = [...posts.map((p) => S(p)), ...shoes.flat().map((p) => S(p)), ...rails.map((p) => S(p)), ...climbers.map((p) => S(p)), ...cores.map((p) => S(p)), ...flakes.map((f) => f.solid)];
  const stems = climbers.map((p) => ({ ...p, id: `th-${p.id}` }));
  const thermal = [
    ...cores.map((p) => th(p.id, matRegion(p.bone))), ...flakes.map((f) => th(f.prim.id, matRegion(f.prim.bone))),
    ...stems.map((p) => S(p, { region: "stems" })), ...FOL_BONES.map((b) => S(twig(b, `th-lateral-${b.slice(4)}`, 0.0045), { region: "stems" })),
    ...["post-l", "post-r", "rail-top", "rail-bottom"].map((id) => th(id, "trellis-timber")),
    ...lathSolids().map((x) => ({ ...x, id: `th-${x.id}`, region: "trellis-timber" })),
    sleeveShell("l", 0), sleeveShell("r", 0),
    ...outlets(MASSES_200, 200, "all").filter((s) => !String(s.bone).startsWith("crown")),
  ];
  const structure = [
    st("post-l", "post-l", "member"), st("post-r", "post-r", "member"), ...supports(["shoe-l", "shoe-r"]),
    st("rail-top", "panel", "member"), st("rail-bottom", "panel", "member"),
    ...[[-1, RAIL.hi, "top-l"], [1, RAIL.hi, "top-r"], [-1, RAIL.lo, "bottom-l"], [1, RAIL.lo, "bottom-r"]].map(([x, y, n]) => S(box(`joint-rail-${n}`, "panel", JOINT, [(x as number) * (RAIL.x - 0.015), y as number, 0], [0.03, 0.05, 0.05]), { member: "panel", part: "joint" })),
    ...K.map((k) => st(`stem-${k}`, `stem-${k}`, "member")),
    ...FOL_BONES.map((b) => S(twig(b, `member-${b}`, 0.0045), { member: b, part: "member" })),
    S(crownTwig("crown-l", "member-crown-l"), { member: "crown-l", part: "member" }), S(crownTwig("crown-r", "member-crown-r"), { member: "crown-r", part: "member" }),
    ...masses(0.04),
  ];
  return { volume: { status: "authored", denominator: 200, body, thermal, structure, decisions: DECISIONS_3D[200] }, fills: [...trellisFills(posts, shoes, rails), ...matFills(MASSES_200, flakes.map((f) => f.prim))], strokes: [...climbers.map((p, i) => ({ id: p.id, bone: p.bone, role: "detail", z: 5 + i / 1000, tone: 0.4, region: "stems", line: p.secs.map((q) => q.c), views: ["elevation", "front"] as View[] }))], textures: [] };
}

/** 🧱️ 1:500: the leaf screen as one mass (height, half width, half depth) between the two posts. */
const SCREEN_500: [number, number, number][] = [[0.8, 0.8, 0.02], [0.86, 0.87, 0.04], [1.1, 0.885, 0.056], [1.6, 0.89, 0.058], [1.95, 0.88, 0.054], [2.03, 0.84, 0.03]];
const screen = (id: string, bone: string, look: Look, rows500: [number, number, number][], shrink = 0): Loft => ({ k: "loft", id, bone, look, secs: rows500.map(([y, a, b]) => ({ c: [0, y, 0] as V3, a: a - shrink, b: b - shrink, e: 5, ax: [1, 0, 0] as V3, n: [0, 1, 0] as V3 })) });
function scale500(): Scale {
  const posts = [postBox("l"), postBox("r")];
  const rails = [rail("top", TIMBER, 0), rail("bottom", TIMBER, 0)];
  const mass: Cushion = { id: "screen", bone: "panel", e: 5, cz: 0, rows: SCREEN_500.map(([y, a, b]) => [y, 0, a, b]) };
  const mat = lobe(mass);
  const climbers = K.map((k) => slim(k, `stem-${k}`, [0, 0.5, 0.62, 1.88, 1.95, 2.05]));
  const flakes = sprays([mass], () => "panel", 0.2, 0.175, 0.33, 0.076, 0.93).map((f) => ({ ...f, lower: f.prim.o[1] < 1.3, solid: { ...f.solid, tone: f.prim.o[1] < 1.3 ? CORE_LO.tone : CORE_HI.tone } }));
  const body = [...posts.map((p) => S(p)), ...rails.map((p) => S(p)), ...climbers.map((p) => S(p)), S(mat), ...flakes.map((f) => f.solid)];
  const cut = (lo: number, hi: number): [number, number, number][] => {
    const at = (y: number): [number, number, number] => {
      for (let i = 1; i < SCREEN_500.length; i++) if (y <= SCREEN_500[i]![0]) {
        const [a, b] = [SCREEN_500[i - 1]!, SCREEN_500[i]!];
        const f = (y - a[0]) / (b[0] - a[0]);
        return [y, a[1] + (b[1] - a[1]) * f, a[2] + (b[2] - a[2]) * f];
      }
      return SCREEN_500[SCREEN_500.length - 1]!;
    };
    return [at(lo), ...SCREEN_500.filter((r) => r[0] > lo && r[0] < hi), at(hi)];
  };
  const thermal = [
    S(screen("th-leaves-upper", "panel", CORE_HI, cut(1.3, 2.03)), { region: "leaves-upper" }), S(screen("th-leaves-lower", "panel", CORE_LO, cut(0.8, 1.3)), { region: "leaves-lower" }),
    ...flakes.map((f) => th(f.prim.id, f.lower ? "leaves-lower" : "leaves-upper")),
    ...climbers.map((p) => S({ ...p, id: `th-${p.id}` }, { region: "stems" })),
    th("post-l", "trellis-timber"), th("post-r", "trellis-timber"), th("rail-top", "trellis-timber"), th("rail-bottom", "trellis-timber"),
    sleeveShell("l", 0), sleeveShell("r", 0),
    ...outlets([["s", "panel", 0, 1.4, 0, 9, 9, 0.05, 0] as Lobe], 500, "first"),
  ];
  const structure = [
    st("post-l", "post-l", "member"), st("post-r", "post-r", "member"),
    S(box("support-base-support-l", "post-l", STEEL, [-POST_X, 0.0375, 0], [0.1, 0.075, 0.1]), { member: "post-l", part: "support", anchor: "base-support-l" }), S(box("support-base-support-r", "post-r", STEEL, [POST_X, 0.0375, 0], [0.1, 0.075, 0.1]), { member: "post-r", part: "support", anchor: "base-support-r" }),
    st("rail-top", "panel", "member"), st("rail-bottom", "panel", "member"),
    ...K.map((k) => st(`stem-${k}`, `stem-${k}`, "member")),
    ...FOL_BONES.map((b) => S(twig(b, `member-${b}`, 0.0045), { member: b, part: "member" })),
    S(crownTwig("crown-l", "member-crown-l"), { member: "crown-l", part: "member" }), S(crownTwig("crown-r", "member-crown-r"), { member: "crown-r", part: "member" }),
    ...masses(0.04),
  ];
  return {
    volume: { status: "authored", denominator: 500, body, thermal, structure, decisions: DECISIONS_3D[500] },
    fills: [...trellisFills(posts, [], rails).map((f) => (f.id.startsWith("post") ? { ...f, region: undefined, regions: ["trellis-timber", "post-shoes"] } : f)), { id: "screen", bone: "panel", role: "body", z: 6, tone: 0.24, regions: ["leaves-upper", "leaves-lower"], prims: [mat, ...flakes.map((f) => f.prim)] }],
    strokes: climbers.map((p, i) => ({ id: p.id, bone: p.bone, role: "body", z: 5 + i / 1000, tone: 0.5, penMm: 0.1, line: p.secs.map((q) => q.c), views: ["elevation", "front"] as View[] })),
    textures: [],
  };
}

/** 🩹️ Root fixes outside `volumes` (rig, anchors, poses, thermal text, summary); they do not move any bone frame. */
function roots(): void {
  for (const b of asset.rig.bones) {
    if (b.id === "root-plate") b.lengthM = 0;
    if (b.id === "crown-l" || b.id === "crown-r") {
      b.lengthM = 0.14;
      b.mass.comX = 0.07;
    }
    if (b.id === "panel") b.mass.comY = 0.825;
  }
  for (const a of asset.anchors) {
    if (a.id === "base-support-l" || a.id === "base-support-r") {
      a.footprintLengthM = 0.1;
      a.footprintWidthM = 0.18;
    }
    if (a.id === "tp-upper-crown-l" || a.id === "tp-upper-crown-r") {
      const l = local(a.bone, crownTop(MASSES_50, a.bone));
      a.x = r4(l[0]);
      a.y = r4(l[1]);
    }
  }
  for (const p of asset.poses) for (const l of p.loads ?? []) if (l.bone === "crown-l" || l.bone === "crown-r") l.x = 0.07;
  const stems = asset.thermal.regions.find((r: Json) => r.id === "stems");
  stems.hidden = T("The stems run behind the leaf mats; they show only in the open strip below the mats, as filled outlines at 1:50 and as strokes at 1:100.", "Die Triebe laufen hinter den Blattteppichen; sie zeigen sich nur im offenen Streifen unter den Teppichen, bei 1:50 als gefüllte Umrisse und bei 1:100 als Striche.");
  asset.summary = T(
    "English ivy (Hedera helix) on a freestanding timber trellis, 1.9 m wide and 2.1 m high: two 90 mm posts in bolted galvanised steel shoes, two rails and a lattice of 20 mm laths. Four main stems are woven through the lattice and carry leaf mats 0.25 m thick over the upper three quarters; shoots overhang the top rail and trail from the ragged lower edge. Trellis and plant are one asset.",
    "Gemeiner Efeu (Hedera helix) an einem freistehenden Holzspalier von 1,9 m Breite und 2,1 m Höhe: zwei Pfosten von 90 mm in verschraubten, verzinkten Stahlschuhen, zwei Riegel und ein Gitter aus Latten von 20 mm. Vier Haupttriebe sind durch das Gitter geflochten und tragen 0,25 m dicke Blattteppiche über den oberen drei Vierteln; Triebe hängen über den oberen Riegel und vom ausgefransten unteren Rand herab. Spalier und Pflanze sind ein Asset.",
  );
  for (const scale of [50, 100, 200, 500]) asset.representations[String(scale)].decisions = DECISIONS_2D[scale];
}

/** 🔪️ A ring clipped to the window lo <= p <= hi (Sutherland-Hodgman): each leaf mat is drawn in its own field (its column between the stems, its band above or below 1.3 m, 12 mm past the seams), so the drawing shows one mat with the seams of the bones and no outlines of hidden overlaps. */
function clipWindow(ring: P2[], lo: P2, hi: P2): P2[] {
  const cut = (pts: P2[], inside: (p: P2) => boolean, at: (a: P2, b: P2) => P2): P2[] => pts.flatMap((p, i) => {
    const q = pts[(i + 1) % pts.length]!;
    if (inside(p)) return inside(q) ? [p] : [p, at(p, q)];
    return inside(q) ? [at(p, q)] : [];
  });
  const atX = (x: number) => (a: P2, b: P2): P2 => ({ x, y: a.y + ((b.y - a.y) * (x - a.x)) / (b.x - a.x) });
  const atY = (y: number) => (a: P2, b: P2): P2 => ({ y, x: a.x + ((b.x - a.x) * (y - a.y)) / (b.y - a.y) });
  return cut(cut(cut(cut(ring, (p) => p.x >= lo.x, atX(lo.x)), (p) => p.x <= hi.x, atX(hi.x)), (p) => p.y >= lo.y, atY(lo.y)), (p) => p.y <= hi.y, atY(hi.y));
}
const SEAMS = [-9, -0.41, 0.01, 0.43, 9];
function field(view: View, id: string): [P2, P2] | null {
  const m = id.match(/^mat-(\d)-(lo|hi)$/);
  if (!m || view === "front") return null;
  const k = Number(m[1]);
  const [x0, x1] = [SEAMS[k - 1]! - 0.012, SEAMS[k]! + 0.012];
  if (view === "plan") return [{ x: x0, y: -9 }, { x: x1, y: 9 }];
  return m[2] === "lo" ? [{ x: x0, y: -9 }, { x: x1, y: 1.312 }] : [{ x: x0, y: 1.288 }, { x: x1, y: 9 }];
}

/** ✏️ Ruling E1: every view of every scale drawn anew as the true orthographic view of that scale's body (filled outlines per part, centre-line strokes for slender parts, leaf outlines as texture); no per-shape pen overrides. */
function draw(scales: Record<number, Scale>): string[] {
  const report: string[] = [];
  for (const scale of [50, 100, 200, 500]) {
    const sc = scales[scale]!;
    const feature = (0.15 * scale) / 1000;
    const tol = feature / (scale === 500 ? 12 : 5);
    for (const view of VIEWS) {
      const shapes: Json[] = [];
      for (const f of sc.fills) {
        if (f.views && !f.views.includes(view)) continue;
        const window = field(view, f.id);
        if (window && view === "plan" && f.id.endsWith("lo")) continue;
        const whole = outline(view, f.prims, tol, (feature * feature) / 4);
        const rings = window ? whole.map((r) => clipWindow(r, window[0], window[1])).filter((r) => r.length >= 3) : whole;
        if (!rings.length) continue;
        const loc = toDrawing(view, F[f.bone]!, rings);
        const { prims: _p, views: _v, ...style } = f;
        if (style.region === undefined) delete style.region;
        shapes.push({ ...style, filled: true, ...(loc.length === 1 ? { kind: "poly", points: pointsOf(loc[0]!), closed: true } : { kind: "path", d: pathOf(loc, true) }) });
      }
      for (const s of sc.strokes) {
        if (s.views && !s.views.includes(view)) continue;
        const loc = toDrawing(view, F[s.bone]!, [s.line.map((p) => project(view, p))]);
        const { line: _l, views: _v, ...style } = s;
        shapes.push({ ...style, filled: false, kind: "poly", points: pointsOf(loc[0]!), closed: false });
      }
      for (const t of sc.textures) {
        if (!t.views.includes(view)) continue;
        const loc = toDrawing(view, F[t.bone]!, t.rings.map((r) => r.map((p) => project(view, p))));
        const { rings: _r, views: _v, region, ...style } = t;
        shapes.push({ ...style, role: "texture", filled: false, ...(region ? { region } : {}), kind: "path", d: pathOf(loc, true) });
      }
      shapes.sort((a, b) => a.z - b.z);
      asset.representations[String(scale)].geometry[view] = shapes;
      report.push(`1:${scale} ${view}: ${shapes.length} shapes`);
    }
  }
  return report;
}

roots();
const SCALES: Record<number, Scale> = { 50: scale50(), 100: scale100(), 200: scale200(), 500: scale500() };
export const volumes = { "50": SCALES[50]!.volume, "100": SCALES[100]!.volume, "200": SCALES[200]!.volume, "500": SCALES[500]!.volume };
const report = draw(SCALES);
if (process.argv.includes("--probe")) for (const l of LATERALS.map((x) => lateral(x))) for (const q of l.pts) if (depth(MASSES_50, -1, q[0], q[1]) <= 0) console.log("[DEBUG] lateral outside mats", l.id, q.map(r4).join(" "));
writeFileSync(sourcePath(entry), JSON.stringify({ volumes }, null, 1) + "\n");
const out = join(GENERATED, "r3-ivy-on-trellis");
mkdirSync(out, { recursive: true });
const doc = { ...JSON.parse(readFileSync(entry.path, "utf8")), asset: { ...asset, volumes } };
writeFileSync(join(out, "🔣️.json"), JSON.stringify(doc, null, 2) + "\n");
const count = (s: Json): string => `${s.body.length}/${s.thermal.length}/${s.structure.length}`;
console.log(`body/thermal/structure: 1:50 ${count(volumes["50"])}, 1:100 ${count(volumes["100"])}, 1:200 ${count(volumes["200"])}, 1:500 ${count(volumes["500"])}`);
for (const line of report) console.log(line);
