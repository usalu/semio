#!/usr/bin/env bun
/** ⚡ `@semio-tech/energy-engine` router: `bun ./script.ts test|lint`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runCargoLint, runCargoTestBudgeted } from "../../../../repo/lib/js/index.ts";
import { join } from "node:path";

class TestScript extends BundleScript {
  run(_segments: string[]): void {
    runCargoTestBudgeted(["energy_engine"], this.repoRoot);
  }
}

/** 🧹Zero-warning clippy gate: `cargo clippy -p energy_engine --all-targets -- -D warnings`. */
class LintScript extends BundleScript {
  run(segments: string[]): void {
    runCargoLint(["energy_engine"], join(this.root, "rs"), segments);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("lint", LintScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
