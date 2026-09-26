#!/usr/bin/env bun
/** 🌀️ `@semio-tech/procedural-plugin` router: `bun ./📜️script.ts test`. */
import { registerPlaygroundSiteBuildCommands, BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain, runCargoTestBudgeted } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargoTestBudgeted(["semio-s-plugin-procedural"], this.repoRoot, rest);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);
registerPlaygroundSiteBuildCommands(router);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
