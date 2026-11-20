/** 🧭️ Runs the authored source publishers needed for the prototype Layout session. */
import { join } from "node:path";

const workspace = process.cwd();
const framework = join(workspace, "🧰️framework");
const library = join(framework, "🛍️products/🦑️repo/🔨️modules/📚️library");
const os = join(framework, "🛍️products/💻️os/🔨️modules");
const dev = join(workspace, "✏️s/🧑‍💻dev");
const processRoot = join(framework, "🔨️modules/🏃️process/📦️packages/🟦️typescript");

const { Script, ScriptRouter, scriptInvocationEnvironment } = await import(join(processRoot, "🧭️routing/🟦️.ts"));
const { runOwnedCommand } = await import(join(processRoot, "🎛️execution/🟦️.ts"));
const { createBuildBootstrapCapabilitiesV1 } = await import(join(library, "🏗️build/📥️caller/📤️child/🟦️.ts"));
const { runWorkspaceScriptMain } = await import(join(library, "🚨️policy/🧭️entrypoints/🟦️.ts"));
const { issueRepositoryScriptProvidersV1 } = await import(join(library, "🏃️process/🔌️providers/🟦️.ts"));

const environment = () => ({
  ...process.env,
  SEMIO_BUILD_MODE: "prototype",
  SEMIO_PLUGIN_CATALOG: join(dev, "🧩️catalog/🤖️generated"),
  SEMIO_WGPU_COMPOSITION_PATH: "✏️s/🧑‍💻dev/🧩️catalog/🧊️wgpu/🟦️.ts",
});
const args = process.argv.slice(2);
const tasks = {
  "publish-frame-worker": [[join(os, "📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/📜️script.ts"), "generate-frame-worker"]],
  "publish-format-claims": [[join(dev, "📜️script.ts"), "stdio-format-claims"]],
  "check-descriptors": [[join(os, "🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts"), "channel-version", "check-descriptors"]],
  "stage-layout-catalog": [
    [join(os, "🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts"), "channel-version", "check-descriptors"],
    [join(dev, "🧩️catalog/📣️publication/📜️script.ts"), "generate"],
    [join(os, "🔌️plugin/📇️registry/📜️script.ts"), "session", "layout"],
  ],
  "activate-layout": [[join(os, "🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts"), "activate", "layout", "wgpu", "prototype"]],
  "serve-layout-wgpu": [[join(os, "📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️server/📜️script.ts"), "serve", "layout", "prototype", "--port", "6179"]],
} as const;

class LayoutStageScript extends Script {
  async run(remaining: string[]): Promise<void> {
    if (remaining.length || !Object.hasOwn(tasks, args[0])) throw Error("Expected a Layout source stage");
    const options = {
      env: scriptInvocationEnvironment(this.invocation, environment()),
      signal: this.invocation.control.signal,
    };
    for (const command of tasks[args[0] as keyof typeof tasks]) {
      const deadline = args[0] === "serve-layout-wgpu" ? 3_600_000 : 300_000;
      await runOwnedCommand("repo", [...command], workspace, "stage:layout:" + command[1], deadline, options);
    }
  }
}

const router = new ScriptRouter(workspace, workspace);
for (const task of Object.keys(tasks)) router.register(task, LayoutStageScript);

await runWorkspaceScriptMain(router, {
  freshCapabilities: async () => {
    issueRepositoryScriptProvidersV1(process.env);
    return createBuildBootstrapCapabilitiesV1(workspace, args, environment());
  },
});
