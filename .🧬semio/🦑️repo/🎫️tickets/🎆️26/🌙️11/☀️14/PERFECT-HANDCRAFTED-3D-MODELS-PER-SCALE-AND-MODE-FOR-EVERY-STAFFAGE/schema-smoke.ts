import { readFileSync } from "node:fs";
import { measureVolumes } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";
const a = JSON.parse(readFileSync(process.argv[2]!, "utf8")).asset;
const rule = process.argv[3] ?? "volumes-silhouette";
const t = Date.now();
for (const m of measureVolumes(a, { rules: [rule as never] })) if (!process.argv[4] || m.name.includes(process.argv[4])) console.log(m.scale, m.name, +m.value.toFixed(4), +m.limit.toFixed(4), m.ok ? "ok" : "BAD");
console.log(a.id, Date.now() - t, "ms");
