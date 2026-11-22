/** 📏 Measurement probe of the candidate: prints every volume measurement (passing or not) of the named rules at one scale, so the model can be compared with the reference table.
 *
 * `$BUN author-measure.ts <scale> <rule,rule,...> [filter]`
 *
 * @see ../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts — `measureVolumes`
 */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { measureVolumes } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";

const asset = JSON.parse(readFileSync(join(import.meta.dir, "🔣️.json"), "utf8")).asset;
const [scale, rules, filter] = process.argv.slice(2);
const list = rules.split(",").map((rule) => (rule.startsWith("volumes-") ? rule : `volumes-${rule}`));
for (const m of measureVolumes(asset, { scale: Number(scale) as never, rules: list as never }) as readonly Record<string, unknown>[]) {
  const line = JSON.stringify(m);
  if (filter === undefined || line.includes(filter)) console.log(`[DEBUG] ${line}`);
}
