#!/usr/bin/env bun
/** 🎞️ `@semio-tech/animate-present-rs` router: `bun ./script.ts test|compile`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runCargoTestBudgeted } from "../../../repo/lib/js/index.ts";

class TestScript extends BundleScript {
  run(segments: string[]): void {
    runCargoTestBudgeted(["animate_core", "animate_present"], this.repoRoot, segments);
  }
}

class CompileScript extends BundleScript {
  run(segments: string[]): void {
    runCargoTestBudgeted(
      ["animate_present"],
      this.repoRoot,
      ["compile_present_site_writes_static_bundle", "--nocapture", ...segments],
    );
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("compile", CompileScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
