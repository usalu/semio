/** 📏️ Prints the product's volume measurements of the candidate for one scale and some rules (`bun author-measure.ts <scale> <rule,rule> [filter]`), passing or not, so the author sees how far each value is from its limit.
 *
 * @see ../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts — `measureVolumes` */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { measureVolumes } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";

const [scale, rules, filter] = process.argv.slice(2);
const asset = JSON.parse(readFileSync(join(import.meta.dir, "🔣️.json"), "utf8")).asset;
const names = (rules ?? "thermal-coverage").split(",").map((r) => (r.startsWith("volumes-") ? r : `volumes-${r}`));
for (const m of measureVolumes(asset, { scale: Number(scale ?? 50), rules: names } as never) as readonly Record<string, unknown>[]) {
  const line = `[DEBUG] ${m.rule} 1:${m.scale} ${m.list ?? ""} ${m.name} ${typeof m.value === "number" ? (m.value as number).toFixed(4) : m.value} limit ${m.limit} ${m.ok ? "ok" : "FAIL"}`;
  if (!filter || line.includes(filter)) console.log(line);
}
