#!/usr/bin/env bun
/** 🧭 `@flow/react` task router. */
import { join } from "node:path";
import { BundleScript, ScriptRouter, playPollingEnv, runBun, runBundleScriptMain, runVitest } from "../../repo/lib/js/src/index.ts";

const wasmScript = join(import.meta.dir, "../core/script.ts");
const moduleWasmScripts = ["core", "math", "text", "logic", "dictionary", "list", "brep", "bim"].map((name) => join(import.meta.dir, `../module/${name}/script.ts`));

function runFlowModuleWasmBuilds(): void {
  for (const script of moduleWasmScripts) {
    runBun([script, "wasm"], import.meta.dir, playPollingEnv());
  }
}

class TestScript extends BundleScript {
  run(segments: string[]): void {
    runBun([wasmScript, "wasm"], this.root, playPollingEnv());
    runFlowModuleWasmBuilds();
    runVitest(this.root, segments);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url);
