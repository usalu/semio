/** 🌲️ Round-3 Lombardy poplar (Populus nigra 'Italica', mature, 26 m): every stem section, buttress, bark ridge, burl, branch row, flame tuft, core and outlet is stated per scale in world metres (x ahead, y up, z lateral, left +) and written as bone-local solids to `volumes/vegetation/lombardy-poplar.json`; the root fixes (poses and clips, emitter anchors, branch-bark area, 2D drawings and plan symbol as true views of each scale's body) go to the candidate document `🗑️generated/r3-lombardy-poplar/🔣️.json`.
 * @see ./round3-brief.md
 * @see ./r3-vegetation-lombardy-poplar.md */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { GENERATED, SOURCES, entryOf, frames, toLocal, toWorld, type Frame, type Json, type V3 } from "./volumes-lib.ts";
import { D2R, add, along, branch, coreSolid, dirOf, dot3, len, mul, r4, r5, solidOf, sub, tuft, unit, type Core, type Lobe, type Tube, type Tuft } from "./r3-lombardy-poplar-kit.ts";
import { draw } from "./r3-lombardy-poplar-draw.ts";

const { entry } = entryOf("vegetation/lombardy-poplar");
const document: Json = JSON.parse(readFileSync(entry.path, "utf8"));
const asset: Json = document.asset;
const bone = (id: string): Json => asset.rig.bones.find((b: Json) => b.id === id);
export const F: Record<string, Frame> = frames(asset);

const BARK = "poplar-bark", LEAF = "poplar-leaf", WOOD = "poplar-wood";
const TONE = { stem: 0.28, ridge: 0.2, branch: 0.32, leafL: 0.5, leafR: 0.55, top: 0.6, core: 0.42 };

const JOINTS: [number, number][] = [[0, 0], [0, 6.3], [F["trunk-3"]!.o[0], F["trunk-3"]!.o[1]], [F["trunk-4"]!.o[0], F["trunk-4"]!.o[1]], [toWorld(F["trunk-4"]!, [6.3, 0, 0])[0], 24.6], [toWorld(F["trunk-4"]!, [7.4, 0, 0])[0], 26]];
/** 📏️ The x of the stem axis at the height `y` (the rig joints joined by straight lines). */
export const axisX = (y: number): number => {
  for (let i = 0; i + 1 < JOINTS.length; i++) {
    const [a, b] = [JOINTS[i]!, JOINTS[i + 1]!];
    if (y <= b[1] || i + 2 === JOINTS.length) return a[0] + ((b[0] - a[0]) * (y - a[1])) / (b[1] - a[1]);
  }
  return 0;
};

type StemRow = [y: number, r: number, dx?: number, dz?: number];
const STEM: Record<number, Record<string, StemRow[]>> = {
  50: {
    "root-plate": [[0, 0.52], [0.06, 0.445], [0.15, 0.372], [0.27, 0.333], [0.42, 0.312]],
    "trunk-1": [[0.3, 0.316], [0.7, 0.296], [1.3, 0.282], [2.2, 0.272, 0.01, 0.005], [3.2, 0.262, 0.012, 0.01], [4.2, 0.252, 0.004, 0.012], [5.2, 0.243, -0.006, 0.006], [6.42, 0.232]],
    "trunk-2": [[6.2, 0.234], [7.4, 0.222, 0.008, 0], [8.6, 0.209, 0.012, -0.006], [9.8, 0.196, 0.006, -0.01], [11, 0.183, -0.004, -0.004], [12.42, 0.168]],
    "trunk-3": [[12.2, 0.17], [13.6, 0.157, 0.004, 0.006], [15, 0.145, 0.008, 0.004], [16.6, 0.13, 0.002, -0.004], [18.42, 0.113]],
    "trunk-4": [[18.2, 0.115], [19.3, 0.103], [20.4, 0.09, 0.004, 0], [21.5, 0.074, 0.006, 0.004], [22.6, 0.058], [23.6, 0.04], [24.6, 0.016]],
  },
  100: {
    "root-plate": [[0, 0.52], [0.08, 0.425], [0.2, 0.35], [0.42, 0.312]],
    "trunk-1": [[0.3, 0.316], [1.3, 0.282], [3.2, 0.262, 0.012, 0.01], [5.2, 0.243, -0.006, 0.006], [6.42, 0.232]],
    "trunk-2": [[6.2, 0.234], [8.6, 0.209, 0.012, -0.006], [11, 0.183, -0.004, -0.004], [12.42, 0.168]],
    "trunk-3": [[12.2, 0.17], [15, 0.145, 0.008, 0.004], [18.42, 0.113]],
    "trunk-4": [[18.2, 0.115], [20.4, 0.09, 0.004, 0], [22.6, 0.058], [24.6, 0.02]],
  },
  200: {
    "root-plate": [[0, 0.56], [0.12, 0.4], [0.42, 0.313]],
    "trunk-1": [[0.3, 0.316], [1.3, 0.282], [4.2, 0.252, 0.004, 0.01], [6.42, 0.232]],
    "trunk-2": [[6.2, 0.234], [9.4, 0.2, 0.01, -0.008], [12.42, 0.168]],
    "trunk-3": [[12.2, 0.17], [18.42, 0.113]],
    "trunk-4": [[18.2, 0.115], [21.5, 0.075], [24.4, 0.03]],
  },
  500: {
    "trunk-1": [[0, 0.6], [0.2, 0.36], [1.3, 0.282], [6.42, 0.232]],
    "trunk-2": [[6.2, 0.234], [12.42, 0.168]],
    "trunk-3": [[12.2, 0.17], [18.42, 0.113]],
    "trunk-4": [[18.2, 0.115], [23.4, 0.045]],
  },
};
const stemTube = (id: string, boneId: string, rows: StemRow[]): Tube => ({ kind: "tube", id, bone: boneId, material: BARK, tone: TONE.stem, group: "stem", shape: boneId, points: rows.map(([y, r, dx = 0, dz = 0]) => ({ p: [axisX(y) + dx, y, dz] as V3, r })) });
/** 📐️ The bark radius of the stem of a scale at the height `y`. */
const stemR = (scale: number, y: number): number => {
  const rows = Object.values(STEM[scale]!).flat().sort((a, b) => a[0] - b[0]);
  for (let i = 0; i + 1 < rows.length; i++) if (y <= rows[i + 1]![0]) return rows[i]![1] + ((rows[i + 1]![1] - rows[i]![1]) * (y - rows[i]![0])) / Math.max(1e-6, rows[i + 1]![0] - rows[i]![0]);
  return 0.02;
};

/** 🫓️ Buttress ridges of the fluted stem foot: [azimuth, reach factor, radius factor]. */
const BUTTRESS: Record<number, [number, number, number][]> = {
  50: [[15, 1, 1], [62, 0.9, 0.9], [118, 1.05, 1.05], [165, 0.92, 0.95], [212, 1, 1], [268, 0.95, 0.92], [320, 1.04, 1]],
  100: [[20, 1, 1], [95, 1, 1], [165, 0.95, 1], [240, 1, 1], [310, 1.02, 1]],
};
const buttress = (scale: number, i: number, [az, k, kr]: [number, number, number]): Tube => {
  const at = (rad: number, y: number): V3 => [axisX(y) + rad * k * Math.cos(az * D2R), y, rad * k * Math.sin(az * D2R)];
  const rows: [number, number, number][] = scale === 50 ? [[0.2, 1.35, 0.05], [0.25, 0.85, 0.075], [0.33, 0.45, 0.1], [0.47, 0.18, 0.105], [0.64, 0.06, 0.078], [0.8, 0.025, 0.04]] : [[0.22, 1.1, 0.06], [0.33, 0.45, 0.1], [0.55, 0.12, 0.1], [0.78, 0.035, 0.05]];
  return { kind: "tube", id: `buttress-${i + 1}`, bone: "root-plate", material: BARK, tone: TONE.stem, group: "stem", shape: "root-plate", points: rows.map(([rad, y, r]) => ({ p: at(rad, y), r: r * kr })) };
};

/** 🪵️ Bark ridges of the lower stem at 1:50 (deep longitudinal furrows of old Lombardy bark): [bone, azimuth, drift, y0, y1, radius]. */
const RIDGES: [string, number, number, number, number, number][] = [
  ["trunk-1", 8, 6, 1.0, 6.3, 0.017], ["trunk-1", 44, -5, 0.9, 6.0, 0.016], ["trunk-1", 80, 8, 1.1, 6.3, 0.018], ["trunk-1", 118, -4, 0.95, 5.6, 0.016], ["trunk-1", 152, 5, 1.0, 6.3, 0.017],
  ["trunk-1", 190, -7, 0.9, 6.2, 0.018], ["trunk-1", 226, 4, 1.05, 5.9, 0.016], ["trunk-1", 262, -6, 1.0, 6.3, 0.017], ["trunk-1", 298, 7, 0.95, 6.1, 0.016], ["trunk-1", 334, -3, 1.0, 6.3, 0.017],
  ["trunk-2", 25, 6, 6.3, 10.5, 0.013], ["trunk-2", 95, -5, 6.3, 11.2, 0.013], ["trunk-2", 160, 4, 6.3, 10.2, 0.012], ["trunk-2", 230, -6, 6.3, 11.0, 0.013], ["trunk-2", 300, 5, 6.3, 10.8, 0.012],
];
const ridge = ([boneId, az, drift, y0, y1, r]: (typeof RIDGES)[number], i: number): Tube => {
  const n = 6;
  const points = Array.from({ length: n + 1 }, (_, k) => {
    const y = y0 + ((y1 - y0) * k) / n;
    const a = (az + drift * Math.sin((k / n) * Math.PI * 1.3)) * D2R;
    const rad = stemR(50, y) - r * 0.35;
    const taper = k === 0 || k === n ? 0.45 : 1;
    return { p: [axisX(y) + rad * Math.cos(a), y, rad * Math.sin(a)] as V3, r: r * taper };
  });
  return { kind: "tube", id: `ridge-${i + 1}`, bone: boneId, material: BARK, tone: TONE.ridge, group: "ridge", shape: boneId, points, flags: { texture: true, role: "texture" } };
};

/** 🌰️ Burls on the lower stem at 1:50: [height, azimuth, out, up, across]. */
const BURLS: [number, number, number, number, number][] = [[1.9, 250, 0.07, 0.12, 0.1], [3.75, 70, 0.06, 0.1, 0.085], [5.6, 160, 0.055, 0.09, 0.08]];

/** 🌳️ Crown envelope: radius of the outer foliage surface (towards world x) by height; world z reaches 4.5 % further (crown 4.20 m wide, 4.39 m deep). */
const ENVELOPE: [number, number][] = [[1.6, 0.35], [2.2, 1.25], [3, 1.72], [4, 1.95], [5.5, 2.04], [7, 2.07], [8.5, 2.05], [10, 2.0], [11.5, 1.9], [13, 1.76], [14.5, 1.6], [16, 1.43], [17.5, 1.25], [19, 1.1], [20.5, 0.96], [22, 0.8], [23.3, 0.63], [24.5, 0.43], [25.3, 0.24], [25.96, 0.02]];
const DEPTH = 1.045;
/** 🎚️ Per-scale fit of the crown to the declared width (world x) and depth (world z): the flames of each scale fall differently on the sheaves. */
const FITS: Record<number, [number, number]> = { 50: [0.973, 1], 100: [0.99, 0.996], 200: [1.005, 1.02], 500: [1, 1] };
let FIT: [number, number] = [1, 1];
/** 🌊️ The vertical sheaves of the fastigiate crown: shallow clefts and swells round the column that drift slowly with height. */
export const sheaves = (y: number, az: number): number => 1 + 0.05 * Math.sin((3 * az) * D2R + 0.22 * y + 0.6) + 0.03 * Math.sin((7 * az) * D2R - 0.31 * y);
export const envelope = (y: number, az?: number): number => {
  let r = 0;
  if (y <= ENVELOPE[0]![0]) r = ENVELOPE[0]![1];
  else if (y >= ENVELOPE[ENVELOPE.length - 1]![0]) r = ENVELOPE[ENVELOPE.length - 1]![1];
  else for (let i = 0; i + 1 < ENVELOPE.length; i++) {
    const [a, b] = [ENVELOPE[i]!, ENVELOPE[i + 1]!];
    if (y <= b[0]) {
      r = a[1] + ((b[1] - a[1]) * (y - a[0])) / (b[0] - a[0]);
      break;
    }
  }
  return az === undefined ? r : (r * sheaves(y, az)) / Math.hypot(Math.cos(az * D2R) / FIT[0], Math.sin(az * D2R) / (DEPTH * FIT[1]));
};

/** 🍃️ Tuft length and radius by height per scale. */
const TUFT: Record<number, [number, number, number][]> = {
  50: [[2, 1.4, 0.34], [6, 1.6, 0.37], [12, 1.5, 0.35], [18, 1.3, 0.3], [22, 1.1, 0.25], [24, 0.9, 0.2]],
  100: [[2, 1.8, 0.52], [6, 2.15, 0.57], [12, 2.0, 0.53], [18, 1.75, 0.45], [22, 1.45, 0.36], [24, 1.2, 0.28]],
  200: [[2, 2.3, 0.7], [6, 2.7, 0.76], [12, 2.5, 0.7], [18, 2.2, 0.6], [22, 1.8, 0.47], [24, 1.5, 0.36]],
};
const tuftSize = (scale: number, y: number): [number, number] => {
  const t = TUFT[scale]!;
  if (y <= t[0]![0]) return [t[0]![1], t[0]![2]];
  for (let i = 0; i + 1 < t.length; i++) if (y <= t[i + 1]![0]) {
    const k = (y - t[i]![0]) / (t[i + 1]![0] - t[i]![0]);
    return [t[i]![1] + (t[i + 1]![1] - t[i]![1]) * k, t[i]![2] + (t[i + 1]![2] - t[i]![2]) * k];
  }
  return [t[t.length - 1]![1], t[t.length - 1]![2]];
};

/** 🌱️ Branch rows: [height of the collar, azimuth (0 = ahead, 90 = left), crown reach factor, rig bone or null]. The rig branches run in the rig plane (azimuth 180 = behind for `br-l*`, 0 = ahead for `br-r*`); the others radiate round the stem about a golden-angle spiral, steeply ascending, as fastigiate poplar branches do. */
type BranchRow = [y: number, az: number, reach: number, rig: string | null];
const BRANCHES: Record<number, BranchRow[]> = {
  50: [
    [1.75, 40, 0.96, null], [2.2, 160, 0.98, null], [2.65, 300, 1, null], [3.3, 180, 1, "br-l1"], [3.5, 75, 1.01, null], [3.95, 215, 0.97, null], [4.4, 330, 1, null], [4.85, 110, 0.99, null],
    [5.1, 0, 1, "br-r1"], [5.5, 250, 1.01, null], [5.95, 25, 0.98, null], [6.4, 160, 1, null], [6.85, 290, 0.99, null], [7.3, 65, 1.01, null], [7.75, 205, 0.97, null], [8.1, 180, 1, "br-l2"],
    [8.35, 320, 1, null], [8.8, 95, 0.99, null], [9.25, 235, 1.01, null], [9.7, 40, 0.98, null], [10.1, 0, 1, "br-r2"], [10.4, 140, 1, null], [10.85, 270, 0.99, null], [11.3, 60, 1.01, null],
    [11.75, 195, 0.98, null], [12.2, 320, 1, null], [12.65, 100, 0.99, null], [13.1, 230, 1.01, null], [13.55, 350, 0.98, null], [13.8, 180, 1, "br-l3"], [14.1, 120, 1, null], [14.55, 255, 0.99, null],
    [15.0, 40, 1.01, null], [15.45, 155, 0.98, null], [15.8, 0, 1, "br-r3"], [16.1, 285, 1, null], [16.55, 75, 0.99, null], [17.0, 205, 1.01, null], [17.45, 330, 0.98, null], [17.9, 110, 1, null],
    [18.35, 240, 0.99, null], [18.8, 20, 1.01, null], [19.25, 140, 0.98, null], [19.8, 180, 1, "br-l4"], [20.1, 280, 1, null], [20.5, 60, 0.99, null], [20.9, 210, 1.01, null], [21.3, 330, 0.98, null],
    [21.8, 0, 1, "br-r4"], [22.1, 120, 1, null], [22.5, 240, 0.99, null], [22.9, 350, 1, null], [23.3, 170, 0.98, null],
  ],
  100: [
    [1.85, 60, 0.97, null], [2.6, 290, 1, null], [3.3, 180, 1, "br-l1"], [3.9, 100, 1, null], [4.6, 330, 0.98, null], [5.1, 0, 1, "br-r1"], [5.7, 230, 1, null], [6.5, 110, 0.99, null],
    [7.3, 300, 1, null], [8.1, 180, 1, "br-l2"], [8.8, 60, 1, null], [9.5, 250, 0.98, null], [10.1, 0, 1, "br-r2"], [10.8, 130, 1, null], [11.6, 290, 0.99, null], [12.4, 70, 1, null],
    [13.1, 230, 0.99, null], [13.8, 180, 1, "br-l3"], [14.5, 30, 1, null], [15.2, 280, 0.99, null], [15.8, 0, 1, "br-r3"], [16.5, 130, 1, null], [17.3, 250, 0.99, null], [18.1, 60, 1, null],
    [18.9, 300, 0.99, null], [19.8, 180, 1, "br-l4"], [20.6, 90, 1, null], [21.2, 270, 0.99, null], [21.8, 0, 1, "br-r4"], [22.6, 200, 1, null],
  ],
  200: [
    [2.0, 70, 0.98, null], [3.3, 180, 1, "br-l1"], [4.4, 300, 1, null], [5.1, 0, 1, "br-r1"], [6.6, 92, 1.03, null], [8.1, 180, 1, "br-l2"], [9.1, 268, 1.075, null], [10.1, 0, 1, "br-r2"],
    [11.8, 90, 1, null], [13.8, 180, 1, "br-l3"], [14.8, 290, 1, null], [15.8, 0, 1, "br-r3"], [17.6, 100, 1, null], [19.8, 180, 1, "br-l4"], [20.8, 280, 1, null], [21.8, 0, 1, "br-r4"],
  ],
};

/** 🎯️ The rig bone a foliage part at height `y` and world x `x` belongs to (four tiers in two halves and the spire), as the 2D binds the flames. */
export function bandOf(y: number, x: number): string {
  if (y >= 22.6) return "fol-top";
  const side = x < axisX(y) ? "l" : "r";
  const tier = y < 7.6 ? 1 : y < 13.2 ? 2 : y < 19.0 ? 3 : 4;
  return `fol-${side}${tier}`;
}
const trunkOf = (y: number): string => (y < 6.3 ? "trunk-1" : y < 12.3 ? "trunk-2" : y < 18.3 ? "trunk-3" : "trunk-4");

/** 🧮️ How far the parts of a tuft stand beyond the crown envelope (positive = outside), over its sections. */
function excess(t: Tuft, reach: number): number {
  let worst = -Infinity;
  for (const q of t.points) {
    const dx = q.p[0] - axisX(q.p[1]), dz = q.p[2];
    const az = Math.atan2(dz, dx) / D2R;
    worst = Math.max(worst, Math.hypot(dx, dz) + q.r - reach * envelope(q.p[1], az));
  }
  const top = t.points[t.points.length - 1]!.p[1];
  return Math.max(worst, top - 25.9);
}

type Grown = { wood: Tube; tufts: Tuft[]; rig: string | null };

/** 🌿️ A branch and its flame tufts: the branch leaves the stem at the collar, rises steeply and bends further upward; its length is found so that its tip tuft just meets the crown envelope; side tufts fan out by 32 degrees and an inner tuft fills towards the stem. */
function grow(scale: number, [y0, az, reach, rig]: BranchRow, n: number): Grown {
  const low = Math.max(0, Math.min(1, (y0 - 1.7) / 3));
  let [e0, e1] = [44 + 10 * low, 72 + 4 * low];
  if (rig) {
    const u = F[rig]!.u;
    const chord = Math.atan2(u[1], Math.hypot(u[0], u[2])) / D2R;
    [e0, e1] = [chord - 10, chord + 10];
  }
  const rTop = 0.036 - 0.02 * Math.min(1, y0 / 23);
  const merged = scale === 200 ? [1.3, 1.8] : scale === 100 ? [1.12, 1.65] : [1, 1];
  const r0 = rig ? (0.06 - 0.0012 * (y0 - 3.3)) * merged[0]! : rTop * merged[1]!;
  const r1 = scale === 50 ? 0.01 : scale === 100 ? 0.012 : 0.016;
  const start: V3 = [axisX(y0) + 0.45 * stemR(scale, y0) * Math.cos(az * D2R), y0, 0.45 * stemR(scale, y0) * Math.sin(az * D2R)];
  const [lt, rt] = tuftSize(scale, y0 + 1.5);
  const material = LEAF;
  const make = (L: number): Grown => {
    const wood = branch(rig ?? `branch-${n}`, rig ?? trunkOf(y0), BARK, TONE.branch, rig ? "rig" : "branch", rig ?? trunkOf(y0), start, az, e0, e1, L, r0, r1, 0, scale === 50 ? 6 : scale === 100 ? 4 : 3);
    const tufts: Tuft[] = [];
    const add1 = (role: Tuft["role"], f: number, dAz: number, el: number, kl: number, kr: number, i: number): void => {
      const dir = dirOf(az + dAz, el);
      const base = role === "tip" ? sub(wood.points[wood.points.length - 1]!.p, mul(dir, 0.45 * lt * kl)) : along(wood, f);
      tufts.push(tuft(`tuft-${n}-${i}`, "", material, 0, base, dir, lt * kl, rt * kr, role));
    };
    add1("tip", 1, 0, 82, 1, 1, 1);
    if (scale !== 200 || y0 < 21) {
      add1("side", 0.62, 34, 74, 0.86, 0.86, 2);
      add1("side", 0.62, -34, 74, 0.86, 0.86, 3);
    }
    if (scale === 50 && y0 < 21) add1("inner", 0.35, 0, 86, 0.78, 0.8, 4);

    if (scale === 100 && y0 < 19) add1("inner", 0.36, 0, 86, 0.8, 0.82, 4);
    return { wood, tufts, rig };
  };
  let [lo, hi] = [0.25, 6];
  for (let k = 0; k < 40; k++) {
    const mid = (lo + hi) / 2;
    const tip = make(mid).tufts[0]!;
    if (excess(tip, reach * 0.93) > 0) hi = mid;
    else lo = mid;
  }
  const g = make(lo);
  for (const t of g.tufts.slice(1)) {
    let k = 1;
    while (excess(t, reach * 0.93) > 0.02 && k > 0.4) {
      k -= 0.04;
      const scaled = tuft(t.id, "", t.material, 0, t.points[0]!.p, t.dir, t.length * k, t.radius * (0.5 + 0.5 * k), t.role);
      Object.assign(t, scaled);
    }
  }
  return g;
}

/** 🗼️ The spire round the leader top: four flames round it and the apex flame that ends at the declared height. */
function spire(scale: number): Tuft[] {
  const out: Tuft[] = [];
  const k = scale === 50 ? 1 : scale === 100 ? 1.1 : 1.2;
  const ring: [number, number][] = scale === 50 ? [[45, 22.9], [135, 23.1], [225, 22.95], [315, 23.05]] : scale === 100 ? [[60, 22.9], [180, 23.0], [300, 22.95]] : [[90, 22.9], [270, 22.9]];
  ring.forEach(([az, y], i) => {
    const base: V3 = [axisX(y) + 0.08 * Math.cos(az * D2R), y, 0.08 * Math.sin(az * D2R)];
    out.push(tuft(`spire-${i + 1}`, "fol-top", LEAF, TONE.top, base, unit(add(dirOf(az, 0), mul([0, 1, 0], 6.5))), 1.45 * Math.min(k, 1.2), 0.26 * k, "spire"));
  });
  const apexBase: V3 = [axisX(24.15), 24.15, 0];
  const apex = tuft("spire-apex", "fol-top", LEAF, TONE.top, apexBase, [0, 1, 0], 25.96 - 24.15, 0.27 * Math.min(k, 1.3), "spire");
  out.push(apex);
  return out;
}

/** 🧶️ The leaf skin of the column at 1:50: flame tufts in rows round the crown, each row turned by half a step against the one below, so that the outer surface is closed by overlapping, slightly outward-leaning flames as on a real fastigiate crown; rows from 2.35 m every 0.78 m, 0.66 m apart round the crown, flames 1.3 m long and 0.33 m thick, shorter and thinner towards the top; the lowest row is a ring of flames leaning outwards that rounds the crown base. */
type Seed = { az: number; y: number; kl: number; r: number; lt: number; sink: number; lean: number; under: boolean };
function skinSeeds(): Seed[][] {
  const [y0, y1, dy, step, lt0, rt0] = [2.35, 24.4, 0.78, 0.66, 1.3, 0.33];
  const rows: Seed[][] = [];
  const under = Math.max(6, Math.round((2 * Math.PI * envelope(2.3) * 0.75) / step));
  rows.push(Array.from({ length: under }, (_, i) => ({ az: (i * 360) / under + 11, y: 2.15, kl: 1, r: rt0 * 0.95, lt: lt0 * 0.8, sink: 0, lean: 0.75, under: true })));
  let row = 0;
  for (let y = y0; y <= y1 + 1e-6; y += dy, row++) {
    const top = Math.max(0.45, Math.min(1, (25.6 - y) / 4));
    const [lt, rt] = [lt0 * (0.75 + 0.25 * top), rt0 * (0.7 + 0.3 * top)];
    const n = Math.max(3, Math.round((2 * Math.PI * envelope(y + 0.2 * lt)) / step));
    const seeds: Seed[] = [];
    for (let i = 0; i < n; i++) {
      const h = (a: number, b: number): number => {
        const v = Math.sin(row * a + i * b) * 43758.5453;
        return v - Math.floor(v) - 0.5;
      };
      const [h1, h2, h3] = [h(12.9898, 78.233), h(39.346, 11.135), h(73.156, 52.235)];
      const r = rt * (1 + 0.2 * h3);
      seeds.push({ az: ((i + (row % 2) * 0.5 + 0.35 * h1) * 360) / n + row * 7, y: y + 0.3 * h3 * dy, kl: 1 + 0.3 * h2, r, lt, sink: h2 < -0.3 ? 0.25 * r : 0, lean: 0.17 + 0.16 * h1, under: false });
    }
    rows.push(seeds);
  }
  return rows;
}

/** 🧵️ Generalisation for a coarser scale: of every `of` neighbouring flames of a row only `keep` stay, broadened and lengthened so that they close the gaps of the merged ones; their places stay those of the finer scale, so both scales keep the same outline. */
function generalise(rows: Seed[][], of: number, keep: number, kl: number, kr: number): Seed[][] {
  return rows.map((seeds) => seeds.filter((_, i) => i % of < keep || seeds.length <= 4).map((sd) => ({ ...sd, r: sd.r * kr, lt: sd.lt * kl, sink: sd.sink * 0.5 })));
}

function skin(scale: number): Tuft[] {
  let rows = skinSeeds();
  if (scale >= 100) rows = generalise(rows, 3, 2, 1.1, 1.32);
  if (scale >= 200) rows = generalise(rows, 2, 1, 1.15, 1.45);
  return rows.flat().map((sd, k) => {
    const R = envelope(sd.y + (sd.under ? 0.1 : 0), sd.az);
    const rad: V3 = [Math.cos(sd.az * D2R), 0, Math.sin(sd.az * D2R)];
    const dir = unit(add(mul(rad, sd.lean), [0, 1, 0]));
    const inset = sd.under ? sd.r * 1.15 : sd.r * 0.98 + sd.sink;
    const c: V3 = [axisX(sd.y) + rad[0] * (R - inset), sd.y, rad[2] * (R - inset)];
    const len = sd.lt * sd.kl;
    return tuft(`skin-${k}`, "", LEAF, 0, sub(c, mul(dir, 0.45 * len)), dir, len, sd.r, "side");
  });
}

type Model = { scale: number; wood: Tube[]; tufts: Tuft[]; cores: Core[]; burls: Lobe[]; lobes: Lobe[]; decisions: { en: string; de: string }[] };

const CORE_K: Record<number, number> = { 50: 0.64, 100: 0.66, 200: 0.7 };
const BANDS: [string, number, number][] = [["1", 2.7, 7.6], ["2", 7.6, 13.2], ["3", 13.2, 19.0], ["4", 19.0, 22.6]];

/** 🫘️ Inner foliage mass of a tier half (left = behind, right = ahead of the stem), so that the gaps between the flames show leaves, not sky. */
function cores(scale: number): Core[] {
  const k = CORE_K[scale]!;
  const out: Core[] = [];
  for (const [tier, ya, yb] of BANDS) for (const side of ["l", "r"] as const) {
    const s = side === "l" ? -1 : 1;
    const [y0, y1] = [ya - (tier === "1" ? 0 : 0.35), yb + 0.35];
    const n = Math.max(3, Math.round((y1 - y0) / (scale === 50 ? 0.8 : scale === 100 ? 1.2 : 1.8)) + 1);
    const rows = Array.from({ length: n }, (_, i) => {
      const y = y0 + ((y1 - y0) * i) / (n - 1);
      const end = i === 0 || i === n - 1 ? 0.55 : 1;
      const R = Math.max(0.12, envelope(Math.max(y, 2.0)) * k * end);
      return { p: [axisX(y) + s * 0.27 * R, y, 0] as V3, rx: 0.73 * R, rz: R * DEPTH * 0.98 };
    });
    out.push({ kind: "core", id: `core-${side}${tier}`, bone: `fol-${side}${tier}`, material: LEAF, tone: TONE.core, rows });
  }
  const top: Core = { kind: "core", id: "core-top", bone: "fol-top", material: LEAF, tone: TONE.core, rows: [22.25, 23.2, 24.1, 24.9].map((y) => ({ p: [axisX(y), y, 0] as V3, rx: Math.max(0.06, envelope(y) * k), rz: Math.max(0.06, envelope(y) * k * DEPTH) })) };
  out.push(top);
  return out;
}

/** 🗿️ The 1:500 crown: four flame-shaped tiers, each bound to its stem part so that the column bends with the stem; every tier but the top ends in a pointed flame that rises 2.5 m into the tier above (the foot of the upper tier is drawn in 7 %), and fourteen vertical flame swells lie on the skin, each bound to the foliage bone of its height and side. */
const TIER500: [string, number[]][] = [
  ["trunk-1", [1.75, 2.4, 3.4, 5, 6.1]],
  ["trunk-2", [7, 8.5, 10, 12.1]],
  ["trunk-3", [13.2, 14.5, 16, 18.1]],
  ["trunk-4", [19.5, 21, 22, 23.8, 25.1, 25.7, 25.96]],
];
const FLAME_TOP: [number, number][] = [[0.6, 0.9], [1.3, 0.7], [1.9, 0.43], [2.3, 0.19], [2.5, 0.03]];
function crown500(): { cores: Core[]; lobes: Lobe[] } {
  const row = (y: number, k: number): Core["rows"][number] => ({ p: [axisX(y), y, 0] as V3, rx: Math.max(0.04, envelope(y) * 0.963 * k), rz: Math.max(0.04, envelope(y) * 0.963 * 1.054 * k) });
  const tiers: Core[] = TIER500.map(([boneId, ys], i) => {
    const foot = F[boneId]!.o[1];
    const top = i + 1 < TIER500.length ? F[TIER500[i + 1]![0]]!.o[1] : 0;
    const rows = [...(i > 0 ? [row(foot - 0.45, 0.93), row(foot, 1)] : []), ...ys.map((y) => row(y, 1)), ...(top ? FLAME_TOP.map(([dy, k]) => row(top + dy, k)) : [])];
    return { kind: "core", id: `crown-${i + 1}`, bone: boneId, material: LEAF, tone: TONE.core, rows };
  });
  const swells: [number, number, number][] = [[3.6, 40, 1.8], [4.2, 160, 1.9], [4.6, 280, 1.9], [7.0, 0, 2.2], [7.6, 90, 2.2], [8.0, 180, 2.2], [8.6, 270, 2.2], [11.6, 30, 2.1], [12.2, 150, 2.1], [12.8, 270, 2.1], [16.2, 100, 1.9], [16.8, 220, 1.9], [17.4, 340, 1.9], [20.8, 60, 1.5]];
  const lobes: Lobe[] = swells.map(([y, az, h], i) => {
    const R = (1.014 * envelope(y)) / Math.hypot(Math.cos(az * D2R), Math.sin(az * D2R) / 1.054);
    const rad: V3 = [Math.cos(az * D2R), 0, Math.sin(az * D2R)];
    const half = Math.min(0.75, R * 0.42);
    const c: V3 = [axisX(y) + rad[0] * (R - half), y, rad[2] * (R - half)];
    return { kind: "lobe", id: `crown-swell-${i + 1}`, bone: bandOf(y, c[0]), material: LEAF, tone: TONE.leafL, group: "swell", pad: "crown", cluster: 0, size: "core", c, axes: [rad, [0, 1, 0], [-rad[2], 0, rad[0]]], radii: [half, h, half * 1.25] };
  });
  return { cores: tiers, lobes };
}

const DECISIONS: Record<number, { en: string; de: string }[]> = {
  50: [
    { en: "Body at 1:50: the buttressed, fluted stem foot with seven buttress ridges on the root flare; the straight stem in four parts tapering from 0.63 m over the flare to a 3 cm leader tip, with fifteen longitudinal bark ridges (texture) and three burls; 53 steeply ascending branches round the stem (the eight rig branches in the rig plane, the others on a golden-angle spiral) that leave at 45 to 55 degrees and bend up to 72 to 80 degrees above the horizontal, each with a pointed flame at its end, two side flames fanned 34 degrees and an inner flame; the crown skin is rows of overlapping pointed flames (1.3 m long, 0.33 m thick, smaller towards the top) every 0.78 m, each row turned half a step, laid on a narrow spindle (widest 4.2 m at 5 to 9 m, pointed top at 25.96 m) with shallow vertical sheaves; a ring of outward-leaning flames rounds the crown base at 2 m, four flames and an apex flame form the spire; behind the flames an inner leaf mass per tier half, so gaps show leaves, not sky.", de: "Körper bei 1:50: der gerillte Brettwurzelfuß mit sieben Wurzelanläufen auf dem Anlauf; der gerade Stamm in vier Teilen, verjüngt von 0,63 m über dem Anlauf auf eine 3 cm starke Gipfelspitze, mit fünfzehn Längsrippen der Borke (Textur) und drei Maserknollen; 53 steil aufsteigende Äste rund um den Stamm (die acht Rig-Äste in der Rig-Ebene, die übrigen auf einer Spirale im goldenen Winkel), die mit 45 bis 55 Grad abgehen und sich auf 72 bis 80 Grad über der Waagrechten aufrichten, jeder mit einer spitzen Laubflamme am Ende, zwei um 34 Grad gefächerten Seitenflammen und einer inneren Flamme; die Kronenhaut sind Reihen sich überlappender spitzer Flammen (1,3 m lang, 0,33 m dick, nach oben kleiner) alle 0,78 m, jede Reihe um einen halben Schritt gedreht, auf einer schmalen Spindel (größte Breite 4,2 m in 5 bis 9 m Höhe, spitzer Wipfel bei 25,96 m) mit flachen senkrechten Garben; ein Kranz nach außen geneigter Flammen rundet den Kronenansatz bei 2 m, vier Flammen und die Gipfelflamme bilden den Wipfel; hinter den Flammen je Stufenhälfte eine innere Laubmasse, damit Lücken Laub und keinen Himmel zeigen." },
    { en: "Thermal at 1:50: stem bark is the root flare, buttresses, stem parts and burls, branch bark all 53 branches, crown leaves every flame and inner mass, all by reference to the body (the bark ridges are texture and add no area). Nine transpiration outlets are lens patches 2 cm proud on the upper outer face of the highest outer flame of each foliage bone.", de: "Thermik bei 1:50: Stammrinde sind Wurzelanlauf, Wurzelanläufe, Stammteile und Maserknollen, Astrinde alle 53 Äste, Kronenblätter jede Flamme und innere Masse, alles als Verweis auf den Körper (die Borkenrippen sind Textur und zählen keine Fläche). Neun Transpirationsauslässe sind 2 cm vorstehende Linsenflächen auf der oberen Außenseite der höchsten äußeren Flamme jedes Laubknochens." },
    { en: "Structure at 1:50: the wood under the bark along the real axes: root flare, the four stem parts, the eight rig branches up to their foliage joint and the branch wood that carries each flame group along its foliage bone, the leader top that carries the spire, and the 45 unboned branches as members of their stem part; joints at the stem joints and at every rig branch collar and foliage joint; the fixed root plate under the flare; stem masses as bands, branch masses inside their wood and the nine flame masses at their declared centres; no bark, no leaves.", de: "Tragwerk bei 1:50: das Holz unter der Rinde entlang der wirklichen Achsen: Wurzelanlauf, die vier Stammteile, die acht Rig-Äste bis zu ihrem Laubgelenk und das Astholz, das jede Flammengruppe entlang ihres Laubknochens trägt, die Gipfelspitze, die den Wipfel trägt, und die 45 Äste ohne eigenen Knochen als Stäbe ihres Stammteils; Gelenke an den Stammgelenken und an jedem Astkragen und Laubgelenk der Rig-Äste; der eingespannte Wurzelteller unter dem Anlauf; Stammmassen als Bänder, Astmassen in ihrem Holz und die neun Flammenmassen in ihren angegebenen Schwerpunkten; keine Rinde, kein Laub." },
  ],
  100: [
    { en: "Body at 1:100: the flare with five buttresses and the four stem parts with three to five sections, without bark ridges and burls; 30 ascending branches (the eight rig branches and 22 on the spiral), thicker than at 1:50 because each stands for about two, each with a tip flame, two side flames and, below 19 m, an inner flame; in the crown skin every third flame of a row is merged into its two neighbours, which become 30 % thicker and 10 % longer and keep their places, so the column keeps the 1:50 outline with coarser scallops; the spire is three flames and the apex flame; inner leaf masses per tier half.", de: "Körper bei 1:100: der Anlauf mit fünf Wurzelanläufen und die vier Stammteile mit drei bis fünf Schnitten, ohne Borkenrippen und Maserknollen; 30 aufsteigende Äste (die acht Rig-Äste und 22 auf der Spirale), dicker als bei 1:50, da jeder für etwa zwei steht, jeder mit einer Endflamme, zwei Seitenflammen und unter 19 m einer inneren Flamme; in der Kronenhaut geht jede dritte Flamme einer Reihe in ihren zwei Nachbarn auf, die 30 % dicker und 10 % länger werden und ihren Platz behalten, sodass die Säule den 1:50-Umriss mit gröberen Bögen behält; der Wipfel aus drei Flammen und der Gipfelflamme; innere Laubmassen je Stufenhälfte." },
    { en: "Thermal at 1:100: the three regions by reference to this scale's body; five transpiration outlets on flames of five foliage bones.", de: "Thermik bei 1:100: die drei Regionen als Verweis auf den Körper dieses Maßstabs; fünf Transpirationsauslässe auf Flammen von fünf Laubknochen." },
    { en: "Structure at 1:100: the wood of flare, stem parts, rig branches, flame carriers and the 22 spiral branches with simplified taper; joints at the stem joints and at the rig branch collars and foliage joints; the fixed root plate; stem, branch and flame masses.", de: "Tragwerk bei 1:100: das Holz von Anlauf, Stammteilen, Rig-Ästen, Flammenträgern und den 22 Spiralästen mit vereinfachter Verjüngung; Gelenke an den Stammgelenken und an den Astkragen und Laubgelenken der Rig-Äste; der eingespannte Wurzelteller; Stamm-, Ast- und Flammenmassen." },
  ],
  200: [
    { en: "Body at 1:200: the stem with its flare in four parts of two to four sections; sixteen ascending branches (eight rig branches, eight on the spiral) of three sections, each standing for about three and drawn thicker, with a broad tip flame and, below 21 m, two side flames; in the crown skin every second flame of the 1:100 rows is merged into its neighbour, which becomes broader and longer; the spire two flames and the apex flame; inner leaf masses per tier half. Buttresses, ridges, burls and inner flames are merged into the flare and the masses.", de: "Körper bei 1:200: der Stamm mit Anlauf in vier Teilen aus zwei bis vier Schnitten; sechzehn aufsteigende Äste (acht Rig-Äste, acht auf der Spirale) aus drei Schnitten, jeder steht für etwa drei und ist dicker gezeichnet, mit einer breiten Endflamme und unter 21 m zwei Seitenflammen; in der Kronenhaut geht jede zweite Flamme der 1:100-Reihen in ihrem Nachbarn auf, der breiter und länger wird; der Wipfel aus zwei Flammen und der Gipfelflamme; innere Laubmassen je Stufenhälfte. Wurzelanläufe, Rippen, Maserknollen und innere Flammen gehen im Anlauf und in den Massen auf." },
    { en: "Thermal at 1:200: bark and leaves by reference to the body; three transpiration outlets.", de: "Thermik bei 1:200: Rinde und Laub als Verweis auf den Körper; drei Transpirationsauslässe." },
    { en: "Structure at 1:200: the wood of the root flare, the four stem parts, the eight rig branches and their flame carriers, the stem joints, the fixed root plate and the stem, branch and flame masses.", de: "Tragwerk bei 1:200: das Holz des Wurzelanlaufs, der vier Stammteile, der acht Rig-Äste und ihrer Flammenträger, die Stammgelenke, der eingespannte Wurzelteller sowie die Stamm-, Ast- und Flammenmassen." },
  ],
  500: [
    { en: "Body at 1:500: the stem with its flare in four plain lofts, the eight rig branches as slender rods of their true thickness (the drawing draws them as lines), and the crown as four flame-shaped tiers, each bound to its stem part so that the column bends with the stem in wind; each tier ends in a pointed flame that rises into the tier above, and fourteen vertical flame swells lie on the skin, each carried by the foliage bone of its height and side, so the column keeps its narrow flame outline, widest in the lower third and pointed at the top.", de: "Körper bei 1:500: der Stamm mit Anlauf in vier einfachen Lofts, die acht Rig-Äste als schlanke Stäbe in wahrer Stärke (die Zeichnung zeigt sie als Linien) und die Krone als vier flammenförmige Stufen, jede an ihren Stammteil gebunden, sodass sich die Säule im Wind mit dem Stamm biegt; jede Stufe endet in einer spitzen Flamme, die in die nächste Stufe aufsteigt, und vierzehn senkrechte Flammenwülste liegen auf der Haut, jeder vom Laubknochen seiner Höhe und Seite getragen, sodass die Säule ihren schmalen, flammenförmigen Umriss behält, am breitesten im unteren Drittel und oben spitz." },
    { en: "Thermal at 1:500: stem bark, branch bark and the crown by reference to the body; two transpiration outlets on the crown.", de: "Thermik bei 1:500: Stammrinde, Astrinde und Krone als Verweis auf den Körper; zwei Transpirationsauslässe auf der Krone." },
    { en: "Structure at 1:500: the wood of the four stem parts, the eight rig branches and their flame carriers as slender members, the fixed root plate and the stem and flame masses; this list is already minimal.", de: "Tragwerk bei 1:500: das Holz der vier Stammteile, der acht Rig-Äste und ihrer Flammenträger als schlanke Stäbe, der eingespannte Wurzelteller sowie die Stamm- und Flammenmassen; diese Liste ist bereits minimal." },
  ],
};

function model(scale: number): Model {
  FIT = FITS[scale]!;
  const wood: Tube[] = [];
  const tufts: Tuft[] = [];
  const burls: Lobe[] = [];
  for (const [id, rows] of Object.entries(STEM[scale]!)) wood.push(stemTube(id, id, rows));
  (BUTTRESS[scale] ?? []).forEach((b, i) => wood.push(buttress(scale, i, b)));
  if (scale === 50) {
    RIDGES.forEach((r, i) => wood.push(ridge(r, i)));
    BURLS.forEach(([y, az, out, up, across], i) => {
      const rad: V3 = [Math.cos(az * D2R), 0, Math.sin(az * D2R)];
      const R = stemR(50, y);
      burls.push({ kind: "lobe", id: `burl-${i + 1}`, bone: "trunk-1", material: BARK, tone: TONE.ridge, group: "burl", pad: "stem", cluster: 0, size: "rim", c: [axisX(y) + rad[0] * (R + out * 0.25), y, rad[2] * (R + out * 0.25)], axes: [rad, [0, 1, 0], [-rad[2], 0, rad[0]]], radii: [out, up, across] });
    });
  }
  let cs: Core[] = [];
  let lobes: Lobe[] = [];
  if (scale === 500) {
    const c = crown500();
    cs = c.cores;
    lobes = c.lobes;
    for (const rig of ["br-l1", "br-r1", "br-l2", "br-r2", "br-l3", "br-r3", "br-l4", "br-r4"]) {
      const b = bone(rig);
      const f = F[rig]!;
      wood.push({ kind: "tube", id: rig, bone: rig, material: BARK, tone: TONE.branch, group: "rig", shape: rig, points: [{ p: f.o, r: 0.05 }, { p: toWorld(f, [b.lengthM * 0.5, 0, 0]), r: 0.045 }, { p: toWorld(f, [b.lengthM, 0, 0]), r: 0.04 }], ...(b.lengthM >= 20 * 0.1 ? { flags: { slender: true } } : {}) });
    }
  } else {
    BRANCHES[scale]!.forEach((row, n) => {
      const g = grow(scale, row, n + 1);
      wood.push(g.wood);
      for (const t of g.tufts) tufts.push(t);
    });
    for (const t of skin(scale)) if (t.points[t.points.length - 1]!.p[1] < 25.7) tufts.push(t);
    for (const t of spire(scale)) tufts.push(t);
    cs = cores(scale);
  }
  tufts.forEach((t, i) => {
    const b = t.bone || bandOf(t.c[1], t.c[0]);
    t.bone = b;
    t.band = b;
    t.shape = b;
    t.id = `${b}-f${String(i + 1).padStart(3, "0")}`;
    t.tone = b === "fol-top" ? TONE.top : b.startsWith("fol-l") ? TONE.leafL : TONE.leafR;
    const rel = Math.atan2(t.c[2], t.c[0] - axisX(t.c[1])) / D2R;
    const azRel = b.startsWith("fol-l") ? ((rel + 360) % 360) - 180 : rel;
    t.cluster = azRel < -30 ? 2 : azRel > 30 ? 1 : 0;
  });
  return { scale, wood, tufts, cores: cs, burls, lobes, decisions: DECISIONS[scale]! };
}

/** 🦴️ Rig root fix: the branch bones end where the modelled 1:50 branch ends (the upper tiers of the old rig reached up to 0.6 m out of the narrowing spire), the flame bones end 0.2 m into the tip flame of their branch; branch centres of mass at mid length, flame masses and wind loads at the flame bone tip. */
const RIG_BEFORE: Record<string, [number, number]> = {};
function fitRig(m: Model): void {
  for (const t of m.wood.filter((x) => x.group === "rig")) {
    const b = bone(t.id);
    const fol = bone(t.id.replace("br-", "fol-"));
    const chord = Math.round(len(sub(t.points[t.points.length - 1]!.p, F[t.id]!.o)) * 100) / 100;
    RIG_BEFORE[b.id] = [b.lengthM, chord];
    b.lengthM = chord;
    b.mass.comX = Math.round(chord * 50) / 100;
    const fl = Math.round(Math.min(fol.lengthM, chord - fol.x + 0.2) * 100) / 100;
    RIG_BEFORE[fol.id] = [fol.lengthM, fl];
    fol.lengthM = fl;
    fol.mass.comX = fl;
    for (const p of asset.poses) for (const l of p.loads ?? []) if (l.bone === fol.id) l.x = fl;
  }
}
const m50 = model(50);
fitRig(m50);
export const models = [m50, model(100), model(200), model(500)];

const AREAS: Record<string, number> = { "branch-bark": 15 };
for (const region of asset.thermal.regions) if (AREAS[region.id] !== undefined) region.areaM2 = AREAS[region.id];

/** 💨️ Poses and clips: the fastigiate crown bends with its stem; the flames deflect a little on their branches instead of turning up to 15 degrees about joints near the stem axis (which tore the column apart). Branch and flame offsets are scaled by 0.3, stem offsets kept. */
const SWAY = 0.3;
const calmBones = (bones: Json): void => {
  for (const [id, off] of Object.entries(bones ?? {}) as [string, Json][]) if ((id.startsWith("br-") || id.startsWith("fol-")) && typeof off.angleDeg === "number") off.angleDeg = Math.round(off.angleDeg * SWAY * 1000) / 1000;
};
const sourceAsset: Json = JSON.parse(readFileSync(entry.path, "utf8")).asset;
const swayed = JSON.stringify(sourceAsset.poses).includes('"fol-l1":{"angleDeg":-10}');
if (swayed) {
  for (const p of asset.poses) calmBones(p.bones);
  for (const c of asset.clips) for (const k of c.keys ?? []) calmBones(k.bones);
}

const anchorsOut: Record<string, { bone: string; local: V3 }> = {};
const barkOf = (t: Tube, r: number): number => (t.group === "stem" ? 0.01 + 0.045 * r : 0.003 + 0.05 * r);
const regionOf = (group: string): string => (group === "stem" || group === "burl" ? "stem-bark" : group === "rig" || group === "branch" ? "branch-bark" : "leaf-crown");

type Outlet = [anchor: string | null, bone: string];
const OUTLETS: Record<number, Outlet[]> = {
  50: [["tp-crown-1", "fol-l1"], ["tp-crown-2", "fol-r1"], ["tp-crown-3", "fol-l2"], ["tp-crown-4", "fol-r2"], ["tp-crown-5", "fol-l3"], ["tp-crown-6", "fol-r3"], ["tp-crown-7", "fol-l4"], ["tp-crown-8", "fol-r4"], ["tp-crown-9", "fol-top"]],
  100: [[null, "fol-l1"], [null, "fol-r2"], [null, "fol-l3"], [null, "fol-r4"], [null, "fol-top"]],
  200: [[null, "fol-l1"], [null, "fol-r3"], [null, "fol-top"]],
  500: [[null, "trunk-2"], [null, "trunk-2"]],
};

/** 🌬️ The outlet patch of a foliage part: on the upper outer face of the highest outer flame of the bone (on the crown swell at 1:500). */
function outlet(m: Model, boneId: string, i: number): { c: V3; n: V3; rho: number } {
  if (m.scale === 500) {
    const l = m.lobes[i === 0 ? 7 : 10]!;
    const n = unit(add(l.axes[0], [0, 0.35, 0]));
    const q = Math.hypot(...l.axes.map((a, k) => dot3(n, a) / l.radii[k]!));
    return { c: add(l.c, mul(n, 1 / q)), n, rho: 0.45 };
  }
  const mine = m.tufts.filter((t) => t.bone === boneId && t.role !== "inner");
  const score = (t: Tuft): number => t.c[1] * 0.2 + Math.hypot(t.c[0] - axisX(t.c[1]), t.c[2]);
  const t = [...mine].sort((a, b) => score(b) - score(a))[0]!;
  const k = 4;
  const s = t.points[k]!;
  const out = unit([s.p[0] - axisX(s.p[1]), 0, s.p[2]]);
  const want = unit(add(out, [0, 0.6, 0]));
  const n = unit(sub(want, mul(t.dir, dot3(want, t.dir))));
  return { c: add(s.p, mul(n, s.r)), n, rho: Math.min(0.16, s.r * 0.45) * (m.scale === 50 ? 1 : 1.3) };
}

function volumeOf(m: Model): Json {
  const s = (p: Tube | Lobe): Json => solidOf(p, F);
  const feature = (0.15 * m.scale) / 1000;
  const tubeSolid = (t: Tube): Json => {
    const flags = { ...(t.flags ?? {}) };
    const role = flags.role;
    delete flags.role;
    return { ...s({ ...t, flags }), ...(role ? { role } : {}) };
  };
  const body: Json[] = [...m.wood.map(tubeSolid), ...m.burls.map((b) => ({ ...s(b), role: "detail" })), ...m.cores.map((c) => coreSolid(c, F)), ...m.lobes.map(s), ...m.tufts.map(s)];
  const thermal: Json[] = [
    ...m.wood.filter((t) => t.group !== "ridge").map((t) => ({ id: `th-${t.id}`, of: t.id, region: regionOf(t.group) })),
    ...m.burls.map((b) => ({ id: `th-${b.id}`, of: b.id, region: "stem-bark" })),
    ...m.cores.map((c) => ({ id: `th-${c.id}`, of: c.id, region: "leaf-crown" })),
    ...m.lobes.map((l) => ({ id: `th-${l.id}`, of: l.id, region: "leaf-crown" })),
    ...m.tufts.map((t) => ({ id: `th-${t.id}`, of: t.id, region: "leaf-crown" })),
  ];
  OUTLETS[m.scale]!.forEach(([anchor, rigBone], i) => {
    const boneId = m.scale === 500 ? m.lobes[i === 0 ? 7 : 10]!.bone : rigBone;
    const { c, n, rho } = outlet(m, boneId, i);
    const side = unit(Math.abs(n[1]) > 0.98 ? [n[1], -n[0], 0] : [n[2], 0, -n[0]]);
    const third: V3 = [side[1] * n[2] - side[2] * n[1], side[2] * n[0] - side[0] * n[2], side[0] * n[1] - side[1] * n[0]];
    const h = Math.max(0.05, feature * 0.6);
    const lens: Lobe = { kind: "lobe", id: `th-${anchor ?? `transpiration-crown-${i + 1}`}`, bone: boneId, material: LEAF, tone: 0.5, group: "outlet", pad: "", cluster: 0, size: "rim", c: sub(c, mul(n, h - 0.02)), axes: [side, n, third], radii: [Math.max(rho, feature), h, Math.max(rho, feature)] };
    thermal.push({ ...s(lens), role: "detail", region: "leaf-crown", tags: ["transpiration-crown"] });
    if (anchor) anchorsOut[anchor] = { bone: boneId, local: toLocal(F[boneId]!, add(c, mul(n, 0.04))) };
  });
  const structure: Json[] = [];
  const woodOf = (t: Tube, id: string, member: string, from = 0, to = 1): Json => {
    const pts = t.points;
    const pick = pts.filter((_, i) => i / (pts.length - 1) >= from - 1e-9 && i / (pts.length - 1) <= to + 1e-9);
    const ends = [...(from > 0 ? [{ p: along(t, from), r: pts[Math.round(from * (pts.length - 1))]!.r }] : []), ...pick, ...(to < 1 ? [{ p: along(t, to), r: pts[Math.round(to * (pts.length - 1))]!.r }] : [])];
    const kept = ends.filter((q, i) => i === 0 || len(sub(q.p, ends[i - 1]!.p)) > 0.005);
    const pathLen = kept.slice(1).reduce((a, q, i) => a + len(sub(q.p, kept[i]!.p)), 0);
    const radius = (q: { r: number }): number => (m.scale === 500 ? Math.max(q.r * 0.8, feature * 0.52) : Math.max(q.r - barkOf(t, q.r), feature * 0.55));
    const slim = !!t.flags?.slender && pathLen >= 20 * 2 * Math.max(...kept.map(radius)) * 1.02;
    return { ...s({ ...t, id, flags: slim ? { slender: true } : undefined, material: WOOD, tone: 0.4, points: kept.map((q) => ({ p: q.p, r: radius(q) })) } as Tube), member, part: "member" };
  };
  for (const t of m.wood.filter((x) => x.group === "stem" && !x.id.startsWith("buttress"))) structure.push(woodOf(t, `member-${t.id}`, t.bone));
  if (m.scale === 500) structure.unshift({ ...s({ ...stemTube("root-plate", "root-plate", [[0, 0.5], [0.2, 0.32], [0.42, 0.29]]), id: "member-root-plate", material: WOOD, tone: 0.4 }), member: "root-plate", part: "member" });
  for (const t of m.wood.filter((x) => x.group === "rig")) {
    const fol = t.id.replace("br-", "fol-");
    const L = t.points.slice(1).reduce((sum, q, i) => sum + len(sub(q.p, t.points[i]!.p)), 0);
    const cut = Math.min(0.5, (bone(fol).x + 0.12) / L);
    structure.push(woodOf(t, `member-${t.id}`, t.id, 0, cut));
    structure.push(woodOf({ ...t, bone: fol }, `member-${fol}`, fol, Math.max(0, cut - 0.08), 1));
  }
  if (m.scale <= 100) for (const t of m.wood.filter((x) => x.group === "branch")) structure.push(woodOf(t, `member-${t.id}`, t.bone));
  const topWood = m.wood.find((x) => x.id === "trunk-4")!;
  const topEnd = Math.min(24.2, topWood.points[topWood.points.length - 1]!.p[1] - 0.15);
  const topRows = [20.7, (20.7 + topEnd) / 2, topEnd].map((y) => ({ p: [axisX(y), y, 0] as V3, r: Math.max(stemR(m.scale, y) - barkOf(topWood, stemR(m.scale, y)), feature * 0.55) }));
  structure.push({ ...s({ ...topWood, id: "member-fol-top", bone: "fol-top", material: WOOD, tone: 0.4, flags: undefined, points: topRows }), member: "fol-top", part: "member" });
  for (const id of ["trunk-1", "trunk-2", "trunk-3", "trunk-4"]) {
    if (m.scale === 500) continue;
    const f = F[id]!;
    const r = (stemR(m.scale, f.o[1]) - barkOf({ group: "stem" } as Tube, stemR(m.scale, f.o[1]))) * 1.06;
    structure.push({ id: `joint-${id}`, kind: "ellipsoid", bone: id, material: WOOD, role: "detail", tone: 0.35, radii: [r5(r * 0.8), r5(r), r5(r)], transform: { translation: [0, 0, 0] }, member: id, part: "joint" });
  }
  if (m.scale <= 100) for (const t of m.wood.filter((x) => x.group === "rig")) {
    const r0 = Math.max(t.points[1]!.r - barkOf(t, t.points[1]!.r), feature * 0.6) * 1.05;
    structure.push({ id: `joint-${t.id}`, kind: "ellipsoid", bone: t.id, material: WOOD, role: "detail", tone: 0.35, radii: [r5(r0), r5(r0), r5(r0)], transform: { translation: toLocal(F[t.id]!, t.points[1]!.p).map(r5) }, member: t.id, part: "joint" });
    const fol = t.id.replace("br-", "fol-");
    const fo = F[fol]!.o;
    const near = [...t.points].sort((a, b) => len(sub(a.p, fo)) - len(sub(b.p, fo)))[0]!;
    const rj = Math.max(near.r - barkOf(t, near.r), feature * 0.6) * 1.05;
    structure.push({ id: `joint-${fol}`, kind: "ellipsoid", bone: fol, material: WOOD, role: "detail", tone: 0.35, radii: [r5(rj), r5(rj), r5(rj)], transform: { translation: toLocal(F[fol]!, near.p).map(r5) }, member: fol, part: "joint" });
  }
  const plate = Math.max(0.04, feature);
  structure.push({ id: "support-base-support", kind: "revolve", bone: "root-plate", material: WOOD, role: "detail", tone: 0.25, profile: [[r5(-plate), 0.7], [0, 0.7]], axis: [1, 0, 0], origin: [0, 0, 0], member: "root-plate", part: "support", anchor: "base-support" });
  for (const id of ["root-plate", "trunk-1", "trunk-2", "trunk-3", "trunk-4"]) {
    const b = bone(id);
    const y = F[id]!.o[1] + b.mass.comX;
    const r = stemR(m.scale, y) - barkOf({ group: "stem" } as Tube, stemR(m.scale, y)) - 0.004;
    const half = Math.max(Math.min(0.6, b.lengthM * 0.12), feature * 0.6);
    structure.push({ id: `mass-${id}`, kind: "ellipsoid", bone: id, material: WOOD, role: "detail", tone: 0.3, radii: [r5(half), r5(Math.max(r, feature * 0.6)), r5(Math.max(r, feature * 0.6))], transform: { translation: [b.mass.comX, b.mass.comY, 0] }, member: id, part: "mass" });
  }
  for (const b of asset.rig.bones.filter((x: Json) => x.id.startsWith("fol-"))) {
    const r = Math.max(feature * 0.6, 0.12);
    structure.push({ id: `mass-${b.id}`, kind: "ellipsoid", bone: b.id, material: LEAF, role: "detail", tone: 0.3, radii: [r5(r), r5(r), r5(r)], transform: { translation: [b.mass.comX, b.mass.comY, 0] }, member: b.id, part: "mass" });
  }
  for (const t of m.wood.filter((x) => x.group === "rig")) {
    const b = bone(t.id);
    const p = toWorld(F[t.id]!, [b.mass.comX, b.mass.comY, 0]);
    const near = [...t.points].sort((a, c) => len(sub(a.p, p)) - len(sub(c.p, p)))[0]!;
    const r = Math.max(near.r - barkOf(t, near.r), feature * 0.6);
    structure.push({ id: `mass-${t.id}`, kind: "ellipsoid", bone: t.id, material: WOOD, role: "detail", tone: 0.3, radii: [r5(Math.max(0.2, feature)), r5(r), r5(r)], transform: { translation: toLocal(F[t.id]!, near.p).map(r5) }, member: t.id, part: "mass" });
  }
  return { status: "authored", denominator: m.scale, body, thermal, structure, decisions: m.decisions };
}

/** 📦️ The world extent of a model's body: [min, max] per axis. */
function extent(m: Model): [V3, V3] {
  const lo: V3 = [Infinity, Infinity, Infinity], hi: V3 = [-Infinity, -Infinity, -Infinity];
  for (const t of [...m.wood.filter((x) => !x.flags?.texture), ...m.tufts]) for (const q of t.points) for (let k = 0; k < 3; k++) {
    lo[k] = Math.min(lo[k]!, q.p[k]! - q.r);
    hi[k] = Math.max(hi[k]!, q.p[k]! + q.r);
  }
  for (const l of m.lobes) for (let k = 0; k < 3; k++) {
    const e = Math.hypot(...l.axes.map((a, i) => a[k]! * l.radii[i]!));
    lo[k] = Math.min(lo[k]!, l.c[k]! - e);
    hi[k] = Math.max(hi[k]!, l.c[k]! + e);
  }
  for (const c of m.cores) for (const s of c.rows) {
    lo[0] = Math.min(lo[0], s.p[0] - s.rx); hi[0] = Math.max(hi[0], s.p[0] + s.rx);
    lo[2] = Math.min(lo[2], s.p[2] - s.rz); hi[2] = Math.max(hi[2], s.p[2] + s.rz);
    lo[1] = Math.min(lo[1], s.p[1]); hi[1] = Math.max(hi[1], s.p[1]);
  }
  return [lo, hi];
}

if (import.meta.main) {
  for (const m of models) {
    const [lo, hi] = extent(m);
    console.log(`[DEBUG] 1:${m.scale} extent x ${lo[0].toFixed(3)} ${hi[0].toFixed(3)} (${(hi[0] - lo[0]).toFixed(3)} / ${asset.dimensions.widthM}) y ${lo[1].toFixed(3)} ${hi[1].toFixed(3)} (/ ${asset.dimensions.heightM}) z ${lo[2].toFixed(3)} ${hi[2].toFixed(3)} (${(hi[2] - lo[2]).toFixed(3)} / ${asset.dimensions.depthM}) tufts ${m.tufts.length} wood ${m.wood.length}`);
    const branchArea = m.wood.filter((t) => t.group === "rig" || t.group === "branch").reduce((sum, t) => sum + t.points.slice(1).reduce((a, q, i) => a + Math.PI * (q.r + t.points[i]!.r) * len(sub(q.p, t.points[i]!.p)), 0), 0);
    const stemArea = m.wood.filter((t) => t.group === "stem").reduce((sum, t) => sum + t.points.slice(1).reduce((a, q, i) => a + Math.PI * (q.r + t.points[i]!.r) * len(sub(q.p, t.points[i]!.p)), 0), 0);
    console.log(`[DEBUG] 1:${m.scale} lateral wood area: branches ${branchArea.toFixed(2)} m2, stem parts ${stemArea.toFixed(2)} m2`);
    for (const t of m.wood.filter((x) => x.group === "rig" || x.group === "branch")) {
      const tip = t.points[t.points.length - 1]!.p;
      const inside = m.tufts.some((u) => u.points.some((q) => len(sub(q.p, tip)) < q.r * 0.9));
      if (!inside) console.log(`[DEBUG] 1:${m.scale} tip of ${t.id} at ${tip.map((v) => v.toFixed(2)).join(" ")} is outside its flames`);
      if (t.group === "rig" && m.scale === 50) console.log(`[DEBUG] 1:50 ${t.id} modelled length ${t.points.slice(1).reduce((a, q, i) => a + len(sub(q.p, t.points[i]!.p)), 0).toFixed(2)} m (bone ${bone(t.id).lengthM})`);
    }
  }
  const volumes: Json = {};
  for (const m of models) volumes[String(m.scale)] = volumeOf(m);
  writeFileSync(join(SOURCES, "vegetation", "lombardy-poplar.json"), JSON.stringify({ volumes }) + "\n");
  for (const a of asset.anchors) {
    const moved = anchorsOut[a.id];
    if (!moved) continue;
    if (moved.bone !== a.bone) throw new Error(`anchor ${a.id} would change its bone from ${a.bone} to ${moved.bone}`);
    a.x = r4(moved.local[0]);
    a.y = r4(moved.local[1]);
    a.z = r4(moved.local[2]);
  }
  asset.volumes = volumes;
  const report = draw(asset, F, models.map((m) => ({ scale: m.scale, wood: m.wood, tufts: m.tufts, cores: m.cores, burls: m.burls, lobes: m.lobes })), axisX);
  const out = join(GENERATED, "r3-lombardy-poplar");
  mkdirSync(out, { recursive: true });
  writeFileSync(join(out, "🔣️.json"), JSON.stringify(document, null, 1) + "\n");
  for (const m of models) console.log(`1:${m.scale} body ${volumes[m.scale].body.length} thermal ${volumes[m.scale].thermal.length} structure ${volumes[m.scale].structure.length}`);
  console.log(report.join("\n"));
}
