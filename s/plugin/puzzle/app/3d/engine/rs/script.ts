#!/usr/bin/env bun
/** 🦀 `@semio-tech/puzzle-3d-rs` router: `bun ./script.ts wasm`. */
import { BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain, runCargoTestBudgeted, runWasmPackWebBuild } from "../../../../../../../repo/lib/js/index.ts";

class WasmScript extends BundleScript {
  run(): void {
    runWasmPackWebBuild({
      rsDir: this.root,
      skipEnvVar: "PUZZLE_3D_RS_SKIP_WASM_BUILD",
      logPrefix: "puzzle/3d/rs",
      wasmBaseName: "puzzle_3d",
      pkg: {
        name: "@semio-tech/puzzle-3d-rs",
        files: ["puzzle_3d_bg.wasm", "puzzle_3d.js", "puzzle_3d.d.ts", "puzzle_3d_bg.wasm.d.ts"],
        main: "puzzle_3d.js",
        module: "puzzle_3d.js",
        types: "puzzle_3d.d.ts",
      },
    });
  }
}

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runCargoTestBudgeted(["semio-s-app-puzzle-3d-engine"], this.repoRoot, rest);
  }
}

const router = new ScriptRouter(import.meta.dir).register("wasm", WasmScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
