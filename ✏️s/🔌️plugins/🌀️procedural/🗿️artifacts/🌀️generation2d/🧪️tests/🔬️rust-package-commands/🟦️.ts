import type { ScriptInvocation } from "@semio-tech/framework/process/routing";
import { runCmd } from "@semio-tech/framework/process/execution";
import { runCargo } from "@semio-tech/framework/process/execution";
import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { resolve as scriptDirectoryResolve } from "node:path";

const SCRIPT_DIR = scriptDirectoryResolve(import.meta.dir, "../../📦️packages/🦀️rust");

const familyRoot = resolve(SCRIPT_DIR, "../..");
export const familyPackages = (Bun.TOML.parse(readFileSync(join(familyRoot, "Cargo.toml"), "utf8")) as { workspace: { members: string[] } }).workspace.members.map((member) => (Bun.TOML.parse(readFileSync(join(familyRoot, member, "Cargo.toml"), "utf8")) as { package: { name: string } }).package.name);

/** 🏃️ Runs the `OwnedVerifyScript` command under the caller's roots and invocation. */
export async function runOwnedVerifyScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, segments: string[]): Promise<void> {
if (segments[0] === "generation2d-window-camera-ownership") {
    const editorRoot = join(context.repoRoot, "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor");
    const oracle = join(editorRoot, "🧪️tests/🪟️generation2d-window-camera-ownership/🟦️.ts");
    const { testGeneration2dWindowCameraOwnershipOracle } = await import(oracle);
    testGeneration2dWindowCameraOwnershipOracle();
    runCmd("bun", [join(context.repoRoot, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--allowImportingTsExtensions", "--esModuleInterop", "--skipLibCheck", join(editorRoot, "🎭️modes/✏️edit/🪟️windows/🕸️flow/🎚️config/🧬️schema/🟦️.ts"), join(editorRoot, "🎭️modes/✏️edit/🪟️windows/👁️preview/🎚️config/🧬️schema/🟦️.ts"), join(editorRoot, "🎭️modes/🧬️generate/🪟️windows/👁️preview/🎚️config/🧬️schema/🟦️.ts"), oracle], { cwd: context.repoRoot });
    if (segments[1] === "native") {
      const { runCargo } = await import("@semio-tech/framework/process/execution");
      await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-procedural-2d-editor", "--test", "editor-routes", "editor_window_camera_ownership", "--", "--nocapture"], context.repoRoot);
    }
    return;
  }
  throw new Error('Unknown owned verification '+segments.join(' '));
}
