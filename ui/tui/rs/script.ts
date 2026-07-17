#!/usr/bin/env bun
/** @emoji ⚙️ Runs `cargo test`/`cargo check --target wasm32-unknown-unknown` for the `ui_tui` crate. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runCmd } from "../../../repo/lib/js/index.ts";

class TestScript extends BundleScript {
  run(): void {
    runCmd("cargo", ["test", "-p", "ui_tui", "--features", "terminal"], { cwd: import.meta.dir });
  }
}

class CheckWasmScript extends BundleScript {
  run(): void {
    runCmd("cargo", ["check", "-p", "ui_tui", "--target", "wasm32-unknown-unknown"], { cwd: import.meta.dir });
    runCmd("cargo", ["check", "-p", "ui_tui", "--target", "wasm32-unknown-unknown", "--features", "bindgen"], { cwd: import.meta.dir });
  }
}

if (import.meta.main) {
  const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("check-wasm", CheckWasmScript);
  await runBundleScriptMain(router, import.meta.url);
}
