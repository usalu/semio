#!/usr/bin/env repo
import { receiveScriptProcessInvocation } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
/** 🌿️ vcs TypeScript package */
import { join } from "node:path";
import { runCmd, orchestratorBudgetOpts } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
class TestScript extends BundleScript {
  run(): void { runCmd(process.execPath,["test",join(this.root,"../../🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts")],{cwd:this.repoRoot,...orchestratorBudgetOpts()}); }
}
const router = new ScriptRouter(import.meta.dir).register("test", TestScript);
await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, ...({ defaultCommand: "test" }) }));
