/** 📏️ Prints the product volume measurements of one candidate for chosen rules and scales: `measure.ts <id> <scale> <rule,rule>`. */
import { readFileSync } from "node:fs";
import { join, dirname } from "node:path";
import { fileURLToPath } from "node:url";
import { measureVolumes } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";
const [id, scale, rules] = process.argv.slice(2);
const doc = JSON.parse(readFileSync(join(dirname(fileURLToPath(import.meta.url)), "../../🗑️generated/PW-01", id!, "🔣️.json"), "utf8"));
for (const m of measureVolumes(doc.asset, { scale: Number(scale) as 100, rules: rules!.split(",") as never })) console.log(`${m.ok ? " " : "!"} ${m.rule} 1:${m.scale} ${m.list ?? ""} ${m.name} ${m.value.toFixed(4)} / ${m.limit === Number.MAX_VALUE ? "-" : m.limit.toFixed(4)}`);
