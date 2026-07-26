#!/usr/bin/env bun
/** 🧭 Compose.Tests test router: `bun ./script.ts test`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, resolveTestLevel, dotnetLevelArgs, dotnetCoverageArgs, runTestBudgeted } from "../../../../../repo/lib/js/index.ts";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { level, rest } = resolveTestLevel(segments);
    await runTestBudgeted("dotnet", ["test", "cs/Compose.Tests.csproj", ...dotnetLevelArgs(level), ...dotnetCoverageArgs(this.repoRoot, this.root), ...rest], { cwd: this.root });
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
