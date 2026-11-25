/** 📷️ The depth `shapeDepth` ranks the extruded solid shapes of a view by, beside what the other views draw of the same shape: for a plan shape the top of its outline in the side elevation (same id and bone), for a front shape the foremost point of that outline, for an elevation shape the smallest lateral position of its front outline, negated (the right of the asset faces the viewer).
 * Also prints the paint order of the view and a digest of all evaluated rings (the outlines must not change with the depth cure).
 * usage: ROOT=<mirror> bun depth-probe.ts used-classical-guitar 50 stored plan
 */
import { createHash } from "node:crypto";
import { mod, readAsset } from "./lib.ts";

const { evaluate, shapeDepth } = await mod("🔨️modules/🧮️evaluation/🟦️.ts");
const [id, scale, pose, view] = process.argv.slice(2);
const asset = readAsset(id!);
const state = evaluate(asset, { kind: "pose", pose }, { kind: "scale", scale: { denominator: Number(scale) } });
const other = view === "elevation" ? "front" : "elevation";
const seen = (shape: any): number | undefined => {
  const twin = state.shapes[other].find((e: any) => e.shape.id === shape.id && e.shape.bone === shape.bone);
  const points = twin?.rings.flat() ?? [];
  if (points.length === 0) return undefined;
  return view === "plan" ? Math.max(...points.map((p: any) => p.y)) : view === "front" ? Math.max(...points.map((p: any) => p.x)) : -Math.min(...points.map((p: any) => p.x));
};
let [count, worst] = [0, 0];
for (const evaluated of state.shapes[view]) {
  const { shape } = evaluated;
  if (shape.solid?.kind !== "extruded") continue;
  const depth = shapeDepth(state, shape, view);
  const reference = seen(shape);
  if (reference !== undefined) {
    count++;
    worst = Math.max(worst, Math.abs(depth - reference));
  }
  if (process.env.ALL || (reference !== undefined && Math.abs(depth - reference) > 0.002)) console.log(`${shape.bone}/${shape.id} sweep ${shape.solid.sweep}: shapeDepth ${depth.toFixed(4)} m, the ${other} outline of the shape reaches ${reference === undefined ? "n/a" : reference.toFixed(4)} m`);
}
const digest = createHash("sha256").update(JSON.stringify(["elevation", "front", "plan"].map((v) => state.shapes[v].map((e: any) => e.rings)))).digest("hex").slice(0, 16);
console.log(`${id} 1:${scale} ${pose} ${view}: ${count} extruded solid shapes with a twin in the ${other}; largest |shapeDepth - twin outline| ${(worst * 1000).toFixed(1)} mm; ring digest ${digest}`);
