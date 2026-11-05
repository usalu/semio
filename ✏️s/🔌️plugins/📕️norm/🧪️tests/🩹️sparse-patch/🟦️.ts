/** 🩹️ Sparse patches follow native defaulted fields and their snapshot field types. */
import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { pathToFileURL } from "node:url";
import { semioSchemaAjvV1 } from "@semio-tech/framework/schema/oracles/validator";
import vectors from "../🧫️fixtures/🩹️sparse-patch/🔣️.json" with { type: "json" };

test("native sparse patch vectors match their independent schema oracle", async () => {
  for (const row of vectors.cases) {
    const root = join(import.meta.dir, "../../🗿️artifacts", row.artifact, "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema");
    const snapshot = JSON.parse(readFileSync(join(root, "📸️snapshot/🔣️.json"), "utf8"));
    const diff = JSON.parse(readFileSync(join(root, "🔺️diff/🔣️.json"), "utf8"));
    const ajv = semioSchemaAjvV1({ strict: true, allErrors: true }).addSchema(snapshot).addSchema(diff);
    const validate = ajv.compile({ $ref: `${diff.$id}#/$defs/${row.patch}` });
    expect(validate(row.value), `${row.patch}: ${JSON.stringify(validate.errors)}`).toBe(row.valid);
    const module = await import(pathToFileURL(join(root, "🔺️diff/🟦️.ts")).href);
    const read = () => module[`parse${row.patch}`](row.value);
    if (row.valid) expect(read()).toEqual(row.value);
    else expect(read).toThrow();
  }
});
