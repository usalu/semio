"""🎨 OSD: rustfmt parse check (skip_children) of every changed or new .rs file of the carve."""
import subprocess, sys, os
ROOT = "/Users/ueli/Documents/semio"
files = set()
for args in (["diff", "--name-only"], ["ls-files", "--others", "--exclude-standard"]):
    out = subprocess.run(["git", "-c", "core.quotepath=false"] + args + ["--", "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory", "🌎️hub/🏗️bootstrap"], cwd=ROOT, capture_output=True, text=True).stdout
    files |= {f for f in out.split("\n") if f.endswith(".rs")}
bad = 0
for f in sorted(files):
    p = subprocess.run(["rustfmt", "+nightly-2026-07-20", "--edition", "2021", "--config", "skip_children=true", "--emit", "stdout", os.path.join(ROOT, f)], capture_output=True, text=True)
    if p.returncode != 0 and "error" in p.stderr:
        bad += 1; print("PARSE ERROR", f, p.stderr[:300])
print("files", len(files), "parse errors", bad)
