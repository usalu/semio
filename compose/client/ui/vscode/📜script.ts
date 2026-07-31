#!/usr/bin/env bun
/** 🧭 `@semio-tech/compose-vscode` test router: `bun ./📜script.ts test [level]`. */
import { BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain } from "../../../../repo/lib/js/index.ts";

/** ⏱️No test suite exists yet for this extension (no unit tests, no `vscode-test` harness wired) — this only surfaces the active level so the leveled scheme has a real hook to grow into. */
class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { level } = resolveTestLevel(segments);
    console.log(`[test] @semio-tech/compose-vscode has no ${level}-level suite yet.`);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url);
