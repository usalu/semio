#!/usr/bin/env bun
/** 🦀 `@semio-tech/raster-rs` router: `bun ./script.ts <wasm|test>`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runCargoTestBudgeted, runWasmPackWebBuild } from "../../repo/lib/js/index.ts";

class WasmScript extends BundleScript {
  run(): void {
    runWasmPackWebBuild({
      rsDir: this.root,
      skipEnvVar: "RASTER_RS_SKIP_WASM_BUILD",
      logPrefix: "raster/rs",
      wasmBaseName: "raster",
      pkg: {
        name: "@semio-tech/raster-rs",
        files: ["raster_bg.wasm", "raster.js", "raster.d.ts", "raster_bg.wasm.d.ts"],
        main: "raster.js",
        module: "raster.js",
        types: "raster.d.ts",
      },
    });
  }
}

class TestScript extends BundleScript {
  run(segments: string[]): void {
    runCargoTestBudgeted(["raster"], this.repoRoot, segments);
  }
}

const router = new ScriptRouter(import.meta.dir).register("wasm", WasmScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
