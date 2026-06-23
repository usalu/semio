#!/usr/bin/env bun
/** 🦀 `@semio-tech/flow-module-math` router: `bun ./script.ts wasm`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runWasmPackWebBuild } from "../../../repo/lib/js/src/index.ts";

class WasmScript extends BundleScript {
  run(): void {
    runWasmPackWebBuild({
      rsDir: this.root,
      skipEnvVar: "FLOW_MODULE_MATH_SKIP_WASM_BUILD",
      logPrefix: "flow/module/math",
      wasmBaseName: "flow_module_math",
      pkg: {
        name: "@semio-tech/flow-module-math",
        files: ["flow_module_math_bg.wasm", "flow_module_math.js", "flow_module_math.d.ts", "flow_module_math_bg.wasm.d.ts"],
        main: "flow_module_math.js",
        module: "flow_module_math.js",
        types: "flow_module_math.d.ts",
      },
    });
  }
}

const router = new ScriptRouter(import.meta.dir).register("wasm", WasmScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
