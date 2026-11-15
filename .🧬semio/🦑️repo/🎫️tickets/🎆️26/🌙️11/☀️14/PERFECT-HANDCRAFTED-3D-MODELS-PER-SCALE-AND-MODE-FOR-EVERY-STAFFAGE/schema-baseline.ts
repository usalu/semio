import { readFileSync } from "node:fs";
import { measureVolumes } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";
const a = JSON.parse(readFileSync(process.argv[2]!, "utf8")).asset;
const t = Date.now();
const rows = new Map<string, string[]>();
for (const m of measureVolumes(a)) {
  const key = `${m.rule} ${m.scale ?? "-"}`;
  rows.set(key, [...(rows.get(key) ?? []), `${m.ok ? "ok" : "BAD"}:${m.name}=${+m.value.toFixed(4)}/${+m.limit.toFixed(4)}`]);
}
for (const [k, v] of rows) console.log(k, v.filter((x) => x.startsWith("BAD")).length ? "FAIL" : "pass", v.filter((x) => x.startsWith("BAD")).join(" ").slice(0, 200));
console.log(Date.now() - t, "ms");
