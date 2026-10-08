import { test, expect } from "bun:test";
import { readFileSync } from "node:fs";
import Ajv from "ajv";

test("production maintenance edit carries explicit schema branch provenance", () => {
  const fixtures = new URL("../../🧫️fixtures/production-envelope-edit/", import.meta.url);
  const fixture = JSON.parse(readFileSync(new URL("🔣️.json", fixtures), "utf8"));
  const ajv = new Ajv({ strict: true });
  const schema = JSON.parse(readFileSync(new URL("../../../../../../../../../../../../🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧬️schema/🔣️.json", import.meta.url), "utf8"));
  const validate = ajv.compile(schema.$defs.Edit);
  for (const row of fixture.cases) expect(validate(row.edit)).toBe(row.accepted);
  const source = readFileSync(new URL("🦀️.rs", import.meta.url), "utf8"), begin = source.indexOf('"edits": [{', source.indexOf("fn production_envelope_wire("));
  const block = source.slice(begin + '"edits": [{'.length, source.indexOf("}],", begin)), edit: Record<string, unknown> = {};
  for (const match of block.matchAll(/"([A-Za-z]+)": ([^\n]+?)(?:,|$)/gm)) edit[match[1]!] = match[2] === "mutation_hex" ? ["00"] : JSON.parse(match[2]!);
  expect(validate(edit)).toBe(true);
  expect(edit.line).toBeNull();
  console.log(`[DEBUG] Independent Ajv maintenance edit: neutral=${fixture.cases.length} required=${schema.$defs.Edit.required.join(",")} explicit-trunk=${edit.line === null}`);
});
