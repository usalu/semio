#!/usr/bin/env bun
/** 🀄️ WFC engine crate router: `bun ./📜️script.ts check|test`. */
import { BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain, runCargo } from "../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

//#region 🧪️Validation
class CheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runCargo(["check", "-p", "semio-s-plugin-wfc-engine", "--lib", "--tests", ...segments], this.repoRoot);
  }
}
class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargo(["test", "-p", "semio-s-plugin-wfc-engine", "--lib", ...rest], this.repoRoot);
  }
}
const router = new ScriptRouter(import.meta.dir).register("check", CheckScript).register("test", TestScript);
await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
//#endregion 🧪️Validation
