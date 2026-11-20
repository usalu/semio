import { join } from "node:path";

const workspace = process.cwd();
const library = join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library");
const processRoot = join(workspace, "🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript");
const { Script, ScriptRouter, SCRIPT_PROCESS_INVOCATION_ENV, scriptInvocationEnvironment } = await import(join(processRoot, "🧭️routing/🟦️.ts"));
const { runOwnedCommand } = await import(join(processRoot, "🎛️execution/🟦️.ts"));
const { ComponentsScript } = await import(join(library, "🎛️commands/🚀️release/🟦️.ts"));
const { runWorkspaceScriptMain } = await import(join(library, "🚨️policy/🧭️entrypoints/🟦️.ts"));
const { createBuildBootstrapCapabilitiesV1 } = await import(join(library, "🏗️build/📥️caller/📤️child/🟦️.ts"));
const { issueRepositoryScriptProvidersV1 } = await import(join(library, "🏃️process/🔌️providers/🟦️.ts"));

const args = process.argv.slice(2);
const command = JSON.stringify(args);
const serveNote = command === '["serve-note"]';
if (command !== '["components","iterate","note"]' && command !== '["prepare-note-sources"]' && command !== '["prepare-note-assets"]' && !serveNote) {
  throw Error("Expected components iterate note, prepare-note-sources, prepare-note-assets or serve-note");
}
class ForwardComponentsScript extends Script {
  async run(): Promise<void> {
    await runOwnedCommand("repo", [process.argv[1]!, ...args], workspace, "stage:note", 3_600_000, {
      env: scriptInvocationEnvironment(this.invocation, { ...process.env, SEMIO_BUILD_MODE: "prototype" }),
    });
  }
}
class PrepareNoteSourcesScript extends Script {
  async run(): Promise<void> {
    const processEnv = { ...process.env, SEMIO_BUILD_MODE: "prototype" };
    const catalog = join(workspace, "✏️s/🧑‍💻dev/🧩️catalog");
    await runOwnedCommand("repo", [join(catalog, "📣️publication/📜️script.ts"), "generate"], workspace, "stage:note-catalog", 300_000, {
      env: scriptInvocationEnvironment(this.invocation, processEnv),
    });
    const registry = join(workspace, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry");
    await runOwnedCommand("repo", [join(registry, "📜️script.ts"), "session", "note"], workspace, "stage:note-session", 300_000, {
      env: scriptInvocationEnvironment(this.invocation, { ...processEnv, SEMIO_PLUGIN_CATALOG: join(catalog, "🤖️generated") }),
    });
  }
}

class PrepareNoteAssetsScript extends Script {
  async run(): Promise<void> {
    const env = scriptInvocationEnvironment(this.invocation, { ...process.env, SEMIO_BUILD_MODE: "prototype" });
    const os = join(workspace, "🧰️framework/🛍️products/💻️os/🔨️modules");
    const fontTool = join(workspace, "🧰️framework/🔨️modules/🖼️canvas/🔤️fonts/📦️packages/🦀️rust/📜️script.ts");
    const browser = runOwnedCommand("repo", [join(os, "🔌️plugin/📦️packages/🟦️typescript/📜️script.ts"), "support"], workspace, "stage:note-browser-support", 300_000, { env });
    const fonts = (async () => {
      await runOwnedCommand("repo", [fontTool, "build"], workspace, "stage:note-font-tool", 1_200_000, { env });
      await runOwnedCommand("repo", [join(os, "♾️infinite/📦️packages/🦀️rust/📜️script.ts"), "fonts"], workspace, "stage:note-fonts", 300_000, { env });
    })();
    const results = await Promise.allSettled([browser, fonts]);
    for (const result of results) if (result.status === "rejected") throw result.reason;
  }
}

class ServeNoteScript extends Script {
  async run(): Promise<void> {
    const osDev = join(workspace, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript");
    const catalog = join(workspace, "✏️s/🧑‍💻dev/🧩️catalog/🤖️generated");
    const env = scriptInvocationEnvironment(this.invocation, {
      ...process.env,
      SEMIO_BUILD_MODE: "prototype",
      SEMIO_PLUGIN_CATALOG: catalog,
    });
    await runOwnedCommand("repo", ["./📜️script.ts", "serve", "note", "react", "prototype"], osDev, "serve:note", 1_200_000, { env });
  }
}

const router = new ScriptRouter(workspace, workspace)
  .register("components", process.env[SCRIPT_PROCESS_INVOCATION_ENV] === undefined ? ForwardComponentsScript : ComponentsScript)
  .register("prepare-note-sources", PrepareNoteSourcesScript)
  .register("prepare-note-assets", PrepareNoteAssetsScript)
  .register("serve-note", ServeNoteScript);
await runWorkspaceScriptMain(router, {
  freshCapabilities: async () => {
    issueRepositoryScriptProvidersV1(process.env);
    return createBuildBootstrapCapabilitiesV1(workspace, args, { ...process.env, SEMIO_BUILD_MODE: "prototype" });
  },
});
