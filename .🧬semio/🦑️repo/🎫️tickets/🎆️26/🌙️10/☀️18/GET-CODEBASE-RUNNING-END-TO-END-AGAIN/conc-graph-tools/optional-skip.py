import os, sys
ROOT = "/Users/ueli/Documents/semio"
if "--root" in sys.argv: ROOT = os.path.abspath(sys.argv[sys.argv.index("--root") + 1])
def resolve(rel):
    cur = ROOT
    for part in rel.split("/"):
        hits = [n for n in os.listdir(cur) if n.replace("️", "") == part.replace("️", "")]
        assert len(hits) == 1, (part, hits); cur = os.path.join(cur, hits[0])
    return cur
LIB = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
OPEN = resolve(LIB + "⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts")
ST = resolve(LIB + "🏗️build/📥️inputs/💾️storage/🟨️.mjs")
STD = ST.replace(".mjs", ".d.mts")
G = resolve(LIB + "🏗️build/🕸️graph/🟦️.ts")
files = {}
def sub(p, marker, old, new, label, count=1):
    if p not in files: files[p] = [open(p, encoding="utf8", newline="").read(), False]
    row = files[p]
    if marker in row[0]: print("applied", label); return
    assert row[0].count(old) == count, (label, row[0].count(old))
    row[0] = row[0].replace(old, new); row[1] = True; print("pending", label)
# A canonical image of a partition
sub(OPEN, "export function repositoryCachePartitionNativeImageV1", "const partitionOwners=new WeakMap<NativeImageStorageV1,PartitionOwner>(),partitionFrames=new AsyncLocalStorage<PartitionFrame>();",
    "const partitionOwners=new WeakMap<NativeImageStorageV1,PartitionOwner>(),partitionFrames=new AsyncLocalStorage<PartitionFrame>();\n/** 🪪️ Returns the exact canonical Native image behind an opened partition (the image itself when it is not a partition), the only object the exclusive writer proofs accept. */\nexport function repositoryCachePartitionNativeImageV1(partition:NativeImageStorageV1):NativeImageStorageV1{return partitionOwners.get(partition)?.image??partition;}", "opening")
# B storage
sub(ST, 'import {openRepositoryCachePartitionV1,repositoryCachePartitionNativeImageV1,repositoryLeaseDirectoryV1}', 'import {openRepositoryCachePartitionV1,repositoryLeaseDirectoryV1}', 'import {openRepositoryCachePartitionV1,repositoryCachePartitionNativeImageV1,repositoryLeaseDirectoryV1}', "import")
sub(ST, "retireOptionalGraphHeadroomV1(repositoryCachePartitionNativeImageV1(image),custody,request)", "retireOptionalGraphHeadroomV1(image,custody,request));if(!result.acquired)return false;", "retireOptionalGraphHeadroomV1(repositoryCachePartitionNativeImageV1(image),custody,request));if(!result.acquired)return false;", "canonical retire")
# C optional caches option
sub(ST, "async function withGraphFactWriterV1(workspace,invocation,phaseControl,body,namespace,options){", "async function withGraphFactWriterV1(workspace,invocation,phaseControl,body,namespace){", "async function withGraphFactWriterV1(workspace,invocation,phaseControl,body,namespace,options){", "signature")
sub(ST, "const optional=options?.optionalCaches!==false;", " const current=writers.getStore(),owner=graphDatasetWorkspaceV1(workspace);\n if(current?.workspace===owner&&current.store.invocation===invocation&&(!namespace||current.namespace===namespace))", " const current=writers.getStore(),owner=graphDatasetWorkspaceV1(workspace),optional=options?.optionalCaches!==false;\n if(current?.workspace===owner&&current.store.invocation===invocation&&(!namespace||current.namespace===namespace))", "flag")
sub(ST, "if(!current&&optional)await maintain(false);", "if(!current)await maintain(false);", "if(!current&&optional)await maintain(false);", "maintain gate")
sub(ST, "if(optional&&!current&&!maintained&&buildGraphOptionalRetirementDueV1(", "if(!current&&!maintained&&buildGraphOptionalRetirementDueV1(", "if(optional&&!current&&!maintained&&buildGraphOptionalRetirementDueV1(", "retry gate")
sub(ST, 'export async function withRepositoryBuildGraphSelectionV1(workspace,invocation,phaseControl,identity,body,options){', 'export async function withRepositoryBuildGraphSelectionV1(workspace,invocation,phaseControl,identity,body){', 'export async function withRepositoryBuildGraphSelectionV1(workspace,invocation,phaseControl,identity,body,options){', "selection signature")
sub(ST, '},"selection:"+identity,options);', '},"selection:"+identity);', '},"selection:"+identity,options);', "selection pass")
# D graph glue
sub(G, "withRepositoryBuildGraphSelectionV1(root,invocation,phaseControl,scope,run,scope===generationScopeV1?{optionalCaches:false}:undefined)", "withRepositoryBuildGraphSelectionV1(root,invocation,phaseControl,scope,run)", "withRepositoryBuildGraphSelectionV1(root,invocation,phaseControl,scope,run,scope===generationScopeV1?{optionalCaches:false}:undefined)", "graph glue")
if STD and os.path.exists(STD):
    p = STD; files[p] = [open(p, encoding="utf8", newline="").read(), False]
    import re
    m = re.search(r"export function withRepositoryBuildGraphSelectionV1\([^\n]*", files[p][0])
    print("d.mts:", m.group(0)[:200] if m else None)
if "--write" in sys.argv:
    for p, (t, ch) in files.items():
        if ch: open(p, "w", encoding="utf8", newline="").write(t)
    print("WRITTEN")
