#!/usr/bin/env bun
/** 📦️ norm contract Rust package router. */
import { runArtifactRustPackageMain, runArtifactRustTests } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import {BundleScript} from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
/** 🧫️ Executes the bounded intrinsic byte property projection against its independent oracle. */
class TestBytePropertyScript extends BundleScript{async run(segments:string[]):Promise<void>{if(segments.length)throw Error("Unknown byte property command");await runArtifactRustTests("semio-s-artifact-norm-contract", this.repoRoot, ["--lib","byte_properties_render_their_bounded_localized_extent"], this.invocation);}}
await runArtifactRustPackageMain(import.meta.dir,"semio-s-artifact-norm-contract",{commands:{"test-byte-property":TestBytePropertyScript}});
