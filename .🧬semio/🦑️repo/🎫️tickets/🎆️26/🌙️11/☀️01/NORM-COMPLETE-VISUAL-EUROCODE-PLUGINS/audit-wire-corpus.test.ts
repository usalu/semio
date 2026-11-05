import { test, expect } from "bun:test";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { isDeepStrictEqual } from "node:util";
const { setups, encode } = await import(join(process.cwd(), "✏️s/🔌️plugins/📕️norm/🧪️tests/🧪️wire-twins/🟦️.ts"));
test("bounded wire corpus census", () => {
  const report = { total: 0, schema: [] as unknown[], readers: [] as unknown[], values: [] as unknown[], format: [] as string[], artifacts: {} as Record<string, unknown> };
  for (const setup of setups) {
    if (process.env.SEMIO_WIRE_CENSUS_FILTER && !setup.artifact.includes(process.env.SEMIO_WIRE_CENSUS_FILTER)) continue;
    const stats = { total: 0, schema: 0, readers: 0, values: 0, format: 0 };
    for (const wire of setup.corpus) {
      stats.total++; report.total++;
      const original = JSON.parse(wire.text);
      const oracle = setup.oracles[wire.role];
      const admitted = oracle(original) === true;
      if (admitted === wire.negative) { report.schema.push({ path: wire.path, negative: wire.negative, errors: (oracle as any).errors }); stats.schema++; }
      let parsed: unknown;
      try { parsed = setup.twins[wire.role](original); }
      catch (error) { if (admitted) { report.readers.push({ path: wire.path, message: String(error) }); stats.readers++; } continue; }
      if (!admitted) { report.readers.push({ path: wire.path, message: "reader admitted schema rejection" }); stats.readers++; continue; }
      if (!isDeepStrictEqual(original, parsed)) { report.values.push(wire.path); stats.values++; }
      if (encode(setup.schemas, parsed as any, setup.roots[wire.role], setup.roots[wire.role]) + "\n" !== wire.text) { report.format.push(wire.path); stats.format++; }
    }
    report.artifacts[setup.artifact] = stats;
  }
  const stem = process.env.SEMIO_WIRE_CENSUS_LABEL ?? "wire-corpus-census";
  mkdirSync(join(import.meta.dir, "🗑️generated"), { recursive: true });
  writeFileSync(join(import.meta.dir, `🗑️generated/${stem}.json`), JSON.stringify(report, null, 2) + "\n");
  const summary = { total: report.total, schema: report.schema.length, readers: report.readers.length, values: report.values.length, format: report.format.length, artifacts: report.artifacts, firstSchema: report.schema.slice(0, 5), firstReader: report.readers.slice(0, 5) };
  writeFileSync(join(import.meta.dir, stem + ".md"), "# 🧾️ Wire Corpus Census\n\n\x60\x60\x60json\n" + JSON.stringify(summary, null, 2) + "\n\x60\x60\x60\n");
  console.log(JSON.stringify(summary));
  expect(report.total).toBeGreaterThan(0);
  expect(report.schema.length, "source-schema corpus failures").toBe(0);
  expect(report.readers.length, "reader/schema disagreements").toBe(0);
  expect(report.values.length, "decoded fixture value changes").toBe(0);
}, 120000);
