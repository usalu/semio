#!/usr/bin/env bun
/** 🧭 `@semio-tech/cad-js-query` task router: `bun ./script.ts test [args…]`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runVitest } from "../../../repo/lib/js/src/index.ts";

class TestScript extends BundleScript {
  run(segments: string[]): void {
    runVitest(this.root, segments);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
