#!/usr/bin/env bun
/** 🧠 `@semio-tech/trinity-ram` router: `bun ./script.ts <test>`. */
import { BundleScript, ScriptRouter, playPollingEnv, runBundleScriptMain, runCargo } from "../../repo/lib/js/src/index.ts";

class TestScript extends BundleScript {
  run(segments: string[]): void {
    runCargo(["test", "-p", "trinity_ram", ...segments], this.repoRoot, playPollingEnv());
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
