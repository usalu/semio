#!/usr/bin/env bun
/** 🦀 `@semio-tech/gis-3d-rs` router: `bun ./script.ts wasm`. */
import { execFileSync } from "node:child_process";
import { BundleScript, ScriptRouter, runBundleScriptMain, runWasmPackWebBuild } from "../../../repo/lib/js/index.ts";

class WasmScript extends BundleScript {
  run(): void {
    runWasmPackWebBuild({
      rsDir: this.root,
      skipEnvVar: "GIS_3D_RS_SKIP_WASM_BUILD",
      logPrefix: "gis/3d/rs",
      wasmBaseName: "gis_3d",
      pkg: {
        name: "@semio-tech/gis-3d-rs",
        files: ["gis_3d_bg.wasm", "gis_3d.js", "gis_3d.d.ts", "gis_3d_bg.wasm.d.ts"],
        main: "gis_3d.js",
        module: "gis_3d.js",
        types: "gis_3d.d.ts",
      },
    });
  }
}

class TestScript extends BundleScript {
  run(segments: string[]): void {
    execFileSync("cargo", ["test", "-p", "gis_3d", ...segments], { stdio: "inherit", cwd: this.repoRoot });
  }
}

const router = new ScriptRouter(import.meta.dir).register("wasm", WasmScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
