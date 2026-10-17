"""📊️ Markdown table of the lib-test unit of every censused crate on the three labels. usage: census_table.py [tsv]"""
import csv, sys, collections
from lib import *
path = sys.argv[1] if len(sys.argv) > 1 else GEN + "/rtb-census-units.tsv"
rows = [r for r in csv.DictReader(open(path, encoding="utf8"), delimiter="\t") if r["unit"] == "lib-test"]
by = collections.defaultdict(dict)
for r in rows:
    by[r["crate"]][r["label"]] = r
labels = ["native", "wasm32-unknown-unknown", "wasm32-wasip2"]
print("| crate | native | wasm32-unknown-unknown | wasm32-wasip2 |")
print("|---|---|---|---|")
for c in sorted(by, key=lambda c: (c != "semio-framework-os-renderer-wgpu", c)):
    name = "`" + c.replace("semio-framework-os-renderer-wgpu", "~") + "`"
    cells = []
    for l in labels:
        r = by[c].get(l)
        cells.append(f"{r['files']} / {int(r['bytes']):,} B / {int(r['lines']):,} L ({float(r['ratio']):.2f})" if r else "n/a")
    print(f"| {name} | " + " | ".join(cells) + " |")
