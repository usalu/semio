#!/usr/bin/env bun
/** 🦀 `@semio-tech/framework-core` task router: `bun ./script.ts test`. */
import { BundleScript, ScriptRouter, runBundleScriptMain } from "../../repo/lib/js/index.ts";
import { spawnSync } from "node:child_process";
import { join } from "node:path";

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const result = spawnSync("cargo", ["test", ...segments], {
      cwd: join(this.root, "rs"),
      stdio: "inherit",
      env: process.env,
    });
    if (result.status !== 0) process.exit(result.status ?? 1);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
