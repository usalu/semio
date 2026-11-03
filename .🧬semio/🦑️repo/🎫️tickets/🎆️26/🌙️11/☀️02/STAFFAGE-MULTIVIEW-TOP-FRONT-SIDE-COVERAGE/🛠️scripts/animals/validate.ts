import { catalog } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
import { validateAsset } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts";
const [, , ...ids] = process.argv;
let total = 0; const codes = new Map<string, number>();
for (const a of catalog().filter((x) => ids.includes(x.id))) {
  const issues: any[] = (validateAsset as any)(a);
  const rows = issues.filter((i) => (i.severity ?? "error") !== "warning");
  for (const i of rows) { total++; codes.set(i.code, (codes.get(i.code) ?? 0) + 1); }
  const by = new Map<string, number>(); for (const i of rows) by.set(i.code, (by.get(i.code) ?? 0) + 1);
  console.log(a.id.padEnd(30), rows.length, [...by].map(([k, v]) => `${k}x${v}`).join(" "));
  if (process.env.VERBOSE) for (const i of rows.slice(0, 30)) console.log("    ", i.code, i.path, String(i.message?.en ?? i.message).slice(0, 170));
}
console.log("TOTAL", total, [...codes]);
