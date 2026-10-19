import { readFileSync } from "node:fs";
import { toolJobProductionSourcesOf } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🏭️factory/🟦️.ts";
const R = "/Users/ueli/Documents/semio";
const all: string[] = JSON.parse(readFileSync("/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/tjcov-listing.json", "utf8"));
const inv = JSON.parse(readFileSync(R + "/" + process.argv[2]!, "utf8"));
const files = toolJobProductionSourcesOf(R, all.filter((f) => f.includes("✏️s/🔌️plugins/")));
let ok = 0;
for (const row of inv.importers) {
  const t = files.get(row.file);
  const has = t?.includes("impl ArtifactEditor for " + row.owner);
  if (!t || !has) { const alt = [...files].filter(([f, x]) => x.includes("impl ArtifactEditor for " + row.owner)).map(([f]) => f.split("/").slice(-6).join("/")); console.log(row.owner, "inMap", !!t, "impl", !!has, "alt", alt, row.file.split("/").slice(-5).join("/")); } else ok++;
}
console.log("ok", ok, "of", inv.importers.length);
