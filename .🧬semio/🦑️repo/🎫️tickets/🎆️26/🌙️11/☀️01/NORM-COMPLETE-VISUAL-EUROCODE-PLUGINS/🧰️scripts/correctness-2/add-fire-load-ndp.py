import json, sys, pathlib
p = pathlib.Path(sys.argv[1])
d = json.loads(p.read_text())
rows = {"dwelling": (948, 780), "hospital": (280, 230), "hotel": (377, 310), "library": (1824, 1500), "office": (511, 420), "school": (347, 285), "shopping": (730, 600), "theatre": (365, 300), "transport": (122, 100)}
have = {e["id"] for e in d["entries"]}
for k, (frac, mean) in rows.items():
    i = f"ndp.en1991.1-2.fire-load-fractile.{k}"
    if i in have: continue
    d["entries"].append({"id": i, "code": "EN 1991", "part": "1-2", "clause": "Annex E Table E.4", "symbol": f"q_f,k 80 % fractile {k}", "recommended": float(frac), "naDe": float(frac), "unit": "MJ/m2",
      "source": {"document": "DIN EN 1991/NA", "edition": "to be recorded at verification", "locator": "Annex E Table E.4"}, "status": "unverified",
      "specRef": "NORM-COMPLETE spec en1990-en1991-target E12.03 (values marked [V]; spec cites the table as E.1, the standard numbers it E.4); mean " + str(mean) + " MJ/m2",
      "asWritten": f"80 % fractile of the fire load density of occupancy {k} | 80 %-Fraktil der Brandlastdichte der Nutzung {k}", "evidence": ""})
d["entries"].sort(key=lambda e: e["id"])
p.write_text(json.dumps(d, indent=1, ensure_ascii=False))
