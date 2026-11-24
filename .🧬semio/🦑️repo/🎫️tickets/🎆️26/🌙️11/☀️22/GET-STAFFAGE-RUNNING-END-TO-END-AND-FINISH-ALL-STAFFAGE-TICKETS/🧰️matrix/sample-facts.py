"""🔎 Facts of the sample matrices beyond the code tables: failing cells by scale kind, by code family, renders per asset against seconds, and the cells that stay failing when a set of codes is taken out. usage: python3 sample-facts.py <matrix-dir>"""
import collections
import glob
import json
import os
import sys

root = sys.argv[1]
cells = []
for folder in sorted(glob.glob(root + "/*/")):
    if os.path.exists(folder + "matrix.json"):
        cells += json.load(open(folder + "matrix.json"))["cells"]
    else:
        for shard in glob.glob(folder + "shard-*.json"):
            cells += json.load(open(shard))
seconds = {}
for line in open(root + "/times.tsv"):
    name, index, ids, code, took, total = line.rstrip("\n").split("\t")
    seconds[ids.split(",")[int(index)]] = int(took) if int(took) > 0 else None
family = lambda code: "sync" if code.startswith("sync") else code
print("cells", len(cells), "assets", len({c["asset"] for c in cells}))
kinds = collections.Counter((c["scaleKind"], c["status"]) for c in cells)
for kind in ("preset", "band-min", "band-max", "alternative"):
    total = sum(n for (k, _), n in kinds.items() if k == kind)
    print(f"{kind}: cells {total}, fail {kinds[(kind, 'fail')]}, ok {kinds[(kind, 'ok')]}, unavailable {kinds[(kind, 'unavailable')]}, group-drift {kinds[(kind, 'group-drift')]}, range-end {kinds[(kind, 'range-end')]}")
fam_cells = collections.Counter()
fam_assets = collections.defaultdict(set)
combos = collections.Counter()
for c in cells:
    fams = sorted({family(code) for code in c["failures"]})
    for f in fams:
        fam_cells[f] += 1
        fam_assets[f].add(c["asset"])
    if fams:
        combos[" + ".join(fams)] += 1
print("\nfamily\tcells\tassets")
for f, n in fam_cells.most_common():
    print(f"{f}\t{n}\t{len(fam_assets[f])}")
print("\ncells per family combination")
for combo, n in combos.most_common():
    print(f"{n}\t{combo}")
failing_assets = {c["asset"] for c in cells if c["status"] == "fail"}
print("\nassets with a failing cell:", len(failing_assets), "of", len({c["asset"] for c in cells}))
def remaining(drop):
    left = [c for c in cells if any(family(code) not in drop for code in c["failures"])]
    return len(left), len({c["asset"] for c in left})
for drop in (set(), {"paper-width"}, {"paper-width", "analysis-cue"}, {"paper-width", "analysis-cue", "derived-view-band-copy"}, {"paper-width", "analysis-cue", "derived-view-band-copy", "anchor-landmark"}, {"paper-width", "analysis-cue", "derived-view-band-copy", "sync"}):
    print("without", sorted(drop) or "nothing", "-> failing cells, assets:", remaining(drop))
print("\nasset\tcategory\tcells\tfail\tsum of checked renders\tseconds\tcategory")
per = collections.defaultdict(lambda: [0, 0, 0, ""])
for c in cells:
    row = per[c["asset"]]
    row[0] += 1
    row[1] += c["status"] == "fail"
    row[2] += c["checked"]
    row[3] = c["category"]
for asset, (n, fail, checked, category) in sorted(per.items(), key=lambda item: (item[1][3], item[0])):
    print(f"{asset}\t{category}\t{n}\t{fail}\t{checked}\t{seconds.get(asset)}")
done = [(checked, seconds[a]) for a, (n, fail, checked, category) in per.items() if seconds.get(a)]
print("renders per second over the finished assets: %.1f" % (sum(c for c, _ in done) / sum(s for _, s in done)))
print("seconds:", {a: s for a, s in seconds.items()})
