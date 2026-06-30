#!/usr/bin/env bun
/** 🦀 `geometry/drawing/rs` router: `bun ./script.ts test`. */
import { BundleScript, ScriptRouter, runBundleScriptMain } from "../../../repo/lib/js/src/index.ts";

class TestScript extends BundleScript {
  run(): void {
    const result = Bun.spawnSync(["cargo", "test", "-p", "geometry_drawing_rs"], { cwd: this.root, stdio: ["inherit", "inherit", "inherit"] });
    if (result.exitCode !== 0) throw new Error(`cargo test failed with exit code ${result.exitCode}`);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
