import { readFileSync, writeFileSync } from "node:fs";
import { toolJobProductionSourcesOf } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🏭️factory/🟦️.ts";
const R = "/Users/ueli/Documents/semio";
const all: string[] = JSON.parse(readFileSync("/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/tjcov-listing.json", "utf8"));
const path = process.argv[2]!;
const inv = JSON.parse(readFileSync(R + "/" + path, "utf8"));
const files = toolJobProductionSourcesOf(R, all.filter((f) => f.includes("✏️s/🔌️plugins/")));
const owners = new Map(inv.importers.map((r: any) => [r.owner, files.get(r.file) ?? ""]));
for (const row of inv.importers) {
  const src = String(owners.get(row.owner));
  const decl = src.match(new RegExp("impl\\s+ArtifactEditor\\s+for\\s+" + row.owner + "\\s*\\{([\\s\\S]{0,4000})"))?.[1] ?? "";
  const ty = (n: string) => decl.match(new RegExp("type\\s+" + n + "\\s*=\\s*([^;]+);"))?.[1]?.trim();
  console.log(row.owner, "|", row.output, "=>", "Mutation", ty("Mutation"), "ConfigMutation", ty("ConfigMutation"), "DraftMutation", ty("DraftMutation"));
}
