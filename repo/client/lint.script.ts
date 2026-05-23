import { existsSync } from "node:fs";
import { join } from "node:path";
import type { BundleLinter } from "../lib/js/src/linter.ts";
import { defineLint } from "../lib/js/src/script.ts";
import { getWorkspaceRoot } from "../lib/js/src/cli.ts";

/** 📜Requires package.json at bundle root. */
export default defineLint("repo-client-bundle", (l: BundleLinter) => {
  const root = getWorkspaceRoot();
  const manifest = join(root, l.root(), "package.json");
  if (existsSync(manifest)) return [];
  return [
    l.breach({
      id: "missing-package-json",
      summary: "Bundle root is missing package.json",
      kind: "lint/bundle/package-json",
      priority: "medium",
    }),
  ];
});
