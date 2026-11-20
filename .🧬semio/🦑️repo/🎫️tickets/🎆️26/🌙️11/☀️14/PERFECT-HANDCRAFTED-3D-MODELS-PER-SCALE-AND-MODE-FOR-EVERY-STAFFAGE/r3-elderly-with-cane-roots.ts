/** 🩼 Root fixes of elderly-with-cane outside `volumes`: the cane (a crook cane cut to the wrist crease of the stooped stance, rigid in the gripping hand), its tip anchor, and the right arm of every pose and clip key re-solved so that the tip stands on the ground while it is a contact.
 *
 * Usage: `bun r3-elderly-with-cane-roots.ts [--write]` (without `--write` it only prints what it would change).
 * Only the members `rig`, `anchors`, `poses` and `clips` of the asset are re-serialised (two-space JSON, as the file has them); every other byte of the file stays.
 * @see ./r3-humans-elderly-with-cane.md */
import { readFileSync, writeFileSync } from "node:fs";
import { rigAt, type Query } from "./r3-lib.ts";
import { entryOf, type Json } from "./volumes-lib.ts";

const WRITE = process.argv.includes("--write");
const { entry } = entryOf("humans/elderly-with-cane");
const FILE: string = (entry as Json).file ?? (entry as Json).path;
const text = readFileSync(FILE, "utf8");
const doc = JSON.parse(text) as Json;
const asset = doc.asset as Json;
const bone = (id: string): Json => asset.rig.bones.find((b: Json) => b.id === id);
const rad = (d: number): number => (d * Math.PI) / 180;
const deg = (r: number): number => (r * 180) / Math.PI;
const r4 = (x: number): number => Math.round(x * 1e4) / 1e4;
const wrap = (d: number): number => ((((d + 180) % 360) + 360) % 360) - 180;

const CANE_L = 0.734;
const CROOK_R = 0.045;
const HAND_SLOPE = -50;
const GRIP: [number, number] = [0.0538, -0.0059];
const CANE_Z = -0.04;
const ORIGIN: [number, number] = [r4(GRIP[0] + CROOK_R * (Math.cos(rad(HAND_SLOPE)) + Math.cos(rad(270 - HAND_SLOPE)))), r4(GRIP[1] + CROOK_R * (Math.cos(rad(90 + HAND_SLOPE)) + Math.cos(rad(270 - 90 - HAND_SLOPE))))];
const CANE_REL = 270 - HAND_SLOPE - 360;
const UPPER = bone("arm-r").lengthM as number;
const FORE = bone("forearm-r").lengthM as number;

const cane = bone("cane");
const before = { x: cane.x, y: cane.y, angleDeg: cane.angleDeg, lengthM: cane.lengthM, comX: cane.mass.comX, comY: cane.mass.comY };
cane.x = ORIGIN[0];
cane.y = ORIGIN[1];
cane.angleDeg = CANE_REL;
cane.lengthM = CANE_L;
cane.mass.comX = 0.29;
cane.mass.comY = -0.005;
cane.z = CANE_Z;
const tip = asset.anchors.find((a: Json) => a.id === "cane-tip");
const grip = asset.anchors.find((a: Json) => a.id === "cane-grip");
const anchorsBefore = { tipX: tip.x, gripX: grip.x, gripY: grip.y };
tip.x = CANE_L;
grip.x = GRIP[0];
grip.y = GRIP[1];
console.log("cane bone", before, "->", { x: cane.x, y: cane.y, angleDeg: cane.angleDeg, lengthM: cane.lengthM, comX: cane.mass.comX, comY: cane.mass.comY }, "anchors", anchorsBefore, "->", { tipX: tip.x, gripX: grip.x, gripY: grip.y });

type Arm = { arm: number; fore: number; hand: number; flexDeg: number; wristDeg: number; reach: number; tipX: number; tipY: number; tauDeg: number };
/** 💪 The right arm for the shoulder `s`, the trunk direction `torsoDeg`, the shaft top `o` and the cane tilt `tauDeg` (tip ahead of the handle is positive); the hand moves towards the shoulder line until the wrist is within reach. */
function solve(s: [number, number], torsoDeg: number, o: [number, number], tauDeg: number): Arm {
  const handDeg = HAND_SLOPE + tauDeg;
  const c = Math.cos(rad(handDeg));
  const n = Math.sin(rad(handDeg));
  const w: [number, number] = [o[0] - (ORIGIN[0] * c - ORIGIN[1] * n), o[1] - (ORIGIN[0] * n + ORIGIN[1] * c)];
  const d = Math.hypot(w[0] - s[0], w[1] - s[1]);
  if (d > (UPPER + FORE) * 0.9985) throw new Error(`wrist out of reach ${d.toFixed(4)}`);
  const interior = Math.acos((UPPER * UPPER + FORE * FORE - d * d) / (2 * UPPER * FORE));
  const flex = Math.PI - interior;
  const toWrist = Math.atan2(w[1] - s[1], w[0] - s[0]);
  const armWorld = toWrist - Math.atan2(FORE * Math.sin(flex), UPPER + FORE * Math.cos(flex));
  const foreWorld = armWorld + flex;
  if (Math.abs(wrap(handDeg - deg(foreWorld))) > 57) throw new Error("wrist beyond its range");
  const armRest = torsoDeg + (bone("arm-r").angleDeg as number);
  return { arm: r4(wrap(deg(armWorld) - armRest)), fore: r4(deg(flex)), hand: r4(wrap(handDeg - deg(foreWorld))), flexDeg: deg(flex), wristDeg: wrap(handDeg - deg(foreWorld)), reach: d, tipX: o[0] + CANE_L * Math.sin(rad(tauDeg)), tipY: o[1] - CANE_L * Math.cos(rad(tauDeg)), tauDeg };
}
const feet = JSON.parse(JSON.stringify(asset)) as Json;
for (const c of feet.clips as Json[]) c.contacts = c.contacts.filter((x: Json) => x.anchor !== "cane-tip");
for (const p of feet.poses as Json[]) p.contacts = p.contacts.filter((x: string) => x !== "cane-tip");
feet.thermal.respiration.motion = [];
/** 🦶 The right shoulder and the trunk direction with the body carried by the feet alone, so the planted cane tip cannot steer the root it is solved against. */
function shoulder(q: Query): { s: [number, number]; torsoDeg: number } {
  const rig = rigAt(feet, 50, q);
  const f = rig.frames["arm-r"]!;
  const t = rig.frames.torso!;
  return { s: [f.o[0], f.o[1]], torsoDeg: deg(Math.atan2(t.u[1], t.u[0])) };
}
/** 📍 A planted cane: the tip stands at (`tipX`, 0) and the shaft top is `ahead` of the shoulder; when the wrist is out of reach the hand comes closer to the shoulder line. */
function planted(q: Query, tipX: number, ahead: number): Arm {
  const { s, torsoDeg } = shoulder(q);
  for (let k = 0; k < 400; k++) {
    const ox = s[0] + ahead + (tipX - s[0] - ahead) * (k / 400);
    const tau = deg(Math.asin((tipX - ox) / CANE_L));
    try {
      return solve(s, torsoDeg, [ox, CANE_L * Math.cos(rad(tau))], tau);
    } catch {
      continue;
    }
  }
  throw new Error("no planted solution");
}
function apply(bones: Json, a: Arm): void {
  bones["arm-r"] = { angleDeg: a.arm };
  bones["forearm-r"] = { angleDeg: a.fore };
  bones["hand-r"] = { angleDeg: a.hand };
  bones.cane = { angleDeg: 0 };
}
const show = (label: string, a: Arm): void => console.log(`${label.padEnd(28)} arm=${a.arm.toFixed(2)} elbow=${a.flexDeg.toFixed(1)} wrist=${a.wristDeg.toFixed(1)} tilt=${a.tauDeg.toFixed(1)} tip=(${a.tipX.toFixed(4)},${a.tipY.toFixed(4)}) reach=${a.reach.toFixed(3)}`);

/** 🖐️ The far arm hangs plumb from the shoulder whatever the lean of the trunk, the elbow a little bent, the hand beside the thigh. */
function hang(bones: Json): void {
  bones["arm-l"] = { angleDeg: r4(-3 - (bones.torso?.angleDeg ?? 0)) };
  bones["forearm-l"] = { angleDeg: 12 };
  bones["hand-l"] = { angleDeg: 6 };
}
const pose = (id: string): Json => asset.poses.find((p: Json) => p.id === id);
const STAND_TIP = 0.34;
const STAND_AHEAD = 0.168;
for (const id of ["stooped-standing", "cane-planted", "resting-on-cane"]) {
  const p = pose(id);
  hang(p.bones);
  if (id === "resting-on-cane") p.description = { en: "Standing still and more upright, the near hand resting on the crook handle, the far arm hanging loosely, head up.", de: "Ruhiger, aufrechterer Stand, die nahe Hand ruht auf dem Krückgriff, der ferne Arm hängt locker, der Kopf ist gehoben." };
  const a = planted({ kind: "pose", pose: id }, STAND_TIP, STAND_AHEAD);
  apply(p.bones, a);
  show(`pose ${id}`, a);
}
pose("stooped-standing").description = { en: "Standing on both feet with a rounded upper back, the cane planted a hand's breadth ahead of the toes beside the foot and lightly loaded.", de: "Stand auf beiden Füßen mit rundem Rücken, der Stock eine Handbreit vor den Zehen neben dem Fuß aufgesetzt und leicht belastet." };
pose("cane-planted").description = { en: "The trunk leans over the planted cane and the hand comes above the tip before the weak foot follows.", de: "Der Rumpf lehnt sich über den aufgesetzten Stock und die Hand kommt über die Spitze, bevor der schwache Fuß folgt." };

const rest = asset.clips.find((c: Json) => c.id === "rest-on-cane");
for (const key of rest.keys as Json[]) {
  if (key.bones === undefined) continue;
  hang(key.bones);
  apply(key.bones, planted({ kind: "clip", clip: "rest-on-cane", timeS: key.t }, STAND_TIP, STAND_AHEAD));
}
show("rest-on-cane t=1.6", planted({ kind: "clip", clip: "rest-on-cane", timeS: 1.6 }, STAND_TIP, STAND_AHEAD));

const gait = asset.clips.find((c: Json) => c.id === "cane-gait");
const LIFT_T = 1.088;
const AHEAD0 = 0.22;
const AHEAD1 = -0.06;
const TILT0 = 10;
const LIFT_M = 0.045;
const sx = (t: number): number => shoulder({ kind: "clip", clip: "cane-gait", timeS: t }).s[0];
const s0 = sx(0);
const s1 = sx(LIFT_T);
const PLANT_X = s0 + AHEAD0 + CANE_L * Math.sin(rad(TILT0));
const TAU0 = deg(Math.asin((PLANT_X - s0 - AHEAD0) / CANE_L));
const TAU1 = deg(Math.asin((PLANT_X - s1 - AHEAD1) / CANE_L));
console.log(`gait: shoulder x ${s0.toFixed(3)} -> ${s1.toFixed(3)}, tip planted at x=${PLANT_X.toFixed(4)}, tilt ${TAU0.toFixed(1)} -> ${TAU1.toFixed(1)}`);
/** 🚶 The right arm at the time `t` of the gait cycle: planted until the cane lifts, then swung to the next plant with the tip clear of the ground. */
function gaitAt(q: Query, t: number): Arm {
  if (t <= LIFT_T) return planted(q, PLANT_X + (q.kind === "pose" ? shoulder(q).s[0] - sx(t) : 0), AHEAD0 + (AHEAD1 - AHEAD0) * (t / LIFT_T));
  const u = (t - LIFT_T) / (gait.durationS - LIFT_T);
  const e = u * u * (3 - 2 * u);
  const { s, torsoDeg } = shoulder(q);
  const lift = LIFT_M * Math.sin(Math.PI * u) ** 2;
  const tipX = PLANT_X + (gait.advanceM as number) * e + (q.kind === "pose" ? s[0] - sx(t) : 0);
  for (let k = 0; k < 400; k++) {
    const ox = s[0] + AHEAD1 + (AHEAD0 - AHEAD1) * e;
    const reachX = ox + (tipX - ox) * (k / 400);
    const tau = deg(Math.asin((tipX - reachX) / CANE_L));
    try {
      return solve(s, torsoDeg, [reachX, lift + CANE_L * Math.cos(rad(tau))], tau);
    } catch {
      continue;
    }
  }
  throw new Error("no swing solution");
}
for (const key of gait.keys as Json[]) {
  const a = gaitAt({ kind: "clip", clip: "cane-gait", timeS: key.t }, key.t);
  apply(key.bones, a);
  if (Math.round(key.t * 60) % 5 === 0) show(`cane-gait t=${key.t}`, a);
}
const LEGS = ["thigh-l", "shank-l", "thigh-r", "shank-r"];
for (const id of ["contact-left", "loading-left", "passing-left", "contact-right", "loading-right", "passing-right"]) {
  const p = pose(id);
  let best = { t: 0, err: Infinity };
  for (const key of gait.keys as Json[]) {
    const err = LEGS.reduce((sum, b) => sum + (key.bones[b].angleDeg - p.bones[b].angleDeg) ** 2, 0);
    if (err < best.err) best = { t: key.t, err };
  }
  const a = gaitAt({ kind: "pose", pose: id }, best.t);
  apply(p.bones, a);
  show(`pose ${id} (t=${best.t})`, a);
}
pose("contact-left").description = { en: "Cane tip and left heel land together, the tip a hand's breadth ahead of the left toes.", de: "Stockspitze und linke Ferse setzen gemeinsam auf, die Spitze eine Handbreit vor den linken Zehen." };

function memberRange(source: string, name: string): [number, number] {
  const start = source.indexOf(`\n    "${name}": `);
  if (start < 0) throw new Error(`member ${name} not found`);
  const open = start + `\n    "${name}": `.length;
  let depth = 0;
  let quoted = false;
  for (let i = open; i < source.length; i++) {
    const ch = source[i]!;
    if (quoted) {
      if (ch === "\\") i++;
      else if (ch === '"') quoted = false;
    } else if (ch === '"') quoted = true;
    else if (ch === "{" || ch === "[") depth++;
    else if (ch === "}" || ch === "]") {
      depth--;
      if (depth === 0) return [open, i + 1];
    }
  }
  throw new Error("unbalanced");
}
const dump = (value: Json): string => JSON.stringify(value, null, 2).replace(/\n/g, "\n    ");
const still = JSON.parse(JSON.stringify(asset)) as Json;
still.thermal.respiration.motion = [];
const anchorX = (a: Json, t: number, id: string): number => {
  const rig = rigAt(a, 50, { kind: "clip", clip: "cane-gait", timeS: t });
  const an = a.anchors.find((x: Json) => x.id === id);
  const f = rig.frames[an.bone]!;
  return f.o[0] + f.u[0] * an.x + f.v[0] * an.y;
};
const anchorY = (a: Json, t: number, id: string): number => {
  const rig = rigAt(a, 50, { kind: "clip", clip: "cane-gait", timeS: t });
  const an = a.anchors.find((x: Json) => x.id === id);
  const f = rig.frames[an.bone]!;
  return f.o[1] + f.u[1] * an.x + f.v[1] * an.y;
};
for (const [label, a] of [["without breathing", still], ["with breathing, cycle 1", asset]] as const) {
  let lo = Infinity;
  let hi = -Infinity;
  let top = -Infinity;
  let ballLo = Infinity;
  let ballHi = -Infinity;
  for (let t = 0; t <= LIFT_T; t += 0.004) {
    const x = anchorX(a, t, "cane-tip");
    lo = Math.min(lo, x);
    hi = Math.max(hi, x);
    top = Math.max(top, Math.abs(anchorY(a, t, "cane-tip")));
    if (t >= 0.26) {
      const bx = anchorX(a, t, "ball-l");
      ballLo = Math.min(ballLo, bx);
      ballHi = Math.max(ballHi, bx);
    }
  }
  console.log(`gait ${label}: planted tip x ${lo.toFixed(4)}..${hi.toFixed(4)} (slide ${((hi - lo) * 1000).toFixed(1)} mm), |y| <= ${(top * 1000).toFixed(1)} mm, left ball slide ${((ballHi - ballLo) * 1000).toFixed(1)} mm`);
}
for (const p of asset.poses as Json[]) {
  const b = rigAt(asset, 50, { kind: "pose", pose: p.id }).bones.find((x) => x.id === "cane")!;
  console.log(`check pose ${p.id} tip=(${b.tip[0].toFixed(4)},${b.tip[1].toFixed(4)}) contacts=${p.contacts.join(",")}`);
}
for (const t of [0, 0.8, 1.6, 3, 4, 4.8, 6]) {
  const b = rigAt(still, 50, { kind: "clip", clip: "rest-on-cane", timeS: Math.min(t, 5.999) }).bones.find((x) => x.id === "cane")!;
  console.log(`check rest-on-cane t=${t} tip=(${b.tip[0].toFixed(4)},${b.tip[1].toFixed(4)})`);
}
let out = readFileSync(FILE, "utf8");
const original = JSON.parse(out).asset as Json;
for (const name of ["rig", "anchors", "poses", "clips"]) {
  const [a, b] = memberRange(out, name);
  if (out.slice(a, b) !== dump(original[name])) throw new Error(`member ${name} is not in the expected two-space style`);
  out = out.slice(0, a) + dump(asset[name]) + out.slice(b);
}
const check = JSON.parse(out).asset as Json;
for (const k of Object.keys(original)) if (!["rig", "anchors", "poses", "clips"].includes(k) && JSON.stringify(check[k]) !== JSON.stringify(original[k])) throw new Error(`member ${k} changed`);
if (Object.keys(check).join() !== Object.keys(original).join()) throw new Error("member order changed");
if (WRITE) writeFileSync(FILE, out);
console.log(WRITE ? `written ${FILE}` : "dry run, nothing written");
