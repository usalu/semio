/** 🫁️ Check the feature rule on the actual tagged apertures and count their eligible tissues. */
import {featureRule} from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import {entryOf} from "./volumes-lib.ts";
for (const name of process.argv.slice(2)) {
  const {asset}=entryOf(name);
  const issues=featureRule(asset as never).findings.filter(f=>f.message.en.includes("breath"));
  console.log(JSON.stringify({id:asset.id,issues:issues.map(f=>f.message.en)}));
  if(issues.length)process.exitCode=1;
}
