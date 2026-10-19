#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { resolveTestLevel } from "../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🎚️budget/🟦️.ts";
/** 🀄️ WFC engine crate router: `bun ./📜️script.ts check|test`. */
import { runCargo } from "../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";

//#region 🧪️Validation
const PACKAGES = ["semio-s-plugin-wfc-engine", "semio-s-plugin-wfc-engine-foundation", "semio-s-plugin-wfc-engine-job", "semio-s-plugin-wfc-engine-kernel", "semio-s-plugin-wfc-engine-lattice", "semio-s-plugin-wfc-engine-oracle", "semio-s-plugin-wfc-engine-solve"].flatMap(name => ["-p", name]);

class CheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runCargo(["check", ...PACKAGES, "--lib", "--tests", ...segments], this.repoRoot);
  }
}
class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargo(["test", ...PACKAGES, "--lib", ...rest], this.repoRoot);
  }
}
const router = new ScriptRouter(import.meta.dir).register("check", CheckScript).register("test", TestScript);
await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, ...({ defaultCommand: "test" }) }));
//#endregion 🧪️Validation
