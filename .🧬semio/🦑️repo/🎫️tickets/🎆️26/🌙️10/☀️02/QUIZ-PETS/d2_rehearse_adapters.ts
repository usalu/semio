/** 🎭️ Ticket tool of work package D2: holds the projections of the Rust adapters of `🚧️clearance-proof` and `👆️gesture-recognition` to the TypeScript adapters' number for number, without the tolerance of the harness profile (`pets-float-v1`, 1e-9).
 *
 * It reads what the real Rust subject projected in the last parity run of each case
 * (`⚡️cache/tests/results/…-<case>-subject-rust/<scenario>.subject.projection.json`) and compares every scenario's
 * projection with what the TypeScript adapter answers right now for the same fixture. Both sides pass through JSON
 * text, like in the harness; numbers must be the same double (`Object.is`, so a −0 on one side and a 0 on the other
 * would be named).
 *
 * Run from the repository root, after the parity runs of both cases:
 *   bun .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/d2_rehearse_adapters.ts
 *
 * @see ./rehearse_motion_adapters.ts — the pattern (work package L)
 * @see ./📓️report2-d2.md — the recorded result
 */
import { existsSync, mkdirSync, readFileSync, readdirSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";

type Scenario = { readonly subject?: (ctx: { fixtureBytes(uri: string): Uint8Array }) => { projection: unknown } | Promise<{ projection: unknown }> };
type Adapter = { readonly scenarios: Readonly<Record<string, Scenario>> };

const ROOT = resolve(import.meta.dir, "../../../../../../..");
const OWNER = "🧰️framework/🛍️products/🐾️pets";
const RESULTS = join(ROOT, ".🧬semio/🦑️repo/⚡️cache/tests/results");
const CASES = ["🚧️clearance-proof", "👆️gesture-recognition"] as const;

/** 🧮️ Counts the numbers of a projection and collects where two projections differ; numbers must be the same double. */
function differences(path: string, left: unknown, right: unknown, found: string[], tally: { numbers: number }): void {
  if (typeof left === "number" && typeof right === "number") {
    tally.numbers += 1;
    if (!Object.is(left, right)) found.push(`${path}: typescript ${left} rust ${right}`);
  } else if (Array.isArray(left) && Array.isArray(right)) {
    if (left.length !== right.length) found.push(`${path}: lengths ${left.length} and ${right.length}`);
    for (let index = 0; index < Math.min(left.length, right.length); index++) differences(`${path}[${index}]`, left[index], right[index], found, tally);
  } else if (left !== null && right !== null && typeof left === "object" && typeof right === "object" && !Array.isArray(left) && !Array.isArray(right)) {
    const keys = [...new Set([...Object.keys(left), ...Object.keys(right)])].sort();
    for (const key of keys) {
      if (!(key in left) || !(key in right)) found.push(`${path}.${key}: only on one side`);
      else differences(`${path}.${key}`, (left as Record<string, unknown>)[key], (right as Record<string, unknown>)[key], found, tally);
    }
  } else if (left !== right) found.push(`${path}: typescript ${JSON.stringify(left)} rust ${JSON.stringify(right)}`);
}

const lines: string[] = [];
let failed = false;
let numbers = 0;
let scenarios = 0;
for (const name of CASES) {
  const entry = readdirSync(RESULTS).find((candidate) => candidate.endsWith(`-${name}-subject-rust`));
  if (entry === undefined) throw new Error(`the harness cache holds no Rust results of ${name}: run its parity first`);
  const projections = join(RESULTS, entry);
  const adapter = ((await import(join(ROOT, OWNER, "🧪️tests", name, "🟦️.ts"))) as { default: Adapter }).default;
  const ctx = {
    fixtureBytes(uri: string): Uint8Array {
      if (!uri.startsWith("shared://")) throw new Error(`unexpected fixture ${uri}`);
      return readFileSync(join(ROOT, OWNER, "🧫️fixtures", uri.slice("shared://".length)));
    },
  };
  for (const [id, scenario] of Object.entries(adapter.scenarios)) {
    if (scenario.subject === undefined) continue;
    const expected = JSON.parse(JSON.stringify((await scenario.subject(ctx)).projection)) as unknown;
    const file = join(projections, `${id}.subject.projection.json`);
    const found: string[] = [];
    const tally = { numbers: 0 };
    if (!existsSync(file)) found.push("the Rust adapter projected nothing");
    else differences("$", expected, JSON.parse(readFileSync(file, "utf8")), found, tally);
    if (found.length > 0) failed = true;
    numbers += tally.numbers;
    scenarios += 1;
    lines.push(`${name}/${id}: ${tally.numbers} numbers, ${found.length} differences${found.length > 0 ? `\n  ${found.slice(0, 8).join("\n  ")}` : ""}`);
  }
}
lines.push(`harness: ${CASES.length} cases, ${scenarios} scenarios, ${numbers} numbers, ${failed ? "DIFFERENCES" : "0 differences"}`);
const out = join(import.meta.dir, "🗑️generated", "d2");
mkdirSync(out, { recursive: true });
writeFileSync(join(out, "rehearsal-harness.txt"), `${lines.join("\n")}\n`);
console.log(lines.map((line) => `[DEBUG] ${line}`).join("\n"));
if (failed) process.exit(1);
