/** 🗺️ Ticket tool of work package D1 (round 2): prints the shape of the committed vectors of the five gear cases — every group, how many vectors it has and the members of its first vector with a short type summary — so the Rust unit suites and adapters read exactly what is there.
 *
 * Run from the repository root:
 *   bun .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/d1_fixture_shapes.ts > <ticket>/🗑️generated/d1/fixture-shapes.txt
 */
import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";

const FIXTURES = join(resolve(import.meta.dir, "../../../../../../.."), "🧰️framework/🛍️products/🐾️pets/🧫️fixtures");
const CASES = ["🪢️swing-dynamics", "🪂️parachute-descent", "🧗️wall-climbing", "🪜️ladder-geometry", "🎣️grapple-reach"];

/** 🔎️ A short summary of a JSON value: its kind, and for objects and arrays what they hold, `depth` levels deep. */
function summary(value: unknown, depth: number): string {
  if (value === null) return "null";
  if (Array.isArray(value)) return value.length === 0 ? "[]" : `[${value.length}× ${depth > 0 ? summary(value[0], depth - 1) : "…"}]`;
  if (typeof value === "object") return depth > 0 ? `{${Object.entries(value as Record<string, unknown>).map(([key, member]) => `${key}: ${summary(member, depth - 1)}`).join(", ")}}` : "{…}";
  if (typeof value === "number") return Number.isInteger(value) ? `int(${value})` : "num";
  return typeof value === "string" ? JSON.stringify(value.length > 24 ? `${value.slice(0, 24)}…` : value) : typeof value;
}

for (const name of CASES) {
  const document = JSON.parse(readFileSync(join(FIXTURES, name, "🔣️.json"), "utf8")) as Record<string, unknown>;
  process.stdout.write(`== ${name}\n`);
  for (const [group, value] of Object.entries(document)) process.stdout.write(Array.isArray(value) ? `${group}: ${value.length} vectors; first ${summary(value[0], 4)}\n` : `${group}: ${summary(value, 2)}\n`);
}
