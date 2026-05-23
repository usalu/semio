import type { TechnologyLinter } from "./lib/js/src/linter.ts";
import { defineLint } from "./lib/js/src/script.ts";

/** 📜Flags technologies with no bundles. */
export default defineLint("repo-technology", (l: TechnologyLinter) => {
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
