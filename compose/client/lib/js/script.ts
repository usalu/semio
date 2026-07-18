#!/usr/bin/env bun
/** 🧭 `@semio-tech/compose-js` policy/test router: `bun ./script.ts policy|test|test-e2e`. */
import type { FileLinter } from "../../../../repo/lib/js/index.ts";
import { dependencyBoundaryBreachesForFile } from "../../../../repo/lib/js/index.ts";
import { getWorkspaceRoot } from "../../../../repo/lib/js/index.ts";
import { BundleScript, ScriptRouter, runBundleScriptMain, runVitest, runBunx, devToolingEnv } from "../../../../repo/lib/js/index.ts";
import { defineLint } from "../../../../repo/lib/js/index.ts";

export const policyFile = "index.ts";

export const policy = defineLint("@semio-tech/compose-js-index", (l: FileLinter) => {
  const repoRoot = getWorkspaceRoot();
  const file = l.path();
  return dependencyBoundaryBreachesForFile(repoRoot, file, l.content(), file);
});

//#region 🧪Test
/** 🧪Fast protocol/wire unit suite (no WASM session boot); budgeted ≤ 30s via {@link runVitest}. */
class TestScript extends BundleScript {
  run(segments: string[]): void {
    process.env["COMPOSE_JS_RUN_EMBEDDED_TESTS"] = "1";
    runVitest(this.root, segments, "vite.config.ts");
  }
}

/** 🐘Full rs↔js WASM session integration suite (`Session.openInMemory` boots the rs engine per test); excluded from the default ≤30s `test` budget. */
class TestE2eScript extends BundleScript {
  run(segments: string[]): void {
    runBunx(["vitest", "run", "--config", "vite.config.ts", "--passWithNoTests", ...segments], this.root, devToolingEnv({ COMPOSE_JS_RUN_EMBEDDED_E2E_TESTS: "1" }));
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("test-e2e", TestE2eScript);
//#endregion 🧪Test

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
