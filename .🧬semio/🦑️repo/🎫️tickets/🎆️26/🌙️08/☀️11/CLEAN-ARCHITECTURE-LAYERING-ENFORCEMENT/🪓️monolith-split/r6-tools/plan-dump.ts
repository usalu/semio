// 🔎️ Dumps the carve writes of a package (content of every created crate root and the rewritten artifact root) to a directory outside the repo, without touching the tree.
import {mkdirSync, writeFileSync} from "node:fs";
import {dirname, join} from "node:path";
import {carveWrites, planArtifactCrates} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🟦️.ts";
const [pkg, out, mode] = process.argv.slice(2);
const plan = await planArtifactCrates("/Users/ueli/Documents/semio", pkg!, {mutationFamilies: (mode as "on" | "off" | "auto") ?? "auto", tests: {fixtures: true, aggregate: true, hoist: true, detach: true}});
if (plan.problems.length) { console.log("problems", plan.problems); process.exit(1); }
const {writes} = await carveWrites(plan);
for (const write of writes) { if (write.action === "delete") continue; const target = join(out!, write.path); mkdirSync(dirname(target), {recursive: true}); writeFileSync(target, write.content); }
console.log(`${writes.length} writes to ${out}`);
