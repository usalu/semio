import type { FileLinter } from "../../../../repo/lib/js/src/linter.ts";
import { defineLint } from "../../../../repo/lib/js/src/script.ts";
import { dependencyBoundaryBreachesForFile } from "../../../../repo/lib/js/src/dependency-boundary.ts";
import { getWorkspaceRoot } from "../../../../repo/lib/js/src/cli.ts";

/** 📜Dependency-boundary lint for @framework/playground-renderer-react entry. */
export default defineLint("@framework/playground-renderer-react-index", (l: FileLinter) => {
  const repoRoot = getWorkspaceRoot();
  const file = l.path();
  return dependencyBoundaryBreachesForFile(repoRoot, file, l.content(), file);
});
