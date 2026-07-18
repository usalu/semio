#!/usr/bin/env bun
/** 🦀 `@semio-tech/framework-surface-tiled-map-rs` router: `bun ./script.ts wasm`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runCargoTestBudgeted, runWasmPackWebBuild } from "../../../../repo/lib/js/index.ts";

class WasmScript extends BundleScript {
  run(): void {
    runWasmPackWebBuild({
      rsDir: this.root,
      skipEnvVar: "FRAMEWORK_SURFACE_TILED_MAP_RS_SKIP_WASM_BUILD",
      logPrefix: "framework/surface/tiled-map/rs",
      wasmBaseName: "framework_surface_tiled_map",
      pkg: {
        name: "@semio-tech/framework-surface-tiled-map-rs",
        files: ["framework_surface_tiled_map_bg.wasm", "framework_surface_tiled_map.js", "framework_surface_tiled_map.d.ts", "framework_surface_tiled_map_bg.wasm.d.ts"],
        main: "framework_surface_tiled_map.js",
        module: "framework_surface_tiled_map.js",
        types: "framework_surface_tiled_map.d.ts",
      },
    });
  }
}

class TestScript extends BundleScript {
  run(segments: string[]): void {
    runCargoTestBudgeted(["framework_surface_tiled_map"], this.repoRoot, segments);
  }
}

const router = new ScriptRouter(import.meta.dir).register("wasm", WasmScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
