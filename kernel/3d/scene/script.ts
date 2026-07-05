#!/usr/bin/env bun
/** 🦀 `kernel/3d/scene` router: `bun ./script.ts test`. */
import { BundleScript, ScriptRouter, runBundleScriptMain } from "../../../repo/lib/js/index.ts";
import { join } from "node:path";

class TestScript extends BundleScript {
  run(): void {
    Bun.spawnSync(["cargo", "test", "-p", "kernel_3d_scene"], {
      cwd: this.repoRoot,
      stdin: "inherit",
      stdout: "inherit",
      stderr: "inherit",
    });
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
