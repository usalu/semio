#!/usr/bin/env bun
/** 🎪️ demonstrator TypeScript package and authored-example verification. */
import { resolve } from "node:path";
import { BundleScript, ScriptRouter, runBundleScriptMain, runCmd } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
class TestScript extends BundleScript {
  run(): void {
    runCmd(process.execPath, ["test", resolve(this.repoRoot, "✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🟦️.ts")]);
  }
}
const router = new ScriptRouter(import.meta.dir).register("test", TestScript);
await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
