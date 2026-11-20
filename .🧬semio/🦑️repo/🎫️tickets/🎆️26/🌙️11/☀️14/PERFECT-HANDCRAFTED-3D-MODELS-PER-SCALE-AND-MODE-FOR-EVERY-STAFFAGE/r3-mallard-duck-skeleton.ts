/** 🦴️ Round 3 mallard, structure: the skeleton of Anas platyrhynchos as the load path, stated bone by bone by hand (trunk frame and bone frames of the default pose), joints at the rig joints. 1:50: long bones as shaped lofts at their real thickness of 7 to 9 mm, ribs, furcula, scapulae, pubes, mandible rami and toe bones as slender bones. 1:100, 1:200 and 1:500: long bones as slender bars along the bone axes (a wire skeleton), skull, sternum and synsacrum as masses. Round 4: the thoracic column and the synsacrum lie dorsally, 12 to 18 mm under the plumage of the back and 15 mm above the hip joints, joined to them by the ilia; the column dips to the neck base at the thoracic inlet.
 * @see ./r3-mallard-duck-kit.ts */
import { Bench, mul, sideBone, sideName, sub, type Look, type PP, type Side, type Station, type V3 } from "./r3-mallard-duck-kit.ts";
import { TOES, footAt } from "./r3-mallard-duck-body.ts";

const BONE: Look = { material: "bone", tone: 0.88 };
const MUSCLE: Look = { material: "plumage", tone: 0.5 };
type Level = 50 | 100 | 200 | 500;
let SPAN = 41;
type R = readonly [f: number, r: number, flat?: number];
const at = (a: V3, b: V3, f: number): V3 => [a[0] + (b[0] - a[0]) * f, a[1] + (b[1] - a[1]) * f, a[2] + (b[2] - a[2]) * f];

function rod(S: Bench, id: string, bone: string, a: V3, b: V3, st: readonly R[], look: Look = BONE): void {
  S.path(id, bone, st.map(([f, r, flat]): PP => ({ p: at(a, b, f), rl: flat ?? r, rs: r })), look);
}

/** 🪡️ A slender bone or feather shaft through the given points: never thicker than a twentieth of its chord (C2), at most its real radius `r`; `tall` stretches the section in the sagittal plane (a blade). */
function wire(S: Bench, id: string, bone: string, pts: readonly V3[], r: number, look: Look = BONE, tall = 1, lat?: V3): void {
  const chord = Math.hypot(...sub(pts[pts.length - 1]!, pts[0]!));
  const rr = Math.min(r, chord / SPAN / tall);
  S.path(id, bone, pts.map((p, i): PP => ({ p, rl: rr * (i === pts.length - 1 ? 0.7 : 1), rs: rr * tall * (i === pts.length - 1 ? 0.7 : 1) })), look, lat);
  (S.body[S.body.length - 1] as { slender?: boolean }).slender = true;
}

/** 💀️ Cranium with the upper bill bone as one loft in the head frame, the mandible as its two rami. */
function skull(S: Bench, level: Level): void {
  const f = S.frame("head");
  const st: readonly (readonly [number, number, number, number])[] = level === 50
    ? [[-0.005, 0.005, 0.008, 0.009], [0.004, 0.009, 0.014, 0.0138], [0.017, 0.0106, 0.0156, 0.0148], [0.03, 0.0092, 0.0136, 0.0136], [0.042, 0.0062, 0.01, 0.0108], [0.052, 0.0048, 0.0066, 0.0092], [0.066, 0.004, 0.0046, 0.0098], [0.084, 0.0022, 0.0036, 0.0106], [0.098, 0.0008, 0.0026, 0.0098], [0.1045, -0.0012, 0.0012, 0.0042]]
    : level === 100
      ? [[-0.005, 0.005, 0.008, 0.009], [0.012, 0.0102, 0.0152, 0.0146], [0.034, 0.0082, 0.0126, 0.0128], [0.054, 0.0024, 0.0078, 0.0092], [0.09, -0.0008, 0.0058, 0.0104], [0.1045, -0.0018, 0.0012, 0.0042]]
      : level === 200
        ? [[-0.006, 0.005, 0.009, 0.01], [0.014, 0.01, 0.0156, 0.0152], [0.034, 0.0075, 0.0125, 0.013], [0.052, 0.0028, 0.0082, 0.0102], [0.086, -0.0006, 0.005, 0.0108], [0.1045, -0.0018, 0.0012, 0.0042]]
        : [[-0.006, 0.005, 0.009, 0.01], [0.012, 0.0098, 0.0152, 0.0148], [0.036, 0.007, 0.012, 0.0125], [0.056, 0.002, 0.0072, 0.0098], [0.1045, -0.0018, 0.0014, 0.0046]];
  S.loft("skull", "head", st.map(([a, b, rs, rl]): Station => ({ c: S.J("head", a, b), a: [0, 0, rl], b: mul(f.v as V3, rs), e: 2.2 })), BONE);
  if (level === 50) S.loft("mandible", "head", ([[0.002, -0.0046, 0.0026, 0.0118], [0.02, -0.0064, 0.0032, 0.0108], [0.045, -0.0058, 0.003, 0.0094], [0.07, -0.0046, 0.0026, 0.0092], [0.09, -0.0042, 0.0022, 0.0096], [0.1025, -0.0044, 0.001, 0.005]] as const).map(([a, b, rs, rl]): Station => ({ c: S.J("head", a, b), a: [0, 0, rl], b: mul(f.v as V3, rs), e: 2.6 })), BONE);
  else if (level === 100) for (const sd of [-1, 1] as const) wire(S, `${sideName(sd)}-mandible`, "head", [S.J("head", 0.002, -0.004, sd * 0.011), S.J("head", 0.045, -0.0056, sd * 0.009), S.J("head", 0.1, -0.004, sd * 0.002)], 0.0012, BONE, 1.4);
}

function column(S: Bench, level: Level): void {
  for (const [bone, len] of [["neck-1", 0.06], ["neck-2", 0.055], ["neck-3", 0.05]] as const) {
    const [a, b] = [S.J(bone, -0.002), S.J(bone, len + 0.002)];
    if (level === 50) rod(S, `cervical-${bone.slice(-1)}`, bone, a, b, [[0, 0.0038], [0.12, 0.0045], [0.3, 0.0039], [0.5, 0.0045], [0.7, 0.0039], [0.88, 0.0045], [1, 0.0038]]);
    else wire(S, `cervical-${bone.slice(-1)}`, bone, [a, b], 0.004);
  }
  if (level === 50) S.path("spine", "trunk", ([[0.107, -0.0205, 0.0042], [0.099, -0.006, 0.0045], [0.086, 0.008, 0.0045], [0.064, 0.0155, 0.0045], [0.04, 0.0165, 0.0042]] as const).map(([s, t, r]): PP => ({ p: S.T(s, t), rl: r, rs: r })), BONE);
  else wire(S, "spine", "trunk", level === 500 ? [S.T(0.107, -0.0205), S.T(0.09, 0.006), S.T(0.04, 0.0165)] : [S.T(0.107, -0.0205), S.T(0.094, 0.002), S.T(0.07, 0.0145), S.T(0.04, 0.0165)], 0.004);
  const shield: readonly (readonly [number, number, number, number])[] = level === 50
    ? [[0.046, 0.0165, 0.006, 0.0045], [0.03, 0.0165, 0.014, 0.0052], [0.008, 0.015, 0.0215, 0.0058], [-0.015, 0.012, 0.0225, 0.0056], [-0.04, 0.008, 0.02, 0.0048], [-0.06, 0.005, 0.014, 0.0042], [-0.071, 0.004, 0.006, 0.0036]]
    : level === 100
      ? [[0.046, 0.0165, 0.006, 0.005], [0.01, 0.014, 0.0215, 0.0076], [-0.04, 0.007, 0.02, 0.0062], [-0.071, 0.004, 0.006, 0.004]]
      : level === 200
        ? [[0.046, 0.014, 0.007, 0.006], [0.0, 0.008, 0.0215, 0.0152], [-0.071, 0.004, 0.006, 0.005]]
        : [[0.05, 0.014, 0.006, 0.006], [0.004, 0.009, 0.02, 0.013], [-0.068, 0.004, 0.006, 0.005]];
  S.loft("synsacrum", "trunk", shield.map(([s, t, half, th]): Station => ({ c: S.T(s, t), a: [0, 0, half], b: mul(S.dorsal, th), e: 2.4 })), BONE);
  if (level === 50) S.path("pygostyle", "tail", [{ p: S.T(-0.066, 0.005), rl: 0.0038, rs: 0.0038 }, { p: S.T(-0.078, 0.004), rl: 0.004, rs: 0.004 }, { p: S.J("tail", 0.0), rl: 0.0042, rs: 0.0042 }, { p: S.J("tail", 0.012), rl: 0.0036, rs: 0.0052 }, { p: S.J("tail", 0.022), rl: 0.002, rs: 0.0036 }], BONE);
  else wire(S, "pygostyle", "tail", [S.T(-0.066, 0.005), S.J("tail", 0.0), S.J("tail", 0.022)], 0.003);
}

/** 🫁️ Sternum with its keel, ribs ([s, half width, depth below the spine, caudal sweep]), coracoids, furcula, scapulae and pubes. */
function thorax(S: Bench, level: Level): void {
  if (level === 50) {
    S.loft("sternum", "trunk", ([[0.119, -0.05, 0.008], [0.106, -0.053, 0.02], [0.08, -0.057, 0.025], [0.05, -0.06, 0.026], [0.02, -0.06, 0.024], [-0.002, -0.058, 0.017]] as const).map(([s, t, half]): Station => ({ c: S.T(s, t), a: [0, 0, half], b: mul(S.dorsal, 0.0038), e: 2.6 })), BONE);
    S.loft("keel", "trunk", ([[0.119, -0.051, -0.062], [0.109, -0.054, -0.077], [0.09, -0.057, -0.0815], [0.06, -0.061, -0.0805], [0.03, -0.061, -0.074], [0.004, -0.059, -0.065]] as const).map(([s, top, bottom]): Station => ({ c: S.T(s, (top + bottom) / 2), a: [0, 0, 0.0038], b: mul(S.dorsal, (top - bottom) / 2 + 0.001), e: 2.6 })), BONE);
  } else {
    const boat: readonly (readonly [number, number, number, number])[] = level === 100
      ? [[0.119, -0.055, 0.008, 0.007], [0.1, -0.066, 0.021, 0.0135], [0.06, -0.069, 0.026, 0.012], [0.02, -0.066, 0.024, 0.0085], [-0.002, -0.06, 0.016, 0.004]]
      : level === 200
        ? [[0.119, -0.055, 0.009, 0.008], [0.08, -0.068, 0.025, 0.0152], [-0.002, -0.06, 0.016, 0.005]]
        : [[0.119, -0.055, 0.009, 0.008], [0.085, -0.067, 0.024, 0.014], [0.03, -0.066, 0.022, 0.009], [-0.002, -0.06, 0.014, 0.005]];
    S.loft("sternum", "trunk", boat.map(([s, t, half, depth]): Station => ({ c: S.T(s, t), a: [0, 0, half], b: mul(S.dorsal, depth), e: 1.6 })), BONE);
  }
  const ribs: readonly (readonly [number, number, number, number])[] = level === 50
    ? [[0.1, 0.03, 0.056, 0.0], [0.09, 0.034, 0.064, 0.004], [0.078, 0.036, 0.069, 0.008], [0.066, 0.0365, 0.072, 0.014], [0.054, 0.0355, 0.073, 0.019], [0.043, 0.033, 0.073, 0.025]]
    : level === 100
      ? [[0.098, 0.031, 0.058, 0.002], [0.074, 0.036, 0.07, 0.011], [0.048, 0.035, 0.073, 0.022]]
      : [];
  for (const sd of [-1, 1] as const) {
    const n = sideName(sd);
    ribs.forEach(([s, half, depth, sweep], i) => {
      wire(S, `${n}-rib-${i + 1}`, "trunk", [0.06, 0.34, 0.6, 0.84, 1].map((k): V3 => {
        const ang = k * 2.75;
        return S.T(s - sweep * k, 0.0155 - (depth / 2) * (1 - Math.cos(ang)), sd * Math.max(half * Math.sin(ang), k === 1 ? 0.02 : 0.003));
      }), 0.0013, BONE, 1, S.axis);
    });
    const shoulder = S.J(`wing-${sideBone(sd)}`);
    const foot = S.T(0.114, -0.05, sd * 0.011);
    if (level === 50) rod(S, `${n}-coracoid`, "trunk", shoulder, foot, [[-0.04, 0.0038], [0.2, 0.0038], [0.7, 0.0038, 0.0046], [1, 0.0036, 0.0062]]);
    else wire(S, `${n}-coracoid`, "trunk", [shoulder, foot], 0.003);
    if (level < 200) {
      wire(S, `${n}-furcula`, "trunk", [[shoulder[0] + 0.003, shoulder[1] + 0.001, sd * 0.027], S.T(0.126, -0.03, sd * 0.021), S.T(0.133, -0.047, sd * 0.011), S.T(0.128, -0.06, sd * 0.0014)], 0.0013);
      wire(S, `${n}-scapula`, "trunk", [[shoulder[0], shoulder[1] + 0.003, sd * 0.03], S.T(0.08, 0.008, sd * 0.031), S.T(0.05, 0.008, sd * 0.03), S.T(0.022, 0.004, sd * 0.027)], 0.001, BONE, 2.2);
    }
    if (level === 50) S.path(`${n}-ilium`, "trunk", [{ p: S.T(0.034, 0.012, sd * 0.013), rl: 0.0038, rs: 0.005 }, { p: S.T(0.012, 0.006, sd * 0.019), rl: 0.0038, rs: 0.008 }, { p: S.T(-0.002, 0.0, sd * 0.0205), rl: 0.0038, rs: 0.0095 }, { p: S.T(-0.03, -0.004, sd * 0.021), rl: 0.0038, rs: 0.009 }, { p: S.T(-0.058, -0.008, sd * 0.017), rl: 0.0038, rs: 0.006 }], BONE);
    else if (level === 100) wire(S, `${n}-ilium`, "trunk", [S.T(0.034, 0.012, sd * 0.013), S.T(-0.002, 0.0, sd * 0.0205), S.T(-0.058, -0.008, sd * 0.017)], 0.002, BONE, 2);
    if (level === 50) wire(S, `${n}-pubis`, "trunk", [S.T(-0.004, -0.012, sd * 0.021), S.T(-0.045, -0.022, sd * 0.0225), S.T(-0.086, -0.028, sd * 0.016)], 0.001);
  }
}

/** 🪽️ The folded arm skeleton on the wing bone: humerus back to the elbow, ulna and radius forward to the wrist, hand back along the lower wing edge. */
function wing(S: Bench, sd: Side, level: Level): void {
  const [n, b] = [sideName(sd), sideBone(sd)];
  const shoulder = S.J(`wing-${b}`);
  const elbow = S.T(0.015, -0.003, sd * 0.043);
  const wrist = S.T(0.088, -0.033, sd * 0.051);
  const finger = S.T(0.004, -0.02, sd * 0.05);
  if (level === 50) {
    rod(S, `${n}-humerus`, `wing-${b}`, shoulder, elbow, [[-0.03, 0.0038], [0.05, 0.0052], [0.25, 0.004], [0.75, 0.0039], [0.95, 0.0048], [1.02, 0.0036]]);
    rod(S, `${n}-forearm-bones`, `wing-${b}`, elbow, wrist, [[-0.02, 0.0036], [0.06, 0.0044, 0.0044], [0.5, 0.0046, 0.0038], [0.94, 0.0044, 0.004], [1.02, 0.0036]]);
    rod(S, `${n}-manus`, `wing-${b}`, wrist, finger, [[-0.02, 0.0036], [0.08, 0.0044, 0.0038], [0.6, 0.004, 0.0038], [0.72, 0.0038], [1, 0.0038, 0.0038]]);
  } else if (level === 500) wire(S, `${n}-wing-bones`, `wing-${b}`, [shoulder, elbow, wrist, finger], 0.003);
  else {
    wire(S, `${n}-humerus`, `wing-${b}`, [shoulder, elbow], 0.0035);
    wire(S, `${n}-forearm-bones`, `wing-${b}`, [elbow, wrist], 0.003);
    wire(S, `${n}-manus`, `wing-${b}`, [wrist, finger], 0.003);
  }
}

function leg(S: Bench, sd: Side, level: Level): void {
  const [n, b] = [sideName(sd), sideBone(sd)];
  const [hip, knee, ankle, mtp] = [S.J(`thigh-${b}`), S.J(`shank-${b}`), S.J(`tarsus-${b}`), S.J(`foot-${b}`)];
  if (level === 50) {
    rod(S, `${n}-femur`, `thigh-${b}`, hip, knee, [[-0.05, 0.0038], [0.06, 0.005], [0.3, 0.0038], [0.8, 0.0038], [0.96, 0.005], [1.04, 0.0038]]);
    rod(S, `${n}-tibiotarsus`, `shank-${b}`, knee, ankle, [[-0.03, 0.0038], [0.05, 0.0048], [0.3, 0.0038], [0.85, 0.0038], [0.97, 0.0046], [1.02, 0.0038]]);
    rod(S, `${n}-tarsometatarsus`, `tarsus-${b}`, ankle, mtp, [[-0.04, 0.0036], [0.06, 0.0042], [0.3, 0.0034, 0.003], [0.8, 0.0034, 0.003], [0.95, 0.004, 0.0044], [1.0, 0.0034]]);
  } else {
    wire(S, `${n}-femur`, `thigh-${b}`, [hip, knee], 0.003);
    wire(S, `${n}-tibiotarsus`, `shank-${b}`, [knee, ankle], 0.003);
    wire(S, `${n}-tarsometatarsus`, `tarsus-${b}`, [ankle, mtp], 0.003);
  }
  const tip = (yawDeg: number, len: number, q: number): V3 => footAt(S, sd, Math.cos((yawDeg * Math.PI) / 180) * len * q, -0.0026 - 0.0018 * q, Math.sin((yawDeg * Math.PI) / 180) * len * q);
  for (const [name, [yaw, len]] of Object.entries(TOES)) {
    if (level >= 200 && name !== "middle") continue;
    if (level === 50) S.path(`${n}-digit-${name}`, `foot-${b}`, [{ p: tip(yaw, len, 0), rl: 0.002, rs: 0.002 }, { p: tip(yaw, len, 0.45), rl: 0.0018, rs: 0.0017 }, { p: tip(yaw, len, 0.8), rl: 0.0012, rs: 0.001 }, { p: tip(yaw, len, 0.95), rl: 0.0005, rs: 0.0005 }], BONE);
    else wire(S, `${n}-digit-${name}`, `foot-${b}`, [tip(yaw, len, 0), tip(yaw, len, 0.5), tip(yaw, len, 0.97)], 0.0016);
  }
}

export function skeleton(S: Bench, level: Level): void {
  SPAN = level === 500 ? 46 : 41;
  const from = S.body.length;
  skull(S, level);
  column(S, level);
  thorax(S, level);
  for (const sd of [-1, 1] as const) {
    wing(S, sd, level);
    leg(S, sd, level);
  }
  const trunkParts = /spine|synsacrum|sternum|keel|rib|coracoid|furcula|scapula|pubis|ilium/;
  S.toStructure(from, (s) => (trunkParts.test(s.id) ? "trunk" : s.bone));
  const j0 = S.body.length;
  const joints: readonly (readonly [string, number])[] = level === 50
    ? [["neck-1", 0.0054], ["neck-2", 0.0052], ["neck-3", 0.0052], ["head", 0.0052], ["tail", 0.0048], ["wing-n", 0.006], ["wing-f", 0.006], ["thigh-n", 0.0058], ["thigh-f", 0.0058], ["shank-n", 0.0056], ["shank-f", 0.0056], ["tarsus-n", 0.0052], ["tarsus-f", 0.0052], ["foot-n", 0.0046], ["foot-f", 0.0046]]
    : level === 100
      ? [["shank-n", 0.005], ["shank-f", 0.005], ["tarsus-n", 0.0048], ["tarsus-f", 0.0048], ["foot-n", 0.0042], ["foot-f", 0.0042]]
      : [];
  for (const [bone, r] of joints) S.ell(`joint-${bone}`, bone, S.J(bone), [r, r, r], BONE);
  S.toStructure(j0, (s) => s.bone, "joint");
  const m0 = S.body.length;
  const f = S.frame("trunk");
  S.ell("mass-trunk", "trunk", S.T(0.035, -0.035), level === 500 ? [0.032, 0.016, 0.023] : level === 200 ? [0.03, 0.0152, 0.022] : [0.03, 0.017, 0.024], MUSCLE, [f.u as V3, f.v as V3]);
  if (level === 50) S.ell("mass-head", "head", S.J("head", 0.04, 0.005), [0.007, 0.0045, 0.006], MUSCLE);
  S.toStructure(m0, (s) => s.bone, "mass");
}
