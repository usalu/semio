"""🧾 OHF undeclared-head triage: for rows of a harvest TSV (E0432/E0433 'cannot find module or crate `X`' / 'unresolved import `X`'), reports per crate the head names that are neither declared in its Cargo.toml (any table, key or package ident) nor defined as `extern crate`/`mod`/`macro_rules`/`use … as` in the crate's sources. Read only.
usage: undecl.py <family> <label>"""
import collections, json, os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE + "/pos")
from poslib import *
fam, label = sys.argv[1:3]
m = Manifests(Source(None)); m.build_index()
rows = collections.defaultdict(set)
for l in open(f"{GEN}/ohf-rows-{fam}-{label}.tsv", encoding="utf-8"):
    x = l.rstrip("\n").split("\t")
    mm = re.match(r"cannot find module or crate `(\w+)` in this scope|unresolved import `(\w+)`$", x[5])
    if mm:
        rows[x[2]].add((mm.group(1) or mm.group(2), x[3], x[4]))
out = []
for pkg_ident, hs in sorted(rows.items()):
    pkg = next((n for n in m.by_name if n.replace("-", "_") == pkg_ident), None)
    if not pkg:
        continue
    man = m.manifest(pkg)
    declared = set()
    for d in m.deps(man):
        declared.add(d[1]); declared.add(d[1].replace("-", "_")); declared.add(d[2].replace("-", "_"))
    for h, f, ln in sorted(hs):
        if h in declared or h in ("crate", "self", "super", "std", "core", "alloc"):
            continue
        try:
            txt = open("/Users/ueli/Documents/semio/" + os.path.normpath(f), encoding="utf-8").read()
        except Exception:
            txt = ""
        if re.search(r"(\buse\b[^;]*\b%s\b|\bmod\s+%s\b|\bextern\s+crate\b[^;]*\b%s\b|\blet\s+%s\b)" % ((re.escape(h),) * 4), txt):
            continue
        out.append(f"{pkg} | {h} | {f.split('/')[-2]}:{ln}")
print("\n".join(out))
