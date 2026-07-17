#!/usr/bin/env bun
/** 🦀 `@semio-tech/sequence-core` router: `bun ./script.ts wasm|test`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runCargoTestBudgeted, runWasmPackWebBuild } from "../../repo/lib/js/index.ts";
import { join } from "node:path";

class WasmScript extends BundleScript {
  run(): void {
    runWasmPackWebBuild({
      rsDir: join(this.root, "rs"),
      skipEnvVar: "SEQUENCE_CORE_SKIP_WASM_BUILD",
      logPrefix: "sequence/core",
      wasmBaseName: "sequence_core",
      threads: false,
      pkg: {
        name: "@semio-tech/sequence-core",
        files: ["sequence_core_bg.wasm", "sequence_core.js", "sequence_core.d.ts", "sequence_core_bg.wasm.d.ts"],
        main: "sequence_core.js",
        module: "sequence_core.js",
        types: "sequence_core.d.ts",
      },
    });
  }
}

/** ⏱️Warm-cache Rust unit tests in `rs/` — this bundle has no JS test suite, so `test` runs the crate's `#[cfg(test)]` modules instead of vitest. */
class TestScript extends BundleScript {
  run(segments: string[]): void {
    runCargoTestBudgeted(["sequence_core"], join(this.root, "rs"), segments);
  }
}

const router = new ScriptRouter(import.meta.dir).register("wasm", WasmScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
