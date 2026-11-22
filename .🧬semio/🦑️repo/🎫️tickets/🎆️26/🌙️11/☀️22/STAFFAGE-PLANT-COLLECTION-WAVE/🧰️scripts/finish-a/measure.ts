/** 📏️ Finisher A measurement: prints the product's volume measurements of one asset document for chosen rules (and optionally one scale); failing lines are marked `!`.
 * Usage: `bun measure.ts <asset document json> <rule,rule> [scale] [--all]` (without `--all` only failing measurements and every thermal-coverage line).
 * @see ../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts */
import { readFileSync } from "node:fs";
import { measureVolumes } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";

const [file, rules, scale] = process.argv.slice(2).filter((a) => !a.startsWith("--"));
const all = process.argv.includes("--all");
const doc = JSON.parse(readFileSync(file!, "utf8"));
const names = rules!.split(",").map((r) => (r.startsWith("volumes-") ? r : `volumes-${r}`));
const options = { rules: names as never, ...(scale === undefined ? {} : { scale: Number(scale) as 100 }) };
for (const m of measureVolumes(doc.asset, options)) {
  if (!all && m.ok && m.rule !== "volumes-thermal-coverage") continue;
  console.log(`${m.ok ? " " : "!"} ${m.rule} 1:${m.scale} ${m.list ?? ""} ${m.name} ${m.value.toFixed(4)} / ${m.limit === Number.MAX_VALUE ? "-" : m.limit.toFixed(4)}`);
}
