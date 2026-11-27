import os, tempfile
B = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📐️schema-contracts/"
def patch(rel, subs):
    P = B + rel
    s = open(P, encoding="utf8").read()
    for old, new in subs:
        assert s.count(old) == 1, (rel, s.count(old), old[:90])
        s = s.replace(old, new)
    fd, tmp = tempfile.mkstemp(dir=os.path.dirname(P), prefix=".tmp-F107-")
    with os.fdopen(fd, "w", encoding="utf8") as f: f.write(s)
    os.replace(tmp, P)

patch("🏗️scope-eligibility-and-tree/🟦️.ts", [
("""import { readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";
import { isExcludedTestPath, TAXONOMY_REL_PATH } from "../../🗂️taxonomy/🟦️.ts";
import { SKIP_DIR_NAMES } from "../../🧭️case-discovery/🟦️.ts";
import { matchesTaxonomyPathPattern, schemaJsonDialect, type SchemaScopeEligibility, schemaScopeOwnerLevels, submodulePaths } from "../🟦️.ts";
""", """import { readFileSync } from "node:fs";
import { TAXONOMY_REL_PATH, testTaxonomy } from "../../🗂️taxonomy/🟦️.ts";
import { isSchemaDefinitionDocument, parseSchemaDefinitionText, walkSchemaTree } from "../../../📚️library/🔍️discovery/🧾️schema-scope-catalog/🟦️.ts";
import { matchesTaxonomyPathPattern, schemaJsonDialect, type SchemaScopeEligibility, schemaScopeOwnerLevels } from "../🟦️.ts";
"""),
("""export function isJsonSchemaDefinition(document: unknown): boolean {
  if (document === null || typeof document !== "object" || Array.isArray(document)) return false;
  const dialect = (document as { $schema?: unknown }).$schema;
  return typeof dialect === "string" && /^https?:\\/\\/json-schema\\.org\\//u.test(dialect);
}""", """export function isJsonSchemaDefinition(document: unknown): boolean {
  return isSchemaDefinitionDocument(document);
}"""),
("""  if (!text.includes("json-schema.org")) return undefined;
  let parsed: unknown;
  try {
    parsed = JSON.parse(text);
  } catch {
    return undefined;
  }
  return isJsonSchemaDefinition(parsed) ? parsed : undefined;
}""", """  return parseSchemaDefinitionText(text);
}"""),
])

import re
P = B + "🏗️scope-eligibility-and-tree/🟦️.ts"
s = open(P, encoding="utf8").read()
m = re.search(r"export function schemaTreeFiles\(repoRoot: string, under = \"\"\): string\[\] \{.*?\n  return files\.sort\(\);\n\}", s, re.S)
assert m
s = s[:m.start()] + """export function schemaTreeFiles(repoRoot: string, under = ""): string[] {
  return [...walkSchemaTree(repoRoot, testTaxonomy(repoRoot), under).files].sort();
}""" + s[m.end():]
s = s.replace(" * 🗂️ Every file under a subtree, repository-relative, skipping the taxonomy's exclusions.", " * 🗂️ Every file under a subtree, repository-relative, skipping the taxonomy's exclusions: the generator's own walk, `walkSchemaTree`, so the tree this layer measures is the tree the catalog is generated from.")
fd, tmp = tempfile.mkstemp(dir=os.path.dirname(P), prefix=".tmp-F107-")
with os.fdopen(fd, "w", encoding="utf8") as f: f.write(s)
os.replace(tmp, P)

patch("🩺️placement-diagnostics/🟦️.ts", [
("""import { isDeclaredDialectDefinition, readJson, readSchemaDefinition, schemaScopeEligibility, schemaTreeFiles } from "../🏗️scope-eligibility-and-tree/🟦️.ts";
""", """import { isDeclaredDialectDefinition, readJson, readSchemaDefinition, schemaScopeEligibility, schemaTreeFiles } from "../🏗️scope-eligibility-and-tree/🟦️.ts";
import { schemaDefinitionModulePaths } from "../../../📚️library/🔍️discovery/🧾️schema-scope-catalog/🟦️.ts";
"""),
("""/** 📁️ Every `🧬️schema/` module directory the FILE WALK reaches that carries a schema definition. */
export function schemaModuleDirectoriesOnDisk(repoRoot: string, files: readonly string[] = schemaTreeFiles(repoRoot)): ReadonlySet<string> {
  const moduleDirName = schemaModuleDirName(repoRoot);
  const found = new Set<string>();
  for (const rel of files) {
    const segments = rel.split("/");
    if (!segments[segments.length - 1]!.endsWith(".json")) continue;
    const moduleDir = schemaModuleDirOf(segments.slice(0, -1), moduleDirName);
    if (moduleDir === null || found.has(moduleDir)) continue;
    if (readSchemaDefinition(join(repoRoot, rel)) !== undefined) found.add(moduleDir);
  }
  return found;
}""", """/** 📁️ Every `🧬️schema/` module directory the FILE WALK reaches that carries a schema definition: the generator's own predicate `schemaDefinitionModulePaths`, so the rule and the catalog generator cannot disagree on what a definition is. */
export function schemaModuleDirectoriesOnDisk(repoRoot: string, files: readonly string[] = schemaTreeFiles(repoRoot)): ReadonlySet<string> {
  return schemaDefinitionModulePaths(files, schemaModuleDirName(repoRoot), (rel) => {
    try {
      return readFileSync(join(repoRoot, rel), "utf8");
    } catch {
      return undefined;
    }
  }, (modulePath) => isFixtureOwnedPath(repoRoot, modulePath));
}"""),
])
print("ok test rule")
