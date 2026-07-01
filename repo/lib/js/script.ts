#!/usr/bin/env bun
/** 🧭 `@semio-tech/repo-lib` router: `bun ./script.ts lint` runs the package lint target via Nx from the monorepo root. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runCmd } from "./index.ts";

class LintScript extends BundleScript {
  run(): void {
    runCmd(process.execPath, ["nx", "run", "@semio-tech/repo-lib:lint"], { cwd: this.repoRoot });
  }
}

const router = new ScriptRouter(import.meta.dir).register("lint", LintScript);

await runBundleScriptMain(router, import.meta.url);
