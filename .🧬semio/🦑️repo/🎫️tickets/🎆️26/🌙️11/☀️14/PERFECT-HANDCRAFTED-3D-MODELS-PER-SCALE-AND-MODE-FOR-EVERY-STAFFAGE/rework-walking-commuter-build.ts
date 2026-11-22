/** 🧥 Rework of `humans/walking-commuter`, pass 1 of the second round: the sagittal chain of the rig (lumbar spine, thorax, cervical spine, shoulder joint 30 mm lower, metacarpus and fifteen finger bones per hand), the right arm re-posed so that the fist closes round the strap, the overcoat cut at the waist with the real sagittal profile, hands on their finger bones, a vertebral column of single vertebrae, and every solid, drawing shape, anchor, pose and clip key carried to the new bones. Every number is stated by hand here.
 *
 * Usage: `bun rework-walking-commuter-build.ts` reads the catalog document as it stood before this pass (`rework-walking-commuter-base.json`) and writes the candidate document `🗑️generated/rework-walking-commuter/🔣️.json`.
 * @see ./rework/assets/humans--walking-commuter.md
 * @see ./rework-04-kit.ts */
import { join } from "node:path";
import { GENERATED, type Json } from "./volumes-lib.ts";
import { skeleton } from "./rework-walking-commuter-skeleton.ts";
import { capsule, deg, dense, dome, ellipsoid, loft, makeKit, r5, rad, readJsonFile, shell, writeJsonFile, type Quat, type St, type V3 } from "./rework-04-kit.ts";

const doc = readJsonFile(join(import.meta.dir, "rework-walking-commuter-base.json"));
const asset = doc.asset as Json;
const SCALES = ["50", "100", "200", "500"] as const;
const LISTS = ["body", "thermal", "structure"] as const;
const text = (en: string, de: string): Json => ({ en, de });
const rev = (minDeg: number, maxDeg: number): Json => ({ type: "revolute", minDeg, maxDeg });
const old = (id: string): Json => JSON.parse(JSON.stringify(asset.rig.bones.find((b: Json) => b.id === id)));
const before = makeKit(JSON.parse(JSON.stringify(asset)));

/* ------------------------------------------------------------------ rig: lengths of the reference table */
const LUMBAR = 0.18;
const THORAX = 0.295;
const DROP = 0.03;
const HAND_Z = 0.031;
const METACARPUS = 0.095;
const neckOld = old("neck");
const neckTip: [number, number] = [0.495 + 0.1 * Math.cos(rad(neckOld.angleDeg)), 0.1 * Math.sin(rad(neckOld.angleDeg))];
const neckVector: [number, number] = [neckTip[0] - LUMBAR - THORAX, neckTip[1]];
const NECK_LEN = r5(Math.hypot(...neckVector));
const NECK_DEG = r5(deg(Math.atan2(neckVector[1], neckVector[0])));

/** ✋ Metacarpophalangeal joints in the hand frame [along, towards the thumb] and joint-to-joint phalanx lengths of a male hand of 0.19 m, metres. */
const FINGERS: Record<string, { at: [number, number]; lens: [number, number, number]; fan: number; radii: [number, number, number] }> = {
  index: { at: [0.092, 0.028], lens: [0.042, 0.024, 0.019], fan: 4, radii: [0.0095, 0.0088, 0.0078] },
  middle: { at: [0.095, 0.008], lens: [0.046, 0.028, 0.02], fan: 0, radii: [0.0098, 0.009, 0.008] },
  ring: { at: [0.09, -0.012], lens: [0.043, 0.027, 0.019], fan: -3, radii: [0.0092, 0.0085, 0.0075] },
  little: { at: [0.08, -0.031], lens: [0.034, 0.019, 0.017], fan: -8, radii: [0.0082, 0.0075, 0.0068] },
};
const THUMB_LENS: [number, number, number] = [0.046, 0.033, 0.025];
const THUMB_RADII: [number, number, number] = [0.0118, 0.0105, 0.0092];
const THUMB_SEGMENTS = ["metacarpal", "proximal", "distal"] as const;
const SEGMENTS = ["proximal", "middle", "distal"] as const;
const SEG_TITLE: Record<string, [string, string]> = { proximal: ["proximal phalanx", "Grundglied"], middle: ["middle phalanx", "Mittelglied"], distal: ["distal phalanx", "Endglied"], metacarpal: ["metacarpal", "Mittelhandbein"] };
const FINGER_TITLE: Record<string, [string, string]> = { thumb: ["thumb", "Daumen"], index: ["index finger", "Zeigefinger"], middle: ["middle finger", "Mittelfinger"], ring: ["ring finger", "Ringfinger"], little: ["little finger", "Kleinfinger"] };
/** 🤏 Curl of the three joints of each finger towards the palm, degrees: the hanging hand and the fist round the 36 mm strap. */
const CURL = { relaxed: { index: [14, 22, 10], middle: [18, 26, 12], ring: [22, 30, 14], little: [26, 32, 16] }, grip: { index: [84, 96, 46], middle: [86, 98, 46], ring: [88, 98, 46], little: [90, 98, 46] } } as const;
const THUMB = { relaxed: { angle: [38, -6, -4], curl: [22, 8, 10] }, grip: { angle: [20, -15, -10], curl: [25, 10, 25] } } as const;
type Side = "l" | "r";
const sign = (side: Side): number => (side === "l" ? 1 : -1);
const stateOf = (side: Side): "relaxed" | "grip" => (side === "r" ? "grip" : "relaxed");

function fingerBones(side: Side): Json[] {
  const k = sign(side);
  const state = stateOf(side);
  const [enF, deF] = side === "l" ? ["Left", "Linkes"] : ["Right", "Rechtes"];
  const out: Json[] = [];
  THUMB_SEGMENTS.forEach((seg, i) => {
    out.push({ id: `thumb-${seg}-${side}`, parent: i === 0 ? `hand-${side}` : `thumb-${THUMB_SEGMENTS[i - 1]}-${side}`, title: text(`${enF} thumb, ${SEG_TITLE[seg]![0]}`, `${deF} ${SEG_TITLE[seg]![1]} des Daumens`), x: i === 0 ? 0.022 : THUMB_LENS[i - 1], y: i === 0 ? 0.028 : 0, z: i === 0 ? -0.008 * k : 0, angleDeg: THUMB[state].angle[i], abductionDeg: -k * THUMB[state].curl[i]!, lengthM: THUMB_LENS[i], joint: rev(-25, 25) });
  });
  for (const [finger, f] of Object.entries(FINGERS)) {
    const curl = CURL[state][finger as keyof typeof CURL.relaxed];
    SEGMENTS.forEach((seg, i) => {
      out.push({ id: `${finger}-${seg}-${side}`, parent: i === 0 ? `hand-${side}` : `${finger}-${SEGMENTS[i - 1]}-${side}`, title: text(`${enF} ${FINGER_TITLE[finger]![0]}, ${SEG_TITLE[seg]![0]}`, `${deF} ${SEG_TITLE[seg]![1]} des ${FINGER_TITLE[finger]![1]}s`), x: i === 0 ? f.at[0] : f.lens[i - 1], y: i === 0 ? f.at[1] : 0, z: 0, angleDeg: i === 0 ? f.fan : 0, abductionDeg: -k * curl[i]!, lengthM: f.lens[i], joint: rev(-20, 20) });
    });
  }
  return out;
}
const arm = (side: Side): Json[] => {
  const h = old(`hand-${side}`);
  return [clavicle(side), hung({ ...old(`arm-${side}`), x: r5(0.42 - LUMBAR - DROP) }, "arm-l", side), old(`forearm-${side}`), { ...h, z: sign(side) * HAND_Z, lengthM: METACARPUS, mass: { ...h.mass, comX: 0.08 }, title: side === "l" ? text("Left hand, metacarpus", "Linke Hand, Mittelhand") : text("Right hand, metacarpus", "Rechte Hand, Mittelhand") }, ...fingerBones(side)];
};
/** 🦴 Rest placement of the girdle bones and of the limbs that hang from them, left side, as `rig-rest.ts --solve` gave it for the joints of the skeleton (right side: z, abductionDeg and twistDeg negated). */
const GIRDLE_REST: Record<string, Json> = {
  "clavicle-l": { parent: "torso", x: 0.251, y: -0.03777, z: 0.02, angleDeg: 90.1739, abductionDeg: 73.6109, twistDeg: -179.8187, lengthM: 0.17512 },
  "ilium-l": { parent: "pelvis", x: 0.07101, y: 0.05277, z: 0.052, angleDeg: -143.3827, abductionDeg: 22.9706, twistDeg: -73.8271, lengthM: 0.09609 },
};
const LIMB_REST: Record<string, Json | null> = {
  "arm-l": { parent: "clavicle-l", x: 0.17378, y: -0.04085, z: -0.01174, angleDeg: -89.9509, abductionDeg: -0.0145, twistDeg: 106.389 },
  "thigh-l": { parent: "ilium-l", x: 0.09609, y: 0, z: 0, angleDeg: -32.2893, abductionDeg: 29.0527, twistDeg: 72.9406 },
};
const sided = (rest: Json, side: Side): Json => (side === "l" ? rest : { ...rest, parent: (rest.parent as string).replace(/-l$/, "-r"), z: -(rest.z ?? 0), ...(rest.abductionDeg === undefined ? {} : { abductionDeg: -rest.abductionDeg }), ...(rest.twistDeg === undefined ? {} : { twistDeg: -rest.twistDeg }) });
const clavicle = (side: Side): Json => ({ id: `clavicle-${side}`, title: side === "l" ? text("Left clavicle and shoulder blade", "Linkes Schlüsselbein mit Schulterblatt") : text("Right clavicle and shoulder blade", "Rechtes Schlüsselbein mit Schulterblatt"), ...sided(GIRDLE_REST["clavicle-l"]!, side), joint: { type: "revolute", minDeg: -8, maxDeg: 35, twistMinDeg: side === "l" ? -40 : -5, twistMaxDeg: side === "l" ? 5 : 40 } });
const ilium = (side: Side): Json => ({ id: `ilium-${side}`, title: side === "l" ? text("Left hip bone", "Linkes Hüftbein") : text("Right hip bone", "Rechtes Hüftbein"), ...sided(GIRDLE_REST["ilium-l"]!, side), joint: rev(-5, 5) });
const hung = (bone: Json, key: string, side: Side): Json => (LIMB_REST[key] ? { ...bone, ...sided(LIMB_REST[key]!, side) } : bone);
const leg = (side: Side): Json[] => [ilium(side), ...["thigh", "shank", "foot", "toe"].map((b) => (b === "thigh" ? hung(old(`${b}-${side}`), "thigh-l", side) : old(`${b}-${side}`)))];
const torsoOld = old("torso");
asset.rig.bones = [
  old("pelvis"),
  { id: "lumbar", parent: "pelvis", title: text("Lumbar spine and abdomen", "Lendenwirbelsäule und Bauch"), x: 0.1, y: 0, angleDeg: 0, lengthM: LUMBAR, joint: rev(-25, 15), mass: { ...torsoOld.mass, kg: 12.35, comX: 0.095 } },
  { ...torsoOld, parent: "lumbar", title: text("Thoracic spine and rib cage with coat", "Brustwirbelsäule und Brustkorb mit Mantel"), x: LUMBAR, lengthM: THORAX, joint: rev(-15, 10), mass: { ...torsoOld.mass, kg: 12.1, comX: 0.15 } },
  { ...neckOld, title: text("Cervical spine and neck", "Halswirbelsäule und Hals"), x: THORAX, angleDeg: NECK_DEG, lengthM: NECK_LEN, mass: { ...neckOld.mass, comX: 0.06 } },
  { ...old("head"), x: NECK_LEN, angleDeg: r5(neckOld.angleDeg + old("head").angleDeg - NECK_DEG) },
  ...arm("l"),
  ...leg("l"),
  ...arm("r"),
  ...leg("r"),
  { ...old("bag"), x: r5(0.42 - LUMBAR) },
];

/* ------------------------------------------------------------------ poses, clips, anchors */
/** 💪 The right arm that holds the strap: upper arm 24 degrees back, elbow flexed 147, wrist 25 degrees towards the little finger so that the fist stands upright round the strap. */
const GRIP = { arm: -23, forearm: 147, hand: -25 };
function carry(b: Json | undefined): void {
  if (!b) return;
  if (b.torso) {
    const a = b.torso.angleDeg as number;
    b.lumbar = { angleDeg: r5(a * 0.4) };
    b.torso = { ...b.torso, angleDeg: r5(a * 0.6) };
  }
  if (b["forearm-r"]?.angleDeg === 145) {
    b["arm-r"] = { angleDeg: GRIP.arm };
    b["forearm-r"] = { angleDeg: GRIP.forearm };
    b["hand-r"] = { angleDeg: GRIP.hand };
  }
}
for (const p of asset.poses as Json[]) carry(p.bones);
for (const c of asset.clips as Json[]) for (const key of c.keys as Json[]) carry(key.bones);
/** ⌚ The watch is read on the far (left) wrist, so the near hand never lets go of the strap: far upper arm 19 degrees forward, elbow flexed 104 so that the forearm rises gently at the height of the chest (the drawings are flat shapes in the sagittal plane, so the forearm cannot be turned across the body), the wrist straight (a bent wrist leaves a sliver of the hand next to the cuff that does not print at 1:1000), the fingers closing a little, the far shoulder lifted 4 degrees on its clavicle. */
{
  const watch = (asset.poses as Json[]).find((p) => p.id === "check-watch")!;
  const closing: Json = {};
  for (const finger of Object.keys(FINGERS)) { closing[`${finger}-proximal-l`] = { angleDeg: 0, abductionDeg: -14 }; closing[`${finger}-middle-l`] = { angleDeg: 0, abductionDeg: -16 }; }
  Object.assign(watch.bones, { "arm-r": { angleDeg: GRIP.arm }, "forearm-r": { angleDeg: GRIP.forearm }, "hand-r": { angleDeg: GRIP.hand }, "arm-l": { angleDeg: 22, abductionDeg: 0 }, "forearm-l": { angleDeg: 112 }, "hand-l": { angleDeg: 0 }, "clavicle-l": { angleDeg: 4 }, bag: { angleDeg: 0 }, ...closing });
  watch.description = text("Standing still, the far forearm raised in front of the body to read the watch on the left wrist, the near hand staying on the bag strap, head and neck bowed towards the wrist.", "Ruhiger Stand, der ferne Unterarm vor dem Körper angehoben zum Ablesen der Uhr am linken Handgelenk, die nahe Hand bleibt am Taschenriemen, Kopf und Hals zum Handgelenk geneigt.");
  const glance = (asset.clips as Json[]).find((c) => c.id === "glance-at-watch")!;
  glance.description = text("Standing still with both feet flat: the far forearm rises in front of the body to read the watch on the left wrist, holds, and is lowered again while the head follows; the near hand keeps the bag strap.", "Ruhiger Stand mit beiden Füßen flach: Der ferne Unterarm hebt sich vor den Körper zum Ablesen der Uhr am linken Handgelenk, hält und wird wieder gesenkt, der Kopf folgt; die nahe Hand hält den Taschenriemen.");
}
/** 🫁 Breathing lifts both shoulders on their clavicles by a degree and a half at the top of the breath. */
Object.assign(((asset.clips as Json[]).find((c) => c.id === "idle-breathing")!.keys as Json[]).find((k) => k.t === 2)!.bones, { "clavicle-l": { angleDeg: 1.5 }, "clavicle-r": { angleDeg: 1.5 } });
Object.assign(asset.anchors.find((a: Json) => a.id === "strap-grip"), { x: 0.076, y: 0, z: 0.024 });

asset.dimensions.widthM = 0.425;
const kit = makeKit(asset);
for (const id of ["head", "neck", "bag", "arm-l", "hand-l", "thigh-l", "foot-l"]) {
  const [a, b] = [before.FRAMES[id]!, kit.FRAMES[id]!];
  console.log(`[DEBUG] frame ${id.padEnd(8)} before o=(${a.o.map((x) => x.toFixed(4))}) u=(${a.u.map((x) => x.toFixed(4))}) now o=(${b.o.map((x) => x.toFixed(4))}) u=(${b.u.map((x) => x.toFixed(4))})`);
}

/* ------------------------------------------------------------------ carrying solids and shapes from an old bone frame to the new one */
type Move = { d: [number, number, number]; turn: number };
const qmul = (a: number[], b: number[]): Quat => [a[3]! * b[0]! + a[0]! * b[3]! + a[1]! * b[2]! - a[2]! * b[1]!, a[3]! * b[1]! - a[0]! * b[2]! + a[1]! * b[3]! + a[2]! * b[0]!, a[3]! * b[2]! + a[0]! * b[1]! - a[1]! * b[0]! + a[2]! * b[3]!, a[3]! * b[3]! - a[0]! * b[0]! - a[1]! * b[1]! - a[2]! * b[2]!].map(r5) as Quat;
const pt = (m: Move, p: readonly number[]): number[] => {
  const [x, y] = [p[0]! - m.d[0], p[1]! - m.d[1]];
  const [c, s] = [Math.cos(rad(m.turn)), Math.sin(rad(m.turn))];
  const out = [r5(c * x + s * y), r5(-s * x + c * y)];
  return p.length > 2 ? [...out, r5(p[2]! - m.d[2])] : out;
};
function moveSolid(s: Json, m: Move): void {
  const qz: Quat = [0, 0, Math.sin(rad(-m.turn) / 2), Math.cos(rad(-m.turn) / 2)];
  const turn = (q: number[] | undefined): Quat | undefined => (m.turn === 0 ? (q as Quat | undefined) : qmul(qz, q ?? [0, 0, 0, 1]));
  if (s.of) return;
  if (s.sections) for (const q of s.sections as Json[]) { q.at = pt(m, q.at); const t = turn(q.rotation); if (t) q.rotation = t; }
  else if (s.p0) { s.p0 = pt(m, s.p0); s.p1 = pt(m, s.p1); }
  else if (s.kind === "ellipsoid" || s.kind === "box") { const tr = s.transform ?? {}; const t = turn(tr.rotation); s.transform = { ...(t ? { rotation: t } : {}), translation: pt(m, tr.translation ?? [0, 0, 0]) }; }
  else if (s.vertices) s.vertices = (s.vertices as number[][]).map((v) => pt(m, v));
  else throw new Error(`cannot carry ${s.id} (${s.kind})`);
}
const centreU = (s: Json): number => (s.sections ? (s.sections as Json[]).reduce((a, q) => a + q.at[0], 0) / s.sections.length : s.p0 ? (s.p0[0] + s.p1[0]) / 2 : s.transform?.translation?.[0] ?? 0);
const NECK_MOVE: Move = { d: [-0.02 * Math.cos(rad(-neckOld.angleDeg)), -0.02 * Math.sin(rad(-neckOld.angleDeg)), 0], turn: NECK_DEG - neckOld.angleDeg };
const THORAX_MOVE: Move = { d: [LUMBAR, 0, 0], turn: 0 };
const GIRDLE = /^(sk-scapula|sk-scapular-spine|sk-glenoid|sk-coracoid)-/;
const SHEAR = /^(sk-clavicle|shoulder-girdle)-/;

/* ------------------------------------------------------------------ the overcoat: horizontal slices of the figure [height, back, front, half breadth, exponent] */
type Slice = [number, number, number, number, number];
const WAIST: Slice[] = [[0.975, -0.14, 0.136, 0.192, 2.4], [1.01, -0.139, 0.135, 0.189, 2.4], [1.05, -0.131, 0.134, 0.185, 2.4], [1.1, -0.117, 0.133, 0.178, 2.4], [1.15, -0.107, 0.132, 0.174, 2.4], [1.19, -0.104, 0.133, 0.172, 2.4], [1.235, -0.107, 0.137, 0.172, 2.4]];
const WAIST_TONGUE: Slice = [1.262, -0.099, 0.129, 0.164, 2.4];
const CHEST_TONGUE: Slice = [1.208, -0.098, 0.128, 0.164, 2.4];
const CHEST: Slice[] = [[1.235, -0.107, 0.137, 0.172, 2.4], [1.28, -0.116, 0.141, 0.173, 2.4], [1.32, -0.128, 0.145, 0.176, 2.35], [1.36, -0.138, 0.1455, 0.18, 2.35], [1.4, -0.145, 0.141, 0.189, 2.3], [1.43, -0.146, 0.134, 0.203, 2.3], [1.455, -0.141, 0.126, 0.213, 2.3], [1.475, -0.132, 0.117, 0.211, 2.25], [1.492, -0.12, 0.108, 0.193, 2.2], [1.508, -0.106, 0.098, 0.165, 2.2], [1.524, -0.09, 0.088, 0.134, 2.1], [1.54, -0.072, 0.078, 0.108, 2.05], [1.554, -0.056, 0.068, 0.092, 2]];
/** 🪜 The slices kept per scale: every slice down to 1:100, the turning points of the profile at 1:200, the four that carry waist, chest, shoulder and collar at 1:500. */
const KEEP: Record<string, { waist: number[]; chest: number[]; density: number }> = { "50": { waist: [0, 1, 2, 3, 4, 5, 6], chest: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12], density: 3 }, "100": { waist: [0, 1, 2, 3, 4, 5, 6], chest: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12], density: 1 }, "200": { waist: [0, 2, 4, 6], chest: [0, 2, 4, 6, 8, 10, 12], density: 1 }, "500": { waist: [0, 3, 6], chest: [0, 3, 6, 9, 12], density: 1 } };
const slice = (bone: string, s: Slice): St => kit.H(bone, s[0], s[1], s[2], s[3], s[4]);
function coat(scale: string, proto: Json): Json[] {
  const keep = KEEP[scale]!;
  const common = { material: proto.material as string, tone: proto.tone as number, role: proto.role as string };
  const waist = [...dense(keep.waist.map((i) => slice("lumbar", WAIST[i]!)), keep.density), slice("lumbar", WAIST_TONGUE)];
  const chest = [slice("torso", CHEST_TONGUE), ...dense(keep.chest.map((i) => slice("torso", CHEST[i]!)), keep.density)];
  return [loft({ id: "coat-waist", bone: "lumbar", ...common }, waist), loft({ id: "coat", bone: "torso", ...common }, chest)];
}
const ALL: Slice[] = [...WAIST, ...CHEST.slice(1)];
/** 📏 The coat surface at the height `y` and the lateral position `z`: its front (`side` 1) or back (-1) x. */
function surface(y: number, z: number, side: 1 | -1): number {
  let i = 0;
  while (i + 2 < ALL.length && ALL[i + 1]![0] < y) i++;
  const [a, b] = [ALL[i]!, ALL[i + 1]!];
  const t = Math.min(1, Math.max(0, (y - a[0]) / (b[0] - a[0])));
  const [back, front, rw, e] = [1, 2, 3, 4].map((k) => a[k]! + (b[k]! - a[k]!) * t) as [number, number, number, number];
  const rest = 1 - Math.abs(z / rw) ** e;
  return (back + front) / 2 + (rest <= 0 ? 0 : side * ((front - back) / 2) * rest ** (1 / e));
}

/* ------------------------------------------------------------------ the strap: from the front lug through the fist, over the shoulder, down the back to the rear lug */
const handR = kit.FRAMES["hand-r"]!;
const GRIP_POINT: V3 = [0, 1, 2].map((k) => handR.o[k]! + 0.076 * handR.u[k]! + 0.024 * handR.w[k]!) as V3;
const ZS = -0.146;
const LUG_FRONT = kit.W("bag", [0.409, 0.13, -0.103]);
const LUG_BACK = kit.W("bag", [0.414, -0.15, -0.103]);
const STRAP: V3[] = [
  LUG_FRONT,
  [0.5 * (LUG_FRONT[0] + GRIP_POINT[0]) + 0.006, 0.5 * (LUG_FRONT[1] + GRIP_POINT[1]), 0.5 * (LUG_FRONT[2] + GRIP_POINT[2]) + 0.004],
  GRIP_POINT,
  [surface(1.385, -0.158, 1) + 0.022, 1.385, -0.158],
  [surface(1.44, ZS, 1) + 0.006, 1.44, ZS],
  [surface(1.478, ZS, 1) + 0.005, 1.478, ZS],
  [surface(1.502, ZS, 1) + 0.005, 1.504, ZS],
  [0.052, 1.519, ZS],
  [0, 1.5285, ZS],
  [-0.058, 1.519, ZS],
  [surface(1.5, ZS, -1) - 0.005, 1.502, ZS],
  [surface(1.47, ZS, -1) - 0.005, 1.47, ZS],
  [surface(1.4, -0.148, -1) - 0.005, 1.4, -0.148],
  [surface(1.3, -0.158, -1) - 0.005, 1.3, -0.158],
  [-0.132, 1.198, -0.208],
  LUG_BACK,
];
function strap(proto: Json): Json {
  const q0 = proto.sections[0] as Json;
  const unit = (v: number[]): V3 => { const n = Math.hypot(...v) || 1; return v.map((x) => x / n) as V3; };
  const sections = STRAP.map((p, i) => {
    const t = unit([0, 1, 2].map((k) => STRAP[Math.min(i + 1, STRAP.length - 1)]![k]! - STRAP[Math.max(i - 1, 0)]![k]!));
    const across = unit([-t[0] * t[2], -t[1] * t[2], 1 - t[2] * t[2]]);
    const normal: V3 = [t[1] * across[2] - t[2] * across[1], t[2] * across[0] - t[0] * across[2], t[0] * across[1] - t[1] * across[0]];
    return kit.S("bag", p, across, normal, q0.rx, q0.ry, q0.exponent ?? 4);
  });
  const fresh = loft({ id: proto.id, bone: "bag", material: proto.material, tone: proto.tone, role: proto.role }, sections);
  return { ...proto, sections: fresh.sections };
}

/* ------------------------------------------------------------------ hands: metacarpus on the hand bone, every phalanx on its own bone */
function hand(side: Side, tone: number, floor: number): Json[] {
  const k = sign(side);
  const bone = `hand-${side}`;
  const skin = { material: "skin", tone };
  const palm: St[] = [kit.st(-0.004, 0, 0, 0.019, 0.029, 2.2), kit.st(0.022, 0.002, 0, 0.0168, 0.036, 2.3), kit.st(0.052, 0.003, 0, 0.0146, 0.0418, 2.4), kit.st(0.078, 0.001, 0, 0.0128, 0.0428, 2.4), kit.st(0.093, -0.001, 0, 0.0108, 0.0405, 2.3)];
  const out: Json[] = [loft({ id: bone, bone, ...skin }, dome(dense(palm, 2), 1, 0.008)), ellipsoid({ id: `${bone}-thenar`, bone, ...skin }, [0.028, 0.015, 0.011], [0.036, 0.026, -k * 0.008]), ellipsoid({ id: `${bone}-hypothenar`, bone, ...skin }, [0.03, 0.011, 0.0095], [0.042, -0.03, -k * 0.006])];
  THUMB_SEGMENTS.forEach((seg, i) => out.push(capsule({ id: `${bone}-thumb-${seg}`, bone: `thumb-${seg}-${side}`, ...skin }, [0, 0, 0], [THUMB_LENS[i]!, 0, 0], Math.max(floor, THUMB_RADII[i]! + (i === 0 ? 0.001 : 0)), Math.max(floor, THUMB_RADII[i]! - 0.0008))));
  for (const [finger, f] of Object.entries(FINGERS)) SEGMENTS.forEach((seg, i) => out.push(capsule({ id: `${bone}-${finger}-${seg}`, bone: `${finger}-${seg}-${side}`, ...skin }, [0, 0, 0], [f.lens[i]!, 0, 0], Math.max(floor, f.radii[i]! + 0.0004), Math.max(floor, f.radii[i]! - 0.0004))));
  return out;
}
const curveOf = (sections: Json[]) => (u: number): number => {
  const s = sections.map((q) => [q.at[0], q.at[1]] as [number, number]);
  if (u <= s[0]![0]) return s[0]![1];
  for (let i = 0; i + 1 < s.length; i++) if (u <= s[i + 1]![0]) return s[i]![1] + ((s[i + 1]![1] - s[i]![1]) * (u - s[i]![0])) / (s[i + 1]![0] - s[i]![0]);
  return s[s.length - 1]![1];
};
/** 🌀 The sagittal curve of the vertebral bodies behind the bone axes: lumbar lordosis and thoracic kyphosis in the frame of the lumbar bone, the cervical lordosis in the frame of the neck. */
const column50 = asset.volumes["50"].structure as Json[];
const back = curveOf(JSON.parse(JSON.stringify(column50.find((s) => s.id === "sk-spine").sections)));
const cervical50 = JSON.parse(JSON.stringify(column50.find((s) => s.id === "cervical-spine")));
moveSolid(cervical50, NECK_MOVE);
const nape = curveOf(cervical50.sections);

/* ------------------------------------------------------------------ every scale */
const lumbarShapes: Record<string, Set<string>> = {};
for (const scale of SCALES) {
  const volume = asset.volumes[scale] as Json;
  const find = (list: string, id: string): Json | undefined => (volume[list] as Json[]).find((s) => s.id === id);
  const coatOld = JSON.parse(JSON.stringify(find("body", "coat")));
  const strapOld = JSON.parse(JSON.stringify(find("body", "bag-strap")));
  const spineOld = JSON.parse(JSON.stringify(find("structure", "sk-spine")));
  const shells = (volume.thermal as Json[]).filter((s) => /^th-coat-(jacket|shirt|tee)$/.test(s.id)).map((s) => ({ id: s.id as string, t: coatOld.sections[0].rx - s.sections[0].rx, layer: s.layer as string, region: s.region as string, tone: s.tone as number }));
  const tones = { l: (find("body", "hand-l")?.tone as number) ?? 0.71, r: (find("body", "hand-r")?.tone as number) ?? 0.78 };

  for (const list of LISTS) {
    const out: Json[] = [];
    for (const s of volume[list] as Json[]) {
      if (s.of) { out.push(s); continue; }
      if (s.bone === "neck") moveSolid(s, NECK_MOVE);
      else if (s.bone === "hand-l") moveSolid(s, { d: [0, 0, HAND_Z], turn: 0 });
      else if (s.bone === "torso") {
        if (/^th-coat-(jacket|shirt|tee)$/.test(s.id) || s.id === "coat" || s.id === "bag-strap") continue;
        if (s.id === "sk-spine") {
          if (scale === "50") continue;
          if (scale === "500") { moveSolid(s, THORAX_MOVE); out.push(s); continue; }
          const q = s.sections as Json[];
          const i = q.findIndex((x) => x.at[0] > LUMBAR);
          const t = (LUMBAR - q[i - 1].at[0]) / (q[i].at[0] - q[i - 1].at[0]);
          const cut = { ...q[i - 1], at: [LUMBAR, r5(q[i - 1].at[1] + (q[i].at[1] - q[i - 1].at[1]) * t), 0], rx: r5(q[i - 1].rx + (q[i].rx - q[i - 1].rx) * t), ry: r5(q[i - 1].ry + (q[i].ry - q[i - 1].ry) * t) };
          out.push({ ...s, id: "sk-lumbar-spine", bone: "lumbar", member: "lumbar", sections: [...q.slice(0, i), cut, { ...cut, at: [LUMBAR + 0.012, cut.at[1], 0] }] });
          const upper = { ...s, sections: [{ ...cut, at: [LUMBAR - 0.012, cut.at[1], 0] }, JSON.parse(JSON.stringify(cut)), ...q.slice(i)] };
          moveSolid(upper, THORAX_MOVE);
          out.push(upper);
          continue;
        }
        if (/^sk-process-/.test(s.id) && scale === "50") continue;
        if (s.id === "mass-torso") {
          out.push({ ...s, id: "mass-lumbar", bone: "lumbar", member: "lumbar", radii: (s.radii as number[]).map((r) => r5(r * 0.8)), transform: { translation: [0.095, 0, 0] } });
          out.push({ ...s, radii: (s.radii as number[]).map((r) => r5(r * 0.8)), transform: { translation: [0.15, 0, 0] } });
          continue;
        }
        if (s.id === "joint-torso") {
          out.push({ ...s, id: "joint-lumbar", bone: "lumbar", member: "lumbar" });
          out.push({ ...s, transform: { translation: [0, r5(s.transform.translation[1] - 0.012), 0] } });
          continue;
        }
        if (centreU(s) < LUMBAR) { s.bone = "lumbar"; if (s.member === "torso") s.member = "lumbar"; }
        else {
          moveSolid(s, THORAX_MOVE);
          if (GIRDLE.test(s.id)) moveSolid(s, { d: [DROP, 0, 0], turn: 0 });
          if (SHEAR.test(s.id)) for (const q of s.sections as Json[]) q.at[0] = r5(q.at[0] - DROP * Math.min(1, Math.abs(q.at[2]) / 0.17));
        }
      }
      out.push(s);
    }
    volume[list] = out;
  }

  /* body */
  let body = volume.body as Json[];
  const place = (after: string, solids: Json[]): void => { const i = body.findIndex((s) => s.id === after); body.splice(i < 0 ? body.length : i + 1, 0, ...solids); };
  place("pelvis-trousers", coat(scale, coatOld));
  body = body.filter((s) => s.id !== "bag-strap");
  body.push(strap(strapOld));
  if (scale === "50" || scale === "100") {
    body = body.filter((s) => !/^hand-[lr]/.test(s.id));
    body.push(...hand("l", tones.l, scale === "100" ? 0.0078 : 0), ...hand("r", tones.r, scale === "100" ? 0.0078 : 0));
  } else {
    const fist = find("body", "hand-r")!;
    const radii: V3 = scale === "200" ? [0.05, 0.044, 0.036] : [0.056, 0.046, 0.0375];
    Object.assign(fist, { radii, transform: { translation: [0.064, 0, scale === "200" ? 0.016 : 0.01] } });
  }
  volume.body = body;

  /* structure: the skeleton of this scale and the bag on its strap */
  const bagParts = (volume.structure as Json[]).filter((s) => /bag/.test(s.id));
  for (const s of bagParts) if (s.id === "joint-bag") s.transform = { translation: kit.L("bag", [0, 1.5285, ZS]).map(r5) };
  volume.structure = [...skeleton(asset, kit, Number(scale) as 50 | 100 | 200 | 500, back, nape), ...bagParts];

  /* thermal: references follow the body list, the coat layers are shells of the two coat pieces */
  const ids = new Set((volume.body as Json[]).map((s) => s.id as string));
  let thermal = (volume.thermal as Json[]).filter((s) => !s.of || ids.has(s.of));
  const referenced = new Set(thermal.filter((s) => s.of).map((s) => s.of as string));
  const coatRef = thermal.find((s) => s.of === "coat")!;
  const handRef = thermal.find((s) => /^th-hand-/.test(s.id)) ?? { region: "hand-skin" };
  if (!referenced.has("coat-waist")) thermal.splice(thermal.indexOf(coatRef) + 1, 0, { ...coatRef, id: "th-coat-waist", of: "coat-waist" });
  for (const s of volume.body as Json[]) if (/^hand-[lr]/.test(s.id) && !referenced.has(s.id)) thermal.push({ ...handRef, id: `th-${s.id}`, of: s.id });
  if (shells.length > 0) {
    const [waist, chest] = coat(scale, coatOld);
    for (const l of shells) thermal.push(shell(chest!, l.id, l.t, l.region, l.layer, l.tone), shell(waist!, l.id.replace("th-coat-", "th-coat-waist-"), l.t, l.region, l.layer, l.tone));
  }
  volume.thermal = thermal;

  /* 2D: shapes keep their place, their frames follow the bones */
  const geometry = asset.representations[scale].geometry as Record<string, Json[]>;
  const low = (lumbarShapes[scale] = new Set<string>());
  for (const sh of geometry.elevation!) {
    if (sh.bone === "neck") sh.points = (sh.points as number[][]).map((p) => pt(NECK_MOVE, p));
    if (sh.bone !== "torso") continue;
    const u = (sh.points as number[][]).reduce((a, p) => a + p[0]!, 0) / sh.points.length;
    if (sh.id !== "coat" && u < LUMBAR) { sh.bone = "lumbar"; low.add(sh.id); }
    else sh.points = (sh.points as number[][]).map((p) => pt(THORAX_MOVE, p));
  }
  for (const sh of geometry.front!) {
    if (/^(arm|forearm|hand)-[lr]$/.test(sh.bone)) sh.points = (sh.points as number[][]).map((p) => [p[0]!, r5(p[1]! - DROP)]);
    if (sh.bone !== "torso" || sh.id === "coat" || sh.id === "coat-front-edge") continue;
    const y = (sh.points as number[][]).reduce((a, p) => a + p[1]!, 0) / sh.points.length;
    if (low.has(sh.id) || y < 1.239) { sh.bone = "lumbar"; low.add(sh.id); }
  }
  for (const sh of geometry.plan!) if (sh.bone === "torso" && low.has(sh.id)) sh.bone = "lumbar";
  console.log(`[DEBUG] 1:${scale} body ${volume.body.length} thermal ${volume.thermal.length} structure ${volume.structure.length} lumbar shapes ${[...low].join(" ")}`);
}

{
  const st50 = asset.volumes["50"].structure as Json[];
  const cl = st50.find((s) => s.id === "sk-clavicle-l");
  const il = st50.find((s) => s.id === "sk-ilium-l");
  if (cl.bone === "torso") {
    const world = (p: number[], y0: number): number[] => [r5(-p[1]!), r5(y0 + p[0]!), r5(p[2]!)];
    const [sc, ac] = [world(cl.sections[0].at, 0.959 + 0.1 + LUMBAR), world(cl.sections[cl.sections.length - 1].at, 0.959 + 0.1 + LUMBAR)];
    const si = world(il.sections[0].at, 0.959);
    const hip = [0, 0.959, 0.0895];
    const d = (a: number[], b: number[]): number[] => a.map((x, i) => r5(b[i]! - x));
    console.log(`SOLVE clavicle-l --at "${sc}" --u "${d(sc, ac)}" --w "-1,0,0" length ${Math.hypot(...d(sc, ac)).toFixed(5)}`);
    console.log(`SOLVE ilium-l --at "${si}" --u "${d(si, hip)}" --w "-1,0,0" length ${Math.hypot(...d(si, hip)).toFixed(5)}`);
  }
}
console.log(`[DEBUG] neck ${NECK_LEN} m at ${NECK_DEG} deg, grip point ${GRIP_POINT.map((x) => x.toFixed(3))}, bones ${asset.rig.bones.length}`);
writeJsonFile(join(GENERATED, "rework-walking-commuter", "🔣️.json"), doc);
