import { readFileSync } from "node:fs";
import { toolJobProductionSourcesOf } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🏭️factory/🟦️.ts";
const R = "/Users/ueli/Documents/semio";
const all: string[] = JSON.parse(readFileSync("/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/tjcov-listing.json", "utf8"));
const inv = JSON.parse(readFileSync(R + "/" + process.argv[2]!, "utf8"));
const files = toolJobProductionSourcesOf(R, all.filter((f) => f.includes("✏️s/🔌️plugins/")));
const impls: { file: string; owner: string; plugin: string }[] = [];
for (const [f, t] of files) for (const m of t.matchAll(/impl\s+(?:semio_framework_plugin::)?ArtifactEditor\s+for\s+([A-Za-z_]\w*)\s*\{/g)) impls.push({ file: f, owner: m[1]!, plugin: f.split("/")[2]! });
await Bun.write("/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/tjcov-impls.json", JSON.stringify(impls));
for (const row of inv.importers) {
  const plugin = row.file.split("/")[2]!;
  const exact = impls.filter((i) => i.owner === row.owner);
  if (exact.length === 1) continue;
  console.log(row.owner, exact.length, "| plugin", plugin, "|", impls.filter((i) => i.plugin === plugin).map((i) => i.owner + "@" + i.file.split("/").slice(3, 5).join("/")).join(", "));
}
console.log("impl count", impls.length, "unique owners", new Set(impls.map((i) => i.owner)).size);
