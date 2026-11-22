/** 📊️ Debug aid: prints the product's volume measurements of the candidate for one scale and rule (`bun author-debug-measure.ts <scale> <rule> [filter]`), the numbers behind the pass, for the report and the area calibration. */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { measureVolumes } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";

const asset = JSON.parse(readFileSync(join(import.meta.dir, "🔣️.json"), "utf8")).asset;
const [scale, rule, filter] = [Number(process.argv[2] ?? 50), process.argv[3], process.argv[4]];
for (const m of measureVolumes(asset, { scale: scale as never, ...(rule ? { rules: [rule as never] } : {}) }) as readonly Record<string, unknown>[]) {
  const line = JSON.stringify(m);
  if (filter === undefined || line.includes(filter)) console.log(`[DEBUG] ${line}`);
}
