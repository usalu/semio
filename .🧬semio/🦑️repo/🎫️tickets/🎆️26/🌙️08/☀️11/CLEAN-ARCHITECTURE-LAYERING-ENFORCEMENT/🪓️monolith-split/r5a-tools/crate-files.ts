// 📃️ Lists the lib and test-only files of one planned crate (diagnostic): crate-files.ts <pkg> <crate-suffix>
import { planArtifactCrates } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🟦️.ts";
const plan = await planArtifactCrates("/Users/ueli/Documents/semio", process.argv[2]!, { mutationFamilies: "off", tests: { fixtures: true, aggregate: true, hoist: true, detach: true } });
for (const c of plan.crates) if (c.name.endsWith(process.argv[3]!)) { for (const p of c.lib!.paths) console.log("lib ", p.split("/").slice(-5).join("/")); const l = new Set(c.lib!.paths); for (const p of c.test!.paths) if (!l.has(p)) console.log("test", p.split("/").slice(-5).join("/")); }
