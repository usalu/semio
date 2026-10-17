import {planArtifactCrates} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🟦️.ts";
const plan: any = await planArtifactCrates("/Users/ueli/Documents/semio", process.argv[2]!, {mutationFamilies: "on", tests: {fixtures: true, aggregate: true, hoist: true, detach: true}} as any);
const needle = process.argv[3] ?? "";
for (const [file, text] of plan.overlay.edits) if (String(file).includes(needle)) { console.log("=== EDIT", String(file).slice(-90)); console.log(text); }
for (const [file, text] of plan.overlay.creates) if (String(file).includes(needle)) { console.log("=== CREATE", String(file).slice(-90)); console.log(String(text).slice(0, 1500)); }
