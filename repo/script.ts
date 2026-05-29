#!/usr/bin/env bun
/** 🧭 `@repo` technology policy router: `bun ./script.ts policy`. */
import type { TechnologyLinter } from "./lib/js/src/linter.ts";
import { runPolicyOnlyMain } from "./lib/js/src/bundle-script.ts";
import { defineLint } from "./lib/js/src/script.ts";

export const policy = defineLint("repo-technology", (l: TechnologyLinter) => {
  if (l.bundles().length > 0) return [];
  return [
    l.breach({
      id: "no-bundles",
      summary: "Technology has zero bundles",
      kind: "lint/technology/no-bundles",
      priority: "medium",
    }),
  ];
});

await runPolicyOnlyMain(import.meta.url);
