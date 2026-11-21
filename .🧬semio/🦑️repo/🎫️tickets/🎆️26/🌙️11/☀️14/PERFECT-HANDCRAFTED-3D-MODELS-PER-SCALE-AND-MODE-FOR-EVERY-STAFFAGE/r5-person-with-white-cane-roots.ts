/** 🦯 Root fixes of `humans/person-with-white-cane` outside `volumes` and `representations`: the rig (joint heights of a woman of 1.68 m in ankle boots, hip and shoulder spacing, carrying angle, neck and head joint, the cane as a rigid body in the fist), the anchors, and every pose and clip key solved anew (upright stance instead of a crouch, a gait from heel strike to toe-off with the cane tip rolling on the ground and sweeping from side to side, the hand held at the midline).
 *
 * Usage: `bun r5-person-with-white-cane-roots.ts [--table]` reads the untouched copy `🗑️generated/r5-person-with-white-cane/original.json` and writes `roots.json` beside it (members `summary`, `rig`, `anchors`, `poses`, `clips`; spliced by `r5-person-with-white-cane-apply.py`). Every value is absolute.
 * @see ./r3-humans-person-with-white-cane.md */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { rigAt } from "./r3-lib.ts";
import { type Json } from "./volumes-lib.ts";

const OUT = join(import.meta.dir, "🗑️generated", "r5-person-with-white-cane");
const asset = JSON.parse(readFileSync(join(OUT, "original.json"), "utf8")).asset as Json;
for (const rep of Object.values(asset.representations) as Json[]) if (rep.geometry) for (const view of Object.keys(rep.geometry)) rep.geometry[view] = [];
const rad = (d: number): number => (d * Math.PI) / 180;
const deg = (r: number): number => (r * 180) / Math.PI;
const r4 = (x: number): number => Math.round(x * 1e4) / 1e4;
const r2 = (x: number): number => Math.round(x * 100) / 100;
const bone = (id: string): Json => asset.rig.bones.find((b: Json) => b.id === id);
const anchor = (id: string): Json => asset.anchors.find((a: Json) => a.id === id);
const pose = (id: string): Json => asset.poses.find((p: Json) => p.id === id);
const smooth = (t: number): number => { const c = Math.min(1, Math.max(0, t)); return c * c * (3 - 2 * c); };

/** 📏 The woman: 1.68 m barefoot, ankle boots with a 25 mm heel (the ankle joint stands 21 mm higher), 1.70 m to the scalp. */
export const BODY = { ankle: 0.0865, hip: 0.9114, thigh: 0.4116, shank: 0.4133, pelvis: 0.084, torso: 0.4706, shoulderX: 0.3984, shoulderZ: 0.165, neck: 0.085, head: 0.15, hipZ: 0.085, elbowOut: 0, wristOut: 0, ballX: 0.1378, toe: 0.0605, heelX: -0.058 } as const;
/** 🦯 The cane: 1.3 m over all, the roller a rounded body of 27 mm whose end dome is centred 13.5 mm before the end. It lies in the fist along the index finger, 10 degrees to the axis of the hand; its bone starts at the centre of the fist (`top` = 78 mm of grip above it, ending under the heel of the hand), so a change of grip turns the cane inside the closed hand. */
export const CANE = { overall: 1.3, top: 0.078, length: 1.222, roller: 0.0135, x: 0.05, y: 0, z: 0.024, angleDeg: 10, parkedDeg: -112 } as const;

/* ---------------------------------------------------------------- rig */
bone("pelvis").y = BODY.hip;
const torso = bone("torso");
torso.mass.comX = r4((torso.mass.comX / torso.lengthM) * BODY.torso);
torso.lengthM = BODY.torso;
bone("neck").x = BODY.torso;
bone("neck").lengthM = BODY.neck;
bone("head").x = BODY.neck;
bone("head").lengthM = BODY.head;
bone("head").mass.comX = 0.014;
const footDeg = deg(Math.atan2(BODY.ballX, BODY.ankle));
const footLen = Math.hypot(BODY.ballX, BODY.ankle);
const FU: [number, number] = [Math.sin(rad(footDeg)), -Math.cos(rad(footDeg))];
const FV: [number, number] = [-FU[1], FU[0]];
/** 👣 A point given from the ankle of the flat foot (x ahead, y up) in the frame of the foot bone. */
const inFoot = (x: number, y: number): [number, number] => [r4(x * FU[0] + y * FU[1]), r4(x * FV[0] + y * FV[1])];
const withZ = (b: Json, z: number): void => {
  const copy: Json = {};
  for (const [k, v] of Object.entries(b)) { if (k === "z") continue; copy[k] = v; if (k === "y" && z !== 0) copy.z = z; }
  for (const k of Object.keys(b)) delete b[k];
  Object.assign(b, copy);
};
for (const side of ["l", "r"]) {
  const s = side === "l" ? 1 : -1;
  withZ(bone(`arm-${side}`), s * BODY.shoulderZ);
  withZ(bone(`forearm-${side}`), s * BODY.elbowOut);
  withZ(bone(`hand-${side}`), s * BODY.wristOut);
  withZ(bone(`thigh-${side}`), s * BODY.hipZ);
  const foot = bone(`foot-${side}`);
  foot.angleDeg = r4(footDeg);
  foot.lengthM = r4(footLen);
  [foot.mass.comX, foot.mass.comY] = inFoot(0.045, -0.045);
  bone(`hand-${side}`).mass.comX = 0.0622;
  bone(`thigh-${side}`).mass.comX = 0.1487;
  const toe = bone(`toe-${side}`);
  toe.x = r4(footLen);
  toe.angleDeg = r4(90 - footDeg);
}
const cane = bone("cane");
Object.assign(cane, { x: CANE.x, y: CANE.y, angleDeg: CANE.angleDeg, lengthM: CANE.length });
withZ(cane, CANE.z);
cane.joint = { type: "revolute", minDeg: -140, maxDeg: 30 };
cane.mass.comX = 0.48;

/* ---------------------------------------------------------------- anchors */
for (const side of ["l", "r"]) {
  const [hx, hy] = inFoot(BODY.heelX, -BODY.ankle);
  Object.assign(anchor(`heel-${side}`), { x: hx, y: hy, footprintLengthM: 0.05, footprintWidthM: 0.055 });
  Object.assign(anchor(`ball-${side}`), { x: r4(footLen), y: 0, footprintLengthM: 0.09, footprintWidthM: 0.08 });
}
Object.assign(anchor("nose"), { x: 0.049, y: -0.108 });
Object.assign(anchor("mouth"), { x: 0.022, y: -0.096 });
Object.assign(anchor("crown"), { x: 0.156, y: -0.012 });
Object.assign(anchor("grip-r"), { x: CANE.x, y: CANE.y, z: CANE.z });
Object.assign(anchor("bag-strap"), { x: 0.452, y: 0.012, z: -0.098 });

/* ---------------------------------------------------------------- solving through the product's own rig */
const TMP = "r3-solve";
type Bones = Record<string, Json>;
function framesOf(bones: Bones, contacts: readonly string[]): Record<string, { o: number[]; u: number[]; v: number[]; w: number[] }> {
  const base = asset.poses[0];
  asset.poses.push({ ...base, id: TMP, bones, root: { x: 0, y: 0 }, ground: "auto", contacts });
  try {
    return rigAt(asset, 50, { kind: "pose", pose: TMP }).frames as never;
  } finally {
    asset.poses.pop();
  }
}
/** 🧮 Damped Gauss-Newton on named bone offsets; residuals in metres. */
function solve(bones: Bones, contacts: readonly string[], params: readonly [string, "angleDeg" | "abductionDeg" | "twistDeg"][], residual: (f: ReturnType<typeof framesOf>) => number[], label: string): number {
  const n = params.length;
  let err = Infinity;
  for (let it = 0; it < 60; it++) {
    const r0 = residual(framesOf(bones, contacts));
    err = Math.hypot(...r0);
    if (err < 2e-6) break;
    const J: number[][] = [];
    for (const [bn, key] of params) {
      bones[bn][key] = (bones[bn][key] ?? 0) + 0.02;
      const r1 = residual(framesOf(bones, contacts));
      bones[bn][key] -= 0.02;
      J.push(r1.map((x, i) => (x - r0[i]!) / 0.02));
    }
    const A = Array.from({ length: n }, (_, i) => Array.from({ length: n + 1 }, (_, j) => (j === n ? -J[i]!.reduce((s, x, k) => s + x * r0[k]!, 0) : J[i]!.reduce((s, x, k) => s + x * J[j]![k]!, 0) + (i === j ? 1e-9 : 0))));
    for (let c = 0; c < n; c++) {
      let piv = c;
      for (let k = c + 1; k < n; k++) if (Math.abs(A[k]![c]!) > Math.abs(A[piv]![c]!)) piv = k;
      [A[c], A[piv]] = [A[piv]!, A[c]!];
      for (let k = 0; k < n; k++) if (k !== c) { const m = A[k]![c]! / A[c]![c]!; for (let j = c; j <= n; j++) A[k]![j]! -= m * A[c]![j]!; }
    }
    const step = params.map((_, i) => Math.max(-20, Math.min(20, A[i]![n]! / A[i]![i]!)));
    let k = 1;
    for (let tries = 0; tries < 8; tries++) {
      params.forEach(([bn, key], i) => { bones[bn][key] += k * step[i]!; });
      if (Math.hypot(...residual(framesOf(bones, contacts))) < err) break;
      params.forEach(([bn, key], i) => { bones[bn][key] -= k * step[i]!; });
      k /= 2;
    }
  }
  if (err > 1e-4) console.log(`[DEBUG] ${label}: residual ${(err * 1000).toFixed(2)} mm`, residual(framesOf(bones, contacts)).map((x) => (x * 1000).toFixed(1)).join(" "), params.map(([bn, key]) => bones[bn][key].toFixed(1)).join(" "));
  for (const [bn, key] of params) bones[bn][key] = r2(bones[bn][key]);
  return err;
}

/* ---------------------------------------------------------------- legs */
const SOLE: [number, number][] = [[BODY.heelX, -BODY.ankle], [-0.066, -BODY.ankle + 0.004], [BODY.ballX, -BODY.ankle]];
const rot = (p: readonly [number, number], d: number): [number, number] => [p[0] * Math.cos(rad(d)) - p[1] * Math.sin(rad(d)), p[0] * Math.sin(rad(d)) + p[1] * Math.cos(rad(d))];
/** 🦵 Two-link solution of one leg: the ankle `a` given from the hip joint (x ahead, y up), the sole pitched by `pitch` degrees (toes up positive); returns thigh, shank and foot offsets and the knee flexion. */
function leg(a: readonly [number, number], pitch: number): { thigh: number; shank: number; foot: number; knee: number; reach: number } {
  const [l1, l2] = [BODY.thigh, BODY.shank];
  const reach = Math.hypot(a[0], a[1]);
  const d = Math.min(reach, l1 + l2 - 1e-6);
  const knee = Math.acos(Math.max(-1, Math.min(1, (l1 * l1 + l2 * l2 - d * d) / (2 * l1 * l2))));
  const hip = Math.acos(Math.max(-1, Math.min(1, (l1 * l1 + d * d - l2 * l2) / (2 * l1 * d))));
  const thigh = deg(Math.atan2(a[0], -a[1]) + hip);
  const shank = -(180 - deg(knee));
  return { thigh: r4(thigh), shank: r4(shank), foot: r4(pitch - thigh - shank), knee: 180 - deg(knee), reach };
}
/** 🧍 Both legs of a stance: left foot `stagger` ahead of the hip, right foot as far behind, soles flat, knees eased by `kneeDeg`. */
function stance(bones: Bones, stagger: number, kneeDeg: number): number {
  const d = Math.sqrt(BODY.thigh ** 2 + BODY.shank ** 2 + 2 * BODY.thigh * BODY.shank * Math.cos(rad(kneeDeg)));
  const drop = Math.sqrt(d * d - stagger * stagger);
  for (const [side, x] of [["l", stagger], ["r", -stagger]] as const) {
    const s = leg([x, -drop], 0);
    bones[`thigh-${side}`] = { angleDeg: s.thigh };
    bones[`shank-${side}`] = { angleDeg: s.shank };
    bones[`foot-${side}`] = { angleDeg: s.foot };
    bones[`toe-${side}`] = { angleDeg: 0 };
  }
  return r4(drop + BODY.ankle - BODY.hip);
}

/* ---------------------------------------------------------------- the right arm and the cane */
type Hold = { wrist: [number, number, number]; elbowZ?: number; tipZ: number; tipAhead?: number; caneDeg?: number; palmUp?: number; seed?: Bones };
const ARM: [string, "angleDeg" | "abductionDeg" | "twistDeg"][] = [["arm-r", "angleDeg"], ["arm-r", "abductionDeg"], ["arm-r", "twistDeg"], ["forearm-r", "angleDeg"], ["forearm-r", "twistDeg"], ["hand-r", "angleDeg"], ["hand-r", "abductionDeg"]];
const SEED: Bones = { "arm-r": { angleDeg: 12, abductionDeg: -8, twistDeg: -30 }, "forearm-r": { angleDeg: 45, twistDeg: 20 }, "hand-r": { angleDeg: -20, abductionDeg: -25 } };
const tipOf = (f: ReturnType<typeof framesOf>): number[] => [0, 1, 2].map((k) => f.cane!.o[k]! + (CANE.length - CANE.roller) * f.cane!.u[k]!);
/** ✊ Solves the right arm (shoulder flexion, abduction and rotation, elbow flexion, forearm rotation, both wrist axes): the palm normal rises by `palmUp` (sine; negative: the palm looks a little down), the wrist at `wrist` given from the pelvis joint in the frame of the trunk (ahead, up along the trunk, left), the end dome of the roller on the ground at the lateral position `tipZ`, and either the elbow at the lateral position `elbowZ` or the roller `tipAhead` of the wrist. */
function hold(bones: Bones, contacts: readonly string[], h: Hold, label: string, seed: Bones = h.seed ?? SEED): void {
  for (const bn of ["arm-r", "forearm-r", "hand-r"]) bones[bn] = {};
  for (const [bn, key] of ARM) bones[bn][key] = seed[bn]![key];
  bones.cane = { angleDeg: h.caneDeg ?? 0 };
  solve(bones, contacts, ARM, (f) => {
    const p = f.pelvis!;
    const t = f.torso!;
    const target = [0, 1, 2].map((k) => p.o[k]! + h.wrist[0] * -t.v[k]! + h.wrist[1] * t.u[k]! + h.wrist[2] * t.w[k]!);
    const w = f["hand-r"]!.o;
    const tip = tipOf(f);
    return [w[0]! - target[0]!, w[1]! - target[1]!, w[2]! - target[2]!, tip[1]! - CANE.roller, tip[2]! - h.tipZ, h.tipAhead === undefined ? f["forearm-r"]!.o[2]! - h.elbowZ! : tip[0]! - w[0]! - h.tipAhead, (f["hand-r"]!.w[1]! - (h.palmUp ?? -0.2)) * 0.05];
  }, label);
}
/** 🤚 Only the wrist: the roller on the ground at the lateral position `tipZ`, arm and forearm as they are. */
function sweep(bones: Bones, contacts: readonly string[], tipZ: number, label: string): void {
  solve(bones, contacts, [["hand-r", "angleDeg"], ["hand-r", "abductionDeg"]], (f) => { const tip = tipOf(f); return [tip[1]! - CANE.roller, tip[2]! - tipZ]; }, label);
}
const report = (label: string, bones: Bones, contacts: readonly string[]): void => {
  const f = framesOf(bones, contacts);
  const tip = tipOf(f);
  const fa = f["forearm-r"]!;
  const hd = f["hand-r"]!;
  const ulnar = deg(Math.acos(Math.min(1, fa.u[0]! * hd.u[0]! + fa.u[1]! * hd.u[1]! + fa.u[2]! * hd.u[2]!)));
  console.log(label.padEnd(26), "hip", f.pelvis!.o[1]!.toFixed(3), "wrist", hd.o.map((x) => x.toFixed(3)).join(" "), "elbow", fa.o.map((x) => x.toFixed(3)).join(" "), "tip", tip.map((x) => x.toFixed(3)).join(" "), "wrist bend", ulnar.toFixed(0), "palm normal", hd.w.map((x) => x.toFixed(2)).join(" "), "arm", ARM.map(([b, k]) => bones[b][k]).join(" "), "top", [0, 1, 2].map((k) => (f.cane!.o[k]! - CANE.top * f.cane!.u[k]!).toFixed(3)).join(" "));
};

/* ---------------------------------------------------------------- poses */
const FEET = ["heel-l", "ball-l", "heel-r", "ball-r"];
const HOLDS: Record<string, Hold> = {
  "constant-contact-stance": { wrist: [0.27, 0.05, -0.08], elbowZ: -0.195, tipZ: 0 },
  "diagonal-technique": { wrist: [0.26, 0.03, -0.2], elbowZ: -0.215, tipZ: 0.22, seed: { "arm-r": { angleDeg: 25, abductionDeg: -8, twistDeg: 0 }, "forearm-r": { angleDeg: 20, twistDeg: 0 }, "hand-r": { angleDeg: -30, abductionDeg: 20 } } },
};
/** 🅿️ The parked cane: the upper arm hangs, the elbow is bent until the fist stands at chest height, the grip is changed to a fist round the upright shaft (the cane turned by `parkedDeg` in the hand) and elbow and wrist put the roller on the ground beside the right foot. */
function parked(bones: Bones): void {
  bones["arm-r"] = { angleDeg: -8, abductionDeg: -7, twistDeg: -18 };
  bones["forearm-r"] = { angleDeg: 106, twistDeg: 0 };
  bones["hand-r"] = { angleDeg: 0, abductionDeg: 0 };
  bones.cane = { angleDeg: CANE.parkedDeg };
  solve(bones, FEET, [["forearm-r", "angleDeg"], ["hand-r", "angleDeg"], ["hand-r", "abductionDeg"]], (f) => { const tip = tipOf(f); return [tip[1]! - CANE.roller, tip[2]! + 0.135, tip[0]! - f["hand-r"]!.o[0]! - 0.075]; }, "cane-parked");
}
for (const id of ["constant-contact-stance", "diagonal-technique", "cane-parked"]) {
  const h = HOLDS[id];
  const p = pose(id);
  const b = p.bones as Bones;
  p.root = { x: 0, y: stance(b, 0.05, id === "cane-parked" ? 3 : 5) };
  b["arm-l"] = { angleDeg: id === "diagonal-technique" ? -3 : -1, abductionDeg: 6 };
  b["forearm-l"] = { angleDeg: id === "diagonal-technique" ? 14 : 9, abductionDeg: 6 };
  b["hand-l"] = { angleDeg: 4 };
  if (h) hold(b, FEET, h, id);
  else parked(b);
  report(id, b, FEET);
}
pose("constant-contact-stance").description = { en: "Standing upright on both feet, the right hand at the midline a forearm's length ahead of the belt, the cane running straight ahead in line with the index finger and its roller tip on the ground a good step in front; the head raised and listening.", de: "Aufrechter Stand auf beiden Füßen, die rechte Hand in der Körpermitte eine Unterarmlänge vor dem Gürtel, der Stock in Verlängerung des Zeigefingers gerade voraus und seine Rollspitze einen guten Schritt voraus am Boden; der Kopf erhoben, lauschend." };
pose("diagonal-technique").description = { en: "The cane held across the body: the right hand low beside the hip, the shaft running diagonally to the ground ahead of the left foot, as used in crowded rooms and along walls.", de: "Der Stock quer vor dem Körper gehalten: die rechte Hand tief neben der Hüfte, der Schaft läuft schräg zum Boden vor dem linken Fuß, wie in engen Räumen und entlang von Wänden." };
pose("cane-parked").description = { en: "Standing and waiting with the cane held almost upright close to the body, the hand at chest height and the tip beside the right foot.", de: "Stehen und Warten, der Stock fast senkrecht nahe am Körper gehalten, die Hand auf Brusthöhe und die Spitze neben dem rechten Fuß." };

/* ---------------------------------------------------------------- gait */
const GAIT = { period: 1.1, speed: 1.0, heelAhead: 0.18, toeOffDeg: -45, strikeDeg: 15, hipMean: 0.9, hipBob: 0.009 } as const;
const hermite = (p0: number, p1: number, m0: number, m1: number, s: number): number => (2 * s ** 3 - 3 * s * s + 1) * p0 + (s ** 3 - 2 * s * s + s) * m0 + (-2 * s ** 3 + 3 * s * s) * p1 + (s ** 3 - s * s) * m1;
const HEEL: [number, number] = [BODY.heelX, -BODY.ankle];
const BALL: [number, number] = [BODY.ballX, -BODY.ankle];
const neg = (p: readonly [number, number]): [number, number] => [-p[0], -p[1]];
/** 👟 The height of the ankle that puts the lowest point of the sole on the ground for a pitch and a toe bend. */
function touching(pitch: number, toe: number): number {
  const tip = rot([BODY.toe, 0], pitch + toe);
  const ball = rot(BALL, pitch);
  return -Math.min(...SOLE.map((p) => rot(p, pitch)[1]), ball[1] + tip[1]);
}
/** 🚶 One foot at the phase `p` of its cycle (0 = its heel strike): the ankle from the hip joint (x ahead of the hip, y above the ground), the pitch of the sole and the bend of the toe joint. */
function footAt(p: number): { x: number; y: number; pitch: number; toe: number } {
  const stride = GAIT.speed * GAIT.period;
  const heel = GAIT.heelAhead - stride * p;
  if (p < 0.1) {
    const pitch = GAIT.strikeDeg * (1 - smooth(p / 0.1));
    const a = rot(neg(HEEL), pitch);
    return { x: heel + a[0], y: a[1], pitch, toe: 0 };
  }
  if (p < 0.25) return { x: heel - HEEL[0], y: BODY.ankle, pitch: 0, toe: 0 };
  const ball = heel + BALL[0] - HEEL[0];
  const off = rot(neg(BALL), GAIT.toeOffDeg);
  if (p < 0.6) {
    const pitch = GAIT.toeOffDeg * ((p - 0.25) / 0.35) ** 2;
    const a = rot(neg(BALL), pitch);
    return { x: ball + a[0], y: a[1], pitch, toe: -pitch };
  }
  const s = (p - 0.6) / 0.4;
  const x0 = GAIT.heelAhead - stride * 0.6 + BALL[0] - HEEL[0] + off[0];
  const x1 = GAIT.heelAhead + rot(neg(HEEL), GAIT.strikeDeg)[0];
  const pitch = GAIT.toeOffDeg + (GAIT.strikeDeg - GAIT.toeOffDeg) * smooth(s / 0.92) ** 0.8;
  const toe = -GAIT.toeOffDeg * (1 - smooth(s / 0.3));
  const lift = 0.02 * Math.sin(Math.PI * s) + 0.03 * Math.sin(Math.PI * Math.min(1, s / 0.5)) ** 2 * (1 - s);
  return { x: hermite(x0, x1, -stride * 0.4, -stride * 0.4, s), y: touching(pitch, toe) + lift, pitch, toe };
}
const hipAt = (t: number): number => GAIT.hipMean - GAIT.hipBob * Math.cos((2 * Math.PI * (t - 0.005)) / (GAIT.period / 2));
const wrapPhase = (x: number): number => ((x % 1) + 1) % 1;
function gaitBones(t: number): { bones: Bones; knees: number[]; reach: number[] } {
  const bones: Bones = { pelvis: { angleDeg: 0 }, torso: { angleDeg: -2 }, neck: { angleDeg: -2 }, head: { angleDeg: 5 } };
  const swing = Math.cos((2 * Math.PI * t) / GAIT.period);
  bones["arm-l"] = { angleDeg: r2(4 - 14 * swing), abductionDeg: 6 };
  bones["forearm-l"] = { angleDeg: r2(16 - 6 * swing), abductionDeg: 6 };
  bones["hand-l"] = { angleDeg: 4 };
  const knees: number[] = [];
  const reach: number[] = [];
  for (const [side, shift] of [["l", 0], ["r", 0.5]] as const) {
    const f = footAt(wrapPhase(t / GAIT.period - shift));
    const s = leg([f.x, f.y - hipAt(t)], f.pitch);
    bones[`thigh-${side}`] = { angleDeg: s.thigh };
    bones[`shank-${side}`] = { angleDeg: s.shank };
    bones[`foot-${side}`] = { angleDeg: s.foot };
    bones[`toe-${side}`] = { angleDeg: r4(f.toe) };
    knees.push(s.knee);
    reach.push(s.reach);
  }
  return { bones, knees, reach };
}
const walk = asset.clips.find((c: Json) => c.id === "constant-contact-walk");
const active = (t: number): string[] => {
  const on = (walk.contacts as Json[]).filter((c) => t >= c.fromS - 1e-9 && t <= c.toS + 1e-9).map((c) => c.anchor as string);
  return on.length > 0 ? [...new Set(on)] : FEET;
};
const WALK_HOLD: Hold = { wrist: [0.27, 0.05, -0.08], elbowZ: -0.195, tipZ: 0 };
walk.contacts = [{ anchor: "heel-l", fromS: 0, toS: 0.275 }, { anchor: "ball-l", fromS: 0.11, toS: 0.66 }, { anchor: "heel-r", fromS: 0.55, toS: 0.825 }, { anchor: "ball-r", fromS: 0, toS: 0.11 }, { anchor: "ball-r", fromS: 0.66, toS: 1.1 }];
const armSeed: Bones = {};
{
  const mid = gaitBones(0.275);
  hold(mid.bones, active(0.275), WALK_HOLD, "walk arm");
  for (const bn of ["arm-r", "forearm-r", "hand-r"]) armSeed[bn] = { ...mid.bones[bn] };
}
const SWEEP = 0.24;
function walkKey(t: number): Bones {
  const g = gaitBones(t % GAIT.period);
  for (const bn of ["arm-r", "forearm-r", "hand-r"]) g.bones[bn] = { ...armSeed[bn] };
  g.bones.cane = { angleDeg: 0 };
  sweep(g.bones, active(t % GAIT.period), -SWEEP * Math.cos((2 * Math.PI * t) / GAIT.period), `walk ${t}`);
  if (process.argv.includes("--table")) {
    const f = framesOf(g.bones, active(t % GAIT.period));
    const tip = tipOf(f);
    console.log(`t ${t.toFixed(2)} hip ${f.pelvis!.o[1]!.toFixed(3)} (meant ${hipAt(t).toFixed(3)}) knee l ${g.knees[0]!.toFixed(0)} r ${g.knees[1]!.toFixed(0)} thigh l ${g.bones["thigh-l"].angleDeg.toFixed(0)} r ${g.bones["thigh-r"].angleDeg.toFixed(0)} reach ${g.reach.map((x) => x.toFixed(3)).join(" ")} tip ${tip.map((x) => x.toFixed(3)).join(" ")} wrist ${g.bones["hand-r"].angleDeg} ${g.bones["hand-r"].abductionDeg} low l ${Math.min(f["foot-l"]!.o[1]! - touching(0, 0) * 0, f["toe-l"]!.o[1]!).toFixed(3)} r ${f["toe-r"]!.o[1]!.toFixed(3)}`);
  }
  return g.bones;
}
const ORDER = ["pelvis", "torso", "neck", "head", "arm-l", "forearm-l", "hand-l", "thigh-l", "shank-l", "foot-l", "toe-l", "arm-r", "forearm-r", "hand-r", "thigh-r", "shank-r", "foot-r", "toe-r", "cane"];
const ordered = (b: Bones): Bones => Object.fromEntries(ORDER.map((k) => [k, b[k]]));
const first = walkKey(0);
for (const key of walk.keys as Json[]) key.bones = ordered(key.t >= GAIT.period - 1e-9 ? JSON.parse(JSON.stringify(first)) : key.t === 0 ? first : walkKey(key.t));
walk.description = { en: "Walking at 1.0 m/s with a 1.1 m stride: heel strike with an almost straight knee, the foot rolls to the toes and swings through; the hand stays at the midline while the wrist sweeps the roller tip over the ground from side to side, always to the side opposite the leading foot, a good step ahead; the free arm swings opposite the leg.", de: "Gehen mit 1,0 m/s und 1,1 m Doppelschritt: Fersenkontakt mit fast gestrecktem Knie, der Fuß rollt bis zu den Zehen ab und schwingt durch; die Hand bleibt in der Körpermitte, während das Handgelenk die Rollspitze am Boden von Seite zu Seite pendelt, stets auf die Seite gegenüber dem vorderen Fuß, einen guten Schritt voraus; der freie Arm schwingt gegengleich zum Bein." };
{
  const p = pose("walking-midstance");
  p.bones = ordered(walkKey(0.3));
  p.root = { x: 0, y: r4(hipAt(0.3) - BODY.hip) };
  p.description = { en: "Left foot flat under the body with the knee almost straight, the right leg swinging through, the left arm swinging opposite and the roller tip of the cane sliding over the ground to the left of the path.", de: "Linker Fuß flach unter dem Körper mit fast gestrecktem Knie, das rechte Bein schwingt durch, der linke Arm schwingt gegengleich und die Rollspitze des Stocks gleitet links der Gehlinie über den Boden." };
  report("walking-midstance", p.bones, p.contacts);
}

/* ---------------------------------------------------------------- listening */
const listen = asset.clips.find((c: Json) => c.id === "listen-and-scan");
const base = pose("constant-contact-stance").bones as Bones;
const SCAN: [number, number, number][] = [[-1, 6, 0], [-2.5, 11, 0.12], [0, -2, 0], [-1.5, 8, -0.12], [-1, 6, 0]];
(listen.keys as Json[]).forEach((key, i) => {
  const [lean, head, tipZ] = SCAN[i]!;
  const b: Bones = JSON.parse(JSON.stringify(base));
  b.torso = { angleDeg: lean };
  b.head = { angleDeg: head };
  sweep(b, FEET, tipZ, `listen ${i}`);
  key.bones = ordered(b);
});
listen.description = { en: "Standing at a crossing: the head turns up and down while the wrist sweeps the roller tip slowly to the left and to the right over the ground ahead; quiet breathing.", de: "Stehen an einer Querung: Der Kopf wendet sich auf und ab, während das Handgelenk die Rollspitze langsam nach links und nach rechts über den Boden voraus führt; ruhiges Atmen." };
for (const p of asset.poses as Json[]) p.bones = ordered(p.bones);

asset.summary = { en: "A blind woman, 1.68 m, in a denim jacket and sunglasses walks with a 1.3 m long white cane: the right hand holds the grip at the midline with the index finger along it, and the roller tip stays on the ground a good step ahead, sweeping from side to side ahead of every step.", de: "Eine blinde Frau, 1,68 m, in Jeansjacke und Sonnenbrille geht mit einem 1,3 m langen weißen Langstock: Die rechte Hand hält den Griff in der Körpermitte, der Zeigefinger liegt am Griff, und die Rollspitze bleibt einen guten Schritt voraus am Boden und pendelt vor jedem Schritt von Seite zu Seite." };

/* the contact point of the roller in the default pose, in the frame of the cane */
{
  const f = framesOf(pose(asset.presentation.defaultPose).bones, FEET);
  const h = f.head!;
  for (const [id, x, y] of [["nose", 0.1255, 1.5555], ["mouth", 0.1135, 1.531], ["crown", 0.008, 1.707]] as const) Object.assign(anchor(id), { x: r4((x - h.o[0]!) * h.u[0]! + (y - h.o[1]!) * h.u[1]!), y: r4((x - h.o[0]!) * h.v[0]! + (y - h.o[1]!) * h.v[1]!) });
  const c = f.cane!;
  const down = [0, -1, 0];
  const lx = down[0]! * c.u[0]! + down[1]! * c.u[1]! + down[2]! * c.u[2]!;
  const ly = down[0]! * c.v[0]! + down[1]! * c.v[1]! + down[2]! * c.v[2]!;
  const lz = down[0]! * c.w[0]! + down[1]! * c.w[1]! + down[2]! * c.w[2]!;
  Object.assign(anchor("cane-tip"), { x: r4(CANE.length - CANE.roller + CANE.roller * lx), y: r4(CANE.roller * ly), z: r4(CANE.roller * lz), footprintLengthM: 0.02, footprintWidthM: 0.02 });
}

for (const a of asset.anchors as Json[]) {
  if (a.z === undefined) continue;
  const copy: Json = {};
  for (const [k, v] of Object.entries(a)) { if (k === "z") continue; copy[k] = v; if (k === "y") copy.z = a.z; }
  for (const k of Object.keys(a)) delete a[k];
  Object.assign(a, copy);
}

if (import.meta.main) {
  mkdirSync(OUT, { recursive: true });
  writeFileSync(join(OUT, "roots.json"), JSON.stringify({ summary: asset.summary, rig: asset.rig, anchors: asset.anchors, poses: asset.poses, clips: asset.clips }));
  console.log("written roots.json");
}
