import json, glob
PSEUDO = {"typical-b-segment", "typical-bev-12m", "typical-20t-excavator", "soil-density", "typical-tower-crane", "typical-forklift-2t5", "typical-long-john"}
def walk(o, hits):
    if isinstance(o, dict):
        if o.get("reference") in PSEUDO:
            hits.append(o["reference"]); o.pop("reference")
            if o.get("source") == "literature": o["source"] = "authored-estimate"
        if o.get("reference") == "eu-2001-85" and "kg" in json.dumps(o)[:200] and False: pass
        for v in o.values(): walk(v, hits)
    elif isinstance(o, list):
        for v in o: walk(v, hits)
for f in sorted(glob.glob("src/*.json")):
    a = json.load(open(f)); hits = []
    refs = a["provenance"]["references"]
    a["provenance"]["references"] = [r for r in refs if r["id"] not in PSEUDO]
    walk(a, hits)
    note = {"en": " The class dimensions, masses and loads are authored typical values for this vehicle class (authored-estimate); they are not taken from a cited data sheet.", "de": " Abmessungen, Massen und Lasten sind selbst gesetzte typische Werte dieser Fahrzeugklasse (authored-estimate); sie stammen nicht aus einem zitierten Datenblatt."}
    n = a["provenance"]["notes"]
    if isinstance(n, dict):
        for k in ("en", "de"):
            if "authored typical values" not in n[k] and "selbst gesetzte typische" not in n[k]: n[k] = n[k].rstrip() + note[k]
    if a["id"] == "city-bus":
        for r in a["provenance"]["references"]:
            if r["id"] == "eu-2001-85":
                r["citation"] = "Directive 2001/85/EC of the European Parliament and of the Council relating to special provisions for vehicles used for the carriage of passengers comprising more than eight seats in addition to the driver's seat (standing area per passenger; repealed by Regulation (EU) 2018/858, with effect from 1 September 2020). The 68 kg design passenger mass stems from Annex I of Directive 97/27/EC (also repealed by Regulation (EU) 2018/858); both values are used here as authored estimates guided by these texts."
    json.dump(a, open(f, "w"), ensure_ascii=False, indent=2)
    print(a["id"], hits[:3], len(hits), [r["id"] for r in a["provenance"]["references"]])
