"""Undeclared external crate idents in new crates' source closures. usage: undeclared.py <manifest>..."""
import sys, re, os
sys.path.insert(0, os.path.dirname(__file__))
from modtree import *
def sec(t, name):
    m = re.search(r'(?ms)^\[' + re.escape(name) + r'\]\s*\n(.*?)(?=^\[|\Z)', t)
    return m.group(1) if m else ""
def libname(mf):
    t = rd(mf)
    m = re.search(r'(?m)^name\s*=\s*"([^"]+)"', sec(t, "lib"))
    if m: return m.group(1)
    return re.search(r'(?m)^name\s*=\s*"([^"]+)"', sec(t, "package")).group(1).replace("-", "_")
def run(mf):
    t = rd(mf); d = os.path.dirname(mf)
    deps = {}
    for sn in ("dependencies", "dev-dependencies", "build-dependencies"):
        for line in sec(t, sn).split("\n"):
            m = re.match(r'([A-Za-z0-9_-]+)\s*(?:=|\.)', line.strip())
            if not m: continue
            k = m.group(1); deps[k.replace("-", "_")] = 1
            pm = re.search(r'path\s*=\s*"([^"]+)"', line)
            if pm:
                try: deps[libname(os.path.normpath(os.path.join(ROOT, d, pm.group(1), "Cargo.toml")))] = 1
                except Exception: pass
    # workspace deps: resolve names by crate name lookups below
    root, nodes = tree(mf)
    files = {root} | {n.file for n in nodes if n.file}
    own = libname(mf)
    used = set()
    for f in files:
        if not os.path.exists(f): continue
        s = re.sub(r'//[^\n]*', '', rd(f))
        s = re.sub(r'"(?:\\.|[^"\\])*"', '""', s)
        for m in re.finditer(r'(?<![A-Za-z0-9_:.])(semio_[a-z0-9_]+|pack|flow|protocol|dsl|ink)::', s): used.add(m.group(1))
    return sorted(u for u in used if u not in deps and u != own), own
if __name__ == "__main__":
    for mf in sys.argv[1:]:
        try:
            miss, own = run(mf); print("OK " if not miss else "MISS", own, miss)
        except Exception as e: print("ERR", mf[-60:], e)
