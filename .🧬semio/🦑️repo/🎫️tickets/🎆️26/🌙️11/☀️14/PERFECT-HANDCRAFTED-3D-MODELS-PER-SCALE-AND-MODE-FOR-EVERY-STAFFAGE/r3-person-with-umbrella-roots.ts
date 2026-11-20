/** ☂️ Root fixes of `humans/person-with-umbrella` outside `volumes` and `representations`: the rig (limb joints after Winter 2009, shoulder and hip spacing, a heel under the ankle, eight ribs, eight stretchers and the runner of a real stick umbrella), the anchors, and every pose and clip key re-solved (the right hand holds the shaft a hand's breadth ahead of the chest, the left arm hangs with the elbow and wrist carried a little outwards of the shoulder joint, the ribs and stretchers fold as a closed linkage).
 *
 * Usage: `bun r3-person-with-umbrella-roots.ts` writes `🗑️generated/r3-person-with-umbrella/roots.json` (members `rig`, `anchors`, `poses`, `clips`, `summary`; spliced by `r3-person-with-umbrella-apply.py`). Every value is absolute, so the script may run on the original or on the corrected asset.
 * @see ./r3-humans-person-with-umbrella.md */
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { rigAt } from "./r3-lib.ts";
import { entryOf, type Json } from "./volumes-lib.ts";

const { asset: catalog } = entryOf("humans/person-with-umbrella");
const asset = JSON.parse(JSON.stringify(catalog)) as Json;
const rad = (d: number): number => (d * Math.PI) / 180;
const deg = (r: number): number => (r * 180) / Math.PI;
const r4 = (x: number): number => Math.round(x * 1e4) / 1e4;
const r2 = (x: number): number => Math.round(x * 100) / 100;
const wrap = (d: number): number => ((((d + 180) % 360) + 360) % 360) - 180;
const bone = (id: string): Json => asset.rig.bones.find((b: Json) => b.id === id);

/** 📐 The umbrella: rib chord, bow of the tensioned rib, where the stretcher meets the rib, stretcher length, and the polar angle of the ribs from the shaft (open, half folded, furled). */
export const RIB = { chord: 0.56, bow: 0.028, joint: 0.25, notch: 0.78, openDeg: 66, halfDeg: 36, furledDeg: 5, runnerOpen: 0.16 } as const;
export const AZIMUTH: Record<string, number> = { front: 0, "front-l": 45, l: 90, "back-l": 135, back: 180, "back-r": 225, r: 270, "front-r": 315 };
const typeA = (key: string): boolean => ["front", "front-l", "front-r", "l"].includes(key);
/** 🔩 The point of the rib where the stretcher is pinned, in the plane of the rib: [radial, along the shaft towards the ferrule], for the polar angle `theta`. */
const pin = (theta: number): [number, number] => {
  const b = 4 * RIB.bow * (RIB.joint / RIB.chord) * (1 - RIB.joint / RIB.chord);
  return [RIB.joint * Math.sin(rad(theta)) + b * Math.cos(rad(theta)), -RIB.joint * Math.cos(rad(theta)) + b * Math.sin(rad(theta))];
};
const P0 = pin(RIB.openDeg);
export const STRETCHER = r4(Math.hypot(P0[0], P0[1] + RIB.runnerOpen));
/** 🛝 The runner travel (negative: towards the handle) and the angle of the stretcher from the shaft for the polar angle `theta` of the ribs. */
function linkage(theta: number): { travel: number; gamma: number } {
  const [r, a] = pin(theta);
  const d = -a + Math.sqrt(STRETCHER * STRETCHER - r * r);
  return { travel: r4(RIB.runnerOpen - d), gamma: deg(Math.atan2(r, a + d)) };
}
/** 🧭 The offsets that point a bone with the rest angle `rest` to the polar angle `polar` from the shaft axis (0 = towards the ferrule) at the azimuth `az` (0 = ahead, 90 = left). */
function aim(rest: number, polar: number, az: number): Json {
  const beta = Math.asin(Math.sin(rad(polar)) * Math.sin(rad(az)));
  const a = deg(Math.atan2(-Math.sin(rad(polar)) * Math.cos(rad(az)), Math.cos(rad(polar))));
  return { angleDeg: r2(wrap(a - rest)), abductionDeg: r2(deg(beta)) };
}
/** 🌂 The offsets of the eight ribs, the eight stretchers and the runner for the polar angle `theta` of the ribs. */
export function canopyOffsets(theta: number): Json {
  const out: Json = {};
  const { travel, gamma } = linkage(theta);
  for (const [key, az] of Object.entries(AZIMUTH)) out[`rib-${key}`] = aim(typeA(key) ? -114 : 114, 180 - theta, az);
  out.runner = { dx: travel };
  for (const [key, az] of Object.entries(AZIMUTH)) out[`stretcher-${key}`] = aim(typeA(key) ? -70 : 70, gamma, az);
  return out;
}

/* ---------------------------------------------------------------- rig */
const scaleMass = (b: Json, length: number): void => {
  if (b.mass) b.mass.comX = r4((b.mass.comX / b.lengthM) * length);
  b.lengthM = length;
};
const LIMB = { shoulderX: 0.3886, shoulderZ: 0.17, upper: 0.316, fore: 0.248, hand: 0.184, hipZ: 0.088, shank: 0.4185, ankle: 0.086, head: 0.2482, grip: 0.075, elbowOut: 0.035, wristOut: 0.02, palm: 0.026 } as const;
if (bone("head").lengthM !== LIMB.head) {
  for (const side of ["l", "r"]) {
    const s = side === "l" ? 1 : -1;
    Object.assign(bone(`upper-arm-${side}`), { x: LIMB.shoulderX, z: s * LIMB.shoulderZ });
    scaleMass(bone(`upper-arm-${side}`), LIMB.upper);
    bone(`forearm-${side}`).x = LIMB.upper;
    scaleMass(bone(`forearm-${side}`), LIMB.fore);
    bone(`hand-${side}`).x = LIMB.fore;
    scaleMass(bone(`hand-${side}`), LIMB.hand);
    bone(`thigh-${side}`).z = s * LIMB.hipZ;
    scaleMass(bone(`shank-${side}`), LIMB.shank);
    bone(`foot-${side}`).x = LIMB.shank;
    bone(`foot-${side}`).mass.comY = -0.043;
    bone(`toe-${side}`).y = -LIMB.ankle;
  }
  scaleMass(bone("head"), LIMB.head);
}
bone("umbrella").x = LIMB.grip;
bone("umbrella").z = LIMB.palm;
for (const side of ["l", "r"]) {
  bone(`forearm-${side}`).z = (side === "l" ? 1 : -1) * LIMB.elbowOut;
  bone(`hand-${side}`).z = (side === "l" ? 1 : -1) * LIMB.wristOut;
}
asset.rig.bones = asset.rig.bones.filter((b: Json) => !/^(rib-|stretcher-|runner)/.test(b.id));
const TITLES: Record<string, [string, string]> = { front: ["front", "vorn"], "front-l": ["front left", "vorn links"], l: ["left", "links"], "back-l": ["rear left", "hinten links"], back: ["rear", "hinten"], "back-r": ["rear right", "hinten rechts"], r: ["right", "rechts"], "front-r": ["front right", "vorn rechts"] };
for (const key of Object.keys(AZIMUTH)) {
  asset.rig.bones.push({ id: `rib-${key}`, parent: "umbrella", title: { en: `Rib ${TITLES[key]![0]} with its canopy panel`, de: `Schirmrippe ${TITLES[key]![1]} mit ihrer Stoffbahn` }, x: RIB.notch, y: 0, angleDeg: typeA(key) ? -114 : 114, lengthM: RIB.chord, joint: { type: "revolute", minDeg: -180, maxDeg: 180 }, mass: { kg: 0.03, comX: 0.3, comY: 0, distribution: "point", source: "authored-estimate" } });
}
asset.rig.bones.push({ id: "runner", parent: "umbrella", title: { en: "Runner on the shaft", de: "Schieber auf dem Schirmstock" }, x: r4(RIB.notch - RIB.runnerOpen), y: 0, angleDeg: 0, lengthM: 0.03, joint: { type: "prismatic", axisDeg: 0, minM: -0.36, maxM: 0.01 } });
for (const key of Object.keys(AZIMUTH)) {
  asset.rig.bones.push({ id: `stretcher-${key}`, parent: "runner", title: { en: `Stretcher ${TITLES[key]![0]}`, de: `Schirmstrebe ${TITLES[key]![1]}` }, x: 0, y: 0, angleDeg: typeA(key) ? -70 : 70, lengthM: STRETCHER, joint: { type: "revolute", minDeg: -180, maxDeg: 180 } });
}

/* ---------------------------------------------------------------- anchors */
const anchor = (id: string): Json => asset.anchors.find((a: Json) => a.id === id);
for (const side of ["l", "r"]) {
  Object.assign(anchor(`contact-${side}`), { x: 0.127, y: -LIMB.ankle });
  Object.assign(anchor(`heel-${side}`), { x: -0.035, y: -LIMB.ankle });
  anchor(`grip-${side}`).x = LIMB.grip;
}
Object.assign(anchor("mouth"), { x: 0.063, y: -0.1 });
Object.assign(anchor("nostrils"), { x: 0.083, y: -0.108 });
Object.assign(anchor("crown"), { x: LIMB.head, y: -0.01 });

/* ---------------------------------------------------------------- poses */
const pose = (id: string): Json => asset.poses.find((p: Json) => p.id === id);
const GRIP_REL = 75;
type Hold = { shaftDeg: number; ahead: number; below: number; relDeg?: number };
/** ✊ Solves the right arm of a pose: the grip lies `ahead` of and `below` the right shoulder joint (measured in the leaning trunk), the shaft stands at `shaftDeg` in the side elevation and upright in the front elevation. */
function hold(id: string, h: Hold): void {
  const p = pose(id);
  const b = p.bones as Json;
  b["upper-arm-r"] = { angleDeg: -10 };
  b["forearm-r"] = { angleDeg: 120 };
  b["hand-r"] = { angleDeg: 0 };
  b.umbrella = { angleDeg: h.relDeg ?? GRIP_REL };
  const params: [string, string][] = [["upper-arm-r", "angleDeg"], ["forearm-r", "angleDeg"], ["hand-r", "angleDeg"]];
  const residual = (): number[] => {
    const f = rigAt(asset, 50, { kind: "pose", pose: id }).frames;
    const lean = rad(deg(Math.atan2(f.torso!.u[1], f.torso!.u[0])) - 90);
    const s = f["upper-arm-r"]!.o;
    const g = f.umbrella!.o;
    const u = f.umbrella!.u;
    const tx = s[0] + h.ahead * Math.cos(lean) + h.below * Math.sin(lean);
    const ty = s[1] + h.ahead * Math.sin(lean) - h.below * Math.cos(lean);
    return [g[0] - tx, g[1] - ty, rad(wrap(deg(Math.atan2(u[1], u[0])) - h.shaftDeg)) * 0.3];
  };
  for (let it = 0; it < 30; it++) {
    const r0 = residual();
    if (Math.hypot(...r0) < 1e-6) break;
    const J: number[][] = [];
    for (const [bn, key] of params) {
      b[bn][key] += 0.01;
      const r1 = residual();
      b[bn][key] -= 0.01;
      J.push(r1.map((x, i) => (x - r0[i]!) / 0.01));
    }
    const n = 3;
    const A = Array.from({ length: n }, (_, i) => [...Array.from({ length: n }, (_, j) => J[j]![i]!), -r0[i]!]);
    for (let c = 0; c < n; c++) {
      let piv = c;
      for (let k = c + 1; k < n; k++) if (Math.abs(A[k]![c]!) > Math.abs(A[piv]![c]!)) piv = k;
      [A[c], A[piv]] = [A[piv]!, A[c]!];
      for (let k = 0; k < n; k++) if (k !== c) { const m = A[k]![c]! / A[c]![c]!; for (let j = c; j <= n; j++) A[k]![j]! -= m * A[c]![j]!; }
    }
    params.forEach(([bn, key], i) => { b[bn][key] += Math.max(-25, Math.min(25, A[i]![n]! / A[i]![i]!)); });
  }
  for (const [bn, key] of params) b[bn][key] = r2(b[bn][key]);
  const f = rigAt(asset, 50, { kind: "pose", pose: id }).frames;
  console.log(id.padEnd(24), "residual", Math.hypot(...residual()).toExponential(1), "upper", b["upper-arm-r"].angleDeg, "elbow", b["forearm-r"].angleDeg, "wrist", b["hand-r"].angleDeg, "grip", f.umbrella!.o.map((x) => x.toFixed(3)).join(" "), "elbow at", f["forearm-r"]!.o.map((x) => x.toFixed(3)).join(" "));
}
/** 🖐️ The left arm hangs plumb beside the coat whatever the lean, the elbow a little bent. */
function hang(id: string): void {
  const p = pose(id);
  const b = p.bones as Json;
  b["upper-arm-l"] = { angleDeg: 0 };
  const f = rigAt(asset, 50, { kind: "pose", pose: id }).frames;
  b["upper-arm-l"].angleDeg = r2(wrap(-93 - deg(Math.atan2(f["upper-arm-l"]!.u[1], f["upper-arm-l"]!.u[0]))));
  b["forearm-l"] = { angleDeg: 13 };
  b["hand-l"] = { angleDeg: 6 };
}
const HOLDS: Record<string, Hold> = {
  "umbrella-up-still": { shaftDeg: 90, ahead: 0.235, below: 0.185 },
  "gust-light": { shaftDeg: 72, ahead: 0.25, below: 0.2 },
  "umbrella-tilted-to-wind": { shaftDeg: 58, ahead: 0.27, below: 0.22 },
  "gust-heavy": { shaftDeg: 50, ahead: 0.285, below: 0.235 },
  "umbrella-furled": { shaftDeg: -63, ahead: 0.09, below: 0.628, relDeg: 30 },
};
for (const [id, h] of Object.entries(HOLDS)) {
  const p = pose(id);
  for (const key of Object.keys(p.bones)) if (/^(rib-|stretcher-|runner)/.test(key)) delete p.bones[key];
  Object.assign(p.bones, canopyOffsets(id === "umbrella-furled" ? RIB.furledDeg : RIB.openDeg));
  hang(id);
  hold(id, h);
}
pose("umbrella-up-still").description = { en: "Standing upright in light rain, the right hand holds the handle a hand's breadth ahead of the chest and the shaft stands upright beside the head, the left arm hangs; a light drizzle breeze of 3 m/s loads the canopy.", de: "Aufrecht stehend im leichten Regen, die rechte Hand hält den Griff eine Handbreit vor der Brust und der Stock steht senkrecht neben dem Kopf, der linke Arm hängt; ein leichter Nieselwind von 3 m/s belastet den Schirm." };
pose("umbrella-furled").description = { en: "Rain has stopped: the umbrella is closed, the runner drawn down and the ribs folded along the shaft; it is carried in the hanging right hand, tip ahead and down like a cane.", de: "Der Regen hat aufgehört: der Schirm ist geschlossen, der Schieber heruntergezogen und die Rippen liegen am Stock; er wird in der hängenden rechten Hand getragen, die Spitze wie ein Stock nach vorn unten." };

/* ---------------------------------------------------------------- clips */
for (const clip of asset.clips as Json[]) for (const key of clip.keys as Json[]) if (key.bones && Object.keys(key.bones).some((k) => k.startsWith("rib-"))) key.bones = canopyOffsets(RIB.halfDeg);

asset.summary = { en: "A person of 1.72 m in a knee-length raincoat holding an open stick umbrella in rain and gusty wind at 8 degrees Celsius: eight ribs on eight stretchers open or furl with the runner, a wind load acts on the canopy and the person leans into the gust; four individually drawn scales.", de: "Eine 1,72 m große Person im knielangen Regenmantel mit aufgespanntem Stockschirm bei Regen und Böen und 8 Grad Celsius: Acht Rippen an acht Streben öffnen oder schließen sich mit dem Schieber, eine Windlast wirkt auf den Schirm und die Person lehnt sich in die Böe; vier einzeln gezeichnete Maßstäbe." };

if (import.meta.main) {
  const out = join(import.meta.dir, "🗑️generated", "r3-person-with-umbrella");
  mkdirSync(out, { recursive: true });
  writeFileSync(join(out, "roots.json"), JSON.stringify({ rig: asset.rig, anchors: asset.anchors, poses: asset.poses, clips: asset.clips, summary: asset.summary }));
  for (const id of ["umbrella-up-still", "umbrella-furled"]) {
    const f = rigAt(asset, 50, { kind: "pose", pose: id }).frames;
    for (const k of ["rib-front", "rib-l", "rib-back-r", "stretcher-front", "stretcher-l", "runner", "foot-l", "hand-l"]) console.log(id, k.padEnd(16), "o", f[k]!.o.map((x) => x.toFixed(3)).join(" "), "tip", f[k]!.o.map((x, i) => (x + f[k]!.u[i]! * bone(k).lengthM).toFixed(3)).join(" "));
  }
}
