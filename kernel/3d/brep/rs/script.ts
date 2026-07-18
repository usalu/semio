#!/usr/bin/env bun
/** 🦀 `kernel/3d/brep/rs` router: `bun ./script.ts test`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runCargoTestBudgeted } from "../../../../repo/lib/js/index.ts";

class TestScript extends BundleScript {
  run(): void {
    runCargoTestBudgeted(["kernel_3d_brepkit"], this.root);
  }
}

/** 📈 Runs the criterion benchmark suite (`benches/kernel.rs`). */
class BenchScript extends BundleScript {
  run(): void {
    Bun.spawnSync(["cargo", "bench", "-p", "kernel_3d_brepkit"], { cwd: this.root, stdio: "inherit" });
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("bench", BenchScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
