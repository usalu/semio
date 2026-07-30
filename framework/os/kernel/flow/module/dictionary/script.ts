#!/usr/bin/env bun
/** 🦀 `@semio-tech/flow-module-dictionary` router: `bun ./script.ts wasm`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runWasmPackWebBuild } from "../../../../../../repo/lib/js/index.ts";
import { join } from "node:path";

class WasmScript extends BundleScript {
  run(): void {
    runWasmPackWebBuild({
      rsDir: join(this.root, "rs"),
      skipEnvVar: "FLOW_MODULE_DICTIONARY_SKIP_WASM_BUILD",
      logPrefix: "flow/module/dictionary",
      wasmBaseName: "flow_module_dictionary",
      pkg: {
        name: "@semio-tech/flow-module-dictionary",
        files: ["flow_module_dictionary_bg.wasm", "flow_module_dictionary.js", "flow_module_dictionary.d.ts", "flow_module_dictionary_bg.wasm.d.ts"],
        main: "flow_module_dictionary.js",
        module: "flow_module_dictionary.js",
        types: "flow_module_dictionary.d.ts",
      },
    });
  }
}

const router = new ScriptRouter(import.meta.dir).register("wasm", WasmScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
