/** 🧫️ Batch payloads match the shared specification and independent Ajv schema validation. */
import { expect, test } from "bun:test";
import Ajv from "ajv";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { parsePinBatch } from "../../🧬️mutations/🟦️.ts";

const fixture = JSON.parse(readFileSync(resolve(import.meta.dir, "../../🧬️mutations/🧪️tests/🧫️pin-batch/🔣️.json"), "utf8"));
const schemaPaths = ["../../🧬️mutations/📌️pin-solution/🧬️schema/🔣️.json", "../../🧬️mutations/↩️restore-pins/🧬️schema/🔣️.json"];

for (const [index, kind] of fixture.kinds.entries()) {
  const schema = JSON.parse(readFileSync(resolve(import.meta.dir, schemaPaths[index]!), "utf8"));
  const validate = new Ajv({ strict: false }).compile(schema);
  for (const row of fixture.cases) {
    test(kind.kind + ": " + row.id, () => {
      const input = row.repeat ? { pins: Array.from({ length: row.repeat }, () => ({ ...row.payload })) } : row.payload;
      expect(validate(input)).toBe(row.valid);
      if (row.valid) expect(parsePinBatch(input)).toEqual(input);
      else expect(() => parsePinBatch(input)).toThrow();
    });
  }
}
