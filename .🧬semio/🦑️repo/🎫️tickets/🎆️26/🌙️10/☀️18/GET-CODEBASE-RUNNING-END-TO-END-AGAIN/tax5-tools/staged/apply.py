# 🔧 Applies the tax5 library patches with anchored, assert-guarded replacements. usage: python3 apply.py [--root <library dir>] [--check] [--only <patch id>...]
# Default root is the live library; --check verifies every anchor resolves exactly once (or the patch is already applied) and writes nothing.
# Composes with C/tax4-tools/staged/apply.py: the two touch disjoint anchors, so either order works.
import os,re,shutil,sys
HERE=os.path.dirname(os.path.abspath(__file__))
LIVE="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
args=sys.argv[1:]
root=args[args.index("--root")+1] if "--root" in args else LIVE
check="--check" in args
only=set(args[args.index("--only")+1:]) if "--only" in args else None
def edit(rel,old,new,applied_marker):
    path=os.path.join(root,rel)
    text=open(path,encoding="utf8").read()
    if applied_marker in text: return "already"
    assert text.count(old)==1,(rel,old[:80],text.count(old))
    if not check: open(path,"w",encoding="utf8").write(text.replace(old,new))
    return "patched"
def add(rel_from,rel_to):
    src=os.path.join(HERE,"new",rel_from); dst=os.path.join(root,rel_to)
    if os.path.isfile(dst) and open(dst,"rb").read()==open(src,"rb").read(): return "already"
    if not check:
        os.makedirs(os.path.dirname(dst),exist_ok=True); shutil.copyfile(src,dst)
    return "copied"
def block(rel,key):
    text=open(os.path.join(root,rel),encoding="utf8").read()
    match=re.search(r'^    "'+re.escape(key)+r'": \{\n(?:.*\n)*?    \},\n',text,re.M)
    assert match,(rel,key)
    return match.group(0)
PATCHES={}
def patch(pid):
    def register(fn): PATCHES[pid]=fn; return fn
    return register
TAXONOMY="🔣️taxonomy.json"
CONTRACTS="🔍️discovery/📜️taxonomy-contracts/🟦️.ts"
LOADER="🧹️normalization/🔣️taxonomy/🪞️projections-and-consumers/🟦️.ts"
AUDIT="🔍️discovery/🗞️audit-reference-consumers-and-catalog/🟦️.ts"
ENGINE="🧹️normalization/👥️mutation-projection/🟦️.ts"
TABLE_DIR="🗂️mutation-leaf-case-table"
STATUTES="🧪️tests/🔏️path-emoji-statutes/🟦️.ts"

@patch("leaf-case-table-schema")
def _():
    out=[]
    kind=block(TAXONOMY,"json-expect").replace("json-expect","json-case-table").replace(".expect.json",".cases.json").replace('"role": "schema"','"role": "test"')
    out.append(edit(TAXONOMY,'    "json-expect": {\n',kind+'    "json-expect": {\n','"json-case-table": {'))
    rule=block(TAXONOMY,"physical-json-expect-json").replace("physical-json-expect-json","physical-json-case-table-json").replace(".expect.json",".cases.json").replace("json-expect","json-case-table")
    out.append(edit(TAXONOMY,'    "physical-json-expect-json": {\n',rule+'    "physical-json-expect-json": {\n','"physical-json-case-table-json": {'))
    text=open(os.path.join(root,TAXONOMY),encoding="utf8").read()
    runner=re.search(r'      "fixtureRunner": \{\n        "relativePath": "[^"]+"\n      \}\n',text)
    anchor=runner.group(0) if runner else ""
    out.append(edit(TAXONOMY,anchor,anchor.rstrip("\n")+',\n      "leafCaseTable": {\n        "fileKindId": "json-case-table",\n        "casesField": "cases"\n      }\n','"leafCaseTable": {'))
    out.append(edit(CONTRACTS,
"or the subset's fixture runner discovers it at run time. */",
"the subset's fixture runner discovers it at run time, or the case table of its leaf lists it. */",
"or the case table of its leaf lists it. */"))
    out.append(edit(CONTRACTS,
"  readonly fixtureRunner: Readonly<{ relativePath: string }>;\n}",
"  readonly fixtureRunner: Readonly<{ relativePath: string }>;\n  readonly leafCaseTable: Readonly<{ fileKindId: string; casesField: string }>;\n}",
"readonly leafCaseTable:"))
    out.append(edit(LOADER,
'requireExactKeys(row, ["implementationCase", "exhaustiveCase", "fixtureRunner"], "mutationCatalogProjection.fixtureCoverage");',
'requireExactKeys(row, ["implementationCase", "exhaustiveCase", "fixtureRunner", "leafCaseTable"], "mutationCatalogProjection.fixtureCoverage");',
'"fixtureRunner", "leafCaseTable"]'))
    out.append(edit(LOADER,
'  requireExactKeys(runner, ["relativePath"], "mutationCatalogProjection.fixtureCoverage.fixtureRunner");\n',
'  requireExactKeys(runner, ["relativePath"], "mutationCatalogProjection.fixtureCoverage.fixtureRunner");\n  const table = requireRecord(row.leafCaseTable, "mutationCatalogProjection.fixtureCoverage.leafCaseTable");\n  requireExactKeys(table, ["fileKindId", "casesField"], "mutationCatalogProjection.fixtureCoverage.leafCaseTable");\n',
'const table = requireRecord(row.leafCaseTable'))
    out.append(edit(LOADER,
'    fixtureRunner: { relativePath: requireString(runner.relativePath, "mutationCatalogProjection.fixtureCoverage.fixtureRunner.relativePath") },\n',
'    fixtureRunner: { relativePath: requireString(runner.relativePath, "mutationCatalogProjection.fixtureCoverage.fixtureRunner.relativePath") },\n    leafCaseTable: { fileKindId: requireString(table.fileKindId, "mutationCatalogProjection.fixtureCoverage.leafCaseTable.fileKindId"), casesField: requireString(table.casesField, "mutationCatalogProjection.fixtureCoverage.leafCaseTable.casesField") },\n',
"    leafCaseTable: { fileKindId:"))
    out.append(edit(AUDIT,
'exactKeys(coverage, ["implementationCase", "exhaustiveCase", "fixtureRunner"], "mutationCatalogProjection.fixtureCoverage");',
'exactKeys(coverage, ["implementationCase", "exhaustiveCase", "fixtureRunner", "leafCaseTable"], "mutationCatalogProjection.fixtureCoverage");',
'"fixtureRunner", "leafCaseTable"]'))
    out.append(edit(AUDIT,
'      exactKeys(coverage.fixtureRunner, ["relativePath"], "mutationCatalogProjection.fixtureCoverage.fixtureRunner");\n',
'      exactKeys(coverage.fixtureRunner, ["relativePath"], "mutationCatalogProjection.fixtureCoverage.fixtureRunner");\n      exactKeys(coverage.leafCaseTable, ["fileKindId", "casesField"], "mutationCatalogProjection.fixtureCoverage.leafCaseTable");\n      if (!taxonomy.fileKinds[coverage.leafCaseTable?.fileKindId] || typeof coverage.leafCaseTable?.casesField !== "string" || coverage.leafCaseTable.casesField === "") problems.push("mutationCatalogProjection.fixtureCoverage.leafCaseTable must name a registered file kind and the member that lists the cases.");\n',
"exactKeys(coverage.leafCaseTable"))
    return out

@patch("leaf-case-table-engine")
def _():
    out=[]
    out.append(add("discovery/"+TABLE_DIR+"/🟦️.ts","🔍️discovery/"+TABLE_DIR+"/🟦️.ts"))
    out.append(edit(ENGINE,
'import { mutationFixturesDeclaredByFeature } from "../../🔍️discovery/🥒️mutation-fixture-coverage/🟦️.ts";\n',
'import { mutationFixturesDeclaredByFeature } from "../../🔍️discovery/🥒️mutation-fixture-coverage/🟦️.ts";\nimport { foldMutationCaseName, mutationLeafCaseTableProblems, parseMutationLeafCaseTable } from "../../🔍️discovery/'+TABLE_DIR+'/🟦️.ts";\n',
"parseMutationLeafCaseTable }"))
    out.append(edit(ENGINE,
"or an exhaustive mutation case declares the bundle in a feature step. Both sources are the `mutationCatalogProjection.fixtureCoverage` schema. */",
"an exhaustive mutation case declares the bundle in a feature step, or the case table of its leaf lists it. All three sources are the `mutationCatalogProjection.fixtureCoverage` schema. */",
"or the case table of its leaf lists it. All three sources"))
    out.append(edit(ENGINE,
"  const subsets = new Map<string, Readonly<{ runner: boolean; declared: ReadonlySet<string> }>>();\n",
'''  const tableKind = requireTaxonomyValue(discovery.fileKinds[coverage.leafCaseTable.fileKindId], "leaf case table file kind");
  const tableName = `${tableKind.emoji}${tableKind.extensionChains[0]}`.normalize("NFC");
  const leafTables = new Map<string, ReadonlySet<string> | null>();
  const leafTableCases = (subsetRoot: string, owner: string): ReadonlySet<string> | null => {
    const path = `${subsetRoot}/${discovery.schemaScopeOwnerLevels.facetDirName}/${discovery.schemaScopeOwnerLevels.mutationsFacetDirName}/${owner}/${discovery.testsDirName}/${tableName}`;
    if (leafTables.has(path)) return leafTables.get(path)!;
    let found: ReadonlySet<string> | null = null;
    if (existsSync(absolutePath(repoRoot, path))) {
      const table = parseMutationLeafCaseTable(readFileSync(absolutePath(repoRoot, path), "utf8"), coverage.leafCaseTable);
      if ("cases" in table) found = new Set(table.cases.map(foldMutationCaseName));
    }
    leafTables.set(path, found);
    return found;
  };
  const subsets = new Map<string, Readonly<{ runner: boolean; declared: ReadonlySet<string> }>>();
''',
"const leafTableCases = ("))
    out.append(edit(ENGINE,
'''    const found = subsetCoverage(fixtureDirectory.slice(0, at));
    return found.runner || found.declared.has(fixtureDirectory.slice(at + fixtureMarker.length).replaceAll("\\uFE0F", "").normalize("NFC"));
''',
'''    const subsetRoot = fixtureDirectory.slice(0, at), bundle = fixtureDirectory.slice(at + fixtureMarker.length), found = subsetCoverage(subsetRoot), split = bundle.lastIndexOf("/");
    return found.runner || found.declared.has(foldMutationCaseName(bundle)) || split > 0 && leafTableCases(subsetRoot, bundle.slice(0, split))?.has(foldMutationCaseName(bundle.slice(split + 1))) === true;
''',
"leafTableCases(subsetRoot, bundle.slice(0, split))"))
    out.append(edit(ENGINE,
'''    if (looksLikeFixtureScenario && !expected.has(entry.sourcePath) && !covered(entry.sourcePath)) entry.violations.push(violation("mutation-fixture-unpaired", entry.sourcePath, "Fixture bundle has no canonical implementation case"));
  }
}''',
'''    if (looksLikeFixtureScenario && !expected.has(entry.sourcePath) && !covered(entry.sourcePath)) entry.violations.push(violation("mutation-fixture-unpaired", entry.sourcePath, "Fixture bundle has no canonical implementation case"));
  }
  const tableContract = taxonomy.schema.mutationCatalogProjection.fixtureCoverage.leafCaseTable, bundleFacets = taxonomy.schema.mutationCatalogProjection.fixtureCoverage.exhaustiveCase.bundleFacetDirectoryNames, discovery = taxonomy.discoverySchema;
  const tableKind = requireTaxonomyValue(discovery.fileKinds[tableContract.fileKindId], "leaf case table file kind");
  const tableName = `${tableKind.emoji}${tableKind.extensionChains[0]}`.normalize("NFC");
  const schemaMarker = `/${discovery.schemaScopeOwnerLevels.facetDirName}/${discovery.schemaScopeOwnerLevels.mutationsFacetDirName}/`, tableSuffix = `/${discovery.testsDirName}/${tableName}`;
  for (const entry of entries.values()) {
    const at = entry.sourcePath.indexOf(schemaMarker);
    if (entry.nodeKind !== "file" || at < 0 || !entry.sourcePath.normalize("NFC").endsWith(tableSuffix)) continue;
    const owner = entry.sourcePath.slice(at + schemaMarker.length, entry.sourcePath.length - tableSuffix.length);
    const fixtureLeaf = `${entry.sourcePath.slice(0, at)}/${discovery.testsDirName}/${discovery.testFixturesDirName}/${discovery.schemaScopeOwnerLevels.mutationsFacetDirName}/${owner}`;
    let table;
    try {
      table = parseMutationLeafCaseTable(readFileSync(absolutePath(repoRoot, entry.sourcePath), "utf8"), tableContract);
    } catch (error) {
      table = { error: `cannot be read: ${(error as Error).message}` };
    }
    const bundles = entrySubtree(entries, fixtureLeaf).filter((row) => row.nodeKind === "directory" && dirname(row.sourcePath) === fixtureLeaf && bundleFacets.every((facet) => entries.has(`${row.sourcePath}/${facet}`))).map((row) => basename(row.sourcePath));
    const problems = "error" in table ? [table.error] : entries.has(fixtureLeaf) || existsSync(absolutePath(repoRoot, fixtureLeaf)) ? mutationLeafCaseTableProblems(table.cases, bundles) : ["has no fixture leaf"];
    for (const problem of problems) entry.violations.push(violation("mutation-case-table-invalid", entry.sourcePath, `Case table ${problem}`));
  }
}''',
"const tableContract = taxonomy.schema.mutationCatalogProjection.fixtureCoverage.leafCaseTable"))
    return out

@patch("leaf-case-table-tests")
def _():
    out=[]
    out.append(add("tests/"+TABLE_DIR+"/🟦️.ts","🧪️tests/"+TABLE_DIR+"/🟦️.ts"))
    out.append(add("fixtures/"+TABLE_DIR+"/🔣️.json","🧪️tests/🧫️fixtures/"+TABLE_DIR+"/🔣️.json"))
    out.append(edit(STATUTES,
'import { mutationFixturesDeclaredByFeature } from "../../🔍️discovery/🥒️mutation-fixture-coverage/🟦️.ts";\n',
'import { mutationFixturesDeclaredByFeature } from "../../🔍️discovery/🥒️mutation-fixture-coverage/🟦️.ts";\nimport { foldMutationCaseName, parseMutationLeafCaseTable } from "../../🔍️discovery/'+TABLE_DIR+'/🟦️.ts";\n',
"parseMutationLeafCaseTable }"))
    out.append(edit(STATUTES,
"pathBudgetStatement, mutationFixturesDeclaredByFeature, projectionBundleProblem:",
"pathBudgetStatement, mutationFixturesDeclaredByFeature, foldMutationCaseName, parseMutationLeafCaseTable, projectionBundleProblem:",
"foldMutationCaseName, parseMutationLeafCaseTable, projectionBundleProblem:"))
    out.append(edit(STATUTES,
'fixtureRunner: { relativePath: "🧬️schema/🧬️mutations/🧪️tests/🗂️fixture-cases/🦀️.rs" } });',
'fixtureRunner: { relativePath: "🧬️schema/🧬️mutations/🧪️tests/🗂️fixture-cases/🦀️.rs" }, leafCaseTable: { fileKindId: "json-case-table", casesField: "cases" } });',
"leafCaseTable: { fileKindId:"))
    return out

for pid,fn in PATCHES.items():
    if only and pid not in only: continue
    print(pid,fn())
print("checked" if check else "done")
