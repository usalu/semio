"""🧮️ One line per asset of a matrix run folder: cells by status and the failing cells by code, view and scale entry: cells.py <run folder>"""
import collections, glob, json, sys
for path in sorted(glob.glob(f"{sys.argv[1]}/cells/*.json")):
    record = json.load(open(path))
    status = collections.Counter(c["status"] for c in record["cells"])
    codes = collections.Counter()
    where = collections.defaultdict(set)
    for c in record["cells"]:
        if c["status"] == "fail":
            for code in c["failures"]:
                codes[code] += 1
                where[code].add(f"{c['scale']} {c['view']}")
    print(f"{record['asset']}: {len(record['cells'])} cells, {dict(status)}, {record['seconds']} s, outcome {record['outcome']}" + "".join(f"\n   {code} {n}: {', '.join(sorted(where[code]))[:420]}" for code, n in codes.items()))
