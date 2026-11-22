import { test, expect } from "bun:test";
import { readFileSync } from "node:fs";
import Ajv from "ajv";
import { concat, difference, isEqual, slice } from "lodash";
import { frameworkSourceUrl } from "@semio-tech/framework/test/source-text";

test("timeline drops conform to the schema and produce the expected stable order", () => {
  const fixture = JSON.parse(readFileSync(new URL("../🧫️fixtures/🪜️timeline-drop.json", import.meta.url), "utf8"));
  const schema = JSON.parse(readFileSync(new URL("../../🎮️commands/🪜️step/🧬️drop.json", import.meta.url), "utf8"));
  const validate = new Ajv({ strict: true }).compile(schema);
  for (const { order, schemaAccepted = true, ...fields } of fixture.cases) {
    const args = { documentId: fixture.documentId, ...fields };
    expect(validate(args)).toBe(schemaAccepted);
    const remaining = difference(fixture.steps, [args.stepId]);
    const target = remaining.indexOf(args.targetId);
    if (!schemaAccepted || args.documentId !== fixture.documentId || target < 0 || remaining.length === fixture.steps.length) {
      expect(order).toBeNull();
      continue;
    }
    const index = target + Number(args.dropPosition === "after");
    const actual = concat(slice(remaining, 0, index), args.stepId, slice(remaining, index));
    expect(isEqual(actual, fixture.steps) ? null : actual).toEqual(order);
  }
});

test("production maintenance edit carries explicit schema branch provenance", () => {
  const fixtures = new URL("../🧫️fixtures/production-envelope-edit/", import.meta.url);
  const fixture = JSON.parse(readFileSync(new URL("🔣️.json", fixtures), "utf8"));
  const ajv = new Ajv({ strict: true });
  const schema = JSON.parse(readFileSync(frameworkSourceUrl("🔨️modules/📡️replication/🎮️mutation/🧬️schema/🔣️.json"), "utf8"));
  const validate = ajv.compile(schema.$defs.Edit);
  for (const row of fixture.cases) expect(validate(row.edit)).toBe(row.accepted);
  const source = readFileSync(new URL("🏭️production-envelope/🦀️.rs", import.meta.url), "utf8"), begin = source.indexOf('"edits": [{', source.indexOf("fn production_envelope_wire("));
  const block = source.slice(begin + '"edits": [{'.length, source.indexOf("}],", begin)), edit: Record<string, unknown> = {};
  for (const match of block.matchAll(/"([A-Za-z]+)": ([^\n]+?)(?:,|$)/gm)) edit[match[1]!] = match[2] === "mutation_hex" ? ["00"] : JSON.parse(match[2]!);
  expect(validate(edit)).toBe(true);
  expect(edit.line).toBeNull();
});
