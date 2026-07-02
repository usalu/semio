#!/usr/bin/env bun
/** 🦀 `kernel/3d/mesh` router: `bun ./script.ts test`. */
import { BundleScript, ScriptRouter, runBundleScriptMain } from "../../../repo/lib/js/index.ts";

class TestScript extends BundleScript {
  run(): void {
    Bun.spawnSync(["cargo", "test", "-p", "kernel_3d_mesh"], {
      cwd: this.repoRoot,
      stdin: "inherit",
      stdout: "inherit",
      stderr: "inherit",
    });
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
