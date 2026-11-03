/** 🧬️ Schema-sync witness: for every norm artifact (EN 1990 to EN 1999) and every shared norm crate, the committed JSON schema (source of truth) agrees with its Rust model and its TypeScript, proto and GraphQL twins on field names, order, types, optionality, integer bounds and enum members. The checker is language-agnostic: it reads the five committed files and shares no code with the twins it judges.
 * @see ./🐍️check.py (report) and ./🐍️sync.py (repair: JSON follows the Rust model, the twins follow the JSON); the twin generator `repo ./📜️script.ts schema twins` produces every twin from the JSON Schema */
import { describe, expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { join } from "node:path";
import { renderTwinReport, runTwins } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🧬️twin-projection/🟦️.ts";

const checker = join(import.meta.dir, "🐍️check.py");
const interpreters: readonly (readonly string[])[] = [["python3", "-I"], ["python", "-I"], ["py", "-3", "-I"]];

const run = (args: readonly string[]) => {
  for (const [command, ...flags] of interpreters) {
    const result = spawnSync(command as string, [...flags, checker, ...args], { encoding: "utf8", env: { ...process.env, PYTHONUTF8: "1" } });
    if (result.error === undefined) return result;
  }
  throw new Error("[schema-sync] no Python 3 interpreter found (python3, python, py -3)");
};

describe("schema sync", () => {
  test("every JSON schema, its Rust model and its TypeScript, proto and GraphQL twins agree", () => {
    const result = run(["--limit", "40"]);
    expect(result.stdout + result.stderr).toContain("mismatches=0");
    expect(result.status).toBe(0);
  });

  test("every committed twin equals what its JSON Schema generates, byte for byte (`repo ./📜️script.ts schema twins check --strict`)", () => {
    const root = join(import.meta.dir, "..", "..", "..", "..", "..");
    const report = runTwins(root, ["check", "--under", "✏️s/🔌️plugins/📕️norm", "--explain"]);
    expect(report.problems, renderTwinReport("check", report)).toEqual([]);
    expect(report.counts.drift, renderTwinReport("check", report)).toBe(0);
    expect(report.counts.style, renderTwinReport("check", report)).toBe(0);
    expect(report.styled, report.styled.slice(0, 40).join("\n")).toEqual([]);
  });
});
