/** 📊 Prints every measurement of the product's measured volume rules for the candidate: `bun author-measure.ts <scale> <rule,rule> [filter]`.
 * @see ../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/r3-validate.ts */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { measureVolumes } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";

const asset = JSON.parse(readFileSync(join(import.meta.dir, "🔣️.json"), "utf8")).asset;
const [scale, rules, filter] = process.argv.slice(2);
for (const m of measureVolumes(asset, { scale: Number(scale), rules: rules.split(",").map((r) => (r.startsWith("volumes-") ? r : `volumes-${r}`)) } as never) as readonly Record<string, unknown>[]) {
  if (filter && !String(m.name).includes(filter)) continue;
  console.log(`[DEBUG] ${m.rule ?? ""} 1:${m.scale} ${m.list ?? ""} ${m.name} = ${typeof m.value === "number" ? (m.value as number).toFixed(4) : JSON.stringify(m.value)} (limit ${m.limit} ${m.bound ?? ""})`);
}
