/** 📏️ Prints every measurement of the product's volume rules for the candidate document next to this file (passing or not, with its limit), the numbers the report quotes.
 *
 * `$BUN author-measure.ts [scale] [rule,rule]`
 * @see ../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts — `measureVolumes` */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { measureVolumes } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";

const asset = JSON.parse(readFileSync(join(import.meta.dir, "🔣️.json"), "utf8")).asset;
const scale = process.argv[2] === undefined || process.argv[2] === "all" ? undefined : Number(process.argv[2]);
const rules = process.argv[3]?.split(",").map((r) => (r.startsWith("volumes-") ? r : `volumes-${r}`));
const measured = measureVolumes(asset, { ...(scale === undefined ? {} : { scale }), ...(rules === undefined ? {} : { rules }) } as never);
for (const m of measured) {
  if (m.name.startsWith("info:") && !m.name.includes("exposed") && !m.name.includes("floodCell")) continue;
  console.log(`${m.ok ? "ok  " : "FAIL"} ${m.rule.replace("volumes-", "")} 1:${m.scale ?? "-"} ${m.list ?? "-"} ${m.name} ${m.value.toFixed(4)} / ${m.limit === Number.MAX_VALUE ? "-" : m.limit.toFixed(4)}`);
}
