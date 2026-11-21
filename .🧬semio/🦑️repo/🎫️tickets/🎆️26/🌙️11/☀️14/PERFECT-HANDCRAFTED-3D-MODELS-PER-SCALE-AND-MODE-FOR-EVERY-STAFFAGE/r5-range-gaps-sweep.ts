/** 🕳️ R01 sweep: every catalog asset through the product rule `uncoveredDenominators` (rule `valid-range-gap`); one line per asset with a hole, then the count.
 * Usage: `bun r5-range-gaps-sweep.ts [--json]` */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { uncoveredDenominators } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts";
import { STAFFAGE, catalogEntries } from "./volumes-lib.ts";

const rows = catalogEntries().map((entry) => {
  const asset = JSON.parse(readFileSync(entry.path, "utf8")).asset;
  const ranges = Object.fromEntries(Object.entries(asset.representations).map(([key, rep]: [string, any]) => [key, rep.status === "authored" ? `${rep.validRange.minDenominator}-${rep.validRange.maxDenominator}` : "unavailable"]));
  return { id: `${asset.category}/${asset.id}`, ranges, gaps: uncoveredDenominators(asset).map(({ from, to, border }) => ({ from, to, border })) };
});
if (process.argv.includes("--json")) console.log(JSON.stringify(rows, null, 1));
else {
  for (const row of rows.filter((r) => r.gaps.length > 0)) console.log(`${row.id} | ${Object.entries(row.ranges).map(([k, v]) => `${k}:${v}`).join(" ")} | ${row.gaps.map((g) => `1:${g.from}…1:${g.to} (border 1:${g.border})`).join(", ")}`);
  console.log(`${rows.filter((r) => r.gaps.length > 0).length} of ${rows.length} assets with a hole`);
}
