#!/usr/bin/env bun
/** 🧭 `@semio/sketchpad-js` policy router: `bun ./script.ts policy`. */
import type { FileLinter } from "../../../../../repo/lib/js/src/linter.ts";
import { dependencyBoundaryBreachesForFile } from "../../../../../repo/lib/js/src/dependency-boundary.ts";
import { getWorkspaceRoot } from "../../../../../repo/lib/js/src/cli.ts";
import { dispatchPolicyArgv } from "../../../../../repo/lib/js/src/policy-cli.ts";
import { defineLint } from "../../../../../repo/lib/js/src/script.ts";

export const policyFile = "index.ts";

export const policy = defineLint("@semio/sketchpad-js-index", (l: FileLinter) => {
  const repoRoot = getWorkspaceRoot();
  const file = l.path();
  return dependencyBoundaryBreachesForFile(repoRoot, file, l.content(), file);
});

const segs = process.argv.slice(2);
if (await dispatchPolicyArgv(segs, import.meta.url)) {
  /* exited */
} else {
  console.error("usage: bun ./script.ts policy");
  process.exit(1);
}
