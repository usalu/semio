import { catalog } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
const [, , ...ids] = process.argv;
for (const a of catalog().filter((x) => ids.includes(x.id))) {
  const seen = new Map<string, Set<string>>();
  const add = (where: string, bones: any) => { for (const [b, o] of Object.entries<any>(bones ?? {})) if (o.abductionDeg || o.twistDeg) { if (!seen.has(b)) seen.set(b, new Set()); seen.get(b)!.add(where); } };
  for (const p of a.poses) add("pose:" + p.id, p.bones);
  for (const c of a.clips) for (const k of c.keys) add("clip:" + c.id, k.bones);
  console.log(a.id, [...seen].map(([b, w]) => `${b}[${[...w].slice(0, 3).join(",")}${w.size > 3 ? ",+" + (w.size - 3) : ""}]`).join(" "));
}
