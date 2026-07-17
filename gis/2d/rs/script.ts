#!/usr/bin/env bun
/** 🦀 `@semio-tech/gis-2d-rs` router: `bun ./script.ts wasm`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runCargoTestBudgeted, runWasmPackWebBuild } from "../../../repo/lib/js/index.ts";

class WasmScript extends BundleScript {
  run(): void {
    runWasmPackWebBuild({
      rsDir: this.root,
      skipEnvVar: "GIS_2D_RS_SKIP_WASM_BUILD",
      logPrefix: "gis/2d/rs",
      wasmBaseName: "gis_2d",
      pkg: {
        name: "@semio-tech/gis-2d-rs",
        files: ["gis_2d_bg.wasm", "gis_2d.js", "gis_2d.d.ts", "gis_2d_bg.wasm.d.ts"],
        main: "gis_2d.js",
        module: "gis_2d.js",
        types: "gis_2d.d.ts",
      },
    });
  }
}

class TestScript extends BundleScript {
  run(segments: string[]): void {
    runCargoTestBudgeted(["gis_2d"], this.repoRoot, segments);
  }
}

const router = new ScriptRouter(import.meta.dir).register("wasm", WasmScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
