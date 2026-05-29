#!/usr/bin/env bun
/** 🧭 `@repo/client` bundle policy router: `bun ./script.ts policy`. */
import { existsSync } from "node:fs";
import { join } from "node:path";
import type { BundleLinter } from "../lib/js/src/linter.ts";
import { getWorkspaceRoot } from "../lib/js/src/cli.ts";
import { dispatchPolicyArgv } from "../lib/js/src/policy-cli.ts";
import { defineLint } from "../lib/js/src/script.ts";

export const policy = defineLint("repo-client-bundle", (l: BundleLinter) => {
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

const segs = process.argv.slice(2);
if (await dispatchPolicyArgv(segs, import.meta.url)) {
  /* exited */
} else {
  console.error("usage: bun ./script.ts policy");
  process.exit(1);
}
