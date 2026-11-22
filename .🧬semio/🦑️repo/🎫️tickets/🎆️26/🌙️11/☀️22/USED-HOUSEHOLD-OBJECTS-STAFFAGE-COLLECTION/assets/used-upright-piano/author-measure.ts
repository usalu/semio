/** 📏 Prints the product's volume measurements of the piano candidate for one scale and rule (`bun author-measure.ts <file> <scale> <rule> [filter]`); with the rule `areas` it writes the 1:50 outer area of every region to `areas.json` next to this file. */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { measureVolumes } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";

const [file, scale, rule, filter] = process.argv.slice(2);
const asset = JSON.parse(readFileSync(file!, "utf8")).asset;
const wanted = rule === "areas" ? "volumes-thermal-coverage" : rule!;
const found = measureVolumes(asset, { scale: Number(scale), rules: [wanted] } as never) as readonly { readonly name?: string; readonly value?: number }[];
if (rule === "areas") {
  const areas: Record<string, number> = {};
  for (const m of found) if (m.name?.startsWith("info:outerAreaM2:")) areas[m.name.slice("info:outerAreaM2:".length)] = Math.round((m.value ?? 0) * 10000) / 10000;
  writeFileSync(join(import.meta.dir, "areas.json"), `${JSON.stringify(areas, null, 1)}\n`);
  console.log(JSON.stringify(areas));
} else {
  for (const m of found) {
    const line = JSON.stringify(m);
    if (filter === undefined || line.includes(filter)) console.log(line.slice(0, 400));
  }
}
