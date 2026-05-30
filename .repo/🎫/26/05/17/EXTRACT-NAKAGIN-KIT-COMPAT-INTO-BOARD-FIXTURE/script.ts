#!/usr/bin/env bun
/** 🧩 Nakagin kit-compat extraction: `bun ./script.ts extract [--write-board]`. */
import { BundleScript, ScriptRouter, runBundleScriptMain } from "../../../../repo/lib/js/src/index.ts";

class ExtractScript extends BundleScript {
  async run(): Promise<void> {
    await import("./extract-nakagin-kit-compat.ts");
  }
}

const router = new ScriptRouter(import.meta.dir).register("extract", ExtractScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "extract" });
