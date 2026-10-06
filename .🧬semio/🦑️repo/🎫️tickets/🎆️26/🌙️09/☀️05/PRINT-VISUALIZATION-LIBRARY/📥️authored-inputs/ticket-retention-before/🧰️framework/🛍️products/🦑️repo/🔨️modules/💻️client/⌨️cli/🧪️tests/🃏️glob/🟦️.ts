import { readFileSync } from "node:fs";
import Ajv from "ajv";
import { minimatch } from "minimatch";

/** 🃏️ One neutral policy-glob expectation: the pattern, the path it is matched against and the verdict. */
interface GlobFixture {
  readonly cases: readonly Readonly<{ id: string; pattern: string; path: string; match: boolean }>[];
}

/** 🧭️ Verifies the neutral policy-glob expectations against an independent matcher. */
export function verifyFixtureGlobOracle(): void {
  const fixture = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🃏️glob/🔣️.json", import.meta.url), "utf8"));
  const schema = JSON.parse(readFileSync(new URL("../../🧬️schema/🃏️glob/🔣️.json", import.meta.url), "utf8"));
  if (!new Ajv({ strict: true }).validate<GlobFixture>(schema, fixture)) throw new Error("Invalid fixture-glob examples");
  for (const row of fixture.cases) if (minimatch(row.path, row.pattern) !== row.match) throw new Error(`Reference glob mismatch: ${row.id}`);
  console.log(`Fixture glob reference: ${fixture.cases.length} vectors verified`);
}
