#!/usr/bin/env bun
/** 🦀 `@puzzle/2d/rs` router: `bun ./script.ts wasm`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runWasmPackWebBuild } from "../../../repo/lib/js/src/index.ts";

class WasmScript extends BundleScript {
  run(): void {
    runWasmPackWebBuild({
      rsDir: this.root,
      skipEnvVar: "PUZZLE_2D_RS_SKIP_WASM_BUILD",
      logPrefix: "puzzle/2d/rs",
      wasmBaseName: "puzzle_board",
      pkg: {
        name: "@puzzle/2d/rs",
        files: ["puzzle_board_bg.wasm", "puzzle_board.js", "puzzle_board.d.ts", "puzzle_board_bg.wasm.d.ts"],
        main: "puzzle_board.js",
        module: "puzzle_board.js",
        types: "puzzle_board.d.ts",
      },
    });
  }
}

const router = new ScriptRouter(import.meta.dir).register("wasm", WasmScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
