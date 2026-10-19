import type { ScriptInvocation } from "@semio-tech/framework/process/routing";
import { runCargo } from "@semio-tech/framework/process/execution";
import { join } from "node:path";

/** 🏃️ Runs the `OwnedVerifyScript` command under the caller's roots and invocation. */
export async function runOwnedVerifyScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, segments: string[]): Promise<void> {
if (segments[0] === "writer-window-state") {
    const { testWriterWindowStateOracle } = await import("../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/✒️main/🎚️config/🧪️tests/🔬️window-state/🟦️.ts");
    testWriterWindowStateOracle();
    if (segments[1] === "native") {
      const { runCargo } = await import("@semio-tech/framework/process/execution");
      await runCargo(["test", "--manifest-path", "Cargo.toml", "--lib", "writer_window_state_", "--", "--nocapture"], join(context.repoRoot, "✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/📦️packages/🦀️rust"));
    }
    return;
  }
  throw new Error('Unknown owned verification '+segments.join(' '));
}
