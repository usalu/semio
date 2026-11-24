/** ⚡️ The model gate of MH-5: `bun mgate.ts <id|file.json> [--all] [--scales 50,100]` (from `T`) prints in seconds what the real gate needs minutes for: per authored scale every shape and its twin of the same id and bone whose extents disagree beyond the tolerance over the subjects, tier HARD when the bone has exactly one shape in both views (the hard tier of the matrix), else `twin`; `--all` lists every pair with its worst delta. Shapes the model does not cover are listed as `unmodelled`. It measures the bodies of `solid.ts`, not the evaluated drawing: the proof is `../MR/gate.ts`. Exit 1 when a HARD pair remains. */
import { readDoc } from "../MR/lib.ts";
import { PAIRS, bodyOf, restOf, subjectsOf, toleranceOf, worst, type View } from "./solid.ts";

const args = process.argv.slice(2);
const asset = readDoc(args[0]!).doc.asset;
const only = args.includes("--scales") ? args[args.indexOf("--scales") + 1]!.split(",") : undefined;
const tolerance = toleranceOf(asset);
const rest = restOf(asset);
const subjects = subjectsOf(asset);
let hard = 0;
for (const [scale, rep] of Object.entries(asset.representations) as [string, any][]) {
  if (rep.status !== "authored" || (only !== undefined && !only.includes(scale))) continue;
  const g = rep.geometry as Record<View, any[]>;
  const count = (view: View, bone: string): number => g[view].filter((shape) => shape.bone === bone && shape.role !== "ground").length;
  const unmodelled = new Set<string>();
  for (const [name, a, b, k] of PAIRS)
    for (const left of g[a]) {
      if (left.role === "ground") continue;
      const right = g[b].find((shape) => shape.id === left.id && shape.bone === left.bone);
      if (right === undefined) continue;
      const twin = g.elevation.find((shape) => shape.id === left.id && shape.bone === left.bone);
      const [ba, bb] = [bodyOf(asset, rest, a, left, twin), bodyOf(asset, rest, b, right, twin)];
      const tier = count(a, left.bone) === 1 && count(b, left.bone) === 1 ? "HARD" : "twin";
      if (ba === undefined || bb === undefined) {
        unmodelled.add(`${left.bone}/${left.id}(${ba === undefined ? a : b},${tier})`);
        continue;
      }
      const w = worst(ba, bb, left.bone, k, subjects);
      if (w.delta > tolerance && tier === "HARD") hard++;
      if (w.delta > tolerance || args.includes("--all")) console.log(`${asset.id} 1:${scale} ${w.delta > tolerance ? tier : "ok"} ${name} ${left.bone}/${left.id} ${(w.delta * 1000).toFixed(0)} mm (mean ${(w.mean * 1000).toFixed(0)}) @ ${w.subject}: ${a} [${w.a.map((v) => v.toFixed(3))}] vs ${b} [${w.b.map((v) => v.toFixed(3))}]`);
    }
  if (unmodelled.size > 0) console.log(`${asset.id} 1:${scale} unmodelled: ${[...unmodelled].join(" ")}`);
}
console.log(`${asset.id}: ${subjects.length} subjects, tolerance ${(tolerance * 1000).toFixed(1)} mm, model HARD pairs: ${hard}`);
process.exit(hard === 0 ? 0 : 1);
