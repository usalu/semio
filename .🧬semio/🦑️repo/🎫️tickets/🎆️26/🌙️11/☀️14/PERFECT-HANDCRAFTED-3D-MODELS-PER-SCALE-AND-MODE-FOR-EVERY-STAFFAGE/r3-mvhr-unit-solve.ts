/** ⚖️ Round 3 MVHR unit: solves the structure of every pose on an asset file and prints every support reaction with its three components, the residual and the solver notes, so the hook-and-rest load path of the corrected anchors can be read in numbers.
 *
 * @see ../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🏗️structure/🟦️.ts
 */
import { readFileSync } from "node:fs";

import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { solveStructure } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🏗️structure/🟦️.ts";

const asset = JSON.parse(readFileSync(process.argv[2]!, "utf8")).asset;
const select = { kind: "scale", scale: { denominator: 50 } } as const;
for (const pose of asset.poses) {
  const result: any = solveStructure(evaluate(asset, { kind: "pose", pose: pose.id, timeS: 0 }, select));
  const weight = result.segmentWeights.reduce((a: number, f: any) => a - f.fyN, 0);
  console.log(`pose ${pose.id} weight ${weight.toFixed(1)} N residual ${Math.hypot(result.residual.fxN, result.residual.fyN, result.residual.fzN).toExponential(1)}`);
  for (const r of result.reactions) console.log(`  ${r.anchor ?? r.id} active=${r.active} F=(${r.fxN.toFixed(1)}, ${r.fyN.toFixed(1)}, ${(r.fzN ?? 0).toFixed(1)}) N at ${JSON.stringify(r.at ?? null)}`);
  for (const n of result.notes) console.log(`  note ${n.en}`);
}
