/** 🪞️ Compares the evaluated outlines of two documents of one asset in a pose (default: the default pose) at every authored scale and prints every shape whose bounding box moved by more than 0.2 mm, was added or removed: `bun restdiff.ts <before.json|id> <after.json|id> [pose]`. A repair that only makes flat shapes solid prints nothing for the rest pose. */
import { join } from "node:path";
import { S, catalogFile } from "./files.ts";

const { evaluate } = await import(join(S, "🔨️modules", "🧮️evaluation", "🟦️.ts"));
const [a, b, poseArg] = process.argv.slice(2);
const load = async (arg: string): Promise<any> => (await Bun.file(arg.endsWith(".json") ? arg : catalogFile(arg)).json()).asset;
const [before, after] = [await load(a!), await load(b!)];
const boxes = (asset: any, key: string, pose: string): Map<string, number[]> => {
  const state = evaluate(asset, { kind: "pose", pose }, { kind: "scale", scale: { denominator: Number(key) } });
  const out = new Map<string, number[]>();
  for (const view of ["elevation", "front", "plan"] as const)
    for (const item of state.shapes[view]) {
      const points = item.rings.flat();
      if (points.length === 0) continue;
      out.set(`${view} ${item.shape.bone} ${item.shape.id}`, [Math.min(...points.map((p: any) => p.x)), Math.max(...points.map((p: any) => p.x)), Math.min(...points.map((p: any) => p.y)), Math.max(...points.map((p: any) => p.y))]);
    }
  return out;
};
let count = 0;
for (const key of Object.keys(after.representations)) {
  if (after.representations[key].geometry === undefined) continue;
  const pose = poseArg ?? after.presentation.defaultPose;
  const [x, y] = [boxes(before, key, pose), boxes(after, key, pose)];
  for (const name of new Set([...x.keys(), ...y.keys()])) {
    const [p, q] = [x.get(name), y.get(name)];
    const moved = p === undefined || q === undefined ? Number.POSITIVE_INFINITY : Math.max(...p.map((v, i) => Math.abs(v - q[i]!)));
    if (moved <= 0.0002) continue;
    count++;
    console.log(`1:${key} ${name}: ${p === undefined ? "added" : q === undefined ? "removed" : `${(moved * 1000).toFixed(1)} mm`} ${p?.map((v) => v.toFixed(4)).join(" ") ?? ""} -> ${q?.map((v) => v.toFixed(4)).join(" ") ?? ""}`);
  }
}
console.log(`${after.id}: ${count} outlines differ`);
