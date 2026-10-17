// 🔬️ Plans the pdf artifact and runs carveWrites to print the stack of a failure.
import {planArtifactCrates, carveWrites} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🟦️.ts";
const root = "/Users/ueli/Documents/semio";
const pkg = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust";
const plan = await planArtifactCrates(root, pkg, {mutationFamilies: "on", tests: {fixtures: true, aggregate: true, hoist: true, detach: true}});
console.log("planned", plan.crates.length, "problems", plan.problems.length);
try { const {writes} = await carveWrites(plan); console.log("writes", writes.length); } catch (e) { console.log((e as Error).stack); }
