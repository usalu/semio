#!/usr/bin/env bun
/** @emoji ⚙️ Runs `cargo test` for the `ui_styling` crate. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runCmd } from "../../../repo/lib/js/index.ts";

class TestScript extends BundleScript {
  run(): void {
    runCmd("cargo", ["test", "-p", "ui_styling"], { cwd: import.meta.dir });
  }
}

if (import.meta.main) {
  const router = new ScriptRouter(import.meta.dir).register("test", TestScript);
  await runBundleScriptMain(router, import.meta.url);
}
