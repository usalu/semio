"""📐 rustfmt parse check of every carved .rs file that differs from the original (exit status of `rustfmt --emit stdout`, skip_children)."""
import os, subprocess, sys, filecmp
a, b = sys.argv[1], sys.argv[2]
bad = checked = 0
for dp, dn, fn in os.walk(b):
    if "node_modules" in dp or "🧭️journeys" in dp:
        continue
    for f in fn:
        if not f.endswith(".rs"):
            continue
        new = os.path.join(dp, f)
        old = os.path.join(a, os.path.relpath(new, b))
        if os.path.exists(old) and filecmp.cmp(old, new, shallow=False):
            continue
        checked += 1
        proc = subprocess.run(["rustfmt", "+nightly-2026-07-20", "--edition", "2021", "--emit", "stdout", "--config", "skip_children=true,unstable_features=true", new], capture_output=True, text=True)
        if proc.returncode != 0 or "error" in proc.stderr[:400]:
            bad += 1
            print("FAIL", os.path.relpath(new, b), proc.stderr[:300])
print("checked", checked, "failed", bad)
