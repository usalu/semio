/** 📍️ Where an anchor is in the poses: `bun where.ts <id|file.json> <anchor id> [bone prefix]` (from `T`) prints, per pose, the world position of the anchor on its bone and the nearest bones whose id starts with the prefix (default: every bone) with the anchor's coordinates in their frames, to rebind an anchor to the bone that draws its place (D45). */
import { readDoc } from "../MR/lib.ts";
import { subjectsOf, toWorld, type Frame, type V3 } from "./solid.ts";

const [file, anchorId, prefix = ""] = process.argv.slice(2);
const asset = readDoc(file!).doc.asset;
const anchor = asset.anchors.find((candidate: any) => candidate.id === anchorId);
const poses = new Set(asset.poses.map((pose: any) => `pose ${pose.id}`));
const local = (f: Frame, p: V3): V3 => {
  const d: V3 = [p[0] - f.o[0], p[1] - f.o[1], p[2] - f.o[2]];
  const dot = (a: V3, b: V3): number => a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
  return [dot(d, f.u), dot(d, f.v), dot(d, f.w)];
};
for (const subject of subjectsOf(asset)) {
  if (!poses.has(subject.id) && !subject.isDefault) continue;
  const world = toWorld(subject.frames[anchor.bone]!, [anchor.x, anchor.y, anchor.z ?? 0]);
  const near = Object.entries(subject.frames)
    .filter(([bone]) => bone.startsWith(prefix) && bone !== anchor.bone)
    .map(([bone, frame]) => ({ bone, at: local(frame, world) }))
    .sort((a, b) => Math.hypot(...a.at) - Math.hypot(...b.at))
    .slice(0, 4);
  console.log(`${subject.id}${subject.isDefault ? " (default)" : ""}: world ${world.map((value) => value.toFixed(3)).join(", ")}; ${near.map((entry) => `${entry.bone} (${entry.at.map((value) => value.toFixed(3)).join(", ")})`).join("; ")}`);
}
