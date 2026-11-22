/** 📏️ Prints the measured volume values of the candidate (thermal areas, silhouettes, bounds) for the report: `bun author-measure.ts [scale] [rule]`. */
import { readFileSync } from "node:fs";
import { measureVolumes } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";

const asset = JSON.parse(readFileSync(`${import.meta.dir}/🔣️.json`, "utf8")).asset;
const scale = process.argv[2] === undefined ? undefined : Number(process.argv[2]);
const rules = process.argv[3] === undefined ? undefined : [process.argv[3]];
for (const m of measureVolumes(asset, { ...(scale === undefined ? {} : { scale }), ...(rules === undefined ? {} : { rules }) } as never) as readonly { rule: string; scale?: number; name: string; value: number; limit?: number; ok?: boolean }[]) console.log(`${m.rule} 1:${m.scale ?? "-"} ${m.name} ${typeof m.value === "number" ? m.value.toFixed(4) : m.value} ${m.limit ?? ""} ${m.ok === false ? "FAIL" : ""}`);
