import { test, expect } from "bun:test";
import { readFileSync } from "node:fs";
import Ajv2020 from "ajv/dist/2020.js";
import { compileJsonSchemaSubset } from "../../../../../../../🧰️framework/🔨️modules/🧬️schema/✅️validator/🟦️.ts";
test("actual retained Pack schema syntax, grammar and instance language", () => {
  const schema = JSON.parse(readFileSync("🧰️framework/🔨️modules/🎒️pack/🌱️value/🛫️encode/🫳️borrowed/🎒️document/⏳️cursor/🧬️schema/🔣️.json", "utf8"));
  const ajv = new Ajv2020({ strict: false });
  expect(ajv.validateSchema(schema)).toBe(true);
  const oracle = ajv.compile(schema), owned = compileJsonSchemaSubset(schema);
  for (const row of JSON.parse(readFileSync(new URL("./schema-pack-syntax-vectors.json", import.meta.url), "utf8"))) {
    expect(oracle(row.input), row.id).toBe(row.expected);
    expect(owned.validate(row.input).length === 0, row.id).toBe(row.expected);
  }
});
