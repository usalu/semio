#!/usr/bin/env bun
/** 🧭️ `@semio-tech/compose-js` policy/test router: `bun ./📜️script.ts policy|test [level]`. */
import type { FileLinter } from "@semio-tech/repo-lib";
import { BundleScript, ScriptRouter, defineLint, dependencyBoundaryBreachesForFile, getWorkspaceRoot, resolveTestLevel, runBundleScriptMain, runVitest } from "@semio-tech/repo-lib";

export const policyFile = "index.ts";

export const policy = defineLint("@semio-tech/compose-js-index", (l: FileLinter) => {
  const repoRoot = getWorkspaceRoot();
  const file = l.path();
  return dependencyBoundaryBreachesForFile(repoRoot, file, l.content(), file);
});

//#region 🧪️Test
/** 🧪️Level-budgeted; the WASM session suite is gated to `long`+ inside `index.ts`'s embedded `⚡️FastUnit`/`🐘️WasmE2e` regions. */
class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    process.env["COMPOSE_JS_RUN_EMBEDDED_TESTS"] = "1";
    runVitest(this.root, rest, "vite.config.ts");
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);
//#endregion 🧪️Test

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
