#!/usr/bin/env bun
/** 🦀️ `@semio-tech/puzzle-2d-rs` router: `bun ./📜️script.ts test` (headless compute — no wasm target; the wasm-bindgen board session lives in `@semio-tech/framework-surface-board-2d-rs`). */
import { BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain, runCargoTestBudgeted } from "../../../../../../../../../🧰️framework/🛍️product/🦑️repo/🔨️module/📚️lib/⚡️implementation/🟦️typescript/📦️index.ts";

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runCargoTestBudgeted(["semio-s-app-puzzle-2d-engine"], this.repoRoot, rest);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
