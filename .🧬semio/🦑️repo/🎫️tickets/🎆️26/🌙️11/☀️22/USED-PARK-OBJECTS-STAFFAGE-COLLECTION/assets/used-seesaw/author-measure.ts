/** 📊️ Prints the product's volume measurements of the used-seesaw candidate (one line per measurement), optionally filtered: `$BUN author-measure.ts [scale] [rule,rule] [name-substring]`.
 *
 * @see ../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts — `measureVolumes` */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { measureVolumes } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";

const [scale, rules, filter] = process.argv.slice(2);
const asset = JSON.parse(readFileSync(join(import.meta.dir, "🔣️.json"), "utf8")).asset;
const options = { ...(scale ? { scale: Number(scale) } : {}), ...(rules ? { rules: rules.split(",").map((r) => (r.startsWith("volumes-") ? r : `volumes-${r}`)) } : {}) };
for (const m of measureVolumes(asset, options as never) as readonly Record<string, unknown>[]) {
  const line = JSON.stringify(m);
  if (!filter || line.includes(filter)) console.log(`[DEBUG] ${line}`);
}
