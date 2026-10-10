#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import { startNativeProgress } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts";
/** 📦️ norm artifact Rust package router + example-asset regeneration. */
import { resolve } from "node:path";
import { BundleScript, ScriptRouter } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runRepositoryCommand } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🎛️owned-execution/🟦️.ts";
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts";

const packageRoot = import.meta.dir;
const cargoName = "semio-s-artifact-norm-en1996";
const command = process.argv[2];

class RegenerateExampleAssetsScript extends BundleScript {
  async run(_segments: string[]): Promise<void> {
    const stop = startNativeProgress(`artifact-rust:${cargoName}:regenerate-example-assets`);
    try {
      await runRepositoryCommand(
        "cargo",
        [
          "run",
          "--locked",
          "--manifest-path",
          resolve(this.root, "Cargo.toml"),
          "--bin",
          "regenerate-en1996-example-assets",
          "--quiet",
        ],
        this.repoRoot,
        `artifact-rust:${cargoName}:regenerate-example-assets`,
      );
    } finally {
      stop();
    }
  }
}

if (command === "regenerate-example-assets") {
  const router = new ScriptRouter(packageRoot).register("regenerate-example-assets", RegenerateExampleAssetsScript);
  await receiveScriptProcessInvocation(process.env, original => (router).run(["regenerate-example-assets"], original));
} else {
  await runArtifactRustPackageMain(packageRoot, cargoName, {snapshotSqliteTests:["../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"]});
}
