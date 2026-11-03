import json, pathlib, re, sys
REG = pathlib.Path(sys.argv[1])
d = json.loads(REG.read_text(encoding="utf-8"))
down = moved = 0
for e in d["entries"]:
    if e["code"] == "EN 1998" and e["id"].startswith("ndp.en1997."):
        e["code"] = "EN 1997"
        moved += 1
    if e["status"] != "verified" or e["code"] != "EN 1995":
        continue
    m = re.search(r"(-?\d+(?:\.\d+)?)\s*\(spec value used\)$", e["asWritten"]) or re.search(r"as used by the artifact: (-?[\d.]+)$", e["asWritten"])
    if not m or float(m.group(1)) == e["recommended"]:
        continue
    spec = m.group(1)
    e["status"] = "unverified"
    e["asWritten"] = f"{e['symbol']} = {e['recommended']:g} in the registry; the earlier spec value {spec} is not reconciled with it"
    e["evidence"] = f"registry-integrity 2026-11-03: downgraded to unverified because the registry value {e['recommended']:g} and the earlier spec value {spec} disagree and the printed {e['source']['document']} was not available to settle it | " + e["evidence"]
    down += 1
REG.write_text(json.dumps(d, indent=1, ensure_ascii=False), encoding="utf-8")
print("downgraded", down, "recoded", moved)
