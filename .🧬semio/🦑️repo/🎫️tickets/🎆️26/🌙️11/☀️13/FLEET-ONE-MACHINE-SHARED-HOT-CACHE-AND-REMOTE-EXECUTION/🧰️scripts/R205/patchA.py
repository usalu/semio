S="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R205/"
def sub(t,o,n,count=1):
    assert t.count(o)==count,(t.count(o),o[:90]);return t.replace(o,n)
t=open(S+"base/schemaindex.ts",encoding="utf8").read()
t=sub(t,'/** 🧫️ An example collection keeps','''/** 🙈️ The bare directory names the workspace ignore list (`.gitignore` at the workspace root) names, such as `temp`, `local` or `generated`: such a directory is machine-local, so a schema inside it never enters the index every machine must produce byte for byte. */
export function schemaIndexIgnoredNamesV1(repoRoot: string): ReadonlySet<string> {
  try {
    return new Set(readFileSync(join(repoRoot, ".gitignore"), "utf8").split(/\\r?\\n/u).map((line) => line.trim().normalize("NFC")).filter((line) => line.length > 0 && !/^[#!]|[*?/[\\\\]/u.test(line)));
  } catch {
    return new Set();
  }
}

/** 🧫️ An example collection keeps''')
t=sub(t,'export function schemaDocumentIndex(root: string): Map<string, string> {','export function schemaDocumentIndex(root: string, ignored: ReadonlySet<string> = new Set()): Map<string, string> {')
t=sub(t,'if (name.startsWith(".") || SCHEMA_INDEX_EXCLUDED.has(name) || schemaIndexExampleCollection(next)) continue;\n        walk(path, next, schema || name === "🧬️schema");','if (name.startsWith(".") || SCHEMA_INDEX_EXCLUDED.has(name) || ignored.has(name) || schemaIndexExampleCollection(next)) continue;\n        walk(path, next, schema || name === "🧬️schema");')
t=sub(t,'function schemaIndexRoots(repoRoot: string): string[] {','function schemaIndexRoots(repoRoot: string, ignored: ReadonlySet<string>): string[] {')
t=sub(t,'''      if (name.startsWith(".") || SCHEMA_INDEX_EXCLUDED.has(name) || schemaIndexExampleCollection(next)) continue;
      const path = join(directory, entry.name);''','''      if (name.startsWith(".") || SCHEMA_INDEX_EXCLUDED.has(name) || ignored.has(name) || schemaIndexExampleCollection(next)) continue;
      const path = join(directory, entry.name);''')
t=sub(t,'if (childName.startsWith(".") || SCHEMA_INDEX_EXCLUDED.has(childName) || schemaIndexExampleCollection(childSegments)) continue;','if (childName.startsWith(".") || SCHEMA_INDEX_EXCLUDED.has(childName) || ignored.has(childName) || schemaIndexExampleCollection(childSegments)) continue;')
t=sub(t,'''  const roots: Record<string, Record<string, string>> = {};
  for (const root of schemaIndexRoots(repoRoot)) {
    const documents: Record<string, string> = {};
    for (const [id, path] of [...schemaDocumentIndex(join(repoRoot, root))]''','''  const roots: Record<string, Record<string, string>> = {};
  const ignored = schemaIndexIgnoredNamesV1(repoRoot);
  for (const root of schemaIndexRoots(repoRoot, ignored)) {
    const documents: Record<string, string> = {};
    for (const [id, path] of [...schemaDocumentIndex(join(repoRoot, root), ignored)]''')
open(S+"new/schemaindex.ts","w",encoding="utf8").write(t)
c=open(S+"base/commands.ts",encoding="utf8").read()
c=sub(c,'/** 🔁️ Writes the projection only when its bytes change, so an unrelated taxonomy edit leaves every deriving crate fresh. */\nexport class GenerateScript','''/** 🔁️ Writes a generated file only when its bytes change, so an unchanged projection keeps its inode and modification time and the replica has nothing to copy; answers what it did. */
export function writeProjectionIfChangedV1(path: string, text: string): "written" | "unchanged" {
  let committed: string | undefined;
  try { committed = readFileSync(path, "utf8"); } catch { committed = undefined; }
  if (committed === text) return "unchanged";
  writeFileSync(path, text);
  return "written";
}

/** 🔁️ Writes the projection only when its bytes change, so an unrelated taxonomy edit leaves every deriving crate fresh. */
export class GenerateScript''')
c=sub(c,'''    const projected = mutationAuthorityProjection(this.repoRoot);
    let committed: string | undefined;
    try { committed = readFileSync(MUTATION_AUTHORITY_PATH, "utf8"); } catch { committed = undefined; }
    if (committed === projected) return console.log("mutation-authority: unchanged");
    writeFileSync(MUTATION_AUTHORITY_PATH, projected);
    console.log("mutation-authority: written");''','''    console.log(`mutation-authority: ${writeProjectionIfChangedV1(MUTATION_AUTHORITY_PATH, mutationAuthorityProjection(this.repoRoot))}`);''')
c=sub(c,'''    const projected = mutationSchemaDocumentIndexProjection(this.repoRoot);
    let committed: string | undefined;
    try { committed = readFileSync(SCHEMA_INDEX_PATH, "utf8"); } catch { committed = undefined; }
    if (committed === projected) return console.log("schema-document-index: unchanged");
    writeFileSync(SCHEMA_INDEX_PATH, projected);
    console.log("schema-document-index: written");''','''    console.log(`schema-document-index: ${writeProjectionIfChangedV1(SCHEMA_INDEX_PATH, mutationSchemaDocumentIndexProjection(this.repoRoot))}`);''')
open(S+"new/commands.ts","w",encoding="utf8").write(c)
p=open(S+"base/derive-project.json",encoding="utf8").read()
tax="{workspaceRoot}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"
sch="{workspaceRoot}/🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🧬️schema/🔣️.json"
inputs='''      "inputs": [
        "default",
        "{workspaceRoot}/📋️project.json",
        "%s",
        "%s"
      ],
'''%(tax,sch)
p=sub(p,'''    "generate": {
      "executor": "repo:script",
      "cache": true,
''','''    "generate": {
      "executor": "repo:script",
      "cache": true,
'''+inputs)
p=sub(p,'''    "check-generated": {
      "executor": "repo:script",
      "cache": true,
''','''    "check-generated": {
      "executor": "repo:script",
      "cache": true,
'''+inputs)
p=sub(p,'''    "preview-generated": {
      "executor": "repo:script",
      "inputs": [
        "default",
        "^default",
        "reads",
        "cargoToolchain"
      ],''','''    "preview-generated": {
      "executor": "repo:script",
      "inputs": [
        "default",
        "^default",
        "reads",
        "cargoToolchain",
        "{workspaceRoot}/📋️project.json",
        "%s"
      ],'''%tax)
open(S+"new/derive-project.json","w",encoding="utf8").write(p)
