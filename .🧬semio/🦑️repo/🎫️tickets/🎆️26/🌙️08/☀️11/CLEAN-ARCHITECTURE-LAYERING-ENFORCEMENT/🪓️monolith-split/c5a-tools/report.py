"""📊 C5A report tables: per file before -> root + children (largest), child line counts, final SHA256 list of every touched/created file. usage: report.py"""
import json, os, sys, hashlib, importlib
HERE = os.path.dirname(os.path.abspath(__file__)); sys.path.insert(0, HERE)
R = "/Users/ueli/Documents/semio/"
ft = dict(l.rstrip("\n").split("\t") for l in open(HERE + "/files.tsv", encoding="utf-8"))
order = ["c6", "c7", "ct", "tr1", "tt1", "tt2", "c1x", "atc", "c3", "c4", "c5", "tt", "tti", "tr", "tri", "rt"]
kids = {}
for t in order:
    if t in ("tt", "tti", "tr", "tri", "rt"):
        kids[t] = [c["folder"] for c in importlib.import_module("specs." + t).SPEC["children"]]
    else:
        kids[t] = json.load(open(f"{HERE}/staged-{t}.json", encoding="utf-8"))["folders"]
sha = []
tot_b = tot_a = nch = 0
for t in order:
    rel = ft[t]; d = os.path.dirname(rel)
    before = len(open(f"{HERE}/orig/{t}.rs", encoding="utf-8").read().split("\n")) - 1
    root = len(open(R + rel, encoding="utf-8").read().split("\n")) - 1
    sizes = [(f, len(open(R + d + "/" + f + "/🦀️.rs", encoding="utf-8").read().split("\n")) - 1) for f in kids[t]]
    mx = max(s for _, s in sizes)
    print(f"| {t} | `{rel.split('/🔨️modules/🔌️plugin/')[1]}` | {before:,} | {root} | {len(sizes)} | {mx} | {sum(s for _, s in sizes) + root:,} |")
    tot_b += before; tot_a += root + sum(s for _, s in sizes); nch += len(sizes)
    for p in [rel] + [d + "/" + f + "/🦀️.rs" for f in kids[t]]:
        sha.append(hashlib.sha256(open(R + p, "rb").read()).hexdigest() + "  " + p)
print("TOTAL", tot_b, "->", tot_a, "children", nch)
open(HERE + "/final-SHA256", "w", encoding="utf-8").write("\n".join(sha) + "\n")
