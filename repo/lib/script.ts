#!/usr/bin/env bun
/** 🧭 `@repo/lib` folder policy router: `bun ./script.ts policy`. */
import { readdirSync, statSync } from "node:fs";
import { join } from "node:path";
import type { FolderLinter } from "./js/src/linter.ts";
import { getWorkspaceRoot } from "./js/src/cli.ts";
import { dispatchPolicyArgv } from "./js/src/policy-cli.ts";
import { defineLint } from "./js/src/script.ts";

export const policy = defineLint("repo-lib-folder", (l: FolderLinter) => {
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

const segs = process.argv.slice(2);
if (await dispatchPolicyArgv(segs, import.meta.url)) {
  /* exited */
} else {
  console.error("usage: bun ./script.ts policy");
  process.exit(1);
}
