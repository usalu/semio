import { writeFileSync } from "node:fs";
import { catalog } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
import { validateAsset } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts";
const [, , out, ...ids] = process.argv;
const res: Record<string, [number, string, number][]> = {};
for (const a of catalog().filter((x) => ids.includes(x.id))) {
  res[a.id] = (validateAsset as any)(a).filter((i: any) => i.code === "lateral-ambiguous" && /representations.d+.geometry.w+[d+]/.test(String(i.path))).map((i: any) => { const m = String(i.path).match(/representations\.(\d+)\.geometry\.(\w+)\[(\d+)\]/)!; return [+m[1], m[2], +m[3]]; }).sort((x: any, y: any) => (x[1] === "elevation" ? 0 : 1) - (y[1] === "elevation" ? 0 : 1));
}
writeFileSync(out, JSON.stringify(res));
console.log(Object.entries(res).map(([k, v]) => `${k}:${v.length}`).join(" "));
