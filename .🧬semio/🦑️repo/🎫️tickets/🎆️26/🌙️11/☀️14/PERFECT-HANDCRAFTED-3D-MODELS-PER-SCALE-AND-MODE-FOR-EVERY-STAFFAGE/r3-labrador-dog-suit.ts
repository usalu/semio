/** 🖨️ Round 3 Labrador: the print-profile findings of every authored representation of an asset document over all named poses. Usage: bun r3-labrador-dog-suit.ts [<asset document json>] */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { DEFAULT_PRINT_PROFILE, measureSuitability } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
import { OUT } from "./r3-labrador-dog-work.ts";
import { type Json } from "./volumes-lib.ts";

const asset: Json = JSON.parse(readFileSync(process.argv[2] ?? join(OUT, "🔣️.json"), "utf8")).asset;
for (const scale of [50, 100, 200, 500]) {
  const s: Json = measureSuitability(asset as never, asset.representations[String(scale)], scale, DEFAULT_PRINT_PROFILE as never);
  console.log(`1:${scale}`, s.suitable ? "suitable" : "UNSUITABLE", "min stroke", s.minStrokeMm, "gap", s.minGapMm, "feature", s.minFeatureMm);
  for (const v of s.violations as Json[]) console.log("   ", v.check, v.shapes.join("+"), v.measuredMm.toFixed(3), "<", v.requiredMm, v.at === undefined ? "" : `at ${v.at.xMm.toFixed(2)},${v.at.yMm.toFixed(2)}`);
}
