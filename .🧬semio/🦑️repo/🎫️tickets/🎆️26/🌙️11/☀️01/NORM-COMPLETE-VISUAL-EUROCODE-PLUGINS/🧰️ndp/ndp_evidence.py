"""NDP evidence: rows verified without an independent source go back to unverified, honestly.

Run: python3 -I ndp_evidence.py REGISTRY.json
Idempotent. A row stays verified only if its evidence names a library citation or a print.
"""
import json, pathlib, sys

registry = pathlib.Path(sys.argv[1])
doc = json.loads(registry.read_text())
NOTE = "ndp-evidence 2026-11-03: UNVERIFIED, no independent source. Searched blue-prints 0.8.0 (en_1995_1_1_2004/2023/2025, en_338_2016), structuralcodes 0.7.3, groundhog 0.15.0 and the repo: none holds this table (EN 1995-1-1 Table 3.1 k_mod, Table 3.2 k_def, EN 14080 Table 5). Needs the standard print and, for NA-DE, the DIN NA."
count = 0
for e in doc["entries"]:
    ev = e["evidence"]
    sourced = any(k in ev for k in ("http", "blue-prints", "structuralcodes", "groundhog", "VERIFIED by"))
    if e["status"] == "verified" and sourced and e["source"]["edition"].startswith("to be recorded") and "EN 14080" in ev:
        e["source"]["edition"] = "EN 14080:2013"
    if e["status"] == "verified" and not sourced:
        e["status"] = "unverified"
        e["evidence"] = NOTE if not ev.strip() or "CARRIED" in ev or "ndp-evidence" in ev else NOTE + " | previous: " + ev
        e["source"]["edition"] = "to be recorded at verification"
        count += 1
registry.write_text(json.dumps(doc, ensure_ascii=False, indent=1))
print("downgraded", count)
