#!/usr/bin/env bun
/** @emoji ⚙️ Runs `cargo test` for the `pack_value` crate. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runCargoTestBudgeted, resolveTestLevel } from "../../../repo/lib/js/index.ts";

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runCargoTestBudgeted(["pack_value"], import.meta.dir, rest);
  }
}

if (import.meta.main) {
  const router = new ScriptRouter(import.meta.dir).register("test", TestScript);
  await runBundleScriptMain(router, import.meta.url);
}
