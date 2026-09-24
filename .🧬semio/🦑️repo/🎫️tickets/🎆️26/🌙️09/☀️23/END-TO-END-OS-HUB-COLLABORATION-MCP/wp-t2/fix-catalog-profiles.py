"""Binds every mutation catalog's standard/subset directory names to the owner path it is declared under."""
import json, sys, re
root = "/Users/ueli/Documents/semio/"
rows = json.load(open(sys.argv[1]))
paths = sorted({x["scope"] for x in rows if x["id"] == "contribution-manifest-invalid" and "catalog profile does not match" in x["summary"]})
for rel in paths:
    seg = rel.split("/")
    std = seg[seg.index("🏅️standards") + 1]; sub = seg[seg.index("🪆️subsets") + 1]
    text = open(root + rel, encoding="utf-8").read(); doc = json.loads(text)
    changed = 0
    for catalog in doc.get("mutationCatalogs", []):
        for key, value in (("standardDirectoryName", std), ("subsetDirectoryName", sub)):
            if catalog.get(key) != value: catalog[key] = value; changed += 1
    open(root + rel, "w", encoding="utf-8").write(json.dumps(doc, ensure_ascii=False, indent=2) + "\n")
    print(changed, rel)
