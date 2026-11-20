/** 🦿️ Root fixes of the rig of humans/runner-with-blade-prosthesis (round 3): joint positions against the anthropometric and manufacturer references, anchors, and every pose and the running clip re-authored on the corrected rig (stance by inverse kinematics on the planted contact, swing from stated joint angles, ballistic flights).
 *
 * References: de Leva 1996 and Drillis and Contini 1966 (segment lengths), Bell et al. 1990 (hip joint centre 0.36 of the inter-ASIS breadth from the midline), ANSUR II (biacromial 0.415 m at 1.756 m), Novacheck 1998 (running kinematics: knee 20 to 45 degrees in stance, about 100 degrees in swing, hip 45 degrees flexion), Össur Flex-Run (build height 261 mm, 277 mm with sole) and Ottobock 1E91 Runner (300 mm, 270 mm under load).
 * @see ./r3-humans-runner-with-blade-prosthesis.md */
export type Json = any;

const RAD = Math.PI / 180;
const r4 = (x: number): number => Math.round(x * 1e4) / 1e4;
const r5 = (x: number): number => Math.round(x * 1e5) / 1e5;
const wrap = (a: number): number => ((((a + 180) % 360) + 360) % 360) - 180;

/** 📐️ Corrected rest geometry (metres, degrees; world x ahead, y up, hip joint axis at x = 0). */
export const RIG = {
  hipY: 0.9335,
  hipZ: 0.09,
  shoulderZ: 0.18,
  torso: 0.51,
  head: 0.151,
  thigh: 0.435,
  shank: 0.425,
  ankleY: 0.0735,
  ballX: 0.135,
  toe: 0.09,
  socket: 0.18,
  bladeHinge: [-0.1108, 0.1891] as [number, number],
  bladeToe: [0.14, 0] as [number, number],
};
const KNEE_Y = RIG.hipY - RIG.thigh;
const ADAPTER_Y = KNEE_Y - RIG.socket;
export const FOOT_LEN = Math.hypot(RIG.ballX, RIG.ankleY);
export const FOOT_REST = 90 - Math.atan2(RIG.ankleY, RIG.ballX) / RAD;
export const TOE_REST = 90 - FOOT_REST;
export const BLADE_LEN = Math.hypot(RIG.bladeHinge[0], RIG.bladeHinge[1] - ADAPTER_Y);
export const BLADE_ABS = Math.atan2(RIG.bladeHinge[1] - ADAPTER_Y, RIG.bladeHinge[0]) / RAD;
export const BLADE_REST = wrap(BLADE_ABS + 90);
export const TIP_LEN = Math.hypot(RIG.bladeToe[0] - RIG.bladeHinge[0], RIG.bladeToe[1] - RIG.bladeHinge[1]);
export const TIP_ABS = Math.atan2(RIG.bladeToe[1] - RIG.bladeHinge[1], RIG.bladeToe[0] - RIG.bladeHinge[0]) / RAD;
export const TIP_REST = wrap(TIP_ABS - BLADE_ABS);

/** 🦴️ Writes the corrected joints, masses and anchors into the asset (idempotent: values are set, never moved). */
export function fixRig(asset: Json): string[] {
  const log: string[] = [];
  const bone = (id: string): Json => asset.rig.bones.find((b: Json) => b.id === id);
  const set = (o: Json, key: string, value: unknown, what: string): void => {
    if (JSON.stringify(o[key]) !== JSON.stringify(value)) log.push(`${what}.${key}: ${JSON.stringify(o[key])} -> ${JSON.stringify(value)}`);
    o[key] = value;
  };
  set(bone("torso").title, "en", "Torso", "rig.torso.title");
  set(bone("torso").title, "de", "Rumpf", "rig.torso.title");
  set(bone("torso"), "lengthM", RIG.torso, "rig.torso");
  set(bone("torso").mass, "comX", r4(RIG.torso * 0.462), "rig.torso.mass");
  set(bone("neck"), "x", RIG.torso, "rig.neck");
  set(bone("head"), "lengthM", RIG.head, "rig.head");
  set(bone("head").mass, "comX", 0.046, "rig.head.mass");
  for (const [id, z] of [["arm-l", RIG.shoulderZ], ["arm-r", -RIG.shoulderZ], ["thigh-l", RIG.hipZ], ["thigh-r", -RIG.hipZ]] as const) set(bone(id), "z", z, `rig.${id}`);
  set(bone("foot-l"), "angleDeg", r4(FOOT_REST), "rig.foot-l");
  set(bone("foot-l"), "lengthM", r5(FOOT_LEN), "rig.foot-l");
  set(bone("foot-l").mass, "comX", r4(FOOT_LEN * 0.43), "rig.foot-l.mass");
  set(bone("toe-l"), "x", r5(FOOT_LEN), "rig.toe-l");
  set(bone("toe-l"), "angleDeg", r4(TOE_REST), "rig.toe-l");
  set(bone("toe-l"), "lengthM", RIG.toe, "rig.toe-l");
  set(bone("toe-l").mass, "comX", r4(RIG.toe * 0.46), "rig.toe-l.mass");
  set(bone("blade-r"), "angleDeg", r4(BLADE_REST), "rig.blade-r");
  set(bone("blade-r"), "lengthM", r5(BLADE_LEN), "rig.blade-r");
  set(bone("blade-r").mass, "comX", r4(BLADE_LEN / 2), "rig.blade-r.mass");
  set(bone("blade-r").title, "en", "Clamp plate and rear curve of the C-shaped carbon blade, bolted to the socket adapter", "rig.blade-r.title");
  set(bone("blade-r").title, "de", "Klemmplatte und hinterer Bogen der C-förmigen Karbonfeder, am Schaftadapter verschraubt", "rig.blade-r.title");
  set(bone("blade-tip-r"), "x", r5(BLADE_LEN), "rig.blade-tip-r");
  set(bone("blade-tip-r"), "angleDeg", r4(TIP_REST), "rig.blade-tip-r");
  set(bone("blade-tip-r"), "lengthM", r5(TIP_LEN), "rig.blade-tip-r");
  set(bone("blade-tip-r").mass, "comX", r4(TIP_LEN / 2), "rig.blade-tip-r.mass");
  set(bone("blade-tip-r").title, "en", "Forefoot lever of the blade with the rubber sole, flexes under load", "rig.blade-tip-r.title");
  set(bone("blade-tip-r").title, "de", "Vorfußhebel der Feder mit Gummisohle, federt unter Last ein", "rig.blade-tip-r.title");
  const anchor = (id: string): Json => asset.anchors.find((a: Json) => a.id === id);
  const a = FOOT_REST - 90;
  const [ux, uy, vx, vy] = [Math.cos(a * RAD), Math.sin(a * RAD), -Math.sin(a * RAD), Math.cos(a * RAD)];
  const heel: [number, number] = [-0.045, -RIG.ankleY];
  set(anchor("heel-l"), "x", r4(heel[0] * ux + heel[1] * uy), "anchors.heel-l");
  set(anchor("heel-l"), "y", r4(heel[0] * vx + heel[1] * vy), "anchors.heel-l");
  set(anchor("ball-l"), "x", r4(FOOT_LEN), "anchors.ball-l");
  set(anchor("blade-toe"), "x", r4(TIP_LEN), "anchors.blade-toe");
  set(anchor("blade-toe"), "footprintLengthM", 0.1, "anchors.blade-toe");
  set(anchor("blade-toe"), "footprintWidthM", 0.062, "anchors.blade-toe");
  set(anchor("mouth"), "x", -0.042, "anchors.mouth");
  set(anchor("mouth"), "y", -0.097, "anchors.mouth");
  set(anchor("nostrils"), "x", -0.017, "anchors.nostrils");
  set(anchor("nostrils"), "y", -0.108, "anchors.nostrils");
  set(anchor("crown"), "x", 0.157, "anchors.crown");
  set(anchor("crown"), "y", 0.004, "anchors.crown");
  return log;
}

/* ---------------------------------------------------------------- running kinematics */

export const CYCLE = { duration: 0.7, advance: 2.17, speed: 3.093, leftOn: 0, leftOff: 7 / 30, bladeOn: 12 / 30, bladeOff: 17 / 30 };
const LEFT = { a: 0.285, yOn: 0.905, yOff: 0.885, dip: 0.042 };
const BLADE = { a: 0.235, yOn: 0.905, yOff: 0.905, dip: 0.008, flexDeg: 11 };
const G = 9.80665;

/** ⛰️ Height of the hip joint over the ground during the stride: a dip in each stance, a ballistic arc in each flight. */
export function hipY(t: number): number {
  const T = CYCLE.duration;
  t = ((t % T) + T) % T;
  const stance = (t0: number, t1: number, p: { yOn: number; yOff: number; dip: number }): number => {
    const s = (t - t0) / (t1 - t0);
    return p.yOn + (p.yOff - p.yOn) * s - p.dip * Math.sin(Math.PI * s) ** 1.2;
  };
  const flight = (t0: number, t1: number, y0: number, y1: number): number => {
    const d = t1 - t0;
    const vy = (y1 - y0 + (G * d * d) / 2) / d;
    return y0 + vy * (t - t0) - (G * (t - t0) ** 2) / 2;
  };
  if (t <= CYCLE.leftOff) return stance(CYCLE.leftOn, CYCLE.leftOff, LEFT);
  if (t < CYCLE.bladeOn) return flight(CYCLE.leftOff, CYCLE.bladeOn, LEFT.yOff, BLADE.yOn);
  if (t <= CYCLE.bladeOff) return stance(CYCLE.bladeOn, CYCLE.bladeOff, BLADE);
  return flight(CYCLE.bladeOff, T, BLADE.yOff, LEFT.yOn);
}

/** 🦵️ Two links from the hip to a target with the knee ahead: absolute angles of both links (degrees, 0 = ahead, 90 = up). */
function twoLink(tx: number, ty: number, l1: number, l2: number): [number, number] {
  const d = Math.min(Math.hypot(tx, ty), l1 + l2 - 1e-4);
  const phi = Math.atan2(ty, tx);
  const alpha = Math.acos(Math.max(-1, Math.min(1, (l1 * l1 + d * d - l2 * l2) / (2 * l1 * d))));
  const a1 = phi + alpha;
  const k: [number, number] = [l1 * Math.cos(a1), l1 * Math.sin(a1)];
  const s = d / Math.hypot(tx, ty);
  return [a1 / RAD, Math.atan2(ty * s - k[1], tx * s - k[0]) / RAD];
}

/** 🌀️ The rigid link from the knee to the blade toe for a flex of the forefoot lever: its length and its angle from the shank axis. */
function bladeLink(flexDeg: number): [number, number] {
  const b = BLADE_REST * RAD;
  const t = (BLADE_REST + TIP_REST + flexDeg) * RAD;
  const ex = RIG.socket + BLADE_LEN * Math.cos(b) + TIP_LEN * Math.cos(t);
  const ey = BLADE_LEN * Math.sin(b) + TIP_LEN * Math.sin(t);
  return [Math.hypot(ex, ey), Math.atan2(ey, ex) / RAD];
}

type Key = [t: number, thigh: number, knee: number];
/** 🔁️ Periodic Catmull-Rom through keys `[t, ...values]` (time in seconds within the stride). */
function spline(keys: readonly number[][], t: number, T: number): number[] {
  const n = keys.length;
  const tt = ((t % T) + T) % T;
  let i = 0;
  for (let k = 0; k < n; k++) {
    const a = keys[k]![0]!;
    const b = keys[(k + 1) % n]![0]! + (k === n - 1 ? T : 0);
    const x = tt < a ? tt + T : tt;
    if (x >= a && x <= b) {
      i = k;
      break;
    }
  }
  const at = (k: number): number[] => keys[((k % n) + n) % n]!;
  const time = (k: number): number => at(k)[0]! + Math.floor(k / n) * T;
  const [t1, t2] = [time(i), time(i + 1)];
  const x = tt < t1 ? tt + T : tt;
  const s = (x - t1) / (t2 - t1);
  return at(i).slice(1).map((_, j) => {
    const [p0, p1, p2, p3] = [at(i - 1)[j + 1]!, at(i)[j + 1]!, at(i + 1)[j + 1]!, at(i + 2)[j + 1]!];
    const m1 = ((p2 - p0) / (time(i + 1) - time(i - 1))) * (t2 - t1);
    const m2 = ((p3 - p1) / (time(i + 2) - time(i))) * (t2 - t1);
    return (2 * s ** 3 - 3 * s ** 2 + 1) * p1 + (s ** 3 - 2 * s ** 2 + s) * m1 + (-2 * s ** 3 + 3 * s ** 2) * p2 + (s ** 3 - s ** 2) * m2;
  });
}

const ease = (s: number): number => s * s * (3 - 2 * s);
const clamp01 = (s: number): number => Math.max(0, Math.min(1, s));

/** 🦶️ Stance of the intact leg at stance fraction `s`: thigh and shank offsets, ankle and toe offsets, from the planted ball of the foot. */
function leftStance(s: number, y: number): { thigh: number; knee: number; foot: number; toe: number } {
  const bx = LEFT.a - CYCLE.speed * s * (CYCLE.leftOff - CYCLE.leftOn);
  let pitch = s < 0.15 ? -33 + (33 - (90 - FOOT_REST)) * ease(s / 0.15) : s < 0.45 ? -(90 - FOOT_REST) : -(90 - FOOT_REST) - (85 - (90 - FOOT_REST)) * clamp01((s - 0.45) / 0.55) ** 1.6;
  let ax = 0;
  let ay = 0;
  for (let i = 0; i < 60; i++) {
    ax = bx - FOOT_LEN * Math.cos(pitch * RAD);
    ay = -y - FOOT_LEN * Math.sin(pitch * RAD);
    if (Math.hypot(ax, ay) <= RIG.thigh + RIG.shank - 0.004) break;
    pitch -= 1;
  }
  const [a1, a2] = twoLink(ax, ay, RIG.thigh, RIG.shank);
  return { thigh: wrap(a1 + 90), knee: wrap(a1 - a2), foot: wrap(pitch - a2 - FOOT_REST), toe: wrap(0 - pitch - TOE_REST) };
}

/** 🌀️ Stance of the blade leg at stance fraction `s`: thigh, knee and flex of the forefoot lever from the planted rubber toe. */
function bladeStance(s: number, y: number): { thigh: number; knee: number; flex: number } {
  const flex = BLADE.flexDeg * Math.sin(Math.PI * clamp01(s)) ** 1.1;
  const [len, delta] = bladeLink(flex);
  const tx = BLADE.a - CYCLE.speed * s * (CYCLE.bladeOff - CYCLE.bladeOn);
  const [a1, a2] = twoLink(tx, -y, RIG.thigh, len);
  return { thigh: wrap(a1 + 90), knee: wrap(a1 - (a2 - delta)), flex };
}

const F = 1 / 30;
const L_ON = leftStance(0, LEFT.yOn);
const L_OFF = leftStance(1, LEFT.yOff);
const B_ON = bladeStance(0, BLADE.yOn);
const B_OFF = bladeStance(1, BLADE.yOff);
const L_THROUGH = leftStance(1 + F / (CYCLE.leftOff - CYCLE.leftOn), hipY(CYCLE.leftOff + F) - 0.09);
const L_REACH = leftStance(-F / (CYCLE.leftOff - CYCLE.leftOn), hipY(CYCLE.duration - F) - 0.045);
const B_THROUGH = bladeStance(1 + F / (CYCLE.bladeOff - CYCLE.bladeOn), hipY(CYCLE.bladeOff + F) - 0.085);
const B_REACH = bladeStance(-F / (CYCLE.bladeOff - CYCLE.bladeOn), hipY(CYCLE.bladeOn - F) - 0.045);
export const KEYED = { L_ON, L_OFF, L_THROUGH, L_REACH, B_ON, B_OFF, B_THROUGH, B_REACH };
const LEFT_SWING: Key[] = [[CYCLE.leftOff, L_OFF.thigh, L_OFF.knee], [CYCLE.leftOff + F, L_THROUGH.thigh, L_THROUGH.knee], [0.32, -14, 66], [0.37, -3, 92], [0.42, 10, 102], [0.48, 25, 96], [0.54, 38, 78], [0.6, 45, 54], [0.7 - F, L_REACH.thigh, L_REACH.knee], [0.7, L_ON.thigh, L_ON.knee]];
const BLADE_SWING: Key[] = [[CYCLE.bladeOff, B_OFF.thigh, B_OFF.knee], [CYCLE.bladeOff + F, B_THROUGH.thigh, B_THROUGH.knee], [0.655, -12, 66], [0.705, -2, 92], [0.76, 11, 102], [0.82, 26, 97], [0.88, 39, 80], [0.94, 46, 58], [1.0, 46, 42], [CYCLE.bladeOn + 0.7 - F, B_REACH.thigh, B_REACH.knee], [CYCLE.bladeOn + 0.7, B_ON.thigh, B_ON.knee]];
const FOOT_SWING: number[][] = [[CYCLE.leftOff, L_OFF.foot, L_OFF.toe], [CYCLE.leftOff + F, L_THROUGH.foot, Math.min(L_THROUGH.toe, 25)], [0.33, -8, 0], [0.45, 5, 0], [0.6, 5, 0], [0.7 - F, L_REACH.foot, L_REACH.toe], [0.7, L_ON.foot, L_ON.toe]];

/** 🏃️ Bone offsets (degrees from rest) and the root height offset of the stride at time `t`. */
export function stride(t: number): { bones: Record<string, Json>; rootY: number; contacts: string[] } {
  const T = CYCLE.duration;
  t = ((t % T) + T) % T;
  const y = hipY(t);
  const open = (keys: readonly number[][], x: number): number[] => {
    let i = 0;
    while (i < keys.length - 2 && keys[i + 1]![0]! < x) i++;
    const [a, b] = [keys[i]!, keys[i + 1]!];
    const s = (x - a[0]!) / (b[0]! - a[0]!);
    const time = (k: number): number => keys[Math.max(0, Math.min(keys.length - 1, k))]![0]!;
    return a.slice(1).map((_, j) => {
      const p = (k: number): number => keys[Math.max(0, Math.min(keys.length - 1, k))]![j + 1]!;
      const m1 = i === 0 ? p(1) - p(0) : ((p(i + 1) - p(i - 1)) / (time(i + 1) - time(i - 1))) * (b[0]! - a[0]!);
      const m2 = i === keys.length - 2 ? p(i + 1) - p(i) : ((p(i + 2) - p(i)) / (time(i + 2) - time(i))) * (b[0]! - a[0]!);
      return (2 * s ** 3 - 3 * s ** 2 + 1) * p(i) + (s ** 3 - 2 * s ** 2 + s) * m1 + (-2 * s ** 3 + 3 * s ** 2) * p(i + 1) + (s ** 3 - s ** 2) * m2;
    });
  };
  const bones: Record<string, Json> = {};
  const contacts: string[] = [];
  if (t <= CYCLE.leftOff + 1e-9) {
    const q = leftStance(t / CYCLE.leftOff, y);
    Object.assign(bones, { "thigh-l": { angleDeg: q.thigh }, "shank-l": { angleDeg: -q.knee }, "foot-l": { angleDeg: q.foot }, "toe-l": { angleDeg: q.toe } });
    contacts.push("ball-l");
  } else {
    const [th, kn] = open(LEFT_SWING, t);
    const [ft, toe] = open(FOOT_SWING, t);
    Object.assign(bones, { "thigh-l": { angleDeg: th }, "shank-l": { angleDeg: -kn! }, "foot-l": { angleDeg: ft }, "toe-l": { angleDeg: Math.max(0, toe!) } });
  }
  if (t >= CYCLE.bladeOn - 1e-9 && t <= CYCLE.bladeOff + 1e-9) {
    const q = bladeStance((t - CYCLE.bladeOn) / (CYCLE.bladeOff - CYCLE.bladeOn), y);
    Object.assign(bones, { "thigh-r": { angleDeg: q.thigh }, "shank-r": { angleDeg: -q.knee }, "blade-r": { angleDeg: 0 }, "blade-tip-r": { angleDeg: q.flex } });
    contacts.push("blade-toe");
  } else {
    const [th, kn] = open(BLADE_SWING, t < CYCLE.bladeOn ? t + T : t);
    Object.assign(bones, { "thigh-r": { angleDeg: th }, "shank-r": { angleDeg: -kn! }, "blade-r": { angleDeg: 0 }, "blade-tip-r": { angleDeg: 0 } });
  }
  const swing = (phase: number): number => Math.cos((2 * Math.PI * (t - phase)) / T);
  const [sl, sr] = [swing(0.27), swing(0.62)];
  Object.assign(bones, {
    "arm-l": { angleDeg: -9 + 33 * sl, abductionDeg: 9 },
    "forearm-l": { angleDeg: 88 + 9 * sl, abductionDeg: -22 },
    "hand-l": { angleDeg: 6 },
    "arm-r": { angleDeg: -9 + 33 * sr, abductionDeg: -9 },
    "forearm-r": { angleDeg: 88 + 9 * sr, abductionDeg: 22 },
    "hand-r": { angleDeg: 6 },
    torso: { angleDeg: -7 + 1.2 * Math.cos((4 * Math.PI * (t - 0.1)) / T) },
    neck: { angleDeg: 4 },
    head: { angleDeg: 1 },
  });
  for (const b of Object.values(bones)) for (const k of Object.keys(b)) b[k] = r4(b[k]);
  return { bones, rootY: r5(y - RIG.hipY), contacts };
}

void spline;

const STATIC: Record<string, Json> = {
  "standing-ready": {
    description: { en: "Upright before the start with relaxed arms: both legs are straight, the weight is shared by the intact foot and the blade, and the rubber toe of the blade stands level with the ball of the foot.", de: "Aufrecht vor dem Start mit locker hängenden Armen: Beide Beine sind gestreckt, das Gewicht verteilt sich auf den gesunden Fuß und die Feder, und die Gummisohle der Feder steht auf Höhe des Fußballens." },
    bones: { torso: { angleDeg: -1.5 }, neck: { angleDeg: 1.5 }, head: { angleDeg: 0 }, "thigh-l": { angleDeg: 0 }, "shank-l": { angleDeg: 0 }, "foot-l": { angleDeg: 0 }, "toe-l": { angleDeg: 0 }, "thigh-r": { angleDeg: 0 }, "shank-r": { angleDeg: 0 }, "blade-r": { angleDeg: 0 }, "blade-tip-r": { angleDeg: 0 }, "arm-l": { angleDeg: 2, abductionDeg: 7 }, "forearm-l": { angleDeg: 14, abductionDeg: -5 }, "hand-l": { angleDeg: 6 }, "arm-r": { angleDeg: -2, abductionDeg: -7 }, "forearm-r": { angleDeg: 16, abductionDeg: 5 }, "hand-r": { angleDeg: 6 } },
  },
  "hands-on-hips": {
    bones: { torso: { angleDeg: 2 }, neck: { angleDeg: 4 }, head: { angleDeg: 12 }, "thigh-l": { angleDeg: 0 }, "shank-l": { angleDeg: 0 }, "foot-l": { angleDeg: 0 }, "toe-l": { angleDeg: 0 }, "thigh-r": { angleDeg: 0 }, "shank-r": { angleDeg: 0 }, "blade-r": { angleDeg: 0 }, "blade-tip-r": { angleDeg: 0 }, "arm-l": { angleDeg: -24, abductionDeg: 34 }, "forearm-l": { angleDeg: 96, abductionDeg: -62 }, "hand-l": { angleDeg: 8 }, "arm-r": { angleDeg: -24, abductionDeg: -34 }, "forearm-r": { angleDeg: 96, abductionDeg: 62 }, "hand-r": { angleDeg: 8 } },
  },
};
const FROM_STRIDE: Record<string, number> = { "left-forefoot-strike": 0, "blade-midstance": 0.4667, "blade-push-off": 0.5667, flight: 0.3 };
export const DEFAULT_POSE = "blade-push-off";
const BONE_ORDER = ["torso", "neck", "head", "thigh-l", "shank-l", "foot-l", "toe-l", "thigh-r", "shank-r", "blade-r", "blade-tip-r", "arm-l", "forearm-l", "hand-l", "arm-r", "forearm-r", "hand-r"];
const ordered = (bones: Record<string, Json>): Record<string, Json> => Object.fromEntries(BONE_ORDER.map((id) => [id, bones[id]]));

/** 🎞️ Re-authors every pose and the running clip on the corrected rig; thermal bindings and loads of the existing poses are kept. */
export function fixPoses(asset: Json): void {
  const old = new Map<string, Json>(asset.poses.map((p: Json) => [p.id, p]));
  const poses: Json[] = [];
  for (const id of ["standing-ready", "hands-on-hips"]) {
    const p = old.get(id)!;
    poses.push({ ...p, ...(STATIC[id].description ? { description: STATIC[id].description } : {}), bones: STATIC[id].bones, ground: "auto", contacts: ["heel-l", "ball-l", "blade-toe"] });
  }
  const fromStride = (id: string, t: number, title: Json, description: Json, template: Json): Json => {
    const s = stride(t);
    const { root: _root, key: _key, ...rest } = template;
    return { ...rest, id, title, description, bones: ordered(s.bones), root: { y: s.rootY }, ground: "authored", contacts: s.contacts };
  };
  poses.push(fromStride("left-forefoot-strike", 0, old.get("left-forefoot-strike")!.title, { en: "The intact foot lands on its forefoot 0.29 m ahead of the hip with the knee flexed 28 degrees; the blade leg is folded behind in its swing.", de: "Der gesunde Fuß setzt 0,29 m vor der Hüfte auf dem Vorfuß auf, das Knie ist 28 Grad gebeugt; das Federbein schwingt angewinkelt hinten durch." }, old.get("left-forefoot-strike")!));
  poses.push(fromStride("blade-midstance", 14 / 30, old.get("blade-midstance")!.title, { en: "The rubber toe of the blade is planted under the hip and the forefoot lever is flexed by about 11 degrees; the intact leg swings through with the knee folded.", de: "Die Gummisohle der Feder steht unter der Hüfte und der Vorfußhebel ist um etwa 11 Grad eingefedert; das gesunde Bein schwingt mit gebeugtem Knie durch." }, old.get("blade-midstance")!));
  poses.push(fromStride("blade-push-off", 17 / 30, { en: "Blade push-off", de: "Federabstoß" }, { en: "The last instant of the blade contact: the hip is 0.27 m ahead of the planted rubber sole, the leg is almost straight, the blade recoils and the intact knee drives forward and up while the arms counter-swing.", de: "Der letzte Augenblick des Federkontakts: Die Hüfte ist 0,27 m über die stehende Gummisohle hinaus, das Bein ist fast gestreckt, die Feder entspannt sich und das gesunde Knie schwingt nach vorn oben, die Arme schwingen gegengleich." }, old.get("blade-midstance")!));
  poses.push(fromStride("flight", 0.3, old.get("flight")!.title, { en: "Both feet are off the ground after the push-off of the intact foot; the blade leg reaches forward for its landing.", de: "Beide Füße sind nach dem Abdruck des gesunden Fußes in der Luft; das Federbein greift nach vorn zur Landung." }, old.get("flight")!));
  const keys: Json[] = [];
  for (let k = 0; k <= 20; k++) {
    const id = `run-${String(k).padStart(2, "0")}`;
    const t = k / 30;
    const s = stride(t);
    const p = old.get(id)!;
    const thermal = JSON.parse(JSON.stringify(p.thermal));
    for (const [region, anchor] of [["running-shoe", "ball-l"], ["blade", "blade-toe"]] as const) {
      const r = thermal.regions[region];
      if (r === undefined) continue;
      if (s.contacts.includes(anchor)) r.contact = { fraction: region === "blade" ? 0.05 : 0.3, tempC: 24 };
      else delete r.contact;
    }
    poses.push({ ...p, bones: ordered(s.bones), root: { y: s.rootY }, ground: "authored", contacts: s.contacts, thermal });
    keys.push({ t: r5(t), pose: id, ease: "smooth" });
  }
  keys.push({ t: 0.7, pose: "run-00", ease: "smooth" });
  for (const id of Object.keys(FROM_STRIDE)) {
    const p = poses.find((x) => x.id === id)!;
    for (const [region, anchor] of [["running-shoe", "ball-l"], ["blade", "blade-toe"]] as const) {
      const r = p.thermal.regions[region];
      if (r === undefined) continue;
      p.thermal = JSON.parse(JSON.stringify(p.thermal));
      const q = p.thermal.regions[region];
      if (p.contacts.includes(anchor)) q.contact = { fraction: region === "blade" ? 0.05 : 0.3, tempC: 24 };
      else delete q.contact;
    }
  }
  asset.poses = poses;
  const clip = asset.clips.find((c: Json) => c.id === "run-cycle");
  clip.description = {
    en: "One stride in 0.7 s (171 steps per minute, 2.17 m): the blade side has a shorter ground contact (0.17 s) than the intact side (0.23 s) and a longer flight before it (0.17 s against 0.13 s), the forefoot lever of the blade flexes about 11 degrees under load, the arms swing with the elbows at about 90 degrees.",
    de: "Ein Doppelschritt in 0,7 s (171 Schritte pro Minute, 2,17 m): Die Federseite hat einen kürzeren Bodenkontakt (0,17 s) als die gesunde Seite (0,23 s) und davor eine längere Flugzeit (0,17 s gegen 0,13 s), der Vorfußhebel der Feder federt unter Last um etwa 11 Grad ein, die Arme schwingen mit etwa 90 Grad Ellbogenwinkel.",
  };
  clip.keys = keys;
  clip.advanceM = CYCLE.advance;
  const [lo, bo, bf] = [r5(CYCLE.leftOff), CYCLE.bladeOn, CYCLE.bladeOff];
  clip.contacts = [{ anchor: "ball-l", fromS: 0, toS: lo }, { anchor: "blade-toe", fromS: bo, toS: bf }];
  const phase = (id: string): Json => clip.phases.find((p: Json) => p.id === id);
  Object.assign(phase("left-stance"), { fromS: 0, toS: lo });
  Object.assign(phase("flight-one"), { fromS: lo, toS: bo });
  Object.assign(phase("blade-stance"), { fromS: bo, toS: bf });
  Object.assign(phase("flight-two"), { fromS: bf, toS: 0.7 });
  clip.events.forEach((e: Json, i: number) => (e.t = [0, lo, bo, bf][i]));
  asset.presentation.defaultPose = DEFAULT_POSE;
}
