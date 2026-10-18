import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from methods import *
ls, a, b, ms = parse(open(os.path.dirname(os.path.abspath(__file__)) + "/pai.orig.rs", encoding="utf-8").read())
for T in (4, 6, 8, 10, 12, 15, 20, 25):
    saved = 0; extra = 0; n = 0
    for m in ms:
        L = m["e"] - m["s"] + 1
        if L < T: continue
        sig, body, pl = split_sig(ls, m)
        names = params(pl)
        call = "self.%s(%s)%s" % (m["name"], ",".join(names), ".await" if m["is_async"] else "")
        fwd = sig.strip() + " { " + call + " }"
        extra += len(("    " + fwd).encode()) + 1 + len("pub(super) ")
        saved += L - 1
        n += 1
    print(T, "methods", n, "lines saved", saved, "extra bytes", extra)
