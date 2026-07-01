#!/usr/bin/env bun
/** 🦀 `@semio-tech/dag-core` router: `bun ./script.ts wasm`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runWasmPackWebBuild } from "../../../../../repo/lib/js/index.ts";

class WasmScript extends BundleScript {
  run(): void {
    runWasmPackWebBuild({
      rsDir: this.root,
      skipEnvVar: "DAG_CORE_SKIP_WASM_BUILD",
      logPrefix: "graph/dag",
      wasmBaseName: "mathematical_graph_port_directed_dag",
      pkg: {
        name: "@semio-tech/dag-core",
        files: [
          "mathematical_graph_port_directed_dag_bg.wasm",
          "mathematical_graph_port_directed_dag.js",
          "mathematical_graph_port_directed_dag.d.ts",
          "mathematical_graph_port_directed_dag_bg.wasm.d.ts",
        ],
        main: "mathematical_graph_port_directed_dag.js",
        module: "mathematical_graph_port_directed_dag.js",
        types: "mathematical_graph_port_directed_dag.d.ts",
      },
    });
  }
}

const router = new ScriptRouter(import.meta.dir).register("wasm", WasmScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
