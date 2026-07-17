#!/usr/bin/env bun
/** 📏 `@semio-tech/norm-din-4108` router: `bun ./script.ts test`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runCargoTestBudgeted } from "../../repo/lib/js/index.ts";

class TestScript extends BundleScript {
  run(_segments: string[]): void {
    runCargoTestBudgeted(["norm_din_4108"], this.repoRoot);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
