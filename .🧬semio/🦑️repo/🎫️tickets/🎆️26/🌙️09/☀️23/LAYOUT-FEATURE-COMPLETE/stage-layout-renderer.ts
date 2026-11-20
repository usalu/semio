/** 🧊️ Publishes the prototype WGPU WebAssembly renderer through its repository owner. */
import { join } from "node:path";
import { setTimeout } from "node:timers/promises";

const workspace = process.cwd();
const framework = join(workspace, "🧰️framework");
const library = join(framework, "🛍️products/🦑️repo/🔨️modules/📚️library");
const processRoot = join(framework, "🔨️modules/🏃️process/📦️packages/🟦️typescript");
const owner = join(framework, "🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🏗️compiler/🌐️wasm/📜️script.ts");

const { Script, ScriptRouter, scriptInvocationEnvironment } = await import(join(processRoot, "🧭️routing/🟦️.ts"));
const { OwnedCommandFailure, runOwnedCommand } = await import(join(framework, "🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts"));
const { createBuildBootstrapCapabilitiesV1 } = await import(join(library, "🏗️build/📥️caller/📤️child/🟦️.ts"));
const { runWorkspaceScriptMain } = await import(join(library, "🚨️policy/🧭️entrypoints/🟦️.ts"));
const { issueRepositoryScriptProvidersV1 } = await import(join(library, "🏃️process/🔌️providers/🟦️.ts"));

const environment = () => ({ ...process.env, SEMIO_BUILD_MODE: "prototype" });

class PrepareRendererScript extends Script {
  async run(remaining: string[]): Promise<void> {
    if (remaining.length) throw Error("prepare-renderer accepts no arguments");
    const env = scriptInvocationEnvironment(this.invocation, environment());
    const signal = this.invocation.control.signal;
    const maximumAttempts = 6;
    for (let attempt = 1; attempt <= maximumAttempts; attempt++) {
      try {
        await runOwnedCommand("repo", [owner, "build", "prototype"], workspace, "stage:layout-renderer", 0, { env, signal });
        return;
      } catch (error) {
        if (!(error instanceof OwnedCommandFailure) || error.outcome.status !== 75 || attempt === maximumAttempts) throw error;
        console.log(`[DEBUG] Retrying temporary renderer refusal (attempt ${attempt + 1} of ${maximumAttempts})`);
        await setTimeout(1000 * attempt, undefined, { signal });
      }
    }
  }
}

const router = new ScriptRouter(workspace, workspace).register("prepare-renderer", PrepareRendererScript);
await runWorkspaceScriptMain(router, {
  freshCapabilities: async () => {
    issueRepositoryScriptProvidersV1(process.env);
    return createBuildBootstrapCapabilitiesV1(workspace, process.argv.slice(2), environment());
  },
});
