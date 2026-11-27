import os, tempfile
D = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🧾️schema-scope-catalog/🧪️tests/"
def write(P, s):
    os.makedirs(os.path.dirname(P), exist_ok=True)
    fd, tmp = tempfile.mkstemp(dir=os.path.dirname(P), prefix=".tmp-F107-")
    with os.fdopen(fd, "w", encoding="utf8") as f: f.write(s)
    os.replace(tmp, P)

P = D + "🟦️.ts"
s = open(P, encoding="utf8").read()
assert "walkSchemaTree" not in s
s = s.replace('''import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { minimatch } from "minimatch";
import { schemaScopeOwnerLevel } from "../🟦️.ts";
import { loadCatalogTaxonomy } from "../../🔐️taxonomy-authority/🟦️.ts";
''', '''import { afterAll, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { minimatch } from "minimatch";
import { inventorySchemaScopes, schemaScopeOwnerLevel, walkSchemaTree } from "../🟦️.ts";
import { loadCatalogTaxonomy } from "../../🔐️taxonomy-authority/🟦️.ts";
import { TAXONOMY_REL_PATH } from "../../../../🧪️test/🗂️taxonomy/🟦️.ts";
import { repoRootFromHere } from "../../../../🧪️test/🧭️case-discovery/🟦️.ts";
import { clearSchemaContractCache, readSchemaCatalog } from "../../../../🧪️test/📐️schema-contracts/🟦️.ts";
import { schemaModuleDirectoriesOnDisk } from "../../../../🧪️test/📐️schema-contracts/🩺️placement-diagnostics/🟦️.ts";
import { schemaTreeFiles } from "../../../../🧪️test/📐️schema-contracts/🏗️scope-eligibility-and-tree/🟦️.ts";
''', 1)
assert "walkSchemaTree" in s
s += '''
const ID_BASE = loadCatalogTaxonomy().schemaExportResolution.idBase;
const MODULES = "🧰️framework/🛍️products/🦑️repo/🔨️modules";
const DIALECT = "http://json-schema.org/draft-07/schema#";
const scratchRoots: string[] = [];
afterAll(() => {
  for (const root of scratchRoots) rmSync(root, { recursive: true, force: true });
  clearSchemaContractCache();
});

function put(root: string, rel: string, body: unknown): void {
  mkdirSync(join(root, dirname(rel)), { recursive: true });
  writeFileSync(join(root, rel), typeof body === "string" ? body : `${JSON.stringify(body, null, 2)}\\n`);
}

function definitionTree(): string {
  const root = mkdtempSync(join(tmpdir(), "schema-scope-catalog-law-"));
  scratchRoots.push(root);
  put(root, TAXONOMY_REL_PATH, readFileSync(join(repoRootFromHere(), TAXONOMY_REL_PATH), "utf8"));
  put(root, `${MODULES}/🧩️alpha/🧬️schema/🔣️.json`, { $schema: DIALECT, $id: `${ID_BASE}alpha/schema.json`, title: "Alpha", type: "object", properties: { name: { type: "string" } } });
  put(root, `${MODULES}/🧩️beta/🧬️schema/🔣️.json`, { $schema: DIALECT, title: "Beta", type: "object" });
  put(root, `${MODULES}/🧩️gamma/🧬️schema/🔣️.json`, { $schema: DIALECT, $id: "urn:semio:gamma:v1", type: "object" });
  put(root, `${MODULES}/🧩️delta/🧬️schema/🔣️gate/🔣️.json`, { $schema: DIALECT, $id: "semio.delta.gate/v1", type: "object" });
  put(root, `${MODULES}/🧩️epsilon/🧬️schema/🔣️.json`, { type: "object", properties: { name: { type: "string" } } });
  put(root, `${MODULES}/🧩️alpha/🧪️tests/🧫️fixtures/specimen/🧬️schema/🔣️.json`, { $schema: DIALECT, $id: `${ID_BASE}alpha/specimen.json`, type: "object" });
  put(root, `${MODULES}/🧩️alpha/node_modules/pkg/🧬️schema/🔣️.json`, { $schema: DIALECT, type: "object" });
  put(root, `${MODULES}/🧩️alpha/.state/🧬️schema/🔣️.json`, { $schema: DIALECT, type: "object" });
  clearSchemaContractCache();
  return root;
}

test("law: the generator and the schema-contract rule name one set of schema definition modules", () => {
  const root = definitionTree();
  const inventory = inventorySchemaScopes(root);
  const rule = [...schemaModuleDirectoriesOnDisk(root)].sort();
  const expected = ["alpha", "beta", "delta", "gamma"].map((name) => `${MODULES}/🧩️${name}/🧬️schema`);
  expect(rule).toEqual(expected);
  expect([...inventory.definitionModules]).toEqual(expected);
});

test("law: every definition module is a catalog row or a finding of the generator, and every row is a definition module", () => {
  const root = definitionTree();
  const inventory = inventorySchemaScopes(root);
  const rows = new Map(Object.entries(inventory.catalog.scopes).map(([id, scope]) => [scope.path, id]));
  expect([...rows.values()]).toEqual(["alpha"]);
  for (const path of rows.keys()) expect(inventory.definitionModules).toContain(path);
  for (const path of inventory.definitionModules.filter((candidate) => !rows.has(candidate))) expect(inventory.diagnostics.some((finding) => finding.code === "schema-module-id-missing" && (finding.path === path || finding.path.startsWith(`${path}/`)))).toBe(true);
});

test("law: the schema-contract rule's tree walk is the generator's tree walk", () => {
  const root = definitionTree();
  const walked = [...walkSchemaTree(root, loadCatalogTaxonomy()).files].sort();
  expect(schemaTreeFiles(root)).toEqual(walked);
  expect(walked.some((path) => path.includes("node_modules") || path.includes("/.state/"))).toBe(false);
  expect(schemaTreeFiles(root, `${MODULES}/🧩️alpha`)).toEqual(walked.filter((path) => path.startsWith(`${MODULES}/🧩️alpha/`)));
});

test("the repository test platform's four schema modules are catalogued, none is a stale gap", () => {
  const { catalog } = readSchemaCatalog(repoRootFromHere());
  const paths = new Set(Object.values(catalog!.scopes).map((scope) => scope.path));
  for (const module of ["🎯️acceptance", "🏭️inventory", "🏭️inventory/🔌️providers", ""]) expect(paths.has(`${MODULES}/🧪️test/${module ? `${module}/` : ""}🧬️schema`)).toBe(true);
});
'''
write(P, s)

write(D + "🧮️catalog-freshness/🟦️.ts", '''import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { inventorySchemaScopes, renderSchemaCatalog, renderSchemaCatalogDocument } from "../../🟦️.ts";
import { loadCatalogTaxonomy } from "../../../🔐️taxonomy-authority/🟦️.ts";
import { repoRootFromHere } from "../../../../../🧪️test/🧭️case-discovery/🟦️.ts";

/**
 * 🧮️ The derived schema catalog and its document are byte-identical to what the generator renders from the schema modules on disk.
 *
 * This is `schema generate --check` as a suite member: a schema edited without regenerating, a module added or removed, or a generator change fails here instead of drifting silently (2844 stale rows once did).
 * The failure names the remedy: `repo ./📜️script.ts schema generate` and `schema docs`.
 */
test("the schema catalog and its document are current", () => {
  const root = repoRootFromHere();
  const resolution = loadCatalogTaxonomy().schemaExportResolution;
  const inventory = inventorySchemaScopes(root);
  const stale = [[resolution.catalogPath, renderSchemaCatalog(inventory.catalog)], [resolution.catalogDocumentPath, renderSchemaCatalogDocument(inventory.catalog)]].filter(([rel, rendered]) => readFileSync(join(root, rel!), "utf8") !== rendered).map(([rel]) => rel);
  expect(stale).toEqual([]);
}, 600_000);
''')
print("ok")
