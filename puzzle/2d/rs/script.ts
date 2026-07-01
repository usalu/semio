#!/usr/bin/env bun
/** 🦀 `@semio-tech/puzzle-2d-rs` router: `bun ./script.ts wasm`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runWasmPackWebBuild } from "../../../repo/lib/js/index.ts";

class WasmScript extends BundleScript {
  run(): void {
    runWasmPackWebBuild({
      rsDir: this.root,
      skipEnvVar: "PUZZLE_2D_RS_SKIP_WASM_BUILD",
      logPrefix: "puzzle/2d/rs",
      wasmBaseName: "puzzle_2d",
      pkg: {
        name: "@semio-tech/puzzle-2d-rs",
        files: ["puzzle_2d_bg.wasm", "puzzle_2d.js", "puzzle_2d.d.ts", "puzzle_2d_bg.wasm.d.ts"],
        main: "puzzle_2d.js",
        module: "puzzle_2d.js",
        types: "puzzle_2d.d.ts",
      },
    });
  }
}

const router = new ScriptRouter(import.meta.dir).register("wasm", WasmScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
