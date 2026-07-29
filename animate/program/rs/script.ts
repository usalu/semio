#!/usr/bin/env bun
/** 🦀 `@semio-tech/animate-program-rs` router: `bun ./script.ts wasm|test`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runCargoTestBudgeted, runWasmPackWebBuild } from "../../../repo/lib/js/index.ts";

class WasmScript extends BundleScript {
  run(): void {
    runWasmPackWebBuild({
      rsDir: this.root,
      skipEnvVar: "ANIMATE_PLUGIN_RS_SKIP_WASM_BUILD",
      logPrefix: "animate/program/rs",
      wasmBaseName: "animate_program",
      pkg: {
        name: "@semio-tech/animate-program-rs",
        files: ["animate_program_bg.wasm", "animate_program.js", "animate_program.d.ts", "animate_program_bg.wasm.d.ts"],
        main: "animate_program.js",
        module: "animate_program.js",
        types: "animate_program.d.ts",
      },
    });
  }
}

class TestScript extends BundleScript {
  run(segments: string[]): void {
    runCargoTestBudgeted(["animate-program"], this.repoRoot, segments);
  }
}

const router = new ScriptRouter(import.meta.dir).register("wasm", WasmScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
