/** 🔤️ Prepares the prototype WGPU font pack through its repository-owned producer. */
import { join } from "node:path";

const workspace = process.cwd();
const framework = join(workspace, "🧰️framework");
const library = join(framework, "🛍️products/🦑️repo/🔨️modules/📚️library");
const processRoot = join(framework, "🔨️modules/🏃️process/📦️packages/🟦️typescript");

const { Script, ScriptRouter, scriptInvocationEnvironment } = await import(join(processRoot, "🧭️routing/🟦️.ts"));
const { runOwnedCommand } = await import(join(processRoot, "🎛️execution/🟦️.ts"));
const { createBuildBootstrapCapabilitiesV1 } = await import(join(library, "🏗️build/📥️caller/📤️child/🟦️.ts"));
const { runWorkspaceScriptMain } = await import(join(library, "🚨️policy/🧭️entrypoints/🟦️.ts"));
const { issueRepositoryScriptProvidersV1 } = await import(join(library, "🏃️process/🔌️providers/🟦️.ts"));

const environment = () => ({ ...process.env, SEMIO_BUILD_MODE: "prototype" });
const args = process.argv.slice(2);
const tool = join(framework, "🔨️modules/🖼️canvas/🔤️fonts/📦️packages/🦀️rust/📜️script.ts");
const fonts = join(framework, "🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/📜️script.ts");

class PrepareFontsScript extends Script {
  async run(remaining: string[]): Promise<void> {
    if (remaining.some(value => value !== "--from-published-tool") || remaining.length > 1) throw Error("prepare-fonts accepts only --from-published-tool");
    const env = scriptInvocationEnvironment(this.invocation, environment());
    const options = { env, signal: this.invocation.control.signal };
    if (!remaining.length) await runOwnedCommand("repo", [tool, "build"], workspace, "stage:layout-font-tool", 1_200_000, options);
    await runOwnedCommand("repo", [fonts, "fonts"], workspace, "stage:layout-fonts", 300_000, options);
  }
}

const router = new ScriptRouter(workspace, workspace).register("prepare-fonts", PrepareFontsScript);
await runWorkspaceScriptMain(router, {
  freshCapabilities: async () => {
    issueRepositoryScriptProvidersV1(process.env);
    return createBuildBootstrapCapabilitiesV1(workspace, args, environment());
  },
});
