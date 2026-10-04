/** ⚖️ Work package A5: holds the TypeScript subject of case 🚧️clearance-proof to the committed vectors (the answers of the Python oracle), number by number.
 *
 * `bun .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/clearance_vectors_check.ts` from the repository root. The harness's
 * `subject` command records the projections but compares them only under `parity`, which needs the Rust adapter of a
 * later phase; this check stands in for it: every scenario of the subject adapter is run on the committed fixture and
 * compared with the committed answers under the rule of `pets-float-v1` (1e-9 absolute, everything else exact). It
 * also counts how many numbers agree to the bit. Exit code 1 on any difference.
 */
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const here = dirname(fileURLToPath(import.meta.url));
const pets = resolve(here, "../../../../../../../🧰️framework/🛍️products/🐾️pets");
const adapter = (await import(resolve(pets, "🧪️tests/🚧️clearance-proof/🟦️.ts"))).default;
const bytes = readFileSync(resolve(pets, "🧫️fixtures/🚧️clearance-proof/🔣️.json"));
const document = JSON.parse(bytes.toString("utf8"));
const context = { fixtureBytes: () => new Uint8Array(bytes) };

let numbers = 0;
let exact = 0;
const differences: string[] = [];

/** 🔍️ Compares a produced answer with the committed one under `pets-float-v1`. */
function compare(produced: unknown, expected: unknown, path: string): void {
  if (typeof produced === "number" && typeof expected === "number") {
    numbers++;
    if (Object.is(produced, expected) || produced === expected) exact++;
    else if (!(Math.abs(produced - expected) <= 1e-9)) differences.push(`${path}: ${produced} ≠ ${expected}`);
    return;
  }
  if (Array.isArray(produced) && Array.isArray(expected)) {
    if (produced.length !== expected.length) differences.push(`${path}: ${produced.length} entries ≠ ${expected.length}`);
    else produced.forEach((entry, index) => compare(entry, expected[index], `${path}[${index}]`));
    return;
  }
  if (produced !== null && expected !== null && typeof produced === "object" && typeof expected === "object") {
    const keys = Object.keys(produced as object).sort();
    const wanted = Object.keys(expected as object).sort();
    if (JSON.stringify(keys) !== JSON.stringify(wanted)) differences.push(`${path}: keys ${keys} ≠ ${wanted}`);
    else for (const key of keys) compare((produced as Record<string, unknown>)[key], (expected as Record<string, unknown>)[key], `${path}.${key}`);
    return;
  }
  if (produced !== expected) differences.push(`${path}: ${JSON.stringify(produced)} ≠ ${JSON.stringify(expected)}`);
}

const scenarios = Object.keys(adapter.scenarios);
for (const scenario of scenarios) {
  const projection = JSON.parse(JSON.stringify((await adapter.scenarios[scenario].subject(context)).projection));
  const expected = scenario === "constants" ? document.constants : Object.fromEntries(document[scenario].map((vector: { id: string; expected: unknown }) => [vector.id, vector.expected]));
  compare(projection, expected, scenario);
}
process.stdout.write(`scenarios=${scenarios.length} vectors=${scenarios.filter((name) => name !== "constants").reduce((sum, name) => sum + document[name].length, 0)} numbers=${numbers} bit-exact=${exact} differences=${differences.length}\n`);
for (const difference of differences.slice(0, 40)) process.stdout.write(`${difference}\n`);
process.exitCode = differences.length === 0 ? 0 : 1;
