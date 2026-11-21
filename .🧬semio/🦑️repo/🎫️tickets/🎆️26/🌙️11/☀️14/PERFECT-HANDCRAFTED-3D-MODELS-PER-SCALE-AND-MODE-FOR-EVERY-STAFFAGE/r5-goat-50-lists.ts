/** 🦴️ Round 5 goat (from round 3), 1:50: the thermal list (every body solid by reference, the breath outlets on muzzle and lips, the plume patch on the back) and the structure list (the skeleton of a goat: skull with horn cores, mandible, cervical, thoracic and lumbar column, eight rib hoops with sternum, sacrum and hip bones, scapula, long bones with their epiphyses, calcaneus, phalanges, tail vertebrae, hinge pins at the rig joints, the two large trunk masses).
 * @see ./r3-goat-50.ts */
import { A, AO, Bench, HEAD_DEG, X, along, dome, far, near, smooth, span, type Look, type St } from "./r3-goat-kit.ts";
import { DECISIONS } from "./r5-goat-decisions.ts";
import { ribCage } from "./r5-goat-ribs.ts";
import { type Json, type V3 } from "./volumes-lib.ts";

export const BONE: Look = { material: "bone", tone: 0.9 };
const OUTLET: Look = { material: "bare-skin", tone: 0.3, role: "detail" };

export const REGION_OF = (id: string): [string, string | undefined] => {
  if (/^(ribcage|abdomen|loin|withers|croup|udder|teat|trunk|chest|hindbody)/.test(id)) return ["coat-trunk", "coat"];
  if (id === "neck") return ["coat-neck", "coat"];
  if (/^(head|jaw|beard|eye|nostril)/.test(id)) return ["face", "coat"];
  if (id.startsWith("ear")) return ["ears", undefined];
  if (id.startsWith("horn")) return ["horns", undefined];
  if (id.startsWith("tail")) return ["tail", "coat"];
  if (/^(thigh|gaskin|shoulder|arm|forearm|hind-upper|fore-upper)/.test(id)) return ["coat-upper-legs", "coat"];
  if (/^(hock|cannon|pastern|hind-lower|fore-lower)/.test(id)) return ["lower-legs", "coat"];
  if (/^(claw|dewclaw|hoof)/.test(id)) return ["hooves", undefined];
  throw new Error(`no region for ${id}`);
};

export function thermalRefs(B: Bench): void {
  B.list("thermal");
  for (const s of B.lists.body) {
    const [region, layer] = REGION_OF(s.id);
    B.ref(`th-${s.id}`, s.id, { region, ...(layer === undefined ? {} : { layer }) });
  }
}

export type Outlets = { nose: readonly [number, number, number, number, number]; mouth: readonly [number, number, number, number]; proud?: number };

/** 💨️ The outlets (D2), each one smallest feature deep and sunk so far that only a skin of it stands proud of the surface it lies in: `nose` = `[s, v, |z|, rv, rz]` of the nostril patch in the head frame (|z| = 0 merges the pair into one patch on the muzzle), `mouth` = `[s, v, rv, rz]` of the lip patch, and the plume patch let into the top line of the back. */
export function outlets(B: Bench, o: Outlets): void {
  const hu: V3 = [Math.cos((HEAD_DEG * Math.PI) / 180), Math.sin((HEAD_DEG * Math.PI) / 180), 0];
  const hn: V3 = [-hu[1], hu[0], 0];
  const h = B.feature / 2 + 0.0002;
  B.list("thermal", { region: "face" });
  const [ns, nv, nz, nrv, nrz] = o.nose;
  if (nz > 0) for (const [side, z] of [["n", -nz], ["f", nz]] as const) B.ell(`th-breath-nose-${side}`, "head", B.world("head", [ns, nv, z]), [h, nrv, nrz], OUTLET, [hu, hn], { tags: ["emitter", "breath-nose"] });
  else B.ell("th-breath-nose", "head", B.world("head", [ns, nv, 0]), [h, nrv, nrz], OUTLET, [hu, hn], { tags: ["emitter", "breath-nose"] });
  const [ms, mv, mrv, mrz] = o.mouth;
  B.ell("th-breath-mouth", "head", B.world("head", [ms, mv, 0]), [h, mrv, mrz], OUTLET, [hu, hn], { tags: ["emitter", "breath-mouth"] });
  plumePatch(B, o.proud ?? 0.003);
}

/** 🌡️ The plume outlet: a strip let into the withers and back, its top `proud` above the top line. */
export function plumePatch(B: Bench, proud: number): void {
  const h = Math.max(0.012, B.feature / 2 + 0.0002);
  const w = (rz: number): number => Math.max(rz, h);
  B.list("thermal", { region: "coat-trunk" });
  B.loft("th-plume-back", "thorax", dome([X(0.4, 0.75 - h + proud, 0, h, w(0.042), 2), X(0.48, 0.75 - h + proud, 0, h, w(0.038), 2), X(0.56, 0.755 - h + proud, 0, h, w(0.033), 2), X(0.63, 0.76 - h + proud, 0, h, w(0.03), 2)], 0.02, 0.02), OUTLET, { tags: ["emitter", "plume-back"] });
}

export function thermal50(B: Bench): void {
  thermalRefs(B);
  outlets(B, { nose: [0.2428, 0.002, 0.011, 0.005, 0.005], mouth: [0.2335, -0.019, 0.005, 0.009] });
}


/** 🩻️ One rib pair with its cartilages as a hoop across the thoracic column: vertical radius and lateral radius of the ribcage wall at world `x`, bar `t` thick. */
export function hoop(B: Bench, id: string, x: number, yc: number, ry: number, rz: number, t: number, inset: number, extra: Json): void {
  const c = B.local("thorax", [x, yc, 0]);
  const R = ry - inset;
  B.lists.structure.push({ id, kind: "revolve", bone: "thorax", material: BONE.material, role: "body", tone: BONE.tone, profile: [[-t / 2, R - t / 2], [t / 2, R - t / 2], [t / 2, R + t / 2], [-t / 2, R + t / 2]], axis: [1, 0, 0], origin: c, lateralScale: Math.round(((rz - inset) / R) * 1e4) / 1e4, ...extra });
}

/** 🔩️ A hinge pin across the sagittal plane at a joint: `h` half-length, `r` radius. */
export function pin(B: Bench, bone: string, at: readonly [number, number], z: number, h: number, r: number): void {
  B.list("structure", { member: bone, part: "joint" });
  B.cap(`joint-${bone}`, bone, [at[0], at[1], z - h], [at[0], at[1], z + h], r, r, { material: "bone", tone: 0.55 });
}

export function structure50(B: Bench): void {
  const M = (bone: string): void => void B.list("structure", { member: bone, part: "member" });
  const H = (s: number, off: number, ra: number, rb: number, e = 2.2): St => A(AO, HEAD_DEG, s, off, ra, rb, 0, e);
  M("head");
  B.loft("skull", "head", smooth([H(-0.022, 0.008, 0.026, 0.028), H(0.005, 0.012, 0.04, 0.042, 2.5), H(0.04, 0.012, 0.043, 0.05, 2.6), H(0.07, 0.01, 0.04, 0.052, 2.5), H(0.1, 0.008, 0.033, 0.04), H(0.14, 0.006, 0.026, 0.028), H(0.19, 0.004, 0.02, 0.022), H(0.225, 0.004, 0.014, 0.017), H(0.236, 0.004, 0.008, 0.011)], 1), BONE);
  for (const [side, sg] of [["n", -1], ["f", 1]] as const) {
    const core = along([[1.041, 0.905, 0.024, 0.0105, 0.0078], [1.047, 0.955, 0.03, 0.0088, 0.0066], [1.038, 1.005, 0.041, 0.0066, 0.005], [1.02, 1.039, 0.052, 0.0045, 0.004], [1.006, 1.056, 0.06, 0.004, 0.004]]);
    B.loft(`horn-core-${side}`, "head", smooth(sg < 0 ? near(core) : far(near(core)), 1), BONE);
  }
  M("jaw");
  B.loft("mandible", "jaw", smooth([H(0.018, -0.022, 0.012, 0.034), H(0.035, -0.045, 0.026, 0.036), H(0.07, -0.052, 0.018, 0.033), H(0.11, -0.046, 0.012, 0.026), H(0.16, -0.035, 0.009, 0.017), H(0.2, -0.028, 0.0075, 0.013), H(0.218, -0.026, 0.005, 0.008)], 1), BONE);
  M("neck");
  B.loft("cervical", "neck", smooth(along([[0.982, 0.893, 0, 0.016, 0.02], [0.935, 0.848, 0, 0.017, 0.021], [0.878, 0.778, 0, 0.019, 0.023], [0.818, 0.708, 0, 0.021, 0.025], [0.758, 0.663, 0, 0.022, 0.026], [0.7, 0.655, 0, 0.022, 0.026]]), 2), BONE);
  M("thorax");
  B.loft("thoracic", "thorax", smooth([span([0.715, 0.69], [0.705, 0.633], 0.013), span([0.675, 0.742], [0.67, 0.64], 0.012), span([0.635, 0.752], [0.63, 0.648], 0.011), span([0.58, 0.749], [0.575, 0.658], 0.011), span([0.5, 0.742], [0.5, 0.672], 0.012), span([0.42, 0.739], [0.42, 0.686], 0.013), span([0.365, 0.738], [0.365, 0.692], 0.014)], 1), BONE);
  ribCage(B, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12], 0.0055, 0.0045, { member: "thorax", part: "member" });
  M("lumbar");
  B.loft("lumbar-column", "lumbar", [X(0.385, 0.713, 0, 0.024, 0.016), X(0.3, 0.715, 0, 0.025, 0.017), X(0.2, 0.716, 0, 0.025, 0.017), X(0.1, 0.714, 0, 0.024, 0.016)], BONE);
  B.loft("lumbar-processes", "lumbar", [X(0.36, 0.71, 0, 0.0045, 0.05, 3), X(0.3, 0.71, 0, 0.0045, 0.058, 3), X(0.2, 0.71, 0, 0.0045, 0.06, 3), X(0.13, 0.71, 0, 0.0045, 0.055, 3)], BONE);
  M("pelvis");
  B.loft("sacrum", "pelvis", smooth([X(0.115, 0.714, 0, 0.022, 0.03), X(0.06, 0.72, 0, 0.02, 0.026), X(0.0, 0.722, 0, 0.015, 0.018), X(-0.04, 0.722, 0, 0.01, 0.011)], 1), BONE);
  for (const [side, sg] of [["n", -1], ["f", 1]] as const) {
    const hip = along([[0.132, 0.708, 0.07, 0.014, 0.009], [0.09, 0.7, 0.056, 0.017, 0.008], [0.04, 0.672, 0.058, 0.014, 0.009], [0, 0.645, 0.062, 0.022, 0.013], [-0.045, 0.648, 0.056, 0.02, 0.007], [-0.088, 0.664, 0.052, 0.014, 0.01]]);
    B.loft(`hip-bone-${side}`, "pelvis", smooth(sg < 0 ? near(hip) : far(near(hip)), 1), BONE);
  }
  M("tail-1");
  B.cap("caudal-1", "tail-1", [-0.05, 0.716, 0], [-0.0245, 0.7865, 0], 0.008, 0.0065, BONE);
  M("tail-2");
  B.cap("caudal-2", "tail-2", [-0.024, 0.7878, 0], [0.003, 0.8345, 0], 0.006, 0.004, BONE);
  for (const [side, sg] of [["n", -1], ["f", 1]] as const) {
    const S = (st: St[]): St[] => (sg < 0 ? near(st) : far(near(st)));
    M(`ear-${side}`);
    B.ref(`ear-cartilage-${side}`, `ear-${side}`);
    M(`scapula-f${side}`);
    const SC: [number, number] = [0.636, 0.745];
    B.loft(`scapula-${side}`, `scapula-f${side}`, smooth(S([A(SC, -58.31, 0.024, -0.004, 0.032, 0.0045, 0.041, 2.6), A(SC, -58.31, 0.055, -0.003, 0.036, 0.0045, 0.052, 2.4), A(SC, -58.31, 0.1, -0.002, 0.026, 0.005, 0.067, 2.2), A(SC, -58.31, 0.145, 0, 0.014, 0.008, 0.079), A(SC, -58.31, 0.172, 0, 0.018, 0.013, 0.085)]), 1), BONE);
    M(`humerus-f${side}`);
    const SH: [number, number] = [0.728, 0.596];
    B.loft(`humerus-${side}`, `humerus-f${side}`, smooth(dome(S([A(SH, -114.8, 0.002, 0.004, 0.02, 0.017, 0.086), A(SH, -114.8, 0.035, 0, 0.013, 0.012, 0.087), A(SH, -114.8, 0.09, -0.002, 0.0105, 0.0105, 0.088), A(SH, -114.8, 0.145, 0, 0.0125, 0.0125, 0.088), A(SH, -114.8, 0.172, 0, 0.018, 0.017, 0.088)]), 0.008, 0.008), 1), BONE);
    M(`forearm-f${side}`);
    const EL: [number, number] = [0.655, 0.438];
    B.loft(`radius-ulna-${side}`, `forearm-f${side}`, smooth(dome(S([A(EL, -87.57, -0.04, -0.02, 0.009, 0.0075, 0.088), A(EL, -87.57, -0.012, -0.012, 0.016, 0.012, 0.088), A(EL, -87.57, 0.012, -0.003, 0.018, 0.016, 0.087), A(EL, -87.57, 0.045, 0, 0.012, 0.0125, 0.085), A(EL, -87.57, 0.12, 0, 0.0105, 0.012, 0.08), A(EL, -87.57, 0.185, 0, 0.013, 0.015, 0.076), A(EL, -87.57, 0.216, 0, 0.016, 0.018, 0.075)]), 0.006, 0.008), 1), BONE);
    M(`metacarpus-f${side}`);
    const CA: [number, number] = [0.664, 0.226];
    B.loft(`metacarpal-${side}`, `metacarpus-f${side}`, smooth(dome(S([A(CA, -91.95, 0.012, 0, 0.014, 0.016, 0.075), A(CA, -91.95, 0.035, 0, 0.0095, 0.0105, 0.075), A(CA, -91.95, 0.1, 0, 0.009, 0.0105, 0.075), A(CA, -91.95, 0.132, 0, 0.0115, 0.014, 0.075), A(CA, -91.95, 0.146, 0, 0.013, 0.016, 0.075)]), 0.006, 0.006), 1), BONE);
    M(`thigh-h${side}`);
    B.loft(`femur-${side}`, `thigh-h${side}`, smooth(dome(S([A([0, 0.64], -56.46, 0.0, 0.004, 0.019, 0.017, 0.07), A([0, 0.64], -56.46, 0.03, 0.002, 0.014, 0.013, 0.078), A([0, 0.64], -56.46, 0.1, 0, 0.0115, 0.0115, 0.084), A([0, 0.64], -56.46, 0.175, 0, 0.015, 0.015, 0.088), A([0, 0.64], -56.46, 0.208, 0, 0.022, 0.02, 0.088)]), 0.01, 0.01), 1), BONE);
    B.ell(`trochanter-${side}`, `thigh-h${side}`, [-0.014, 0.657, sg * 0.088], [0.013, 0.02, 0.011], BONE);
    B.ell(`patella-${side}`, `thigh-h${side}`, [0.146, 0.47, sg * 0.088], [0.009, 0.015, 0.011], BONE);
    M(`shank-h${side}`);
    const ST: [number, number] = [0.118, 0.462];
    B.loft(`tibia-${side}`, `shank-h${side}`, smooth(dome(S([A(ST, -116.13, 0.006, 0.002, 0.022, 0.02, 0.088), A(ST, -116.13, 0.045, 0.002, 0.014, 0.013, 0.085), A(ST, -116.13, 0.12, 0, 0.0105, 0.0105, 0.08), A(ST, -116.13, 0.2, 0, 0.012, 0.012, 0.074), A(ST, -116.13, 0.232, 0, 0.016, 0.016, 0.071)]), 0.008, 0.006), 1), BONE);
    M(`metatarsus-h${side}`);
    const HK: [number, number] = [0.014, 0.25];
    B.loft(`calcaneus-${side}`, `metatarsus-h${side}`, dome(S([A([-0.006, 0.29], -72, 0, 0, 0.008, 0.008, 0.071), A([-0.006, 0.29], -72, 0.025, 0.002, 0.01, 0.009, 0.071), A([-0.006, 0.29], -72, 0.05, 0.006, 0.016, 0.014, 0.071)]), 0.005, 0.008), BONE);
    B.loft(`metatarsal-${side}`, `metatarsus-h${side}`, smooth(dome(S([A(HK, -84.99, 0.008, 0, 0.015, 0.016, 0.071), A(HK, -84.99, 0.035, 0, 0.0105, 0.0105, 0.071), A(HK, -84.99, 0.1, 0, 0.0095, 0.0105, 0.072), A(HK, -84.99, 0.155, 0, 0.012, 0.014, 0.072), A(HK, -84.99, 0.17, 0, 0.013, 0.016, 0.072)]), 0.006, 0.006), 1), BONE);
    for (const [leg, fx, zc] of [["h", 0.029, 0.072], ["f", 0.659, 0.075]] as const) {
      M(`hoof-${leg}${side}`);
      for (const [digit, dz] of [["o", 0.0095], ["i", -0.0095]] as const) {
        const z = sg * (zc + dz);
        B.cap(`phalanx-${leg}${side}-${digit}`, `hoof-${leg}${side}`, [fx + 0.006, 0.072, z], [fx + 0.034, 0.034, z], 0.0065, 0.006, BONE);
        B.loft(`coffin-${leg}${side}-${digit}`, `hoof-${leg}${side}`, S([{ c: [fx + 0.03, 0.03, zc + dz], a: [0.006, 0.005, 0], b: [0, 0, 0.0065] }, { c: [fx + 0.04, 0.017, zc + dz], a: [0.007, 0.006, 0], b: [0, 0, 0.0065] }, { c: [fx + 0.049, 0.009, zc + dz], a: [0.004, 0.0035, 0], b: [0, 0, 0.004] }]), BONE);
      }
    }
  }
  const joints: [string, [number, number], number, number, number][] = [["lumbar", [0.108, 0.712], 0, 0.016, 0.008], ["thorax", [0.375, 0.712], 0, 0.015, 0.008], ["neck", [0.7, 0.655], 0, 0.02, 0.009], ["head", AO, 0, 0.022, 0.008], ["jaw", [1.0077, 0.8679], 0, 0.036, 0.006], ["tail-1", [-0.0481, 0.722], 0, 0.009, 0.006], ["tail-2", [-0.024, 0.7878], 0, 0.007, 0.0045]];
  for (const [bone, at, z, h, r] of joints) pin(B, bone, at, z, h, r);
  for (const [side, sg] of [["n", -1], ["f", 1]] as const) {
    const limb: [string, [number, number], number, number, number][] = [["ear-" + side, [1.0215, 0.9026], 0.046, 0.005, 0.005], ["thigh-h" + side, [0, 0.64], 0.068, 0.012, 0.011], ["shank-h" + side, [0.118, 0.462], 0.088, 0.018, 0.009], ["metatarsus-h" + side, [0.014, 0.25], 0.071, 0.01, 0.0075], ["hoof-h" + side, [0.029, 0.079], 0.072, 0.011, 0.006], ["scapula-f" + side, [0.636, 0.745], 0.036, 0.004, 0.006], ["humerus-f" + side, [0.728, 0.596], 0.085, 0.011, 0.011], ["forearm-f" + side, [0.655, 0.438], 0.088, 0.015, 0.008], ["metacarpus-f" + side, [0.664, 0.226], 0.075, 0.012, 0.0075], ["hoof-f" + side, [0.659, 0.079], 0.075, 0.011, 0.006]];
    for (const [bone, at, z, h, r] of limb) pin(B, bone, at, sg * z, h, r);
  }
  for (const [bone, r] of [["thorax", 0.034], ["lumbar", 0.027]] as const) {
    const m = (B.asset.rig.bones as Json[]).find((b) => b.id === bone).mass;
    B.list("structure", { member: bone, part: "mass" });
    B.ell(`mass-${bone}`, bone, B.world(bone, [m.comX, m.comY, 0]), [r, r, r], { material: "bone", tone: 0.3 });
  }
}

export const DECISIONS_50: Json[] = DECISIONS[50]!;
