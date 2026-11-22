/** 🦴 Rig rework of `humans/elderly-with-cane`: lumbar and thoracic spine as two bones along the vertebral bodies with the kyphosis in the rest angles, clavicles from the sternoclavicular joint to the acromion, an atlanto-occipital head pivot, fifteen finger bones per hand, the wrist extended over a derby handle, the cane cut to the wrist crease; anchors, standing poses (slight hip and knee flexion) and the arm of every pose and clip key re-solved.
 *
 * Usage: `bun rework-04-elderly-with-cane-rig.ts` writes `🗑️generated/rework-04/elderly-with-cane/rig.json` (the asset with the new rig, anchors, poses and clips; volumes and drawings still the old ones).
 * @see ./rework/04.md */
import { join } from "node:path";
import { rigAt, type Query } from "./r3-lib.ts";
import type { Json } from "./volumes-lib.ts";
import { deg, openCatalog, rad, workDir, writeJsonFile } from "./rework-04-kit.ts";

const { asset } = openCatalog("humans/elderly-with-cane");
const r4 = (x: number): number => Math.round(x * 1e4) / 1e4;
const wrap = (d: number): number => ((((d + 180) % 360) + 360) % 360) - 180;
const old = (id: string): Json => asset.rig.bones.find((b: Json) => b.id === id);
const text = (en: string, de: string): Json => ({ en, de });
const rev = (minDeg: number, maxDeg: number): Json => ({ type: "revolute", minDeg, maxDeg });
const mass = (kg: number, comX: number, comY: number, source = "literature", reference = "de-leva-1996"): Json => ({ kg, comX, comY, distribution: "point", source, reference });

/* ------------------------------------------------------------------ spine and girdle, in figure coordinates relative to the hip axis (x ahead, y up) */
const LS: [number, number] = [-0.035, 0.091];
const LUMBAR = { deg: 84, len: 0.165 };
const THORAX = { deg: 70, len: 0.27 };
const NECK = { deg: 57, len: 0.105 };
const HEAD = { deg: 84, len: 0.14 };
const along = (p: [number, number], d: number, l: number): [number, number] => [p[0] + l * Math.cos(rad(d)), p[1] + l * Math.sin(rad(d))];
const T12 = along(LS, LUMBAR.deg, LUMBAR.len);
/** 📌 A point of the figure (relative to the hip axis) in the thorax frame: [along, normal]. */
const inThorax = (p: [number, number]): [number, number] => {
  const [dx, dy] = [p[0] - T12[0], p[1] - T12[1]];
  const [c, s] = [Math.cos(rad(THORAX.deg)), Math.sin(rad(THORAX.deg))];
  return [r4(dx * c + dy * s), r4(-dx * s + dy * c)];
};
const HIP_Y = 0.884;
const GH: [number, number] = [0.09, 1.35 - HIP_Y];
const SC: [number, number, number] = [0.146, 1.34 - HIP_Y, 0.025];
const AC: [number, number, number] = [0.096, 1.38 - HIP_Y, 0.17];
const sc = inThorax([SC[0], SC[1]]);
const ac = inThorax([AC[0], AC[1]]);
const cl = [ac[0] - sc[0], ac[1] - sc[1], AC[2] - SC[2]];
const CL_LEN = r4(Math.hypot(...cl));
const CL_ANGLE = r4(deg(Math.atan2(cl[1]!, cl[0]!)));
const CL_SPREAD = r4(deg(Math.asin(cl[2]! / CL_LEN)));
const gh = inThorax(GH);

/* ------------------------------------------------------------------ hands */
/** ✋ Joint-to-joint lengths of a male hand of 0.177 m (wrist crease to the middle fingertip), metres: the metacarpophalangeal joint in the hand frame [along, across towards the thumb], then proximal, middle and distal phalanx. */
const FINGERS: Record<string, { at: [number, number]; lens: [number, number, number]; fan: number }> = {
  index: { at: [0.086, 0.03], lens: [0.04, 0.023, 0.018], fan: 4 },
  middle: { at: [0.088, 0.009], lens: [0.044, 0.026, 0.019], fan: 0 },
  ring: { at: [0.083, -0.011], lens: [0.041, 0.025, 0.018], fan: -3 },
  little: { at: [0.074, -0.03], lens: [0.032, 0.018, 0.016], fan: -8 },
};
const SEGMENTS = ["proximal", "middle", "distal"] as const;
const SEG_TITLE: Record<string, [string, string]> = { proximal: ["proximal phalanx", "Grundglied"], middle: ["middle phalanx", "Mittelglied"], distal: ["distal phalanx", "Endglied"], metacarpal: ["metacarpal", "Mittelhandbein"] };
const FINGER_TITLE: Record<string, [string, string]> = { thumb: ["thumb", "Daumen"], index: ["index finger", "Zeigefinger"], middle: ["middle finger", "Mittelfinger"], ring: ["ring finger", "Ringfinger"], little: ["little finger", "Kleinfinger"] };
/** 🤏 The curl of the three finger joints towards the palm, degrees: the relaxed hanging hand and the hand closed round the 26 mm handle. */
const CURL = { relaxed: { index: [16, 24, 12], middle: [20, 28, 14], ring: [24, 32, 16], little: [28, 34, 18] }, grip: { index: [76, 92, 20], middle: [80, 95, 20], ring: [82, 96, 22], little: [84, 98, 24] } } as const;
const THUMB = { relaxed: { angle: [36, -6, -4], curl: [22, 8, 10] }, grip: { angle: [66, -12, -12], curl: [18, 24, 28] } } as const;
const WRIST_EXTENSION = 50;

function hand(side: "l" | "r", state: "relaxed" | "grip"): Json[] {
  const k = side === "l" ? 1 : -1;
  const [enF, deF] = side === "l" ? ["Left", "Linkes"] : ["Right", "Rechtes"];
  const out: Json[] = [];
  const t = THUMB[state];
  const thumbLens = [0.042, 0.031, 0.025];
  ["metacarpal", "proximal", "distal"].forEach((seg, i) => {
    out.push({ id: `thumb-${seg}-${side}`, parent: i === 0 ? `hand-${side}` : `thumb-${["metacarpal", "proximal"][i - 1]}-${side}`, title: text(`${enF} thumb, ${SEG_TITLE[seg]![0]}`, `${deF} ${SEG_TITLE[seg]![1]} des Daumens`), x: i === 0 ? 0.022 : thumbLens[i - 1], y: i === 0 ? 0.03 : 0, z: i === 0 ? -0.008 * k : 0, angleDeg: t.angle[i], abductionDeg: -k * t.curl[i]!, lengthM: thumbLens[i], joint: rev(-25, 25) });
  });
  for (const [finger, f] of Object.entries(FINGERS)) {
    const curl = CURL[state][finger as keyof typeof CURL.relaxed];
    SEGMENTS.forEach((seg, i) => {
      out.push({ id: `${finger}-${seg}-${side}`, parent: i === 0 ? `hand-${side}` : `${finger}-${SEGMENTS[i - 1]}-${side}`, title: text(`${enF} ${FINGER_TITLE[finger]![0]}, ${SEG_TITLE[seg]![0]}`, `${deF} ${SEG_TITLE[seg]![1]} des ${FINGER_TITLE[finger]![1]}s`), x: i === 0 ? f.at[0] : f.lens[i - 1], y: i === 0 ? f.at[1] : 0, z: 0, angleDeg: i === 0 ? f.fan : 0, abductionDeg: -k * curl[i]!, lengthM: f.lens[i], joint: rev(-20, 20) });
    });
  }
  return out;
}

/* ------------------------------------------------------------------ cane: derby handle under the palm, shaft ahead of the index finger */
const CANE_L = 0.772;
/** 🩼 The centre of the handle under the palm in the hand frame (along, across, palmar) and the shaft top the same handle carries ahead of the index finger. */
const HANDLE = { along: 0.066, palmar: 0.027, shaftAcross: 0.045 };
const cw = Math.cos(rad(WRIST_EXTENSION));
const sw = Math.sin(rad(WRIST_EXTENSION));
/** 📐 The shaft top relative to the wrist in the sagittal frame of the hand before the wrist extension: [down the forearm line, ahead]. */
const ORIGIN: [number, number] = [r4(HANDLE.along * cw + HANDLE.palmar * sw), HANDLE.shaftAcross];

const arm = (side: "l" | "r"): Json[] => {
  const k = side === "l" ? 1 : -1;
  const a = old(`arm-${side}`);
  const f = old(`forearm-${side}`);
  const h = old(`hand-${side}`);
  return [
    { id: `clavicle-${side}`, parent: "chest", title: side === "l" ? text("Left clavicle and shoulder blade", "Linkes Schlüsselbein mit Schulterblatt") : text("Right clavicle and shoulder blade", "Rechtes Schlüsselbein mit Schulterblatt"), x: sc[0], y: sc[1], z: k * SC[2], angleDeg: CL_ANGLE, abductionDeg: k * CL_SPREAD, lengthM: CL_LEN, joint: rev(-10, 25) },
    { ...a, parent: "chest", x: gh[0], y: gh[1], z: k * 0.18, angleDeg: -90 - THORAX.deg },
    f,
    side === "r" ? { ...h, lengthM: 0.09, mass: { ...h.mass, comX: 0.07 }, abductionDeg: -WRIST_EXTENSION, title: text("Right hand on the cane handle", "Rechte Hand auf dem Stockgriff") } : { ...h, lengthM: 0.09, mass: { ...h.mass, comX: 0.07 } },
    ...hand(side, side === "r" ? "grip" : "relaxed"),
  ];
};
const leg = (side: "l" | "r"): Json[] => ["thigh", "shank", "foot", "toe"].map((b) => old(`${b}-${side}`));
const pelvis = old("pelvis");
const torsoOld = old("torso");
const bones: Json[] = [
  pelvis,
  { ...torsoOld, title: text("Lumbar spine and abdomen", "Lendenwirbelsäule und Bauch"), x: LS[1], y: -LS[0], angleDeg: LUMBAR.deg - 90, lengthM: LUMBAR.len, joint: rev(-35, 30), mass: mass(12.1, 0.08, -0.06) },
  { id: "chest", parent: "torso", title: text("Thoracic spine and rib cage", "Brustwirbelsäule und Brustkorb"), x: LUMBAR.len, y: 0, angleDeg: THORAX.deg - LUMBAR.deg, lengthM: THORAX.len, joint: rev(-20, 15), mass: mass(11.8, 0.14, -0.065) },
  { ...old("neck"), parent: "chest", title: text("Cervical spine and neck", "Halswirbelsäule und Hals"), x: THORAX.len, y: 0, angleDeg: NECK.deg - THORAX.deg, lengthM: NECK.len, mass: mass(0.7, 0.05, -0.01, "authored-estimate") },
  { ...old("head"), x: NECK.len, y: 0, angleDeg: HEAD.deg - NECK.deg, lengthM: HEAD.len, mass: mass(4.45, 0.035, -0.015) },
  ...arm("l"),
  ...leg("l"),
  ...arm("r"),
  ...leg("r"),
  { ...old("cane"), title: text("Walking cane with derby handle", "Gehstock mit Derbygriff"), x: HANDLE.along, y: HANDLE.shaftAcross, z: HANDLE.palmar, angleDeg: 0, abductionDeg: WRIST_EXTENSION, lengthM: CANE_L, mass: { ...old("cane").mass, comX: 0.32, comY: 0 } },
];
asset.rig.bones = bones;

/* ------------------------------------------------------------------ anchors */
const anchor = (id: string): Json => asset.anchors.find((a: Json) => a.id === id);
anchor("cane-tip").x = CANE_L;
Object.assign(anchor("cane-grip"), { x: HANDLE.along, y: 0, z: HANDLE.palmar });
Object.assign(anchor("mouth"), { x: -0.042, y: -0.107 });
Object.assign(anchor("nose"), { x: -0.022, y: -0.116 });
Object.assign(anchor("crown"), { x: 0.152, y: -0.005 });
asset.thermal.respiration.motion = [{ bone: "chest", angleDeg: 0.8, dy: 0.004 }];
asset.dimensions.widthM = 0.48;
asset.dimensions.heightM = 1.63;

/* ------------------------------------------------------------------ poses and clips */
const UPPER = old("arm-r").lengthM as number;
const FORE = old("forearm-r").lengthM as number;
const WRIST_LIMIT = Number(process.argv[process.argv.indexOf("--wrist") + 1] ?? 46) || 46;
type Arm = { arm: number; fore: number; hand: number; flexDeg: number; wristDeg: number; tipX: number; tipY: number; tauDeg: number };
/** 💪 The right arm for the shoulder `s`, the thorax direction `chestDeg`, the shaft top `o` and the cane tilt `tauDeg` (tip ahead of the handle is positive). */
function solve(s: [number, number], chestDeg: number, o: [number, number], tauDeg: number): Arm {
  const handDeg = -90 + tauDeg;
  const [c, n] = [Math.cos(rad(handDeg)), Math.sin(rad(handDeg))];
  const w: [number, number] = [o[0] - (ORIGIN[0] * c - ORIGIN[1] * n), o[1] - (ORIGIN[0] * n + ORIGIN[1] * c)];
  const d = Math.hypot(w[0] - s[0], w[1] - s[1]);
  if (d > (UPPER + FORE) * 0.9985) throw new Error(`wrist out of reach ${d.toFixed(4)}`);
  const flex = Math.PI - Math.acos((UPPER * UPPER + FORE * FORE - d * d) / (2 * UPPER * FORE));
  const armWorld = Math.atan2(w[1] - s[1], w[0] - s[0]) - Math.atan2(FORE * Math.sin(flex), UPPER + FORE * Math.cos(flex));
  const foreWorld = armWorld + flex;
  if (Math.abs(wrap(handDeg - deg(foreWorld))) > WRIST_LIMIT) throw new Error(`wrist beyond its range ${wrap(handDeg - deg(foreWorld)).toFixed(1)} elbow ${deg(flex).toFixed(1)}`);
  return { arm: r4(wrap(deg(armWorld) - (chestDeg + (old("arm-r").angleDeg as number)))), fore: r4(deg(flex)), hand: r4(wrap(handDeg - deg(foreWorld))), flexDeg: deg(flex), wristDeg: wrap(handDeg - deg(foreWorld)), tipX: o[0] + CANE_L * Math.sin(rad(tauDeg)), tipY: o[1] - CANE_L * Math.cos(rad(tauDeg)), tauDeg };
}
const armRest = -90 - THORAX.deg;
const solveArm = solve;

const FLEX = { thigh: 7, shank: -12, foot: 5 };
const pose = (id: string): Json => asset.poses.find((p: Json) => p.id === id);
for (const id of ["stooped-standing", "cane-planted", "resting-on-cane"]) for (const side of ["l", "r"]) Object.assign(pose(id).bones, { [`thigh-${side}`]: { angleDeg: FLEX.thigh }, [`shank-${side}`]: { angleDeg: FLEX.shank }, [`foot-${side}`]: { angleDeg: FLEX.foot } });

const feet = (): Json => {
  const f = JSON.parse(JSON.stringify(asset)) as Json;
  for (const c of f.clips as Json[]) c.contacts = c.contacts.filter((x: Json) => x.anchor !== "cane-tip");
  for (const p of f.poses as Json[]) p.contacts = p.contacts.filter((x: string) => x !== "cane-tip");
  f.thermal.respiration.motion = [];
  return f;
};
let FEET = feet();
/** 🦶 The right shoulder and the thorax direction with the body carried by the feet alone. */
function shoulder(q: Query): { s: [number, number]; chestDeg: number } {
  const rig = rigAt(FEET, 50, q);
  const f = rig.frames["arm-r"]!;
  const t = rig.frames.chest!;
  return { s: [f.o[0], f.o[1]], chestDeg: deg(Math.atan2(t.u[1], t.u[0])) };
}
/** 📍 A planted cane: the tip stands at (`tipX`, 0), the shaft top `ahead` of the shoulder; when the wrist is out of reach the top comes closer to the tip line. */
function planted(q: Query, tipX: number, ahead: number): Arm {
  const { s, chestDeg } = shoulder(q);
  if (process.argv.includes("--debug")) console.log("[DEBUG] shoulder", s, chestDeg);
  for (let k = 0; k < 400; k++) {
    const ox = s[0] + ahead + (tipX - s[0] - ahead) * (k / 400);
    const tau = deg(Math.asin((tipX - ox) / CANE_L));
    try { return solveArm(s, chestDeg, [ox, CANE_L * Math.cos(rad(tau))], tau); } catch (error) { if (k % 100 === 0 && process.argv.includes("--debug")) console.log("[DEBUG]", (error as Error).message); continue; }
  }
  throw new Error("no planted solution");
}
function apply(b: Json, a: Arm): void {
  b["arm-r"] = { angleDeg: a.arm };
  b["forearm-r"] = { angleDeg: a.fore };
  b["hand-r"] = { angleDeg: a.hand };
  b.cane = { angleDeg: 0 };
}
const show = (label: string, a: Arm): void => console.log(`${label.padEnd(30)} arm=${a.arm.toFixed(2)} elbow=${a.flexDeg.toFixed(1)} wrist=${a.wristDeg.toFixed(1)} tilt=${a.tauDeg.toFixed(1)} tip=(${a.tipX.toFixed(4)},${a.tipY.toFixed(4)})`);
/** 🖐️ The far arm hangs plumb from the shoulder whatever the lean of the trunk, the elbow a little bent. */
function hang(b: Json, q: Query): void {
  const { chestDeg } = shoulder(q);
  b["arm-l"] = { angleDeg: r4(wrap(-93 - (chestDeg + armRest))) };
  b["forearm-l"] = { angleDeg: 14 };
  b["hand-l"] = { angleDeg: 4 };
}
const STAND_TIP = 0.33;
const STAND_AHEAD = 0.175;
for (const id of ["stooped-standing", "cane-planted", "resting-on-cane"]) {
  const p = pose(id);
  FEET = feet();
  hang(p.bones, { kind: "pose", pose: id });
  const a = planted({ kind: "pose", pose: id }, STAND_TIP, STAND_AHEAD);
  apply(p.bones, a);
  show(`pose ${id}`, a);
}
pose("stooped-standing").description = text("Standing on both feet with a rounded upper back and slightly bent hips and knees, the cane planted a hand's breadth ahead of the toes beside the foot and lightly loaded.", "Stand auf beiden Füßen mit rundem Rücken und leicht gebeugten Hüften und Knien, der Stock eine Handbreit vor den Zehen neben dem Fuß aufgesetzt und leicht belastet.");
pose("resting-on-cane").description = text("Standing still and more upright, the near hand resting on the derby handle, the far arm hanging loosely, head up.", "Ruhiger, aufrechterer Stand, die nahe Hand ruht auf dem Derbygriff, der ferne Arm hängt locker, der Kopf ist gehoben.");

FEET = feet();
const rest = asset.clips.find((c: Json) => c.id === "rest-on-cane");
for (const phase of rest.phases as Json[]) if (phase.id === "settle") phase.title = text("Settling on the handle", "Auf den Griff stützen");
for (const key of rest.keys as Json[]) {
  if (key.bones === undefined) continue;
  const q: Query = { kind: "clip", clip: "rest-on-cane", timeS: key.t };
  hang(key.bones, q);
  apply(key.bones, planted(q, STAND_TIP, STAND_AHEAD));
}

const gait = asset.clips.find((c: Json) => c.id === "cane-gait");
const LIFT_T = 1.088;
const AHEAD0 = 0.23;
const AHEAD1 = -0.04;
const TILT0 = 9;
const LIFT_M = 0.03;
const sx = (t: number): number => shoulder({ kind: "clip", clip: "cane-gait", timeS: t }).s[0];
const s0 = sx(0);
const PLANT_X = s0 + AHEAD0 + CANE_L * Math.sin(rad(TILT0));
/** 🚶 The right arm at the time `t` of the gait cycle: planted until the cane lifts, then swung to the next plant with the tip clear of the ground. */
function gaitAt(q: Query, t: number): Arm {
  if (t <= LIFT_T) return planted(q, PLANT_X + (q.kind === "pose" ? shoulder(q).s[0] - sx(t) : 0), AHEAD0 + (AHEAD1 - AHEAD0) * (t / LIFT_T));
  const u = (t - LIFT_T) / (gait.durationS - LIFT_T);
  const e = u * u * (3 - 2 * u);
  const { s, chestDeg } = shoulder(q);
  const lift = LIFT_M * Math.sin(Math.PI * u) ** 2;
  const tipX = PLANT_X + (gait.advanceM as number) * e + (q.kind === "pose" ? s[0] - sx(t) : 0);
  for (let k = 0; k < 400; k++) {
    const ox = s[0] + AHEAD1 + (AHEAD0 - AHEAD1) * e;
    const reachX = ox + (tipX - ox) * (k / 400);
    const tau = deg(Math.asin((tipX - reachX) / CANE_L));
    try { return solveArm(s, chestDeg, [reachX, lift + CANE_L * Math.cos(rad(tau))], tau); } catch { continue; }
  }
  throw new Error("no swing solution");
}
for (const key of gait.keys as Json[]) {
  const q: Query = { kind: "clip", clip: "cane-gait", timeS: key.t };
  const a = gaitAt(q, key.t);
  apply(key.bones, a);
  if (Math.round(key.t * 60) % 10 === 0) show(`cane-gait t=${key.t}`, a);
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

for (const p of asset.poses as Json[]) {
  const rig = rigAt(asset, 50, { kind: "pose", pose: p.id });
  const b = rig.bones.find((x) => x.id === "cane")!;
  const sh = rig.frames["arm-r"]!.o;
  const hd = rig.bones.find((x) => x.id === "head")!;
  console.log(`check pose ${p.id.padEnd(18)} cane top=(${b.o.map((v) => v.toFixed(3)).join(",")}) tip=(${b.tip.map((v) => v.toFixed(3)).join(",")}) shoulder=(${sh[0].toFixed(3)},${sh[1].toFixed(3)}) vertex=(${hd.tip[0].toFixed(3)},${hd.tip[1].toFixed(3)})`);
}
{
  const rig = rigAt(asset, 50, { kind: "pose", pose: "stooped-standing" });
  for (const id of ["pelvis", "torso", "chest", "neck", "head", "clavicle-l", "arm-l", "arm-r", "forearm-r", "hand-r", "thigh-l", "shank-l", "foot-l"]) {
    const b = rig.bones.find((x) => x.id === id)!;
    console.log(`  ${id.padEnd(12)} o=(${b.o.map((v) => v.toFixed(3)).join(", ")}) tip=(${b.tip.map((v) => v.toFixed(3)).join(", ")})`);
  }
}
/** 🔤 The texts that still named the crook of the former cane. */
const WORDS: [string, string][] = [["a wooden crook cane", "a wooden cane with a derby handle"], ["einem Krückstock", "einem Holzstock mit Derbygriff"], ["Hand on the crook handle", "Hand on the derby handle"], ["Hand am Krückgriff", "Hand am Derbygriff"], ["hand on the crook", "hand on the handle"], ["settle on the crook", "settle on the handle"]];
const reword = (o: Json): Json => (typeof o === "string" ? WORDS.reduce((x, [from, to]) => x.split(from).join(to), o) : Array.isArray(o) ? o.map(reword) : o && typeof o === "object" ? Object.fromEntries(Object.entries(o).map(([k, v]) => [k, reword(v)])) : o);
for (const k of ["summary", "anchors", "poses", "clips", "thermal", "structure"]) asset[k] = reword(asset[k]);
asset.provenance.notes = text("A 70 kg man, stature 1.70 m in his prime and 1.65 m now; in the stooped stance (lumbar spine 6 degrees and thoracic chord 20 degrees forward of the vertical, thoracic kyphosis of about 55 degrees, hips and knees a little bent) he stands 1.63 m with shoes and cap. Segment masses from the de Leva male fractions plus 2.3 kg of clothing, the trunk split into abdomen and thorax; limb lengths scaled by 0.93. The rig runs along the vertebral bodies and carries clavicles and fifteen finger bones per hand; the right wrist is extended over the derby handle of a 78.5 cm cane fitted to the wrist crease of the stooped stance. Gait key frames from inverse kinematics of smooth foot paths with shortened strides; the cane tip path and the arm that holds it are solved so that the tip stays fixed in the stance.", "Ein 70 kg schwerer Mann, Körpergröße 1,70 m in seinen besten Jahren und 1,65 m heute; in der gebeugten Haltung (Lendenwirbelsäule 6 Grad und Brustsehne 20 Grad vor der Lotrechten, Brustkyphose von etwa 55 Grad, Hüften und Knie leicht gebeugt) steht er mit Schuhen und Mütze 1,63 m hoch. Segmentmassen nach den männlichen Anteilen von de Leva plus 2,3 kg Kleidung, der Rumpf in Bauch und Brustkorb geteilt; Gliedlängen mit 0,93 skaliert. Das Skelett läuft entlang der Wirbelkörper und trägt Schlüsselbeine und fünfzehn Fingerknochen je Hand; das rechte Handgelenk ist über den Derbygriff eines 78,5 cm langen Stocks gestreckt, der auf die Handgelenkfalte der gebeugten Haltung angepasst ist. Ganghaltungen aus inverser Kinematik glatter Fußbahnen mit verkürzten Schritten; die Bahn der Stockspitze und der haltende Arm sind so gelöst, dass die Spitze in der Standphase fest bleibt.");
writeJsonFile(join(workDir("elderly-with-cane"), "rig.json"), asset);
console.log("bones", bones.length, "clavicle", { CL_LEN, CL_ANGLE, CL_SPREAD }, "shoulder in thorax", gh, "sc", sc, "cane origin", ORIGIN);
