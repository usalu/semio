import re, collections, sys
from lib import *
import rsparse as P
from rs import blank
for rp in sys.argv[1:]:
    f = PLUG + "/" + rp
    t = read(f); c = blank(t)
    sts = P.statements(t, c, 0, len(t))
    print("==", rp, len(t), "bytes", t.count("\n"), "lines; top-level:", collections.Counter(s.kind for s in sts))
    for s in sts:
        if s.kind in ("mod",) or (s.kind == "fn"):
            print("   ", s.kind, s.name, "inline" if s.inline else "", t.count("\n", s.start, s.end) + 1, (s.attrs or "")[:60].replace("\n", " "))
    tests = len(re.findall(r"#\[(?:[\w:]*::)?(?:async_)?test\]", t))
    print("   tests:", tests)
