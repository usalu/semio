import json, sys

dups = []

def hook(pairs):
    keys = [k for k, _ in pairs]
    for k in set(keys):
        if keys.count(k) > 1:
            dups.append(k)
    return dict(pairs)

for p in sys.argv[1:]:
    dups.clear()
    with open(p, encoding="utf-8") as f:
        json.load(f, object_pairs_hook=hook)
    print(p.split("/")[-2], "duplicate keys:", sorted(set(dups)))
