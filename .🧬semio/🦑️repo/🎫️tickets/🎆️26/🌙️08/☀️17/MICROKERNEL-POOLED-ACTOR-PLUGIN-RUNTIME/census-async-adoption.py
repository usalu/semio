#!/usr/bin/env python3
"""📊️ Async-adoption census across the plugin fleet.

Shell grep silently under-reports on this repo's emoji paths (rule 21), so this walks
with python over explicit absolute paths. Run before and after the wave; the headline
claim is that the `host::*` / jobs / tasks columns move off zero while block_on and
pending_effects reach zero.
"""
import os, re, sys, json
ROOT = "/Users/ueli/Documents/semio"
PLUG = os.path.join(ROOT, "✏️s", "🔌️plugins")
PATS = {
    "host_calls":      re.compile(r'\bhost\(\)\.\w+|\bhost::\w+|ctx\.host\(\)\.'),
    "async_fn":        re.compile(r'\basync fn\b'),
    "await":           re.compile(r'\.await\b'),
    "block_on":        re.compile(r'\bblock_on\s*\('),
    "pending_effects": re.compile(r'\bfn pending_effects\b'),
    "job_reg":         re.compile(r'register_job_kind|\.job\s*\('),
    "async_task":      re.compile(r'\bAsyncTask\b|Emit::task\b|\.with_task\('),
    "dl_export":       re.compile(r'DownloadMediaExport'),
}
rows = {}
for plugin in sorted(os.listdir(PLUG)):
    d = os.path.join(PLUG, plugin)
    if not os.path.isdir(d): continue
    c = dict.fromkeys(PATS, 0); c["rs_files"] = 0
    for dp, dn, fs in os.walk(d):
        dn[:] = [x for x in dn if "target" not in x and x != "node_modules"]
        for f in fs:
            if not f.endswith(".rs"): continue
            c["rs_files"] += 1
            try: t = open(os.path.join(dp, f), encoding="utf-8", errors="replace").read()
            except OSError: continue
            for k, p in PATS.items(): c[k] += len(p.findall(t))
    rows[plugin] = c
cols = ["rs_files","host_calls","async_fn","await","block_on","pending_effects","job_reg","async_task","dl_export"]
w = max(len(p) for p in rows) + 1
print(f"{'plugin':<{w}}" + "".join(f"{c:>16}" for c in cols))
tot = dict.fromkeys(cols, 0)
for p, c in rows.items():
    print(f"{p:<{w}}" + "".join(f"{c[k]:>16}" for k in cols))
    for k in cols: tot[k] += c[k]
print(f"{'TOTAL':<{w}}" + "".join(f"{tot[k]:>16}" for k in cols))
if "--json" in sys.argv:
    open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "🔣️census-async.json"), "w").write(json.dumps({"rows":rows,"total":tot}, indent=1))
