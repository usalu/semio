#!/usr/bin/env bun
/** 🧭️ `@semio-tech/mit-bestand-demonstrator` task router: `bun ./📜️script.ts <test> [args…]`. */
import { join } from "node:path";
import { BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain, runVitest } from "../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { demonstratorRuntimeBuildVariants } from "./🔨️modules/🧩️runtime/🟦️.ts";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runVitest(this.root, rest, "./🧪️tests/🎚️config/🟦️.ts");
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

if (import.meta.main) await runBundleScriptMain(router, import.meta.url);

if (import.meta.vitest) {
  const { registerTests1 } = await import("./🧪️tests/🧪️demonstratorruntimebuildvariants/🟦️.ts");
  await registerTests1(import.meta.vitest, { demonstratorRuntimeBuildVariants, join }, { directory: import.meta.dir, url: import.meta.url });
}
