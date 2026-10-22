#!/usr/bin/env repo
/** 📦️ norm contract Rust package router. */
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import {BundleScript, ScriptRouter} from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { runTestBytePropertyScript } from "../../🧪️tests/🔬️rust-package-commands/🟦️.ts";
/** 🧫️ Executes the bounded intrinsic byte property projection against its independent oracle. */
class TestBytePropertyScript extends BundleScript {
  async run(segments:string[]): Promise<void> {
    await runTestBytePropertyScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}
const router = new ScriptRouter(import.meta.dir).register("test-byte-property", TestBytePropertyScript);
await runArtifactRustPackageMain(import.meta.dir,"semio-s-artifact-norm-contract",{commands: router});
