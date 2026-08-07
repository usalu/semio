#!/usr/bin/env bun
/** 🏢️ `@semio-tech/cad-extension-aec-building-rust` router: `bun ./📜️script.ts test`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runCargoTestBudgeted } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️lib/📦️packages/🟦️typescript/📦️index.ts";

class TestScript extends BundleScript {
  run(_segments: string[]): void {
    runCargoTestBudgeted(["semio-s-plugin-cad-aec-building"], this.repoRoot);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);
await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
