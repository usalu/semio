#!/usr/bin/env bun
/** 🦀 `@semio-tech/flow-module-core` router: `bun ./script.ts wasm`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runWasmPackWebBuild } from "../../../repo/lib/js/index.ts";

class WasmScript extends BundleScript {
  run(): void {
    runWasmPackWebBuild({
      rsDir: this.root,
      skipEnvVar: "FLOW_MODULE_CORE_SKIP_WASM_BUILD",
      logPrefix: "flow/module/core",
      wasmBaseName: "flow_module_core",
      pkg: {
        name: "@semio-tech/flow-module-core",
        files: ["flow_module_core_bg.wasm", "flow_module_core.js", "flow_module_core.d.ts", "flow_module_core_bg.wasm.d.ts"],
        main: "flow_module_core.js",
        module: "flow_module_core.js",
        types: "flow_module_core.d.ts",
      },
    });
  }
}

const router = new ScriptRouter(import.meta.dir).register("wasm", WasmScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
