#!/usr/bin/env bun
/** 🦀️ `@semio-tech/puzzle-2d-rs` router: `bun ./📜️script.ts wasm|test`. */
import { BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain, runCargoTestBudgeted, runWasmPackWebBuild } from "../../../../../../../../../🧰️framework/🛍️product/🦑️repo/🔨️module/📚️lib/⚡️implementation/🟦️typescript/📦️index.ts";

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

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runCargoTestBudgeted(["semio-s-app-puzzle-2d-engine"], this.repoRoot, rest);
  }
}

const router = new ScriptRouter(import.meta.dir).register("wasm", WasmScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
