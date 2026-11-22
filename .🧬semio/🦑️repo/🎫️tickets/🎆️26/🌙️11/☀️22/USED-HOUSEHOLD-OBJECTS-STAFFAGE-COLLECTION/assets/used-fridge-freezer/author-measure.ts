/**
 * 📏 Prints the product's volume measurements of the candidate document whose name contains a filter (default: area), for setting `areaM2` from the 1:50 model.
 * Usage: bun author-measure.ts <scale> [filter]
 * @see ../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts
 */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { measureVolumes } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";

const scale = Number(process.argv[2] ?? "50");
const filter = process.argv[3] ?? "rea";
const asset = JSON.parse(readFileSync(join(import.meta.dir, "🔣️.json"), "utf8")).asset;
for (const m of measureVolumes(asset, { scale, rules: ["volumes-thermal-coverage", "volumes-silhouette", "volumes-cross-scale", "volumes-bounds"] } as never) as readonly { name: string; value: number; limit: number; scale?: number; list?: string }[])
  if (m.name.includes(filter)) console.log(`[DEBUG] 1:${m.scale ?? scale} ${m.list ?? ""} ${m.name} ${m.value.toFixed(4)} (limit ${m.limit === Number.MAX_VALUE ? "-" : m.limit.toFixed(4)})`);
