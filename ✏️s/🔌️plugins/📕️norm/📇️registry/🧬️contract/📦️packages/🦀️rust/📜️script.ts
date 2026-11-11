#!/usr/bin/env repo
/** 📦️ norm contract Rust package router. */
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts";
import {BundleScript, ScriptRouter} from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runTestBytePropertyScript } from "../../🧪️tests/🔬️rust-package-commands/🟦️.ts";
/** 🧫️ Executes the bounded intrinsic byte property projection against its independent oracle. */
class TestBytePropertyScript extends BundleScript {
  async run(segments:string[]): Promise<void> {
    await runTestBytePropertyScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}
const router = new ScriptRouter(import.meta.dir).register("test-byte-property", TestBytePropertyScript);
await runArtifactRustPackageMain(import.meta.dir,"semio-s-artifact-norm-contract",{packages:["semio-s-artifact-norm-contract","semio-s-plugin-norm-ndp","semio-s-plugin-norm-ndp-model","semio-s-plugin-norm-ndp-en1990","semio-s-plugin-norm-ndp-en1991","semio-s-plugin-norm-ndp-en1992","semio-s-plugin-norm-ndp-en1993","semio-s-plugin-norm-ndp-en1994","semio-s-plugin-norm-ndp-en1995","semio-s-plugin-norm-ndp-en1996","semio-s-plugin-norm-ndp-en1997","semio-s-plugin-norm-ndp-en1998","semio-s-plugin-norm-ndp-en1999","semio-s-plugin-norm-compliance","semio-s-plugin-norm-compliance-core","semio-s-plugin-norm-results-config","semio-s-plugin-norm-viewport-config","semio-s-plugin-norm-structural-scene","semio-s-plugin-norm-structural-scene-meshing","semio-s-plugin-norm-structural-scene-case-kit","semio-s-plugin-norm-structural-scene-foundation","semio-s-plugin-norm-app-surface","semio-s-plugin-norm-app-surface-foundation","semio-s-plugin-norm-app-surface-live-core","semio-s-plugin-norm-app-surface-live-editor","semio-s-plugin-norm-app-surface-visual-chrome","semio-s-plugin-norm-app-surface-panels","semio-s-plugin-norm-app-surface-panels-inspection","semio-s-plugin-norm-app-surface-viewport3d"],commands: router});
