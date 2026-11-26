import { readFileSync } from "node:fs";
import { join } from "node:path";
import { BundleScript, ScriptRouter, receiveScriptProcessInvocation, runScriptMain } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { scriptCompilerAdmissionControlV1 } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { nativeBuildPipelinePlanV1 } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🟦️.ts";
import { repositoryCargoNetworkV1 } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🦀️cargo/🟦️.ts";
import { buildReleasePipelineV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🧩️driver-stages/🟦️.ts";

/** 🚀 Runs the repository's release pipeline for the selected browser components. */
class Release extends BundleScript {
  async run(args: string[]): Promise<void> {
    const [operation, ...ids] = args;
    const components = new Set(ids);
    if (!["plan", "build"].includes(operation) || !ids.length || ids.length !== components.size) throw Error("Expected plan or build and distinct component IDs");
    const pipeline = JSON.parse(readFileSync(join(this.repoRoot, "📋️project.json"), "utf8")).targets.release.metadata.semio.release;
    if (operation === "plan") {
      const { plan } = nativeBuildPipelinePlanV1(this.repoRoot, pipeline, "release", components);
      console.log(JSON.stringify(plan));
      return;
    }
    const outcome = await buildReleasePipelineV1(this.repoRoot, pipeline, {
      invocation: this.invocation, components, facet: "browser",
      network: repositoryCargoNetworkV1(this.invocation, this.repoRoot),
      control: scriptCompilerAdmissionControlV1(this.invocation, "demonstrator:release", 0),
    });
    if (outcome.failed.length || outcome.staged.length !== components.size) throw Error("Incomplete browser component staging: " + outcome.report);
    console.log("[release] " + outcome.report);
  }
}
const router = new ScriptRouter(import.meta.dirname).register("release", Release);
await receiveScriptProcessInvocation(process.env, invocation => runScriptMain(router, { invocation }));
