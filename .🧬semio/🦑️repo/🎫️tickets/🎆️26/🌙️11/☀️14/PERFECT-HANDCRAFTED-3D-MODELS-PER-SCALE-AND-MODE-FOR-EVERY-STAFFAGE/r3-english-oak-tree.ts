/** 🌳️ Round 3 English oak: the hand-stated tree in world metres (x east, y up, z towards the left of the asset) at every authored scale: woody axes as stations `[x, y, z, radius]`, foliage clumps as world-aligned ellipsoid lobes, and the rest frames of the rig with the lateral bone offsets of this round.
 * @see ./round3-brief.md
 * @see ./r3-english-oak-model.ts */
export type V3 = [number, number, number];
export type Station = [number, number, number, number, number?];
export type Tube = { id: string; bone: string; region: "trunk-bark" | "limb-bark"; pts: Station[]; sub: number; slender?: boolean; texture?: boolean };
export type Lobe = { id: string; bone: string; c: V3; r: V3; shade: boolean; main: boolean };
export type Tree = { tubes: Tube[]; lobes: Lobe[] };

/** 📍️ Lateral offsets of the bone origins in the parent frame: the joints of the branches and foliage carriers sit where the limbs really end in depth. */
export const BONE_Z: Record<string, number> = {
  "br-w-low": 1.8, "fol-w-low": 1.0, "br-low-w-up": 0.95,
  "br-e-low": -1.9, "fol-e-low": -0.9, "br-low-e-up": -1.0,
  "br-w-mid": -1.7, "fol-w-mid": -0.9,
  "br-e-mid": 1.7, "fol-e-mid": 0.9,
  "fol-leader-w": 1.4, "fol-leader-e": -1.4,
};

export type Frame = { o: V3; ang: number };
/** 🦴️ Rest frames of the planar chain with cumulative lateral offsets. */
export function restFrames(bones: { id: string; parent: string | null; x: number; y: number; z?: number; angleDeg: number }[]): Record<string, Frame> {
  const out: Record<string, Frame> = {};
  for (const b of bones) {
    if (b.parent === null) {
      out[b.id] = { o: [b.x, b.y, b.z ?? 0], ang: (b.angleDeg * Math.PI) / 180 };
      continue;
    }
    const p = out[b.parent]!;
    const [c, s] = [Math.cos(p.ang), Math.sin(p.ang)];
    out[b.id] = { o: [p.o[0] + b.x * c - b.y * s, p.o[1] + b.x * s + b.y * c, p.o[2] + (b.z ?? 0)], ang: p.ang + (b.angleDeg * Math.PI) / 180 };
  }
  return out;
}
export const toLocal = (f: Frame, p: readonly number[]): V3 => {
  const [c, s] = [Math.cos(f.ang), Math.sin(f.ang)];
  const [dx, dy] = [p[0]! - f.o[0], p[1]! - f.o[1]];
  return [dx * c + dy * s, -dx * s + dy * c, p[2]! - f.o[2]];
};

const LIMBS: Record<string, { limb: Station[]; branch: Station[]; branchBone: string }> = {
  "limb-w-low": {
    limb: [[-0.1, 2.88, 0.04, 0.23, 500], [-0.55, 3.1, 0.18, 0.2, 50], [-1.4, 3.42, 0.5, 0.185, 100], [-2.49, 3.66, 0.95, 0.17, 200], [-3.4, 3.92, 1.4, 0.155, 50], [-4.21, 4.19, 1.8, 0.14, 500]],
    branch: [[-5.2, 4.15, 2.2, 0.1, 100], [-6.4, 4.22, 2.55, 0.075, 500], [-7.61, 4.31, 2.8, 0.05, 200], [-8.0, 4.5, 2.9, 0.035, 50]],
    branchBone: "br-w-low",
  },
  "limb-e-low": {
    limb: [[0.1, 3.18, -0.04, 0.22, 500], [0.6, 3.42, -0.2, 0.19, 50], [1.5, 3.78, -0.55, 0.175, 100], [2.63, 4.16, -1.0, 0.16, 200], [3.6, 4.52, -1.45, 0.145, 50], [4.51, 4.84, -1.9, 0.13, 500]],
    branch: [[5.6, 4.95, -2.3, 0.1, 100], [6.9, 5.15, -2.6, 0.07, 500], [8.08, 5.34, -2.8, 0.045, 200], [8.45, 5.55, -2.9, 0.035, 50]],
    branchBone: "br-e-low",
  },
  "limb-w-mid": {
    limb: [[0.05, 5.95, 0.0, 0.17, 500], [-0.5, 6.55, -0.28, 0.15, 50], [-1.4, 7.5, -0.75, 0.135, 100], [-2.3, 8.45, -1.25, 0.12, 200], [-3.12, 9.25, -1.7, 0.105, 500]],
    branch: [[-4.2, 9.7, -2.1, 0.085, 100], [-5.3, 10.1, -2.4, 0.06, 200], [-6.31, 10.41, -2.6, 0.04, 500]],
    branchBone: "br-w-mid",
  },
  "limb-e-mid": {
    limb: [[0.2, 5.95, 0.1, 0.17, 500], [0.75, 6.65, 0.3, 0.15, 50], [1.6, 7.75, 0.78, 0.135, 100], [2.5, 8.85, 1.27, 0.12, 200], [3.35, 9.83, 1.7, 0.105, 500]],
    branch: [[4.4, 10.4, 2.1, 0.085, 100], [5.5, 10.9, 2.4, 0.06, 200], [6.61, 11.35, 2.6, 0.04, 500]],
    branchBone: "br-e-mid",
  },
};

const SMALL: Tube[] = [
  { id: "br-low-w-up", bone: "br-low-w-up", region: "limb-bark", sub: 3, pts: [[-2.49, 3.66, 0.95, 0.085], [-2.65, 4.6, 2.0, 0.075], [-2.85, 5.8, 3.3, 0.065], [-3.08, 7.01, 4.4, 0.05], [-3.15, 7.5, 4.8, 0.035]] },
  { id: "br-low-e-up", bone: "br-low-e-up", region: "limb-bark", sub: 3, pts: [[2.63, 4.16, -1.0, 0.085], [2.85, 5.1, -2.1, 0.075], [3.15, 6.3, -3.4, 0.065], [3.51, 7.44, -4.5, 0.05], [3.6, 7.95, -4.9, 0.035]] },
  { id: "br-leader-w", bone: "br-leader-w", region: "limb-bark", sub: 3, pts: [[-0.32, 10.37, 0.05, 0.08], [-0.9, 11.2, 0.5, 0.068], [-1.55, 12.15, 1.0, 0.055], [-2.16, 12.99, 1.4, 0.04]] },
  { id: "br-leader-e", bone: "br-leader-e", region: "limb-bark", sub: 3, pts: [[-0.32, 10.37, 0.05, 0.08], [0.3, 11.3, -0.5, 0.068], [1.0, 12.3, -1.0, 0.055], [1.63, 13.16, -1.4, 0.04]] },
];

/** 🌿️ Secondary boughs: one from each scaffold limb to the far side of the plan, where it carries a foliage clump of its own. */
const BOUGHS: Tube[] = [
  { id: "bough-w-low", bone: "limb-w-low", region: "limb-bark", sub: 3, pts: [[-1.4, 3.42, 0.5, 0.072], [-1.75, 3.8, -0.1, 0.068], [-2.3, 4.45, -1.0, 0.06], [-3.2, 5.3, -2.3, 0.047], [-4.1, 6.3, -3.4, 0.03]] },
  { id: "bough-e-low", bone: "limb-e-low", region: "limb-bark", sub: 3, pts: [[1.5, 3.78, -0.55, 0.072], [1.85, 4.2, 0.1, 0.068], [2.45, 4.85, 1.0, 0.06], [3.4, 5.7, 2.4, 0.047], [4.3, 6.65, 3.5, 0.03]] },
  { id: "bough-w-mid", bone: "limb-w-mid", region: "limb-bark", sub: 3, pts: [[-1.4, 7.5, -0.75, 0.068], [-1.65, 8.1, -0.1, 0.064], [-2.05, 8.9, 0.9, 0.056], [-2.7, 9.8, 2.2, 0.043], [-3.2, 10.6, 3.3, 0.03]] },
  { id: "bough-e-mid", bone: "limb-e-mid", region: "limb-bark", sub: 3, pts: [[1.6, 7.75, 0.78, 0.068], [1.85, 8.4, 0.1, 0.064], [2.25, 9.3, -0.9, 0.056], [2.85, 10.3, -2.3, 0.043], [3.3, 11.2, -3.4, 0.03]] },
];

const TRUNK: Record<number, Tube[]> = {
  50: [
    { id: "root-flare", bone: "root-plate", region: "trunk-bark", sub: 2, pts: [[0, 0, 0, 0.66], [0, 0.08, 0, 0.585], [0, 0.18, 0, 0.535], [0, 0.3, 0, 0.5], [0, 0.44, 0, 0.475]] },
    { id: "bole", bone: "trunk", region: "trunk-bark", sub: 3, pts: [[0, 0.38, 0, 0.48], [0.01, 0.9, 0, 0.435], [0.02, 1.3, 0.01, 0.405], [0, 2.0, 0.02, 0.39], [-0.01, 2.7, 0.01, 0.385], [0, 3.1, 0, 0.4], [0.01, 3.45, 0, 0.415]] },
    { id: "trunk-upper", bone: "trunk-upper", region: "trunk-bark", sub: 3, pts: [[0, 3.35, 0, 0.4], [0.03, 3.8, 0.01, 0.345], [0.08, 4.6, 0.03, 0.315], [0.12, 5.4, 0.06, 0.305], [0.14, 6.05, 0.08, 0.33]] },
    { id: "leader", bone: "leader", region: "trunk-bark", sub: 3, pts: [[0.14, 5.95, 0.08, 0.26], [0.1, 6.5, 0.1, 0.215], [-0.02, 7.4, 0.16, 0.2], [-0.14, 8.4, 0.18, 0.185], [-0.25, 9.4, 0.12, 0.17], [-0.32, 10.4, 0.05, 0.165]] },
    { id: "leader-top", bone: "leader-top", region: "trunk-bark", sub: 3, pts: [[-0.32, 10.3, 0.05, 0.14], [-0.26, 10.9, 0.02, 0.105], [-0.12, 11.8, -0.04, 0.09], [0.03, 12.8, -0.08, 0.07], [0.15, 13.74, -0.1, 0.055], [0.14, 14.6, -0.08, 0.035]] },
  ],
  100: [
    { id: "root-flare", bone: "root-plate", region: "trunk-bark", sub: 1, pts: [[0, 0, 0, 0.72], [0, 0.12, 0, 0.58], [0, 0.28, 0, 0.51], [0, 0.44, 0, 0.475]] },
    { id: "bole", bone: "trunk", region: "trunk-bark", sub: 2, pts: [[0, 0.38, 0, 0.48], [0.02, 1.3, 0.01, 0.405], [-0.01, 2.7, 0.01, 0.385], [0.01, 3.45, 0, 0.415]] },
    { id: "trunk-upper", bone: "trunk-upper", region: "trunk-bark", sub: 2, pts: [[0, 3.35, 0, 0.4], [0.07, 4.5, 0.03, 0.32], [0.14, 6.05, 0.08, 0.33]] },
    { id: "leader", bone: "leader", region: "trunk-bark", sub: 2, pts: [[0.14, 5.95, 0.08, 0.26], [-0.02, 7.4, 0.16, 0.2], [-0.25, 9.4, 0.12, 0.17], [-0.3, 10.6, 0.04, 0.13], [-0.05, 12.2, -0.06, 0.08], [0.15, 13.74, -0.1, 0.055]] },
  ],
  200: [
    { id: "root-flare", bone: "root-plate", region: "trunk-bark", sub: 1, pts: [[0, 0, 0, 0.76], [0, 0.16, 0, 0.56], [0, 0.44, 0, 0.475]] },
    { id: "bole", bone: "trunk", region: "trunk-bark", sub: 1, pts: [[0, 0.38, 0, 0.48], [0.01, 1.6, 0.01, 0.4], [0.01, 3.45, 0, 0.41]] },
    { id: "trunk-upper", bone: "trunk-upper", region: "trunk-bark", sub: 1, pts: [[0, 3.35, 0, 0.4], [0.14, 6.05, 0.08, 0.32]] },
    { id: "leader", bone: "leader", region: "trunk-bark", sub: 1, pts: [[0.14, 5.95, 0.08, 0.25], [-0.14, 8.4, 0.18, 0.185], [-0.32, 10.4, 0.05, 0.14], [0.15, 13.74, -0.1, 0.06]] },
  ],
  500: [
    { id: "root-flare", bone: "root-plate", region: "trunk-bark", sub: 1, pts: [[0, 0, 0, 0.8], [0, 0.2, 0, 0.56], [0, 0.44, 0, 0.48]] },
    { id: "bole", bone: "trunk", region: "trunk-bark", sub: 1, pts: [[0, 0.38, 0, 0.48], [0.01, 1.5, 0, 0.41], [0.01, 3.4, 0, 0.41], [0.14, 6.05, 0.08, 0.32]] },
    { id: "leader", bone: "leader", region: "trunk-bark", sub: 1, pts: [[0.14, 5.95, 0.08, 0.25], [-0.3, 10.4, 0.05, 0.14], [0.15, 13.74, -0.1, 0.06]] },
  ],
};

/** 🌱️ Buttress roots: azimuth in plan from +x towards +z, reach from the stem axis. */
const SPURS: [number, number][] = [[8, 1.13], [62, 0.98], [118, 1.04], [172, 1.12], [228, 1.0], [281, 1.08], [334, 0.96]];

/** 🪵️ Bark ridges of the bole at 1:50: `[azimuth, foot height, head height, drift of the azimuth]`, the long plates between the deep fissures of old oak bark, 28 mm wide and 10 mm proud. */
const RIDGES: [number, number, number, number][] = [
  [0, 0.55, 1.5, 4], [22, 1.35, 2.3, -3], [41, 0.6, 1.4, 3], [63, 1.6, 2.6, 5], [84, 0.5, 1.25, -4], [102, 1.45, 2.45, 2], [125, 0.7, 1.6, 4], [147, 1.75, 2.7, -3], [166, 0.55, 1.35, 3],
  [188, 1.5, 2.5, 4], [207, 0.65, 1.55, -3], [229, 1.7, 2.65, 3], [248, 0.5, 1.3, 5], [270, 1.4, 2.4, -4], [291, 0.7, 1.5, 2], [312, 1.65, 2.6, 4], [333, 0.55, 1.4, -3], [351, 1.8, 2.75, 3],
];
const BOLE_R: [number, number][] = [[0.38, 0.48], [0.9, 0.435], [1.3, 0.405], [2.0, 0.39], [2.7, 0.385], [3.1, 0.4]];
const boleRadius = (y: number): number => {
  const i = Math.max(0, Math.min(BOLE_R.length - 2, BOLE_R.findIndex((p) => p[0] > y) - 1));
  const [a, b] = [BOLE_R[i]!, BOLE_R[i + 1]!];
  return a[1] + ((b[1] - a[1]) * (y - a[0])) / (b[0] - a[0]);
};

/** 🪹️ Fine branchlets of the four outer branches at 1:50: start as a fraction of the branch, then the world offset of the tip. */
const TWIGS: Record<string, [number, number, number, number][]> = {
  "br-w-low": [[0.22, -0.5, 1.35, 0.6], [0.42, -0.6, 1.25, -0.9], [0.6, -0.9, 1.15, 0.9], [0.78, -1.0, 1.0, -0.6], [0.92, -0.75, 1.45, 0.3]],
  "br-e-low": [[0.2, 0.45, 1.4, -0.7], [0.4, 0.7, 1.2, 0.8], [0.58, 0.85, 1.25, -0.85], [0.76, 1.05, 0.95, 0.5], [0.9, 0.7, 1.5, -0.3]],
  "br-w-mid": [[0.25, -0.3, 1.5, -0.6], [0.45, -0.85, 1.1, 0.9], [0.62, -0.6, 1.4, -1.0], [0.8, -1.1, 0.9, 0.4], [0.95, -0.9, 1.3, -0.4]],
  "br-e-mid": [[0.24, 0.35, 1.45, 0.7], [0.44, 0.8, 1.15, -0.85], [0.63, 0.65, 1.35, 1.0], [0.8, 1.05, 0.95, -0.45], [0.94, 0.85, 1.35, 0.35]],
};

type Boss = [number, number, number];
type Clump = { id: string; bone: string; shade?: boolean; c: V3; r: V3; bosses: Boss[]; tufts?: [number, number, number, number, number?][] };
/** 🥦️ Foliage clumps: main lobe (centre, semi-axes), bosses `[azimuth, elevation, radius]` seated on its surface (the first one sets the outermost reach of the clump) and tufts `[x, y, z, radius, coarsest scale that still draws it]` that close the foliage around and below a branch tip. Three tiers: six low clumps on the two low limbs, four on the mid limbs, three on the leader, around a shaded inner crown of two masses. */
const CLUMPS: Clump[] = [
  { id: "w-low", bone: "fol-w-low", c: [-6.3, 6.2, 2.8], r: [2.55, 1.95, 3.1], bosses: [[185, 8, 1.22], [130, -12, 1.47], [70, 20, 1.54], [15, -5, 1.33], [300, 12, 1.4], [245, -18, 1.26], [200, 55, 1.4], [40, 62, 1.26], [150, -50, 1.32], [330, -48, 1.26], [230, -45, 1.19]], tufts: [[-7.75, 4.95, 2.8, 0.95, 200], [-6.55, 4.8, 2.5, 0.85, 100], [-5.45, 4.9, 2.15, 0.8, 100]] },
  { id: "wb-low", bone: "fol-w-low", c: [-4.5, 7.0, -3.7], r: [2.3, 1.7, 2.4], bosses: [[205, 6, 1.25], [265, -10, 1.3], [320, 14, 1.2], [150, -8, 1.15], [240, 55, 1.15], [80, 30, 1.05], [300, -45, 1.1]] },
  { id: "w-up", bone: "fol-w-low", c: [-2.7, 8.2, 5.1], r: [2.1, 1.5, 2.0], bosses: [[90, 5, 1.19], [150, 25, 1.17], [30, 18, 1.17], [250, 40, 1.04], [100, 60, 1.04], [200, -40, 1.04]] },
  { id: "e-low", bone: "fol-e-low", c: [6.4, 6.9, -2.8], r: [2.6, 1.95, 3.1], bosses: [[355, 6, 1.26], [300, -14, 1.4], [250, 22, 1.54], [195, -6, 1.26], [60, 10, 1.47], [110, -16, 1.33], [330, 58, 1.33], [150, 60, 1.26], [20, -50, 1.32], [200, -47, 1.26], [100, -45, 1.19]], tufts: [[8.2, 5.95, -2.8, 0.95, 200], [7.0, 5.6, -2.6, 0.85, 100], [5.85, 5.45, -2.3, 0.8, 100]] },
  { id: "ef-low", bone: "fol-e-low", c: [4.7, 7.3, 3.8], r: [2.3, 1.7, 2.4], bosses: [[25, 6, 1.25], [85, -10, 1.3], [140, 14, 1.2], [330, -8, 1.15], [60, 55, 1.15], [260, 30, 1.05], [120, -45, 1.1]] },
  { id: "e-up", bone: "fol-e-low", c: [3.0, 8.6, -5.1], r: [2.1, 1.5, 2.0], bosses: [[270, 4, 1.16], [215, 22, 1.17], [330, 20, 1.1], [80, 42, 1.04], [280, 62, 1.04], [30, -40, 1.04]] },
  { id: "w-mid", bone: "fol-w-mid", c: [-4.7, 11.45, -2.6], r: [2.85, 2.05, 3.2], bosses: [[180, 5, 1.61], [230, -15, 1.47], [285, 15, 1.68], [340, -8, 1.4], [120, 12, 1.54], [60, -20, 1.33], [200, 58, 1.47], [320, 62, 1.33], [150, -48, 1.26], [300, -50, 1.26]] },
  { id: "wf-mid", bone: "fol-w-mid", c: [-3.5, 11.0, 3.7], r: [2.3, 1.7, 2.3], bosses: [[160, 8, 1.3], [95, -8, 1.3], [40, 16, 1.2], [215, -12, 1.15], [120, 58, 1.15], [300, 35, 1.0], [70, -46, 1.1]] },
  { id: "e-mid", bone: "fol-e-mid", c: [5.0, 12.2, 2.6], r: [2.8, 2.05, 3.2], bosses: [[2, 8, 1.61], [55, -12, 1.54], [95, 16, 1.68], [150, -10, 1.4], [300, 14, 1.47], [240, -18, 1.33], [20, 60, 1.4], [130, 64, 1.33], [330, -48, 1.26], [120, -50, 1.26]] },
  { id: "eb-mid", bone: "fol-e-mid", c: [3.6, 11.7, -3.8], r: [2.3, 1.7, 2.3], bosses: [[340, 8, 1.3], [275, -8, 1.3], [220, 16, 1.2], [35, -12, 1.15], [300, 58, 1.15], [120, 35, 1.0], [250, -46, 1.1]] },
  { id: "leader-w", bone: "fol-leader-w", c: [-2.4, 14.9, 1.4], r: [2.3, 1.7, 2.4], bosses: [[175, 10, 1.4], [120, -8, 1.33], [70, 22, 1.4], [250, 5, 1.33], [320, -15, 1.19], [150, 60, 1.26], [20, 30, 1.19]], tufts: [[-2.1, 13.45, 1.3, 0.85, 200]] },
  { id: "leader-e", bone: "fol-leader-e", c: [2.1, 15.0, -1.4], r: [2.3, 1.7, 2.4], bosses: [[5, 10, 1.4], [300, -8, 1.33], [250, 24, 1.4], [70, 6, 1.33], [140, -14, 1.19], [330, 62, 1.26], [200, 30, 1.19]], tufts: [[1.6, 13.6, -1.3, 0.85, 200]] },
  { id: "f-mid", bone: "fol-leader-w", c: [1.2, 10.2, 5.0], r: [2.2, 1.6, 2.1], bosses: [[90, 4, 1.1], [30, 10, 1.2], [150, 12, 1.2], [80, 55, 1.1], [200, 30, 1.0], [340, -30, 1.05], [110, -44, 1.05]] },
  { id: "b-mid", bone: "fol-leader-e", c: [-1.2, 10.4, -5.0], r: [2.2, 1.6, 2.1], bosses: [[270, 4, 1.1], [210, 10, 1.2], [330, 12, 1.2], [260, 55, 1.1], [20, 30, 1.0], [160, -30, 1.05], [290, -44, 1.05]] },
  { id: "top", bone: "fol-top", c: [0.1, 16.3, 0], r: [2.2, 1.65, 2.3], bosses: [[350, 70, 1.2], [40, 12, 1.33], [110, -5, 1.26], [170, 20, 1.33], [235, -8, 1.26], [300, 15, 1.33], [200, 68, 1.12]], tufts: [[0.14, 14.95, -0.08, 0.85, 200]] },
  { id: "core", bone: "fol-core", shade: true, c: [-0.21, 8.9, 0], r: [3.3, 2.0, 3.6], bosses: [[30, 40, 1.82], [120, 45, 1.82], [215, 38, 1.82], [305, 44, 1.82], [0, -10, 1.54], [90, -5, 1.61], [180, -12, 1.54], [270, -6, 1.61]] },
  { id: "inner", bone: "fol-core", shade: true, c: [-0.1, 12.4, 0], r: [2.6, 1.8, 2.4], bosses: [[20, 5, 1.4], [110, -5, 1.4], [200, 5, 1.4], [290, -5, 1.4], [60, 50, 1.3], [240, 50, 1.3]] },
];
/** 🫥️ Bosses of 1:50 that shape no outline of the crown in elevation, front or plan (leaving any of them out moves no outline cell of 30 mm, measured by `r3-english-oak-hidden.ts`): the 1:100 crown leaves them out and keeps every boss that the drawings of that scale can show. */
const HIDDEN_100 = new Set(["b-mid-5", "b-mid-6", "core-1", "core-2", "core-3", "core-4", "core-5", "core-7", "eb-mid-1", "eb-mid-3", "eb-mid-4", "eb-mid-6", "eb-mid-7", "ef-low-1", "ef-low-4", "ef-low-6", "e-low-4", "e-mid-6", "e-mid-10", "e-up-4", "e-up-6", "f-mid-5", "f-mid-6", "inner-2", "inner-4", "leader-e-4", "leader-e-7", "leader-w-4", "leader-w-7", "top-5", "wb-low-3", "wb-low-4", "wb-low-6", "wf-mid-1", "wf-mid-2", "wf-mid-3", "wf-mid-4", "wf-mid-5", "wf-mid-6", "wf-mid-7", "w-low-5", "w-mid-4", "w-mid-6", "w-mid-10", "w-up-4", "w-up-5", "w-up-6"]);
/** 🫧️ Bosses that the 1:200 crown leaves out in addition to {@link HIDDEN_100}: left out one at a time, smallest first, each moves the outline of the remaining crown by less than 0.20 m (`r3-english-oak-hidden.ts 100 0.03 0.2`); the bosses that carry the scallops of the outline stay. */
const MERGED_200 = new Set(["leader-w-5", "leader-e-5", "inner-1", "inner-3", "inner-5", "w-low-11", "wb-low-1", "ef-low-3", "ef-low-7", "e-up-5", "e-mid-4", "inner-6", "w-low-8", "e-low-6", "w-mid-3", "w-low-6", "core-8", "w-low-4", "b-mid-7", "w-low-9", "top-3", "e-low-8", "e-mid-5", "f-mid-7", "wb-low-5", "e-low-11", "ef-low-5", "w-mid-9", "e-mid-9", "wb-low-7", "w-mid-8", "e-mid-8", "core-6", "eb-mid-2", "top-2"]);
/** 🏔️ Bosses and tufts that the 1:500 crown leaves out in addition to {@link HIDDEN_100} and {@link MERGED_200}: left out one at a time, smallest first, each moves the outline of the remaining 1:200 crown by less than 0.40 m (`r3-english-oak-hidden.ts 200 0.03 0.4 tufts`), except the top boss and the outermost bosses of the two low clumps, which set the height and the spread of the tree. The rest stay as one crown on the leader: the seventeen clump lobes in three tiers with the bosses that make the scallops of the outline and the sky windows a 1:500 drawing still shows. */
const MERGED_500 = new Set(["leader-e-tuft-1", "top-tuft-1", "leader-w-tuft-1", "w-low-10", "w-mid-5", "e-low-10", "e-up-3", "leader-w-2", "e-low-2", "w-mid-2", "eb-mid-5", "leader-e-2", "e-low-3", "e-low-9", "w-up-2", "w-up-3", "e-up-2", "f-mid-4", "b-mid-4", "top-6", "w-low-3", "e-low-7", "top-7", "b-mid-1", "f-mid-1", "f-mid-3", "e-low-5", "e-mid-7", "e-mid-2", "b-mid-3", "leader-w-6", "f-mid-2", "b-mid-2", "w-low-2", "leader-e-6", "w-low-7"]);

const rad = (d: number): number => (d * Math.PI) / 180;
const lerp = (a: Station, b: Station, t: number): Station => [a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t, a[2] + (b[2] - a[2]) * t, a[3] + (b[3] - a[3]) * t];

/** 〰️ The stations of a tube refined by a Catmull-Rom curve through them (`sub` pieces per span). */
export function refine(pts: readonly Station[], sub: number): Station[] {
  if (sub <= 1 || pts.length < 3) return pts.map((p) => [p[0], p[1], p[2], p[3]] as Station);
  const out: Station[] = [];
  const at = (i: number): Station => pts[Math.max(0, Math.min(pts.length - 1, i))]!;
  for (let i = 0; i + 1 < pts.length; i++) {
    const [p0, p1, p2, p3] = [at(i - 1), at(i), at(i + 1), at(i + 2)];
    for (let k = 0; k < sub; k++) {
      const t = k / sub;
      const q = [0, 1, 2, 3].map((d) => 0.5 * (2 * p1[d]! + (-p0[d]! + p2[d]!) * t + (2 * p0[d]! - 5 * p1[d]! + 4 * p2[d]! - p3[d]!) * t * t + (-p0[d]! + 3 * p1[d]! - 3 * p2[d]! + p3[d]!) * t * t * t));
      out.push([q[0]!, q[1]!, q[2]!, Math.max(0.02, q[3]!)]);
    }
  }
  const last = pts[pts.length - 1]!;
  out.push([last[0], last[1], last[2], last[3]]);
  return out;
}

const alongPath = (pts: readonly Station[], f: number): Station => {
  const len = pts.slice(1).map((p, i) => Math.hypot(p[0] - pts[i]![0], p[1] - pts[i]![1], p[2] - pts[i]![2]));
  let s = f * len.reduce((a, b) => a + b, 0);
  for (let i = 0; i < len.length; i++) {
    if (s <= len[i]! || i === len.length - 1) return lerp(pts[i]!, pts[i + 1]!, Math.min(1, s / len[i]!));
    s -= len[i]!;
  }
  return pts[pts.length - 1]!;
};

/** 🌑️ The upper shaded mass at 1:500, swollen so that it closes the gaps of a few centimetres between the top clumps that a 1:500 elevation could only show as specks (semi-axes in metres); the three sky windows of the plan, about 0.5 m across, stay open as at every finer scale. */
const SHADE_500: Record<string, V3> = { inner: [3.0, 2.3, 2.6] };
function lobesOf(scale: number): Lobe[] {
  const out: Lobe[] = [];
  const [prefix, bone] = scale === 500 ? ["crown", (_: Clump) => "leader"] : ["fol", (m: Clump) => m.bone];
  const left = (id: string): boolean => (scale >= 100 && HIDDEN_100.has(id)) || (scale >= 200 && MERGED_200.has(id)) || (scale === 500 && MERGED_500.has(id));
  for (const m of CLUMPS) {
    out.push({ id: `${prefix}-${m.id}`, bone: bone(m), c: m.c, r: scale === 500 ? (SHADE_500[m.id] ?? m.r) : m.r, shade: m.shade === true, main: true });
    (m.tufts ?? []).forEach(([x, y, z, b, upTo], i) => {
      if ((upTo ?? 50) >= Math.min(scale, 200) && !left(`${m.id}-tuft-${i + 1}`)) out.push({ id: `${prefix}-${m.id}-tuft-${i + 1}`, bone: bone(m), c: [x, y, z], r: [b, b * 0.8, b], shade: false, main: false });
    });
    m.bosses.forEach(([az, el, b], i) => {
      if (left(`${m.id}-${i + 1}`)) return;
      const d: V3 = [Math.cos(rad(el)) * Math.cos(rad(az)), Math.sin(rad(el)), Math.cos(rad(el)) * Math.sin(rad(az))];
      out.push({ id: `${prefix}-${m.id}-${i + 1}`, bone: bone(m), c: [m.c[0] + m.r[0] * d[0] * 0.62, m.c[1] + m.r[1] * d[1] * 0.62, m.c[2] + m.r[2] * d[2] * 0.62], r: [b, b * 0.8, b], shade: m.shade === true, main: false });
    });
  }
  return out;
}

/** 🌳️ The tree of one scale. */
export function treeOf(scale: number): Tree {
  const tubes: Tube[] = TRUNK[scale]!.map((t) => ({ ...t }));
  if (scale === 50) {
    for (const [az, y0, y1, drift] of RIDGES) {
      const at = (y: number, a: number): Station => [Math.cos(rad(a)) * (boleRadius(y) - 0.004), y, Math.sin(rad(a)) * (boleRadius(y) - 0.004), 0.014];
      tubes.push({ id: `ridge-${az}`, bone: "trunk", region: "trunk-bark", sub: 1, slender: true, texture: true, pts: [at(y0, az), at(y1, az + drift)] });
    }
  }
  if (scale <= 100)
    for (const [az, reach] of SPURS) {
      const [c, s] = [Math.cos(rad(az)), Math.sin(rad(az))];
      const at = (d: number, y: number, r: number): Station => [c * d, y, s * d, r];
      tubes.push({ id: `root-${az}`, bone: "root-plate", region: "trunk-bark", sub: 2, pts: scale === 50 ? [at(0.3, 0.58, 0.17), at(0.5, 0.33, 0.15), at(0.72, 0.17, 0.115), at(reach - 0.14, 0.085, 0.075), at(reach - 0.05, 0.055, 0.05)] : [at(0.3, 0.58, 0.17), at(0.72, 0.17, 0.115), at(reach - 0.05, 0.055, 0.05)] });
    }
  for (const [bone, l] of Object.entries(LIMBS)) {
    const keep = (p: Station): boolean => (p[4] ?? 50) >= scale;
    if (scale === 50) {
      tubes.push({ id: bone, bone, region: "limb-bark", sub: 3, pts: l.limb });
      const joint = l.limb[l.limb.length - 1]!;
      tubes.push({ id: l.branchBone, bone: l.branchBone, region: "limb-bark", sub: 3, pts: [joint, ...l.branch] });
    } else tubes.push({ id: bone, bone, region: "limb-bark", sub: scale === 100 ? 2 : 1, pts: [...l.limb, ...l.branch].filter(keep) });
  }
  const thick = (p: Station): Station => [p[0], p[1], p[2], scale === 500 ? Math.max(0.04, p[3]) : p[3]];
  for (const t of SMALL) if (scale <= 100 || t.bone.startsWith("br-low-")) tubes.push(scale === 50 ? t : scale === 100 ? { ...t, sub: 2 } : { ...t, sub: 1, pts: [t.pts[0]!, t.pts[3]!].map(thick) });
  for (const t of BOUGHS) tubes.push(scale === 50 ? t : scale === 100 ? { ...t, sub: 2 } : { ...t, sub: 1, pts: [t.pts[0]!, t.pts[4]!].map(thick) });
  if (scale === 50)
    for (const [bone, list] of Object.entries(TWIGS)) {
      const limb = Object.values(LIMBS).find((l) => l.branchBone === bone)!;
      const path = [limb.limb[limb.limb.length - 1]!, ...limb.branch];
      list.forEach(([f, dx, dy, dz], i) => {
        const a = alongPath(path, f);
        tubes.push({ id: `twig-${bone.slice(3)}-${i + 1}`, bone, region: "limb-bark", sub: 1, slender: true, pts: [[a[0], a[1], a[2], 0.03], [a[0] + dx, a[1] + dy, a[2] + dz, 0.03]] });
      });
    }
  return { tubes, lobes: lobesOf(scale) };
}

/** 🧵 The woody axis of a bone as stations (for members, hints and strokes): the 1:100 tube of the bone or the piece of its limb. */
export function axisOf(bone: string, scale = 100): Station[] | null {
  for (const [limbBone, l] of Object.entries(LIMBS)) {
    if (bone === limbBone) return scale <= 100 ? l.limb : l.limb.filter((p) => (p[4] ?? 50) >= scale);
    if (bone === l.branchBone) return [l.limb[l.limb.length - 1]!, ...l.branch.filter((p) => (p[4] ?? 50) >= Math.max(100, scale))];
  }
  const small = SMALL.find((t) => t.bone === bone);
  if (small !== undefined) return small.pts.slice(0, 4);
  const trunk = TRUNK[50]!.find((t) => t.bone === bone && !t.id.startsWith("root-"));
  return trunk === undefined ? null : bone === "leader-top" ? trunk.pts.slice(0, 5) : trunk.pts;
}
export const boughs = (): Tube[] => BOUGHS;
export const clumpTop = (id: string, k = 0): { lobe: Lobe; top: V3 } => {
  const lobes = lobesOf(50).filter((l) => l.id === `fol-${id}` || l.id.startsWith(`fol-${id}-`));
  const lobe = lobes[k]!;
  return { lobe, top: [lobe.c[0], lobe.c[1] + lobe.r[1], lobe.c[2]] };
};
