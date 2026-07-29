#!/usr/bin/env bun
/** 🦀 `@semio-tech/flow-core` router: `bun ./script.ts wasm`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runWasmPackWebBuild } from "../../repo/lib/js/index.ts";
import { join } from "node:path";

class WasmScript extends BundleScript {
  run(): void {
    runWasmPackWebBuild({
      rsDir: join(this.root, "rs"),
      skipEnvVar: "FLOW_CORE_SKIP_WASM_BUILD",
      logPrefix: "flow/core",
      wasmBaseName: "flow_core",
      threads: false,
      pkg: {
        name: "@semio-tech/flow-core",
        files: ["flow_core_bg.wasm", "flow_core.js", "flow_core.d.ts", "flow_core_bg.wasm.d.ts"],
        main: "flow_core.js",
        module: "flow_core.js",
        types: "flow_core.d.ts",
      },
    });
  }
}

const router = new ScriptRouter(import.meta.dir).register("wasm", WasmScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
