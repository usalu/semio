#!/usr/bin/env bun
/** 🛒 `@semio-tech/sourcing-curate-rs` router: `bun ./script.ts <wasm|test>`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runCargoTestBudgeted, runWasmPackWebBuild } from "../../../repo/lib/js/index.ts";

class WasmScript extends BundleScript {
  run(): void {
    runWasmPackWebBuild({
      rsDir: this.root,
      skipEnvVar: "SOURCING_CURATE_RS_SKIP_WASM_BUILD",
      logPrefix: "sourcing/curate/rs",
      wasmBaseName: "sourcing_curate",
      pkg: {
        name: "@semio-tech/sourcing-curate-rs",
        files: ["sourcing_curate_bg.wasm", "sourcing_curate.js", "sourcing_curate.d.ts", "sourcing_curate_bg.wasm.d.ts"],
        main: "sourcing_curate.js",
        module: "sourcing_curate.js",
        types: "sourcing_curate.d.ts",
      },
    });
  }
}

class TestScript extends BundleScript {
  run(segments: string[]): void {
    runCargoTestBudgeted(["sourcing_curate"], this.repoRoot, segments);
  }
}

const router = new ScriptRouter(import.meta.dir).register("wasm", WasmScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
