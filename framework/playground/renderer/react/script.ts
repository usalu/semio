#!/usr/bin/env bun
/** 🧭 `@framework/playground-renderer-react` task router: `bun ./script.ts test|policy`. */
import type { FileLinter } from "../../../../repo/lib/js/src/linter.ts";
import { dependencyBoundaryBreachesForFile } from "../../../../repo/lib/js/src/dependency-boundary.ts";
import { getWorkspaceRoot } from "../../../../repo/lib/js/src/cli.ts";
import { BundleScript, ScriptRouter, runBundleScriptMain, runVitest } from "../../../../repo/lib/js/src/bundle-script.ts";
import { defineLint } from "../../../../repo/lib/js/src/script.ts";

export const policyFile = "index.tsx";

export const policy = defineLint("@framework/playground-renderer-react-index", (l: FileLinter) => {
  const repoRoot = getWorkspaceRoot();
  const file = l.path();
  return dependencyBoundaryBreachesForFile(repoRoot, file, l.content(), file);
});

class TestScript extends BundleScript {
  run(segments: string[]): void {
    runVitest(this.root, segments);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url);
