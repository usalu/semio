#!/usr/bin/env repo
import { receiveScriptProcessInvocation } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { BundleScript, ScriptRouter, runScriptMain } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { runScriptProgram } from "../../🧪️tests/🔬️typescript-package-commands/🟦️.ts";
/** 🚪️ `verify` — runs the program of this script. */
class VerifyScript extends BundleScript {
  async run(): Promise<void> {
    await runScriptProgram();
  }
}
const router = new ScriptRouter(import.meta.dir).register("verify", VerifyScript);
if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, argv: ["verify"] }));
