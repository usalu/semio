#!/usr/bin/env bun
/** 🀄️ WFC engine crate router: `bun ./📜️script.ts check|test`. */
import { resolveTestLevel, runCargo } from "../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";

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
await runScriptMain(router, { defaultCommand: "test" });
//#endregion 🧪️Validation
