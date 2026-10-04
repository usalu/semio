/** 🔎️ Tool of work package A8: runs the TypeScript subject adapters of the cases ✨️particle-motion and 🪄️mischief-choice over the committed vectors and lists where a projection leaves the committed `expected` (numbers within 1e-9, everything else exactly). `bun TK/compare_effects_vectors.ts` from the repository root. */
import { readFileSync } from "node:fs";
import { resolve } from "node:path";

const pets = resolve(import.meta.dir, "../../../../../../../🧰️framework/🛍️products/🐾️pets");
const TOLERANCE = 1e-9;

function differences(path: string, produced: unknown, expected: unknown, found: string[]): void {
  if (typeof produced === "number" && typeof expected === "number") {
    if (!(Math.abs(produced - expected) <= TOLERANCE)) found.push(`${path}: ${produced} ≠ ${expected}`);
  } else if (Array.isArray(produced) && Array.isArray(expected)) {
    if (produced.length !== expected.length) found.push(`${path}: ${produced.length} entries ≠ ${expected.length}`);
    for (let index = 0; index < Math.min(produced.length, expected.length); index++) differences(`${path}[${index}]`, produced[index], expected[index], found);
  } else if (produced !== null && expected !== null && typeof produced === "object" && typeof expected === "object") {
    const keys = new Set([...Object.keys(produced), ...Object.keys(expected)]);
    for (const key of keys) differences(`${path}.${key}`, (produced as Record<string, unknown>)[key], (expected as Record<string, unknown>)[key], found);
  } else if (produced !== expected) found.push(`${path}: ${JSON.stringify(produced)} ≠ ${JSON.stringify(expected)}`);
}

let total = 0;
for (const name of ["✨️particle-motion", "🪄️mischief-choice"]) {
  const bytes = readFileSync(resolve(pets, "🧫️fixtures", name, "🔣️.json"));
  const committed = JSON.parse(new TextDecoder().decode(bytes)) as Record<string, { id: string; expected: unknown }[]>;
  const adapter = (await import(resolve(pets, "🧪️tests", name, "🟦️.ts"))).default as { scenarios: Record<string, { subject: (ctx: unknown) => { projection: Record<string, unknown> } }> };
  for (const [scenario, handlers] of Object.entries(adapter.scenarios)) {
    const projection = JSON.parse(JSON.stringify(handlers.subject({ fixtureBytes: () => bytes }).projection)) as Record<string, unknown>;
    const found: string[] = [];
    for (const vector of committed[scenario]!) differences(`${scenario}/${vector.id}`, projection[vector.id], vector.expected, found);
    total += found.length;
    console.log(`${name} ${scenario}: ${committed[scenario]!.length} vectors, ${found.length} differences`);
    for (const line of found.slice(0, 12)) console.log(`  ${line}`);
  }
}
process.exitCode = total === 0 ? 0 : 1;
