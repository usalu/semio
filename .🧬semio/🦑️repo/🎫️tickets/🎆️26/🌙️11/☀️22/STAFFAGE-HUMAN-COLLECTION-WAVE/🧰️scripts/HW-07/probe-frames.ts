/** 🧪️ Probe: prints the solved frames (origin, u, v, w) of every bone of a human in a pose (or rest). bun probe-frames.ts <slug> [pose] */
import { solveChain } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts";
import { samplePose } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎞️animation/🟦️.ts";
import { loadHuman } from "./lib.ts";
const a = (await loadHuman(process.argv[2]!)).asset;
const pose = process.argv[3];
const f = (v: number[]) => v.map((x) => x.toFixed(3).padStart(7)).join("");
let frames;
if (pose) { const s: any = samplePose(a, a.poses.find((p: any) => p.id === pose)); console.log(JSON.stringify(s.root)); frames = solveChain(a.rig, s.offsets, s.root); }
else frames = solveChain(a.rig, {}, { x: 0, y: 0, angleDeg: 0 });
for (const [id, fr] of Object.entries(frames) as any) console.log(id.padEnd(14), "o", f(fr.frame.o), " u", f(fr.frame.u), " v", f(fr.frame.v), " w", f(fr.frame.w));
