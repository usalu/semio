/** 🦴️ Round 3 herring gull, structure: the skeleton of Larus argentatus as the load path, stated bone by bone by hand (trunk frame and bone frames of the default pose), joints at the rig joints, masses at the centres of mass. 1:50: skull with the bill bones and mandible, cervical column, thoracic spine, synsacrum and pygostyle, sternum with keel, ribs, coracoids, furcula, scapulae, the folded wing bones (humerus, ulna and radius, carpometacarpus) with the shafts of the outer primaries that carry the hand wing, femur, tibiotarsus, tarsometatarsus and toe bones at their real thickness where the scale's feature allows, else as slender bones. 1:100: the same chain with merged parts (skull one loft, sternum a hull, three ribs, one primary shaft). 1:200: a wire skeleton of slender bars along the bone axes with skull, sternum and synsacrum as masses.
 * @see ./r3-herring-gull-kit.ts */
import { Bench, mul, sideBone, sideName, sub, type Look, type PP, type Side, type Station, type V3 } from "./r3-herring-gull-kit.ts";
import { TOES, footAt } from "./r3-herring-gull-body.ts";

const BONE: Look = { material: "bone", tone: 0.88 };
const SHAFT: Look = { material: "wing-feathers", tone: 0.7 };
const MUSCLE: Look = { material: "plumage", tone: 0.5 };
type Level = 50 | 100 | 200;
type R = readonly [f: number, r: number, flat?: number];
const at = (a: V3, b: V3, f: number): V3 => [a[0] + (b[0] - a[0]) * f, a[1] + (b[1] - a[1]) * f, a[2] + (b[2] - a[2]) * f];

function rod(S: Bench, id: string, bone: string, a: V3, b: V3, st: readonly R[], look: Look = BONE): void {
  S.path(id, bone, st.map(([f, r, flat]): PP => ({ p: at(a, b, f), rl: flat ?? r, rs: r })), look);
}

/** 🪡️ A slender bone or feather shaft through the given points: never thicker than a twenty-first of its chord (C2), at most its real radius `r`; `tall` stretches the section in the sagittal plane (a blade). */
export function wire(S: Bench, id: string, bone: string, pts: readonly V3[], r: number, look: Look = BONE, tall = 1, lat?: V3): void {
  const chord = Math.hypot(...sub(pts[pts.length - 1]!, pts[0]!));
  const rr = Math.min(r, chord / 42 / tall);
  S.path(id, bone, pts.map((p, i): PP => ({ p, rl: rr * (i === pts.length - 1 ? 0.7 : 1), rs: rr * tall * (i === pts.length - 1 ? 0.7 : 1) })), look, lat);
  S.body[S.body.length - 1]!.slender = true;
}

/** 💀️ Cranium with the upper bill bone as one loft in the head frame (occiput 24 mm behind the condyle, braincase 30 mm deep and 30 mm wide, premaxilla to the hook), the mandible as its two rami. */
function skull(S: Bench, level: Level): void {
  const f = S.frame("head");
  const st: readonly (readonly [number, number, number, number])[] = level === 50
    ? [[-0.018, 0.009, 0.008, 0.009], [-0.008, 0.011, 0.0135, 0.0135], [0.008, 0.0115, 0.0155, 0.0148], [0.024, 0.0098, 0.0135, 0.0135], [0.04, 0.0058, 0.0098, 0.011], [0.054, 0.0026, 0.0068, 0.0072], [0.072, 0.0004, 0.0062, 0.0042], [0.095, -0.0024, 0.0058, 0.0036], [0.112, -0.0052, 0.0044, 0.003], [0.1235, -0.0094, 0.0016, 0.0012]]
    : level === 100
      ? [[-0.018, 0.009, 0.008, 0.009], [0.006, 0.0112, 0.0155, 0.0148], [0.034, 0.0072, 0.0115, 0.0122], [0.06, 0.0014, 0.0075, 0.0076], [0.1, -0.0034, 0.0075, 0.0076], [0.1235, -0.0094, 0.0016, 0.0016]]
      : [[-0.018, 0.009, 0.009, 0.01], [0.01, 0.0105, 0.0155, 0.0152], [0.06, 0.0005, 0.0152, 0.0152], [0.1235, -0.0094, 0.0016, 0.0016]];
  S.loft("skull", "head", st.map(([a, b, rs, rl]): Station => ({ c: S.J("head", a, b), a: [0, 0, rl], b: mul(f.v as V3, rs), e: 2.2 })), BONE);
  if (level === 50) S.loft("mandible", "head", ([[0.004, -0.0062, 0.0026, 0.0112], [0.025, -0.0074, 0.0032, 0.0098], [0.05, -0.0086, 0.0034, 0.0072], [0.075, -0.0108, 0.0036, 0.0046], [0.1, -0.0132, 0.0036, 0.0036], [0.118, -0.0124, 0.0012, 0.0016]] as const).map(([a, b, rs, rl]): Station => ({ c: S.J("head", a, b), a: [0, 0, rl], b: mul(f.v as V3, rs), e: 2.6 })), BONE);
  else if (level === 100) for (const sd of [-1, 1] as const) wire(S, `${sideName(sd)}-mandible`, "head", [S.J("head", 0.004, -0.0062, sd * 0.011), S.J("head", 0.05, -0.0086, sd * 0.007), S.J("head", 0.116, -0.012, sd * 0.0015)], 0.0014, BONE, 1.4);
}

/** 🦴️ Cervical column in its S (two links), thoracic spine to the synsacrum, synsacrum as a shield over the acetabula, pygostyle with the rectrix shafts. */
function column(S: Bench, level: Level): void {
  for (const [bone, len] of [["neck-1", 0.07], ["neck-2", 0.065]] as const) {
    const [a, b] = [S.J(bone, -0.003), S.J(bone, len + 0.002)];
    if (level === 50) rod(S, `cervical-${bone.slice(-1)}`, bone, a, b, [[0, 0.004], [0.12, 0.0048], [0.3, 0.0041], [0.5, 0.0048], [0.7, 0.0041], [0.88, 0.0048], [1, 0.004]]);
    else wire(S, `cervical-${bone.slice(-1)}`, bone, [a, b], 0.0045);
  }
  if (level === 50) S.path("spine", "trunk", ([[0.103, 0.014, 0.0045], [0.09, 0.026, 0.0048], [0.07, 0.031, 0.0048], [0.05, 0.031, 0.0046], [0.036, 0.029, 0.0044]] as const).map(([s, t, r]): PP => ({ p: S.T(s, t), rl: r, rs: r })), BONE);
  else wire(S, "spine", "trunk", [S.T(0.103, 0.014), S.T(0.07, 0.031), S.T(0.036, 0.029)], 0.0045);
  const shield: readonly (readonly [number, number, number, number])[] = level === 50
    ? [[0.046, 0.027, 0.007, 0.0045], [0.03, 0.025, 0.015, 0.0052], [0.008, 0.021, 0.022, 0.0058], [-0.015, 0.018, 0.023, 0.0056], [-0.04, 0.017, 0.02, 0.0048], [-0.06, 0.017, 0.014, 0.0042], [-0.07, 0.016, 0.007, 0.0036]]
    : level === 100
      ? [[0.046, 0.027, 0.007, 0.005], [0.008, 0.02, 0.022, 0.0076], [-0.04, 0.017, 0.02, 0.0062], [-0.07, 0.016, 0.007, 0.004]]
      : [[0.046, 0.026, 0.0076, 0.0076], [0.0, 0.016, 0.022, 0.0152], [-0.07, 0.016, 0.0076, 0.0076]];
  S.loft("synsacrum", "trunk", shield.map(([s, t, half, th]): Station => ({ c: S.T(s, t), a: [0, 0, half], b: mul(S.dorsal, th), e: 2.4 })), BONE);
  if (level === 50) {
    S.path("pygostyle", "tail", [{ p: S.T(-0.064, 0.016), rl: 0.0038, rs: 0.0038 }, { p: S.J("tail", 0.0), rl: 0.0042, rs: 0.0042 }, { p: S.J("tail", 0.014), rl: 0.0036, rs: 0.0054 }, { p: S.J("tail", 0.026), rl: 0.002, rs: 0.0036 }], BONE);
    for (const [i, w] of [0.003, 0.01, 0.017].entries()) for (const sd of [-1, 1] as const) wire(S, `${sideName(sd)}-rectrix-${i + 1}`, "tail", [S.J("tail", 0.012, -0.001, sd * w * 0.5), S.J("tail", 0.09, -0.0012, sd * w * 0.9), S.J("tail", 0.165, -0.0006, sd * w)], 0.0012, SHAFT);
  } else wire(S, "pygostyle", "tail", [S.T(-0.064, 0.016), S.J("tail", 0.0), S.J("tail", 0.16)], 0.003, level === 100 ? BONE : SHAFT);
}

/** 🫁️ Sternum with its keel (73 mm, IMNH R-976), ribs ([s, half width, depth below the spine, caudal sweep]), coracoids (50 mm), furcula, scapulae and pubes. */
function thorax(S: Bench, level: Level): void {
  if (level === 50) {
    S.loft("sternum", "trunk", ([[0.128, -0.042, 0.008], [0.118, -0.047, 0.019], [0.095, -0.053, 0.024], [0.07, -0.056, 0.025], [0.05, -0.057, 0.022], [0.04, -0.056, 0.016]] as const).map(([s, t, half]): Station => ({ c: S.T(s, t), a: [0, 0, half], b: mul(S.dorsal, 0.004), e: 2.6 })), BONE);
    S.loft("keel", "trunk", ([[0.128, -0.044, -0.058], [0.118, -0.049, -0.07], [0.1, -0.054, -0.0745], [0.075, -0.057, -0.0735], [0.055, -0.058, -0.068], [0.042, -0.057, -0.062]] as const).map(([s, top, bottom]): Station => ({ c: S.T(s, (top + bottom) / 2), a: [0, 0, 0.0038], b: mul(S.dorsal, (top - bottom) / 2 + 0.001), e: 2.6 })), BONE);
  } else {
    const boat: readonly (readonly [number, number, number, number])[] = level === 100
      ? [[0.128, -0.048, 0.008, 0.0076], [0.11, -0.058, 0.021, 0.014], [0.075, -0.062, 0.025, 0.012], [0.04, -0.058, 0.016, 0.0076]]
      : [[0.128, -0.05, 0.0152, 0.0152], [0.085, -0.06, 0.025, 0.0152], [0.04, -0.056, 0.0152, 0.0152]];
    S.loft("sternum", "trunk", boat.map(([s, t, half, depth]): Station => ({ c: S.T(s, t), a: [0, 0, half], b: mul(S.dorsal, depth), e: 1.6 })), BONE);
  }
  const ribs: readonly (readonly [number, number, number, number])[] = level === 50
    ? [[0.1, 0.03, 0.062, 0.006], [0.087, 0.036, 0.068, 0.01], [0.074, 0.039, 0.072, 0.014], [0.061, 0.04, 0.074, 0.018], [0.048, 0.039, 0.074, 0.022], [0.036, 0.036, 0.072, 0.026]]
    : level === 100
      ? [[0.095, 0.032, 0.064, 0.007], [0.07, 0.039, 0.072, 0.015], [0.045, 0.038, 0.074, 0.023]]
      : [];
  for (const sd of [-1, 1] as const) {
    const n = sideName(sd);
    ribs.forEach(([s, half, depth, sweep], i) => {
      wire(S, `${n}-rib-${i + 1}`, "trunk", [0.06, 0.34, 0.6, 0.84, 1].map((k): V3 => {
        const ang = k * 2.75;
        return S.T(s - sweep * k, 0.029 - ((depth + 0.004) / 2) * (1 - Math.cos(ang)), sd * Math.max(half * Math.sin(ang), k === 1 ? 0.018 : 0.003));
      }), 0.0014, BONE, 1, S.axis);
    });
    const glenoid = S.T(0.095, 0.016, sd * 0.026);
    const foot = S.T(0.126, -0.03, sd * 0.012);
    if (level === 50) rod(S, `${n}-coracoid`, "trunk", glenoid, foot, [[-0.04, 0.0038], [0.2, 0.0038], [0.7, 0.0038, 0.0048], [1, 0.0036, 0.0064]]);
    else wire(S, `${n}-coracoid`, "trunk", [glenoid, foot], 0.003);
    if (level !== 200) {
      wire(S, `${n}-furcula`, "trunk", [S.T(0.1, 0.018, sd * 0.024), S.T(0.122, -0.006, sd * 0.02), S.T(0.134, -0.026, sd * 0.011), S.T(0.13, -0.041, sd * 0.0014)], 0.0014);
      wire(S, `${n}-scapula`, "trunk", [S.T(0.094, 0.022, sd * 0.024), S.T(0.07, 0.033, sd * 0.024), S.T(0.04, 0.033, sd * 0.022), S.T(0.012, 0.03, sd * 0.02)], 0.0012, BONE, 2.2);
    }
    if (level === 50) wire(S, `${n}-pubis`, "trunk", [S.T(-0.004, -0.004, sd * 0.021), S.T(-0.045, -0.012, sd * 0.022), S.T(-0.082, -0.006, sd * 0.016)], 0.0012);
  }
}

/** 🪽️ The wing bones on their own rig bones: humerus from the glenoid back to the elbow, ulna and radius forward to the wrist, carpometacarpus (72 mm, IMNH R-976) and digits back along the hand, and the shafts of the outer primaries that carry the hand wing to its tip. */
function wing(S: Bench, sd: Side, level: Level): void {
  const [n, b] = [sideName(sd), sideBone(sd)];
  const [shoulder, elbow, wrist] = [S.J(`humerus-${b}`), S.J(`forearm-${b}`), S.J(`hand-${b}`)];
  const digits = S.J(`hand-${b}`, 0.1);
  const vane = (s: number, t: number, z: number): V3 => S.T(s, t, sd * z);
  const shaft = (k: number): V3[] => [vane(-0.03, -0.006 + k, 0.042), vane(-0.1, 0.016 + k, 0.027), vane(-0.17, 0.034 + k, 0.017), vane(-0.25, 0.044 + k * 0.6, 0.0095), vane(-0.3 + Math.abs(k) * 2, 0.049, 0.0055)];
  if (level === 50) {
    rod(S, `${n}-humerus`, `humerus-${b}`, shoulder, elbow, [[-0.03, 0.004], [0.05, 0.0058], [0.25, 0.0042], [0.75, 0.004], [0.95, 0.0052], [1.02, 0.0038]]);
    rod(S, `${n}-ulna-radius`, `forearm-${b}`, elbow, wrist, [[-0.02, 0.0038], [0.06, 0.0046, 0.0044], [0.5, 0.0048, 0.0038], [0.94, 0.0046, 0.004], [1.02, 0.0038]]);
    rod(S, `${n}-manus`, `hand-${b}`, wrist, digits, [[-0.02, 0.0038], [0.08, 0.0046, 0.0038], [0.6, 0.0042, 0.0038], [0.75, 0.0038], [1, 0.0024, 0.0024]]);
    for (const [i, k] of [-0.004, 0.0, 0.004].entries()) wire(S, `${n}-primary-shaft-${i + 1}`, `hand-${b}`, [digits, ...shaft(k)], 0.0016, SHAFT);
  } else {
    wire(S, `${n}-humerus`, `humerus-${b}`, [shoulder, elbow], 0.004);
    wire(S, `${n}-ulna-radius`, `forearm-${b}`, [elbow, wrist], 0.0035);
    wire(S, `${n}-hand-bones`, `hand-${b}`, [wrist, digits, ...shaft(0)], 0.0035, level === 100 ? BONE : SHAFT);
  }
}

function leg(S: Bench, sd: Side, level: Level): void {
  const [n, b] = [sideName(sd), sideBone(sd)];
  const [hip, knee, ankle, mtp] = [S.J(`thigh-${b}`), S.J(`shank-${b}`), S.J(`tarsus-${b}`), S.J(`foot-${b}`)];
  if (level === 50) {
    rod(S, `${n}-femur`, `thigh-${b}`, hip, knee, [[-0.05, 0.004], [0.06, 0.0054], [0.3, 0.004], [0.8, 0.004], [0.96, 0.0054], [1.04, 0.004]]);
    rod(S, `${n}-tibiotarsus`, `shank-${b}`, knee, ankle, [[-0.03, 0.004], [0.05, 0.0052], [0.3, 0.0038], [0.85, 0.0038], [0.97, 0.005], [1.02, 0.004]]);
    rod(S, `${n}-tarsometatarsus`, `tarsus-${b}`, ankle, mtp, [[-0.04, 0.0038], [0.06, 0.0048], [0.3, 0.0036, 0.003], [0.8, 0.0036, 0.003], [0.95, 0.0046, 0.0048], [1.0, 0.0036]]);
  } else {
    wire(S, `${n}-femur`, `thigh-${b}`, [hip, knee], 0.0035);
    wire(S, `${n}-tibiotarsus`, `shank-${b}`, [knee, ankle], 0.0035);
    wire(S, `${n}-tarsometatarsus`, `tarsus-${b}`, [ankle, mtp], 0.0035);
  }
  const tip = (yawDeg: number, len: number, q: number): V3 => footAt(S, sd, Math.cos((yawDeg * Math.PI) / 180) * len * q, -0.0022 - 0.0016 * q, Math.sin((yawDeg * Math.PI) / 180) * len * q);
  for (const [name, [yaw, len]] of Object.entries(TOES)) {
    if (level !== 50 && name !== "middle") continue;
    if (level === 50) S.path(`${n}-digit-${name}`, `foot-${b}`, [{ p: tip(yaw, len, 0), rl: 0.002, rs: 0.002 }, { p: tip(yaw, len, 0.45), rl: 0.0018, rs: 0.0016 }, { p: tip(yaw, len, 0.8), rl: 0.0012, rs: 0.001 }, { p: tip(yaw, len, 0.95), rl: 0.0005, rs: 0.0005 }], BONE);
    else wire(S, `${n}-digit-${name}`, `foot-${b}`, [tip(yaw, len, 0), tip(yaw, len, 0.5), tip(yaw, len, 0.97)], 0.0016);
  }
}

export function skeleton(S: Bench, level: Level): void {
  const from = S.body.length;
  skull(S, level);
  column(S, level);
  thorax(S, level);
  for (const sd of [-1, 1] as const) {
    wing(S, sd, level);
    leg(S, sd, level);
  }
  const trunkParts = /spine|synsacrum|sternum|keel|rib-|coracoid|furcula|scapula|pubis/;
  S.toStructure(from, (s) => (trunkParts.test(s.id) ? "trunk" : s.bone));
  const j0 = S.body.length;
  const big = level === 50 ? 1 : level === 100 ? 1.25 : 0;
  const joints: readonly (readonly [string, number])[] = big === 0 ? [] : [["neck-1", 0.0055], ["neck-2", 0.0052], ["head", 0.0052], ["tail", 0.0048], ["humerus-n", 0.006], ["humerus-f", 0.006], ["forearm-n", 0.0052], ["forearm-f", 0.0052], ["hand-n", 0.005], ["hand-f", 0.005], ["thigh-n", 0.006], ["thigh-f", 0.006], ["shank-n", 0.0058], ["shank-f", 0.0058], ["tarsus-n", 0.0054], ["tarsus-f", 0.0054], ["foot-n", 0.0048], ["foot-f", 0.0048]];
  for (const [bone, r] of joints) {
    const rr = Math.max(r * big, level === 100 ? 0.0076 : 0);
    S.ell(`joint-${bone}`, bone, S.J(bone), [rr, rr, rr], BONE);
  }
  S.toStructure(j0, (s) => s.bone, "joint");
  const m0 = S.body.length;
  const f = S.frame("trunk");
  S.ell("mass-trunk", "trunk", S.T(0.035, -0.03), level === 200 ? [0.032, 0.0152, 0.022] : [0.03, 0.017, 0.024], MUSCLE, [f.u as V3, f.v as V3]);
  if (level !== 200) S.ell("mass-head", "head", S.J("head", 0.035, 0.008), level === 50 ? [0.0075, 0.006, 0.0075] : [0.0078, 0.0078, 0.0078], MUSCLE);
  S.toStructure(m0, (s) => s.bone, "mass");
}
