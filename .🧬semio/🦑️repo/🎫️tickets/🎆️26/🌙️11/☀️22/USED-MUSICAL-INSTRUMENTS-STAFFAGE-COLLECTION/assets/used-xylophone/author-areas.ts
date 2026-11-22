/** 📐️ Prints the measured outer area of every thermal region at the given scales (`bun author-areas.ts 50 100`), the values `areaM2` is authored from.
 * @see ../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts */
import { readFileSync } from "node:fs";
import { join } from "node:path";
const S = join(import.meta.dir, "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage");
const { measureVolumes } = await import(join(S, "🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts"));
const asset = JSON.parse(readFileSync(join(import.meta.dir, "🔣️.json"), "utf8")).asset;
for (const scale of process.argv.slice(2).map(Number)) {
  const ms = measureVolumes(asset, { scale, rules: ["volumes-thermal-coverage"] });
  for (const m of ms) if (/area/i.test(m.name)) console.log(`[DEBUG] 1:${scale} ${m.name} ${Number(m.value).toFixed(4)} limit ${m.limit}`);
}
