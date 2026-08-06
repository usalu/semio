#!/usr/bin/env bun
/** 🏗️ `@semio-tech/flow-extension-bim-rust` router: `bun ./📜️script.ts <test|wasm>`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runCargoTestBudgeted, runWasmPackWebBuild } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️lib/📦️packages/🟦️typescript/📦️index.ts";

class TestScript extends BundleScript {
  run(_segments: string[]): void {
    runCargoTestBudgeted(["semio-s-plugin-flow-extension-bim"], this.repoRoot);
  }
}

/** 📦️ Builds the standalone-wasm `pkg/` output consumed as `@semio-tech/flow-module-bim` — generated,
 * gitignored, and registered directly as a root `package.json` workspace member (mirrors flow's own
 * `🫀️core` wasm crate: no hand-authored npm wrapper, since Node's `exports` resolution forbids a
 * sibling-directory `../` escape and `wasm-pack`'s output dir is pinned to `<rsDir>/pkg`). */
class WasmScript extends BundleScript {
  run(): void {
    runWasmPackWebBuild({
      rsDir: this.root,
      skipEnvVar: "FLOW_MODULE_BIM_SKIP_WASM_BUILD",
      logPrefix: "flow/module/bim",
      wasmBaseName: "semio_s_plugin_flow_extension_bim",
      cargoFeatures: ["standalone-wasm"],
      noDefaultFeatures: true,
      pkg: {
        name: "@semio-tech/flow-module-bim",
        files: ["semio_s_plugin_flow_extension_bim_bg.wasm", "semio_s_plugin_flow_extension_bim.js", "semio_s_plugin_flow_extension_bim.d.ts", "semio_s_plugin_flow_extension_bim_bg.wasm.d.ts"],
        main: "semio_s_plugin_flow_extension_bim.js",
        module: "semio_s_plugin_flow_extension_bim.js",
        types: "semio_s_plugin_flow_extension_bim.d.ts",
      },
    });
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("wasm", WasmScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
