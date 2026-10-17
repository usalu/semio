import { planArtifactCrates } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🟦️.ts";
const [pkg, name] = process.argv.slice(2);
const plan: any = await planArtifactCrates("/Users/ueli/Documents/semio", pkg!, { mutationFamilies: "auto", tests: { fixtures: true, aggregate: true, hoist: true, detach: true } });
for (const crate of plan.crates) if (crate.name === name) {
  console.log(Object.keys(crate));
  for (const i of crate.imports) console.log(i.crateIdent, i.glob ? "glob" : "item", i.path.join("::"));
}
