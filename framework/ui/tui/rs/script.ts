#!/usr/bin/env bun
/** @emoji ⚙️ Runs `cargo test`/`cargo check --target wasm32-unknown-unknown` for the `ui_tui` crate. */
import { BundleScript, ScriptRouter, runBundleScriptMain, resolveTestLevel, runCargoTestBudgeted, runCmd, buildBudgetMs } from "../../../../repo/lib/js/index.ts";

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runCargoTestBudgeted(["ui_tui"], import.meta.dir, ["--features", "terminal", ...rest]);
  }
}

class CheckWasmScript extends BundleScript {
  run(): void {
    runCmd("cargo", ["check", "-p", "ui_tui", "--target", "wasm32-unknown-unknown"], { cwd: import.meta.dir, budgetMs: buildBudgetMs() });
    runCmd("cargo", ["check", "-p", "ui_tui", "--target", "wasm32-unknown-unknown", "--features", "bindgen"], { cwd: import.meta.dir, budgetMs: buildBudgetMs() });
  }
}

if (import.meta.main) {
  const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("check-wasm", CheckWasmScript);
  await runBundleScriptMain(router, import.meta.url);
}
