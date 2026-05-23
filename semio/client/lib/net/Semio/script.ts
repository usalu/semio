#!/usr/bin/env bun
/** 🧭 Semio.NET build router: `bun ./script.ts build`. */
import { spawnSync } from "node:child_process";
import { BundleScript, ScriptRouter, runBundleScriptMain } from "../../../../../repo/lib/js/src/index.ts";

class BuildScript extends BundleScript {
  run(): void {
    const buildResult = spawnSync("dotnet", ["build", "Semio.csproj", "-c", "Debug"], {
      cwd: this.root,
      stdio: "inherit",
    });
    if (buildResult.status !== 0) process.exit(buildResult.status ?? 1);
    console.log("✅ Semio.NET build complete");
  }
}

const router = new ScriptRouter(import.meta.dir).register("build", BuildScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "build" });
