/** 📏️ Author 01: prints the measurements of chosen volume rules of a candidate asset document.
 *
 * Usage: `bun rework-01-measure.ts <asset document json> <scale> <rule>[,<rule>] [name filter]` */
import { readFileSync } from "node:fs";

import { checkVolumes, measureVolumes } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";

const [file, scale, rules, filter] = process.argv.slice(2);
const asset = JSON.parse(readFileSync(file!, "utf8")).asset;
for (const m of measureVolumes(asset as never, { scale: Number(scale) as never, rules: rules!.split(",") as never }) as readonly Record<string, unknown>[]) {
  if (filter !== undefined && !String(m.name).includes(filter)) continue;
  process.stdout.write(`${m.list ?? ""} ${m.name} = ${typeof m.value === "number" ? m.value.toFixed(4) : m.value} (${m.bound} ${m.limit})\n`);
}
if (filter === "findings") for (const f of checkVolumes(asset as never, { scale: Number(scale) as never, rules: rules!.split(",") as never }) as readonly { message: { en: string } }[]) process.stdout.write(`FINDING ${f.message.en}\n`);
