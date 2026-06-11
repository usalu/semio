#!/usr/bin/env bun
/** 🦀 `@flow/module-logic` router: `bun ./script.ts wasm`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runWasmPackWebBuild } from "../../../repo/lib/js/src/index.ts";

class WasmScript extends BundleScript {
  run(): void {
    runWasmPackWebBuild({
      rsDir: this.root,
      skipEnvVar: "FLOW_MODULE_LOGIC_SKIP_WASM_BUILD",
      logPrefix: "flow/module/logic",
      wasmBaseName: "flow_module_logic",
      pkg: {
        name: "@flow/module-logic",
        files: ["flow_module_logic_bg.wasm", "flow_module_logic.js", "flow_module_logic.d.ts", "flow_module_logic_bg.wasm.d.ts"],
        main: "flow_module_logic.js",
        module: "flow_module_logic.js",
        types: "flow_module_logic.d.ts",
      },
    });
  }
}

const router = new ScriptRouter(import.meta.dir).register("wasm", WasmScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
