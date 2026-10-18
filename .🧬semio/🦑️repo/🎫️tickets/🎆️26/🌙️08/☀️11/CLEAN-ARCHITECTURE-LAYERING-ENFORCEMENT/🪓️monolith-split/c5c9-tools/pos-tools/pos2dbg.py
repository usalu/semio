"""🩺 POS2 debug runner: re-runs rustc on an existing family sandbox (`pos2-work/<family>/<side>`) with extra flags, writes stderr to GEN/pos2-<family>-<tag>.txt.
usage: pos2dbg.py <family.json> <side> <cfg> <tag> [rustc flags...]"""
import json, os, subprocess, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import posunion as U
from poslib import *
from pos import CFGS

spec = json.load(open(sys.argv[1], encoding="utf-8"))
side, cfg, tag = sys.argv[2], sys.argv[3], sys.argv[4]
extra = [x for x in sys.argv[5:] if x != "--priv-only"]
priv_only = "--priv-only" in sys.argv[5:]
mans = Manifests(Source(None))
mans.build_index()
pk = spec["crates"] if side == "cur" else spec["base"]["crates"]
src = Source(None) if side == "cur" else Source(spec["base"]["rev"])
crates = U.prepare(src, mans, pk, False)
en = U.feature_closure(crates, "test" in cfg, mans)
for c in crates:
    c.features = en[c.pkg]
d = f"{SANDBOX_BASE}/{spec['name']}/{side}"
out = d + "/__out_dbg.rmeta"
Sandbox(SANDBOX_BASE).check(out)
cmd = ["rustc", TOOLCHAIN, "--edition", "2021", "--crate-type", "lib", "--crate-name", "pos_union", "--emit=metadata", "-o", out, "--cap-lints", "allow", "-Ztrim-diagnostic-paths=no", *U.cfg_args(crates), *CFGS[cfg], *extra, "__pos_root.rs"]
p = subprocess.run(cmd, capture_output=True, text=True, cwd=d)
if os.path.exists(out):
    os.remove(out)
if priv_only:
    import re as _re
    rx = _re.compile(r'"code":\{"code":"(?:E0603|E0616|E0624|E0451)"')
    p.stderr = "\n".join(l for l in p.stderr.split("\n") if rx.search(l[:2000])) + "\n"
log_write(f"{GEN}/{PREFIX}-{spec['name']}-{tag}.txt", p.stderr)
print(len(p.stderr), p.returncode)
