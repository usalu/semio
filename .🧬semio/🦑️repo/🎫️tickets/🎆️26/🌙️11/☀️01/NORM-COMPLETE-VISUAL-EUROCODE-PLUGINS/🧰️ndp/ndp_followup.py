"""NDP follow-up: adopt EN 338:2016 values from the third-party library for conflicting strength rows, name the anchorage type in the EN 1997 R-set rows.

Run: /home/deck/semio/.venv/bin/python -I ndp_followup.py REGISTRY.json
Idempotent. Never touches a row without a library fact; values are copied from the library, never typed.
"""
import json, pathlib, re, sys
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
import verify

EDITION = "EN 338:2016"
registry = pathlib.Path(sys.argv[1])
doc = json.loads(registry.read_text())
adopted = labelled = retitled = carried = 0
for e in doc["entries"]:
    m = re.fullmatch(r"ndp\.en1995\.strength\.([cd]\d+)\.\w+", e["id"])
    if m and verify.FACTS.get(e["id"]):
        table = "1" if m.group(1)[0] == "c" else "3"
        fact = verify.FACTS[e["id"]][0]
        want = verify.norm(fact["value"], fact["unit"]) / verify.UNIT.get(e["unit"], 1.0)
        if e["status"] == "unverified" and not verify.same(verify.norm(e["recommended"], e["unit"]), verify.norm(fact["value"], fact["unit"])):
            old = e["recommended"]
            e["recommended"] = e["naDe"] = float(want)
            e["evidence"] = (e["evidence"] + " | " if e["evidence"] else "") + f"ndp-followup 2026-11-03: value replaced, registry held {old} (EN 338:2009-era transcription), EN 338:2016 value {want:g} {e['unit']} from {fact['cite']}"
            adopted += 1
        if e["status"] == "unverified":
            e["status"] = "verified"
            e["evidence"] = (e["evidence"] + " | " if e["evidence"] else "") + f"ndp-followup 2026-11-03: VERIFIED by {fact['cite']}"
        e["clause"] = e["source"]["locator"] = f"Table {table}"
        e["source"]["document"] = f"{EDITION} Table {table}"
        e["source"]["edition"] = EDITION
        retitled += 1
    r = re.fullmatch(r"ndp\.en1997\.(r[1-4])\.a", e["id"])
    if r and "prestressed anchorage" not in e["symbol"]:
        e["symbol"] = f"γ_a;t {r.group(1).upper()} (prestressed anchorage, temporary)"
        labelled += 1
for e in doc["entries"]:
    if e["status"] == "verified" and not e["evidence"].strip():
        e["evidence"] = "ndp-followup 2026-11-03: CARRIED-OVER FLAG, no independent source. Marked verified in the code before wave 2 without a recorded source; no third-party library holds EN 1995-1-1 Tables 3.1/3.2 or the DIN NA values. Needs the print or a downgrade to unverified (coordinator decision, it turns dependent verdicts into UnverifiedBasis)."
        carried += 1
registry.write_text(json.dumps(doc, ensure_ascii=False, indent=1))
print("values replaced", adopted, "strength rows normalised", retitled, "anchorage rows labelled", labelled, "carried-over flags annotated", carried)
