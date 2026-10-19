# 🔧 Applies the fixture-coverage engine edits to the mutation projection (run after the library freeze lifts). Idempotent: refuses when the anchors are gone.
import os as _os
L=_os.environ.get("TAX3_L") or "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
p=f"{L}/🧹️normalization/👥️mutation-projection/🟦️.ts"
s=open(p,encoding="utf8").read()
def rep(old,new):
    global s
    assert s.count(old)==1,(old[:80],s.count(old))
    s=s.replace(old,new)
rep('''export function validateMutationCasePairs(
  _repoRoot: string,''','''export function validateMutationCasePairs(
  repoRoot: string,''')
rep('''  const expected = new Set<string>();
  const implementations =''','''  const expected = new Set<string>(), covered = fixtureCoverageOf(repoRoot, entries, taxonomy);
  const implementations =''')
rep('looksLikeFixtureScenario && !expected.has(entry.sourcePath)) entry.violations','looksLikeFixtureScenario && !expected.has(entry.sourcePath) && !covered(entry.sourcePath)) entry.violations')
rep('''    for (const vector of catalog.vectors) {
      const key = `${vector.mutationId}\\u0000${vector.sourceMutationDirectoryName}\\u0000${vector.scenarioId}`;
      if (!consumed.has(key) && vector.catalogPath)''','''    const fixtureBase = `${subsetRoot}/🧪️tests/🧫️fixtures/🧬️mutations`;
    let physicalFixtures: Map<string, string> | undefined;
    for (const vector of catalog.vectors) {
      const key = `${vector.mutationId}\\u0000${vector.sourceMutationDirectoryName}\\u0000${vector.scenarioId}`;
      if (consumed.has(key)) continue;
      physicalFixtures ??= new Map(entrySubtree(entries, fixtureBase).filter((entry) => entry.nodeKind === "directory").map((entry) => [entry.sourcePath.replaceAll("\\uFE0F", "").normalize("NFC"), entry.sourcePath]));
      const owner = canonicalProjectedMutationOwner(vector.mutationDirectoryName, vector.mutationId, subsetRoot, taxonomy);
      const found = [owner, vector.sourceMutationDirectoryName].filter((name): name is string => name !== null).map((name) => physicalFixtures!.get(`${fixtureBase}/${name}/${vector.scenarioDirectoryName}`.replaceAll("\\uFE0F", "").normalize("NFC"))).find((path): path is string => path !== undefined);
      if (found === undefined || !covered(found)) continue;
      const problem = projectionBundleProblem({ scenarioRoot: found }, entries, kinds, fixtureContract, taxonomy);
      if (problem) entries.get(found)?.violations.push(violation("mutation-fixture-invalid", found, problem));
      else consumed.add(key);
    }
    for (const vector of catalog.vectors) {
      const key = `${vector.mutationId}\\u0000${vector.sourceMutationDirectoryName}\\u0000${vector.scenarioId}`;
      if (!consumed.has(key) && vector.catalogPath)''')
rep('''  const ids = taxonomy.schema.mutationCatalogProjection;
  const implementationContract = mutationDescendantContract(taxonomy, "implementation");
  const fixtureContract = mutationDescendantContract(taxonomy, "fixture");''','''  const ids = taxonomy.schema.mutationCatalogProjection, covered = fixtureCoverageOf(repoRoot, entries, taxonomy);
  const implementationContract = mutationDescendantContract(taxonomy, "implementation");
  const fixtureContract = mutationDescendantContract(taxonomy, "fixture");''')
open(p,"w",encoding="utf8").write(s)
print("ok")
