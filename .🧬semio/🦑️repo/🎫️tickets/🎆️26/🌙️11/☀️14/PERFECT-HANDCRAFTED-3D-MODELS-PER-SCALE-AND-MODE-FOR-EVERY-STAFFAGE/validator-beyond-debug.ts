/** 🔎️ Lists the face samples of the sample figure whose cover is refused because the free space beyond the covering solid is not reached by the air (`DEBUG_BEYOND=1`). Usage: `DEBUG_BEYOND=1 bun validator-beyond-debug.ts <scale>` */
import { readFileSync } from "node:fs";
import { measureVolumes } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";
const asset = JSON.parse(readFileSync("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧫️fixtures/👤️sample-figure/🔣️.json", "utf8")).asset;
const found = measureVolumes(asset, { scale: Number(process.argv[2] ?? 200) as never, rules: ["volumes-thermal-coverage"] });
for (const m of found) if (m.name.startsWith("info:outerAreaM2")) console.log(m.name, m.value.toFixed(6));
