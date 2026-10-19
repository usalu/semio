import type { ScriptInvocation } from "@semio-tech/framework/process/routing";
import { runCargo } from "@semio-tech/framework/process/execution";
import { join } from "node:path";

/** 🏃️ Runs the `OwnedVerifyScript` command under the caller's roots and invocation. */
export async function runOwnedVerifyScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, segments: string[]): Promise<void> {
if (segments[0] === "wires-document-contract") {
    const { testWiresDocumentContractOracle } = await import("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🪪️document-contract/🟦️.ts");
    testWiresDocumentContractOracle();
    if (segments[1] === "oracle") return;
    const { runCargo } = await import("@semio-tech/framework/process/execution");
    await runCargo(["test", "--manifest-path", "Cargo.toml", "--lib", "committed_diff", "--", "--nocapture"], join(context.repoRoot, "✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/📦️packages/🦀️rust"));
    return;
  }
if (segments[0] === "wires-window-transient") {
    const { runCargo } = await import("@semio-tech/framework/process/execution");
    await runCargo(["test", "--manifest-path", "Cargo.toml", "--lib", "window_transient", "--", "--nocapture"], join(context.repoRoot, "✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/📦️packages/🦀️rust"));
    return;
  }
  throw new Error('Unknown owned verification '+segments.join(' '));
}
