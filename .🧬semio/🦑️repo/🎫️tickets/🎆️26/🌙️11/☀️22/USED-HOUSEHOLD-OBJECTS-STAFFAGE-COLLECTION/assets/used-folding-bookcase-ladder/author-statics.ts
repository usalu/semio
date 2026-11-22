/** ⚖️ Prints the reactions, residual and notes of the product's structural solve for every named pose (and optional clip samples) of the candidate: `bun author-statics.ts [clip samples]`. */
import { readFileSync } from "node:fs";
import { evaluate } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { solveStructure } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🏗️structure/🟦️.ts";

const asset = JSON.parse(readFileSync(new URL("./🔣️.json", import.meta.url), "utf8")).asset;
const samples = Number(process.argv[2] ?? 0);
const queries = [...asset.poses.map((p: { id: string }) => ({ kind: "pose", pose: p.id })), ...asset.clips.flatMap((c: { id: string; durationS: number }) => Array.from({ length: samples }, (_, i) => ({ kind: "clip", clip: c.id, timeS: (c.durationS * i) / Math.max(1, samples - 1) })))];
for (const q of queries) {
  const r = solveStructure(evaluate(asset, q as never));
  const label = q.kind === "pose" ? q.pose : `${q.clip}@${q.timeS.toFixed(2)}`;
  console.log(`[DEBUG] ${label} mass ${r.totalMassKg.toFixed(2)} residual ${r.residual.fxN.toFixed(2)} ${r.residual.fyN.toFixed(2)} ${r.residual.fzN.toFixed(2)}`);
  for (const re of r.reactions) console.log(`[DEBUG]   ${re.anchor} active ${re.active} f ${[re.fxN, re.fyN, re.fzN].map((v: number) => v.toFixed(1)).join(" ")}`);
  for (const n of r.notes ?? []) console.log(`[DEBUG]   note ${(n as { en?: string }).en ?? JSON.stringify(n)}`.slice(0, 200));
}
