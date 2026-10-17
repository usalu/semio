// 🔭️ Prints modules kept in the artifact root, moved units and problems of one artifact plan.
import {planArtifactCrates} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🟦️.ts";
const plan: any = await planArtifactCrates("/Users/ueli/Documents/semio", process.argv[2]!, {mutationFamilies: (process.env.MF ?? "auto"), tests: {fixtures: true, aggregate: true, hoist: true, detach: true}} as any);
console.log("keptModules", JSON.stringify(plan.keptModules));
console.log("movedUnits", JSON.stringify(plan.movedUnits));
console.log("problems", JSON.stringify(plan.problems, null, 1));
console.log("blockers", JSON.stringify(plan.blockers, null, 1));
console.log("rootEdits", JSON.stringify(plan.rootEdits)?.slice(0, 1500));
