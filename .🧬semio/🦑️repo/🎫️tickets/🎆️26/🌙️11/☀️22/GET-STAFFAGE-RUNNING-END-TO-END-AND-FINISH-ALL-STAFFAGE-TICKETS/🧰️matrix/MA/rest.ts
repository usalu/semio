/** 🛏️ The rest drawing of single shapes: `bun rest.ts <id|file.json> <scale> <shape-id>…` prints for each shape id and each view the bone and the extent of what the evaluator draws in the default pose (x and y range in paper metres of the view). Run it from `T`. */
import { join } from "node:path";
import { S, readDoc } from "../MR/lib.ts";

const { evaluate } = await import(join(S, "🔨️modules", "🧮️evaluation", "🟦️.ts"));
const engine = await import(join(S, "🧪️tests", "🔭️multiview-coverage", "🧰️.ts"));
const [arg, scale, ...ids] = process.argv.slice(2) as [string, string, ...string[]];
const asset = readDoc(arg).doc.asset;
const pose = ids[0]?.startsWith("pose=") ? ids.shift()!.slice(5) : asset.presentation.defaultPose;
const state = evaluate(asset, { kind: "pose", pose }, engine.scaleSelector(Number(scale)));
for (const view of ["elevation", "front", "plan"])
  for (const entry of state.shapes[view]) {
    if (ids.length > 0 && !ids.some((id) => entry.shape.id === id || entry.shape.bone === id)) continue;
    const points = entry.rings.flat();
    const range = (axis: "x" | "y"): string => (points.length === 0 ? "-" : `[${Math.min(...points.map((p: any) => p[axis])).toFixed(3)}, ${Math.max(...points.map((p: any) => p[axis])).toFixed(3)}]`);
    console.log(`${view} ${entry.shape.id} (${entry.shape.bone}) x ${range("x")} y ${range("y")} ${entry.shape.solid ? `${entry.shape.solid.kind}${entry.shape.solid.sweep ?? entry.shape.solid.axis}` : entry.shape.followTip ? "followTip" : "flat"}`);
  }
