#!/usr/bin/env repo
import { receiveScriptProcessInvocation } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { runScriptProgram } from "../../🧪️tests/🔬️rust-package-commands/🟦️.ts";


const packageRoot = import.meta.dir;/** 🚪️ `regenerate-example-assets` — runs the program of this script. */
class RegenerateExampleAssetsScript extends BundleScript {
  async run(): Promise<void> {
    await runScriptProgram(packageRoot);
  }
}
const router = new ScriptRouter(import.meta.dir).register("regenerate-example-assets", RegenerateExampleAssetsScript);
if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, argv: ["regenerate-example-assets"] }));
