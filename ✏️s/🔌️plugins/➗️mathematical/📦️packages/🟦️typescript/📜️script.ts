#!/usr/bin/env bun
/** ➗️ Mathematical package command router. */
import { BundleScript, ScriptRouter, runBundleScriptMain } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { verifyMathematicalPublicationAuthority } from "../../🧪️tests/📣️publication-authority/🟦️.ts";

class TestScript extends BundleScript {
  async run(): Promise<void> {
    await verifyMathematicalPublicationAuthority(this.repoRoot, this.root);
  }
}
const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("publication-authority-audit", TestScript);
await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
