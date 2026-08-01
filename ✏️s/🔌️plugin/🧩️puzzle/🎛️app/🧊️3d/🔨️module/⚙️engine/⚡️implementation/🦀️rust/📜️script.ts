#!/usr/bin/env bun
/** 🦀️ `semio-s-app-puzzle-3d-engine` router: `bun ./📜️script.ts test`. Headless (constitutional:
 * engine) — no wasm-bindgen build here anymore (`HEADLESS-ENGINE-LAW-AND-OFFENDER-FIXES`); the
 * wasm-pack build this crate used to own moved to the `🖱️ui` slot's own `📜️script.ts`, the only
 * remaining wasm-bindgen-exporting constitutional crate for this app. */
import { BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain, runCargoTestBudgeted } from "../../../../../../../../../🧰️framework/🛍️product/🦑️repo/🔨️module/📚️lib/⚡️implementation/🟦️typescript/📦️index.ts";

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runCargoTestBudgeted(["semio-s-app-puzzle-3d-engine"], this.repoRoot, rest);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
