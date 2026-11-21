/** 🧪️ R01 quick check of the coverage vectors against `uncoveredDenominators` outside vitest (the product test is `🧪️tests/🕳️valid-range-coverage`). */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { uncoveredDenominators } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts";
import { STAFFAGE } from "./volumes-lib.ts";

const vectors = JSON.parse(readFileSync(join(STAFFAGE, "🧫️fixtures/🕳️valid-range-coverage/🔣️.json"), "utf8"));
let failed = 0;
for (const c of vectors.cases) {
  const representations = Object.fromEntries(Object.entries(c.ranges).map(([k, r]: [string, any]) => [k, r === "unavailable" ? { status: "unavailable" } : { status: "authored", validRange: { minDenominator: r[0], maxDenominator: r[1] } }]));
  const got = uncoveredDenominators({ representations } as never).map((g) => [g.from, g.to, g.border]);
  const ok = JSON.stringify(got) === JSON.stringify(c.gaps);
  if (!ok) failed++;
  console.log(`${ok ? "ok  " : "FAIL"} ${c.name} ${JSON.stringify(got)}`);
}
process.exit(failed === 0 ? 0 : 1);
