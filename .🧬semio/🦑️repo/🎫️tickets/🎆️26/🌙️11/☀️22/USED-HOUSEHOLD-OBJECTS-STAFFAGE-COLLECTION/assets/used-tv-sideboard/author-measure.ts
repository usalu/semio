/** 📏 Prints the product's volume measurements of the candidate for one scale and rule: `bun author-measure.ts <file> <scale> <rule> [filter]`. */
import { readFileSync } from "node:fs";
import { measureVolumes } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";

const [file, scale, rule, filter] = process.argv.slice(2);
const asset = JSON.parse(readFileSync(file!, "utf8")).asset;
for (const m of measureVolumes(asset, { scale: Number(scale), rules: [rule!] } as never) as readonly Record<string, unknown>[]) {
  const line = JSON.stringify(m);
  if (filter === undefined || line.includes(filter)) console.log(line.slice(0, 400));
}
