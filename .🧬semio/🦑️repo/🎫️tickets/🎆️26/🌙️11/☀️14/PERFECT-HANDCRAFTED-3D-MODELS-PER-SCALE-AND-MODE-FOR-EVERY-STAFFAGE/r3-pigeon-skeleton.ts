/** 🦴️ Round 3 pigeon, structure: the skeleton of Columba livia as the load path, stated bone by bone in world metres (default pose), joints at the rig joints. 1:50: every long bone as a shaped loft at the thickness the scale can show (7.5 mm, the real size of humerus head, vertebrae, forearm pair and carpometacarpus), ribs, furcula, scapulae, pubes, mandible rami and digits as true-size slender bones. 1:100 and 1:200: long bones as slender bars along the bone axes (a wire skeleton; a bar of 15 or 30 mm would be thicker than the neck), skull, sternum, synsacrum and pygostyle as masses.
 * @see ./r3-pigeon-kit.ts */
import { Bench, mul, sideBone, sideName, sub, type Look, type PP, type Side, type Station, type V3 } from "./r3-pigeon-kit.ts";
import { H } from "./r3-pigeon-body50.ts";

const BONE: Look = { material: "bone", tone: 0.88 };
const KERATIN: Look = { material: "plumage", tone: 0.7 };
const MUSCLE: Look = { material: "bare-skin", tone: 0.5 };
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

function skull(S: Bench, level: Level): void {
  const f = S.frame("head");
  const st: readonly (readonly [number, number, number, number])[] = level === 50
    ? [[-0.004, 0.003, 0.006, 0.007], [0.004, 0.005, 0.0105, 0.0105], [0.012, 0.0055, 0.0115, 0.011], [0.02, 0.0042, 0.0098, 0.009], [0.0265, 0.0026, 0.0062, 0.006], [0.031, 0.0012, 0.0036, 0.0036], [0.037, 0.0006, 0.0028, 0.003], [0.046, -0.0004, 0.002, 0.0022], [0.0535, -0.0018, 0.0006, 0.0006]]
    : level === 100
      ? [[-0.004, 0.003, 0.006, 0.007], [0.008, 0.0052, 0.0112, 0.0108], [0.02, 0.004, 0.0095, 0.009], [0.032, 0.0014, 0.004, 0.0042], [0.0535, -0.0018, 0.0006, 0.0006]]
      : level === 200
        ? [[-0.007, 0.003, 0.008, 0.009], [0.009, 0.0042, 0.0146, 0.0146], [0.03, 0.0016, 0.005, 0.005], [0.0535, -0.0018, 0.0006, 0.0006]]
        : [[-0.007, 0.003, 0.008, 0.009], [0.007, 0.005, 0.012, 0.0118], [0.021, 0.0038, 0.0092, 0.0088], [0.032, 0.001, 0.0038, 0.0038], [0.0535, -0.0018, 0.0006, 0.0006]];
  S.loft("skull", "head", st.map(([a, b, rs, rl]): Station => ({ c: H(S, a, b), a: [0, 0, rl], b: mul(f.v as V3, rs) })), BONE);
  if (level === 50) S.loft("mandible", "jaw", ([[0.008, -0.0084, 0.0018, 0.0078], [0.02, -0.0066, 0.0018, 0.0056], [0.032, -0.0047, 0.0018, 0.0032], [0.044, -0.004, 0.0014, 0.0021], [0.0525, -0.0035, 0.0004, 0.0005]] as const).map(([a, b, rs, rl]): Station => ({ c: H(S, a, b), a: [0, 0, rl], b: mul(f.v as V3, rs), e: 2.2 })), BONE);
  else wire(S, "mandible", "jaw", [H(S, 0.008, -0.0082), H(S, 0.03, -0.0052), H(S, 0.0525, -0.0035)], 0.001);
}

function column(S: Bench, level: Level): void {
  for (const [bone, len] of [["neck-1", 0.03], ["neck-2", 0.025], ["neck-3", 0.02]] as const) {
    const [a, b] = [S.J(bone, -0.002), S.J(bone, len + 0.002)];
    if (level === 50) rod(S, `cervical-${bone.slice(-1)}`, bone, a, b, [[0, 0.0036], [0.2, 0.0041], [0.5, 0.0037], [0.8, 0.0041], [1, 0.0036]]);
    else wire(S, `cervical-${bone.slice(-1)}`, bone, [a, b], 0.003);
  }
  if (level === 50) S.path("spine", "trunk", ([[0.156, 0.0, 0.0038], [0.135, -0.0005, 0.0041], [0.115, -0.001, 0.0041], [0.098, -0.001, 0.0038]] as const).map(([s, t, r]): PP => ({ p: S.T(s, t), rl: r, rs: r })), BONE);
  else wire(S, "spine", "trunk", [S.T(0.156, 0), S.T(0.128, -0.0008), S.T(0.098, -0.001)], 0.0035);
  const shield: readonly (readonly [number, number, number, number])[] = level === 50
    ? [[0.102, -0.002, 0.005, 0.004], [0.09, -0.003, 0.012, 0.0042], [0.073, -0.004, 0.019, 0.0048], [0.05, -0.004, 0.0175, 0.0042], [0.028, -0.003, 0.013, 0.0034], [0.01, -0.002, 0.007, 0.0028], [0.002, -0.001, 0.003, 0.002]]
    : level === 100
      ? [[0.102, -0.003, 0.005, 0.004], [0.073, -0.0065, 0.019, 0.0076], [0.035, -0.005, 0.015, 0.0055], [0.002, -0.001, 0.003, 0.002]]
      : level === 200
        ? [[0.102, -0.004, 0.006, 0.006], [0.07, -0.0135, 0.019, 0.0152], [0.002, -0.002, 0.004, 0.003]]
        : [[0.1, -0.004, 0.006, 0.006], [0.066, -0.012, 0.0185, 0.0138], [0.03, -0.006, 0.013, 0.007], [0.002, -0.002, 0.004, 0.003]];
  S.loft("synsacrum", "pelvis", shield.map(([s, t, half, th]): Station => ({ c: S.T(s, t), a: [0, 0, half], b: mul(S.dorsal, th), e: 2.4 })), BONE);
  const py = level === 50 ? 0.0038 : 0.0053;
  S.path("pygostyle", "tail", level === 500
    ? [{ p: S.J("tail", -0.006), rl: py * 0.9, rs: py * 0.9 }, { p: S.J("tail", 0.01), rl: py, rs: py }, { p: S.J("tail", 0.025), rl: py * 0.5, rs: py }]
    : level === 200
    ? [{ p: S.J("tail", -0.006), rl: py, rs: py }, { p: S.J("tail", 0.024), rl: py * 0.6, rs: py * 1.1 }]
    : [{ p: S.J("tail", -0.006), rl: py * 0.8, rs: py * 0.8 }, { p: S.J("tail", 0.008), rl: py, rs: py }, { p: S.J("tail", 0.017), rl: py * 0.8, rs: py * 1.25 }, { p: S.J("tail", 0.026), rl: py * 0.4, rs: py * 0.8 }], BONE);
  const shaft = (id: string, bone: string, z: number): void => wire(S, id, bone, [S.J("tail", 0.02, 0, z * 0.3), S.J("tail", 0.055, 0, z * 0.7), S.J("tail", 0.086, 0, z)], 0.0008, KERATIN);
  shaft("rectrix-shaft", "tail", 0);
  shaft("rectrix-shaft-near", "tail-f", -0.011);
  if (level < 200) shaft("rectrix-shaft-far", "tail-f", 0.011);
}

/** 🫁️ Sternum with its keel, ribs ([s, half width, depth below the spine, caudal sweep]), coracoids, furcula, scapulae and pubes. */
function thorax(S: Bench, level: Level): void {
  if (level === 50) {
    S.loft("sternum", "trunk", ([[0.128, -0.0462, 0.006], [0.122, -0.0462, 0.0125], [0.105, -0.0485, 0.0135], [0.085, -0.0495, 0.012], [0.068, -0.0485, 0.0095], [0.055, -0.046, 0.006]] as const).map(([s, t, half]): Station => ({ c: S.T(s, t), a: [0, 0, half], b: mul(S.dorsal, 0.0038), e: 2.6 })), BONE);
    S.loft("keel", "trunk", ([[0.128, -0.047, -0.056], [0.123, -0.047, -0.064], [0.112, -0.048, -0.07], [0.095, -0.05, -0.0685], [0.078, -0.05, -0.062], [0.064, -0.048, -0.055]] as const).map(([s, top, bottom]): Station => ({ c: S.T(s, (top + bottom) / 2), a: [0, 0, 0.0038], b: mul(S.dorsal, (top - bottom) / 2 + 0.001), e: 2.6 })), BONE);
  } else {
    const boat: readonly (readonly [number, number, number, number])[] = level === 100
      ? [[0.129, -0.05, 0.006, 0.005], [0.118, -0.057, 0.013, 0.0125], [0.095, -0.058, 0.013, 0.011], [0.07, -0.054, 0.0095, 0.0065], [0.055, -0.049, 0.005, 0.003]]
      : level === 200
        ? [[0.129, -0.05, 0.007, 0.006], [0.112, -0.056, 0.0152, 0.0152], [0.055, -0.049, 0.005, 0.003]]
        : [[0.129, -0.05, 0.007, 0.006], [0.114, -0.056, 0.0145, 0.014], [0.085, -0.056, 0.012, 0.009], [0.055, -0.049, 0.005, 0.003]];
    S.loft("sternum", "trunk", boat.map(([s, t, half, depth]): Station => ({ c: S.T(s, t), a: [0, 0, half], b: mul(S.dorsal, depth), e: 1.5 })), BONE);
  }
  const ribs: readonly (readonly [number, number, number, number])[] = level === 50
    ? [[0.132, 0.017, 0.04, 0.004], [0.122, 0.0205, 0.044, 0.007], [0.112, 0.0225, 0.046, 0.01], [0.102, 0.0225, 0.046, 0.013], [0.093, 0.0205, 0.044, 0.016]]
    : level === 100
      ? [[0.13, 0.018, 0.041, 0.005], [0.113, 0.0225, 0.046, 0.01], [0.096, 0.021, 0.045, 0.015]]
      : [];
  for (const sd of [-1, 1] as const) {
    const n = sideName(sd);
    ribs.forEach(([s, half, depth, sweep], i) => {
      wire(S, `${n}-rib-${i + 1}`, "trunk", [0.06, 0.34, 0.6, 0.84, 1].map((k): V3 => {
        const ang = k * 2.75;
        return S.T(s - sweep * k, -0.002 - (depth / 2) * (1 - Math.cos(ang)), sd * Math.max(half * Math.sin(ang), k === 1 ? 0.0095 : 0.002));
      }), 0.001, BONE, 1, S.axis);
    });
    const shoulder = S.J(`humerus-${sideBone(sd)}`);
    const foot = S.T(0.125, -0.0455, sd * 0.009);
    if (level === 50) rod(S, `${n}-coracoid`, "trunk", shoulder, foot, [[-0.04, 0.0038], [0.2, 0.0038], [0.7, 0.0038, 0.0042], [1, 0.0034, 0.0055]]);
    else wire(S, `${n}-coracoid`, "trunk", [shoulder, foot], 0.002);
    if (level < 200) {
      wire(S, `${n}-furcula`, "trunk", [[shoulder[0] + 0.002, shoulder[1] + 0.001, sd * 0.019], S.T(0.139, -0.03, sd * 0.016), S.T(0.143, -0.046, sd * 0.009), S.T(0.138, -0.056, sd * 0.0012)], 0.0009);
      wire(S, `${n}-scapula`, "trunk", [[shoulder[0] + 0.001, shoulder[1] + 0.002, sd * 0.021], S.T(0.1, -0.005, sd * 0.0215), S.T(0.08, -0.005, sd * 0.0205), S.T(0.064, -0.007, sd * 0.019)], 0.0007, BONE, 1.6);
    }
    if (level === 50) wire(S, `${n}-pubis`, "pelvis", [S.T(0.07, -0.013, sd * 0.019), S.T(0.045, -0.0165, sd * 0.019), S.T(0.02, -0.017, sd * 0.015)], 0.0009);
  }
}

function wing(S: Bench, sd: Side, level: Level): void {
  const [n, b] = [sideName(sd), sideBone(sd)];
  const on = (p: V3, min: number): V3 => [p[0], p[1], sd * Math.max(min, S.flank(p[0], p[1]) + 0.002)];
  const shoulder = S.J(`humerus-${b}`);
  const elbow = on(S.J(`forearm-${b}`), 0.03);
  const wrist = on(S.J(`hand-${b}`), 0.034);
  const d = S.frame(`hand-${b}`).u as V3;
  const along = (q: number, z: number): V3 => [wrist[0] + d[0] * q, wrist[1] + d[1] * q, z > 0 ? sd * z : sd * (S.flank(wrist[0] + d[0] * q, wrist[1] + d[1] * q) + 0.0025)];
  if (level === 50) {
    rod(S, `${n}-humerus`, `humerus-${b}`, shoulder, elbow, [[-0.03, 0.0036], [0.07, 0.0046], [0.3, 0.0038], [0.75, 0.0038], [0.95, 0.0043], [1.03, 0.0034]]);
    rod(S, `${n}-forearm-bones`, `forearm-${b}`, elbow, wrist, [[-0.03, 0.0034], [0.06, 0.004], [0.5, 0.0038], [0.94, 0.004], [1.02, 0.0034]]);
    rod(S, `${n}-manus`, `hand-${b}`, wrist, along(0.058, 0), [[-0.03, 0.0034], [0.08, 0.004], [0.55, 0.0038], [0.8, 0.003], [1, 0.0016]]);
  } else {
    wire(S, `${n}-humerus`, `humerus-${b}`, [shoulder, elbow], 0.0025);
    wire(S, `${n}-forearm-bones`, `forearm-${b}`, [elbow, wrist], 0.002);
    wire(S, `${n}-manus`, `hand-${b}`, [wrist, along(0.058, 0)], 0.002);
  }
  if (level !== 500) wire(S, `${n}-primary-shaft`, `hand-${b}`, [along(0.05, 0), along(0.1, 0.028), along(0.15, 0.018), along(0.197, 0.01)], 0.0009, KERATIN);
}

function leg(S: Bench, sd: Side, level: Level): void {
  const [n, b] = [sideName(sd), sideBone(sd)];
  const [hip, knee0, ankle0, mtp] = [S.J(`thigh-${b}`), S.J(`shank-${b}`), S.J(`tarsus-${b}`), S.J(`toes-${b}`)];
  const knee: V3 = [knee0[0], knee0[1], knee0[2] + sd * 0.005];
  const ankle: V3 = [ankle0[0], ankle0[1], ankle0[2] + sd * 0.0005];
  if (level === 50) rod(S, `${n}-femur`, `thigh-${b}`, hip, knee, [[-0.05, 0.0036], [0.06, 0.0044], [0.3, 0.0038], [0.8, 0.0038], [0.96, 0.0044], [1.04, 0.0034]]);
  else wire(S, `${n}-femur`, `thigh-${b}`, [hip, knee], 0.002);
  if (level >= 200) {
    wire(S, `${n}-tibiotarsus`, `shank-${b}`, [knee, ankle], 0.002);
    wire(S, `${n}-tarsometatarsus`, `tarsus-${b}`, [ankle, mtp], 0.002);
  } else {
    rod(S, `${n}-tibiotarsus`, `shank-${b}`, knee, ankle, level === 50 ? [[-0.03, 0.0028], [0.05, 0.0036], [0.3, 0.0027], [0.85, 0.0024], [0.97, 0.003], [1.02, 0.0024]] : [[-0.02, 0.0028], [0.5, 0.0026], [1.02, 0.0028]]);
    rod(S, `${n}-tarsometatarsus`, `tarsus-${b}`, ankle, mtp, level === 50 ? [[-0.04, 0.0022], [0.06, 0.0027], [0.3, 0.002], [0.8, 0.002], [0.95, 0.0027], [1.0, 0.0022]] : [[-0.03, 0.0025], [0.5, 0.0022], [1.0, 0.0025]]);
  }
  const tip = (yawDeg: number, len: number): V3 => [mtp[0] + Math.cos((yawDeg * Math.PI) / 180) * len, 0.0014, mtp[2] - sd * Math.sin((yawDeg * Math.PI) / 180) * len];
  const pad: V3 = [mtp[0] + 0.001, 0.0026, mtp[2]];
  if (level === 50) {
    for (const [k, yawDeg, len] of [[3, 0, 0.031], [2, -38, 0.0225], [4, 36, 0.0245], [1, 192, 0.013]] as const) {
      const end = tip(yawDeg, len);
      const dir: V3 = [Math.cos((yawDeg * Math.PI) / 180), 0, -sd * Math.sin((yawDeg * Math.PI) / 180)];
      S.path(`${n}-digit-${k}`, `toes-${b}`, [{ p: [mtp[0], 0.0024, mtp[2]], rl: 0.0018, rs: 0.0018 }, { p: [mtp[0] + dir[0] * len * 0.35, 0.0021, mtp[2] + dir[2] * len * 0.35], rl: 0.0015, rs: 0.0015 }, { p: [mtp[0] + dir[0] * len * 0.8, 0.0017, mtp[2] + dir[2] * len * 0.8], rl: 0.0011, rs: 0.0011 }, { p: end, rl: 0.0005, rs: 0.0005 }], BONE, [-dir[2], 0, dir[0]]);
    }
    return;
  }
  wire(S, `${n}-digits-1-3`, `toes-${b}`, [tip(192, 0.014), [mtp[0] - 0.003, 0.0026, mtp[2] + sd * 0.0006], pad, tip(0, 0.016), tip(0, 0.032)], 0.001);
  if (level < 200) wire(S, `${n}-digits-2-4`, `toes-${b}`, [tip(-38, 0.024), tip(-38, 0.01), pad, tip(36, 0.011), tip(36, 0.026)], 0.0009, BONE, 1, [1, 0, 0]);
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
  const trunkParts = /spine|sternum|keel|rib|coracoid|furcula|scapula/;
  S.toStructure(from, (s) => (trunkParts.test(s.id) ? "trunk" : s.bone));
  const j0 = S.body.length;
  const joints: readonly (readonly [string, number])[] = level === 50
    ? [["neck-1", 0.0045], ["neck-2", 0.0045], ["neck-3", 0.0045], ["head", 0.0045], ["jaw", 0.0023], ["tail", 0.0044], ["thigh-n", 0.005], ["thigh-f", 0.005], ["shank-n", 0.0038], ["shank-f", 0.0038], ["tarsus-n", 0.0032], ["tarsus-f", 0.0032], ["toes-n", 0.003], ["toes-f", 0.003], ["humerus-n", 0.0052], ["humerus-f", 0.0052], ["forearm-n", 0.0046], ["forearm-f", 0.0046], ["hand-n", 0.0044], ["hand-f", 0.0044]]
    : level === 100
      ? [["shank-n", 0.0034], ["shank-f", 0.0034], ["tarsus-n", 0.003], ["tarsus-f", 0.003], ["toes-n", 0.003], ["toes-f", 0.003]]
      : [];
  for (const [bone, r] of joints) {
    const sd: Side = bone.endsWith("-n") ? -1 : 1;
    const p = S.J(bone);
    const z = /^shank/.test(bone) ? p[2] + sd * 0.005 : /^forearm/.test(bone) ? sd * Math.max(0.03, S.flank(p[0], p[1]) + 0.002) : /^hand/.test(bone) ? sd * Math.max(0.034, S.flank(p[0], p[1]) + 0.002) : p[2];
    S.ell(`joint-${bone}`, bone, [p[0], p[1], z], [r, r, r], BONE);
  }
  S.toStructure(j0, (s) => s.bone, "joint");
  const m0 = S.body.length;
  const f = S.frame("trunk");
  S.ell("mass-trunk", "trunk", S.J("trunk", 0.03198, -0.022), level === 500 ? [0.022, 0.0155, 0.0155] : level === 200 ? [0.021, 0.0152, 0.0152] : [0.022, 0.014, 0.016], MUSCLE, [f.u as V3, f.v as V3]);
  if (level === 50) S.ell("mass-head", "head", S.J("head", 0.012, 0.005), [0.007, 0.006, 0.0065], MUSCLE);
  S.toStructure(m0, (s) => s.bone, "mass");
}
