#!/usr/bin/env bun
/** vcs TypeScript package */
import { BundleScript, ScriptRouter } from "../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
class TestScript extends BundleScript {
  run(): void { console.log("[TRACE] vcs ts ok"); }
}
const router = new ScriptRouter(import.meta.dir).register("test", TestScript);
await runScriptMain(router, { defaultCommand: "test" });
