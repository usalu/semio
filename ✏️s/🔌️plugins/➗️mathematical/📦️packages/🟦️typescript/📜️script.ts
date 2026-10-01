#!/usr/bin/env bun
/** ➗️ Mathematical package command router. */
import { BundleScript, ScriptRouter } from "../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { verifyMathematicalPublicationAuthority } from "../../🧪️tests/📣️publication-authority/🟦️.ts";

class TestScript extends BundleScript {
  async run(): Promise<void> {
    await verifyMathematicalPublicationAuthority(this.repoRoot, this.root);
  }
}
const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("publication-authority-audit", TestScript);
await runScriptMain(router, { defaultCommand: "test" });
