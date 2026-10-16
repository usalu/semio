import sys, os
sys.path.insert(0, '.')
from oinf_rs import *
INF = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite"
bad = 0
tot = 0
for root in ("🌍️world", "🎲️board"):
    for dp, dn, fn in os.walk(os.path.join(INF, root)):
        for f in fn:
            if not f.endswith('.rs'): continue
            p = os.path.join(dp, f)
            text = open(p, encoding='utf-8').read()
            try:
                items, _ = chunk_file(text)
            except Exception as e:
                print("FAIL", p[len(INF):], repr(e)[:200]); bad += 1; continue
            tot += 1
            cols = [(it.kind, it.name, it.start - line_start(text, it.start)) for it in items]
            nz = [c for c in cols if c[2] != 0]
            if nz: print("NONZERO-COL", p[len(INF):], nz[:5])
            # coverage
            cov = ''.join(text[it.full_start:it.full_end] for it in items)
            tail_start = items[-1].full_end if items else 0
            if cov + text[tail_start:] != text and not items == []:
                print("COVER-MISMATCH", p[len(INF):])
print("files", tot, "bad", bad)
