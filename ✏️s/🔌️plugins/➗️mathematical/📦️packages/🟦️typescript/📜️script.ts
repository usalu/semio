#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
/** ➗️ Mathematical package command router. */
import { BundleScript, ScriptRouter } from "../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { verifyMathematicalPublicationAuthority } from "../../🧪️tests/📣️publication-authority/🟦️.ts";

class TestScript extends BundleScript {
  async run(): Promise<void> {
    await verifyMathematicalPublicationAuthority(this.repoRoot, this.root);
  }
}
const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("publication-authority-audit", TestScript);
await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, ...({ defaultCommand: "test" }) }));
