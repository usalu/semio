/** 🦶️ Finds the angle of one bone in one clip key at which a contact anchor lies at the height of the lowest other contact of that instant, so the automatic ground does not lift the body when the contact set changes there: `bun <this file> <asset json> <clip> <key index> <bone> <anchor> [<pose to change with the key>]` (a key without own `bones` changes its pose). Prints the heights before and after and the angle to write by hand (four decimals); it writes nothing. */
import { readFileSync } from "node:fs";
import { evaluate } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { solveStructure } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🏗️structure/🟦️.ts";

const [file, clipId, keyIndex, bone, anchor, poseId] = process.argv.slice(2) as [string, string, string, string, string, string | undefined];
const source = JSON.parse(readFileSync(file, "utf8")).asset;
const heights = (angleDeg: number | null): Record<string, number> => {
  const asset = structuredClone(source);
  const key = asset.clips.find((clip: { id: string }) => clip.id === clipId).keys[Number(keyIndex)];
  if (angleDeg !== null) {
    if (key.bones !== undefined) key.bones[bone].angleDeg = angleDeg;
    if (poseId !== undefined || key.bones === undefined) asset.poses.find((pose: { id: string }) => pose.id === (poseId ?? key.pose)).bones[bone].angleDeg = angleDeg;
  }
  const result = solveStructure(evaluate(asset, { kind: "clip", clip: clipId, timeS: key.t }));
  return Object.fromEntries(result.reactions.map((reaction) => [reaction.anchor, reaction.at.y]));
};
const gap = (angleDeg: number | null): number => {
  const all = heights(angleDeg);
  return all[anchor]! - Math.min(...Object.entries(all).filter(([id]) => id !== anchor).map(([, y]) => y));
};
const startKey = source.clips.find((clip: { id: string }) => clip.id === clipId).keys[Number(keyIndex)];
const start = (startKey.bones ?? source.poses.find((pose: { id: string }) => pose.id === startKey.pose).bones)[bone].angleDeg as number;
console.log("before", start, heights(null), "gap", gap(null));
let [a, b] = [start - 3, start + 3];
if (Math.sign(gap(a)) === Math.sign(gap(b))) throw new Error("no sign change within 3 degrees");
for (let i = 0; i < 60; i++) {
  const mid = (a + b) / 2;
  if (Math.sign(gap(mid)) === Math.sign(gap(a))) a = mid;
  else b = mid;
}
const angle = Math.round(((a + b) / 2) * 1e4) / 1e4;
console.log("after", angle, heights(angle), "gap", gap(angle));
