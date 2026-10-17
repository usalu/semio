// Prints the full blocker report of a plan (read-only).
import {planArtifactCrates} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🟦️.ts";
const [pkg, lift] = process.argv.slice(2);
const plan = await planArtifactCrates("/Users/ueli/Documents/semio", pkg!, {mutationFamilies: (process.env.MF as "on" | "off" | "auto") ?? "on", lift: lift ? lift.split(",") : undefined, tests: {fixtures: true, aggregate: true, hoist: true, detach: true}});
console.log("problems", plan.problems);
for (const b of plan.blockers) {
  console.log("BLOCKER slices", b.slices, "files", b.files, "bytes", b.bytes, "lines", b.lines);
  console.log(" coherence", b.coherence);
  console.log(" causes", b.causes);
  console.log(" sample", b.sample);
  console.log(" edges", b.edges.slice(0, Number(process.env.EDGES ?? 40)));
  console.log(" rings", b.rings.slice(0, 20));
  console.log(" group", b.groupReport);
}
console.log("crates", plan.crates.map(c => `${c.name} ${c.lib?.files}/${c.lib?.bytes}/${c.lib?.lines} ${c.test?.files}/${c.test?.bytes}/${c.test?.lines}`));
