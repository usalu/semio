/** 📐️ Prints every product volume measurement of the candidate (`measureVolumes`), filtered by an optional substring: `bun author-measure.ts [filter] [--scale N]`.
 * @see ../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { measureVolumes } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";

const args = process.argv.slice(2);
const scaleAt = args.indexOf("--scale");
const scale = scaleAt >= 0 ? Number(args[scaleAt + 1]) : undefined;
const filter = args.find((a, i) => !a.startsWith("--") && !(i > 0 && args[i - 1] === "--scale")) ?? "";
const asset = JSON.parse(readFileSync(join(import.meta.dir, "🔣️.json"), "utf8")).asset;
for (const m of measureVolumes(asset, scale === undefined ? {} : { scale } as never) as readonly Record<string, unknown>[]) {
  const line = JSON.stringify(m);
  if (line.includes(filter)) console.log(line.slice(0, 300));
}
