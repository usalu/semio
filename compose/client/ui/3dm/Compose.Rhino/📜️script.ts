#!/usr/bin/env bun
/** 🧭️ Compose.Rhino build/test router: `bun ./📜️script.ts build|test`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, resolveTestLevel, dotnetLevelArgs, dotnetCoverageArgs, runCmd, runTestBudgeted } from "@semio-tech/repo-lib";

class BuildScript extends BundleScript {
  run(): void {
    runCmd("dotnet", ["build"], { cwd: this.root });
    console.log("✅️ Compose.Rhino build complete");
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { level, rest } = resolveTestLevel(segments);
    await runTestBudgeted(
      "dotnet",
      ["test", "../Compose.Rhino.Tests/cs/Compose.Rhino.Tests.csproj", "-c", "UnitTest", ...dotnetLevelArgs(level), ...dotnetCoverageArgs(this.repoRoot, this.root), ...rest],
      { cwd: this.root },
    );
  }
}

const router = new ScriptRouter(import.meta.dir).register("build", BuildScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "build" });
