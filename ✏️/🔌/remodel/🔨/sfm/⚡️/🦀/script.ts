#!/usr/bin/env bun
/** 🏗️ `@semio-tech/remodel-sfm` — Structure from motion: two-view geometry, triangulation, PnP, incremental and global reconstruction, bundle adjustment, loop closure and pose priors. */
import { BundleScript, ScriptRouter, resolveTestLevel, runCargoTestBudgeted, runCargoLint, runBundleScriptMain } from "../../../../../../../🧰/🛍️/🦑/🔨/lib/⚡️/🟦/📦.ts";
import { join } from "node:path";

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runCargoTestBudgeted(["remodel_sfm"], join(this.root, "rs"), rest);
  }
}

class LintScript extends BundleScript {
  run(): void {
    runCargoLint(["remodel_sfm"], join(this.root, "rs"));
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("lint", LintScript);
await runBundleScriptMain(router, import.meta.url);
