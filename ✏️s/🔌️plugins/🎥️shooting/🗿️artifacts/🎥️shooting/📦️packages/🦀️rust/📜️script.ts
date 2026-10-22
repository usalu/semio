#!/usr/bin/env repo
/** 📦️ shooting shooting Rust artifact package router. */
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import {BundleScript, ScriptRouter} from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { runVerifyScript } from "../../🧪️tests/🔬️rust-package-commands/🟦️.ts";
/** 🛂️ Checks the actual public snapshot facade and independent neutral oracle types. */
class VerifyScript extends BundleScript {
  async run(segments:string[]): Promise<void> {
    await runVerifyScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}
const router = new ScriptRouter(import.meta.dir).register("verify", VerifyScript);
await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-shooting-shooting", { packages: ["semio-s-artifact-shooting-shooting", "semio-s-artifact-shooting-shooting-config", "semio-s-artifact-shooting-shooting-mut-family", "semio-s-artifact-shooting-shooting-inferences", "semio-s-artifact-shooting-shooting-mut-any", "semio-s-artifact-shooting-shooting-mut-any-2", "semio-s-artifact-shooting-shooting-mut-any-3", "semio-s-artifact-shooting-shooting-mutations", "semio-s-artifact-shooting-shooting-schema", "semio-s-artifact-shooting-shooting-set-active-asset", "semio-s-artifact-shooting-shooting-shooting", "semio-s-artifact-shooting-shooting-tests", "semio-s-artifact-shooting-shooting-tests-dims-scene-sun-to-half"],commands: router,snapshotSqliteTests:["../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"]});
