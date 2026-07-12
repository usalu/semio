#!/usr/bin/env bun
/** 🦀 `@semio-tech/dag-core` router: `bun ./script.ts wasm`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runWasmPackWebBuild } from "../../../../../repo/lib/js/index.ts";
import { join } from "node:path";

class WasmScript extends BundleScript {
  run(): void {
    runWasmPackWebBuild({
      rsDir: join(this.root, "rs"),
      skipEnvVar: "DAG_CORE_SKIP_WASM_BUILD",
      logPrefix: "board/dag",
      wasmBaseName: "infinite_board_port_directed_dag",
      pkg: {
        name: "@semio-tech/dag-core",
        files: ["infinite_board_port_directed_dag_bg.wasm", "infinite_board_port_directed_dag.js", "infinite_board_port_directed_dag.d.ts", "infinite_board_port_directed_dag_bg.wasm.d.ts"],
        main: "infinite_board_port_directed_dag.js",
        module: "infinite_board_port_directed_dag.js",
        types: "infinite_board_port_directed_dag.d.ts",
      },
    });
  }
}

const router = new ScriptRouter(import.meta.dir).register("wasm", WasmScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
