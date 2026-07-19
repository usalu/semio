#!/usr/bin/env bun
/** 🏛️ `@semio-tech/architect-program` router: `bun ./script.ts test|lint`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runCargoLint, runCargoTestBudgeted } from "../../repo/lib/js/index.ts";
import { join } from "node:path";

class TestScript extends BundleScript {
  run(_segments: string[]): void {
    runCargoTestBudgeted(["architect_program"], this.repoRoot);
  }
}

/** 🧹Zero-warning clippy gate: `cargo clippy -p architect_program --all-targets -- -D warnings`. */
class LintScript extends BundleScript {
  run(segments: string[]): void {
    runCargoLint(["architect_program"], join(this.root, "rs"), segments);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("lint", LintScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
