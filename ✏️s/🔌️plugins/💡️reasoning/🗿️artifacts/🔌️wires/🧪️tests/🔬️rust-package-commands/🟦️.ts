import type { ScriptInvocation } from "@semio-tech/process/routing";
import { runCargo } from "@semio-tech/process/execution";
import { join } from "node:path";

/** 🏃️ Runs the `OwnedVerifyScript` command under the caller's roots and invocation. */
export async function runOwnedVerifyScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, segments: string[]): Promise<void> {
if (segments[0] === "wires-document-contract") {
    const { testWiresDocumentContractOracle } = await import("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🪪️document-contract/🟦️.ts");
    testWiresDocumentContractOracle();
    if (segments[1] === "oracle") return;
    const { runCargo } = await import("@semio-tech/process/execution");
    await runCargo(["test", "--manifest-path", "Cargo.toml", "--lib", "committed_diff", "--", "--nocapture"], join(context.repoRoot, "✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/📦️packages/🦀️rust"));
    return;
  }
if (segments[0] === "wires-window-transient") {
    const { runCargo } = await import("@semio-tech/process/execution");
    await runCargo(["test", "--manifest-path", "Cargo.toml", "--lib", "window_transient", "--", "--nocapture"], join(context.repoRoot, "✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/📦️packages/🦀️rust"));
    return;
  }
    if (segments[0] === "wires-pointer-move") {
      await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-reasoning-wires", "--lib", "wires_pointer_move_", "--", "--nocapture"], context.repoRoot);
      return;
    }
    if (segments[0] === "wires-reset-document-ownership") {
      await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-reasoning-wires", "--lib", "reset_document_ownership", "--", "--nocapture"], context.repoRoot);
      return;
    }
  throw new Error('Unknown owned verification '+segments.join(' '));
}
