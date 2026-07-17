#!/usr/bin/env bun
/** 🏙️ `@semio-tech/fem-3d-rs` router: `bun ./script.ts <wasm|test>`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runCargoTestBudgeted, runWasmPackWebBuild } from "../../../repo/lib/js/index.ts";

class WasmScript extends BundleScript {
  run(): void {
    runWasmPackWebBuild({
      rsDir: this.root,
      skipEnvVar: "FEM_3D_RS_SKIP_WASM_BUILD",
      logPrefix: "fem/3d/rs",
      wasmBaseName: "fem_3d",
      pkg: {
        name: "@semio-tech/fem-3d-rs",
        files: ["fem_3d_bg.wasm", "fem_3d.js", "fem_3d.d.ts", "fem_3d_bg.wasm.d.ts"],
        main: "fem_3d.js",
        module: "fem_3d.js",
        types: "fem_3d.d.ts",
      },
    });
  }
}

class TestScript extends BundleScript {
  run(segments: string[]): void {
    runCargoTestBudgeted(["fem_3d"], this.repoRoot, segments);
  }
}

const router = new ScriptRouter(import.meta.dir).register("wasm", WasmScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
