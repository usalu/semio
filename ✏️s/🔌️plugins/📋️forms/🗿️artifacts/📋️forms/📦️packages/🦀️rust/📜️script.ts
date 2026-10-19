#!/usr/bin/env bun
/** 📦️ forms forms Rust artifact package router. */
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { runOwnedVerifyScript } from "../../🧪️tests/🔬️rust-package-commands/🟦️.ts";

/** 🧪️ Executes the contracts owned by this component. */
class OwnedVerifyScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runOwnedVerifyScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}
const router = new ScriptRouter(import.meta.dir).register("verify", OwnedVerifyScript);
await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-forms-forms", { packages: ["semio-s-artifact-forms-forms", "semio-s-artifact-forms-forms-commands", "semio-s-artifact-forms-forms-config", "semio-s-artifact-forms-forms-config-tests", "semio-s-artifact-forms-forms-definition", "semio-s-artifact-forms-forms-diff", "semio-s-artifact-forms-forms-archive-codecs", "semio-s-artifact-forms-forms-forms", "semio-s-artifact-forms-forms-inferences", "semio-s-artifact-forms-forms-mutations", "semio-s-artifact-forms-forms-questions", "semio-s-artifact-forms-forms-schema", "semio-s-artifact-forms-forms-snapshot", "semio-s-artifact-forms-forms-terminology-tests", "semio-s-artifact-forms-forms-tests", "semio-s-artifact-forms-forms-tests-2", "semio-s-artifact-forms-forms-tests-3"], snapshotSqliteTests:["../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"], commands: router });
