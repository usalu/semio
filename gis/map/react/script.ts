#!/usr/bin/env bun
/** 🧭 `@gis/map/react` task router: `bun ./script.ts test`. */
import { join } from "node:path";
import { BundleScript, ScriptRouter, runBundleScriptMain, runBun, runVitest } from "../../../repo/lib/js/src/index.ts";

const wasmScript = join(import.meta.dir, "../rs/script.ts");

class TestScript extends BundleScript {
  run(segments: string[]): void {
    runBun([wasmScript, "wasm"], this.root);
    runVitest(this.root, segments);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url);
