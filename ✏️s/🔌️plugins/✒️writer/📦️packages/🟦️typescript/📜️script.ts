#!/usr/bin/env bun
/** writer TypeScript package */
import { BundleScript, ScriptRouter, runBundleScriptMain } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts";
class TestScript extends BundleScript {
  run(): void { console.log("[DEBUG] writer ts ok"); }
}
const router = new ScriptRouter(import.meta.dir).register("test", TestScript);
await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
