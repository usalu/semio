import { readFileSync } from "node:fs";

const path = "🧰️framework/🔨️modules/🌱️value/📝️text/🚶️utf8/🧪️tests/🧫️fixtures/🔤️bytes.json";
const fixture = JSON.parse(readFileSync(path, "utf8")) as { cases: { bytes: number[]; valid: boolean }[] };
const decoder = new TextDecoder("utf-8", { fatal: true });
for (const row of fixture.cases) {
  let valid = true;
  try { decoder.decode(Uint8Array.from(row.bytes)); } catch { valid = false; }
  if (valid !== row.valid) throw new Error(`UTF-8 fixture disagrees with TextDecoder: ${JSON.stringify(row)}`);
}
console.log(`UTF-8 fixture: ${fixture.cases.length} independent decoder comparisons passed`);
