"""✂️ [nxdeadline] anchored edits of the store child law for batch `release` (the transpiled harnesses bind the new owner-frame, abort-aware helpers and nxsparse's backing rule): edit_test.py <root>. Exit 1 when an anchor is lost."""
import pathlib
import sys

ROOT = pathlib.Path(sys.argv[1])
REL = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🧪️tests/📦️child/🟦️.ts"
HELPER = 'const helper=tree.statements.find(row=>ts.isFunctionDeclaration(row)&&row.name?.text==="untilAbortedV1");'
EDITS = [
    ('retainCode=extract(graphPath,["retainNxGraphChildV1"])', 'retainCode=extract(graphPath,["untilAbortedV1","retainNxGraphChildV1"])'),
    ("retainBindings={check:()=>{},", "retainBindings={check:()=>{},poolBases:new WeakMap([[store,parent]]),withRepositoryCacheStoreOwnerFrameV1:(_parent:unknown,body:()=>Promise<unknown>)=>body(),"),
    (
        'row.name?.text==="acquireNamespace");if(!node)throw Error("Actual graph acquisition required");const code=ts.transpileModule(node.getText(tree),',
        'row.name?.text==="acquireNamespace");' + HELPER + 'if(!node||!helper)throw Error("Actual graph acquisition required");const code=ts.transpileModule(helper.getText(tree)+"\\n"+node.getText(tree),',
    ),
    ("check:()=>{},scopesV1:()=>entries", "check:()=>{},nxGraphBackingAdmittedV1:()=>true,scopesV1:()=>entries"),
    (
        'row.name?.text==="retainNxGraphChildV1");if(!node)throw Error("Actual retained graph child owner required");const code=ts.transpileModule(node.getText(tree),',
        'row.name?.text==="retainNxGraphChildV1");' + HELPER + 'if(!node||!helper)throw Error("Actual retained graph child owner required");const code=ts.transpileModule(helper.getText(tree)+"\\n"+node.getText(tree),',
    ),
    (
        "bindings={check:(actual:unknown)=>expect(actual).toBe(store),",
        "bindings={check:(actual:unknown)=>expect(actual).toBe(store),poolBases:new WeakMap([[store,image]]),withRepositoryCacheStoreOwnerFrameV1:(parent:unknown,body:()=>Promise<unknown>)=>{expect(parent).toBe(image);return body();},",
    ),
]
path = ROOT / REL
text = path.read_text(encoding="utf-8")
lost = [old for old, _ in EDITS if text.count(old) != 1]
for old in lost:
    print(f"ANCHOR x{text.count(old)}: {old[:80]}")
if lost:
    sys.exit(1)
for old, new in EDITS:
    text = text.replace(old, new)
mode = path.stat().st_mode
path.write_text(text, encoding="utf-8")
path.chmod(mode & 0o7777)
print("edited", REL)
