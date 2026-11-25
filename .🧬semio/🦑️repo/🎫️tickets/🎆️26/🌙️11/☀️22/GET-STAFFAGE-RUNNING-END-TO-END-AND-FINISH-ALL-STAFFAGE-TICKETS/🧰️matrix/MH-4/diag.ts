/** 🧪️ The frame of a bone at rest and in the default pose beside the flat shapes it draws: `bun diag.ts <id|file.json> <scale> <bone>` prints the axes u, v, w (asset x ahead, y up, z left) and the origin of the bone at rest and in the default pose, and for every shape of the bone in the three views its kind and the box of its points, so the picture a sweep along a bone axis has in a view can be read. */
import { join } from "node:path";
import { S, readDoc } from "../MR/lib.ts";

const { restFrames, solveChain } = await import(join(S, "🔨️modules", "🦴️rig", "🟦️.ts"));
const [arg, scale, bone] = process.argv.slice(2);
const asset = readDoc(arg!).doc.asset;
const pose = asset.poses.find((candidate: any) => candidate.id === asset.presentation.defaultPose);
const show = (label: string, frame: any): void => console.log(`${label} o ${frame.o.map((x: number) => x.toFixed(3))} u ${frame.u.map((x: number) => x.toFixed(3))} v ${frame.v.map((x: number) => x.toFixed(3))} w ${frame.w.map((x: number) => x.toFixed(3))}`);
show(`${asset.id} ${bone} rest   `, restFrames(asset.rig)[bone!].frame);
show(`${asset.id} ${bone} ${pose.id}`, solveChain(asset.rig, pose.bones, { x: 0, y: 0, angleDeg: 0 })[bone!].frame);
for (const view of ["elevation", "front", "plan"]) for (const shape of asset.representations[scale!].geometry[view]) {
  if (shape.bone !== bone) continue;
  const points = (shape.points ?? []) as number[][];
  const box = points.length === 0 ? "" : `a [${Math.min(...points.map((p) => p[0]!)).toFixed(3)}, ${Math.max(...points.map((p) => p[0]!)).toFixed(3)}] b [${Math.min(...points.map((p) => p[1]!)).toFixed(3)}, ${Math.max(...points.map((p) => p[1]!)).toFixed(3)}]`;
  console.log(`   ${view} ${shape.id} ${shape.kind} ${shape.solid ? JSON.stringify(shape.solid) : "flat"} ${points.length} points ${box}`);
}
