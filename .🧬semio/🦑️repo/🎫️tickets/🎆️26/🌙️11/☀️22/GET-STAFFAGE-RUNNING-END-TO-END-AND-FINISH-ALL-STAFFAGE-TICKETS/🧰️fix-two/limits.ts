/** 📏️ Samples one clip of an asset every 1/480 s and prints, per bone, the range of the splined joint offsets (without the breathing motion) against the joint limits (a spline may leave the limits between keys, where the rig clamps silently) and, per contact anchor, the lowest height it reaches while it is not in contact.
 *
 * `bun <this file> <asset json> <clip id>`
 *
 * @see ./scan.ts — the forces on the same grid */
import { readFileSync } from "node:fs";
import { sampleClip } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎞️animation/🟦️.ts";
import { apply, solveChain } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts";

const asset = JSON.parse(readFileSync(process.argv[2]!, "utf8")).asset;
const clip = asset.clips.find((candidate: { id: string }) => candidate.id === process.argv[3]);
const range: Record<string, [number, number]> = {};
const lowest: Record<string, [number, number]> = {};
for (let i = 0; i <= Math.round(clip.durationS * 480); i++) {
  const timeS = Math.min(i / 480, clip.durationS);
  const sample = sampleClip(asset, clip, timeS);
  for (const [id, offset] of Object.entries(sample.offsets) as [string, { angleDeg: number }][]) {
    const r = (range[id] ??= [Infinity, -Infinity]);
    r[0] = Math.min(r[0], offset.angleDeg);
    r[1] = Math.max(r[1], offset.angleDeg);
  }
  const frames = solveChain(asset.rig, sample.offsets, sample.root);
  for (const anchor of asset.anchors) {
    if (anchor.kind !== "contact" || sample.contacts.includes(anchor.id)) continue;
    const y = apply(frames[anchor.bone]!.affine, anchor.x, anchor.y).y;
    if (y < (lowest[anchor.id]?.[0] ?? Infinity)) lowest[anchor.id] = [y, timeS];
  }
}
for (const bone of asset.rig.bones) {
  const [low, high] = range[bone.id]!;
  const out = low < (bone.joint.minDeg ?? -Infinity) - 1e-9 || high > (bone.joint.maxDeg ?? Infinity) + 1e-9;
  if (out || high - low > 1e-9) console.log(`${out ? "OUTSIDE " : ""}${bone.id}: ${low.toFixed(4)}..${high.toFixed(4)} within ${bone.joint.minDeg ?? "-inf"}..${bone.joint.maxDeg ?? "inf"}`);
}
for (const [id, [y, timeS]] of Object.entries(lowest)) console.log(`${id} out of contact: lowest ${(y * 1000).toFixed(3)} mm at ${timeS.toFixed(4)} s`);
