#!/usr/bin/env bun
/** 🧭 `@semio-tech/cad-js-renderer` task router: `bun ./script.ts test [args…]`. */
import type { FileLinter } from "../../repo/lib/js/index.ts";
import { dependencyBoundaryBreachesForFile } from "../../repo/lib/js/index.ts";
import { getWorkspaceRoot } from "../../repo/lib/js/index.ts";
import { BundleScript, ScriptRouter, runBundleScriptMain, runVitest, resolveTestLevel } from "../../repo/lib/js/index.ts";
import { defineLint } from "../../repo/lib/js/index.ts";

export const policyFile = "index.tsx";

export const policy = defineLint("@semio-tech/cad-js-renderer-index", (l: FileLinter) => {
  const repoRoot = getWorkspaceRoot();
  const file = l.path();
  return dependencyBoundaryBreachesForFile(repoRoot, file, l.content(), file);
});

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runVitest(this.root, rest, "js/vitest.config.ts");
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url);
