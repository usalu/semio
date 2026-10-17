import {planArtifactCrates} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🟦️.ts";
const plan: any = await planArtifactCrates("/Users/ueli/Documents/semio", process.argv[2]!, {mutationFamilies: "on", tests: {fixtures: true, aggregate: true, hoist: true, detach: true}} as any);
for (const n of plan.overlay.notes) console.log("note:", String(n).slice(0, 300));
console.log("crates", plan.crates.map((c: any) => `${c.name} lib ${c.lib?.files}/${c.lib?.bytes}/${c.lib?.lines} test ${c.test?.files}/${c.test?.bytes}/${c.test?.lines}`).join("\n"));
