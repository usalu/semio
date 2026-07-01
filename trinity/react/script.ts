#!/usr/bin/env bun
/** 🧭 `@semio-tech/trinity-react` task router. */
import { join } from "node:path";
import { BundleScript, ScriptRouter, playPollingEnv, runBun, runBundleScriptMain, runVitest } from "../../repo/lib/js/index.ts";

const wasmScript = join(import.meta.dir, "../rewrite/engine/script.ts");

class TestScript extends BundleScript {
  run(segments: string[]): void {
    runBun([wasmScript, "wasm"], join(import.meta.dir, "../rewrite/engine"), playPollingEnv());
    runVitest(this.root, segments);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
