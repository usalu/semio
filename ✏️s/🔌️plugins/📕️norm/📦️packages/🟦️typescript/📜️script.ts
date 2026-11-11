#!/usr/bin/env repo
import { receiveScriptProcessInvocation } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { join } from "node:path";
import { orchestratorBudgetOpts, runCmd } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runTestScript } from "../../🧪️tests/🔬️typescript-package-commands/🟦️.ts";
//#endregion 🔖️Types

//#region 🔖️Test
/** 🧵️ Independent TypeScript check of the same language-neutral cohort fixture the Rust oracle reads. */
class TestScript extends BundleScript {
  async run(): Promise<void> {
    await runTestScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation });
  }
}
//#endregion 🔖️Test

/** 🧾️ Runs the wire-twin witness: every committed norm mutation and snapshot against its generated TypeScript twin and the strict Ajv oracle, and the drift gate that every twin equals the generator output. */
class WireTwinsScript extends BundleScript {
  run(): void {
    runCmd(process.execPath, ["test", join(this.root, "../../🧪️tests/🧪️wire-twins/🟦️.ts"), join(this.root, "../../🧪️tests/🧪️wire-twins/🧬️drift/🟦️.ts")], { cwd: this.repoRoot, ...orchestratorBudgetOpts() });
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("wire-twins", WireTwinsScript);
await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, defaultCommand: "test" }));
