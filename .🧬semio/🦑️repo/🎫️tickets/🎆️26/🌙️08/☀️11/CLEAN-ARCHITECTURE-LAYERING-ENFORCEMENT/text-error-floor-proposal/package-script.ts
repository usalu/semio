#!/usr/bin/env bun
import { resolve } from "node:path";
import { runOwnedCommand } from "../../../🏃️process/🎛️owned-execution/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { runCargoTestsV1, readCargoTestPolicyV1 } from "../../../🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import { resolveTestLevel } from "../../../🏃️process/🧪️testing/🎚️budget/🟦️.ts";
import { TestScript } from "../../🧪️tests/🎛️controlled/🟦️.ts";

/** ⚠️ Executes the complete original diagnostic and span laws. */
class NativeScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargoTestsV1({ manifestPath: resolve(this.root, "Cargo.toml"), packages: ["semio-framework-diagnostic"], cwd: this.root, extraArgs: rest }, readCargoTestPolicyV1(process.env));
  }
}

/** 🚧️ Checks the owned typed source error against the neutral schema and independent references. */
class TextErrorPortableScript extends BundleScript {
 async run(args: string[]): Promise<void> {
  if (args.length) throw Error("Expected test-text-error-portable");
  const test = resolve(this.root, "../../🚧️text-error/🧪️tests/🟦️.ts");
  await runOwnedCommand(process.execPath, [resolve(this.repoRoot, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--noUncheckedIndexedAccess", "--skipLibCheck", "--allowImportingTsExtensions", "--resolveJsonModule", "--esModuleInterop", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--types", "bun", test], this.repoRoot, "diagnostic:text-error:types", 30000);
  await runOwnedCommand(process.execPath, ["test", "--timeout", "30000", test], this.repoRoot, "diagnostic:text-error:portable", 30000);
 }
}

const router = new ScriptRouter(import.meta.dir).register("test-native", NativeScript).register("test-controlled-oracle", TestScript).register("test-text-error-portable", TextErrorPortableScript);
await runScriptMain(router, { defaultCommand: "test-native" });
