// 🔍️ Prints the carved facade root text and the lib root of one planned crate (argv[4] = crate-name substring).
import {planArtifactCrates, carveArtifactRootText, carveCrateRootText} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🟦️.ts";
const [pkg, lift, which] = process.argv.slice(2);
const plan = await planArtifactCrates("/Users/ueli/Documents/semio", pkg!, {mutationFamilies: (process.env.MF as any) ?? "auto", lift: (lift ?? "").split(",").filter(Boolean), tests: {fixtures: true, aggregate: true, hoist: true, detach: true}});
if (which === "facade") console.log(await carveArtifactRootText(plan));
else for (const c of plan.crates) if (c.name.includes(which!)) console.log("=== " + c.name + "\n" + carveCrateRootText(plan, c));
