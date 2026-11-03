"""EN 1997 v12: registers the unverified CPT cone factor row in the shared NDP registry, idempotently, then regenerates `🗃️table.rs`.
Usage: /home/deck/semio/.venv/bin/python -I ndp.py"""
import json, subprocess, sys

NDP = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/⚖️compliance/📜️annex-and-tables/🔖️ndp/"
CODEGEN = "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS/🧰️ndp/codegen.py"
ENTRY = {"id": "ndp.en1997.2.cpt-nkt", "code": "EN 1997", "part": "2", "clause": "EN 1997-2 Annex D (CPT)", "symbol": "N_kt (cone factor of c_u = (q_t - sigma_v0) / N_kt)",
         "recommended": 15.0, "naDe": 15.0, "unit": "",
         "source": {"document": "EN 1997-2 / DIN EN 1997-2/NA", "edition": "to be recorded at verification", "locator": "EN 1997-2 Annex D (CPT)"},
         "status": "unverified", "specRef": "en1997-en1998-target.md GI-03: N_kt = 15...20 by project calibration; the repo spec gives no single value and the German annex value is not recorded",
         "asWritten": "N_kt = 15 (lower end of the 15...20 range in the spec) as used by the artifact; a calibrated project value is entered per soil layer (cptNkt)",
         "evidence": "range from the spec text; the single value is a placeholder, unverified"}

path = NDP + "🔣️.json"
text = open(path, encoding="utf-8").read()
data = json.loads(text)
assert json.dumps(data, indent=1, ensure_ascii=False) == text, "registry formatting changed"
if any(e["id"] == ENTRY["id"] for e in data["entries"]):
    print("registry already complete")
else:
    data["entries"] = sorted(data["entries"] + [ENTRY], key=lambda e: e["id"])
    open(path, "w", encoding="utf-8").write(json.dumps(data, indent=1, ensure_ascii=False))
    subprocess.run([sys.executable, "-I", CODEGEN, path, NDP + "🗃️table.rs"], check=True)
    print("added 1")
