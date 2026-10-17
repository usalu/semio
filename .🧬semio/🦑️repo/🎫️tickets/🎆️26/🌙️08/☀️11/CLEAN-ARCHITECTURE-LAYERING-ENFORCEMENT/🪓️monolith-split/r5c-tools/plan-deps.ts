// 🔍️ Prints crate dependencies of a plan. Usage: <pkg> <lift>
import {planArtifactCrates} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🟦️.ts";
const [pkg, lift] = process.argv.slice(2);
const plan = await planArtifactCrates("/Users/ueli/Documents/semio", pkg!, {mutationFamilies: (process.env.MF as any) ?? "auto", lift: (lift ?? "").split(",").filter(Boolean), tests: {fixtures: true, aggregate: true, hoist: true, detach: true}});
const name = (id: string) => plan.crates.find(c => c.id === id)!.name.replace(/^semio-s-artifact-stdio-gltf-/, "");
for (const c of plan.crates) console.log(`${name(c.id)} <- [${c.dependsOn.map(name).join(", ")}] slices=${c.slices.length} ${c.slices.slice(0, 2).map(s => s.attach[0]?.eff.slice(-2).join("::")).join(" | ")}`);
