"""⚖️ C5B glob-ambiguity audit: names that a child exports through the root's `pub use child::*` and that a glob import of the root ALSO supplies (an external `use X::*;` or a local `pub use module::*;`) become ambiguous glob-vs-glob names for every consumer of the root (the original local definition shadowed the glob).
usage: ambig.py stage|live <tag>...   external glob sources are scanned approximately (every `pub` item name of the crate source tree: over-approximation, false positives possible)."""
import importlib, os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE + "/../rf8-tools"); sys.path.insert(0, HERE)
from rsitems import items, name_kind
REPO = "/Users/ueli/Documents/semio"
SCR = "/Users/ueli/.cache/c6fin"
UICON = REPO + "/🧰️framework/🔨️modules/🖱️ui/🧬️contract"
PUB = re.compile(r"^\s*pub(?:\([^)]*\))?\s+(?:(?:async|unsafe|const|extern(?: \"[^\"]*\")?)\s+)*(?:fn|struct|enum|trait|type|const|static|union|mod)\s+(\w+)|^\s*macro_rules!\s+(\w+)|^\s*pub\s+use\s+[^;]*?(?:as\s+(\w+)|::(\w+))\s*;", re.M)
def names_of_tree(root, skip=()):
    out = set()
    for dp, dn, fn in os.walk(root):
        dn[:] = [d for d in dn if d not in ("target", "node_modules", "📦️packages")]
        for f in fn:
            if f.endswith(".rs"):
                t = open(os.path.join(dp, f), encoding="utf-8", errors="ignore").read()
                for m in PUB.finditer(t):
                    out.add(next(g for g in m.groups() if g))
    return out
mode = sys.argv[1]
ok = True
for tag in sys.argv[2:]:
    sp = importlib.import_module("specs." + tag).SPEC
    base = os.path.join(SCR, "stage", tag) if mode == "stage" else os.path.join(REPO, os.path.dirname(sp["file"]))
    root = open(os.path.join(base, "🦀️.rs"), encoding="utf-8").read()
    globs = re.findall(r"^\s*(?:pub(?:\([^)]*\))?\s+)?use\s+(?:::)?([\w:]+)::\*;", root, re.M)
    child_names = {}
    for c in sp["children"]:
        t = open(os.path.join(base, c["folder"], "🦀️.rs"), encoding="utf-8").read()
        for it in items(t):
            v, k, n = name_kind(it["toks"])
            if n and k not in ("use", "impl", "mod") and not v.startswith("pub(self)"):
                child_names[n] = c["folder"]
    srcs = {}
    for g in globs:
        if g.split("::")[0] in [c["mod"] for c in sp["children"]]:
            continue
        if g == "semio_framework_ui_contract":
            srcs[g] = names_of_tree(UICON)
        else:
            m = re.search(r"#\[path\s*=\s*\"([^\"]+)\"\]\s*(?:#\[[^\]]*\]\s*)*(?:pub(?:\([^)]*\))?\s+)?mod\s+%s;" % re.escape(g.split("::")[0]), root)
            if m:
                p = os.path.normpath(os.path.join(REPO, os.path.dirname(sp["file"]), m.group(1)))
                srcs[g] = names_of_tree(os.path.dirname(p)) if False else set(n for it in items(open(p, encoding="utf-8").read()) for _, k, n in [name_kind(it["toks"])] if n)
            else:
                srcs[g] = set()
    bad = []
    for g, s in srcs.items():
        for n, f in child_names.items():
            if n in s:
                bad.append((g, n, f))
    print(f"[{tag}] glob sources {sorted(srcs)}; child item names {len(child_names)}; ambiguous candidates {len(bad)}")
    for b in bad:
        print("   CANDIDATE", b)
    ok = ok and not bad
print("AMBIG", "OK" if ok else "CANDIDATES")
