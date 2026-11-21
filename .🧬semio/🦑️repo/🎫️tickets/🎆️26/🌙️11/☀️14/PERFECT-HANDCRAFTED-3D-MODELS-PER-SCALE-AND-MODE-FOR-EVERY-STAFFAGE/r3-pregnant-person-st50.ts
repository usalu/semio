/** 🦴 The 1:50 load path of `humans/pregnant-person`: the female skeleton (cranium, facial skeleton, mandible, cervical spine, thoracolumbar spine with the deepened lumbar lordosis, twelve ribs each side with the lower ribs flared by the pregnancy, sternum, clavicles, scapulae, sacrum, the wide female hip bones with their pubic arch, femur, patella, tibia, fibula, the bones of the foot and the hand) and the abdomen member: the skeleton of the fetus at 36 weeks in the left occiput anterior position, head engaged in the pelvic inlet, back along the left front of the uterus, legs folded at the fundus; joints at the rig joints and the masses at the centres of mass.
 *
 * Stations as in the body (`sa`, `pa`); the torso frame runs from the lumbosacral joint to C7, the pelvis frame from the midpoint of the hip joints, the abdomen frame from the front of L3 straight ahead (`f` up).
 * @see ./r3-pregnant-person-s50.ts */
import type { Json } from "./volumes-lib.ts";
import { ASSET, capsule, ellipsoid, loft, mirrored, pa, sa, toRight, tube, aboutW, type V3 } from "./r3-pregnant-person-kit.ts";
import { C, MAT, TONE } from "./r3-pregnant-person-s50.ts";
import { RIG } from "./r3-pregnant-person-data.ts";

type Part = "member" | "joint" | "support" | "mass";
const B = (id: string, bone: string): Json => C(id, bone, MAT.bone, TONE.bone);
export const bar = (id: string, bone: string, pts: V3[], r: number[] | number, member = bone): Json => ({ ...tube(B(id, bone), pts, Array.isArray(r) ? r : pts.map(() => r)), member, part: "member" });
export const el = (id: string, bone: string, radii: V3, at: V3, part: Part = "member", member = bone, rot?: [number, number, number, number]): Json => ({ ...ellipsoid(C(id, bone, MAT.bone, part === "member" ? TONE.bone : part === "joint" ? TONE.joint : part === "mass" ? TONE.mass : TONE.bone), radii, at, rot), member, part });
const lo = (id: string, bone: string, st: Parameters<typeof loft>[1], member = bone): Json => ({ ...loft(B(id, bone), st), member, part: "member" });

/** 🦴 The thoracolumbar spine (vertebral body centres): L5/S1 at the joint, the lumbar curve ahead of the trunk line, the thoracic kyphosis behind it, C7 at the tip. */
export const SPINE: [number, number][] = [[-0.01, 0.002], [0.04, 0.016], [0.09, 0.022], [0.14, 0.014], [0.19, -0.004], [0.24, -0.02], [0.29, -0.031], [0.34, -0.034], [0.39, -0.026], [0.43, -0.012], [0.458, 0]];
const spineF = (a: number): number => { let i = 0; while (i + 2 < SPINE.length && SPINE[i + 1]![0] < a) i++; const p = SPINE[i]!; const q = SPINE[i + 1]!; return p[1] + ((q[1] - p[1]) * (a - p[0])) / (q[0] - p[0]); };
/** 🦴 The ribs: [a at the spine, half-width of the cage, drop from the vertebra to the front, ends free]. The lower ribs flare wider (subcostal angle of late pregnancy). */
const RIBS: [number, number, number, boolean][] = [
  [0.425, 0.055, 0.05, false], [0.405, 0.08, 0.075, false], [0.385, 0.095, 0.09, false], [0.362, 0.105, 0.1, false], [0.338, 0.112, 0.108, false], [0.313, 0.117, 0.112, false],
  [0.288, 0.122, 0.115, false], [0.263, 0.126, 0.11, false], [0.238, 0.128, 0.1, false], [0.213, 0.126, 0.085, false], [0.19, 0.115, 0.05, true], [0.172, 0.095, 0.03, true],
];

function axial(): Json[] {
  const out: Json[] = [];
  out.push(bar("spine", "torso", SPINE.map(([a, f]) => pa("torso", a, f, 0)), [0.022, 0.023, 0.023, 0.022, 0.02, 0.018, 0.016, 0.015, 0.014, 0.0125, 0.012]));
  for (const a of [0.04, 0.09, 0.14, 0.19, 0.24, 0.29, 0.34, 0.39]) {
    const f = spineF(a);
    out.push({ ...capsule(B(`spinous-process-${Math.round(a * 100)}`, "torso"), pa("torso", a, f - 0.02, 0), pa("torso", a - 0.02, f - 0.046, 0), 0.0058, 0.0045), member: "torso", part: "member" });
  }
  RIBS.forEach(([a, w, drop, free], i) => {
    const s = spineF(a);
    const back = s - 0.045;
    const pts: V3[] = [[a, s - 0.012, 0.02], [a + 0.004, back, w * 0.42], [a - drop * 0.22, back + 0.022, w * 0.86], [a - drop * 0.5, s + 0.045, w], [a - drop * 0.78, s + 0.098, w * 0.82]];
    if (!free) pts.push([a - drop * 0.95, s + 0.125 + (i > 6 ? 0.005 : 0), w * 0.42], [Math.max(0.215, a - drop + 0.02), 0.1 + 0.02 * Math.min(1, (0.4 - a) / 0.18), 0.016]);
    const rib = bar(`rib-${i + 1}-l`, "torso", pts.map(([aa, f, l]) => pa("torso", aa, f, l)), i === 0 ? 0.0062 : 0.0052);
    out.push(rib, mirrored(rib, toRight));
  });
  out.push(lo("sternum", "torso", [sa("torso", 0.408, 0.088, 0, 0.024, 0.0075, 2.4), sa("torso", 0.37, 0.097, 0, 0.017, 0.0075, 2.4), sa("torso", 0.3, 0.111, 0, 0.016, 0.0075, 2.4), sa("torso", 0.245, 0.12, 0, 0.012, 0.006, 2.2), sa("torso", 0.22, 0.121, 0, 0.006, 0.005, 2)]));
  const clav = bar("clavicle-l", "torso", [[0.41, 0.083, 0.02], [0.415, 0.088, 0.065], [0.425, 0.06, 0.115], [0.43, 0.025, 0.155]].map(([a, f, l]) => pa("torso", a, f, l)), [0.007, 0.0065, 0.006, 0.0068]);
  const scap = { ...lo("scapula-l", "torso", [sa("torso", 0.405, -0.062, 0.1, 0.046, 0.0065, 2.6), sa("torso", 0.365, -0.07, 0.098, 0.042, 0.0065, 2.6), sa("torso", 0.31, -0.073, 0.09, 0.026, 0.006, 2.4), sa("torso", 0.275, -0.071, 0.086, 0.009, 0.005, 2)].map((q) => ({ ...q, yaw: 18 }))) };
  const spineScap = bar("scapular-spine-l", "torso", [[0.385, -0.07, 0.062], [0.405, -0.068, 0.12], [0.425, -0.035, 0.16], [0.428, 0.0, 0.168]].map(([a, f, l]) => pa("torso", a, f, l)), [0.0055, 0.0065, 0.0072, 0.0072]);
  const glenoid = el("glenoid-l", "torso", [0.018, 0.012, 0.007], pa("torso", 0.396, 0.02, 0.135));
  out.push(clav, mirrored(clav, toRight), scap, mirrored(scap, toRight), spineScap, mirrored(spineScap, toRight), glenoid, mirrored(glenoid, toRight));
  out.push(lo("sacrum", "pelvis", [sa("pelvis", 0.095, -0.032, 0, 0.056, 0.02, 2.2, -28), sa("pelvis", 0.055, -0.058, 0, 0.048, 0.017, 2.2, -36), sa("pelvis", 0.015, -0.082, 0, 0.032, 0.013, 2, -46), sa("pelvis", -0.02, -0.098, 0, 0.016, 0.009, 2, -58), sa("pelvis", -0.04, -0.1, 0, 0.008, 0.006, 2, -70)]));
  const ilium = { ...lo("hip-bone-l", "pelvis", [sa("pelvis", 0.122, -0.012, 0.118, 0.078, 0.0065, 2.2), sa("pelvis", 0.095, -0.008, 0.12, 0.08, 0.0075, 2.2), sa("pelvis", 0.055, -0.004, 0.112, 0.056, 0.011, 2.2), sa("pelvis", 0.02, -0.004, 0.098, 0.03, 0.019, 2), sa("pelvis", -0.025, -0.012, 0.084, 0.024, 0.016, 2)].map((q, i) => ({ ...q, yaw: [56, 54, 42, 18, 0][i] }))) };
  const pubis = bar("pubic-ramus-l", "pelvis", [[-0.008, 0.018, 0.084], [-0.022, 0.052, 0.046], [-0.03, 0.064, 0.008]].map(([a, f, l]) => pa("pelvis", a, f, l)), [0.0105, 0.0095, 0.0095]);
  const ischium = bar("ischial-ramus-l", "pelvis", [[-0.02, -0.022, 0.082], [-0.068, -0.035, 0.07], [-0.076, -0.008, 0.056], [-0.052, 0.046, 0.014]].map(([a, f, l]) => pa("pelvis", a, f, l)), [0.011, 0.013, 0.0095, 0.0085]);
  const acet = el("acetabulum-l", "pelvis", [0.026, 0.026, 0.012], pa("pelvis", 0, 0, RIG.hipZ + 0.012));
  out.push(ilium, mirrored(ilium, toRight), pubis, mirrored(pubis, toRight), ischium, mirrored(ischium, toRight), acet, mirrored(acet, toRight));
  out.push(el("cranium", "head", [0.074, 0.088, 0.068], pa("head", 0.072, -0.004, 0)));
  out.push(el("facial-skeleton", "head", [0.042, 0.03, 0.05], pa("head", 0.024, 0.058, 0)));
  out.push({ ...tube(B("mandible", "head"), [[0.015, -0.006, 0.05], [-0.025, -0.004, 0.047], [-0.045, 0.035, 0.037], [-0.058, 0.08, 0.0], [-0.045, 0.035, -0.037], [-0.025, -0.004, -0.047], [0.015, -0.006, -0.05]].map(([a, f, l]) => pa("head", a, f, l)), [0.0075, 0.0085, 0.0085, 0.0105, 0.0085, 0.0085, 0.0075]), member: "head", part: "member" });
  out.push(el("nasal-bone", "head", [0.012, 0.006, 0.006], pa("head", 0.035, 0.094, 0)));
  out.push(bar("cervical-spine", "neck", [[-0.005, 0.0, 0], [0.03, 0.002, 0], [0.065, 0.004, 0], [0.098, 0.002, 0]].map(([a, f, l]) => pa("neck", a, f, l)), [0.012, 0.0115, 0.0115, 0.012]));
  return out;
}

function limbs(): Json[] {
  const out: Json[] = [];
  const th = "thigh-l";
  const femur = [
    el("femoral-head-l", th, [0.022, 0.022, 0.022], [0, 0, 0]),
    bar("femoral-neck-l", th, [pa(th, 0.0, 0, 0), pa(th, 0.028, -0.004, 0.042), pa(th, 0.05, -0.008, 0.056)], [0.0145, 0.013, 0.016]),
    el("greater-trochanter-l", th, [0.028, 0.019, 0.016], pa(th, 0.036, -0.012, 0.064)),
    bar("femur-l", th, [pa(th, 0.05, -0.006, 0.055), pa(th, 0.15, 0.007, 0.038), pa(th, 0.27, 0.01, 0.018), pa(th, 0.37, 0.002, 0.004)], [0.0145, 0.013, 0.013, 0.0155]),
    el("femoral-condyles-l", th, [0.03, 0.031, 0.038], pa(th, 0.392, -0.006, 0)),
  ];
  const sh = "shank-l";
  const shank = [
    el("tibial-plateau-l", sh, [0.023, 0.028, 0.036], pa(sh, 0.024, 0, 0)),
    bar("tibia-l", sh, [pa(sh, 0.04, 0.006, -0.004), pa(sh, 0.2, 0.01, -0.006), pa(sh, 0.365, 0.004, -0.008)], [0.0165, 0.0115, 0.014]),
    el("medial-malleolus-l", sh, [0.02, 0.014, 0.012], pa(sh, 0.385, 0.004, -0.016)),
    bar("fibula-l", sh, [pa(sh, 0.035, -0.012, 0.03), pa(sh, 0.22, -0.008, 0.026), pa(sh, 0.395, -0.008, 0.026)], [0.0085, 0.007, 0.0095]),
    el("patella-l", sh, [0.021, 0.01, 0.019], pa(sh, -0.012, 0.042, 0)),
  ];
  const ft = "foot-l";
  const a0 = RIG.lift;
  const foot = [
    el("talus-l", ft, [0.026, 0.016, 0.019], [0.006, -0.008, 0]),
    bar("calcaneus-l", ft, [[0.0, -0.025, 0.002], [-0.028, -0.042, 0.004], [-0.048, -0.05 + a0 * 0.4, 0.004]], [0.018, 0.019, 0.017]),
    el("midfoot-l", ft, [0.028, 0.015, 0.028], [0.044, -0.03, -0.002]),
  ];
  for (const [k, l0, l1] of [[1, -0.016, -0.024], [2, -0.006, -0.008], [3, 0.005, 0.007], [4, 0.014, 0.02], [5, 0.023, 0.032]] as const) foot.push(bar(`metatarsal-${k}-l`, ft, [[0.058, -0.035, l0], [0.122, -RIG.ankle + RIG.ball + 0.012, l1]], [k === 1 ? 0.007 : 0.0055, k === 1 ? 0.008 : 0.006]));
  const toe = "toe-l";
  const toes: Json[] = [];
  for (const [k, l, len] of [[1, -0.025, 0.05], [2, -0.008, 0.044], [3, 0.007, 0.038], [4, 0.02, 0.032], [5, 0.032, 0.026]] as const) toes.push(bar(`phalanges-${k}-l`, toe, [[-0.004, RIG.ball + 0.012, l], [len * 0.6, RIG.ball + 0.011, l - 0.002], [len, RIG.ball + 0.009, l - 0.004]], [k === 1 ? 0.0078 : 0.0055, 0.005, 0.0045]));
  const ua = "arm-l";
  const arm = [
    el("humeral-head-l", ua, [0.022, 0.022, 0.022], pa(ua, 0, 0, -0.004)),
    el("greater-tubercle-l", ua, [0.016, 0.012, 0.011], pa(ua, 0.012, 0.006, 0.014)),
    bar("humerus-l", ua, [pa(ua, 0.02, 0.003, 0.004), pa(ua, 0.15, 0.002, 0.003), pa(ua, 0.285, -0.004, 0)], [0.0145, 0.0105, 0.0115]),
    el("humeral-condyles-l", ua, [0.014, 0.015, 0.029], pa(ua, 0.302, -0.004, 0)),
  ];
  const fa = "forearm-l";
  const fore = [
    bar("ulna-l", fa, [pa(fa, -0.022, -0.012, -0.011), pa(fa, 0.03, -0.006, -0.011), pa(fa, 0.15, 0, -0.013), pa(fa, 0.238, -0.004, -0.015)], [0.0105, 0.0085, 0.0065, 0.007]),
    bar("radius-l", fa, [pa(fa, 0.01, 0.004, 0.011), pa(fa, 0.12, 0.006, 0.011), pa(fa, 0.234, 0.006, 0.013)], [0.0075, 0.007, 0.0105]),
  ];
  const ha = "hand-l";
  const hand: Json[] = [el("carpus-l", ha, [0.017, 0.024, 0.0085], pa(ha, 0.016, 0.002, -0.001))];
  for (const [name, f] of [["index", 0.024], ["middle", 0.008], ["ring", -0.008], ["little", -0.022]] as const) hand.push(bar(`hand-bones-${name}-l`, ha, [pa(ha, 0.03, f * 0.8, -0.002), pa(ha, 0.092, f, -0.002), pa(ha, 0.13, f, -0.008), pa(ha, 0.16, f, -0.016)], [0.0042, 0.0042, 0.0038, 0.0034]));
  hand.push(bar("thumb-bones-l", ha, [pa(ha, 0.012, 0.02, -0.006), pa(ha, 0.045, 0.038, -0.014), pa(ha, 0.072, 0.05, -0.022), pa(ha, 0.092, 0.058, -0.028)], [0.0048, 0.0043, 0.0038, 0.0034]));
  for (const q of [...femur, ...shank, ...foot, ...toes, ...arm, ...fore, ...hand]) out.push(q, mirrored(q, toRight));
  return out;
}

/** 👶 The fetus at 36 weeks (crown-rump 0.33 m, biparietal diameter 90 mm), left occiput anterior: the head engaged low in the pelvic inlet and flexed, the spine curved along the mother's left front, the limbs folded towards her right. */
function fetus(): Json[] {
  const m = "belly";
  const P = (a: number, f: number, l: number): V3 => pa(m, a, f, l);
  const out: Json[] = [];
  out.push(el("fetal-skull", m, [0.055, 0.046, 0.045], P(0.115, -0.115, 0.004), "member", m, aboutW(-62)));
  out.push(bar("fetal-spine", m, [P(0.13, -0.065, 0.03), P(0.16, -0.02, 0.05), P(0.18, 0.04, 0.058), P(0.175, 0.095, 0.05), P(0.15, 0.13, 0.032)], [0.0075, 0.0085, 0.0085, 0.0075, 0.0065]));
  out.push(el("fetal-ribcage", m, [0.04, 0.038, 0.034], P(0.16, 0.0, 0.02), "member"));
  out.push(el("fetal-pelvis", m, [0.022, 0.022, 0.026], P(0.14, 0.12, 0.012)));
  for (const [s, k] of [["right", -1], ["left", 1]] as const) {
    out.push(bar(`fetal-femur-${s}`, m, [P(0.135, 0.11, 0.012 + k * 0.018), P(0.19, 0.075, -0.035 + k * 0.022)], 0.0055));
    out.push(bar(`fetal-tibia-${s}`, m, [P(0.19, 0.075, -0.035 + k * 0.022), P(0.13, 0.135, -0.065 + k * 0.02)], 0.0048));
    out.push(bar(`fetal-humerus-${s}`, m, [P(0.13, -0.035, 0.02 + k * 0.025), P(0.175, -0.05, -0.02 + k * 0.022)], 0.0045));
    out.push(bar(`fetal-forearm-${s}`, m, [P(0.175, -0.05, -0.02 + k * 0.022), P(0.17, -0.005, -0.045 + k * 0.012)], 0.004));
  }
  return out;
}

/** ⚙️ Joints at the rig joints, masses at the centres of mass of the heavy segments and of the fetal unit. */
function joints(): Json[] {
  const out: Json[] = [];
  out.push(el("joint-lumbosacral", "torso", [0.012, 0.024, 0.026], [0, 0, 0], "joint"));
  out.push(el("joint-cervicothoracic", "neck", [0.009, 0.016, 0.018], [0, 0, 0], "joint"));
  out.push(el("joint-atlanto-occipital", "head", [0.008, 0.015, 0.02], [0, 0, 0], "joint"));
  out.push(el("joint-abdomen", "belly", [0.012, 0.016, 0.02], pa("belly", 0.035, 0, 0), "joint"));
  for (const s of ["l", "r"]) {
    out.push(el(`joint-hip-${s}`, `thigh-${s}`, [0.025, 0.025, 0.025], [0, 0, 0], "joint"));
    out.push(el(`joint-knee-${s}`, `shank-${s}`, [0.012, 0.025, 0.036], [0, 0, 0], "joint"));
    out.push(el(`joint-ankle-${s}`, `foot-${s}`, [0.015, 0.013, 0.022], [0, 0, 0], "joint"));
    out.push(el(`joint-toe-${s}`, `toe-${s}`, [0.009, 0.009, 0.034], [0, RIG.ball + 0.012, 0], "joint"));
    out.push(el(`joint-shoulder-${s}`, `arm-${s}`, [0.024, 0.024, 0.024], [0, 0, 0], "joint"));
    out.push(el(`joint-elbow-${s}`, `forearm-${s}`, [0.012, 0.014, 0.024], [0, 0, 0], "joint"));
    out.push(el(`joint-wrist-${s}`, `hand-${s}`, [0.008, 0.015, 0.01], [0, 0, 0], "joint"));
  }
  for (const b of ASSET.rig.bones as Json[]) if (b.mass && b.mass.kg >= 2.5) { const r = Math.min(0.03, 0.012 + 0.0009 * b.mass.kg); out.push(el(`mass-${b.id}`, b.id, [r, r, r], [b.mass.comX, b.mass.comY ?? 0, 0], "mass")); }
  return out;
}

export function structure50(): Json[] {
  return [...axial(), ...limbs(), ...fetus(), ...joints()];
}
