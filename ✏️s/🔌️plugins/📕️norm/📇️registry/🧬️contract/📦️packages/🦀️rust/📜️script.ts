#!/usr/bin/env bun
/** 📦️ norm contract Rust package router. */
import { runArtifactRustPackageMain,runArtifactRustTests } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts";
import {BundleScript} from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { join } from "node:path";
import { runRepositoryTestCommand } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts";
import { repoTestArtifactEnvironment } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🌿️environment/🧪️test-output/🟦️.ts";
/** 🧫️ Executes the bounded intrinsic byte property projection against its independent oracle. */
class TestBytePropertyScript extends BundleScript{async run(segments:string[]):Promise<void>{if(segments.length)throw Error("Unknown byte property command");await runArtifactRustTests("semio-s-artifact-norm-contract",this.repoRoot,["--lib","byte_properties_render_their_bounded_localized_extent"]);}}
/** 🧱️ Verifies the artifact-free assembler source against the closed ownership corpus. */
class TestDefinitionOwnershipScript extends BundleScript { async run(segments: string[]): Promise<void> { if (segments.length) throw Error("Unknown definition ownership command"); await runRepositoryTestCommand(process.execPath, ["test", join(this.root, "../../🧪️tests/🧱️definition-ownership/🟦️.ts")], { cwd: this.repoRoot, env: repoTestArtifactEnvironment(this.repoRoot, "norm-definition-ownership"), budgetMs: 15_000 }); } }
await runArtifactRustPackageMain(import.meta.dir,"semio-s-artifact-norm-contract",{commands:{"test-byte-property":TestBytePropertyScript,"test-definition-ownership":TestDefinitionOwnershipScript}});
