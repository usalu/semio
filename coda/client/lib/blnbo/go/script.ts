#!/usr/bin/env bun
/** 🧭 Coda blnbo go router: `bun ./script.ts test`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runTestBudgeted, resolveTestLevel, goLevelTestArgs, goCoverageArgs } from "../../../../../repo/lib/js/index.ts";

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { level, rest } = resolveTestLevel(segments);
    runTestBudgeted("go", ["test", "./...", ...goLevelTestArgs(level), ...goCoverageArgs(this.repoRoot, this.root), ...rest], { cwd: this.root });
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url);
