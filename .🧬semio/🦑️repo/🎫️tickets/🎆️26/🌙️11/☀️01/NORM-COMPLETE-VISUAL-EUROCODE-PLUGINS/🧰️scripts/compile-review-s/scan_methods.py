"""🔎 Method calls `.name(` in artifact code whose name is neither defined by any fn in the repo (norm + framework) nor used by already compiled shared code (std/derive methods)."""
import re, sys, os, pickle
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from rs import *

cache = os.path.join(os.path.dirname(__file__), "..", "..", "🗑️generated", "compile-review-s", "methods-known.pkl")
CALL = re.compile(r'\.\s*([a-z_]\w*)\s*(?:::<[^>]*>)?\s*\(')
DEF = re.compile(r'\bfn\s+([a-z_]\w*)')
if os.path.exists(cache): known = pickle.load(open(cache, "rb"))
else:
    known = set()
    roots = ["/home/deck/semio/🧰️framework", NORM]
    for r in roots:
        for f in rs_files(r):
            try: s = strip(open(f, encoding="utf-8").read())
            except Exception: continue
            known |= set(DEF.findall(s))
            if r == NORM and "/🗿️artifacts/" not in f: known |= set(CALL.findall(s))
    pickle.dump(known, open(cache, "wb"))
code = sys.argv[1]
adir = ART + "/" + CODE[code]
res = {}
for f in rs_files(adir):
    s = strip(open(f, encoding="utf-8").read())
    for m in CALL.finditer(s):
        n = m.group(1)
        if n not in known: res.setdefault(n, []).append((f, s.count("\n", 0, m.start()) + 1))
for n, locs in sorted(res.items()):
    f, l = locs[0]
    print(f"{n}\t{len(locs)}\t{rel(f)}:{l}")
