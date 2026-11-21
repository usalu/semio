import { readFileSync } from "node:fs";
import { validateRanges } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts";
import { DEFAULT_PRINT_PROFILE } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
for (const path of process.argv.slice(2)) {
  const {asset} = JSON.parse(readFileSync(path,"utf8"));
  console.log(asset.id,JSON.stringify(validateRanges(asset,DEFAULT_PRINT_PROFILE)));
}
