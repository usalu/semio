# 📊️ Numbers for RESULT-RENDERER-WP5.md: sizes before and after, methods per crate, bridges, consumer rewrites, moves.
import os, re, sys, json, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import cfg, measure, deps, ctx
a = measure.measure(cfg.ORIG); b = measure.measure(cfg.NEW)
ns_o = deps.load(cfg.ORIG); ns_n = deps.load(cfg.NEW)
def mc(ns): 
    c = collections.Counter(); l = collections.Counter(); f = collections.Counter()
    for n in ns:
        if n.kind == "method": c[n.crate] += 1
        else: f[n.crate] += 1
    return c, f
mo, fo = mc(ns_o); mn, fn = mc(ns_n)
print("| crate | files before | lines before | bytes before | methods before | files after | lines after | bytes after | methods after | free items after |\n|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|")
for _, c, _ in cfg.CRATES:
    print(f"| `{c}` | {a[c][0]} | {a[c][1]} | {a[c][2]} | {mo[c]} | {b[c][0]} | {b[c][1]} | {b[c][2]} | {mn[c]} | {fn[c]} |")
rep = json.load(open(cfg.GEN + "/build-report.json"))
print(rep["report"]); print(len(rep["ext"]), "external files")
bridged = collections.Counter()
for crate in ctx.BRIDGE:
    for slug, p in ctx.crate_files(cfg.NEW, crate):
        for m in re.finditer(r"self\.(as_\w+)\(\)", open(p, encoding="utf-8").read()): bridged[(crate, m.group(1))] += 1
for k, v in sorted(bridged.items()): print(k, v)
print(sum(bridged.values()))
