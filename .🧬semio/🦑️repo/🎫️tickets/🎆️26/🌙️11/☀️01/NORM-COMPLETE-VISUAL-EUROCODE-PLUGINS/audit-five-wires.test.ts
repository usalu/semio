import { expect, test } from "bun:test";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
const { setups } = await import(join(process.cwd(), "✏️s/🔌️plugins/📕️norm/🧪️tests/🧪️wire-twins/🟦️.ts"));
test("five artifact semantic corpus census", () => {
  const failures = [];
  const stats = {};
  for (const setup of setups.filter(row => /en199[24689]/u.test(row.artifact))) {
    let rejected = 0;
    for (const wire of setup.corpus) {
      const value = JSON.parse(wire.text);
      const oracle = setup.oracles[wire.role];
      const admitted = oracle(value) === true;
      if (admitted === wire.negative) { rejected++; failures.push({ artifact: setup.artifact, path: wire.path, role: wire.role, errors: oracle.errors }); }
      if (admitted) {
        try { setup.twins[wire.role](value); } catch (error) { failures.push({ artifact: setup.artifact, path: wire.path, reader: String(error) }); }
      }
    }
    stats[setup.artifact] = { total: setup.corpus.length, rejected };
  }
  mkdirSync(join(import.meta.dir, "🗑️generated"), { recursive: true });
  writeFileSync(join(import.meta.dir, "🗑️generated/five-wire-census.json"), JSON.stringify({ stats, failures }, null, 2) + "\n");
  console.log(JSON.stringify({ stats, failures: failures.length }));
  expect(failures).toHaveLength(0);
}, 120000);
