// 🔭️ Plans the carve and writes the text of the carved files whose path contains a given substring (dry: nothing in the tree changes).
import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { carveWrites, planArtifactCrates } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🟦️.ts";
const [pkg, outDir, ...needles] = process.argv.slice(2);
const plan: any = await planArtifactCrates("/Users/ueli/Documents/semio", pkg!, { mutationFamilies: "on", lift: "standards::v1::subsets::any::schema::*,standards::v1::subsets::any::io::*,editor::*,render::*".split(","), tests: { fixtures: true, aggregate: true, hoist: true, detach: true } } as any);
console.log("problems", plan.problems.length);
const { writes } = await carveWrites(plan);
let n = 0;
for (const write of writes) if (needles.some((needle) => write.path.includes(needle))) { const target = join(outDir!, String(n++) + ".rs.txt"); mkdirSync(dirname(target), { recursive: true }); writeFileSync(target, `// ${write.path}\n${write.content}`); }
console.log("dumped", n, "of", writes.length);
