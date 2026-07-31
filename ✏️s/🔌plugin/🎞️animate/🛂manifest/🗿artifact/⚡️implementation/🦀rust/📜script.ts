#!/usr/bin/env bun
/** 🦀 `@semio-tech/animate-plugin-rs` router: `bun ./📜script.ts wasm|test`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runCargoTestBudgeted, runWasmPackWebBuild } from "../../../../../../../🧰framework/🛍️product/🦑repo/🔨module/📚lib/⚡️implementation/🟦typescript/📦index.ts";

class WasmScript extends BundleScript {
  run(): void {
    runWasmPackWebBuild({
      rsDir: this.root,
      skipEnvVar: "ANIMATE_PLUGIN_RS_SKIP_WASM_BUILD",
      logPrefix: "animate/rs",
      wasmBaseName: "animate_plugin",
      pkg: {
        name: "@semio-tech/animate-plugin-rs",
        files: ["animate_plugin_bg.wasm", "animate_plugin.js", "animate_plugin.d.ts", "animate_plugin_bg.wasm.d.ts"],
        main: "animate_plugin.js",
        module: "animate_plugin.js",
        types: "animate_plugin.d.ts",
      },
    });
  }
}

class TestScript extends BundleScript {
  run(segments: string[]): void {
    runCargoTestBudgeted(["animate-plugin"], this.repoRoot, segments);
  }
}

const router = new ScriptRouter(import.meta.dir).register("wasm", WasmScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
