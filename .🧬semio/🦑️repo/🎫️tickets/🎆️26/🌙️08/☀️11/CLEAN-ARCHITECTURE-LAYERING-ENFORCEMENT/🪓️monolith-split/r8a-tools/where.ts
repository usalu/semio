// usage: bun where.ts <pkg> <substr>...: prints the crate that mounts each file whose path contains a substring
import { planArtifactCrates } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🟦️.ts";
const [pkg, ...subs] = process.argv.slice(2);
const plan = await planArtifactCrates("/Users/ueli/Documents/semio", pkg!, { mutationFamilies: "on", tests: { fixtures: true, aggregate: false, hoist: true, detach: true } });
const c0: any = plan.crates[0];
for (const crate of plan.crates as any[]) {
  const lists: string[] = [...(crate.lib.paths ?? [])];
  for (const f of lists) if (subs.some((s) => f.includes(s))) console.log(crate.name, "::", f.slice(-110));
}
