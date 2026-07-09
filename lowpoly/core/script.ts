#!/usr/bin/env bun
/** 🦀 `@semio-tech/lowpoly-core` router: `bun ./script.ts test`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runCargo } from "../../repo/lib/js/index.ts";

class TestScript extends BundleScript {
  run(_segments: string[]): void {
    runCargo(["test", "-p", "lowpoly_core"], this.repoRoot);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
