#!/usr/bin/env bun
/** 📦️ cad cad Rust artifact package router. */
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts";
import { cadPresenceRetirementSelfTests } from "../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧪️tests/🔬️cad-presence-retirement/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { runCmd, runCargo, runVitest } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { existsSync, readFileSync, readdirSync } from "node:fs";
import { dirname, extname, join, relative, resolve } from "node:path";
import { runRepositoryCommand } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🎛️owned-execution/🟦️.ts";

/** 🧪️ Executes the contracts owned by this component. */
class OwnedVerifyScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length === 1 && segments[0] === "snapshot-sqlite-source") {
      const snapshot = resolve(this.root, "../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot");
      await runRepositoryCommand(process.execPath, [resolve(this.repoRoot, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--allowImportingTsExtensions", "--resolveJsonModule", "--esModuleInterop", "--skipLibCheck", resolve(snapshot, "🟦️.ts"), resolve(snapshot, "🧪️tests/🪶️sqlite/🟦️.ts")], this.repoRoot, "cad-snapshot-sqlite-public-types");
      return;
    }
if (segments[0] === "cad-document-contract") {
      const { testCadDocumentContractOracle } = await import("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🪪️document/🟦️.ts");
      testCadDocumentContractOracle();
      const { testCadWorldWindowTransientContract } = await import("../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🫧️transient/🧬️schema/🧪️tests/🪪️document/🟦️.ts");
      console.log(`cad-window-transient-contract rows=${testCadWorldWindowTransientContract()}`);
      const schemaRoot = join(this.repoRoot, "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema");
      runCmd("bun", [join(this.repoRoot, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--allowImportingTsExtensions", "--esModuleInterop", "--skipLibCheck", ...["🟦️.ts", "📸️snapshot/🟦️.ts", "🔺️diff/🟦️.ts", "🧪️tests/🪪️document/🟦️.ts"].map((file) => `${schemaRoot}/${file}`)], { cwd: this.repoRoot });
      if (segments[1] === "native") {
        const { runCargo } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts");
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-cad-cad", "--lib", "cad_document_contract", "--", "--nocapture"], this.repoRoot);
      }
      return;
    }
    throw new Error('Unknown owned verification '+segments.join(' '));
  }
}

await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-cad-cad", { ...{ twins: [{ name: "cad-presence-retirement", run: cadPresenceRetirementSelfTests }] }, commands: { verify: OwnedVerifyScript }, snapshotSqliteTests: ["../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🟦️.ts"] });
