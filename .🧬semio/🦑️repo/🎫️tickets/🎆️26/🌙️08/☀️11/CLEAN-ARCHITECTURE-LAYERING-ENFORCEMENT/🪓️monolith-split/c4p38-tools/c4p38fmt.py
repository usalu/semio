"""🎨 c4p38 rustfmt: hunk counts (`rustfmt --check`, repo config, skip_children) of each snapshot versus the live root and children, plus a parse check (--emit stdout) of every live file.
usage: c4p38fmt.py"""
import os, re, subprocess, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from c4p38lib import *
from plans38 import PLANS, ORDER

CFG = ["+nightly-2026-07-20", "--edition", "2021", "--config-path", ROOT, "--config", "skip_children=true"]


def hunks(path):
    p = subprocess.run(["rustfmt"] + CFG + ["--check", path], capture_output=True, text=True)
    return len(re.findall(r"^Diff in ", p.stdout, flags=re.M)), p.returncode, p.stderr.strip()[:200]


def parses(path):
    p = subprocess.run(["rustfmt"] + CFG + ["--emit", "stdout", path], capture_output=True, text=True)
    return p.returncode == 0, p.stderr.strip()[:200]


tot_base = tot_new = 0
bad = []
for key in ORDER:
    spec = PLANS[key]
    base_dir = os.path.dirname(spec["rel"])
    snap = f"{WORK}/fmt-{spec['n']}.rs"
    write(snap, orig_text(spec["n"]))
    hb, rc, err = hunks(snap)
    files = [spec["rel"]] + [f"{base_dir}/{c['folder']}/🦀️.rs" for c in spec["children"]]
    hn = 0
    for rel in files:
        h, rc2, err2 = hunks(ROOT + "/" + rel)
        ok, perr = parses(ROOT + "/" + rel)
        if not ok:
            bad.append((rel, perr))
        hn += h
    tot_base += hb
    tot_new += hn
    print(f"[{key}] rustfmt hunks snapshot={hb} live-total={hn} (rc snapshot {rc}) parse failures so far {len(bad)}")
print("TOTAL hunks snapshot", tot_base, "live", tot_new, "parse failures", bad)
