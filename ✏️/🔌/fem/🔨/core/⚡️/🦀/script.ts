#!/usr/bin/env bun
/** 🏗️ `@semio-tech/fem-core-rs` router: `bun ./script.ts test`. */
import { BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain, runCargoTestBudgeted } from "../../../../../../../🧰/🛍️/🦑/🔨/lib/⚡️/🟦/📦.ts";

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runCargoTestBudgeted(["fem_core"], this.repoRoot, rest);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
