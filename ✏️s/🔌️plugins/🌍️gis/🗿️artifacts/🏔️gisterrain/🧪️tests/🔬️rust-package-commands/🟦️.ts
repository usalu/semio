import type { ScriptInvocation } from "@semio-tech/framework/process/routing";
import { runCmd } from "@semio-tech/framework/process/execution";
import { runCargo } from "@semio-tech/framework/process/execution";
import { join } from "node:path";

/** 🏃️ Runs the `OwnedVerifyScript` command under the caller's roots and invocation. */
export async function runOwnedVerifyScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, segments: string[]): Promise<void> {
if (segments[0] === "terrain-document-contract") {
    const { testTerrainDocumentContractOracle } = await import("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🪪️document/🟦️.ts");
    testTerrainDocumentContractOracle();
    const schemaRoot = join(context.repoRoot, "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema");
    runCmd("bun", [join(context.repoRoot, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--allowImportingTsExtensions", "--skipLibCheck", ...["🟦️.ts", "📸️snapshot/🟦️.ts", "🔺️diff/🟦️.ts"].map((file) => `${schemaRoot}/${file}`)], { cwd: context.repoRoot });
    if (segments[1] === "native") {
      const { runCargo } = await import("@semio-tech/framework/process/execution");
      await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-gis-gisterrain", "--lib", "terrain_document_contract", "--", "--nocapture"], context.repoRoot);
    }
    return;
  }
if (segments[0] === "gis-terrain-window-config") {
    const testRoot = join(context.repoRoot, "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/👁️view/🪟️windows/🏔️terrain/🎚️config");
    const { testGisTerrainWindowConfigContract } = await import(`${testRoot}/🧪️tests/🔬️contract/🟦️.ts`);
    testGisTerrainWindowConfigContract();
    runCmd("bun", [join(context.repoRoot, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--allowImportingTsExtensions", "--skipLibCheck", `${testRoot}/🧬️schema/🟦️.ts`, `${testRoot}/🧪️tests/🔬️contract/🟦️.ts`], { cwd: context.repoRoot });
    if (segments[1] === "native") {
      const { runCargo } = await import("@semio-tech/framework/process/execution");
      await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-gis-gisterrain", "--lib", "gis_terrain_window_config_", "--", "--nocapture"], context.repoRoot);
    }
    return;
  }
  throw new Error('Unknown owned verification '+segments.join(' '));
}
