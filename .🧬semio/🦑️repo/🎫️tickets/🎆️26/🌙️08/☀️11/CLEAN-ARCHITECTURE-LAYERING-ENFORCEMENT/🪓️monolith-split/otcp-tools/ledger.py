"""Widening ledger (fast): per edited file, the added lines whose only change against a removed line is the visibility / doc(hidden) / cfg gate, and the added testkit hooks."""
import collections, os, re, sys
sys.path.insert(0, os.path.dirname(__file__))
import edit as E
NORM = re.compile(r"#\[doc\(hidden\)\]\s*|\bpub(?:\([^)]*\))?\s+")


def norm(s):
    return NORM.sub("", s.strip())


rows = []
for dp, dn, fn in os.walk(E.PRE):
    for f in fn:
        p = os.path.join(dp, f)
        rel = os.path.relpath(p, E.PRE)
        live = os.path.join(E.ROOT, rel)
        if not os.path.exists(live):
            continue
        a = collections.Counter(l for l in open(p, encoding="utf-8").read().split("\n") if l.strip())
        b = collections.Counter(l for l in open(live, encoding="utf-8").read().split("\n") if l.strip())
        removed = a - b
        added = b - a
        rn = collections.Counter(norm(l) for l in removed.elements())
        for l in added.elements():
            n = norm(l)
            if rn.get(n, 0) > 0 and l.strip() != n and ("pub" in l):
                rn[n] -= 1
                rows.append((rel, "WIDEN", l.strip()[:150]))
            elif "testkit" in l or "mutation-testing" in l:
                rows.append((rel, "HOOK", l.strip()[:150]))
for r in sorted(rows):
    print("\t".join(r))
