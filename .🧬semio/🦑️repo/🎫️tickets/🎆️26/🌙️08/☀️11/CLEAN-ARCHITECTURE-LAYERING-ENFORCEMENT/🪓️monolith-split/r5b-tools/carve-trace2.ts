import {planArtifactCrates} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🟦️.ts";
const root = "/Users/ueli/Documents/semio";
const pkg = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust";
const plan = await planArtifactCrates(root, pkg, {mutationFamilies: "on", tests: {fixtures: true, aggregate: true, hoist: true, detach: true}});
for (const k of [...plan.overlay.creates.keys(), ...plan.overlay.edits.keys()]) if (k.includes("modules/🧪️tests") || k.includes("leaf_tests")) console.log("overlay", k);
console.log("deletes", [...(plan.overlay as any).deletes ?? []].filter((k: string) => k.includes("modules")).slice(0, 5));
console.log("rootSource has leaf_tests:", plan.tree.source.includes("leaf_tests"));
const idx = plan.tree.source.indexOf("leaf_tests"); console.log(plan.tree.source.slice(Math.max(0, idx - 200), idx + 60));
