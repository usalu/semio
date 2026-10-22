#!/usr/bin/env repo
import { receiveScriptProcessInvocation } from "../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { runTestScript } from "../../🧪️tests/🔬️typescript-package-commands/🟦️.ts";
//#endregion 🔖️InteractiveJobExamples

//#region 🧪️InteractiveJobSourceTest
class TestScript extends BundleScript {
  async run(): Promise<void> {
    await runTestScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation });
  }
}
//#endregion 🧪️InteractiveJobSourceTest

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);
await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, defaultCommand: "test" }));
