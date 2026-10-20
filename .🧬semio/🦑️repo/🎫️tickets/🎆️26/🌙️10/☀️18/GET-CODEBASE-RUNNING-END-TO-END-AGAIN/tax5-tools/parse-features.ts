// 🥒️ Parses the wired norm features with the repository's own Gherkin planner and prints scenario counts. usage: bun parse-features.ts
import {readFileSync} from "node:fs";
import {Glob} from "bun";
const {parseFeature} = await import("/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧪️test/🥒️gherkin/🟦️.ts");
const root = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts";
let failed = 0;
for (const path of new Glob("*/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/*mutate-*-1/🥒️.feature").scanSync({cwd: root, onlyFiles: true})) {
  try {
    const plan = parseFeature(readFileSync(`${root}/${path}`, "utf8"));
    console.log(path.split("/")[0], "scenarios", plan.scenarios.length);
  } catch (error) {
    failed++;
    console.log("FAIL", path, String(error).slice(0, 200));
  }
}
console.log("failed", failed);
