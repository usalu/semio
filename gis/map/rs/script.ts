#!/usr/bin/env bun
/** 🦀 `@gis/map/rs` router: `bun ./script.ts wasm`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runWasmPackWebBuild } from "../../../repo/lib/js/src/index.ts";

class WasmScript extends BundleScript {
  run(): void {
    runWasmPackWebBuild({
      rsDir: this.root,
      skipEnvVar: "GIS_MAP_RS_SKIP_WASM_BUILD",
      logPrefix: "gis/map/rs",
      wasmBaseName: "gis_map",
      pkg: {
        name: "@gis/map/rs",
        files: ["gis_map_bg.wasm", "gis_map.js", "gis_map.d.ts", "gis_map_bg.wasm.d.ts"],
        main: "gis_map.js",
        module: "gis_map.js",
        types: "gis_map.d.ts",
      },
    });
  }
}

const router = new ScriptRouter(import.meta.dir).register("wasm", WasmScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
