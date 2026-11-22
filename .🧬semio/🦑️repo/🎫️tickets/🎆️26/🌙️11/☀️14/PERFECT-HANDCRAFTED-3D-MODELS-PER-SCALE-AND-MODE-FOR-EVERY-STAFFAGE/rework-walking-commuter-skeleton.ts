/** 🦴 The skeleton of `humans/walking-commuter` (man of 1.78 m) as the structural list of each scale, stated bone by bone: braincase, maxilla and mandible, seven cervical, twelve thoracic and five lumbar vertebrae with discs and processes on the lordosis and kyphosis of the upright spine, twelve rib pairs from their vertebra round the flank to the sternum or the costal arch, clavicles and scapulae, sacrum, iliac blades, pubic and ischial rami, long bones with heads and condyles, carpus, metacarpals and every phalanx, tarsus, metatarsals and toes. Every bone touches the one that carries it; heel and sole pads of the shoes carry the feet to the ground. Coarser scales merge what falls below their feature (15, 30, 75 mm).
 *
 * Bone-local frames: u along the bone, v its sagittal normal (backwards for a bone that points up, ahead for one that points down), w to the left. Figure coordinates: x ahead, y up, z left, default pose.
 * @see ./rework-walking-commuter-build.ts
 * @see ./rework/assets/humans--walking-commuter.md */
import type { Json } from "./volumes-lib.ts";
import { box, capsule, ellipsoid, loft, mirrored, part, revolve, tube, tubeStations, type Kit, type V3 } from "./rework-04-kit.ts";

const BONE = { material: "bone", tone: 0.62 };
/** 🫁 Rib rings in the thorax frame, rib 1 to 12: half breadth, half depth, the angle at which the bony rib ends and the fall of its front end below its vertebra. */
const RIB_A = [0.058, 0.086, 0.108, 0.123, 0.133, 0.141, 0.146, 0.147, 0.143, 0.135, 0.12, 0.098];
const RIB_B = [0.06, 0.069, 0.077, 0.084, 0.089, 0.093, 0.097, 0.098, 0.097, 0.093, 0.086, 0.078];
const RIB_END = [168, 169, 170, 171, 172, 172, 172, 150, 138, 125, 96, 72];
const RIB_DROP = [0.045, 0.055, 0.065, 0.073, 0.08, 0.088, 0.095, 0.1, 0.095, 0.085, 0.05, 0.028];
const RIB_ANGLE = 0.05;
const STERNUM_LEN = 0.2;
/** ✋ Knuckles in the hand frame [along, towards the thumb] and phalanx lengths; the same numbers as the rig. */
const RAYS: [string, number, number][] = [["index", 0.092, 0.028], ["middle", 0.095, 0.008], ["ring", 0.09, -0.012], ["little", 0.08, -0.031]];

export function skeleton(asset: Json, kit: Kit, scale: 50 | 100 | 200 | 500, back: (u: number) => number, nape: (u: number) => number): Json[] {
  const out: Json[] = [];
  const floor = { 50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075 }[scale];
  const fine = scale === 50;
  const mid = scale <= 100;
  const r = (x: number): number => Math.max(x, floor / 2);
  const boneOf = (id: string): Json => (asset.rig.bones as Json[]).find((b) => b.id === id);
  const len = (id: string): number => boneOf(id).lengthM as number;
  const m = (solid: Json, member: string): void => { out.push(part(solid, member, "member")); };
  const both = (solid: Json, member: string): void => { m(solid, member); m(mirrored(solid), member.replace(/-l(?=$|-)/, "-r")); };
  const b = (id: string, bone: string): { id: string; bone: string; material: string; tone: number } => ({ id, bone, ...BONE });
  const joint = (member: string, at: V3, rr: number): void => { out.push(part(ellipsoid({ id: `joint-${member}`, bone: member, material: "bone", tone: 0.4 }, [r(rr), r(rr), r(rr)], at), member, "joint")); };
  const mass = (member: string): void => {
    const k = boneOf(member).mass;
    if (!k) return;
    const rr = r(0.0125 * Math.cbrt(k.kg));
    out.push(part(ellipsoid({ id: `mass-${member}`, bone: member, material: "bone", tone: 0.2, role: "detail" }, [rr, rr, rr], [k.comX, k.comY, 0]), member, "mass"));
  };
  const { L, S, st } = kit;
  const LUMBAR = len("lumbar");
  const THORAX = len("torso");
  const NECK = len("neck");

  /* pelvis: sacrum, iliac blades, their bodies over the sockets, pubic and ischial rami */
  const P = (x: number, y: number, z = 0): V3 => L("pelvis", [x, y, z]);
  const s1 = -back(0);
  m(tube(b("sk-sacrum", "pelvis"), [P(s1 - 0.002, 1.064), P(s1 - 0.03, 1.022), P(s1 - 0.052, 0.978), P(s1 - 0.05, 0.94)], [[r(0.052), r(0.021)], [r(0.046), r(0.018)], [r(0.027), r(0.012)], [r(0.011), r(0.008)]], [1, 0, 0]), "pelvis");
  const I = (x: number, y: number, z = 0): V3 => L("ilium-l", [x, y, z]);
  if (mid) {
    both(loft(b("sk-ilium-l", "ilium-l"), [S("ilium-l", [s1 - 0.024, 1.03, 0.052], [0.35, 1, 0], [0.6, -0.2, -0.78], 0.042, r(0.0085), 2.2), S("ilium-l", [-0.036, 1.035, 0.1], [0.2, 1, 0], [0.45, -0.1, -0.88], 0.062, r(0.007), 2.2), S("ilium-l", [-0.004, 1.03, 0.138], [0.1, 1, 0], [0.2, -0.25, -0.95], 0.058, r(0.007), 2.2), S("ilium-l", [0.038, 1.02, 0.149], [0, 1, 0], [-0.35, -0.2, -0.9], 0.036, r(0.0075), 2.2)]), "ilium-l");
    both(capsule(b("sk-ilium-body-l", "ilium-l"), I(-0.004, 0.976, 0.098), I(-0.024, 1.03, 0.114), r(0.019), r(0.015)), "ilium-l");
    both(tube(b("sk-pubis-ischium-l", "ilium-l"), [I(0.004, 0.964, 0.086), I(0.042, 0.946, 0.048), I(0.064, 0.928, 0.008), I(0.034, 0.89, 0.027), I(-0.036, 0.865, 0.062), I(-0.054, 0.91, 0.08), I(-0.02, 0.96, 0.094)], [r(0.0115), r(0.0095), r(0.01), r(0.0085), r(0.013), r(0.0125), r(0.0125)]), "ilium-l");
  } else {
    both(tube(b("sk-hip-bone-l", "ilium-l"), [I(0.03, 0.935, 0.012), I(0, 0.96, 0.1), I(-0.02, 1.04, 0.13), I(s1 - 0.02, 1.05, 0.045)], [r(0.022)]), "ilium-l");
  }

  /* spine */
  const TU: number[] = [];
  const TP: number[] = [];
  let top = 0;
  for (let j = 0; j < 12; j++) { const pitch = 0.029 - (j * 0.009) / 11; TU[12 - j] = top + pitch / 2; TP[12 - j] = pitch; top += pitch; }
  const TV = (i: number): number => back(TU[i]! + LUMBAR);
  const vertebra = (name: string, bone: string, u: number, v: number, pitch: number, depth: number, width: number, process: [number, number], transverse: number): void => {
    if (!fine) { m(ellipsoid(b(`sk-${name}`, bone), [Math.max(pitch * 0.56, floor / 2), r(depth / 2), r(width / 2)], [u, v, 0]), bone); return; }
    m(box(b(`sk-${name}`, bone), [pitch * 0.8, depth, width], [u, v, 0], undefined, Math.min(0.007, pitch * 0.22)), bone);
    m(ellipsoid(b(`sk-disc-${name}`, bone), [Math.max(0.004, pitch * 0.2), depth * 0.46, width * 0.46], [u - pitch / 2, v, 0]), bone);
    m(capsule(b(`sk-process-${name}`, bone), [u, v + depth * 0.4, 0], [u - process[0], v + depth * 0.4 + process[1], 0], 0.005, 0.0042), bone);
    if (transverse > 0) m(capsule(b(`sk-transverse-${name}`, bone), [u, v + depth * 0.3, -transverse], [u, v + depth * 0.3, transverse], 0.0045, 0.0045), bone);
  };
  if (mid) {
    for (let i = 0; i < 5; i++) { const u = 0.018 + 0.036 * i; vertebra(`l${5 - i}`, "lumbar", u, back(u), 0.036, 0.034, 0.048 - i * 0.001, [0.012, 0.038], 0.042); }
    for (let i = 12; i >= 1; i--) vertebra(`t${i}`, "torso", TU[i]!, TV(i), TP[i]!, 0.031 - (12 - i) * 0.0011, 0.042 - (12 - i) * 0.0012, [0.02, 0.03], 0.03 - (12 - i) * 0.0004);
    const pitch = NECK / 7;
    for (let i = 0; i < 7; i++) { const u = pitch * (i + 0.5); vertebra(`c${7 - i}`, "neck", u, nape(Math.max(u, 0.02)), pitch, i === 6 ? 0.022 : 0.017, i === 6 ? 0.046 : 0.025, i === 0 ? [0.006, 0.034] : [0.005, 0.022], 0); }
  } else {
    m(tube(b("sk-lumbar-spine", "lumbar"), [[-0.006, back(0), 0], [LUMBAR / 2, back(LUMBAR / 2), 0], [LUMBAR + 0.006, back(LUMBAR), 0]], [r(0.02)]), "lumbar");
    m(tube(b("sk-spine", "torso"), [0, 0.25, 0.5, 0.75, 1].map((t) => [t * THORAX, back(t * THORAX + LUMBAR), 0] as V3), [r(0.016)]), "torso");
    m(tube(b("cervical-spine", "neck"), [[-0.006, nape(0.02), 0], [NECK / 2, nape(NECK / 2), 0], [NECK + 0.004, nape(NECK), 0]], [r(0.0125)]), "neck");
  }

  /* rib cage */
  const ringBack = (i: number): number => TV(i) + RIB_ANGLE;
  const SC_U = TU[1]! - RIB_DROP[0]!;
  const V0 = ringBack(1) - 2 * RIB_B[0]! - 0.004;
  const V1 = ringBack(7) - 2 * RIB_B[6]!;
  const sternumAt = (t: number): V3 => [SC_U + 0.008 - t * STERNUM_LEN, V0 + (V1 - V0) * t, 0];
  const ribPath = (i: number): V3[] => {
    const [a, bb, end, drop] = [RIB_A[i - 1]!, RIB_B[i - 1]!, RIB_END[i - 1]!, RIB_DROP[i - 1]!];
    const path: V3[] = [[TU[i]!, TV(i) + 0.006, 0.01]];
    for (let j = 1; j <= 9; j++) {
      const deg = 12 + (end - 12) * (j / 9);
      const s = Math.pow((deg - 12) / 168, 1.25);
      path.push([TU[i]! - drop * s, ringBack(i) - bb + bb * Math.cos((deg * Math.PI) / 180), a * Math.sin((deg * Math.PI) / 180)]);
    }
    if (i <= 7) path.push([path[9]![0], sternumAt((SC_U + 0.008 - path[9]![0]) / STERNUM_LEN)[1], 0.008]);
    return path;
  };
  if (mid) {
    for (const i of fine ? [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12] : [1, 3, 5, 7, 9, 11]) {
      const full = tubeStations(ribPath(i), [[r(0.004), r(0.0065)]], [1, 0, 0]);
      both(loft(b(`sk-rib-${i}-l`, "torso"), fine ? full : full.filter((_, j) => j % 3 === 0 || j === full.length - 1)), "torso");
    }
    const dir = [sternumAt(1)[0] - sternumAt(0)[0], sternumAt(1)[1] - sternumAt(0)[1]];
    const n: V3 = [dir[1]! / Math.hypot(dir[0]!, dir[1]!), -dir[0]! / Math.hypot(dir[0]!, dir[1]!), 0];
    m(loft(b("sk-sternum", "torso"), [0, 0.22, 0.3, 0.75, 0.86, 1].map((t, j) => ({ at: sternumAt(t), ex: [0, 0, 1] as V3, ey: n, rx: r([0.031, 0.027, 0.018, 0.021, 0.012, 0.007][j]!), ry: r([0.0072, 0.0066, 0.0056, 0.0056, 0.005, 0.004][j]!), e: 2.6 }))), "torso");
    const end = (i: number): V3 => ribPath(i)[9]!;
    const start = sternumAt(0.8);
    both(tube(b("sk-costal-arch-l", "torso"), fine ? [[start[0], start[1], 0.012], end(8), end(9), end(10)] : [[start[0], start[1], 0.012], end(9)], [r(0.005)]), "torso");
  } else {
    const c = ringBack(6) - RIB_B[5]!;
    m(loft(b("sk-rib-cage", "torso"), [st(TU[12]!, c + 0.012, 0, 0.112, 0.08, 2.2), st(TU[8]!, c, 0, 0.147, 0.098, 2.2), st(TU[4]!, c + 0.006, 0, 0.123, 0.084, 2.2), st(TU[1]!, c + 0.016, 0, 0.062, 0.06, 2.2)]), "torso");
  }

  /* shoulder girdle on the clavicle bones: clavicle from the jugular notch to the acromion, scapula on the back of the ribs */
  const G = (x: number, y: number, z: number): V3 => L("clavicle-l", [x, y, z]);
  const notch = sternumAt(0);
  both(tube(b("sk-clavicle-l", "clavicle-l"), [L("clavicle-l", kit.W("torso", [notch[0] + 0.004, notch[1], 0.02])), G(0.078, 1.474, 0.07), G(0.03, 1.486, 0.132), G(-0.006, 1.49, 0.188)], [r(0.0088), r(0.0062), r(0.0066), r(0.0078)]), "clavicle-l");
  if (mid) {
    both(loft(b("sk-scapula-l", "clavicle-l"), [S("clavicle-l", [-0.098, 1.4, 0.076], [0, 1, 0], [1, 0, -0.35], 0.068, r(0.004), 2.4), S("clavicle-l", [-0.093, 1.41, 0.122], [0, 1, 0], [1, 0, -0.5], 0.052, r(0.004), 2.4), S("clavicle-l", [-0.058, 1.438, 0.166], [0, 1, 0], [1, 0, -0.9], 0.03, r(0.0056), 2.4), S("clavicle-l", [-0.014, 1.45, 0.183], [0, 1, 0], [1, 0, -1.2], 0.022, r(0.01), 2.4)]), "clavicle-l");
    both(capsule(b("sk-scapular-spine-l", "clavicle-l"), G(-0.1, 1.446, 0.08), G(-0.008, 1.49, 0.19), r(0.0056), r(0.0072)), "clavicle-l");
  } else if (scale === 200) both(capsule(b("sk-scapula-l", "clavicle-l"), G(-0.09, 1.4, 0.1), G(-0.012, 1.478, 0.186), r(0.016), r(0.016)), "clavicle-l");

  /* arm */
  const AL = len("arm-l");
  const FL = len("forearm-l");
  const long = scale === 500 ? { slender: true } : {};
  const thin = (x: number): number => (scale === 500 ? x : r(x));
  both(tube(b("humerus-l", "arm-l"), [[mid ? 0.018 : -0.012, 0, 0], [0.1, 0.003, 0], [0.23, 0.002, 0], [mid ? AL - 0.015 : AL + 0.006, 0, 0]], [thin(0.0138), thin(0.0118), thin(0.0108), thin(0.0128)], [0, 1, 0], long), "arm-l");
  if (mid) {
    both(ellipsoid(b("humerus-head-l", "arm-l"), [r(0.0245), r(0.0235), r(0.0225)], [0.002, 0, -0.004]), "arm-l");
    both(ellipsoid(b("humerus-condyles-l", "arm-l"), [r(0.0145), r(0.0145), r(0.032)], [AL - 0.004, 0, 0]), "arm-l");
    both(tube(b("ulna-l", "forearm-l"), [[-0.018, -0.014, 0.002], [0.02, -0.012, 0.004], [0.14, -0.014, 0.014], [FL - 0.002, -0.015, 0.027]], [r(0.0112), r(0.0086), r(0.0066), r(0.0056)]), "forearm-l");
    both(tube(b("radius-l", "forearm-l"), [[0.004, 0.011, 0.004], [0.06, 0.014, 0.008], [0.17, 0.014, 0.018], [FL, 0.012, 0.03]], [r(0.0082), r(0.0064), r(0.0076), r(0.0118)]), "forearm-l");
  } else {
    both(tube(b("ulna-l", "forearm-l"), [[-0.01, 0, 0], [FL / 2, 0, 0.012], [FL + 0.004, 0, 0.028]], [r(0.012)]), "forearm-l");
  }
  for (const side of ["l", "r"] as const) {
    const h = `hand-${side}`;
    const rig = (id: string): number => len(id);
    if (fine) {
      m(ellipsoid(b(`carpals-${side}`, h), [0.016, 0.027, 0.011], [0.012, 0.002, 0]), h);
      for (const [f, u, v] of RAYS) {
        m(capsule(b(`sk-${f}-metacarpal-${side}`, h), [0.024, v * 0.55, 0], [u, v, 0], 0.0052, 0.006), h);
        ["proximal", "middle", "distal"].forEach((s, i) => { const id = `${f}-${s}-${side}`; m(capsule(b(`sk-${id}`, id), [0, 0, 0], [rig(id), 0, 0], 0.0048 - i * 0.0003, 0.0042 - i * 0.0002), id); });
      }
      ["metacarpal", "proximal", "distal"].forEach((s, i) => { const id = `thumb-${s}-${side}`; m(capsule(b(`sk-${id}`, id), [0, 0, 0], [rig(id), 0, 0], 0.006 - i * 0.0006, 0.0052 - i * 0.0004), id); });
    } else if (scale === 100) {
      m(loft(b(`hand-bones-${side}`, h), [st(-0.004, 0.001, 0, 0.0085, 0.026), st(0.03, 0.001, 0, 0.008, 0.034), st(0.094, -0.001, 0, 0.0075, 0.041)]), h);
      m(loft(b(`finger-bones-${side}`, `middle-proximal-${side}`), [st(-0.004, -0.01, 0, 0.0075, 0.037), st(0.047, -0.01, 0, 0.0075, 0.033)]), `middle-proximal-${side}`);
      m(loft(b(`finger-end-bones-${side}`, `middle-middle-${side}`), [st(-0.004, -0.01, 0, 0.0075, 0.033), st(0.042, -0.01, 0, 0.0075, 0.028)]), `middle-middle-${side}`);
      m(capsule(b(`thumb-bones-${side}`, `thumb-metacarpal-${side}`), [-0.006, 0, 0], [0.072, 0, 0], 0.0075, 0.0075), `thumb-metacarpal-${side}`);
    } else {
      m(loft(b(`hand-bones-${side}`, h), [st(-0.006, 0, 0, r(0.012), r(0.024)), st(0.06, 0, 0, r(0.011), 0.038), st(side === "l" ? 0.15 : 0.1, 0, 0, r(0.009), r(0.03))]), h);
    }
  }

  /* leg */
  const TL = len("thigh-l");
  const SL = len("shank-l");
  both(tube(b("femur-l", "thigh-l"), mid ? [[0.05, -0.006, 0.056], [0.15, 0.006, 0.042], [0.29, 0.01, 0.018], [TL - 0.018, 0.002, 0.002]] : [[-0.004, 0, 0.004], [0.05, -0.006, 0.05], [0.29, 0.01, 0.018], [TL + 0.006, 0.002, 0.002]], [thin(0.017), thin(0.0146), thin(0.0142), thin(0.019)], [0, 1, 0], long), "thigh-l");
  if (mid) {
    both(ellipsoid(b("femur-head-l", "thigh-l"), [r(0.0245), r(0.0245), r(0.0245)], [0, 0, 0]), "thigh-l");
    both(capsule(b("femur-neck-l", "thigh-l"), [0.004, 0, 0.008], [0.04, -0.008, 0.056], r(0.015), r(0.0175)), "thigh-l");
    both(ellipsoid(b("femur-trochanter-l", "thigh-l"), [r(0.025), r(0.0175), r(0.0145)], [0.03, -0.012, 0.066]), "thigh-l");
    both(ellipsoid(b("femur-condyles-l", "thigh-l"), [r(0.023), r(0.033), r(0.042)], [TL - 0.006, -0.004, 0]), "thigh-l");
    both(ellipsoid(b("patella-l", "thigh-l"), [r(0.022), r(0.011), r(0.022)], [TL - 0.004, 0.037, 0]), "thigh-l");
  }
  both(tube(b("tibia-l", "shank-l"), [[mid ? 0.004 : -0.006, 0, -0.002], [0.036, 0.006, -0.004], [0.12, 0.009, -0.006], [0.31, 0.004, -0.006], [mid ? SL - 0.004 : SL + 0.008, 0, -0.004]], scale === 500 ? [0.02, 0.018, 0.014, 0.012, 0.016] : [[r(0.038), r(0.027)], r(0.0205), r(0.0145), r(0.0122), r(0.0165)], [0, 1, 0], long), "shank-l");
  if (mid) {
    both(tube(b("fibula-l", "shank-l"), [[0.022, -0.012, 0.03], [0.2, -0.012, 0.033], [SL + 0.008, -0.008, 0.027]], [r(0.0092), r(0.0056), r(0.0088)]), "shank-l");
    both(ellipsoid(b("malleolus-l", "shank-l"), [r(0.0145), r(0.0115), r(0.0095)], [SL + 0.002, 0, -0.026]), "shank-l");
  }

  /* foot: figure coordinates relative to the left ankle (0, 0.099, 0.0895), sole on the ground */
  const ankle = kit.FRAMES["foot-l"]!.o;
  const F = (x: number, y: number, z: number): V3 => L("foot-l", [ankle[0] + x, y, ankle[2] + z]);
  const E = (x: number, y: number, z: number): V3 => L("toe-l", [ankle[0] + x, y, ankle[2] + z]);
  if (mid) {
    both(ellipsoid(b("talus-l", "foot-l"), [r(0.025), r(0.017), r(0.021)], F(0.006, 0.076, 0)), "foot-l");
    both(tube(b("calcaneus-l", "foot-l"), [F(-0.046, 0.05, 0.002), F(-0.02, 0.05, 0.003), F(0.014, 0.058, 0.004)], [[r(0.0175), r(0.0195)], [r(0.0175), r(0.0205)], [r(0.0165), r(0.0145)]]), "foot-l");
    both(tube(b("midfoot-l", "foot-l"), [F(0.022, 0.066, 0), F(0.05, 0.06, 0), F(0.076, 0.052, 0)], [[r(0.024), r(0.014)], [r(0.027), r(0.0145)], [r(0.029), r(0.012)]]), "foot-l");
    if (fine) {
      const rays: [number, number, number, number][] = [[-0.03, 0.172, 0.214, 0.0072], [-0.014, 0.176, 0.222, 0.0056], [0.002, 0.171, 0.216, 0.0053], [0.017, 0.163, 0.207, 0.0051], [0.031, 0.153, 0.196, 0.0049]];
      rays.forEach(([z, ball, tip, rr], i) => {
        both(capsule(b(`metatarsal-${i + 1}-l`, "foot-l"), F(0.068, 0.054 - 0.003 * i, z * 0.6), F(ball - 0.006, 0.031, z), rr, rr + 0.001), "foot-l");
        both(capsule(b(`toe-phalanges-${i + 1}-l`, "toe-l"), E(ball - 0.004, 0.03, z), E(tip, 0.027, z * 1.02), rr, rr - 0.0008), "toe-l");
      });
    } else {
      both(loft(b("metatarsus-l", "foot-l"), tubeStations([F(0.066, 0.052, 0), F(0.115, 0.04, 0), F(0.165, 0.031, 0)], [[0.03, 0.0085], [0.036, 0.008], [0.04, 0.0078]])), "foot-l");
      both(loft(b("phalanges-l", "toe-l"), tubeStations([E(0.16, 0.031, 0), E(0.214, 0.027, 0)], [[0.04, 0.0078], [0.03, 0.0076]])), "toe-l");
    }
  } else {
    both(tube(b("foot-bones-l", "foot-l"), [F(-0.04, 0.05, 0), F(0.01, 0.066, 0), F(0.085, 0.05, 0), F(0.166, 0.036, 0)], [[r(0.02), r(0.02)], [r(0.024), r(0.02)], [r(0.03), r(0.016)], [r(0.036), r(0.012)]]), "foot-l");
    if (scale === 200) both(tube(b("phalanges-l", "toe-l"), [E(0.156, 0.034, 0), E(0.214, 0.03, 0)], [[r(0.036), r(0.012)], [r(0.026), r(0.01)]]), "toe-l");
  }

  /* skull */
  m(loft(b("cranium", "head"), [st(-0.004, 0.014, 0, 0.04, 0.05, 2.2), st(0.008, 0.012, 0, 0.054, 0.066, 2.2), st(0.03, 0.008, 0, 0.0665, 0.083, 2.2), st(0.06, 0.004, 0, 0.071, 0.091, 2.2), st(0.09, 0.003, 0, 0.068, 0.088, 2.1), st(0.118, 0.004, 0, 0.054, 0.07, 2), st(0.134, 0.005, 0, 0.034, 0.044, 2), st(0.142, 0.005, 0, 0.012, 0.016, 2)]), "head");
  if (scale <= 200) {
    m(ellipsoid(b("face-bones", "head"), [r(0.03), r(0.026), r(0.032)], [-0.016, -0.05, 0]), "head");
    m(tube(b("mandible", "head"), [[0.006, -0.012, 0.05], [-0.056, -0.004, 0.048], [-0.07, -0.038, 0.034], [-0.077, -0.064, 0], [-0.07, -0.038, -0.034], [-0.056, -0.004, -0.048], [0.006, -0.012, -0.05]], [[r(0.0056), r(0.012)], [r(0.006), r(0.013)], [r(0.006), r(0.011)], [r(0.008), r(0.013)], [r(0.006), r(0.011)], [r(0.006), r(0.013)], [r(0.0056), r(0.012)]], [1, 0, 0]), "head");
  }
  if (fine) {
    both(capsule(b("zygomatic-arch-l", "head"), [-0.002, -0.06, 0.046], [0.01, -0.006, 0.056], 0.0058, 0.0048), "head");
    both(ellipsoid({ id: "orbit-l", bone: "head", material: "bone", tone: 0.3, role: "detail" }, [0.016, 0.008, 0.018], [0.02, -0.07, 0.032]), "head");
  }

  /* the load path ends on the ground: heel and sole of each shoe carry the foot bones */
  for (const side of ["l", "r"] as const) {
    const bone = `foot-${side}`;
    const f = kit.frame(bone);
    const up: V3 = [f.u[1], f.v[1], f.w[1]];
    const pad = (id: string, x: number, topY: number, rr: number): void => m(revolve({ id, bone, material: "leather-shoe", tone: 0.3 }, [[0, 0], [0, r(rr)], [topY, r(rr)], [topY, 0]], L(bone, [f.o[0] + x, 0, f.o[2]]), up), bone);
    pad(`heel-pad-${side}`, -0.028, Math.max(0.036, floor), 0.02);
    pad(`sole-pad-${side}`, 0.15, Math.max(0.028, floor), 0.024);
  }

  /* joints and masses */
  if (scale <= 200) {
    joint("lumbar", [0, back(0), 0], 0.017);
    joint("torso", [0, back(LUMBAR), 0], 0.015);
    joint("neck", [0, nape(0.02), 0], 0.012);
    joint("head", [0, 0, 0], 0.011);
  }
  for (const side of ["l", "r"]) {
    joint(`thigh-${side}`, [0, 0, 0], 0.0255);
    joint(`shank-${side}`, [0, 0, 0], 0.02);
    joint(`arm-${side}`, [0, 0, 0], 0.0255);
    joint(`forearm-${side}`, [0, 0, 0], 0.016);
    if (scale <= 200) { joint(`foot-${side}`, [0, 0.002, 0], 0.016); joint(`hand-${side}`, [0, 0, 0], 0.0125); joint(`clavicle-${side}`, [0, 0, 0], 0.011); }
    if (mid) joint(`toe-${side}`, [-0.004, 0.02, 0], 0.01);
  }
  const masses = mid ? ["pelvis", "lumbar", "torso", "head", "thigh-l", "thigh-r", "shank-l", "shank-r", "arm-l", "arm-r", "forearm-l", "forearm-r", "foot-l", "foot-r"] : scale === 200 ? ["pelvis", "lumbar", "torso", "head", "thigh-l", "thigh-r"] : ["pelvis", "torso"];
  for (const id of masses) mass(id);
  return out;
}
