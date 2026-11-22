/** 📐️ Prints the measured volume numbers (areas, silhouettes) of the candidate for one scale and some rules: `$BUN author-measure.ts <scale> <rule,rule> [name-filter]`.
 * @see /Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts — `measureVolumes` */
import { readFileSync } from "node:fs";
import { measureVolumes } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";

const doc = JSON.parse(readFileSync(new URL("./🔣️.json", import.meta.url), "utf8"));
const scale = Number(process.argv[2]);
const rules = (process.argv[3] ?? "volumes-thermal-coverage").split(",").map((r) => (r.startsWith("volumes-") ? r : `volumes-${r}`));
const filter = process.argv[4] ?? "";
for (const m of measureVolumes(doc.asset, { scale: scale as never, rules: rules as never })) {
  const line = JSON.stringify(m);
  if (line.includes(filter)) console.log(`[DEBUG] ${line.slice(0, 300)}`);
}
