#!/usr/bin/env bun
/** 🧭 `@semio-tech/cad-js-kernel-brepjs` task router: `bun ./script.ts test|fixture [fundamental|quick|long|exhaustive] [args…]`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runVitest, resolveTestLevel } from "../../../../../repo/lib/js/index.ts";

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runVitest(this.root, rest, "js/vitest.config.ts");
  }
}

class FixtureScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    process.env.CAD_GENERATE_STEP_FIXTURES = "1";
    runVitest(this.root, rest, "js/vitest.config.ts");
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("fixture", FixtureScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
