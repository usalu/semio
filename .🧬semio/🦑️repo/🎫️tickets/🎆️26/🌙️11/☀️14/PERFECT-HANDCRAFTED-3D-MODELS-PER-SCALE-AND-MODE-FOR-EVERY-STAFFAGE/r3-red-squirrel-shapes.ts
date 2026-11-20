/** 🔎️ Round 3 red squirrel: the extent of every placed 2D shape of a pose in the three views, from the product's `evaluate` (millimetres). Usage: bun r3-red-squirrel-shapes.ts <file> <scale> [pose] [--all] */
import { readFileSync } from "node:fs";
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
const [file, scale, pose] = process.argv.slice(2);
const asset: any = JSON.parse(readFileSync(file!, "utf8")).asset;
const state: any = evaluate(asset, { kind: "pose", pose: pose !== undefined && !pose.startsWith("--") ? pose : asset.presentation.defaultPose } as never, { kind: "scale", scale: { denominator: Number(scale) } } as never);
if (process.argv.includes("--keys")) console.log(Object.keys(state.shapes), JSON.stringify(state.bounds));
for (const viewpoint of ["elevation", "front", "plan"]) {
  const total = [Infinity, Infinity, -Infinity, -Infinity];
  for (const s of state.shapes[viewpoint] ?? []) {
    const pts = s.rings.flat();
    const b = [Math.min(...pts.map((p: any) => p.x)), Math.min(...pts.map((p: any) => p.y)), Math.max(...pts.map((p: any) => p.x)), Math.max(...pts.map((p: any) => p.y))];
    for (let k = 0; k < 2; k++) { total[k] = Math.min(total[k]!, b[k]!); total[k + 2] = Math.max(total[k + 2]!, b[k + 2]!); }
    if (process.argv.includes("--all") || b.some((v) => Math.abs(v) > 1)) console.log(viewpoint, s.shape.id, b.map((v) => (v * 1000).toFixed(1)).join(" "));
  }
  console.log(viewpoint, "TOTAL", total.map((v) => (v * 1000).toFixed(1)).join(" "), "size", ((total[2]! - total[0]!) * 1000).toFixed(1), ((total[3]! - total[1]!) * 1000).toFixed(1));
}
