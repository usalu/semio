import collections, json, pathlib
G = pathlib.Path("../🗑️generated/catalogue-bridge")
out = []
for code in ["en1997", "en1990", "en1996", "en1991", "en1992"]:
    rows = json.load(open(G / f"{code}-inputs" / "🔣️.json", encoding="utf-8"))["inputs"]
    conf = collections.Counter(r.get("confidence", "high") for r in rows)
    out.append(f"### {code.upper().replace('EN', 'EN ')}: {len(rows)} rows (high {conf['high']}, medium {conf['medium']}, low {conf['low']})")
    for level in ("low", "medium"):
        by = collections.defaultdict(list)
        for r in rows:
            if r.get("confidence", "high") == level:
                tag = r["key"] + ("@" + r["scope"] if r["scope"] else "")
                by[r.get("unit", r["kind"])].append(tag)
        if by:
            out.append(f"- {level} confidence, by stored unit:")
            for unit, keys in sorted(by.items()):
                out.append(f"  - `{unit}`: " + " ".join(f"`{k}`" for k in keys))
open("../🗑️generated/catalogue-bridge/review.md", "w", encoding="utf-8").write("\n".join(out) + "\n")
print(len(out), "lines")
