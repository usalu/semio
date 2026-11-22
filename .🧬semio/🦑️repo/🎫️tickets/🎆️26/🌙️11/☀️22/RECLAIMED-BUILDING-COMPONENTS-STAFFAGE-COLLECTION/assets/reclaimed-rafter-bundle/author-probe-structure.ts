/** 🔬️ Structure probe of the candidate `reclaimed-rafter-bundle`: evaluates every named pose and two clip instants, solves the structure with the product solver and prints the reactions, the residual and the end moments of the members, for comparison with the hand values of the report.
 *
 * `$BUN author-probe-structure.ts` (from this folder).
 *
 * @see ../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🏗️structure/🟦️.ts — the solver */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { evaluate } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { solveStructure } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🏗️structure/🟦️.ts";

const asset = JSON.parse(readFileSync(join(import.meta.dir, "🔣️.json"), "utf8")).asset;
const queries = [
  ...asset.poses.map((pose: { id: string }) => ({ label: pose.id, query: { kind: "pose", pose: pose.id } })),
  { label: "lift t=2.5", query: { kind: "clip", clip: "lift", timeS: 2.5 } },
  { label: "roofer-walk t=3", query: { kind: "clip", clip: "roofer-walk", timeS: 3 } },
];
for (const { label, query } of queries) {
  const state = evaluate(asset, query as never);
  const result = solveStructure(state);
  const reactions = result.reactions.filter((r) => r.active).map((r) => `${r.anchor} ${(r.fyN / 1000).toFixed(3)} kN (x ${(r.fxN / 1000).toFixed(3)}, z ${(r.fzN / 1000).toFixed(3)})`);
  const residual = result.residual;
  console.log(`== ${label}: ${reactions.join("; ")} | residual F ${Math.hypot(residual.fxN, residual.fyN, residual.fzN).toFixed(3)} N, M ${Math.hypot(residual.mxNm, residual.myNm, residual.mzNm).toFixed(3)} Nm`);
  for (const [bone, diagram] of Object.entries(result.members)) {
    const stations = (diagram as { stations: readonly { s: number; momentNm: number; shearN: number; axialN: number }[] }).stations;
    if (!stations?.length) continue;
    const peak = stations.reduce((a, b) => (Math.abs(b.momentNm) > Math.abs(a.momentNm) ? b : a));
    const first = stations[0]!;
    const last = stations[stations.length - 1]!;
    const axial = stations.reduce((a, b) => (Math.abs(b.axialN) > Math.abs(a.axialN) ? b : a));
    console.log(`   ${bone}: M peak ${peak.momentNm.toFixed(1)} Nm at s ${peak.s.toFixed(3)}; ends M ${first.momentNm.toFixed(1)} / ${last.momentNm.toFixed(1)} Nm, V ${first.shearN.toFixed(1)} / ${last.shearN.toFixed(1)} N; N peak ${axial.axialN.toFixed(1)} N`);
  }
  for (const note of result.notes ?? []) console.log(`   note: ${typeof note === "string" ? note : (note as { en: string }).en}`);
}
