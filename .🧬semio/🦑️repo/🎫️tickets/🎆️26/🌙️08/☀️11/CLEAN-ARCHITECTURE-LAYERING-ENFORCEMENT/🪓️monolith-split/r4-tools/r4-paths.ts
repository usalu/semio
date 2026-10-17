import {planArtifactCrates, carveWrites, measureRustCrate} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🟦️.ts";
import {resolve} from "node:path";
const [pkg, needle, mode] = process.argv.slice(2);
const plan = await planArtifactCrates("/Users/ueli/Documents/semio", pkg!, {mutationFamilies: "on", tests: {fixtures: true, aggregate: true, hoist: true, detach: true}});
if (mode === "root") {
  const {rootTest, rootLib} = await carveWrites(plan);
  for (const [label, m] of [["rootLib", rootLib], ["rootTest", rootTest]] as const) { console.log(label, m.files, m.bytes, m.lines); const rows = [...m.entries].map(([p, e]) => [p, (e as any).bytes ?? 0] as const).sort((a, b) => b[1] - a[1]); for (const [p, b] of rows.slice(0, 120)) console.log(" ", b, p.split("/").slice(-6).join("/")); }
} else for (const c of plan.crates) if (c.name.includes(needle ?? "")) { console.log("CRATE", c.name, c.lib?.files); for (const [p, m] of c.lib!.entries) console.log(" ", (m as any).bytes ?? "", p.split("/").slice(-5).join("/")); }
