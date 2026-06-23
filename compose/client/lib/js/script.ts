#!/usr/bin/env bun
/** 🧭 `@compose/js` policy router: `bun ./script.ts policy`. */
import type { FileLinter } from "../../../../repo/lib/js/src/index.ts";
import { dependencyBoundaryBreachesForFile } from "../../../../repo/lib/js/src/index.ts";
import { getWorkspaceRoot } from "../../../../repo/lib/js/src/index.ts";
import { runPolicyOnlyMain } from "../../../../repo/lib/js/src/index.ts";
import { defineLint } from "../../../../repo/lib/js/src/index.ts";

export const policyFile = "index.ts";

export const policy = defineLint("@compose/js-index", (l: FileLinter) => {
  const repoRoot = getWorkspaceRoot();
  const file = l.path();
  return dependencyBoundaryBreachesForFile(repoRoot, file, l.content(), file);
});

await runPolicyOnlyMain(import.meta.url);
