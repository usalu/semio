#!/usr/bin/env bun
/** 🧭 `@flow/react` task router. */
import { join } from "node:path";
import { BundleScript, ScriptRouter, playPollingEnv, runBun, runBundleScriptMain, runVitest } from "../../repo/lib/js/src/index.ts";

const wasmScript = join(import.meta.dir, "../core/script.ts");

class TestScript extends BundleScript {
  run(segments: string[]): void {
    runBun([wasmScript, "wasm"], this.root, playPollingEnv());
    runVitest(this.root, segments);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url);
