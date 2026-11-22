/** 🦴 The skeleton of `humans/elderly-with-cane` as the structural list, stated bone by bone for each scale: skull with mandible, seven cervical, twelve thoracic (kyphotic arc) and five lumbar vertebrae standing on each other, twelve pairs of ribs running from their vertebra round the flank to the sternum or the costal arch, clavicles and scapulae, pelvis with sacrum, long bones with their heads and condyles, hands and feet. Every bone touches the one that carries it and the load path ends in the support pads under heels, balls of the feet and the cane tip. Coarser scales merge what falls below their feature (15, 30, 75 mm) and never draw a bone thinner than that feature.
 *
 * Bone-local frames: u along the bone, v its sagittal normal (backwards for a bone that points up, ahead for one that points down), w to the left.
 * @see ./rework-04-elderly-with-cane-s50.ts */
import type { Json } from "./volumes-lib.ts";
import { capsule, ellipsoid, loft, mirrored, part, quatOf, revolve, tube, tubeStations, type V3 } from "./rework-04-kit.ts";
import { ANKLE_X, ASSET, D, HEAD, K, L, S, SHOE_Z, boneLen, headSt, st } from "./rework-04-elderly-with-cane-s50.ts";

const BONE = { material: "bone", tone: 0.62 };
const kgOf = (bone: string): Json | undefined => (ASSET.rig.bones as Json[]).find((b) => b.id === bone).mass;
export type Detail = { scale: 50 | 100 | 200 | 500; floor: number };

/** 🫁 The rib cage in the thorax frame: per rib the half-breadth and half-depth of its ring; the ring leaves the vertebra 38 mm behind the bodies and falls towards the sternum. */
const RIB_A = [0.056, 0.082, 0.102, 0.116, 0.126, 0.133, 0.138, 0.139, 0.136, 0.129, 0.116, 0.096];
const RIB_B = [0.0615, 0.07, 0.078, 0.085, 0.091, 0.096, 0.1015, 0.103, 0.102, 0.098, 0.09, 0.08];
const RIB_END = [168, 169, 170, 171, 172, 172, 172, 150, 138, 125, 96, 72];
const RIB_DROP = [0.016, 0.0258, 0.0356, 0.0454, 0.0552, 0.065, 0.075, 0.08, 0.078, 0.07, 0.045, 0.025];
const TH_LEN = boneLen("chest");
const vertebraU = (i: number): number => TH_LEN - 0.011 - 0.0225 * (i - 1);
/** 🌀 The backward bow of the thoracic bodies off the chord of the bone (the kyphosis of about 55 degrees Cobb). */
const kyphosis = (u: number): number => 0.03 * 4 * (u / TH_LEN) * (1 - u / TH_LEN);
const SC_U = vertebraU(1) - RIB_DROP[0]!;
const STERNUM_V0 = 0.038 - 2 * RIB_B[0]! - 0.004;
const STERNUM_V1 = 0.038 - 2 * RIB_B[6]!;
const STERNUM_LEN = 0.21;
/** 🦴 The sternum line from the jugular notch (t = 0) to the tip of the xiphoid (t = 1). */
const sternumAt = (t: number): V3 => [SC_U + 0.008 - t * STERNUM_LEN, STERNUM_V0 + (STERNUM_V1 - STERNUM_V0) * t, 0];
/** 🩻 One rib: from inside its vertebral body round the ring; the true ribs end on the sternum line. */
function ribPath(i: number, k: 1 | -1): V3[] {
  const [a, b, end, drop] = [RIB_A[i - 1]!, RIB_B[i - 1]!, RIB_END[i - 1]!, RIB_DROP[i - 1]!];
  const u0 = vertebraU(i);
  const out: V3[] = [[u0, kyphosis(u0) + 0.006, k * 0.01]];
  for (let j = 1; j <= 9; j++) {
    const deg = 12 + (end - 12) * (j / 9);
    const s = Math.pow((deg - 12) / 168, 1.25);
    out.push([u0 - drop * s, kyphosis(u0) * (1 - s) + 0.038 - b + b * Math.cos((deg * Math.PI) / 180), k * a * Math.sin((deg * Math.PI) / 180)]);
  }
  if (i <= 7) {
    const u = out[9]![0];
    out.push([u, sternumAt((SC_U + 0.008 - u) / STERNUM_LEN)[1], k * 0.008]);
  }
  return out;
}

export function skeleton(d: Detail): Json[] {
  const out: Json[] = [];
  const fine = d.scale === 50;
  const mid = d.scale <= 100;
  const r = (x: number): number => Math.max(x, d.floor / 2);
  const m = (solid: Json, member: string): void => { out.push(part(solid, member, "member")); };
  const both = (solid: Json, member: string): void => { m(solid, member); m(mirrored(solid), member.replace(/-l$/, "-r")); };
  const joint = (member: string, at: V3, rr: number): void => { out.push(part(ellipsoid({ id: `joint-${member}`, bone: member, material: "bone", tone: 0.4 }, [r(rr), r(rr), r(rr)], at), member, "joint")); };
  const mass = (member: string): void => {
    const k = kgOf(member);
    if (!k) return;
    const rr = r(0.0125 * Math.cbrt(k.kg));
    out.push(part(ellipsoid({ id: `mass-${member}`, bone: member, material: "bone", tone: 0.2, role: "detail" }, [rr, rr, rr], [k.comX, k.comY, 0]), member, "mass"));
  };
  const b = (id: string, bone: string): { id: string; bone: string; material: string; tone: number } => ({ id, bone, ...BONE });

  /* pelvis: sacrum, iliac blades with their body over the socket, pubic and ischial rami */
  const P = (x: number, y: number, z = 0): V3 => L("pelvis", [x, y, z]);
  m(tube(b("sacrum", "pelvis"), [P(-0.036, 0.972), P(-0.062, 0.94), P(-0.082, 0.9), P(-0.08, 0.868)], [[r(0.05), r(0.02)], [r(0.045), r(0.017)], [r(0.026), r(0.011)], [r(0.011), r(0.008)]], [1, 0, 0]), "pelvis");
  if (mid) {
    const ilium = loft(b("ilium-l", "pelvis"), [S("pelvis", [-0.052, 0.95, 0.05], [0.35, 1, 0], [0.6, -0.2, -0.78], 0.04, r(0.0085), 2.2), S("pelvis", [-0.03, 0.965, 0.092], [0.2, 1, 0], [0.45, -0.1, -0.88], 0.066, r(0.007), 2.2), S("pelvis", [-0.005, 0.962, 0.128], [0.1, 1, 0], [0.2, -0.25, -0.95], 0.062, r(0.007), 2.2), S("pelvis", [0.03, 0.948, 0.142], [0, 1, 0], [-0.35, -0.2, -0.9], 0.036, r(0.0075), 2.2)]);
    m(ilium, "pelvis");
    m(mirrored(ilium), "pelvis");
    const socket = capsule(b("ilium-body-l", "pelvis"), P(-0.004, 0.9, 0.094), P(-0.022, 0.95, 0.108), r(0.018), r(0.014));
    m(socket, "pelvis");
    m(mirrored(socket), "pelvis");
    const rami = tube(b("pubis-ischium-l", "pelvis"), [P(0.004, 0.889, 0.082), P(0.038, 0.872, 0.045), P(0.058, 0.855, 0.008), P(0.03, 0.82, 0.025), P(-0.035, 0.797, 0.058), P(-0.05, 0.84, 0.075), P(-0.02, 0.885, 0.09)], [r(0.011), r(0.009), r(0.0095), r(0.008), r(0.0125), r(0.012), r(0.012)]);
    m(rami, "pelvis");
    m(mirrored(rami), "pelvis");
  } else {
    m(tube(b("pelvic-ring", "pelvis"), [P(0, 0.885, 0.098), P(-0.02, 0.95, 0.12), P(-0.05, 0.955, 0.05), P(-0.05, 0.955, -0.05), P(-0.02, 0.95, -0.12), P(0, 0.885, -0.098)], [r(0.022)]), "pelvis");
  }

  /* spine: the bodies stand on each other */
  const LUMBAR = boneLen("torso");
  if (mid) {
    for (let i = 0; i < 5; i++) {
      const u = 0.0165 + 0.033 * i;
      m(ellipsoid(b(`vertebra-l${5 - i}`, "torso"), [0.017, r(0.0175), r(0.0235)], [u, 0, 0]), "torso");
      if (fine) m(capsule(b(`spinous-l${5 - i}`, "torso"), [u, 0.012, 0], [u - 0.008, 0.062, 0], 0.0055, 0.0045), "torso");
    }
    for (let i = 1; i <= 12; i++) {
      const u = vertebraU(i);
      const k = (i - 1) / 11;
      m(ellipsoid(b(`vertebra-t${i}`, "chest"), [0.0118, r(0.0115 + 0.004 * k), r(0.0145 + 0.006 * k)], [u, kyphosis(u), 0]), "chest");
      if (fine) m(capsule(b(`spinous-t${i}`, "chest"), [u - 0.002, kyphosis(u) + 0.008, 0], [u - 0.026, kyphosis(u) + 0.047, 0], 0.0048, 0.004), "chest");
    }
    for (let i = 0; i < 7; i++) {
      const u = 0.008 + 0.0145 * i;
      m(ellipsoid(b(`vertebra-c${7 - i}`, "neck"), [0.0078, r(0.0105), r(0.0135 - 0.0005 * i)], [u, 0, 0]), "neck");
      if (fine && i < 6) m(capsule(b(`spinous-c${7 - i}`, "neck"), [u, 0.006, 0], [u - 0.006, i === 0 ? 0.04 : 0.032, 0], 0.0042, 0.0038), "neck");
    }
  } else {
    m(tube(b("lumbar-spine", "torso"), [[-0.004, 0, 0], [LUMBAR / 2, -0.004, 0], [LUMBAR + 0.004, 0, 0]], [r(0.02)]), "torso");
    m(tube(b("thoracic-spine", "chest"), [0, 0.25, 0.5, 0.75, 1].map((t) => [t * TH_LEN, kyphosis(t * TH_LEN), 0] as V3), [r(0.016), r(0.0135)]), "chest");
    m(tube(b("cervical-spine", "neck"), [[-0.004, 0, 0], [boneLen("neck") + 0.004, 0, 0]], [r(0.0125)]), "neck");
  }

  /* rib cage: every rib from its vertebra to the sternum or the costal arch */
  if (mid) {
    for (const i of fine ? [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12] : [1, 3, 5, 7, 9, 11]) {
      const full = tubeStations(ribPath(i, 1), [[r(0.004), r(0.0065)]], [1, 0, 0]);
      both(loft(b(`rib-${i}-l`, "chest"), fine ? full : full.filter((_, j) => j % 3 === 0 || j === full.length - 1)), "chest");
    }
    const dir = [sternumAt(1)[0] - sternumAt(0)[0], sternumAt(1)[1] - sternumAt(0)[1]];
    const n: V3 = [dir[1]! / Math.hypot(dir[0]!, dir[1]!), -dir[0]! / Math.hypot(dir[0]!, dir[1]!), 0];
    m(loft(b("sternum", "chest"), [0, 0.22, 0.3, 0.75, 0.86, 1].map((t, j) => ({ at: sternumAt(t), ex: [0, 0, 1] as V3, ey: n, rx: r([0.03, 0.026, 0.017, 0.02, 0.012, 0.007][j]!), ry: r([0.007, 0.0065, 0.0055, 0.0055, 0.005, 0.004][j]!), e: 2.6 }))), "chest");
    const end = (i: number): V3 => ribPath(i, 1)[9]!;
    const start = sternumAt(0.8);
    const arch = tube(b("costal-arch-l", "chest"), fine ? [[start[0], start[1], 0.012], end(8), end(9), end(10)] : [[start[0], start[1], 0.012], end(9)], [r(0.005)]);
    m(arch, "chest");
    m(mirrored(arch), "chest");
  } else {
    m(loft(b("rib-cage", "chest"), [st(0.012, -0.05, 0, 0.105, 0.085, 2.2), st(0.11, -0.06, 0, 0.132, 0.098, 2.2), st(0.2, -0.04, 0, 0.11, 0.08, 2.2), st(0.262, -0.024, 0, 0.062, 0.06, 2.2)]), "chest");
  }

  /* shoulder girdle */
  const CL = boneLen("clavicle-l");
  both(tube(b("clavicle-l", "clavicle-l"), [[-0.004, 0, 0], [0.04, -0.007, 0], [0.085, 0.002, 0], [0.125, 0.011, 0], [CL, 0.004, 0]], [r(0.0085), r(0.006), r(0.0058), r(0.0065), r(0.0075)]), "clavicle-l");
  if (mid) {
    const blade = loft(b("scapula-l", "clavicle-l"), [S("clavicle-l", [-0.032, 1.288, 0.072], [0, 1, 0], [1, 0, -0.35], 0.06, r(0.004), 2.4), S("clavicle-l", [-0.016, 1.3, 0.115], [0, 1, 0], [1, 0, -0.5], 0.046, r(0.004), 2.4), S("clavicle-l", [0.03, 1.332, 0.153], [0, 1, 0], [1, 0, -0.9], 0.026, r(0.0055), 2.4), S("clavicle-l", [0.066, 1.343, 0.166], [0, 1, 0], [1, 0, -1.2], 0.02, r(0.009), 2.4)]);
    both(blade, "clavicle-l");
    both(capsule(b("scapular-spine-l", "clavicle-l"), L("clavicle-l", [-0.034, 1.33, 0.078]), L("clavicle-l", [0.088, 1.373, 0.172]), r(0.0055), r(0.007)), "clavicle-l");
  }

  /* arm */
  const AL = boneLen("arm-l");
  both(tube(b("humerus-l", "arm-l"), [[mid ? 0.018 : -0.012, 0, 0], [0.1, 0.003, 0], [0.22, 0.002, 0], [mid ? AL - 0.015 : AL + 0.006, 0, 0]], [r(0.0135), r(0.0115), r(0.0105), r(0.0125)]), "arm-l");
  if (mid) {
    both(ellipsoid(b("humeral-head-l", "arm-l"), [r(0.024), r(0.023), r(0.022)], [0.002, 0, -0.004]), "arm-l");
    both(ellipsoid(b("humeral-condyles-l", "arm-l"), [r(0.014), r(0.014), r(0.031)], [AL - 0.004, 0, 0]), "arm-l");
  }
  const FL = boneLen("forearm-l");
  if (mid) {
    both(tube(b("ulna-l", "forearm-l"), [[-0.018, -0.014, 0.002], [0.02, -0.012, 0.002], [0.13, -0.014, 0], [FL - 0.002, -0.015, 0]], [r(0.011), r(0.0085), r(0.0065), r(0.0055)]), "forearm-l");
    both(tube(b("radius-l", "forearm-l"), [[0.004, 0.011, 0.004], [0.06, 0.014, 0.002], [0.16, 0.014, 0], [FL, 0.012, 0]], [r(0.008), r(0.0062), r(0.0075), r(0.0115)]), "forearm-l");
  } else {
    both(tube(b("ulna-l", "forearm-l"), [[-0.01, 0, 0], [FL + 0.004, 0, 0]], [r(0.012)]), "forearm-l");
  }
  for (const side of ["l", "r"] as const) {
    const h = `hand-${side}`;
    if (fine) {
      m(ellipsoid(b(`carpus-${side}`, h), [0.0145, 0.027, 0.0105], [0.013, 0.002, 0]), h);
      const fingers: [string, number, number][] = [["index", 0.086, 0.03], ["middle", 0.088, 0.009], ["ring", 0.083, -0.011], ["little", 0.074, -0.03]];
      for (const [f, u, v] of fingers) {
        m(capsule(b(`metacarpal-${f}-${side}`, h), [0.026, v * 0.55, 0], [u, v, 0], 0.0052, 0.006), h);
        for (const s of ["proximal", "middle", "distal"]) { const id = `${f}-${s}-${side}`; m(capsule(b(`phalanx-${id}`, id), [0, 0, 0], [boneLen(id), 0, 0], 0.0048, 0.004), id); }
      }
      for (const s of ["metacarpal", "proximal", "distal"]) { const id = `thumb-${s}-${side}`; m(capsule(b(`phalanx-${id}`, id), [0, 0, 0], [boneLen(id), 0, 0], s === "metacarpal" ? 0.006 : 0.005, 0.0045), id); }
    } else if (d.scale === 100) {
      m(loft(b(`metacarpus-${side}`, h), [st(-0.004, 0.001, 0, 0.0085, 0.026), st(0.03, 0.001, 0, 0.008, 0.034), st(0.09, 0, 0, 0.0075, 0.04)]), h);
      m(loft(b(`phalanges-${side}`, `middle-proximal-${side}`), [st(-0.004, -0.011, 0, 0.0075, 0.036), st(0.046, -0.011, 0, 0.0075, 0.032)]), `middle-proximal-${side}`);
      m(loft(b(`phalanges-end-${side}`, `middle-middle-${side}`), [st(-0.004, -0.011, 0, 0.0075, 0.032), st(0.04, -0.011, 0, 0.0075, 0.028)]), `middle-middle-${side}`);
      m(capsule(b(`thumb-ray-${side}`, `thumb-metacarpal-${side}`), [-0.006, 0, 0], [0.07, 0, 0], 0.0075, 0.0075), `thumb-metacarpal-${side}`);
    } else {
      m(loft(b(`hand-bones-${side}`, h), [st(-0.006, 0, 0, r(0.012), 0.024), st(0.06, 0, 0, r(0.011), 0.038), st(d.scale === 200 ? 0.12 : 0.1, 0, 0, r(0.009), 0.03)]), h);
    }
  }

  /* leg */
  const TL = boneLen("thigh-l");
  both(tube(b("femur-l", "thigh-l"), mid ? [[0.05, -0.006, 0.056], [0.14, 0.006, 0.042], [0.27, 0.01, 0.018], [TL - 0.018, 0.002, 0.002]] : [[-0.004, 0, 0.004], [0.05, -0.006, 0.05], [0.27, 0.01, 0.018], [TL + 0.006, 0.002, 0.002]], [r(0.0165), r(0.0142), r(0.0138), r(0.0185)]), "thigh-l");
  if (mid) {
    both(ellipsoid(b("femoral-head-l", "thigh-l"), [r(0.0235), r(0.0235), r(0.0235)], [0, 0, 0]), "thigh-l");
    both(capsule(b("femoral-neck-l", "thigh-l"), [0.004, 0, 0.008], [0.04, -0.008, 0.056], r(0.0145), r(0.017)), "thigh-l");
    both(ellipsoid(b("greater-trochanter-l", "thigh-l"), [r(0.024), r(0.017), r(0.014)], [0.03, -0.012, 0.068]), "thigh-l");
    both(ellipsoid(b("femoral-condyles-l", "thigh-l"), [r(0.022), r(0.032), r(0.041)], [TL - 0.006, -0.004, 0]), "thigh-l");
    both(ellipsoid(b("patella-l", "thigh-l"), [r(0.021), r(0.0105), r(0.021)], [TL - 0.004, 0.036, 0]), "thigh-l");
  }
  const SL = boneLen("shank-l");
  both(tube(b("tibia-l", "shank-l"), [[mid ? 0.004 : -0.006, 0, -0.002], [0.035, 0.006, -0.004], [0.12, 0.009, -0.006], [0.3, 0.004, -0.006], [mid ? SL - 0.004 : SL + 0.008, 0, -0.004]], [[r(0.037), r(0.026)], r(0.02), r(0.014), r(0.0118), r(0.016)]), "shank-l");
  if (mid) {
    both(tube(b("fibula-l", "shank-l"), [[0.022, -0.012, 0.03], [0.2, -0.012, 0.032], [SL + 0.008, -0.008, 0.026]], [r(0.009), r(0.0055), r(0.0085)]), "shank-l");
    both(ellipsoid(b("medial-malleolus-l", "shank-l"), [r(0.014), r(0.011), r(0.009)], [SL + 0.002, 0, -0.025]), "shank-l");
  }

  /* foot, in figure coordinates of the standing shoe */
  const F = (x: number, y: number, z: number): V3 => L("foot-l", [ANKLE_X + x, y, SHOE_Z + z]);
  const T = (x: number, y: number, z: number): V3 => L("toe-l", [ANKLE_X + x, y, SHOE_Z + z]);
  if (mid) {
    both(ellipsoid(b("talus-l", "foot-l"), [r(0.024), r(0.016), r(0.02)], F(0.006, 0.07, 0)), "foot-l");
    both(tube(b("calcaneus-l", "foot-l"), [F(-0.056, 0.045, 0.002), F(-0.03, 0.045, 0.003), F(0.012, 0.05, 0.004)], [[r(0.017), r(0.019)], [r(0.017), r(0.02)], [r(0.016), r(0.014)]]), "foot-l");
    both(ellipsoid(b("tarsus-l", "foot-l"), [r(0.024), r(0.014), r(0.027)], F(0.05, 0.056, 0), quatOf(D("foot-l", [1, -0.25, 0]), [0, 0, 1])), "foot-l");
    if (fine) {
      const rays: [number, number, number, number][] = [[-0.03, 0.158, 0.19, 0.0072], [-0.014, 0.16, 0.2, 0.0055], [0.002, 0.155, 0.196, 0.0052], [0.017, 0.147, 0.189, 0.005], [0.031, 0.137, 0.178, 0.0048]];
      rays.forEach(([z, ball, tip, rr], i) => {
        both(capsule(b(`metatarsal-${i + 1}-l`, "foot-l"), F(0.066, 0.052 - 0.004 * i, z * 0.6), F(ball - 0.008, 0.024, z), rr, rr + 0.001), "foot-l");
        both(capsule(b(`toe-phalanges-${i + 1}-l`, "toe-l"), T(ball - 0.004, 0.023, z), T(tip, 0.02, z * 1.02), rr, rr - 0.0008), "toe-l");
      });
    } else {
      both(loft(b("metatarsus-l", "foot-l"), tubeStations([F(0.062, 0.05, 0), F(0.11, 0.036, 0), F(0.152, 0.024, 0)], [[0.03, 0.0085], [0.036, 0.008], [0.04, 0.0078]])), "foot-l");
      both(loft(b("toe-bones-l", "toe-l"), tubeStations([T(0.146, 0.024, 0), T(0.196, 0.021, 0)], [[0.04, 0.0078], [0.03, 0.0076]])), "toe-l");
    }
  } else {
    both(tube(b("foot-bones-l", "foot-l"), [F(-0.05, 0.045, 0), F(0.01, 0.062, 0), F(0.08, 0.045, 0), F(0.15, 0.03, 0)], [[r(0.02), r(0.02)], [r(0.024), r(0.02)], [r(0.03), r(0.016)], [r(0.036), r(0.012)]]), "foot-l");
    if (d.scale === 200) both(tube(b("toe-bones-l", "toe-l"), [T(0.14, 0.03, 0), T(0.196, 0.026, 0)], [[r(0.036), r(0.012)], [r(0.026), r(0.01)]]), "toe-l");
  }

  /* skull */
  const cranium = HEAD.filter((row) => row[0] >= -0.012);
  m(loft(b("cranium", "head"), headSt(cranium, -0.006)), "head");
  if (d.scale <= 200) {
    m(ellipsoid(b("maxilla", "head"), [r(0.022), r(0.024), r(0.031)], [-0.03, -0.068, 0]), "head");
    m(tube(b("mandible", "head"), [[-0.004, -0.012, 0.052], [-0.048, -0.006, 0.048], [-0.068, -0.05, 0.034], [-0.077, -0.084, 0], [-0.068, -0.05, -0.034], [-0.048, -0.006, -0.048], [-0.004, -0.012, -0.052]], [[r(0.0055), r(0.012)], [r(0.006), r(0.013)], [r(0.006), r(0.011)], [r(0.008), r(0.013)], [r(0.006), r(0.011)], [r(0.006), r(0.013)], [r(0.0055), r(0.012)]], [1, 0, 0]), "head");
  }
  if (fine) {
    const zyg = capsule(b("zygomatic-arch-l", "head"), [0.002, -0.078, 0.05], [0.006, -0.004, 0.064], 0.0058, 0.0048);
    m(zyg, "head");
    m(mirrored(zyg), "head");
    const orbit = ellipsoid({ id: "orbit-l", bone: "head", material: "bone", tone: 0.3, role: "detail" }, [0.016, 0.006, 0.018], [0.02, -0.089, 0.032]);
    m(orbit, "head");
    m(mirrored(orbit), "head");
  }

  /* the load path ends on the ground: heel and sole of each shoe carry the foot bones, the ferrule of the cane stands on the ground itself */
  const pad = (id: string, bone: string, at: readonly number[], top: number, rr: number): void => {
    const f = K.frame(bone);
    const up: V3 = [f.u[1], f.v[1], f.w[1]];
    m(revolve({ id, bone, material: "leather-shoe", tone: 0.3 }, [[0, 0], [0, r(rr)], [top, r(rr)], [top, 0]], L(bone, [at[0]!, 0, at[2]!]), up), bone);
  };
  for (const side of ["l", "r"] as const) {
    const z = side === "l" ? SHOE_Z : -SHOE_Z;
    pad(`heel-pad-${side}`, `foot-${side}`, [ANKLE_X - 0.036, 0, z], Math.max(0.03, d.floor), 0.02);
    pad(`sole-pad-${side}`, `foot-${side}`, [ANKLE_X + 0.146, 0, z], Math.max(0.02, d.floor), 0.024);
  }

  /* joints and masses */
  if (d.scale <= 200) {
    joint("torso", [0, 0, 0], 0.017);
    joint("chest", [0, 0, 0], 0.015);
    joint("neck", [0, 0, 0], 0.012);
    joint("head", [0, 0, 0], 0.011);
  }
  for (const side of ["l", "r"]) {
    joint(`thigh-${side}`, [0, 0, 0], 0.0255);
    joint(`shank-${side}`, [0, 0, 0], 0.02);
    joint(`arm-${side}`, [0, 0, 0], 0.0255);
    joint(`forearm-${side}`, [0, 0, 0], 0.016);
    if (d.scale <= 200) {
      joint(`foot-${side}`, [0, 0.002, 0], 0.016);
      joint(`hand-${side}`, [0, 0, 0], 0.0125);
      joint(`clavicle-${side}`, [0, 0, 0], 0.011);
    }
    if (mid) joint(`toe-${side}`, [-0.004, 0.02, 0], 0.01);
  }
  const masses = mid ? ["pelvis", "torso", "chest", "head", "thigh-l", "thigh-r", "shank-l", "shank-r", "arm-l", "arm-r", "forearm-l", "forearm-r", "foot-l", "foot-r"] : d.scale === 200 ? ["pelvis", "torso", "chest", "head", "thigh-l", "thigh-r"] : ["pelvis", "chest"];
  for (const id of masses) mass(id);
  return out;
}
