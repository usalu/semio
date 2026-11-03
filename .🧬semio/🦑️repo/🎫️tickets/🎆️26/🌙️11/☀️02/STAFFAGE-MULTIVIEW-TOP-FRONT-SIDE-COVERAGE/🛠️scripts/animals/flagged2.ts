import { writeFileSync } from "node:fs";
import { catalog } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
import { validateAsset } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts";
const [, , out, ...ids] = process.argv;
const res: Record<string, { bone: string; ids: string[]; combos: [number, string][] }[]> = {};
for (const a of catalog().filter((x) => ids.includes(x.id))) {
  res[a.id] = [];
  for (const i of (validateAsset as any)(a).filter((i: any) => i.code === "lateral-ambiguous")) {
    const m = String(i.message?.en ?? i.message).match(/\(([^)]*)\) on bone "([^"]+)".*? in ((?:1:\d+ \w+(?:, )?)+)/);
    if (!m) { console.log("UNPARSED", a.id, String(i.message?.en).slice(0, 200)); continue; }
    res[a.id].push({ bone: m[2], ids: m[1].split(", "), combos: m[3].split(", ").map((c) => { const [s, v] = c.split(" "); return [+s.slice(2), v] as [number, string]; }) });
  }
}
writeFileSync(out, JSON.stringify(res, null, 1));
console.log(Object.entries(res).map(([k, v]) => `${k}:${v.length}`).join(" "));
