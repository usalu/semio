import {planArtifactCrates} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🟦️.ts";
const plan = await planArtifactCrates("/Users/ueli/Documents/semio", process.argv[2]!, {mutationFamilies: "on", tests: {fixtures: true, aggregate: true, hoist: true, detach: true}});
console.log("problems", plan.problems);
for (const u of plan.movedUnits) console.log("MOVED", u.names.join(","), "->", plan.crates.find((c) => c.id === u.to)?.name);
console.log("kept", plan.keptModules);
