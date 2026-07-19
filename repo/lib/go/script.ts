#!/usr/bin/env bun
/** 🧭 `repo-go-lib` router: `bun ./script.ts test`. */
import { BundleScript, ScriptRouter, goLevelTestArgs, resolveTestLevel, runBundleScriptMain, runTestBudgeted } from "../../../repo/lib/js/index.ts";

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { level, rest } = resolveTestLevel(segments);
    runTestBudgeted("go", ["test", "./...", ...goLevelTestArgs(level), ...rest], { cwd: this.root });
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url);
