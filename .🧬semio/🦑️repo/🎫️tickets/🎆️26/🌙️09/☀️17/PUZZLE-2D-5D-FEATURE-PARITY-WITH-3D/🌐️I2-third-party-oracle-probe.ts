/**
 * 🌐️ Integrator I2 — runs EVERY scenario of the 2d bun third-party oracle over the whole committed
 * fixture corpus, without nx and without the test host.
 *
 *     bun '.../🌐️I2-third-party-oracle-probe.ts'
 *
 * Slice 2F reported that `vectors()` discovered ZERO vectors from a stale layout constant, which made
 * this a silent-green gate. The probe prints the vector count each scenario saw, so a green that
 * checked nothing is impossible.
 */

import { join } from "node:path";

const ROOT = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any";
const FIX = join(ROOT, "🧫️fixtures/🧬️mutations");

const adapter = (await import(join(ROOT, "🧪️tests/🌐️third-party-puzzle-2d-1/🟦️.ts"))).default as {
  scenarios: Record<string, { oracle: (ctx: unknown) => { projection: Record<string, unknown> } }>;
};

const ctx = { fixture: () => join(FIX, "🔣️.json") };

let failures = 0;
for (const [scenario, definition] of Object.entries(adapter.scenarios)) {
  try {
    const payload = definition.oracle(ctx).projection as { scenario: string; checked: number; vectors: unknown[] };
    console.log(`  PASS ${payload.scenario.padEnd(22)} ${String(payload.checked).padStart(5)} checks over ${String(payload.vectors.length).padStart(4)} vectors`);
  } catch (error) {
    failures += 1;
    console.log(`  FAIL ${scenario.padEnd(22)} ${String(error).slice(0, 4000)}`);
  }
}
console.log(`\n${Object.keys(adapter.scenarios).length - failures}/${Object.keys(adapter.scenarios).length} scenarios agree`);
process.exit(failures ? 1 : 0);
