#!/usr/bin/env bun
/** ✏️ `@semio-tech/draw-rs` router: `bun ./script.ts <wasm|test [fundamental|quick|long|exhaustive]>`. */
import { BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain, runCargoTestBudgeted, runWasmPackWebBuild } from "../../repo/lib/js/index.ts";

class WasmScript extends BundleScript {
  run(): void {
    runWasmPackWebBuild({
      rsDir: this.root,
      skipEnvVar: "DRAW_RS_SKIP_WASM_BUILD",
      logPrefix: "draw/rs",
      wasmBaseName: "draw",
      pkg: {
        name: "@semio-tech/draw-rs",
        files: ["draw_bg.wasm", "draw.js", "draw.d.ts", "draw_bg.wasm.d.ts"],
        main: "draw.js",
        module: "draw.js",
        types: "draw.d.ts",
      },
    });
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargoTestBudgeted(["draw"], this.root, rest);
  }
}

const router = new ScriptRouter(import.meta.dir).register("wasm", WasmScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
