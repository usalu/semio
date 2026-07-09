#!/usr/bin/env bun
/** 🦀 `@semio-tech/sequence-core` router: `bun ./script.ts wasm|test`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runVitest, runWasmPackWebBuild } from "../../repo/lib/js/index.ts";
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

class TestScript extends BundleScript {
  run(segments: string[]): void {
    runVitest(this.root, segments, "js/vitest.config.ts");
  }
}

const router = new ScriptRouter(import.meta.dir).register("wasm", WasmScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
