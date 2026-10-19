#!/usr/bin/env bun
import { GenerateScript as GraphGenerateScript, PreviewGeneratedScript as GraphPreviewScript, OwnerGraphWireCheckScript } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🕸️graph/🛂️manifest/🏃️execution/🟦️.ts";
/** 📦️ writer writer Rust artifact package router. */
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { runOwnedVerifyScript } from "../../🧪️tests/🔬️rust-package-commands/🟦️.ts";

/** 🧪️ Executes the contracts owned by this component. */
class OwnedVerifyScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runOwnedVerifyScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}

const router = new ScriptRouter(import.meta.dir).register("graph-generate", GraphGenerateScript).register("preview-generated", GraphPreviewScript).register("graph-wire-check", OwnerGraphWireCheckScript).register("verify", OwnedVerifyScript);
await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-writer-writer", { packages: ["semio-s-artifact-writer-writer", "semio-s-artifact-writer-writer-editor", "semio-s-artifact-writer-writer-inferences", "semio-s-artifact-writer-writer-operations", "semio-s-artifact-writer-writer-rename-writer", "semio-s-artifact-writer-writer-schema", "semio-s-artifact-writer-writer-tests", "semio-s-artifact-writer-writer-tests-2", "semio-s-artifact-writer-writer-v1"], snapshotSqliteTests: ["../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"], commands: router });
