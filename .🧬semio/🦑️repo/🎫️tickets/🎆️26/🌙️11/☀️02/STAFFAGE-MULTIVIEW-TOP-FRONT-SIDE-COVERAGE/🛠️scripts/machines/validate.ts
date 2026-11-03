import { catalog } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
import { validateAsset } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts";
const ids = (process.env.IDS ?? "").split(",").filter(Boolean);
let total=0;
for (const a of catalog()) {
  if (!["machinery","vehicles"].includes(a.category)) continue;
  if (ids.length && !ids.includes(a.id)) continue;
  const all = [...catalog()].map(x=>x.id);
  const issues = validateAsset(a, {assets:new Set(all)});
  total+=issues.length;
  console.log(`## ${a.id}: ${issues.length}`);
  for (const i of issues.slice(0,Number(process.env.MAX??40))) console.log(`  [${i.code}] ${i.path}: ${i.message.en.slice(0,230)}`);
}
console.log("total",total);
