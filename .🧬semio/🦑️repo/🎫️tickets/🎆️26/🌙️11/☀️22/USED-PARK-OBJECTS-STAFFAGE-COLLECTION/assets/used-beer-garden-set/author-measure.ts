/** 📏️ Prints the measured volume numbers of the candidate (areas, silhouettes, bounds) that the report quotes: `$BUN author-measure.ts [filter]`.
 * @see ../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts — `measureVolumes` */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { measureVolumes } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";

const asset = JSON.parse(readFileSync(join(import.meta.dir, "🔣️.json"), "utf8")).asset;
const filter = process.argv[2] ?? "";
for (const m of measureVolumes(asset) as readonly Record<string, unknown>[]) {
  const line = JSON.stringify(m);
  if (line.includes(filter)) console.log(line.slice(0, 400));
}
