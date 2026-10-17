"""Summarises rtb-census-units.tsv: per crate the lib-test unit on each label, with the over-limit units listed first. usage: census_report.py [tsv]"""
import csv, sys, collections
from lib import *
path = sys.argv[1] if len(sys.argv) > 1 else GEN + "/rtb-census-units.tsv"
rows = list(csv.DictReader(open(path, encoding="utf8"), delimiter="\t"))
units = [r for r in rows if r["unit"] in ("lib-test", "integration", "bin-test")]
by = collections.defaultdict(dict)
for r in units:
    name = r["crate"].replace("semio-framework-os-renderer-wgpu", "~").replace("semio-framework-os-", "o-")
    by[name][r["label"]] = r
over = [(n, l, r) for n, d in by.items() for l, r in d.items() if r["over"] == "True"]
errs = [r for r in rows if r["error"]]
print(f"crates {len(by)}; test units over the gate: {len(over)}; error rows: {len(errs)}")
for n, l, r in sorted(over, key=lambda x: -float(x[2]["ratio"] or 0)):
    print(f"  OVER {n:36} {l:24} {r['unit']:9} {r['files']:>4} files {int(r['bytes']):>9} B {r['lines']:>6} lines ratio {r['ratio']} ({r['binding']})")
for r in errs[:10]:
    print("  ERR", r["crate"], r["label"], r["unit"], r["error"][:120])
print("--- all lib-test units, native / wasm32-unknown-unknown (files bytes lines ratio)")
for n in sorted(by):
    d = by[n]
    def f(l):
        r = d.get(l)
        return f"{r['files']:>3} {int(r['bytes']):>8} {r['lines']:>5} {r['ratio']:>6}" if r else "           -"
    print(f"{n:38} {f('native')} | {f('wasm32-unknown-unknown')}")
