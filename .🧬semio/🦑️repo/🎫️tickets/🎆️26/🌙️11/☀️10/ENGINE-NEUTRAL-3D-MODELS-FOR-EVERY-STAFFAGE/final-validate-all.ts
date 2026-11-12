import { catalog } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
import { validateAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts";
let n = 0;
for (const asset of catalog()) {
  const issues = validateAsset(asset, {});
  for (const i of issues) { n++; console.log(asset.id, i.code, i.path, String(i.message?.en ?? i.message).slice(0, 200)); }
}
console.log("total", n);
