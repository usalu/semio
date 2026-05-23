import type { FileLinter } from "../../lib/js/src/linter.ts";
import { defineLint } from "../../lib/js/src/script.ts";

/** 📜Flags very large files (medium priority; avoids failing CI on normal sizes). */
export default defineLint("repo-client-cli-main-go", (l: FileLinter) => {
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
