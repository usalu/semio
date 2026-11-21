/** 🦔️ Round 5 hedgehog (round-3 model remodelled: spines laid back towards the rump on dome, saddle and skirt bowls as in a foraging animal, a head with a domed brow, full cheeks and a fine downturned snout, ears at the edge of the spine fringe): body, thermal and structure of 1:50 and 1:100 from hand tables (world of the foraging stance: x ahead from the hip joint, y up from the ground, z lateral, near side z < 0, metres).
 * Usage: bun r3-hedgehog-model.ts <50|100> [--write]
 * @see ./r3-hedgehog-kit.ts
 * @see ./r3-animals-hedgehog.md */
import { saveLists, shellOffset } from "./kit-lib.ts";
import { Hog, SIDES, sx, type Json, type Look, type PP, type Row, type V3 } from "./r3-hedgehog-kit.ts";
import { DECISIONS } from "./r3-hedgehog-texts.ts";

const SPINE: Look = { material: "spines", tone: 0.36 };
const TUFT: Look = { material: "spines", tone: 0.52 };
const BELLY: Look = { material: "belly-fur", tone: 0.74 };
const LIMB: Look = { material: "belly-fur", tone: 0.58 };
const FACE: Look = { material: "face-fur", tone: 0.64 };
const SKIN: Look = { material: "bare-skin", tone: 0.42 };
const DARK: Look = { material: "bare-skin", tone: 0.12, role: "detail" };
const BONE: Look = { material: "bone", tone: 0.9 };

/** Radius of the coat bowls and the reach of the spine tips beyond the coat mass. */
const RB = 0.048;
type Spec = {
  dome: Row[]; trunk: Row[]; e: number;
  /** Rings of tufts on the dome: [x, count over the arc from skirt edge to skirt edge]; odd and even counts alternate so that the rings interlock. */
  domeRings: [number, number][];
  /** Rings of tufts on a bowl: [polar angle from the pole (degrees), count round the pole, phase]. */
  bowlRings: [number, number, number][];
  tuft: { len: number; sink: number; r0: number; r1: number; tilt: number };
  bowlPolar: number[];
  /** Stations of the saddle, a body of revolution about the lateral axis through the mid-back joint (lateral position as a share of its half width; its radius is the height of the dome crown above the joint) and its own rings of tufts [degrees ahead of the vertical through the joint, count]: hidden inside the front half of the dome in the standing animal, they show where the back opens when it flexes. */
  saddle: number[];
  saddleRings: [number, number][];
};
/** The mid-back joint (world of the foraging stance) and the overlap of the two halves of the trunk beyond it. */
const BACK: [number, number] = [0.066, 0.088];
const LAP = 0.012;
const SPEC: Record<number, Spec> = {
  50: {
    e: 2.35,
    dome: [[-0.046, 0.062, 0.01, 0.018], [-0.04, 0.066, 0.021, 0.036], [-0.028, 0.066, 0.031, 0.051], [-0.012, 0.069, 0.037, 0.06], [0.01, 0.0718, 0.0407, 0.065], [0.04, 0.074, 0.043, 0.067], [0.075, 0.0738, 0.0427, 0.066], [0.105, 0.0715, 0.0395, 0.062], [0.128, 0.069, 0.033, 0.055], [0.146, 0.072, 0.02, 0.042], [0.158, 0.076, 0.01, 0.026], [0.163, 0.076, 0.005, 0.012]],
    trunk: [[-0.039, 0.05, 0.007, 0.009], [-0.032, 0.054, 0.02, 0.028], [-0.012, 0.06, 0.033, 0.045], [0.025, 0.064, 0.0375, 0.052], [0.07, 0.065, 0.038, 0.053], [0.102, 0.062, 0.035, 0.048], [0.126, 0.059, 0.029, 0.039], [0.146, 0.056, 0.023, 0.03]],
    domeRings: [[-0.036, 5], [-0.023, 8], [-0.01, 11], [0.003, 12], [0.016, 13], [0.029, 12], [0.042, 13], [0.055, 12], [0.068, 13], [0.081, 12], [0.094, 13], [0.107, 12], [0.12, 11], [0.133, 8], [0.146, 7], [0.157, 4]],
    bowlRings: [[0, 1, 0], [20, 6, 0], [40, 11, 0.5], [60, 15, 0], [78, 18, 0.5], [92, 20, 0]],
    tuft: { len: 0.0155, sink: 0.0045, r0: 0.0042, r1: 0.0008, tilt: 1.7 },
    bowlPolar: [96, 88, 78, 66, 54, 42, 30, 18, 8, 0],
    saddle: [-1, -0.97, -0.9, -0.78, -0.62, -0.42, -0.2, 0, 0.2, 0.42, 0.62, 0.78, 0.9, 0.97, 1],
    saddleRings: [[5, 9], [27, 8], [55, 9], [72, 8], [89, 9]],
  },
  100: {
    e: 2.35,
    dome: [[-0.046, 0.063, 0.011, 0.02], [-0.034, 0.066, 0.028, 0.046], [-0.01, 0.0705, 0.0385, 0.062], [0.04, 0.0755, 0.0445, 0.068], [0.09, 0.0735, 0.0425, 0.065], [0.126, 0.07, 0.033, 0.056], [0.15, 0.074, 0.018, 0.038], [0.163, 0.076, 0.007, 0.016]],
    trunk: [[-0.039, 0.05, 0.008, 0.01], [-0.028, 0.056, 0.024, 0.033], [0.01, 0.063, 0.037, 0.05], [0.07, 0.065, 0.038, 0.053], [0.115, 0.061, 0.032, 0.044], [0.146, 0.056, 0.023, 0.03]],
    domeRings: [[-0.03, 4], [-0.006, 5], [0.018, 6], [0.042, 7], [0.066, 6], [0.09, 7], [0.114, 6], [0.138, 5]],
    bowlRings: [[0, 1, 0], [32, 5, 0], [62, 8, 0.5], [90, 10, 0]],
    tuft: { len: 0.0185, sink: 0.0075, r0: 0.0078, r1: 0.0012, tilt: 1.5 },
    bowlPolar: [96, 80, 60, 40, 20, 0],
    saddle: [-1, -0.93, -0.75, -0.45, 0, 0.45, 0.75, 0.93, 1],
    saddleRings: [[8, 4], [62, 5], [92, 4]],
  },
  200: {
    e: 2.3,
    dome: [[-0.052, 0.062, 0.016, 0.03], [-0.042, 0.064, 0.033, 0.054], [-0.012, 0.0715, 0.0445, 0.07], [0.045, 0.0775, 0.0505, 0.075], [0.095, 0.076, 0.047, 0.072], [0.135, 0.07, 0.039, 0.064], [0.158, 0.068, 0.03, 0.05], [0.17, 0.07, 0.014, 0.026]],
    trunk: [[-0.039, 0.05, 0.009, 0.012], [-0.02, 0.058, 0.028, 0.038], [0.04, 0.064, 0.038, 0.052], [0.11, 0.061, 0.033, 0.045], [0.146, 0.056, 0.023, 0.03]],
    domeRings: [],
    bowlRings: [],
    tuft: { len: 0, sink: 0, r0: 0, r1: 0, tilt: 0.75 },
    bowlPolar: [96, 70, 45, 20, 0],
    saddle: [-1, -0.9, -0.6, 0, 0.6, 0.9, 1],
    saddleRings: [],
  },
  500: {
    e: 2.2,
    dome: [[-0.052, 0.063, 0.018, 0.034], [-0.03, 0.068, 0.04, 0.064], [0.045, 0.0775, 0.0505, 0.075], [0.13, 0.071, 0.041, 0.066], [0.168, 0.07, 0.016, 0.03]],
    trunk: [[-0.039, 0.05, 0.01, 0.013], [0, 0.061, 0.034, 0.046], [0.075, 0.064, 0.037, 0.051], [0.146, 0.056, 0.023, 0.03]],
    domeRings: [],
    bowlRings: [],
    tuft: { len: 0, sink: 0, r0: 0, r1: 0, tilt: 0.75 },
    bowlPolar: [94, 60, 30, 0],
    saddle: [-1, -0.8, 0, 0.8, 1],
    saddleRings: [],
  },
};
/** Radius of the coat bowls where the scale carries no tufts: the dome is the envelope of the spine tips down to the skirt edge and the bowls lie inside it until the animal rolls up. */
const RB_COARSE: Record<number, number> = { 200: 0.044, 500: 0.042 };

const sp = (c: number, e: number): number => Math.sign(c) * Math.abs(c) ** (2 / e);

function build(scale: number): { S: Hog; body: Json[]; thermal: Json[]; structure: Json[] } {
  const S = new Hog(scale);
  const q = SPEC[scale]!;
  const fine = scale === 50;

  const at = (rows: Row[], x: number): Row => {
    for (let i = 1; i < rows.length; i++) if (x <= rows[i]![0]!) { const a = rows[i - 1]!; const b = rows[i]!; const t = (x - a[0]!) / (b[0]! - a[0]!); return a.map((v, k) => v + (b[k]! - v) * t); }
    return rows[rows.length - 1]!;
  };
  const rear = (rows: Row[], x: number): Row[] => [...rows.filter((r) => r[0]! < x - 1e-6), at(rows, x)];
  const fore = (rows: Row[], x: number): Row[] => [at(rows, x), ...rows.filter((r) => r[0]! > x + 1e-6)];
  S.hull("belly", "trunk", rear(q.trunk, BACK[0] + LAP), BELLY, { e: 2.3 });
  S.hull("chest", "chest", fore(q.trunk, BACK[0] - LAP), BELLY, { e: 2.3 });
  S.hull("dome", "trunk", rear(q.dome, BACK[0]), SPINE, { e: q.e });
  S.hull("dome-front", "chest", fore(q.dome, BACK[0]), SPINE, { e: q.e });
  const crown = at(q.dome, BACK[0]);
  const RS = crown[1]! + crown[2]! - BACK[1];
  const rAt = (z: number): number => crown[1]! + crown[2]! * Math.max(0, 1 - Math.abs(z / crown[3]!) ** q.e) ** (1 / q.e) - BACK[1];
  const zMax = crown[3]! * (1 - ((BACK[1] - crown[1]!) / crown[2]!) ** q.e) ** (1 / q.e);
  const prof: [number, number][] = q.saddle.map((t) => [t * zMax, Math.abs(t) === 1 ? 0 : rAt(t * zMax) - (scale >= 200 ? 0.002 : 0)]);
  S.add({ id: "saddle", kind: "revolve", bone: "trunk", material: SPINE.material, role: "body", tone: SPINE.tone, profile: prof, axis: [0, 0, 1], origin: S.local("trunk", [BACK[0], BACK[1], 0]) });
  for (const [deg, n] of q.saddleRings) {
    const a = (deg * Math.PI) / 180;
    for (let i = 0; i < n; i++) {
      const z = zMax * 0.93 * Math.sin(((-90 + (180 * (i + 0.5)) / n) * Math.PI) / 180);
      const r = rAt(z);
      const slope = (rAt(z + 0.002) - rAt(z - 0.002)) / 0.004;
      const nl = Math.hypot(1, slope);
      const lean = a - Math.atan(q.tuft.tilt);
      const dir: V3 = [Math.sin(lean) / nl, Math.cos(lean) / nl, -slope / nl];
      S.cone(`sp-s-${deg}-${i}`, "trunk", [BACK[0] + (r - q.tuft.sink) * Math.sin(a), BACK[1] + (r - q.tuft.sink) * Math.cos(a), z + (slope / nl) * q.tuft.sink], dir, q.tuft.len, q.tuft.r0, q.tuft.r1, TUFT);
    }
  }
  const rowAt = (x: number): Row => at(q.dome, x);
  for (const [x, n, own] of q.domeRings.map(([x, n]) => [x, n, x < BACK[0] ? "trunk" : "chest"] as const)) {
    const [, yc, hh, hw] = rowAt(x) as [number, number, number, number];
    const span = 104;
    for (let i = 0; i < n; i++) {
      const phi = ((-span + (2 * span * (i + 0.5)) / n) * Math.PI) / 180;
      const at: V3 = [x, yc + hh * sp(Math.cos(phi), q.e), hw * sp(Math.sin(phi), q.e)];
      if (own === "chest" && x - BACK[0] < 0.8 * RS && at[1] > BACK[1] + 0.006) continue;
      const nrm: V3 = [0, sp(Math.cos(phi), 2 * q.e / (2 * q.e - 2)) / hh, sp(Math.sin(phi), 2 * q.e / (2 * q.e - 2)) / hw];
      const nl = Math.hypot(...nrm) || 1;
      const slope = (rowAt(x + 0.004)[2]! + rowAt(x + 0.004)[1]! - rowAt(x - 0.004)[2]! - rowAt(x - 0.004)[1]!) / 0.008;
      const dir: V3 = [-q.tuft.tilt - Math.max(-0.9, Math.min(0.9, slope)) * Math.max(0, Math.cos(phi)), nrm[1] / nl, nrm[2] / nl];
      const dl = Math.hypot(...dir);
      const base: V3 = [at[0] - (dir[0] / dl) * q.tuft.sink, at[1] - (dir[1] / dl) * q.tuft.sink, at[2] - (dir[2] / dl) * q.tuft.sink];
      S.cone(`sp-d-${Math.round((x + 0.05) * 1000)}-${i}`, own, base, dir, q.tuft.len, q.tuft.r0, q.tuft.r1, TUFT);
    }
  }
  for (const end of ["r", "f"] as const) for (const sd of SIDES) {
    const bone = `coat-${end}${sx(sd)}`;
    S.bowl(`bowl-${end}${sx(sd)}`, bone, RB_COARSE[scale] ?? RB, q.bowlPolar, SPINE);
    const f = S.frame(bone);
    const u: V3 = [f.u[0]!, f.u[1]!, 0];
    const v: V3 = [f.v[0]!, f.v[1]!, 0];
    for (const [polar, n, phase] of q.bowlRings) for (let i = 0; i < n; i++) {
      const t = (polar * Math.PI) / 180;
      const psi = -sd * ((i + phase) / n) * 2 * Math.PI;
      const dir: V3 = [u[0] * Math.cos(t) + v[0] * Math.sin(t) * Math.cos(psi), u[1] * Math.cos(t) + v[1] * Math.sin(t) * Math.cos(psi), Math.sin(t) * Math.sin(psi)];
      if (dir[2] * sd < -0.3 && polar > 0) continue;
      const base: V3 = [f.o[0]! + dir[0] * (RB - q.tuft.sink), f.o[1]! + dir[1] * (RB - q.tuft.sink), f.o[2]! + dir[2] * (RB - q.tuft.sink)];
      const back: V3 = [dir[0] - 0.55 * q.tuft.tilt, dir[1], dir[2]];
      S.cone(`sp-${end}${sx(sd)}-${polar}-${i}`, bone, base, back, q.tuft.len - 0.001, q.tuft.r0, q.tuft.r1, TUFT);
    }
  }

  const hd = (rows: Row[]): Row[] => rows;
  if (fine) {
    S.limb("neck", "neck", [[-0.014, -0.002, 0.025, 0.031], [0, -0.002, 0.023, 0.028], [0.014, -0.001, 0.0205, 0.025], [0.03, 0, 0.018, 0.022]], FACE);
    S.limb("head", "head", hd([[-0.01, 0, 0.016, 0.019], [0, 0.0012, 0.0188, 0.0226], [0.01, 0.0018, 0.0194, 0.024], [0.02, 0.0012, 0.0176, 0.0232], [0.03, -0.0002, 0.0142, 0.0194], [0.038, -0.0012, 0.0112, 0.0146], [0.046, -0.0018, 0.0088, 0.0102], [0.053, -0.0019, 0.0068, 0.0076], [0.059, -0.0016, 0.0052, 0.0058]]), FACE);
    S.limb("jaw", "head", [[0.004, -0.0108, 0.0085, 0.0188], [0.018, -0.0112, 0.0078, 0.0172], [0.032, -0.0098, 0.006, 0.0122], [0.043, -0.0082, 0.0045, 0.0078], [0.0505, -0.0068, 0.0039, 0.0046]], FACE);
    for (const sd of SIDES) {
      S.ball(`eye-${sx(sd)}`, "head", S.at("head", 0.0295, 0.0068, sd * 0.0152), [0.0042, 0.0038, 0.0042], DARK, [S.frame("head").u[0]!, S.frame("head").u[1]!, 0]);
      const eu = S.frame("head");
      const up: V3 = [eu.v[0]! * 0.9 - eu.u[0]! * 0.25, eu.v[1]! * 0.9 - eu.u[1]! * 0.25, sd * 0.38];
      S.ball(`ear-${sx(sd)}`, "head", S.at("head", 0.009, 0.0135, sd * 0.0215), [0.0092, 0.0074, 0.0039], FACE, up, [eu.u[0]!, eu.u[1]!, 0]);
    }
  } else if (scale === 200) {
    S.limb("head", "head", [[-0.012, 0, 0.019, 0.023], [0.012, 0.0008, 0.019, 0.0232], [0.034, 0, 0.0145, 0.0185], [0.052, -0.0006, 0.009, 0.0105], [0.064, 0, 0.005, 0.0055]], FACE);
    S.limb("neck", "neck", [[-0.014, -0.002, 0.025, 0.031], [0.03, 0, 0.018, 0.022]], FACE);
  } else if (scale === 500) {
    S.limb("head", "head", [[-0.036, 0.004, 0.024, 0.03], [-0.008, 0.001, 0.021, 0.025], [0.03, 0, 0.0155, 0.0195], [0.064, 0, 0.0052, 0.006]], FACE);
  } else {
    S.limb("head", "head", [[-0.012, 0, 0.0185, 0.0225], [0.006, 0.0008, 0.019, 0.023], [0.02, 0.0005, 0.0175, 0.0224], [0.032, 0, 0.015, 0.0196], [0.044, -0.0008, 0.011, 0.0135], [0.054, -0.0006, 0.0082, 0.0095], [0.0605, 0, 0.0058, 0.0064]], FACE);
    S.limb("neck", "neck", [[-0.014, -0.002, 0.025, 0.031], [0.01, -0.002, 0.022, 0.027], [0.03, 0, 0.018, 0.022]], FACE);
  }
  S.ball("nose", "head", S.at("head", 0.0575, fine ? -0.0012 : 0.0004), fine ? [0.0098, 0.0056, 0.006] : scale === 100 ? [0.0098, 0.0058, 0.0062] : scale === 200 ? [0.009, 0.0056, 0.006] : [0.0085, 0.0055, 0.0058], { material: "bare-skin", tone: 0.12 }, [S.frame("head").u[0]!, S.frame("head").u[1]!, 0]);
  if (fine) S.run("tail", "trunk", [{ p: [-0.03, 0.05, 0], rl: 0.006, rs: 0.006 }, { p: [-0.04, 0.0465, 0], rl: 0.005, rs: 0.005 }, { p: [-0.047, 0.043, 0], rl: 0.0039, rs: 0.0039 }], LIMB);

  for (const sd of SIDES) {
    const s = sx(sd);
    if (fine) {
      S.limb(`thigh-${s}`, `hind-${s}`, [[-0.012, 0.002, 0.014, 0.011, 0.026], [0.002, 0.001, 0.0165, 0.0125, 0.027], [0.016, 0.0005, 0.0145, 0.012, 0.03], [0.03, 0, 0.011, 0.0098, 0.033], [0.04, 0, 0.0086, 0.0082, 0.0352], [0.0465, -0.0005, 0.0064, 0.0066, 0.036]], LIMB, { sd });
      S.limb(`shank-${s}`, `shank-${s}`, [[-0.0075, 0, 0.007, 0.0068, 0.0358], [0.002, 0.0005, 0.009, 0.008, 0.036], [0.014, 0.001, 0.0088, 0.0076, 0.036], [0.028, 0.0005, 0.0064, 0.006, 0.036], [0.04, 0, 0.0052, 0.0052, 0.036], [0.048, 0, 0.0046, 0.0048, 0.036]], LIMB, { sd });
      S.limb(`foot-h${s}`, `hindpaw-${s}`, [[-0.0085, 0.0002, 0.0038, 0.0042, 0.036], [-0.003, 0, 0.005, 0.0056, 0.036], [0.008, 0, 0.005, 0.0066, 0.036], [0.018, 0.0002, 0.0048, 0.0078, 0.0362, 2.6], [0.0255, 0.0003, 0.0044, 0.0082, 0.0363, 2.6]], LIMB, { sd });
      S.limb(`toes-h${s}`, `hindpaw-${s}`, [[0.0215, 0.0002, 0.0039, 0.0074, 0.0362, 2.6], [0.027, 0.0003, 0.0044, 0.0088, 0.0364, 3], [0.034, 0.0002, 0.0041, 0.0092, 0.0364, 3.2], [0.0385, 0.0001, 0.0038, 0.0084, 0.0364, 3.2], [0.0405, 0, 0.003, 0.0062, 0.0364, 3]], SKIN, { sd });
      S.limb(`arm-${s}`, `fore-${s}`, [[-0.01, 0.001, 0.0115, 0.0092, 0.029], [0.002, 0.0005, 0.013, 0.0102, 0.0295], [0.016, 0, 0.0115, 0.0096, 0.031], [0.03, 0, 0.0088, 0.0082, 0.033], [0.038, 0, 0.0074, 0.0072, 0.034], [0.044, 0, 0.0056, 0.0058, 0.0342]], LIMB, { sd });
      S.limb(`forearm-${s}`, `forearm-${s}`, [[-0.007, 0, 0.0062, 0.0062, 0.0338], [0.002, 0, 0.0078, 0.007, 0.034], [0.012, 0.0003, 0.0074, 0.0066, 0.034], [0.024, 0, 0.0056, 0.0056, 0.034], [0.034, 0, 0.0048, 0.0052, 0.034], [0.04, 0, 0.0044, 0.005, 0.034]], LIMB, { sd });
      S.limb(`foot-f${s}`, `forepaw-${s}`, [[-0.006, 0.0002, 0.0036, 0.0044, 0.034], [0, 0, 0.0046, 0.0058, 0.034], [0.008, 0, 0.0046, 0.0076, 0.0342, 2.6], [0.0155, 0.0002, 0.0043, 0.0088, 0.0345, 2.6]], LIMB, { sd });
      S.limb(`toes-f${s}`, `forepaw-${s}`, [[0.0115, 0.0001, 0.0038, 0.0078, 0.0344, 2.6], [0.0165, 0.0002, 0.0044, 0.0096, 0.0346, 3], [0.023, 0.0002, 0.0041, 0.01, 0.0348, 3.2], [0.0268, 0.0001, 0.0038, 0.0092, 0.0348, 3.2], [0.0285, 0, 0.003, 0.0068, 0.0348, 3]], SKIN, { sd });
    } else if (scale === 200) {
      S.limb(`thigh-${s}`, `hind-${s}`, [[-0.012, 0.002, 0.014, 0.011, 0.026], [0.012, 0.001, 0.0155, 0.012, 0.029], [0.0465, 0, 0.007, 0.0072, 0.036]], LIMB, { sd });
      S.limb(`shank-${s}`, `shank-${s}`, [[-0.0075, 0, 0.0076, 0.0074, 0.036], [0.02, 0.0004, 0.0082, 0.0074, 0.036], [0.048, 0, 0.0058, 0.006, 0.036]], LIMB, { sd });
      S.limb(`toes-h${s}`, `hindpaw-${s}`, [[-0.008, 0, 0.0046, 0.0052, 0.036], [0.016, 0.0002, 0.005, 0.0078, 0.0362, 2.6], [0.0405, 0, 0.0042, 0.0078, 0.0364, 3]], LIMB, { sd });
      S.limb(`arm-${s}`, `fore-${s}`, [[-0.01, 0.001, 0.0115, 0.0092, 0.029], [0.012, 0.0003, 0.0122, 0.0098, 0.0305], [0.044, 0, 0.0064, 0.0066, 0.0342]], LIMB, { sd });
      S.limb(`forearm-${s}`, `forearm-${s}`, [[-0.007, 0, 0.0068, 0.0068, 0.034], [0.016, 0, 0.0072, 0.0068, 0.034], [0.04, 0, 0.0054, 0.0058, 0.034]], LIMB, { sd });
      S.limb(`toes-f${s}`, `forepaw-${s}`, [[-0.006, 0, 0.0042, 0.0052, 0.034], [0.01, 0.0001, 0.0046, 0.008, 0.0344, 2.6], [0.0285, 0, 0.004, 0.008, 0.0348, 3]], LIMB, { sd });
    } else if (scale === 500) {
      S.run(`shank-${s}`, `shank-${s}`, [{ p: S.at(`hind-${s}`, -0.006, 0.002, sd * 0.027), rl: 0.012, rs: 0.015 }, { p: S.at(`shank-${s}`, 0, 0, sd * 0.036), rl: 0.0085, rs: 0.009 }, { p: S.at(`shank-${s}`, 0.046, 0, sd * 0.036), rl: 0.006, rs: 0.006 }], LIMB);
      S.limb(`toes-h${s}`, `hindpaw-${s}`, [[-0.008, 0, 0.005, 0.006, 0.036], [0.0405, 0, 0.0045, 0.008, 0.0364, 2.8]], LIMB, { sd });
      S.run(`forearm-${s}`, `forearm-${s}`, [{ p: S.at(`fore-${s}`, -0.005, 0.001, sd * 0.029), rl: 0.0098, rs: 0.012 }, { p: S.at(`forearm-${s}`, 0, 0, sd * 0.034), rl: 0.0075, rs: 0.0078 }, { p: S.at(`forearm-${s}`, 0.039, 0, sd * 0.034), rl: 0.0056, rs: 0.0056 }], LIMB);
      S.limb(`toes-f${s}`, `forepaw-${s}`, [[-0.006, 0, 0.0046, 0.0058, 0.034], [0.0285, 0, 0.0042, 0.008, 0.0348, 2.8]], LIMB, { sd });
    } else {
      S.limb(`thigh-${s}`, `hind-${s}`, [[-0.012, 0.002, 0.014, 0.011, 0.026], [0.006, 0.001, 0.0165, 0.0125, 0.028], [0.028, 0, 0.0115, 0.01, 0.033], [0.0465, 0, 0.0068, 0.007, 0.036]], LIMB, { sd });
      S.limb(`shank-${s}`, `shank-${s}`, [[-0.0075, 0, 0.0074, 0.0072, 0.036], [0.008, 0.0005, 0.009, 0.008, 0.036], [0.03, 0, 0.0066, 0.0064, 0.036], [0.048, 0, 0.0056, 0.0058, 0.036]], LIMB, { sd });
      S.limb(`foot-h${s}`, `hindpaw-${s}`, [[-0.008, 0, 0.0044, 0.005, 0.036], [0.006, 0, 0.005, 0.0068, 0.036], [0.0255, 0.0003, 0.0045, 0.0082, 0.0363, 2.6]], LIMB, { sd });
      S.limb(`toes-h${s}`, `hindpaw-${s}`, [[0.0215, 0.0002, 0.0046, 0.0076, 0.0362, 2.6], [0.03, 0.0003, 0.005, 0.0092, 0.0364, 3], [0.0405, 0, 0.0042, 0.0074, 0.0364, 3]], SKIN, { sd });
      S.limb(`arm-${s}`, `fore-${s}`, [[-0.01, 0.001, 0.0115, 0.0092, 0.029], [0.006, 0.0005, 0.013, 0.0102, 0.03], [0.026, 0, 0.0096, 0.0088, 0.0325], [0.044, 0, 0.0062, 0.0064, 0.0342]], LIMB, { sd });
      S.limb(`forearm-${s}`, `forearm-${s}`, [[-0.007, 0, 0.0066, 0.0066, 0.034], [0.006, 0, 0.0078, 0.0072, 0.034], [0.024, 0, 0.006, 0.006, 0.034], [0.04, 0, 0.0052, 0.0056, 0.034]], LIMB, { sd });
      S.limb(`foot-f${s}`, `forepaw-${s}`, [[-0.006, 0, 0.004, 0.005, 0.034], [0.004, 0, 0.0046, 0.007, 0.0342], [0.0155, 0.0002, 0.0043, 0.0088, 0.0345, 2.6]], LIMB, { sd });
      S.limb(`toes-f${s}`, `forepaw-${s}`, [[0.0115, 0.0001, 0.0044, 0.008, 0.0344, 2.6], [0.02, 0.0002, 0.0048, 0.01, 0.0348, 3], [0.0285, 0, 0.004, 0.0078, 0.0348, 3]], SKIN, { sd });
    }
  }
  const body = S.body;

  const regionOf = (id: string): [string, string, string] => {
    if (/^(dome|dome-front|saddle)$/.test(id) || /^sp-[ds]-/.test(id)) return ["spines-dome", "dome-spines", "dome-air"];
    if (id.startsWith("bowl-") || id.startsWith("sp-")) return ["spines-skirt", "skirt-spines", "skirt-air"];
    if (id === "belly" || id === "chest" || id === "tail") return ["belly-fur", "belly-guard", "belly-underfur"];
    if (id === "nose") return ["nose", "", ""];
    if (id.startsWith("toes-")) return ["paws-skin", "", ""];
    if (/^(head|neck|jaw|eye|ear)/.test(id)) return ["face-fur", "face-guard", "face-underfur"];
    if (/^(thigh|shank|arm|forearm|foot)-/.test(id)) return ["limbs-fur", "limbs-guard", "limbs-underfur"];
    throw new Error(`no region for ${id}`);
  };
  const INNER: Record<string, number> = { "dome-air": 0.01, "skirt-air": 0.008, "belly-underfur": 0.0048, "face-underfur": 0.0024, "limbs-underfur": 0.002 };
  const thermal: Json[] = [];
  for (const b of body) {
    const [region, outer, inner] = regionOf(b.id);
    thermal.push({ id: `th-${b.id}`, of: b.id, region, ...(fine && outer !== "" ? { layer: outer } : {}) });
    if (fine && inner !== "" && !b.id.startsWith("sp-") && !/^(eye|ear|tail)/.test(b.id)) {
      const shell = b.id.startsWith("bowl-") ? new Hog(scale).bowl(`th-${b.id}-${inner}`, b.bone, RB - INNER[inner]!, q.bowlPolar, SPINE) : shellOffset(b, INNER[inner]!, `th-${b.id}-${inner}`);
      if (shell !== null) thermal.push({ ...shell, tone: 0.8, region, layer: inner });
    }
  }
  const E = new Hog(scale);
  const hu: V3 = [E.frame("head").u[0]!, E.frame("head").u[1]!, 0];
  E.ball("th-breath-nose", "head", E.at("head", fine ? 0.0632 : 0.0625, 0.0004), fine ? [0.0041, 0.0039, 0.0039] : scale === 100 ? [0.0048, 0.0046, 0.0046] : scale === 200 ? [0.0052, 0.005, 0.005] : [0.0056, 0.0054, 0.0054], { material: "bare-skin", tone: 0.1 }, hu);
  const top = q.dome.reduce((a, r) => (r[1]! + r[2]! > a[1]! + a[2]! ? r : a));
  E.ball("th-plume-back", "trunk", [scale >= 200 ? 0.05 : 0.056, top[1]! + top[2]! - (fine ? 0.0026 : 0.0062), 0], [scale === 200 ? 0.046 : scale === 500 ? 0.042 : 0.05, scale >= 200 ? 0.034 : 0.036, fine ? 0.0042 : 0.0077], { material: "spines", tone: 0.3 }, [1, 0, 0]);
  thermal.push({ ...E.body[0]!, region: "nose", tags: ["emitter", "breath-nose"] }, { ...E.body[1]!, region: "spines-dome", ...(fine ? { layer: "dome-spines" } : {}), tags: ["emitter", "plume-back"] });

  const K = new Hog(scale);
  const structure: Json[] = [];
  const mem = (s: Json, member: string, part = "member"): Json => { structure.push({ ...s, member, part }); return structure[structure.length - 1]!; };
  skeleton(K, mem, fine, scale);
  return { S, body, thermal, structure };
}

/** 🦴️ The skeleton of Erinaceus europaeus on the rig: skull and mandible, cervical, thoracic and lumbar spine in the arched back, ribs, sternum, scapulae, pelvis with sacrum, caudal rod, the long bones with their epiphyses, the five rays of every hand and foot; the zygomatic arches; the skin muscle (orbicularis ring) that carries each coat bowl. Parts thinner than the smallest feature of the scale are slender rods at their true thickness (C2). */
function skeleton(K: Hog, mem: (s: Json, member: string, part?: string) => Json, fine: boolean, scale: number): void {
  const hf = K.frame("head");
  const hu: V3 = [hf.u[0]!, hf.u[1]!, 0];
  const MUSCLE: Look = { material: "belly-fur", tone: 0.5 };
  const JOINTLOOK: Look = { material: "bone", tone: 0.7 };
  const last = (): Json => K.body[K.body.length - 1]!;
  const rod = (id: string, bone: string, pts: V3[], r: number, look: Look = BONE): Json => { const j = mem(K.run(id, bone, pts.map((p) => ({ p, rl: r, rs: r })), look, { floor: 0.0002 }), bone); j.slender = true; return j; };
  const tube = (id: string, bone: string, pts: [number, number, number, number, number?][]): Json => mem(K.run(id, bone, pts.map(([x, y, rl, rs, z]) => ({ p: [x, y, z ?? 0] as V3, rl, rs })), BONE), bone);
  K.limb("skull", "head", fine ? [[-0.001, 0.003, 0.0085, 0.0105], [0.008, 0.004, 0.0105, 0.0135], [0.02, 0.003, 0.0098, 0.0172], [0.03, 0.0015, 0.0078, 0.0125], [0.042, 0.0005, 0.0056, 0.0078], [0.054, 0.0002, 0.0042, 0.0052], [0.06, 0, 0.003, 0.0036]] : scale === 100 ? [[-0.001, 0.002, 0.0095, 0.011], [0.016, 0.002, 0.0115, 0.017], [0.036, 0, 0.0085, 0.0105], [0.06, -0.001, 0.0048, 0.0048]] : scale === 200 ? [[-0.001, 0.002, 0.0098, 0.0112], [0.018, 0.002, 0.0112, 0.0168], [0.06, -0.001, 0.005, 0.005]] : [[-0.001, 0.002, 0.0102, 0.0118], [0.017, 0.002, 0.011, 0.0165], [0.06, -0.001, 0.0052, 0.0052]], BONE);
  mem(last(), "head");
  if (fine) for (const sd of SIDES) rod(`zygomatic-${sx(sd)}`, "head", [K.at("head", 0.011, -0.0015, sd * 0.0128), K.at("head", 0.022, -0.004, sd * 0.0178), K.at("head", 0.035, -0.003, sd * 0.0118)], 0.0006);
  if (fine) { K.limb("mandible", "head", [[0.008, -0.0045, 0.0062, 0.0128], [0.016, -0.0082, 0.004, 0.0122], [0.03, -0.0086, 0.0039, 0.0086], [0.044, -0.0072, 0.0038, 0.0048], [0.05, -0.006, 0.0036, 0.0038]], BONE); mem(last(), "head"); }
  const nk = K.frame("neck");
  const c0: V3 = [nk.o[0]!, nk.o[1]!, 0];
  const c1 = K.at("neck", 0.025, 0);
  if (fine) tube("cervical", "neck", [[c0[0] - 0.004, c0[1] + 0.001, 0.0058, 0.004], [(c0[0] + c1[0]) / 2, (c0[1] + c1[1]) / 2 - 0.0012, 0.0062, 0.004], [c1[0] + 0.002, c1[1], 0.0068, 0.0042]]);
  else rod("cervical", "neck", [[c0[0] - 0.003, c0[1], 0], [(c0[0] + c1[0]) / 2, (c0[1] + c1[1]) / 2 - 0.001, 0], [c1[0] + 0.002, c1[1], 0]], 0.00052);
  const SPINE_PATH: [number, number, number, number][] = [[-0.004, 0.0775, 0.005, 0.004], [0.012, 0.0835, 0.0056, 0.0042], [0.03, 0.0885, 0.0056, 0.0042], [0.048, 0.0915, 0.0054, 0.0042], [0.066, 0.0925, 0.005, 0.0041], [0.086, 0.0915, 0.0048, 0.004], [0.106, 0.0875, 0.0046, 0.0039], [0.124, 0.0805, 0.0046, 0.0039], [0.136, 0.0735, 0.0048, 0.004], [0.144, 0.0665, 0.005, 0.004]];
  if (fine) { tube("lumbar", "trunk", SPINE_PATH.filter((q) => q[0] <= BACK[0])); tube("thoracic", "chest", SPINE_PATH.filter((q) => q[0] >= BACK[0])); } else {
    rod("lumbar", "trunk", SPINE_PATH.filter((q) => q[0] <= BACK[0]).map((q) => [q[0], q[1], 0] as V3), 0.0017);
    rod("thoracic", "chest", SPINE_PATH.filter((q) => q[0] >= BACK[0]).map((q) => [q[0], q[1], 0] as V3), 0.0019);
  }
  if (fine) rod("caudal", "trunk", [[-0.03, 0.0655, 0], [-0.039, 0.0575, 0], [-0.047, 0.0445, 0]], 0.00062);
  const spineAt = (x: number): number => { for (let i = 1; i < SPINE_PATH.length; i++) if (x <= SPINE_PATH[i]![0]) { const a = SPINE_PATH[i - 1]!; const b = SPINE_PATH[i]!; return a[1] + ((b[1] - a[1]) * (x - a[0])) / (b[0] - a[0]); } return SPINE_PATH[SPINE_PATH.length - 1]![1]; };
  const RIBS: [number, number, number, number][] = fine
    ? [[0.136, 0.019, 0.026, 0.01], [0.129, 0.025, 0.033, 0.006], [0.122, 0.03, 0.039, 0.003], [0.114, 0.034, 0.044, 0.001], [0.106, 0.037, 0.048, -0.002], [0.098, 0.04, 0.051, -0.004], [0.09, 0.042, 0.052, -0.007], [0.082, 0.043, 0.052, -0.01], [0.074, 0.0435, 0.05, -0.013], [0.066, 0.043, 0.047, -0.016], [0.058, 0.042, 0.043, -0.018], [0.05, 0.04, 0.038, -0.02], [0.043, 0.037, 0.032, -0.021], [0.036, 0.033, 0.026, -0.021], [0.03, 0.028, 0.02, -0.02]]
    : scale === 100 ? [[0.133, 0.022, 0.03, 0.008], [0.118, 0.032, 0.042, 0.002], [0.102, 0.039, 0.05, -0.003], [0.086, 0.0425, 0.052, -0.008], [0.07, 0.0432, 0.049, -0.014], [0.054, 0.041, 0.041, -0.019], [0.038, 0.034, 0.028, -0.021]]
      : scale === 200 ? [[0.13, 0.024, 0.033, 0.007], [0.108, 0.037, 0.048, -0.001], [0.086, 0.0425, 0.054, -0.008], [0.064, 0.0425, 0.048, -0.016], [0.042, 0.036, 0.031, -0.021]]
        : [[0.124, 0.028, 0.038, 0.005], [0.09, 0.042, 0.054, -0.007], [0.054, 0.041, 0.042, -0.019]];
  for (const sd of SIDES) RIBS.forEach(([x, w, drop, sweep], i) => {
    const y0 = spineAt(x) - 0.002;
    const pts: V3[] = [0, 0.2, 0.42, 0.64, 0.84, 1].map((t) => { const a = t * 2.35; return [x + sweep * t * t, y0 - ((drop + 0.004) * (1 - Math.cos(a))) / 1.7, sd * (0.003 + w * Math.sin(a))] as V3; });
    let len = 0;
    for (let k = 1; k < pts.length; k++) len += Math.hypot(pts[k]![0] - pts[k - 1]![0], pts[k]![1] - pts[k - 1]![1], pts[k]![2] - pts[k - 1]![2]);
    rod(`rib-${sx(sd)}-${i + 1}`, x < BACK[0] ? "trunk" : "chest", pts, Math.min(fine ? 0.0011 : 0.0013, len / 46));
  });
  rod("sternum", "chest", [[0.138, 0.042, 0], [0.115, 0.0355, 0], [0.09, 0.033, 0], [0.07, 0.034, 0]], 0.0016);
  if (fine) mem(K.run("sacrum", "trunk", [{ p: [-0.03, 0.0655, 0], rl: 0.005, rs: 0.0038 }, { p: [-0.02, 0.0705, 0], rl: 0.0088, rs: 0.0042 }, { p: [-0.008, 0.0755, 0], rl: 0.0118, rs: 0.0042 }, { p: [0.0, 0.079, 0], rl: 0.0098, rs: 0.004 }], BONE), "trunk");
  else {
    const k = scale === 100 ? 0 : scale === 200 ? 0.001 : 0.002;
    mem(K.run("pelvis", "trunk", [{ p: [0.022, 0.0805, 0], rl: 0.0105 + k, rs: 0.0076 }, { p: [0.008, 0.071, 0], rl: 0.0145 + k, rs: 0.0078 }, { p: [-0.004, 0.062, 0], rl: 0.0175 + k, rs: 0.008 }, { p: [-0.022, 0.054, 0], rl: 0.0125 + k, rs: 0.0076 }], BONE), "trunk");
    rod("lumbar-sacral", "trunk", [[-0.004, 0.0775, 0], [-0.016, 0.0725, 0], [-0.03, 0.0655, 0]], 0.0006);
  }
  for (const sd of SIDES) {
    const s = sx(sd);
    const glenoid: V3 = [0.118, 0.058, sd * 0.03];
    rod(`scapula-spine-${s}`, "chest", [glenoid, [0.109, 0.0715, sd * 0.0295], [0.1, 0.085, sd * 0.026]], 0.0008);
    if (fine) {
      rod(`scapula-cranial-${s}`, "chest", [glenoid, [0.114, 0.0745, sd * 0.029], [0.109, 0.0875, sd * 0.025]], 0.0007);
      rod(`scapula-caudal-${s}`, "chest", [glenoid, [0.104, 0.068, sd * 0.03], [0.09, 0.08, sd * 0.027]], 0.0007);
      rod(`scapula-vertebral-${s}`, "chest", [[0.109, 0.0875, sd * 0.025], [0.1, 0.0855, sd * 0.026], [0.09, 0.08, sd * 0.027]], 0.0005);
      mem(K.run(`ilium-${s}`, "trunk", [{ p: [0.026, 0.0835, sd * 0.012], rl: 0.0038, rs: 0.0052 }, { p: [0.013, 0.0735, sd * 0.016], rl: 0.0038, rs: 0.0046 }, { p: [0.001, 0.059, sd * 0.0235], rl: 0.0042, rs: 0.0048 }, { p: [-0.012, 0.0535, sd * 0.021], rl: 0.0038, rs: 0.0046 }, { p: [-0.022, 0.0515, sd * 0.017], rl: 0.0038, rs: 0.004 }], BONE), "trunk");
    }
    const long = (id: string, bone: string, len: number, z: number, heads: [number, number, number]): void => {
      if (!fine) { rod(id, bone, [K.at(bone, 0, 0, sd * z), K.at(bone, len / 2, 0, sd * z), K.at(bone, len, 0, sd * z)], 0.0009); return; }
      const [r0, rm, r1] = heads;
      mem(K.run(id, bone, ([[0, r0 * 0.75], [0.1, r0], [0.24, rm * 1.15], [0.5, rm], [0.78, rm * 1.1], [0.92, r1], [1, r1 * 0.8]] as [number, number][]).map(([t, r]) => ({ p: K.at(bone, t * len, 0, sd * z), rl: r, rs: r })), BONE), bone);
    };
    long(`humerus-${s}`, `fore-${s}`, 0.038, 0.031, [0.0046, 0.0021, 0.0042]);
    long(`radius-ulna-${s}`, `forearm-${s}`, 0.036, 0.034, [0.0044, 0.0022, 0.0042]);
    long(`femur-${s}`, `hind-${s}`, 0.04, 0.03, [0.0048, 0.0022, 0.0045]);
    long(`tibia-${s}`, `shank-${s}`, 0.044, 0.036, [0.0046, 0.002, 0.004]);
    const rays = (id: string, bone: string, z: number, len: number[], spread: number, r: number): void => len.forEach((l, i) => {
      const k = len.length === 1 ? 0 : i / (len.length - 1) - 0.5;
      rod(`${id}-${len.length === 1 ? "rays" : i + 1}-${s}`, bone, [K.at(bone, 0.002, 0, sd * (z + k * 0.004)), K.at(bone, l * 0.55, 0.0006, sd * (z + k * spread * 0.75)), K.at(bone, l, -0.0012, sd * (z + k * spread))], r);
    });
    rays("manus", `forepaw-${s}`, 0.0345, fine ? [0.019, 0.025, 0.0275, 0.026, 0.021] : [0.0275], 0.0145, fine ? 0.0004 : 0.0006);
    rays("pes", `hindpaw-${s}`, 0.0362, fine ? [0.026, 0.036, 0.039, 0.0375, 0.031] : [0.039], 0.0135, fine ? 0.00055 : 0.0009);
    for (const end of ["f", "r"] as const) {
      const bone = `coat-${end}${s}`;
      const f = K.frame(bone);
      const rr = RB - 0.006;
      const n = fine ? 14 : scale === 100 ? 8 : scale === 200 ? 6 : 4;
      const ring: V3[] = [];
      for (let i = 0; i <= n; i++) {
        const psi = (i / n) * Math.PI;
        ring.push([f.o[0]! + f.v[0]! * rr * Math.cos(psi), f.o[1]! + f.v[1]! * rr * Math.cos(psi), f.o[2]! + sd * rr * Math.sin(psi)]);
      }
      rod(`orbicularis-${end}${s}`, bone, ring, 0.0014, MUSCLE);
    }
    for (const [bone, r50, r100] of [[`fore-${s}`, 0.0048, 0.0076], [`forearm-${s}`, 0.0044, 0.0074], [`forepaw-${s}`, 0.004, 0.005], [`hind-${s}`, 0.005, 0.0076], [`shank-${s}`, 0.0046, 0.0074], [`hindpaw-${s}`, 0.004, 0.005]] as [string, number, number][]) {
      const f = K.frame(bone);
      const z = bone.startsWith("fore-") ? 0.031 : bone.startsWith("hind-") ? 0.03 : Math.abs(f.o[2]!);
      const r = fine ? r50 : r100;
      if (scale === 500 || (scale === 200 && /paw/.test(bone))) continue;
      mem(K.ball(`joint-${bone}`, bone, [f.o[0]!, f.o[1]!, sd * z], [r, r, r], JOINTLOOK), bone, "joint");
    }
  }
  const rj = fine ? 0.0046 : scale === 100 ? 0.0076 : scale === 200 ? 0.008 : 0.0084;
  mem(K.ball("joint-chest", "chest", [BACK[0], BACK[1], 0], [rj + 0.0004, rj + 0.0004, rj + 0.0004], JOINTLOOK), "chest", "joint");
  mem(K.ball("joint-neck", "neck", [nk.o[0]!, nk.o[1]!, 0], [rj, rj, rj], JOINTLOOK), "neck", "joint");
  mem(K.ball("joint-head", "head", [hf.o[0]!, hf.o[1]!, 0], [rj, rj, rj], JOINTLOOK, hu), "head", "joint");
  const rm = fine ? 0.0062 : scale === 100 ? 0.0077 : scale === 200 ? 0.0082 : 0.0088;
  mem(K.ball("mass-trunk", "trunk", [0.028, 0.06, 0], [rm, rm, rm], { material: "belly-fur", tone: 0.4 }), "trunk", "mass");
  mem(K.ball("mass-chest", "chest", [0.1, 0.06, 0], [rm, rm, rm], { material: "belly-fur", tone: 0.4 }), "chest", "mass");
  mem(K.ball("mass-head", "head", K.at("head", 0.026, 0.002), [0.0047, 0.0047, 0.0047], { material: "belly-fur", tone: 0.4 }, hu), "head", "mass");
}

const scale = Number(process.argv[2] ?? 50);
const { S, body, thermal, structure } = build(scale);
console.log(`1:${scale} body ${body.length} thermal ${thermal.length} structure ${structure.length}`);
if (process.argv.includes("--write")) saveLists(S.c, { body, thermal, structure, decisions: DECISIONS[scale]! });
