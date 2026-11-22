/** 📏️ Prints every measured volume rule value of the candidate with its limit, so the author sees how far off each number is.
 *
 * `$BUN author-measure.ts [--scale N] [--rules a,b] [--grep text] [--fail]` reads `🔣️.json` next to this file.
 *
 * @see ../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts — `measureVolumes` */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { measureVolumes } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";

const args = process.argv.slice(2);
const opt = (name: string): string | undefined => {
  const i = args.indexOf(name);
  return i < 0 ? undefined : args[i + 1];
};
const asset = JSON.parse(readFileSync(join(import.meta.dir, "🔣️.json"), "utf8")).asset;
const scale = opt("--scale");
const rules = opt("--rules")?.split(",").map((rule) => (rule.startsWith("volumes-") ? rule : `volumes-${rule}`));
const grep = opt("--grep");
const options = { ...(scale === undefined ? {} : { scale: Number(scale) }), ...(rules === undefined ? {} : { rules }) };
for (const m of measureVolumes(asset, options as never) as readonly { rule: string; scale?: number; list?: string; name: string; value: number; limit: number; bound?: string; ok?: boolean }[]) {
  const line = `${m.rule} 1:${m.scale ?? "-"} ${m.list ?? "-"} ${m.name} = ${Number(m.value).toFixed(4)} (${m.bound ?? ""} ${m.limit}) ${m.ok === false ? "FAIL" : ""}`;
  if (grep !== undefined && !line.includes(grep)) continue;
  if (args.includes("--fail") && m.ok !== false) continue;
  console.log(line);
}
