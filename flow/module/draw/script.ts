#!/usr/bin/env bun
/** 🦀 `@semio-tech/flow-module-draw` router: `bun ./script.ts wasm`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runWasmPackWebBuild } from "../../../repo/lib/js/src/index.ts";

class WasmScript extends BundleScript {
  run(): void {
    runWasmPackWebBuild({
      rsDir: this.root,
      skipEnvVar: "FLOW_MODULE_DRAW_SKIP_WASM_BUILD",
      logPrefix: "flow/module/draw",
      wasmBaseName: "flow_module_draw",
      threads: false,
      pkg: {
        name: "@semio-tech/flow-module-draw",
        files: ["flow_module_draw_bg.wasm", "flow_module_draw.js", "flow_module_draw.d.ts", "flow_module_draw_bg.wasm.d.ts"],
        main: "flow_module_draw.js",
        module: "flow_module_draw.js",
        types: "flow_module_draw.d.ts",
      },
    });
  }
}

class TestScript extends BundleScript {
  run(): void {
    Bun.spawnSync(["cargo", "test", "-p", "flow_module_draw"], { cwd: this.root, stdio: "inherit" });
  }
}

const router = new ScriptRouter(import.meta.dir).register("wasm", WasmScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
