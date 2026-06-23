#!/usr/bin/env bun
/** 🧭 `@semio-tech/dag-react` task router. */
import { join } from "node:path";
import { BundleScript, ScriptRouter, playPollingEnv, runBun, runBundleScriptMain, runVitest } from "../../../../../../repo/lib/js/src/index.ts";

const wasmScript = join(import.meta.dir, "../script.ts");

class TestScript extends BundleScript {
  run(segments: string[]): void {
    runBun([wasmScript, "wasm"], join(import.meta.dir, ".."), playPollingEnv());
    runVitest(this.root, segments);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url);
