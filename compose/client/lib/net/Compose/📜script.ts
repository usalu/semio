#!/usr/bin/env bun
/** 🧭 Compose.NET build router: `bun ./📜script.ts build`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, resolveTestLevel, dotnetLevelArgs, dotnetCoverageArgs, runCmd, runTestBudgeted } from "../../../../../repo/lib/js/index.ts";

class BuildScript extends BundleScript {
  run(): void {
    runCmd("dotnet", ["build", "cs/Compose.csproj", "-c", "Debug"], { cwd: this.root });
    console.log("✅ Compose.NET build complete");
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { level, rest } = resolveTestLevel(segments);
    await runTestBudgeted("dotnet", ["test", "../Compose.Tests/cs/Compose.Tests.csproj", ...dotnetLevelArgs(level), ...dotnetCoverageArgs(this.repoRoot, this.root), ...rest], { cwd: this.root });
  }
}

const router = new ScriptRouter(import.meta.dir).register("build", BuildScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "build" });
