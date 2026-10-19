"""📝 C5B source-text reader re-points: every `readFileSync(<root file of a split module>, "utf8")` of a TS law becomes `readModuleTreeSource(<same path>)` (the module tree is the text the unsplit file held). Each edit asserts a unique anchor; the import of the module-tree reader is added where the file lacks it (relative path computed from the file). usage: tsedit.py [--apply]"""
import os, re, sys
R = "/Users/ueli/Documents/semio/"
M = "🧰️framework/🛍️products/💻️os/🔨️modules/"
PL = M + "🔌️plugin/"
TREE = R + "🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts"
EDITS = {
    M + "🏪️store/🧩️composition/📨️emission/📦️owned/🧪️tests/🟦️.ts": [
        ('const source=readFileSync(join(base,"🗃️document/🦀️.rs"),"utf8");', 'const source=readModuleTreeSource(join(base,"🗃️document/🦀️.rs"));')],
    PL + "🧵️retained-command/🪟️mounted/♻️frontier/🧪️tests/🟦️.ts": [
        ('document=readFileSync(join(root,"🗃️document/🦀️.rs"),"utf8")', 'document=readModuleTreeSource(join(root,"🗃️document/🦀️.rs"))')],
    PL + "🧠️interpreter/🧪️tests/🔌️exports/🟦️.ts": [
        ('const source = readFileSync(resolve(owner, "🦀️.rs"), "utf8"), tree = parser.parse(source);', 'const source = readModuleTreeSource(resolve(owner, "🦀️.rs")), tree = parser.parse(source);'),
        ('const runtimeTree = parser.parse(readFileSync(resolve(owner, "🏃️runtime/🦀️.rs"), "utf8"));', 'const runtimeTree = parser.parse(readModuleTreeSource(resolve(owner, "🏃️runtime/🦀️.rs")));'),
        ('tree = parser.parse(readFileSync(resolve(owner, "🏃️runtime/🦀️.rs"), "utf8"));\n  try {', 'tree = parser.parse(readModuleTreeSource(resolve(owner, "🏃️runtime/🦀️.rs")));\n  try {')],
}
apply = "--apply" in sys.argv
bad = 0
for path, edits in EDITS.items():
    p = R + path if not path.startswith("/") else path
    text = open(p, encoding="utf-8").read()
    for old, new in edits:
        n = text.count(old)
        if n < 1:
            print("MISSING anchor", p.split("/")[-3], old[:70]); bad += 1; continue
        print(f"{p.split('/')[-3]}/{p.split('/')[-2]}: {n} x {old[:60]}")
        text = text.replace(old, new)
    if "readModuleTreeSource" in text and not re.search(r"^import\b[^\n]*readModuleTreeSource", text, flags=re.M):
        rel = "@semio-tech/framework-compiler-ts/syntax/module-tree"
        imp = f'import {{ readModuleTreeSource }} from "{rel}";\n'
        lines = text.split("\n")
        idx = max(i for i, l in enumerate(lines) if re.match(r"^import\b", l) and l.rstrip().endswith(";")) if any(re.match(r"^import\b", l) for l in lines) else -1
        lines.insert(idx + 1, imp.rstrip("\n"))
        text = "\n".join(lines)
        print("   + import", rel[-40:])
    if apply and not bad:
        open(p, "w", encoding="utf-8").write(text)
print("bad", bad, "applied" if apply and not bad else "dry")
