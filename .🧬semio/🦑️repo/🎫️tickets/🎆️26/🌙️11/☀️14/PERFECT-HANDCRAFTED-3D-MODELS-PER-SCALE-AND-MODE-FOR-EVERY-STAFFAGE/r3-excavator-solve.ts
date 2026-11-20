/** ⚖️ Round 3 excavator: solves the structure of every pose and of sampled clip frames on an asset file and prints the support reactions, the residual and the notes, and the thermal surface temperatures of the poses, so that the corrected rig, masses and areas are seen to give a standing, balanced machine.
 *
 * @see ../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🏗️structure-projection/🟦️.ts
 */
import { readFileSync } from "node:fs";

import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { solveStructure } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🏗️structure/🟦️.ts";

const asset = JSON.parse(readFileSync(process.argv[2]!, "utf8")).asset;
const select = { kind: "scale", scale: { denominator: 50 } } as const;
const show = (label: string, query: any): void => {
  const result: any = solveStructure(evaluate(asset, query, select));
  const active = result.reactions.filter((r: any) => r.active);
  const weight = result.segmentWeights.reduce((a: number, f: any) => a - f.fyN, 0);
  const sum = active.reduce((a: number, r: any) => a + r.fyN, 0);
  console.log(label, "weight", (weight / 1000).toFixed(1), "kN  active", active.length, "of", result.reactions.length, " sum", (sum / 1000).toFixed(1), "kN  residual", Math.hypot(result.residual.fxN, result.residual.fyN, result.residual.fzN).toExponential(1), " reactions", active.map((r: any) => `${r.anchor ?? r.id}:${(r.fyN / 1000).toFixed(0)}`).join(" "), result.notes.map((n: any) => n.en).join(" | ").slice(0, 300));
};
for (const pose of asset.poses) show(`pose ${pose.id}`, { kind: "pose", pose: pose.id, timeS: 0 });
for (const clip of asset.clips) for (const f of [0.25, 0.5, 0.75]) show(`clip ${clip.id} @${(clip.durationS * f).toFixed(2)}`, { kind: "clip", clip: clip.id, timeS: clip.durationS * f });
