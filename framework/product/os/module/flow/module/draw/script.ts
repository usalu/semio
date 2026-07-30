#!/usr/bin/env bun
/** 🦀 `@semio-tech/flow-module-draw` router: `bun ./script.ts wasm`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runWasmPackWebBuild, runCargoTestBudgeted, resolveTestLevel } from "../../../../../../../repo/lib/js/index.ts";
import { join } from "node:path";

class WasmScript extends BundleScript {
  run(): void {
    runWasmPackWebBuild({
      rsDir: join(this.root, "rs"),
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

/** ⏱️Level-budgeted; unmarked `mod tests` cases are `fundamental`. */
class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runCargoTestBudgeted(["flow_module_draw"], join(this.root, "rs"), rest);
  }
}

const router = new ScriptRouter(import.meta.dir).register("wasm", WasmScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
