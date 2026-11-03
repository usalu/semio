import { catalog } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
import { validateAsset } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts";
const [, , ...ids] = process.argv;
for (const a of catalog().filter((x) => ids.includes(x.id))) for (const i of (validateAsset as any)(a).filter((i: any) => i.code === "lateral-ambiguous")) console.log(a.id, i.path, String(i.message?.en ?? i.message).slice(0, 230));
