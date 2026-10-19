import { readFileSync, writeFileSync } from "node:fs";
import { toolJobProductionSourcesOf } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🏭️factory/🟦️.ts";
import { toolJobRustBlock } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/📋️inventory/🟦️.ts";
const R = "/Users/ueli/Documents/semio";
const all: string[] = JSON.parse(readFileSync("/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/tjcov-listing.json", "utf8"));
const path = process.argv[2]!;
const inv = JSON.parse(readFileSync(R + "/" + path, "utf8"));
const files = toolJobProductionSourcesOf(R, all.filter((f) => f.includes("✏️s/🔌️plugins/")));
const renamed: Record<string, string> = { Procedural2dPlayApp: "Generation2dPlayApp", Procedural3dPlayApp: "Generation3dPlayApp", AnimatePresentPlayApp: "AnimatePresentationPlayApp", RemodelPlayApp: "RemodelingPlayApp", TrinityRewritePlayApp: "TrinityRewritingPlayApp", SourcingCurateApp: "SourcingCurationApp" };
const corpus = [...files.values()].join("\n");
const out = [];
for (const row of inv.importers) {
  const owner = renamed[row.owner] ?? row.owner;
  const hits = [...files].filter(([, t]) => t.includes("impl ArtifactEditor for " + owner));
  if (hits.length !== 1) { console.log("UNRESOLVED", row.owner, owner, hits.length); out.push(row); continue; }
  const [file, source] = hits[0]!;
  const start = source.indexOf("impl ArtifactEditor for " + owner);
  const body = toolJobRustBlock(source, source.indexOf("{", start))!.body;
  const migrated = body.includes("fn register_tool_job_factories(") && body.includes("fn build_reserved_tool_job(") && body.includes("ArtifactReservedToolInput::Media") && body.includes("ArtifactReservedToolJob::new(") && source.includes('"import-media"') && source.includes("impl InteractiveJob for ") && source.includes("cx.is_cancelled()");
  const idents = [...String(row.output).matchAll(/\b([A-Z][A-Za-z0-9]*Mutation)\b/g)].map((m) => m[1]!);
  const missing = idents.filter((i) => !new RegExp("\\b" + i + "\\b").test(corpus));
  console.log(row.owner === owner ? "same" : "renamed", owner, "migrated", migrated, "was", row.currentMonolith, missing.length ? "MISSING-IDENTS " + missing : "", file.split("/").slice(-5).join("/"));
  const ty = (n: string) => body.match(new RegExp("type\\s+" + n + "\\s*=\\s*([^;]+);"))?.[1]?.trim();
  const draft = String(row.output).endsWith("NoDraftMutation>") ? "NoDraftMutation" : "Self::DraftMutation";
  out.push({ ...row, owner, file, output: "Emit<" + ty("Mutation") + ", " + ty("ConfigMutation") + ", " + draft + ">", currentMonolith: !migrated });
}
if (process.env.WRITE) writeFileSync(R + "/" + path, JSON.stringify({ ...inv, importers: out }, null, 2) + "\n");
