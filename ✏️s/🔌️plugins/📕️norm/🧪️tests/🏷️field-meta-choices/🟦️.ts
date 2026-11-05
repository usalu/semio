/** 🏷️ Field-meta choice witness: every choice token of a properties-panel picker is a member of the schema enum the field edits, every enum member is offered, and each generated table equals what `🐍️generate.py` renders from the schema options (EN/DE labels). The checker is language-agnostic and shares no code with the Rust tables it judges.
 * @see ./🐍️check.py (report) and ./🐍️generate.py (rewrite the tables from the schema) */
import { describe, expect, test } from "bun:test";
import { runToolV1 } from "@semio-tech/process/tools/run";
import { join } from "node:path";

const checker = join(import.meta.dir, "🐍️check.py");
const run = (args: readonly string[] = []) => runToolV1("python", ["-I", checker, ...args], {
  root: join(import.meta.dir, "../../../../.."),
  capture: true,
  check: false,
  overrides: { PYTHONUTF8: "1", PYTHONDONTWRITEBYTECODE: "1" },
});

describe("field-meta choices", () => {
  test("every choice token is a member of the schema enum it edits", async () => {
    const result = await run();
    expect(result.stdout + result.stderr).toContain("mismatches=0");
    expect(result.status).toBe(0);
  }, 30000);
});
