#!/usr/bin/env python3
"""Stage-time generator of group 3 (declared image budget, scratch sweep): cuts anchors from the LIVE storage module, where groups 1 and 2 are already applied.

usage: mkops3.py [--root R]   rewrites the group-3 rows of staged/ops.json and staged/ops/g3-*.txt; groups 1 and 2 are kept as they were.
"""
import hashlib
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
OPS = os.path.join(HERE, "staged", "ops")
ROOT = os.path.abspath(os.path.join(HERE, *[".."] * 8))
if "--root" in sys.argv:
    ROOT = os.path.abspath(sys.argv[sys.argv.index("--root") + 1])
LIBRARY = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
STORAGE = LIBRARY + "/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts"
text = open(os.path.join(ROOT, STORAGE), encoding="utf-8").read()
rows = []


def operation(identifier, old, new, marker):
    if text.count(old) != 1:
        raise SystemExit("anchor of %s must occur once, found %d:\n%s" % (identifier, text.count(old), old[:200]))
    rows.append({"id": identifier, "group": 3, "target": "storage", "path": STORAGE, "marker": marker, "old": old, "new": new})


operation("g3-01-core-import", "import {concurrentAdmissionV1,concurrentCollectionV1,", "import {concurrentAdmissionV1,concurrentCollectionV1,concurrentRequiredImageBytesV1,", "concurrentRequiredImageBytesV1,")
operation("g3-02-glue-import", "import {acquireConcurrentLeaseV1,acquireConcurrentPinsV1,", "import {acquireConcurrentLeaseV1,acquireConcurrentPinsV1,sweepConcurrentTemporariesV1,", "acquireConcurrentPinsV1,sweepConcurrentTemporariesV1,")
operation(
    "g3-03-budget-import", 'import {repositoryCachePartitionRootV1,repositoryLeaseDirectoryV1} from',
    'import {repositoryCacheBudgetV1}from"../../../💾️storage/🔒️enforcement/📦️budget/🟦️.ts";\nimport {repositoryCachePartitionRootV1,repositoryLeaseDirectoryV1} from', 'import {repositoryCacheBudgetV1}from')
BUDGET = r'''/** 📐️ Refuses early, with a bilingual diagnostic and before anything is touched, when the declared dependencies partition cannot hold the package cache, scratch space and the installed generation set plus one building set for this platform's install backend. */
async function requireJavascriptDependencyBudgetV1(workspace:string,control:ResourceLeaseControl):Promise<void>{
 const backends=(installSchema.$defs.ProducerRecipeProtocol.const as unknown as {backends:Readonly<Record<string,string>>}).backends,required=concurrentRequiredImageBytesV1(backends[process.platform]==="clonefile"),declared=repositoryCacheBudgetV1(workspace).partitions.dependencies.usableBytes;
 if(declared>=required)return;
 await javascriptDependencyConcurrencyReportV1(control)(concurrentEventV1({event:"refused",neededBytes:required,freeBytes:declared}));
 const values={needed:Math.ceil(required/1048576),free:Math.floor(declared/1048576)};
 throw Error(concurrentTextV1(concurrentLanguageV1(process.env),"budgetTooSmall",values),{cause:Object.freeze({version:1,purpose:"dependency-budget-refusal",neededBytes:required,declaredBytes:declared,en:concurrentTextV1("en","budgetTooSmall",values),de:concurrentTextV1("de","budgetTooSmall",values)})});
}
'''
ADMIT_DOC = '/** 🛂️ Refuses to start one more generation the image cannot hold:'
operation("g3-04-budget-function", ADMIT_DOC, BUDGET + ADMIT_DOC, "async function requireJavascriptDependencyBudgetV1(")
operation(
    "g3-05-budget-first", "async function admitJavascriptDependencyGenerationV1(workspace:string,store:RepositoryCacheStoreV1,control:ResourceLeaseControl,scope:string):Promise<void>{\n const report=",
    "async function admitJavascriptDependencyGenerationV1(workspace:string,store:RepositoryCacheStoreV1,control:ResourceLeaseControl,scope:string):Promise<void>{\n await requireJavascriptDependencyBudgetV1(workspace,control);\n const report=", "await requireJavascriptDependencyBudgetV1(workspace,control);")
operation(
    "g3-06-sweep-scratch", "  const pointers=readConcurrentPointersV1(store.root),current=new Set(",
    '  const swept=sweepConcurrentTemporariesV1(store.root,Date.now(),options.neededBytes!==undefined||await concurrentFreeFractionV1(store.root)<concurrentProtocolV1.retention.pressureFreeFraction,checkpoint);\n  if(swept)await writeOwnedCommandOutputV1(process.stderr,"[DATA] Store dependency temporaries swept "+JSON.stringify({version:1,purpose:"current-dependency-temporaries-swept",count:swept})+"\\n",control);\n  const pointers=readConcurrentPointersV1(store.root),current=new Set(',
    "sweepConcurrentTemporariesV1(store.root,Date.now()")

path = os.path.join(HERE, "staged", "ops.json")
index = [row for row in json.load(open(path, encoding="utf-8")) if row["group"] != 3]
for name in os.listdir(OPS):
    if name.startswith("g3-"):
        os.remove(os.path.join(OPS, name))
for row in rows:
    for part in ("old", "new"):
        open(os.path.join(OPS, "%s.%s.txt" % (row["id"], part)), "w", encoding="utf-8").write(row[part])
    index.append({"id": row["id"], "group": 3, "target": "storage", "path": row["path"], "marker": row["marker"], "oldSha256": hashlib.sha256(row["old"].encode()).hexdigest()})
json.dump(index, open(path, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
open(path, "a").write("\n")
print("%d group-3 operations staged" % len(rows))
