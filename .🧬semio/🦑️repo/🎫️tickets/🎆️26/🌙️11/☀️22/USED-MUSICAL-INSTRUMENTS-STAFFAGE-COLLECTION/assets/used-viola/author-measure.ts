/** 📏️ Prints the product's volume measurements of the candidate for one scale and some rules: `$BUN author-measure.ts <scale> <rule,rule> [filter]`. */
import { readFileSync } from "node:fs";
import { measureVolumes } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";
const asset = JSON.parse(readFileSync(new URL("./🔣️.json", import.meta.url), "utf8")).asset;
const rules = process.argv[3]!.split(",").map((r) => (r.startsWith("volumes-") ? r : `volumes-${r}`));
for (const m of measureVolumes(asset, { scale: Number(process.argv[2]) as never, rules: rules as never }) as any[]) {
  const line = JSON.stringify(m);
  if (!process.argv[4] || line.includes(process.argv[4])) console.log("[DEBUG]", line.slice(0, 300));
}
