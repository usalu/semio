#!/usr/bin/env bun
/** 🔍️ Validates Sourcing catalog neutral semantics and canonical JSON ownership independently. */
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
  const base = join(workspace(), "✏️s/🔌️plugins/🪵️sourcing/🧩️extensions"), folder = join(base, "🧪️tests/🧫️fixtures");
  const rows: { moduleId: string; typology: { id: string }; kindIds: string[]; availability: number[] }[] = JSON.parse(readFileSync(join(folder, "🔣️.json"), "utf8"));
  const schema = JSON.parse(readFileSync(join(folder, "📐️schema.json"), "utf8")), validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
  assert.equal(validate(rows), true, JSON.stringify(validate.errors));
  assert.equal(validate(rows.map((row, index) => index === 0 ? { ...row, moduleId: "unknown" } : row)), false);
  assert.equal(validate(rows.map((row, index) => index === 0 ? { ...row, unexpected: true } : row)), false);
  const paths = new Map([["beams", "🪵️beams"], ["slabs", "🧱️slabs"], ["windows", "🪟️windows"]]);
  assert.equal(new Set(rows.map(row => row.moduleId)).size, 3);
  for (const row of rows) {
    assert.equal(row.typology.id, row.moduleId);
    assert.equal(row.kindIds.length, row.availability.length);
    const root = join(base, paths.get(row.moduleId)!), source = readFileSync(join(root, "🦀️.rs"), "utf8"), tests = readFileSync(join(root, "🧪️tests/🔬️unit/🦀️.rs"), "utf8"), manifest = readFileSync(join(root, "📦️packages/🦀️rust/Cargo.toml"), "utf8");
    assert.equal(source.includes("semio_framework_os_kernel::json"), false, row.moduleId + " must call the canonical JSON owner");
    assert.equal(source.includes("semio_framework_pack_json::to_json_string"), true);
    assert.equal(tests.includes("semio_framework_os_kernel::json"), false);
    assert.equal(tests.includes("JsonMemberPolicy::Reject"), true);
    assert.equal(manifest.includes("semio-framework-pack-json ="), true);
    assert.equal(manifest.includes("semio-framework-os-kernel ="), false);
  }
  console.log("[DEBUG] Independent Ajv Sourcing catalog oracle: 3 typologies, " + rows.reduce((sum, row) => sum + row.kindIds.length, 0) + " kinds");
}

if (process.argv.length !== 3 || process.argv[2] !== "oracle") throw new Error("Usage: 📜️script.ts oracle");
oracle();
