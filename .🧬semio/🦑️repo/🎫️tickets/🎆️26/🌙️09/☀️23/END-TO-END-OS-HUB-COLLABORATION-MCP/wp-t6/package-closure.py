"""🧭️ For every print fixture document, the transitive package closure, and whether it reaches the missing distribution package or the tikz patterns library."""
import re, pathlib, collections
root = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/📓️print")
latex = root / "🖋️latex"
req = {}
libs = {}
for f in list(latex.glob("*.sty")) + list(latex.glob("*.cls")):
    t = f.read_text(errors="replace")
    t = "\n".join(l.split("%")[0] for l in t.split("\n"))
    names = set()
    for m in re.finditer(r"\\(?:RequirePackage|usepackage)(?:\[[^\]]*\])?\{([^}]*)\}", t):
        names |= {n.strip() for n in m.group(1).split(",") if n.strip()}
    req[f.stem] = names
    libs[f.stem] = set(re.findall(r"\\usetikzlibrary\{([^}]*)\}", t))
def closure(names):
    seen, stack = set(), list(names)
    while stack:
        n = stack.pop()
        if n in seen: continue
        seen.add(n)
        stack.extend(req.get(n, ()))
    return seen
patt = {k for k, v in libs.items() if any("patterns" in x for x in v)}
rows = collections.Counter()
for f in sorted((root / "🧫️fixtures").rglob("*.tex")):
    t = f.read_text(errors="replace")
    t = "\n".join(l.split("%")[0] for l in t.split("\n"))
    names = set()
    for m in re.finditer(r"\\(?:RequirePackage|usepackage|documentclass)(?:\[[^\]]*\])?\{([^}]*)\}", t):
        names |= {n.strip() for n in m.group(1).split(",") if n.strip()}
    c = closure(names)
    dist = "semio-viz-charts-distribution" in c
    pat = bool(c & patt)
    rows[(dist, pat)] += 1
    print(("D" if dist else "-") + ("P" if pat else "-"), f.relative_to(root / "🧫️fixtures"))
print(rows, "patterns packages:", sorted(patt))
