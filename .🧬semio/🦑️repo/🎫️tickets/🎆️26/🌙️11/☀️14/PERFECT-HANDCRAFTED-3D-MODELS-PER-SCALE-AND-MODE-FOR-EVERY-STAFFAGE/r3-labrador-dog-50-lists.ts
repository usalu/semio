/** 🦴️ Round 3 Labrador, 1:50: the thermal list (every body solid by reference, two nostril patches and a lip patch as breath outlets, the plume strip let into the back) and the structure list (the skeleton of a dog: skull, mandible, cervical, thoracic and lumbar column with their processes, rib hoops and sternum, sacrum and hip bones, caudal vertebrae, scapula, long bones with their ends, calcaneus, metapodials and four digits per paw, hinge pins at the rig joints, the three trunk masses).
 * @see ./r3-labrador-dog-50.ts */
import { A, Bench, X, along, dome, far, near, smooth, span, type Look, type St } from "./r3-labrador-dog-kit.ts";
import { DEG, H, HN, HU, J, Z } from "./r3-labrador-dog-50.ts";
import { DECISIONS } from "./r3-labrador-dog-decisions.ts";
import { type Json, type V3 } from "./volumes-lib.ts";

export const BONE: Look = { material: "bone", tone: 0.9 };
export const OUTLET: Look = { material: "bare-skin", tone: 0.3, role: "detail" };

export const REGION_OF = (id: string): [string, string | undefined] => {
  if (/^(chest|hindbody|croup|neck|trunk)/.test(id)) return ["coat-trunk", "fur"];
  if (/^(head|jaw|eye)/.test(id)) return ["coat-head", "fur"];
  if (id.startsWith("ear")) return ["ears", "fur"];
  if (id.startsWith("tail")) return ["coat-tail", "fur"];
  if (/^(pad|toe-pad|carpal-pad)/.test(id)) return ["pads", undefined];
  if (/^(thigh|shank|hock|metatarsus|foot-h|toe-h|hind|paw-h)/.test(id)) return ["coat-hindlimbs", "fur"];
  if (/^(shoulder|upper-arm|forearm|pastern|foot-f|toe-f|fore|paw-f)/.test(id)) return ["coat-forelimbs", "fur"];
  if (id.startsWith("nose")) return ["nose-skin", undefined];
  if (id.startsWith("tongue")) return ["tongue", undefined];
  throw new Error(`no region for ${id}`);
};

export function thermalRefs(B: Bench): void {
  B.list("thermal");
  for (const s of B.lists.body) {
    const [region, layer] = REGION_OF(s.id);
    B.ref(`th-${s.id}`, s.id, { region, ...(layer === undefined ? {} : { layer }) });
  }
}

export type Outlets = { nose: readonly [number, number, number, number, number]; mouth: readonly [number, number, number, number]; plume: readonly (readonly [number, number, number])[]; proud?: number };

/** 💨️ The outlets (D2), each one smallest feature deep and sunk so far that only a skin of it stands proud of the surface it lies in: `nose` = `[s, v, |z|, rv, rz]` of the nostril patch in the head frame (|z| = 0 merges the pair into one patch on the nose), `mouth` = `[s, v, rv, rz]` of the lip patch, `plume` = `[x, top line y, half-width]` stations of the strip let into the back. */
export function outlets(B: Bench, o: Outlets): void {
  const h = B.feature / 2 + 0.0002;
  B.list("thermal", { region: "nose-skin" });
  const [ns, nv, nz, nrv, nrz] = o.nose;
  if (nz > 0) for (const [side, z] of [["n", -nz], ["f", nz]] as const) B.ell(`th-breath-nose-${side}`, "head", B.world("head", [ns, nv, z]), [h, nrv, nrz], OUTLET, [HU, HN], { tags: ["emitter", "breath-nose"] });
  else B.ell("th-breath-nose", "head", B.world("head", [ns, nv, 0]), [h, nrv, nrz], OUTLET, [HU, HN], { tags: ["emitter", "breath-nose"] });
  const [ms, mv, mrv, mrz] = o.mouth;
  B.list("thermal", { region: "tongue" });
  B.ell("th-breath-mouth", "jaw", B.world("head", [ms, mv, 0]), [h, mrv, mrz], OUTLET, [HU, HN], { tags: ["emitter", "breath-mouth"] });
  const hp = Math.max(0.012, h);
  const proud = o.proud ?? 0.003;
  B.list("thermal", { region: "coat-trunk" });
  B.loft("th-plume-back", "thorax", dome(o.plume.map(([x, y, rz]) => X(x, y - hp + proud, 0, hp, Math.max(rz, hp), 2)), 0.02, 0.02), OUTLET, { tags: ["emitter", "plume-back"] });
}

export function thermal50(B: Bench): void {
  thermalRefs(B);
  outlets(B, { nose: [0.2452, 0.006, 0.0085, 0.005, 0.005], mouth: [0.2265, -0.031, 0.004, 0.012], plume: [[0.22, 0.5545, 0.04], [0.28, 0.5565, 0.04], [0.33, 0.5595, 0.036], [0.38, 0.565, 0.03]] });
}

/** 🩻️ One rib pair with its cartilages as a hoop across the thoracic column: vertical radius and lateral radius of the ribcage wall at world `x`, bar `t` thick. */
export function hoop(B: Bench, id: string, x: number, yc: number, ry: number, rz: number, t: number, inset: number, extra: Json): void {
  const c = B.local("thorax", [x, yc, 0]);
  const R = ry - inset;
  B.lists.structure.push({ id, kind: "revolve", bone: "thorax", material: BONE.material, role: "body", tone: BONE.tone, profile: [[-t / 2, R - t / 2], [t / 2, R - t / 2], [t / 2, R + t / 2], [-t / 2, R + t / 2], [-t / 2, R - t / 2]], axis: [1, 0, 0], origin: c, lateralScale: Math.round(((rz - inset) / R) * 1e4) / 1e4, ...extra });
}

/** 🔩️ A hinge pin across the sagittal plane at a joint: `h` half-length, `r` radius. */
export function pin(B: Bench, bone: string, at: readonly [number, number], z: number, h: number, r: number): void {
  B.list("structure", { member: bone, part: "joint" });
  B.cap(`joint-${bone}`, bone, [at[0], at[1], z - h], [at[0], at[1], z + h], r, r, { material: "bone", tone: 0.55 });
}

/** ⚖️ The concentrated masses of the trunk at their centres of mass. */
export function masses(B: Bench, list: readonly (readonly [string, number])[]): void {
  for (const [bone, r] of list) {
    const m = (B.asset.rig.bones as Json[]).find((b) => b.id === bone).mass;
    B.list("structure", { member: bone, part: "mass" });
    B.ell(`mass-${bone}`, bone, B.world(bone, [m.comX, m.comY, 0]), [r, r, r], { material: "bone", tone: 0.3 });
  }
}

export const RIBS: readonly (readonly [number, number, number, number])[] = [[0.255, 0.4255, 0.13, 0.127], [0.285, 0.4235, 0.133, 0.129], [0.315, 0.4225, 0.136, 0.13], [0.345, 0.423, 0.1385, 0.13], [0.375, 0.4245, 0.1405, 0.127], [0.4, 0.426, 0.142, 0.124], [0.425, 0.4285, 0.138, 0.115], [0.445, 0.4305, 0.126, 0.104], [0.46, 0.433, 0.108, 0.09]];

/** 🐾️ The bones of one paw: per digit the proximal phalanx from the pastern joint to the knuckle and the middle and distal phalanx down to the claw. */
export function digits(B: Bench, leg: "h" | "f", side: "n" | "f", m: number, zc: number, k: number, r: number): void {
  const sg = side === "n" ? -1 : 1;
  const bone = `paw-${leg}${side}`;
  B.list("structure", { member: bone, part: "member" });
  ([["o", 0.0245, 0.064, 0.016], ["p", 0.0082, 0.077, 0.006], ["q", -0.0082, 0.077, -0.006], ["i", -0.0245, 0.064, -0.016]] as const).forEach(([toe, dz, reach, z0]) => {
    const z = sg * (zc + dz * k);
    const t = m + reach * k;
    B.cap(`phalanx-1-${leg}${side}-${toe}`, bone, [m + 0.004, 0.033, sg * (zc + z0 * k)], [t - 0.034 * k, 0.03, z], r, r, BONE);
    B.cap(`phalanx-2-${leg}${side}-${toe}`, bone, [t - 0.034 * k, 0.03, z], [t - 0.011 * k, 0.014, z], r, r * 0.9, BONE);
  });
}

export function structure50(B: Bench): void {
  const M = (bone: string): void => void B.list("structure", { member: bone, part: "member" });
  M("head");
  B.loft("skull", "head", smooth([H(-0.014, 0.01, 0.02, 0.022), H(0.008, 0.014, 0.035, 0.038, 2.4), H(0.045, 0.014, 0.04, 0.046, 2.5), H(0.075, 0.012, 0.038, 0.055, 2.4), H(0.1, 0.006, 0.032, 0.045), H(0.125, 0, 0.026, 0.03), H(0.16, -0.002, 0.023, 0.025), H(0.2, -0.002, 0.02, 0.022), H(0.225, -0.001, 0.016, 0.018), H(0.234, 0, 0.008, 0.01)], 1), BONE);
  M("jaw");
  B.loft("mandible", "jaw", smooth([H(0.03, -0.02, 0.012, 0.035), H(0.048, -0.04, 0.019, 0.037), H(0.08, -0.049, 0.013, 0.033), H(0.12, -0.049, 0.011, 0.028), H(0.165, -0.048, 0.0095, 0.023), H(0.2, -0.046, 0.008, 0.018), H(0.213, -0.045, 0.005, 0.011)], 1), BONE);
  M("neck");
  B.loft("cervical", "neck", smooth(along([[0.61, 0.647, 0, 0.015, 0.026], [0.59, 0.622, 0, 0.016, 0.02], [0.562, 0.578, 0, 0.017, 0.02], [0.532, 0.528, 0, 0.018, 0.021], [0.502, 0.483, 0, 0.019, 0.022], [0.47, 0.449, 0, 0.019, 0.022]]), 2), BONE);
  M("thorax");
  B.loft("thoracic", "thorax", smooth([span([0.456, 0.528], [0.468, 0.432], 0.012), span([0.428, 0.553], [0.44, 0.444], 0.011), span([0.39, 0.552], [0.398, 0.458], 0.011), span([0.345, 0.545], [0.35, 0.47], 0.011), span([0.295, 0.542], [0.297, 0.482], 0.012), span([0.248, 0.542], [0.248, 0.49], 0.013)], 1), BONE);
  RIBS.forEach(([x, yc, ry, rz], i) => hoop(B, `rib-${i + 1}`, x, yc, ry, rz, 0.01, 0.016, { member: "thorax", part: "member" }));
  B.loft("sternum", "thorax", smooth([X(0.27, 0.305, 0, 0.008, 0.012), X(0.33, 0.3, 0, 0.0085, 0.013), X(0.39, 0.3, 0, 0.009, 0.013), X(0.44, 0.316, 0, 0.009, 0.012), X(0.475, 0.348, 0, 0.009, 0.011), X(0.495, 0.39, 0, 0.008, 0.01)], 1), BONE);
  M("lumbar");
  B.loft("lumbar-column", "lumbar", [X(0.25, 0.501, 0, 0.019, 0.015), X(0.18, 0.504, 0, 0.02, 0.016), X(0.11, 0.507, 0, 0.02, 0.016), X(0.05, 0.508, 0, 0.019, 0.015)], BONE);
  B.loft("lumbar-spines", "lumbar", [X(0.245, 0.526, 0, 0.016, 0.0045, 3), X(0.18, 0.529, 0, 0.017, 0.0045, 3), X(0.11, 0.531, 0, 0.017, 0.0045, 3), X(0.06, 0.531, 0, 0.015, 0.0045, 3)], BONE);
  B.loft("lumbar-processes", "lumbar", [X(0.235, 0.496, 0, 0.0045, 0.04, 3), X(0.18, 0.496, 0, 0.0045, 0.05, 3), X(0.12, 0.497, 0, 0.0045, 0.052, 3), X(0.07, 0.499, 0, 0.0045, 0.044, 3)], BONE);
  M("pelvis");
  B.loft("sacrum", "pelvis", smooth([X(0.06, 0.51, 0, 0.019, 0.027), X(0.025, 0.516, 0, 0.017, 0.021), X(-0.02, 0.52, 0, 0.012, 0.013), X(-0.066, 0.522, 0, 0.009, 0.009)], 1), BONE);
  for (const [side, sg] of [["n", -1], ["f", 1]] as const) {
    const hip = along([[0.092, 0.532, 0.056, 0.021, 0.007], [0.055, 0.521, 0.05, 0.02, 0.008], [0.022, 0.503, 0.05, 0.013, 0.009], [0, 0.485, 0.054, 0.02, 0.012], [-0.04, 0.472, 0.05, 0.016, 0.007], [-0.084, 0.465, 0.05, 0.013, 0.011]]);
    B.loft(`hip-bone-${side}`, "pelvis", smooth(sg < 0 ? near(hip) : far(near(hip)), 1), BONE);
  }
  M("tail-1");
  B.cap("caudal-1", "tail-1", [-0.066, 0.5216, 0], [-0.2017, 0.5129, 0], 0.009, 0.007, BONE);
  M("tail-2");
  B.cap("caudal-2", "tail-2", [-0.2017, 0.5129, 0], [-0.3217, 0.5129, 0], 0.007, 0.0055, BONE);
  M("tail-3");
  B.cap("caudal-3", "tail-3", [-0.3217, 0.5129, 0], [-0.424, 0.52, 0], 0.0055, 0.004, BONE);
  for (const [side, sg] of [["n", -1], ["f", 1]] as const) {
    const S = (st: St[]): St[] => (sg < 0 ? near(st) : far(near(st)));
    M(`ear-${side}`);
    B.ref(`ear-cartilage-${side}`, `ear-${side}`);
    M(`scapula-f${side}`);
    B.loft(`scapula-${side}`, `scapula-f${side}`, smooth(S([A(J.scapula, DEG.scapula, 0.008, -0.002, 0.03, 0.0045, 0.04), A(J.scapula, DEG.scapula, 0.04, -0.002, 0.036, 0.0045, 0.052), A(J.scapula, DEG.scapula, 0.09, -0.001, 0.026, 0.005, 0.066), A(J.scapula, DEG.scapula, 0.134, 0, 0.014, 0.008, 0.074), A(J.scapula, DEG.scapula, 0.156, 0, 0.017, 0.013, 0.077)]), 1), BONE);
    M(`humerus-f${side}`);
    B.loft(`humerus-${side}`, `humerus-f${side}`, smooth(dome(S([A(J.shoulder, DEG.humerus, 0.002, 0.004, 0.02, 0.017, 0.078), A(J.shoulder, DEG.humerus, 0.035, 0, 0.013, 0.012, 0.079), A(J.shoulder, DEG.humerus, 0.09, -0.002, 0.011, 0.011, 0.08), A(J.shoulder, DEG.humerus, 0.145, 0, 0.0125, 0.0125, 0.08), A(J.shoulder, DEG.humerus, 0.172, 0, 0.018, 0.017, 0.08)]), 0.008, 0.008), 1), BONE);
    M(`forearm-f${side}`);
    B.loft(`radius-ulna-${side}`, `forearm-f${side}`, smooth(dome(S([A(J.elbow, DEG.forearm, -0.034, -0.02, 0.009, 0.008, 0.082), A(J.elbow, DEG.forearm, -0.01, -0.012, 0.016, 0.012, 0.082), A(J.elbow, DEG.forearm, 0.012, -0.003, 0.018, 0.016, 0.081), A(J.elbow, DEG.forearm, 0.045, 0, 0.013, 0.013, 0.08), A(J.elbow, DEG.forearm, 0.11, 0, 0.0115, 0.012, 0.078), A(J.elbow, DEG.forearm, 0.158, 0, 0.013, 0.015, 0.076), A(J.elbow, DEG.forearm, 0.177, 0, 0.016, 0.018, 0.075)]), 0.006, 0.006), 1), BONE);
    M(`metacarpus-f${side}`);
    B.loft(`metacarpals-${side}`, `metacarpus-f${side}`, smooth(dome(S([A(J.carpus, DEG.metacarpus, 0.004, 0, 0.015, 0.019, Z.carpus), A(J.carpus, DEG.metacarpus, 0.026, 0, 0.011, 0.017, Z.carpus, 2.6), A(J.carpus, DEG.metacarpus, 0.058, 0, 0.011, 0.019, Z.carpus, 2.6), A(J.carpus, DEG.metacarpus, 0.072, 0, 0.012, 0.021, Z.carpus, 2.6)]), 0.005, 0.005), 1), BONE);
    digits(B, "f", side, J.mcp[0], Z.carpus, 1.15, 0.0048);
    M(`thigh-h${side}`);
    B.loft(`femur-${side}`, `thigh-h${side}`, smooth(dome(S([A(J.hip, DEG.thigh, 0, 0.004, 0.018, 0.016, 0.06), A(J.hip, DEG.thigh, 0.03, 0.002, 0.013, 0.013, 0.067), A(J.hip, DEG.thigh, 0.1, 0, 0.0115, 0.0115, 0.074), A(J.hip, DEG.thigh, 0.165, 0, 0.015, 0.015, 0.079), A(J.hip, DEG.thigh, 0.194, 0, 0.021, 0.02, 0.08)]), 0.008, 0.008), 1), BONE);
    B.ell(`trochanter-${side}`, `thigh-h${side}`, [-0.013, 0.499, sg * 0.074], [0.011, 0.016, 0.01], BONE);
    B.ell(`patella-${side}`, `thigh-h${side}`, [0.066, 0.296, sg * 0.081], [0.008, 0.013, 0.01], BONE);
    M(`shank-h${side}`);
    B.loft(`tibia-${side}`, `shank-h${side}`, smooth(dome(S([A(J.stifle, DEG.shank, 0.006, 0.002, 0.021, 0.02, 0.08), A(J.stifle, DEG.shank, 0.045, 0.002, 0.014, 0.013, 0.078), A(J.stifle, DEG.shank, 0.11, 0, 0.011, 0.011, 0.074), A(J.stifle, DEG.shank, 0.172, 0, 0.012, 0.012, 0.07), A(J.stifle, DEG.shank, 0.198, 0, 0.016, 0.016, 0.068)]), 0.007, 0.005), 1), BONE);
    M(`metatarsus-h${side}`);
    B.cap(`calcaneus-${side}`, `metatarsus-h${side}`, [-0.114, 0.157, sg * Z.hock], [-0.134, 0.18, sg * Z.hock], 0.0095, 0.008, BONE);
    B.loft(`metatarsals-${side}`, `metatarsus-h${side}`, smooth(dome(S([A(J.hock, DEG.metatarsus, 0.004, 0, 0.015, 0.016, Z.hock), A(J.hock, DEG.metatarsus, 0.03, 0, 0.011, 0.0135, Z.hock, 2.6), A(J.hock, DEG.metatarsus, 0.08, 0, 0.0105, 0.0145, Z.hock, 2.6), A(J.hock, DEG.metatarsus, 0.116, 0, 0.012, 0.018, Z.hock, 2.6)]), 0.005, 0.005), 1), BONE);
    digits(B, "h", side, J.mtp[0], Z.hock, 1.04, 0.0045);
  }
  const joints: [string, readonly [number, number], number, number, number][] = [["lumbar", J.ls, 0, 0.014, 0.007], ["thorax", J.t13, 0, 0.013, 0.007], ["neck", J.c7, 0, 0.018, 0.008], ["head", [0.612, 0.648], 0, 0.02, 0.0075], ["jaw", [0.64, 0.6319], 0, 0.036, 0.0055], ["tail-1", J.tail1, 0, 0.009, 0.006], ["tail-2", J.tail2, 0, 0.007, 0.005], ["tail-3", J.tail3, 0, 0.006, 0.004]];
  for (const [bone, at, z, h, r] of joints) pin(B, bone, at, z, h, r);
  for (const [side, sg] of [["n", -1], ["f", 1]] as const) {
    const ear = B.world(`ear-${side}`, [0, 0, 0]);
    const limb: [string, readonly [number, number], number, number, number][] = [["ear-" + side, [ear[0], ear[1]], 0.066, 0.0045, 0.0045], ["thigh-h" + side, J.hip, 0.056, 0.011, 0.01], ["shank-h" + side, J.stifle, Z.stifle, 0.016, 0.008], ["metatarsus-h" + side, J.hock, Z.hock, 0.012, 0.0065], ["paw-h" + side, J.mtp, Z.hock, 0.02, 0.005], ["scapula-f" + side, J.scapula, 0.036, 0.004, 0.0055], ["humerus-f" + side, J.shoulder, 0.077, 0.011, 0.01], ["forearm-f" + side, J.elbow, Z.elbow, 0.014, 0.0075], ["metacarpus-f" + side, J.carpus, Z.carpus, 0.016, 0.0065], ["paw-f" + side, J.mcp, Z.carpus, 0.022, 0.005]];
    for (const [bone, at, z, h, r] of limb) pin(B, bone, at, sg * z, h, r);
  }
  masses(B, [["thorax", 0.032], ["lumbar", 0.024], ["pelvis", 0.022]]);
}

export const DECISIONS_50: Json[] = DECISIONS[50]!;
export type { V3 };
