#!/usr/bin/env bun
/** 🧭 `@cad/js/kernel/brepjs` task router: `bun ./script.ts test|fixture [args…]`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runVitest } from "../../../../repo/lib/js/src/index.ts";

class TestScript extends BundleScript {
  run(segments: string[]): void {
    runVitest(this.root, segments);
  }
}

class FixtureScript extends BundleScript {
  run(segments: string[]): void {
    process.env.CAD_GENERATE_STEP_FIXTURES = "1";
    runVitest(this.root, segments);
  }
}

const router = new ScriptRouter(import.meta.dir)
  .register("test", TestScript)
  .register("fixture", FixtureScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
