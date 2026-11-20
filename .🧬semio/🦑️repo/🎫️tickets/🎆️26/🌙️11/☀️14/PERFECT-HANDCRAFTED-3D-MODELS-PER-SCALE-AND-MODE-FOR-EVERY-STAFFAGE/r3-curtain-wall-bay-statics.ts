/** 🏗️ Round 3 curtain-wall bay: solves the statics of every pose of an asset document (catalog asset or a candidate file) and prints reactions, residual and notes, to check the support layout.
 *
 * Usage: `bun r3-curtain-wall-bay-statics.ts [<asset document json>]` */
import { readFileSync } from "node:fs";
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { solveStructure } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🏗️structure/🟦️.ts";

const CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🧱️components/curtain-wall-bay/🔣️.json";
const asset = JSON.parse(readFileSync(process.argv[2] ?? CATALOG, "utf8")).asset;
const f = (v: number): string => v.toFixed(1).padStart(8);
for (const pose of asset.poses) {
  const state = evaluate(asset, { kind: "pose", pose: pose.id }, { kind: "scale", scale: { denominator: 50 } });
  const result = solveStructure(state as never) as any;
  console.log(`== ${pose.id}`);
  for (const r of result.reactions) console.log(`  ${r.anchor.padEnd(22)} F ${f(r.fxN)} ${f(r.fyN)} ${f(r.fzN)}  M ${f(r.mxNm)} ${f(r.myNm)} ${f(r.mzNm)}`);
  const res = result.residual;
  console.log(`  residual F ${f(res.fxN)} ${f(res.fyN)} ${f(res.fzN)}  M ${f(res.mxNm)} ${f(res.myNm)} ${f(res.mzNm)}`);
  for (const n of result.notes ?? []) { const t = typeof n === "string" ? n : (n.en ?? JSON.stringify(n)); if (/nbalanced|ollinear|ndeterminate|equal-stiffness/.test(t)) console.log("  note:", t.slice(0, 200)); }
}
