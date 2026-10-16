import sys, os, re, collections
sys.path.insert(0, ".")
from scaffold import *
S = "/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/applied/"
MCPS = S + "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/"
import plan3
idents = {k.replace("-", "_"): k for k in list(DEPS) + list(TARGET_DEPS) + list(DEV_DEPS)}
idents["store"] = "semio-framework-os-kernel"
crates = [(n, d[0]) for n, d in plan3.NODES.items() if d[0]]
for n, nd in crates + [("runtime", None)]:
    pkgdir = MCPS + (nd + "/📦️packages/🦀️rust" if nd else "📦️packages/🦀️rust")
    man = open(pkgdir + "/Cargo.toml", encoding="utf-8").read()
    declared = set(re.findall(r'^([A-Za-z0-9_-]+) = ', man, re.M)) | set(re.findall(r'package = "([^"]+)"', man))
    lib = open(MCPS + (nd + "/🦀️.rs" if nd else "📦️packages/🦀️rust/🦀️.rs"), encoding="utf-8").read()
    files = [lib]
    for m in re.finditer(r'#\[path = "([^"]+)"\]', lib):
        p = os.path.normpath(os.path.join(os.path.dirname(MCPS + (nd + "/🦀️.rs" if nd else "📦️packages/🦀️rust/🦀️.rs")), m.group(1)))
        if os.path.exists(p): files.append(open(p, encoding="utf-8").read())
        else:
            r = p.replace(S, "/Users/ueli/Documents/semio/")
            if os.path.exists(r) and "trace" not in r: files.append(open(r, encoding="utf-8").read())
    used = set()
    for t in files:
        code = re.sub(r'//[^\n]*', '', t)
        for m in re.finditer(r'(?<![\w:$.])(?:::)?([a-z][a-z0-9_]*)::', code):
            if m.group(1) in idents: used.add(idents[m.group(1)])
        for m in re.finditer(r'^\s*(?:pub )?use (?:::)?([a-z][a-z0-9_]*)\b', code, re.M):
            if m.group(1) in idents: used.add(idents[m.group(1)])
    missing = sorted(u for u in used if u not in declared)
    extra = sorted(d for d in declared if d in idents.values() and d not in used)
    print(f"{n:16s} missing={missing} unused={extra}")
