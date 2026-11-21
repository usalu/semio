/** 🌳️ The round-5 multi-stem birch (*Betula pendula*, three-stemmed clump of about 13 m) stated by hand in world metres (x ahead, y up, z lateral; azimuth from +x towards +z): the stool with its surface roots, three slender stems that splay as a tripod, their leaders, six boned and forty unboned ascending branches, the leafy sleeves that clothe them, the pendulous tresses that hang from them, bare whip ends and the bark marks, per scale.
 * @see ./round3-brief.md
 * @see ./r3-vegetation-multi-stem-birch.md */
import type { Frame, Json, V3 } from "./volumes-lib.ts";
import { D2R, add, along, cross, dirOf, mul, smooth, sub, tubeSections, unit, type Mark, type Part, type Sec } from "./r3-multi-stem-birch-kit.ts";

export const WHITE = "birch-bark", DARK = "birch-bark-base", LEAF = "birch-leaf", WOOD = "birch-wood";
export const TONE = { stool: 0.3, base: 0.32, stem: 0.92, branch: 0.8, strand: 0.25, mark: 0.12 };
const LEAF_TONE: Record<string, number> = { "fol-c-top": 0.7, "fol-l-b": 0.62, "fol-r-b": 0.62, "fol-l-a": 0.62, "fol-r-a": 0.62 };
const toneOf = (bone: string): number => LEAF_TONE[bone] ?? 0.66;

/** 🦴️ Where the three stems leave the stool: across offset (rig `y` of the lower stem bone, + is towards -x) and lateral position (world z). */
export const BASE: Record<string, [y: number, z: number]> = { "stem-l-1": [0.11, -0.07], "stem-c-1": [0, 0.13], "stem-r-1": [-0.11, -0.06] };
/** ↗️ Lateral lean of every wood bone: world z gained from its joint to its tip (the tripod: left and right stems lean to -z, the centre stem to +z; branches drift). */
export const LEAN: Record<string, number> = {
  "stem-l-1": -0.42, "stem-l-2": -0.33, "fol-l-top": -0.08, "stem-c-1": 0.72, "stem-c-2": 0.55, "fol-c-top": 0.12, "stem-r-1": -0.38, "stem-r-2": -0.3, "fol-r-top": -0.07,
  "br-l-a": -0.35, "br-l-b": 0.45, "br-c-a": 0.45, "br-c-b": 0.35, "br-r-a": 0.4, "br-r-b": -0.5,
  "fol-l-a": -0.12, "fol-l-b": 0.14, "fol-c-a": 0.12, "fol-c-b": 0.1, "fol-r-a": 0.12, "fol-r-b": -0.14,
};

/** 🧭️ Writes the stool offsets and the lateral joint offsets (rig `y`, `z`, relative to the parent) into the rig so that every joint lies on the splayed wood. */
export function applyRig(rig: Json): void {
  const byId: Record<string, Json> = Object.fromEntries(rig.bones.map((b: Json) => [b.id, b]));
  const abs: Record<string, number> = {};
  const zOf = (b: Json): number => {
    if (abs[b.id] !== undefined) return abs[b.id]!;
    const parent = b.parent ? byId[b.parent] : null;
    const z = !parent ? 0 : BASE[b.id] ? BASE[b.id]![1] : zOf(parent) + (LEAN[parent.id] ?? 0) * (b.x / parent.lengthM);
    return (abs[b.id] = z);
  };
  for (const b of rig.bones) {
    const parent = b.parent ? byId[b.parent] : null;
    const rel = Math.round((zOf(b) - (parent ? zOf(parent) : 0)) * 1e4) / 1e4;
    const { id, parent: p, title, x, y, z: _z, ...rest } = b;
    for (const k of Object.keys(b)) delete b[k];
    Object.assign(b, { id, parent: p, title, x, y: BASE[id] ? BASE[id]![0] : y, ...(rel !== 0 ? { z: rel } : {}), ...rest });
  }
}

type St = [u: number, wx: number, wz: number, r: number, lod: number];
type Chain = { id: string; kind: "stem" | "branch"; bones: [bone: string, rows: St[]][] };

/** 🪵️ Stem stations per bone: u along the bone, wx and wz the sinuous offset from the bone line (world x, z), r the radius over bark, lod the coarsest level that keeps the station (0 = 1:500, 1 = 1:200, 2 = 1:100, 3 = 1:50). */
const STEMS: Chain[] = [
  { id: "l", kind: "stem", bones: [
    ["stem-l-1", [[-0.26, 0, 0, 0.118, 0], [0, 0, 0, 0.104, 1], [0.35, 0.004, 0, 0.094, 2], [0.75, 0.01, 0.004, 0.088, 1], [1.3, 0.02, 0.01, 0.082, 3], [2.1, 0.028, 0.016, 0.076, 1], [3.0, 0.018, 0.012, 0.069, 3], [3.9, 0.004, -0.004, 0.063, 2], [5.0, 0, 0, 0.057, 0]]],
    ["stem-l-2", [[0.9, -0.018, 0.008, 0.053, 3], [1.8, -0.026, 0.004, 0.049, 1], [2.7, -0.014, -0.008, 0.044, 3], [3.6, -0.004, -0.004, 0.039, 2], [4.5, 0, 0, 0.033, 0]]],
    ["fol-l-top", [[0.5, 0.012, 0.006, 0.026, 3], [1.0, 0.02, 0.004, 0.018, 1], [1.45, 0.01, 0, 0.011, 3], [1.72, 0, 0, 0.006, 0]]],
  ] },
  { id: "c", kind: "stem", bones: [
    ["stem-c-1", [[-0.26, 0, 0, 0.135, 0], [0, 0, 0, 0.12, 1], [0.35, 0, 0, 0.108, 2], [0.75, -0.006, 0.006, 0.1, 1], [1.3, -0.014, 0.012, 0.094, 3], [2.2, -0.024, 0.02, 0.087, 1], [3.2, -0.016, 0.014, 0.08, 3], [4.3, 0.006, 0, 0.072, 2], [5.5, 0, 0, 0.065, 0]]],
    ["stem-c-2", [[1.0, 0.016, -0.01, 0.059, 3], [2.0, 0.024, -0.012, 0.053, 1], [2.9, 0.012, 0, 0.047, 3], [3.9, 0.004, 0.006, 0.04, 2], [4.8, 0, 0, 0.034, 0]]],
    ["fol-c-top", [[0.55, -0.012, 0.008, 0.027, 3], [1.1, -0.02, 0.004, 0.019, 1], [1.6, -0.008, 0, 0.011, 3], [1.9, 0, 0, 0.006, 0]]],
  ] },
  { id: "r", kind: "stem", bones: [
    ["stem-r-1", [[-0.26, 0, 0, 0.126, 0], [0, 0, 0, 0.112, 1], [0.35, -0.004, 0, 0.101, 2], [0.75, -0.01, 0.004, 0.094, 1], [1.3, -0.02, 0.008, 0.088, 3], [2.1, -0.03, 0.014, 0.082, 1], [3.0, -0.02, 0.012, 0.075, 3], [3.9, -0.006, 0, 0.068, 2], [5.0, 0, 0, 0.062, 0]]],
    ["stem-r-2", [[0.9, 0.02, 0.006, 0.057, 3], [1.8, 0.028, 0.008, 0.052, 1], [2.7, 0.016, -0.004, 0.046, 3], [3.6, 0.004, -0.004, 0.04, 2], [4.5, 0, 0, 0.034, 0]]],
    ["fol-r-top", [[0.5, -0.012, 0.004, 0.027, 3], [1.0, -0.018, 0.006, 0.018, 1], [1.45, -0.008, 0, 0.011, 3], [1.72, 0, 0, 0.006, 0]]],
  ] },
];

/** 🧵️ Hanging tress row: f the fraction along its branch where it leaves, daz the side it drifts to from the branch heading before it hangs (90 = a curtain that lies along the branch), L its length, w its half width. */
type T = [f: number, daz: number, L: number, w: number];
type Leafage = { br: T[]; tip: T[] };

/** 🦴️ The six branches that carry bones: stations on the branch bone and on the foliage bone that continues it (wx is the bow upwards here); `sl` is the leafy sleeve round the outer branch (from the fraction f0, half width w), then the tresses (`br` hang from the branch bone, `tip` from the foliage bone); 1:50, 1:100 and 1:200 share them, on fewer sections each. */
type Boned = Chain & { sl: [number, number]; p: Leafage };
const BONED: Boned[] = [
  { id: "l-a", kind: "branch", bones: [
    ["br-l-a", [[-0.03, 0, 0, 0.036, 0], [0.14, 0, 0, 0.031, 2], [0.75, 0.035, -0.02, 0.028, 1], [1.5, 0.04, 0.015, 0.024, 3], [2.4, 0, 0, 0.019, 0]]],
    ["fol-l-a", [[0.5, -0.015, 0.01, 0.014, 1], [0.95, -0.01, 0, 0.009, 3], [1.3, 0, 0, 0.005, 0]]],
  ], sl: [0.38, 0.36], p: { br: [[0.45, -85, 1.1, 0.15], [0.62, 80, 1.35, 0.16], [0.8, -75, 1.55, 0.17], [0.95, 85, 1.7, 0.17]], tip: [[0.2, -80, 1.5, 0.16], [0.42, 78, 1.35, 0.16], [0.62, -82, 1.2, 0.15], [0.8, 75, 1.05, 0.14], [1, 0, 0.95, 0.14]] } },
  { id: "l-b", kind: "branch", bones: [
    ["br-l-b", [[-0.03, 0, 0, 0.035, 0], [0.14, 0, 0, 0.03, 2], [0.7, 0.03, 0.02, 0.027, 1], [1.4, 0.035, -0.015, 0.023, 3], [2.2, 0, 0, 0.018, 0]]],
    ["fol-l-b", [[0.5, -0.012, -0.01, 0.014, 1], [0.95, -0.008, 0, 0.009, 3], [1.3, 0, 0, 0.005, 0]]],
  ], sl: [0.36, 0.37], p: { br: [[0.44, 82, 1.2, 0.15], [0.6, -80, 1.45, 0.16], [0.78, 78, 1.7, 0.17], [0.94, -84, 1.85, 0.18]], tip: [[0.18, 84, 1.65, 0.17], [0.4, -76, 1.5, 0.16], [0.6, 80, 1.3, 0.15], [0.8, -78, 1.15, 0.15], [1, 0, 1.0, 0.14]] } },
  { id: "c-a", kind: "branch", bones: [
    ["br-c-a", [[-0.03, 0, 0, 0.034, 0], [0.14, 0, 0, 0.029, 2], [0.7, 0.03, -0.015, 0.026, 1], [1.4, 0.03, 0.02, 0.022, 3], [2.2, 0, 0, 0.018, 0]]],
    ["fol-c-a", [[0.5, -0.012, 0.01, 0.014, 1], [0.95, -0.008, 0, 0.009, 3], [1.3, 0, 0, 0.005, 0]]],
  ], sl: [0.4, 0.35], p: { br: [[0.46, 80, 1.0, 0.15], [0.63, -84, 1.25, 0.16], [0.8, 76, 1.45, 0.16], [0.95, -80, 1.6, 0.17]], tip: [[0.2, 80, 1.4, 0.16], [0.42, -78, 1.3, 0.16], [0.62, 84, 1.15, 0.15], [0.8, -76, 1.0, 0.14], [1, 0, 0.9, 0.14]] } },
  { id: "c-b", kind: "branch", bones: [
    ["br-c-b", [[-0.03, 0, 0, 0.032, 0], [0.14, 0, 0, 0.027, 2], [0.7, 0.03, 0.015, 0.024, 1], [1.4, 0.03, -0.015, 0.02, 3], [2.2, 0, 0, 0.016, 0]]],
    ["fol-c-b", [[0.5, -0.012, -0.008, 0.013, 1], [0.95, -0.008, 0, 0.008, 3], [1.3, 0, 0, 0.005, 0]]],
  ], sl: [0.4, 0.34], p: { br: [[0.46, -82, 0.95, 0.15], [0.63, 80, 1.15, 0.15], [0.8, -78, 1.35, 0.16], [0.95, 82, 1.5, 0.16]], tip: [[0.2, -78, 1.3, 0.16], [0.42, 80, 1.2, 0.15], [0.62, -80, 1.05, 0.15], [0.8, 78, 0.95, 0.14], [1, 0, 0.85, 0.13]] } },
  { id: "r-a", kind: "branch", bones: [
    ["br-r-a", [[-0.03, 0, 0, 0.036, 0], [0.14, 0, 0, 0.031, 2], [0.75, 0.035, 0.02, 0.028, 1], [1.5, 0.04, -0.015, 0.024, 3], [2.4, 0, 0, 0.019, 0]]],
    ["fol-r-a", [[0.5, -0.015, -0.01, 0.014, 1], [0.95, -0.01, 0, 0.009, 3], [1.3, 0, 0, 0.005, 0]]],
  ], sl: [0.38, 0.36], p: { br: [[0.44, 84, 1.1, 0.15], [0.6, -80, 1.3, 0.16], [0.78, 78, 1.55, 0.17], [0.95, -84, 1.7, 0.17]], tip: [[0.2, 82, 1.5, 0.16], [0.4, -78, 1.4, 0.16], [0.6, 80, 1.2, 0.15], [0.8, -76, 1.05, 0.14], [1, 0, 0.95, 0.14]] } },
  { id: "r-b", kind: "branch", bones: [
    ["br-r-b", [[-0.03, 0, 0, 0.035, 0], [0.14, 0, 0, 0.03, 2], [0.7, 0.03, -0.02, 0.027, 1], [1.4, 0.035, 0.015, 0.023, 3], [2.2, 0, 0, 0.018, 0]]],
    ["fol-r-b", [[0.5, -0.012, 0.01, 0.014, 1], [0.95, -0.008, 0, 0.009, 3], [1.3, 0, 0, 0.005, 0]]],
  ], sl: [0.36, 0.37], p: { br: [[0.45, -82, 1.2, 0.15], [0.62, 80, 1.4, 0.16], [0.8, -78, 1.7, 0.17], [0.95, 84, 1.85, 0.18]], tip: [[0.18, -84, 1.65, 0.17], [0.4, 76, 1.5, 0.16], [0.6, -80, 1.3, 0.15], [0.8, 78, 1.15, 0.15], [1, 0, 1.0, 0.14]] } },
];

/** 🌱️ The unboned branches: from the stem at u of a stem bone, heading az, rising at e0 and arching to e1 over the length L, radius r0 at the stem to 5 mm at the tip, heading turning by bend; lod 1 marks a scaffold branch (bark of its own in the thermal model), lod 2 a twig-thin lateral; `sl` the leafy sleeve (from f0, half width w); then the tresses. */
type Free = { id: string; bone: string; u: number; az: number; e0: number; e1: number; L: number; r0: number; bend: number; lod: number; sl: [number, number]; p: T[] };
const FREE: Free[] = [
  { id: "l1", bone: "stem-l-1", u: 2.9, az: 250, e0: 48, e1: 0, L: 2.3, r0: 0.017, bend: -15, lod: 2, sl: [0.38, 0.36], p: [[0.45, -82, 1.0, 0.15], [0.6, 80, 1.25, 0.16], [0.75, -78, 1.45, 0.17], [0.9, 82, 1.6, 0.17], [1, 0, 1.75, 0.18]] },
  { id: "l2", bone: "stem-l-1", u: 4.3, az: 95, e0: 52, e1: 8, L: 2.1, r0: 0.018, bend: 12, lod: 1, sl: [0.38, 0.36], p: [[0.44, 80, 1.1, 0.15], [0.6, -82, 1.35, 0.16], [0.76, 78, 1.6, 0.17], [0.9, -80, 1.75, 0.17], [1, 0, 1.9, 0.18]] },
  { id: "l3", bone: "stem-l-2", u: 0.4, az: 290, e0: 50, e1: 5, L: 2.85, r0: 0.019, bend: -10, lod: 1, sl: [0.36, 0.38], p: [[0.42, -80, 1.15, 0.15], [0.56, 82, 1.4, 0.16], [0.7, -78, 1.6, 0.17], [0.84, 80, 1.8, 0.18], [0.94, -84, 1.95, 0.18], [1, 0, 2.05, 0.18]] },
  { id: "l4", bone: "stem-l-2", u: 2.0, az: 60, e0: 55, e1: 15, L: 1.6, r0: 0.015, bend: 10, lod: 2, sl: [0.4, 0.33], p: [[0.5, 80, 0.9, 0.14], [0.68, -80, 1.1, 0.15], [0.85, 78, 1.25, 0.15], [1, 0, 1.4, 0.16]] },
  { id: "l5", bone: "stem-l-2", u: 2.6, az: 235, e0: 52, e1: 8, L: 2.1, r0: 0.016, bend: 12, lod: 2, sl: [0.38, 0.35], p: [[0.44, -80, 0.95, 0.15], [0.6, 82, 1.15, 0.15], [0.76, -78, 1.35, 0.16], [0.9, 80, 1.5, 0.16], [1, 0, 1.6, 0.17]] },
  { id: "l6", bone: "stem-l-2", u: 3.2, az: 130, e0: 55, e1: 12, L: 1.7, r0: 0.014, bend: -8, lod: 2, sl: [0.4, 0.32], p: [[0.5, -80, 0.8, 0.14], [0.68, 80, 0.95, 0.14], [0.85, -78, 1.1, 0.15], [1, 0, 1.2, 0.15]] },
  { id: "c1", bone: "stem-c-1", u: 3.4, az: 80, e0: 48, e1: 4, L: 2.75, r0: 0.019, bend: 10, lod: 1, sl: [0.36, 0.38], p: [[0.42, 80, 1.05, 0.15], [0.56, -82, 1.3, 0.16], [0.7, 78, 1.5, 0.17], [0.84, -80, 1.7, 0.18], [0.94, 84, 1.8, 0.18], [1, 0, 1.9, 0.18]] },
  { id: "c2", bone: "stem-c-1", u: 4.4, az: 300, e0: 50, e1: 8, L: 1.9, r0: 0.017, bend: -12, lod: 2, sl: [0.38, 0.35], p: [[0.46, -80, 1.0, 0.15], [0.62, 80, 1.2, 0.16], [0.78, -78, 1.4, 0.16], [0.92, 82, 1.55, 0.17], [1, 0, 1.65, 0.17]] },
  { id: "c3", bone: "stem-c-1", u: 5.2, az: 150, e0: 50, e1: 8, L: 2.5, r0: 0.018, bend: 10, lod: 1, sl: [0.36, 0.38], p: [[0.42, -82, 1.1, 0.15], [0.56, 80, 1.35, 0.16], [0.7, -78, 1.55, 0.17], [0.84, 82, 1.75, 0.18], [0.94, -80, 1.85, 0.18], [1, 0, 1.95, 0.18]] },
  { id: "c4", bone: "stem-c-2", u: 0.5, az: 35, e0: 50, e1: 8, L: 2.5, r0: 0.018, bend: -10, lod: 1, sl: [0.36, 0.38], p: [[0.42, 82, 1.05, 0.15], [0.56, -80, 1.3, 0.16], [0.7, 78, 1.5, 0.17], [0.84, -82, 1.7, 0.18], [0.94, 80, 1.8, 0.18], [1, 0, 1.9, 0.18]] },
  { id: "c5", bone: "stem-c-2", u: 1.9, az: 95, e0: 52, e1: 10, L: 2.0, r0: 0.016, bend: 8, lod: 2, sl: [0.38, 0.35], p: [[0.45, -80, 0.95, 0.15], [0.6, 82, 1.15, 0.15], [0.76, -78, 1.3, 0.16], [0.9, 80, 1.45, 0.16], [1, 0, 1.55, 0.17]] },
  { id: "c6", bone: "stem-c-2", u: 3.3, az: 250, e0: 55, e1: 12, L: 1.7, r0: 0.014, bend: 10, lod: 2, sl: [0.4, 0.32], p: [[0.5, 80, 0.8, 0.14], [0.68, -80, 0.95, 0.14], [0.85, 78, 1.05, 0.15], [1, 0, 1.15, 0.15]] },
  { id: "c7", bone: "stem-c-2", u: 3.9, az: 120, e0: 56, e1: 15, L: 1.5, r0: 0.012, bend: -8, lod: 2, sl: [0.42, 0.3], p: [[0.52, -80, 0.7, 0.13], [0.72, 80, 0.85, 0.14], [0.9, -78, 0.95, 0.14], [1, 0, 1.0, 0.14]] },
  { id: "c8", bone: "stem-c-2", u: 4.4, az: 340, e0: 58, e1: 18, L: 1.2, r0: 0.01, bend: 8, lod: 2, sl: [0.42, 0.28], p: [[0.55, 80, 0.6, 0.13], [0.8, -80, 0.75, 0.13], [1, 0, 0.85, 0.13]] },
  { id: "r1", bone: "stem-r-1", u: 2.8, az: 290, e0: 48, e1: 0, L: 2.3, r0: 0.017, bend: 15, lod: 2, sl: [0.38, 0.36], p: [[0.45, 80, 1.0, 0.15], [0.6, -82, 1.25, 0.16], [0.75, 78, 1.45, 0.17], [0.9, -80, 1.6, 0.17], [1, 0, 1.75, 0.18]] },
  { id: "r2", bone: "stem-r-1", u: 4.2, az: 85, e0: 52, e1: 8, L: 2.1, r0: 0.018, bend: -12, lod: 1, sl: [0.38, 0.36], p: [[0.44, -82, 1.1, 0.15], [0.6, 80, 1.35, 0.16], [0.76, -78, 1.6, 0.17], [0.9, 82, 1.75, 0.17], [1, 0, 1.9, 0.18]] },
  { id: "r3", bone: "stem-r-2", u: 0.5, az: 250, e0: 50, e1: 5, L: 2.75, r0: 0.019, bend: 10, lod: 1, sl: [0.36, 0.38], p: [[0.42, 80, 1.15, 0.15], [0.56, -82, 1.4, 0.16], [0.7, 78, 1.6, 0.17], [0.84, -80, 1.8, 0.18], [0.94, 84, 1.9, 0.18], [1, 0, 2.0, 0.18]] },
  { id: "r4", bone: "stem-r-2", u: 2.1, az: 120, e0: 55, e1: 15, L: 1.6, r0: 0.015, bend: -10, lod: 2, sl: [0.4, 0.33], p: [[0.5, -80, 0.9, 0.14], [0.68, 80, 1.1, 0.15], [0.85, -78, 1.25, 0.15], [1, 0, 1.4, 0.16]] },
  { id: "r5", bone: "stem-r-2", u: 2.7, az: 305, e0: 52, e1: 8, L: 2.1, r0: 0.016, bend: -12, lod: 2, sl: [0.38, 0.35], p: [[0.44, 80, 0.95, 0.15], [0.6, -82, 1.15, 0.15], [0.76, 78, 1.35, 0.16], [0.9, -80, 1.5, 0.16], [1, 0, 1.6, 0.17]] },
  { id: "r6", bone: "stem-r-2", u: 3.3, az: 50, e0: 55, e1: 12, L: 1.7, r0: 0.014, bend: 8, lod: 2, sl: [0.4, 0.32], p: [[0.5, 80, 0.8, 0.14], [0.68, -80, 0.95, 0.14], [0.85, 78, 1.1, 0.15], [1, 0, 1.2, 0.15]] },
  { id: "l7", bone: "stem-l-2", u: 1.2, az: 330, e0: 52, e1: 10, L: 1.9, r0: 0.015, bend: -10, lod: 2, sl: [0.38, 0.34], p: [[0.48, 80, 0.95, 0.15], [0.66, -80, 1.15, 0.15], [0.84, 78, 1.3, 0.16], [1, 0, 1.45, 0.16]] },
  { id: "r7", bone: "stem-r-2", u: 1.3, az: 215, e0: 52, e1: 10, L: 1.9, r0: 0.015, bend: 10, lod: 2, sl: [0.38, 0.34], p: [[0.48, -80, 0.95, 0.15], [0.66, 80, 1.15, 0.15], [0.84, -78, 1.3, 0.16], [1, 0, 1.45, 0.16]] },
  { id: "c9", bone: "stem-c-2", u: 2.6, az: 278, e0: 52, e1: 8, L: 2.2, r0: 0.016, bend: 8, lod: 2, sl: [0.38, 0.35], p: [[0.46, 80, 0.95, 0.15], [0.62, -80, 1.15, 0.15], [0.78, 78, 1.3, 0.16], [0.92, -80, 1.4, 0.16], [1, 0, 1.5, 0.17]] },
  { id: "tl1", bone: "stem-l-2", u: 3.75, az: 300, e0: 60, e1: 20, L: 1.25, r0: 0.011, bend: 8, lod: 2, sl: [0.3, 0.32], p: [[0.55, 80, 0.64, 0.13], [0.8, -80, 0.74, 0.13], [1, 0, 0.8, 0.14]] },
  { id: "tl2", bone: "stem-l-2", u: 4.2, az: 170, e0: 60, e1: 22, L: 1.15, r0: 0.01, bend: -8, lod: 2, sl: [0.3, 0.31], p: [[0.55, -80, 0.6, 0.13], [0.8, 80, 0.69, 0.13], [1, 0, 0.75, 0.14]] },
  { id: "tl3", bone: "fol-l-top", u: 0.25, az: 50, e0: 62, e1: 25, L: 1.05, r0: 0.009, bend: 8, lod: 2, sl: [0.3, 0.3], p: [[0.55, 80, 0.56, 0.13], [0.8, -80, 0.64, 0.13], [1, 0, 0.7, 0.14]] },
  { id: "tl4", bone: "fol-l-top", u: 0.6, az: 250, e0: 64, e1: 28, L: 0.95, r0: 0.008, bend: -8, lod: 2, sl: [0.3, 0.28], p: [[0.55, -80, 0.5, 0.13], [0.8, 80, 0.57, 0.13], [1, 0, 0.62, 0.14]] },
  { id: "tl5", bone: "fol-l-top", u: 0.95, az: 130, e0: 66, e1: 30, L: 0.8, r0: 0.007, bend: 8, lod: 2, sl: [0.3, 0.26], p: [[0.55, 80, 0.44, 0.13], [0.8, -80, 0.51, 0.13], [1, 0, 0.55, 0.14]] },
  { id: "tl6", bone: "fol-l-top", u: 1.25, az: 340, e0: 68, e1: 34, L: 0.65, r0: 0.006, bend: -6, lod: 2, sl: [0.3, 0.23], p: [[0.55, -80, 0.37, 0.13], [0.8, 80, 0.42, 0.13], [1, 0, 0.46, 0.14]] },
  { id: "tc1", bone: "stem-c-2", u: 4.65, az: 200, e0: 60, e1: 22, L: 1.2, r0: 0.01, bend: 8, lod: 2, sl: [0.3, 0.31], p: [[0.55, 80, 0.62, 0.13], [0.8, -80, 0.72, 0.13], [1, 0, 0.78, 0.14]] },
  { id: "tc2", bone: "fol-c-top", u: 0.3, az: 70, e0: 62, e1: 25, L: 1.1, r0: 0.009, bend: -8, lod: 2, sl: [0.3, 0.3], p: [[0.55, -80, 0.58, 0.13], [0.8, 80, 0.66, 0.13], [1, 0, 0.72, 0.14]] },
  { id: "tc3", bone: "fol-c-top", u: 0.65, az: 300, e0: 64, e1: 28, L: 1.0, r0: 0.008, bend: 8, lod: 2, sl: [0.3, 0.29], p: [[0.55, 80, 0.52, 0.13], [0.8, -80, 0.6, 0.13], [1, 0, 0.65, 0.14]] },
  { id: "tc4", bone: "fol-c-top", u: 1.0, az: 170, e0: 66, e1: 30, L: 0.85, r0: 0.007, bend: -8, lod: 2, sl: [0.3, 0.27], p: [[0.55, -80, 0.46, 0.13], [0.8, 80, 0.52, 0.13], [1, 0, 0.57, 0.14]] },
  { id: "tc5", bone: "fol-c-top", u: 1.35, az: 30, e0: 68, e1: 34, L: 0.7, r0: 0.006, bend: 6, lod: 2, sl: [0.3, 0.24], p: [[0.55, 80, 0.38, 0.13], [0.8, -80, 0.44, 0.13], [1, 0, 0.48, 0.14]] },
  { id: "tr1", bone: "stem-r-2", u: 3.75, az: 240, e0: 60, e1: 20, L: 1.25, r0: 0.011, bend: -8, lod: 2, sl: [0.3, 0.32], p: [[0.55, -80, 0.64, 0.13], [0.8, 80, 0.74, 0.13], [1, 0, 0.8, 0.14]] },
  { id: "tr2", bone: "stem-r-2", u: 4.2, az: 10, e0: 60, e1: 22, L: 1.15, r0: 0.01, bend: 8, lod: 2, sl: [0.3, 0.31], p: [[0.55, 80, 0.6, 0.13], [0.8, -80, 0.69, 0.13], [1, 0, 0.75, 0.14]] },
  { id: "tr3", bone: "fol-r-top", u: 0.25, az: 130, e0: 62, e1: 25, L: 1.05, r0: 0.009, bend: -8, lod: 2, sl: [0.3, 0.3], p: [[0.55, -80, 0.56, 0.13], [0.8, 80, 0.64, 0.13], [1, 0, 0.7, 0.14]] },
  { id: "tr4", bone: "fol-r-top", u: 0.6, az: 290, e0: 64, e1: 28, L: 0.95, r0: 0.008, bend: 8, lod: 2, sl: [0.3, 0.28], p: [[0.55, 80, 0.5, 0.13], [0.8, -80, 0.57, 0.13], [1, 0, 0.62, 0.14]] },
  { id: "tr5", bone: "fol-r-top", u: 0.95, az: 50, e0: 66, e1: 30, L: 0.8, r0: 0.007, bend: -8, lod: 2, sl: [0.3, 0.26], p: [[0.55, -80, 0.44, 0.13], [0.8, 80, 0.51, 0.13], [1, 0, 0.55, 0.14]] },
  { id: "tr6", bone: "fol-r-top", u: 1.25, az: 200, e0: 68, e1: 34, L: 0.65, r0: 0.006, bend: 6, lod: 2, sl: [0.3, 0.23], p: [[0.55, 80, 0.37, 0.13], [0.8, -80, 0.42, 0.13], [1, 0, 0.46, 0.14]] },
];

/** 🌿️ Side shoots of a branch (second order): f where it leaves, daz its turn from the branch heading, L its length, w the half width of its leafy sleeve, then the lengths of the two tresses that hang from its middle and its end. */
type Side = [f: number, daz: number, L: number, w: number, t1: number, t2: number];
const SIDES: Record<string, Side[]> = {
  "l-a": [[0.5, -55, 1.1, 0.3, 1.0, 1.25], [0.78, 52, 1.0, 0.29, 1.2, 1.4]],
  "l-b": [[0.5, 55, 1.15, 0.31, 1.15, 1.4], [0.78, -52, 1.05, 0.3, 1.35, 1.55]],
  "c-a": [[0.52, 52, 1.05, 0.29, 0.95, 1.15], [0.8, -55, 0.95, 0.28, 1.1, 1.3]],
  "c-b": [[0.52, -52, 1.0, 0.28, 0.9, 1.05], [0.8, 55, 0.9, 0.27, 1.0, 1.2]],
  "r-a": [[0.5, 55, 1.1, 0.3, 1.0, 1.25], [0.78, -52, 1.0, 0.29, 1.2, 1.4]],
  "r-b": [[0.5, -55, 1.15, 0.31, 1.15, 1.4], [0.78, 52, 1.05, 0.3, 1.35, 1.55]],
  "l-a-t": [[0.3, 42, 0.62, 0.27, 1.05, 1.25], [0.68, -48, 0.56, 0.25, 0.95, 1.1]], "l-b-t": [[0.3, -42, 0.85, 0.28, 1.2, 1.4], [0.68, 48, 0.75, 0.26, 1.05, 1.2]], "c-a-t": [[0.3, -45, 0.8, 0.26, 1.0, 1.15], [0.68, 45, 0.7, 0.25, 0.9, 1.05]],
  "c-b-t": [[0.3, 45, 0.75, 0.26, 0.95, 1.1], [0.68, -45, 0.65, 0.24, 0.85, 1.0]], "r-a-t": [[0.3, -42, 0.62, 0.27, 1.05, 1.25], [0.68, 48, 0.6, 0.25, 0.95, 1.1]], "r-b-t": [[0.3, 42, 0.85, 0.28, 1.2, 1.4], [0.68, -48, 0.75, 0.26, 1.05, 1.2]],
  "l7": [[0.56, -52, 0.9, 0.27, 0.9, 1.05]], "r7": [[0.56, 52, 0.9, 0.27, 0.9, 1.05]], "c9": [[0.5, 52, 0.95, 0.28, 0.9, 1.1], [0.76, -52, 0.85, 0.26, 1.0, 1.2]],
  "l1": [[0.5, -55, 1.0, 0.29, 1.0, 1.2], [0.76, 50, 0.95, 0.28, 1.15, 1.35]],
  "l2": [[0.5, 52, 1.05, 0.3, 1.1, 1.3], [0.76, -55, 0.95, 0.28, 1.3, 1.45]],
  "l3": [[0.46, -52, 1.15, 0.31, 1.1, 1.35], [0.72, 55, 1.05, 0.3, 1.35, 1.6]],
  "l4": [[0.6, 50, 0.8, 0.26, 0.85, 1.0]],
  "l5": [[0.5, -55, 1.0, 0.29, 0.9, 1.1], [0.76, 52, 0.9, 0.27, 1.05, 1.25]],
  "l6": [[0.6, -50, 0.8, 0.25, 0.75, 0.9]],
  "c1": [[0.46, 52, 1.15, 0.31, 1.05, 1.3], [0.72, -55, 1.05, 0.3, 1.3, 1.5]],
  "c2": [[0.52, -52, 0.95, 0.28, 0.95, 1.15], [0.78, 50, 0.85, 0.27, 1.1, 1.3]],
  "c3": [[0.46, -55, 1.15, 0.31, 1.05, 1.3], [0.72, 52, 1.05, 0.3, 1.3, 1.5]],
  "c4": [[0.46, 55, 1.15, 0.31, 1.05, 1.3], [0.72, -52, 1.05, 0.3, 1.3, 1.5]],
  "c5": [[0.5, -52, 1.0, 0.28, 0.9, 1.1], [0.76, 55, 0.9, 0.27, 1.05, 1.2]],
  "c6": [[0.6, 50, 0.8, 0.25, 0.75, 0.9]],
  "c7": [[0.62, -50, 0.7, 0.24, 0.65, 0.8]],
  "r1": [[0.5, 55, 1.0, 0.29, 1.0, 1.2], [0.76, -50, 0.95, 0.28, 1.15, 1.35]],
  "r2": [[0.5, -52, 1.05, 0.3, 1.1, 1.3], [0.76, 55, 0.95, 0.28, 1.3, 1.45]],
  "r3": [[0.46, 52, 1.15, 0.31, 1.1, 1.35], [0.72, -55, 1.05, 0.3, 1.35, 1.6]],
  "r4": [[0.6, -50, 0.8, 0.26, 0.85, 1.0]],
  "r5": [[0.5, 55, 1.0, 0.29, 0.9, 1.1], [0.76, -52, 0.9, 0.27, 1.05, 1.25]],
  "r6": [[0.6, 50, 0.8, 0.25, 0.75, 0.9]],
};

/** 🍃️ Tresses that hang straight from the upper stems and the leaders (short shoots): bone, u along it, side it drifts to, L, w. */
type Shoot = [bone: string, u: number, az: number, L: number, w: number];
const SHOOTS: Record<number, Shoot[]> = {
  50: [
    ["stem-l-2", 3.6, 300, 1.0, 0.15], ["stem-l-2", 4.0, 190, 0.95, 0.15], ["stem-l-2", 4.35, 70, 0.9, 0.14],
    ["fol-l-top", 0.15, 250, 1.0, 0.15], ["fol-l-top", 0.4, 20, 0.95, 0.14], ["fol-l-top", 0.65, 140, 0.85, 0.14], ["fol-l-top", 0.9, 300, 0.75, 0.13], ["fol-l-top", 1.12, 200, 0.65, 0.13], ["fol-l-top", 1.32, 80, 0.55, 0.12],
    ["stem-c-2", 4.05, 210, 1.0, 0.15], ["stem-c-2", 4.4, 70, 0.95, 0.15], ["stem-c-2", 4.7, 320, 0.9, 0.14],
    ["fol-c-top", 0.15, 170, 1.05, 0.15], ["fol-c-top", 0.42, 300, 1.0, 0.15], ["fol-c-top", 0.7, 40, 0.9, 0.14], ["fol-c-top", 0.98, 265, 0.8, 0.14], ["fol-c-top", 1.22, 125, 0.7, 0.13], ["fol-c-top", 1.45, 350, 0.6, 0.12],
    ["stem-r-2", 3.6, 240, 1.0, 0.15], ["stem-r-2", 4.0, 350, 0.95, 0.15], ["stem-r-2", 4.35, 110, 0.9, 0.14],
    ["fol-r-top", 0.15, 290, 1.0, 0.15], ["fol-r-top", 0.4, 160, 0.95, 0.14], ["fol-r-top", 0.65, 40, 0.85, 0.14], ["fol-r-top", 0.9, 240, 0.75, 0.13], ["fol-r-top", 1.12, 340, 0.65, 0.13], ["fol-r-top", 1.32, 100, 0.55, 0.12],
  ],
  500: [["fol-l-top", 0.4, 200, 1.05, 0.34], ["fol-c-top", 0.4, 320, 1.1, 0.34], ["fol-r-top", 0.4, 60, 1.05, 0.34]],
};
/** 🗼️ The leafy spire round each leader per scale: from the fraction f0 of the leader, half width w, reaching ext beyond the leader tip. */
const SPIRE: Record<number, Record<string, [number, number, number]>> = {
  50: { l: [0.55, 0.2, 0.44], c: [0.55, 0.21, 0.53], r: [0.55, 0.2, 0.44] },
  500: { l: [0.45, 0.3, 0.44], c: [0.45, 0.31, 0.53], r: [0.45, 0.3, 0.44] },
};
/** 🌫️ The 1:500 crown: one weeping bough per kept branch, a single mass that follows the outer branch and turns over to hang (f0 where it starts on the branch, L its length along the arch and down, w its half width); `-t` rows are the boughs of the upturned tips of the boned branches. */
const COARSE: Record<string, [f0: number, L: number, w: number]> = {
  "l-a": [0.45, 1.8, 0.7], "l-a-t": [0.15, 2.1, 0.6], "l-b": [0.45, 2.6, 0.72], "l-b-t": [0.15, 2.7, 0.62], "c-a": [0.45, 2.7, 0.68], "c-a-t": [0.15, 2.4, 0.58],
  "c-b": [0.45, 2.6, 0.66], "c-b-t": [0.15, 2.3, 0.56], "r-a": [0.45, 1.62, 0.7], "r-a-t": [0.15, 1.85, 0.6], "r-b": [0.45, 2.6, 0.72], "r-b-t": [0.15, 2.7, 0.62],
  l1: [0.35, 3.1, 0.68], l2: [0.35, 3.3, 0.7], l3: [0.35, 3.52, 0.74], l5: [0.35, 3.0, 0.66], l7: [0.4, 2.6, 0.6],
  c1: [0.35, 3.3, 0.74], c2: [0.35, 2.9, 0.64], c3: [0.35, 3.5, 0.72], c4: [0.35, 3.5, 0.72], c5: [0.35, 2.9, 0.64], c9: [0.4, 2.9, 0.62],
  r1: [0.35, 3.1, 0.68], r2: [0.35, 3.3, 0.7], r3: [0.35, 3.6, 0.74], r5: [0.35, 3.0, 0.66], r7: [0.4, 2.6, 0.6],
  l4: [0.4, 2.3, 0.56], l6: [0.4, 2.1, 0.52], c6: [0.4, 2.1, 0.52], c7: [0.4, 1.9, 0.5], c8: [0.4, 1.6, 0.44], r4: [0.4, 2.3, 0.56], r6: [0.4, 2.1, 0.52],
  tl1: [0.3, 1.9, 0.48], tl2: [0.3, 1.8, 0.46], tl3: [0.3, 1.65, 0.44], tl4: [0.3, 1.5, 0.42], tl5: [0.3, 1.3, 0.38], tl6: [0.3, 1.05, 0.34],
  tc1: [0.3, 1.85, 0.47], tc2: [0.3, 1.75, 0.45], tc3: [0.3, 1.6, 0.43], tc4: [0.3, 1.4, 0.4], tc5: [0.3, 1.15, 0.36],
  tr1: [0.3, 1.9, 0.48], tr2: [0.3, 1.8, 0.46], tr3: [0.3, 1.65, 0.44], tr4: [0.3, 1.5, 0.42], tr5: [0.3, 1.3, 0.38], tr6: [0.3, 1.05, 0.34],
};


/** 🪨️ The stool per scale: sections (height, half width along x, half width along z). */
const STOOL: Record<number, [number, number, number][]> = {
  50: [[0, 0.37, 0.35], [0.04, 0.325, 0.31], [0.12, 0.275, 0.265], [0.24, 0.24, 0.235], [0.38, 0.2, 0.2], [0.5, 0.13, 0.14]],
  100: [[0, 0.37, 0.35], [0.06, 0.31, 0.295], [0.2, 0.25, 0.245], [0.38, 0.2, 0.2], [0.5, 0.13, 0.14]],
  200: [[0, 0.37, 0.35], [0.1, 0.285, 0.275], [0.36, 0.205, 0.205], [0.5, 0.14, 0.15]],
  500: [[0, 0.37, 0.35], [0.14, 0.27, 0.262], [0.5, 0.16, 0.17]],
};
/** 🌱️ Surface roots of the shallow birch root plate: azimuth, reach, radius at the stool. */
const ROOTS: Record<number, [number, number, number][]> = {
  50: [[20, 0.78, 0.075], [95, 0.7, 0.066], [165, 0.82, 0.078], [232, 0.72, 0.068], [300, 0.76, 0.07]],
  100: [[20, 0.78, 0.075], [95, 0.7, 0.066], [165, 0.82, 0.078], [232, 0.72, 0.068], [300, 0.76, 0.07]],
  200: [], 500: [],
};

/** 🔘️ Bark marks on the white stems: dark horizontal lenticel bands and the black diamond fissures above the dark base (bone, u, azimuth on the stem). */
const BANDS: [string, number, number][] = [
  ["stem-l-1", 1.55, 200], ["stem-l-1", 2.2, 320], ["stem-l-1", 2.6, 100], ["stem-l-1", 3.3, 240], ["stem-l-1", 4.0, 20], ["stem-l-1", 4.7, 150], ["stem-l-2", 0.9, 280], ["stem-l-2", 1.9, 60], ["stem-l-2", 2.9, 190],
  ["stem-c-1", 1.6, 300], ["stem-c-1", 2.3, 60], ["stem-c-1", 2.9, 200], ["stem-c-1", 3.8, 330], ["stem-c-1", 4.2, 120], ["stem-c-1", 5.0, 250], ["stem-c-2", 1.0, 20], ["stem-c-2", 2.4, 170], ["stem-c-2", 3.1, 300],
  ["stem-r-1", 1.5, 340], ["stem-r-1", 2.1, 220], ["stem-r-1", 2.5, 80], ["stem-r-1", 3.2, 300], ["stem-r-1", 3.9, 160], ["stem-r-1", 4.6, 30], ["stem-r-2", 1.0, 260], ["stem-r-2", 1.9, 120], ["stem-r-2", 3.0, 350],
];
const FISSURES: [string, number, number][] = [
  ["stem-l-1", 0.86, 250], ["stem-l-1", 0.98, 120], ["stem-l-1", 1.14, 330], ["stem-l-1", 1.3, 200], ["stem-l-1", 1.72, 40],
  ["stem-c-1", 0.86, 40], ["stem-c-1", 1.0, 290], ["stem-c-1", 1.12, 160], ["stem-c-1", 1.34, 230], ["stem-c-1", 1.86, 350],
  ["stem-r-1", 0.86, 310], ["stem-r-1", 0.96, 80], ["stem-r-1", 1.14, 210], ["stem-r-1", 1.28, 350], ["stem-r-1", 1.7, 140],
];

export type Tree = { scale: number; wood: Part[]; strands: Part[]; leaves: Part[]; marks: Mark[]; hidden: Part[]; cores: Part[] };

const LOD: Record<number, number> = { 50: 3, 100: 2, 200: 1, 500: 0 };
const SPAN: Record<number, number> = { 50: 3, 100: 2, 200: 2, 500: 1 };
const lengths = (asset: Json): Record<string, number> => Object.fromEntries(asset.rig.bones.map((b: Json) => [b.id, b.lengthM]));

/** 📍️ World point of a station: on the bone line at u, moved by the lateral lean and the stated offsets (a branch bows upwards by wx, a stem swerves in x). */
function place(F: Record<string, Frame>, L: Record<string, number>, kind: "stem" | "branch", bone: string, u: number, wx: number, wz: number): V3 {
  const f = F[bone]!;
  const lean = (LEAN[bone] ?? 0) * Math.max(0, u) / L[bone]!;
  return add(add(f.o, mul(f.u, u)), kind === "stem" ? [wx, 0, wz + lean] : [0, wx, wz + lean]);
}

/** 🎯️ The centre and radius of a stem at u of one of its bones, from the 1:50 stations (shared by every scale, so joints do not move between scales). */
export function stemAt(F: Record<string, Frame>, asset: Json, bone: string, u: number): { p: V3; r: number; t: V3 } {
  const L = lengths(asset);
  const chain = STEMS.find((c) => c.bones.some(([b]) => b === bone))!;
  const i = chain.bones.findIndex(([b]) => b === bone);
  const rows: St[] = [...(i > 0 ? [[0, 0, 0, chain.bones[i - 1]![1].at(-1)![3], 0] as St] : []), ...chain.bones[i]![1]];
  let k = 0;
  while (k + 2 < rows.length && rows[k + 1]![0] < u) k++;
  const [a, b] = [rows[k]!, rows[k + 1]!];
  const s = (u - a[0]) / (b[0] - a[0]);
  const at = (q: St): V3 => place(F, L, "stem", bone, q[0], q[1], q[2]);
  const [pa, pb] = [at(a), at(b)];
  return { p: add(pa, mul(sub(pb, pa), s)), r: a[3] + (b[3] - a[3]) * s, t: unit(sub(pb, pa)) };
}

/** ✂️ A chain of bones as one smoothed centre line, cut into one part per bone; every part reaches 8 cm past its joints into its neighbours so that poses stay closed. */
function chainParts(F: Record<string, Frame>, L: Record<string, number>, chain: Chain, scale: number): { bone: string; points: { p: V3; r: number }[] }[] {
  const lod = LOD[scale]!, n = SPAN[scale]!;
  const stations: { p: V3; r: number }[] = [];
  const last: number[] = [];
  for (const [bone, rows] of chain.bones) {
    for (const q of rows) if (q[4] <= lod) stations.push({ p: place(F, L, chain.kind, bone, q[0], q[1], q[2]), r: q[3] });
    last.push(stations.length - 1);
  }
  const line = smooth(stations, n);
  const step = stations.length >= 3 && n > 1 ? n : 1;
  const toward = (a: { p: V3; r: number }, b: { p: V3; r: number }): { p: V3; r: number } => {
    const d = Math.hypot(...sub(b.p, a.p)), s = Math.min(0.9, 0.08 / d);
    return { p: add(a.p, mul(sub(b.p, a.p), s)), r: a.r + (b.r - a.r) * s };
  };
  return chain.bones.map(([bone], i) => {
    const from = i === 0 ? 0 : last[i - 1]! * step, to = last[i]! * step;
    const points = line.slice(from, to + 1);
    if (from > 0) points.unshift(toward(line[from]!, line[from - 1]!));
    if (to + 1 < line.length) points.push(toward(line[to]!, line[to + 1]!));
    return { bone, points };
  });
}

const easeOver = (s: number, span: number): number => {
  const t = Math.max(0, Math.min(1, s / span));
  return t * t * (3 - 2 * t);
};
const curve = (table: [number, number][], s: number): number => {
  let k = 0;
  while (k + 2 < table.length && table[k + 1]![0] < s) k++;
  const [a, b] = [table[k]!, table[k + 1]!];
  return a[1] + ((b[1] - a[1]) * (s - a[0])) / (b[0] - a[0]);
};
/** 🧵️ Width profile of a hanging tress: full under the branch, tapering to a soft point. */
const TRESS: [number, number][] = [[0, 0.18], [0.1, 0.7], [0.22, 0.95], [0.45, 1], [0.72, 0.88], [0.88, 0.58], [1, 0.1]];
/** 🥖️ Width profile of a leafy sleeve along a branch, and of the spire round a leader. */
const SLEEVE: [number, number][] = [[0, 0.14], [0.1, 0.56], [0.25, 0.86], [0.5, 1], [0.8, 0.86], [0.93, 0.6], [1, 0.22]];
const SPIRE_PROFILE: [number, number][] = [[0, 0.5], [0.2, 0.95], [0.4, 1], [0.7, 0.74], [0.9, 0.44], [1, 0.14]];

/** 🧵️ A pendulous tress of leafy branchlets: leaves `start` drifting towards `az` at 25 degrees below the horizontal, turns down within its first third and hangs plumb; elliptic sections, half width w across the drift, half thickness t along it. */
export function tress(id: string, bone: string, start: V3, az: number, L: number, w: number, t: number, n: number, owner: string): Part {
  const x: V3 = [-Math.sin(az * D2R), 0, Math.cos(az * D2R)];
  const el = (s: number): number => -25 - 64 * easeOver(s, 0.35);
  const sections: Sec[] = [];
  let p = start;
  for (let i = 0; i <= n; i++) {
    const s = i / n;
    if (i > 0) p = add(p, mul(dirOf(az, el(s - 0.5 / n)), L / n));
    const z = dirOf(az, el(s));
    sections.push({ p, x, y: cross(z, x), rx: w * curve(TRESS, s), ry: t * curve(TRESS, s) });
  }
  return { id, bone, material: LEAF, tone: toneOf(bone), group: "leaf", sections, owner, az };
}

const PLAIN: Pick<Sec, "x" | "y"> = { x: [1, 0, 0], y: [0, 0, -1] };
/** 💊️ The same tress for 1:100 and 1:200: a tapering capsule from below its turn to its hanging end, as wide as the mean of its width and thickness. */
export function plainTress(part: Part, w: number, t: number): Part {
  const r0 = 0.5 * (w + t), r1 = 0.45 * r0;
  const p0 = part.sections[1]!.p, p1 = add(part.sections[part.sections.length - 1]!.p, [0, 0.6 * r1, 0]);
  return { ...part, sections: [{ p: p0, ...PLAIN, rx: r0, ry: r0 }, { p: p1, ...PLAIN, rx: r1, ry: r1 }], capsule: { p0, p1, r0, r1 } };
}

/** 〽️ The irregular run of a sleeve: sideways swerve of its centre (share of the half width) and width factor at eight places along it, in turn; read by the fraction along the sleeve, so every scale samples the same run. */
const SWERVE = [0, 0.16, -0.14, 0.18, -0.15, 0.1, -0.08, 0.05], SWELL = [1, 0.94, 1.1, 0.92, 1.08, 0.96, 1.04, 1];

/** 🥖️ A leafy sleeve: the short shoots and leaves that clothe the outer part of a branch, as a loft along the branch from the fraction f0 to `ext` beyond its tip, where it droops; half width w across, half as high (round for a spire), lifted a little onto the upper side, swerving and swelling irregularly from section to section (`turn` shifts that run). */
export function sleeve(id: string, bone: string, owner: Part, f0: number, w: number, ext: number, n: number, spire: boolean, turn = 0): Part {
  const line: V3[] = Array.from({ length: n }, (_, i) => along(owner, f0 + ((1 - f0) * i) / (n - 1)));
  const end = unit(sub(line[n - 1]!, line[n - 2]!));
  line.push(add(line[n - 1]!, spire ? mul(end, ext) : add(mul(end, ext * 0.8), [0, -0.55 * ext, 0])));
  const sections: Sec[] = line.map((q, i) => {
    const z = unit(sub(line[Math.min(n, i + 1)]!, line[Math.max(0, i - 1)]!));
    const flat = cross([0, 1, 0], z);
    const x = unit(Math.hypot(...flat) > 0.2 ? flat : cross(z, [0, 0, 1]));
    const run = (table: number[]): number => {
      const at = (i / n) * 7 + turn, k0 = Math.floor(at);
      return table[k0 % 8]! + (table[(k0 + 1) % 8]! - table[k0 % 8]!) * (at - k0);
    };
    const k = curve(spire ? SPIRE_PROFILE : SLEEVE, i / n) * (spire ? 1 : run(SWELL));
    const ry = (spire ? 1 : 0.5) * w * k;
    const swerve = spire || i === 0 || i === n ? 0 : run(SWERVE) * w;
    return { p: add(add(q, mul(x, swerve)), [0, spire ? 0 : 0.35 * ry, 0]), x, y: cross(z, x), rx: w * k, ry };
  });
  return { id, bone, material: LEAF, tone: toneOf(bone), group: "leaf", sections, owner: owner.id };
}

/** 🥬️ Width profile of a weeping bough along its length: thin where it leaves the branch, fullest past the middle, a blunt hanging end. */
const BOUGH: [number, number][] = [[0, 0.2], [0.12, 0.55], [0.3, 0.88], [0.5, 1], [0.72, 0.95], [0.88, 0.7], [0.96, 0.42], [1, 0.15]];

/** 🌿️ A weeping bough as one mass: from `start` heading `az`, rising at e0 along its branch for the first quarter, turning over and hanging at 80 degrees below the horizontal; half width w across, half thickness t in the plane of the bend. */
export function bough(id: string, bone: string, start: V3, az: number, e0: number, L: number, w: number, t: number, n: number, owner: string): Part {
  const x: V3 = [-Math.sin(az * D2R), 0, Math.cos(az * D2R)];
  const el = (s: number): number => e0 + (-80 - e0) * easeOver(s - 0.22, 0.5);
  const sections: Sec[] = [];
  let p = start;
  for (let i = 0; i <= n; i++) {
    const s = i / n;
    if (i > 0) p = add(p, mul(dirOf(az, el(s - 0.5 / n)), L / n));
    const z = dirOf(az, el(s));
    sections.push({ p, x, y: cross(z, x), rx: w * curve(BOUGH, s), ry: t * curve(BOUGH, s) });
  }
  return { id, bone, material: LEAF, tone: toneOf(bone), group: "leaf", sections, owner };
}

const rise = (part: Part, f: number): number => {
  const [a, b] = [along(part, Math.max(0, f - 0.08)), along(part, Math.min(1, f + 0.08))];
  return Math.atan2(b[1] - a[1], Math.hypot(b[0] - a[0], b[2] - a[2])) / D2R;
};
const heading = (part: Part, f: number): number => {
  const [a, b] = [along(part, Math.max(0, f - 0.08)), along(part, Math.min(1, f + 0.08))];
  return Math.atan2(b[2] - a[2], b[0] - a[0]) / D2R;
};
const TRESS_N = 9;
/** 🪟️ At 1:50 a tress is a curtain of pendulous twigs: 1.2 times as wide as its capsule at the coarser scales and half of its half width thick. */
const CURTAIN = 1.2, FLAT = 0.5;
const SLEEVE_N: Record<number, number> = { 50: 10, 100: 6, 200: 5, 500: 6 };
const thick = (w: number): number => Math.min(0.75 * w, 0.11 + 0.3 * w);
/** 🪢️ Lengths of the bare whip ends that hang below every second tress at 1:50, in turn. */
const WHIPS = [0.45, 0.6, 0.35];

export function tree(scale: number, F: Record<string, Frame>, asset: Json): Tree {
  const L = lengths(asset);
  const feature = (0.15 * scale) / 1000;
  const wood: Part[] = [], strands: Part[] = [], leaves: Part[] = [], marks: Mark[] = [], hidden: Part[] = [], cores: Part[] = [];
  const slim = (points: { p: V3; r: number }[]): Json | undefined => {
    const d = 2 * Math.max(...points.map((q) => q.r));
    const length = points.slice(1).reduce((s, q, i) => s + Math.hypot(...sub(q.p, points[i]!.p)), 0);
    return d < feature && length >= 20 * d ? { slender: true } : undefined;
  };
  const stool = STOOL[scale]!.map(([y, a, b]): Sec => ({ p: [0, y, 0], x: [1, 0, 0], y: [0, 0, -1], rx: a, ry: b }));
  wood.push({ id: "stool", bone: "root-plate", material: DARK, tone: TONE.stool, group: "stool", sections: stool });
  ROOTS[scale]!.forEach(([az, reach, r], i) => {
    const d: V3 = [Math.cos(az * D2R), 0, Math.sin(az * D2R)];
    const pts = [{ p: add(mul(d, 0.16), [0, 0.2, 0]), r: r * 1.15 }, { p: add(mul(d, 0.34), [0, 0.1, 0]), r }, { p: add(mul(d, 0.34 + (reach - 0.34) * 0.5), [0, 0.045, 0]), r: r * 0.72 }, { p: add(mul(d, reach), [0, 0.022, 0]), r: 0.024 }];
    wood.push({ id: `root-${i + 1}`, bone: "root-plate", material: DARK, tone: TONE.stool, group: "root", sections: tubeSections(smooth(pts, scale === 50 ? 2 : 1)).map((s) => ({ ...s, ry: s.ry * 0.8 })) });
  });
  const n = TRESS_N, m = SLEEVE_N[scale]!;
  let whip = 0;
  const hang = (owner: Part, bone: string, key: string, list: T[]): void => list.forEach(([f, daz, len, w], i) => {
    const ww = scale === 50 ? CURTAIN * w : w;
    const loft = tress(`leaf-${key}-${i + 1}`, bone, along(owner, f), heading(owner, f) + daz, len, ww, scale === 50 ? FLAT * ww : thick(w), n, owner.id);
    const part = scale === 50 ? loft : plainTress(loft, w, thick(w));
    leaves.push(part);
    if (scale === 50 && i % 2 === 1 && whip++ % 2 === 0) strands.push(strand(part, WHIPS[whip % 3]!));
  });
  let turn = 0;
  const shoots = (owner: Part, bone: string, key: string): void => (SIDES[key] ?? []).forEach(([f, daz, len, w, t1, t2], i) => {
    const az = heading(owner, f) + daz;
    const pts: { p: V3; r: number }[] = [{ p: along(owner, f), r: 0.006 }];
    for (let k = 1; k <= 4; k++) pts.push({ p: add(pts[k - 1]!.p, mul(dirOf(az + (daz > 0 ? -12 : 12) * (k / 4), 18 - 40 * ((k - 0.5) / 4)), len / 4)), r: 0.005 });
    const guide: Part = { id: `side-${key}-${i + 1}`, bone, material: WHITE, tone: TONE.branch, group: "branch", sections: tubeSections(pts) };
    leaves.push(sleeve(`leaf-${key}-v${i + 1}`, bone, guide, 0.12, w, 0.16, scale === 200 ? 4 : scale === 100 ? 5 : 6, false, ++turn));
    hang(guide, bone, `${key}-v${i + 1}`, [[0.55, daz > 0 ? 80 : -80, t1, 0.14], [1, 0, t2, 0.15]]);
  });
  const strand = (part: Part, length: number): Part => {
    const from = part.sections[part.sections.length - 2]!.p;
    const sway: V3 = [0.03 * Math.cos((part.az ?? 0) * D2R), 0, 0.03 * Math.sin((part.az ?? 0) * D2R)];
    const to = add(from, add(sway, [0, -(length + Math.hypot(...sub(part.sections[part.sections.length - 1]!.p, from))), 0]));
    return { id: `strand-${part.id.slice(5)}`, bone: part.bone, material: WHITE, tone: TONE.strand, group: "strand", sections: [{ p: from, ...PLAIN, rx: 0.0045, ry: 0.0045 }, { p: to, ...PLAIN, rx: 0.0035, ry: 0.0035 }], capsule: { p0: from, p1: to, r0: 0.0045, r1: 0.0035 }, flags: { slender: true, texture: true }, owner: part.id };
  };
  for (const chain of STEMS) {
    const parts = chainParts(F, L, chain, scale);
    parts.forEach((part, i) => {
      const bone = part.bone;
      if (i === 0 && scale !== 500) {
        const top = part.points.reduce((best, q, k) => (Math.abs(q.p[1] - 0.85) < Math.abs(part.points[best]!.p[1] - 0.85) ? k : best), 0);
        const from = Math.max(1, top - (scale === 50 ? 3 : scale === 100 ? 2 : 1));
        const base = part.points.slice(0, top + 1).map((q, k) => ({ p: q.p, r: k === top ? q.r - 0.004 : q.r + 0.007 }));
        wood.push({ id: `base-${chain.id}`, bone, material: DARK, tone: TONE.base, group: "base", sections: tubeSections(base) });
        wood.push({ id: bone, bone, material: WHITE, tone: TONE.stem, group: "stem", sections: tubeSections(part.points.slice(from)) });
        cores.push({ id: bone, bone, material: WOOD, tone: 0.4, group: "stem", sections: tubeSections(part.points) });
      } else {
        const leader: Part = { id: i === 2 ? `leader-${chain.id}` : bone, bone, material: WHITE, tone: TONE.stem, group: i === 2 ? "leader" : "stem", sections: tubeSections(part.points), flags: slim(part.points) };
        wood.push(leader);
        if (i === 2) {
          const [f0, w, ext] = SPIRE[scale === 500 ? 500 : 50]![chain.id]!;
          leaves.push(sleeve(`leaf-${bone}-s`, bone, leader, f0, w, ext, m + 1, true));
        }
      }
    });
  }
  for (const chain of BONED) {
    const [br, tip] = chainParts(F, L, chain, scale);
    const a: Part = { id: br!.bone, bone: br!.bone, material: WHITE, tone: TONE.branch, group: "branch", sections: tubeSections(br!.points), flags: scale === 500 ? { slender: true } : slim(br!.points) };
    const b: Part = { id: `tip-${chain.id}`, bone: tip!.bone, material: WHITE, tone: TONE.branch, group: "tip", sections: tubeSections(tip!.points), flags: scale === 500 ? { slender: true } : slim(tip!.points) };
    (scale === 500 ? hidden : wood).push(a, b);
    if (scale === 500) {
      const [f0, len, w] = COARSE[chain.id]!, [g0, glen, gw] = COARSE[`${chain.id}-t`]!;
      leaves.push(bough(`leaf-${chain.id}-s`, a.bone, along(a, f0), heading(a, f0), rise(a, f0), len, w, 0.55 * w, 9, a.id));
      leaves.push(bough(`leaf-${chain.id}-t-s`, b.bone, along(b, g0), heading(a, 0.9), rise(b, 0.5), glen, gw, 0.55 * gw, 9, b.id));
      continue;
    }
    const [f0, w] = [chain.sl[0], chain.sl[1] * 1.15];
    leaves.push(sleeve(`leaf-${chain.id}-s`, a.bone, a, 0.52, w * 0.86, 0.12, m, false, ++turn));
    leaves.push(sleeve(`leaf-${chain.id}-t-s`, b.bone, b, 0.1, w * 0.78, 0.22, m, false, ++turn));
    void f0;
    shoots(a, a.bone, chain.id);
    shoots(b, b.bone, `${chain.id}-t`);
    hang(a, a.bone, chain.id, chain.p.br);
    hang(b, b.bone, `${chain.id}-t`, chain.p.tip);
  }
  for (const b of FREE) {

    const at = stemAt(F, asset, b.bone, b.u);
    const k = scale === 50 ? 6 : scale === 100 ? 4 : scale === 200 ? 3 : 2;
    const path: V3[] = [at.p];
    for (let i = 1; i <= 12; i++) {
      const s = (i - 0.5) / 12;
      path.push(add(path[i - 1]!, mul(dirOf(b.az + b.bend * s, b.e0 + (b.e1 - b.e0) * s ** 0.9), b.L / 12)));
    }
    const pts: { p: V3; r: number }[] = Array.from({ length: k + 1 }, (_, i) => ({ p: path[(i * 12) / k]!, r: i === 0 ? b.r0 * 1.25 : b.r0 + (0.005 - b.r0) * (i / k) }));
    const part: Part = { id: `branch-${b.id}`, bone: b.bone, material: WHITE, tone: TONE.branch, group: b.lod === 1 ? "branch" : "twig", sections: tubeSections(pts), flags: slim(pts) };
    if (scale !== 500) wood.push(part);
    if (scale === 500) {
      const [f0, len, w] = COARSE[b.id]!;
      leaves.push(bough(`leaf-${b.id}-s`, b.bone, along(part, f0), heading(part, f0), rise(part, f0), len, w, 0.55 * w, 9, part.id));
      continue;
    }
    const [f0, w] = [b.sl[0], b.sl[1] * 1.15];
    leaves.push(sleeve(`leaf-${b.id}-s`, b.bone, part, f0, w, 0.26, m, false, ++turn));
    shoots(part, b.bone, b.id);
    hang(part, b.bone, b.id, b.p);
    if (scale <= 100) marks.push(scar(F, asset, `scar-${b.id}`, b.bone, b.u - 0.05, b.az, scale));
  }
  if (scale <= 100) for (const chain of BONED) {
    const bone = asset.rig.bones.find((x: Json) => x.id === chain.bones[0]![0]);
    marks.push(scar(F, asset, `scar-${chain.id}`, bone.parent, bone.x - 0.06, /^(l-a|l-b|c-a)$/.test(chain.id) ? 180 : 0, scale));
  }
  if (scale === 50) {
    BANDS.forEach(([bone, u, az], i) => marks.push(patch(F, asset, `lenticel-${i + 1}`, bone, u, az, [0.006, 0.008, 0.62])));
    FISSURES.forEach(([bone, u, az], i) => marks.push(patch(F, asset, `fissure-${i + 1}`, bone, u, az, [0.011, 0.075, 0.26])));
  }
  if (scale === 100) BANDS.filter((_, i) => i % 3 === 0).forEach(([bone, u, az], i) => marks.push(patch(F, asset, `lenticel-${i + 1}`, bone, u, az, [0.008, 0.016, 0.7])));
  SHOOTS[scale === 500 ? 500 : 50]!.forEach(([bone, u, az, len, w], i) => {
    const loft = tress(`leaf-${bone}-${i + 1}`, bone, stemAt(F, asset, bone, u).p, az, len, w, thick(w), n, bone.startsWith("fol-") ? `leader-${bone[4]}` : bone);
    const part = scale === 50 ? loft : plainTress(loft, w, thick(w));
    leaves.push(part);
    if (scale === 50 && i % 3 === 1 && whip++ % 2 === 0) strands.push(strand(part, WHIPS[whip % 3]!));
  });
  return { scale, wood, strands, leaves, marks, hidden, cores };
}

/** ⚫️ A dark patch on the stem surface at u of a bone, facing the azimuth az: radii are thickness, height along the stem and half width as a share of the stem radius. */
function patch(F: Record<string, Frame>, asset: Json, id: string, bone: string, u: number, az: number, [thick, high, wide]: V3): Mark {
  const at = stemAt(F, asset, bone, u);
  const out0: V3 = [Math.cos(az * D2R), 0, Math.sin(az * D2R)];
  const out = unit(sub(out0, mul(at.t, out0[0] * at.t[0] + out0[2] * at.t[2])));
  return { id, bone, material: DARK, tone: TONE.mark, c: add(at.p, mul(out, at.r - thick * 0.35)), axes: [out, at.t, cross(out, at.t)], radii: [thick, high, wide * at.r] };
}
/** 🔺️ The black scar under a branch (the "moustache" of a birch): a dark eye on the stem just below the branch collar. */
const scar = (F: Record<string, Frame>, asset: Json, id: string, bone: string, u: number, az: number, scale: number): Mark => patch(F, asset, id, bone, u, az, scale === 50 ? [0.009, 0.034, 0.8] : [0.01, 0.04, 0.85]);
