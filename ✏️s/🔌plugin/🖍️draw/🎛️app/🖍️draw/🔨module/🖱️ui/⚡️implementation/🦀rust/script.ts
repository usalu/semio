#!/usr/bin/env bun
/** 🖥️ `@semio-tech/draw-ui-rs` router: `bun ./script.ts <wasm|test [fundamental|quick|long|exhaustive]>`. */
import { BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain, runCargoTestBudgeted, runWasmPackWebBuild } from "../../../../../../../../../🧰framework/🛍️product/🦑repo/🔨module/📚lib/⚡️implementation/🟦typescript/📦index.ts";

class WasmScript extends BundleScript {
  run(): void {
    runWasmPackWebBuild({
      rsDir: this.root,
      skipEnvVar: "DRAW_UI_RS_SKIP_WASM_BUILD",
      logPrefix: "draw/app/draw/ui/rs",
      wasmBaseName: "draw",
      pkg: {
        name: "@semio-tech/draw-ui-rs",
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
    await runCargoTestBudgeted(
      ["semio-s-app-draw", "semio-s-app-draw-engine", "semio-s-app-draw-op", "semio-s-app-draw-dsl", "semio-s-app-draw-pack", "semio-s-app-draw-protocol", "semio-s-app-draw-ui"],
      this.root,
      rest,
    );
  }
}

const router = new ScriptRouter(import.meta.dir).register("wasm", WasmScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
