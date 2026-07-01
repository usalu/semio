#!/usr/bin/env bun
/** 🧭 Compose.NET build router: `bun ./script.ts build`. */
import { execFileSync, spawnSync } from "node:child_process";
import { BundleScript, ScriptRouter, runBundleScriptMain } from "../../../../../repo/lib/js/index.ts";

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
  run(): void {
    execFileSync("dotnet", ["test", "../Compose.Tests/Compose.Tests.csproj"], { cwd: this.root, stdio: "inherit" });
  }
}

const router = new ScriptRouter(import.meta.dir).register("build", BuildScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "build" });
