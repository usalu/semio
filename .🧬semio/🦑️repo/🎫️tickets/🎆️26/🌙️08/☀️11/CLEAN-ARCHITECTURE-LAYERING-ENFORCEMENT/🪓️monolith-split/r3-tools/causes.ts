// 🔎️ Prints the merge causes of the blockers of a plan (read-only, no writes).
import {planArtifactCrates} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🗺️plan/🟦️.ts";
const [pkg, mode = "on"] = process.argv.slice(2);
const plan = await planArtifactCrates("/Users/ueli/Documents/semio", pkg!, {mutationFamilies: mode as "on", tests: {fixtures: true, aggregate: true, hoist: true, detach: true}});
console.log("problems", plan.problems.length);
for (const b of plan.blockers) {
  console.log(`BLOCKER slices=${b.slices} files=${b.files}`);
  for (const c of b.causes) console.log("  cause", c);
}
