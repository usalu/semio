#!/usr/bin/env bun
/** 🧭 `@semio-tech/framework-os-core` task router. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runVitest, resolveTestLevel } from "../../../../repo/lib/js/index.ts";

/** ⏱️Level-budgeted; unmarked `import.meta.vitest` cases are `fundamental`. */
class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runVitest(this.root, rest, "js/vitest.config.ts");
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url);
