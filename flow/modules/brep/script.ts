#!/usr/bin/env bun
/** 🦀 `@flow/module-brep` router: `bun ./script.ts wasm`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runWasmPackWebBuild } from "../../../repo/lib/js/src/index.ts";

class WasmScript extends BundleScript {
  run(): void {
    runWasmPackWebBuild({
      rsDir: this.root,
      skipEnvVar: "FLOW_MODULE_BREP_SKIP_WASM_BUILD",
      logPrefix: "flow/modules/brep",
      wasmBaseName: "flow_module_brep",
      pkg: {
        name: "@flow/module-brep",
        files: ["flow_module_brep_bg.wasm", "flow_module_brep.js", "flow_module_brep.d.ts", "flow_module_brep_bg.wasm.d.ts"],
        main: "flow_module_brep.js",
        module: "flow_module_brep.js",
        types: "flow_module_brep.d.ts",
      },
    });
  }
}

const router = new ScriptRouter(import.meta.dir).register("wasm", WasmScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
