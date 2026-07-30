#!/usr/bin/env bun
/** 🦀 `@semio-tech/animate-core-rs` router: `bun ./script.ts test`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runCargoTestBudgeted } from "../../../../../../../🧰/🛍️/🦑/🔨/lib/⚡️/🟦/📦.ts";

class TestScript extends BundleScript {
  run(segments: string[]): void {
    runCargoTestBudgeted(["animate_core"], this.repoRoot, segments);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
