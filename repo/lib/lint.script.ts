import { readdirSync, statSync } from "node:fs";
import { join } from "node:path";
import type { FolderLinter } from "./js/src/linter.ts";
import { defineLint } from "./js/src/script.ts";
import { getWorkspaceRoot } from "./js/src/cli.ts";

/** 📜Flags direct children larger than 1 MiB. */
export default defineLint("repo-lib-folder", (l: FolderLinter) => {
  const root = getWorkspaceRoot();
  const dir = join(root, l.path());
  const big: string[] = [];
  for (const name of readdirSync(dir, { withFileTypes: true })) {
    if (!name.isFile()) continue;
    const p = join(dir, name.name);
    if (statSync(p).size > 1 << 20) big.push(name.name);
  }
  if (big.length === 0) return [];
  return [
    l.breach({
      id: "big-child",
      summary: `Child files exceed 1 MiB: ${big.join(", ")}`,
      kind: "lint/folder/child-size",
      priority: "medium",
    }),
  ];
});
