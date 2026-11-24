/** 🔦️ The extents one subject of the matrix draws for single bones: `bun probe.ts <id|file.json> <scale> <subject text|default> <bone>…` evaluates the first subject whose id contains the text (`default` is the default pose) and prints, per bone and shape, the elevation x and y, the front x (lateral) and y, and the plan x and y (lateral) ranges in metres, then the three whole-figure extents. Run it from the suite folder `T`. */
import { join } from "node:path";
import { S, readDoc } from "../MR/lib.ts";

const T = join(S, "🧪️tests", "🔭️multiview-coverage");
const [arg, key, text, ...bones] = process.argv.slice(2);
const engine = await import(join(T, "🧰️.ts"));
const { evaluate } = await import(join(S, "🔨️modules", "🧮️evaluation", "🟦️.ts"));
const asset = readDoc(arg!).doc.asset;
const all = engine.subjects(asset, Number(key));
const subject = text === "default" ? all.find((candidate: any) => candidate.isDefault) : all.find((candidate: any) => candidate.id.includes(text!));
if (subject === undefined) throw new Error(`no subject "${text}" among ${all.length}: ${all.slice(0, 12).map((candidate: any) => candidate.id).join(" | ")}`);
const state = evaluate(asset, subject.query, engine.scaleSelector(Number(key)));
const range = (rings: any[], axis: "x" | "y"): string => {
  const values = rings.flat().map((p: any) => p[axis] as number);
  return values.length === 0 ? "[ - ]" : `[${Math.min(...values).toFixed(3)}, ${Math.max(...values).toFixed(3)}]`;
};
console.log(`${asset.id} 1:${key} ${subject.id}`);
for (const view of ["elevation", "front", "plan"]) {
  for (const placed of state.shapes[view]) if (bones.includes(placed.shape.bone) || bones.includes("*")) console.log(`  ${view.padEnd(9)} ${placed.shape.bone}/${placed.shape.id}: x ${range(placed.rings, "x")} y ${range(placed.rings, "y")}`);
  const rings = state.shapes[view].filter((placed: any) => placed.shape.role !== "ground").flatMap((placed: any) => placed.rings);
  console.log(`  ${view.padEnd(9)} whole: x ${range(rings, "x")} y ${range(rings, "y")}`);
}
for (const [id, anchor] of Object.entries(state.anchors ?? {}) as [string, any][]) if (bones.includes(`@${id}`)) console.log(`  anchor ${id}: ${JSON.stringify(anchor)}`);
