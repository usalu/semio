/** 🧪️ Probe: world positions of anchors and bone origins of an asset file in a pose (after grounding). bun probe-pose.ts <file> <pose> */
import { samplePose } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎞️animation/🟦️.ts";
import { solveChain } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts";
import { anchorWorld, comOf, framesOf } from "./lib.ts";
const a = (await Bun.file(process.argv[2]!).json()).asset;
const s: any = samplePose(a, a.poses.find((p: any) => p.id === process.argv[3]));
const solved: any = solveChain(a.rig, s.offsets, s.root);
console.log("root", JSON.stringify(s.root));
for (const an of a.anchors) console.log(an.id.padEnd(12), anchorWorld(a, solved, an.id).map((x: number) => x.toFixed(3)).join(" "));
const { frames } = framesOf(a, s.offsets, s.root);
console.log("com", comOf(a, frames).map((x) => x.toFixed(3)).join(" "));
