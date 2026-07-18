#!/usr/bin/env bun
/** 🎭 `@semio-tech/fsm-rs` router: `bun ./script.ts <wasm|test>`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runCargoTestBudgeted, runWasmPackWebBuild } from "../../repo/lib/js/index.ts";

class WasmScript extends BundleScript {
  run(): void {
    runWasmPackWebBuild({
      rsDir: this.root,
      skipEnvVar: "FSM_RS_SKIP_WASM_BUILD",
      logPrefix: "fsm/rs",
      wasmBaseName: "fsm",
      pkg: {
        name: "@semio-tech/fsm-rs",
        files: ["fsm_bg.wasm", "fsm.js", "fsm.d.ts", "fsm_bg.wasm.d.ts"],
        main: "fsm.js",
        module: "fsm.js",
        types: "fsm.d.ts",
      },
    });
  }
}

class TestScript extends BundleScript {
  run(segments: string[]): void {
    runCargoTestBudgeted(["fsm", "fsm_macros"], this.root, segments);
  }
}

const router = new ScriptRouter(import.meta.dir).register("wasm", WasmScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
