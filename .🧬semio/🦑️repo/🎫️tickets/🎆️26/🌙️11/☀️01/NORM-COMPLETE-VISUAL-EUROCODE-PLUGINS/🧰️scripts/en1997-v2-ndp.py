"""EN 1997 v2: registers the unverified constants of the Annex A chord rotation, Annex B liquefaction screen, Annex F bearing surface and pile expressions in the shared NDP registry (`⚖️compliance/📜️annex-and-tables/🔖️ndp/🔣️.json`), idempotently, then regenerates `🗃️table.rs` with `🧰️ndp/codegen.py`.
Usage: /home/deck/semio/.venv/bin/python -I en1998-v2-ndp.py"""
import json, subprocess, sys

NDP = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/⚖️compliance/📜️annex-and-tables/🔖️ndp/"
CODEGEN = "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS/🧰️ndp/codegen.py"
PRE = "ndp.en1997."
DOC = "DIN EN 1997-1/NA:2023-11"
SPEC = "wave2 en1998 v2: constant recalled from the Annex text, not checked against the licensed edition"

ROWS = []
STATIC_EN = {1: (1.40, 1.40), 2: (1.30, 1.20), 3: (1.20, 1.05), 4: (1.10, 1.00), 5: (1.00, 1.00)}
STATIC_DE = {1: (1.35, 1.35), 2: (1.25, 1.15), 3: (1.15, 1.05), 4: (1.05, 1.00), 5: (1.00, 1.00)}
for n in STATIC_EN:
    for k, (symbol, i) in enumerate((("xi1", 0), ("xi2", 1)), start=1):
        ROWS.append(("xi.static.n%d.%d" % (n, k), "EN 1997-1 Table A.9 / DIN 1054 Table A 7.1", "%s, n = %s%d (static load tests)" % (symbol, "" if n < 5 else ">= ", n), STATIC_EN[n][i], STATIC_DE[n][i], "", "DE values recalled from DIN 1054:2010-12; DIN 1054:2021-04 revised Table A 7.1 not yet checked"))
GROUND = {1: (1.40, 1.40), 2: (1.35, 1.27), 3: (1.33, 1.23), 4: (1.31, 1.20), 5: (1.29, 1.15), 7: (1.27, 1.12), 10: (1.25, 1.08)}
for n, pair in GROUND.items():
    for k, symbol in ((3, "xi3"), (4, "xi4")):
        ROWS.append(("xi.ground.n%d.%d" % (n, k), "EN 1997-1 Table A.10", "%s, n = %d (ground-test profiles)" % (symbol, n), pair[k - 3], pair[k - 3], "", "EN recommended values; the German annex is assumed to adopt them"))
DYNAMIC = {2: (1.60, 1.50), 5: (1.50, 1.35), 10: (1.45, 1.30), 15: (1.42, 1.25), 20: (1.40, 1.25)}
for n, pair in DYNAMIC.items():
    for k, symbol in ((5, "xi5"), (6, "xi6")):
        ROWS.append(("xi.dynamic.n%d.%d" % (n, k), "EN 1997-1 Table A.11", "%s, n = %d (dynamic tests)" % (symbol, n), pair[k - 5], pair[k - 5], "", "EN recommended values; the German annex is assumed to adopt them"))

ROWS.append(("sls.angular-distortion-limit", "EN 1997-1 Annex H.2", "limit of the angular distortion / relative rotation", 0.002, 0.002, "", "1/500 as used by SLS-04; the Annex H limits are guidance values, DIN 1054:2021 tolerances not checked"))
ROWS.append(("sls.tilt-limit", "EN 1997-1 Annex H.2", "limit of the structure tilt", 0.004, 0.004, "", "1/250 recalled from Annex H.2; unverified"))



def entry(row):
    key, clause, symbol, rec, na, unit, note = row
    return {"id": PRE + key, "code": "EN 1998", "part": "", "clause": clause, "symbol": symbol, "recommended": rec, "naDe": na, "unit": unit,
            "source": {"document": DOC, "edition": "to be recorded at verification", "locator": clause}, "status": "unverified", "specRef": SPEC,
            "asWritten": "%s = %s as used by the artifact" % (symbol, rec), "evidence": note}


def main():
    path = NDP + "🔣️.json"
    text = open(path, encoding="utf-8").read()
    data = json.loads(text)
    assert json.dumps(data, indent=1, ensure_ascii=False) == text, "registry formatting changed"
    known = {e["id"] for e in data["entries"]}
    added = [entry(r) for r in ROWS if PRE + r[0] not in known]
    if not added:
        print("registry already complete")
        return
    data["entries"] = sorted(data["entries"] + added, key=lambda e: e["id"])
    open(path, "w", encoding="utf-8").write(json.dumps(data, indent=1, ensure_ascii=False))
    subprocess.run([sys.executable, "-I", CODEGEN, path, NDP + "🗃️table.rs"], check=True)
    print("added", len(added))


main()
