#!/usr/bin/env bun
/** 📏 `@semio-tech/fem-2d-rs` router: `bun ./script.ts <wasm|test>`. */
import { BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain, runCargoTestBudgeted, runWasmPackWebBuild } from "../../../repo/lib/js/index.ts";

class WasmScript extends BundleScript {
  run(): void {
    runWasmPackWebBuild({
      rsDir: this.root,
      skipEnvVar: "FEM_2D_RS_SKIP_WASM_BUILD",
      logPrefix: "fem/2d/rs",
      wasmBaseName: "fem_2d",
      pkg: {
        name: "@semio-tech/fem-2d-rs",
        files: ["fem_2d_bg.wasm", "fem_2d.js", "fem_2d.d.ts", "fem_2d_bg.wasm.d.ts"],
        main: "fem_2d.js",
        module: "fem_2d.js",
        types: "fem_2d.d.ts",
      },
    });
  }
}

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runCargoTestBudgeted(["fem_2d"], this.repoRoot, rest);
  }
}

const router = new ScriptRouter(import.meta.dir).register("wasm", WasmScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
