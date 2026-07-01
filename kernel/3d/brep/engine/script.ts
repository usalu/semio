#!/usr/bin/env bun
/** 🦀 `kernel/3d/brep/engine` router: `bun ./script.ts test`. */
import { BundleScript, ScriptRouter, runBundleScriptMain } from "../../../../repo/lib/js/index.ts";

class TestScript extends BundleScript {
  run(): void {
    Bun.spawnSync(["cargo", "test", "-p", "geometry_brep_engine"], { cwd: this.root, stdio: "inherit" });
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
