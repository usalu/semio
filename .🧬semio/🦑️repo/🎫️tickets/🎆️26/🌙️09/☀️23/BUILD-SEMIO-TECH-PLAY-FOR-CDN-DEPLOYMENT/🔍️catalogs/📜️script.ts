#!/usr/bin/env bun
/** 🔎️ Validates the shared language-neutral Process catalog witnesses with independent Ajv. */
import { strict as assert } from "node:assert";
import { existsSync, readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import Ajv from "ajv";

function workspace(): string {
  let directory = import.meta.dir;
  while (!existsSync(join(directory, "✏️s"))) {
    const parent = dirname(directory);
    if (parent === directory) throw new Error("Semio workspace is absent");
    directory = parent;
  }
  return directory;
}

function oracle(): void {
  const folder = join(workspace(), "✏️s/🔌️plugins/🏭️process/🧩️extensions/🧪️tests/🧫️fixtures");
  const schema = JSON.parse(readFileSync(join(folder, "🧬️schema/🔣️.json"), "utf8"));
  const cases: { catalogId: string; machineIds: string[]; capabilityCounts: number[] }[] = JSON.parse(readFileSync(join(folder, "🔣️.json"), "utf8"));
  const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
  assert.equal(validate(cases), true, JSON.stringify(validate.errors));
  assert.equal(new Set(cases.map(row => row.catalogId)).size, 4);
  assert.equal(validate(cases.map((row, index) => index === 0 ? { ...row, catalogId: "unknown" } : row)), false);
  assert.equal(validate(cases.map((row, index) => index === 0 ? { ...row, unexpected: true } : row)), false);
  for (const row of cases) assert.equal(row.machineIds.length, row.capabilityCounts.length);
  console.log("[DEBUG] Independent Ajv Process catalog oracle: " + cases.length + " catalogs, " + cases.reduce((count, row) => count + row.machineIds.length, 0) + " machines, " + cases.reduce((count, row) => count + row.capabilityCounts.reduce((sum, value) => sum + value, 0), 0) + " capabilities");
}

if (process.argv.length !== 3 || process.argv[2] !== "oracle") throw new Error("Usage: 📜️script.ts oracle");
oracle();
