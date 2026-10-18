import re, os, sys
# 🕸️ hubdeps.py <manifest> : for a hub root whose children `use super::*`, prints which sibling definitions and root definitions each child really uses (names of 7+ chars, ignoring trait-method noise).
R = "/Users/ueli/Documents/semio"
if sys.argv[1].endswith(".rs"):
    root = os.path.join(R, sys.argv[1])
else:
    man = os.path.join(R, sys.argv[1])
    mt = open(man).read()
    lib = re.search(r'\[lib\][^\[]*?path = "([^"]+)"', mt).group(1)
    root = os.path.normpath(os.path.join(os.path.dirname(man), lib))
rd = os.path.dirname(root)
rt = open(root).read()
mods = {}
for m in re.finditer(r'#\[path ?= ?"([^"]+)"\]\n(?:pub )?mod (\w+);', rt):
    f = os.path.normpath(os.path.join(rd, m.group(1)))
    if os.path.isfile(f):
        mods[m.group(2)] = open(f).read()
def defs(t):
    return set(re.findall(r"(?m)^\s*(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?(?:unsafe\s+)?(?:fn|struct|enum|const|static|trait|type)\s+(\w+)", t))
D = {m: defs(t) for m, t in mods.items()}
Dr = defs(rt)
for m, t in mods.items():
    sup = "use super::*" in t
    uses = {}
    for o in mods:
        if o == m: continue
        hit = sorted(n for n in D[o] if len(n) > 7 and re.search(r"\b%s\b" % re.escape(n), t))
        if hit: uses[o] = hit[:3] + (["+%d" % (len(hit) - 3)] if len(hit) > 3 else [])
    r = sorted(n for n in Dr if len(n) > 4 and re.search(r"\b%s\b" % re.escape(n), t))
    print("==", m, "lines", len(t.split("\n")), "super*" if sup else "", "\n   siblings:", uses, "\n   root:", r[:8])
