# 🔧 Copies the staged tests and fixtures into the library and extends the pair-contract test. Run after apply-1 and apply-3.
import os,shutil
R="/Users/ueli/Documents/semio"
import os as _os
L=_os.environ.get("TAX3_L") or f"{R}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
C=f"{R}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/tax3-tools/new"
for name in ("🥒️mutation-fixture-coverage","🧱️package-implementation-admission"):
    os.makedirs(f"{L}/🧪️tests/{name}",exist_ok=True); shutil.copy(f"{C}/tests/{name}/🟦️.ts",f"{L}/🧪️tests/{name}/🟦️.ts")
    os.makedirs(f"{L}/🧪️tests/🧫️fixtures/{name}",exist_ok=True); shutil.copy(f"{C}/fixtures/{name}/🔣️.json",f"{L}/🧪️tests/🧫️fixtures/{name}/🔣️.json")
p=f"{L}/🧪️tests/🔏️path-emoji-statutes/🟦️.ts"
s=open(p,encoding="utf8").read()
old='  expect(pair.coverage).toBe("every-catalog-vector-has-one-fixture-bundle-with-declared-coverage");\n'
assert s.count(old)==1
s=s.replace(old,old+'''  expect(pair.fixtureCoverage).toEqual({ implementationCase: pair.implementationDescendantContractId, exhaustiveCase: { featureFileKindId: "gherkin-feature", stepUriPrefix: "shared://🧬️mutations/", bundleFacetDirectoryNames: ["📸️snapshot", "🦠️mutation", "🔺️diff", "🎯️outcome"] }, fixtureRunner: { relativePath: "🧬️schema/🧬️mutations/🧪️tests/🗂️fixture-cases/🦀️.rs" } });
''')
open(p,"w",encoding="utf8").write(s); print("ok")
# the normalization compile-harness in path-emoji-statutes mocks the engine's dependencies by name
s=open(p,encoding="utf8").read()
def rep(old,new):
    global s
    assert s.count(old)==1,(old[:70],s.count(old)); s=s.replace(old,new)
rep('"normalizeMutationCasePairs", "entrySubtree", "entrySubtreeIndexes"]','"normalizeMutationCasePairs", "fixtureCoverageOf", "entrySubtree", "entrySubtreeIndexes"]')
rep('mutationDescendantContract: () => ({ pathBudgetReserve: { bytes: 0 } }),','mutationDescendantContract: () => ({ pathBudgetReserve: { utf16Units: 0 } }), pathBudgetFitsWithReserve, pathBudgetStatement, mutationFixturesDeclaredByFeature,')
rep('import { leadingEmojiIdentity } from "@semio-tech/framework/identity/grapheme";\n','import { leadingEmojiIdentity } from "@semio-tech/framework/identity/grapheme";\nimport { pathBudgetFitsWithReserve, pathBudgetStatement } from "../../🔍️discovery/📏️path-budget/🟦️.ts";\nimport { mutationFixturesDeclaredByFeature } from "../../🔍️discovery/🥒️mutation-fixture-coverage/🟦️.ts";\n')
open(p,"w",encoding="utf8").write(s); print("tests patched")
