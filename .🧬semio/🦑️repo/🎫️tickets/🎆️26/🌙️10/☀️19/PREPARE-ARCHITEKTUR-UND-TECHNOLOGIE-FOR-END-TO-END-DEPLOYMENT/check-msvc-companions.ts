import { strict as assert } from "node:assert";
import { readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";
import { pathToFileURL } from "node:url";
import { createHash } from "node:crypto";
import { Database } from "bun:sqlite";

const root = process.cwd();
const authorizer = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers";
const schema = JSON.parse(readFileSync(join(root, authorizer, "🧬️schema/🔣️.json"), "utf8"));
const fixture = JSON.parse(readFileSync(join(root, authorizer, "🧪️tests/🧫️fixtures/🔣️.json"), "utf8"));
const row = fixture.runtimeCompanions.cases.find((row: { path: string }) => row.path.endsWith("/acquire.go"));
const module = row.path.slice(0, -"/acquire.go".length);
const expected = readdirSync(join(root, module)).filter(name => name === "go.mod" || name.endsWith(".go") && !name.endsWith("_test.go")).map(name => module + "/" + name).sort();
const actual = schema.$defs.RuntimeDataCompanions.const.filter((entry: { owner: string }) => entry.owner === row.owner).flatMap((entry: { paths: string[] }) => entry.paths).sort();
assert.deepEqual(actual, expected, "Every production helper input must enter the exact owner's Source closure");
const database = new Database(":memory:");
try {
  database.run("CREATE TABLE declared(owner TEXT,path TEXT,PRIMARY KEY(owner,path))");
  for (const entry of schema.$defs.RuntimeDataCompanions.const) for (const path of entry.paths) database.run("INSERT INTO declared VALUES (?,?)", [entry.owner, path]);
  assert.deepEqual(database.query("SELECT path FROM declared WHERE owner=? ORDER BY path").all(row.owner).map((entry: any) => entry.path).sort(), expected);
  assert.equal((database.query("SELECT count(DISTINCT path) AS n FROM declared").get() as { n: number }).n, fixture.runtimeCompanions.uniquePaths);
  assert.equal(database.query("SELECT path FROM declared WHERE path=?").get(row.excluded), null);
} finally { database.close(); }
console.log(JSON.stringify({ productionCompanions: actual.length, fixturePaths: fixture.runtimeCompanions.uniquePaths, independentSQLite: true }));
if (process.argv.includes("--capture")) {
  const { captureNxAuthorizerSourcesV1, nxAuthorizerCapturedBytesV1 } = await import(pathToFileURL(join(root, authorizer, "🟦️.ts")).href);
  const captured = await captureNxAuthorizerSourcesV1(root, row.owner, AbortSignal.timeout(600000));
  const images = nxAuthorizerCapturedBytesV1(captured);
  for (const path of expected) {
    const bytes = images.get(path);
    assert.ok(bytes, "Actual captured bytes required: " + path);
    assert.equal(createHash("sha256").update(bytes).digest("hex"), createHash("sha256").update(readFileSync(join(root, path))).digest("hex"));
  }
  assert.equal(images.has(row.excluded), false);
  console.log(JSON.stringify({ actualCapturedCompanions: expected.length, closureFiles: captured.manifest.sources.length, testsExcluded: true }));
}
