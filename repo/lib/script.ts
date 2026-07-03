#!/usr/bin/env bun
/** 🧭 `@semio-tech/repo-lib` folder policy router: `bun ./script.ts policy`. */
import { readdirSync, statSync } from "node:fs";
import { join } from "node:path";
import type { FolderLinter } from "./src/index.ts";
import { getWorkspaceRoot, runPolicyOnlyMain, defineLint } from "./src/index.ts";

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

await runPolicyOnlyMain(import.meta.url);
