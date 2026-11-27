import os, sys, tempfile
P = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🧾️schema-scope-catalog/🟦️.ts"
s = open(P, encoding="utf8").read()

def sub(old, new):
    global s
    assert s.count(old) == 1, (s.count(old), old[:80])
    s = s.replace(old, new)

sub("""  readonly placement: readonly SchemaScopeDiagnostic[];
}
""", """  readonly placement: readonly SchemaScopeDiagnostic[];
  /** 🧬️ Every `🧬️schema` module outside a test or fixture collection that holds a schema definition, by the one shared predicate {@link schemaDefinitionModulePaths}; each is either a catalog row or a `schema-module-id-missing` finding. */
  readonly definitionModules: readonly string[];
}
""")

sub("""interface SchemaWalkResult {
  readonly directories: readonly string[];
  readonly files: readonly string[];
}
""", """/** 🌲️ What one walk of the repository tree reached: repository-relative directories and files, byte-sorted per directory. */
export interface SchemaWalkResult {
  readonly directories: readonly string[];
  readonly files: readonly string[];
}

/** 🚧️ The only taxonomy slice the walk reads: the declared opaque subtrees. */
export interface SchemaWalkTaxonomy {
  readonly pathExclusions: Readonly<Record<string, { readonly path: string }>>;
}

/**
 * 🧬️ Whether one parsed JSON document DEFINES a schema, as opposed to being validated by one.
 *
 * Policy is schema-first: a document that names a json-schema.org `$schema` IS a schema definition, wherever
 * its module places it and whatever its `$id` says. The generator and every law that measures the tree ask this one
 * predicate, so no second opinion on what a definition is can exist.
 * @see https://json-schema.org/understanding-json-schema/reference/schema
 */
export function isSchemaDefinitionDocument(document: unknown): boolean {
  if (document === null || typeof document !== "object" || Array.isArray(document)) return false;
  const dialect = (document as { $schema?: unknown }).$schema;
  return typeof dialect === "string" && /^https?:\\/\\/json-schema\\.org\\//u.test(dialect);
}

/** 📖️ Parses a document only when its text can name a json-schema.org meta-schema, and returns it when it defines a schema; the substring test only prefilters, the parse decides. */
export function parseSchemaDefinitionText(text: string): unknown | undefined {
  if (!text.includes("json-schema.org")) return undefined;
  let parsed: unknown;
  try {
    parsed = JSON.parse(text);
  } catch {
    return undefined;
  }
  return isSchemaDefinitionDocument(parsed) ? parsed : undefined;
}

/**
 * 📁️ The `🧬️schema` module directories that carry a schema definition: the nearest enclosing module of every `.json` file that defines one.
 *
 * `readText` is the only IO and answers `undefined` for a file that vanished mid-walk; `excluded` drops module paths a caller never catalogues (test and fixture collections).
 */
export function schemaDefinitionModulePaths(files: readonly string[], moduleDirName: string, readText: (path: string) => string | undefined, excluded: (modulePath: string) => boolean = () => false): ReadonlySet<string> {
  const found = new Set<string>();
  const rejected = new Set<string>();
  for (const path of files) {
    if (!path.endsWith(".json")) continue;
    const directories = path.split("/").slice(0, -1);
    const at = directories.lastIndexOf(moduleDirName);
    if (at < 0) continue;
    const modulePath = directories.slice(0, at + 1).join("/");
    if (found.has(modulePath) || rejected.has(modulePath)) continue;
    if (excluded(modulePath)) {
      rejected.add(modulePath);
      continue;
    }
    const text = readText(path);
    if (text !== undefined && parseSchemaDefinitionText(text) !== undefined) found.add(modulePath);
  }
  return found;
}
""")

sub("""function walkRepositoryTree(repoRoot: string, taxonomy: Taxonomy): SchemaWalkResult {""",
"""/**
 * 🌲️ The ONE walk of the repository tree for schema work: the catalog generator and every law that compares the tree with the catalog read this walk, so their file sets cannot disagree.
 *
 * Dot directories, discovery skip directories, symbolic links, declared git submodules and the taxonomy's opaque path exclusions are never entered; `under` narrows the walk to one subtree.
 */
export function walkSchemaTree(repoRoot: string, taxonomy: SchemaWalkTaxonomy, under = ""): SchemaWalkResult {""")

sub("""  visit("");
  return { directories, files };""", """  visit(under.replace(/\\/+$/u, ""));
  return { directories, files };""")

sub("""const { directories, files } = walkRepositoryTree(root, taxonomy);""", """const { directories, files } = walkSchemaTree(root, taxonomy);""")

sub("""    if (documents.length > 0 && !rootScopeId) diagnostics.push(""", """    if ((documents.length > 0 || definitionModules.has(modulePath)) && !rootScopeId) diagnostics.push(""")

sub("""  const modules: SchemaScopeModule[] = [];
  const rootScopePaths = new Map<string, readonly string[] | null>();""", """  const definitionModules = schemaDefinitionModulePaths(sortedFiles, facetDir, (path) => {
    try {
      return readFileSync(join(root, path), "utf8");
    } catch {
      return undefined;
    }
  }, (modulePath) => schemaScopeCollectionPath(modulePath, taxonomy, matcher));
  const modules: SchemaScopeModule[] = [];
  const rootScopePaths = new Map<string, readonly string[] | null>();""")

sub("""placement };
}

/** 🖌️ Deterministic catalog bytes""", """placement, definitionModules: [...definitionModules].sort(byteSort) };
}

/** 🖌️ Deterministic catalog bytes""")

fd, tmp = tempfile.mkstemp(dir=os.path.dirname(P), prefix=".tmp-F107-")
with os.fdopen(fd, "w", encoding="utf8") as f: f.write(s)
os.replace(tmp, P)
print("ok")
