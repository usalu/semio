/** 🦴️ Dumps the rest frames and the frames of every pose (offsets and root of `samplePose`) of one asset file as JSON.
 *  bun frames.ts <asset.json> [pose ...] */
import { samplePose } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";
import { restFrames, solveChain } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts";

const [file, ...wanted] = process.argv.slice(2);
const asset = (await Bun.file(file!).json()).asset;
const pick = (frames: Record<string, { frame: { o: number[]; u: number[]; v: number[]; w: number[] } }>) => Object.fromEntries(Object.entries(frames).map(([id, f]) => [id, f.frame]));
const out: Record<string, unknown> = { rest: pick(restFrames(asset.rig)) };
for (const pose of asset.poses) {
  if (wanted.length && !wanted.includes(pose.id)) continue;
  const sample = samplePose(asset, pose);
  out[pose.id] = { root: sample.root, frames: pick(solveChain(asset.rig, sample.offsets, sample.root)) };
}
console.log(JSON.stringify(out));
