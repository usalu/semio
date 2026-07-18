#!/usr/bin/env bun
/** 🖥️ `@semio-tech/s-studio-rs` router: `bun ./script.ts <wasm|test>`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runCargoTestBudgeted, runWasmPackWebBuild } from "../../repo/lib/js/index.ts";

class WasmScript extends BundleScript {
  run(): void {
    runWasmPackWebBuild({
      rsDir: this.root,
      skipEnvVar: "S_STUDIO_RS_SKIP_WASM_BUILD",
      logPrefix: "s/rs",
      wasmBaseName: "s_studio",
      pkg: {
        name: "@semio-tech/s-studio-rs",
        files: ["s_studio_bg.wasm", "s_studio.js", "s_studio.d.ts", "s_studio_bg.wasm.d.ts"],
        main: "s_studio.js",
        module: "s_studio.js",
        types: "s_studio.d.ts",
      },
    });
  }
}

class TestScript extends BundleScript {
  run(segments: string[]): void {
    runCargoTestBudgeted(["s_studio"], this.root, segments);
  }
}

const router = new ScriptRouter(import.meta.dir).register("wasm", WasmScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
