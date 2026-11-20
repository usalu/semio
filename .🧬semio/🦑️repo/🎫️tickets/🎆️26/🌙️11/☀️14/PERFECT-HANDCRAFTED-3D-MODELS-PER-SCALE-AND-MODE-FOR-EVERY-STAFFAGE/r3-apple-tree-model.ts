/** 🍎️ Round-3 apple tree (full-standard orchard tree, Oeschberg crown): every limb section, branch, lobe, fruit and outlet is stated by hand per scale in world metres (x ahead, y up, z lateral) and written as bone-local solids to `volumes/vegetation/apple-tree.json`; the root fixes (lateral bone offsets, emitter anchors, 2D drawings as true views of each scale's body) go to the candidate document `🗑️generated/r3-apple-tree/🔣️.json`.
 * @see ./round3-brief.md
 * @see ./r3-vegetation-apple-tree.md */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { GENERATED, SOURCES, entryOf, frames, toLocal, type Frame, type Json, type V3 } from "./volumes-lib.ts";
import { D2R, add, along, branch, dirOf, dot3, mul, r4, r5, ringLobe, solidOf, sub, surfacePoint, unit, type Fruit, type Lobe, type Part, type Tube, type View } from "./r3-apple-tree-kit.ts";
import { draw } from "./r3-apple-tree-draw.ts";

const { entry } = entryOf("vegetation/apple-tree");
const document: Json = JSON.parse(readFileSync(entry.path, "utf8"));
const asset: Json = document.asset;
const bone = (id: string): Json => asset.rig.bones.find((b: Json) => b.id === id);

const LIMBS = { w1: { az: 189, a: "sc-w1a", b: "sc-w1b" }, e1: { az: 9, a: "sc-e1a", b: "sc-e1b" }, w2: { az: 143, a: "sc-w2a", b: "sc-w2b" }, e2: { az: 323, a: "sc-e2a", b: "sc-e2b" } } as const;
type LimbKey = keyof typeof LIMBS;

for (const b of asset.rig.bones) delete b.z;
const planar = frames(asset);
const lateral = (limb: LimbKey, x: number): number => (x - planar[LIMBS[limb].a]!.o[0]) * Math.tan(LIMBS[limb].az * D2R);
const absoluteZ: Record<string, number> = {};
for (const [key, l] of Object.entries(LIMBS) as [LimbKey, (typeof LIMBS)[LimbKey]][]) {
  absoluteZ[l.b] = lateral(key, planar[l.b]!.o[0]);
  const n = key;
  absoluteZ[`spur-${n}`] = lateral(key, planar[`spur-${n}`]!.o[0]);
  absoluteZ[`fol-${n}`] = lateral(key, planar[`fol-${n}`]!.o[0]);
}
for (const b of asset.rig.bones) if (absoluteZ[b.id] !== undefined) b.z = r4(absoluteZ[b.id]! - (absoluteZ[b.parent] ?? 0));
const F: Record<string, Frame> = frames(asset);

const BARK = "apple-bark", YOUNG = "apple-bark-young", TWIG = "apple-twig", SUN = "apple-leaf-sun", SHADE = "apple-leaf-shade";
const TONE = { root: 0.28, stem: 0.36, limb: 0.38, twig: 0.2, sun: 0.6, shade: 0.45 };

/** 🪵️ A scaffold part: rows [u along the bone, bow upwards, radius]; the lateral position follows the limb's azimuth. */
function scaffold(limb: LimbKey, part: "a" | "b", rows: [number, number, number][]): Tube {
  const id = LIMBS[limb][part];
  const f = planar[id]!;
  const up: V3 = f.v[1] > 0 ? f.v : mul(f.v, -1);
  return {
    kind: "tube", id, bone: id, material: YOUNG, tone: TONE.limb, group: "limb",
    points: rows.map(([u, bow, r]) => {
      const p = add(add(f.o, mul(f.u, u)), mul(up, bow));
      return { p: [p[0], p[1], lateral(limb, p[0])] as V3, r };
    }),
  };
}

/** 🌳️ A stem part on the stem axis: rows [height y, radius, x offset, z offset]. */
const stem = (id: string, material: string, tone: number, group: string, rows: [number, number, number, number][]): Tube => ({ kind: "tube", id, bone: id, material, tone, group, points: rows.map(([y, r, dx, dz]) => ({ p: [-0.0087 * Math.max(0, y - 1.6) + dx, y, dz] as V3, r })) });

const envelope = (az: number, rx: number, rz: number): number => 1 / Math.hypot(Math.cos(az * D2R) / rx, Math.sin(az * D2R) / rz);
const ENV = (az: number): number => envelope(az, 3.3945, 3.1);
const dome = (y: number): number => (y >= 3.25 ? (1 - ((y - 3.25) / 3.1) ** 2.5) ** 0.4 : Math.sqrt(1 - ((3.25 - y) / 1.3) ** 2));
type LobeRow = [id: string, bone: string, az: number, k: number, y: number, a: number, b: number, c: number, tilt: number];
const lobes = (rows: LobeRow[]): Lobe[] => rows.map(([id, b, az, k, y, a, bb, c, tilt]) => {
  const shade = b.startsWith("fol-core") || id === "crown-core";
  const reach = k > 1.5 ? k - 100 : ENV(Math.abs(az - 89) < 0.5 || Math.abs(az - 269) < 0.5 ? az + 1 : az) * dome(y) * k;
  return ringLobe(id, b, shade ? SHADE : SUN, shade ? TONE.shade : TONE.sun, shade ? "shade" : "sun", az, reach, y, a, bb, c, tilt, -0.02);
});
const FIX = (reach: number): number => 100 + reach;

type FruitRow = [view: View | "none", lobe: string, az: number, el: number];
const fruits = (rows: FruitRow[], all: Lobe[]): Fruit[] => {
  const count: Record<string, number> = {};
  return rows.map(([view, lobeId, az, el]) => {
    const l = all.find((x) => x.id === lobeId);
    if (!l) throw new Error(`fruit on unknown lobe ${lobeId}`);
    const d = dirOf(az, el);
    const n = (count[l.bone] = (count[l.bone] ?? 0) + 1);
    return { kind: "fruit", id: `fruit-${l.bone.startsWith("fol-") ? l.bone.slice(4) : l.bone}-${n}`, bone: l.bone, c: add(surfacePoint(l, d), mul(d, 0.024)), r: 0.04, view: view as View };
  });
};

type Outlet = [emitter: string, anchor: string | null, lobe: string, az: number, el: number];
type Model = { scale: number; wood: Tube[]; twigs: Tube[]; hiddenWood: Tube[]; hiddenTwigs: Tube[]; knot: Json[]; lobes: Lobe[]; crownBone?: string; fruit: Fruit[]; hiddenFruit: Fruit[]; outlets: Outlet[]; decisions: { en: string; de: string }[] };

const SCAFFOLD_50: [LimbKey, "a" | "b", [number, number, number][]][] = [
  ["w1", "a", [[-0.05, 0, 0.118], [0.26, 0, 0.122], [0.45, 0.008, 0.104], [0.8, 0.022, 0.095], [1.2, 0.03, 0.087], [1.55, 0.02, 0.08], [1.8, 0, 0.074], [1.87, -0.012, 0.07]]],
  ["w1", "b", [[-0.06, 0.012, 0.072], [0, 0, 0.07], [0.4, 0.02, 0.062], [0.8, 0.03, 0.053], [1.2, 0.025, 0.043], [1.45, 0.012, 0.035], [1.6, 0, 0.028]]],
  ["e1", "a", [[-0.05, 0, 0.118], [0.27, 0, 0.12], [0.48, 0.01, 0.103], [0.85, 0.025, 0.094], [1.3, 0.03, 0.086], [1.65, 0.018, 0.079], [1.9, 0, 0.073], [1.97, -0.012, 0.069]]],
  ["e1", "b", [[-0.06, 0.012, 0.071], [0, 0, 0.069], [0.42, 0.022, 0.061], [0.82, 0.03, 0.052], [1.2, 0.022, 0.042], [1.45, 0.01, 0.034], [1.6, 0, 0.027]]],
  ["w2", "a", [[-0.05, 0, 0.1], [0.2, 0, 0.104], [0.38, 0.006, 0.09], [0.8, 0.02, 0.082], [1.3, 0.028, 0.074], [1.8, 0.018, 0.066], [2.2, 0, 0.06], [2.27, -0.008, 0.057]]],
  ["w2", "b", [[-0.06, 0.008, 0.058], [0, 0, 0.056], [0.4, 0.018, 0.049], [0.8, 0.022, 0.04], [1.2, 0.014, 0.031], [1.5, 0, 0.022]]],
  ["e2", "a", [[-0.05, 0, 0.1], [0.2, 0, 0.103], [0.4, 0.006, 0.09], [0.85, 0.02, 0.082], [1.35, 0.028, 0.074], [1.9, 0.018, 0.066], [2.3, 0, 0.06], [2.37, -0.008, 0.057]]],
  ["e2", "b", [[-0.06, 0.008, 0.058], [0, 0, 0.056], [0.4, 0.018, 0.048], [0.8, 0.022, 0.039], [1.2, 0.014, 0.03], [1.5, 0, 0.021]]],
];
const SCAFFOLD_100: typeof SCAFFOLD_50 = [
  ["w1", "a", [[-0.05, 0, 0.115], [0.3, 0, 0.115], [0.9, 0.024, 0.093], [1.8, 0, 0.074], [1.88, -0.012, 0.07]]],
  ["w1", "b", [[-0.06, 0.012, 0.071], [0.5, 0.024, 0.059], [1.1, 0.026, 0.045], [1.6, 0, 0.028]]],
  ["e1", "a", [[-0.05, 0, 0.115], [0.3, 0, 0.114], [0.95, 0.026, 0.092], [1.9, 0, 0.073], [1.98, -0.012, 0.069]]],
  ["e1", "b", [[-0.06, 0.012, 0.07], [0.5, 0.024, 0.058], [1.1, 0.024, 0.044], [1.6, 0, 0.027]]],
  ["w2", "a", [[-0.05, 0, 0.098], [0.25, 0, 0.098], [1.1, 0.026, 0.077], [2.2, 0, 0.06], [2.28, -0.008, 0.057]]],
  ["w2", "b", [[-0.06, 0.008, 0.057], [0.5, 0.02, 0.046], [1.0, 0.018, 0.035], [1.5, 0, 0.022]]],
  ["e2", "a", [[-0.05, 0, 0.098], [0.25, 0, 0.097], [1.15, 0.026, 0.077], [2.3, 0, 0.06], [2.38, -0.008, 0.057]]],
  ["e2", "b", [[-0.06, 0.008, 0.057], [0.5, 0.02, 0.045], [1.0, 0.018, 0.034], [1.5, 0, 0.021]]],
];
const SCAFFOLD_200: typeof SCAFFOLD_50 = [
  ["w1", "a", [[-0.05, 0, 0.112], [0.9, 0.02, 0.092], [1.9, -0.01, 0.07]]],
  ["w1", "b", [[-0.06, 0.01, 0.07], [0.8, 0.028, 0.052], [1.6, 0, 0.028]]],
  ["e1", "a", [[-0.05, 0, 0.112], [0.95, 0.022, 0.091], [2.0, -0.01, 0.069]]],
  ["e1", "b", [[-0.06, 0.01, 0.069], [0.8, 0.028, 0.051], [1.6, 0, 0.027]]],
  ["w2", "a", [[-0.05, 0, 0.096], [1.1, 0.024, 0.076], [2.3, -0.008, 0.057]]],
  ["w2", "b", [[-0.06, 0.008, 0.056], [0.75, 0.02, 0.04], [1.5, 0, 0.022]]],
  ["e2", "a", [[-0.05, 0, 0.096], [1.15, 0.024, 0.076], [2.4, -0.008, 0.057]]],
  ["e2", "b", [[-0.06, 0.008, 0.056], [0.75, 0.02, 0.039], [1.5, 0, 0.021]]],
];
const SCAFFOLD_500: typeof SCAFFOLD_50 = [
  ["w1", "a", [[0, 0, 0.1], [1.86, 0, 0.068]]], ["w1", "b", [[-0.04, 0, 0.066], [1.6, 0, 0.04]]],
  ["e1", "a", [[0, 0, 0.1], [1.96, 0, 0.068]]], ["e1", "b", [[-0.04, 0, 0.066], [1.6, 0, 0.04]]],
  ["w2", "a", [[0, 0, 0.086], [2.26, 0, 0.054]]], ["w2", "b", [[-0.04, 0, 0.052], [1.5, 0, 0.038]]],
  ["e2", "a", [[0, 0, 0.086], [2.36, 0, 0.054]]], ["e2", "b", [[-0.04, 0, 0.052], [1.5, 0, 0.038]]],
];

type SideRow = [id: string, parent: string, f: number, az: number, e0: number, e1: number, L: number, r0: number, r1: number, bend: number];
const SIDES_50: SideRow[] = [
  ["sc-w1a-s1", "sc-w1a", 0.6, 258, 28, 4, 1.8, 0.035, 0.012, 8],
  ["sc-w1a-s2", "sc-w1a", 0.8, 148, 25, 0, 1.5, 0.028, 0.01, -10],
  ["sc-w1b-s1", "sc-w1b", 0.4, 236, 22, -6, 0.95, 0.022, 0.009, 6],
  ["sc-e1a-s1", "sc-e1a", 0.6, 78, 28, 4, 1.8, 0.035, 0.012, 8],
  ["sc-e1a-s2", "sc-e1a", 0.8, -32, 25, 0, 1.5, 0.028, 0.01, -10],
  ["sc-e1b-s1", "sc-e1b", 0.4, 56, 22, -6, 0.95, 0.022, 0.009, 6],
  ["sc-w2a-s1", "sc-w2a", 0.55, 92, 30, 4, 1.3, 0.028, 0.01, -8],
  ["sc-w2a-s2", "sc-w2a", 0.78, 196, 34, 8, 1.4, 0.026, 0.01, 8],
  ["sc-w2b-s1", "sc-w2b", 0.4, 112, 26, 4, 0.8, 0.02, 0.008, -6],
  ["sc-e2a-s1", "sc-e2a", 0.55, 272, 30, 4, 1.3, 0.028, 0.01, -8],
  ["sc-e2a-s2", "sc-e2a", 0.78, 16, 34, 8, 1.4, 0.026, 0.01, 8],
  ["sc-e2b-s1", "sc-e2b", 0.4, 292, 26, 4, 0.8, 0.02, 0.008, -6],
  ["ld-s1", "ld", 0.5, 98, 32, 10, 1.7, 0.034, 0.012, 10],
  ["ld-s2", "ld", 0.62, 278, 32, 10, 1.7, 0.034, 0.012, 10],
];
const TWIGLETS_50: [parent: string, f: number, dAz: number][] = [
  ["sc-w1a-s1", 0.55, 38], ["sc-w1a-s2", 0.5, -36], ["sc-e1a-s1", 0.55, 38], ["sc-e1a-s2", 0.5, -36],
  ["sc-w2a-s1", 0.55, -40], ["sc-w2a-s2", 0.5, 18], ["sc-e2a-s1", 0.55, -40], ["sc-e2a-s2", 0.5, 36],
];
const SIDES_100: SideRow[] = [
  ["sc-w1a-s1", "sc-w1a", 0.6, 260, 26, 4, 1.5, 0.03, 0.012, 6],
  ["sc-w1a-s2", "sc-w1a", 0.78, 150, 24, 2, 1.45, 0.028, 0.012, -8],
  ["sc-e1a-s1", "sc-e1a", 0.6, 80, 26, 4, 1.5, 0.03, 0.012, 6],
  ["sc-e1a-s2", "sc-e1a", 0.78, -30, 24, 2, 1.45, 0.028, 0.012, -8],
  ["sc-w2a-s1", "sc-w2a", 0.6, 96, 28, 6, 1.15, 0.028, 0.012, -6],
  ["sc-w2a-s2", "sc-w2a", 0.8, 195, 32, 10, 1.3, 0.026, 0.012, 6],
  ["sc-e2a-s1", "sc-e2a", 0.6, 276, 28, 6, 1.15, 0.028, 0.012, -6],
  ["sc-e2a-s2", "sc-e2a", 0.8, 15, 32, 10, 1.3, 0.026, 0.012, 6],
];

function woody(scale: number): { wood: Tube[]; twigs: Tube[] } {
  const wood: Tube[] = [];
  if (scale === 50) {
    wood.push(stem("root-plate", BARK, TONE.root, "root", [[0, 0.3, 0, 0], [0.04, 0.285, 0, 0], [0.1, 0.255, 0.004, 0], [0.18, 0.232, 0.006, -0.003], [0.27, 0.214, 0.008, -0.005], [0.35, 0.207, 0.009, -0.006], [0.41, 0.204, 0.01, -0.007]]));
    const buttress: [number, number][] = [[18, 1], [96, 0.9], [162, 1.05], [236, 0.92], [303, 1]];
    buttress.forEach(([az, k], i) => {
      const d = (r: number, y: number): V3 => [r * k * Math.cos(az * D2R), y, r * k * Math.sin(az * D2R)];
      wood.push({ kind: "tube", id: `root-${i + 1}`, bone: "root-plate", material: BARK, tone: TONE.root, group: "root", points: [{ p: d(0.13, 0.36), r: 0.085 }, { p: d(0.22, 0.23), r: 0.086 }, { p: d(0.33, 0.12), r: 0.074 }, { p: d(0.45, 0.062), r: 0.052 }, { p: d(0.56, 0.04), r: 0.034 }, { p: d(0.64, 0.03), r: 0.022 }] });
    });
    wood.push(stem("trunk", BARK, TONE.stem, "stem", [[0.33, 0.207, 0.009, -0.006], [0.6, 0.2, 0.012, -0.008], [0.95, 0.192, 0.015, -0.002], [1.25, 0.188, 0.008, 0.008], [1.45, 0.196, 0.003, 0.004], [1.6, 0.212, 0, 0], [1.7, 0.2, 0, 0], [1.76, 0.178, 0, 0], [1.8, 0.152, 0, 0]]));
    wood.push(stem("ld", BARK, TONE.stem, "stem", [[1.56, 0.165, 0, 0], [1.78, 0.152, 0, 0], [2.1, 0.14, 0.004, 0], [2.32, 0.136, 0.006, 0.004], [2.6, 0.116, 0.004, 0.008], [3.0, 0.105, -0.004, 0.006], [3.4, 0.094, -0.006, 0], [3.8, 0.084, 0, 0], [3.88, 0.08, 0, 0]]));
    wood.push(stem("ld-b", YOUNG, TONE.limb, "limb", [[3.76, 0.082, -0.019, 0], [4.0, 0.075, -0.016, 0.004], [4.4, 0.062, -0.012, 0.008], [4.8, 0.048, -0.02, 0.004], [5.1, 0.036, -0.03, -0.002], [5.3, 0.022, -0.036, -0.004]]));
  } else if (scale === 100) {
    wood.push(stem("root-plate", BARK, TONE.root, "root", [[0, 0.52, 0, 0], [0.03, 0.42, 0, 0], [0.09, 0.32, 0.003, 0], [0.18, 0.26, 0.005, 0], [0.28, 0.22, 0.008, -0.004], [0.4, 0.205, 0.01, -0.006]]));
    wood.push(stem("trunk", BARK, TONE.stem, "stem", [[0.33, 0.207, 0.009, -0.006], [0.8, 0.195, 0.014, -0.004], [1.3, 0.189, 0.007, 0.006], [1.6, 0.21, 0, 0], [1.7, 0.2, 0, 0], [1.79, 0.153, 0, 0]]));
    wood.push(stem("ld", BARK, TONE.stem, "stem", [[1.56, 0.164, 0, 0], [2.1, 0.14, 0.004, 0], [2.35, 0.134, 0.006, 0.004], [3.0, 0.105, -0.004, 0.006], [3.88, 0.08, 0, 0]]));
    wood.push(stem("ld-b", YOUNG, TONE.limb, "limb", [[3.76, 0.082, -0.019, 0], [4.4, 0.062, -0.012, 0.006], [4.9, 0.044, -0.022, 0.002], [5.3, 0.022, -0.036, -0.004]]));
  } else if (scale === 200) {
    wood.push(stem("root-plate", BARK, TONE.root, "root", [[0, 0.5, 0, 0], [0.08, 0.32, 0.004, 0], [0.4, 0.206, 0.01, -0.006]]));
    wood.push(stem("trunk", BARK, TONE.stem, "stem", [[0.33, 0.207, 0.009, -0.006], [1.0, 0.192, 0.012, 0], [1.62, 0.206, 0, 0], [1.78, 0.156, 0, 0]]));
    wood.push(stem("ld", BARK, TONE.stem, "stem", [[1.56, 0.162, 0, 0], [2.3, 0.136, 0.005, 0.003], [3.88, 0.08, 0, 0]]));
    wood.push(stem("ld-b", YOUNG, TONE.limb, "limb", [[3.76, 0.082, -0.019, 0], [4.6, 0.055, -0.015, 0.004], [5.3, 0.022, -0.036, -0.004]]));
  } else {
    wood.push(stem("root-plate", BARK, TONE.root, "root", [[0, 0.44, 0, 0], [0.4, 0.21, 0.008, 0]]));
    wood.push(stem("trunk", BARK, TONE.stem, "stem", [[0.33, 0.21, 0.008, 0], [1.3, 0.19, 0.004, 0], [1.72, 0.172, 0, 0]]));
    wood.push(stem("ld", BARK, TONE.stem, "stem", [[1.56, 0.17, 0, 0], [3.86, 0.082, 0, 0]]));
    wood.push(stem("ld-b", YOUNG, TONE.limb, "limb", [[3.76, 0.082, -0.019, 0], [5.3, 0.04, -0.036, 0]]));
  }
  const table = scale === 50 ? SCAFFOLD_50 : scale === 100 ? SCAFFOLD_100 : scale === 200 ? SCAFFOLD_200 : SCAFFOLD_500;
  for (const [limb, part, rows] of table) wood.push(scaffold(limb, part, rows));
  const byId = (id: string): Tube => wood.find((t) => t.id === id)!;
  for (const [id, parent, f, az, e0, e1, L, r0, r1, bend] of scale === 50 ? SIDES_50 : scale === 100 ? SIDES_100 : []) wood.push(branch(id, parent, YOUNG, TONE.limb, "side", along(byId(parent), f), az, e0, e1, L, r0, r1, bend, scale === 50 ? 6 : 4));
  if (scale === 50) for (const [parent, f, dAz] of TWIGLETS_50) {
    const p = byId(parent);
    const row = SIDES_50.find((s) => s[0] === parent)!;
    wood.push(branch(`${parent}-t`, p.bone, YOUNG, TONE.limb, "side", along(p, f), row[3] + row[9] * f + dAz, 24, 6, 0.6, 0.013, 0.006, dAz > 0 ? 8 : -8, 4));
  }
  const twigs: Tube[] = [];
  for (const key of ["w1", "e1", "w2", "e2"] as const) {
    const id = `spur-${key}`;
    const f = F[id]!;
    const L = bone(id).lengthM;
    const at = (u: number, v: number, w: number): V3 => add(add(add(f.o, mul(f.u, u)), mul(f.v, v)), mul(f.w, w));
    const flags = { slender: true, texture: true };
    if (scale === 50) {
      twigs.push({ kind: "tube", id, bone: id, material: TWIG, tone: TONE.twig, group: "twig", flags, points: [{ p: at(-0.03, 0, 0), r: 0.011 }, { p: at(0.15, 0.006, 0.004), r: 0.01 }, { p: at(0.32, 0.01, -0.004), r: 0.0085 }, { p: at(0.47, 0.004, 0.002), r: 0.007 }, { p: at(L, -0.006, 0), r: 0.005 }] });
      const stubs: [number, number, number][] = [[0.17, 0.05, 0.07], [0.31, -0.06, -0.06], [0.45, 0.07, -0.03]];
      stubs.forEach(([u, dv, dw], i) => twigs.push({ kind: "tube", id: `${id}-s${i + 1}`, bone: id, material: TWIG, tone: TONE.twig, group: "twig", flags: { texture: true }, points: [{ p: at(u, 0, 0), r: 0.0055 }, { p: at(u + 0.035, dv * 0.55, dw * 0.55), r: 0.005 }, { p: at(u + 0.06, dv, dw), r: 0.0042 }] }));
    } else twigs.push({ kind: "tube", id, bone: id, material: TWIG, tone: TONE.twig, group: "twig", flags, points: [{ p: at(-0.03, 0, 0), r: 0.011 }, { p: at(0.3, 0.008, 0), r: 0.009 }, { p: at(L, -0.004, 0), r: 0.0075 }] });
  }
  return { wood, twigs };
}

const LOBES_50: LobeRow[] = [
  ["fol-e1-m1", "fol-e1", 0, 1, 3.25, 0.95, 0.8, 0.9, 5],
  ["fol-e1-m2", "fol-e1", 30, 0.975, 3.18, 0.9, 0.76, 0.86, 8],
  ["fol-e1-m3", "fol-e1", 60, 0.99, 3.32, 0.93, 0.78, 0.88, 6],
  ["fol-e1-m4", "fol-e1", 89, 1, 3.25, 0.94, 0.78, 0.9, 5],
  ["fol-w1-m5", "fol-w1", 120, 0.97, 3.2, 0.9, 0.75, 0.86, 8],
  ["fol-w1-m6", "fol-w1", 150, 0.985, 3.3, 0.92, 0.78, 0.88, 7],
  ["fol-w1-m7", "fol-w1", 180, 1, 3.25, 0.95, 0.8, 0.9, 5],
  ["fol-w1-m8", "fol-w1", 210, 0.97, 3.16, 0.9, 0.76, 0.86, 9],
  ["fol-w1-m9", "fol-w1", 240, 0.99, 3.3, 0.92, 0.77, 0.87, 6],
  ["fol-w1-m10", "fol-w1", 269, 1, 3.25, 0.94, 0.78, 0.9, 5],
  ["fol-e1-m11", "fol-e1", 300, 0.975, 3.2, 0.9, 0.76, 0.86, 8],
  ["fol-e1-m12", "fol-e1", 330, 0.985, 3.3, 0.92, 0.78, 0.88, 7],
  ["fol-e1-k1", "fol-e1", 15, 0.99, 2.8, 0.55, 0.46, 0.6, 20],
  ["fol-e1-k2", "fol-e1", 45, 0.98, 2.74, 0.52, 0.44, 0.58, 22],
  ["fol-e1-k3", "fol-e1", 75, 0.99, 2.84, 0.56, 0.47, 0.6, 18],
  ["fol-w1-k4", "fol-w1", 105, 0.985, 2.78, 0.54, 0.45, 0.58, 20],
  ["fol-w1-k5", "fol-w1", 135, 0.98, 2.72, 0.52, 0.44, 0.57, 22],
  ["fol-w1-k6", "fol-w1", 165, 0.99, 2.82, 0.56, 0.46, 0.6, 19],
  ["fol-w1-k7", "fol-w1", 195, 0.985, 2.76, 0.54, 0.45, 0.58, 21],
  ["fol-w1-k8", "fol-w1", 225, 0.98, 2.74, 0.52, 0.44, 0.57, 22],
  ["fol-w1-k9", "fol-w1", 255, 0.99, 2.83, 0.55, 0.46, 0.6, 18],
  ["fol-e1-k10", "fol-e1", 285, 0.985, 2.77, 0.54, 0.45, 0.58, 20],
  ["fol-e1-k11", "fol-e1", 315, 0.98, 2.73, 0.52, 0.44, 0.57, 22],
  ["fol-e1-k12", "fol-e1", 345, 0.99, 2.82, 0.56, 0.46, 0.6, 19],
  ["fol-e1-s1", "fol-e1", 42, 0.995, 3.74, 0.5, 0.42, 0.55, 12],
  ["fol-e1-s2", "fol-e1", 74, 0.99, 3.8, 0.48, 0.4, 0.54, 12],
  ["fol-w1-s3", "fol-w1", 106, 0.995, 3.72, 0.5, 0.42, 0.56, 13],
  ["fol-w1-s4", "fol-w1", 148, 0.99, 3.78, 0.5, 0.41, 0.55, 12],
  ["fol-w1-s5", "fol-w1", 214, 0.995, 3.7, 0.52, 0.43, 0.56, 13],
  ["fol-w1-s6", "fol-w1", 256, 0.99, 3.79, 0.49, 0.41, 0.54, 12],
  ["fol-e1-s7", "fol-e1", 286, 0.995, 3.73, 0.5, 0.42, 0.55, 12],
  ["fol-e1-s8", "fol-e1", 322, 0.99, 3.8, 0.5, 0.41, 0.55, 13],
  ["fol-e1-h1", "fol-e1", 18, 0.99, 4.12, 0.86, 0.68, 0.9, 6],
  ["fol-e1-h2", "fol-e1", 54, 0.97, 4.2, 0.84, 0.66, 0.88, 7],
  ["fol-e1-h3", "fol-e1", 90, 0.985, 4.1, 0.86, 0.68, 0.9, 6],
  ["fol-w1-h4", "fol-w1", 126, 0.97, 4.22, 0.83, 0.66, 0.88, 7],
  ["fol-w1-h5", "fol-w1", 162, 0.99, 4.14, 0.86, 0.68, 0.9, 6],
  ["fol-w1-h6", "fol-w1", 198, 0.975, 4.2, 0.84, 0.67, 0.88, 7],
  ["fol-w1-h7", "fol-w1", 234, 0.99, 4.1, 0.86, 0.68, 0.9, 6],
  ["fol-w1-h8", "fol-w1", 270, 0.97, 4.2, 0.84, 0.66, 0.88, 7],
  ["fol-e1-h9", "fol-e1", 306, 0.99, 4.13, 0.86, 0.68, 0.9, 6],
  ["fol-e1-h10", "fol-e1", 342, 0.975, 4.22, 0.84, 0.66, 0.88, 7],
  ["fol-e2-u1", "fol-e2", 30, 0.99, 4.92, 0.8, 0.64, 0.88, 5],
  ["fol-w2-u2", "fol-w2", 70, 0.975, 5.0, 0.78, 0.62, 0.86, 6],
  ["fol-w2-u3", "fol-w2", 110, 0.99, 4.9, 0.8, 0.64, 0.88, 5],
  ["fol-w2-u4", "fol-w2", 150, 0.98, 5.0, 0.8, 0.63, 0.86, 5],
  ["fol-w2-u5", "fol-w2", 190, 0.99, 4.94, 0.8, 0.64, 0.88, 5],
  ["fol-w2-u6", "fol-w2", 230, 0.975, 5.0, 0.78, 0.62, 0.86, 6],
  ["fol-e2-u7", "fol-e2", 270, 0.99, 4.9, 0.8, 0.64, 0.88, 5],
  ["fol-e2-u8", "fol-e2", 310, 0.98, 5.02, 0.8, 0.63, 0.86, 5],
  ["fol-e2-u9", "fol-e2", 350, 0.99, 4.95, 0.8, 0.64, 0.88, 5],
  ["fol-e2-v1", "fol-e2", 8, 0.985, 5.5, 0.7, 0.54, 0.82, 7],
  ["fol-e2-v2", "fol-e2", 52, 0.97, 5.55, 0.68, 0.53, 0.8, 8],
  ["fol-w2-v3", "fol-w2", 98, 0.985, 5.48, 0.7, 0.54, 0.82, 7],
  ["fol-w2-v4", "fol-w2", 143, 1, 5.46, 0.74, 0.56, 0.84, 6],
  ["fol-w2-v5", "fol-w2", 188, 0.985, 5.52, 0.7, 0.54, 0.82, 7],
  ["fol-w2-v6", "fol-w2", 232, 0.97, 5.55, 0.68, 0.53, 0.8, 8],
  ["fol-e2-v7", "fol-e2", 278, 0.985, 5.5, 0.7, 0.54, 0.82, 7],
  ["fol-e2-v8", "fol-e2", 323, 1, 5.52, 0.74, 0.56, 0.84, 6],
  ["fol-e2-s1", "fol-e2", 34, 0.99, 4.56, 0.48, 0.4, 0.52, 10],
  ["fol-w2-s2", "fol-w2", 108, 0.985, 4.5, 0.5, 0.4, 0.53, 11],
  ["fol-w2-s3", "fol-w2", 176, 0.99, 4.58, 0.48, 0.4, 0.52, 10],
  ["fol-w2-s4", "fol-w2", 216, 0.985, 4.52, 0.5, 0.41, 0.53, 10],
  ["fol-e2-s5", "fol-e2", 290, 0.99, 4.55, 0.48, 0.4, 0.52, 11],
  ["fol-e2-s6", "fol-e2", 326, 0.985, 4.5, 0.5, 0.4, 0.53, 10],
  ["fol-top-l1", "fol-top", 0, FIX(1.15), 5.77, 1.15, 0.58, 1.2, 0],
  ["fol-top-l2", "fol-top", 20, 0.9, 5.86, 0.72, 0.44, 0.88, 5],
  ["fol-top-l3", "fol-top", 80, 0.88, 5.9, 0.7, 0.42, 0.86, 5],
  ["fol-top-l4", "fol-top", 140, 0.9, 5.84, 0.72, 0.45, 0.88, 5],
  ["fol-top-l5", "fol-top", 200, 0.88, 5.9, 0.7, 0.43, 0.86, 5],
  ["fol-top-l6", "fol-top", 260, 0.9, 5.85, 0.72, 0.44, 0.88, 5],
  ["fol-top-l7", "fol-top", 320, 0.88, 5.9, 0.7, 0.42, 0.86, 5],
  ["fol-core-up-l1", "fol-core-up", 20, FIX(1.85), 4.5, 1.0, 0.72, 1.0, 0],
  ["fol-core-up-l2", "fol-core-up", 92, FIX(1.8), 4.58, 0.98, 0.7, 1.0, 0],
  ["fol-core-up-l3", "fol-core-up", 164, FIX(1.85), 4.48, 1.0, 0.72, 1.0, 0],
  ["fol-core-up-l4", "fol-core-up", 236, FIX(1.8), 4.56, 0.98, 0.7, 1.0, 0],
  ["fol-core-up-l5", "fol-core-up", 308, FIX(1.85), 4.5, 1.0, 0.72, 1.0, 0],
  ["fol-core-low-l1", "fol-core-low", 10, FIX(2.1), 3.42, 1.05, 0.7, 1.05, 0],
  ["fol-core-low-l2", "fol-core-low", 70, FIX(2.05), 3.5, 1.03, 0.7, 1.05, 0],
  ["fol-core-low-l3", "fol-core-low", 130, FIX(2.1), 3.4, 1.05, 0.7, 1.05, 0],
  ["fol-core-low-l4", "fol-core-low", 190, FIX(2.05), 3.48, 1.03, 0.7, 1.05, 0],
  ["fol-core-low-l5", "fol-core-low", 250, FIX(2.1), 3.42, 1.05, 0.7, 1.05, 0],
  ["fol-core-low-l6", "fol-core-low", 310, FIX(2.05), 3.5, 1.03, 0.7, 1.05, 0],
];
const LOBES_100: LobeRow[] = [
  ["fol-e1-m1", "fol-e1", 0, 1, 3.25, 1.0, 0.96, 1.02, 6],
  ["fol-e1-m2", "fol-e1", 36, 0.975, 3.18, 0.97, 0.92, 1.0, 8],
  ["fol-e1-m3", "fol-e1", 89, 1, 3.25, 1.0, 0.95, 1.05, 6],
  ["fol-w1-m4", "fol-w1", 144, 0.975, 3.2, 0.97, 0.92, 1.0, 8],
  ["fol-w1-m5", "fol-w1", 180, 1, 3.25, 1.0, 0.96, 1.02, 6],
  ["fol-w1-m6", "fol-w1", 216, 0.975, 3.18, 0.97, 0.92, 1.0, 8],
  ["fol-w1-m7", "fol-w1", 269, 1, 3.25, 1.0, 0.95, 1.05, 6],
  ["fol-e1-m8", "fol-e1", 324, 0.975, 3.2, 0.97, 0.92, 1.0, 8],
  ["fol-e1-m9", "fol-e1", 62, 0.985, 3.3, 0.95, 0.9, 0.98, 7],
  ["fol-w1-m10", "fol-w1", 118, 0.985, 3.3, 0.95, 0.9, 0.98, 7],
  ["fol-w1-m11", "fol-w1", 242, 0.985, 3.3, 0.95, 0.9, 0.98, 7],
  ["fol-e1-m12", "fol-e1", 298, 0.985, 3.3, 0.95, 0.9, 0.98, 7],
  ["fol-e1-k1", "fol-e1", 15, 0.99, 2.8, 0.57, 0.47, 0.64, 20],
  ["fol-e1-k2", "fol-e1", 45, 0.98, 2.75, 0.57, 0.47, 0.64, 20],
  ["fol-e1-k3", "fol-e1", 75, 0.99, 2.83, 0.57, 0.47, 0.64, 20],
  ["fol-w1-k4", "fol-w1", 105, 0.985, 2.78, 0.57, 0.47, 0.64, 20],
  ["fol-w1-k5", "fol-w1", 135, 0.98, 2.73, 0.57, 0.47, 0.64, 20],
  ["fol-w1-k6", "fol-w1", 165, 0.99, 2.82, 0.57, 0.47, 0.64, 20],
  ["fol-w1-k7", "fol-w1", 195, 0.985, 2.77, 0.57, 0.47, 0.64, 20],
  ["fol-w1-k8", "fol-w1", 225, 0.98, 2.75, 0.57, 0.47, 0.64, 20],
  ["fol-w1-k9", "fol-w1", 255, 0.99, 2.83, 0.57, 0.47, 0.64, 20],
  ["fol-e1-k10", "fol-e1", 285, 0.985, 2.78, 0.57, 0.47, 0.64, 20],
  ["fol-e1-k11", "fol-e1", 315, 0.98, 2.74, 0.57, 0.47, 0.64, 20],
  ["fol-e1-k12", "fol-e1", 345, 0.99, 2.82, 0.57, 0.47, 0.64, 20],
  ["fol-e1-h1", "fol-e1", 22, 0.985, 4.14, 0.92, 0.85, 1.1, 6],
  ["fol-e1-h2", "fol-e1", 67, 0.975, 4.2, 0.9, 0.83, 1.08, 6],
  ["fol-w1-h3", "fol-w1", 112, 0.985, 4.12, 0.92, 0.85, 1.1, 6],
  ["fol-w1-h4", "fol-w1", 157, 0.975, 4.2, 0.9, 0.83, 1.08, 6],
  ["fol-w1-h5", "fol-w1", 202, 0.985, 4.14, 0.92, 0.85, 1.1, 6],
  ["fol-w1-h6", "fol-w1", 247, 0.975, 4.2, 0.9, 0.83, 1.08, 6],
  ["fol-e1-h7", "fol-e1", 292, 0.985, 4.12, 0.92, 0.85, 1.1, 6],
  ["fol-e1-h8", "fol-e1", 337, 0.975, 4.2, 0.9, 0.83, 1.08, 6],
  ["fol-e2-u1", "fol-e2", 14, 0.99, 5.02, 0.9, 0.84, 1.05, 5],
  ["fol-w2-u2", "fol-w2", 66, 0.98, 5.06, 0.88, 0.82, 1.02, 5],
  ["fol-w2-u3", "fol-w2", 143, 0.99, 5.0, 0.9, 0.84, 1.05, 5],
  ["fol-w2-u4", "fol-w2", 194, 0.98, 5.06, 0.88, 0.82, 1.02, 5],
  ["fol-e2-u5", "fol-e2", 246, 0.99, 5.02, 0.9, 0.84, 1.05, 5],
  ["fol-e2-u6", "fol-e2", 323, 0.98, 5.04, 0.9, 0.84, 1.05, 5],
  ["fol-w2-u7", "fol-w2", 104, 0.985, 5.04, 0.88, 0.82, 1.0, 5],
  ["fol-e2-u8", "fol-e2", 284, 0.985, 5.04, 0.88, 0.82, 1.0, 5],
  ["fol-e2-s1", "fol-e2", 34, 0.99, 4.58, 0.52, 0.44, 0.6, 10],
  ["fol-w2-s2", "fol-w2", 108, 0.985, 4.54, 0.52, 0.44, 0.6, 10],
  ["fol-w2-s3", "fol-w2", 176, 0.99, 4.6, 0.52, 0.44, 0.6, 10],
  ["fol-w2-s4", "fol-w2", 216, 0.985, 4.55, 0.52, 0.44, 0.6, 10],
  ["fol-e2-s5", "fol-e2", 290, 0.99, 4.58, 0.52, 0.44, 0.6, 10],
  ["fol-e2-s6", "fol-e2", 326, 0.985, 4.54, 0.52, 0.44, 0.6, 10],
  ["fol-top-l1", "fol-top", 0, FIX(1.3), 5.72, 1.3, 0.63, 1.35, 0],
  ["fol-top-l2", "fol-top", 20, 0.96, 5.8, 0.8, 0.48, 0.95, 5],
  ["fol-top-l3", "fol-top", 80, 0.95, 5.82, 0.8, 0.47, 0.95, 5],
  ["fol-top-l4", "fol-top", 140, 0.96, 5.8, 0.8, 0.48, 0.95, 5],
  ["fol-top-l5", "fol-top", 200, 0.95, 5.82, 0.8, 0.47, 0.95, 5],
  ["fol-top-l6", "fol-top", 260, 0.96, 5.8, 0.8, 0.48, 0.95, 5],
  ["fol-top-l7", "fol-top", 320, 0.95, 5.82, 0.8, 0.47, 0.95, 5],
  ["fol-core-up-l1", "fol-core-up", 30, FIX(1.85), 4.52, 1.05, 0.74, 1.2, 0],
  ["fol-core-up-l2", "fol-core-up", 150, FIX(1.85), 4.5, 1.05, 0.74, 1.2, 0],
  ["fol-core-up-l3", "fol-core-up", 270, FIX(1.85), 4.54, 1.05, 0.74, 1.2, 0],
  ["fol-core-low-l1", "fol-core-low", 90, FIX(2.1), 3.44, 1.1, 0.72, 1.5, 0],
  ["fol-core-low-l2", "fol-core-low", 210, FIX(2.1), 3.46, 1.1, 0.72, 1.5, 0],
  ["fol-core-low-l3", "fol-core-low", 330, FIX(2.1), 3.44, 1.1, 0.72, 1.5, 0],
];
const LOBES_200: LobeRow[] = [
  ["fol-e1-m1", "fol-e1", 0, 1, 3.25, 1.15, 0.97, 1.4, 5],
  ["fol-e1-m2", "fol-e1", 45, 0.98, 3.25, 1.12, 0.95, 1.4, 5],
  ["fol-e1-m3", "fol-e1", 89, 1, 3.25, 1.15, 0.97, 1.4, 5],
  ["fol-w1-m4", "fol-w1", 135, 0.98, 3.25, 1.12, 0.95, 1.4, 5],
  ["fol-w1-m5", "fol-w1", 180, 1, 3.25, 1.15, 0.97, 1.4, 5],
  ["fol-w1-m6", "fol-w1", 225, 0.98, 3.25, 1.12, 0.95, 1.4, 5],
  ["fol-w1-m7", "fol-w1", 269, 1, 3.25, 1.15, 0.97, 1.4, 5],
  ["fol-e1-m8", "fol-e1", 315, 0.98, 3.25, 1.12, 0.95, 1.4, 5],
  ["fol-e1-h1", "fol-e1", 30, 0.99, 4.2, 1.1, 0.88, 1.9, 4],
  ["fol-e1-h2", "fol-e1", 90, 0.99, 4.2, 1.1, 0.88, 1.9, 4],
  ["fol-w1-h3", "fol-w1", 150, 0.99, 4.2, 1.1, 0.88, 1.9, 4],
  ["fol-w1-h4", "fol-w1", 210, 0.99, 4.2, 1.1, 0.88, 1.9, 4],
  ["fol-w1-h5", "fol-w1", 270, 0.99, 4.2, 1.1, 0.88, 1.9, 4],
  ["fol-e1-h6", "fol-e1", 330, 0.99, 4.2, 1.1, 0.88, 1.9, 4],
  ["fol-e2-u1", "fol-e2", 53, 1, 5.05, 1.1, 0.86, 1.9, 4],
  ["fol-w2-u2", "fol-w2", 143, 1, 5.05, 1.1, 0.86, 1.9, 4],
  ["fol-w2-u3", "fol-w2", 233, 1, 5.05, 1.1, 0.86, 1.9, 4],
  ["fol-e2-u4", "fol-e2", 323, 1, 5.05, 1.1, 0.86, 1.9, 4],
  ["fol-top-l1", "fol-top", 0, FIX(1.75), 5.62, 1.75, 0.73, 1.8, 0],
  ["fol-core-up-l1", "fol-core-up", 0, FIX(1.7), 4.5, 1.7, 0.8, 1.7, 0],
  ["fol-core-low-l1", "fol-core-low", 0, FIX(2.0), 3.5, 2.0, 0.78, 2.0, 0],
];
const LOBES_500: LobeRow[] = [
  ["crown-e", "ld", 0, 1, 3.38, 1.5, 1.13, 2.2, 0],
  ["crown-n", "ld", 89, 1, 3.38, 1.45, 1.13, 2.3, 0],
  ["crown-w", "ld", 180, 1, 3.38, 1.5, 1.13, 2.2, 0],
  ["crown-s", "ld", 269, 1, 3.38, 1.45, 1.13, 2.3, 0],
  ["crown-ne", "ld", 53, 0.99, 4.62, 1.35, 1.2, 2.1, 0],
  ["crown-nw", "ld", 143, 1, 4.62, 1.35, 1.2, 2.1, 0],
  ["crown-sw", "ld", 233, 0.99, 4.62, 1.35, 1.2, 2.1, 0],
  ["crown-se", "ld", 323, 1, 4.62, 1.35, 1.2, 2.1, 0],
  ["crown-top", "ld", 0, FIX(1.7), 5.5, 1.7, 0.85, 1.7, 0],
  ["crown-core", "ld", 0, FIX(1.5), 3.9, 1.5, 1.2, 1.5, 0],
];

const FRUIT_50: FruitRow[] = [
  ["elevation", "fol-w1-m10", 262, 8], ["elevation", "fol-w1-m9", 255, -20], ["elevation", "fol-w1-k9", 262, -5], ["elevation", "fol-w1-h8", 265, 10], ["elevation", "fol-w1-h7", 248, -12],
  ["elevation", "fol-e1-m11", 285, 10], ["elevation", "fol-e1-k10", 278, -8], ["elevation", "fol-e1-m12", 300, -22], ["elevation", "fol-e1-h9", 292, 6],
  ["elevation", "fol-w2-u6", 250, 8],
  ["elevation", "fol-e2-u7", 272, 12], ["elevation", "fol-e2-u7", 290, -18], ["elevation", "fol-e2-u8", 295, 6], ["elevation", "fol-e2-s5", 280, 0],
  ["elevation", "fol-top-l6", 268, 10], ["elevation", "fol-top-l6", 285, -15], ["elevation", "fol-top-l5", 250, 0], ["elevation", "fol-top-l7", 295, 5],
  ["front", "fol-e1-m1", 8, 10], ["front", "fol-e1-m2", 18, -20], ["front", "fol-e1-h1", 10, 5], ["front", "fol-e1-m12", 345, -8], ["front", "fol-e1-h10", 350, 12],
  ["front", "fol-e2-u1", 12, 12], ["front", "fol-e2-s1", 20, -5], ["front", "fol-e2-u8", 340, 0],
  ["front", "fol-top-l2", 20, 8], ["front", "fol-top-l7", 348, -10],
  ["plan", "fol-w1-h5", 170, 50], ["plan", "fol-w1-h7", 230, 52], ["plan", "fol-e1-h1", 15, 50], ["plan", "fol-e1-h9", 310, 52],
  ["plan", "fol-w2-v4", 150, 60], ["plan", "fol-w2-v5", 185, 62], ["plan", "fol-e2-v8", 330, 60], ["plan", "fol-e2-v1", 10, 62], ["plan", "fol-top-l3", 85, 65],
  ["none", "fol-e1-m4", 95, 5], ["none", "fol-w1-m5", 110, -15], ["none", "fol-e1-h3", 85, 10], ["none", "fol-w1-h4", 120, -10], ["none", "fol-e1-m3", 60, -12], ["none", "fol-e1-h2", 58, 8],
  ["none", "fol-w2-u3", 100, 0], ["none", "fol-w2-u4", 135, 10], ["none", "fol-top-l4", 150, 5], ["none", "fol-top-l3", 85, -5],
  ["none", "fol-w1-m7", 175, 8], ["none", "fol-w1-m6", 160, -18], ["none", "fol-w1-h5", 165, 5], ["none", "fol-w1-h6", 190, -8], ["none", "fol-w2-u5", 180, 5], ["none", "fol-w1-m8", 200, 0],
];
const FRUIT_100: FruitRow[] = [
  ["elevation", "fol-w1-m7", 264, 6], ["elevation", "fol-w1-m6", 250, -15], ["elevation", "fol-e1-m8", 290, 10], ["elevation", "fol-e1-m8", 305, -22], ["elevation", "fol-w1-h6", 262, 8],
  ["elevation", "fol-e2-u5", 250, 8], ["elevation", "fol-e2-u8", 280, 12], ["elevation", "fol-e2-u6", 300, -16], ["elevation", "fol-top-l1", 270, 8],
  ["front", "fol-e1-m1", 8, 8], ["front", "fol-e1-h1", 15, 5], ["front", "fol-e2-u1", 10, 10], ["front", "fol-e1-m8", 340, -12], ["front", "fol-e1-h8", 345, 6],
  ["plan", "fol-w1-h4", 155, 50], ["plan", "fol-e1-h8", 335, 50], ["plan", "fol-w2-u3", 150, 62], ["plan", "fol-e2-u1", 20, 62], ["plan", "fol-w2-u4", 200, 60], ["plan", "fol-e2-u6", 320, 60], ["plan", "fol-top-l1", 120, 75],
  ["none", "fol-e1-m3", 92, 5], ["none", "fol-w1-m4", 130, -12], ["none", "fol-e1-h2", 80, 8], ["none", "fol-w2-u7", 95, 0], ["none", "fol-w1-m5", 178, 6], ["none", "fol-w1-h4", 200, -6],
];
const FRUIT_200: FruitRow[] = [
  ["elevation", "fol-w1-m7", 255, 6], ["elevation", "fol-e1-m8", 295, -8], ["elevation", "fol-e2-u4", 290, 8], ["elevation", "fol-w1-m6", 235, 0], ["elevation", "fol-top-l1", 270, 10],
  ["front", "fol-e1-m1", 5, 5], ["front", "fol-e2-u1", 20, 8], ["none", "fol-e1-m3", 80, 4], ["none", "fol-w2-u2", 110, 6], ["none", "fol-w1-m4", 150, -5],
];
const FRUIT_500: FruitRow[] = [["elevation", "crown-s", 262, 5], ["elevation", "crown-se", 280, 10], ["elevation", "crown-e", 300, 0]];

const OUTLETS: Record<number, Outlet[]> = {
  50: [
    ["transpiration-sun", "tp-sun-1", "fol-w1-h5", 162, 38], ["transpiration-sun", "tp-sun-2", "fol-e1-h1", 18, 38], ["transpiration-sun", "tp-sun-3", "fol-w2-v4", 143, 62], ["transpiration-sun", "tp-sun-4", "fol-e2-v8", 323, 62], ["transpiration-sun", "tp-sun-5", "fol-top-l1", 0, 90], ["transpiration-sun", "tp-sun-6", "fol-w1-h7", 234, 38], ["transpiration-sun", "tp-sun-7", "fol-e1-h9", 306, 38],
    ["transpiration-shade", "tp-shade-1", "fol-core-low-l1", 10, 90], ["transpiration-shade", "tp-shade-2", "fol-core-low-l3", 130, 90], ["transpiration-shade", "tp-shade-3", "fol-core-low-l5", 250, 90], ["transpiration-shade", "tp-shade-4", "fol-core-up-l2", 92, 90], ["transpiration-shade", "tp-shade-5", "fol-core-up-l4", 236, 90],
  ],
  100: [
    ["transpiration-sun", null, "fol-w1-h4", 157, 36], ["transpiration-sun", null, "fol-e1-h8", 337, 36], ["transpiration-sun", null, "fol-w2-u3", 143, 62], ["transpiration-sun", null, "fol-e2-u6", 323, 62], ["transpiration-sun", null, "fol-top-l1", 0, 90],
    ["transpiration-shade", null, "fol-core-low-l1", 90, 90], ["transpiration-shade", null, "fol-core-low-l2", 210, 90], ["transpiration-shade", null, "fol-core-up-l1", 30, 90],
  ],
  200: [["transpiration-sun", null, "fol-w2-u2", 143, 50], ["transpiration-sun", null, "fol-e2-u4", 323, 50], ["transpiration-sun", null, "fol-top-l1", 0, 90], ["transpiration-shade", null, "fol-core-low-l1", 0, 90], ["transpiration-shade", null, "fol-core-up-l1", 0, 90]],
  500: [["transpiration-sun", null, "crown-top", 0, 90], ["transpiration-shade", null, "crown-core", 0, 90]],
};

const DECISIONS: Record<number, { en: string; de: string }[]> = {
  50: [
    { en: "Body at 1:50: the root flare with five buttress roots, the clear stem with its swelling below the fork and a knot, the leader in two parts, four scaffold limbs in two tiers (swept lofts with branch collars, tapered and bowed, running out of the rig plane at 189, 9, 143 and 323 degrees in plan), fourteen second-order fruit branches (the four that fill the quarters between the scaffolds are the stoutest) with eight forks, four upright fruiting shoots with their spurs, and one broad rounded crown of eighty-three leaf lobes: a drooping skirt of twelve, the widest ring of twelve at 3.25 m, ten shoulder lobes, nine and eight lobes on the two upper rings, seven at the top, fourteen small lobes between the rings and eleven shade lobes of the inner crown; fifty-three apples of 8 cm hang all round the crown.", de: "Körper bei 1:50: Wurzelanlauf mit fünf Wurzelanläufen, Stamm mit Verdickung unter der Verzweigung und einem Astknoten, Stammverlängerung in zwei Teilen, vier Leitäste in zwei Etagen (geführte Lofts mit Astringen, verjüngt und gebogen, im Grundriss unter 189, 9, 143 und 323 Grad aus der Rig-Ebene laufend), vierzehn Fruchtäste zweiter Ordnung (die vier, die die Viertel zwischen den Leitästen füllen, sind die kräftigsten) mit acht Gabelungen, vier aufrechte Fruchttriebe mit ihren Fruchtspießen und eine breite, runde Krone aus dreiundachtzig Laublappen: ein hängender Saum aus zwölf, der breiteste Ring aus zwölf auf 3,25 m, zehn Schulterlappen, neun und acht Lappen auf den beiden oberen Ringen, sieben am Gipfel, vierzehn kleine Lappen zwischen den Ringen und elf Schattenlappen der Innenkrone; dreiundfünfzig Äpfel von 8 cm hängen rund um die Krone." },
    { en: "Thermal at 1:50: stem bark is the root flare, buttresses, stem and lower leader; limb bark the upper leader, scaffolds, fruit branches and forks; the fruiting shoots are the twig region; sun leaves are the outer lobes, shade leaves the inner lobes, the apples the fruit region, all by reference to the body. The twelve transpiration outlets are lens patches lying in the free outer face of the lobe that carries each emitter anchor (in the top of the shade lobes), 2 cm proud.", de: "Thermik bei 1:50: Stammrinde sind Wurzelanlauf, Wurzelanläufe, Stamm und untere Stammverlängerung; Astrinde obere Stammverlängerung, Leitäste, Fruchtäste und Gabelungen; die Fruchttriebe sind die Zweigregion; Sonnenblätter sind die äußeren Lappen, Schattenblätter die inneren, die Äpfel die Fruchtregion, alle als Verweis auf den Körper. Die zwölf Transpirationsauslässe sind Linsenflächen in der freien Außenseite des Lappens, der den jeweiligen Emitteranker trägt (bei den Schattenlappen in der Oberseite), 2 cm vorstehend." },
    { en: "Structure at 1:50: the wood under the bark of every load-bearing bone along the real limb axes (root flare, stem, leader, eight scaffold parts, four shoots), joints at the rig joints, the round fixed root plate under the ground, and the masses of stem, leader and the seven foliage masses at their centres of mass; no leaves, no bark.", de: "Tragwerk bei 1:50: das Holz unter der Rinde jedes tragenden Knochens entlang der wirklichen Astachsen (Wurzelanlauf, Stamm, Stammverlängerung, acht Leitastteile, vier Triebe), Gelenke an den Rig-Gelenken, der runde eingespannte Wurzelteller unter dem Boden und die Massen von Stamm, Stammverlängerung und der sieben Laubmassen in ihren Schwerpunkten; kein Laub, keine Rinde." },
  ],
  100: [
    { en: "Body at 1:100: the buttress roots merge into one flared root loft, every limb keeps its taper with four or five sections, eight fruit branches remain and the forks and spurs are dropped, the fruiting shoots are single twigs; the crown is redrawn with fifty-nine broader lobes (skirt and widest ring of twelve each, eight shoulder lobes, eight on the upper ring, six small ones between, seven at the top, six shade lobes) and twenty-seven apples.", de: "Körper bei 1:100: Die Wurzelanläufe gehen in einem ausgestellten Wurzelloft auf, jeder Ast behält seine Verjüngung mit vier bis fünf Schnitten, acht Fruchtäste bleiben, Gabelungen und Fruchtspieße entfallen, die Fruchttriebe sind einzelne Zweige; die Krone ist mit neunundfünfzig breiteren Lappen neu gezeichnet (Saum und breitester Ring aus je zwölf, acht Schulterlappen, acht auf dem oberen Ring, sechs kleine dazwischen, sieben am Gipfel, sechs Schattenlappen) und siebenundzwanzig Äpfeln." },
    { en: "Thermal at 1:100: the same six regions by reference to this scale's body; eight transpiration outlets as lens patches in the free faces of the lobes.", de: "Thermik bei 1:100: dieselben sechs Regionen als Verweis auf den Körper dieses Maßstabs; acht Transpirationsauslässe als Linsenflächen in den freien Seiten der Lappen." },
    { en: "Structure at 1:100: the wood of the sixteen load-bearing bones with simplified taper, joints at the rig joints, the round root plate and the masses.", de: "Tragwerk bei 1:100: das Holz der sechzehn tragenden Knochen mit vereinfachter Verjüngung, Gelenke an den Rig-Gelenken, der runde Wurzelteller und die Massen." },
  ],
  200: [
    { en: "Body at 1:200: root flare, stem, leader and the eight scaffold parts with three sections each, no fruit branches and no shoots; the crown is twenty-one masses (a widest ring of eight, six shoulder masses, four upper masses over the ends of the upper scaffolds, one top mass, two shade cores) and ten apples.", de: "Körper bei 1:200: Wurzelanlauf, Stamm, Stammverlängerung und die acht Leitastteile mit je drei Schnitten, keine Fruchtäste und keine Triebe; die Krone besteht aus einundzwanzig Massen (ein breitester Ring aus acht, sechs Schultermassen, vier obere Massen über den Enden der oberen Leitäste, eine Gipfelmasse, zwei Schattenkerne) und zehn Äpfeln." },
    { en: "Thermal at 1:200: bark, leaves and fruit by reference to the body; the fruiting shoots stay as slender twigs in the thermal list only; five outlets.", de: "Thermik bei 1:200: Rinde, Laub und Früchte als Verweis auf den Körper; die Fruchttriebe bleiben als schlanke Zweige nur in der Thermikliste; fünf Auslässe." },
    { en: "Structure at 1:200: the wood of the sixteen load-bearing bones, joints where they are at least one smallest feature thick, the round root plate and the masses.", de: "Tragwerk bei 1:200: das Holz der sechzehn tragenden Knochen, Gelenke dort, wo sie mindestens ein kleinstes Merkmal dick sind, der runde Wurzelteller und die Massen." },
  ],
  500: [
    { en: "Body at 1:500: root flare, stem and lower leader as three plain lofts and the crown as ten masses bound to the leader as the drawing binds it (four low side masses, four shoulder masses, the top, the shade core); the scaffold limbs are merged into the crown.", de: "Körper bei 1:500: Wurzelanlauf, Stamm und untere Stammverlängerung als drei einfache Lofts und die Krone als zehn Massen, an die Stammverlängerung gebunden wie in der Zeichnung (vier tiefe Seitenmassen, vier Schultermassen, der Gipfel, der Schattenkern); die Leitäste gehen in der Krone auf." },
    { en: "Thermal at 1:500: stem bark and leaves by reference; the upper leader, the scaffold limbs, the fruiting shoots and three apples are thermal solids of their own inside the crown; two outlets.", de: "Thermik bei 1:500: Stammrinde und Laub als Verweis; obere Stammverlängerung, Leitäste, Fruchttriebe und drei Äpfel sind eigene Thermikkörper in der Krone; zwei Auslässe." },
    { en: "Structure at 1:500: the sixteen load-bearing bones as members of their declared section, the joints of stem and leader, the round root plate and the three masses of the stem; this list is already minimal.", de: "Tragwerk bei 1:500: die sechzehn tragenden Knochen als Stäbe ihres angegebenen Querschnitts, die Gelenke von Stamm und Stammverlängerung, der runde Wurzelteller und die drei Massen des Stamms; diese Liste ist bereits minimal." },
  ],
};

function model(scale: number): Model {
  const { wood, twigs } = woody(scale);
  const table = scale === 50 ? LOBES_50 : scale === 100 ? LOBES_100 : scale === 200 ? LOBES_200 : LOBES_500;
  const all = lobes(table);
  const fruit = fruits(scale === 50 ? FRUIT_50 : scale === 100 ? FRUIT_100 : scale === 200 ? FRUIT_200 : FRUIT_500, all);
  const limbs = (t: Tube): boolean => t.group === "limb" && t.id !== "ld-b" ? true : false;
  const hiddenWood = scale === 500 ? wood.filter((t) => limbs(t) || t.id === "ld-b") : [];
  const shown = scale === 500 ? wood.filter((t) => !hiddenWood.includes(t)) : wood;
  const k = F.trunk!;
  const knot = scale === 50 ? [{ id: "knot", kind: "ellipsoid", bone: "trunk", material: BARK, role: "detail", tone: 0.3, radii: [0.062, 0.055, 0.03], transform: { translation: toLocal(k, [0.013, 0.9, -0.184]).map(r5) } }] : [];
  return { scale, wood: shown, twigs: scale <= 100 ? twigs : [], hiddenWood, hiddenTwigs: scale <= 100 ? [] : twigs, knot, lobes: all, fruit: scale === 500 ? [] : fruit, hiddenFruit: scale === 500 ? fruit : [], outlets: OUTLETS[scale]!, decisions: DECISIONS[scale]! };
}

const AREAS: Record<string, number> = { "trunk-bark": 4.8, "limb-bark": 9.1 };
for (const region of asset.thermal.regions) if (AREAS[region.id] !== undefined) region.areaM2 = AREAS[region.id];

const DRAWN: Record<number, Record<string, { en: string; de: string }>> = {
  50: {
    silhouette: { en: "At 1:50 (1 m is 20 mm) the three drawings are the true views of the 1:50 body: root flare with its buttress roots, scaly stem with cracks and a knot, the leader and the scaffold limbs running into the crown, the fruit branches where they show under the crown, the fruiting shoots, and the broad rounded crown as seven foliage masses whose outlines follow their lobes, with leaf-cluster arcs, ovate leaf marks and every apple that is visible from the viewpoint at its real size (1.6 mm): twenty-eight in the side elevation, twenty-six in the front elevation, thirty-six in the plan. Plan: the four scaffolds as branch hints over the crown, the shoots at the lobe tips and the stem as a dot.", de: "Bei 1:50 (1 m sind 20 mm) sind die drei Zeichnungen die wahren Ansichten des 1:50-Körpers: Wurzelanlauf mit seinen Wurzelanläufen, schuppiger Stamm mit Rissen und einem Astknoten, Stammverlängerung und Leitäste, die in die Krone laufen, die Fruchtäste, wo sie unter der Krone sichtbar sind, die Fruchttriebe und die breite, runde Krone aus sieben Laubmassen, deren Umrisse ihren Lappen folgen, mit Blattbüschelbögen, eiförmigen Blattzeichen und jedem aus der Blickrichtung sichtbaren Apfel in wahrer Größe (1,6 mm): achtundzwanzig in der Seitenansicht, sechsundzwanzig in der Vorderansicht, sechsunddreißig in der Draufsicht. Draufsicht: die vier Leitäste als Astandeutung über der Krone, die Triebe an den Lappenspitzen und der Stamm als Punkt." },
  },
  100: {
    silhouette: { en: "At 1:100 (1 m is 10 mm) the drawings are the true views of the 1:100 body: flared root, stem, leader and scaffold limbs under the crown, four fruiting-shoot strokes, the crown as seven masses with broader lobes and leaf-cluster arcs, and the visible apples (0.8 mm): eighteen in the side elevation, seventeen in the front elevation, twenty-two in the plan. Plan: lobes in sun and shade, leaf arcs and fruit; the stem is a dot under the crown.", de: "Bei 1:100 (1 m sind 10 mm) sind die Zeichnungen die wahren Ansichten des 1:100-Körpers: ausgestellter Wurzelanlauf, Stamm, Stammverlängerung und Leitäste unter der Krone, vier Fruchttriebstriche, die Krone aus sieben Massen mit breiteren Lappen und Blattbüschelbögen und die sichtbaren Äpfel (0,8 mm): achtzehn in der Seitenansicht, siebzehn in der Vorderansicht, zweiundzwanzig in der Draufsicht. Draufsicht: Lappen in Sonne und Schatten, Blattbögen und Früchte; der Stamm ist ein Punkt unter der Krone." },
    retained: { en: "Retained: seven foliage masses, root flare, stem, leader, all scaffold sections, the fruiting shoots and the visible apples as filled dots.", de: "Beibehalten: sieben Laubmassen, Wurzelanlauf, Stamm, Stammverlängerung, alle Leitastabschnitte, die Fruchttriebe und die sichtbaren Äpfel als gefüllte Punkte." },
    omitted: { en: "Omitted: individual leaves, fine twigs, forks and knots; most apples (the body carries a sample of twenty-seven of about 750); the fine 1:50 leaf marks are replaced by arcs.", de: "Weggelassen: einzelne Blätter, feine Zweige, Gabelungen und Astknoten; die meisten Äpfel (der Körper trägt eine Auswahl von siebenundzwanzig von etwa 750); die feinen Blattzeichen von 1:50 sind durch Bögen ersetzt." },
  },
  200: {
    silhouette: { en: "At 1:200 (1 m is 5 mm) the drawings are the true views of the 1:200 body: stem with root flare, leader and the scaffold limbs of two tiers under a broad rounded crown of seven foliage masses (five sun masses, two shade cores) with scalloped outlines; the visible apples are single dots (six in each elevation, seven in the plan). Plan: the lobes around the top mass and the stem as a dot.", de: "Bei 1:200 (1 m sind 5 mm) sind die Zeichnungen die wahren Ansichten des 1:200-Körpers: Stamm mit Wurzelanlauf, Stammverlängerung und die Leitäste zweier Etagen unter einer breiten, runden Krone aus sieben Laubmassen (fünf Sonnenmassen, zwei Schattenkerne) mit gewellten Umrissen; die sichtbaren Äpfel sind einzelne Punkte (sechs in jeder Ansicht, sieben in der Draufsicht). Draufsicht: die Lappen um die Gipfelmasse und der Stamm als Punkt." },
    retained: { en: "Retained: root flare, stem, leader and the scaffold limbs thicker than 0.5 mm on paper, one crack line, and the visible sample apples at the real fruit size (0.4 mm).", de: "Beibehalten: Wurzelanlauf, Stamm, Stammverlängerung und die Leitäste, die auf dem Papier dicker als 0,5 mm sind, eine Risslinie und die sichtbaren Beispieläpfel in wahrer Fruchtgröße (0,4 mm)." },
  },
  500: {
    silhouette: { en: "At 1:500 (1 m is 2 mm) the apple tree is one closed, broad and rounded crown, 14 mm wide, with a scalloped underside, four low side lobes and four shoulder lobes under a rounded top, above a short stem of 1.6 m with its root flare; the drawings are the true views of the 1:500 body. Front: stem, lower leader and the crown. Plan: one scalloped crown outline from above; the stem is hidden.", de: "Bei 1:500 (1 m sind 2 mm) ist der Apfelbaum eine geschlossene, breite und runde Krone von 14 mm Breite mit gewellter Unterseite, vier tiefen Seitenlappen und vier Schulterlappen unter einem runden Gipfel über einem kurzen Stamm von 1,6 m mit Wurzelanlauf; die Zeichnungen sind die wahren Ansichten des 1:500-Körpers. Vorderansicht: Stamm, untere Stammverlängerung und Krone. Draufsicht: ein gewellter Kronenumriss von oben; der Stamm ist verdeckt." },
  },
};
for (const [scale, texts] of Object.entries(DRAWN)) Object.assign(asset.representations[scale].decisions, texts);

const anchorsOut: Record<string, { bone: string; local: V3 }> = {};

function volumeOf(m: Model): Json {
  const s = (p: Part): Json => solidOf(p, F);
  const body: Json[] = [...m.wood.map(s), ...m.knot, ...m.twigs.map(s), ...m.lobes.map(s), ...m.fruit.map(s)];
  const region = (p: Part | Json): string => {
    if (p.kind === "fruit") return "fruit";
    if (p.kind === "lobe") return p.group === "shade" ? "leaf-shade" : "leaf-sun";
    if (p.group === "twig") return "spur-twig";
    return p.group === "root" || p.group === "stem" ? "trunk-bark" : "limb-bark";
  };
  const thermal: Json[] = [];
  for (const p of [...m.wood, ...m.twigs, ...m.lobes, ...m.fruit]) thermal.push({ id: `th-${p.id}`, of: p.id, region: region(p) });
  for (const kn of m.knot) thermal.push({ id: `th-${kn.id}`, of: kn.id, region: "trunk-bark" });
  for (const p of [...m.hiddenWood, ...m.hiddenTwigs, ...m.hiddenFruit]) thermal.push({ ...s({ ...p, id: `th-${p.id}` } as Part), region: region(p) });
  m.outlets.forEach(([emitter, anchor, lobeId, az, el], i) => {
    const l = m.lobes.find((x) => x.id === lobeId)!;
    const d = dirOf(az, el);
    const p = surfacePoint(l, d);
    const q = sub(p, l.c);
    const n = unit(l.axes.reduce((sum, axis, k) => add(sum, mul(axis, dot3(q, axis) / l.radii[k]! ** 2)), [0, 0, 0] as V3));
    const side = unit(Math.abs(n[1]) > 0.98 ? [1, 0, 0] : [n[2], 0, -n[0]]);
    const third: V3 = [side[1] * n[2] - side[2] * n[1], side[2] * n[0] - side[0] * n[2], side[0] * n[1] - side[1] * n[0]];
    const rho = Math.min(l.radii[0], l.radii[2]) * (m.scale === 500 ? 0.45 : 0.6);
    const h = Math.max(0.1, (0.15 * m.scale) / 1000, (rho * rho) / (2 * Math.min(...l.radii)) * 0.8);
    const lens: Lobe = { ...l, id: `th-${anchor ?? `${emitter}-${i + 1}`}`, c: sub(p, mul(n, h - 0.02)), axes: [side, n, third], radii: [rho, h, rho] };
    thermal.push({ ...s(lens), role: "detail", region: l.group === "shade" ? "leaf-shade" : "leaf-sun", tags: [emitter] });
    const out = add(p, mul(n, 0.03));
    const buried = m.lobes.find((o) => o !== l && Math.hypot(...o.axes.map((axis, k) => dot3(sub(out, o.c), axis) / o.radii[k]!)) < 1);
    if (buried && l.group !== "shade") console.log(`[DEBUG] 1:${m.scale} outlet on ${lobeId} is buried in ${buried.id}`);
    if (anchor) anchorsOut[anchor] = { bone: l.bone, local: toLocal(F[l.bone]!, add(p, mul(n, 0.04))) };
  });
  const structure: Json[] = [];
  const bark = (r: number): number => 0.002 + 0.04 * r;
  const members = [...m.wood, ...m.hiddenWood, ...m.twigs, ...m.hiddenTwigs].filter((t) => t.id === t.bone);
  for (const t of members) {
    const section = bone(t.bone).section;
    const declared = Math.sqrt(section.areaM2 / Math.PI);
    const pts = m.scale === 500 ? t.points.map((q) => ({ p: q.p, r: t.group === "root" ? q.r - bark(q.r) : declared })) : t.points.map((q) => ({ p: q.p, r: q.r - bark(q.r) }));
    structure.push({ ...s({ ...t, id: `member-${t.id}`, material: "apple-wood", tone: 0.4, points: pts, flags: t.group === "twig" ? { slender: true } : undefined } as Tube), member: t.bone, part: "member" });
  }
  const feature = (0.15 * m.scale) / 1000;
  for (const t of members) {
    if (t.bone === "root-plate") continue;
    const f = F[t.bone]!;
    const first = t.points.find((q) => toLocal(f, q.p)[0] >= -1e-6) ?? t.points[0]!;
    const r = (first.r - bark(first.r)) * 1.12;
    if (2 * r < feature || (m.scale === 500 && !["trunk", "ld", "ld-b"].includes(t.bone))) continue;
    structure.push({ id: `joint-${t.bone}`, kind: "ellipsoid", bone: t.bone, material: "apple-wood", role: "detail", tone: 0.35, radii: [r5(r), r5(r), r5(r)], transform: { translation: [0, 0, 0] }, member: t.bone, part: "joint" });
  }
  const plate = Math.max(0.04, feature);
  structure.push({ id: "support-base-support", kind: "revolve", bone: "root-plate", material: "apple-wood", role: "detail", tone: 0.25, profile: [[r5(-plate), 0.8], [0, 0.8]], axis: [1, 0, 0], origin: [0, 0, 0], member: "root-plate", part: "support", anchor: "base-support" });
  const masses: [string, number, number, number][] = [["root-plate", 0.11, 0.15, 0.15], ["trunk", 0.3, 0.11, 0.11], ["ld", 0.5, 0.07, 0.07]];
  for (const [id, a, b, c] of masses) structure.push({ id: `mass-${id}`, kind: "ellipsoid", bone: id, material: "apple-wood", role: "detail", tone: 0.3, radii: [a, b, c], transform: { translation: [bone(id).mass.comX, bone(id).mass.comY, 0] }, member: id, part: "mass" });
  if (m.scale !== 500) for (const b of asset.rig.bones.filter((x: Json) => x.id.startsWith("fol-"))) {
    const r = Math.max(feature * 0.6, Math.cbrt((3 * b.mass.kg) / (4 * Math.PI * 800)));
    structure.push({ id: `mass-${b.id}`, kind: "ellipsoid", bone: b.id, material: b.id.startsWith("fol-core") ? SHADE : SUN, role: "detail", tone: 0.3, radii: [r5(r), r5(r), r5(r)], transform: { translation: [b.mass.comX, b.mass.comY, 0] }, member: b.parent, part: "mass" });
  }
  return { status: "authored", denominator: m.scale, body, thermal, structure, decisions: m.decisions };
}

export const models = [50, 100, 200, 500].map(model);
for (const m of models) for (const t of [...m.wood, ...m.twigs]) {
  if ((t.group !== "side" && t.group !== "twig" && t.group !== "limb") || /^sc-..a$/.test(t.id)) continue;
  const tip = t.points[t.points.length - 1]!.p;
  const depth = Math.max(...m.lobes.map((l) => 1 - Math.hypot(...l.axes.map((a, i) => (a[0] * (tip[0] - l.c[0]) + a[1] * (tip[1] - l.c[1]) + a[2] * (tip[2] - l.c[2])) / l.radii[i]!))));
  if (depth < 0.06) console.log(`[DEBUG] 1:${m.scale} tip of ${t.id} at ${tip.map((v) => v.toFixed(2)).join(" ")} is ${depth < 0 ? "outside" : "shallow in"} the foliage (${depth.toFixed(2)})`);
}
const volumes: Json = {};
for (const m of models) volumes[String(m.scale)] = volumeOf(m);
writeFileSync(join(SOURCES, "vegetation", "apple-tree.json"), JSON.stringify({ volumes }) + "\n");

for (const a of asset.anchors) {
  const moved = anchorsOut[a.id];
  if (!moved) continue;
  if (moved.bone !== a.bone) throw new Error(`anchor ${a.id} would change its bone`);
  a.x = r4(moved.local[0]);
  a.y = r4(moved.local[1]);
  a.z = r4(moved.local[2]);
}
asset.volumes = volumes;
const report = draw(asset, F, models.map((m) => ({ scale: m.scale, tubes: [...m.wood, ...m.twigs], lobes: m.lobes, fruit: m.fruit })));
const out = join(GENERATED, "r3-apple-tree");
mkdirSync(out, { recursive: true });
writeFileSync(join(out, "🔣️.json"), JSON.stringify(document, null, 2) + "\n");
for (const m of models) console.log(`1:${m.scale} body ${volumes[m.scale].body.length} thermal ${volumes[m.scale].thermal.length} structure ${volumes[m.scale].structure.length}`);
console.log(report.join("\n"));
