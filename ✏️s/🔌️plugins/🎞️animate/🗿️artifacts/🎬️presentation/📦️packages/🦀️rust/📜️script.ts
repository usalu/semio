#!/usr/bin/env repo
/** 📦️ animate presentation Rust artifact package router. */
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
await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-animate-presentation", { packages: ["semio-s-artifact-animate-presentation", "semio-s-artifact-animate-presentation-animate", "semio-s-artifact-animate-presentation-animation", "semio-s-artifact-animate-presentation-examples", "semio-s-artifact-animate-presentation-host", "semio-s-artifact-animate-presentation-inferences", "semio-s-artifact-animate-presentation-mutations", "semio-s-artifact-animate-presentation-mut-family", "semio-s-artifact-animate-presentation-schema", "semio-s-artifact-animate-presentation-tests", "semio-s-artifact-animate-presentation-tile-grid", "semio-s-artifact-animate-presentation-v1", "semio-s-artifact-animate-presentation-video", "semio-s-artifact-animate-presentation-viewer"], commands: router, snapshotSqliteTests:["../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"] });
