import { readFileSync } from "node:fs";
import { toolJobProductionSourcesOf } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🏭️factory/🟦️.ts";
import { toolJobRustBlock } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/📋️inventory/🟦️.ts";
const R = "/Users/ueli/Documents/semio";
const all: string[] = JSON.parse(readFileSync("/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/tjcov-listing.json", "utf8"));
const files = toolJobProductionSourcesOf(R, all.filter((f) => f.includes("✏️s/🔌️plugins/🧩️puzzle")));
for (const owner of ["Puzzle2dPlayApp", "Puzzle5dPlayApp", "Puzzle3dPlayApp"]) {
  const [file, source] = [...files].find(([, t]) => t.includes("impl ArtifactEditor for " + owner))!;
  const start = source.indexOf("impl ArtifactEditor for " + owner);
  const body = toolJobRustBlock(source, source.indexOf("{", start))!.body;
  console.log(owner, {
    reg: body.includes("fn register_tool_job_factories("), build: body.includes("fn build_reserved_tool_job("), media: body.includes("ArtifactReservedToolInput::Media"), job: body.includes("ArtifactReservedToolJob::new("),
    imp: source.includes('"import-media"'), implIJ: source.includes("impl InteractiveJob for "), ck: source.includes("CheckpointReady("), cancel: source.includes("cx.is_cancelled()"),
    implIJany: [...source.matchAll(/impl[^{]{0,80}InteractiveJob[^{]{0,80}\{/g)].map((m) => m[0]).slice(0, 3), ckany: [...source.matchAll(/\w*Checkpoint\w*\(/g)].map((m) => m[0]).slice(0, 5), cancelAny: [...source.matchAll(/\w+\.is_cancelled\(\)/g)].map((m) => m[0]).slice(0, 3),
  });
}
