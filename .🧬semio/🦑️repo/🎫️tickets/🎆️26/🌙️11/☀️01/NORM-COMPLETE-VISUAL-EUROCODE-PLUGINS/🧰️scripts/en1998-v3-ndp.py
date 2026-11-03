"""EN 1998 v3: reconciles the Table F.1 rows of the shared NDP registry with the target spec (`📐️specs/en1997-en1998-target.md`, GE-06, line 559), idempotently; all rows stay `unverified`.
Usage: /home/deck/semio/.venv/bin/python -I en1998-v3-ndp.py"""
import json, subprocess, sys

NDP = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/⚖️compliance/📜️annex-and-tables/🔖️ndp/"
CODEGEN = "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS/🧰️ndp/codegen.py"
PRE = "ndp.en1998.en5.annex-f.sand."
SPEC = "📐️specs/en1997-en1998-target.md line 559 (GE-06): a 0.92, b 1.25, c 0.92, d 1.25, e 0.92, f 0.41, m 0.96, k 0.39, k' 0.39, c_T 2.14, c'_T 1.81, beta 2.57, gamma 4.0 (informative, [V])"
SET = {"e": 0.92, "f": 0.41, "k": 0.39, "beta": 2.57, "gamma": 4.0}
SAME = {"a": 0.92, "b": 1.25, "c": 0.92, "d": 1.25, "m": 0.96, "k-prime": 0.39, "c-t": 2.14, "c-t-prime": 1.81}
ABSENT = {"c-m": 2.57, "c-m-prime": 1.85}
EVIDENCE = {
    "set": "spec GE-06 line 559 lists %s; the v2 recollection was %s and is dropped",
    "same": "spec GE-06 line 559 lists %s (agrees)",
    "absent": "NOT in the spec (GE-06 line 559 lists no c_M or c'_M): recollection %s kept, unverified, to be replaced from the licensed Table F.1",
}
OLD = {"e": 1.0, "f": 1.0, "k": 1.0, "beta": 2.9, "gamma": 2.8}


PRE_ALL = "ndp.en1998."
DOC = "DIN EN 1998-1/NA:2023-11"
NEW = [
    ("en1.diaphragm.transfer-factor", "EN 1998-1 4.4.2.5", "γ_d transfer amplification", 1.3, 1.3, "", "spec SEI-46 line 462 and SEI-48: diaphragm forces amplified by 1.3 where they transfer between vertical elements, marked [V]"),
    ("en1.importance.nu-i-ii", "EN 1998-1 4.4.3.2", "ν importance I, II", 0.5, 0.5, "", "spec SEI-11: recommended 0.5 for importance I and II, NA-DE 0.5 [V]"),
    ("en1.importance.nu-iii-iv", "EN 1998-1 4.4.3.2", "ν importance III, IV", 0.4, 0.4, "", "spec SEI-11: recommended 0.4 for importance III and IV, NA-DE 0.4 [V]"),
    ("en1.time-history.min-records", "EN 1998-1 3.2.3.1.2", "minimum number of accelerograms", 3.0, 3.0, "", "spec SEI-06 line 374: at least 3 accelerograms"),
    ("en1.time-history.mean-spectrum-ratio", "EN 1998-1 3.2.3.1.2", "mean spectrum over elastic spectrum", 0.9, 0.9, "", "spec SEI-06 line 374: mean Se >= 0.9 in the range 0.2 T1 to 2 T1"),
]


def make_entry(row):
    key, clause, symbol, rec, na, unit, note = row
    return {"id": PRE_ALL + key, "code": "EN 1998", "part": "", "clause": clause, "symbol": symbol, "recommended": rec, "naDe": na, "unit": unit,
            "source": {"document": DOC, "edition": "to be recorded at verification", "locator": clause}, "status": "unverified", "specRef": "📐️specs/en1997-en1998-target.md SEI items",
            "asWritten": "%s = %s as used by the artifact" % (symbol, rec), "evidence": note}


def main():
    path = NDP + "🔣️.json"
    text = open(path, encoding="utf-8").read()
    data = json.loads(text)
    assert json.dumps(data, indent=1, ensure_ascii=False) == text, "registry formatting changed"
    changed = 0
    known = {e["id"] for e in data["entries"]}
    added = [make_entry(r) for r in NEW if PRE_ALL + r[0] not in known]
    if added:
        data["entries"] = sorted(data["entries"] + added, key=lambda e: e["id"])
        changed += len(added)
    for entry in data["entries"]:
        if not entry["id"].startswith(PRE):
            continue
        key = entry["id"][len(PRE):]
        if key in SET:
            value, note = SET[key], EVIDENCE["set"] % (SET[key], OLD[key])
        elif key in SAME:
            value, note = SAME[key], EVIDENCE["same"] % SAME[key]
        elif key in ABSENT:
            value, note = ABSENT[key], EVIDENCE["absent"] % ABSENT[key]
        else:
            continue
        new = dict(entry, recommended=value, naDe=value, evidence=note, specRef=SPEC, status="unverified")
        if new != entry:
            entry.update(new)
            changed += 1
    if not changed:
        print("Table F.1 already reconciled")
        return
    open(path, "w", encoding="utf-8").write(json.dumps(data, indent=1, ensure_ascii=False))
    subprocess.run([sys.executable, "-I", CODEGEN, path, NDP + "🗃️table.rs"], check=True)
    print("updated", changed)


main()
