"""🧹 Reports unused imports of the union sandbox (analysis only; the crate-wide allow of the old root is gone)."""
import json, os, subprocess, sys, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from rlib import *
import roracle as o
glue = SP + "/S1/" + FACADE_ROOT
def warnings(extra):
    cmd = ["rustc", o.TOOLCHAIN, "--edition", "2021", "--crate-type", "lib", "--crate-name", "protocol", "--emit=metadata", "-o", "/dev/null", "--error-format=json", "-W", "unused_imports", *extra, "--cfg", 'feature="deflate"', glue]
    p = subprocess.run(cmd, cwd=os.path.dirname(glue), capture_output=True, text=True)
    out = []
    for line in p.stderr.splitlines():
        try: d = json.loads(line)
        except Exception: continue
        if d.get("level") != "warning" or (d.get("code") or {}).get("code") != "unused_imports": continue
        sp = next((s for s in d["spans"] if s["is_primary"]), None)
        if not sp: continue
        f = sp["file_name"].split("/📡️replication/")[-1]
        out.append((f, sp["line_start"], d["message"][:100]))
    return out
for label, extra in (("native", []), ("test", ["--test"])):
    w = warnings(extra)
    c = collections.Counter(f for f, l, m in w)
    print(label, len(w), "unused-import warnings in", len(c), "files")
    for f, l, m in w[:60]: print("   ", f, l, m)
