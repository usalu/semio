/** ⚖️ Prints the support reactions of every named pose of one candidate as the product's structure solver computes them, to measure how lifting poses share the weight.
 *
 * `$BUN reactions.ts <slug> [pose …]` prints per pose the total weight, the applied loads and the reaction of every active support.
 *
 * @see ../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🏗️structure/🟦️.ts — `solveStructure` */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { solveStructure } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🏗️structure/🟦️.ts";

const slug = process.argv[2] ?? "";
const asset = JSON.parse(readFileSync(join(import.meta.dir, "assets", slug, "🔣️.json"), "utf8")).asset;
const wanted = process.argv.slice(3);
for (const pose of asset.poses) {
  if (wanted.length > 0 && !wanted.includes(pose.id)) continue;
  const result = solveStructure(evaluate(asset, { kind: "pose", pose: pose.id }));
  const n = (value: number): string => value.toFixed(1);
  console.log(`${slug} ${pose.id}: weight ${n(result.totalMassKg * result.gravityMS2)} N (${result.totalMassKg.toFixed(1)} kg), residual fy ${n(result.residual.fyN)} N`);
  for (const load of result.applied) console.log(`  load fx ${n(load.fxN)} fy ${n(load.fyN)} fz ${n(load.fzN ?? 0)}`);
  for (const reaction of result.reactions) if (reaction.active) console.log(`  ${reaction.anchor}: fx ${n(reaction.fxN)} fy ${n(reaction.fyN)} fz ${n(reaction.fzN)} N, m ${n(reaction.mxNm)} ${n(reaction.myNm)} ${n(reaction.mzNm)} Nm`);
}
