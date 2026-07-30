#!/usr/bin/env bun
/** ⚙️ `@semio-tech/remodel-engine` — Reconstruction engine: cooperative staged pipeline turning decoded frames into textured meshes, previews and quality reports. */
import { BundleScript, ScriptRouter, resolveTestLevel, runCargoTestBudgeted, runCargoLint, runBundleScriptMain } from "../../../../../../../🧰/🛍️/🦑/🔨/lib/⚡️/🟦/📦.ts";
import { join } from "node:path";

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runCargoTestBudgeted(["remodel_engine"], join(this.root, "rs"), rest);
  }
}

class LintScript extends BundleScript {
  run(): void {
    runCargoLint(["remodel_engine"], join(this.root, "rs"));
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("lint", LintScript);
await runBundleScriptMain(router, import.meta.url);
