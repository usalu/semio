#!/usr/bin/env bun
/** 🦀 `@semio-tech/flow-module-list` router: `bun ./script.ts wasm`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runWasmPackWebBuild } from "../../../repo/lib/js/index.ts";

class WasmScript extends BundleScript {
  run(): void {
    runWasmPackWebBuild({
      rsDir: this.root,
      skipEnvVar: "FLOW_MODULE_LIST_SKIP_WASM_BUILD",
      logPrefix: "flow/module/list",
      wasmBaseName: "flow_module_list",
      pkg: {
        name: "@semio-tech/flow-module-list",
        files: [
          "flow_module_list_bg.wasm",
          "flow_module_list.js",
          "flow_module_list.d.ts",
          "flow_module_list_bg.wasm.d.ts",
        ],
        main: "flow_module_list.js",
        module: "flow_module_list.js",
        types: "flow_module_list.d.ts",
      },
    });
  }
}

const router = new ScriptRouter(import.meta.dir).register("wasm", WasmScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
