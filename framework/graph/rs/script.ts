#!/usr/bin/env bun
/** 🦀 `@semio-tech/framework-graph-rs` router: `bun ./script.ts wasm`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runCargoTestBudgeted, runWasmPackWebBuild } from "../../../repo/lib/js/index.ts";

class WasmScript extends BundleScript {
  run(): void {
    runWasmPackWebBuild({
      rsDir: this.root,
      skipEnvVar: "FRAMEWORK_GRAPH_RS_SKIP_WASM_BUILD",
      logPrefix: "framework/graph/rs",
      wasmBaseName: "framework_graph",
      pkg: {
        name: "@semio-tech/framework-graph-rs",
        files: ["framework_graph_bg.wasm", "framework_graph.js", "framework_graph.d.ts", "framework_graph_bg.wasm.d.ts"],
        main: "framework_graph.js",
        module: "framework_graph.js",
        types: "framework_graph.d.ts",
      },
    });
  }
}

class TestScript extends BundleScript {
  run(segments: string[]): void {
    runCargoTestBudgeted(["framework_graph"], this.repoRoot, segments);
  }
}

const router = new ScriptRouter(import.meta.dir).register("wasm", WasmScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
