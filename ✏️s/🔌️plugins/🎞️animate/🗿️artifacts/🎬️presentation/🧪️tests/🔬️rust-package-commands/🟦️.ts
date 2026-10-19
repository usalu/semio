import type { ScriptInvocation } from "@semio-tech/framework/process/routing";
import { runCmd } from "@semio-tech/framework/process/execution";
import { runCargo } from "@semio-tech/framework/process/execution";
import { join } from "node:path";

/** 🏃️ Runs the `OwnedVerifyScript` command under the caller's roots and invocation. */
export async function runOwnedVerifyScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, segments: string[]): Promise<void> {
if (segments[0] === "presentation-document-contract") {
    const { runCargo } = await import("@semio-tech/framework/process/execution");
    const { testPresentationDocumentContractOracle } = await import("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🪪️document-contract/🟦️.ts");
    testPresentationDocumentContractOracle();
    const schemaRoot = join(context.repoRoot, "✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema");
    runCmd("bun", [join(context.repoRoot, "node_modules/typescript/bin/tsc"), ...["--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--allowImportingTsExtensions", "--resolveJsonModule", "--esModuleInterop", "--skipLibCheck", ...["🟦️.ts", "📸️snapshot/🟦️.ts", "🔺️diff/🟦️.ts", "📸️snapshot/🧪️tests/🪶️sqlite/🟦️.ts"].map((file) => `${schemaRoot}/${file}`)]], { cwd: context.repoRoot });
    if (segments[1] === "native") await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-animate-presentation", "--lib", "presentation_document_contract", "--", "--nocapture"], context.repoRoot);
    return;
  }
  throw new Error('Unknown owned verification '+segments.join(' '));
}
