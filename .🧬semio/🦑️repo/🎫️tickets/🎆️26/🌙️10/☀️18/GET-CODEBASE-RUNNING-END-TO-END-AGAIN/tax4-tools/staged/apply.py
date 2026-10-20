# 🔧 Applies the tax4 library patches with anchored, assert-guarded replacements. usage: python3 apply.py [--root <library dir>] [--check] [--only <patch id>...]
# Default root is the live library; --check verifies every anchor resolves exactly once (or the patch is already applied) and writes nothing.
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
def retarget(rel,renames):
    path=os.path.join(root,rel)
    text=open(path,encoding="utf8").read()
    pending=[(old,new) for old,new in renames if re.search(bounded(old),text)]
    if not pending:
        assert all(new in text for _,new in renames),(rel,"neither old nor new names present")
        return "already"
    for old,new in pending:
        assert len(re.findall(bounded(old),text))==1,(rel,old)
    if not check:
        for old,new in pending: text=re.sub(bounded(old),new,text)
        open(path,"w",encoding="utf8").write(text)
    return "patched"
def bounded(name): return r'(?<![^/"\s(`])'+re.escape(name)+r'(?=[/"\s)`]|$)'
PATCHES={}
def patch(pid):
    def register(fn): PATCHES[pid]=fn; return fn
    return register

@patch("io-owner-descriptor-roots")
def _():
    m="🧹️normalization/🗺️artifact-projection-catalogs/🟦️.ts"
    out=[]
    out.append(edit(m,
'''/** 🪢️ Validates each descriptor's authored schema pointer without relocating its payload file. */
export function validateMutationPayloadSchemas(''',
'''/** 🛂️ Whether the direct children of one `🧬️mutations` root are payload owners that must carry their own canonical descriptor. A fixture mirror (`🧪️tests/🧫️fixtures/🧬️mutations`) and a codec projection (`🚪️io/<representation>/🧬️mutations`) consume the schema-side owner, which holds the descriptor. @see https://json-schema.org/draft-07/schema */
export function mutationOwnerRootHoldsDescriptors(root: string, taxonomy: Pick<LoadedTaxonomy["discoverySchema"], "testFixturesDirName" | "representationDirs">): boolean {
  const parent = basename(dirname(root));
  return parent !== taxonomy.testFixturesDirName && !taxonomy.representationDirs.includes(parent);
}

/** 🪢️ Validates each descriptor's authored schema pointer without relocating its payload file. */
export function validateMutationPayloadSchemas(''',
"export function mutationOwnerRootHoldsDescriptors"))
    out.append(edit(m,
"    if (basename(dirname(root)) === taxonomy.discoverySchema.testFixturesDirName) continue;\n    const identity = mutationOwnerIdentity(",
"    if (!mutationOwnerRootHoldsDescriptors(root, taxonomy.discoverySchema)) continue;\n    const identity = mutationOwnerIdentity(",
"mutationOwnerRootHoldsDescriptors(root, taxonomy.discoverySchema)"))
    out.append(add("tests/🛂️mutation-owner-descriptor-roots/🟦️.ts","🧪️tests/🛂️mutation-owner-descriptor-roots/🟦️.ts"))
    out.append(add("fixtures/🛂️mutation-owner-descriptor-roots/🔣️.json","🧪️tests/🧫️fixtures/🛂️mutation-owner-descriptor-roots/🔣️.json"))
    return out

@patch("schema-catalog-identity-paths")
def _():
    renames=[("🏷️change-node","🏷️change-node-label"),("🧭️change-graph","🧭️change-graph-directed"),("🧮️update-graph","🧮️update-graph-algorithm")]
    return [retarget("📓️schema-catalog.md",renames),retarget("🔣️schema-catalog.json",renames)]

@patch("statutes-owner-root-predicate")
def _():
    s="🧪️tests/🔏️path-emoji-statutes/🟦️.ts"
    out=[edit(s,
'import { inventoryTaxonomySources } from "../../🧹️normalization/🚪️source-admission/📁️io/🟦️.ts";\n',
'import { inventoryTaxonomySources } from "../../🧹️normalization/🚪️source-admission/📁️io/🟦️.ts";\nimport { mutationOwnerRootHoldsDescriptors } from "../../🧹️normalization/🗺️artifact-projection-catalogs/🟦️.ts";\n',
"import { mutationOwnerRootHoldsDescriptors }")]
    path=os.path.join(root,s)
    text=open(path,encoding="utf8").read()
    old="mutationOwnerIdentity, mutationPayloadSchemaProblems, requireRecord, readFileSync:"
    new="mutationOwnerIdentity, mutationOwnerRootHoldsDescriptors, mutationPayloadSchemaProblems, requireRecord, readFileSync:"
    if old in text:
        assert text.count(old)==2,(s,old,text.count(old))
        if not check: open(path,"w",encoding="utf8").write(text.replace(old,new))
        out.append("patched")
    else:
        assert text.count(new)==2,(s,"support rows")
        out.append("already")
    return out

for pid,fn in PATCHES.items():
    if only and pid not in only: continue
    print(pid,fn())
print("checked" if check else "done")
