"""🎨 C4P28 formatting debt: rustfmt --check hunk count (pinned nightly, repo rustfmt.toml, skip_children) of each ORIGINAL snapshot versus the sum over the root and its children (live tree). usage: fmtdebt.py"""
import glob, json, os, subprocess, sys

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = "/Users/ueli/Documents/semio"


def hunks(path):
    r = subprocess.run(["rustfmt", "+nightly-2026-07-20", "--edition", "2021", "--check", "--config-path", REPO + "/rustfmt.toml", "--config", "skip_children=true", path], capture_output=True, text=True, cwd=REPO)
    return sum(1 for l in (r.stdout + r.stderr).split("\n") if l.startswith("Diff in")), r.returncode


tb = ta = 0
for p in sorted(glob.glob(HERE + "/specs/*.json")):
    n = int(os.path.basename(p).split(".")[0])
    s = json.load(open(p, encoding="utf-8"))
    b, rb = hunks(s["original"])
    files = [s["file"]] + [f"{os.path.dirname(s['file'])}/{c['folder']}/🦀️.rs" for c in s["chunks"]]
    a = sum(hunks(f"{REPO}/{f}")[0] for f in files)
    tb += b
    ta += a
    print(f"[{n}] rustfmt --check hunks: original {b} -> root plus children {a}")
print(f"TOTAL original {tb} -> split {ta}")
