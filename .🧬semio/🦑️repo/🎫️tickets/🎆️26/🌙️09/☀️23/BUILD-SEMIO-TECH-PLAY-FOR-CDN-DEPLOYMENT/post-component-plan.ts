import { readFileSync, existsSync } from "node:fs";
import { join } from "node:path";
import { nativeBuildPipelinePlanV1 } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🦀️cargo/🟦️.ts";
import { playRuntimeComponentIds } from "../../../../../../../🏢️semio-tech/🎡️play/🔨️modules/🧩️runtime/🟦️.ts";
const root = process.argv[2]!, play = join(root, "🏢️semio-tech/🎡️play");
const pipeline = JSON.parse(readFileSync(join(play, "📋️project.json"), "utf8")).targets.build.metadata.semio.release;
const { plan, jobEnvironment } = nativeBuildPipelinePlanV1(root, pipeline, "release", new Set(playRuntimeComponentIds()));
console.log("passes", plan.passes.length, "components", plan.components?.length, "jobs", plan.jobs?.length ?? Object.keys(plan));
for (const job of pipeline.jobs) { const script = job.command[1]; const dir = join(root, job.root); console.log(existsSync(dir) ? "ok " : "MISSING root", job.id, existsSync(join(dir, script)) ? "script ok" : `MISSING script ${script}`); }
