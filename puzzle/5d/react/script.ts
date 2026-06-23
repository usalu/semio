#!/usr/bin/env bun
/** 🧭 `@semio-tech/puzzle-5d-react` task router: `bun ./script.ts test|policy [args…]`. */
import type { FileLinter } from "../../../repo/lib/js/src/index.ts";
import { dependencyBoundaryBreachesForFile } from "../../../repo/lib/js/src/index.ts";
import { getWorkspaceRoot } from "../../../repo/lib/js/src/index.ts";
import { BundleScript, ScriptRouter, runBundleScriptMain, runVitest } from "../../../repo/lib/js/src/index.ts";
import { defineLint } from "../../../repo/lib/js/src/index.ts";

export const policyFile = "index.tsx";

export const policy = defineLint("@semio-tech/puzzle-5d-react-index", (l: FileLinter) => {
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

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
