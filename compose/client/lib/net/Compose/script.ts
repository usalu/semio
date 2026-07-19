#!/usr/bin/env bun
/** 🧭 Compose.NET build router: `bun ./script.ts build`. */
import { spawnSync } from "node:child_process";
import { BundleScript, ScriptRouter, runBundleScriptMain, resolveTestLevel, dotnetLevelArgs, runTestBudgeted } from "../../../../../repo/lib/js/index.ts";

class BuildScript extends BundleScript {
  run(): void {
    const buildResult = spawnSync("dotnet", ["build", "Compose.csproj", "-c", "Debug"], {
      cwd: this.root,
      stdio: "inherit",
    });
    if (buildResult.status !== 0) process.exit(buildResult.status ?? 1);
    console.log("✅ Compose.NET build complete");
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { level, rest } = resolveTestLevel(segments);
    await runTestBudgeted("dotnet", ["test", "../Compose.Tests/Compose.Tests.csproj", ...dotnetLevelArgs(level), ...rest], { cwd: this.root });
  }
}

const router = new ScriptRouter(import.meta.dir).register("build", BuildScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "build" });
