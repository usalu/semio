import { planArtifactCrates } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🟦️.ts";
const [pkg, needle] = process.argv.slice(2);
const plan: any = await planArtifactCrates("/Users/ueli/Documents/semio", pkg!, { mutationFamilies: "auto", tests: { fixtures: true, aggregate: true, hoist: true, detach: true } });
for (const crate of plan.crates) {
  for (const slice of crate.slices) for (const decl of slice.decls) {
    const eff = decl.eff.join("::");
    if (!needle || eff.includes(needle)) console.log(crate.name, "|", eff, decl.testOnly ? "[test]" : "", decl.file?.split("/").slice(-4).join("/") ?? "");
  }
}
