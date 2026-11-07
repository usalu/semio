import { strict as assert } from "node:assert";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { msiDestination } from "./msi-paths.ts";

const fixture = JSON.parse(readFileSync(join(import.meta.dir, "msi-path-cases.json"), "utf8"));
for (const row of fixture.cases) {
  const run = () => msiDestination(row.directories, row.directory, row.file);
  if (row.error) assert.throws(run, undefined, row.name);
  else assert.equal(run(), row.expected, row.name);
}
console.log(`${fixture.cases.length} MSI destination cases passed`);
