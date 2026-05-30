#!/usr/bin/env bun
/** 🧭 `@repo/lib` router: `bun ./script.ts lint` runs the package lint target via Nx from the monorepo root. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runCmd } from "./src/index.ts";

class LintScript extends BundleScript {
  run(): void {
    runCmd(process.execPath, ["nx", "run", "@repo/lib:lint"], { cwd: this.repoRoot });
  }
}

const router = new ScriptRouter(import.meta.dir).register("lint", LintScript);

await runBundleScriptMain(router, import.meta.url);
