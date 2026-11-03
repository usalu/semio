import json, sys
from collections import Counter
sys.argv = ["x", "out.json", "all"]
exec(open("report.py", encoding="utf-8").read().split("mode = sys.argv[2]")[0])
out = []
out.append("| Code | Items | Implemented | Partial | Missing | Blocked on print/unverified | Trace-only share |")
out.append("|---|---|---|---|---|---|---|")
tot = Counter()
for code in o:
    c = Counter(r["status"] for r in o[code]["items"])
    n = len(o[code]["items"])
    tot.update(c)
    stale = sum(1 for r in o[code]["items"] if r["kind"] == "check" and r["status"] == "implemented" and any("stale" in x for x in r["notes"]))
    out.append("| %s | %d | %d | %d | %d | %d | %d stale-fragment rows |" % (code, n, c["implemented"], c["partial"], c["missing"], c["blocked-unverified"], stale))
out.append("| **Total** | %d | %d | %d | %d | %d | |" % (sum(tot.values()), tot["implemented"], tot["partial"], tot["missing"], tot["blocked-unverified"]))
out.append("")
out.append("### Missing and partial (clause, spec line, evidence)")
out.append("")
out.append("| Code | Id | Clause | Status | Spec line | Requirement (abridged) | Evidence |")
out.append("|---|---|---|---|---|---|---|")
for code in o:
    for r in o[code]["items"]:
        if r["status"] in ("missing", "partial"):
            ev = r["notes"][0] if r["notes"] else ""
            if r["kind"] == "check" and r["ev"].get("idHits"):
                ev += "; id literal present in " + r["ev"]["idHits"][0].split("/")[0]
            out.append("| %s | %s | %s | %s | %s | %s | %s |" % (code, r["id"], r["clause"][:40].replace("|", "/"), r["status"], lineno(code, r["id"]), r["en"][:170].replace("|", "/"), ev.replace("|", "/")))
open("tables.md", "w", encoding="utf-8").write("\n".join(out) + "\n")
blk = []
for code in o:
    ids = [r["id"] for r in o[code]["items"] if r["status"] == "blocked-unverified"]
    blk.append("- %s (%d): %s" % (code, len(ids), ", ".join(ids)))
open("blocked.md", "w", encoding="utf-8").write("\n".join(blk) + "\n")
print("\n".join(out))
