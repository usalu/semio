import { join } from "node:path";

process.env.SEMIO_BUILD_MODE ??= "prototype";
const workspace = process.cwd();
const processRoot = join(workspace, "🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript");
const repoRoot = join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library");
const { Script, ScriptRouter } = await import(join(processRoot, "🧭️routing/🟦️.ts"));
const { runArtifactRustTests } = await import(join(processRoot, "🧪️testing/🦀️cargo/🟦️.ts"));
const { runWorkspaceScriptMain } = await import(join(repoRoot, "🚨️policy/🧭️entrypoints/🟦️.ts"));
const { createBuildBootstrapCapabilitiesV1 } = await import(join(repoRoot, "🏗️build/📥️caller/📤️child/🟦️.ts"));
const { issueRepositoryScriptProvidersV1 } = await import(join(repoRoot, "🏃️process/🔌️providers/🟦️.ts"));

class VerifyNoteInlineContent extends Script {
  async run(): Promise<void> {
    const note = join(workspace, "✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/📦️packages/🦀️rust");
    await runArtifactRustTests("semio-s-artifact-note-note", note, ["quick", "--lib", "inline_block_edits_replay_and_undo"], this.invocation);
  }
}

const args = process.argv.slice(2);
const router = new ScriptRouter(workspace, workspace).register("test", VerifyNoteInlineContent);
await runWorkspaceScriptMain(router, {
  freshCapabilities: async () => {
    issueRepositoryScriptProvidersV1(process.env);
    return createBuildBootstrapCapabilitiesV1(workspace, args, process.env);
  },
});
