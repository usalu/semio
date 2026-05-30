#!/usr/bin/env bun
/** 🧭 Repo CLI bundle policy: `bun ./script.ts policy` lints `main.go`. */
import type { FileLinter } from "../../lib/js/src/index.ts";
import { runPolicyOnlyMain } from "../../lib/js/src/index.ts";
import { defineLint } from "../../lib/js/src/index.ts";

export const policyFile = "main.go";

export const policy = defineLint("repo-client-cli-main-go", (l: FileLinter) => {
  const n = l.lines().length;
  if (n > 10000) {
    return [
      l.breach({
        id: "line-budget",
        summary: `File has ${n} lines (> 10000)`,
        kind: "lint/file/line-budget",
        priority: "medium",
        reason: "Large files are harder to review",
        solution: "Split into smaller modules",
      }),
    ];
  }
  return [];
});

await runPolicyOnlyMain(import.meta.url);
