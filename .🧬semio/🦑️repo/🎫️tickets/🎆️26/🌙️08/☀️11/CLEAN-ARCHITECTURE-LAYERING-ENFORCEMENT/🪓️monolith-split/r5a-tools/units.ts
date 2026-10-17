// 🧱️ Prints the moved root units of a plan with the crate that hosts them (diagnostic).
import { planArtifactCrates } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🟦️.ts";
const plan = await planArtifactCrates("/Users/ueli/Documents/semio", process.argv[2]!, { mutationFamilies: "off", tests: { fixtures: true, aggregate: true, hoist: true, detach: true }, lift: (process.argv[3] ?? "").split(",").filter(Boolean) });
for (const unit of plan.movedUnits) console.log(plan.crates.find((c) => c.id === unit.to)?.name.replace("semio-s-artifact-stdio-", ""), unit.names.join(","));
console.log("kept", plan.keptModules.join(","));
for (const c of plan.crates) console.log(c.name.replace("semio-s-artifact-stdio-", ""), "attach:", c.attach.map((d) => `${d.eff.slice(-3).join("::")}${d.file ? "" : "(inline)"}${d.isComponent ? "(c)" : ""}`).join(" | ").slice(0, 400));
