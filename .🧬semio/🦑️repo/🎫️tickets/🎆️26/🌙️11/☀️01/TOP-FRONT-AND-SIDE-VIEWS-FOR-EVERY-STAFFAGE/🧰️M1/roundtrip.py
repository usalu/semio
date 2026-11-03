import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import lib
for aid in sys.argv[1:]:
    d, indent, raw = lib.load(aid)
    out = lib.dumps(d, indent)
    print(aid, "indent", indent, "identical" if out == raw else "DIFFERENT", len(raw), len(out))
    if out != raw:
        for i, (a, b) in enumerate(zip(raw.splitlines(), out.splitlines())):
            if a != b:
                print("  line", i, repr(a[:100]), repr(b[:100]))
                break
