import { catalog } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
import { validateAsset } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts";
const [, , ...ids] = process.argv;
for (const a of catalog().filter((x) => ids.includes(x.id))) {
  const rows = (validateAsset as any)(a).filter((i: any) => i.code === "lateral-ambiguous");
  const by = new Map<string, string[]>();
  for (const i of rows) { const m = String(i.path).match(/representations\.(\d+)\.geometry\.(\w+)\[(\d+)\]/)!; const r: any = (a.representations as any)[m[1]]; const s = r.geometry[m[2]][+m[3]]; const k = `${m[1]} ${m[2]}`; (by.get(k) ?? by.set(k, []).get(k)!).push(`${s.id}/${s.bone}:${s.kind}`); }
  console.log("##", a.id, rows.length); for (const [k, v] of by) console.log("  ", k, v.join(" "));
}
