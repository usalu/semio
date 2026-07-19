#!/usr/bin/env bun
/** 📏 `@semio-tech/procedural-2d-rs` router: `bun ./script.ts <wasm|test>`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runWasmPackWebBuild, resolveTestLevel, runCargoTestBudgeted } from "../../../repo/lib/js/index.ts";

class WasmScript extends BundleScript {
  run(): void {
    runWasmPackWebBuild({
      rsDir: this.root,
      skipEnvVar: "PROCEDURAL_2D_RS_SKIP_WASM_BUILD",
      logPrefix: "procedural/2d/rs",
      wasmBaseName: "procedural_2d",
      pkg: {
        name: "@semio-tech/procedural-2d-rs",
        files: ["procedural_2d_bg.wasm", "procedural_2d.js", "procedural_2d.d.ts", "procedural_2d_bg.wasm.d.ts"],
        main: "procedural_2d.js",
        module: "procedural_2d.js",
        types: "procedural_2d.d.ts",
      },
    });
  }
}

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runCargoTestBudgeted(["procedural_2d"], this.repoRoot, rest);
  }
}

const router = new ScriptRouter(import.meta.dir).register("wasm", WasmScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
