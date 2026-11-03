/** 🏷️ Field-meta choice witness: every choice token of a properties-panel picker is a member of the schema enum the field edits, every enum member is offered, and each generated table equals what `🐍️generate.py` renders from the schema options (EN/DE labels). The checker is language-agnostic and shares no code with the Rust tables it judges.
 * @see ./🐍️check.py (report) and ./🐍️generate.py (rewrite the tables from the schema) */
import { describe, expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { join } from "node:path";

const checker = join(import.meta.dir, "🐍️check.py");
const interpreters: readonly (readonly string[])[] = [["python3", "-I"], ["python", "-I"], ["py", "-3", "-I"]];

const run = () => {
  for (const [command, ...flags] of interpreters) {
    const result = spawnSync(command as string, [...flags, checker], { encoding: "utf8", env: { ...process.env, PYTHONUTF8: "1" } });
    if (result.error === undefined) return result;
  }
  throw new Error("[field-meta-choices] no Python 3 interpreter found (python3, python, py -3)");
};

describe("field-meta choices", () => {
  test("every choice token is a member of the schema enum it edits", () => {
    const result = run();
    expect(result.stdout + result.stderr).toContain("mismatches=0");
    expect(result.status).toBe(0);
  });
});
