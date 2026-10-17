// 🧪️ Prints the test-only source files of every planned crate (diagnostic).
import { planArtifactCrates } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🟦️.ts";
const plan = await planArtifactCrates("/Users/ueli/Documents/semio", process.argv[2]!, { mutationFamilies: (process.argv[3] ?? "off") as "off", tests: { fixtures: true, aggregate: process.env.AGG !== "0", hoist: true, detach: true }, lift: (process.argv[4] ?? "").split(",").filter(Boolean) });
for (const c of plan.crates) {
  if (!c.lib || !c.test) continue;
  const lib = new Set(c.lib.sourcePaths);
  const libAll = new Set(c.lib.paths); const extra = c.test.paths.filter((p) => !libAll.has(p));
  console.log(c.name.replace("semio-s-artifact-stdio-", ""), `lib ${c.lib.files} test ${c.test.files}`);
  if (process.argv[5] === c.name.split("-").slice(-1)[0] || c.test.files > 32) for (const p of extra) console.log("   ", p.split("/").slice(-6).join("/"));
}
