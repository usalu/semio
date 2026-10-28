"""🧮 Counts the witness rows of a live plan receipt by kind and splits the path rows into snapshot reads (S), source classes (l only) and other; read-only, aborts above 500 MB RSS.
usage: python3 -I receipt-kinds.py <plan.receipt.json>..."""
import json, resource, sys, collections
for path in sys.argv[1:]:
    receipt = json.load(open(path, encoding="utf8"))
    rows = receipt["witness"]["observations"]
    if resource.getrusage(resource.RUSAGE_SELF).ru_maxrss > 500 * 1024 * 1024: sys.exit("RSS above 500 MB")
    kinds = collections.Counter(row[0] for row in rows)
    by = collections.defaultdict(set)
    for kind, subject, _ in rows: by[kind].add(subject)
    lonly = by["l"] - by["S"]
    print(path.split("/workspaces/")[-1], len(rows), dict(kinds.most_common()), "S:", len(by["S"]), "l-not-S:", len(lonly), "f-not-S:", len(by["f"] - by["S"]), "outputs:", len(receipt["outputs"]))
    print("  sample l-not-S:", sorted(lonly)[:3])
    print("  sample f-not-S:", sorted(by["f"] - by["S"])[:5])
