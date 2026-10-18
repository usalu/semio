"""🔁 OHF sandbox rerun: runs the oracle rustc command again inside an existing sandbox (no prepare) and prints error counts by code. Writes only GEN/ohr-<family>-rerun-<tag>.err.
usage: rerun.py <family> <lib|test|wasm|p2> <tag>"""
import collections, json, os, re, subprocess, sys, time
HERE = os.path.dirname(os.path.abspath(__file__))
GEN = os.path.dirname(HERE) + "/🗑️generated"
fam, cfg, tag = sys.argv[1:4]
feats = json.load(open(f"{GEN}/ohr-{fam}-feats.json", encoding="utf-8"))["test" if "test" in cfg else "lib"]
extra = {"lib": [], "test": ["--test"], "wasm": ["--target", "wasm32-unknown-unknown"], "p2": ["--target", "wasm32-wasip2"]}[cfg]
d = f"/Users/ueli/.cache/ohr/ohr-work/{fam}/cur"
cmd = ["rustc", "+nightly-2026-07-20", "--edition", "2021", "--crate-type", "lib", "--crate-name", "pos_union", "--emit=metadata", "-o", d + "/__out_rerun.rmeta", "--cap-lints", "allow", "--error-format=json", "-Ztrim-diagnostic-paths=no", *os.environ.get("OHR_EXTRA", "").split(), *feats, *extra, "__pos_root.rs"]
t = time.time()
p = subprocess.run(cmd, capture_output=True, text=True, cwd=d)
if os.path.exists(d + "/__out_rerun.rmeta"):
    os.remove(d + "/__out_rerun.rmeta")
open(f"{GEN}/ohr-{fam}-rerun-{tag}.err", "w", encoding="utf-8").write(p.stderr)
c = collections.Counter()
for l in p.stderr.split("\n"):
    l = l.strip()
    if l.startswith("{"):
        try:
            x = json.loads(l)
        except Exception:
            continue
        if x.get("level") == "error":
            c[((x.get("code") or {}).get("code") or "-")] += 1
print(round(time.time() - t), "s", sum(c.values()), dict(c.most_common(12)))
