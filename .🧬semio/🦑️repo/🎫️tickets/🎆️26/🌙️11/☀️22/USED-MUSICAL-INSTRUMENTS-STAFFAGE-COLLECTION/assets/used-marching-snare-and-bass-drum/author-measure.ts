/** 📏️ Prints the product's volume measurements of the candidate `🔣️.json` next to this file for one scale and the named rules (default: thermal coverage), as `[DEBUG] name value` lines, so the declared `areaM2` can be set from the honestly modelled 1:50 regions.
 *
 * `$BUN author-measure.ts [scale] [rule,rule] [filter]`
 *
 * @see ../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts — `measureVolumes` */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { measureVolumes } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";

const scale = Number(process.argv[2] ?? 50);
const rules = (process.argv[3] ?? "volumes-thermal-coverage").split(",").map((r) => (r.startsWith("volumes-") ? r : `volumes-${r}`));
const filter = process.argv[4] ?? "";
const asset = JSON.parse(readFileSync(join(import.meta.dir, "🔣️.json"), "utf8")).asset;
for (const m of measureVolumes(asset, { scale, rules } as never) as readonly { name: string; value: unknown; scale?: number }[]) if (m.name.includes(filter)) console.log(`[DEBUG] ${m.scale ?? scale} ${m.name} ${JSON.stringify(m.value)}`);
