#!/usr/bin/env bun
/** 🧭 `@semio-tech/framework-platform-renderer-react` task router: `bun ./script.ts test|policy`. */
import type { BundleLinter } from "../../../../../repo/lib/js/index.ts";
import { dependencyBoundaryBreachesForBundleDir } from "../../../../../repo/lib/js/index.ts";
import { getWorkspaceRoot } from "../../../../../repo/lib/js/index.ts";
import { BundleScript, ScriptRouter, runBundleScriptMain, runVitest } from "../../../../../repo/lib/js/index.ts";
import { defineLint } from "../../../../../repo/lib/js/index.ts";

export const policy = defineLint("@semio-tech/framework-platform-renderer-react-bundle", (l: BundleLinter) => {
  const repoRoot = getWorkspaceRoot();
  return dependencyBoundaryBreachesForBundleDir(repoRoot, l.root());
});

class TestScript extends BundleScript {
  run(segments: string[]): void {
    runVitest(this.root, segments, "vitest.config.ts");
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
