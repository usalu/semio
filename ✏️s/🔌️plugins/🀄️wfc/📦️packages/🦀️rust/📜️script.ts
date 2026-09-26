#!/usr/bin/env bun
/** 🀄️ `@semio-tech/wfc-plugin` router: `bun ./📜️script.ts test`. */
import { registerPlaygroundSiteBuildCommands, BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain, runCargoTestBudgeted } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

process.env.RUST_MIN_STACK ??= String(32 * 1024 * 1024);

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargoTestBudgeted(["semio-s-plugin-wfc"], this.repoRoot, rest);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);
registerPlaygroundSiteBuildCommands(router);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
