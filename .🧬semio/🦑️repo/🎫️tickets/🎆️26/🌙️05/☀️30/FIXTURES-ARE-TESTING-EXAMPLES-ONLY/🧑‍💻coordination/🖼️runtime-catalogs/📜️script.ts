import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { createHash } from "node:crypto";
import { semanticPackageProjectionCatalog, semanticExactOwnedFileCatalog, type Taxonomy } from "../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";
const root = process.env.SEMIO_FIXTURE_REPO_ROOT!, library = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const taxonomy = JSON.parse(readFileSync(join(root, library, "🔣️taxonomy.json"), "utf8")) as Taxonomy;
const catalogs = [...Object.values(taxonomy.semanticPackageProjectionContracts), ...Object.values(taxonomy.semanticOwnedFileProjectionContracts)].filter(contract => "authorityCatalogPath" in contract);
assert.equal(catalogs.length, 2);
for (const contract of catalogs) {
  assert(contract.authorityCatalogPath.split("/").includes(taxonomy.exampleAssetsDirName));
  assert(!contract.authorityCatalogPath.split("/").includes(taxonomy.testFixturesDirName));
  const bytes = readFileSync(join(root, contract.authorityCatalogPath));
  const nodeDigest = createHash("sha256").update(bytes).digest("hex");
  const bunDigest = new Bun.CryptoHasher("sha256").update(bytes).digest("hex");
  assert.equal(nodeDigest, bunDigest);
  assert.equal(contract.authorityCatalogSha256, nodeDigest);
}
const packages = semanticPackageProjectionCatalog(root, taxonomy);
assert.equal(packages?.packages.length, 2);
console.log("[DEBUG] Production nested Cargo catalog loader accepted both asset-backed packages");
const owners = semanticExactOwnedFileCatalog(root, taxonomy);
assert.equal(owners?.cases.length, 40);
console.log("[DEBUG] Production exact-owner catalog loader accepted 40 asset-backed ownership records");

