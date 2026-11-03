import json, sys, pathlib
p = pathlib.Path(sys.argv[1])
d = json.loads(p.read_text())
rows = [("gravel", 400000.0, "Pa"), ("sand", 200000.0, "Pa"), ("silt", 150000.0, "Pa"), ("clay", 120000.0, "Pa"), ("other", 180000.0, "Pa"), ("embedment-gain", 0.1, "1/m"), ("embedment-cap", 3.0, "m")]
have = {e["id"] for e in d["entries"]}
for k, v, u in rows:
    i = f"ndp.en1997.gk1.presumptive.{k}"
    if i in have: continue
    d["entries"].append({"id": i, "code": "EN 1997", "part": "1", "clause": "DIN 1054 Table A.6.10", "symbol": f"GK1 presumptive bearing {k}", "recommended": v, "naDe": v, "unit": u,
      "source": {"document": "DIN EN 1997/NA", "edition": "to be recorded at verification", "locator": "DIN 1054 Table A.6.10"}, "status": "unverified",
      "specRef": "wave2 correctness-2: value as used by the artifact (presumptive_bearing_pa); the specs hold no table, not verified against DIN 1054",
      "asWritten": f"Presumptive design bearing pressure of geotechnical category 1, {k} | Aufnehmbarer Sohldruck GK 1, {k}", "evidence": ""})
d["entries"].sort(key=lambda e: e["id"])
p.write_text(json.dumps(d, indent=1, ensure_ascii=False))
