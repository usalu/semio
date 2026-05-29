import type { FileLinter } from "../../../../repo/lib/js/src/linter.ts";
import { defineLint } from "../../../../repo/lib/js/src/script.ts";
import { dependencyBoundaryBreachesForFile } from "../../../../repo/lib/js/src/dependency-boundary.ts";
import { getWorkspaceRoot } from "../../../../repo/lib/js/src/cli.ts";

/** 📜Dependency-boundary lint for @cad/js-machine-stately entry. */
export default defineLint("@cad/js-machine-stately-index", (l: FileLinter) => {
  const repoRoot = getWorkspaceRoot();
  const file = l.path();
  return dependencyBoundaryBreachesForFile(repoRoot, file, l.content(), file);
});
