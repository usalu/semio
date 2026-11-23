/** 🔁️ Closes the loop seam of a planted gait: the root follows the trailing contact before the seam and the striking contact after it, and because the trailing span is split at the seam the two are not crossfaded, so a striking foot that still swings forward at the seam jumps the root velocity (an acceleration of that jump times 240 Hz). Solves the angles of two bones of the swing leg in one key before the seam so that the root velocity is continuous across the seam while the striking anchor keeps its height above the trailing one in that key, then reports the planted travel of one cycle (the `advanceM` to write): `bun <this file> <asset json> <clip> <key index> <bone a> <bone b> <striking anchor> <trailing anchor>` (a key without own `bones` changes its pose). Prints values to write by hand (four decimals for angles, six for the advance); it writes nothing. */
import { readFileSync } from "node:fs";
import { sampleClip } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎞️animation/🟦️.ts";
import { evaluate } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";

const [file, clipId, keyIndex, boneA, boneB, striking, trailing] = process.argv.slice(2) as [string, string, string, string, string, string, string];
const source = JSON.parse(readFileSync(file, "utf8")).asset;
const clipOf = (asset: typeof source) => asset.clips.find((clip: { id: string }) => clip.id === clipId);
const bonesOf = (asset: typeof source) => {
  const key = clipOf(asset).keys[Number(keyIndex)];
  return key.bones ?? asset.poses.find((pose: { id: string }) => pose.id === key.pose).bones;
};
const start: [number, number] = [bonesOf(source)[boneA].angleDeg, bonesOf(source)[boneB].angleDeg];
const EPS = 1e-4;
const measure = ([a, b]: readonly [number, number]): { jump: number; lift: number; travel: number } => {
  const asset = structuredClone(source);
  const clip = clipOf(asset);
  const key = clip.keys[Number(keyIndex)];
  bonesOf(asset)[boneA].angleDeg = a;
  bonesOf(asset)[boneB].angleDeg = b;
  const x = (t: number): number => sampleClip(asset, clip, t).root.x;
  const T = clip.durationS;
  const before = (x(T - EPS) - x(T - 2 * EPS)) / EPS;
  const after = (x(2 * EPS) - x(EPS)) / EPS;
  const state = evaluate(asset, { kind: "clip", clip: clipId, timeS: key.t === T ? T - 1e-9 : key.t });
  return { jump: after - before, lift: state.anchors[striking]!.at.y - state.anchors[trailing]!.at.y, travel: x(T - 1e-9) - x(0) };
};
const initial = measure(start);
console.log("before", start, initial, "advanceM", clipOf(source).advanceM);
let angles: [number, number] = [...start];
for (let i = 0; i < 12; i++) {
  const now = measure(angles);
  const f = [now.jump, now.lift - initial.lift];
  if (Math.abs(f[0]!) < 1e-7 && Math.abs(f[1]!) < 1e-8) break;
  const h = 1e-3;
  const da = measure([angles[0] + h, angles[1]]);
  const db = measure([angles[0], angles[1] + h]);
  const j = [[(da.jump - now.jump) / h, (db.jump - now.jump) / h], [(da.lift - now.lift) / h, (db.lift - now.lift) / h]];
  const det = j[0]![0]! * j[1]![1]! - j[0]![1]! * j[1]![0]!;
  angles = [angles[0] - (f[0]! * j[1]![1]! - f[1]! * j[0]![1]!) / det, angles[1] - (j[0]![0]! * f[1]! - j[1]![0]! * f[0]!) / det];
}
const rounded: [number, number] = [Math.round(angles[0] * 1e4) / 1e4, Math.round(angles[1] * 1e4) / 1e4];
const final = measure(rounded);
console.log("after", { [boneA]: rounded[0], [boneB]: rounded[1] }, final, "advanceM", Math.round(final.travel * 1e6) / 1e6);
