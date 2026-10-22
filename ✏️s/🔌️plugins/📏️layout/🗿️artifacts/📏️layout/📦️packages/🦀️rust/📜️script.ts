#!/usr/bin/env repo
/** 📦️ layout layout Rust artifact package router. */
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { runOwnedVerifyScript, runMediaAdoptionSourceScript } from "../../🧪️tests/🔬️rust-package-commands/🟦️.ts";
/** 🧪️ Executes the contracts owned by this component. */
class OwnedVerifyScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runOwnedVerifyScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}
/** 🎞️ Checks S Layout's owned Source adoption under the original caller's control. */
class MediaAdoptionSourceScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runMediaAdoptionSourceScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}
const router = new ScriptRouter(import.meta.dir).register("verify", OwnedVerifyScript).register("test-media-adoption-source", MediaAdoptionSourceScript);
await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-layout-layout", { testPackages: ["semio-s-artifact-layout-layout", "semio-s-artifact-layout-layout-editor-commands-tests", "semio-s-artifact-layout-layout-mutations-content-tests-1", "semio-s-artifact-layout-layout-mutations-content-tests-2", "semio-s-artifact-layout-layout-mutations-frames-tests-1", "semio-s-artifact-layout-layout-mutations-frames-tests-2"], commands: router });



