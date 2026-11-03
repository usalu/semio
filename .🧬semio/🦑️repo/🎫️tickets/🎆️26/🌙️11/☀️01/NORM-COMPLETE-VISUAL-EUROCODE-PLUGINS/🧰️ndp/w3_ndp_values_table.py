import sys, os, collections
sys.path.insert(0, os.path.dirname(__file__))
import w3_ndp_values_scan as s, w3_ndp_values_kind as k
out = ["code\tid\tclause\tsymbol\ten\tde\tread_kind\tfirst_use\tclass"]
GATED = {"ndp.en1991.alpha-a.", "ndp.en1991.vb0.", "ndp.en1992.nu2-", "ndp.en1992.vrdcc-k", "ndp.en1995.k-cr.", "ndp.en1996.fvlt.", "ndp.en1996.gamma-m.accidental", "ndp.en1998.en3.na.return-period"}
summary = []
for code in s.CODES:
    kinds = k.classify(code)
    n = collections.Counter()
    for i, kd in sorted(kinds.items()):
        for x in s.resolve(i):
            e = s.E[x]
            if e["recommended"] is not None and e["naDe"] is not None: continue
            if len(s.resolve(i)) > 1 and x != s.resolve(i)[0]: continue
            kind = "value" if "value" in kd else ("cite" if "cite" in kd and len(kd) == 1 else "cite-or-table")
            cls = "annex-gated value read" if any(x.startswith(g) for g in GATED) else ("optional value read (None handled)" if x == "ndp.en1995.fire.connection-reduced-load-k" else ("one-sided row, EN-only constant" if (e["recommended"] is not None) else ("citation of a rule or formula block" )))
            if x.startswith(("ndp.en1997.7.7", "ndp.en1997.9.")) and e["recommended"] is not None: cls = "fallback removed, DE row made explicit"
            first = next(iter(next(iter(kd.values()))))
            out.append("\t".join([code, x, e["clause"], e["symbol"][:40].replace("\t", " "), str(e["recommended"]), str(e["naDe"]), kind, first.split("/")[-2] + ":" + first.split(":")[-1], cls]))
            n[cls] += 1
    summary.append((code, sum(n.values()), dict(n)))
open(sys.argv[1], "w", encoding="utf-8").write("\n".join(out) + "\n")
for r in summary: print(r)
