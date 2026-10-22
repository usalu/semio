#!/usr/bin/env repo
import { GenerateScript as GraphGenerateScript, PreviewGeneratedScript as GraphPreviewScript, OwnerGraphWireCheckScript } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🕸️graph/🛂️manifest/🏃️execution/🟦️.ts";
/** 📦️ reasoning wires Rust artifact package router. */
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
await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-reasoning-wires", { packages: ["semio-s-artifact-reasoning-wires", "semio-s-artifact-reasoning-wires-inferences", "semio-s-artifact-reasoning-wires-schema", "semio-s-artifact-reasoning-wires-snapshot", "semio-s-artifact-reasoning-wires-snapshot-2", "semio-s-artifact-reasoning-wires-v1", "semio-s-artifact-reasoning-wires-viewer", "semio-s-artifact-reasoning-wires-window-transient", "semio-s-artifact-reasoning-wires-windows", "semio-s-artifact-reasoning-wires-wires"], snapshotSqliteTests: ["../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"], commands: router });
