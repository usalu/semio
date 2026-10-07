/** 🎛️ GIS mutation descriptors against a language-neutral fixture and the independent jsonschema validator. */
import { expect, test } from "bun:test";
import { readFileSync, readdirSync } from "node:fs";
import { resolve } from "node:path";
import { Validator } from "jsonschema";
import { argControl, mutationInputAudit } from "../../../../../🧰️framework/🔨️modules/🛂️manifest/🟦️.ts";
import fixture from "../../🧫️fixtures/🎛️mutation-inputs/🔣️.json";

const root = resolve(import.meta.dir, "../..");
const documents = new Map<string, object>();
const visit = (path: string): void => {
  for (const entry of readdirSync(path, { withFileTypes: true })) {
    const child = resolve(path, entry.name);
    if (entry.isDirectory()) visit(child);
    else if (entry.name === "🔣️.json" && child.includes("🧬️schema")) {
      const schema = JSON.parse(readFileSync(child, "utf8"));
      if (schema.$id) documents.set(schema.$id, schema);
    }
  }
};
visit(resolve(root, "🗿️artifacts"));
const validator = new Validator();
validator.setSchemas(Object.fromEntries(documents));

for (const row of fixture.cases) test(`GIS ${row.kind} publishes editable localized inputs`, () => {
  const schema = JSON.parse(readFileSync(resolve(root, row.schema), "utf8"));
  expect(validator.validate(row.payload, schema).valid).toBe(true);
  const audit = mutationInputAudit(schema, id => documents.get(id));
  expect(audit.findings).toEqual([]);
  expect(audit.inputs.map(input => input.id)).toEqual(row.inputs);
  for (const input of audit.inputs) expect(input.label.native).toMatchObject({ en: expect.any(String), de: expect.any(String) });
  const index = audit.inputs.find(input => input.id === "/index" || input.id === "/toIndex");
  if (index) expect(argControl(index).kind).toBe("stepper");
});
