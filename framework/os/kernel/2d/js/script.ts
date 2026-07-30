#!/usr/bin/env bun
/** 🧭 `@semio-tech/kernel-2d-js` router: `bun ./script.ts test`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, resolveTestLevel, runVitest } from "../../../../../repo/lib/js/index.ts";

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runVitest(this.root, rest);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url);
