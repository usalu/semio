/** 🩼 Root fixes of `humans/person-on-crutches` outside `volumes` and the 2D drawings: the trunk, neck and head lengths, the forearm crutch as a rigid unit in the gripping hand (shaft behind the grip, cuff arm along the forearm), the anchors on head, hand and crutch, and every pose and clip key stated anew as a three-point swing-to gait whose planted feet and tips stay where they stand.
 *
 * `applyRoots(asset)` changes an asset object in memory (used by the kit, the drawing and the writer); run as a program it prints the solved poses and the planted-contact check.
 * Usage: `bun r3-person-on-crutches-roots.ts`
 * @see ./r3-humans-person-on-crutches.md */
import { solveChain } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts";
import { rigAt } from "./r3-lib.ts";
import { entryOf, type Json } from "./volumes-lib.ts";

const rad = (d: number): number => (d * Math.PI) / 180;
const deg = (r: number): number => (r * 180) / Math.PI;
const r4 = (x: number): number => Math.round(x * 1e4) / 1e4;
const smooth = (u: number): number => { const c = Math.min(1, Math.max(0, u)); return c * c * (3 - 2 * c); };
const mix = (a: number, b: number, w: number): number => a + (b - a) * w;
type P = [number, number];

/** 📐 The corrected rig lengths: C7 at 0.86 of the stature, the atlanto-occipital joint 0.10 m above it, the vertex 0.152 m above that. */
export const TORSO_L = 0.497;
export const NECK_L = 0.1;
export const HEAD_L = 0.156;
/** 🩼 The crutch in the hand frame: the junction of handle and shaft, the direction of the shaft relative to the hand, the shaft length to the tip, the wrist extension on the handle and the spread of the arm. */
export const CRUTCH_AT: P = [-0.0017, -0.0471];
export const CRUTCH_REST = -51.5;
export const CRUTCH_L = 0.9;
export const WRIST = 33;
/** 🪽 The spread of the arm and crutch plane about the shoulder, written as lateral offsets of elbow, wrist, crutch junction and tip (the rig stays sagittal): metres to the outside per metre of drop. */
export const SPREAD = 0.0875;
export const ELBOW_Z = 0.0295;
export const WRIST_Z = 0.0205;
export const JUNCTION_Z = 0.0035;
export const TIP_Z = 0.0775;
/** ✊ The middle of the gripped handle in the hand frame. */
export const GRIP_AT: P = [0.05, -0.017];

const THIGH = 0.4459;
/** 🦶 Shank, foot and toe in the sneaker: the ankle joint 95 mm above the sole plane (0.039 H + 25 mm heel stack), the knee kept at 0.53 m; the foot bone runs to the ball contact 149 mm ahead of the ankle. */
const SHANK = 0.4357;
const FOOT = 0.17693;
const FOOT_REST = 57.5234;
const TOE_REST = 32.4766;
const ANKLE_Y = FOOT * Math.sin(rad(TOE_REST));
const BALL_X = FOOT * Math.cos(rad(TOE_REST));
const HEEL: P = [-0.02579, -0.12902];
const HIP_Y = 0.9766;
const ADVANCE = 0.4;
const T_LAND = 0.7;
const T_OFF = 0.83;
const T_STRIKE = 1.35;
const T_FLAT = 1.5;
const T_SETTLE = 1.6;
const DURATION = 1.8;

type Offsets = Record<string, { angleDeg: number; abductionDeg?: number }>;
type State = { offsets: Offsets; root: P };

function rigFix(asset: Json): void {
  const bone = (id: string): Json => asset.rig.bones.find((b: Json) => b.id === id);
  const torso = bone("torso");
  torso.mass.comX = r4((torso.mass.comX / torso.lengthM) * TORSO_L);
  torso.lengthM = TORSO_L;
  bone("neck").x = TORSO_L;
  bone("neck").lengthM = NECK_L;
  bone("head").x = NECK_L;
  bone("head").lengthM = HEAD_L;
  bone("head").mass.comX = 0.045;
  for (const side of ["l", "r"]) {
    const c = bone(`crutch-${side}`);
    c.x = CRUTCH_AT[0];
    c.y = CRUTCH_AT[1];
    c.angleDeg = CRUTCH_REST;
    c.lengthM = CRUTCH_L;
    c.mass.comX = 0.29;
    c.mass.comY = -0.01;
    const sign = side === "l" ? 1 : -1;
    c.z = sign * JUNCTION_Z;
    bone(`forearm-${side}`).z = sign * ELBOW_Z;
    bone(`hand-${side}`).z = sign * WRIST_Z;
    asset.anchors.find((a: Json) => a.id === `tip-${side}`).z = sign * TIP_Z;
    asset.anchors.find((a: Json) => a.id === `grip-${side}`).z = sign * 0.003;
    const hand = bone(`hand-${side}`);
    hand.mass.comX = 0.055;
    hand.mass.comY = -0.012;
    const grip = asset.anchors.find((a: Json) => a.id === `grip-${side}`);
    grip.x = GRIP_AT[0];
    grip.y = GRIP_AT[1];
    asset.anchors.find((a: Json) => a.id === `tip-${side}`).x = CRUTCH_L;
  }
  for (const side of ["l", "r"]) {
    bone(`shank-${side}`).lengthM = SHANK;
    Object.assign(bone(`foot-${side}`), { x: SHANK, angleDeg: FOOT_REST, lengthM: FOOT });
    Object.assign(bone(`toe-${side}`), { x: FOOT, angleDeg: TOE_REST });
    Object.assign(asset.anchors.find((a: Json) => a.id === `heel-${side}`), { x: HEEL[0], y: HEEL[1] });
    Object.assign(asset.anchors.find((a: Json) => a.id === `ball-${side}`), { x: FOOT, y: 0 });
  }
  asset.representations["500"].validRange.maxDenominator = 750;
  if (!asset.materials.some((m: Json) => m.id === "bone")) asset.materials.push({ id: "bone", title: { en: "Bone", de: "Knochen" }, emissivity: 0.9, solarAbsorptivity: 0.5, densityKgM3: 1900, source: "authored-estimate" });
  const anchor = (id: string): Json => asset.anchors.find((a: Json) => a.id === id);
  Object.assign(anchor("nose"), { x: -0.016, y: -0.1 });
  Object.assign(anchor("mouth"), { x: -0.038, y: -0.094 });
  Object.assign(anchor("crown"), { x: 0.164, y: -0.012 });
}

/** 🦴 The chain of a state: origin and axes of every bone in the side plane, with the lateral position. */
function frames(asset: Json, s: State): Record<string, Json> {
  return solveChain(asset.rig, s.offsets as never, { x: s.root[0], y: s.root[1], angleDeg: 0 }) as never;
}
function at(asset: Json, s: State, boneId: string, x: number, y: number): P {
  const f = frames(asset, s)[boneId]!.frame;
  return [f.o[0] + f.u[0] * x + f.v[0] * y, f.o[1] + f.u[1] * x + f.v[1] * y];
}
const tipOf = (asset: Json, s: State, side: string): P => at(asset, s, `crutch-${side}`, CRUTCH_L, 0);

/** 🦵 Thigh and shank offsets that carry the ankle from the hip `h` to `a`, the knee ahead; an ankle out of reach is drawn in along the hip line. */
function leg(h: P, a: P): { thigh: number; shank: number; shankWorld: number; short: number } {
  let d = Math.hypot(a[0] - h[0], a[1] - h[1]);
  const max = (THIGH + SHANK) * 0.9992;
  const short = Math.max(0, d - max);
  d = Math.min(d, max);
  const base = Math.atan2(a[1] - h[1], a[0] - h[0]);
  const thighWorld = base + Math.acos((THIGH * THIGH + d * d - SHANK * SHANK) / (2 * THIGH * d));
  const flex = Math.PI - Math.acos((THIGH * THIGH + SHANK * SHANK - d * d) / (2 * THIGH * SHANK));
  return { thigh: deg(thighWorld) + 90, shank: -deg(flex), shankWorld: deg(thighWorld - flex), short };
}

type Design = { hip: P; torso: number; headWorld: number; arm: number; fore: number; ankleR: P; pitchR: number; toeR: number; ankleL: P };
/** 🧍 The offsets of a designed state: trunk, both arms alike, the right leg to its ankle with the foot pitched (`pitchR` toe up), the left leg to its hovering ankle with the boot rigid at the neutral angle. */
function stateOf(d: Design): State {
  const neck = r4(-d.torso * 0.45);
  const right = leg(d.hip, d.ankleR);
  const left = leg(d.hip, d.ankleL);
  const footR = -TOE_REST + d.pitchR - right.shankWorld - FOOT_REST;
  const o: Offsets = {
    pelvis: { angleDeg: 0 },
    torso: { angleDeg: r4(d.torso) },
    neck: { angleDeg: neck },
    head: { angleDeg: r4(d.headWorld - 90 - d.torso - neck) },
    "arm-l": { angleDeg: r4(d.arm) },
    "forearm-l": { angleDeg: r4(d.fore) },
    "hand-l": { angleDeg: WRIST },
    "thigh-l": { angleDeg: r4(left.thigh) },
    "shank-l": { angleDeg: r4(left.shank) },
    "foot-l": { angleDeg: 0 },
    "toe-l": { angleDeg: 0 },
    "arm-r": { angleDeg: r4(d.arm) },
    "forearm-r": { angleDeg: r4(d.fore) },
    "hand-r": { angleDeg: WRIST },
    "thigh-r": { angleDeg: r4(right.thigh) },
    "shank-r": { angleDeg: r4(right.shank) },
    "foot-r": { angleDeg: r4(footR) },
    "toe-r": { angleDeg: r4(d.toeR) },
    "crutch-l": { angleDeg: 0 },
    "crutch-r": { angleDeg: 0 },
  };
  return { offsets: o, root: [d.hip[0], d.hip[1] - HIP_Y] };
}

/** 🔧 Solves two unknowns of a design by Newton steps so that `f` vanishes. */
function solve2(start: P, f: (q: P) => P): P {
  let q: P = [...start];
  for (let i = 0; i < 60; i++) {
    const e = f(q);
    if (Math.hypot(e[0], e[1]) < 2e-6) return q;
    const h = 0.02;
    const ea = f([q[0] + h, q[1]]);
    const eb = f([q[0], q[1] + h]);
    const [a, b, c, d2] = [(ea[0] - e[0]) / h, (eb[0] - e[0]) / h, (ea[1] - e[1]) / h, (eb[1] - e[1]) / h];
    const det = a * d2 - b * c;
    if (Math.abs(det) < 1e-12) break;
    const step: P = [(-e[0] * d2 + e[1] * b) / det, (e[0] * c - e[1] * a) / det];
    const k = Math.min(1, 8 / Math.max(1e-9, Math.hypot(step[0], step[1])));
    q = [q[0] + step[0] * k, q[1] + step[1] * k];
  }
  const e = f(q);
  if (Math.hypot(e[0], e[1]) > 1e-5) throw new Error(`no solution, residual ${e}`);
  return q;
}

const HOVER_Y = 0.137;
const HOVER_AHEAD = 0.165;
const STAND_HIP: P = [0, ANKLE_Y + (THIGH + SHANK) * Math.cos(rad(4))];
const STAND_TORSO = -7;
const STAND_FORE = 27;
const REACH_HIP: P = [0.17, STAND_HIP[1] - 0.03];
const REACH_FORE = 15;
const flat = (x: number): P => [x, ANKLE_Y];

/** 🎼 The gait as designed states over one cycle, the right ankle starting at x = 0. */
function choreography(asset: Json): (t: number) => State {
  const stand = (arm: number, hip: P = STAND_HIP, torso = STAND_TORSO, fore = STAND_FORE): Design => ({ hip, torso, headWorld: 90, arm, fore, ankleR: flat(0), pitchR: 0, toeR: 0, ankleL: [hip[0] + HOVER_AHEAD, HOVER_Y] });
  const [arm0] = solve2([10, 0], (q) => [tipOf(asset, stateOf(stand(q[0])), "l")[1], q[1]]);
  const tip0 = tipOf(asset, stateOf(stand(arm0)), "l");
  const [arm1, torso1] = solve2([25, -20], (q) => {
    const tip = tipOf(asset, stateOf(stand(q[0], REACH_HIP, q[1], REACH_FORE)), "l");
    return [tip[0] - tip0[0] - ADVANCE, tip[1]];
  });
  const tip1: P = [tip0[0] + ADVANCE, 0];
  const heelLand: P = [ADVANCE + (flat(0)[0] + HEEL[0] * Math.cos(rad(-TOE_REST)) - HEEL[1] * Math.sin(rad(-TOE_REST))), 0];
  const ankleOnHeel = (pitch: number): P => {
    const a = rad(-TOE_REST + pitch);
    return [heelLand[0] - (HEEL[0] * Math.cos(a) - HEEL[1] * Math.sin(a)), -(HEEL[0] * Math.sin(a) + HEEL[1] * Math.cos(a))];
  };
  const ankleOnBall = (pitch: number): P => {
    const a = rad(-TOE_REST + pitch);
    return [BALL_X - FOOT * Math.cos(a), -FOOT * Math.sin(a)];
  };
  const OFF_PITCH = -32;
  const STRIKE_PITCH = 11;
  return (time: number): State => {
    const t = Math.min(DURATION, Math.max(0, time));
    if (t <= T_LAND) {
      const w = smooth(t / T_LAND);
      const hip: P = [mix(STAND_HIP[0], REACH_HIP[0], w), mix(STAND_HIP[1], REACH_HIP[1], w)];
      const air = Math.sin(Math.PI * Math.min(1, t / T_LAND)) ** 2;
      const d = stand(mix(arm0, arm1, w) - 2 * air, hip, mix(STAND_TORSO, torso1, w), mix(STAND_FORE, REACH_FORE, w) + 9 * air);
      d.headWorld = mix(90, 82, w);
      d.ankleL = [hip[0] + HOVER_AHEAD + 0.02 * w, HOVER_Y + 0.012 * w];
      return stateOf(d);
    }
    if (t >= T_SETTLE) { const s0 = stateOf(stand(arm0)); return { offsets: s0.offsets, root: [ADVANCE, s0.root[1]] }; }
    const w = smooth((t - T_LAND) / (T_SETTLE - T_LAND));
    const d = stand(mix(arm1, arm0, w), [0, 1], mix(torso1, STAND_TORSO, w), mix(REACH_FORE, STAND_FORE, w));
    d.headWorld = mix(82, 90, w);
    const loose = stateOf({ ...d, ankleR: [0, 0.1], ankleL: [0.1, 0.2] });
    const tip = tipOf(asset, loose, "l");
    const hip: P = [tip1[0] - tip[0], 1 + tip1[1] - tip[1]];
    d.hip = hip;
    const swing = Math.sin(Math.PI * smooth((t - T_LAND) / (T_STRIKE - T_LAND))) ** 2;
    d.ankleL = [hip[0] + HOVER_AHEAD + 0.02 * (1 - w) + 0.03 * swing, HOVER_Y + 0.012 * (1 - w) + 0.01 * swing];
    if (t <= T_OFF) {
      const p = OFF_PITCH * smooth((t - T_LAND) / (T_OFF - T_LAND));
      d.ankleR = ankleOnBall(p);
      d.pitchR = p;
      d.toeR = -p;
    } else if (t < T_STRIKE) {
      const u = (t - T_OFF) / (T_STRIKE - T_OFF);
      const a = ankleOnBall(OFF_PITCH);
      const b = ankleOnHeel(STRIKE_PITCH);
      const e = smooth(u);
      d.ankleR = [mix(a[0], b[0], e), mix(a[1], b[1], e) + 0.035 * Math.sin(Math.PI * u) ** 2];
      d.pitchR = mix(OFF_PITCH, STRIKE_PITCH, smooth(u * 1.15));
      d.toeR = -OFF_PITCH * (1 - smooth(u * 2.5));
    } else if (t < T_FLAT) {
      const p = STRIKE_PITCH * (1 - smooth((t - T_STRIKE) / (T_FLAT - T_STRIKE)));
      d.ankleR = ankleOnHeel(p);
      d.pitchR = p;
    } else d.ankleR = flat(ADVANCE);
    return stateOf(d);
  };
}

const plain = (o: Offsets): Json => Object.fromEntries(Object.entries(o).map(([k, v]) => [k, v.abductionDeg === undefined ? { angleDeg: v.angleDeg } : { angleDeg: v.angleDeg, abductionDeg: v.abductionDeg }]));

/** 🌱 Applies every root fix to the asset object: rig, anchors, poses, clips. */
export function applyRoots(asset: Json): Json {
  rigFix(asset);
  const gait = choreography(asset);
  const pose = (id: string): Json => asset.poses.find((p: Json) => p.id === id);
  const set = (id: string, t: number, en: string, de: string): void => {
    const p = pose(id);
    p.bones = plain(gait(t).offsets);
    p.root = { x: 0, y: 0 };
    p.description = { en, de };
  };
  set("tripod-stand", 0, "Standing upright on the straight right leg, both crutch tips planted a hand's breadth ahead of the toes and a hand's breadth outside the feet, elbows bent 30 degrees, the left foot in its boot held level a few centimetres above the ground ahead of the body.", "Aufrechter Stand auf dem gestreckten rechten Bein, beide Krückenspitzen eine Handbreit vor den Zehen und eine Handbreit außerhalb der Füße aufgesetzt, Ellbogen 30 Grad gebeugt, der linke Fuß im Stiefel waagrecht wenige Zentimeter über dem Boden vor dem Körper gehalten.");
  set("crutches-forward", 0.4, "Weight on the right leg alone, the trunk leaning forward, while both crutches are lifted and reach forward with the elbows opening.", "Das Gewicht allein auf dem rechten Bein, der Rumpf neigt sich vor, während beide Krücken angehoben nach vorn greifen und die Ellbogen sich öffnen.");
  set("body-swing", 1.05, "Weight on the hands and both planted crutch tips: the body swings forward between the crutches, the right foot is off the ground and reaches for its landing just behind the tips.", "Das Gewicht auf den Händen und beiden aufgesetzten Krückenspitzen: Der Körper schwingt zwischen den Krücken nach vorn, der rechte Fuß ist vom Boden gelöst und greift nach seiner Landung knapp hinter den Spitzen.");
  const walk = asset.clips.find((c: Json) => c.id === "three-point-gait");
  for (const key of walk.keys as Json[]) key.bones = plain(gait(key.t).offsets);
  walk.description = { en: "Swing-to gait with the left foot off-loaded: the crutches reach 0.4 m forward while the right leg carries, the body swings on the hands to the crutch line and the right foot lands just behind the tips; 0.4 m per 1.8 s.", de: "Durchschwung bis zur Krückenlinie bei entlastetem linkem Fuß: Die Krücken greifen 0,4 m vor, während das rechte Bein trägt, der Körper schwingt auf den Händen zur Krückenlinie und der rechte Fuß landet knapp hinter den Spitzen; 0,4 m pro 1,8 s." };
  const rest = asset.clips.find((c: Json) => c.id === "rest-on-crutches");
  const stand0 = gait(0);
  const tip0 = tipOf(asset, stand0, "l");
  const sway: Record<string, [number, number]> = { "0": [STAND_TORSO, 90], "1.5": [-10, 84], "3": [-5, 94], "4.5": [-8, 88], "6": [STAND_TORSO, 90] };
  for (const key of rest.keys as Json[]) {
    const [torso, headWorld] = sway[String(key.t)]!;
    const make = (q: P): State => stateOf({ hip: STAND_HIP, torso, headWorld, arm: q[0], fore: q[1], ankleR: flat(0), pitchR: 0, toeR: 0, ankleL: [HOVER_AHEAD, HOVER_Y] });
    const q = solve2([stand0.offsets["arm-l"]!.angleDeg, STAND_FORE], (v) => { const tip = tipOf(asset, make(v), "l"); return [tip[0] - tip0[0], tip[1]]; });
    key.bones = plain(make(q).offsets);
  }
  return asset;
}

if (import.meta.main) {
  const { asset: catalog } = entryOf("humans/person-on-crutches");
  const asset = applyRoots(JSON.parse(JSON.stringify(catalog)) as Json);
  const still = JSON.parse(JSON.stringify(asset)) as Json;
  still.thermal.respiration.motion = [];
  const anchorAt = (q: Parameters<typeof rigAt>[2], id: string): number[] => {
    const rig = rigAt(still, 50, q);
    const an = still.anchors.find((x: Json) => x.id === id);
    const f = rig.frames[an.bone]!;
    return [0, 1, 2].map((k) => f.o[k]! + f.u[k]! * an.x + f.v[k]! * an.y);
  };
  const f3 = (v: number[]): string => v.map((x) => x.toFixed(3)).join(",");
  for (const p of asset.poses as Json[]) {
    console.log(`pose ${p.id}: ${["arm-l", "forearm-l", "thigh-r", "shank-r", "foot-r", "thigh-l", "shank-l", "torso", "neck", "head"].map((b) => `${b}=${p.bones[b].angleDeg}`).join(" ")}`);
    console.log(`   tip-l ${f3(anchorAt({ kind: "pose", pose: p.id }, "tip-l"))} tip-r ${f3(anchorAt({ kind: "pose", pose: p.id }, "tip-r"))} heel-r ${f3(anchorAt({ kind: "pose", pose: p.id }, "heel-r"))} ball-r ${f3(anchorAt({ kind: "pose", pose: p.id }, "ball-r"))} grip-l ${f3(anchorAt({ kind: "pose", pose: p.id }, "grip-l"))} crown ${f3(anchorAt({ kind: "pose", pose: p.id }, "crown"))}`);
  }
  for (let t = 0; t <= 1.8001; t += 0.05) {
    const q = { kind: "clip" as const, clip: "three-point-gait", timeS: Math.min(t, 1.7999) };
    const rig = rigAt(still, 50, q);
    const k = asset.clips[0].keys.find((x: Json) => Math.abs(x.t - t) < 1e-6)?.bones;
    console.log(`t=${t.toFixed(2)} hip ${f3(rig.frames.pelvis!.o.slice(0, 2))} tip ${f3(anchorAt(q, "tip-l").slice(0, 2))} heel ${f3(anchorAt(q, "heel-r").slice(0, 2))} ball ${f3(anchorAt(q, "ball-r").slice(0, 2))} | arm ${k?.["arm-l"].angleDeg} fore ${k?.["forearm-l"].angleDeg} torso ${k?.torso.angleDeg} thR ${k?.["thigh-r"].angleDeg} shR ${k?.["shank-r"].angleDeg} ftR ${k?.["foot-r"].angleDeg} thL ${k?.["thigh-l"].angleDeg} shL ${k?.["shank-l"].angleDeg}`);
  }
  for (const t of [0, 1.5, 3, 4.5]) {
    const q = { kind: "clip" as const, clip: "rest-on-crutches", timeS: t };
    console.log(`rest t=${t} tip ${f3(anchorAt(q, "tip-l"))} heel ${f3(anchorAt(q, "heel-r"))}`);
  }
}
