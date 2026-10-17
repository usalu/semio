import {planArtifactCrates} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🟦️.ts";
const plan: any = await planArtifactCrates("/Users/ueli/Documents/semio", process.argv[2]!, {mutationFamilies: "auto", tests: {fixtures: true, aggregate: true, hoist: true, detach: true}} as any);
console.log(Object.keys(plan));
console.log(JSON.stringify(plan.packing, null, 1)?.slice(0, 6000));
console.log(JSON.stringify(plan.blockers.map((b: any) => Object.keys(b))));
