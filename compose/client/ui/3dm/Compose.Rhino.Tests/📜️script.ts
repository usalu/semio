#!/usr/bin/env bun
/** 🧭️Compose.Rhino.Tests test router: `bun ./📜️script.ts test`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, resolveTestLevel, dotnetLevelArgs, dotnetCoverageArgs, runTestBudgeted } from "@semio-tech/repo-lib";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { level, rest } = resolveTestLevel(segments);
    await runTestBudgeted("dotnet", ["test", "cs/Compose.Rhino.Tests.csproj", "-c", "UnitTest", ...dotnetLevelArgs(level), ...dotnetCoverageArgs(this.repoRoot, this.root), ...rest], { cwd: this.root });
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
