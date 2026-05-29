#!/usr/bin/env bun
/** 🧭 Repo CLI bundle policy: `bun ./script.ts policy` lints `main.go`. */
import type { FileLinter } from "../../lib/js/src/linter.ts";
import { dispatchPolicyArgv } from "../../lib/js/src/policy-cli.ts";
import { defineLint } from "../../lib/js/src/script.ts";

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

const segs = process.argv.slice(2);
if (await dispatchPolicyArgv(segs, import.meta.url)) {
  /* exited */
} else {
  console.error("usage: bun ./script.ts policy");
  process.exit(1);
}
