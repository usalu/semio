import json, re, sys
path = sys.argv[1]
doc = json.load(open(path, encoding="utf8"))
for case in doc["search"]:
    flags = 0 if case["match_case"] else re.IGNORECASE
    hits = []
    if case["query"]:
        for page, spec in enumerate(doc["pages"]):
            for text in spec["texts"]:
                hits += [[page, m.start()] for m in re.finditer("(?=" + re.escape(case["query"]) + ")", text, flags)]
    assert hits == case["hits"], (case["query"], hits, case["hits"])
print("oracle agrees with", path)
