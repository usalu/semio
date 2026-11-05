/** 🧬️ Schema-sync witness: for every norm artifact (EN 1990 to EN 1999) and every shared norm crate, the committed JSON schema (source of truth) agrees with its Rust model and its TypeScript, proto and GraphQL twins on field names, order, types, optionality, integer bounds and enum members. The checker is language-agnostic: it reads the five committed files and shares no code with the twins it judges.
 * @see ./🐍️check.py (report) and ./🐍️sync.py (repair: JSON follows the Rust model, the twins follow the JSON); the twin generator `repo ./📜️script.ts schema twins` produces every twin from the JSON Schema */
import { describe, expect, test } from "bun:test";
import { runToolV1 } from "@semio-tech/process/tools/run";
import enumVectors from "../🧫️fixtures/🧬️proto-enums/🔣️.json" with { type: "json" };
import { join } from "node:path";
import { renderTwinReport, runTwins } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🧬️twin-projection/🟦️.ts";

const checker = join(import.meta.dir, "🐍️check.py");
const run = (args: readonly string[] = []) => runToolV1("python", ["-I", checker, ...args], {
  root: join(import.meta.dir, "../../../../.."),
  capture: true,
  check: false,
  overrides: { PYTHONUTF8: "1", PYTHONDONTWRITEBYTECODE: "1" },
});

describe("schema sync", () => {
  test("protobuf enum prefixes preserve consecutive uppercase word boundaries", async () => {
    const result = await runToolV1("python", ["-I", "-c", "import json,runpy,sys; module=runpy.run_path(sys.argv[1]); print(json.dumps(module['parse_proto'](sys.argv[2]).enums))", checker, join(import.meta.dir, "../🧫️fixtures/🧬️proto-enums/🛰️.proto")], { root: join(import.meta.dir, "../../../../.."), capture: true, check: false, overrides: { PYTHONDONTWRITEBYTECODE: "1" } });
    expect(result.status).toBe(0);
    expect(JSON.parse(result.stdout)).toEqual(enumVectors);
  }, 30000);

  test("every JSON schema, its Rust model and its TypeScript, proto and GraphQL twins agree", async () => {
    const result = await run(["--limit", "40"]);
    expect(result.stdout + result.stderr).toContain("mismatches=0");
    expect(result.status).toBe(0);
  }, 120000);

  test("every committed twin equals what its JSON Schema generates, byte for byte (`repo ./📜️script.ts schema twins check --strict`)", () => {
    const root = join(import.meta.dir, "..", "..", "..", "..", "..");
    const report = runTwins(root, ["check", "--under", "✏️s/🔌️plugins/📕️norm", "--explain"]);
    expect(report.problems, renderTwinReport("check", report)).toEqual([]);
    expect(report.counts.drift, renderTwinReport("check", report)).toBe(0);
    expect(report.counts.style, renderTwinReport("check", report)).toBe(0);
    expect(report.styled, report.styled.slice(0, 40).join("\n")).toEqual([]);
  }, 120000);
});
