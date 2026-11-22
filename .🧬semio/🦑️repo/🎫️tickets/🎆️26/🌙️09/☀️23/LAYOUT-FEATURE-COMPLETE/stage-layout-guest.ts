/** 📏️ Runs the repository's owned prototype browser descriptor producer for Layout. */
import { join } from "node:path";

const workspace = process.cwd();
const library = join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library");
const processRoot = join(workspace, "🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript");
const { Script, ScriptRouter, SCRIPT_PROCESS_INVOCATION_ENV, scriptInvocationEnvironment } = await import(join(processRoot, "🧭️routing/🟦️.ts"));
const { runOwnedCommand } = await import(join(processRoot, "🎛️execution/🟦️.ts"));
const { createBuildBootstrapCapabilitiesV1 } = await import(join(library, "🏗️build/📥️caller/📤️child/🟦️.ts"));
const { ComponentsScript } = await import(join(library, "🎛️commands/🚀️release/🟦️.ts"));
const { runWorkspaceScriptMain } = await import(join(library, "🚨️policy/🧭️entrypoints/🟦️.ts"));
const { issueRepositoryScriptProvidersV1 } = await import(join(library, "🏃️process/🔌️providers/🟦️.ts"));

const args = ["components", "describe", "layout", "--profile", "prototype", "--facet", "browser"];

class LayoutDescriptorScript extends Script {
  async run(): Promise<void> {
    await runOwnedCommand("repo", [process.argv[1]!, ...args], workspace, "stage:layout-guest", 0, {
      env: scriptInvocationEnvironment(this.invocation, { ...process.env, SEMIO_BUILD_MODE: "prototype" }),
    });
  }
}

if (JSON.stringify(process.argv.slice(2)) !== JSON.stringify(args)) {
  throw Error("Expected the owned Layout browser descriptor command");
}

const inherited = process.env[SCRIPT_PROCESS_INVOCATION_ENV] !== undefined;
const router = new ScriptRouter(workspace, workspace).register("components", inherited ? ComponentsScript : LayoutDescriptorScript);
await runWorkspaceScriptMain(router, {
  freshCapabilities: async () => {
    issueRepositoryScriptProvidersV1(process.env);
    return createBuildBootstrapCapabilitiesV1(workspace, args, { ...process.env, SEMIO_BUILD_MODE: "prototype" });
  },
});
