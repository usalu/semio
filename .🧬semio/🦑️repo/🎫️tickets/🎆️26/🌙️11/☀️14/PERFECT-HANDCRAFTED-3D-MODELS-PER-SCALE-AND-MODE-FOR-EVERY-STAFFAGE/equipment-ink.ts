/** 🖊️ Checks the office print profile at both ends of each hand-drawn band. */
import {readFileSync} from "node:fs";
import {validateRanges} from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts";
import {DEFAULT_PRINT_PROFILE} from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";

for (const file of process.argv.slice(2)) {
  const {asset}=JSON.parse(readFileSync(file,"utf8"));
  const issues=validateRanges(asset,DEFAULT_PRINT_PROFILE);
  console.log(asset.id,issues.map(issue=>issue.message.en));
  if (issues.length) process.exitCode=1;
}
