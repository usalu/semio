from pathlib import Path

path = Path(__file__).resolve().parents[7] / "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🎟️reservation/🧪️tests/🟦️.ts"
extended = Path("\\\\?\\" + str(path))
text = extended.read_text(encoding="utf-8")
old = 'fn=tree.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text==="acquireNativeStorageOwnerV1")!,code=ts.transpileModule(fn.getText(tree).replace(/^export /u,"")+";return acquireNativeStorageOwnerV1;"'
new = 'declarations=["acquireNativeStorageOwnerBodyV1","acquireNativeStorageOwnerV1"].map(name=>{const node=tree.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text===name);if(!node)throw Error("Actual Native owner admission required");return node.getText(tree).replace(/^export /u,"");}),code=ts.transpileModule(declarations.join("\\n")+";return acquireNativeStorageOwnerV1;"'
if old not in text:
    raise SystemExit("owner extraction not found")
updated = text.replace(old, new, 1)
print(repr(updated[updated.index("declarations"):updated.index("declarations")+80]))
data = updated.encode("utf-8")
with path.open("r+b") as handle:
    handle.write(data)
    handle.truncate()
print("patched")
