#!/usr/bin/env bun
/** 🧭 `repo-go-lib` router: `bun ./script.ts test`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runTestBudgeted } from "../../../repo/lib/js/index.ts";

class TestScript extends BundleScript {
  run(): void {
    runTestBudgeted("go", ["test", "./..."], { cwd: this.root });
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url);
