#!/usr/bin/env bun
/** 🧭 Coda programming go router: `bun ./script.ts test`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runCmd } from "../../../../repo/lib/js/index.ts";

class TestScript extends BundleScript {
  run(): void {
    runCmd("go", ["test", "./..."], { cwd: this.root });
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url);
