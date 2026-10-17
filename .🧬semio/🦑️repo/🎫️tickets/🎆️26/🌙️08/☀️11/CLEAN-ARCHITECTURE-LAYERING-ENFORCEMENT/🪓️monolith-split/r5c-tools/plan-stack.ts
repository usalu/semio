// 🔍️ Runs the crate planner on one package and prints the full stack of a failure.
import {planArtifactCrates} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🟦️.ts";
const [pkg, lift] = process.argv.slice(2);
try {
  const plan = await planArtifactCrates("/Users/ueli/Documents/semio", pkg!, {mutationFamilies: (process.env.MF as any) ?? "auto", lift: (lift ?? "").split(",").filter(Boolean), tests: {fixtures: true, aggregate: true, hoist: true, detach: true}});
  console.log(plan.crates.map(c => `${c.name} lib ${c.lib?.files}/${c.lib?.bytes}/${c.lib?.lines} test ${c.test?.files}/${c.test?.bytes}/${c.test?.lines}`).join("\n"));
  console.log(plan.problems.join("\n"));
  for (const b of plan.blockers) { console.log(`BLOCKER slices=${b.slices} files=${b.files} bytes=${b.bytes}`); for (const c of b.coherence) console.log("  COH " + c.replace(/standards::v2_0::subsets::any::/g, "")); for (const c of b.causes.slice(0, Number(process.env.CAUSES ?? 0))) console.log("  CAUSE " + c.replace(/standards::v2_0::subsets::any::/g, "")); for (const c of b.rings.slice(0, Number(process.env.CAUSES ?? 0))) console.log("  RING " + c.replace(/standards::v2_0::subsets::any::/g, "").slice(0, 600)); }
} catch (e) { console.log((e as Error).stack); }
