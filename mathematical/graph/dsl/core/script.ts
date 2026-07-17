#!/usr/bin/env bun
/** 🃏 `@semio-tech/graph-dsl-core` router: `bun ./script.ts test` (pure JS Jack query execution over board fixtures). */
import { BundleScript, ScriptRouter, runBundleScriptMain, runVitest } from "../../../../repo/lib/js/index.ts";

class TestScript extends BundleScript {
  run(segments: string[]): void {
    runVitest(this.root, segments, "js/vitest.config.ts");
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url);
