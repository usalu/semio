"""🧮 Crosses the sample matrices with the whole-catalog scan: which codes the scan (poses only) finds against the matrix per sample asset, the cells a change of an expectation would clear in the sample, the assets left for data work in the catalog, and the extrapolation of the cells per family. usage: python3 cross.py <matrix-dir> <scan.jsonl> <classify.txt>"""
import collections
import glob
import json
import os
import sys

root, scan_path, classify = sys.argv[1:4]
cells = []
for folder in sorted(glob.glob(root + "/*/")):
    if os.path.exists(folder + "matrix.json"):
        cells += json.load(open(folder + "matrix.json"))["cells"]
    else:
        for shard in glob.glob(folder + "shard-*.json"):
            cells += json.load(open(shard))
scan = {row["id"]: row for row in (json.loads(line) for line in open(scan_path) if line.strip())}
family = lambda code: "sync" if code.startswith("sync") else code

def scan_codes(row):
    codes = {code for counts in row["geometry"].values() for code in counts if not code.startswith("group-drift")}
    if any(f["against"].startswith("1:") for f in row["derived"]):
        codes.add("derived-view-band-copy")
    if any(not f["against"].startswith("1:") for f in row["derived"]):
        codes.add("derived-view-in-band")
    if any("unavailable" not in c for c in row["cue"]):
        codes.add("analysis-cue")
    return codes

def anchors_all_descendant(row):
    classes = row.get("anchorClasses") or {}
    return bool(classes) and all(key.endswith("on a descendant bone") for key in classes)

def sync_all_other_id(row):
    classes = {k: v for k, v in (row.get("syncClasses") or {}).items() if not k.startswith("group")}
    extent = any(code in ("sync-forward", "sync-height", "sync-lateral") for counts in row["geometry"].values() for code in counts)
    return bool(classes) and all(key.endswith("other shape id") for key in classes) and not extent

matrix_codes = collections.defaultdict(set)
for c in cells:
    matrix_codes[c["asset"]] |= set(c["failures"])
    matrix_codes[c["asset"]] |= set()
print("asset\tmatrix codes not seen by the scan\tscan codes not in the matrix")
missed = 0
for asset in sorted(matrix_codes):
    m, s = {family(x) for x in matrix_codes[asset]}, {family(x) for x in scan_codes(scan[asset])}
    if m - s or s - m:
        print(f"{asset}\t{sorted(m - s)}\t{sorted(s - m)}")
    missed += bool(m) and not s
print("sample assets failing in the matrix but clean in the scan:", missed)

cleared = collections.Counter()
left = 0
left_assets = set()
for c in cells:
    if c["status"] != "fail":
        continue
    row = scan[c["asset"]]
    remaining = set()
    for code in c["failures"]:
        f = family(code)
        if f == "paper-width":
            continue
        if f == "analysis-cue":
            continue
        if f == "anchor-landmark" and anchors_all_descendant(row):
            continue
        if f == "sync" and code.startswith("sync-bone") and sync_all_other_id(row):
            continue
        remaining.add(f)
    if remaining:
        left += 1
        left_assets.add(c["asset"])
    else:
        cleared[c["asset"]] += 1
print("\nsample: failing cells", sum(c["status"] == "fail" for c in cells), "-> cleared by the four expectation changes", sum(cleared.values()), dict(cleared), "-> left", left, "in", len(left_assets), "assets")

categories = collections.Counter(row["category"] for row in scan.values())
def count(test):
    hit = [row for row in scan.values() if test(row)]
    per = collections.Counter(row["category"] for row in hit)
    return len(hit), " ".join(f"{category} {per[category]}" for category in sorted(categories) if per[category])
has_sync = lambda row: any(code.startswith("sync") for code in scan_codes(row))
has_anchor = lambda row: "anchor-landmark" in scan_codes(row)
print("\ncatalog (scan, poses only)")
print("sync findings:", count(has_sync))
print("sync findings, all of other shape id (cleared by a twin rule):", count(lambda row: has_sync(row) and sync_all_other_id(row)))
print("sync findings left for drawing work:", count(lambda row: has_sync(row) and not sync_all_other_id(row)))
print("  of them with a finding already at the rest pose:", count(lambda row: has_sync(row) and not sync_all_other_id(row) and any(code.startswith("sync") for code in (row.get("rest") or {}))))
print("  of them only in other poses:", count(lambda row: has_sync(row) and not sync_all_other_id(row) and not any(code.startswith("sync") for code in (row.get("rest") or {}))))
print("anchor findings:", count(has_anchor))
print("anchor findings, all on a descendant bone (cleared by a subtree rule):", count(lambda row: has_anchor(row) and anchors_all_descendant(row)))
print("anchor findings left for data work:", count(lambda row: has_anchor(row) and not anchors_all_descendant(row)))
print("  of them without any sync finding (anchor data alone):", count(lambda row: has_anchor(row) and not anchors_all_descendant(row) and not (has_sync(row) and not sync_all_other_id(row))))
print("  of them with an anchor off every drawn shape:", count(lambda row: has_anchor(row) and any(key.endswith("off every drawn shape") for key in row["anchorClasses"])))
print("  of them with a landmark anchor that has no z among the findings:", count(lambda row: has_anchor(row) and any("without z" in key and not key.endswith("on a descendant bone") for key in row["anchorClasses"])))
needs_drawing = lambda row: (has_sync(row) and not sync_all_other_id(row)) or "derived-view-band-copy" in scan_codes(row)
needs_data = lambda row: needs_drawing(row) or (has_anchor(row) and not anchors_all_descendant(row)) or "derived-view-in-band" in scan_codes(row)
print("band copies:", count(lambda row: "derived-view-band-copy" in scan_codes(row)))
print("in-band derived:", count(lambda row: "derived-view-in-band" in scan_codes(row)))
print("assets needing a drawing change (sync left or band copy):", count(needs_drawing))
print("assets needing any asset or fixture data change:", count(needs_data))
print("assets with a hard finding today:", count(lambda row: bool(scan_codes(row))))
print("assets clean after the expectation changes alone:", count(lambda row: not needs_data(row)))

universe = {}
section = None
for line in open(classify):
    line = line.rstrip("\n")
    if line.startswith("## "):
        section = line[3:]
        continue
    if section == "catalog universe" and line and not line.startswith("total"):
        category, assets, n = line.split("\t")
        universe[category] = int(n)
sample = collections.Counter(c["category"] for c in cells)
fam = collections.defaultdict(collections.Counter)
only = collections.defaultdict(collections.Counter)
for c in cells:
    fams = {family(code) for code in c["failures"]}
    for f in fams:
        fam[f][c["category"]] += 1
    if len(fams) == 1:
        only[next(iter(fams))][c["category"]] += 1
print("\nfamily\tsample cells\tsample cells with this family alone\textrapolated cells\textrapolated alone")
for f in sorted(fam, key=lambda f: -sum(fam[f].values())):
    est = sum(n / sample[cat] * universe[cat] for cat, n in fam[f].items())
    est_only = sum(n / sample[cat] * universe[cat] for cat, n in only[f].items())
    print(f"{f}\t{sum(fam[f].values())}\t{sum(only[f].values())}\t{est:.0f}\t{est_only:.0f}")
fail = collections.Counter(c["category"] for c in cells if c["status"] == "fail")
print("category\tsample cells\tsample fail\trate\tuniverse\textrapolated fail")
total = 0
for cat in sorted(sample):
    est = fail[cat] / sample[cat] * universe[cat]
    total += est
    print(f"{cat}\t{sample[cat]}\t{fail[cat]}\t{fail[cat] / sample[cat]:.2f}\t{universe[cat]}\t{est:.0f}")
print("total", sum(universe.values()), round(total))
